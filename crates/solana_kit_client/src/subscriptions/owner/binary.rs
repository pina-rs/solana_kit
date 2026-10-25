//! Binary fan-out reserves aggregate capacity before copying any frame bytes.

use super::{Connection, Error, Failure, NotificationKind, Payload, Phase, SubscriptionMethod};

impl Connection {
    /// Requires one routing identity because binary frames have no per-message tag.
    pub(super) fn binary_namespace<R: SubscriptionMethod>(&self) -> Result<(), Failure> {
        let NotificationKind::Binary(namespace) = R::NOTIFICATION_KIND else {
            return Ok(());
        };

        if namespace.is_empty() {
            return Err(Failure::Method);
        }

        let ambiguous = self.physical.values().any(|entry| {
            let NotificationKind::Binary(existing) = entry.kind else {
                return false;
            };
            (!matches!(entry.phase, Phase::Ended) || entry.quarantined)
                && (existing != namespace
                    || entry.subscribe != R::NAME
                    || entry.unsubscribe != R::UNSUBSCRIBE
                    || entry.notification != R::NOTIFICATION
                    || entry.schema != R::schema_identity())
        });

        if ambiguous {
            return Err(Failure::Method);
        }

        Ok(())
    }

    pub(super) fn binary(&mut self, body: &[u8]) -> Result<(), Failure> {
        if body.len() > self.budget.frame.bytes {
            return Err(Failure::Protocol(Error::BodyLimit));
        }

        if !self
            .physical
            .values()
            .any(|entry| matches!(entry.kind, NotificationKind::Binary(_)))
        {
            return Err(Failure::Method);
        }

        let keys: Vec<_> = self
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                let physical = self.physical.get(&entry.physical)?;
                (matches!(physical.kind, NotificationKind::Binary(_))
                    && matches!(physical.phase, Phase::Subscribing { .. } | Phase::Active(_))
                    && super::sharing::receives(entry))
                .then_some(*key)
            })
            .collect();
        self.preflight(&keys, body.len())?;

        for key in keys {
            let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
            entry.bytes += body.len();
            entry.queue.push_back(Payload::Binary(body.to_vec()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;

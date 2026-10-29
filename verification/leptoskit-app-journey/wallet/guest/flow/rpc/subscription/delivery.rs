//! A decoded update can outlive the subscription that admitted its callback.

#[derive(Clone, Copy, Debug)]
pub(super) struct Update {
    pub generation: u64,
    pub lamports: u64,
}

pub(super) struct Delivery {
    generation: Option<u64>,
    pub balance: Option<u64>,
}

impl Delivery {
    pub fn new(generation: u64) -> Self {
        Self {
            generation: Some(generation),
            balance: None,
        }
    }

    pub fn apply(&mut self, update: Update) -> bool {
        if self.generation != Some(update.generation) {
            return false;
        }

        self.balance = Some(update.lamports);
        true
    }

    pub fn cancel(&mut self) {
        self.generation = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_GENERATION: u64 = 7;
    const REPLACEMENT_GENERATION: u64 = 8;
    const ACCEPTED_BALANCE: u64 = 1_000_000;
    const HELD_BALANCE: u64 = 2_000_000;
    const REPLACEMENT_BALANCE: u64 = 3_000_000;

    #[test]
    fn an_accepted_notification_cannot_commit_after_its_generation_is_cancelled() {
        let mut delivery = Delivery::new(FIRST_GENERATION);
        assert!(delivery.apply(Update {
            generation: FIRST_GENERATION,
            lamports: ACCEPTED_BALANCE,
        }));
        let held = Update {
            generation: FIRST_GENERATION,
            lamports: HELD_BALANCE,
        };
        delivery.cancel();
        assert!(!delivery.apply(held));
        assert_eq!(delivery.balance, Some(ACCEPTED_BALANCE));
    }

    #[test]
    fn an_old_generation_cannot_change_a_replacement_subscription() {
        let mut delivery = Delivery::new(REPLACEMENT_GENERATION);
        assert!(!delivery.apply(Update {
            generation: FIRST_GENERATION,
            lamports: HELD_BALANCE,
        }));
        assert_eq!(delivery.balance, None);
        assert!(delivery.apply(Update {
            generation: REPLACEMENT_GENERATION,
            lamports: REPLACEMENT_BALANCE,
        }));
        assert_eq!(delivery.balance, Some(REPLACEMENT_BALANCE));
    }
}

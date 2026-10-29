//! Publish one real socket observation without making background traffic redraw the UI.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Activity {
    Idle,
    Waiting {
        context: u64,
        balance: u64,
    },
    Observed {
        context: u64,
        balance: u64,
        slot: u64,
    },
}

impl Activity {
    /// A processed slot must cross the confirmed HTTP observation made after
    /// the late credit. Neither an equal slot nor later background traffic redraws.
    pub fn observe(&mut self, slot: u64) -> bool {
        let Self::Waiting { context, balance } = *self else {
            return false;
        };

        if slot <= context {
            return false;
        }

        *self = Self::Observed {
            context,
            balance,
            slot,
        };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTEXT: u64 = 31;
    const BALANCE: u64 = 3_000_000;

    #[test]
    fn a_socket_notification_must_pass_the_observed_confirmed_context() {
        let mut activity = Activity::Waiting {
            context: CONTEXT,
            balance: BALANCE,
        };

        for slot in [CONTEXT - 1, CONTEXT] {
            assert!(!activity.observe(slot));
            assert_eq!(
                activity,
                Activity::Waiting {
                    context: CONTEXT,
                    balance: BALANCE
                }
            );
        }

        assert!(activity.observe(CONTEXT + 1));
        assert_eq!(
            activity,
            Activity::Observed {
                context: CONTEXT,
                balance: BALANCE,
                slot: CONTEXT + 1,
            }
        );
    }

    #[test]
    fn background_slots_do_not_redraw_before_or_after_the_explicit_observation() {
        let mut idle = Activity::Idle;
        assert!(!idle.observe(CONTEXT));
        assert_eq!(idle, Activity::Idle);

        let mut activity = Activity::Waiting {
            context: CONTEXT,
            balance: BALANCE,
        };
        assert!(activity.observe(CONTEXT + 1));
        let observed = activity;
        assert!(!activity.observe(CONTEXT + 2));
        assert_eq!(activity, observed);
    }
}

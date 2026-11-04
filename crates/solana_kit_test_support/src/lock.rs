//! A poisoned fixture invariant must fail the consuming test visibly.

use std::sync::{Mutex, MutexGuard};

pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().expect("scripted fixture state poisoned")
}

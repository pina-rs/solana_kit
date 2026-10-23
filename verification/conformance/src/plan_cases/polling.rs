//! Local fixture polling with an actual registered wake token and no host pump.

use alloc::{boxed::Box, sync::Arc, task::Wake};
use core::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};

const MAX_FIXTURE_POLLS: usize = 1024;

struct Readiness(AtomicBool);

impl Wake for Readiness {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
}

pub(super) fn block_on<F: Future>(future: F) -> F::Output {
    let readiness = Arc::new(Readiness(AtomicBool::new(true)));
    let waker = Waker::from(readiness.clone());
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    for _ in 0..MAX_FIXTURE_POLLS {
        core::assert!(
            readiness.0.swap(false, Ordering::SeqCst),
            "fixture did not wake"
        );

        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }

    panic!("fixture exceeded its finite polling bound")
}

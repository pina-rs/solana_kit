use solana_kit_client::{Client, Limits, TransportError, TransportFailureKind, methods::Balance};
use solana_kit_core::Address;
use solana_kit_test_support::{PendingExchange, ScriptStep, ScriptedTransport};
use std::{
    fmt,
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    task::{Context, Wake, Waker},
    thread,
    time::Duration,
};

struct ObserveDrop {
    pending: PendingExchange,
    observed: Arc<AtomicBool>,
}

impl Drop for ObserveDrop {
    fn drop(&mut self) {
        let _ = self.pending.status();
        self.observed.store(true, Ordering::SeqCst);
    }
}

impl Wake for ObserveDrop {
    fn wake(self: Arc<Self>) {
        let _ = self.pending.status();
    }
}

impl fmt::Debug for ObserveDrop {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ObserveDrop")
    }
}

impl fmt::Display for ObserveDrop {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("fixture failure")
    }
}

impl std::error::Error for ObserveDrop {}

fn bounded(work: impl FnOnce() + Send + 'static) {
    let (sender, receiver) = mpsc::channel();
    let worker = thread::spawn(move || {
        work();
        sender.send(()).unwrap();
    });

    receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("fixture callbacks must be able to reenter pending status");
    worker.join().unwrap();
}

#[test]
fn cancelling_a_pending_call_drops_its_waker_without_holding_state() {
    bounded(|| {
        let (step, pending) = ScriptStep::pending();
        let client = Client::new(ScriptedTransport::new([step]), Limits::default());
        let mut call = Box::pin(client.call(Balance(Address::from_bytes([1; 32]), None)));
        let observed = Arc::new(AtomicBool::new(false));

        {
            let waker = Waker::from(Arc::new(ObserveDrop {
                pending: pending.clone(),
                observed: observed.clone(),
            }));
            assert!(
                call.as_mut()
                    .poll(&mut Context::from_waker(&waker))
                    .is_pending()
            );
        }

        drop(call);
        assert!(observed.load(Ordering::SeqCst));
        assert!(pending.status().cancelled);
    });
}

#[test]
fn replacing_a_waker_allows_its_destructor_to_observe_pending_status() {
    bounded(|| {
        let (step, pending) = ScriptStep::pending();
        let client = Client::new(ScriptedTransport::new([step]), Limits::default());
        let mut call = Box::pin(client.call(Balance(Address::from_bytes([1; 32]), None)));
        let observed = Arc::new(AtomicBool::new(false));

        {
            let waker = Waker::from(Arc::new(ObserveDrop {
                pending: pending.clone(),
                observed: observed.clone(),
            }));
            assert!(
                call.as_mut()
                    .poll(&mut Context::from_waker(&waker))
                    .is_pending()
            );
        }

        assert!(
            call.as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert!(observed.load(Ordering::SeqCst));
        assert!(!pending.status().cancelled);
    });
}

#[test]
fn cancelling_a_completed_call_drops_its_error_source_without_holding_state() {
    bounded(|| {
        let (step, pending) = ScriptStep::pending();
        let client = Client::new(ScriptedTransport::new([step]), Limits::default());
        let mut call = Box::pin(client.call(Balance(Address::from_bytes([1; 32]), None)));
        let observed = Arc::new(AtomicBool::new(false));
        assert!(
            call.as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert!(pending.complete(Err(TransportError::with_source(
            TransportFailureKind::Io,
            ObserveDrop {
                pending: pending.clone(),
                observed: observed.clone()
            },
        ))));

        drop(call);
        assert!(observed.load(Ordering::SeqCst));
        assert!(pending.status().cancelled);
    });
}

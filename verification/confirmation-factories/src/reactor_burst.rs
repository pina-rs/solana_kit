#[path = "reactor_frames.rs"]
mod frames;

use frames::ReadySocket;
use solana_kit_client::subscriptions::{Budget, Clock, Drain, Root, Session, Subscription};
use solana_kit_wasi::{DriveError, Reactor, WasiClock};
use std::future::{Future, pending, poll_fn};

const GENERATION: u64 = 7;
const UNRELATED_FRAMES: usize = 33;
const WANTED_VALUE: u64 = 99;
const TIMER_MILLIS: u64 = 1000;
type Owner = Session<ReadySocket>;

pub(super) fn run() {
    let mode = std::env::args().nth(1).expect("composition case");
    let reactor = Reactor::default();

    if mode == "unwakeable" {
        assert_eq!(
            reactor.block_on(pending::<()>()),
            Err(DriveError::NoReadiness)
        );
        println!("PASS unwakeable explicit NoReadiness");

        return;
    }

    assert!(mode == "no-signal" || mode == "with-timer");
    burst(&reactor, mode == "with-timer");
    println!(
        "PASS {mode}: 33 unrelated frames retained; requested frame delivered; timer stayed pending"
    );
}

fn burst(reactor: &Reactor, with_timer: bool) {
    let clock = WasiClock::new(reactor.clone());
    let deadline = clock.now() + TIMER_MILLIS;
    let socket = ReadySocket::new(UNRELATED_FRAMES, WANTED_VALUE);
    let mut session = Session::new(socket, GENERATION, Budget::default());
    let other = session.subscribe(Root, deadline).unwrap();
    let wanted = session.subscribe(Root, deadline).unwrap();

    let case = Case {
        reactor,
        clock: &clock,
        deadline,
        with_timer,
    };
    let observed = case.requested(&mut session, &wanted);
    assert!(matches!(observed, Drain::Notification(WANTED_VALUE)));
    case.drain_unrelated(&mut session, &other);
    reactor
        .block_on(session.shutdown(&clock, deadline))
        .unwrap()
        .unwrap();
    assert_eq!(reactor.registrations(), 0);
}

struct Case<'a> {
    reactor: &'a Reactor,
    clock: &'a WasiClock,
    deadline: u64,
    with_timer: bool,
}

impl Case<'_> {
    fn requested(&self, session: &mut Owner, wanted: &Subscription<Root>) -> Drain<u64> {
        let mut timer = self
            .with_timer
            .then(|| Box::pin(self.clock.sleep_until(self.deadline)));
        let observed = self
            .reactor
            .block_on(poll_fn(|cx| {
                if let Some(timer) = timer.as_mut() {
                    assert!(
                        timer.as_mut().poll(cx).is_pending(),
                        "unrelated timer drove session"
                    );
                }

                session.poll_next(wanted, cx, self.clock.now(), self.deadline)
            }))
            .expect("self-woken session must progress without host readiness")
            .unwrap();
        drop(timer);

        observed
    }

    fn drain_unrelated(&self, session: &mut Owner, other: &Subscription<Root>) {
        for expected in 0..UNRELATED_FRAMES {
            let value = self
                .reactor
                .block_on(poll_fn(|cx| {
                    session.poll_next(other, cx, self.clock.now(), self.deadline)
                }))
                .unwrap()
                .unwrap();
            let Drain::Notification(value) = value else {
                panic!("unrelated notification missing");
            };
            assert_eq!(value, expected as u64);
        }
    }
}

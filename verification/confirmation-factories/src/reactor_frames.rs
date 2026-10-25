use solana_kit_client::subscriptions::{ReceivedFrame, TextSocket};
use std::{
    collections::VecDeque,
    convert::Infallible,
    task::{Context, Poll},
};

pub(super) const OTHER_REMOTE: u64 = 70;
pub(super) const WANTED_REMOTE: u64 = 71;
pub(super) struct ReadySocket(VecDeque<String>);

impl ReadySocket {
    pub(super) fn new(unrelated: usize, wanted: u64) -> Self {
        let mut frames = VecDeque::from([
            format!(r#"{{"jsonrpc":"2.0","id":1,"result":{OTHER_REMOTE}}}"#),
            format!(r#"{{"jsonrpc":"2.0","id":2,"result":{WANTED_REMOTE}}}"#),
        ]);

        for value in 0..unrelated {
            frames.push_back(notification(OTHER_REMOTE, value as u64));
        }

        frames.push_back(notification(WANTED_REMOTE, wanted));
        Self(frames)
    }
}

fn notification(remote: u64, value: u64) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","method":"rootNotification","params":{{"subscription":{remote},"result":{value}}}}}"#
    )
}

impl TextSocket for ReadySocket {
    type Error = Infallible;

    fn start_send(&mut self, text: &str) -> Result<(), Self::Error> {
        assert!(text.contains("rootSubscribe"));
        Ok(())
    }

    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_receive(
        &mut self,
        _: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, Self::Error>> {
        Poll::Ready(Ok(self.0.pop_front().map(ReceivedFrame::Text)))
    }

    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

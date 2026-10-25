//! Native tasks own sockets only; no interpreter reference crosses an await.
use crate::socket_wire::SocketFailure;
use leptoskit_bridge::CallId;
use solana_kit_client::subscriptions::ReceivedFrame;
use solana_kit_native::{NativeWebSocket, WsConfig, WsError};
use tokio::sync::mpsc;

pub enum Command {
    Send(CallId, String),
    Close(CallId),
}

pub enum Notice {
    Opened(CallId),
    Text(CallId, String),
    Binary(CallId, Vec<u8>),
    Reply(CallId, CallId, Result<(), SocketFailure>),
}

pub struct Exit {
    pub lease: CallId,
    pub close: Option<CallId>,
    pub result: Result<(), SocketFailure>,
}

enum Ready {
    Command(Option<Command>),
    Frame(Result<Option<ReceivedFrame>, WsError>),
}

pub async fn run(
    lease: CallId,
    endpoint: String,
    config: WsConfig,
    mut commands: mpsc::Receiver<Command>,
    notices: mpsc::Sender<Notice>,
) -> Exit {
    let mut socket = match NativeWebSocket::connect(&endpoint, config).await {
        Ok(socket) => socket,
        Err(error) => {
            return Exit {
                lease,
                close: None,
                result: Err(failure(error)),
            };
        }
    };

    let outcome = traffic(&mut socket, lease, &mut commands, &notices).await;

    let (close, result) = match outcome {
        Ok(Some(call)) => (Some(call), socket.close().await.map_err(failure)),
        Ok(None) => (None, Ok(())),
        Err(error) => (None, Err(error)),
    };

    Exit {
        lease,
        close,
        result,
    }
}

/// Receive can be cancelled safely; a cancelled send instead drops the whole actor.
async fn traffic(
    socket: &mut NativeWebSocket,
    lease: CallId,
    commands: &mut mpsc::Receiver<Command>,
    notices: &mpsc::Sender<Notice>,
) -> Result<Option<CallId>, SocketFailure> {
    notice(notices, Notice::Opened(lease)).await?;

    loop {
        let ready = tokio::select! {
            biased;
            command = commands.recv() => Ready::Command(command),
            incoming = socket.receive_frame() => Ready::Frame(incoming),
        };

        match ready {
            Ready::Command(Some(Command::Close(call))) => return Ok(Some(call)),
            Ready::Command(Some(Command::Send(call, text))) => {
                send(socket, notices, lease, call, text).await?;
            }
            Ready::Command(None) => return Err(SocketFailure::Closed),
            Ready::Frame(Ok(Some(ReceivedFrame::Text(text)))) => {
                notice(notices, Notice::Text(lease, text)).await?;
            }
            Ready::Frame(Ok(Some(ReceivedFrame::Binary(bytes)))) => {
                notice(notices, Notice::Binary(lease, bytes)).await?;
            }
            Ready::Frame(Ok(None)) => return Ok(None),
            Ready::Frame(Err(error)) => return Err(failure(error)),
        }
    }
}

async fn send(
    socket: &mut NativeWebSocket,
    notices: &mpsc::Sender<Notice>,
    lease: CallId,
    call: CallId,
    text: String,
) -> Result<(), SocketFailure> {
    let result = socket.send(&text).await.map_err(failure);
    notice(notices, Notice::Reply(lease, call, result)).await?;
    result
}

async fn notice(sender: &mpsc::Sender<Notice>, value: Notice) -> Result<(), SocketFailure> {
    sender.send(value).await.map_err(|_| SocketFailure::Closed)
}

fn failure(error: WsError) -> SocketFailure {
    match error {
        WsError::Limit => SocketFailure::Limit,
        WsError::Closed => SocketFailure::Closed,
        _ => SocketFailure::Io,
    }
}

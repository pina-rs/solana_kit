//! Bounded, joined local fixture owners.
use crate::cases;
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use tungstenite::{Message, WebSocket};
const FIXTURE_TIMEOUT: Duration = Duration::from_secs(5);
const OVERSIZE_BYTES: usize = 65;
const DELIVERY_DELAY: Duration = Duration::from_millis(40);

pub fn serve(
    mut stream: TcpStream,
    handshake: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(FIXTURE_TIMEOUT))?;
    stream.set_write_timeout(Some(FIXTURE_TIMEOUT))?;
    let mut bytes = [0; 4096];
    let size = stream.peek(&mut bytes)?;

    if size == 0 {
        return Ok(());
    }

    let request = std::str::from_utf8(&bytes[..size])?;
    let path = request
        .split_whitespace()
        .nth(1)
        .ok_or("missing fixture path")?;

    if path == "/state" {
        return state(&mut stream, &handshake);
    }

    if path == "/refused" {
        stream.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n")?;

        return Ok(());
    }

    if path == "/stall" {
        let mut request = [0; 4096];
        assert!(stream.read(&mut request)? > 0);
        handshake.store(true, Ordering::SeqCst);
        assert_eq!(stream.read(&mut request)?, 0);

        return Ok(());
    }

    let path = path.to_owned();
    let mut socket = tungstenite::accept_hdr(stream, Protocol)?;

    dispatch(&path, &mut socket)?;
    closed(&mut socket, path == "/early-close")
}
fn state(stream: &mut TcpStream, handshake: &AtomicBool) -> Result<(), Box<dyn std::error::Error>> {
    let body = u8::from(handshake.load(Ordering::SeqCst));
    let response = format!(
        concat!(
            "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\n",
            "Access-Control-Allow-Methods: POST, OPTIONS\r\n",
            "Access-Control-Allow-Headers: content-type\r\n",
            "Content-Length: 1\r\nConnection: close\r\n\r\n{}"
        ),
        body,
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}
fn dispatch(
    path: &str,
    socket: &mut WebSocket<TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    match path {
        "/family" => family(socket)?,
        "/survive" => survive(socket)?,
        "/burst" => {
            assert_eq!(socket.read()?.into_text()?, "go");

            for _ in 0..4 {
                socket.send(Message::Text("flood".into()))?;
            }
        }
        "/binary" => socket.send(Message::Binary(vec![0xff].into()))?,
        "/unicode" => socket.send(Message::Text("💖".into()))?,
        "/unicode-overflow" => socket.send(Message::Text("€€".into()))?,
        "/oversize" => socket.send(Message::Text("x".repeat(OVERSIZE_BYTES).into()))?,
        "/malformed" => socket.send(Message::Text("{invalid".into()))?,
        "/peer-close" => {
            socket.close(None)?;
            let _ = socket.read();
        }
        "/echo" => {
            assert_eq!(socket.read()?.into_text()?, "go");
            thread::sleep(DELIVERY_DELAY);
            socket.send(Message::Text("resumed".into()))?;
        }
        "/quiet" | "/early-close" => {}
        "/send-limit" => {
            assert_eq!(socket.read()?.into_text()?, "1234");
        }
        _ => return Err("unknown fixture path".into()),
    }

    Ok(())
}
struct Protocol;
impl tungstenite::handshake::server::Callback for Protocol {
    fn on_request(
        self,
        request: &tungstenite::handshake::server::Request,
        mut response: tungstenite::handshake::server::Response,
    ) -> Result<
        tungstenite::handshake::server::Response,
        tungstenite::handshake::server::ErrorResponse,
    > {
        if let Some(protocol) = request.headers().get("sec-websocket-protocol") {
            response
                .headers_mut()
                .insert("sec-websocket-protocol", protocol.clone());
        }

        Ok(response)
    }
}
fn family(socket: &mut WebSocket<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let request = socket.read()?.into_text()?;
    let value: serde_json::Value = serde_json::from_str(&request)?;
    let method = value["method"].as_str().ok_or("missing method")?;
    assert_eq!(value["id"], 1);
    assert_eq!(value["jsonrpc"], "2.0");
    let notification = method.replace("Subscribe", "Notification");
    let result = cases::result(method);
    let notification = format!(
        concat!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"{}\",",
            "\"params\":{{\"subscription\":17,\"result\":{}}}}}"
        ),
        notification, result,
    );
    socket.send(Message::Text(notification.into()))?;
    socket.send(Message::Text(
        r#"{"jsonrpc":"2.0","id":1,"result":17}"#.into(),
    ))?;

    if method != "signatureSubscribe" {
        let request = socket.read()?.into_text()?;
        let value: serde_json::Value = serde_json::from_str(&request)?;
        assert_eq!(value["method"], method.replace("Subscribe", "Unsubscribe"));
        assert_eq!(value["params"], serde_json::json!([17]));
        assert_eq!(value["id"], 2);
        socket.send(Message::Text(
            r#"{"jsonrpc":"2.0","id":2,"result":true}"#.into(),
        ))?;
    }

    Ok(())
}
fn survive(socket: &mut WebSocket<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    for id in 1..=2 {
        let request = socket.read()?.into_text()?;
        let value: serde_json::Value = serde_json::from_str(&request)?;
        assert_eq!(value["id"], id);
        socket.send(Message::Text(
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{id}}}"#).into(),
        ))?;
    }

    for text in [
        r#"{"jsonrpc":"2.0","method":"signatureNotification","params":{"subscription":1,"result":{"context":{"slot":1},"value":{"err":"AccountInUse"}}}}"#,
        r#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":2,"result":18446744073709551615}}"#,
        r#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":2,"result":7}}"#,
    ] {
        socket.send(Message::Text(text.into()))?;
    }

    Ok(())
}
fn closed(
    socket: &mut WebSocket<TcpStream>,
    early: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match socket.read() {
        Ok(Message::Close(_)) => match socket.flush() {
            Ok(()) | Err(tungstenite::Error::ConnectionClosed) => Ok(()),
            Err(error) => Err(error.into()),
        },

        Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => Ok(()),

        Err(tungstenite::Error::Protocol(
            tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
        )) if early => Ok(()),
        value => Err(format!("fixture expected terminal socket, received {value:?}").into()),
    }
}

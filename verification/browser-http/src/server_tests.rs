//! A speculative connection can arrive before its first request bytes.
use crate::server;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

#[test]
fn accepted_nonblocking_socket_waits_for_request_bytes() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut peer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
    let (socket, _) = listener.accept().unwrap();
    socket.set_nonblocking(true).unwrap();
    let (tx, rx) = mpsc::channel();
    let owner = thread::spawn(move || {
        let result = server::serve(socket, server::Events::default(), "http://127.0.0.1:1/sink");
        tx.send(result).unwrap();
    });
    let early = rx.recv_timeout(Duration::from_millis(30));
    peer.write_all(b"POST /exact HTTP/1.1\r\nHost: localhost\r\nContent-Length: 2\r\n\r\n{}")
        .unwrap();

    let mut response = String::new();
    peer.read_to_string(&mut response).unwrap();
    owner.join().unwrap();
    assert!(
        matches!(early, Err(mpsc::RecvTimeoutError::Timeout)),
        "fixture returned before the request arrived"
    );
    rx.recv().unwrap().unwrap();
    assert!(response.ends_with("abcdef"));
}

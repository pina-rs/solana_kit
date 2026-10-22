//! Native owned TCP teardown; these tests do not claim guest unload integration.

use solana_kit_async_boundaries::OwnedTask;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

const COMPLETION_LIMIT: Duration = Duration::from_secs(5);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dropping_owner_closes_pending_tcp_io() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback listener");
    let address = listener.local_addr().expect("bound address");
    let (connected, accepted) = tokio::join!(TcpStream::connect(address), listener.accept());
    let mut client = connected.expect("loopback client");
    let (mut server, _) = accepted.expect("loopback server");
    let (started, ready) = oneshot::channel();

    let owner = OwnedTask::new(tokio::spawn(async move {
        let mut bytes = [0; 1];
        started.send(()).expect("owner observer is alive");
        client
            .read(&mut bytes)
            .await
            .expect("read would await remote data")
    }));
    ready.await.expect("task owns socket");
    drop(owner);

    let mut bytes = [0; 1];
    let eof = tokio::time::timeout(COMPLETION_LIMIT, server.read(&mut bytes))
        .await
        .expect("owner abort must close socket promptly")
        .expect("remote read");

    assert_eq!(eof, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dropping_bare_join_handle_detaches_negative_control() {
    let (complete, completion) = oneshot::channel();
    let (observed, observation) = oneshot::channel();
    let handle = tokio::spawn(async move {
        completion.await.expect("completion sender exists");
        observed.send(()).expect("observer exists");
    });
    drop(handle);
    complete
        .send(())
        .expect("dropping bare handle leaves task alive");

    tokio::time::timeout(COMPLETION_LIMIT, observation)
        .await
        .expect("detached task continues")
        .expect("detached task completed");
}

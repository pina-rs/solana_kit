//! Approval uses an independent deadline against the actual encrypted socket peer.
use super::*;

async fn signing_request(
    listener: TcpListener,
    public: mwa::PublicPoint,
) -> (PeerSocket, Session, Value) {
    let (mut socket, mut session) = handshake(listener, public).await;
    let Message::Binary(request) = socket.next().await.unwrap().unwrap() else {
        panic!("expected encrypted signing request");
    };
    let request: Value = serde_json::from_str(&session.decrypt(&request).unwrap()).unwrap();
    assert_eq!(request["method"], "sign_messages");
    (socket, session, request)
}

#[tokio::test]
async fn approval_can_outlast_the_socket_deadline_and_preserve_the_wallet_reply() {
    let (proposal, listener) = association().await;
    let public = proposal.association.public_key();
    let server = tokio::spawn(async move {
        let (mut socket, mut session, request) = signing_request(listener, public).await;
        tokio::time::sleep(Duration::from_millis(250)).await;
        let reply = serde_json::json!({
            "jsonrpc": "2.0", "id": request["id"],
            "error": {"code": -1, "message": "declined after approval"},
        });
        socket
            .send(Message::Binary(
                session
                    .encrypt(&reply.to_string(), &[1; 12])
                    .unwrap()
                    .into(),
            ))
            .await
            .unwrap();
        assert!(matches!(
            socket.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        socket.flush().await.unwrap();
    });
    let mut config = MwaConfig {
        approval_timeout: Duration::from_secs(3),
        ..MwaConfig::default()
    };
    config.socket.operation_timeout = Duration::from_millis(100);
    let mut wallet = proposal.connect(config).await.unwrap();
    assert!(matches!(
        wallet.call(Method::SignMessages, Map::new()).await.unwrap(),
        Response::Error(_)
    ));
    wallet.close().await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn approval_deadline_closes_the_actual_peer_and_invalidates_the_session() {
    use tokio_tungstenite::tungstenite::{Error as SocketError, error::ProtocolError};
    let (proposal, listener) = association().await;
    let public = proposal.association.public_key();
    let server = tokio::spawn(async move {
        let (mut socket, _, _) = signing_request(listener, public).await;
        assert!(matches!(
            socket.next().await,
            None | Some(Ok(Message::Close(_)))
                | Some(Err(SocketError::Protocol(
                    ProtocolError::ResetWithoutClosingHandshake
                )))
        ));
    });
    let config = MwaConfig {
        approval_timeout: Duration::from_millis(50),
        ..MwaConfig::default()
    };
    let mut wallet = proposal.connect(config).await.unwrap();
    assert!(matches!(
        wallet.call(Method::SignMessages, Map::new()).await,
        Err(MwaError::Deadline(_))
    ));
    assert!(matches!(
        wallet.call(Method::GetCapabilities, Map::new()).await,
        Err(MwaError::Closed)
    ));
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}

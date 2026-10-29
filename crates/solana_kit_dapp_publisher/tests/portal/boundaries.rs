//! Bounded protocol envelopes, typed absence and owned cancellation are independently observed.

use super::checks::{Outcome, require};
use super::support::*;
use solana_kit_client::{
    Presence,
    rest::{self, Budgets, JsonError},
};
use solana_kit_dapp_publisher::portal::*;
use std::{
    boxed::Box,
    format,
    future::Future,
    string::ToString,
    task::{Context, Poll, Waker},
    vec,
};

#[cfg_attr(test, test)]
pub fn direct_data_direct_result_batch_and_right_envelopes_share_the_same_typed_boundary() -> Outcome
{
    let envelopes = [
        r#"{"result":{"data":{"action":"deleted"}}}"#,
        r#"{"result":{"action":"deleted"}}"#,
        r#"{"0":{"result":{"data":{"action":"deleted"}}}}"#,
        r#"{"result":{"data":{"_tag":"Right","right":{"action":"deleted"}}}}"#,
    ];

    for envelope in envelopes {
        let (client, _) = client(vec![reply(200, envelope)]);
        let result = complete(client.call(&CleanupRelease {
            release_id: "fixture".into(),
        }))?
        .map_err(|_| "valid portal envelope rejected")?;
        require(result.action == "deleted", "portal envelope action changed")?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn provider_and_http_errors_preserve_bounded_evidence_without_implicit_disclosure() -> Outcome {
    let secret = "synthetic-url-token";
    let envelopes = [
        r#"{"error":{"message":"synthetic-url-token"}}"#,
        r#"{"result":{"data":{"_tag":"Left","left":{"message":"synthetic-url-token"}}}}"#,
    ];

    for envelope in envelopes {
        let (client, _) = client(vec![reply(200, envelope)]);
        let error = complete(client.call(&CleanupRelease {
            release_id: "fixture".into(),
        }))?
        .err()
        .ok_or("provider rejection became success")?;
        require(
            !format!("{error:?} {error}").contains(secret),
            "provider diagnostics disclosed untrusted text",
        )?;

        match error {
            Error::Provider { message, body } => {
                require(
                    message.as_deref() == Some(secret),
                    "provider message evidence changed",
                )?;
                require(
                    body == envelope.as_bytes(),
                    "provider body evidence changed",
                )?;
            }
            _ => return Err("provider error was not preserved"),
        }
    }

    let (client, _) = client(vec![reply(403, secret)]);
    let error = complete(client.call(&CleanupRelease {
        release_id: "fixture".into(),
    }))?
    .err()
    .ok_or("HTTP rejection became success")?;
    require(
        !format!("{error:?} {error}").contains(secret),
        "HTTP diagnostics disclosed untrusted text",
    )?;
    require(
        matches!(error, Error::Http { status:403, body } if body == secret.as_bytes()),
        "HTTP status/body evidence changed",
    )
}

#[cfg_attr(test, test)]
pub fn invalid_envelopes_known_fields_arrays_and_private_number_objects_are_rejected() -> Outcome {
    for body in [
        "[]",
        "{}",
        r#"{"result":null}"#,
        r#"{"result":{},"error":{}}"#,
        r#"{"0":[],"result":{"action":"deleted"}}"#,
        r#"{"0":{"result":{"action":"deleted"}},"result":{"action":"conflict"}}"#,
        r#"{"0":null,"result":{"action":"deleted"}}"#,
        r#"{"0":{"0":{},"result":{"action":"deleted"}}}"#,
        r#"{"result":{"data":[]}}"#,
        r#"{"result":{"data":{"_tag":"future","right":{}}}}"#,
        r#"{"result":{"data":{"_tag":null,"action":"deleted"}}}"#,
        r#"{"result":{"data":{"_tag":"Right","right":{},"left":{}}}}"#,
        r#"{"result":{"data":{"action":null}}}"#,
        r#"{"result":{"data":{"action":1}}}"#,
        r#"{"result":{"data":{"action":"a","action":"b"}}}"#,
    ] {
        let (client, _) = client(vec![reply(200, body)]);
        require(
            complete(client.call(&CleanupRelease {
                release_id: "fixture".into(),
            }))?
            .is_err(),
            "invalid portal envelope accepted",
        )?;
    }

    for data in [
        r#"{"dapp":[]}"#,
        r#"{"release":{"versionCode":"1"}}"#,
        r#"{"release":{"versionCode":{"$serde_json::private::Number":"1"}}}"#,
    ] {
        let (client, _) = client(vec![success(data)]);
        require(
            complete(client.call(&GetPublicationBundle {
                release_id: "fixture".into(),
            }))?
            .is_err(),
            "invalid known bundle field accepted",
        )?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn optional_backend_fields_keep_missing_null_false_and_exact_numbers_distinct() -> Outcome {
    let payload = r#"{"releaseId":null,"dapp":{"languages":[]},"release":{"versionCode":9007199254740993},"signerAuthority":{"sameSignerRequired":false},"futureField":true}"#;
    let (client, _) = client(vec![success(payload)]);
    let bundle = complete(client.call(&GetPublicationBundle {
        release_id: "fixture".into(),
    }))?
    .map_err(|_| "valid optional backend fields rejected")?;
    require(
        matches!(bundle.ingestion_session_id, Presence::Missing),
        "missing backend identity changed",
    )?;
    require(
        matches!(bundle.release_id, Presence::Present(None)),
        "null backend identity changed",
    )?;
    let Presence::Present(Some(release)) = bundle.release else {
        return Err("release missing");
    };
    let Presence::Present(Some(number)) = release.version_code else {
        return Err("number missing");
    };
    require(
        number.to_string() == "9007199254740993",
        "full-width JSON number changed",
    )?;
    let Presence::Present(Some(authority)) = bundle.signer_authority else {
        return Err("authority missing");
    };
    require(
        matches!(
            authority.same_signer_required,
            Presence::Present(Some(false))
        ),
        "false optional signer flag changed",
    )?;
    let target = PortalUploadTarget {
        upload_url: "synthetic-secret".into(),
        public_url: "synthetic-public".into(),
    };
    require(
        !format!("{target:?}").contains("synthetic-secret"),
        "upload target diagnostics disclosed its capability",
    )
}

#[cfg_attr(test, test)]
pub fn request_and_response_budgets_fail_without_empty_success_or_unbounded_exchange() -> Outcome {
    let state = std::rc::Rc::new(State::default());
    let client = Client::new(
        Transport(state.clone()),
        Budgets {
            bytes: 2,
            ..BUDGETS
        },
    );
    require(
        matches!(
            complete(client.call(&CleanupRelease {
                release_id: "too-large".into()
            }))?,
            Err(Error::Json(JsonError::Bytes))
        ),
        "oversized portal input was not rejected",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "oversized input reached transport",
    )?;

    let client = Client::new(
        Transport(state.clone()),
        Budgets {
            bytes: 4,
            ..BUDGETS
        },
    );
    require(
        matches!(
            complete(client.call(&GetBlockData {}))?,
            Err(Error::Transport(error)) if error.kind == solana_kit_client::TransportFailureKind::BodyLimit
        ),
        "oversized portal request target was not rejected",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "oversized target reached transport",
    )?;

    for budgets in [
        Budgets {
            bytes: 64,
            ..BUDGETS
        },
        Budgets {
            depth: 1,
            ..BUDGETS
        },
        Budgets {
            items: 2,
            ..BUDGETS
        },
        Budgets {
            string_bytes: 3,
            ..BUDGETS
        },
    ] {
        let state = std::rc::Rc::new(State::default());
        let payload = format!(r#"{{"action":"deleted","padding":"{}"}}"#, "x".repeat(64));
        state.replies.borrow_mut().push_back(success(&payload));
        let client = Client::new(Transport(state.clone()), budgets);
        require(
            matches!(
                complete(client.call(&GetBlockData {}))?,
                Err(Error::Json(_))
            ),
            "portal response admission cap was not enforced",
        )?;
        require(
            state.requests.borrow().len() == 1,
            "bounded response exchange count changed",
        )?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn dropping_a_pending_portal_future_cancels_the_real_owned_exchange() -> Outcome {
    let (client, state) = client(vec![None]);
    let request = GetBlockData {};
    let mut operation = Box::pin(client.call(&request));
    let poll = operation
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()));
    require(
        matches!(poll, Poll::Pending),
        "pending portal exchange completed",
    )?;
    require(
        state.requests.borrow().len() == 1,
        "pending exchange count changed",
    )?;
    require(
        state.cancelled.get() == 0,
        "live portal exchange already cancelled",
    )?;
    drop(operation);
    require(
        state.cancelled.get() == 1,
        "dropped portal exchange was not cancelled",
    )
}

#[cfg_attr(test, test)]
pub fn concrete_native_adapters_preserve_send_while_rc_adapters_remain_usable() -> Outcome {
    fn requires_send(_: impl Send) {}
    let client = Client::new(Native, rest::Budgets::default());
    requires_send(client.call(&CleanupRelease {
        release_id: "fixture".into(),
    }));
    Ok(())
}

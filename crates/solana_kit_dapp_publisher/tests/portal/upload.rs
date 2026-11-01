//! The portal transport and binary uploader receive independent capability/data objects.

use super::checks::{Outcome, require};
use super::support::*;
use solana_kit_client::TransportError;
use solana_kit_dapp_publisher::portal::*;
use std::{
    boxed::Box,
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    string::String,
    task::{Context, Poll, Waker},
    vec,
    vec::Vec,
};

#[cfg(not(target_arch = "wasm32"))]
use std::future::{Ready, ready};

struct Observation {
    url: String,
    body: Vec<u8>,
    content_type: String,
}

struct Uploads {
    calls: Cell<usize>,
    cancelled: Cell<usize>,
    pending: bool,
    observed: RefCell<Vec<Observation>>,
}

impl Uploader for Uploads {
    type Upload<'a> = Upload<'a>;

    fn upload<'a>(&'a self, request: UploadRequest<'a>) -> Self::Upload<'a> {
        self.calls.set(self.calls.get() + 1);
        self.observed.borrow_mut().push(Observation {
            url: request.target.upload_url.clone(),
            body: request.body.to_vec(),
            content_type: request.content_type.into(),
        });
        Upload {
            owner: self,
            completed: false,
        }
    }
}

fn check_observations(uploader: &Uploads) -> Outcome {
    let observed = uploader.observed.borrow();
    require(!observed.is_empty(), "binary upload was not observed")?;

    for request in observed.iter() {
        require(
            request.url == "https://upload.invalid/?signature=synthetic",
            "binary upload capability changed",
        )?;
        require(request.body == [1, 2, 3], "binary upload bytes changed")?;
        require(
            request.content_type == "application/octet-stream",
            "binary upload content type changed",
        )?;
    }

    Ok(())
}

struct Upload<'a> {
    owner: &'a Uploads,
    completed: bool,
}

impl Future for Upload<'_> {
    type Output = Result<(), TransportError>;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.owner.pending {
            return Poll::Pending;
        }
        self.completed = true;
        Poll::Ready(Ok(()))
    }
}

impl Drop for Upload<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.owner.cancelled.set(self.owner.cancelled.get() + 1);
        }
    }
}

fn request(target: &PortalUploadTarget) -> UploadRequest<'_> {
    UploadRequest {
        target,
        body: &[1, 2, 3],
        content_type: "application/octet-stream",
        max_bytes: 3,
    }
}

#[cfg_attr(test, test)]
pub fn requesting_a_target_does_not_upload_and_uploading_cannot_access_portal_auth() -> Outcome {
    let (client, state) = client(vec![success(
        r#"{"uploadUrl":"https://upload.invalid/?signature=synthetic","publicUrl":"https://public.invalid/artifact"}"#,
    )]);
    let target = complete(client.call(&CreateUploadTarget {
        file_hash: "synthetic".into(),
        file_extension: "bin".into(),
        content_type: "application/octet-stream".into(),
    }))?
    .map_err(|_| "valid upload target rejected")?;
    let uploader = Uploads {
        calls: Cell::new(0),
        cancelled: Cell::new(0),
        pending: false,
        observed: RefCell::new(Vec::new()),
    };
    require(
        uploader.calls.get() == 0,
        "requesting target already uploaded bytes",
    )?;
    complete(upload_bytes(&uploader, request(&target)))?
        .map_err(|_| "valid independent binary upload rejected")?;
    check_observations(&uploader)?;
    require(
        uploader.calls.get() == 1,
        "binary upload call count changed",
    )?;
    require(
        state.requests.borrow().len() == 1,
        "binary upload reused portal transport",
    )
}

#[cfg_attr(test, test)]
pub fn body_limits_header_rejection_and_drop_are_enforced_before_or_during_upload() -> Outcome {
    let uploader = Uploads {
        calls: Cell::new(0),
        cancelled: Cell::new(0),
        pending: true,
        observed: RefCell::new(Vec::new()),
    };
    let target = PortalUploadTarget {
        upload_url: "https://upload.invalid/?signature=synthetic".into(),
        public_url: "https://public.invalid/artifact".into(),
    };
    require(
        matches!(
            complete(upload_bytes(
                &uploader,
                UploadRequest {
                    max_bytes: 2,
                    ..request(&target)
                }
            ))?,
            Err(Error::Request(_))
        ),
        "binary upload body cap was not enforced",
    )?;
    require(
        matches!(
            complete(upload_bytes(
                &uploader,
                UploadRequest {
                    content_type: "a\r\nb",
                    ..request(&target)
                }
            ))?,
            Err(Error::Request(_))
        ),
        "binary upload header injection accepted",
    )?;
    require(
        uploader.calls.get() == 0,
        "invalid binary upload reached adapter",
    )?;
    let mut operation = Box::pin(upload_bytes(&uploader, request(&target)));
    require(
        matches!(
            operation
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        ),
        "pending binary upload completed",
    )?;
    require(
        uploader.calls.get() == 1,
        "pending binary upload call count changed",
    )?;
    check_observations(&uploader)?;
    require(
        uploader.cancelled.get() == 0,
        "live binary upload already cancelled",
    )?;
    drop(operation);
    require(
        uploader.cancelled.get() == 1,
        "binary upload did not cancel on drop",
    )
}

#[cfg(not(target_arch = "wasm32"))]
struct Native;
#[cfg(not(target_arch = "wasm32"))]
impl Uploader for Native {
    type Upload<'a> = Ready<Result<(), TransportError>>;
    fn upload<'a>(&'a self, _: UploadRequest<'a>) -> Self::Upload<'a> {
        ready(Ok(()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(test, test)]
pub fn concrete_native_uploader_futures_preserve_send_without_imposing_it_on_local_uploaders()
-> Outcome {
    fn requires_send(_: impl Send) {}
    let target = PortalUploadTarget {
        upload_url: "https://upload.invalid/?signature=synthetic".into(),
        public_url: "https://public.invalid/artifact".into(),
    };
    requires_send(upload_bytes(&Native, request(&target)));
    Ok(())
}

#[cfg(test)]
mod observation_controls {
    use super::*;

    #[test]
    fn a_later_valid_upload_cannot_hide_any_earlier_data_mismatch() -> Outcome {
        earlier_mismatch(
            "https://wrong.invalid/target",
            &[1, 2, 3],
            "application/octet-stream",
            "binary upload capability changed",
        )?;
        earlier_mismatch(
            "https://upload.invalid/?signature=synthetic",
            &[9],
            "application/octet-stream",
            "binary upload bytes changed",
        )?;
        earlier_mismatch(
            "https://upload.invalid/?signature=synthetic",
            &[1, 2, 3],
            "application/different",
            "binary upload content type changed",
        )
    }

    fn earlier_mismatch(url: &str, body: &[u8], mime: &str, expected: &'static str) -> Outcome {
        let uploader = Uploads {
            calls: Cell::new(0),
            cancelled: Cell::new(0),
            pending: false,
            observed: RefCell::new(Vec::new()),
        };
        let mut target = PortalUploadTarget {
            upload_url: url.into(),
            public_url: "https://public.invalid/artifact".into(),
        };
        complete(uploader.upload(UploadRequest {
            target: &target,
            body,
            content_type: mime,
            max_bytes: 3,
        }))?
        .map_err(|_| "synthetic first upload failed")?;
        target.upload_url = "https://upload.invalid/?signature=synthetic".into();
        complete(uploader.upload(request(&target)))?
            .map_err(|_| "synthetic later upload failed")?;

        require(
            uploader.calls.get() == 2,
            "two invocation control did not execute",
        )?;
        require(
            uploader.observed.borrow().len() == 2,
            "an upload observation was lost",
        )?;
        require(
            check_observations(&uploader) == Err(expected),
            "later valid upload hid an earlier mismatch",
        )
    }
}

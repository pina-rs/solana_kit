//! Actual local suspension, wake, ordered validation and cancellation.

use super::*;

const FIRST_ADDRESS: u8 = 7;
const SECOND_ADDRESS: u8 = 8;

#[derive(Clone, Copy)]
enum Response {
    Foreign,
    Changed,
    Cancelled,
}

struct Backend {
    address: Address,
    delay: usize,
    response: Response,
    polls: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
}

struct Operation<'a> {
    backend: &'a Backend,
    transaction: &'a Transaction,
    remaining: usize,
    local: Rc<Cell<usize>>,
}

impl Future for Operation<'_> {
    type Output = Result<SignatureOutput, Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.local.set(self.local.get() + 1);

        if self.remaining > 0 {
            self.remaining -= 1;
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        Poll::Ready(match self.backend.response {
            Response::Foreign => Ok(SignatureOutput::new(
                Address::from_bytes([200; 32]),
                self.transaction.message().bytes(),
                Signature::from_bytes([1; 64]),
            )),
            Response::Changed => Ok(SignatureOutput::new(
                self.backend.address,
                b"different snapshot",
                Signature::from_bytes([1; 64]),
            )),
            Response::Cancelled => Err(Error::Cancelled),
        })
    }
}

impl Drop for Operation<'_> {
    fn drop(&mut self) {
        self.backend.drops.set(self.backend.drops.get() + 1);
    }
}

impl Signer for Backend {
    fn address(&self) -> Address {
        self.address
    }
}

impl TransactionPartialSigner for Backend {
    type Sign<'a> = Operation<'a>;
    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a> {
        Operation {
            backend: self,
            transaction,
            remaining: self.delay,
            local: self.polls.clone(),
        }
    }
}

fn backend(seed: u8, delay: usize, response: Response) -> Backend {
    Backend {
        address: Address::from_bytes([seed; 32]),
        delay,
        response,
        polls: Rc::new(Cell::new(0)),
        drops: Rc::new(Cell::new(0)),
    }
}

fn ordered() -> Outcome {
    let (a, b) = (
        backend(FIRST_ADDRESS, 2, Response::Foreign),
        backend(SECOND_ADDRESS, 1, Response::Changed),
    );
    let (first_handle, second_handle) = (SignerCapability::new(&a), SignerCapability::new(&b));
    let composition = Composition::new(vec![
        Registration::partial(&first_handle),
        Registration::partial(&second_handle),
    ])
    .map_err(|_| "local composition")?;
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut cx = Context::from_waker(&waker);
    let tx = transaction(a.address())?;
    let mut future = Box::pin(composition.partially_sign(&tx));

    crate::require(
        future.as_mut().poll(&mut cx).is_pending() && (a.polls.get(), b.polls.get()) == (1, 1),
        "both local futures must suspend",
    )?;

    crate::require(
        future.as_mut().poll(&mut cx).is_pending() && (a.drops.get(), b.drops.get()) == (0, 1),
        "second local future must complete first",
    )?;

    crate::require(
        future.as_mut().poll(&mut cx) == Poll::Ready(Err(Error::ForeignSigner)),
        "merge validation must follow input order",
    )?;
    crate::require(
        counter.0.load(Ordering::SeqCst) == 3,
        "actual local wake count",
    )?;
    drop(future);

    reversed(&first_handle, &second_handle, &tx, &mut cx)
}

fn reversed<'a>(
    first: &'a SignerCapability<'a, Backend>,
    second: &'a SignerCapability<'a, Backend>,
    tx: &Transaction,
    cx: &mut Context<'_>,
) -> Outcome {
    let reverse = Composition::new(vec![
        Registration::partial(second),
        Registration::partial(first),
    ])
    .map_err(|_| "reverse local composition")?;
    let mut future = Box::pin(reverse.partially_sign(tx));

    crate::require(
        future.as_mut().poll(cx).is_pending(),
        "reverse first suspension",
    )?;
    crate::require(
        future.as_mut().poll(cx).is_pending(),
        "reverse second suspension",
    )?;
    crate::require(
        future.as_mut().poll(cx) == Poll::Ready(Err(Error::ChangedSnapshot)),
        "reverse merge error precedence",
    )
}

fn cancelled() -> Outcome {
    let (a, b) = (
        backend(FIRST_ADDRESS, 0, Response::Cancelled),
        backend(SECOND_ADDRESS, 1, Response::Foreign),
    );
    let (first_handle, second_handle) = (SignerCapability::new(&a), SignerCapability::new(&b));
    let composition = Composition::new(vec![
        Registration::partial(&first_handle),
        Registration::partial(&second_handle),
    ])
    .map_err(|_| "cancel composition")?;
    let waker = Waker::from(Arc::new(Counter::default()));
    let mut cx = Context::from_waker(&waker);
    let tx = transaction(a.address())?;
    let mut future = Box::pin(composition.partially_sign(&tx));

    crate::require(
        future.as_mut().poll(&mut cx) == Poll::Ready(Err(Error::Cancelled)),
        "backend cancellation must propagate",
    )?;

    crate::require(
        (a.polls.get(), b.polls.get(), a.drops.get(), b.drops.get()) == (1, 0, 1, 1),
        "cancellation must drop constructed unpolled operation",
    )?;
    drop(future);

    let pending = Composition::new(Registration::partial(&second_handle))
        .map_err(|_| "pending composition")?;
    let mut future = Box::pin(pending.partially_sign(&tx));
    crate::require(
        future.as_mut().poll(&mut cx).is_pending(),
        "drop control must suspend",
    )?;
    drop(future);

    crate::require(
        b.drops.get() == 2,
        "dropping pending local composition must drop backend operation",
    )
}

pub(super) fn run() -> Outcome {
    ordered()?;
    cancelled()
}

use crate::BrowserCause;
use js_sys::{Array, Function, Object, Promise, Reflect, Uint8Array};
use solana_kit_wallet::WalletErrorCode;
use std::{error::Error, fmt, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// A local wallet failure with an inspectable original cause and redacted text.
#[derive(Clone)]
pub struct WalletFailure {
    code: WalletErrorCode,
    context: &'static str,
    cause: Option<Rc<dyn Error>>,
}

impl WalletFailure {
    /// Stable portable failure category.
    pub fn code(&self) -> WalletErrorCode {
        self.code
    }

    pub(super) fn new(code: WalletErrorCode, context: &'static str) -> Self {
        Self {
            code,
            context,
            cause: None,
        }
    }

    pub(super) fn caused<E: Error + 'static>(context: &'static str, cause: E) -> Self {
        Self {
            code: WalletErrorCode::InvalidResponse,
            context,
            cause: Some(Rc::new(cause)),
        }
    }

    pub(super) fn js(value: JsValue) -> Self {
        Self {
            code: WalletErrorCode::Transport,
            context: "wallet JavaScript operation failed",
            cause: Some(Rc::new(BrowserCause::new(value))),
        }
    }

    pub(super) fn invalid() -> Self {
        Self::new(WalletErrorCode::InvalidResponse, "malformed wallet value")
    }

    pub(super) fn disconnected() -> Self {
        Self::new(
            WalletErrorCode::Disconnected,
            "wallet session is no longer current",
        )
    }

    pub(super) fn exception(&self) -> JsValue {
        if let Some(original) = self
            .source()
            .and_then(|cause| cause.downcast_ref::<BrowserCause>())
        {
            return original.value().clone();
        }

        js_sys::Error::new(self.context).into()
    }
}

impl fmt::Debug for WalletFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WalletFailure")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for WalletFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.context)
    }
}

impl Error for WalletFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_deref()
    }
}

pub(super) fn get(value: &JsValue, key: &str) -> Result<JsValue, WalletFailure> {
    Reflect::get(value, &JsValue::from_str(key)).map_err(WalletFailure::js)
}

pub(super) fn has(value: &JsValue, key: &str) -> Result<bool, WalletFailure> {
    Reflect::has(value, &JsValue::from_str(key)).map_err(WalletFailure::js)
}

pub(super) fn object(value: JsValue) -> Result<JsValue, WalletFailure> {
    if !value.is_object() || value.is_null() || Array::is_array(&value) {
        return Err(WalletFailure::invalid());
    }

    Ok(value)
}

pub(super) fn string(value: &JsValue, key: &str) -> Result<String, WalletFailure> {
    get(value, key)?
        .as_string()
        .ok_or_else(WalletFailure::invalid)
}

pub(super) fn optional_string(value: &JsValue, key: &str) -> Result<Option<String>, WalletFailure> {
    let value = get(value, key)?;

    if value.is_undefined() || value.is_null() {
        return Ok(None);
    }

    value
        .as_string()
        .map(Some)
        .ok_or_else(WalletFailure::invalid)
}

pub(super) fn array(value: JsValue) -> Result<Array, WalletFailure> {
    value
        .dyn_into::<Array>()
        .map_err(|_| WalletFailure::invalid())
}

pub(super) fn strings(value: &JsValue, key: &str) -> Result<Vec<String>, WalletFailure> {
    array(get(value, key)?)?
        .iter()
        .map(|value| value.as_string().ok_or_else(WalletFailure::invalid))
        .collect()
}

pub(super) fn bytes(value: &JsValue, key: &str) -> Result<Vec<u8>, WalletFailure> {
    get(value, key)?
        .dyn_into::<Uint8Array>()
        .map(|bytes| bytes.to_vec())
        .map_err(|_| WalletFailure::invalid())
}

pub(super) fn function(value: JsValue) -> Result<Function, WalletFailure> {
    value
        .dyn_into::<Function>()
        .map_err(|_| WalletFailure::invalid())
}

pub(super) fn set(value: &Object, key: &str, field: &JsValue) -> Result<(), WalletFailure> {
    if !Reflect::set(value, &JsValue::from_str(key), field).map_err(WalletFailure::js)? {
        return Err(WalletFailure::invalid());
    }

    Ok(())
}

pub(super) fn call(target: &JsValue, method: &str, args: &Array) -> Result<JsValue, WalletFailure> {
    function(get(target, method)?)?
        .apply(target, args)
        .map_err(WalletFailure::js)
}

pub(super) async fn promise(
    target: &JsValue,
    method: &str,
    args: &Array,
) -> Result<JsValue, WalletFailure> {
    let promise = call(target, method, args)?
        .dyn_into::<Promise>()
        .map_err(|_| WalletFailure::invalid())?;
    JsFuture::from(promise).await.map_err(WalletFailure::js)
}

pub(super) fn one(value: JsValue) -> Result<JsValue, WalletFailure> {
    let outputs = array(value)?;

    if outputs.length() != 1 {
        return Err(WalletFailure::invalid());
    }

    object(outputs.get(0))
}

impl From<solana_kit_signers::Error> for WalletFailure {
    fn from(error: solana_kit_signers::Error) -> Self {
        let code = match error {
            solana_kit_signers::Error::Rejected => WalletErrorCode::UserRejected,
            solana_kit_signers::Error::Cancelled => WalletErrorCode::Disconnected,
            _ => WalletErrorCode::InvalidResponse,
        };
        Self {
            code,
            context: "signer pipeline rejected wallet authorization",
            cause: Some(Rc::new(error)),
        }
    }
}

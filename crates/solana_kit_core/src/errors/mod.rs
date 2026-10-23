//! Stable diagnostic facts shared by existing Kit error owners.
//!
//! Codes preserve unknown signed integers. Domain classification follows the
//! pinned source, including its historical account typo and unclassified codes.
//! Diagnostics retain typed context and the original error through `source()`.
//! Display and Debug omit context and causes. Explicit message rendering is
//! bounded and should receive only public diagnostic facts.
mod catalog;
mod context;
mod domain;
mod native;

use alloc::{boxed::Box, string::String};
pub use context::{Context, ContextError, ContextValue};
use core::{error::Error, fmt};
pub use domain::ErrorDomain;

/// A stable Kit code or an unknown signed protocol integer, without truncation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorCode(i64);

/// One pinned named error and its source message template.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodeDefinition {
    /// Exact stable numeric code.
    pub code: ErrorCode,
    /// Exact source declaration name.
    pub name: &'static str,
    /// Uninterpolated source template, or empty when the source has none.
    pub template: &'static str,
}

impl ErrorCode {
    /// Preserves a known or unknown signed integer exactly.
    #[must_use]
    pub const fn from_value(value: i64) -> Self {
        Self(value)
    }
    /// Returns the exact integer supplied at the boundary.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }
    /// Looks up a defined code without treating reserved family numbers as known.
    #[must_use]
    pub fn definition(self) -> Option<&'static CodeDefinition> {
        Self::definitions().find(|definition| definition.code == self)
    }
    /// Iterates every source-defined code in source declaration order.
    pub fn definitions() -> impl Iterator<Item = &'static CodeDefinition> {
        catalog::definitions()
    }
    /// Classifies known and unknown integers by the pinned domain rules.
    #[must_use]
    pub fn domain(self) -> ErrorDomain {
        domain::classify(self.0)
    }
    /// Renders the source template with explicit public context, up to 4096 bytes.
    pub fn message(self, context: &Context) -> Result<String, ContextError> {
        context.render(self)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Existing error owners expose only facts they actually retain.
pub trait ErrorFacts: Error {
    /// Exact source-equivalent code, or absent when the native reason is broader.
    fn error_code(&self) -> Option<ErrorCode>;
    /// Public structured facts, without serializing the original input or cause.
    fn error_context(&self) -> Context {
        Context::default()
    }
}

/// A cold owned diagnostic that retains its original error without cloning it.
pub struct Diagnostic {
    code: ErrorCode,
    context: Context,
    source: Option<Box<dyn Error + Send + Sync>>,
    source_is_cause: bool,
}

impl Diagnostic {
    /// Creates a diagnostic with already validated public context.
    #[must_use]
    pub fn new(code: ErrorCode, context: Context) -> Self {
        Self {
            code,
            context,
            source: None,
            source_is_cause: false,
        }
    }
    /// Wraps the original cause, preserving its concrete type and source chain.
    #[must_use]
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self.source_is_cause = true;
        self
    }
    /// Retains the original boundary error as provenance rather than a simulation cause.
    #[must_use]
    pub fn with_origin(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self.source_is_cause = false;
        self
    }
    /// Normalizes null facts and retains a typed nested diagnostic as the cause.
    /// Cause code/context are available through `source`, without unsafe strings.
    #[must_use]
    pub fn wrap(code: ErrorCode, context: Context, cause: Self) -> Self {
        Self::from_context(code, context).with_source(cause)
    }
    /// Creates a diagnostic using helper normalization: null top-level facts disappear.
    #[must_use]
    pub fn from_context(code: ErrorCode, context: Context) -> Self {
        Self::new(code, context.normalized())
    }
    /// Stable code, including unknown values.
    #[must_use]
    pub const fn code(&self) -> ErrorCode {
        self.code
    }
    /// Validated public context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        &self.context
    }
    /// Unwraps only the source-defined simulation wrappers, one level at a time.
    #[must_use]
    pub fn simulation_cause(&self) -> &(dyn Error + 'static) {
        if self.source_is_cause
            && matches!(self.code.value(), -32002 | 5663019 | 5663037)
            && let Some(source) = self.source()
        {
            return source;
        }

        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "SolanaError#{}", self.code)
    }
}
impl fmt::Debug for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Diagnostic")
            .field("code", &self.code)
            .finish_non_exhaustive()
    }
}
impl Error for Diagnostic {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}
impl ErrorFacts for Diagnostic {
    fn error_code(&self) -> Option<ErrorCode> {
        Some(self.code)
    }
    fn error_context(&self) -> Context {
        self.context.clone()
    }
}

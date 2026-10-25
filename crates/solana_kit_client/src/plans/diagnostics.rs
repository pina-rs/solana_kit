//! Typed failure diagnostics keep original causes and preflight payloads available.
use solana_kit_core::{
    Signature,
    plans::{Status, TransactionPlanResult},
};
use std::fmt;

const MAX_LOG_LINES: usize = 8;

/// Structured error enrichment, replacing untyped context-map unwrapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailureDetails<Failure, Preflight = ()> {
    /// Original error, unwrapped by the adapter at its boundary.
    pub cause: Failure,
    /// Simulation/preflight logs, preserving absence and an empty list.
    pub logs: Option<Vec<String>>,
    /// Complete original preflight data when this failed before submission.
    pub preflight: Option<Preflight>,
    /// Explicit submitted signature, if one was obtained.
    pub signature: Option<Signature>,
}
impl<Failure: fmt::Display, Preflight> fmt::Display for FailureDetails<Failure, Preflight> {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cause.fmt(output)
    }
}
impl<Failure: core::error::Error + 'static, Preflight: fmt::Debug> core::error::Error
    for FailureDetails<Failure, Preflight>
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// Consumer-facing error wording for signing or submission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureAction {
    /// Signatures were requested without submission.
    Sign,
    /// Transactions were submitted or failed preflight.
    Send,
}

/// A borrowed report retaining the complete result, indexed failures, logs and abort reason.
pub struct FailureReport<'a, Progress, Failure, Preflight> {
    /// Complete original result tree.
    pub result: &'a TransactionPlanResult<Progress, FailureDetails<Failure, Preflight>>,
    /// Whether this report describes signing or submission.
    pub action: FailureAction,
    /// Optional explicit caller cancellation reason.
    pub abort_reason: Option<&'a str>,
}
impl<Progress, Failure, Preflight> FailureReport<'_, Progress, Failure, Preflight> {
    /// Indexed original failures, using zero-based depth-first transaction positions.
    #[must_use]
    pub fn failures(&self) -> Vec<(usize, &FailureDetails<Failure, Preflight>)> {
        self.result
            .flatten()
            .into_iter()
            .enumerate()
            .filter_map(|(index, leaf)| match &leaf.status {
                Status::Failed(error) => Some((index, error)),
                _ => None,
            })
            .collect()
    }
}
impl<Progress, Failure: fmt::Display, Preflight> fmt::Display
    for FailureReport<'_, Progress, Failure, Preflight>
{
    /// Keeps single failures concise while indexing failures in a transaction batch.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        let multiple = self.result.flatten().len() != 1;

        let action = match self.action {
            FailureAction::Sign => "sign",
            FailureAction::Send => "send",
        };
        let suffix = if multiple {
            "transactions"
        } else {
            "transaction"
        };

        write!(output, "Failed to {action} {suffix}")?;
        let failures = self.failures();

        if failures.is_empty() {
            return write_cancel(output, self.abort_reason);
        }

        if !multiple {
            return write_single(output, failures[0].1, self.action);
        }

        output.write_str(".")?;

        for (index, failure) in &failures {
            write!(output, "\n[Tx #{}", index + 1)?;
            write_indicator(output, failure, self.action)?;
            write!(output, "] {}", failure.cause)?;
        }

        if failures.len() == 1 && has_logs(failures[0].1) {
            return write_logs(output, failures[0].1.logs.as_deref());
        }

        output.write_str("\n")
    }
}
fn write_cancel(output: &mut fmt::Formatter<'_>, reason: Option<&str>) -> fmt::Result {
    match reason {
        Some(reason) => write!(output, ". Canceled with abort reason: {reason}"),
        None => output.write_str(": Canceled"),
    }
}
fn write_single<Failure: fmt::Display, Preflight>(
    output: &mut fmt::Formatter<'_>,
    failure: &FailureDetails<Failure, Preflight>,
    action: FailureAction,
) -> fmt::Result {
    write_indicator(output, failure, action)?;
    write!(output, ": {}", failure.cause)?;
    write_logs(output, failure.logs.as_deref())
}
fn write_indicator<Failure, Preflight>(
    output: &mut fmt::Formatter<'_>,
    failure: &FailureDetails<Failure, Preflight>,
    action: FailureAction,
) -> fmt::Result {
    if action == FailureAction::Sign {
        return Ok(());
    }

    if failure.preflight.is_some() {
        return output.write_str(" (preflight)");
    }

    if let Some(signature) = failure.signature {
        return write!(output, " ({signature})");
    }

    Ok(())
}
fn has_logs<Failure, Preflight>(failure: &FailureDetails<Failure, Preflight>) -> bool {
    failure.logs.as_ref().is_some_and(|logs| !logs.is_empty())
}
fn write_logs(output: &mut fmt::Formatter<'_>, logs: Option<&[String]>) -> fmt::Result {
    let Some(logs) = logs.filter(|logs| !logs.is_empty()) else {
        return Ok(());
    };
    let start = logs.len().saturating_sub(MAX_LOG_LINES);

    if start > 0 {
        write!(
            output,
            "\n\nLogs (last {MAX_LOG_LINES} of {}):\n",
            logs.len()
        )?;
    } else {
        output.write_str("\n\nLogs:\n")?;
    }

    for line in &logs[start..] {
        writeln!(output, "  > {line}")?;
    }

    Ok(())
}

//! The sealed task table determines the only phase that can legitimately be ready.

use super::StartupError;
use serde_json::Value;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Planning,
    Remote,
    Runbooks,
    Ready,
}

#[derive(Default)]
struct Progress {
    remote: Option<bool>,
    runbooks: Option<bool>,
}

pub(super) fn ready(info: &Value) -> Result<bool, StartupError> {
    let startup = info
        .get("startup")
        .filter(|value| value.is_object())
        .ok_or(StartupError::Malformed)?;
    let phase = phase(text(startup, "phase")?)?;
    no_error(startup)?;
    let sealed = startup
        .get("planSealed")
        .and_then(Value::as_bool)
        .ok_or(StartupError::Malformed)?;
    let tasks = tasks(startup)?;

    let expected = match (sealed, tasks.remote, tasks.runbooks) {
        (false, None, None) => Phase::Planning,
        (false, _, _) => return Err(StartupError::Inconsistent),
        (true, Some(false), _) => Phase::Remote,
        (true, _, Some(false)) => Phase::Runbooks,
        (true, _, _) => Phase::Ready,
    };

    if phase != expected {
        return Err(StartupError::Inconsistent);
    }

    Ok(phase == Phase::Ready)
}

fn phase(value: &str) -> Result<Phase, StartupError> {
    match value {
        "planning" => Ok(Phase::Planning),
        "cloningRemoteAccounts" => Ok(Phase::Remote),
        "executingRunbooks" => Ok(Phase::Runbooks),
        "ready" => Ok(Phase::Ready),
        "failed" => Err(StartupError::Failed),
        _ => Err(StartupError::Malformed),
    }
}

fn text<'a>(value: &'a Value, field: &str) -> Result<&'a str, StartupError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or(StartupError::Malformed)
}

fn no_error(value: &Value) -> Result<(), StartupError> {
    match value.get("error") {
        Some(Value::Null) => Ok(()),
        Some(_) => Err(StartupError::Failed),
        None => Err(StartupError::Malformed),
    }
}

fn tasks(startup: &Value) -> Result<Progress, StartupError> {
    let tasks = startup
        .get("tasks")
        .and_then(Value::as_array)
        .ok_or(StartupError::Malformed)?;
    let mut progress = Progress::default();

    for task in tasks {
        no_error(task)?;

        let completed = match text(task, "state")? {
            "succeeded" => true,
            "pending" | "running" => false,
            "failed" => return Err(StartupError::Failed),
            _ => return Err(StartupError::Malformed),
        };

        progress.record(text(task, "task")?, completed)?;
    }

    Ok(progress)
}

impl Progress {
    fn record(&mut self, kind: &str, completed: bool) -> Result<(), StartupError> {
        let slot = match kind {
            "remoteAccounts" => &mut self.remote,
            "runbookExecutions" => &mut self.runbooks,
            _ => return Err(StartupError::Malformed),
        };

        if slot.replace(completed).is_some() {
            return Err(StartupError::Malformed);
        }

        Ok(())
    }
}

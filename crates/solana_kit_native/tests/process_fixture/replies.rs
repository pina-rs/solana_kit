//! Scripted stock-shaped startup projections and deterministic shutdown behavior.

use serde_json::{Value, json};

const OTHER_GENESIS: &str = "11111111111111111111111111111111";
const RPC_ERROR_CODE: i64 = -32000;

pub struct State {
    mode: String,
    genesis: String,
    genesis_calls: usize,
    info_calls: usize,
}

impl State {
    pub fn new(mode: String, genesis: String) -> Self {
        Self {
            mode,
            genesis,
            genesis_calls: 0,
            info_calls: 0,
        }
    }

    pub fn reply(&mut self, method: &str) -> (Value, bool) {
        match method {
            "getGenesisHash" => (self.genesis(), false),
            "surfnet_getSurfnetInfo" => {
                self.info_calls += 1;

                (self.startup(), false)
            }
            "exit" => (
                Value::Null,
                !matches!(self.mode.as_str(), "ignore-exit" | "exit-error"),
            ),
            _ => panic!("unexpected fixture RPC method"),
        }
    }

    fn genesis(&mut self) -> Value {
        self.genesis_calls += 1;
        let wrong = self.mode == "wrong-genesis"
            || (self.mode == "switch-genesis" && self.genesis_calls > 1);

        if wrong {
            return OTHER_GENESIS.into();
        }

        self.genesis.clone().into()
    }

    pub fn stalled(&self, method: &str) -> bool {
        self.mode == "stall"
            || (self.mode == "stall-close" && self.genesis_calls > 1)
            || (self.mode == "stall-exit" && method == "exit")
    }

    pub fn envelope(&self, request: &Value, result: Value) -> Value {
        let id = &request["id"];

        if self.mode == "rpc-error" || (self.mode == "exit-error" && request["method"] == "exit") {
            return json!({"jsonrpc":"2.0", "id":id, "error":{
                "code":RPC_ERROR_CODE, "message":"private-rpc-sentinel"
            }});
        }

        json!({"jsonrpc":"2.0", "id":id, "result":result})
    }

    fn startup(&self) -> Value {
        let phase = match self.mode.as_str() {
            "planning" => "planning",
            "failed" => "failed",
            "unknown" => "futurePhase",
            "progress" => self.progress_phase(),
            _ => "ready",
        };

        let mut startup =
            json!({"phase":phase,"planSealed":phase != "planning","tasks":[],"error":null});
        self.adjust(&mut startup);

        json!({"startup":startup})
    }

    fn progress_phase(&self) -> &'static str {
        match self.info_calls {
            1 => "planning",
            2 => "cloningRemoteAccounts",
            3 => "executingRunbooks",
            _ => "ready",
        }
    }

    fn adjust(&self, startup: &mut Value) {
        match self.mode.as_str() {
            "malformed" => drop(startup.as_object_mut().unwrap().remove("planSealed")),
            "unsealed" => startup["planSealed"] = false.into(),
            "incomplete" => startup["tasks"] = tasks("remoteAccounts", "running"),
            "task-failed" => startup["tasks"] = tasks("remoteAccounts", "failed"),
            "task-error" => {
                startup["tasks"] = json!([{
                    "task":"remoteAccounts", "state":"succeeded", "error":"private-startup-sentinel"
                }])
            }
            "progress" => startup["tasks"] = self.progress_tasks(),
            _ => {}
        }
    }

    fn progress_tasks(&self) -> Value {
        match self.info_calls {
            2 => tasks("remoteAccounts", "running"),
            3 => tasks("runbookExecutions", "running"),
            _ => json!([]),
        }
    }
}

fn tasks(kind: &str, state: &str) -> Value {
    json!([{"task":kind,"state":state,"error":null}])
}

//! Deterministic public Session/Reactor composition in a real Preview2 runtime.
#[cfg(all(target_os = "wasi", target_env = "p2"))]
#[path = "../reactor_burst.rs"]
mod reactor_burst;

#[cfg(all(target_os = "wasi", target_env = "p2"))]
fn main() {
    reactor_burst::run();
}

#[cfg(not(all(target_os = "wasi", target_env = "p2")))]
fn main() {
    panic!("this verification binary requires wasm32-wasip2");
}

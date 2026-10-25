//! Source-pinned invocation projection over the independently generated data codecs.
#[path = "../client_projection/mod.rs"]
mod client_projection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    client_projection::run()
}

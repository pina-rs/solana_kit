//! Independent stdio-only component isolates compiler and libstd CLI resources.

fn main() {
    println!("PASS independent CLI stdout");
    eprintln!("PASS independent CLI stderr");
}

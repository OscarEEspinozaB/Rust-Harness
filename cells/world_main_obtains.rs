// rule: WORLD kind: control
#![allow(dead_code)]
use std::io::Write;
fn greet(name: &str, out: &mut dyn Write) -> std::io::Result<()> { writeln!(out, "hello {name}") }
#[allow(clippy::disallowed_methods)]
fn main() {
    let name = std::env::var("USER").unwrap_or_default();
    let mut stdout = std::io::stdout();
    let _ = greet(&name, &mut stdout);
}

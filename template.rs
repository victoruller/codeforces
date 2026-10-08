/*
cp template.rs src/bin/cf2275b.rs
cargo run --release --bin xxxx < inputs/xxxx.txt
 */

use std::io::{self, Read, Write};

fn main() {
    // Lê toda a entrada de uma vez
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut next = || it.next().unwrap();

    // Saída bufferizada (bem mais rápida que println! em loop)
    let mut out = io::BufWriter::new(io::stdout().lock());
}
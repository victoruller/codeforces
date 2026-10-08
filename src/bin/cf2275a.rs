/*
cp template.rs src/bin/cf2275a.rs
cargo run --release --bin cf2275a.rs < inputs/cf2275a.txt
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

    let n: u16 = next().parse().unwrap();
    for i in 0..n {
        let x = next();
        let y: i8 = next().parse().unwrap();
        let r: i8 = next().parse().unwrap();

        writeln!(out, "{} {}", x, y + r).unwrap();
    }
}
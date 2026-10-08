/*
cp template.rs src/bin/cf1915a.rs
cargo run --release --bin cf1915a < inputs/cf1915a.txt
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
    for _ in 0..n {
        let a: u8 = next().parse().unwrap();
        let b: u8 = next().parse().unwrap();
        let c: u8 = next().parse().unwrap();
        
        writeln!(out, "{}", a ^ b ^ c).unwrap();
    }
}
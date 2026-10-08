/*
cp template.rs src/bin/cf2275b.rs
cargo run --release --bin cf2275b < inputs/cf2275b.txt
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


    let n: usize = next().parse().unwrap();
    for _ in 0..n {
        let n: usize = next().parse().unwrap();
        let s = next();
        let mut stack = Vec::<usize>::new();
        let mut lost = Vec::<usize>::new();

        for (index, c) in s.chars().enumerate() {
            match c {
                '1' => {
                    stack.push(index + 1);
                },
                '2' => {
                    if !stack.is_empty() {
                        stack.pop();
                        lost.push(index + 1);
                    }
                },
                '3' => (),
                _ => unreachable!()
            }
        }
        stack.extend(lost);
        stack.extend(s.len()..n);
        stack.sort_unstable();
        let lost_files = stack.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(" ");
        let n_lost = stack.len();
        writeln!(out, "{}\n{}", n_lost, lost_files).unwrap();
    }
}
/*
cp template.rs src/bin/cf2275c.rs
cargo run --release --bin cf2275c < inputs/cf2275c.txt
 */

use std::io::{self, Read, Write};
use std::collections::HashMap;

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
        let mut loves_map = HashMap::<i32, Vec<usize>>::new();
        let mut values = Vec::<i32>::new();
        let mut duos: u64 = 0;

        let n: usize = next().parse().unwrap();
        for i in 0..n {

            let a4: i32 = next().parse().unwrap();
            values.push(a4);

            if i >= 4 {
                let x = i - 4;
                let love = values[x] + values[x + 2] - a4;

                match loves_map.get_mut(&love) {
                    Some(ys) => {
                        let n = ys.len();
                        duos += n as u64;

                        for j in (0..n).rev() {
                            let y = ys[j];

                            if y + 4 < x { break; }
                            if y + 2 == x || y + 4 == x { duos -= 1; }
                        }
                        ys.push(x);

                    },
                    None => {
                        loves_map.insert(love, vec![x]);
                    },
                }
            }
        }
        writeln!(out, "{}", duos).unwrap();
    }
}
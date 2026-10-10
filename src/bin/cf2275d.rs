/*
cp template.rs src/bin/cf2275d.rs
cargo run --release --bin cf2275d < inputs/cf2275d.txt
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

    let t: usize = next().parse().unwrap();
    for _ in 0..t {
        let n: usize = next().parse().unwrap();
        let k: usize = next().parse().unwrap();

        let mut labs = Vec::<(i64, i64, bool)>::with_capacity(n);// (S, d, dead)
        let mut s_min = i64::MAX;
        let mut dead_min = i64::MAX;
        for _ in 0..n {
            let a: i64 = next().parse().unwrap();
            let b: i64 = next().parse().unwrap();
            let c: i64 = next().parse().unwrap();
            let d = if a <= b && b <= c { (b - a).min(c - b) + 1 } else { 0 };
            let lab = ((a + b + c), d, a == b && b == c);
            labs.push(lab);

            if lab.2 {
                dead_min = dead_min.min(lab.0);
            }
            s_min = s_min.min(lab.0);
        }
        let mut s_max = dead_min.min(s_min + k as i64) + 1;

        let is_ok = |m: i64| -> bool {
            let mut needed: i64 = 0;
            for i in 0..n {
                if labs[i].0 >= m { continue; }
                needed += 2 * labs[i].1 + (m - labs[i].0);

                if needed > k as i64 { return false; }
            }
            true
        };

        while s_max > s_min + 1 {
            let mid = (s_max + s_min) / 2;
            if is_ok(mid) {
                s_min = mid;
            } else {
                s_max = mid;
            }
        }
        writeln!(out, "{}", s_min).unwrap();
    }
}

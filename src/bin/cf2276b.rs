/*
cp template.rs src/bin/cf2276b.rs; New-Item -Name "inputs/cf2276b.txt"
cargo run --release --bin cf2276b < inputs/cf2276b.txt
*/

use std::io::{self, Read, Write};

#[allow(unused_macros)]
fn main() {
    // Lê toda a entrada de uma vez
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut next = || it.next().unwrap();

    // Saída bufferizada (bem mais rápida que println! em loop)
    let mut out = io::BufWriter::new(io::stdout().lock());

    macro_rules! rin {
        () => { next().parse().unwrap() };
        ($t:ty) => { next().parse::<$t>().unwrap() };
        ($($v:ident : $t:ty),* $(,)?) => {
            $( let $v: $t = next().parse().unwrap(); )*
        };
        ($t:ty; $n:expr) => {
            (0..$n).map(|_| next().parse::<$t>().unwrap()).collect::<Vec<$t>>()
        };
    }
    macro_rules! rout {
        ($x:expr) => { let _ = writeln!(out, "{}", $x); };
        ($($arg:tt)*) => { let _ = writeln!(out, $($arg)*).unwrap(); };
    }

    let t: usize = rin!();

    for _ in 0..t {
        let s: String = rin!();

        let bytes = s.as_bytes();
        
        let mut cur = !bytes[0];
        let mut seq = 0;
        let mut ar = (0, 0);
        for i in 0..bytes.len() {
            if bytes[i] == cur {
                seq += 1;
                if seq > 2 && i > 1 {
                    ar.0 = 1;
                    if ar.1 == 1 { break; }
                }
                continue;
            } else {
                if seq == 1 && i > 1 {
                    ar.1 = 1;
                    if ar.0 == 1 { break; }
                }
                seq = 1;
                cur = bytes[i];
                continue;
            }
        }
        
        rout!("{}", 1 + ar.0 + ar.1);
    }
}
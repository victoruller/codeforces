/*
cp template.rs src/bin/cf2275e.rs; New-Item -Name "inputs/cf2275f.txt"
cargo run --release --bin xxxx < inputs/xxxx.txt
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


    let t: usize = next().parse().unwrap();

    for _ in 0..t {

    }
}
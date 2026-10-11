/*
cp template.rs src/bin/cf2275e.rs; echo > inputs/cf2275e.txt
cargo run --release --bin cf2275e < inputs/cf2275e.txt
*/

use std::io::{self, Read, Write};

fn dist(a: u32, b: u32) -> u32 {
    if a == b { 2 } else { 1 }
}

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
        let n: usize = rin!();

        let va = rin!(u32; n);
        let vb = rin!(u32; n);

        let mut cur = dist(*va.last().unwrap(), *vb.last().unwrap());

        /* trilha o caminho alternado */
        for i in 0..(n - 1) {
            cur += dist(va[i], vb[i + 1]);
            cur += dist(va[i + 1], vb[i]);
        }

        let mut ans = cur;
        /*
            cur passa a mostrar como teria sido o peso trilhando o caminho direto até i
            ans só atualiza nos momentos em que ele está melhor do que ter trilhado o caminho alternado até i
         */
        for i in 0..(n - 1) {
            cur -= dist(va[i], vb[i + 1]);
            cur += dist(va[i], vb[i]);
            ans = ans.max(cur);
        }

        rout!(ans);
    }
}
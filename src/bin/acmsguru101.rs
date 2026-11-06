/*
cp template.rs src/bin/acmsguru101.rs; New-Item -Name "inputs/acmsguru101.txt"
cargo run --release --bin acmsguru101 < inputs/acmsguru101.txt
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


    let n: usize = rin!();
    let mut pcs = Vec::with_capacity(n);
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new();7];
    let mut deg = [0usize; 7];

    for i in 0..n {
        rin!(a: usize, b: usize);
        pcs.push((i, a, b));
        adj[a].push((b, i));
        adj[b].push((a, i));
        deg[a] += 1;
        deg[b] += 1;
    }

    let odd: Vec<usize> = (0..7).filter(|&v| deg[v] % 2 == 1).collect();
    if odd.len() > 2 {
        rout!("No solution");
        return;
    }
    let start = if odd.is_empty() {
        (0..7).find(|&v| deg[v] > 0).unwrap()
    } else {
        odd[0]
    };

    let mut used = vec![false; n];
    let mut ptr = [0usize; 7];
    let mut stack: Vec<(usize, Option<(usize, char)>)> = vec![(start, None)];
    let mut path = Vec::with_capacity(n);

    while let Some(&(v, e)) = stack.last() {
        while ptr[v] < adj[v].len() && used[adj[v][ptr[v]].1] {
            ptr[v] += 1;
        }
        if ptr[v] < adj[v].len() {
            let (to, id) = adj[v][ptr[v]];
            used[id] = true;
            let dir = if pcs[id].0 == v { '+' } else { '-' };
            stack.push((to, Some((id, dir))));
        } else {
            stack.pop();
            if let Some(x) = e {
                path.push(x);
            }
        }
    }

    if path.len() != n {
        rout!("No solution");
        return;
    }

    path.reverse();
    for (id, dir) in path {
        rout!("{} {}", id + 1, dir);
    }
}
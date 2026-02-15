use std::io;
use io::Write;

fn read() -> Vec<usize> {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let nm = read();
    let mut v = Vec::new();

    for i in 1..=nm[0] {
        v.push(i);
    }

    for _ in 0..nm[1] {
        let p = read();
        let (i, j) = (p[0], p[1]);
        v[(i-1)..j].reverse();
    }

    for x in v {
        write!(out, "{} ", x).unwrap();
    }
}
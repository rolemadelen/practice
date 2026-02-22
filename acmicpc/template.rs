use std::io::{self, Write};

fn reads() -> i32 {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn read() -> Vec<i32> {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    writeln!(out, "{}", 1234).unwrap();
}
use std::io;
use io::Write;

fn read_int() -> i32 {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn read_str() -> String {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let t = read_int();
    for _ in 0..t {
        let s = read_str();

        let first = s.chars().next().unwrap();
        let last = s.chars().last().unwrap();

        writeln!(out, "{}{}", first, last).unwrap();
    }
}
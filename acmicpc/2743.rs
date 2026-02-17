use std::io;
use io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    writeln!(out, "{}", s.len() - 1).unwrap();
}
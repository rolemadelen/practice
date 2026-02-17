use std::io;
use io::Write;

fn reads() -> String {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let s = reads();
    let s = s.trim();
    let idx = reads();
    let idx = idx.trim().parse::<usize>().unwrap();
    let ch = s.chars().nth(idx-1).unwrap();
    writeln!(out, "{}", ch).unwrap();

}
use std::io;
use io::Write;

fn reads() -> usize {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut v = vec![false; 30];

    for _ in 0..28 {
        let idx = reads();
        v[idx-1] = true;
    }

    for i in 0..30 {
        if v[i] == false {
            writeln!(out, "{}", i+1).unwrap();
        }
    }
}
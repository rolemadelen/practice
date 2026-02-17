use std::io;
use io::Write;

fn reads() -> usize {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn read() -> Vec<i32> {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = reads();
    let mut v = read();
    v.sort();
    let m = *v.last().unwrap();
    let mut score: f64 = 0.0;

    for x in v {
        score += (x as f64 / m as f64) * 100.0
    }
    score /= n as f64;

    writeln!(out, "{}", score).unwrap();
}
use std::io;
use io::Write;

fn reads() -> i32 {
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

    let _ = reads();
    let mut v = read();
    let z = reads();

    v.retain(|&x| x == z);
    writeln!(out, "{}", v.len()).unwrap();
}
use std::io;
use io::Write;

fn read_ints() -> Vec<i32> {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    // writeln!(out, "{}", 1234).unwrap();

    let x = read_ints();
    let x = x[1];

    let mut v = read_ints();
    v.retain(|&n| n < x);

    for x in v {
        write!(out, "{} ", x).unwrap();
    }
}
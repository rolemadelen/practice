use std::io;
use io::Write;

fn read_int() -> i32 {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut max: i32 = 0;
    let mut idx: usize = 0;

    for i in 0..9 {
        let x = read_int();

        if x > max {
            max = x;
            idx = i+1;
        }
    }

    writeln!(out, "{}\n{}", max, idx).unwrap();
}
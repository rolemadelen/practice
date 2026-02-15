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
    let mut v = vec![false; 42];

    for _ in 0..10 {
        let x = reads();
        v[x%42] = true;
    }

    let mut cnt = 0;
    for i in 0..42 {
        if v[i] == true {
            cnt += 1
        }
    }
    writeln!(out, "{}", cnt).unwrap();
}
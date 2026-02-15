use std::io;
use io::Write;


fn read() -> Vec<usize> {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let nm = read();
    let mut v = vec![0; nm[0]];

    for _ in 0..nm[1] {
        let nums = read();
        let (i, j, k) = (nums[0], nums[1], nums[2]);

        for idx in (i-1)..j {
            v[idx] = k;
        }
    }

    for x in v {
        write!(out, "{} ", x).unwrap();
    }
}
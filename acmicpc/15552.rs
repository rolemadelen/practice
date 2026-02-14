use std::io;
use io::Write;

fn read_int() -> i32 {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn read_ints() -> Vec<i32> {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}
fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    
    let t = read_int();
    for _ in 0..t {
        let v = read_ints();
        writeln!(out, "{}", v[0]+v[1]).unwrap();
    }

}
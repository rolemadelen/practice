use std::collections::HashMap;
use std::io::{self, Write};

fn reads() -> i32 {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn read() -> HashMap<i32, i32> {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let v: Vec<i32> = s.trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut map: HashMap<i32, i32> = HashMap::new();
    for x in &v {
        let cnt = map.entry(*x).or_insert(0);
        *cnt += 1;
    }
    
    map
}
fn read_ints() -> Vec<i32> {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    reads();
    let mut map: HashMap<i32, i32> = read();
    reads();
    
    for x in read_ints() {
        match map.get(&x) {
            Some(v) => write!(out, "{v} ").unwrap(),
            None => write!(out, "0 ").unwrap(),
        }
    }
}
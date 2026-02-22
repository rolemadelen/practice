#![allow(unused)]

use std::collections::HashSet;
use std::io::{self, Write};

fn read_tuple() -> (i32, i32) {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let mut s = s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());

    (s.next().unwrap(), s.next().unwrap())
}

fn read_str() -> String {
    let stdin = io::stdin();
    stdin.lock();
    
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let (n, m) = read_tuple();
    let mut set: HashSet<String> = HashSet::new();

    for _ in 0..n {
        set.insert(read_str());
    }
    
    let mut cnt = 0;
    for _ in 0..m {
        let s = read_str();
        if set.contains(&s) {
            cnt += 1;
        }
    }

    writeln!(out, "{cnt}").unwrap();
}
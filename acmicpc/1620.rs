#![allow(unused)]

use std::collections::HashMap;
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

fn read_vec() -> Vec<i32> {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let (n, m) = read_tuple();
    let mut map: HashMap<String, String> = HashMap::new();

    for i in 0..n {
        let idx = (i+1).to_string();
        let s = read_str();
        map.insert(s.clone(), idx.clone());
        map.insert(idx, s);
    }

    for _ in 0..m {
        let s = read_str();
        writeln!(out, "{}", map.get(&s).unwrap()).unwrap();
    }
}
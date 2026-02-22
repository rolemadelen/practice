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

fn read() -> String {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let (n, m) = read_tuple();
    let mut map: HashMap<String, i32> = HashMap::new();

    for _ in 0..n {
        map.insert(read(), 1);
    }

    for _ in 0..m {
        let cnt = map.entry(read()).or_insert(0);
        *cnt += 1;   
    }

    let mut res: Vec<_> = map.iter().
        filter(|&(k, v)| *v == 2).
        collect();

    res.sort();
    writeln!(out, "{}", res.len()).unwrap();
    for (name, _) in res {
        writeln!(out, "{}", name).unwrap();
    }
}
#![allow(unused)]

use std::collections::HashMap;
use std::io::{self, Write};

fn read_int() -> i32 {
    let stdin = io::stdin();
    stdin.lock();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

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
    let n = read_int();
    let mut map: HashMap<String, String> = HashMap::new();

    for _ in 0..n {
        let s = read_str();
        let mut s = s.split_whitespace();
        let name = s.next().unwrap();
        let log = s.next().unwrap();

        map.insert(name.to_string(), log.to_string());
    }

    let mut res: Vec<_> = map.iter().filter(|&(_, log)| log == "enter").collect();
    res.sort();
    for (name, _) in res.iter().rev() {
        writeln!(out, "{}", name).unwrap();
    }

}
use std::io;

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
    let t = read_int();

    for i in 1..=t {
        let v = read_ints();
        println!("Case #{i}: {}", v[0] + v[1]);
    }
}
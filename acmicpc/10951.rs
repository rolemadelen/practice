use std::io;

fn main() {
    loop {
        let mut s = String::new();
        if let Ok(b) = io::stdin().read_line(&mut s) {
                let v: Vec<i32> = s.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
                if b == 0 {
                    break;
                }
                println!("{}", v[0]+v[1])
        }
    }
}
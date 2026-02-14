use std::io;

fn read_int() -> i32 {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let row = read_int();

    for i in 0..row {
        for _ in 1..(row-i) {
            print!(" ");
        }

        for _ in 0..i+1 {
            print!("*");
        }

        println!("");
    }
}
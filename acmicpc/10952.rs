use std::io;

fn read_ints() -> Vec<i32> {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    loop {
        let nums = read_ints();
        match (nums[0], nums[1]) {
            (0, 0) => break,
            (a, b) => println!("{}", a + b),
        }
    }
}
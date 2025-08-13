use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    let mut input = String::new();
    println!("Enter n:");
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    println!("Fibonacci({}) = {}", n, fibonacci(n));
}

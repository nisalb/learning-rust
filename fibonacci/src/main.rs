use std::io;

fn main() {
    println!("Enter 'n' to find fib(n): ");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Unable to read input");

    let n: u32 = n.trim().parse().expect("Not a number");

    let nth_fib = fib(n);

    println!("fib({n}) = {nth_fib}");
}

fn fib(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..=n {
        (a, b) = (b, (a + b));
    }

    a
}

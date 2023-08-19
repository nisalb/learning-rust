fn main() {
    let number = 7;

    if number > 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2, or 1");
    }

    let x = if number > 5 { 21 } else { 34 };

    println!("conditional variable is {x}");
}

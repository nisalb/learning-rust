fn main() {
    println!("Hello World!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = square(9);
    println!("square returned {x}");
}

fn another_function(x: i32) {
    println!("Value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn square(x: i32) -> i32 {
    x * x
}

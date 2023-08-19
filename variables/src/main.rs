use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    println!("Input an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Unable to read line.");

    let index: usize = match index.index().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entered index is not a number. Assuming 0");
            0
        }
    };

    let element = arr[index];

    println!("Value at index is {element}");
}

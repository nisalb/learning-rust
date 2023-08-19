fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result of the loop is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("T minus {number}");

        number -= 1;
    }

    println!("LIFTOFF!!");

    let arr = [1, 2, 3, 4, 5];

    for element in arr {
        println!("Its {element}");
    }
    println!("It has ended");

    for number in (1..4).rev() {
        println!("T minus {number}");
    }
    println!("LIFTOFF!!");
}

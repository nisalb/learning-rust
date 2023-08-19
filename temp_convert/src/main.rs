use std::io;

const C: &str = "°C";
const F: &str = "°F";

fn main() {
    println!("Welcome to Temperature Convert");

    let mode = get_mode();

    let from_temp = get_initial_temperature(mode);

    let result = match mode {
        1 => ctof(from_temp),
        2 => ftoc(from_temp),
        _ => {
            panic!("Invalid conversion method: {mode}. This is a bug!")
        }
    };

    print_result(from_temp, result, mode);
}

fn get_mode() -> u8 {
    println!("Please select the conversion mode");
    println!("\t(1) {C} -> {F}");
    println!("\t(2) {F} -> {C}");

    // idiomatic C approach would be to run a while
    // loop until mode is 1 or 2. But that would require
    // a mutable mode variable. Additionally this approach
    // clearly indicate the loop and loop-halting condition
    // at the bottom
    loop {
        println!("Select 1 or 2:");

        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("Unable to read input");

        // I could have add the validation for mode values in
        // the Ok arm, but it would couple the parsing logic
        // and validaing logic into on statement
        let mode: u8 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // validating the values at the bottom, separately, is
        // more clearer.
        if mode == 1 || mode == 2 {
            return mode;
        }
    }
}

fn get_initial_temperature(mode: u8) -> f64 {
    let unit = match mode {
        1 => C,
        2 => F,
        _ => {
            panic!("Invalid covnersion mode: {mode}. This is a bug!");
        }
    };

    loop {
        println!("Enter {unit} value: ");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Unable to read the input");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return temp;
    }
}

fn ftoc(fahr: f64) -> f64 {
    (fahr - 32.0) * 5.0 / 9.0
}

fn ctof(celc: f64) -> f64 {
    9.0 * celc / 5.0 - 32.0
}

fn print_result(from: f64, to: f64, mode: u8) {
    let (from_unit, to_unit) = match mode {
        1 => (C, F),
        2 => (F, C),
        _ => {
            panic!("Unknown conversion mode: {mode}. This is a bug")
        }
    };

    println!("{from} {from_unit} is {to} {to_unit}");
}

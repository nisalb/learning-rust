use pig_latin;

fn main() {
    let input = "Mommy";
    let output = pig_latin::encode(&input);

    println!("input = '{input}'");
    println!("output = '{output}'");
}

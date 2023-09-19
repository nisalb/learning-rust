fn main() {
    let a = "abcd".to_string();
    let b = "xyz";

    let result = longest(a.as_str(), b);
    println!("Longest is {}", result);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

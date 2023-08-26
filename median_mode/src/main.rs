use median_mode::stats::basic;

fn main() {
    let nums = vec![0, 1, 2, 3, 3, 4];

    let median = basic::median(&nums);
    println!("median = {median}");

    if let Some(mode) = basic::mode(&nums) {
        println!("mode = {mode}");
    } else {
        println!("no items");
    }
}

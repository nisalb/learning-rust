use median_mode::stats;

fn main() {
    let nums = vec![0, 1, 2, 3];

    let median = stats::basic::median(&nums);
    let mode = stats::basic::mode(&nums);

    println!("median = {median}, mode = {mode}");
}

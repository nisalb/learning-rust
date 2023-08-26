use std::collections::HashMap;

pub fn median(nums: &Vec<i64>) -> f64 {
    let mut cloned = nums.clone();
    cloned.sort();

    let mid = cloned.len() / 2;

    if cloned.len() % 2 != 0 {
        cloned[mid] as f64
    } else {
        (cloned[mid] + cloned[mid - 1]) as f64 / 2.0
    }
}

pub fn mode(nums: &Vec<i64>) -> Option<i64> {
    let mut counts = HashMap::new();

    for i in nums {
        counts.entry(*i).and_modify(|n| *n += 1).or_insert(1);
    }

    counts
        .into_iter()
        .max_by_key(|entry| entry.1)
        .map(|entry| entry.0)
}

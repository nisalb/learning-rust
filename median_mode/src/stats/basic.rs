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

pub fn mode(nums: &Vec<i64>) -> i64 {
    let mut counts = HashMap::new();

    for i in nums {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;

    for (elem, count) in &counts {
        if max_count < *count {
            max_count = *count;
            mode = *elem;
        }
    }

    mode
}

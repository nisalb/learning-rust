use std::collections::HashMap;

fn main() {
    let _int_vec: Vec<i32> = Vec::new();

    let _flt_vec = vec![3.14, 2.72, 1.98];

    let mut mut_vec = Vec::new();
    mut_vec.push(10);
    mut_vec.push(20);
    mut_vec.push(30);

    let third = &mut_vec[2];
    println!("Third of mut_vec is {third}");

    let second = match mut_vec.get(1) {
        Some(val) => val,
        None => &0, // second has the type &i32.
    };

    println!("Second of the mut_vec is {second}");

    let vec = vec![String::from("one"), String::from("two")];

    for i in &vec {
        // without reference (slice) vec is moved
        use_val(i);

        //println!("After use_val {}", i);
    }

    for i in &mut mut_vec {
        *i *= 3
    }

    for i in &mut_vec {
        println!("{i}");
    }

    let nothing = String::from("Nothing");
    let _s = match vec.get(1) {
        Some(val) => val,
        None => &nothing,
    };

    let hindi_word = String::from("नमस्ते");

    println!("नमस्ते in bytes = {:?}", hindi_word.as_bytes());
    println!("नमस्ते in chars = {:?}", hindi_word.chars());
    println!("नमस्ते in grapheme clusters = [न, म, स्, ते]"); // getting graphemes are comples and not provided in std lib

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("s1 is moved, s2 is {s2}, and s3 is {s3}");

    let _s1 = String::from("Hello");
    // let h = s1[0]; // Strings cannot be indexed. Because it is ambiguous on what to return. a byte, scalar or a grapheme?

    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 13);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Blue team score = {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(24); // will not be added
    scores.entry(String::from("Yellow")).or_insert(34); // will be added

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}

fn use_val(s: &String) {
    println!("{s}");
}

const CARDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const COUNTS: [&str; 12] = [
    "A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve",
];

const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "french hens",
    "calling birds",
    "golden rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

fn main() {
    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me,",
            CARDINALS[i]
        );

        // better capture this concept in a separate function
        for j in (0..=i).rev() {
            if j == 1 {
                println!("{} {}, and", COUNTS[j], GIFTS[j]);
            } else {
                println!("{} {}", COUNTS[j], GIFTS[j]);
            }
        }

        println!();
    }
}

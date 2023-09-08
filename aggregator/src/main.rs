use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let article = NewsArticle {
        author: "Nisal Bandara".to_string(),
        headline: "Rust as a Primary Language".to_string(),
        location: "Colombo".to_string(),
        content: "TBW".to_string(),
    };

    println!("{}", article.summarize());

    let tweet = Tweet {
        username: "nisalb".to_string(),
        content: "Rust allows me to be a better programmer".to_string(),
        retweet: false,
        reply: false,
    };

    println!("{}", tweet.summarize());
}

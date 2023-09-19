use aggregator::{BlogPost, NewsArticle, Pair, Summary, Tweet};

fn main() {
    let article = NewsArticle {
        author: "Nisal Bandara".to_string(),
        headline: "Rust as a Primary Language".to_string(),
        location: "Colombo".to_string(),
        content: "TBW".to_string(),
    };

    let tweet = Tweet {
        username: "nisalb".to_string(),
        content: "Rust allows me to be a better programmer".to_string(),
        retweet: false,
        reply: false,
    };

    let blog = BlogPost {
        author: "nisalb".to_string(),
        content: "This is a blog post".to_string(),
    };

    notify(&article);
    notify(&tweet);
    notify(&blog);

    let pair = Pair::new(10, 25);
    pair.cmp_display();
}

// pub fn notify(item : &impl Summary)
// pub fn notify<T: Summary>(item: &T)
pub fn notify<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

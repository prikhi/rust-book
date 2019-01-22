use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        tweet_type: TweetType::NewTweet,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize_def());

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub tweet_type: TweetType,
}

pub enum TweetType {
    NewTweet,
    ReTweet,
    Reply,
}

pub trait Summary {
    fn summarize(&self) -> String;
    // can implement a default method as well:
    /*
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    */
}
// this uses the default implementation
/*
impl Summary for NewsArticle {}
*/

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait SummaryDef {
    fn summarize_author(&self) -> String;

    fn summarize_def(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl SummaryDef for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits in args
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// trait bounds
pub fn notify_bounds<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// multiple traits
pub fn notify_mult(_item: impl Summary + Display) {}
pub fn notify_bounds_mult<T: Summary + Display>(_item: T) {}
// where clause reduces noise
pub fn some_function<T, U>(_t: T, _u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    3
}

// returning traits - note a single distinct type must still be returned
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        tweet_type: TweetType::NewTweet,
    }
}

pub fn largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut largest = arr[0];
    for &item in arr.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_clone<T: PartialOrd + Clone>(arr: &[T]) -> T {
    let mut largest = arr[0].clone();
    for item in arr.iter() {
        let cloned = item.clone();
        if cloned > largest {
            largest = cloned
        }
    }
    largest
}

pub fn largest_ref<T: PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];
    for item in arr.iter() {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

// condition methods for trait bounds
pub struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

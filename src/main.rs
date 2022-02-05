use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin (&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub trait Summary {
    fn summarize_author (&self) -> String;
    fn summarize (&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub content: String,
    pub author: String
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("at {}", self.author)
    }       
}

pub struct Twitter {
    pub username: String,
    pub content: String
}

impl Summary for Twitter {
    fn summarize_author(&self) -> String {
        format!("tweet {}", self.username)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify (item: &impl Summary) {
    println!("Break news!{}", item.summarize())   
}

pub fn notify1<T: Summary> (item: &T) {
    println!("Break news!{}", item.summarize()) 
}

pub fn notify2<T: Summary + Display> (item: &T) {
    println!("Break news!{}", item.summarize()) 
}

struct Pair<T> {
    x: T,
    y: T
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display (&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn get_summarize () -> impl Summary {
    Twitter{username: "sunface".to_string(),content: "Twitter 好用".to_string()} 
}

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let point = Point {x: 2, y: 2};
    let float = Point {x: 1.0, y: 4.0};
    let f32: Point<f32> = Point {x: 1.0, y: 4.0};

    println!("x value is {}", point.x());
    println!("float x value is {}", float.x);
    println!("f32 distance value is {}", f32.distance_from_origin());

    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let twitter = Twitter{username: "sunface".to_string(),content: "Twitter 好用".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}\n", post.summarize());
    println!("{}\n", twitter.summarize());
    println!("{}\n", weibo.summarize());
    notify(&weibo);

    let pair = Pair{x: 10, y: 9};
    pair.cmp_display();

    let summary = get_summarize();
    println!("{}\n", summary.summarize());

    let numbers = vec![1, 4, 6, 9, 3, 2, 5, 10];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}



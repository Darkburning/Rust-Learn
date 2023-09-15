#![allow(unused)]
mod generics;
mod lifetime;
mod r#trait;

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // generics::generics_demo();
    // r#trait::trait_demo();
    lifetime::lifetime_demo();
    println!(
        "longest str is：{}",
        longest_with_an_announcement("long str", "long long str", "hello world")
    );
}

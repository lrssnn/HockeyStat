extern crate curl;

use std::io::{stdout, Write};

use curl::easy;
fn main() {
    println!("Hello, world!");

    let mut curl = easy::Easy::new();
    curl.url("https://en.wikipedia.org/wiki/Nail").unwrap();
    curl.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    });
    curl.perform().unwrap();
}

extern crate curl;
extern crate html5ever;

use std::io::{stdout, Write};
use curl::easy::{Easy2, Handler, WriteError};

use curl::easy;
fn main() {
    println!("Hello, world!");


    let mut curl = Easy2::new(Collector(Vec::new()));
    curl.get(true).unwrap();
    curl.url("https://en.wikipedia.org/wiki/Nail").unwrap();
    curl.perform().unwrap();

    let web = curl.get_ref();

    println!("{}", String::from_utf8_lossy(&web.0));
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}


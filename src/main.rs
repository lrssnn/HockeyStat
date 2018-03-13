extern crate curl;
extern crate serde_json;

use std::io::{stdout, Write, Read};
use std::iter::repeat;
use curl::easy::{Easy2, Handler, WriteError};

use serde_json::{Value, Error};

fn main() {
    println!("Hello, world!");


    let mut curl = Easy2::new(Collector(Vec::new()));
    curl.get(true).unwrap();
	curl.useragent("Chrome/41.0.2227.0").unwrap();
    curl.url("https://statsapi.web.nhl.com/api/v1/schedule?startDate=2017-12-12&endDate=2017-12-12").unwrap();
    curl.perform().unwrap();

    let web = curl.get_ref();

    let json = String::from_utf8(web.0.as_slice().to_vec()).unwrap();

	println!("{}", json);

    let v: Value = serde_json::from_str(&json).unwrap();

    println!("{}", v["copyright"]);

/*
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut web.0.as_slice())
        .unwrap();


    walk(0, dom.document);
*/
}


struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

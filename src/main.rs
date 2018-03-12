extern crate curl;
#[macro_use] extern crate html5ever;

use std::io::{stdout, Write, Read};
use std::iter::repeat;
use curl::easy::{Easy2, Handler, WriteError};

use html5ever::parse_document;
use html5ever::rcdom::{NodeData, RcDom, Handle};
use html5ever::tendril::TendrilSink;

use curl::easy;
fn main() {
    println!("Hello, world!");


    let mut curl = Easy2::new(Collector(Vec::new()));
    curl.get(true).unwrap();
    curl.url("https://en.wikipedia.org/wiki/Nail").unwrap();
    curl.perform().unwrap();

    let web = curl.get_ref();

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut web.0.as_slice())
        .unwrap();

    walk(0, dom.document);
}


struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn walk(indent: usize, handle: Handle) {
    let node = handle;

    print!("{}", repeat(" ").take(indent).collect::<String>());

    let mut desired = true;

    match node.data {
        NodeData::Document
            => println!("#Document"),

        NodeData::Doctype { ref name, ref public_id, ref system_id }
            => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        NodeData::Text { ref contents }
            => println!("#text: {}", escape_default(&contents.borrow())),

        NodeData::Element {ref name, ref attrs, ..} => {
            assert!(name.ns == ns!(html));
            if name.local == local_name!("script")  { desired = false; }
            else {
                print!("<{}", name.local);
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    print!(" {}=\"{}\"", attr.name.local, attr.value);
                }
                println!(">");
            }
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),

        NodeData::Comment{..} => ()
    }

    if !desired { return; }

    for child in node.children.borrow().iter() {
        walk(indent + 4, child.clone());
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

extern crate quick_xml;
extern crate dom;

use std::fs::File;
use std::io::prelude::*;
use quick_xml::Reader;

fn main() {
    main1();
}

#[allow(dead_code)]
fn main1() {
    let filename = "tests/test.bpmn2";//"xsd.xml/BPMN20.xsd"; //"xsd.xml/Semantic.xsd";

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let xml: &str = &contents;

    let mut reader = Reader::from_str(xml);
//    reader.trim_text(true);

    let dom = dom::dom_from_reader(&mut reader).unwrap();
    //println!("{:?}", dom);
    dom::walk_and_print(0, 0, &dom);
    //TODO: dom::walk(&dom, fn) or map?
}

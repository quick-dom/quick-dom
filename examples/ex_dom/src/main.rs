extern crate quick_xml;
extern crate dom;

use std::fs::File;
use std::io::prelude::*;
use quick_xml::Reader;
use dom::DomNode;

fn main() {
    main0();
//    main1();
}

#[allow(dead_code)]
fn main0() {
    //https://doc.servo.org/alloc/borrow/enum.Cow.html

    use std::borrow::Cow;

    fn abs_all(input: &mut Cow<[i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                // Clones into a vector if not already owned.
                input.to_mut()[i] = -v;
            }
        }
    }

// No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}", input);

// Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}", input);

// No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
    println!("{:?}", input);

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
//    println!("{:?}", dom);

    dom::walk_and_print(0, 0, &dom);
    //TODO: dom::walk(&dom, fn) or map?

    let children = match dom {
        DomNode::Dom(element) => {
            element.children
        },
        _ => Vec::new(),
    };

//    children.iter().map().filter()

    for ch in children {
        let child = ch;
        match child {
            DomNode::DomElement(dom_element) => {
                let owned = dom_element.into_owned();
                println!("owned: {:?}", owned);
            },
            ref c => {
                println!("{:?}", c);
            },
        }
    }

    //walk_dom_element_and_text?
}

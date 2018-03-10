//! A module to handle `dom` enumerator

use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::BufRead;

use error::*;
use node::*;

///// Parse a document from an Event `Reader`.
pub fn dom_from_reader<R: BufRead>(reader: &mut Reader<R>) -> Result<DomNode> {
    let mut buf = Vec::new();

    let dom: Dom = Dom::new();
    let mut stack = vec![DomNode::Dom(dom)];

    loop {
        match reader.read_event(&mut buf)? {
            Event::Empty(event) => {
                let element = DomElement::from_start(event.into_owned());
                // Since there is no Event::End after, directly append it to the current node
                stack.last_mut().unwrap().append_child(DomNode::DomElement(element));
            },
            Event::Start(event) => {
                let element = DomElement::from_start(event.into_owned());
                stack.push(DomNode::DomElement(element));
            },
            Event::End(ref _e) => {
                if stack.len() <= 1 {
                    break;
                }
                let element = stack.pop().unwrap();
                if let Some(to_element) = stack.last_mut() {
                    //TODO: check
                    to_element.append_child(element);
                }
            },
            Event::Eof => {
                break;
            },
            _ => (),
        }
        buf.clear();
    }
    Ok(stack.pop().unwrap())
}

use std::str::from_utf8;
use std::iter::repeat;
pub fn walk<'a>(indent: usize, handle: &DomNode<'a>) {
    let node = handle;
    // FIXME: don't allocate
    let indents = format!("{}", repeat(" ").take(indent).collect::<String>());
    match node {
        &DomNode::DomElement(ref element) => {
            print!("{}<{}", indents, from_utf8(element.bytes_start.name()).unwrap());
            for attr in element.bytes_start.attributes() {
                let a =attr.unwrap();
                print!(" {}", from_utf8(a.key).unwrap());
                print!("=\"{}\"", from_utf8(&a.value.into_owned()).unwrap());
            }
            print!(">");
            if element.children.len()>0 {
                println!();
                for child in element.children.iter() {
                    walk(indent + 4, &child);
                }
                print!("{}", indents);
            }
            println!("</{}>", from_utf8(element.bytes_start.name()).unwrap());
        }
        &DomNode::Dom(ref element) => {
            for child in element.children.iter() {
                walk(indent, &child);
            }
        }
    }
}
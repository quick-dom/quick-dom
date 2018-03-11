//! A module to handle `dom` enumerator

use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::BufRead;
use std::ops::Deref;

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
                let element = DomElement::from_start(event.into_owned(), true);
                // Since there is no Event::End after, directly append it to the current node
                // remember Empty to print from origin
                stack.last_mut().unwrap().append_child(DomNode::DomElement(element));
            },
            Event::Start(event) => {
                let element = DomElement::from_start(event.into_owned(), false);
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
//            //ByteDecl
//            Event::Decl(_event)|
//            //BytesText:
//            Event::Text(event)|
//            Event::Comment(event)|
//            Event::PI(event)|
//            Event::DocType(event)
            event => {
                if stack.len() < 1 {
                    break;
                }
                let element = DomNode::DomEvent(event.into_owned());
                if let Some(to_element) = stack.last_mut() {
                    //TODO: check
                    to_element.append_child(element);
                }
            },
        }
        buf.clear();
    }
    Ok(stack.pop().unwrap())
}

use std::str::from_utf8;
//use std::iter::repeat;
pub fn walk_and_print<'a>(indent: usize, indent_delta: usize, handle: &DomNode<'a>) {
    let node = handle;
//    TODO: indents on reader.trim_text(true);
//    FIXME: don't allocate
//    let indents = format!("{}", repeat(" ").take(indent).collect::<String>());
    match  *node {
        DomNode::DomElement(ref element) => {
            print!("<{}", from_utf8(element.bytes_start.deref()).unwrap());
//            print!("{}<{}", indents, from_utf8(element.bytes_start.name()).unwrap());
//            for attr in element.bytes_start.attributes() {
//                let a =attr.unwrap();
//                print!(" {}", from_utf8(a.key).unwrap());
//                print!("=\"{}\"", from_utf8(&a.value.into_owned()).unwrap());
//            }

            if element.children.len()>0 {
                print!(">");
                for child in element.children.iter() {
                    walk_and_print(indent + indent_delta, indent_delta, &child);
                }
                print!("</{}>", from_utf8(element.bytes_start.name()).unwrap());
            }else{
                if element.is_empty_event {
                    print!("/>");
                } else {
                    print!(">");
                    print!("</{}>", from_utf8(element.bytes_start.name()).unwrap());
                }
            }
        }
        DomNode::Dom(ref element) => {
            for child in element.children.iter() {
                walk_and_print(indent, indent_delta, &child);
            }
        }
        DomNode::DomEvent(Event::Decl(ref decl)) => {
            let ver = decl.version().unwrap();
            let enc = match decl.encoding(){
                Some(Ok(ref encoding)) => format!(" encoding=\"{}\"", from_utf8(encoding).unwrap()),
                _ => "".to_string(),
            };
            let sta = match decl.standalone(){
                Some(Ok(ref standalone)) => format!(" standalone=\"{}\"", from_utf8(standalone).unwrap()),
                _ => "".to_string(),
            };
            print!("<?xml version=\"{}\"{}{}?>", from_utf8(&ver).unwrap(), enc, sta)
        },
        DomNode::DomEvent(Event::DocType(ref doc_type)) => {
            print!("<!DOCTYPE{}>", from_utf8(doc_type.escaped()).unwrap());
        },
        DomNode::DomEvent(Event::Comment(ref doc_type)) => {
            print!("<!-- {} -->", from_utf8(doc_type.escaped()).unwrap());
        },
        DomNode::DomEvent(Event::CData(ref doc_type)) => {
            print!("<![CDATA[{}]]>", from_utf8(doc_type.escaped()).unwrap());
        },
        DomNode::DomEvent(Event::PI(ref doc_type)) => {
            print!("<?{}?>", from_utf8(doc_type.escaped()).unwrap());
        },
        DomNode::DomEvent(Event::Text(ref text)) => {
            print!("{}", from_utf8(text.escaped()).unwrap());
        },
//// unexpected Event:
//// Start, Empty, End - but was tested by remove them on dom_from_reader
//// and Eof
//        DomNode::DomEvent(ref event) => {
//            print!(" test: {}", from_utf8(event.deref()).unwrap());
//        },
        DomNode::DomEvent(Event::Start(ref byte_start)) => {
            print!("<{}>", from_utf8(byte_start.deref()).unwrap());
        },
        DomNode::DomEvent(Event::Empty(ref byte_start)) => {
            print!("<{}/>", from_utf8(byte_start.deref()).unwrap());
        },
        DomNode::DomEvent(Event::End(ref byte_start)) => {
            print!("</{}>", from_utf8(byte_start.deref()).unwrap());
        },
        DomNode::DomEvent(Event::Eof) => {
        },
    }
}
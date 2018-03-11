use node::DomNode;
use quick_xml::events::BytesStart;

#[derive(Clone, Debug)]
/// A struct representing a DOM Element.
pub struct DomElement<'a> {
    pub bytes_start: BytesStart<'a>,
    pub is_empty_event: bool,
    pub children: Vec<DomNode<'a>>,
}

impl<'a> DomElement<'a> {
    /// Creates a `DomElement` from a `BytesStart`
    pub fn from_start(bytes_start: BytesStart<'a>, is_empty_event:bool) -> DomElement<'a> {
        DomElement {
            bytes_start,
            is_empty_event,
            children: Vec::<DomNode>::new(),
        }
    }
    pub fn append_child(&mut self, child: DomNode<'a>) {
        self.children.push(child);
    }
}

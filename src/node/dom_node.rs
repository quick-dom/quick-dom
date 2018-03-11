use node::Dom;
use node::DomElement;
use quick_xml::events::Event;
/// A node in an element tree.
#[derive(Clone, Debug)]
pub enum DomNode<'a> {
    Dom(Dom<'a>),
    DomElement(DomElement<'a>),
    DomEvent(Event<'a>)
}
impl<'a> DomNode<'a> {
    pub fn append_child(&mut self, child: DomNode<'a>)  {
        match self {
            &mut DomNode::Dom(ref mut e) => {
                e.append_child(child);
            },
            &mut DomNode::DomElement(ref mut e) => {
                e.append_child(child);
            },
            _ => {

            },
        }
    }
    /// TODO: lifetime static
    /// Converts the dom node to an owned version, untied to the lifetime of
    /// buffer used when reading but incurring a new, seperate allocation.
    pub fn into_owned(self) -> DomNode<'a> {
        match self {
            DomNode::Dom(e) => DomNode::Dom(e.into_owned()),
            DomNode::DomElement(e) => DomNode::DomElement(e.into_owned()),
            DomNode::DomEvent(e) => DomNode::DomEvent(e.into_owned()),
        }
    }
}

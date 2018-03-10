use node::Dom;
use node::DomElement;

/// A node in an element tree.
#[derive(Clone, Debug)]
pub enum DomNode<'a> {
    /// An `Dom`.
    Dom(Dom<'a>),
    /// An `DomElement`.
    DomElement(DomElement<'a>),
//TODO:
//    /// A text node.
//    DomText(DomText),
//    /// A comment node.
//    DomComment(DomComment),
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
        }
    }
}

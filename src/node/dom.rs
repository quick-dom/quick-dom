//! A module to handle `Dom` enumerator

//https://docs.microsoft.com/en-us/dotnet/standard/data/xml/types-of-xml-nodes
//https://www.tutorialspoint.com/dom/dom_node_object.htm
//https://www.w3.org/DOM/

use node::DomNode;

#[derive(Clone, Debug)]
/// A struct representing a DOM Element.
pub struct Dom<'a> {
    pub children: Vec<DomNode<'a>>,
}
impl<'a> Dom<'a> {
    pub fn new() -> Dom<'a> {
        Dom {
            children: Vec::<DomNode<'a>>::new(),
        }
    }
    pub fn append_child(&mut self, child: DomNode<'a>) {
        self.children.push(child);
    }
}

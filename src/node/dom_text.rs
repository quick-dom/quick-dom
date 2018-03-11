use node::DomNode;
use quick_xml::events::BytesText;

#[derive(Clone, Debug)]
/// A struct representing a DOM Element.
pub struct DomText<'a> {
    pub bytes_start: BytesText<'a>,
}

impl<'a> DomText<'a> {
    /// Creates a `DomText` from a `BytesStart`
    pub fn from_start(bytes_start: BytesStart<'a>) -> DomText<'a> {
        DomText {
            bytes_start,
        }
    }
}

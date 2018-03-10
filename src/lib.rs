extern crate quick_xml;
#[macro_use]
extern crate failure;

mod node;
mod error;
mod dom_reader;

// reexports
pub use self::node::*;
pub use self::error::*;
pub use self::dom_reader::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

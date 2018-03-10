//! Provides an error type for this crate.
//! copy from minidom create

use std::convert::From;

/// Our main error type.
#[derive(Debug, Fail)]
pub enum Error {
    /// An error from quick_xml.
    #[fail(display = "XML error: {}", _0)]
    XmlError(#[cause] ::quick_xml::Error),

    /// An UTF-8 conversion error.
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8Error(#[cause] ::std::str::Utf8Error),

    /// An I/O error, from std::io.
    #[fail(display = "IO error: {}", _0)]
    IoError(#[cause] ::std::io::Error),

    /// An error which is returned when the end of the document was reached prematurely.
    #[fail(display = "the end of the document has been reached prematurely")]
    EndOfDocument,

    /// An error which is returned when an element is closed when it shouldn't be
    #[fail(display = "the XML is invalid, an element was wrongly closed")]
    InvalidElementClosed,

    /// An error which is returned when an elemet's name contains more than one colon
    #[fail(display = "the XML element is invalid")]
    InvalidElement,
}

impl From<::quick_xml::Error> for Error {
    fn from(err: ::quick_xml::Error) -> Error {
        Error::XmlError(err)
    }
}

impl From<::std::str::Utf8Error> for Error {
    fn from(err: ::std::str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::IoError(err)
    }
}

/// Our simplified Result type.
pub type Result<T> = ::std::result::Result<T, Error>;

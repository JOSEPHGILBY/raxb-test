use crate::sax_parse_exception::SAXParseException;

pub trait ErrorHandler {
    fn warning(&self, exception: SAXParseException);
    fn error(&self, exception: SAXParseException);
    fn fatal_error(&self, exception: SAXParseException);
}
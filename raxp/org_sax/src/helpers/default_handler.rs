// jdk/src/java.xml/share/classes/org/xml/sax/helpers/DefaultHandler.java

use crate::{input_source::InputSource, entity_resolver::EntityResolver, dtd_handler::DTDHandler, content_handler::ContentHandler, error_handler::ErrorHandler};

pub struct DefaultHandler {

}

impl DefaultHandler {
    pub fn new() -> DefaultHandler {
        DefaultHandler{}
    }

}

impl EntityResolver for DefaultHandler {
    fn resolve_entity(&self, public_id: &str, system_id: &str) -> InputSource {
        todo!()
    }
}

impl DTDHandler for DefaultHandler {
    fn notation_decl(&self, name: &str, public_id: &str, system_id: &str) {
        todo!()
    }

    fn unparsed_entity_decl(&self, name: &str, public_id: &str, system_id: &str, notation_name: &str) {
        todo!()
    }
}

impl ContentHandler for DefaultHandler {
    fn set_document_locator(&self, locator: Box<dyn crate::locator::Locator>) {
        todo!()
    }

    fn start_document(&self) {
        todo!()
    }

    fn end_document(&self) {
        todo!()
    }

    fn start_prefix_mapping(&self, prefix: &str, uri: &str) {
        todo!()
    }

    fn end_prefix_mapping(&self, prefix: &str) {
        todo!()
    }

    fn start_element(&self, uri: &str, local_name: &str, q_name: &str, atts: &str) {
        todo!()
    }

    fn end_element(&self, uri: &str, local_name: &str, q_name: &str) {
        todo!()
    }

    fn characters(&self, ch: &[char], start: i32, length: i32) {
        todo!()
    }

    fn processing_instruction(&self, target: &str, data: &str) {
        todo!()
    }

    fn skipped_entity(&self, name: &str) {
        todo!()
    }
}

impl ErrorHandler for DefaultHandler {
    fn warning(&self, exception: crate::sax_parse_exception::SAXParseException) {
        todo!()
    }

    fn error(&self, exception: crate::sax_parse_exception::SAXParseException) {
        todo!()
    }

    fn fatal_error(&self, exception: crate::sax_parse_exception::SAXParseException) {
        todo!()
    }
}
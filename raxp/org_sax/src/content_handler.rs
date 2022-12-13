// jdk/src/java.xml/share/classes/org/xml/sax/ContentHandler.java

use crate::locator::Locator;

pub trait ContentHandler {
    fn set_document_locator(&self, locator: Box<dyn Locator>);

    fn start_document(&self);

    fn declaration(&self, version: &str, encoding: &str, standalone: &str) {} // no-op default implementation

    fn end_document(&self);

    fn start_prefix_mapping(&self, prefix: &str, uri: &str);

    fn end_prefix_mapping(&self, prefix: &str);

    fn start_element(&self, uri: &str, local_name: &str, q_name: &str, atts: &str);

    fn end_element(&self, uri: &str, local_name: &str, q_name: &str);

    // TODO: double-check 'char' slice
    fn characters(&self, ch: &[char], start: i32, length: i32);

    fn processing_instruction(&self, target: &str, data: &str);

    fn skipped_entity(&self, name: &str);

}
// jdk/src/java.xml/share/classes/org/w3c/dom/Document.java

use std::rc::Rc;

use crate::{node::Node, document_type::DocumentType, dom_implementation::DOMImplementation, element::Element, document_fragment::DocumentFragment, text::Text, comment::Comment, cdata_section::CDATASection};

pub trait Document: Node {
    fn get_doc_type(&self) -> Rc<dyn DocumentType>;
    fn get_implementation(&self) -> Rc<dyn DOMImplementation>;
    fn get_document_element(&self) -> Rc<dyn Element>;
    fn create_element(&self, tag_name: &'static str) -> Rc<dyn Element>;
    fn create_document_fragment(&self) -> Rc<dyn DocumentFragment>;
    fn create_text_node(&self, data: &'static str) -> Rc<dyn Text>;
    fn create_comment(&self, data: &'static str) -> Rc<dyn Comment>;
    fn create_cdata_section(&self, data: &'static str) -> Rc<dyn CDATASection>;
    fn create_processing_instruction(&self, target: &'static str, data: &'static str) -> Rc<dyn ProcessingInstruction>;
    fn create_attribute(&self, name: &'static str) -> Rc<dyn Attr>;

}
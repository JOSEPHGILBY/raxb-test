// jdk/src/java.xml/share/classes/org/w3c/dom/DOMImplementation.java

use std::rc::Rc;

use crate::{document_type::DocumentType, document::Document};

pub trait DOMImplementation {
    fn has_feature(&self, feature: &'static str, version: &'static str) -> bool;
    fn create_document_type(&self, qualified_name: &'static str, public_id: &'static str, system_id: &'static str) -> Rc<dyn DocumentType>;
    fn create_document(&self, namespace_uri: &'static str, qualified_name: &'static str, doctype: Rc<dyn DocumentType>) -> Rc<dyn Document>;
    fn get_feature(&self, feature: &'static str, version: &'static str) -> String; // TODO: object
    
}
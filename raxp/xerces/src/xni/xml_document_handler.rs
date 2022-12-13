// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLDocumentHandler.java

use std::{sync::Arc, rc::Rc};

use super::{xml_locator::XMLLocator, namespace_context::NamespaceContext, augmentations::Augmentations, xml_string::XMLString, q_name::QName, xml_attributes::XMLAttributes, xml_resource_identifier::XMLResourceIdentifier, parser::xml_document_source::XMLDocumentSource};

pub trait XMLDocumentHandler {
    fn start_document(&mut self, locator: Option<Rc<dyn XMLLocator>>, encoding: &'static str, namespace_context: Box<dyn NamespaceContext>, augs: Box<dyn Augmentations>);
    fn xml_decl(&self, version: &'static str, encoding: &'static str, standalone: &'static str, augs: Box<dyn Augmentations>);
    fn doctype_decl(&self, root_element: &'static str, public_id: &'static str, system_id: &'static str, augs: Box<dyn Augmentations>);
    fn comment(&self, text: XMLString, augs: Box<dyn Augmentations>);
    fn processing_instruction(&self, target: &'static str, data: XMLString, augs: Box<dyn Augmentations>);
    fn start_element(&self, element: &QName, attributes: Box<dyn XMLAttributes>, augs: Arc<dyn Augmentations>);
    fn empty_element(&self, element: &QName, attributes: Box<dyn XMLAttributes>, augs: Arc<dyn Augmentations>);
    fn start_general_entity(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, encoding: &'static str, augs: Box<dyn Augmentations>);
    fn text_decl(&self, version: &'static str, encoding: &'static str, augs: Box<dyn Augmentations>);
    fn end_general_entity(&self, name: &'static str, augs: Box<dyn Augmentations>);
    fn characters(&self, text: XMLString, augs: Box<dyn Augmentations>);
    fn ignorable_whitespace(&self, text: XMLString, augs: Box<dyn Augmentations>);
    fn end_element(&self, element: &QName, augs: Arc<dyn Augmentations>);
    fn start_cdata(&self, augs: Box<dyn Augmentations>);
    fn end_cdata(&self, augs: Box<dyn Augmentations>);
    fn end_document(&self, augs: Box<dyn Augmentations>);
    fn set_document_source(&mut self, source: Arc<dyn XMLDocumentSource>);
    fn get_document_source(&self) -> Option<Arc<dyn XMLDocumentSource>>;
}
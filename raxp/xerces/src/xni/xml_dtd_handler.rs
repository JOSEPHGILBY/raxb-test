// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLDTDHandler.java

use std::sync::Arc;

use super::{xml_locator::XMLLocator, augmentations::Augmentations, xml_resource_identifier::XMLResourceIdentifier, xml_string::XMLString, parser::xml_dtd_source::XMLDTDSource};

pub trait XMLDTDHandler {
    fn conditional_include(&self) -> i16 { 0 }
    fn conditional_ignore(&self) -> i16 { 1 }
    fn start_dtd(&mut self, locator: Box<dyn XMLLocator>, augmentations: Box<dyn Augmentations>);
    fn start_parameter_entity(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, encoding: &'static str, augmentations: Box<dyn Augmentations>);
    fn text_decl(&self, version: &'static str, encoding: &'static str, augmentations: Box<dyn Augmentations>);
    fn end_parametner_family(&self, name: &'static str, augmentations: Box<dyn Augmentations>);
    fn start_external_subset(&self, identifer: Box<dyn XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>);
    fn end_external_subset(&self, augmentations: Box<dyn Augmentations>);
    fn comment(&self, text: XMLString, augmentations: Box<dyn Augmentations>);
    fn processing_instruction(&self, target: &'static str, data: XMLString, augmentations: Box<dyn Augmentations>);
    fn element_decl(&self, name: &'static str, content_model: &'static str, augmentations: Box<dyn Augmentations>);
    fn start_attlist(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>);
    fn attribute_decl(
        &self,
        element_name: &'static str, 
        attribute_name: &'static str, 
        the_type: &'static str, 
        enumeration: &[&'static str], 
        default_type: &'static str, 
        default_value: XMLString,
        non_normalized_default_value: XMLString,
        augmentations: Box<dyn Augmentations>
    );
    fn end_attlist(&self, augmentations: Box<dyn Augmentations>);
    fn internal_entity_decl(
        &self,
        name: &'static str, 
        text: XMLString,
        non_normalized_text: XMLString,
        augmentations: Box<dyn Augmentations>
    );
    fn external_entity_decl(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>);
    fn unparsed_entity_decl(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, notation: &'static str, augmentations: Box<dyn Augmentations>);
    fn notation_decl(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>);
    fn start_conditional(&self, the_type: i16, augmentations: Box<dyn Augmentations>);
    fn ignored_characters(&self, text: XMLString, augmentations: Box<dyn Augmentations>);
    fn end_conditional(&self, augmentations: Box<dyn Augmentations>);
    fn end_dtd(&mut self, augmentations: Box<dyn Augmentations>);
    fn set_dtd_source(&mut self, source: Arc<dyn XMLDTDSource>);
    fn get_dtd_source(&self) -> Option<Arc<dyn XMLDTDSource>>;
}
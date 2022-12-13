//jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLParserConfiguration.java


use std::{sync::Arc, rc::Rc};

use crate::xni::{xml_document_handler::XMLDocumentHandler, xml_dtd_handler::XMLDTDHandler, xml_dtd_content_model_handler::XMLDTDContentModelHandler};

use super::{xml_input_source::XMLInputSource, xml_component_manager::XMLComponentManager};


pub trait XMLParserConfiguration: XMLComponentManager {
    fn parse(&self, input_source: XMLInputSource);
    fn add_recognized_features(&self, feature_ids: &[&'static str]); // String or &str?
    fn set_feature(&self, feature_id: &'static str, state: bool);
    fn get_feature(&self, feature_id: &'static str) -> bool;
    fn add_recognized_properties(&self, property_ids: &[&'static str]);
    fn set_property(&self, property_id: &'static str, value: String); // TODO: value: Object
    fn get_property(&self, property_id: &'static str) -> String; // TODO: return Object?
    fn set_error_handler(&self, error_handler: String); // TODO: XMLErrorHandler
    fn get_error_handler(&self) -> String;
    fn set_document_handler(&self, document_handler: Rc<dyn XMLDocumentHandler>);
    fn get_document_handler(&self) -> Box<dyn XMLDocumentHandler>;
    fn set_dtd_handler(&self, dtd_handler: Rc<dyn XMLDTDHandler>);
    fn get_dtd_handler(&self) -> Box<dyn XMLDTDHandler>;
    fn set_dtd_content_model_handler(&self, dtd_content_model_handler: Rc<dyn XMLDTDContentModelHandler>);
    fn get_dtd_content_model_handler(&self) -> Box<dyn XMLDTDContentModelHandler>;
}
//jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLParserConfiguration.java

use crate::xml_input_source::{XMLInputSource, self};
pub trait XMLParserConfiguration {
    fn parse(&self, input_source: XMLInputSource);
    // fn add_recognized_features(&self, feature_ids: &[String]); // String or &str?
    // fn set_feature(&self, feature_id: String, state: bool);
    // fn get_feature(&self, feature_id: String) -> bool;
    fn add_recognized_properties(&self, property_ids: &[&str]);
    // fn set_property(&self, property_id: String, value: String); // TODO: value: Object
    // fn get_property(&self, property_id: String) -> String; // TODO: return Object?
    // fn set_error_handler(&self, error_handler: String); // TODO: XMLErrorHandler
    // fn get_error_handler(&self) -> String;
    // fn set_document_handler(&self, document_handler: String); // TODO: XMLDocumentHandler
    // fn get_document_handler(&self) -> String;
    // fn set_dtd_handler(&self, dtd_handler: String); // TODO: XMLDTDHandler
    // fn set_dtd_content_handler(&self, dtd_content_model_handler: String); // TODO: XMLDTDContentModelHandler
}
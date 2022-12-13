// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/XIncludeAwareParserConfiguration.java

use crate::xni::parser::{xml_parser_configuration::XMLParserConfiguration, xml_component_manager::XMLComponentManager};

pub struct XIncludeAwareParserConfiguration {

}

impl XMLComponentManager for XIncludeAwareParserConfiguration {
    fn get_feature(&self, feature_id: &'static str) -> Result<bool, crate::xni::xni_error::XNIError> {
        todo!()
    }

    fn get_feature_2(&self, feature_id: &'static str, default_value: bool) -> bool {
        todo!()
    }

    fn get_property(&self, property_id: &'static str) -> Result<Option<String>, crate::xni::xni_error::XNIError> {
        todo!()
    }

    fn get_property_2(&self, property_id: &'static str, default_object: String) -> Result<Option<String>, crate::xni::xni_error::XNIError> {
        todo!()
    }

    fn get_feature_state(&self, feature_id: &'static str) -> crate::util::feature_state::FeatureStateType {
        todo!()
    }

    fn get_property_state(&self, property_id: &'static str) -> Result<crate::util::property_state::PropertyStateType, crate::xni::xni_error::XNIError> {
        todo!()
    }
}


impl XMLParserConfiguration for XIncludeAwareParserConfiguration {
    fn parse(&self, input_source: crate::xni::parser::xml_input_source::XMLInputSource) {
        todo!()
    }

    fn add_recognized_features(&self, feature_ids: &[&'static str]) {
        todo!()
    }

    fn set_feature(&self, feature_id: &'static str, state: bool) {
        todo!()
    }

    fn get_feature(&self, feature_id: &'static str) -> bool {
        todo!()
    }

    fn add_recognized_properties(&self, property_ids: &[&'static str]) {
        todo!()
    }

    fn set_property(&self, property_id: &'static str, value: String) {
        todo!()
    }

    fn get_property(&self, property_id: &'static str) -> String {
        todo!()
    }

    fn set_error_handler(&self, error_handler: String) {
        todo!()
    }

    fn get_error_handler(&self) -> String {
        todo!()
    }

    fn set_document_handler(&self, document_handler: std::rc::Rc<dyn crate::xni::xml_document_handler::XMLDocumentHandler>) {
        todo!()
    }

    fn get_document_handler(&self) -> Box<dyn crate::xni::xml_document_handler::XMLDocumentHandler> {
        todo!()
    }

    fn set_dtd_handler(&self, dtd_handler: std::rc::Rc<dyn crate::xni::xml_dtd_handler::XMLDTDHandler>) {
        todo!()
    }

    fn get_dtd_handler(&self) -> Box<dyn crate::xni::xml_dtd_handler::XMLDTDHandler> {
        todo!()
    }

    fn set_dtd_content_model_handler(&self, dtd_content_model_handler: std::rc::Rc<dyn crate::xni::xml_dtd_content_model_handler::XMLDTDContentModelHandler>) {
        todo!()
    }

    fn get_dtd_content_model_handler(&self) -> Box<dyn crate::xni::xml_dtd_content_model_handler::XMLDTDContentModelHandler> {
        todo!()
    }
}
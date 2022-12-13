//jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/AbstractSAXParser.java

use std::rc::Rc;

use const_format::concatcp;
use org_sax::{xml_reader::XMLReader, content_handler::ContentHandler, dtd_handler::DTDHandler, ext::{decl_handler::DeclHandler, lexical_handler::LexicalHandler, locator_2::Locator2}, locator::Locator};

use crate::{xs::psvi_provider::PSVIProvider, implementation::constants::Constants, xni::{namespace_context::NamespaceContext, q_name::QName, augmentations::Augmentations, parser::xml_parser_configuration::XMLParserConfiguration, xml_document_handler::XMLDocumentHandler, xml_dtd_content_model_handler::XMLDTDContentModelHandler, xml_dtd_handler::XMLDTDHandler, xml_locator::XMLLocator}, util::symbol_hash::SymbolHash};

use super::{abstract_xml_document_parser::{AbstractXMLDocumentParser, AbstractXMLDocumentParserAbstractItems}, xml_parser::XMLParser};

// skip Parser since it is deprecated
pub trait AbstractSAXParser: AbstractXMLDocumentParser + PSVIProvider + XMLReader { 
    

}

pub struct AbstractSAXParserAbstractItems {
    xml_document_parser: Rc<dyn AbstractXMLDocumentParser>,
    f_namespaces: bool,
    f_namespaces_prefixes: bool, // = false
    f_lexical_handler_parameter_entities: bool, // true
    f_standalone: bool,
    f_resolve_dtd_uris: bool, // true
    f_use_entity_resolver_2: bool, // true
    f_xmlns_uris: bool, // false
    f_content_handler: Option<Rc<dyn ContentHandler>>,
    //f_document_handler: Box<dyn DocumentHandler>, deprecated
    f_namespace_context: Option<Box<dyn NamespaceContext>>,
    f_dtd_handler: Option<Box<dyn DTDHandler>>,
    f_decl_handler: Option<Box<dyn DeclHandler>>,
    f_lexical_handler: Option<Box<dyn LexicalHandler>>,
    f_q_name: QName,

    f_parse_in_progress: bool, // false
    f_version: Option<&'static str>,
    f_attribute_proxy: AttributesProxy,
    f_augmentations: Option<Box<dyn Augmentations>>, // null
    f_char_buffer: [char; AbstractSAXParserAbstractItems::BUFFER_SIZE],
    f_declared_attrs: Option<SymbolHash>

}

impl AbstractSAXParserAbstractItems {
    pub const NAMESPACES: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::NAMESPACES_FEATURE);
    pub const NAMESPACE_PREFIXES: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::NAMESPACE_PREFIXES_FEATURE);
    pub const STRING_INTERNING: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::STRING_INTERNING_FEATURE);
    pub const ALLOW_UE_AND_NOTATION_EVENTS: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::ALLOW_DTD_EVENTS_AFTER_ENDDTD_FEATURE);

    const RECOGNIZED_FEATURES: &'static [&'static str] = &[
        AbstractSAXParserAbstractItems::NAMESPACES,
        AbstractSAXParserAbstractItems::NAMESPACE_PREFIXES,
        AbstractSAXParserAbstractItems::STRING_INTERNING,
    ];

    pub const LEXICAL_HANDLER: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::LEXICAL_HANDLER_PROPERTY);
    pub const DECLARATION_HANDLER: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::DECLARATION_HANDLER_PROPERTY);
    pub const DOM_NODE: &'static str = concatcp!(Constants::SAX_FEATURE_PREFIX, Constants::DOM_NODE_PROPERTY);
    pub const SECURITY_MANAGER: &'static str = concatcp!(Constants::XERCES_PROPERTY_PREFIX, Constants::SECURITY_MANAGER_PROPERTY);

    const RECOGNIZED_PROPERTIES: &'static [&'static str] = &[
        AbstractSAXParserAbstractItems::LEXICAL_HANDLER,
        AbstractSAXParserAbstractItems::DECLARATION_HANDLER,
        AbstractSAXParserAbstractItems::DOM_NODE,
    ];

    const BUFFER_SIZE: usize = 20;

    pub fn new_dyn(config: Rc<dyn XMLParserConfiguration>) -> Rc<dyn AbstractSAXParser> {
        let xml_document_parser = AbstractXMLDocumentParserAbstractItems::new_dyn(config.clone());
        let obj = Rc::new(AbstractSAXParserAbstractItems {
            xml_document_parser,
            f_namespaces: false,
            f_namespaces_prefixes: false,
            f_lexical_handler_parameter_entities: true,
            f_standalone: false,
            f_resolve_dtd_uris: true,
            f_use_entity_resolver_2: true,
            f_xmlns_uris: false,
            f_content_handler: None,
            f_namespace_context: None,
            f_dtd_handler: None,
            f_decl_handler: None,
            f_lexical_handler: None,
            f_q_name: QName::new_default(),
            f_parse_in_progress: false,
            f_version: None,
            f_attribute_proxy: AttributesProxy {  },
            f_augmentations: None,
            f_char_buffer: ['\0'; AbstractSAXParserAbstractItems::BUFFER_SIZE],
            f_declared_attrs: None,
        });
        config.add_recognized_features(AbstractSAXParserAbstractItems::RECOGNIZED_FEATURES);
        config.add_recognized_properties(AbstractSAXParserAbstractItems::RECOGNIZED_PROPERTIES);
        // TODO: try?
        config.set_feature(AbstractSAXParserAbstractItems::ALLOW_UE_AND_NOTATION_EVENTS, false);
        obj
    }
}

impl XMLDocumentHandler for AbstractSAXParserAbstractItems {
    fn start_document(&mut self, locator: Option<Rc<dyn XMLLocator>>, encoding: &'static str, namespace_context: Box<dyn NamespaceContext>, augs: Box<dyn Augmentations>) {
        self.f_namespace_context = Some(namespace_context);
        
        //skip document handler, SAX2 only

        let Some(ch) = self.f_content_handler.clone() else { return };
        if let Some(locator) = locator {
            ch.set_document_locator(Box::new(LocatorProxy{f_locator: locator}));
        }
        ch.start_document();
        
    }

    fn xml_decl(&self, version: &'static str, encoding: &'static str, standalone: &'static str, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn doctype_decl(&self, root_element: &'static str, public_id: &'static str, system_id: &'static str, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn comment(&self, text: crate::xni::xml_string::XMLString, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn processing_instruction(&self, target: &'static str, data: crate::xni::xml_string::XMLString, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn start_element(&self, element: &QName, attributes: Box<dyn crate::xni::xml_attributes::XMLAttributes>, augs: std::sync::Arc<dyn Augmentations>) {
        todo!()
    }

    fn empty_element(&self, element: &QName, attributes: Box<dyn crate::xni::xml_attributes::XMLAttributes>, augs: std::sync::Arc<dyn Augmentations>) {
        todo!()
    }

    fn start_general_entity(&self, name: &'static str, identifier: Box<dyn crate::xni::xml_resource_identifier::XMLResourceIdentifier>, encoding: &'static str, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn text_decl(&self, version: &'static str, encoding: &'static str, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_general_entity(&self, name: &'static str, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn characters(&self, text: crate::xni::xml_string::XMLString, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn ignorable_whitespace(&self, text: crate::xni::xml_string::XMLString, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_element(&self, element: &QName, augs: std::sync::Arc<dyn Augmentations>) {
        todo!()
    }

    fn start_cdata(&self, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_cdata(&self, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_document(&self, augs: Box<dyn Augmentations>) {
        todo!()
    }

    fn set_document_source(&mut self, source: std::sync::Arc<dyn crate::xni::parser::xml_document_source::XMLDocumentSource>) {
        todo!()
    }

    fn get_document_source(&self) -> Option<std::sync::Arc<dyn crate::xni::parser::xml_document_source::XMLDocumentSource>> {
        todo!()
    }
}

impl XMLParser for AbstractSAXParserAbstractItems {
    fn get_features(&self, feature_id: &str) -> bool {
        todo!()
    }

    fn parse(&mut self, input_source: crate::xni::parser::xml_input_source::XMLInputSource) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}

impl XMLDTDContentModelHandler for AbstractSAXParserAbstractItems {
    fn start_content_model(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn any(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn empty(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn start_group(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn pcdata(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn element(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn separator(&self, separator: i16, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn occurrence(&self, occurrence: i16, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_group(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_content_model(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn set_dtd_content_model_source(&mut self, source: std::sync::Arc<dyn crate::xni::parser::xml_dtd_content_model_source::XMLDTDContentModelSource>) {
        todo!()
    }

    fn get_dtd_content_model_source(&self) -> Option<std::sync::Arc<dyn crate::xni::parser::xml_dtd_content_model_source::XMLDTDContentModelSource>> {
        todo!()
    }
}

impl XMLDTDHandler for AbstractSAXParserAbstractItems {
    fn start_dtd(&mut self, locator: Box<dyn crate::xni::xml_locator::XMLLocator>, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn start_parameter_entity(&self, name: &'static str, identifier: Box<dyn crate::xni::xml_resource_identifier::XMLResourceIdentifier>, encoding: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn text_decl(&self, version: &'static str, encoding: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_parametner_family(&self, name: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn start_external_subset(&self, identifer: Box<dyn crate::xni::xml_resource_identifier::XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_external_subset(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn comment(&self, text: crate::xni::xml_string::XMLString, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn processing_instruction(&self, target: &'static str, data: crate::xni::xml_string::XMLString, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn element_decl(&self, name: &'static str, content_model: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn start_attlist(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn attribute_decl(
        &self,
        element_name: &'static str, 
        attribute_name: &'static str, 
        the_type: &'static str, 
        enumeration: &[&'static str], 
        default_type: &'static str, 
        default_value: crate::xni::xml_string::XMLString,
        non_normalized_default_value: crate::xni::xml_string::XMLString,
        augmentations: Box<dyn Augmentations>
    ) {
        todo!()
    }

    fn end_attlist(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn internal_entity_decl(
        &self,
        name: &'static str, 
        text: crate::xni::xml_string::XMLString,
        non_normalized_text: crate::xni::xml_string::XMLString,
        augmentations: Box<dyn Augmentations>
    ) {
        todo!()
    }

    fn external_entity_decl(&self, name: &'static str, identifier: Box<dyn crate::xni::xml_resource_identifier::XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn unparsed_entity_decl(&self, name: &'static str, identifier: Box<dyn crate::xni::xml_resource_identifier::XMLResourceIdentifier>, notation: &'static str, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn notation_decl(&self, name: &'static str, identifier: Box<dyn crate::xni::xml_resource_identifier::XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn start_conditional(&self, the_type: i16, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn ignored_characters(&self, text: crate::xni::xml_string::XMLString, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_conditional(&self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn end_dtd(&mut self, augmentations: Box<dyn Augmentations>) {
        todo!()
    }

    fn set_dtd_source(&mut self, source: std::sync::Arc<dyn crate::xni::parser::xml_dtd_source::XMLDTDSource>) {
        todo!()
    }

    fn get_dtd_source(&self) -> Option<std::sync::Arc<dyn crate::xni::parser::xml_dtd_source::XMLDTDSource>> {
        todo!()
    }
}

impl AbstractXMLDocumentParser for AbstractSAXParserAbstractItems {

}
impl XMLReader for AbstractSAXParserAbstractItems {
    fn get_feature(&self, name: &str) -> bool {
        todo!()
    }

    fn set_feature(&self, name: &str, value: bool) {
        todo!()
    }

    fn get_property(&self, name: &str) {
        todo!()
    }

    fn set_property(&self, name: &str, value: &str) {
        todo!()
    }

    fn set_entity_resolver(&self, resolver: Rc<dyn org_sax::entity_resolver::EntityResolver>) {
        todo!()
    }

    fn get_entity_resolver(&self) -> Box<dyn org_sax::entity_resolver::EntityResolver> {
        todo!()
    }

    fn set_dtd_handler(&self, handler: Rc<dyn DTDHandler>) {
        todo!()
    }

    fn get_dtd_handler(&self) -> Box<dyn DTDHandler> {
        todo!()
    }

    fn set_content_handler(&self, handler: Rc<dyn ContentHandler>) {
        todo!()
    }

    fn get_content_handler(&self) -> Box<dyn ContentHandler> {
        todo!()
    }

    fn set_error_handler(&self, handler: Rc<dyn org_sax::error_handler::ErrorHandler>) {
        todo!()
    }

    fn get_error_handler(&self) -> Box<dyn org_sax::error_handler::ErrorHandler> {
        todo!()
    }

    fn parse_from_input_source(&self, input: org_sax::input_source::InputSource) {
        todo!()
    }

    fn parse_from_system_id(&self, system_id: &str) {
        todo!()
    }
}

impl PSVIProvider for AbstractSAXParserAbstractItems {
    fn get_element_psvi(&self) -> Box<dyn crate::xs::element_psvi::ElementPSVI> {
        todo!()
    }

    fn get_attribute_psvi(&self, index: i32) -> Box<dyn crate::xs::attribute_psvi::AttributePSVI> {
        todo!()
    }

    fn get_attribute_psvi_by_name(&self, uri: &'static str, localname: &'static str) -> Box<dyn crate::xs::attribute_psvi::AttributePSVI> {
        todo!()
    }
}

impl AbstractSAXParser for AbstractSAXParserAbstractItems {

}

pub struct LocatorProxy {
    f_locator: Rc<dyn XMLLocator>
}

impl Locator for LocatorProxy {
    fn get_public_id(&self) -> &'static str {
        todo!()
    }

    fn get_system_id(&self) -> &'static str {
        todo!()
    }

    fn get_line_number(&self) -> i32 {
        todo!()
    }

    fn get_column_number(&self) -> i32 {
        todo!()
    }
}

impl Locator2 for LocatorProxy {
    fn get_xml_version(&self) -> &'static str {
        todo!()
    }

    fn get_encoding(&self) -> &'static str {
        todo!()
    }
}

pub struct AttributesProxy {

}

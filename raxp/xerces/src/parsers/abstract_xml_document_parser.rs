// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/AbstractXMLDocumentParser.java

use std::{sync::Arc, rc::Rc};

use delegate::delegate;

use crate::{parsers::xml_parser::XMLParser, xni::{xml_document_handler::XMLDocumentHandler, xml_dtd_handler::XMLDTDHandler, xml_dtd_content_model_handler::XMLDTDContentModelHandler, parser::{xml_document_source::XMLDocumentSource, xml_dtd_source::XMLDTDSource, xml_dtd_content_model_source::XMLDTDContentModelSource, xml_input_source::XMLInputSource, xml_parser_configuration::XMLParserConfiguration}, xml_locator::XMLLocator, namespace_context::NamespaceContext, augmentations::Augmentations, xml_string::XMLString, q_name::QName, xml_attributes::XMLAttributes, xml_resource_identifier::XMLResourceIdentifier}};

use super::xml_parser::XMLParserAbstractItems;

pub trait AbstractXMLDocumentParser: XMLParser + XMLDocumentHandler + XMLDTDHandler + XMLDTDContentModelHandler {
    
}

pub struct AbstractXMLDocumentParserAbstractItems {
    xml_parser: Box<dyn XMLParser>,
    f_in_dtd: bool,
    f_document_source: Option<Arc<dyn XMLDocumentSource>>,
    f_dtd_source: Option<Arc<dyn XMLDTDSource>>,
    f_dtd_content_mode_source: Option<Arc<dyn XMLDTDContentModelSource>>,
}

impl AbstractXMLDocumentParserAbstractItems {
    pub fn new_dyn(config: Rc<dyn XMLParserConfiguration>) -> Rc<dyn AbstractXMLDocumentParser> {
        let xml_parser = XMLParserAbstractItems::new_dyn(config.clone());
        let obj = Rc::new(AbstractXMLDocumentParserAbstractItems {
            xml_parser,
            f_in_dtd: false,
            f_document_source: None,
            f_dtd_source: None,
            f_dtd_content_mode_source: None,
        });
        config.set_document_handler(obj.clone());
        config.set_dtd_handler(obj.clone());
        config.set_dtd_content_model_handler(obj.clone());
        obj
    }
}

impl XMLParser for AbstractXMLDocumentParserAbstractItems {
    
    delegate! {
        to self.xml_parser {
            fn get_features(&self, feature_id: &str) -> bool;
            fn parse(&mut self, input_source: XMLInputSource);
        }
    }

    fn reset(&mut self) {
        self.xml_parser.reset();
        self.f_in_dtd = false;
    }
}

impl XMLDocumentHandler for AbstractXMLDocumentParserAbstractItems {
    fn start_document(&mut self, locator: Option<Rc<dyn XMLLocator>>, encoding: &'static str, namespace_context: Box<dyn NamespaceContext>, augs: Box<dyn Augmentations>) {}
    fn xml_decl(&self, version: &'static str, encoding: &'static str, standalone: &'static str, augs: Box<dyn Augmentations>) {}
    fn doctype_decl(&self, root_element: &'static str, public_id: &'static str, system_id: &'static str, augs: Box<dyn Augmentations>) {}
    fn start_element(&self, element: &QName, attributes: Box<dyn XMLAttributes>, augs: Arc<dyn Augmentations>) {}
    fn empty_element(&self, element: &QName, attributes: Box<dyn XMLAttributes>, augs: Arc<dyn Augmentations>) {
        self.start_element(element, attributes, augs.clone());
        self.end_element(element, augs)
    }
    fn characters(&self, text: XMLString, augs: Box<dyn Augmentations>) {}
    fn ignorable_whitespace(&self, text: XMLString, augs: Box<dyn Augmentations>) {}
    fn end_element(&self, element: &QName, augs: Arc<dyn Augmentations>) {}
    fn start_cdata(&self, augs: Box<dyn Augmentations>) {}
    fn end_cdata(&self, augs: Box<dyn Augmentations>) {}
    fn end_document(&self, augs: Box<dyn Augmentations>) {}
    fn start_general_entity(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, encoding: &'static str, augs: Box<dyn Augmentations>) {}
    fn text_decl(&self, version: &'static str, encoding: &'static str, augs: Box<dyn Augmentations>) {}
    fn end_general_entity(&self, name: &'static str, augs: Box<dyn Augmentations>) {}
    fn comment(&self, text: XMLString, augs: Box<dyn Augmentations>) {}
    fn processing_instruction(&self, target: &'static str, data: XMLString, augs: Box<dyn Augmentations>) {}
    fn set_document_source(&mut self, source: Arc<dyn XMLDocumentSource>) {
        self.f_document_source = Some(source);
    }
    fn get_document_source(&self) -> Option<Arc<dyn XMLDocumentSource>> {
        self.f_document_source.clone()
    }
    
}

impl XMLDTDHandler for AbstractXMLDocumentParserAbstractItems {
    fn start_dtd(&mut self, locator: Box<dyn XMLLocator>, augmentations: Box<dyn Augmentations>) {
        self.f_in_dtd = true;
    }
    fn start_external_subset(&self, identifer: Box<dyn XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>) {}
    fn end_external_subset(&self, augmentations: Box<dyn Augmentations>) {}
    fn start_parameter_entity(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, encoding: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn end_parametner_family(&self, name: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn ignored_characters(&self, text: XMLString, augmentations: Box<dyn Augmentations>) {}
    fn element_decl(&self, name: &'static str, content_model: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn start_attlist(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>) {}
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
    ) {}
    fn end_attlist(&self, augmentations: Box<dyn Augmentations>) {}
    fn internal_entity_decl(
        &self,
        name: &'static str, 
        text: XMLString,
        non_normalized_text: XMLString,
        augmentations: Box<dyn Augmentations>
    ) {}
    fn external_entity_decl(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>) {}
    fn unparsed_entity_decl(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, notation: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn notation_decl(&self, name: &'static str, identifier: Box<dyn XMLResourceIdentifier>, augmentations: Box<dyn Augmentations>) {}
    fn start_conditional(&self, the_type: i16, augmentations: Box<dyn Augmentations>) {}
    fn end_conditional(&self, augmentations: Box<dyn Augmentations>) {}
    fn end_dtd(&mut self, augmentations: Box<dyn Augmentations>) {
        self.f_in_dtd = false;
    }
    fn set_dtd_source(&mut self, source: Arc<dyn XMLDTDSource>) {
        self.f_dtd_source = Some(source);
    }
    fn get_dtd_source(&self) -> Option<Arc<dyn XMLDTDSource>> {
        self.f_dtd_source.clone()
    }

    // same method defined in both traits
    fn text_decl(&self, version: &'static str, encoding: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn comment(&self, text: XMLString, augmentations: Box<dyn Augmentations>) {}
    fn processing_instruction(&self, target: &'static str, data: XMLString, augmentations: Box<dyn Augmentations>) {}
}

impl XMLDTDContentModelHandler for AbstractXMLDocumentParserAbstractItems {
    fn start_content_model(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn any(&self, augmentations: Box<dyn Augmentations>) {}
    fn empty(&self, augmentations: Box<dyn Augmentations>) {}
    fn start_group(&self, augmentations: Box<dyn Augmentations>) {}
    fn pcdata(&self, augmentations: Box<dyn Augmentations>) {}
    fn element(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>) {}
    fn separator(&self, separator: i16, augmentations: Box<dyn Augmentations>) {}
    fn occurrence(&self, occurrence: i16, augmentations: Box<dyn Augmentations>) {}
    fn end_group(&self, augmentations: Box<dyn Augmentations>) {}
    fn end_content_model(&self, augmentations: Box<dyn Augmentations>) {}
    fn set_dtd_content_model_source(&mut self, source: Arc<dyn XMLDTDContentModelSource>) {
        self.f_dtd_content_mode_source = Some(source);
    }
    fn get_dtd_content_model_source(&self) -> Option<Arc<dyn XMLDTDContentModelSource>> {
        self.f_dtd_content_mode_source.clone()
    }
}

impl AbstractXMLDocumentParser for AbstractXMLDocumentParserAbstractItems {

}


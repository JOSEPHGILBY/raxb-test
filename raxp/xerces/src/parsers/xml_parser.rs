// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/XMLParser.java

use std::{sync::Arc, rc::Rc};

use const_format::concatcp;

use crate::{implementation::constants::Constants, xni::parser::{xml_input_source::XMLInputSource, xml_parser_configuration::XMLParserConfiguration}, utils::{xml_security_manager::XMLSecurityManager, xml_security_property_manager::XMLSecurityPropertyManager}};

pub trait XMLParser {
    fn get_features(&self, feature_id: &str) -> bool;
    fn parse(&mut self, input_source: XMLInputSource);
    fn reset(&mut self);
}

pub struct XMLParserAbstractItems {
    // TODO: figure out
    f_configuration: Rc<dyn XMLParserConfiguration>,
    security_manager: Option<XMLSecurityManager>,
    security_property_manager: Option<XMLSecurityPropertyManager>,
}

impl XMLParserAbstractItems {
    const ENTITY_RESOLVER: &str = concatcp!(Constants::XERCES_PROPERTY_PREFIX, Constants::ENTITY_RESOLVER_PROPERTY);
    const ERROR_HANDLER: &str = concatcp!(Constants::XERCES_PROPERTY_PREFIX, Constants::ERROR_HANDLER_PROPERTY);
    const RECOGNIZED_PROPERTIES: [&str; 2] = [XMLParserAbstractItems::ENTITY_RESOLVER, XMLParserAbstractItems::ENTITY_RESOLVER];

    pub fn new_dyn(config: Rc<dyn XMLParserConfiguration>) -> Box<dyn XMLParser> {
        Box::new(XMLParserAbstractItems {
            f_configuration: config,
            security_manager: None,
            security_property_manager: None,
        })
    }
}

impl XMLParser for XMLParserAbstractItems {
    fn get_features(&self, feature_id: &str) -> bool {
        todo!()
    }

    fn parse(&mut self, input_source: XMLInputSource) {
        // TODO: Security manager config
        
        self.reset();
        self.f_configuration.parse(input_source);
    }

    fn reset(&mut self) {
        todo!()
    }
}

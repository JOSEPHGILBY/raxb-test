// jdk/src/java.xml/share/classes/org/xml/sax/XMLReader.java

use std::rc::Rc;

use crate::{entity_resolver::EntityResolver, dtd_handler::DTDHandler, content_handler::ContentHandler, error_handler::ErrorHandler, input_source::InputSource};

pub trait XMLReader {
    fn get_feature(&self, name: &str) -> bool;

    fn set_feature(&self, name: &str, value: bool);

    // TODO: Object
    fn get_property(&self, name: &str);
    // TODO: Object
    fn set_property(&self, name: &str, value: &str);

    fn set_entity_resolver(&self, resolver: Rc<dyn EntityResolver>);

    fn get_entity_resolver(&self) -> Box<dyn EntityResolver>;

    fn set_dtd_handler(&self, handler: Rc<dyn DTDHandler>);

    fn get_dtd_handler(&self) -> Box<dyn DTDHandler>;

    fn set_content_handler(&self, handler: Rc<dyn ContentHandler>);

    fn get_content_handler(&self) -> Box<dyn ContentHandler>;

    fn set_error_handler(&self, handler: Rc<dyn ErrorHandler>);

    fn get_error_handler(&self) -> Box<dyn ErrorHandler>;

    fn parse_from_input_source(&self, input: InputSource);

    fn parse_from_system_id(&self, system_id: &str);

}
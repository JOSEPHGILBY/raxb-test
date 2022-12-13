use crate::parsers::sax_parser::SAXParser;



// jdk/src/java.xml/share/classes/javax/xml/parsers/SAXParserFactory.java
pub trait SAXParserFactory {
    fn new_sax_parser(&self) -> Box<dyn SAXParser>;
    fn set_namespace_aware(&self, awareness: bool);
    fn set_validating(&self, validating: bool);
    fn is_namespace_aware(&self) -> bool;
    fn is_validating(&self) -> bool;
    fn set_feature(&self, name: String, value: bool);
    fn get_feature(&self, name: String) -> bool;
    fn get_schema(&self) -> String;
    fn set_schema(&self, schema: String);
    fn set_xinclude_aware(&self, state: bool);
    fn is_xinclude_aware(&self) -> bool;
}

pub struct SAXParserFactoryAbstractItems {
    validating: bool,
    namespace_aware: bool,
}

impl SAXParserFactoryAbstractItems {
    pub const DEFAULT_IMPL: &str = "com.sun.org.apache.xerces.internal.jaxp.SAXParserFactoryImpl";

    // pub fn new_default_ns_instance() -> impl SAXParserFactory {

    // }
    // pub fn new_ns_instance_1() -> impl SAXParserFactory {

    // }
    // pub fn new_ns_instance_2(factory_class_name: String, class_loader: String) -> impl SAXParserFactory {

    // }
    // pub fn new_default_instance() -> impl SAXParserFactory {
    //     SAXParserFactoryImpl::new()
    // }
    pub fn new_1() -> Box<dyn SAXParserFactory> {
        todo!()
    }

    // pub fn new_instance_2() -> impl SAXParserFactory {
    //     SAXParserFactory::new()
    // }

    // fn make_ns_aware() -> impl SAXParserFactory {

    // }

}
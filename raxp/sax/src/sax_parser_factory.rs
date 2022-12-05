use crate::sax_parser_factory_impl::SAXParserFactoryImpl;

// jdk/src/java.xml/share/classes/javax/xml/parsers/SAXParserFactory.java
pub struct SAXParserFactory {
    validating: bool,
    namespace_aware: bool,
}

impl SAXParserFactory {
    pub const DEFAULT_IMPL: &str = "com.sun.org.apache.xerces.internal.jaxp.SAXParserFactoryImpl";

    pub fn new_default_instance() -> SAXParserFactoryImpl {
        SAXParserFactoryImpl::new()
    }

    pub fn new_instance() -> SAXParserFactoryImpl {
        SAXParserFactoryImpl::new()
    }

}
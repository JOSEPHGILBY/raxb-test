
use crate::impl_constants::ImplConstants;
use crate::combine;
use crate::sax_parser::SAXParser;

use std::collections::HashMap;

//jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl.java
#[derive(Default)]
pub struct SAXParserFactoryImpl {
    features: HashMap<String, bool>,
    // TODO: Schema type
    grammar: String, // Schema
    is_xinclude_aware: bool,
    f_secure_process: bool,
}

impl SAXParserFactoryImpl {
    const VALIDATION_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::VALIDATION_FEATURE);
    const NAMESPACES_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::NAMESPACES_FEATURE);
    const XINCLUDE_FEATURE: &str = combine!(ImplConstants::XERCES_FEATURE_PREFIX, ImplConstants::XINCLUDE_FEATURE);

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new_sax_parser(&self) -> SAXParser {
        SAXParser {}
    }
    
}
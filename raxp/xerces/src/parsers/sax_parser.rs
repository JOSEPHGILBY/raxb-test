// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/SAXParser.java


use std::rc::Rc;

use const_format::concatcp;

use crate::{implementation::constants::Constants, xni::parser::xml_parser_configuration::XMLParserConfiguration};

use super::{abstract_sax_parser::{AbstractSAXParser, AbstractSAXParserAbstractItems}, x_include_aware_parser_configuration::XIncludeAwareParserConfiguration};

pub struct SAXParser {
    abstract_sax_parser: Rc<dyn AbstractSAXParser>
}

impl SAXParser {
    const NOTIFY_BUILTIN_REFS: &'static str = concatcp!(Constants::XERCES_FEATURE_PREFIX, Constants::NOTIFY_BUILTIN_REFS_FEATURE);
    const REPORT_WHITESPACE: &'static str = concatcp!(Constants::SUN_SCHEMA_FEATURE_PREFIX, Constants::SUN_REPORT_IGNORED_ELEMENT_CONTENT_WHITESPACE);

    const RECOGNIZED_FEATURES: &'static [&'static str] = &[
        SAXParser::NOTIFY_BUILTIN_REFS,
        SAXParser::REPORT_WHITESPACE,
    ];

    const SYMBOL_TABLE: &'static str = concatcp!(Constants::XERCES_PROPERTY_PREFIX, Constants::SYMBOL_TABLE_PROPERTY);
    const XMLGRAMMAR_POOL: &'static str = concatcp!(Constants::XERCES_PROPERTY_PREFIX, Constants::XMLGRAMMAR_POOL_PROPERTY);

    const RECOGNIZED_PROPERTIES: &'static [&'static str] = &[
        SAXParser::SYMBOL_TABLE,
        SAXParser::XMLGRAMMAR_POOL,
    ];

    fn new_1(config: Rc<dyn XMLParserConfiguration>) -> SAXParser {
        let abstract_sax_parser = AbstractSAXParserAbstractItems::new_dyn(config);
        SAXParser {
            abstract_sax_parser
        }
    }
    
    fn new_2() -> SAXParser {
        Self::new_4(None, None)
    }

    fn new_3(symbol_table: Option<String>) -> SAXParser {
        Self::new_4(symbol_table, None)
    }

    fn new_4(symbol_table: Option<String>, grammar_pool: Option<String>) -> SAXParser {
        let config = Rc::new(XIncludeAwareParserConfiguration {});
        let parser = SAXParser::new_1(config.clone());

        config.add_recognized_features(Self::RECOGNIZED_FEATURES);
        config.set_feature(Self::NOTIFY_BUILTIN_REFS, true);

        config.add_recognized_properties(Self::RECOGNIZED_PROPERTIES);
        if let Some(table) = symbol_table {
            config.set_property(Self::SYMBOL_TABLE, table);
        }
        if let Some(pool) = grammar_pool {
            config.set_property(Self::XMLGRAMMAR_POOL, pool);
        }
        parser
    }

    fn set_property(&self, name: &'static str, value: String) {
        if name == Constants::SECURITY_MANAGER {
            todo!()
        }
    }

}
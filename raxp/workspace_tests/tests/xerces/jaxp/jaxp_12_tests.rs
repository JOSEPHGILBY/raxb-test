use javax_xml::parsers::{sax_parser::SAXParser, sax_parser_factory::SAXParserFactory};

struct JAXP12Tests {

    sax_factory: Box<dyn SAXParserFactory>,
    sax_parser_ns_aware: Box<dyn SAXParser>,
    sax_parser_validating: Box<dyn SAXParser>,
}

#[fixture]
fn parser_provider() -> Box<dyn SAXParser> {
    let spf = SAXParserFactoryAbstractItems::new_1();
    spf.new_sax_parser()
}

#[test]
fn test_sax_parse_xsd() {

}
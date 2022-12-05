//jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/SAXParser.java
pub struct SAXParser {

}

impl SAXParser {

    pub fn reset(){}
    // HandlerBase is deprecated, only use default handler
    //pub fn parse_with_input_stream_handler_base(istr: String, hb: String) {(iso, hb)}
    //pub fn parse_with_input_stream_handler_base_system_id(istr: String, hb: String, system_id: String) {(iso, hb)}
    pub fn parse_with_input_stream_default_handler(istr: String, dh: String, system_id: String) { // (iso, dh)
    }
    pub fn parse_with_input_stream_default_handler_system_id(istr: String, dh: String, system_id: String) { // (iso, dh)

    }
    // pub fn parse_with_uri_handler_base(uri: String, hb: String) {(iso, hb)}
    pub fn parse_with_uri_default_handler(uri: String, dh: String) { // (iso, dh)

    }
    // pub fn parse_with_file_handler_base(f: String, hb: String) {(iso, hb)}
    pub fn parse_with_file_default_handler(uri: String, dh: String) { // (iso, dh)

    }
    // pub fn parse_with_input_source_handler_base(iso: String, hb: String) {get_parser.parse(iso)}
    pub fn parse_with_input_source_default_handler(iso: String, dh: String) { // get_xml_reader.parse(iso)

    }

    pub fn test(&self) -> i32 {
        27
    }
}


// jdk/test/jaxp/javax/xml/jaxp/unittest/sax/SAXParserTest.java
#[cfg(test)]
mod tests {
    use crate::sax_parser_factory::SAXParserFactory;
    use tempfile::{Builder, NamedTempFile};
    use std::io::Write;
    use rstest::{rstest, fixture};
    use rstest_reuse::{template, apply};

    use super::SAXParser;
    // jdk/test/jaxp/javax/xml/jaxp/unittest/sax/SAXParserTest.java
    

    struct testStruct {
        a: String
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_close_readers() {
        let factory = SAXParserFactory::new_default_instance();
        let parser = factory.new_sax_parser();
        let file_and_path = create_test_file();
        //parser.parse(file_and_path.)
    }

    fn create_test_file(name: String) -> NamedTempFile {
        let mut tmp = Builder::new().prefix(&name).suffix(".xml").tempfile().unwrap();
        tmp.write("<?xml version=\"1.0\"?><test a1=\"x\" a2=\"y\"/>".as_bytes());
        tmp
    }

    //jdk/test/jaxp/javax/xml/jaxp/functional/javax/xml/parsers/ptests/SAXParserTest.java
    #[fixture]
    fn parser_provider() -> SAXParser {
        let spf = SAXParserFactory::new_instance();
        spf.new_sax_parser()
    }

    #[rstest]
    fn test_parse_01(
        #[from(parser_provider)]
        sax_parser: SAXParser
    ) {
        assert_eq!(sax_parser.test(), 29);
    }

    // additional unit tests
    #[test]
    fn test_sax_parser() {
        let factory = SAXParserFactory::new_default_instance();
        let parser = factory.new_sax_parser();
        let file_and_path = create_test_file("Test".to_string());
        // parse file_and_path.as_file()
    }

}

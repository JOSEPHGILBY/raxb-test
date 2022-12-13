// jdk/src/java.xml/share/classes/javax/xml/parsers/SAXParser.java

use std::rc::Rc;

use org_sax::{input_source::InputSource, helpers::default_handler::DefaultHandler, xml_reader::XMLReader};

pub trait SAXParser {
    fn reset(&self);
    // HandlerBase is deprecated, skip all with it
    fn parse_with_input_stream(&self, istr: String, dh: Option<DefaultHandler>);
    fn parse_with_input_stream_system_id(&self, istr: String, dh: Option<DefaultHandler>, system_id: String);
    fn parse_with_uri(&self, uri: String, dh: Option<DefaultHandler>);
    fn parse_with_file(&self, file: String, dh: Option<DefaultHandler>);
    fn parse_with_input_source(&self, iso: InputSource, dh: Option<DefaultHandler>);
    // get_parser parser is deprecated, use xml_reader
    fn get_xml_reader(&self) -> Box<dyn XMLReader>;
    fn is_namespace_aware(&self) -> bool;
    fn is_validating(&self) -> bool;
    fn set_property(&self, name: &'static str, value: String);
    fn get_property(&self, name: &'static str);
    fn get_schema(&self) -> String; // TODO:
    fn is_x_include_aware(&self) -> bool;
}
pub struct SAXParserAbstractItems {

}

impl SAXParserAbstractItems {

    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self) -> i32 {
        27
    }
}

impl SAXParser for SAXParserAbstractItems {

    fn reset(&self) {

    }

    fn parse_with_input_stream(&self, istr: String, dh: Option<DefaultHandler>) {
        let input = InputSource::new_with_byte_stream(istr);
        self.parse_with_input_source(input, dh);
    }

    fn parse_with_input_stream_system_id(&self, istr: String, dh: Option<DefaultHandler>, system_id: String) {
        let mut input = InputSource::new_with_byte_stream(istr);
        input.system_id = system_id;
        self.parse_with_input_source(input, dh);

    }

    fn parse_with_uri(&self, uri: String, dh: Option<DefaultHandler>) {
        let input = InputSource::new_with_system_id(uri);
        self.parse_with_input_source(input, dh);
    }

    fn parse_with_file(&self, file: String, dh: Option<DefaultHandler>) {
        todo!()
    }

    fn parse_with_input_source(&self, iso: InputSource, dh: Option<DefaultHandler>) {
        let reader = self.get_xml_reader();
        if let Some(dh) = dh {
            let dyn_dh = Rc::new(dh);
            reader.set_content_handler(dyn_dh.clone());
            reader.set_entity_resolver(dyn_dh.clone());
            reader.set_error_handler(dyn_dh.clone());
            reader.set_dtd_handler(dyn_dh);
        }
        reader.parse_from_input_source(iso);
    }

    fn get_xml_reader(&self) -> Box<dyn XMLReader> {
        todo!()
    }

    fn is_namespace_aware(&self) -> bool {
        todo!()
    }

    fn is_validating(&self) -> bool {
        todo!()
    }

    fn set_property(&self, name: &'static str, value: String) {
        todo!()
    }

    fn get_property(&self, name: &'static str) {
        todo!()
    }

    fn get_schema(&self) -> String {
        todo!()
    }

    fn is_x_include_aware(&self) -> bool {
        todo!()
    }


}
// jdk/test/jaxp/javax/xml/jaxp/unittest/sax/SAXParserTest.java
#[cfg(test)]
mod tests {
    use org_sax::helpers::default_handler::DefaultHandler;
    use tempfile::{Builder, NamedTempFile};
    use std::io::Write;
    use rstest::{rstest, fixture};
    use rstest_reuse::{template, apply};

    use crate::parsers::sax_parser_factory::SAXParserFactoryAbstractItems;

    use super::SAXParser;
    // jdk/test/jaxp/javax/xml/jaxp/unittest/sax/SAXParserTest.java
    

    struct testStruct {
        a: String
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_close_readers() {
        //let factory = AbstractSAXParserFactory::new_default_instance();
        //let parser = factory.new_sax_parser();
        //let file_and_path = create_test_file();
        //parser.parse(file_and_path.)
    }

    fn create_test_file(name: String) -> NamedTempFile {
        let mut tmp = Builder::new().prefix(&name).suffix(".xml").tempfile().unwrap();
        tmp.write("<?xml version=\"1.0\"?><test a1=\"x\" a2=\"y\"/>".as_bytes());
        tmp
    }

    //jdk/test/jaxp/javax/xml/jaxp/functional/javax/xml/parsers/ptests/SAXParserTest.java
    #[fixture]
    fn parser_provider() -> Box<dyn SAXParser> {
        let spf = SAXParserFactoryAbstractItems::new_1();
        spf.new_sax_parser()
    }
    #[rstest]
    #[ignore = "no need for null check in rust"]
    fn test_parse_01(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // FileInputStream instream = null;
        // saxparser.parse(instream, new HandlerBase());
    }

    #[rstest]
    #[ignore = "no need for null check in rust"]
    fn test_parse_02(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // String uri = null;
        // saxparser.parse(uri, new HandlerBase());
    }

    #[rstest]
    #[ignore = "HandlerBase is deprecated and not included in rust distribution"]
    fn test_parse_03(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // saxparser.parse("", new HandlerBase());
    }

    #[rstest]
    #[ignore = "no need for null check in rust"]
    fn test_parse_04(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // File file = null;
        // saxparser.parse(file, new HandlerBase());
    }

    #[rstest]
    fn test_parse_05(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // tryRunWithTmpPermission(() -> saxparser.parse(new File(""), new HandlerBase()), new PropertyPermission("user.dir", "read"));
    }

    #[rstest]
    #[ignore = "no need for null check in rust"]
    fn test_parse_06(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // InputSource is = null;
        // saxparser.parse(is, new HandlerBase());
    }

    #[rstest]
    #[ignore = "no need for null check in rust"]
    fn test_parse_07(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // FileInputStream instream = null;
        // saxparser.parse(instream, new DefaultHandler());
    }

    #[rstest]
    #[ignore = "no need for null check in rust"]
    fn test_parse_08(
        #[from(parser_provider)]
        _sax_parser: Box<dyn SAXParser>
    ) {
        // String uri = null;
        // saxparser.parse(uri, new DefaultHandler());
    }

    #[rstest]
    fn test_parse_09(
        #[from(parser_provider)]
        sax_parser: Box<dyn SAXParser>
    ) {
        sax_parser.parse_with_uri("no-such-file".to_string(), Some(DefaultHandler::new()))
    }

    #[rstest]
    fn test_parse_10(
        #[from(parser_provider)]
        sax_parser: Box<dyn SAXParser>
    ) {
        //sax_parser.parse(iso, dh)
    }




    // additional unit tests
    #[test]
    fn test_sax_parser() {
        //let factory = SAXParserFactory::new_default_instance();
        //let parser = factory.new_sax_parser();
        //let file_and_path = create_test_file("Test".to_string());
        // parse file_and_path.as_file()
    }

}
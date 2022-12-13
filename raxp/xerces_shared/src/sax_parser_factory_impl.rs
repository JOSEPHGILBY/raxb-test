
// use crate::impl_constants::ImplConstants;
// use crate::combine;
// use crate::abstract_sax_parser::SAXParser;
// use crate::abstract_sax_parser_factory::SAXParserFactory;
// use crate::sax_parser_impl::SAXParserImpl;

// use std::collections::HashMap;

// //jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl.java
// #[derive(Default)]
// pub struct SAXParserFactoryImpl {
//     features: HashMap<String, bool>,
//     // TODO: Schema type
//     grammar: String, // Schema
//     is_xinclude_aware: bool,
//     f_secure_process: bool,
// }

// impl SAXParserFactoryImpl {
//     const VALIDATION_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::VALIDATION_FEATURE);
//     const NAMESPACES_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::NAMESPACES_FEATURE);
//     const XINCLUDE_FEATURE: &str = combine!(ImplConstants::XERCES_FEATURE_PREFIX, ImplConstants::XINCLUDE_FEATURE);

//     pub fn new() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }
// }

// impl SAXParserFactory for SAXParserFactoryImpl {
//     fn new_sax_parser(&self) -> Box<dyn SAXParser> {
//         Box::new(SAXParserImpl::new_with_secure_processing(self, &self.features, self.f_secure_process))
//     }
//     fn set_namespace_aware(awareness: bool) {
//         todo!()
//     }

//     fn set_validating(validating: bool) {
//         todo!()
//     }

//     fn is_namespace_aware() -> bool {
//         todo!()
//     }

//     fn is_validating() -> bool {
//         todo!()
//     }

//     fn set_feature(name: String, value: bool) {
//         todo!()
//     }

//     fn get_feature(name: String) -> bool {
//         todo!()
//     }

//     fn get_schema() -> String {
//         todo!()
//     }

//     fn set_schema(schema: String) {
//         todo!()
//     }

//     fn set_xinclude_aware(state: bool) {
//         todo!()
//     }

//     fn is_xinclude_aware() -> bool {
//         todo!()
//     }
// }
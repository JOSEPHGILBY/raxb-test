use crate::combine;
use crate::impl_constants::ImplConstants;
use crate::xml_input_source::XMLInputSource;
use crate::xml_parser_configuration::XMLParserConfiguration;
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/parsers/XMLParser.java

pub struct XMLParser<C: XMLParserConfiguration> {
    // TODO: 
    f_configuration: C,
    // TODO:
    security_manager: String,
    // TODO:
    security_property_manager: String,
}

impl<C: XMLParserConfiguration> XMLParser<C> {

    //
    // Constants
    // 
    pub const ENTITY_RESOLVER: &str = combine!(ImplConstants::XERCES_PROPERTY_PREFIX, ImplConstants::ENTITY_RESOLVER_PROPERTY);
    pub const ERROR_HANDLER: &str = combine!(ImplConstants::XERCES_PROPERTY_PREFIX, ImplConstants::ERROR_HANDLER_PROPERTY);
    const RECOGNIZED_PROPERTIES: &'static [&'static str] = &[
        XMLParser::<C>::ENTITY_RESOLVER,
        XMLParser::<C>::ERROR_HANDLER,
    ];

    // TODO:
    pub fn get_feature(&self, feature_id: String) -> bool {
        false
        // return fConfiguration.getFeature(featureId);
    }

    pub fn new(config: C) -> XMLParser<C> {
        let to_return = XMLParser {
            f_configuration: config,
            security_manager: "".to_string(),
            security_property_manager: "".to_string(),
        };
        to_return.f_configuration.add_recognized_properties(XMLParser::<C>::RECOGNIZED_PROPERTIES);
        to_return
    }

    pub fn parse(&self, input_source: XMLInputSource) {
        // if (securityManager == null) {
        //     securityManager = new XMLSecurityManager(true);
        //     fConfiguration.setProperty(Constants.SECURITY_MANAGER, securityManager);
        // }
        // if (securityPropertyManager == null) {
        //     securityPropertyManager = new XMLSecurityPropertyManager();
        //     fConfiguration.setProperty(JdkConstants.XML_SECURITY_PROPERTY_MANAGER, securityPropertyManager);
        // }
        self.reset();
        self.f_configuration.parse(input_source);
    }

    pub fn reset(&self) {

    }

}
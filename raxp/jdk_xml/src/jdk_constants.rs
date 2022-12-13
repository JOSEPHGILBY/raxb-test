use const_format::concatcp;



// jdk/src/java.xml/share/classes/jdk/xml/internal/JdkConstants.java
pub struct JdkConstants {}
// TODO: rename or refactor or something
impl JdkConstants {

    //
    // Constants
    //
    //Xerces security manager
    pub const SECURITY_MANAGER: &str = "http://apache.org/xml/properties/security-manager";

    //
    // Implementation limits: API properties
    //
    // TODO: deprecation warning
    pub const ORACLE_JAXP_PROPERTY_PREFIX: &str = "http://www.oracle.com/xml/jaxp/properties/";
    // TODO: deprecation warning
    pub const JDK_ENTITY_EXPANSION_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "entityExpansionLimit");
    // TODO: deprecation warning
    pub const JDK_ELEMENT_ATTRIBUTE_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "elementAttributeLimit");
    // TODO: deprecation warning
    pub const JDK_MAX_OCCUR_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "maxOccurLimit");
    // TODO: deprecation warning
    pub const JDK_TOTAL_ENTITY_SIZE_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "totalEntitySizeLimit");
    // TODO: deprecation warning
    pub const JDK_GENERAL_ENTITY_SIZE_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "maxGeneralEntitySizeLimit");
    // TODO: deprecation warning
    pub const JDK_ENTITY_REPLACEMENT_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "entityReplacementLimit");
    // TODO: deprecation warning
    pub const JDK_PARAMETER_ENTITY_SIZE_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "maxParameterEntitySizeLimit");
    // TODO: deprecation warning
    pub const JDK_XML_NAME_LIMIT: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "maxXMLNameLimit");
    // TODO: deprecation warning
    pub const JDK_MAX_ELEMENT_DEPTH: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "maxElementDepth");
    // TODO: deprecation warning
    pub const JDK_ENTITY_COUNT_INFO: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "getEntityCountInfo");
    pub const JDK_DEBUG_LIMIT: &str = "jdk.xml.getEntityCountInfo";
    pub const SP_ENTITY_EXPANSION_LIMIT: &str = "jdk.xml.entityExpansionLimit";
    pub const SP_ELEMENT_ATTRIBUTE_LIMIT: &str = "jdk.xml.elementAttributeLimit";
    pub const SP_MAX_OCCUR_LIMIT: &str = "jdk.xml.maxOccurLimit";
    pub const SP_TOTAL_ENTITY_SIZE_LIMIT: &str = "jdk.xml.totalEntitySizeLimit";
    pub const SP_GENERAL_ENTITY_SIZE_LIMIT: &str = "jdk.xml.maxGeneralEntitySizeLimit";
    pub const SP_ENTITY_REPLACEMENT_LIMIT: &str = "jdk.xml.entityReplacementLimit";
    pub const SP_PARAMETER_ENTITY_SIZE_LIMIT: &str = "jdk.xml.maxParameterEntitySizeLimit";
    pub const SP_XML_NAME_LIMIT: &str = "jdk.xml.maxXMLNameLimit";
    pub const SP_MAX_ELEMENT_DEPTH: &str = "jdk.xml.maxElementDepth";
    pub const XPATH_GROUP_LIMIT: &str = "jdk.xml.xpathExprGrpLimit";
    pub const XPATH_OP_LIMIT: &str = "jdk.xml.xpathExprOpLimit";
    pub const XPATH_TOTALOP_LIMIT: &str = "jdk.xml.xpathTotalOpLimit";
    pub const JDK_EXTENSION_CLASSLOADER: &str = "jdk.xml.transform.extensionClassLoader";
    pub const JDK_EXT_CLASSLOADER: &str = "jdk.xml.extensionClassLoader";
    // legacy system properties
    pub const ENTITY_EXPANSION_LIMIT: &str = "entityExpansionLimit";
    pub const ELEMENT_ATTRIBUTE_LIMIT: &str = "elementAttributeLimit";
    pub const MAX_OCCUR_LIMIT: &str = "maxOccurLimit";
    pub const JDK_YES: &str = "yes";
    // TODO: deprecation warning
    pub const ORACLE_FEATURE_SERVICE_MECHANISM: &str = "http://www.oracle.com/feature/use-service-mechanism";

    // System Properties corresponding to ACCESS_EXTERNAL_* properties
    pub const SP_ACCESS_EXTERNAL_STYLESHEET: &str = "javax.xml.accessExternalStylesheet";
    pub const SP_ACCESS_EXTERNAL_DTD: &str = "javax.xml.accessExternalDTD";
    pub const SP_ACCESS_EXTERNAL_SCHEMA: &str = "javax.xml.accessExternalSchema";


    // all access keyword
    pub const ACCESS_EXTERNAL_ALL: &str = "all";

    pub const EXTERNAL_ACCESS_DEFAULT_FSP: &str = "";
    pub const EXTERNAL_ACCESS_DEFAULT: &str = JdkConstants::ACCESS_EXTERNAL_ALL;
    pub const XML_SECURITY_PROPERTY_MANAGER: &str = "jdk.xml.xmlSecurityPropertyManager";

    pub const FEATURE_TRUE: &str = "true";
    pub const FEATURE_FALSE: &str = "false";

    pub const S_IS_STANDALONE: &str = "isStandalone";

    // TODO: deprecation warning
    pub const FQ_IS_STANDALONE: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, JdkConstants::S_IS_STANDALONE);

    pub const SP_IS_STANDALONE: &str = "jdk.xml.isStandalone";

    // TODO: deprecation warning
    pub const ORACLE_IS_STANDALONE: &str = "http://www.oracle.com/xml/is-standalone";
    // TODO: deprecation warning
    pub const JDK_IS_STANDALONE: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "xsltcIsStandalone");
    // TODO: deprecation warning
    pub const SP_XSLTC_IS_STANDALONE: &str = "jdk.xml.xsltcIsStandalone";
    pub const ORACLE_ENABLE_EXTENSION_FUNCTION: &str = concatcp!(JdkConstants::ORACLE_JAXP_PROPERTY_PREFIX, "enableExtensionFunctions");
    pub const SP_ENABLE_EXTENSION_FUNCTION: &str = "javax.xml.enableExtensionFunctions";
    pub const SP_ENABLE_EXTENSION_FUNCTION_SPEC: &str = "jdk.xml.enableExtensionFunctions";
    pub const RESET_SYMBOL_TABLE: &str = "jdk.xml.resetSymbolTable";
    // TODO: 'getjaxsystemproperty'
    pub const RESET_SYMBOL_TABLE_DEFAULT: bool = false;
    pub const OVERRIDE_PARSER: &str = "jdk.xml.overrideDefaultParser";
    // TODO: 'getjaxsystemproperty'
    pub const OVERRIDE_PARSER_DEFAULT: bool = false;
    pub const CDATA_CHUNK_SIZE: &str = "jdk.xml.cdataChunkSize";
    // TODO: 'getjaxsystemproperty'
    pub const CDATA_CHUNK_SIZE_DEFAULT: i32 = 0;
}
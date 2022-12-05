use crate::combine;
pub struct ImplConstants {}


//jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/impl/Constants.java
impl ImplConstants {

    //
    // Constants
    //
    // Schema Types: 
    //TODO: intern?
    pub const NS_XMLSCHEMA: &str = "http://www.w3.org/2001/XMLSchema";
    //TODO: intern?
    pub const NS_DTD: &str = "http://www.w3.org/TR/REC-xml";
    
    // Schema Versions:
    //TODO: intern?
    pub const W3C_XML_SCHEMA10_NS_URI: &str = "http://www.w3.org/XML/XMLSchema/v1.0";
    
    // Schema features:
    pub const SUN_SCHEMA_FEATURE_PREFIX: &str = "http://java.sun.com/xml/schema/features/";
    pub const SUN_REPORT_IGNORED_ELEMENT_CONTENT_WHITESPACE: &str = "report-ignored-element-content-whitespace";
    
    // stax properties: 
    pub const ZEPHYR_PROPERTY_PREFIX: &str = "http://java.sun.com/xml/stream/properties/";
    pub const STAX_PROPERTIES: &str = "stax-properties";
    pub const STAX_ENTITY_RESOLVER_PROPERTY: &str = "internal/stax-entity-resolver";
    pub const STAX_REPORT_CDATA_EVENT: &str = "report-cdata-event";
    pub const READER_IN_DEFINED_STATE: &str = combine!(ImplConstants::ZEPHYR_PROPERTY_PREFIX, "reader-in-defined-state");
    pub const ADD_NAMESPACE_DECL_AS_ATTRIBUTE: &str = "add-namespacedecl-as-attrbiute";
    pub const ESCAPE_CHARACTERS: &str = "escapeCharacters";
    pub const REUSE_INSTANCE: &str = "reuse-instance" ;
    
    // DOM properties
    pub const SUN_DOM_PROPERTY_PREFIX: &str = "http://java.sun.com/xml/dom/properties/";
    pub const SUN_DOM_ANCESTOR_CHECCK: &str = "ancestor-check";
    pub const IGNORE_EXTERNAL_DTD: &str = "ignore-external-dtd";

    // sax features
    pub const SAX_FEATURE_PREFIX: &str = "http://xml.org/sax/features/";
    pub const NAMESPACES_FEATURE: &str = "namespaces";
    pub const NAMESPACE_PREFIXES_FEATURE: &str = "namespace-prefixes";
    pub const STRING_INTERNING_FEATURE: &str = "string-interning";
    pub const VALIDATION_FEATURE: &str = "validation";
    pub const EXTERNAL_GENERAL_ENTITIES_FEATURE: &str = "external-general-entities";
    pub const EXTERNAL_PARAMETER_ENTITIES_FEATURE: &str = "external-parameter-entities";
    pub const LEXICAL_HANDLER_PARAMETER_ENTITIES_FEATURE: &str = "lexical-handler/parameter-entities";
    pub const IS_STANDALONE_FEATURE: &str = "is-standalone";
    pub const RESOLVE_DTD_URIS_FEATURE: &str = "resolve-dtd-uris";
    pub const USE_ATTRIBUTES2_FEATURE: &str = "use-attributes2";
    pub const USE_LOCATOR2_FEATURE: &str = "use-locator2";
    pub const USE_ENTITY_RESOLVER2_FEATURE: &str = "use-entity-resolver2";
    pub const UNICODE_NORMALIZATION_CHECKING_FEATURE: &str = "unicode-normalization-checking";
    pub const XMLNS_URIS_FEATURE: &str = "xmlns-uris";
    pub const XML_11_FEATURE: &str = "xml-1.1";
    pub const ALLOW_DTD_EVENTS_AFTER_ENDDTD_FEATURE: &str = "allow-dtd-events-after-endDTD";

    // sax properties
    pub const SAX_PROPERTY_PREFIX: &str = "http://xml.org/sax/properties/";
    pub const DECLARATION_HANDLER_PROPERTY: &str = "declaration-handler";
    pub const LEXICAL_HANDLER_PROPERTY: &str = "lexical-handler";
    pub const DOM_NODE_PROPERTY: &str = "dom-node";
    pub const XML_STRING_PROPERTY: &str = "xml-string";
    pub const FEATURE_SECURE_PROCESSING: &str = "http://javax.xml.XMLConstants/feature/secure-processing";
    pub const DOCUMENT_XML_VERSION_PROPERTY: &str = "document-xml-version";

    //
    // R(J)AXP properties
    //

    // TODO: rename
    pub const JAXP_PROPERTY_PREFIX: &str = "http://java.sun.com/xml/jaxp/properties/";
    pub const SCHEMA_SOURCE: &str = "schemaSource";
    pub const SCHEMA_LANGUAGE: &str = "schemaLanguage";
    // TODO: rename
    pub const JAXPAPI_PROPERTY_PREFIX: &str = "http://javax.xml.XMLConstants/property/";

    //
    // DOM features
    //

    pub const INCLUDE_COMMENTS_FEATURE: &str = "include-comments";
    pub const CREATE_CDATA_NODES_FEATURE: &str = "create-cdata-nodes";
    pub const LOAD_AS_INFOSET: &str = "load-as-infoset";

    //
    // Constants: DOM Level 3 feature ids
    //

    pub const DOM_CANONICAL_FORM: &str = "canonical-form";
    pub const DOM_CDATA_SECTIONS: &str = "cdata-sections";
    pub const DOM_COMMENTS: &str = "comments";
    pub const DOM_CHARSET_OVERRIDES_XML_ENCODING: &str = "charset-overrides-xml-encoding";
    pub const DOM_DATATYPE_NORMALIZATION: &str = "datatype-normalization";
    pub const DOM_ENTITIES: &str = "entities";
    pub const DOM_INFOSET: &str = "infoset";
    pub const DOM_NAMESPACES: &str = "namespaces";
    pub const DOM_NAMESPACE_DECLARATIONS: &str = "namespace-declarations";
    pub const DOM_SUPPORTED_MEDIATYPES_ONLY: &str = "supported-media-types-only";
    pub const DOM_VALIDATE_IF_SCHEMA: &str = "validate-if-schema";
    pub const DOM_VALIDATE: &str = "validate";
    pub const DOM_ELEMENT_CONTENT_WHITESPACE: &str = "element-content-whitespace";

    // DOM Level 3 features defined in Core:
    pub const DOM_DISCARD_DEFAULT_CONTENT: &str = "discard-default-content";
    pub const DOM_NORMALIZE_CHARACTERS: &str = "normalize-characters";
    pub const DOM_CHECK_CHAR_NORMALIZATION: &str = "check-character-normalization";
    pub const DOM_WELLFORMED: &str = "well-formed";
    pub const DOM_SPLIT_CDATA: &str = "split-cdata-sections";

    // Load and Save
    pub const DOM_FORMAT_PRETTY_PRINT: &str = "format-pretty-print";
    pub const DOM_XMLDECL: &str = "xml-declaration";
    pub const DOM_UNKNOWNCHARS: &str = "unknown-characters";
    pub const DOM_CERTIFIED: &str = "certified";
    pub const DOM_DISALLOW_DOCTYPE: &str = "disallow-doctype";
    pub const DOM_IGNORE_UNKNOWN_CHARACTER_DENORMALIZATIONS: &str = "ignore-unknown-character-denormalizations";

    // DOM properties
    pub const DOM_RESOURCE_RESOLVER: &str = "resource-resolver";
    pub const DOM_ERROR_HANDLER: &str = "error-handler";
    pub const DOM_SCHEMA_TYPE: &str = "schema-type";
    pub const DOM_SCHEMA_LOCATION: &str = "schema-location";
    pub const DOM_ANCESTOR_CHECCK: &str = "ancestor-check";

    // XSModel
    pub const DOM_PSVI: &str = "psvi";

    // xerces features
    pub const XERCES_FEATURE_PREFIX: &str = "http://apache.org/xml/features/";
    pub const SCHEMA_VALIDATION_FEATURE: &str = "validation/schema";
    pub const SCHEMA_NORMALIZED_VALUE: &str = "validation/schema/normalized-value";
    pub const SCHEMA_ELEMENT_DEFAULT: &str = "validation/schema/element-default";
    pub const SCHEMA_FULL_CHECKING: &str = "validation/schema-full-checking";
    pub const SCHEMA_AUGMENT_PSVI: &str = "validation/schema/augment-psvi";
    pub const DYNAMIC_VALIDATION_FEATURE: &str = "validation/dynamic";
    pub const WARN_ON_DUPLICATE_ATTDEF_FEATURE: &str = "validation/warn-on-duplicate-attdef";
    pub const WARN_ON_UNDECLARED_ELEMDEF_FEATURE: &str = "validation/warn-on-undeclared-elemdef";
    pub const WARN_ON_DUPLICATE_ENTITYDEF_FEATURE: &str = "warn-on-duplicate-entitydef";
    // TODO: 'rust' encodings?
    pub const ALLOW_JAVA_ENCODINGS_FEATURE: &str = "allow-java-encodings";
    pub const DISALLOW_DOCTYPE_DECL_FEATURE: &str = "disallow-doctype-decl";
    pub const CONTINUE_AFTER_FATAL_ERROR_FEATURE: &str = "continue-after-fatal-error";
    pub const LOAD_DTD_GRAMMAR_FEATURE: &str = "nonvalidating/load-dtd-grammar";
    pub const LOAD_EXTERNAL_DTD_FEATURE: &str = "nonvalidating/load-external-dtd";
    pub const DEFER_NODE_EXPANSION_FEATURE: &str = "dom/defer-node-expansion";
    pub const CREATE_ENTITY_REF_NODES_FEATURE: &str = "dom/create-entity-ref-nodes";
    pub const INCLUDE_IGNORABLE_WHITESPACE: &str = "dom/include-ignorable-whitespace";
    pub const DEFAULT_ATTRIBUTE_VALUES_FEATURE: &str = "validation/default-attribute-values";
    pub const VALIDATE_CONTENT_MODELS_FEATURE: &str = "validation/validate-content-models";
    pub const VALIDATE_DATATYPES_FEATURE: &str = "validation/validate-datatypes";
    pub const BALANCE_SYNTAX_TREES: &str = "validation/balance-syntax-trees";
    pub const NOTIFY_CHAR_REFS_FEATURE: &str = "scanner/notify-char-refs";
    pub const NOTIFY_BUILTIN_REFS_FEATURE: &str = "scanner/notify-builtin-refs";
    pub const STANDARD_URI_CONFORMANT_FEATURE: &str = "standard-uri-conformant";
    pub const GENERATE_SYNTHETIC_ANNOTATIONS_FEATURE: &str = "generate-synthetic-annotations";
    pub const VALIDATE_ANNOTATIONS_FEATURE: &str = "validate-annotations";
    pub const HONOUR_ALL_SCHEMALOCATIONS_FEATURE: &str = "honour-all-schemaLocations";
    pub const NAMESPACE_GROWTH_FEATURE: &str = "namespace-growth";
    pub const TOLERATE_DUPLICATES_FEATURE: &str = "internal/tolerate-duplicates";
    pub const XINCLUDE_FEATURE: &str = "xinclude";
    pub const XINCLUDE_FIXUP_BASE_URIS_FEATURE: &str = "xinclude/fixup-base-uris";
    pub const XINCLUDE_FIXUP_LANGUAGE_FEATURE: &str = "xinclude/fixup-language";
    pub const IGNORE_XSI_TYPE_FEATURE: &str = "validation/schema/ignore-xsi-type-until-elemdecl";
    pub const ID_IDREF_CHECKING_FEATURE: &str = "validation/id-idref-checking";
    pub const IDC_CHECKING_FEATURE: &str = "validation/identity-constraint-checking";
    pub const UNPARSED_ENTITY_CHECKING_FEATURE: &str = "validation/unparsed-entity-checking";
    pub const USE_GRAMMAR_POOL_ONLY_FEATURE: &str = "internal/validation/schema/use-grammar-pool-only";
    pub const PARSER_SETTINGS: &str = "internal/parser-settings";
    pub const XINCLUDE_AWARE: &str = "xinclude-aware";
    pub const IGNORE_SCHEMA_LOCATION_HINTS: &str = "validation/schema/ignore-schema-location-hints";
    pub const CHANGE_IGNORABLE_CHARACTERS_INTO_IGNORABLE_WHITESPACES: &str = "validation/change-ignorable-characters-into-ignorable-whitespaces";
    pub const XERCES_PROPERTY_PREFIX: &str = "http://apache.org/xml/properties/";
    pub const CURRENT_ELEMENT_NODE_PROPERTY: &str = "dom/current-element-node";
    pub const DOCUMENT_CLASS_NAME_PROPERTY: &str = "dom/document-class-name";
    pub const SYMBOL_TABLE_PROPERTY: &str = "internal/symbol-table";
    pub const ERROR_REPORTER_PROPERTY: &str = "internal/error-reporter";
    pub const ERROR_HANDLER_PROPERTY: &str = "internal/error-handler";
    pub const XINCLUDE_HANDLER_PROPERTY: &str = "internal/xinclude-handler";
    pub const XPOINTER_HANDLER_PROPERTY: &str = "internal/xpointer-handler";
    pub const ENTITY_MANAGER_PROPERTY: &str = "internal/entity-manager";
    pub const BUFFER_SIZE_PROPERTY: &str = "input-buffer-size";
    pub const SECURITY_MANAGER_PROPERTY: &str = "security-manager";
    pub const LOCALE_PROPERTY: &str = "locale";
    pub const SECURITY_MANAGER: &str = combine!(ImplConstants::XERCES_PROPERTY_PREFIX, ImplConstants::SECURITY_MANAGER_PROPERTY);
    pub const ENTITY_RESOLVER_PROPERTY: &str = "internal/entity-resolver";
    pub const XMLGRAMMAR_POOL_PROPERTY: &str = "internal/grammar-pool";
    pub const DATATYPE_VALIDATOR_FACTORY_PROPERTY: &str = "internal/datatype-validator-factory";
    pub const DOCUMENT_SCANNER_PROPERTY: &str = "internal/document-scanner";
    pub const DTD_SCANNER_PROPERTY: &str = "internal/dtd-scanner";
    pub const DTD_PROCESSOR_PROPERTY: &str = "internal/dtd-processor";
    pub const VALIDATOR_PROPERTY: &str = "internal/validator";
    pub const DTD_VALIDATOR_PROPERTY: &str = "internal/validator/dtd";
    pub const SCHEMA_VALIDATOR_PROPERTY: &str = "internal/validator/schema";
    pub const SCHEMA_LOCATION: &str = "schema/external-schemaLocation";
    pub const SCHEMA_NONS_LOCATION: &str = "schema/external-noNamespaceSchemaLocation";
    pub const NAMESPACE_BINDER_PROPERTY: &str = "internal/namespace-binder";
    pub const NAMESPACE_CONTEXT_PROPERTY: &str = "internal/namespace-context";
    pub const VALIDATION_MANAGER_PROPERTY: &str = "internal/validation-manager";
    pub const ROOT_TYPE_DEFINITION_PROPERTY: &str = "validation/schema/root-type-definition";
    pub const ROOT_ELEMENT_DECLARATION_PROPERTY: &str = "validation/schema/root-element-declaration";
    pub const XPOINTER_SCHEMA_PROPERTY: &str = "xpointer-schema";
    pub const SCHEMA_DV_FACTORY_PROPERTY: &str = "internal/validation/schema/dv-factory";

    // general constants
    pub const ELEMENT_PSVI: &str = "ELEMENT_PSVI";
    pub const ATTRIBUTE_PSVI: &str = "ATTRIBUTE_PSVI";
    pub const ATTRIBUTE_DECLARED: &str = "ATTRIBUTE_DECLARED";
    pub const TYPEINFO: &str = "org.w3c.dom.TypeInfo";
    pub const ID_ATTRIBUTE: &str = "ID_ATTRIBUTE";

    // XML version constants
    pub const ENTITY_SKIPPED: &str = "ENTITY_SKIPPED";
    pub const CHAR_REF_PROBABLE_WS: &str = "CHAR_REF_PROBABLE_WS";
    pub const LAST_ENTITY: &str = "LAST_ENTITY";
    pub const XML_VERSION_ERROR: i16 = -1;
    pub const XML_VERSION_1_0: i16 = 1;
    pub const XML_VERSION_1_1: i16 = 2;

    // DOM related constants
    pub const ANONYMOUS_TYPE_NAMESPACE: &str = "http://apache.org/xml/xmlschema/1.0/anonymousTypes";

    // constants to enable Schema 1.1 support
    pub const SCHEMA_1_1_SUPPORT: bool = false;
    pub const SCHEMA_VERSION_1_0: i16 = 1;
    pub const SCHEMA_VERSION_1_0_EXTENDED: i16 = 2;

    // private 
    const fg_SAX_features: &'static [&'static str] = &[
        ImplConstants::NAMESPACES_FEATURE,
        ImplConstants::NAMESPACE_PREFIXES_FEATURE,
        ImplConstants::STRING_INTERNING_FEATURE,
        ImplConstants::VALIDATION_FEATURE,
        ImplConstants::EXTERNAL_GENERAL_ENTITIES_FEATURE,
        ImplConstants::EXTERNAL_PARAMETER_ENTITIES_FEATURE,
    ];

    const fg_SAX_properties: &'static [&'static str] = &[
        ImplConstants::DECLARATION_HANDLER_PROPERTY,
        ImplConstants::LEXICAL_HANDLER_PROPERTY,
        ImplConstants::DOM_NODE_PROPERTY,
        ImplConstants::XML_STRING_PROPERTY,
    ];

    const fg_xerces_features: &'static [&'static str] = &[
        ImplConstants::SCHEMA_VALIDATION_FEATURE,
        ImplConstants::SCHEMA_FULL_CHECKING,
        ImplConstants::DYNAMIC_VALIDATION_FEATURE,
        ImplConstants::WARN_ON_DUPLICATE_ATTDEF_FEATURE,
        ImplConstants::WARN_ON_UNDECLARED_ELEMDEF_FEATURE,
        ImplConstants::ALLOW_JAVA_ENCODINGS_FEATURE,
        ImplConstants::CONTINUE_AFTER_FATAL_ERROR_FEATURE,
        ImplConstants::LOAD_DTD_GRAMMAR_FEATURE,
        ImplConstants::LOAD_EXTERNAL_DTD_FEATURE,
        //ImplConstants::DEFER_NODE_EXPANSION_FEATURE,
        ImplConstants::CREATE_ENTITY_REF_NODES_FEATURE,
        ImplConstants::XINCLUDE_AWARE,
        ImplConstants::INCLUDE_IGNORABLE_WHITESPACE,
        //ImplConstants::GRAMMAR_ACCESS_FEATURE,
        ImplConstants::DEFAULT_ATTRIBUTE_VALUES_FEATURE,
        ImplConstants::VALIDATE_CONTENT_MODELS_FEATURE,
        ImplConstants::VALIDATE_DATATYPES_FEATURE,
        ImplConstants::BALANCE_SYNTAX_TREES,
        ImplConstants::NOTIFY_CHAR_REFS_FEATURE,
        ImplConstants::NOTIFY_BUILTIN_REFS_FEATURE,
        ImplConstants::DISALLOW_DOCTYPE_DECL_FEATURE,
        ImplConstants::STANDARD_URI_CONFORMANT_FEATURE,
        ImplConstants::GENERATE_SYNTHETIC_ANNOTATIONS_FEATURE,
        ImplConstants::VALIDATE_ANNOTATIONS_FEATURE,
        ImplConstants::HONOUR_ALL_SCHEMALOCATIONS_FEATURE,
        ImplConstants::XINCLUDE_FEATURE,
        ImplConstants::XINCLUDE_FIXUP_BASE_URIS_FEATURE,
        ImplConstants::XINCLUDE_FIXUP_LANGUAGE_FEATURE,
        ImplConstants::IGNORE_XSI_TYPE_FEATURE,
        ImplConstants::ID_IDREF_CHECKING_FEATURE,
        ImplConstants::IDC_CHECKING_FEATURE,
        ImplConstants::UNPARSED_ENTITY_CHECKING_FEATURE,
        ImplConstants::NAMESPACE_GROWTH_FEATURE,
        ImplConstants::TOLERATE_DUPLICATES_FEATURE,
    ];

    const fg_xerces_properties: &'static [&'static str] = &[
        ImplConstants::CURRENT_ELEMENT_NODE_PROPERTY,
        ImplConstants::DOCUMENT_CLASS_NAME_PROPERTY,
        ImplConstants::SYMBOL_TABLE_PROPERTY,
        ImplConstants::ERROR_HANDLER_PROPERTY,
        ImplConstants::ERROR_REPORTER_PROPERTY,
        ImplConstants::ENTITY_MANAGER_PROPERTY,
        ImplConstants::ENTITY_RESOLVER_PROPERTY,
        ImplConstants::XMLGRAMMAR_POOL_PROPERTY,
        ImplConstants::DATATYPE_VALIDATOR_FACTORY_PROPERTY,
        ImplConstants::DOCUMENT_SCANNER_PROPERTY,
        ImplConstants::DTD_SCANNER_PROPERTY,
        ImplConstants::VALIDATOR_PROPERTY,
        ImplConstants::SCHEMA_LOCATION,
        ImplConstants::SCHEMA_NONS_LOCATION,
        ImplConstants::VALIDATION_MANAGER_PROPERTY,
        ImplConstants::BUFFER_SIZE_PROPERTY,
        ImplConstants::SECURITY_MANAGER_PROPERTY,
        ImplConstants::ROOT_TYPE_DEFINITION_PROPERTY,
        ImplConstants::ROOT_ELEMENT_DECLARATION_PROPERTY,
        ImplConstants::LOCALE_PROPERTY,
        ImplConstants::SCHEMA_DV_FACTORY_PROPERTY,
    ];

    // TODO: convert
    // Empty enumeration. /
    //private static final Enumeration<Object> fgEmptyEnumeration = new ArrayEnumeration(new Object[] {});
    
    // TODO: Rust equivalent
    // This class cannot be instantiated. /
    // private Constants() {}

    // TODO: Do we need to convert this and do we need the 'printer'?

    //
    // Public methods
    //

    // // sax

    // /** Returns an enumeration of the SAX features. */
    // public static Enumeration<Object> getSAXFeatures() {
    //     return fgSAXFeatures.length > 0
    //     ? new ArrayEnumeration(fgSAXFeatures) : fgEmptyEnumeration;
    // } // getSAXFeatures():Enumeration

    // /** Returns an enumeration of the SAX properties. */
    // public static Enumeration<Object> getSAXProperties() {
    //     return fgSAXProperties.length > 0
    //     ? new ArrayEnumeration(fgSAXProperties) : fgEmptyEnumeration;
    // } // getSAXProperties():Enumeration

    // // xerces

    // /** Returns an enumeration of the Xerces features. */
    // public static Enumeration<Object> getXercesFeatures() {
    //     return fgXercesFeatures.length > 0
    //     ? new ArrayEnumeration(fgXercesFeatures) : fgEmptyEnumeration;
    // } // getXercesFeatures():Enumeration

    // /** Returns an enumeration of the Xerces properties. */
    // public static Enumeration<Object> getXercesProperties() {
    //     return fgXercesProperties.length > 0
    //     ? new ArrayEnumeration(fgXercesProperties) : fgEmptyEnumeration;
    // } // getXercesProperties():Enumeration

    // //
    // // Classes
    // //

    // /**
    //  * An array enumeration.
    //  *
    //  * @author Andy Clark, IBM
    //  */
    // static class ArrayEnumeration
    // implements Enumeration<Object> {

    //     //
    //     // Data
    //     //

    //     /** Array. */
    //     private Object[] array;

    //     /** Index. */
    //     private int index;

    //     //
    //     // Constructors
    //     //

    //     /** Constructs an array enumeration. */
    //     public ArrayEnumeration(Object[] array) {
    //         this.array = array;
    //     } // <init>(Object[])

    //     //
    //     // Enumeration methods
    //     //

    //     /**
    //      * Tests if this enumeration contains more elements.
    //      *
    //      * @return  <code>true</code> if this enumeration contains more elements;
    //      *          <code>false</code> otherwise.
    //      * @since   JDK1.0
    //      */
    //     public boolean hasMoreElements() {
    //         return index < array.length;
    //     } // hasMoreElement():boolean

    //     /**
    //      * Returns the next element of this enumeration.
    //      *
    //      * @return     the next element of this enumeration.
    //      * @exception  NoSuchElementException  if no more elements exist.
    //      * @since      JDK1.0
    //      */
    //     public Object nextElement() {
    //         if (index < array.length) {
    //             return array[index++];
    //         }
    //         throw new NoSuchElementException();
    //     } // nextElement():Object

    // } // class ArrayEnumeration

    // //
    // // MAIN
    // //

    // /** Prints all of the constants to standard output. */
    // public static void main(String[] argv) {

    //     print("SAX features:", SAX_FEATURE_PREFIX, fgSAXFeatures);
    //     print("SAX properties:", SAX_PROPERTY_PREFIX, fgSAXProperties);
    //     print("Xerces features:", XERCES_FEATURE_PREFIX, fgXercesFeatures);
    //     print("Xerces properties:", XERCES_PROPERTY_PREFIX, fgXercesProperties);

    // } // main(String[])

    // /** Prints a list of features/properties. */
    // private static void print(String header, String prefix, Object[] array) {
    //     System.out.print(header);
    //     if (array.length > 0) {
    //         System.out.println();
    //         for (int i = 0; i < array.length; i++) {
    //             System.out.print("  ");
    //             System.out.print(prefix);
    //             System.out.println(array[i]);
    //         }
    //     }
    //     else {
    //         System.out.println(" none.");
    //     }
    // } // print(String,String,Object[])
}   
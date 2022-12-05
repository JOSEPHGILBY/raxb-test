use crate::impl_constants::ImplConstants;
use crate::jdk_constants::JdkConstants;

macro_rules! combine {
    ($A:expr, $B:expr) => {{
        const A: &str = $A;
        const B: &str = $B;
        const LEN: usize = A.len() + B.len();
        const fn combined() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = copy_slice(A.as_bytes(), out, 0);
            out = copy_slice(B.as_bytes(), out, A.len());
            out
        }
        const fn copy_slice(input: &[u8], mut output: [u8; LEN], offset: usize) -> [u8; LEN] {
            let mut index = 0;
            loop {
                output[offset+index] = input[index];
                index += 1;
                if index == input.len() { break }
            }
            output
        }
        const RESULT: &[u8] = &combined();
        // how bad is the assumption that `&str` and `&[u8]` have the same layout?
        const RESULT_STR: &str = unsafe { std::mem::transmute(RESULT) };
        RESULT_STR
    }}
}

// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/jaxp/SAXParserImpl.java

pub struct SAXParserImpl {
}

impl SAXParserImpl {
    const NAMESPACES_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::NAMESPACES_FEATURE);
    const NAMESPACE_PREFIXES_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::NAMESPACE_PREFIXES_FEATURE);
    const VALIDATION_FEATURE: &str = combine!(ImplConstants::SAX_FEATURE_PREFIX, ImplConstants::VALIDATION_FEATURE);
    const XMLSCHEMA_VALIDATION_FEATURE: &str = combine!(ImplConstants::XERCES_FEATURE_PREFIX , ImplConstants::SCHEMA_VALIDATION_FEATURE);
    const XINCLUDE_FEATURE: &str = combine!(ImplConstants::XERCES_FEATURE_PREFIX , ImplConstants::XINCLUDE_FEATURE);
    const SECURITY_MANAGER: &str = combine!(ImplConstants::XERCES_FEATURE_PREFIX , ImplConstants::SECURITY_MANAGER_PROPERTY);
    const XML_SECURITY_PROPERTY_MANAGER: &str = JdkConstants::XML_SECURITY_PROPERTY_MANAGER;

}

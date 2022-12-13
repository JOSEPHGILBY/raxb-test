// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/NamespaceContext.java

pub trait NamespaceContext {
    // TODO: intern?
    fn xml_uri(&self) -> &'static str {
        "http://www.w3.org/XML/1998/namespace"
    }
    fn xmlns_uri(&self) -> &'static str {
        "http://www.w3.org/2000/xmlns/"
    }
    fn push_context(&self);
    fn pop_context(&self);
    fn declare_prefix(&self, prefix: &'static str, uri: &'static str) -> bool;
    fn get_uri(&self, prefix: &'static str) -> &'static str;
    fn get_prefix(&self, uri: &'static str) -> &'static str;
    fn get_declaration_prefix_count(&self) -> i32;
    fn get_declaration_prefix_at(&self, index: i32) -> &'static str;
    // TODO: Enumeration
    fn get_all_prefixes(&self) -> String;
    fn reset(&self);
}
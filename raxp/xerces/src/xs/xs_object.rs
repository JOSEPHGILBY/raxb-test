// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSObject.java

use super::xs_namespace_item::XSNamespaceItem;

pub trait XSObject {
    fn get_type(&self) -> i16;
    fn get_name(&self) -> &'static str;
    fn get_namespace(&self) -> &'static str;
    fn get_namespace_item(&self) -> Box<dyn XSNamespaceItem>;
}
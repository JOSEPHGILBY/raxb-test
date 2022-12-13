// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSNamespaceItemList.java

use super::xs_namespace_item::XSNamespaceItem;

// TODO: deal with List<XSNamespaceItem>
pub trait XSNamespaceItemList {
    fn get_length(&self) -> i32;
    fn item(&self, index: i32) -> Box<dyn XSNamespaceItem>;
}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSNamedMap.java

use super::xs_object::XSObject;

pub trait XSNamedMap {
    // TODO: extends Map need methods from that delegate

    fn get_length(&self) -> i32;
    fn item(&self, index: i32) -> Box<dyn XSObject>;
    fn item_by_name(&self, namespace: &'static str, local_name: &'static str) -> Box<dyn XSObject>;
}
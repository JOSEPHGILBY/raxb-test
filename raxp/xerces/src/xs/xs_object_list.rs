// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSObjectList.java

use super::xs_object::XSObject;

pub trait XSObjectList {
    // TODO: extends List?

    fn get_length(&self) -> i32;
    fn item(&self, index: i32) -> Box<dyn XSObject>;
}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/StringList.java

pub trait StringList {
    fn get_length(&self) -> i32;
    fn contains(&self, item: &'static str) -> bool;
    fn item(&self, index: i32) -> &'static str;
}

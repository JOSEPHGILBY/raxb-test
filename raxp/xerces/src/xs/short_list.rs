// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/ShortList.java

pub trait ShortList {
    fn get_length(&self) -> i32;
    fn contains(&self, item: i16) -> bool;
    fn item(&self, index: i32) -> i16;
}
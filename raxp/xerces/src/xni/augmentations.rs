// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/Augmentations.java

pub trait Augmentations {
    fn put_item(&self, key: &'static str, item: String) -> String;
    fn get_item(&self, key: &'static str) -> String;
    fn remove_item(&self, key: &'static str) -> String;
    fn keys(&self) -> String;
    fn remove_all_items(&self);
}
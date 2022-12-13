// jdk/src/java.xml/share/classes/org/xml/sax/Locator.java

pub trait Locator {
    fn get_public_id(&self) -> &'static str;
    fn get_system_id(&self) -> &'static str;
    fn get_line_number(&self) -> i32;
    fn get_column_number(&self) -> i32;
}
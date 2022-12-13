// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLLocator.java

pub trait XMLLocator {

    fn get_public_id(&self) -> &'static str;
    fn get_literal_system_id(&self) -> &'static str;
    fn get_base_system_id(&self) -> &'static str;
    fn get_expanded_system_id(&self) -> &'static str;
    fn get_line_number(&self) -> i32;
    fn get_column_number(&self) -> i32;
    fn get_character_offset(&self) -> i32;
    fn get_encoding(&self) -> &'static str;
    fn get_xml_version(&self) -> &'static str;
}
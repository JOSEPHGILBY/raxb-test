// jdk/src/java.xml/share/classes/org/xml/sax/ext/Locator2.java

use crate::locator::Locator;

pub trait Locator2: Locator {
    fn get_xml_version(&self) -> &'static str;
    fn get_encoding(&self) -> &'static str;
}
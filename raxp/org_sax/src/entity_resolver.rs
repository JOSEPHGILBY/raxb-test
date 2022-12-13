// jdk/src/java.xml/share/classes/org/xml/sax/EntityResolver.java

use crate::input_source::InputSource;
pub trait EntityResolver {
    fn resolve_entity(&self, public_id: &str, system_id: &str) -> InputSource;
}
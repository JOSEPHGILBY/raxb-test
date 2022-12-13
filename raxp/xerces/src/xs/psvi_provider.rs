// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/PSVIProvider.java

use super::{element_psvi::ElementPSVI, attribute_psvi::AttributePSVI};

pub trait PSVIProvider {
    fn get_element_psvi(&self) -> Box<dyn ElementPSVI>;
    fn get_attribute_psvi(&self, index: i32) -> Box<dyn AttributePSVI>;
    fn get_attribute_psvi_by_name(&self, uri: &'static str, localname: &'static str) -> Box<dyn AttributePSVI>;

}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSAnnotation.java

use super::xs_object::XSObject;

pub trait XSAnnotation: XSObject {
    fn w3c_dom_element(&self) -> i16 { 1 }
    fn sax_contenthandler(&self) -> i16 { 2 }
    fn w3c_dom_document(&self) -> i16 { 3 }

    fn write_annotation(&self, target: String, target_type: i16) -> bool; // TODO: Object
    fn get_annotation_string(&self) -> &'static str;
}
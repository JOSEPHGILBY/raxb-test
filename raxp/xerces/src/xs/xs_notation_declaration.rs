// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSNotationDeclaration.java

use super::{xs_object::XSObject, xs_annotation::XSAnnotation, xs_object_list::XSObjectList};

pub trait XSNotationDeclaration: XSObject {
    fn get_system_id(&self) -> &'static str;
    fn get_public_id(&self) -> &'static str;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
}
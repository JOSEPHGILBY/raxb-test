// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSAttributeGroupDefinition.java

use super::{xs_object::XSObject, xs_wildcard::XSWildcard, xs_object_list::XSObjectList, xs_annotation::XSAnnotation};

pub trait XSAttributeGroupDefinition: XSObject {
    fn get_attribute_uses(&self) -> Box<dyn XSObjectList>;
    fn get_attribute_wildcard(&self) -> Box<dyn XSWildcard>;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
}
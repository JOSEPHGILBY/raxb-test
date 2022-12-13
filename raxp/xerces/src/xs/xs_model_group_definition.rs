// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSModelGroupDefinition.java

use super::{xs_object::XSObject, xs_model_group::XSModelGroup, xs_annotation::XSAnnotation, xs_object_list::XSObjectList};

pub trait XSModelGroupDefinition: XSObject {

    fn get_model_group(&self) -> Box<dyn XSModelGroup>;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
    
}
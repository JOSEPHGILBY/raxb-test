// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSParticle.java

use super::{xs_object::XSObject, xs_term::XSTerm, xs_object_list::XSObjectList};

pub trait XSParticle: XSObject {
    fn get_min_occurs(&self) -> i32;
    fn get_max_occurs(&self) -> i32;
    fn get_max_occurs_unbounded(&self) -> bool;
    fn get_term(&self) -> Box<dyn XSTerm>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
}
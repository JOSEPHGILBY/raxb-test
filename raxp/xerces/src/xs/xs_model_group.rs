// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSModelGroup.java

use super::{xs_term::XSTerm, xs_object_list::XSObjectList, xs_annotation::XSAnnotation};

pub trait XSModelGroup: XSTerm {
    fn compositor_sequence(&self) -> i16 { 1 }
    fn compositor_choice(&self) -> i16 { 2 }
    fn compositor_all(&self) -> i16 { 3 }

    fn get_compositor(&self) -> i16;
    fn get_particles(&self) -> Box<dyn XSObjectList>;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
}
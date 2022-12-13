// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSWildcard.java

use super::{xs_term::XSTerm, string_list::StringList, xs_annotation::XSAnnotation, xs_object_list::XSObjectList};

pub trait XSWildcard: XSTerm {
    fn nsconstraint_any(&self) -> i16 { 1 }
    fn nsconstraint_not(&self) -> i16 { 2 }
    fn nsconstraint_list(&self) -> i16 { 3 }
    fn pc_strict(&self) -> i16 { 1 }
    fn pc_skip(&self) -> i16 { 2 }
    fn pc_lax(&self) -> i16 { 3 }

    fn get_constraint_type(&self) -> i16;
    fn get_ns_constraint_list(&self) -> Box<dyn StringList>;
    fn get_process_contents(&self) -> i16;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
    
}
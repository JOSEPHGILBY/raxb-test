// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSComplexTypeDefinition.java

use super::{xs_type_definition::XSTypeDefinition, xs_object_list::XSObjectList, xs_wildcard::XSWildcard, xs_simple_type_definition::XSSimpleTypeDefinition, xs_particle::XSParticle};

pub trait XSComplexTypeDefinition: XSTypeDefinition {
    fn contenttype_empty(&self) -> i16 { 0 }
    fn contenttype_simple(&self) -> i16 { 1 }
    fn contenttype_element(&self) -> i16 { 2 }
    fn contenttype_mixed(&self) -> i16 { 3 }
    fn get_derivation_method(&self) -> i16;
    fn get_abstract(&self) -> bool;
    fn get_attribute_uses(&self) -> Box<dyn XSObjectList>;
    fn get_attribute_wildcard(&self) -> Box<dyn XSWildcard>;
    fn get_content_type(&self) -> i16;
    fn get_simple_type(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_particle(&self) -> Box<dyn XSParticle>;
    fn is_prohibited_substitution(&self, restriction: i16) -> bool;
    fn get_prohibited_substitution(&self) -> i16;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
}
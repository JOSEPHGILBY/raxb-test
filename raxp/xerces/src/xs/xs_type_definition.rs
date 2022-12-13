// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSTypeDefinition.java

use super::xs_object::XSObject;

pub trait XSTypeDefinition: XSObject {
    fn complex_type(&self) -> i16 { 15 }
    fn simple_type(&self) -> i16 { 16 }
    
    fn get_type_category(&self) -> i16;
    fn get_base_type(&self) -> Box<dyn XSTypeDefinition>;
    fn is_final(&self, restriction: i16) -> bool;
    fn get_final(&self) -> i16;
    fn get_anonymous(&self) -> bool;
    fn derived_from_type(&self, ancestor_type: Box<dyn XSTypeDefinition>, derivation_method: i16) -> bool;
    fn derived_from(&self, namespace: &'static str, name: &'static str, derivation_method: i16);
}
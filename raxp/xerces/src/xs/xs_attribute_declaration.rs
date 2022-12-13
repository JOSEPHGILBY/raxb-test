// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSAttributeDeclaration.java

use super::{xs_object::XSObject, xs_simple_type_definition::XSSimpleTypeDefinition, xs_complex_type_definition::XSComplexTypeDefinition, xs_value::XSValue, xs_annotation::XSAnnotation, xs_object_list::XSObjectList};

pub trait XSAttributeDeclaration: XSObject {
    fn get_type_definition(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_scope(&self) -> i16;
    fn get_enclosing_ct_definition(&self) -> Box<dyn XSComplexTypeDefinition>;
    fn get_constraint_type(&self) -> i16;
    //fn get_constraint_type_value(&self) -> &'static str; deprecated
    //fn get_actual_vc()
    // fn get_actual_vc_type
    // fn get_item_value_types
    fn get_value_constraint_value(&self) -> Box<dyn XSValue>;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;

}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSElementDeclaration.java

use super::{xs_term::XSTerm, xs_type_definition::XSTypeDefinition, xs_complex_type_definition::XSComplexTypeDefinition, xs_value::XSValue, xs_named_map::XSNamedMap, xs_annotation::XSAnnotation, xs_object_list::XSObjectList};

pub trait XSElementDeclaration: XSTerm {
    fn get_type_definition(&self) -> Box<dyn XSTypeDefinition>;
    fn get_scope(&self) -> i16;
    fn get_enclosing_ct_definition(&self) -> Box<dyn XSComplexTypeDefinition>;
    fn get_constraint_type(&self) -> i16;
    // fn get_constraint_value(); deprecated
    // fn get_actual_vc() deprecated
    // fn get_actual_vc_type() deprecated
    // fn get_item_value_types() deprecated

    fn get_value_constraint_value(&self) -> Box<dyn XSValue>;
    fn get_nillable(&self) -> bool;
    fn get_identity_constraints(&self) -> Box<dyn XSNamedMap>;
    fn get_substitution_group_affiliation(&self) -> Box<dyn XSElementDeclaration>;
    fn is_substitution_group_exclusion(&self, exclusion: i16) -> bool;
    fn get_substitution_group_exclusions(&self) -> i16;
    fn is_disallowed_substitution(&self, disallowed: i16) -> bool;
    fn get_disallowed_substitutions(&self) -> i16;
    fn get_abstract(&self) -> bool;
    fn get_annotation(&self) -> Box<dyn XSAnnotation>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;

}
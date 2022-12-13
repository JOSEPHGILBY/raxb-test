// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSModel.java

use super::{string_list::StringList, xs_namespace_item_list::XSNamespaceItemList, xs_named_map::XSNamedMap, xs_object_list::XSObjectList, xs_element_declaration::XSElementDeclaration, xs_attribute_declaration::XSAttributeDeclaration, xs_type_definition::XSTypeDefinition, xs_attribute_group_definition::XSAttributeGroupDefinition, xs_notation_declaration::XSNotationDeclaration, xs_model_group_definition::XSModelGroupDefinition, xs_idc_definition::XSIDCDefinition};

pub trait XSModel {
    fn get_namespaces(&self) -> Box<dyn StringList>;
    fn get_namespace_items(&self) -> Box<dyn XSNamespaceItemList>;
    fn get_components(&self, object_type: i16) -> Box<dyn XSNamedMap>;
    fn get_components_by_namespace(&self, object_type: i16, namespace: &'static str) -> Box<dyn XSNamedMap>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
    fn get_element_declaration(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSElementDeclaration>;
    fn get_attribute_declaration(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSAttributeDeclaration>;
    fn get_type_definition(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSTypeDefinition>;
    fn get_attribute_group(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSAttributeGroupDefinition>;
    fn get_model_group_definition(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSModelGroupDefinition>;
    fn get_notation_declaration(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSNotationDeclaration>;
    fn get_idc_definition(&self, name: &'static str, namespace: &'static str) -> Box<dyn XSIDCDefinition>;
    fn get_substitution_group(&self, head: Box<dyn XSElementDeclaration>) -> Box<dyn XSObjectList>;
}
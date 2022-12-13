// jdk/src/java.xml/share/classes/org/w3c/dom/Node.java

use std::rc::Rc;

use crate::document::Document;

pub trait Node {
    fn element_node(&self) -> i16 { 1 }
    fn attribute_node(&self) -> i16 { 2 }
    fn text_node(&self) -> i16 { 3 }
    fn cdata_section_node(&self) -> i16 { 4 }
    fn entity_reference_node(&self) -> i16 { 5 }
    fn entity_node(&self) -> i16 { 6 }
    fn processing_instruction_node(&self) -> i16 { 7 }
    fn comment_node(&self) -> i16 { 8 } 
    fn document_node(&self) -> i16 { 9 }
    fn document_type_node(&self) -> i16 { 10 }
    fn document_fragment_node(&self) -> i16 { 11 }
    fn notation_node(&self) -> i16 { 12 }

    fn get_node_name(&self) -> &'static str;
    fn get_node_value(&self) -> &'static str;
    fn set_node_value(&self, node_value: &'static str);
    fn get_node_type(&self) -> i16;
    fn get_parent_node(&self) -> Rc<dyn Node>;
    fn get_child_nodes(&self) -> Vec<Rc<dyn Node>>;
    fn get_first_child(&self) -> Rc<dyn Node>;
    fn get_last_child(&self) -> Rc<dyn Node>;
    fn get_previous_sibling(&self) -> Rc<dyn Node>;
    fn get_next_sibling(&self) -> Rc<dyn Node>;
    fn get_attributes(&self) -> String; // todo NamedNodeMa
    fn get_owner_document(&self) -> Rc<dyn Document>;
    fn insert_before(&self, new_child: Rc<dyn Node>, ref_child: Rc<dyn Node>) -> Rc<dyn Node>;
    fn replace_child(&self, new_child: Rc<dyn Node>, old_child: Rc<dyn Node>) -> Rc<dyn Node>;
    fn remove_child(&self, old_child: Rc<dyn Node>) -> Rc<dyn Node>;
    fn append_child(&self, new_child: Rc<dyn Node>) -> Rc<dyn Node>;
    fn has_child_nodes(&self) -> bool;
    fn clone_node(&self, deep: bool) -> Rc<dyn Node>; // TODO: use Clone derive
    fn normalize(&self);
    fn is_supported(&self, feature: &'static str, version: &'static str) -> bool;
    fn get_namespace_uri(&self) -> &'static str;
    fn get_prefix(&self) -> &'static str;
    fn set_prefix(&self, prefix: &'static str);
    fn get_local_name(&self) -> &'static str;
    fn has_attributes(&self) -> bool;
    fn get_base_uri(&self) -> &'static str;

    fn document_position_disconnected(&self) -> i16 { 0x01 }
    fn document_position_preceding(&self) -> i16 { 0x02 }
    fn document_position_following(&self) -> i16 { 0x04 }
    fn document_position_contains(&self) -> i16 { 0x08 }
    fn document_position_contained_by(&self) -> i16 { 0x10 }
    fn document_position_implementation_specific(&self) -> i16 { 0x20 }

    fn compare_document_position(&self, other: Rc<dyn Node>) -> i16;
    fn get_text_content(&self) -> &'static str;
    fn set_text_content(&self, text_content: &'static str);
    fn is_same_node(&self, other: Rc<dyn Node>) -> bool;
    fn lookup_prefix(&self, namespace_uri: &'static str) -> &'static str;
    fn is_default_namespace(&self, namespace: &'static str) -> bool;
    fn lookup_namespace_uri(&self, prefix: &'static str) -> &'static str;
    fn is_equal_node(&self, arg: Rc<dyn Node>) -> bool;
    fn set_user_data(&self, key: &'static str, data: String, handler: String) -> String; // TODO: Object and UserDataHandler
    fn get_user_data(&self, key: &'static str) -> String;

}
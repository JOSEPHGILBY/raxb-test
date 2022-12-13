use std::rc::Rc;

use crate::{node::Node, attr::Attr, node_list::NodeList, type_info::TypeInfo};

// jdk/src/java.xml/share/classes/org/w3c/dom/Element.java
pub trait Element: Node {
    
    fn get_tag_name(&self) -> &'static str;
    fn get_attribute(&self, name: &'static str) -> &'static str;
    fn set_attribute(&self, name: &'static str, value: &'static str);
    fn remove_attribute(&self, name: &'static str);
    fn get_attribute_node(&self, name: &'static str) -> Rc<dyn Attr>;
    fn set_attribute_node(&self, new_attr: Rc<dyn Attr>) -> Rc<dyn Attr>;
    fn remove_attribute_node(&self, old_attr: Rc<dyn Attr>) -> Rc<dyn Attr>;
    fn get_elements_by_tag_name(&self, name: &'static str) -> Rc<dyn NodeList>;
    fn get_attribute_ns(&self, namespace_uri: &'static str, local_name: &'static str) -> &'static str;
    fn set_attribute_ns(&self, namespace_uri: &'static str, qualified_name: &'static str, value: &'static str);
    fn remove_attribute_ns(&self, namespace_uri: &'static str, local_name: &'static str);
    fn get_attribute_node_ns(&self, namespace_uri: &'static str, local_name: &'static str) -> Rc<dyn Attr>;
    fn set_attribute_node_ns(&self, new_attr: Rc<dyn Attr>) -> Rc<dyn Attr>;
    fn get_elements_by_tag_name_ns(&self, namespace_uri: &'static str, local_name: &'static str) -> Rc<dyn NodeList>;
    fn has_attribute(&self, name: &'static str) -> bool;
    fn has_attribute_ns(&self, namespace_uri: &'static str, local_name: &'static str) -> bool;
    fn get_schema_type_info(&self) -> Rc<dyn TypeInfo>;
    fn set_id_attribute(&self, name: &'static str, is_id: bool);
    fn set_id_attribute_ns(&self, namespace_uri: &'static str, local_name: &'static str, is_id: bool);
    fn set_id_attribute_node(&self, id_attr: Rc<dyn Attr>, is_id: bool);

}
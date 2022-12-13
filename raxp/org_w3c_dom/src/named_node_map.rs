// jdk/src/java.xml/share/classes/org/w3c/dom/NamedNodeMap.java

use std::rc::Rc;

use crate::node::Node;

pub trait NamedNodeMap {
    fn get_named_item(&self, name: &'static str) -> Rc<dyn Node>;
    fn set_named_item(&self, arg: Rc<dyn Node>) -> Rc<dyn Node>;
    fn remove_named_item(&self, name: &'static str) -> Rc<dyn Node>;
    fn item(&self, index: i32) -> Rc<dyn Node>;
    fn get_length(&self) -> i32;
    fn get_named_item_ns(&self, namespace_uri: &'static str, local_name: &'static str) -> Rc<dyn Node>;
    fn set_named_item_ns(&self, arg: Rc<dyn Node>) -> Rc<dyn Node>;
    fn remove_named_item_ns(&self, namespace_uri: &'static str, local_name: &'static str) -> Rc<dyn Node>;
    
}
use std::rc::Rc;

use crate::node::Node;

// jdk/src/java.xml/share/classes/org/w3c/dom/NodeList.java
pub trait NodeList {
    fn item(&self, index: i32) -> Rc<dyn Node>;
    fn get_length(&self) -> i32;
}
// jdk/src/java.xml/share/classes/org/w3c/dom/DocumentType.java

use std::rc::Rc;

use crate::{node::Node, named_node_map::NamedNodeMap};

pub trait DocumentType: Node {
    fn get_name(&self) -> &'static str;
    fn get_entities(&self) -> Rc<dyn NamedNodeMap>;
    fn get_notations(&self) -> Rc<dyn NamedNodeMap>;
    fn get_public_id(&self) -> &'static str;
    fn get_system_id(&self) -> &'static str;
    fn get_internal_subset(&self) -> &'static str;
}
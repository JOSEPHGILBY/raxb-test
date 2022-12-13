// jdk/src/java.xml/share/classes/org/w3c/dom/Attr.java

use std::rc::Rc;

use crate::{node::Node, element::Element, type_info::TypeInfo};

pub trait Attr: Node {
    fn get_name(&self) -> &'static str;
    fn get_specified(&self) -> bool;
    fn get_value(&self) -> &'static str;
    fn set_value(&self, value: &'static str);
    fn get_owner_element(&self) -> Rc<dyn Element>;
    fn get_schema_type_info(&self) -> Rc<dyn TypeInfo>;
    fn is_id(&self) -> bool;
    
}
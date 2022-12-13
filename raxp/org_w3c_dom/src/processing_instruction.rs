// jdk/src/java.xml/share/classes/org/w3c/dom/ProcessingInstruction.java

use crate::node::Node;

pub trait ProcessingInstruction: Node {
    fn get_target(&self) -> &'static str;
    fn get_data(&self) -> &'static str;
    fn set_data(&self, data: &'static str);
    
}
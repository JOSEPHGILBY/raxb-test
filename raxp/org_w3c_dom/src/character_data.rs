// jdk/src/java.xml/share/classes/org/w3c/dom/CharacterData.java

use crate::node::Node;

pub trait CharacterData: Node {
    fn get_data(&self) -> &'static str;
    fn set_data(&self, data: &'static str);
    fn get_length(&self) -> i32;
    fn substring_data(&self, offset: i32, count: i32) -> &'static str;
    fn append_data(&self, arg: &'static str);
    fn insert_data(&self, offset: i32, arg: &'static str);
    fn delete_data(&self, offset: i32, count: i32);
    fn replace_data(&self, offset: i32, count: i32, arg: &'static str);
    
}
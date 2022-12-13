// jdk/src/java.xml/share/classes/org/w3c/dom/Text.java

use std::rc::Rc;

use crate::character_data::CharacterData;

pub trait Text: CharacterData {
    fn split_text(&self, offset: i32) -> Rc<dyn Text>;
    fn is_element_content_whitespace(&self) -> bool;
    fn get_whole_text(&self) -> &'static str;
    fn replace_whole_text(&self, content: &'static str) -> Rc<dyn Text>;
}
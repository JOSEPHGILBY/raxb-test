// jdk/src/java.xml/share/classes/org/xml/sax/ext/LexicalHandler.java

pub trait LexicalHandler {
    fn start_dtd(&self, name: &'static str, public_id: &'static str, system_id: &'static str);
    fn end_dtd(&self);
    fn start_entity(&self, name: &'static str);
    fn end_entity(&self, name: &'static str);
    fn start_cdata(&self);
    fn end_cdata(&self);
    fn comment(&self, ch: [char], start: i32, length: i32);
}
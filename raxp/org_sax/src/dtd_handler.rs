// jdk/src/java.xml/share/classes/org/xml/sax/DTDHandler.java

pub trait DTDHandler {
    fn notation_decl(&self, name: &str, public_id: &str, system_id: &str);
    fn unparsed_entity_decl(&self, name: &str, public_id: &str, system_id: &str, notation_name: &str);
    
}
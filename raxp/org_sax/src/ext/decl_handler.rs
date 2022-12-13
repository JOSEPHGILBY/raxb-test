// jdk/src/java.xml/share/classes/org/xml/sax/ext/DeclHandler.java

pub trait DeclHandler {
    fn element_decl(&self, name: &'static str, model: &'static str);
    fn attribute_decl(&self, e_name: &'static str, a_name: &'static str, the_type: &'static str, mode: &'static str, value: &'static str);
    fn internal_entity_decl(&self, name: &'static str, value: &'static str);
    fn external_entity_decl(&self, name: &'static str, public_id: &'static str, system_id: &'static str);
}
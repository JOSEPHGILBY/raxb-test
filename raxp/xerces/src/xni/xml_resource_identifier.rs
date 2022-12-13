// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLResourceIdentifier.java

pub trait XMLResourceIdentifier {
    fn set_public_id(&self, public_id: &'static str);
    fn get_public_id(&self) -> &'static str;
    fn set_expanded_system_id(&self, system_id: &'static str);
    fn get_expanded_system_id(&self) -> &'static str;
    fn set_literal_system_id(&self, system_id: &'static str);
    fn get_literal_system_id(&self) -> &'static str;
    fn set_base_system_id(&self, system_id: &'static str);
    fn get_base_system_id(&self) -> &'static str;
    fn set_namespace(&self, namespace: &'static str);
    fn get_namespace(&self) -> &'static str;
}
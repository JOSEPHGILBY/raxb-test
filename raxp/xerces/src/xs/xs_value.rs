// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSValue.java

use super::{xs_simple_type_definition::XSSimpleTypeDefinition, xs_object_list::XSObjectList, short_list::ShortList};

pub trait XSValue {

    fn get_normalized_value(&self) -> &'static str;
    fn get_actual_value(&self) -> String; // TODO: object
    fn get_type_definition(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_member_type_definition(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_member_type_definitions(&self) -> Box<dyn XSObjectList>;
    fn get_actual_value_type(&self) -> i16;
    fn get_list_value_types(&self) -> Box<dyn ShortList>;

}
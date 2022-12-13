// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/ItemPSVI.java

use super::{string_list::StringList, xs_value::XSValue, xs_type_definition::XSTypeDefinition, xs_simple_type_definition::XSSimpleTypeDefinition};

pub trait ItemPSVI {
    fn validity_notknown(&self) -> i16 { 0 }
    fn validity_invalid(&self) -> i16 { 1 }
    fn validity_valid(&self) -> i16 { 2 }
    fn validation_none(&self) -> i16 { 0 }
    fn validation_partial(&self) -> i16 { 1 }
    fn validation_full(&self) -> i16 { 2 }

    fn constant(&self) -> Box<dyn ItemPSVI>;
    fn is_constant(&self) -> bool;
    fn get_validation_context(&self) -> &'static str;
    fn get_validity(&self) -> i16;
    fn get_validation_attempted(&self) -> i16;
    fn get_error_codes(&self) -> Box<dyn StringList>;
    fn get_error_messages(&self) -> Box<dyn StringList>;
    
    //fn get_schema_normalized_value() deprecated
    //fn get_actual_normalized_value() deprecated
    //fn get_actual_normalized_value_type() deprecated
    //fn get_item_value_types() deprecated

    fn get_schema_value(&self) -> Box<dyn XSValue>;
    fn get_type_definition(&self) -> Box<dyn XSTypeDefinition>;
    fn get_member_type_definition(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_schema_default(&self) -> &'static str;
    fn get_is_schema_specified(&self) -> bool;

}
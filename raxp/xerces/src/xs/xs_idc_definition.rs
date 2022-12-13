// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSIDCDefinition.java

use super::{xs_object::XSObject, string_list::StringList, xs_object_list::XSObjectList};

pub trait XSIDCDefinition: XSObject {
    fn ic_key(&self) -> i16 { 1 }
    fn ic_keyref(&self) -> i16 { 2 }
    fn ic_unique(&self) -> i16 { 3 }

    fn get_category(&self) -> i16;
    fn get_selector_str(&self) -> &'static str;
    fn get_field_strs(&self) -> Box<dyn StringList>;
    fn get_ref_key(&self) -> Box<dyn XSIDCDefinition>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;
}
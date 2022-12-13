// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLAttributes.java

use super::{q_name::QName, xml_string::XMLString, augmentations::Augmentations};

pub trait XMLAttributes {
    fn add_attribute(&self, attr_name: QName, attr_type: &'static str, attr_value: &'static str) -> i32;
    fn remove_all_attributes(&self);
    fn remove_attribute_at(&self, attr_index: i32);
    fn get_length(&self) -> i32;
    fn get_index_from(&self, q_name: &'static str) -> i32;
    fn get_index_from_2(&self, uri: &'static str, local_part: &'static str) -> i32;
    fn set_name(&self, attr_index: i32, attr_name: QName);
    fn get_name(&self, attr_index: i32, attr_name: QName);
    fn get_prefix(&self, index: i32) -> &'static str;
    fn get_uri(&self, index: i32) -> &'static str;
    fn get_local_name(&self, index: i32) -> &'static str;
    fn get_q_name(&self, index: i32) -> &'static str;
    fn get_qualified_name(&self, index: i32) -> QName;
    fn set_type(&self, attr_index: i32, attr_type: &'static str);
    fn get_type(&self, index: i32) -> &'static str;
    fn get_type_2(&self, q_name: &'static str) -> &'static str;
    fn get_type_3(&self, uri: &'static str, localname: &'static str);
    fn set_value(&self, attr_index: i32, attr_value: &'static str);
    fn set_value_2(&self, attr_index: i32, attr_value: &'static str, value: XMLString);
    fn get_value(&self, index: i32) -> &'static str;
    fn get_value_2(&self, q_name: &'static str) -> &'static str;
    fn get_value_3(&self, uri: &'static str, local_name: &'static str) -> &'static str;
    fn set_non_normalized_value(&self, attr_index: i32, attr_value: &'static str);
    fn get_non_normalized_value(&self, attr_index: i32) -> &'static str;
    fn set_specified(&self, attr_index: i32, specified: bool);
    fn is_specified(&self, attr_index: i32) -> bool;
    fn get_augmentations(&self, attribute_index: i32) -> Box<dyn Augmentations>;
    fn get_augmentations_2(&self, uri: &'static str, local_part: &'static str) -> Box<dyn Augmentations>;
    fn get_augmenations_3(&self, q_name: &'static str) -> Box<dyn Augmentations>;
    fn set_augmenations(&self, attr_index: i32, augs: Box<dyn Augmentations>);
}
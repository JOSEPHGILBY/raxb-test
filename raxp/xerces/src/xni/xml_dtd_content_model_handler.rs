// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLDTDContentModelHandler.java

use std::sync::Arc;

use super::{augmentations::Augmentations, parser::xml_dtd_content_model_source::XMLDTDContentModelSource};


pub trait XMLDTDContentModelHandler {
    fn separator_choice(&self) -> i16 { 0 }
    fn separator_sequence(&self) -> i16 { 1 }
    fn occurs_zero_or_one(&self) -> i16 { 2 }
    fn occurs_zero_or_more(&self) -> i16 { 3 }
    fn occurs_one_or_more(&self) -> i16 { 4 }

    fn start_content_model(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>);
    fn any(&self, augmentations: Box<dyn Augmentations>);
    fn empty(&self, augmentations: Box<dyn Augmentations>);
    fn start_group(&self, augmentations: Box<dyn Augmentations>);
    fn pcdata(&self, augmentations: Box<dyn Augmentations>);
    fn element(&self, element_name: &'static str, augmentations: Box<dyn Augmentations>);
    fn separator(&self, separator: i16, augmentations: Box<dyn Augmentations>);
    fn occurrence(&self, occurrence: i16, augmentations: Box<dyn Augmentations>);
    fn end_group(&self, augmentations: Box<dyn Augmentations>);
    fn end_content_model(&self, augmentations: Box<dyn Augmentations>);
    fn set_dtd_content_model_source(&mut self, source: Arc<dyn XMLDTDContentModelSource>);
    fn get_dtd_content_model_source(&self) -> Option<Arc<dyn XMLDTDContentModelSource>>;
}
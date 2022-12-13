// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/AttributePSVI.java

use super::{item_psvi::ItemPSVI, xs_attribute_declaration::XSAttributeDeclaration};

pub trait AttributePSVI: ItemPSVI {
    
    fn get_attribute_declaration(&self) -> Box<dyn XSAttributeDeclaration>;
}
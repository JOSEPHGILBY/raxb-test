// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/ElementPSVI.java

use super::{item_psvi::ItemPSVI, xs_element_declaration::XSElementDeclaration, xs_notation_declaration::XSNotationDeclaration, xs_model::XSModel};

pub trait ElementPSVI: ItemPSVI {
    fn get_element_declaration(&self) -> Box<dyn XSElementDeclaration>;
    fn get_notation(&self) -> Box<dyn XSNotationDeclaration>;
    fn get_nil(&self) -> bool;
    fn get_schema_information(&self) -> Box<dyn XSModel>;
}
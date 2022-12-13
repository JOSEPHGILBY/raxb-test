// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLDocumentSource.java

use crate::xni::xml_document_handler::XMLDocumentHandler;


pub trait XMLDocumentSource {
    fn set_document_handler(&self, handler: Box<dyn XMLDocumentHandler>);
    fn get_document_handler(&self) -> Box<dyn XMLDocumentHandler>;
}
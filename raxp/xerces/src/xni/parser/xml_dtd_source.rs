// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLDTDSource.java

use crate::xni::xml_dtd_handler::XMLDTDHandler;

pub trait XMLDTDSource {
    fn set_dtd_handler(&self, handler: Box<dyn XMLDTDHandler>);
    fn get_dtd_handler(&self) -> Box<dyn XMLDTDHandler>;
}
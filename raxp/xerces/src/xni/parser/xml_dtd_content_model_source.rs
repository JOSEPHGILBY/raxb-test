// /workspaces/raxb-test/jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLDTDContentModelSource.java

use crate::xni::xml_dtd_content_model_handler::XMLDTDContentModelHandler;

pub trait XMLDTDContentModelSource {
    fn set_dtd_content_model_handler(&self, handler: Box<dyn XMLDTDContentModelHandler>);
    fn get_dtd_content_model_handler(&self) -> Box<dyn XMLDTDContentModelHandler>;
}
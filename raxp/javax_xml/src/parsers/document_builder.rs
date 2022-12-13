use std::rc::Rc;

use org_w3c_dom::document::Document;



pub trait DocumentBuilder {
    fn reset();
    fn parse(istr: String) -> Rc<dyn Document>;
}

pub struct DocumentBuilderAbstractItems {

}

impl DocumentBuilderAbstractItems {
    pub fn new() -> DocumentBuilderAbstractItems {
        DocumentBuilderAbstractItems {  }
    }
}

impl DocumentBuilder for DocumentBuilderAbstractItems {
    fn reset() {

    }


}
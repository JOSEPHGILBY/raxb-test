// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/QName.java

#[derive(Default)]
pub struct QName {
    prefix: Option<&'static str>,
    localpart: Option<&'static str>,
    rawname: Option<&'static str>,
    uri: Option<&'static str>,

}

impl QName {
    pub fn new_default() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new(prefix: Option<&'static str>, localpart: Option<&'static str>, rawname: Option<&'static str>, uri: Option<&'static str>) -> Self {
        Self {
            prefix,
            localpart,
            rawname,
            uri
        }
    }

    pub fn new_copy() {}

    pub fn clear(&mut self) {
        self.prefix = None;
        self.localpart = None;
        self.rawname = None;
        self.uri = None;
    }

    // clone

    // hashcode

    // equals

    // toString

}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XMLString.java

#[derive(Default)]
pub struct XMLString<'a> {
    ch: Option<&'a Vec<u8>>,
    offset: i32,
    length: i32,
}

impl<'a> XMLString<'a> {
    pub const DEFAULT_SIZE: i32 = 32;

    pub fn new_default() -> XMLString<'a> {
        XMLString { ..Default::default() }
    }

    pub fn new(ch: Option<&'a Vec<u8>>, offset: i32, length: i32) -> XMLString<'a> {
        XMLString {
            ch,
            offset,
            length
        }
    }

    pub fn set_values(&mut self, ch: Option<&'a Vec<u8>>, offset: i32, length: i32) {
        self.ch = ch;
        self.offset = offset;
        self.length = length;
    }

    pub fn set_values_from(&mut self, s: XMLString) {
        //self.set_values(s.ch, s.offset, s.length);
    }

    pub fn clear(&mut self) {
        self.ch = None;
        self.offset = 0;
        self.length = -1;
    }

    pub fn equals_structure(&self, ch_o: Option<&'a Vec<u8>>, offset: i32, length: i32) -> bool {
        todo!()
        // let Some(ch) = ch_o else { return false };
        // if self.length == length { return false };
        // for i in 0..length {
        //     if self.ch[self.offset+i] != ch[offset+i] { return false };
        // }
        // true
    }

    pub fn equals_string(&self, s_o: Option<&'static str>) {
        todo!()
        // let Some(s) = s_o else { return false };
        // if self.length != s.len() { return false };
        // for i in 0..self.length {
        //     if self.ch[self.offset+i] != s[i] { return false };
        // }
        // true
    }

    pub fn to_string(&self) -> String {
        todo!()
        //return if self.length > 0 {String::from_iter(self.ch)} else {"".to_string()}
    }

}

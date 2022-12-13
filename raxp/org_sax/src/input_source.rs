// jdk/src/java.xml/share/classes/org/xml/sax/InputSource.java

#[derive(Default)]
pub struct InputSource {
    pub public_id: String,
    pub system_id: String,
    pub byte_stream: String, // InputStream equivalent
    pub encoding: String,
    pub character_stream: String,
}

impl InputSource {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new_with_system_id(system_id: String) -> Self {
        Self {
            system_id,
            ..Default::default()
        }
    }

    pub fn new_with_byte_stream(byte_stream: String) -> Self {
        Self {
            byte_stream,
            ..Default::default()
        }
    }

    pub fn new_with_character_sream(character_stream: String) -> Self {
        Self {
            character_stream,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn is_stream_empty() -> bool {
        true
    }
}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLInputSource.java


// TODO: https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits

use org_sax::input_source::InputSource;


#[derive(Default)]
pub struct XMLInputSource {
    // TODO: protected
    f_public_id: String,
    f_system_id: String,
    f_base_system_id: String,
    // TODO: inputstream
    f_byte_stream: Option<String>,
    // TODO: reader
    f_char_stream: Option<String>,
    f_encoding: String,
    f_is_created_by_resolver: bool, // default: false
}

impl XMLInputSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_public_system_identifiers(
        public_id: String,
        system_id: String,
        base_system_id: String,
        is_created_by_resolver: bool,
    ) -> Self {
        Self {
            f_public_id: public_id,
            f_system_id: system_id,
            f_base_system_id: base_system_id,
            f_is_created_by_resolver: is_created_by_resolver,
            ..Default::default()
        }
    }

    pub fn new_with_resource_identifier(
        resource_identifier: String
    ) -> Self {
        Self {
            f_public_id: "todo".to_string(),
            f_system_id: "todo".to_string(),
            f_base_system_id: "todo".to_string(),
            ..Default::default()
        }
    }

    pub fn new_with_sax_input_source(
        input_source: InputSource,
        is_created_by_resolver: bool
    ) -> Self {
        Self {
            f_public_id: input_source.public_id,
            f_system_id: input_source.system_id,
            f_byte_stream: Some(input_source.byte_stream),
            f_char_stream: Some(input_source.character_stream),
            f_encoding: input_source.encoding,
            f_is_created_by_resolver: is_created_by_resolver,
            ..Default::default()
        }
    }

    pub fn new_with_byte_stream(
        public_id: String,
        system_id: String,
        base_system_id: String,
        byte_stream: Option<String>,
        encoding: String,
    ) -> Self {
        Self {
            f_public_id: public_id,
            f_system_id: system_id,
            f_base_system_id: base_system_id,
            f_byte_stream: byte_stream,
            f_encoding: encoding,
            ..Default::default()
        }
    }

    pub fn new_with_character_stream(
        public_id: String,
        system_id: String,
        base_system_id: String,
        char_stream: Option<String>,
        encoding: String,
    ) -> Self {
        Self {
            f_public_id: public_id,
            f_system_id: system_id,
            f_base_system_id: base_system_id,
            f_char_stream: char_stream,
            f_encoding: encoding,
            ..Default::default()
        }
    }
    
}
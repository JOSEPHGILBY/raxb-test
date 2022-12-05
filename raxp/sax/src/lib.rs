#[cfg(test)]
use rstest_reuse;

pub mod sax_parser_xerces;
pub mod sax_parser;
pub mod sax_parser_factory;
pub mod sax_parser_factory_impl;
pub mod sax_parser_impl;
pub mod xml_constants;
pub mod impl_constants;
pub mod jdk_constants;
pub mod abstract_sax_parser;
pub mod abstract_xml_document_parser;
pub mod xml_parser;
pub mod combine;
pub mod xml_input_source;
pub mod xml_parser_configuration;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

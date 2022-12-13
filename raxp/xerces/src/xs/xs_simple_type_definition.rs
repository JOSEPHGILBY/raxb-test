// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xs/XSSimpleTypeDefinition.java

use super::{xs_type_definition::XSTypeDefinition, xs_object_list::XSObjectList, string_list::StringList, xs_object::XSObject};

pub trait XSSimpleTypeDefinition: XSTypeDefinition {

    fn variety_absent(&self) -> i16 { 0 }
    fn variety_atomic(&self) -> i16 { 1 }
    fn variety_list(&self) -> i16 { 2 }
    fn variety_union(&self) -> i16 { 3 }

    fn facet_none(&self) -> i16 { 0 }
    fn facet_length(&self) -> i16 { 1 }
    fn facet_minlength(&self) -> i16 { 2 }
    fn facet_maxlength(&self) -> i16 { 4 }
    fn facet_pattern(&self) -> i16 { 8 }
    fn facet_whitespace(&self) -> i16 { 16 }
    fn facet_maxinclusive(&self) -> i16 { 32 }
    fn facet_maxexclusive(&self) -> i16 { 64 }
    fn facet_minexclusive(&self) -> i16 { 128 }
    fn facet_mininclusive(&self) -> i16 { 256 }
    fn facet_totaldigits(&self) -> i16 { 512 }
    fn facet_fractiondigits(&self) -> i16 { 1024 }
    fn facet_enumeration(&self) -> i16 { 2048 }

    fn ordered_false(&self) -> i16 { 0 }
    fn ordered_partial(&self) -> i16 { 1 }
    fn ordered_total(&self) -> i16 { 2 }

    fn get_variety(&self) -> i16;
    fn get_primitive_type(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_built_in_kind(&self) -> i16;
    fn get_item_type(&self) -> Box<dyn XSSimpleTypeDefinition>;
    fn get_member_types(&self) -> Box<dyn XSObjectList>;
    fn get_defined_facets(&self) -> i16;
    fn is_defined_facet(&self, facet_name: i16) -> bool;
    fn get_fixed_facets(&self) -> i16;
    fn is_fixed_facet(&self, facet_name: i16) -> bool;
    fn get_lexical_facet_value(&self, facet_name: i16) -> &'static str;
    fn get_lexical_enumeration(&self) -> Box<dyn StringList>;
    fn get_lexical_pattern(&self) -> Box<dyn StringList>;
    fn get_ordered(&self) -> i16;
    fn get_finite(&self) -> bool;
    fn get_bounded(&self) -> bool;
    fn get_numeric(&self) -> bool;
    fn get_facets(&self) -> Box<dyn XSObjectList>;
    fn get_multi_value_facets(&self) -> Box<dyn XSObjectList>;
    fn get_facet(&self, facet_type: i32) -> Box<dyn XSObject>;
    fn get_annotations(&self) -> Box<dyn XSObjectList>;


}
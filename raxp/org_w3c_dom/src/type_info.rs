// jdk/src/java.xml/share/classes/org/w3c/dom/TypeInfo.java

pub trait TypeInfo {
    fn get_type_name(&self) -> &'static str;
    fn get_type_namespace(&self) -> &'static str;

    fn derivation_restriction(&self) -> i32 { 0x00000001 }
    fn derivation_extension(&self) -> i32 { 0x00000002 }
    fn derivation_union(&self) -> i32 { 0x00000004 }
    fn derivation_list(&self) -> i32 { 0x00000008 }

    fn is_derived_from(&self, type_namespace_arg: &'static str, type_name_arg: &'static str, derivation_method: i32) -> bool;
    
}
// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/utils/XMLSecurityManager.java

use enum_ordinalize::Ordinalize;

//use crate::{jdk_constants::JdkConstants, jdk_property::{StateTypes, ImplPropMapTypes}};
use jdk_xml::{jdk_constants::JdkConstants, jdk_property::{StateTypes, ImplPropMapTypes}};
pub struct XMLSecurityManager {
    values: Vec<i32>,
    states: Vec<StateTypes>,
    secure_processing: bool,
    is_set: Vec<bool>,
    print_entity_count_info: String,

}

impl XMLSecurityManager {
    const NO_LIMIT: i32 = 0;
    const INDEX_ENTITY_COUNT_INFO: i32 = 10000;

    pub fn new() -> Self {
        XMLSecurityManager::new_with_secure_processing(false)
    }

    // TODO: update this once const_generic complex expressions stabilize
    pub fn new_with_secure_processing(secure_processing: bool) -> Self {
        let mut values = vec![0; LimitTypes::variant_count()];
        let mut states = vec![StateTypes::DEFAULT; StateTypes::variant_count()];
        let is_set = vec![false; LimitTypes::variant_count()];
        for limit in LimitTypes::variants().iter() {
            if secure_processing {
                values[limit.ordinal() as usize] = limit.secure_value();
                states[limit.ordinal() as usize] = StateTypes::FSP;
            } else {
                values[limit.ordinal() as usize] = limit.default_value();
                states[limit.ordinal() as usize] = StateTypes::DEFAULT;
            }
        }
        XMLSecurityManager {
            values,
            states,
            secure_processing,
            is_set,
            print_entity_count_info: "".to_string(),
        }
    }

    pub fn set_secure_processing(&mut self, secure: bool) {
        self.secure_processing = secure;
        for limit in LimitTypes::variants().iter() {
            if secure {

            }
        }
    }

    pub fn find(property_name: &String) -> Option<&'static str> {
        for limit in LimitTypes::variants() {
            if limit.is(property_name) {
                return Some(limit.system_property())
            }
        }
        //ENTITYCOUNT's new name is qName
        if ImplPropMapTypes::ENTITYCOUNT.is(property_name) {
            return Some(ImplPropMapTypes::ENTITYCOUNT.q_name());
        }
        return None;
    }

    // TODO:
    // pub fn set_limit(property_name: String, state: StateTypes, value: Object?????????)
    pub fn set_limit(limit: LimitTypes, state: StateTypes, value: i32) {

    }

    // pub fn set_limit_by_index_value(&self, index: i32, state: StateTypes, value: i32) {
    //     if index == XMLSecurityManager::INDEX_ENTITY_COUNT_INFO {
    //         self.print_entity_count_info = JdkConstants::JDK_YES.to_string();
    //         return;
    //     }

    //     if (state.compareTo)
    // }
    // pub fn set_limit(index: i32, state: StateTypes, value: Object????????)
    // pub fn set_limit(index: i32, state: StateTypes, value: int)
}   

#[derive(Ordinalize)]
pub enum LimitTypes {
    ENTITY_EXPANSION_LIMIT,
    MAX_OCCUR_NODE_LIMIT,
    ELEMENT_ATTRIBUTE_LIMIT,
    TOTAL_ENTITY_SIZE_LIMIT,
    GENERAL_ENTITY_SIZE_LIMIT,
    PARAMETER_ENTITY_SIZE_LIMIT,
    MAX_ELEMENT_DEPTH_LIMIT,
    MAX_NAME_LIMIT,
    ENTITY_REPLACEMENT_LIMIT,
}

impl LimitTypes {
    const fn value(&self) -> LimitValue {
        use LimitTypes::*;
        match self {
            ENTITY_EXPANSION_LIMIT => 
                LimitValue::new(
                    "EntityExpansionLimit",
                    JdkConstants::JDK_ENTITY_EXPANSION_LIMIT,
                    JdkConstants::SP_ENTITY_EXPANSION_LIMIT,
                    0,
                    64000
                ),
            MAX_OCCUR_NODE_LIMIT => 
            LimitValue::new(
                "MaxOccurLimit",
                JdkConstants::JDK_MAX_OCCUR_LIMIT,
                JdkConstants::SP_MAX_OCCUR_LIMIT,
                0,
                5000
            ),
            ELEMENT_ATTRIBUTE_LIMIT => 
            LimitValue::new(
                "ElementAttributeLimit",
                JdkConstants::JDK_ELEMENT_ATTRIBUTE_LIMIT,
                JdkConstants::SP_ELEMENT_ATTRIBUTE_LIMIT,
                0,
                10000
            ),
            TOTAL_ENTITY_SIZE_LIMIT => 
            LimitValue::new(
                "TotalEntitySizeLimit",
                JdkConstants::JDK_TOTAL_ENTITY_SIZE_LIMIT,
                JdkConstants::SP_TOTAL_ENTITY_SIZE_LIMIT,
                0,
                50000000
            ),
            GENERAL_ENTITY_SIZE_LIMIT => 
            LimitValue::new(
                "MaxEntitySizeLimit",
                JdkConstants::JDK_GENERAL_ENTITY_SIZE_LIMIT,
                JdkConstants::SP_GENERAL_ENTITY_SIZE_LIMIT,
                0,
                0
            ),
            PARAMETER_ENTITY_SIZE_LIMIT => 
            LimitValue::new(
                "MaxEntitySizeLimit",
                JdkConstants::JDK_PARAMETER_ENTITY_SIZE_LIMIT,
                JdkConstants::SP_PARAMETER_ENTITY_SIZE_LIMIT,
                0,
                1000000
            ),
            MAX_ELEMENT_DEPTH_LIMIT => 
            LimitValue::new(
                "MaxElementDepthLimit",
                JdkConstants::JDK_MAX_ELEMENT_DEPTH,
                JdkConstants::SP_MAX_ELEMENT_DEPTH,
                0,
                0
            ),
            MAX_NAME_LIMIT => 
            LimitValue::new(
                "MaxXMLNameLimit",
                JdkConstants::JDK_XML_NAME_LIMIT,
                JdkConstants::SP_XML_NAME_LIMIT,
                1000,
                1000
            ),
            ENTITY_REPLACEMENT_LIMIT => 
            LimitValue::new(
                "EntityReplacementLimit",
                JdkConstants::JDK_ENTITY_REPLACEMENT_LIMIT,
                JdkConstants::SP_ENTITY_REPLACEMENT_LIMIT,
                0,
                3000000
            ),
        }

    }

    // TODO: Why is there a null check for system property?
    pub fn is(&self, name: &String) -> bool{
        let val = self.value();
        val.system_property == name || val.api_property == name
    }

    pub fn get_state(&self, name: String) -> Option<StateTypes> {
        let val = self.value();
        if val.system_property == name {
            Some(StateTypes::APIPROPERTY)
        } else if val.api_property == name {
            Some(StateTypes::LEGACY_APIPROPERTY)
        } else {
            None
        }
    }

    pub fn key(&self) -> &'static str {
        self.value().key
    }

    pub fn api_property(&self) -> &'static str {
        self.value().api_property
    }

    pub fn system_property(&self) -> &'static str {
        self.value().system_property
    }

    pub fn default_value(&self) -> i32 {
        self.value().default_value
    }

    pub fn secure_value(&self) -> i32 {
        self.value().secure_value
    }


}

pub struct LimitValue {
    key: &'static str,
    api_property: &'static str,
    system_property: &'static str,
    default_value: i32,
    secure_value: i32
}

impl LimitValue {
    pub const fn new(key: &'static str, api_property: &'static str, system_property: &'static str, value: i32, secure_value: i32) -> Self {
        Self {
            key,
            api_property,
            system_property, 
            default_value: value,
            secure_value,
        }
    }
}

pub enum NameMapTypes {
    ENTITY_EXPANSION_LIMIT,
    MAX_OCCUR_NODE_LIMIT,
    ELEMENT_ATTRIBUTE_LIMIT,

}

impl NameMapTypes {
    const fn value(&self) -> NameMapValue {
        use NameMapTypes::*;
        match self {
            ENTITY_EXPANSION_LIMIT => 
                NameMapValue {
                    new_name: JdkConstants::SP_ENTITY_EXPANSION_LIMIT,
                    old_name: JdkConstants::ENTITY_EXPANSION_LIMIT,
                },
            MAX_OCCUR_NODE_LIMIT => NameMapValue { 
                new_name: JdkConstants::SP_MAX_OCCUR_LIMIT, 
                old_name: JdkConstants::MAX_OCCUR_LIMIT, 
            },
            ELEMENT_ATTRIBUTE_LIMIT => NameMapValue { 
                new_name: JdkConstants::SP_ELEMENT_ATTRIBUTE_LIMIT, 
                old_name: JdkConstants::ELEMENT_ATTRIBUTE_LIMIT,
            }
        }
    }

    pub fn get_old_name(&self, new_name: String) -> Option<&'static str> {
        let val = self.value();
        if new_name == val.new_name {
            return Some(val.old_name)
        }
        None
    }
}

pub struct NameMapValue {
    new_name: &'static str,
    old_name: &'static str,
}
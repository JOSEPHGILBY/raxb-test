// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/utils/XMLSecurityPropertyManager.java

use enum_ordinalize::Ordinalize;
use jdk_xml::{xml_constants::XMLConstants, jdk_constants::JdkConstants};


pub struct XMLSecurityPropertyManager {
    
}

pub enum State {
    DEFAULT,
    FSP,
    JAXPDOTPROPERTIES,
    SYSTEMPROPERTY,
    APIPROPERTY,
}

#[derive(Ordinalize, Clone, PartialEq)]
pub enum PropertyType {
    ACCESS_EXTERNAL_DTD, 
    ACCESS_EXTERNAL_SCHEMA,
}

impl PropertyType {
    const fn value(&self) -> PropertyValue {
        use PropertyType::*;
        match self {
            ACCESS_EXTERNAL_DTD => PropertyValue {
                name: XMLConstants::ACCESS_EXTERNAL_DTD,
                default_value: JdkConstants::EXTERNAL_ACCESS_DEFAULT
            },
            ACCESS_EXTERNAL_SCHEMA => PropertyValue {
                name: XMLConstants::ACCESS_EXTERNAL_SCHEMA,
                default_value: JdkConstants::EXTERNAL_ACCESS_DEFAULT
            }
        }
    }

    pub fn equals_name(&self, property_name: Option<&'static str>) -> bool {
        let Some(prop) = property_name else {
            return false;
        };
        prop == self.value().name
    }

    pub fn property_name(&self) -> &'static str {
        self.value().name
    }

    pub fn default_value(&self) -> &'static str {
        self.value().default_value
    }
}

pub struct PropertyValue {
    pub name: &'static str,
    pub default_value: &'static str,
}
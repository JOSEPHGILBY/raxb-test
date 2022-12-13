// jdk/src/java.xml/share/classes/jdk/xml/internal/JdkProperty.java

use enum_ordinalize::Ordinalize;
use crate::jdk_constants::JdkConstants;

pub struct JdkProperty<T> {
    p_name: ImplPropMapTypes,
    p_type: String, // TODO: deal with class type
    p_value: T,
    p_state: StateTypes,
}

impl<T> JdkProperty<T> {
    pub fn new(name: ImplPropMapTypes, the_type: String, value: T, state: StateTypes) -> Self {
        Self {
            p_name: name,
            p_type: the_type,
            p_value: value,
            p_state: state
        }
    }

    // TODO: 
    fn read_system_property(&self) {
        if self.p_state != StateTypes::DEFAULT {return;}
        let mut value = None::<T>;
        if let Some(prop) = self.p_name.system_property() {
            //
        }
    }

    fn get_value(&self) -> &T {
        &self.p_value
    }

    fn set_value(&mut self, name: String, value: T, state: StateTypes) -> bool {
        let Some(p_state_1) = self.p_name.get_state(name) else { return false; };
        if p_state_1 == self.p_state {
            self.p_state = p_state_1;
            self.p_value = value;
            return true;
        }
        false
    }
}

#[derive(Ordinalize, Clone)]
pub enum ImplPropMapTypes {
    ISSTANDALONE,
    XSLTCISSTANDALONE,
    CDATACHUNKSIZE,
    EXTCLSLOADER,
    ENABLEEXTFUNC,
    OVERRIDEPARSER,
    RESETSYMBOLTABLE,
    ENTITYCOUNT,
}

impl ImplPropMapTypes {
    const fn value(&self) -> ImplPropMapValue {
        use ImplPropMapTypes::*;
        match self {
            ISSTANDALONE => ImplPropMapValue { 
                name: "isStandalone",
                q_name: JdkConstants::FQ_IS_STANDALONE,
                sp_name: Some(JdkConstants::SP_IS_STANDALONE),
                differ: true,
                old_q_name: None,
                old_sp_name: None,
            },
            XSLTCISSTANDALONE => ImplPropMapValue { 
                name: "xsltcIsStandalone",
                q_name: JdkConstants::JDK_IS_STANDALONE,
                sp_name: Some(JdkConstants::SP_XSLTC_IS_STANDALONE),
                differ: true,
                old_q_name: Some(JdkConstants::ORACLE_IS_STANDALONE),
                old_sp_name: None,
            },
            CDATACHUNKSIZE => ImplPropMapValue { 
                name: "cdataChunkSize",
                q_name: JdkConstants::CDATA_CHUNK_SIZE,
                sp_name: Some(JdkConstants::CDATA_CHUNK_SIZE),
                differ: false,
                old_q_name: None,
                old_sp_name: None,
            },
            EXTCLSLOADER => ImplPropMapValue { 
                name: "extensionClassLoader",
                q_name: JdkConstants::JDK_EXT_CLASSLOADER,
                sp_name: None,
                differ: true,
                old_q_name: Some(JdkConstants::JDK_EXTENSION_CLASSLOADER),
                old_sp_name: None,
            },
            ENABLEEXTFUNC => ImplPropMapValue { 
                name: "enableExtensionFunctions",
                q_name: JdkConstants::ORACLE_ENABLE_EXTENSION_FUNCTION,
                sp_name: Some(JdkConstants::SP_ENABLE_EXTENSION_FUNCTION_SPEC),
                differ: true,
                old_q_name: None,
                old_sp_name: Some(JdkConstants::SP_ENABLE_EXTENSION_FUNCTION),
            },
            OVERRIDEPARSER => ImplPropMapValue { 
                name: "overrideDefaultParser",
                q_name: JdkConstants::OVERRIDE_PARSER,
                sp_name: Some(JdkConstants::OVERRIDE_PARSER),
                differ: false,
                old_q_name: Some(JdkConstants::ORACLE_FEATURE_SERVICE_MECHANISM),
                old_sp_name: Some(JdkConstants::ORACLE_FEATURE_SERVICE_MECHANISM),
            },
            RESETSYMBOLTABLE => ImplPropMapValue { 
                name: "resetSymbolTable",
                q_name: JdkConstants::RESET_SYMBOL_TABLE,
                sp_name: Some(JdkConstants::RESET_SYMBOL_TABLE),
                differ: false,
                old_q_name: None,
                old_sp_name: None,
            },
            ENTITYCOUNT => ImplPropMapValue { 
                name: "getEntityCountInfo",
                q_name: JdkConstants::JDK_DEBUG_LIMIT,
                sp_name: None,
                differ: true,
                old_q_name: Some(JdkConstants::JDK_ENTITY_COUNT_INFO),
                old_sp_name: None,
            },
           
        }

    }

    pub fn is(&self, name: &String) -> bool {
        let v = self.value();

        v.sp_name.map_or(false, |sp| sp.to_string() == *name) ||
        (v.differ && v.q_name.to_string() == *name) ||
        v.old_q_name.map_or(false, |old| old.to_string() == *name)
    }

    fn is_name_differ(&self) -> bool {
        self.value().differ
    }

    fn get_state(&self, name: String) -> Option<StateTypes> {
        let v = self.value();
        if v.sp_name.map_or(false, |sp| sp.to_string() == name) ||
            (v.sp_name.is_none() && v.q_name.to_string() == name) {
                return Some(StateTypes::APIPROPERTY);
        } else if (v.differ && v.q_name.to_string() == name) ||
            v.old_q_name.map_or(false, |old| old.to_string() == name) {
                return Some(StateTypes::LEGACY_APIPROPERTY);
        }
        None
    }

    pub fn q_name(&self) -> &'static str {
        self.value().q_name
    }

    fn q_name_old(&self) -> Option<&'static str> {
        self.value().old_q_name
    }

    fn system_property(&self) -> Option<&'static str> {
        self.value().sp_name
    }

    fn system_property_old(&self) -> Option<&'static str> {
        self.value().old_sp_name
    }

}

pub struct ImplPropMapValue {
    name: &'static str,
    q_name: &'static str,
    sp_name: Option<&'static str>,
    differ: bool,
    old_q_name: Option<&'static str>,
    old_sp_name: Option<&'static str>,
}


#[derive(Ordinalize, Clone, PartialEq)]
pub enum StateTypes {
    DEFAULT, 
    FSP,
    JAXPDOTPROPERTIES,
    LEGACY_SYSTEMPROPERTY,
    SYSTEMPROPERTY,
    LEGACY_APIPROPERTY,
    APIPROPERTY,
}

impl StateTypes {
    const fn value(self) -> StateValue {
        use StateTypes::*;
        match self {
            DEFAULT => StateValue{literal: "default"},
            FSP => StateValue {literal: "FEATURE_SECURE_PROCESSING"},
            JAXPDOTPROPERTIES => StateValue{literal: "jaxp.properties"},
            LEGACY_SYSTEMPROPERTY => StateValue{literal: "legacy system property"},
            SYSTEMPROPERTY => StateValue{literal: "system property"},
            LEGACY_APIPROPERTY => StateValue{literal: "legacy property" },
            APIPROPERTY => StateValue { literal: "property" },
        }

    }
}

pub struct StateValue {
    pub literal: &'static str,
}
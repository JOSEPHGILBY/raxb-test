// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/util/PropertyState.java

// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/util/FeatureState.java

use super::status::StatusType;

pub enum PropertyStateType {
    UNKNOWN,
    RECOGNIZED,
    NOT_SUPPORTED,
    NOT_RECOGNIZED,
    NOT_ALLOWED,
    DYN(PropertyStateValue),
}

impl PropertyStateType {
    pub fn value(&self) -> &PropertyStateValue {
        use PropertyStateType::*;
        match self {
            UNKNOWN => &PropertyStateValue { status: StatusType::UNKNOWN, state: None },
            RECOGNIZED => &PropertyStateValue { status: StatusType::RECOGNIZED, state: None },
            NOT_SUPPORTED => &PropertyStateValue { status: StatusType::NOT_SUPPORTED, state: None },
            NOT_RECOGNIZED => &PropertyStateValue { status: StatusType::NOT_RECOGNIZED, state: None },
            NOT_ALLOWED => &PropertyStateValue { status: StatusType::NOT_ALLOWED, state: None },
            DYN(val) => val,
        }
    }

    pub const fn of(status: StatusType) -> PropertyStateType {
        PropertyStateType::DYN(PropertyStateValue { status, state: None })
    }

    pub const fn is(value: Option<String>) -> PropertyStateType {
        PropertyStateType::DYN(PropertyStateValue { status: StatusType::SET, state: value })
    }

    pub fn is_exceptional(&self) -> bool {
        self.value().status.is_exceptional()
    }

    pub fn get_status(&self) -> &StatusType {
        &self.value().status
    }

    pub fn get_state(&self) -> Option<String> {
        self.value().state.clone()
    }
}

// TODO: dealw with object
pub struct PropertyStateValue {
    status: StatusType,
    state: Option<String>,
}

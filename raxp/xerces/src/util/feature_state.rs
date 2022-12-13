// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/util/FeatureState.java

use super::status::StatusType;

pub enum FeatureStateType {
    SET_ENABLED,
    SET_DISABLED,
    UNKNOWN,
    RECOGNIZED,
    NOT_SUPPORTED,
    NOT_RECOGNIZED,
    NOT_ALLOWED,
    DYN(FeatureStateValue),
}

impl FeatureStateType {
    pub const fn value(&self) -> &FeatureStateValue {
        use FeatureStateType::*;
        match self {
            SET_ENABLED => &FeatureStateValue { status: StatusType::SET, state: true },
            SET_DISABLED => &FeatureStateValue { status: StatusType::SET, state: false },
            UNKNOWN => &FeatureStateValue { status: StatusType::UNKNOWN, state: false },
            RECOGNIZED => &FeatureStateValue { status: StatusType::RECOGNIZED, state: false },
            NOT_SUPPORTED => &FeatureStateValue { status: StatusType::NOT_SUPPORTED, state: false },
            NOT_RECOGNIZED => &FeatureStateValue { status: StatusType::NOT_RECOGNIZED, state: false },
            NOT_ALLOWED => &FeatureStateValue { status: StatusType::NOT_ALLOWED, state: false },
            DYN(value) => value
        }
    }

    pub const fn of(status: StatusType) -> FeatureStateType {
        FeatureStateType::DYN(FeatureStateValue { status, state: false })
    }

    pub const fn is(value: bool) -> FeatureStateType {
        FeatureStateType::DYN(FeatureStateValue { status: StatusType::SET, state: value })
    }

    pub const fn is_exceptional(&self) -> bool {
        self.value().status.is_exceptional()
    }

    pub fn get_status(&self) -> &StatusType {
        &self.value().status
    }

    pub fn get_state(&self) -> bool {
        self.value().state
    }

}

pub struct FeatureStateValue {
    status: StatusType,
    state: bool,
}

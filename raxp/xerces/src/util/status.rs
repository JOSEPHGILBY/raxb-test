// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/util/Status.java

pub enum StatusType {
    SET,
    UNKNOWN,
    RECOGNIZED,
    NOT_SUPPORTED,
    NOT_RECOGNIZED,
    NOT_ALLOWED
}

impl StatusType {
    const fn value(&self) -> StatusValue {
        use StatusType::*;
        match self {
            SET => StatusValue { the_type: -3, is_exceptional: false },
            UNKNOWN => StatusValue { the_type: -2, is_exceptional: false },
            RECOGNIZED => StatusValue { the_type: -1, is_exceptional: false },
            NOT_SUPPORTED => StatusValue { the_type: 0, is_exceptional: true },
            NOT_RECOGNIZED => StatusValue { the_type: 1, is_exceptional: true },
            NOT_ALLOWED => StatusValue { the_type: 2, is_exceptional: true }
        }
    }

    pub const fn get_type(&self) -> i16 {
        self.value().the_type
    }

    pub const fn is_exceptional(&self) -> bool {
        self.value().is_exceptional
    }
}

pub struct StatusValue {
    the_type: i16,
    is_exceptional: bool
}
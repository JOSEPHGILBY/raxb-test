// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/XNIException.java

use std::fmt;

#[derive(Debug)]
pub enum XNIError {
    RuntimeError,
    AnXNIError,
    XMLConfigurationError,
}

impl XNIError {
    const fn serial_version_uid(&self) -> i64 { 
        use XNIError::*;
        match self {
            RuntimeError => -7034897190745766939,
            AnXNIError => 9019819772686063775,
            XMLConfigurationError => -5437427404547669188,
        }
    }
}

impl std::error::Error for XNIError {}

impl fmt::Display for XNIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use XNIError::*;
        match self {
            RuntimeError => write!(f, "Runtime error"),
            AnXNIError => write!(f, "XNI error"),
            XMLConfigurationError => write!(f, "XMLConfiguration error"),
        }
    }
}
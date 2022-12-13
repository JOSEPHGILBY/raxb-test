// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/xni/parser/XMLComponentManager.java

use crate::{util::{feature_state::{FeatureStateType, FeatureStateValue}, property_state::PropertyStateType}, xni::xni_error::XNIError};

pub trait XMLComponentManager {
    fn get_feature(&self, feature_id: &'static str) -> Result<bool, XNIError>;
    fn get_feature_2(&self, feature_id: &'static str, default_value: bool) -> bool;
    fn get_property(&self, property_id: &'static str) -> Result<Option<String>, XNIError>;
    fn get_property_2(&self, property_id: &'static str, default_object: String) -> Result<Option<String>, XNIError>; // TODO: Object
    fn get_feature_state(&self, feature_id: &'static str) -> FeatureStateType;
    fn get_property_state(&self, property_id: &'static str) -> Result<PropertyStateType, XNIError>;
}
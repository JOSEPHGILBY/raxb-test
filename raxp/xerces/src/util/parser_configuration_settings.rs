// jdk/src/java.xml/share/classes/com/sun/org/apache/xerces/internal/util/ParserConfigurationSettings.java

use std::{collections::HashMap, rc::Rc};

use const_format::concatcp;

use crate::{xni::{parser::xml_component_manager::XMLComponentManager, xni_error::XNIError}, implementation::constants::Constants};

use super::{feature_state::{FeatureStateType, FeatureStateValue}, property_state::PropertyStateType};

pub struct ParserConfigurationSettings {
    f_recognized_properties: Vec<&'static str>,
    f_properties: HashMap<&'static str, String>, // TODO: Object
    f_recognized_features: Vec<&'static str>,
    f_features: HashMap<&'static str, bool>,
    f_parent_settings: Option<Rc<dyn XMLComponentManager>>,
}

impl ParserConfigurationSettings {
    const PARSER_SETTINGS: &'static str = concatcp!(Constants::XERCES_FEATURE_PREFIX, Constants::PARSER_SETTINGS);

    fn new_1() -> Self {
        Self::new_2(None)
    }

    fn new_2(parent: Option<Rc<dyn XMLComponentManager>>) -> Self {
        Self {
            f_recognized_features: Vec::new(),
            f_recognized_properties: Vec::new(),
            f_features: HashMap::new(),
            f_properties: HashMap::new(),
            f_parent_settings: parent.clone()
        }
    }

    fn add_recognized_features(&mut self, feature_ids: Option<&'static [&'static str]>) {
        let Some(ids) = feature_ids else { return };
        for id in ids.iter() {
            if !self.f_recognized_features.contains(id) {
                self.f_recognized_features.push(id);
            }
        }
    }

    fn set_feature(&mut self, feature_id: &'static str, state: bool) -> Result<(), XNIError> {
        let check_state = self.check_feature(feature_id);
        if check_state.is_exceptional() { return Err(XNIError::XMLConfigurationError) }

        self.f_features.insert(feature_id, state);
        Ok(())

    }

    fn add_recognized_properties(&mut self, property_ids: &'static [&'static str]) {
        self.f_recognized_properties.extend_from_slice(property_ids);
    }

    fn set_property(&mut self, property_id: &'static str, value: String) -> Result<(), XNIError> {
        let check_state = self.check_property(property_id)?;
        if check_state.is_exceptional() { return Err(XNIError::XMLConfigurationError);}
        self.f_properties.insert(property_id, value);
        Ok(())
    }

    fn check_feature(&self, feature_id: &'static str) -> FeatureStateType {
        if self.f_recognized_features.contains(&feature_id) {
            return FeatureStateType::RECOGNIZED;
        }

        let Some(parent_settings) = self.f_parent_settings.clone() else { 
            return FeatureStateType::NOT_RECOGNIZED;
        };
        return parent_settings.get_feature_state(feature_id)
    }

    fn check_property(&self, property_id: &'static str) -> Result<PropertyStateType, XNIError> {
        if self.f_recognized_properties.contains(&property_id) { return Ok(PropertyStateType::RECOGNIZED); }

        let Some(parent_settings) = self.f_parent_settings.clone() else { return Ok(PropertyStateType::NOT_RECOGNIZED); };
        let state = parent_settings.get_property_state(property_id)?;
        if state.is_exceptional() { return Ok(state); }
        Ok(PropertyStateType::RECOGNIZED)
    }   
}

impl XMLComponentManager for ParserConfigurationSettings {
    fn get_feature(&self, feature_id: &'static str) -> Result<bool, XNIError> {
        let state = self.get_feature_state(feature_id);
        if state.is_exceptional() { return Err(XNIError::XMLConfigurationError);}
        Ok(state.get_state())
    }

    fn get_feature_2(&self, feature_id: &'static str, default_value: bool) -> bool {
        let state = self.get_feature_state(feature_id);
        if state.is_exceptional() { return default_value}
        state.get_state()
    }

    fn get_feature_state(&self, feature_id: &'static str) -> FeatureStateType {
        let state_opt = self.f_features.get(feature_id);
        if let Some(state) = state_opt {
            return FeatureStateType::is(*state);
        }

        let check_state = self.check_feature(feature_id);
        if check_state.is_exceptional() { return check_state; }
        FeatureStateType::is(false)

    }

    fn get_property(&self, property_id: &'static str) -> Result<Option<String>, XNIError> {
        let state = self.get_property_state(property_id)?;
        if state.is_exceptional() { return Err(XNIError::XMLConfigurationError);}
        Ok(state.get_state())
    }

    fn get_property_2(&self, property_id: &'static str, default_object: String) -> Result<Option<String>, XNIError> {
        let state = self.get_property_state(property_id)?;
        if state.is_exceptional() { return Ok(Some(default_object)) }
        Ok(state.get_state())
    }

    fn get_property_state(&self, property_id: &'static str) -> Result<PropertyStateType, XNIError> {
        let property_value_opt = self.f_properties.get(property_id);
        if let Some(property_value) = property_value_opt {
            return Ok(PropertyStateType::is(Some(property_value.clone())));
        }

        let state = self.check_property(property_id)?;
        if state.is_exceptional() { return Ok(state); }
        Ok(PropertyStateType::is(None))

    }
}
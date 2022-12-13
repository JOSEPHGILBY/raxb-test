// jdk/src/java.xml/share/classes/jdk/xml/internal/SecuritySupport.java


pub struct SecuritySupport {

}

impl SecuritySupport {
    #[cfg(windows)]
    const NEWLINE: &'static str = "\r\n";
    #[cfg(not(windows))]
    const NEWLINE: &'static str = "\n";

    pub fn get_error_message(locale: String, bundle: String, key: String) -> String {
        todo!()
    }

    pub fn get_system_property(prop_name: String) -> Option<String> {
        todo!()
    }

    pub fn get_system_property_def_value_fallback(prop_name: String, def_value: String) -> String {
        let value = Self::get_system_property(prop_name);
        if let Some(val) = value {
            return val;
        }
        def_value
    }

    pub fn get_system_property_from_class(the_type: String, prop_name: String, def_value: String) -> String {
        todo!()
    }

    // rename
    pub fn get_jaxp_system_property_casted(the_type: String, prop_name: String, def_value: String) -> String {
        todo!()
    }

    pub fn get_jaxp_system_property_as_string(prop_name: String) -> String {
        todo!()
    }
    
    pub fn is_directory(f: String) -> bool {
        todo!()
    }

    pub fn is_file_exists(f: String) -> bool {
        todo!()
    }

    pub fn get_file_input_stream(f: String) -> String {
        todo!()
    }

    pub fn get_resource_as_stream(name: String) -> String {
        todo!()
    }
   
    pub fn get_resource_bundle(bundle: String) -> String {
        todo!()
    }

    pub fn get_resource_bundle_2(bundle: String, locale: String) -> String {
        todo!()
    }

    pub fn does_file_exist(f: String) -> bool {
        todo!()
    }

    pub fn get_last_modified(f: String) -> i64 {
        todo!()
    }

    pub fn sanitize_path(uri: String) -> String {
        todo!()
    }

    pub fn check_access(system_id: String, allowed_protocols: String, access_any: String) -> String {
        todo!()
    }

    pub fn is_protocol_allowed(protocol: String, allowed_protocol: String) -> bool {
        todo!()
    }

    pub fn get_context_class_loader() -> String {
        todo!()
    }

    pub fn get_system_class_loader() -> String {
        todo!()
    }

    pub fn get_parent_class_loader() -> String {
        todo!()
    }

    pub fn get_class_source() -> String {
        todo!()
    }

    pub fn get_class_loader() -> String {
        todo!()
    }

    pub fn get_resource_as_stream_2() -> String {
        todo!()
    }
}
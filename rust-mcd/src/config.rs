/// Define available config parameters, see [DynamicMCDxDAS::mcd_qry_servers_f]
macro_rules! define_config {
    ($(($field_name: ident, $field_type:ty, $description: expr, $config_name: expr)),*) => {
        #[derive(Default)]
        pub struct ServerConfig {
            $(
                #[doc = $description]
                pub $field_name: Option<$field_type>
            ),*
        }

        impl ServerConfig {
            /// Returns this configuration as a string compatible to the MCD library
            pub fn as_config_string(&self) -> std::ffi::CString {
                let mut composed_string = String::new();

                $(
                    if let Some(value) = self.$field_name.as_ref() {
                        // FIXME: We might not need the double quotes for all field types
                        // FIXME: Only tested for string parameters
                        composed_string += &format!("{}=\"{}\"\n", $config_name, value);
                    }
                )*

                composed_string += "\0";

                std::ffi::CString::new(composed_string).unwrap()
            }
        }
    };
}

// Partially list of available parameters, check [DynamicMCDxDAS::mcd_qry_servers_f] when a parameter should be added
define_config!(
    (acc_hw, String, "Restricts this server to connect to devices via a specific access hardware as determined by the string.", "McdAccHw")
);

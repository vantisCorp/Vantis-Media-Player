//! Procedural macros for Vantis Plugin SDK
//!
//! This crate provides derive macros and attribute macros for
//! simplifying plugin development.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, ItemStruct};

/// Derive macro for Plugin trait implementation
///
/// # Example
/// ```ignore
/// use vantis_plugin_macros::Plugin;
///
/// #[derive(Plugin)]
/// struct MyPlugin {
///     name: String,
///     version: String,
/// }
/// ```
#[proc_macro_derive(Plugin, attributes(plugin))]
pub fn derive_plugin(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    
    let expanded = quote! {
        impl vantis_plugin_sdk::Plugin for #name {
            fn name(&self) -> &str {
                stringify!(#name)
            }
            
            fn version(&self) -> &str {
                env!("CARGO_PKG_VERSION")
            }
            
            fn on_load(&mut self) -> Result<(), vantis_plugin_sdk::PluginError> {
                Ok(())
            }
            
            fn on_unload(&mut self) -> Result<(), vantis_plugin_sdk::PluginError> {
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Attribute macro for marking plugin entry point
///
/// # Example
/// ```ignore
/// use vantis_plugin_macros::plugin_entry;
///
/// #[plugin_entry]
/// fn init_plugin() -> MyPlugin {
///     MyPlugin::new()
/// }
/// ```
#[proc_macro_attribute]
pub fn plugin_entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn vantis_plugin_init() -> *mut dyn vantis_plugin_sdk::Plugin {
            #input
            
            let plugin = #fn_name();
            Box::into_raw(Box::new(plugin)) as *mut dyn vantis_plugin_sdk::Plugin
        }
    };
    
    TokenStream::from(expanded)
}

/// Derive macro for serializable plugin configuration
///
/// # Example
/// ```ignore
/// use vantis_plugin_macros::PluginConfig;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(PluginConfig, Deserialize, Serialize)]
/// struct MyConfig {
///     enabled: bool,
///     threshold: f32,
/// }
/// ```
#[proc_macro_derive(PluginConfig, attributes(config))]
pub fn derive_plugin_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    
    let expanded = quote! {
        impl vantis_plugin_sdk::PluginConfig for #name {
            fn from_json(json: &str) -> Result<Self, vantis_plugin_sdk::PluginError> {
                serde_json::from_str(json)
                    .map_err(|e| vantis_plugin_sdk::PluginError::ConfigError(e.to_string()))
            }
            
            fn to_json(&self) -> Result<String, vantis_plugin_sdk::PluginError> {
                serde_json::to_string_pretty(self)
                    .map_err(|e| vantis_plugin_sdk::PluginError::ConfigError(e.to_string()))
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Attribute macro for creating plugin commands
///
/// # Example
/// ```ignore
/// use vantis_plugin_macros::plugin_command;
///
/// #[plugin_command(name = "process", description = "Process data")]
/// fn process_data(input: &[u8]) -> Vec<u8> {
///     input.to_vec()
/// }
/// ```
#[proc_macro_attribute]
pub fn plugin_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attr = attr;
    let input = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input.sig.ident;
    
    let expanded = quote! {
        #input
        
        #[vantis_plugin_sdk::async_trait]
        impl vantis_plugin_sdk::Command for #fn_name {
            fn name(&self) -> &str {
                stringify!(#fn_name)
            }
            
            fn description(&self) -> &str {
                ""
            }
            
            async fn execute(&self, args: vantis_plugin_sdk::CommandArgs) -> vantis_plugin_sdk::CommandResult {
                vantis_plugin_sdk::CommandResult::Success(vec![])
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Derive macro for plugin events
///
/// # Example
/// ```ignore
/// use vantis_plugin_macros::PluginEvent;
///
/// #[derive(PluginEvent)]
/// enum MyEvent {
///     Started,
///     Stopped,
///     Error(String),
/// }
/// ```
#[proc_macro_derive(PluginEvent)]
pub fn derive_plugin_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    
    let expanded = quote! {
        impl vantis_plugin_sdk::Event for #name {
            fn event_name(&self) -> &str {
                stringify!(#name)
            }
            
            fn to_payload(&self) -> Result<vantis_plugin_sdk::EventData, vantis_plugin_sdk::PluginError> {
                Ok(vantis_plugin_sdk::EventData::default())
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_plugin_parsing() {
        let input = quote! {
            #[derive(Plugin)]
            struct TestPlugin;
        };
        
        let _result: DeriveInput = syn::parse2(input).unwrap();
    }
}
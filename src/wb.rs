//! Bindings with wasm-bindgen
use crate::Globals;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mtots {
    globals: Globals,
}

#[wasm_bindgen]
impl Mtots {
    pub fn new() -> Mtots {
        Mtots {
            globals: Globals::new(),
        }
    }

    pub fn set_custom_source_finder(&mut self, f: js_sys::Function) {
        self.globals.set_custom_source_finder(move |name| {
            let ret = match f.call1(&JsValue::null(), &JsValue::from_str(name)) {
                Ok(ret) => ret,
                Err(error) => return Err(format!("source finder failed with error: {:?}", error)),
            };
            if ret == JsValue::null() || ret == JsValue::undefined() {
                Ok(None)
            } else {
                match ret.as_string() {
                    Some(string) => Ok(Some(string)),
                    None => Err(format!("source finder did not return a string ({:?}", ret)),
                }
            }
        });
    }

    pub fn load(&mut self, module_name: &str) -> bool {
        match self.globals.load(&module_name.into()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

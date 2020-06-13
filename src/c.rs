use crate::Globals;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::c_void;
use std::ffi::CString;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn mtots_new() -> *mut Globals {
    Box::leak(Box::new(Globals::new()))
}

#[no_mangle]
pub unsafe extern "C" fn mtots_set_source_finder(
    globals: *mut Globals,
    data: *mut c_void,
    f: extern "C" fn(*mut c_void, *const c_char) -> *const c_char,
) {
    if globals == std::ptr::null_mut() {
        panic!("null passed for Globals argument");
    }
    let globals: &mut Globals = &mut *globals;

    globals.set_custom_source_finder(move |name| {
        let mod_cstring = match CString::new(name) {
            Ok(s) => s,
            Err(error) => return Err(format!("{:?}", error)),
        };
        let raw_mod_cstr = mod_cstring.into_raw();
        let raw_data_cstr = f(data, raw_mod_cstr);
        CString::from_raw(raw_mod_cstr);
        let data_cstr = CStr::from_ptr(raw_data_cstr);
        match data_cstr.to_str() {
            Ok(s) => Ok(Some(s.into())),
            Err(error) => Err(format!("{:?}", error)),
        }
    });
}

/// Tries to load the given module
/// returns 1 on success, 0 on failure
#[no_mangle]
pub unsafe extern "C" fn mtots_load(
    globals: *mut Globals,
    module_name: *const c_char,
) -> c_int {
    if globals == std::ptr::null_mut() {
        panic!("null passed for Globals argument");
    }
    let globals: &mut Globals = &mut *globals;
    let module_name = CStr::from_ptr(module_name);
    let module_name = module_name.to_str().unwrap();
    match globals.load(&module_name.into()) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

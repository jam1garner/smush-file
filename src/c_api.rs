use std::os::raw::c_char;
use std::ffi::{CStr, CString};

/// Returns an string representing the contents of the file
#[no_mangle]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn get_from_extension(
    buffer: *const u8,
    len: usize,
    extension: *const c_char,
) -> *mut c_char {
    let extension = unsafe { CStr::from_ptr(extension) };
    let extension = extension.to_str().unwrap_or("");

    let buffer = unsafe { std::slice::from_raw_parts(buffer, len) };
    let description = crate::get_from_extension(buffer, extension);
    
    CString::new(description)
        .unwrap()
        .into_raw()
}

/// Free a description returned from get_from_extension
#[no_mangle]
pub unsafe extern "C" fn free_description(x: *mut c_char) {
    drop(CString::from_raw(x))
}

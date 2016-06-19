use ::ffi;

use ::libc::c_char;
use std::ffi::CStr;

// To-string converters
// See http://www.libnfc.org/api/group__string-converter.html

/// Converts nfc_modulation_type value to string
pub fn modulation_type(pnd: ffi::nfc_modulation_type) -> &'static str {
    unsafe {
        let modulation_type = CStr::from_ptr(ffi::str_nfc_modulation_type(pnd)).to_str().unwrap();

        modulation_type
    }
}

/// Converts nfc_baud_rate value to string
pub fn baud_rate(baud_rate: ffi::nfc_baud_rate) -> &'static str {
    unsafe {
        let baud_rate = CStr::from_ptr(ffi::str_nfc_baud_rate(baud_rate)).to_str().unwrap();

        baud_rate
    }
}

/// Returns the number of characters printed
pub fn target(buf: *mut *mut c_char, pnt: *mut ffi::nfc_target, verbose: u8) -> i32 {
    unsafe { ffi::str_nfc_target(buf, pnt, verbose) }
}
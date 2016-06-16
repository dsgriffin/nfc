use ::ffi;

use libc::{c_char, size_t};
use std::ffi::CStr;

/// Error reporting
/// See http://www.libnfc.org/api/group__error.html

pub fn strerror(pnd: *const ffi::nfc_device) -> &'static str {
    unsafe {
        let last_error = CStr::from_ptr(ffi::nfc_strerror(pnd)).to_str().unwrap();

        last_error
    }
}

pub fn strerror_r(pnd: *const ffi::nfc_device, pc_str_err_buf: *mut c_char, sz_buf_len: size_t) -> i32 {
    unsafe { ffi::nfc_strerror_r(pnd, pc_str_err_buf, sz_buf_len) }
}

pub fn perror(pnd: *const ffi::nfc_device, pc_string: *const c_char) {
    unsafe { ffi::nfc_perror(pnd, pc_string); }
}

pub fn device_get_last_error(pnd: *const ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_device_get_last_error(pnd) }
}
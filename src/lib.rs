#![crate_name = "nfc"]
#![crate_type = "dylib"]

extern crate libc;

mod ffi;
pub mod initiator;
pub mod target;
pub mod device;

use libc::{c_char, size_t};
use std::ffi::CStr;

/// Library initialization/deinitialization

pub fn register_driver(ndr: *const ffi::nfc_driver) -> i32 {
    unsafe { ffi::nfc_register_driver(ndr) }
}

pub fn init(context: *mut *mut ffi::nfc_context) {
    unsafe { ffi::nfc_init(context); }
}

pub fn exit(context: *mut ffi::nfc_context) {
    unsafe { ffi::nfc_exit(context); }
}

/// NFC Device/Hardware manipulation

pub fn open(context: *mut ffi::nfc_context, connstring: ffi::nfc_connstring) -> *mut ffi::nfc_device {
    unsafe { ffi::nfc_open(context, connstring) }
}

pub fn close(pnd: *mut ffi::nfc_device) {
    unsafe { ffi::nfc_close(pnd); }
}

pub fn list_devices(context: *mut ffi::nfc_context, connstrings: *mut ffi::nfc_connstring, constrings_len: size_t) -> size_t {
    unsafe { ffi::nfc_list_devices(context, connstrings, constrings_len) }
}

pub fn idle(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_idle(pnd) }
}

pub fn abort_command(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_abort_command(pnd) }
}

/// Error reporting

pub fn strerror(pnd: *const ffi::nfc_device) -> &'static str {
    unsafe {
        let last_error = CStr::from_ptr(ffi::nfc_strerror(pnd)).to_str().unwrap();

        last_error
    }
}

pub fn strerror_r(pnd: *const ffi::nfc_device, pcStrErrBuf: *mut c_char, szBufLen: size_t) -> i32 {
    unsafe { ffi::nfc_strerror_r(pnd, pcStrErrBuf, szBufLen) }
}

pub fn perror(pnd: *const ffi::nfc_device, pcString: *const c_char) {
    unsafe { ffi::nfc_perror(pnd, pcString); }
}

pub fn device_get_last_error(pnd: *const ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_device_get_last_error(pnd) }
}

/// Miscellaneous

pub fn version() -> &'static str {
    unsafe {
        let version = CStr::from_ptr(ffi::nfc_version()).to_str().unwrap();

        version
    }
}

pub fn free(p: *mut std::os::raw::c_void) {
    unsafe { ffi::nfc_free(p) }
}

pub fn device_get_information_about(pnd: *mut ffi::nfc_device, buf: *mut *mut c_char) -> i32 {
    unsafe { ffi::nfc_device_get_information_about(pnd, buf) }
}

/// To-string converters

pub fn str_modulation_type(pnd: ffi::nfc_modulation_type) -> &'static str {
    unsafe {
        let modulation_type = CStr::from_ptr(ffi::str_nfc_modulation_type(pnd)).to_str().unwrap();

        modulation_type
    }
}

pub fn str_baud_rate(baud_rate: ffi::nfc_baud_rate) -> &'static str {
    unsafe {
        let baud_rate = CStr::from_ptr(ffi::str_nfc_baud_rate(baud_rate)).to_str().unwrap();

        baud_rate
    }
}

pub fn str_target(buf: *mut *mut c_char, pnt: *mut ffi::nfc_target, verbose: u8) -> i32 {
    unsafe { ffi::str_nfc_target(buf, pnt, verbose) }
}
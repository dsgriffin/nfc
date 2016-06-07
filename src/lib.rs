#![crate_name = "nfc"]
#![crate_type = "dylib"]

extern crate libc;

mod ffi;

use std::ffi::CStr;

pub fn register_driver(ndr: *const ffi::nfc_driver) {
    unsafe {
        ffi::nfc_register_driver(ndr);
    }
}

pub fn init(context: *mut *mut ffi::nfc_context) {
    unsafe {
        ffi::nfc_init(context);
    }
}

pub fn exit(context: *mut ffi::nfc_context) {
    unsafe {
        ffi::nfc_exit(context);
    }
}


pub fn open(context: *mut ffi::nfc_context, connstring: ffi::nfc_connstring) {
    unsafe {
        ffi::nfc_open(context, connstring);
    }
}


pub fn strerror(pnd: *const ffi::nfc_device) -> &'static str {
    unsafe {
        let last_error = CStr::from_ptr(ffi::nfc_strerror(pnd)).to_str().unwrap();

        last_error
    }
}

pub fn device_get_last_error(pnd: *const ffi::nfc_device) -> i32 {
    unsafe {
        ffi::nfc_device_get_last_error(pnd)
    }
}

pub fn str_modulation_type(pnd: ffi::nfc_modulation_type) -> &'static str {
    unsafe {
        let modulation_type = CStr::from_ptr(ffi::str_nfc_modulation_type(pnd)).to_str().unwrap();

        modulation_type
    }
}

pub fn version() -> &'static str {
    unsafe {
        let version = CStr::from_ptr(ffi::nfc_version()).to_str().unwrap();

        version
    }
}

pub fn str_baud_rate(baud_rate: ffi::nfc_baud_rate) -> &'static str {
    unsafe {
        let baud_rate = CStr::from_ptr(ffi::str_nfc_baud_rate(baud_rate)).to_str().unwrap();

        baud_rate
    }
}
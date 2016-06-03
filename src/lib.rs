#![crate_name = "nfc"]
#![crate_type = "dylib"]

extern crate libc;

mod ffi;

use libc::{uint8_t, uint32_t, size_t};
use std::ptr;
use std::ffi::CStr;
use std::str;
use std::os::raw::c_void;

pub fn device_get_information_about(pnd: ffi::nfc_device, buf: *mut isize) -> i8 {
    4
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
extern crate nfc_sys;

use nfc_sys::nfc_version;

use std::ffi::CStr;
use std::str;

pub fn version() -> &'static str {
    unsafe {
        let version = CStr::from_ptr(nfc_version()).to_str().unwrap();

        version
    }
}
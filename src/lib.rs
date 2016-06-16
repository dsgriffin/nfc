#![crate_name = "nfc"]
#![crate_type = "dylib"]

extern crate libc;

mod ffi;

pub mod initiator;
pub mod target;
pub mod device;
pub mod error;
pub mod misc;
pub mod to_str;

use libc::size_t;

/// Library initialization/deinitialization
/// See http://www.libnfc.org/api/group__lib.html

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
/// http://www.libnfc.org/api/group__dev.html

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

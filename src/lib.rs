#![crate_name = "nfc"]
#![crate_type = "dylib"]

extern crate libc;

pub mod ffi;

pub mod initiator;
pub mod target;
pub mod device;
pub mod context;
pub mod error;
pub mod misc;
pub mod to_str;

use libc::size_t;

// Library initialization/deinitialization
// See http://www.libnfc.org/api/group__lib.html

/// Registers an NFC device driver with libnfc
pub fn register_driver(ndr: *const ffi::nfc_driver) -> i32 {
    unsafe { ffi::nfc_register_driver(ndr) }
}

/// Initializes libnfc. This function must be called before calling any other libnfc function
pub fn init(context: *mut *mut ffi::nfc_context) {
    unsafe { ffi::nfc_init(context); }
}

/// Deinitializes libnfc. Should be called after closing all open devices and before your application terminates
pub fn exit(context: *mut ffi::nfc_context) {
    unsafe { ffi::nfc_exit(context); }
}

// NFC Device/Hardware manipulation
// http://www.libnfc.org/api/group__dev.html

/// Open an NFC device
pub fn open(context: *mut ffi::nfc_context, connstring: ffi::nfc_connstring) -> *mut ffi::nfc_device {
    unsafe { ffi::nfc_open(context, connstring) }
}

/// Close from a NFC device
pub fn close(pnd: *mut ffi::nfc_device) {
    unsafe { ffi::nfc_close(pnd); }
}

/// Scan for discoverable supported devices
pub fn list_devices(context: *mut ffi::nfc_context, connstrings: *mut ffi::nfc_connstring, constrings_len: size_t) -> size_t {
    unsafe { ffi::nfc_list_devices(context, connstrings, constrings_len) }
}

/// Switches the NFC device to idle mode
pub fn idle(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_idle(pnd) }
}

/// Aborts current running command
pub fn abort_command(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_abort_command(pnd) }
}

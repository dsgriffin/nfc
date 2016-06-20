use ::ffi;

use ::libc::c_char;
use ::std::ffi::CStr;

// Internal

/// Decode Connstring
pub fn connstring_decode(connstring: ffi::nfc_connstring, driver_name: *const ::std::os::raw::c_char, bus_name: *const ::std::os::raw::c_char, pparam1: *mut *mut ::std::os::raw::c_char, pparam2: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    unsafe { ffi::connstring_decode(connstring, driver_name, bus_name, pparam1, pparam2) }
}

/// String as Boolean
pub fn string_as_boolean(s: *const ::std::os::raw::c_char, value: *mut u8) {
    unsafe { ffi::string_as_boolean(s, value) }
}

/// Add cascade tags (0x88) in UID.
pub fn iso14443_cascade_uid(abt_uid: *mut u8, sz_uid: usize, pbt_cascaded_uid: *mut u8, psz_cascaded_uid: *mut usize) {
    unsafe { ffi::iso14443_cascade_uid(abt_uid, sz_uid, pbt_cascaded_uid, psz_cascaded_uid) }
}

// Miscellaneous
// See http://www.libnfc.org/api/group__misc.html

/// Returns the library version
pub fn version() -> &'static str {
    unsafe {
        let version = CStr::from_ptr(ffi::nfc_version()).to_str().unwrap();

        version
    }
}

/// Frees buffer allocated by libnfc
pub fn free(p: *mut ::std::os::raw::c_void) {
    unsafe { ffi::nfc_free(p) }
}

/// Prints information about the NFC device
pub fn device_get_information_about(pnd: *mut ffi::nfc_device, buf: *mut *mut c_char) -> i32 {
    unsafe { ffi::nfc_device_get_information_about(pnd, buf) }
}
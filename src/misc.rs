use ::ffi;

use ::libc::c_char;
use ::std::ffi::CStr;

/// Miscellaneous
/// See http://www.libnfc.org/api/group__misc.html

pub fn version() -> &'static str {
    unsafe {
        let version = CStr::from_ptr(ffi::nfc_version()).to_str().unwrap();

        version
    }
}

pub fn free(p: *mut ::std::os::raw::c_void) {
    unsafe { ffi::nfc_free(p) }
}

pub fn device_get_information_about(pnd: *mut ffi::nfc_device, buf: *mut *mut c_char) -> i32 {
    unsafe { ffi::nfc_device_get_information_about(pnd, buf) }
}
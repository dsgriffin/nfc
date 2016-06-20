use ::ffi;

use std::ffi::CStr;

// Internal

/// New Device
pub fn new(context: *const ffi::nfc_context, connstring: ffi::nfc_connstring) -> *mut ffi::nfc_device {
    unsafe { ffi::nfc_device_new(context, connstring) }
}

/// Free Device
pub fn free(dev: *mut ffi::nfc_device) {
    unsafe { ffi::nfc_device_free(dev) }
}

// Special data accessors
// See http://www.libnfc.org/api/group__data.html

/// Gets the device's name
pub fn get_name(pnd: *mut ffi::nfc_device) -> &'static str {
    unsafe {
        let name = CStr::from_ptr(ffi::nfc_device_get_name(pnd)).to_str().unwrap();

        name
    }
}

/// Gets the device's connection string
pub fn get_connstring(pnd: *mut ffi::nfc_device) -> &'static str {
    unsafe {
        let connstring = CStr::from_ptr(ffi::nfc_device_get_connstring(pnd)).to_str().unwrap();

        connstring
    }
}

/// Get supported modulations
pub fn get_supported_modulation(pnd: *mut ffi::nfc_device, mode: ffi::nfc_mode, supported_mt: *mut *const ffi::nfc_modulation_type) -> i32 {
    unsafe { ffi::nfc_device_get_supported_modulation(pnd, mode, supported_mt) }
}

/// Get supported baud rates
pub fn get_supported_baud_rate(pnd: *mut ffi::nfc_device, nmt: ffi::nfc_modulation_type, supported_br: *mut *const ffi::nfc_baud_rate) -> i32 {
    unsafe { ffi::nfc_device_get_supported_baud_rate(pnd, nmt, supported_br) }
}

// Properties accessors
// See http://www.libnfc.org/api/group__properties.html

/// Set a device's integer-property value
pub fn set_property_int(pnd: *mut ffi::nfc_device, property: ffi::nfc_property, value: i32) -> i32 {
    unsafe { ffi::nfc_device_set_property_int(pnd, property, value) }
}

/// Set a device's boolean-property value
pub fn set_property_bool(pnd: *mut ffi::nfc_device, property: ffi::nfc_property, b_enable: u8) -> i32 {
    unsafe { ffi::nfc_device_set_property_bool(pnd, property, b_enable) }
}

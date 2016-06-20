use ::ffi;

// Internal methods

/// New NFC context
pub fn new() -> *mut ffi::nfc_context {
    unsafe { ffi::nfc_context_new() }
}

/// Free NFC context
pub fn free(context: *mut ffi::nfc_context) {
    unsafe { ffi::nfc_context_free(context) }
}
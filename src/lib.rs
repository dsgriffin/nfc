#![crate_name = "nfc"]
#![crate_type = "dylib"]

extern crate libc;

mod ffi;

use libc::c_char;
use libc::size_t;
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

/// NFC initiator

pub fn initiator_init(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init(pnd) }
}

pub fn initiator_init_secure_element(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init_secure_element(pnd) }
}

pub fn initiator_select_passive_target(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, pbtInitData: *mut u8, szInitData: size_t, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_select_passive_target(pnd, nm, pbtInitData, szInitData, pnt) }
}

pub fn initiator_list_passive_targets(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, ant: *mut ffi::nfc_target, szTargets: size_t) -> i32 {
    unsafe { ffi::nfc_initiator_list_passive_targets(pnd, nm, ant, szTargets) }
}

pub fn initiator_poll_target(pnd: *mut ffi::nfc_device, pnmModulations: *const ffi::nfc_modulation, szModulations: size_t, uiPollNr: u8, uiPeriod: u8, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_poll_target(pnd, pnmModulations, szModulations, uiPollNr, uiPeriod, pnt) }
}

pub fn initiator_select_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndiInitiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_select_dep_target(pnd, ndm, nbr, pndiInitiator, pnt, timeout) }
}

pub fn initiator_poll_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndiInitiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_poll_dep_target(pnd, ndm, nbr, pndiInitiator, pnt, timeout) }
}

pub fn initiator_transceive_bytes(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTx: size_t, pbtRx: *mut u8, szRx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes(pnd, pbtTx, szTx, pbtRx, szRx, timeout) }
}

pub fn initiator_transceive_bits(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTxBits: size_t, pbtTxPar: *mut u8, pbtRx: *mut u8, szRx: size_t, pbtRxPar: *mut u8) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits(pnd, pbtTx, szTxBits, pbtTxPar, pbtRx, szRx, pbtRxPar) }
}

pub fn initiator_transceive_bytes_timed(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTx: size_t, pbtRx: *mut u8, szRx: size_t, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes_timed(pnd, pbtTx, szTx, pbtRx, szRx, cycles) }
}

pub fn initiator_target_is_present(pnd: *mut ffi::nfc_device, pnt: *const ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_target_is_present(pnd, pnt) }
}

pub fn initiator_transceive_bits_timed(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTxBits: size_t, pbtTxPar: *mut u8, pbtRx: *mut u8, szRx: size_t, pbtRxPar: *mut u8, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits_timed(pnd, pbtTx, szTxBits, pbtTxPar, pbtRx, szRx, pbtRxPar, cycles) }
}

/// NFC target

pub fn target_init(pnd: *mut ffi::nfc_device, pnt: *mut ffi::nfc_target, pbtRx: *mut u8, szRx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_init(pnd, pnt, pbtRx, szRx, timeout) }
}

pub fn target_send_bytes(pnd: *mut ffi::nfc_device, pbtTx: *mut u8, szTx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_send_bytes(pnd, pbtTx, szTx, timeout) }
}

pub fn target_receive_bytes(pnd: *mut ffi::nfc_device, pbtRx: *mut u8, szRx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_receive_bytes(pnd, pbtRx, szRx, timeout) }
}

pub fn target_send_bits(pnd: *mut ffi::nfc_device, pbtTx: *mut u8, szTx: size_t, pbtTxPar: *mut u8) -> i32 {
    unsafe { ffi::nfc_target_send_bits(pnd, pbtTx, szTx, pbtTxPar) }
}

pub fn target_receive_bits(pnd: *mut ffi::nfc_device, pbtRx: *mut u8, szRx: size_t, pbtRxPar: *mut u8) -> i32 {
    unsafe { ffi::nfc_target_receive_bits(pnd, pbtRx, szRx, pbtRxPar) }
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

/// Special data accessors

pub fn device_get_name(pnd: *mut ffi::nfc_device) -> &'static str {
    unsafe {
        let name = CStr::from_ptr(ffi::nfc_device_get_name(pnd)).to_str().unwrap();

        name
    }
}

pub fn device_get_connstring(pnd: *mut ffi::nfc_device) -> &'static str {
    unsafe {
        let connstring = CStr::from_ptr(ffi::nfc_device_get_connstring(pnd)).to_str().unwrap();

        connstring
    }
}

pub fn device_get_supported_modulation(pnd: *mut ffi::nfc_device, mode: ffi::nfc_mode, supported_mt: *mut *const ffi::nfc_modulation_type) -> i32 {
    unsafe { ffi::nfc_device_get_supported_modulation(pnd, mode, supported_mt) }
}

pub fn device_get_supported_baud_rate(pnd: *mut ffi::nfc_device, nmt: ffi::nfc_modulation_type, supported_br: *mut *const ffi::nfc_baud_rate) -> i32 {
    unsafe { ffi::nfc_device_get_supported_baud_rate(pnd, nmt, supported_br) }
}

/// Properties accessors

pub fn device_set_property_int(pnd: *mut ffi::nfc_device, property: ffi::nfc_property, value: i32) -> i32 {
    unsafe { ffi::nfc_device_set_property_int(pnd, property, value) }
}

pub fn device_set_property_bool(pnd: *mut ffi::nfc_device, property: ffi::nfc_property, bEnable: u8) -> i32 {
    unsafe { ffi::nfc_device_set_property_bool(pnd, property, bEnable) }
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
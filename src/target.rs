use ::ffi;
use ::libc::size_t;

/// NFC target

pub fn init(pnd: *mut ffi::nfc_device, pnt: *mut ffi::nfc_target, pbtRx: *mut u8, szRx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_init(pnd, pnt, pbtRx, szRx, timeout) }
}

pub fn send_bytes(pnd: *mut ffi::nfc_device, pbtTx: *mut u8, szTx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_send_bytes(pnd, pbtTx, szTx, timeout) }
}

pub fn receive_bytes(pnd: *mut ffi::nfc_device, pbtRx: *mut u8, szRx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_receive_bytes(pnd, pbtRx, szRx, timeout) }
}

pub fn send_bits(pnd: *mut ffi::nfc_device, pbtTx: *mut u8, szTx: size_t, pbtTxPar: *mut u8) -> i32 {
    unsafe { ffi::nfc_target_send_bits(pnd, pbtTx, szTx, pbtTxPar) }
}

pub fn receive_bits(pnd: *mut ffi::nfc_device, pbtRx: *mut u8, szRx: size_t, pbtRxPar: *mut u8) -> i32 {
    unsafe { ffi::nfc_target_receive_bits(pnd, pbtRx, szRx, pbtRxPar) }
}
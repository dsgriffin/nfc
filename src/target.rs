use ::ffi;

use ::libc::size_t;

// NFC target
// See http://www.libnfc.org/api/group__target.html

/// Initializes the NFC device as an emulated tag
pub fn init(pnd: *mut ffi::nfc_device, pnt: *mut ffi::nfc_target, pbt_rx: *mut u8, sz_rx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_init(pnd, pnt, pbt_rx, sz_rx, timeout) }
}

/// Sends bytes and APDU frames
pub fn send_bytes(pnd: *mut ffi::nfc_device, pbt_tx: *mut u8, sz_tx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_send_bytes(pnd, pbt_tx, sz_tx, timeout) }
}

/// Receives bytes and APDU frames
pub fn receive_bytes(pnd: *mut ffi::nfc_device, pbt_rx: *mut u8, sz_rx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_target_receive_bytes(pnd, pbt_rx, sz_rx, timeout) }
}

/// Sends raw bit-frames
pub fn send_bits(pnd: *mut ffi::nfc_device, pbt_tx: *mut u8, sz_tx: size_t, pbt_tx_par: *mut u8) -> i32 {
    unsafe { ffi::nfc_target_send_bits(pnd, pbt_tx, sz_tx, pbt_tx_par) }
}

/// Receives bit-frames
pub fn receive_bits(pnd: *mut ffi::nfc_device, pbt_rx: *mut u8, sz_rx: size_t, pbt_rx_par: *mut u8) -> i32 {
    unsafe { ffi::nfc_target_receive_bits(pnd, pbt_rx, sz_rx, pbt_rx_par) }
}
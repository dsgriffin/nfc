use ::ffi;
use ::libc::size_t;

/// NFC initiator
/// See http://www.libnfc.org/api/group__initiator.html

pub fn init(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init(pnd) }
}

pub fn init_secure_element(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init_secure_element(pnd) }
}

pub fn select_passive_target(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, pbt_init_data: *mut u8, sz_init_data: size_t, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_select_passive_target(pnd, nm, pbt_init_data, sz_init_data, pnt) }
}

pub fn list_passive_targets(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, ant: *mut ffi::nfc_target, sz_targets: size_t) -> i32 {
    unsafe { ffi::nfc_initiator_list_passive_targets(pnd, nm, ant, sz_targets) }
}

pub fn poll_target(pnd: *mut ffi::nfc_device, pnm_modulations: *const ffi::nfc_modulation, sz_modulations: size_t, ui_poll_nr: u8, ui_period: u8, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_poll_target(pnd, pnm_modulations, sz_modulations, ui_poll_nr, ui_period, pnt) }
}

pub fn select_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndi_initiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_select_dep_target(pnd, ndm, nbr, pndi_initiator, pnt, timeout) }
}

pub fn poll_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndi_initiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_poll_dep_target(pnd, ndm, nbr, pndi_initiator, pnt, timeout) }
}

pub fn transceive_bytes(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx: size_t, pbt_rx: *mut u8, sz_rx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes(pnd, pbt_tx, sz_tx, pbt_rx, sz_rx, timeout) }
}

pub fn transceive_bits(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx_bits: size_t, pbt_tx_par: *mut u8, pbt_rx: *mut u8, sz_rx: size_t, pbt_rx_par: *mut u8) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits(pnd, pbt_tx, sz_tx_bits, pbt_tx_par, pbt_rx, sz_rx, pbt_rx_par) }
}

pub fn transceive_bytes_timed(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx: size_t, pbt_rx: *mut u8, sz_rx: size_t, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes_timed(pnd, pbt_tx, sz_tx, pbt_rx, sz_rx, cycles) }
}

pub fn target_is_present(pnd: *mut ffi::nfc_device, pnt: *const ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_target_is_present(pnd, pnt) }
}

pub fn transceive_bits_timed(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx_bits: size_t, pbt_tx_par: *mut u8, pbt_rx: *mut u8, sz_rx: size_t, pbt_rx_par: *mut u8, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits_timed(pnd, pbt_tx, sz_tx_bits, pbt_tx_par, pbt_rx, sz_rx, pbt_rx_par, cycles) }
}
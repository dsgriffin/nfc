use ::ffi;
use ::libc::size_t;

// Internal

/// Prepare Initiator data
pub fn prepare_initiator_data(nm: ffi::nfc_modulation, ppbt_initiator_data: *mut *mut u8, psz_initiator_data: *mut usize) {
    unsafe { ffi::prepare_initiator_data(nm, ppbt_initiator_data, psz_initiator_data) }
}

// NFC initiator methods
// See http://www.libnfc.org/api/group__initiator.html

/// Initialize an NFC device as initiator (reader)
pub fn init(pnd: Box<*mut ffi::nfc_device>) -> i32 {
    unsafe { ffi::nfc_initiator_init(*pnd) }
}

/// Initialize NFC device as initiator with its secure element initiator (reader)
pub fn init_secure_element(pnd: &mut *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init_secure_element(*pnd) }
}

/// Select a passive or emulated tag
pub fn select_passive_target(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, pbt_init_data: *mut u8, sz_init_data: size_t, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_select_passive_target(pnd, nm, pbt_init_data, sz_init_data, pnt) }
}

/// List passive or emulated tags
pub fn list_passive_targets(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, ant: *mut ffi::nfc_target, sz_targets: size_t) -> i32 {
    unsafe { ffi::nfc_initiator_list_passive_targets(pnd, nm, ant, sz_targets) }
}

/// Polling for NFC targets
pub fn poll_target(pnd: *mut ffi::nfc_device, pnm_modulations: *const ffi::nfc_modulation, sz_modulations: size_t, ui_poll_nr: u8, ui_period: u8, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_poll_target(pnd, pnm_modulations, sz_modulations, ui_poll_nr, ui_period, pnt) }
}

/// Select a target and request active or passive mode for D.E.P. (Data Exchange Protocol)
pub fn select_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndi_initiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_select_dep_target(pnd, ndm, nbr, pndi_initiator, pnt, timeout) }
}

/// Poll a target and request active or passive mode for D.E.P. (Data Exchange Protocol)
pub fn poll_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndi_initiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_poll_dep_target(pnd, ndm, nbr, pndi_initiator, pnt, timeout) }
}

/// Send data to target then retrieve data from target
pub fn transceive_bytes(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx: size_t, pbt_rx: *mut u8, sz_rx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes(pnd, pbt_tx, sz_tx, pbt_rx, sz_rx, timeout) }
}

/// Transceive raw bit-frames to a target
pub fn transceive_bits(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx_bits: size_t, pbt_tx_par: *mut u8, pbt_rx: *mut u8, sz_rx: size_t, pbt_rx_par: *mut u8) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits(pnd, pbt_tx, sz_tx_bits, pbt_tx_par, pbt_rx, sz_rx, pbt_rx_par) }
}

/// Send data to target then retrieve data from target with a precise cycles counter which indicates the number of cycles between emission & reception of frames
pub fn transceive_bytes_timed(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx: size_t, pbt_rx: *mut u8, sz_rx: size_t, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes_timed(pnd, pbt_tx, sz_tx, pbt_rx, sz_rx, cycles) }
}

/// Check target presence
pub fn target_is_present(pnd: *mut ffi::nfc_device, pnt: *const ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_target_is_present(pnd, pnt) }
}

/// Transceive raw bit-frames to a target with a precise cycles counter that indicates the number of cycles between emission & reception of frames
pub fn transceive_bits_timed(pnd: *mut ffi::nfc_device, pbt_tx: *const u8, sz_tx_bits: size_t, pbt_tx_par: *mut u8, pbt_rx: *mut u8, sz_rx: size_t, pbt_rx_par: *mut u8, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits_timed(pnd, pbt_tx, sz_tx_bits, pbt_tx_par, pbt_rx, sz_rx, pbt_rx_par, cycles) }
}
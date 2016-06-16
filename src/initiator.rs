use ::ffi;
use ::libc::size_t;

/// NFC initiator

pub fn init(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init(pnd) }
}

pub fn init_secure_element(pnd: *mut ffi::nfc_device) -> i32 {
    unsafe { ffi::nfc_initiator_init_secure_element(pnd) }
}

pub fn select_passive_target(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, pbtInitData: *mut u8, szInitData: size_t, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_select_passive_target(pnd, nm, pbtInitData, szInitData, pnt) }
}

pub fn list_passive_targets(pnd: *mut ffi::nfc_device, nm: ffi::nfc_modulation, ant: *mut ffi::nfc_target, szTargets: size_t) -> i32 {
    unsafe { ffi::nfc_initiator_list_passive_targets(pnd, nm, ant, szTargets) }
}

pub fn poll_target(pnd: *mut ffi::nfc_device, pnmModulations: *const ffi::nfc_modulation, szModulations: size_t, uiPollNr: u8, uiPeriod: u8, pnt: *mut ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_poll_target(pnd, pnmModulations, szModulations, uiPollNr, uiPeriod, pnt) }
}

pub fn select_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndiInitiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_select_dep_target(pnd, ndm, nbr, pndiInitiator, pnt, timeout) }
}

pub fn poll_dep_target(pnd: *mut ffi::nfc_device, ndm: ffi::nfc_dep_mode, nbr: ffi::nfc_baud_rate, pndiInitiator: *const ffi::nfc_dep_info, pnt: *mut ffi::nfc_target, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_poll_dep_target(pnd, ndm, nbr, pndiInitiator, pnt, timeout) }
}

pub fn transceive_bytes(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTx: size_t, pbtRx: *mut u8, szRx: size_t, timeout: i32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes(pnd, pbtTx, szTx, pbtRx, szRx, timeout) }
}

pub fn transceive_bits(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTxBits: size_t, pbtTxPar: *mut u8, pbtRx: *mut u8, szRx: size_t, pbtRxPar: *mut u8) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits(pnd, pbtTx, szTxBits, pbtTxPar, pbtRx, szRx, pbtRxPar) }
}

pub fn transceive_bytes_timed(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTx: size_t, pbtRx: *mut u8, szRx: size_t, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bytes_timed(pnd, pbtTx, szTx, pbtRx, szRx, cycles) }
}

pub fn target_is_present(pnd: *mut ffi::nfc_device, pnt: *const ffi::nfc_target) -> i32 {
    unsafe { ffi::nfc_initiator_target_is_present(pnd, pnt) }
}

pub fn transceive_bits_timed(pnd: *mut ffi::nfc_device, pbtTx: *const u8, szTxBits: size_t, pbtTxPar: *mut u8, pbtRx: *mut u8, szRx: size_t, pbtRxPar: *mut u8, cycles: *mut u32) -> i32 {
    unsafe { ffi::nfc_initiator_transceive_bits_timed(pnd, pbtTx, szTxBits, pbtTxPar, pbtRx, szRx, pbtRxPar, cycles) }
}
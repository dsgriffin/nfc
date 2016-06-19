#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[link(name = "nfc", kind = "dylib")]

use libc::{uint8_t, uint32_t, size_t};

pub enum Struct_nfc_context { }
pub type nfc_context = Struct_nfc_context;
pub enum Struct_nfc_device { }
pub type nfc_device = Struct_nfc_device;
pub enum Struct_nfc_driver { }
pub type nfc_driver = Struct_nfc_driver;
pub type nfc_connstring = [::std::os::raw::c_char; 1024usize];
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed1 {
    NP_TIMEOUT_COMMAND = 0,
    NP_TIMEOUT_ATR = 1,
    NP_TIMEOUT_COM = 2,
    NP_HANDLE_CRC = 3,
    NP_HANDLE_PARITY = 4,
    NP_ACTIVATE_FIELD = 5,
    NP_ACTIVATE_CRYPTO1 = 6,
    NP_INFINITE_SELECT = 7,
    NP_ACCEPT_INVALID_FRAMES = 8,
    NP_ACCEPT_MULTIPLE_FRAMES = 9,
    NP_AUTO_ISO14443_4 = 10,
    NP_EASY_FRAMING = 11,
    NP_FORCE_ISO14443_A = 12,
    NP_FORCE_ISO14443_B = 13,
    NP_FORCE_SPEED_106 = 14,
}
pub type nfc_property = Enum_Unnamed1;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed2 { NDM_UNDEFINED = 0, NDM_PASSIVE = 1, NDM_ACTIVE = 2, }
pub type nfc_dep_mode = Enum_Unnamed2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub abtNFCID3: [uint8_t; 10usize],
    pub btDID: uint8_t,
    pub btBS: uint8_t,
    pub btBR: uint8_t,
    pub btTO: uint8_t,
    pub btPP: uint8_t,
    pub abtGB: [uint8_t; 48usize],
    pub szGB: size_t,
    pub ndm: nfc_dep_mode,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_dep_info = Struct_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub abtAtqa: [uint8_t; 2usize],
    pub btSak: uint8_t,
    pub szUidLen: size_t,
    pub abtUid: [uint8_t; 10usize],
    pub szAtsLen: size_t,
    pub abtAts: [uint8_t; 254usize],
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443a_info = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed5 {
    pub szLen: size_t,
    pub btResCode: uint8_t,
    pub abtId: [uint8_t; 8usize],
    pub abtPad: [uint8_t; 8usize],
    pub abtSysCode: [uint8_t; 2usize],
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_felica_info = Struct_Unnamed5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed6 {
    pub abtPupi: [uint8_t; 4usize],
    pub abtApplicationData: [uint8_t; 4usize],
    pub abtProtocolInfo: [uint8_t; 3usize],
    pub ui8CardIdentifier: uint8_t,
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443b_info = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub abtDIV: [uint8_t; 4usize],
    pub btVerLog: uint8_t,
    pub btConfig: uint8_t,
    pub szAtrLen: size_t,
    pub abtAtr: [uint8_t; 33usize],
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443bi_info = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed8 {
    pub abtUID: [uint8_t; 8usize],
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443b2sr_info = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed9 {
    pub abtUID: [uint8_t; 4usize],
    pub btProdCode: uint8_t,
    pub btFabCode: uint8_t,
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443b2ct_info = Struct_Unnamed9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed10 {
    pub btSensRes: [uint8_t; 2usize],
    pub btId: [uint8_t; 4usize],
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_jewel_info = Struct_Unnamed10;
#[repr(C)]
//#[derive(Copy, Clone)]
pub struct Union_Unnamed11 {
    pub _bindgen_data_: [u8; 283usize],
}
impl Union_Unnamed11 {
    pub unsafe fn nai(&mut self) -> *mut nfc_iso14443a_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nfi(&mut self) -> *mut nfc_felica_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nbi(&mut self) -> *mut nfc_iso14443b_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nii(&mut self) -> *mut nfc_iso14443bi_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nsi(&mut self) -> *mut nfc_iso14443b2sr_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nci(&mut self) -> *mut nfc_iso14443b2ct_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nji(&mut self) -> *mut nfc_jewel_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ndi(&mut self) -> *mut nfc_dep_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_target_info = Union_Unnamed11;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed12 {
    NBR_UNDEFINED = 0,
    NBR_106 = 1,
    NBR_212 = 2,
    NBR_424 = 3,
    NBR_847 = 4,
}
pub type nfc_baud_rate = Enum_Unnamed12;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed13 {
    NMT_ISO14443A = 1,
    NMT_JEWEL = 2,
    NMT_ISO14443B = 3,
    NMT_ISO14443BI = 4,
    NMT_ISO14443B2SR = 5,
    NMT_ISO14443B2CT = 6,
    NMT_FELICA = 7,
    NMT_DEP = 8,
}
pub type nfc_modulation_type = Enum_Unnamed13;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed14 { N_TARGET = 0, N_INITIATOR = 1, }
pub type nfc_mode = Enum_Unnamed14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed15 {
    pub nmt: nfc_modulation_type,
    pub nbr: nfc_baud_rate,
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_modulation = Struct_Unnamed15;
#[repr(C)]
//#[derive(Copy, Clone)]
pub struct Struct_Unnamed16 {
    pub nti: nfc_target_info,
    pub nm: nfc_modulation,
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_target = Struct_Unnamed16;

#[link(name = "nfc", kind = "dylib")]
extern "C" {
    pub fn nfc_init(context: *mut *mut nfc_context);
    pub fn nfc_exit(context: *mut nfc_context);
    pub fn nfc_register_driver(driver: *const nfc_driver)
        -> ::std::os::raw::c_int;
    pub fn nfc_open(context: *mut nfc_context, connstring: nfc_connstring)
        -> *mut nfc_device;
    pub fn nfc_close(pnd: *mut nfc_device);
    pub fn nfc_abort_command(pnd: *mut nfc_device) -> ::std::os::raw::c_int;
    pub fn nfc_list_devices(context: *mut nfc_context,
        connstrings: *mut nfc_connstring,
        connstrings_len: size_t) -> size_t;
    pub fn nfc_idle(pnd: *mut nfc_device) -> ::std::os::raw::c_int;
    pub fn nfc_initiator_init(pnd: *mut nfc_device) -> ::std::os::raw::c_int;
    pub fn nfc_initiator_init_secure_element(pnd: *mut nfc_device)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_select_passive_target(pnd: *mut nfc_device,
        nm: nfc_modulation,
        pbtInitData: *const uint8_t,
        szInitData: size_t,
        pnt: *mut nfc_target)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_list_passive_targets(pnd: *mut nfc_device,
        nm: nfc_modulation,
        ant: *mut nfc_target,
        szTargets: size_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_poll_target(pnd: *mut nfc_device,
        pnmTargetTypes: *const nfc_modulation,
        szTargetTypes: size_t, uiPollNr: uint8_t,
        uiPeriod: uint8_t, pnt: *mut nfc_target)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_select_dep_target(pnd: *mut nfc_device,
        ndm: nfc_dep_mode,
        nbr: nfc_baud_rate,
        pndiInitiator: *const nfc_dep_info,
        pnt: *mut nfc_target,
        timeout: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_poll_dep_target(pnd: *mut nfc_device,
        ndm: nfc_dep_mode,
        nbr: nfc_baud_rate,
        pndiInitiator: *const nfc_dep_info,
        pnt: *mut nfc_target,
        timeout: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_deselect_target(pnd: *mut nfc_device)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bytes(pnd: *mut nfc_device,
        pbtTx: *const uint8_t, szTx: size_t,
        pbtRx: *mut uint8_t, szRx: size_t,
        timeout: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bits(pnd: *mut nfc_device,
        pbtTx: *const uint8_t,
        szTxBits: size_t,
        pbtTxPar: *const uint8_t,
        pbtRx: *mut uint8_t, szRx: size_t,
        pbtRxPar: *mut uint8_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bytes_timed(pnd: *mut nfc_device,
        pbtTx: *const uint8_t,
        szTx: size_t,
        pbtRx: *mut uint8_t,
        szRx: size_t,
        cycles: *mut uint32_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bits_timed(pnd: *mut nfc_device,
        pbtTx: *const uint8_t,
        szTxBits: size_t,
        pbtTxPar: *const uint8_t,
        pbtRx: *mut uint8_t,
        szRx: size_t,
        pbtRxPar: *mut uint8_t,
        cycles: *mut uint32_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_initiator_target_is_present(pnd: *mut nfc_device,
        pnt: *const nfc_target)
        -> ::std::os::raw::c_int;
    pub fn nfc_target_init(pnd: *mut nfc_device, pnt: *mut nfc_target,
        pbtRx: *mut uint8_t, szRx: size_t,
        timeout: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_target_send_bytes(pnd: *mut nfc_device, pbtTx: *const uint8_t,
        szTx: size_t, timeout: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_target_receive_bytes(pnd: *mut nfc_device, pbtRx: *mut uint8_t,
        szRx: size_t,
        timeout: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_target_send_bits(pnd: *mut nfc_device, pbtTx: *const uint8_t,
        szTxBits: size_t, pbtTxPar: *const uint8_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_target_receive_bits(pnd: *mut nfc_device, pbtRx: *mut uint8_t,
        szRx: size_t, pbtRxPar: *mut uint8_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_strerror(pnd: *const nfc_device)
        -> *const ::std::os::raw::c_char;
    pub fn nfc_strerror_r(pnd: *const nfc_device,
        buf: *mut ::std::os::raw::c_char, buflen: size_t)
        -> ::std::os::raw::c_int;
    pub fn nfc_perror(pnd: *const nfc_device,
        s: *const ::std::os::raw::c_char);
    pub fn nfc_device_get_last_error(pnd: *const nfc_device)
        -> ::std::os::raw::c_int;
    pub fn nfc_device_get_name(pnd: *mut nfc_device)
        -> *const ::std::os::raw::c_char;
    pub fn nfc_device_get_connstring(pnd: *mut nfc_device)
        -> *const ::std::os::raw::c_char;
    pub fn nfc_device_get_supported_modulation(pnd: *mut nfc_device,
        mode: nfc_mode,
        supported_mt:
        *mut *const nfc_modulation_type)
        -> ::std::os::raw::c_int;
    pub fn nfc_device_get_supported_baud_rate(pnd: *mut nfc_device,
        nmt: nfc_modulation_type,
        supported_br:
        *mut *const nfc_baud_rate)
        -> ::std::os::raw::c_int;
    pub fn nfc_device_get_supported_baud_rate_target_mode(pnd:
    *mut nfc_device,
        nmt:
        nfc_modulation_type,
        supported_br:
        *mut *const nfc_baud_rate)
        -> ::std::os::raw::c_int;
    pub fn nfc_device_set_property_int(pnd: *mut nfc_device,
        property: nfc_property,
        value: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn nfc_device_set_property_bool(pnd: *mut nfc_device,
        property: nfc_property, bEnable: u8)
        -> ::std::os::raw::c_int;
    pub fn iso14443a_crc(pbtData: *mut uint8_t, szLen: size_t,
        pbtCrc: *mut uint8_t);
    pub fn iso14443a_crc_append(pbtData: *mut uint8_t, szLen: size_t);
    pub fn iso14443b_crc(pbtData: *mut uint8_t, szLen: size_t,
        pbtCrc: *mut uint8_t);
    pub fn iso14443b_crc_append(pbtData: *mut uint8_t, szLen: size_t);
    pub fn iso14443a_locate_historical_bytes(pbtAts: *mut uint8_t,
        szAts: size_t,
        pszTk: *mut size_t)
        -> *mut uint8_t;
    pub fn nfc_free(p: *mut ::std::os::raw::c_void);
    pub fn nfc_version() -> *const ::std::os::raw::c_char;
    pub fn nfc_device_get_information_about(pnd: *mut nfc_device,
        buf:
        *mut *mut ::std::os::raw::c_char)
        -> ::std::os::raw::c_int;
    pub fn str_nfc_modulation_type(nmt: nfc_modulation_type)
        -> *const ::std::os::raw::c_char;
    pub fn str_nfc_baud_rate(nbr: nfc_baud_rate)
        -> *const ::std::os::raw::c_char;
    pub fn str_nfc_target(buf: *mut *mut ::std::os::raw::c_char,
        pnt: *const nfc_target, verbose: u8)
        -> ::std::os::raw::c_int;
}

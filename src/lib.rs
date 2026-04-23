//! Safe Rust bindings for `libnfc` built on top of [`nfc-sys`].
//!
//! The [`ffi`] module re-exports the raw `nfc-sys` API for advanced use cases.
//! The top-level API in this crate focuses on ownership, resource cleanup, and
//! slice-based wrappers for the parts of libnfc that can be expressed safely.
//!
//! Some libnfc capabilities remain raw-only because the C API itself requires
//! unchecked callbacks or driver pointers. In particular, custom driver
//! registration and target emulation state machines still live in [`ffi`].
//!
//! # Native Dependency
//!
//! This crate links against a system installation of `libnfc`. You must install
//! the native library before building or running code that depends on `nfc`.
//! The underlying `nfc-sys` build script handles the actual linking step.
//!
//! Common setups:
//!
//! - macOS: `brew install libnfc`
//! - Debian/Ubuntu: install the `libnfc` development package
//! - Custom installs: configure your linker environment as needed for `libnfc`
//!
//! If you need a libnfc capability that is not exposed through the safe API,
//! use [`ffi`] directly and keep the native library installed as above.
//!
//! # Example
//!
//! ```no_run
//! use nfc::{version, Context};
//!
//! fn main() -> nfc::Result<()> {
//!     let context = Context::new()?;
//!     println!("libnfc version: {}", version());
//!
//!     for connstring in context.list_devices(8)? {
//!         println!("found device: {connstring}");
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod ffi;

pub use crate::error::{Error, Result};

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr::{self, NonNull};
use std::rc::Rc;

/// Owns a libnfc context and releases it with `nfc_exit` on drop.
///
/// Create this with [`Context::new`] and keep it alive for as long as any
/// [`Device`] values borrowed from it remain open.
pub struct Context {
    raw: NonNull<ffi::nfc_context>,
    _not_send_sync: PhantomData<Rc<()>>,
}

/// Owns an open NFC device and closes it with `nfc_close` on drop.
///
/// A `Device` borrows its parent [`Context`], which keeps the lifetime relation
/// between the native libnfc handles explicit in Rust.
pub struct Device<'ctx> {
    raw: NonNull<ffi::nfc_device>,
    _context: PhantomData<&'ctx Context>,
    _not_send_sync: PhantomData<Rc<()>>,
}

/// Result of a timed transceive operation.
///
/// libnfc reports both the amount of received data and the number of hardware
/// cycles spent on the exchange.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimedResponse {
    /// Number of bytes or bits reported by libnfc, depending on the call used.
    pub received: usize,
    /// Number of cycles reported by libnfc.
    pub cycles: u32,
}

impl Context {
    /// Initializes libnfc and returns an owned context.
    ///
    /// This requires a working native `libnfc` installation at runtime. If
    /// libnfc cannot initialize, this returns an [`Error`].
    pub fn new() -> Result<Self> {
        let mut raw = ptr::null_mut();
        unsafe {
            ffi::nfc_init(&mut raw);
        }

        let raw = NonNull::new(raw).ok_or_else(|| Error::new("libnfc returned a null context"))?;

        Ok(Self {
            raw,
            _not_send_sync: PhantomData,
        })
    }

    /// Returns the borrowed raw libnfc context pointer.
    ///
    /// This is mainly useful when integrating with [`ffi`].
    pub fn as_ptr(&self) -> *mut ffi::nfc_context {
        self.raw.as_ptr()
    }

    /// Opens an NFC device.
    ///
    /// Pass `None` to let libnfc choose the default device, or a connection
    /// string previously returned by [`Context::list_devices`].
    ///
    /// The returned [`Device`] closes itself automatically when dropped.
    pub fn open(&self, connstring: Option<&str>) -> Result<Device<'_>> {
        let connstring = match connstring {
            Some(value) => Some(
                CString::new(value)
                    .map_err(|_| Error::new("connection string contains an interior NUL byte"))?,
            ),
            None => None,
        };

        let raw = unsafe {
            ffi::nfc_open(
                self.raw.as_ptr(),
                connstring.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            )
        };

        let raw = NonNull::new(raw).ok_or_else(|| Error::new("libnfc could not open an NFC device"))?;

        Ok(Device {
            raw,
            _context: PhantomData,
            _not_send_sync: PhantomData,
        })
    }

    /// Lists up to `max` discoverable device connection strings.
    ///
    /// The strings returned here can be passed back into [`Context::open`] to
    /// target a specific reader.
    pub fn list_devices(&self, max: usize) -> Result<Vec<String>> {
        if max == 0 {
            return Ok(Vec::new());
        }

        let mut connstrings = vec![[0 as c_char; ffi::NFC_BUFSIZE_CONNSTRING]; max];
        let count = unsafe {
            ffi::nfc_list_devices(self.raw.as_ptr(), connstrings.as_mut_ptr(), connstrings.len())
        };

        Ok(connstrings
            .into_iter()
            .take(count)
            .map(|connstring| c_char_array_to_string(&connstring))
            .collect())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::nfc_exit(self.raw.as_ptr());
        }
    }
}

impl Device<'_> {
    /// Returns the borrowed raw libnfc device pointer.
    ///
    /// This is mainly useful when integrating with [`ffi`].
    pub fn as_ptr(&self) -> *mut ffi::nfc_device {
        self.raw.as_ptr()
    }

    /// Returns the device name as a borrowed C string.
    ///
    /// This avoids allocation and is useful when interoperating with other
    /// C-oriented APIs.
    pub fn name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::nfc_device_get_name(self.raw.as_ptr())) }
    }

    /// Returns the device name as UTF-8, replacing invalid bytes if needed.
    pub fn name(&self) -> Cow<'_, str> {
        self.name_cstr().to_string_lossy()
    }

    /// Returns the device connection string as a borrowed C string.
    pub fn connstring_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::nfc_device_get_connstring(self.raw.as_ptr())) }
    }

    /// Returns the device connection string as UTF-8, replacing invalid bytes if needed.
    pub fn connstring(&self) -> Cow<'_, str> {
        self.connstring_cstr().to_string_lossy()
    }

    /// Returns the last libnfc error code recorded for this device.
    ///
    /// This is mostly helpful for diagnostics; normal safe API calls already
    /// convert failures into [`Error`].
    pub fn last_error_code(&self) -> i32 {
        unsafe { ffi::nfc_device_get_last_error(self.raw.as_ptr()) }
    }

    /// Returns the last libnfc error message recorded for this device.
    pub fn last_error_message(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(ffi::nfc_strerror(self.raw.as_ptr())).to_string_lossy() }
    }

    /// Returns the human-readable information string for this device.
    ///
    /// libnfc allocates the underlying C string and this wrapper frees it for
    /// you before returning an owned [`String`].
    pub fn information_about(&self) -> Result<String> {
        let mut buffer = ptr::null_mut();
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_device_get_information_about(self.raw.as_ptr(), &mut buffer) },
        )?;

        unsafe { owned_libnfc_string(buffer) }
    }

    /// Returns the supported modulation types for the requested mode.
    ///
    /// Use [`ffi::nfc_mode::N_INITIATOR`] for reader mode and
    /// [`ffi::nfc_mode::N_TARGET`] for emulation mode.
    pub fn supported_modulations(
        &self,
        mode: ffi::nfc_mode,
    ) -> Result<Vec<ffi::nfc_modulation_type>> {
        let mut values = ptr::null();
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_device_get_supported_modulation(self.raw.as_ptr(), mode, &mut values) },
        )?;

        decode_modulation_types(values)
    }

    /// Returns the supported baud rates for a modulation type.
    pub fn supported_baud_rates(
        &self,
        modulation_type: ffi::nfc_modulation_type,
    ) -> Result<Vec<ffi::nfc_baud_rate>> {
        let mut values = ptr::null();
        check_code(
            self.raw.as_ptr(),
            unsafe {
                ffi::nfc_device_get_supported_baud_rate(self.raw.as_ptr(), modulation_type, &mut values)
            },
        )?;

        decode_baud_rates(values)
    }

    /// Returns the supported baud rates for target mode.
    pub fn supported_target_baud_rates(
        &self,
        modulation_type: ffi::nfc_modulation_type,
    ) -> Result<Vec<ffi::nfc_baud_rate>> {
        let mut values = ptr::null();
        check_code(
            self.raw.as_ptr(),
            unsafe {
                ffi::nfc_device_get_supported_baud_rate_target_mode(
                    self.raw.as_ptr(),
                    modulation_type,
                    &mut values,
                )
            },
        )?;

        decode_baud_rates(values)
    }

    /// Sets an integer device property.
    ///
    /// This is the safe wrapper over `nfc_device_set_property_int`.
    pub fn set_property_int(&mut self, property: ffi::nfc_property, value: i32) -> Result<()> {
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_device_set_property_int(self.raw.as_ptr(), property, value) },
        )?;
        Ok(())
    }

    /// Sets a boolean device property.
    ///
    /// This is the safe wrapper over `nfc_device_set_property_bool`.
    pub fn set_property_bool(&mut self, property: ffi::nfc_property, enabled: bool) -> Result<()> {
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_device_set_property_bool(self.raw.as_ptr(), property, enabled) },
        )?;
        Ok(())
    }

    /// Initializes this device in initiator mode.
    ///
    /// Call this before reader-style operations such as
    /// [`Device::select_passive_target`] or [`Device::transceive_bytes`].
    pub fn initiator_init(&mut self) -> Result<()> {
        check_code(self.raw.as_ptr(), unsafe { ffi::nfc_initiator_init(self.raw.as_ptr()) })?;
        Ok(())
    }

    /// Initializes this device's secure element in initiator mode.
    pub fn initiator_init_secure_element(&mut self) -> Result<()> {
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_initiator_init_secure_element(self.raw.as_ptr()) },
        )?;
        Ok(())
    }

    /// Selects a passive target and returns it if one was found.
    ///
    /// `init_data` is the optional initialization payload expected by libnfc
    /// for the selected modulation. An empty slice means "no init data".
    pub fn select_passive_target(
        &mut self,
        modulation: ffi::nfc_modulation,
        init_data: &[u8],
    ) -> Result<Option<ffi::nfc_target>> {
        let mut target = MaybeUninit::<ffi::nfc_target>::uninit();
        let result = unsafe {
            ffi::nfc_initiator_select_passive_target(
                self.raw.as_ptr(),
                modulation,
                ptr_or_null(init_data),
                init_data.len(),
                target.as_mut_ptr(),
            )
        };

        if result < 0 {
            return Err(Error::from_device(self.raw.as_ptr(), result));
        }

        if result == 0 {
            return Ok(None);
        }

        Ok(Some(unsafe { target.assume_init() }))
    }

    /// Lists passive targets matching the requested modulation.
    ///
    /// `max_targets` controls the maximum number of targets libnfc will write
    /// into the returned vector.
    pub fn list_passive_targets(
        &mut self,
        modulation: ffi::nfc_modulation,
        max_targets: usize,
    ) -> Result<Vec<ffi::nfc_target>> {
        let mut targets = Vec::<MaybeUninit<ffi::nfc_target>>::with_capacity(max_targets);
        let result = unsafe {
            ffi::nfc_initiator_list_passive_targets(
                self.raw.as_ptr(),
                modulation,
                targets.as_mut_ptr().cast(),
                max_targets,
            )
        };

        if result < 0 {
            return Err(Error::from_device(self.raw.as_ptr(), result));
        }

        let count = result as usize;
        unsafe {
            targets.set_len(count);
        }

        Ok(targets
            .into_iter()
            .map(|target| unsafe { target.assume_init() })
            .collect())
    }

    /// Polls for a target using one or more modulations.
    ///
    /// `poll_count` and `period` are passed directly to libnfc and therefore
    /// use libnfc's native polling semantics.
    pub fn poll_target(
        &mut self,
        modulations: &[ffi::nfc_modulation],
        poll_count: u8,
        period: u8,
    ) -> Result<Option<ffi::nfc_target>> {
        let mut target = MaybeUninit::<ffi::nfc_target>::uninit();
        let result = unsafe {
            ffi::nfc_initiator_poll_target(
                self.raw.as_ptr(),
                ptr_or_null(modulations),
                modulations.len(),
                poll_count,
                period,
                target.as_mut_ptr(),
            )
        };

        if result < 0 {
            return Err(Error::from_device(self.raw.as_ptr(), result));
        }

        if result == 0 {
            return Ok(None);
        }

        Ok(Some(unsafe { target.assume_init() }))
    }

    /// Selects a DEP target and returns it if one was found.
    ///
    /// Pass `None` for `initiator_info` when you do not need to provide custom
    /// DEP negotiation data.
    pub fn select_dep_target(
        &mut self,
        mode: ffi::nfc_dep_mode,
        baud_rate: ffi::nfc_baud_rate,
        initiator_info: Option<&ffi::nfc_dep_info>,
        timeout: i32,
    ) -> Result<Option<ffi::nfc_target>> {
        let mut target = MaybeUninit::<ffi::nfc_target>::uninit();
        let result = unsafe {
            ffi::nfc_initiator_select_dep_target(
                self.raw.as_ptr(),
                mode,
                baud_rate,
                initiator_info.map_or(ptr::null(), |value| value),
                target.as_mut_ptr(),
                timeout,
            )
        };

        if result < 0 {
            return Err(Error::from_device(self.raw.as_ptr(), result));
        }

        if result == 0 {
            return Ok(None);
        }

        Ok(Some(unsafe { target.assume_init() }))
    }

    /// Polls for a DEP target and returns it if one was found.
    pub fn poll_dep_target(
        &mut self,
        mode: ffi::nfc_dep_mode,
        baud_rate: ffi::nfc_baud_rate,
        initiator_info: Option<&ffi::nfc_dep_info>,
        timeout: i32,
    ) -> Result<Option<ffi::nfc_target>> {
        let mut target = MaybeUninit::<ffi::nfc_target>::uninit();
        let result = unsafe {
            ffi::nfc_initiator_poll_dep_target(
                self.raw.as_ptr(),
                mode,
                baud_rate,
                initiator_info.map_or(ptr::null(), |value| value),
                target.as_mut_ptr(),
                timeout,
            )
        };

        if result < 0 {
            return Err(Error::from_device(self.raw.as_ptr(), result));
        }

        if result == 0 {
            return Ok(None);
        }

        Ok(Some(unsafe { target.assume_init() }))
    }

    /// Deselects the currently selected target.
    pub fn deselect_target(&mut self) -> Result<()> {
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_initiator_deselect_target(self.raw.as_ptr()) },
        )?;
        Ok(())
    }

    /// Sends bytes to the current target and writes the reply into `rx`.
    ///
    /// The returned value is the number of bytes written into `rx`.
    ///
    /// If `rx` is too small, libnfc reports an error and this method returns
    /// that error instead of truncating the response.
    pub fn transceive_bytes(&mut self, tx: &[u8], rx: &mut [u8], timeout: i32) -> Result<usize> {
        let result = unsafe {
            ffi::nfc_initiator_transceive_bytes(
                self.raw.as_ptr(),
                ptr_or_null(tx),
                tx.len(),
                mut_ptr_or_null(rx),
                rx.len(),
                timeout,
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Sends bit frames to the current target and writes the reply into `rx`.
    ///
    /// The returned value is the number of bits reported by libnfc.
    ///
    /// `tx_bits` lets you send a partial final byte. Parity buffers are passed
    /// through to libnfc unchanged when provided.
    pub fn transceive_bits(
        &mut self,
        tx: &[u8],
        tx_bits: usize,
        tx_parity: Option<&[u8]>,
        rx: &mut [u8],
        rx_parity: Option<&mut [u8]>,
    ) -> Result<usize> {
        validate_bit_frame_args(tx, tx_bits, tx_parity, rx, rx_parity.as_deref())?;

        let result = unsafe {
            ffi::nfc_initiator_transceive_bits(
                self.raw.as_ptr(),
                ptr_or_null(tx),
                tx_bits,
                tx_parity.map_or(ptr::null(), ptr_or_null),
                mut_ptr_or_null(rx),
                rx.len(),
                rx_parity.map_or(ptr::null_mut(), mut_ptr_or_null),
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Sends bytes and returns both the received length and libnfc cycle count.
    pub fn transceive_bytes_timed(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
    ) -> Result<TimedResponse> {
        let mut cycles = 0;
        let result = unsafe {
            ffi::nfc_initiator_transceive_bytes_timed(
                self.raw.as_ptr(),
                ptr_or_null(tx),
                tx.len(),
                mut_ptr_or_null(rx),
                rx.len(),
                &mut cycles,
            )
        };

        Ok(TimedResponse {
            received: check_code(self.raw.as_ptr(), result)? as usize,
            cycles,
        })
    }

    /// Sends bit frames and returns both the received length and libnfc cycle count.
    pub fn transceive_bits_timed(
        &mut self,
        tx: &[u8],
        tx_bits: usize,
        tx_parity: Option<&[u8]>,
        rx: &mut [u8],
        rx_parity: Option<&mut [u8]>,
    ) -> Result<TimedResponse> {
        validate_bit_frame_args(tx, tx_bits, tx_parity, rx, rx_parity.as_deref())?;

        let mut cycles = 0;
        let result = unsafe {
            ffi::nfc_initiator_transceive_bits_timed(
                self.raw.as_ptr(),
                ptr_or_null(tx),
                tx_bits,
                tx_parity.map_or(ptr::null(), ptr_or_null),
                mut_ptr_or_null(rx),
                rx.len(),
                rx_parity.map_or(ptr::null_mut(), mut_ptr_or_null),
                &mut cycles,
            )
        };

        Ok(TimedResponse {
            received: check_code(self.raw.as_ptr(), result)? as usize,
            cycles,
        })
    }

    /// Checks whether the provided target is still present.
    ///
    /// Returns `Ok(false)` when libnfc reports that the target has been
    /// released, and `Err` for other libnfc failures.
    pub fn target_is_present(&mut self, target: &ffi::nfc_target) -> Result<bool> {
        let result = unsafe { ffi::nfc_initiator_target_is_present(self.raw.as_ptr(), target) };
        if result == ffi::NFC_ETGRELEASED {
            return Ok(false);
        }

        check_code(self.raw.as_ptr(), result)?;
        Ok(true)
    }

    /// Initializes this device in target emulation mode.
    ///
    /// The returned value is the number of bytes written into `rx`.
    ///
    /// This covers libnfc's basic target mode setup. More advanced emulator
    /// callback state machines are still available only through [`ffi`].
    pub fn target_init(
        &mut self,
        target: &mut ffi::nfc_target,
        rx: &mut [u8],
        timeout: i32,
    ) -> Result<usize> {
        let result = unsafe {
            ffi::nfc_target_init(
                self.raw.as_ptr(),
                target,
                mut_ptr_or_null(rx),
                rx.len(),
                timeout,
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Sends bytes while acting as a target.
    pub fn target_send_bytes(&mut self, tx: &[u8], timeout: i32) -> Result<usize> {
        let result = unsafe {
            ffi::nfc_target_send_bytes(
                self.raw.as_ptr(),
                ptr_or_null(tx),
                tx.len(),
                timeout,
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Receives bytes while acting as a target.
    pub fn target_receive_bytes(&mut self, rx: &mut [u8], timeout: i32) -> Result<usize> {
        let result = unsafe {
            ffi::nfc_target_receive_bytes(
                self.raw.as_ptr(),
                mut_ptr_or_null(rx),
                rx.len(),
                timeout,
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Sends raw bit frames while acting as a target.
    pub fn target_send_bits(
        &mut self,
        tx: &[u8],
        tx_bits: usize,
        tx_parity: Option<&[u8]>,
    ) -> Result<usize> {
        validate_tx_bits_args(tx, tx_bits, tx_parity)?;

        let result = unsafe {
            ffi::nfc_target_send_bits(
                self.raw.as_ptr(),
                ptr_or_null(tx),
                tx_bits,
                tx_parity.map_or(ptr::null(), ptr_or_null),
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Receives raw bit frames while acting as a target.
    pub fn target_receive_bits(
        &mut self,
        rx: &mut [u8],
        rx_parity: Option<&mut [u8]>,
    ) -> Result<usize> {
        validate_rx_parity_args(rx, rx_parity.as_deref())?;

        let result = unsafe {
            ffi::nfc_target_receive_bits(
                self.raw.as_ptr(),
                mut_ptr_or_null(rx),
                rx.len(),
                rx_parity.map_or(ptr::null_mut(), mut_ptr_or_null),
            )
        };

        Ok(check_code(self.raw.as_ptr(), result)? as usize)
    }

    /// Switches the device to idle mode.
    pub fn idle(&mut self) -> Result<()> {
        check_code(self.raw.as_ptr(), unsafe { ffi::nfc_idle(self.raw.as_ptr()) })?;
        Ok(())
    }

    /// Aborts the currently running command.
    pub fn abort_command(&mut self) -> Result<()> {
        check_code(
            self.raw.as_ptr(),
            unsafe { ffi::nfc_abort_command(self.raw.as_ptr()) },
        )?;
        Ok(())
    }
}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::nfc_close(self.raw.as_ptr());
        }
    }
}

/// Returns the linked libnfc version string.
///
/// This requires the native `libnfc` library to be available at runtime.
pub fn version_cstr() -> &'static CStr {
    unsafe { CStr::from_ptr(ffi::nfc_version()) }
}

/// Returns the linked libnfc version string as UTF-8.
pub fn version() -> Cow<'static, str> {
    version_cstr().to_string_lossy()
}

/// Converts a modulation type to the corresponding libnfc display string.
pub fn modulation_type_name(value: ffi::nfc_modulation_type) -> Cow<'static, str> {
    unsafe { CStr::from_ptr(ffi::str_nfc_modulation_type(value)).to_string_lossy() }
}

/// Converts a baud rate to the corresponding libnfc display string.
pub fn baud_rate_name(value: ffi::nfc_baud_rate) -> Cow<'static, str> {
    unsafe { CStr::from_ptr(ffi::str_nfc_baud_rate(value)).to_string_lossy() }
}

/// Formats an NFC target using libnfc's built-in target formatter.
///
/// For lower-level formatting control, use the raw formatter in [`ffi`].
pub fn target_to_string(target: &ffi::nfc_target, verbose: bool) -> Result<String> {
    let mut buffer = ptr::null_mut();
    let result = unsafe { ffi::str_nfc_target(&mut buffer, target, verbose) };
    if result < 0 {
        return Err(Error::new(format!("libnfc could not format target: error code {result}")));
    }

    unsafe { owned_libnfc_string(buffer) }
}

/// Computes the ISO14443-A CRC for the provided data.
///
/// This allocates a temporary owned buffer because libnfc expects a mutable
/// pointer even though the payload itself is not modified.
pub fn iso14443a_crc(data: &[u8]) -> [u8; 2] {
    let mut owned = data.to_vec();
    let mut crc = [0u8; 2];
    unsafe {
        ffi::iso14443a_crc(owned.as_mut_ptr(), data.len(), crc.as_mut_ptr());
    }
    crc
}

/// Appends an ISO14443-A CRC to the provided buffer.
///
/// The buffer is extended by exactly two bytes.
pub fn append_iso14443a_crc(data: &mut Vec<u8>) {
    let payload_len = data.len();
    data.resize(payload_len + 2, 0);
    unsafe {
        ffi::iso14443a_crc_append(data.as_mut_ptr(), payload_len);
    }
}

/// Computes the ISO14443-B CRC for the provided data.
pub fn iso14443b_crc(data: &[u8]) -> [u8; 2] {
    let mut owned = data.to_vec();
    let mut crc = [0u8; 2];
    unsafe {
        ffi::iso14443b_crc(owned.as_mut_ptr(), data.len(), crc.as_mut_ptr());
    }
    crc
}

/// Appends an ISO14443-B CRC to the provided buffer.
///
/// The buffer is extended by exactly two bytes.
pub fn append_iso14443b_crc(data: &mut Vec<u8>) {
    let payload_len = data.len();
    data.resize(payload_len + 2, 0);
    unsafe {
        ffi::iso14443b_crc_append(data.as_mut_ptr(), payload_len);
    }
}

/// Locates the ISO14443-A historical bytes inside an ATS buffer.
///
/// Returns a subslice of `ats` when libnfc can identify the historical bytes,
/// or `None` if no such region is present.
pub fn locate_iso14443a_historical_bytes(ats: &mut [u8]) -> Option<&mut [u8]> {
    let mut historical_len = 0;
    let ptr = unsafe {
        ffi::iso14443a_locate_historical_bytes(ats.as_mut_ptr(), ats.len(), &mut historical_len)
    };

    if ptr.is_null() {
        return None;
    }

    let start = unsafe { ptr.offset_from(ats.as_ptr()) };
    if start < 0 {
        return None;
    }

    let start = start as usize;
    let end = start.checked_add(historical_len)?;
    if end > ats.len() {
        return None;
    }

    Some(&mut ats[start..end])
}

fn check_code(device: *mut ffi::nfc_device, code: i32) -> Result<i32> {
    if code < 0 {
        Err(Error::from_device(device, code))
    } else {
        Ok(code)
    }
}

fn c_char_array_to_string(connstring: &ffi::nfc_connstring) -> String {
    unsafe { CStr::from_ptr(connstring.as_ptr()).to_string_lossy().into_owned() }
}

unsafe fn owned_libnfc_string(buffer: *mut c_char) -> Result<String> {
    if buffer.is_null() {
        return Ok(String::new());
    }

    struct OwnedString(*mut c_char);

    impl Drop for OwnedString {
        fn drop(&mut self) {
            unsafe {
                ffi::nfc_free(self.0.cast());
            }
        }
    }

    let buffer = OwnedString(buffer);
    Ok(CStr::from_ptr(buffer.0).to_string_lossy().into_owned())
}

fn decode_modulation_types(
    mut values: *const ffi::nfc_modulation_type,
) -> Result<Vec<ffi::nfc_modulation_type>> {
    let mut out = Vec::new();
    while !values.is_null() {
        let raw = unsafe { *(values as *const i32) };
        let value = match raw {
            0 => break,
            1 => ffi::nfc_modulation_type::NMT_ISO14443A,
            2 => ffi::nfc_modulation_type::NMT_JEWEL,
            3 => ffi::nfc_modulation_type::NMT_ISO14443B,
            4 => ffi::nfc_modulation_type::NMT_ISO14443BI,
            5 => ffi::nfc_modulation_type::NMT_ISO14443B2SR,
            6 => ffi::nfc_modulation_type::NMT_ISO14443B2CT,
            7 => ffi::nfc_modulation_type::NMT_FELICA,
            8 => ffi::nfc_modulation_type::NMT_DEP,
            9 => ffi::nfc_modulation_type::NMT_BARCODE,
            10 => ffi::nfc_modulation_type::NMT_ISO14443BICLASS,
            _ => return Err(Error::new(format!("libnfc returned an unknown modulation type: {raw}"))),
        };
        out.push(value);
        values = unsafe { values.add(1) };
    }
    Ok(out)
}

fn decode_baud_rates(mut values: *const ffi::nfc_baud_rate) -> Result<Vec<ffi::nfc_baud_rate>> {
    let mut out = Vec::new();
    while !values.is_null() {
        let raw = unsafe { *(values as *const i32) };
        let value = match raw {
            0 => break,
            1 => ffi::nfc_baud_rate::NBR_106,
            2 => ffi::nfc_baud_rate::NBR_212,
            3 => ffi::nfc_baud_rate::NBR_424,
            4 => ffi::nfc_baud_rate::NBR_847,
            _ => return Err(Error::new(format!("libnfc returned an unknown baud rate: {raw}"))),
        };
        out.push(value);
        values = unsafe { values.add(1) };
    }
    Ok(out)
}

fn ptr_or_null<T>(slice: &[T]) -> *const T {
    if slice.is_empty() {
        ptr::null()
    } else {
        slice.as_ptr()
    }
}

fn mut_ptr_or_null<T>(slice: &mut [T]) -> *mut T {
    if slice.is_empty() {
        ptr::null_mut()
    } else {
        slice.as_mut_ptr()
    }
}

fn validate_bit_frame_args(
    tx: &[u8],
    tx_bits: usize,
    tx_parity: Option<&[u8]>,
    rx: &[u8],
    rx_parity: Option<&[u8]>,
) -> Result<()> {
    validate_tx_bits_args(tx, tx_bits, tx_parity)?;
    validate_rx_parity_args(rx, rx_parity)
}

fn validate_tx_bits_args(tx: &[u8], tx_bits: usize, tx_parity: Option<&[u8]>) -> Result<()> {
    let required_tx_bytes = tx_bits.div_ceil(8);
    if required_tx_bytes > tx.len() {
        return Err(Error::new(format!(
            "tx_bits ({tx_bits}) requires at least {required_tx_bytes} bytes, but tx only has {}",
            tx.len()
        )));
    }

    if let Some(parity) = tx_parity {
        if parity.len() < required_tx_bytes {
            return Err(Error::new(format!(
                "tx_parity must contain at least {required_tx_bytes} bytes, but only {} were provided",
                parity.len()
            )));
        }
    }

    Ok(())
}

fn validate_rx_parity_args(rx: &[u8], rx_parity: Option<&[u8]>) -> Result<()> {
    if let Some(parity) = rx_parity {
        if parity.len() < rx.len() {
            return Err(Error::new(format!(
                "rx_parity must contain at least {} bytes to match rx, but only {} were provided",
                rx.len(),
                parity.len()
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn append_iso14443a_crc_matches_direct_crc() {
        let mut payload = vec![0x01, 0x02, 0x03];
        let crc = iso14443a_crc(&payload);
        append_iso14443a_crc(&mut payload);

        assert_eq!(&payload[3..], &crc);
    }

    #[test]
    fn append_iso14443b_crc_matches_direct_crc() {
        let mut payload = vec![0x04, 0x05, 0x06];
        let crc = iso14443b_crc(&payload);
        append_iso14443b_crc(&mut payload);

        assert_eq!(&payload[3..], &crc);
    }
}

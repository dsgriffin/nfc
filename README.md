# NFC 🛜

[![Crates.io](https://img.shields.io/crates/v/nfc.svg?maxAge=2592000)](https://crates.io/crates/nfc)

Safe Rust bindings for the [libnfc](https://github.com/nfc-tools/libnfc) library.

This crate builds on top of [nfc-sys](https://github.com/dsgriffin/nfc-sys) and
focuses on the parts of libnfc that can be wrapped safely with ownership,
automatic cleanup, and slice-based APIs.

## Installation

Install `libnfc` first:

- Debian/Ubuntu: see the [libnfc installation notes](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu)
- macOS: `brew install libnfc`
- Other systems: see the [libnfc installation guide](http://nfc-tools.org/index.php?title=Libnfc#Installation)

Then add:

```toml
[dependencies]
nfc = "1.0.0"
```

## Example

```rust
use nfc::{version, Context};

fn main() -> nfc::Result<()> {
    let context = Context::new()?;
    println!("libnfc version: {}", version());

    for connstring in context.list_devices(8)? {
        println!("found device: {connstring}");
    }

    Ok(())
}
```

## What Is Safe

`nfc` 1.0.0 now provides safe wrappers for the normal libnfc workflow:

- context creation and cleanup via `Context`
- device open/close via `Device`
- string accessors for device names and connection strings
- device property setters
- initiator and target send/receive operations using Rust slices
- device info and target formatting as owned `String`s
- CRC helpers and ATS historical-byte helpers

## What Stays Raw

Some libnfc entry points cannot honestly be made fully safe without exposing raw
C invariants. Those remain available through `nfc::ffi`, including:

- custom driver registration
- emulation state-machine callbacks
- any workflow that needs direct raw-pointer interop

## License

MIT

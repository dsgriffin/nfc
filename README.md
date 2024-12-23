# NFC

[![Crates.io](https://img.shields.io/crates/v/nfc.svg?maxAge=2592000)](https://crates.io/crates/nfc)

Rust bindings for the [libnfc](https://github.com/nfc-tools/libnfc) library.

For raw FFI bindings for `libnfc`, see [nfc-sys](https://github.com/dsgriffin/nfc-sys).

## Installation

Install `libnfc` (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OSx, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    nfc = "0.1.11"
    
## Example Usage

#### // main.rs    
```rust
extern crate nfc;

use nfc::context;
use nfc::misc;

fn main() {
    let mut context = context::new();

    if context.is_null() {
        println!("Unable to initialize new NFC context!");
    }

    // Initialize libnfc
    nfc::init(&mut context);
    
    // Print libnfc version
    println!("libnfc version: {}", misc::version());
}
```
    
## TODO

* Replace any raw pointers
* Documentation + more in-depth examples
  
## Contributing
    
If you've found a bug or have an idea, feel free to open an Issue. If you've got a fix or feature ready, open a PR. Thanks!
    
## License
    
MIT    

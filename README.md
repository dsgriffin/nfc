# nfc

[![Crates.io](https://img.shields.io/crates/v/nfc.svg?maxAge=2592000)](https://crates.io/crates/nfc)

Rust bindings for the [libnfc](https://github.com/nfc-tools/libnfc) library.

For raw FFI bindings for `libnfc`, see [nfc-sys](https://github.com/dsgriffin/nfc-sys).

## Installation

Install `libnfc` (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OSx, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    nfc = "0.1.2"
    
## Example Usage

#### // main.rs    
    extern crate nfc;
    
    use nfc::version;
    
    fn main() {
        println!("libnfc version: {}", version());
    }
    
## Implemented/TODO

##### [Library initialization/deinitialization](http://www.libnfc.org/api/modules.html) methods:

 -

##### [NFC Device/Hardware manipulation](http://www.libnfc.org/api/group__dev.html) methods: 

 -
 
##### [NFC Initiator](http://www.libnfc.org/api/group__initiator.html) methods:  

 -
 
##### [NFC Target](http://www.libnfc.org/api/group__target.html) methods:  

 -
 
##### [Error Reporting](http://www.libnfc.org/api/group__error.html) methods:  

 -
 
##### [Special Data Accessors](http://www.libnfc.org/api/group__data.html) methods:  

 -
 
##### [Properties Accessors](http://www.libnfc.org/api/group__properties.html) methods:  

 -
 
##### [Miscellaneous](http://www.libnfc.org/api/group__misc.html) methods:  

&#x2713; [nfc_version](http://www.libnfc.org/api/group__misc.html#gaa48f27c0f93d6508ad9a5ae01ab054d4)
 
##### [To-string Converters](http://www.libnfc.org/api/group__string-converter.html) methods:  

&#x2713; [str_nfc_baud_rate](http://www.libnfc.org/api/group__string-converter.html#ga3c105fdfaf8753b33246d131fbeb95db)
    
## Contributing
    
I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!    
    
## License
    
MIT    

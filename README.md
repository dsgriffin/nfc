# nfc

[![Crates.io](https://img.shields.io/crates/v/nfc.svg?maxAge=2592000)](https://crates.io/crates/nfc)

Rust bindings for the [libnfc](https://github.com/nfc-tools/libnfc) library.

For raw FFI bindings for `libnfc`, see [nfc-sys](https://github.com/dsgriffin/nfc-sys).

## Installation

Install `libnfc` (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OSx, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    nfc = "0.1.3"
    
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

&#x2713; [nfc_strerror](http://www.libnfc.org/api/group__error.html#gab7864c2678696e920b966f47986d31d9) -> `strerror`

&#x2713; [nfc_device_get_last_error](http://www.libnfc.org/api/group__error.html#gacca948e9866dacdc680187343f460341) -> `device_get_last_error`
 
##### [Special Data Accessors](http://www.libnfc.org/api/group__data.html) methods:  

 -
 
##### [Properties Accessors](http://www.libnfc.org/api/group__properties.html) methods:  

 -
 
##### [Miscellaneous](http://www.libnfc.org/api/group__misc.html) methods:  

&#x2713; [nfc_version](http://www.libnfc.org/api/group__misc.html#gaa48f27c0f93d6508ad9a5ae01ab054d4) -> `version`
 
##### [To-string Converters](http://www.libnfc.org/api/group__string-converter.html) methods:  

&#x2713; [str_nfc_baud_rate](http://www.libnfc.org/api/group__string-converter.html#ga3c105fdfaf8753b33246d131fbeb95db) -> `str_baud_rate`

&#x2713; [str_nfc_modulation_type](http://www.libnfc.org/api/group__string-converter.html#gaa349f6eabeb7dbc5b03e92a9bcdc8733) -> `str_modulation_type`
    
## Contributing
    
I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!    
    
## License
    
MIT    

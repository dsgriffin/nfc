# nfc

[![Crates.io](https://img.shields.io/crates/v/nfc.svg?maxAge=2592000)](https://crates.io/crates/nfc)

Rust bindings for the [libnfc](https://github.com/nfc-tools/libnfc) library.

For raw FFI bindings for `libnfc`, see [nfc-sys](https://github.com/dsgriffin/nfc-sys).

## Installation

Install `libnfc` (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OSx, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    nfc = "0.1.6"
    
## Example Usage

#### // main.rs    
    extern crate nfc;
    
    use nfc::version;
    
    fn main() {
        println!("libnfc version: {}", version());
    }
    
## Implemented/TODO

##### [Library initialization/deinitialization](http://www.libnfc.org/api/modules.html) methods (3 out of 3 implemented):

As `register_device`, `init` and `exit`.

##### [NFC Device/Hardware manipulation](http://www.libnfc.org/api/group__dev.html) methods (5 out of 5 implemented):
 
As `open`, `close`, `list_devices`, `idle` and `abort_command`.

##### [NFC Initiator](http://www.libnfc.org/api/group__initiator.html) methods (12 out of 12 implemented):  

As `initiator_init`, `initiator_init_secure_element`, `initiator_select_passive_target`, `initiator_list_passive_targets`, `initiator_poll_target`, `initiator_select_dep_target`, `initiator_poll_dep_target`, `initiator_transceive_bytes`, `initiator_transceive_bits`, `initiator_transceive_bytes_timed`, `initiator_target_is_present` and `initiator_transceive_bits_timed`.
 
##### [NFC Target](http://www.libnfc.org/api/group__target.html) methods (5 out of 5 implemented):  

As `target_init`, `target_send_bytes`, `target_receive_bytes`, `target_send_bits` and `target_receive_bits`.
 
##### [Error Reporting](http://www.libnfc.org/api/group__error.html) methods (4 out of 4 implemented):
  
As `strerror`, `strerror_r`, `perror` and `device_get_last_error`.

##### [Special Data Accessors](http://www.libnfc.org/api/group__data.html) methods (4 out of 4 implemented):  

As `device_get_name`, `device_get_connstring`, `device_get_supported_modulation` and `device_get_supported_baud_rate`.
 
##### [Properties Accessors](http://www.libnfc.org/api/group__properties.html) methods (2 out of 2 implemented):  

As `device_set_property_int` and `device_set_property_bool`.
 
##### [Miscellaneous](http://www.libnfc.org/api/group__misc.html) methods (3 out of 3 implemented):  

As `version`, `free` and `device_get_information_about`.
 
##### [To-string Converters](http://www.libnfc.org/api/group__string-converter.html) methods (3 out of 3 implemented):
  
As `str_baud_rate`, `str_modulation_type` and `str_target`.
  
## Contributing
    
I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!    
    
## License
    
MIT    

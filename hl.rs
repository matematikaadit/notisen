#![allow(non_upper_case_globals)]

#[no_mangle]
pub static weechat_plugin_api_version: [u8; 12] = *b"20190810-01\0";
#[no_mangle]
pub static weechat_plugin_name: [u8; 6] = *b"libhl\0";
#[no_mangle]
pub static weechat_plugin_author: [u8; 47] = *b"Adit Cahya Ramadhan <matematikaadit@gmail.com>\0";
#[no_mangle]
pub static weechat_plugin_description: [u8; 37] = *b"Collect highlight in a single buffer\0";
#[no_mangle]
pub static weechat_plugin_version: [u8; 4] = *b"0.1\0";
#[no_mangle]
pub static weechat_plugin_license: [u8; 4] = *b"MIT\0";
#[no_mangle]
pub static weechat_plugin_priority: u32 = 1000; // default priority


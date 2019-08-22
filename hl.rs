#![allow(non_upper_case_globals)]
use std::os::raw::{c_int, c_char};
//use std::ffi::CString;
use std::ptr;
use std::panic::catch_unwind;

mod plugin;

use plugin::{t_weechat_plugin, WEECHAT_RC_OK, WEECHAT_RC_ERROR};

#[no_mangle] pub static weechat_plugin_api_version: [u8; 12] = *b"20190810-01\0";
#[no_mangle] pub static weechat_plugin_name: [u8; 3] = *b"hl\0";
#[no_mangle] pub static weechat_plugin_author: [u8; 47] = *b"Adit Cahya Ramadhan <matematikaadit@gmail.com>\0";
#[no_mangle] pub static weechat_plugin_description: [u8; 48] = *b"Collect highlights/mentions in a single buffer.\0";
#[no_mangle] pub static weechat_plugin_version: [u8; 4] = *b"0.1\0";
#[no_mangle] pub static weechat_plugin_license: [u8; 4] = *b"MIT\0";
#[no_mangle] pub static weechat_plugin_priority: u32 = 1000; // default priority

#[no_mangle] pub static mut weechat_plugin: *mut t_weechat_plugin = ptr::null_mut();

// #define weechat_printf(__buffer, __message, __argz...)                  \
//     (weechat_plugin->printf_date_tags)(__buffer, 0, NULL, __message,    \
//                                        ##__argz)

#[no_mangle]
pub extern "C" fn weechat_plugin_init(plugin: *mut t_weechat_plugin, _argc: c_int, _argv: *const *const c_char) -> c_int {
    unsafe {
        weechat_plugin = plugin;
        let result = catch_unwind(|| {
            let printf_date_tags = (*weechat_plugin).printf_date_tags.unwrap();
            let hello_world = [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33, 0];
            printf_date_tags(ptr::null_mut(), 0, ptr::null(), hello_world.as_ptr()); // experiment
        });
        match result {
            Ok(_) => WEECHAT_RC_OK as i32,
            Err(_) => WEECHAT_RC_ERROR,
        }
    }
}

#[no_mangle]
pub extern "C" fn weechat_plugin_end(_plugin: *mut t_weechat_plugin) -> c_int {
    WEECHAT_RC_OK as i32
}

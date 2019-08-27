// Copyright (c) 2019 by Adit Cahya Ramadhan <matematika.adit@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_upper_case_globals)]
use std::ptr::{null, null_mut};

mod plugin;

use plugin::{t_weechat_plugin, WEECHAT_RC_OK, WEECHAT_PLUGIN_API_VERSION};

#[no_mangle] pub static weechat_plugin_api_version: [u8; 12] = WEECHAT_PLUGIN_API_VERSION;
#[no_mangle] pub static weechat_plugin_name:        [u8;  3] = *b"hl\0";
#[no_mangle] pub static weechat_plugin_author:      [u8; 47] = *b"Adit Cahya Ramadhan <matematikaadit@gmail.com>\0";
#[no_mangle] pub static weechat_plugin_description: [u8; 48] = *b"Collect highlights/mentions in a single buffer.\0";
#[no_mangle] pub static weechat_plugin_version:     [u8;  4] = *b"0.1\0";
#[no_mangle] pub static weechat_plugin_license:     [u8;  4] = *b"MIT\0";
#[no_mangle] pub static weechat_plugin_priority: u32 = 1000; // default priority

#[no_mangle] pub static mut weechat_plugin: *mut t_weechat_plugin = null_mut();

#[no_mangle]
pub extern "C" fn weechat_plugin_init(
    plugin: *mut t_weechat_plugin,
    _argc: i32,
    _argv: *const *const u8
) -> i32 {
    unsafe {
        weechat_plugin = plugin;
        let printf_date_tags = (*weechat_plugin).printf_date_tags;
        printf_date_tags(null_mut(), 0, null(), b"==> Hello, World! <==\0".as_ptr());
        WEECHAT_RC_OK
    }
}

#[no_mangle]
pub extern "C" fn weechat_plugin_end(_plugin: *mut t_weechat_plugin) -> i32 {
    WEECHAT_RC_OK
}

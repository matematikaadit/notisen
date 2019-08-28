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
use std::ffi::c_void;

mod plugin;

use plugin::{
    time_t,
    t_weechat_plugin,
    t_gui_buffer,
    WEECHAT_RC_OK,
    WEECHAT_PLUGIN_API_VERSION
};

#[no_mangle] pub static weechat_plugin_api_version: [u8; 12] = WEECHAT_PLUGIN_API_VERSION;
#[no_mangle] pub static weechat_plugin_name:        [u8;  3] = *b"hl\0";
#[no_mangle] pub static weechat_plugin_author:      [u8; 47] = *b"Adit Cahya Ramadhan <matematikaadit@gmail.com>\0";
#[no_mangle] pub static weechat_plugin_description: [u8; 48] = *b"Collect highlights/mentions in a single buffer.\0";
#[no_mangle] pub static weechat_plugin_version:     [u8;  4] = *b"0.1\0";
#[no_mangle] pub static weechat_plugin_license:     [u8;  5] = *b"GPL3\0";
#[no_mangle] pub static weechat_plugin_priority: u32 = 1000; // default priority

// required
#[no_mangle] pub static mut weechat_plugin: *mut t_weechat_plugin = null_mut();

// holds our highlight buffer
pub static mut hl_buffer: *mut t_gui_buffer = null_mut();

#[no_mangle]
pub extern "C" fn weechat_plugin_init(
    plugin: *mut t_weechat_plugin,
    _argc: i32,
    _argv: *const *const u8
) -> i32 {
    unsafe {
        weechat_plugin = plugin;
        buffer_open();
        hook_setup();
        WEECHAT_RC_OK
    }
}

#[no_mangle]
pub extern "C" fn weechat_plugin_end(_plugin: *mut t_weechat_plugin) -> i32 {
    WEECHAT_RC_OK
}

fn buffer_open() {
    unsafe {
        let buffer_new = (*weechat_plugin).buffer_new;
        hl_buffer = buffer_new(
            weechat_plugin,
            b"[hl]\0".as_ptr(),
            input_callback,
            null(),
            null_mut(),
            close_callback,
            null(),
            null_mut(),
        );
    }
}

unsafe extern "C" fn input_callback(
    _pointer: *const c_void,
    _data: *mut c_void,
    _buffer: *mut t_gui_buffer,
    _input_data: *const u8,
) -> i32 {
    WEECHAT_RC_OK
}

unsafe extern "C" fn close_callback(
    _pointer: *const c_void,
    _data: *mut c_void,
    _buffer: *mut t_gui_buffer,
) -> i32 {
    hl_buffer = null_mut();
    WEECHAT_RC_OK
}

fn hook_setup() {
    unsafe {
        let hook_print = (*weechat_plugin).hook_print;
        hook_print(
            weechat_plugin,
            null_mut(),
            null(),
            null(),
            0,
            new_message_callback,
            null(),
            null_mut(),
        );
    }
}

unsafe extern "C" fn new_message_callback(
    _pointer: *const c_void,
    _data: *mut c_void,
    buffer: *mut t_gui_buffer,
    date: time_t,
    _tags_count: i32,
    _tags: *mut *const u8,
    displayed: i32,
    highlight: i32,
    prefix: *const u8,
    message: *const u8,
) -> i32 {
    if highlight == 1 && displayed == 1 && is_irc_channel_buffer(buffer) {
        let printf_date_tags = (*weechat_plugin).printf_date_tags;
        let buffer_get_string = (*weechat_plugin).buffer_get_string;
        let buffer_set = (*weechat_plugin).buffer_set;

        let channel = buffer_get_string(buffer, b"localvar_channel\0".as_ptr());

        // print the message
        printf_date_tags(
            hl_buffer,
            date,
            b"no_highlight\0".as_ptr(),
            b"(%s) %s\t%s\0".as_ptr(),
            channel,
            prefix,
            message,
        );

        // put the [hl] buffer on hotlist
        buffer_set(hl_buffer, b"hotlist\0".as_ptr(), b"3\0".as_ptr());
    }
    WEECHAT_RC_OK
}

fn is_irc_channel_buffer(buffer: *mut t_gui_buffer) -> bool {
    unsafe {
        let buffer_get_string = (*weechat_plugin).buffer_get_string;
        let strcasecmp = (*weechat_plugin).strcasecmp;

        let plugin = buffer_get_string(buffer, b"plugin\0".as_ptr());
        let ty = buffer_get_string(buffer, b"localvar_type\0".as_ptr());
        strcasecmp(plugin, b"irc\0".as_ptr()) == 0 &&
        strcasecmp(ty, b"channel\0".as_ptr()) == 0
    }
}

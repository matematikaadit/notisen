#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]

use std::os::raw::c_void;

pub type time_t = std::os::raw::c_long;
pub type socklen_t = std::os::raw::c_uint;
pub type sa_family_t = std::os::raw::c_ushort;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [u8; 14],
}

#[test]
fn test_layout_sockaddr() {
    assert_eq!(
        mem::size_of::<sockaddr>(),
        16,
        concat!("Size of: ", stringify!(sockaddr))
    );
    assert_eq!(
        mem::align_of::<sockaddr>(),
        2,
        concat!("Alignment of ", stringify!(sockaddr))
    );
}

pub const WEECHAT_PLUGIN_API_VERSION: [u8; 12] = *b"20190810-01\0";

#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_config_option  { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_config_section { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_config_file    { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_window     { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_buffer     { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_bar        { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_bar_item   { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_bar_window { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_completion { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_nick       { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_gui_nick_group { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_infolist       { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_infolist_item  { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_upgrade_file   { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_weelist        { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_weelist_item   { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_arraylist      { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_hashtable      { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_hdata          { _unused: [u8; 0], }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct timeval          { _unused: [u8; 0], }

#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_hook           { pub _address: u8, }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_hashtable_item { pub _address: u8, }
#[repr(C)] #[derive(Debug, Copy, Clone)] pub struct t_infolist_var   { pub _address: u8, }

/* return codes for plugin functions */
pub const WEECHAT_RC_OK:     i32 =  0;
pub const WEECHAT_RC_OK_EAT: i32 =  1;
pub const WEECHAT_RC_ERROR:  i32 = -1;

/* flags for string_split function */
pub const WEECHAT_STRING_SPLIT_STRIP_LEFT:    i32 = (1 << 0);
pub const WEECHAT_STRING_SPLIT_STRIP_RIGHT:   i32 = (1 << 1);
pub const WEECHAT_STRING_SPLIT_COLLAPSE_SEPS: i32 = (1 << 2);
pub const WEECHAT_STRING_SPLIT_KEEP_EOL:      i32 = (1 << 3);

/* return codes for config read functions/callbacks */
pub const WEECHAT_CONFIG_READ_OK:             i32 =  0;
pub const WEECHAT_CONFIG_READ_MEMORY_ERROR:   i32 = -1;
pub const WEECHAT_CONFIG_READ_FILE_NOT_FOUND: i32 = -2;

/* return codes for config write functions/callbacks */
pub const WEECHAT_CONFIG_WRITE_OK:           i32 =  0;
pub const WEECHAT_CONFIG_WRITE_ERROR:        i32 = -1;
pub const WEECHAT_CONFIG_WRITE_MEMORY_ERROR: i32 = -2;

/* null value for option */
// #define WEECHAT_CONFIG_OPTION_NULL                 "null"
// still don't know how to translate this

/* return codes for config option set */
pub const WEECHAT_CONFIG_OPTION_SET_OK_CHANGED:       i32 =  2;
pub const WEECHAT_CONFIG_OPTION_SET_OK_SAME_VALUE:    i32 =  1;
pub const WEECHAT_CONFIG_OPTION_SET_ERROR:            i32 =  0;
pub const WEECHAT_CONFIG_OPTION_SET_OPTION_NOT_FOUND: i32 = -1;

/* return codes for config option unset */
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_NO_RESET: i32 =  0;
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_RESET:    i32 =  1;
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_REMOVED:  i32 =  2;
pub const WEECHAT_CONFIG_OPTION_UNSET_ERROR:       i32 = -1;

/* list management (order of elements) */
// #define WEECHAT_LIST_POS_SORT                       "sort"
// #define WEECHAT_LIST_POS_BEGINNING                  "beginning"
// #define WEECHAT_LIST_POS_END                        "end"

/* type for keys and values in hashtable */
// #define WEECHAT_HASHTABLE_INTEGER                   "integer"
// #define WEECHAT_HASHTABLE_STRING                    "string"
// #define WEECHAT_HASHTABLE_POINTER                   "pointer"
// #define WEECHAT_HASHTABLE_BUFFER                    "buffer"
// #define WEECHAT_HASHTABLE_TIME                      "time"

/* types for hdata */
pub const WEECHAT_HDATA_OTHER:         i32 = 0;
pub const WEECHAT_HDATA_CHAR:          i32 = 1;
pub const WEECHAT_HDATA_INTEGER:       i32 = 2;
pub const WEECHAT_HDATA_LONG:          i32 = 3;
pub const WEECHAT_HDATA_STRING:        i32 = 4;
pub const WEECHAT_HDATA_POINTER:       i32 = 5;
pub const WEECHAT_HDATA_TIME:          i32 = 6;
pub const WEECHAT_HDATA_HASHTABLE:     i32 = 7;
pub const WEECHAT_HDATA_SHARED_STRING: i32 = 8;

/* flags for hdata lists */
pub const WEECHAT_HDATA_LIST_CHECK_POINTERS: i32 = 1;

/* buffer hotlist */
// #define WEECHAT_HOTLIST_LOW                         "0"
// #define WEECHAT_HOTLIST_MESSAGE                     "1"
// #define WEECHAT_HOTLIST_PRIVATE                     "2"
// #define WEECHAT_HOTLIST_HIGHLIGHT                   "3"

/*
 * process return code (for callback):
 *   if >= 0, the process ended and it's return code of command
 *   if -1, the process is still running
 *   if -2, the process ended with an error
 *   if -3, the callback is called in the child process (exec of function)
 *          (note: the return code -3 is NEVER sent to script plugins,
 *           it can be used only in C API)
 */
pub const WEECHAT_HOOK_PROCESS_RUNNING: i32 = -1;
pub const WEECHAT_HOOK_PROCESS_ERROR:   i32 = -2;
pub const WEECHAT_HOOK_PROCESS_CHILD:   i32 = -3;

/* connect status for connection hooked */
pub const WEECHAT_HOOK_CONNECT_OK:                     i32 =  0;
pub const WEECHAT_HOOK_CONNECT_ADDRESS_NOT_FOUND:      i32 =  1;
pub const WEECHAT_HOOK_CONNECT_IP_ADDRESS_NOT_FOUND:   i32 =  2;
pub const WEECHAT_HOOK_CONNECT_CONNECTION_REFUSED:     i32 =  3;
pub const WEECHAT_HOOK_CONNECT_PROXY_ERROR:            i32 =  4;
pub const WEECHAT_HOOK_CONNECT_LOCAL_HOSTNAME_ERROR:   i32 =  5;
pub const WEECHAT_HOOK_CONNECT_GNUTLS_INIT_ERROR:      i32 =  6;
pub const WEECHAT_HOOK_CONNECT_GNUTLS_HANDSHAKE_ERROR: i32 =  7;
pub const WEECHAT_HOOK_CONNECT_MEMORY_ERROR:           i32 =  8;
pub const WEECHAT_HOOK_CONNECT_TIMEOUT:                i32 =  9;
pub const WEECHAT_HOOK_CONNECT_SOCKET_ERROR:           i32 = 10;

/* action for gnutls callback: verify or set certificate */
pub const WEECHAT_HOOK_CONNECT_GNUTLS_CB_VERIFY_CERT: i32 = 0;
pub const WEECHAT_HOOK_CONNECT_GNUTLS_CB_SET_CERT:    i32 = 1;

/* type of data for signal hooked */
// #define WEECHAT_HOOK_SIGNAL_STRING                  "string"
// #define WEECHAT_HOOK_SIGNAL_INT                     "int"
// #define WEECHAT_HOOK_SIGNAL_POINTER                 "pointer"


// /* macro to format string with variable args, using dynamic buffer size */
// #define weechat_va_format(__format)                                     \
//     va_list argptr;                                                     \
//     int vaa_size, vaa_num;                                              \
//     char *vbuffer, *vaa_buffer2;                                        \
//     vaa_size = 1024;                                                    \
//     vbuffer = malloc (vaa_size);                                        \
//     if (vbuffer)                                                        \
//     {                                                                   \
//         while (1)                                                       \
//         {                                                               \
//             va_start (argptr, __format);                                \
//             vaa_num = vsnprintf (vbuffer, vaa_size, __format, argptr);  \
//             va_end (argptr);                                            \
//             if ((vaa_num >= 0) && (vaa_num < vaa_size))                 \
//                 break;                                                  \
//             vaa_size = (vaa_num >= 0) ? vaa_num + 1 : vaa_size * 2;     \
//             vaa_buffer2 = realloc (vbuffer, vaa_size);                  \
//             if (!vaa_buffer2)                                           \
//             {                                                           \
//                 free (vbuffer);                                         \
//                 vbuffer = NULL;                                         \
//                 break;                                                  \
//             }                                                           \
//             vbuffer = vaa_buffer2;                                      \
//         }                                                               \
//     }
//
// /*
//  * macro to return error in case of missing arguments in callback of
//  * hook_command
//  */
// #define WEECHAT_COMMAND_MIN_ARGS(__min_args, __option)                  \
//     if (argc < __min_args)                                              \
//     {                                                                   \
//         weechat_printf_date_tags (                                      \
//             NULL, 0, "no_filter",                                       \
//             _("%sToo few arguments for command \"%s%s%s\" "             \
//               "(help on command: /help %s)"),                           \
//             weechat_prefix ("error"),                                   \
//             argv[0],                                                    \
//             (__option && __option[0]) ? " " : "",                       \
//             (__option && __option[0]) ? __option : "",                  \
//             argv[0] + 1);                                               \
//         return WEECHAT_RC_ERROR;                                        \
//     }
//
// /* macro to return error in callback of hook_command */
// #define WEECHAT_COMMAND_ERROR                                           \
//     {                                                                   \
//         weechat_printf_date_tags (                                      \
//             NULL, 0, "no_filter",                                       \
//             _("%sError with command \"%s\" "                            \
//               "(help on command: /help %s)"),                           \
//             weechat_prefix ("error"),                                   \
//             argv_eol[0],                                                \
//             argv[0] + 1);                                               \
//         return WEECHAT_RC_ERROR;                                        \
//     }
//

// or you can define it manually
//
//     enum c_void {
//         __variant0,
//         __variant1,
//     }
//
// see the source to know why they use that definition.
// https://doc.rust-lang.org/src/core/ffi.rs.html#33-40

// Big struct incoming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t_weechat_plugin {
    /* plugin variables */
    pub filename:    *mut u8,               /* name of plugin on disk            */
    pub handle:      *mut c_void,           /* handle of plugin (given by dlopen)*/
    pub name:        *mut u8,               /* short name                        */
    pub description: *mut u8,               /* description                       */
    pub author:      *mut u8,               /* author                            */
    pub version:     *mut u8,               /* plugin version                    */
    pub license:     *mut u8,               /* license                           */
    pub charset:     *mut u8,               /* charset used by plugin            */
    pub priority:    i32,                   /* plugin priority (default is 1000) */
    pub initialized: i32,                   /* plugin initialized? (init called) */
    pub debug:       i32,                   /* debug level for plugin (0=off)    */
    pub upgrading:   i32,                   /* 1 if the plugin must load upgrade */
                                            /* info on startup (if weechat is    */
                                            /* run with --upgrade)               */
    pub variables:   *mut t_hashtable,      /* plugin custom variables           */
    pub prev_plugin: *mut t_weechat_plugin, /* link to previous plugin           */
    pub next_plugin: *mut t_weechat_plugin, /* link to next plugin               */

    /*
     * plugin functions (API)
     * WeeChat developers: if you add functions in API, update value of
     * constant WEECHAT_PLUGIN_API_VERSION
     */

    /* plugins */
    pub plugin_get_name:
        unsafe extern "C" fn(plugin: *mut t_weechat_plugin) -> *const u8,

    /* strings */
    pub charset_set:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            charset: *const u8
        ),

    pub iconv_to_internal:
        unsafe extern "C" fn(
            charset: *const u8,
            string: *const u8
        ) -> *mut u8,

    pub iconv_from_internal:
        unsafe extern "C" fn(
            charset: *const u8,
            string: *const u8
        ) -> *mut u8,

    pub gettext:
        unsafe extern "C" fn(string: *const u8) -> *const u8,

    pub ngettext:
        unsafe extern "C" fn(
            single: *const u8,
            plural: *const u8,
            count: i32
        ) -> *const u8,

    pub strndup:
        unsafe extern "C" fn(
            string: *const u8,
            length: i32
        ) -> *mut u8,

    pub string_tolower:
        unsafe extern "C" fn(string: *mut u8),

    pub string_toupper:
        unsafe extern "C" fn(string: *mut u8),

    pub strcasecmp:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8
        ) -> i32,

    pub strcasecmp_range:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8,
            range: i32,
        ) -> i32,

    pub strncasecmp:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8,
            max: i32,
        ) -> i32,

    pub strncasecmp_range:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8,
            max: i32,
            range: i32,
        ) -> i32,

    pub strcmp_ignore_chars:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8,
            chars_ignored: *const u8,
            case_sensitive: i32,
        ) -> i32,

    pub strcasestr:
        unsafe extern "C" fn(
            string: *const u8,
            search: *const u8,
        ) -> *const u8,

    pub strlen_screen:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub string_match:
        unsafe extern "C" fn(
            string: *const u8,
            mask: *const u8,
            case_sensitive: i32,
        ) -> i32,

    pub string_match_list:
        unsafe extern "C" fn(
            string: *const u8,
            masks: *mut *const u8,
            case_sensitive: i32,
        ) -> i32,

    pub string_replace:
        unsafe extern "C" fn(
            string: *const u8,
            search: *const u8,
            replace: *const u8,
        ) -> *mut u8,

    pub string_expand_home:
        unsafe extern "C" fn(path: *const u8) -> *mut u8,

    pub string_eval_path_home:
        unsafe extern "C" fn(
            path: *const u8,
            pointers: *mut t_hashtable,
            extra_vars: *mut t_hashtable,
            options: *mut t_hashtable,
        ) -> *mut u8,

    pub string_remove_quotes:
        unsafe extern "C" fn(
            string: *const u8,
            quotes: *const u8,
        ) -> *mut u8,

    pub string_strip:
        unsafe extern "C" fn(
            string: *const u8,
            left: i32,
            right: i32,
            chars: *const u8,
        ) -> *mut u8,

    pub string_convert_escaped_chars:
        unsafe extern "C" fn(string: *const u8) -> *mut u8,

    pub string_mask_to_regex:
        unsafe extern "C" fn(mask: *const u8) -> *mut u8,

    pub string_regex_flags:
        unsafe extern "C" fn(
            regex: *const u8,
            default_flags: i32,
            flags: *mut i32,
        ) -> *const u8,

    pub string_regcomp:
        unsafe extern "C" fn(
            preg: *mut c_void,
            regex: *const u8,
            default_flags: i32,
        ) -> i32,

    pub string_has_highlight:
        unsafe extern "C" fn(
            string: *const u8,
            highlight_words: *const u8,
        ) -> i32,

    pub string_has_highlight_regex:
        unsafe extern "C" fn(
            string: *const u8,
            regex: *const u8,
        ) -> i32,

    pub string_replace_regex:
        unsafe extern "C" fn(
            string: *const u8,
            regex: *mut c_void,
            replace: *const u8,
            reference_char: u8,
            callback:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    text: *const u8,
                ) -> *mut u8,
            callback_data: *mut c_void,
        ) -> *mut u8,

    pub string_split:
        unsafe extern "C" fn(
            string: *const u8,
            separators: *const u8,
            strip_items: *const u8,
            flags: i32,
            num_items_max: i32,
            num_items: *mut i32,
        ) -> *mut *mut u8,

    pub string_split_shell:
        unsafe extern "C" fn(
            string: *const u8,
            num_items: *mut i32,
        ) -> *mut *mut u8,

    pub string_free_split:
        unsafe extern "C" fn(split_string: *mut *mut u8),

    pub string_build_with_split_string:
        unsafe extern "C" fn(
            split_string: *mut *const u8,
            separator: *const u8,
        ) -> *mut u8,

    pub string_split_command:
        unsafe extern "C" fn(
            command: *const u8,
            separator: u8,
        ) -> *mut *mut u8,

    pub string_free_split_command:
        unsafe extern "C" fn(split_command: *mut *mut u8),

    pub string_format_size:
        unsafe extern "C" fn(size: u64) -> *mut u8,

    pub string_remove_color:
        unsafe extern "C" fn(
            string: *const u8,
            replacement: *const u8,
        ) -> *mut u8,

    pub string_base_encode:
        unsafe extern "C" fn(
            base: i32,
            from: *const u8,
            length: i32,
            to: *mut u8,
        ) -> i32,

    pub string_base_decode:
        unsafe extern "C" fn(
            base: i32,
            from: *const u8,
            to: *mut u8,
        ) -> i32,

    pub string_hex_dump:
        unsafe extern "C" fn(
            data: *const u8,
            data_size: i32,
            bytes_per_line: i32,
            prefix: *const u8,
            suffix: *const u8,
        ) -> *mut u8,

    pub string_is_command_char:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub string_input_for_buffer:
        unsafe extern "C" fn(string: *const u8) -> *const u8,

    pub string_eval_expression:
        unsafe extern "C" fn(
            expr: *const u8,
            pointers: *mut t_hashtable,
            extra_vars: *mut t_hashtable,
            options: *mut t_hashtable,
        ) -> *mut u8,

    pub string_dyn_alloc:
        unsafe extern "C" fn(size_alloc: i32) -> *mut *mut u8,

    pub string_dyn_copy:
        unsafe extern "C" fn(
            string: *mut *mut u8,
            new_string: *const u8,
        ) -> i32,

    pub string_dyn_concat:
        unsafe extern "C" fn(
            string: *mut *mut u8,
            add: *const u8,
        ) -> i32,

    pub string_dyn_free:
        unsafe extern "C" fn(
            string: *mut *mut u8,
            free_string: i32,
        ) -> *mut u8,

    /* UTF-8 strings */
    pub utf8_has_8bits:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub utf8_is_valid:
        unsafe extern "C" fn(
            string: *const u8,
            length: i32,
            error: *mut *mut u8,
        ) -> i32,

    pub utf8_normalize:
        unsafe extern "C" fn(
            string: *mut u8,
            replacement: u8,
        ),

    pub utf8_prev_char:
        unsafe extern "C" fn(
            string_start: *const u8,
            string: *const u8,
        ) -> *const u8,

    pub utf8_next_char:
        unsafe extern "C" fn(string: *const u8) -> *const u8,

    pub utf8_char_int:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub utf8_char_size:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub utf8_strlen:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub utf8_strnlen:
        unsafe extern "C" fn(
            string: *const u8,
            bytes: i32,
        ) -> i32,

    pub utf8_strlen_screen:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub utf8_charcmp:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8,
        ) -> i32,

    pub utf8_charcasecmp:
        unsafe extern "C" fn(
            string1: *const u8,
            string2: *const u8,
        ) -> i32,

    pub utf8_char_size_screen:
        unsafe extern "C" fn(string: *const u8) -> i32,

    pub utf8_add_offset:
        unsafe extern "C" fn(
            string: *const u8,
            offset: i32,
        ) -> *const u8,

    pub utf8_real_pos:
        unsafe extern "C" fn(
            string: *const u8,
            pos: i32,
        ) -> i32,

    pub utf8_pos:
        unsafe extern "C" fn(
            string: *const u8,
            real_pos: i32,
        ) -> i32,

    pub utf8_strndup:
        unsafe extern "C" fn(
            string: *const u8,
            length: i32,
        ) -> *mut u8,

    /* directories/files */
    pub mkdir_home:
        unsafe extern "C" fn(
            directory: *const u8,
            mode: i32,
        ) -> i32,

    pub mkdir:
        unsafe extern "C" fn(
            directory: *const u8,
            mode: i32,
        ) -> i32,

    pub mkdir_parents:
        unsafe extern "C" fn(
            directory: *const u8,
            mode: i32,
        ) -> i32,

    pub exec_on_files:
        unsafe extern "C" fn(
            directory: *const u8,
            recurse_subdirs: i32,
            hidden_files: i32,
            callback:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    filename: *const u8,
                ),
            callback_data: *mut c_void,
        ),

    pub file_get_content:
        unsafe extern "C" fn(filename: *const u8) -> *mut u8,

    /* util */
    pub util_timeval_cmp:
        unsafe extern "C" fn(
            tv1: *mut timeval,
            tv2: *mut timeval
        ) -> i32,

    pub util_timeval_diff:
        unsafe extern "C" fn(
            tv1: *mut timeval,
            tv2: *mut timeval
        ) -> i64,

    pub util_timeval_add:
        unsafe extern "C" fn(
            tv: *mut timeval,
            interval: i64
        ),

    pub util_get_time_string:
        unsafe extern "C" fn(date: *const time_t) -> *const  u8,

    pub util_version_number:
        unsafe extern "C" fn(version: *const u8) -> i32,

    /* sorted lists */
    pub list_new:
        unsafe extern "C" fn() -> *mut t_weelist,

    pub list_add:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            data: *const u8,
            where_: *const u8,
            user_data: *mut c_void,
        ) -> *mut t_weelist_item,

    pub list_search:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            data: *const u8,
        ) -> *mut t_weelist_item,

    pub list_search_pos:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            data: *const u8,
        ) -> i32,

    pub list_casesearch:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            data: *const u8,
        ) -> *mut t_weelist_item,

    pub list_casesearch_pos:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            data: *const u8,
        ) -> i32,

    pub list_get:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            position: i32,
        ) -> *mut t_weelist_item,

    pub list_set:
        unsafe extern "C" fn(
            item: *mut t_weelist_item,
            value: *const u8
        ),

    pub list_next:
        unsafe extern "C" fn(
            item: *mut t_weelist_item
        ) -> *mut t_weelist_item,

    pub list_prev:
        unsafe extern "C" fn(
            item: *mut t_weelist_item
        ) -> *mut t_weelist_item,

    pub list_string:
        unsafe extern "C" fn(
            item: *mut t_weelist_item
        ) -> *const u8,

    pub list_user_data:
        unsafe extern "C" fn(item: *mut t_weelist_item) -> *mut c_void,

    pub list_size:
        unsafe extern "C" fn(weelist: *mut t_weelist) -> i32,

    pub list_remove:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            item: *mut t_weelist_item
        ),

    pub list_remove_all:
        unsafe extern "C" fn(weelist: *mut t_weelist),

    pub list_free:
        unsafe extern "C" fn(weelist: *mut t_weelist),

    /* array lists */
    pub arraylist_new:
        unsafe extern "C" fn(
            initial_size: i32,
            sorted: i32,
            allow_duplicates: i32,
            callback_cmp:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    arraylist: *mut t_arraylist,
                    pointer1: *mut c_void,
                    pointer2: *mut c_void,
                ) -> i32,
            callback_cmp_data: *mut c_void,
            callback_free:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    arraylist: *mut t_arraylist,
                    pointer: *mut c_void,
                ),
            callback_free_data: *mut c_void,
        ) -> *mut t_arraylist,

    pub arraylist_size:
        unsafe extern "C" fn(arraylist: *mut t_arraylist) -> i32,

    pub arraylist_get:
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist,
            index: i32,
        ) -> *mut c_void,

    pub arraylist_search:
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist,
            pointer: *mut c_void,
            index: *mut i32,
            index_insert: *mut i32,
        ) -> *mut c_void,

    pub arraylist_insert:
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist,
            index: i32,
            pointer: *mut c_void,
        ) -> i32,

    pub arraylist_add:
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist,
            pointer: *mut c_void,
        ) -> i32,

    pub arraylist_remove:
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist,
            index: i32,
        ) -> i32,

    pub arraylist_clear:
        unsafe extern "C" fn(arraylist: *mut t_arraylist) -> i32,

    pub arraylist_free:
        unsafe extern "C" fn(arraylist: *mut t_arraylist),

    /* hash tables */
    pub hashtable_new:
        unsafe extern "C" fn(
            size: i32,
            type_keys: *const u8,
            type_values: *const u8,
            callback_hash_key:
                unsafe extern "C" fn(
                    hashtable: *mut t_hashtable,
                    key: *const c_void,
                ) -> i64,
            callback_keycmp:
                unsafe extern "C" fn(
                    hashtable: *mut t_hashtable,
                    key1: *const c_void,
                    key2: *const c_void,
                ) -> i32,
        ) -> *mut t_hashtable,

    pub hashtable_set_with_size:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            key: *const c_void,
            key_size: i32,
            value: *const c_void,
            value_size: i32,
        ) -> *mut t_hashtable_item,

    pub hashtable_set:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            key: *const c_void,
            value: *const c_void,
        ) -> *mut t_hashtable_item,

    pub hashtable_get:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            key: *const c_void,
        ) -> *mut c_void,

    pub hashtable_has_key:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            key: *const c_void,
        ) -> i32,

    pub hashtable_map:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            callback_map:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    hashtable: *mut t_hashtable,
                    key: *const c_void,
                    value: *const c_void,
                ),
            callback_map_data: *mut c_void,
        ),

    pub hashtable_map_string:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            callback_map:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    hashtable: *mut t_hashtable,
                    key: *const u8,
                    value: *const u8,
                ),
            callback_map_data: *mut c_void,
        ),

    pub hashtable_dup:
        unsafe extern "C" fn(hashtable: *mut t_hashtable) -> *mut t_hashtable,

    pub hashtable_get_integer:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            property: *const u8,
        ) -> i32,

    pub hashtable_get_string:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            property: *const u8,
        ) -> *const u8,

    pub hashtable_set_pointer:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            property: *const u8,
            pointer: *mut c_void,
        ),

    pub hashtable_add_to_infolist:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            infolist_item: *mut t_infolist_item,
            prefix: *const u8,
        ) -> i32,

    pub hashtable_add_from_infolist:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            infolist: *mut t_infolist,
            prefix: *const u8,
        ) -> i32,
    pub hashtable_remove:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable,
            key: *const c_void
        ),

    pub hashtable_remove_all:
        unsafe extern "C" fn(hashtable: *mut t_hashtable),

    pub hashtable_free:
        unsafe extern "C" fn(hashtable: *mut t_hashtable),

    /* config files */
    pub config_new:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            name: *const u8,
            callback_reload:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    config_file: *mut t_config_file,
                ) -> i32,
            callback_reload_pointer: *const c_void,
            callback_reload_data: *mut c_void,
        ) -> *mut t_config_file,

    pub config_new_section:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            name: *const u8,
            user_can_add_options: i32,
            user_can_delete_options: i32,
            callback_read:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    config_file: *mut t_config_file,
                    section: *mut t_config_section,
                    option_name: *const u8,
                    value: *const u8,
                ) -> i32,
            callback_read_pointer: *const c_void,
            callback_read_data: *mut c_void,
            callback_write:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    config_file: *mut t_config_file,
                    section_name: *const u8,
                ) -> i32,
            callback_write_pointer: *const c_void,
            callback_write_data: *mut c_void,
            callback_write_default:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    config_file: *mut t_config_file,
                    section_name: *const u8,
                ) -> i32,
            callback_write_default_pointer: *const c_void,
            callback_write_default_data: *mut c_void,
            callback_create_option:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    config_file: *mut t_config_file,
                    section: *mut t_config_section,
                    option_name: *const u8,
                    value: *const u8,
                ) -> i32,
            callback_create_option_pointer: *const c_void,
            callback_create_option_data: *mut c_void,
            callback_delete_option:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    config_file: *mut t_config_file,
                    section: *mut t_config_section,
                    option: *mut t_config_option,
                ) -> i32,
            callback_delete_option_pointer: *const c_void,
            callback_delete_option_data: *mut c_void,
        ) -> *mut t_config_section,

    pub config_search_section:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            section_name: *const u8,
        ) -> *mut t_config_section,

    pub config_new_option:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            section: *mut t_config_section,
            name: *const u8,
            type_: *const u8,
            description: *const u8,
            string_values: *const u8,
            min: i32,
            max: i32,
            default_value: *const u8,
            value: *const u8,
            null_value_allowed: i32,
            callback_check_value:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    option: *mut t_config_option,
                    value: *const u8,
                ) -> i32,
            callback_check_value_pointer: *const c_void,
            callback_check_value_data: *mut c_void,
            callback_change:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    option: *mut t_config_option,
                ),
            callback_change_pointer: *const c_void,
            callback_change_data: *mut c_void,
            callback_delete:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    option: *mut t_config_option,
                ),
            callback_delete_pointer: *const c_void,
            callback_delete_data: *mut c_void,
        ) -> *mut t_config_option,

    pub config_search_option:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            section: *mut t_config_section,
            option_name: *const u8,
        ) -> *mut t_config_option,

    pub config_search_section_option:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            section: *mut t_config_section,
            option_name: *const u8,
            section_found: *mut *mut t_config_section,
            option_found: *mut *mut t_config_option,
        ),

    pub config_search_with_string:
        unsafe extern "C" fn(
            option_name: *const u8,
            config_file: *mut *mut t_config_file,
            section: *mut *mut t_config_section,
            option: *mut *mut t_config_option,
            pos_option_name: *mut *mut u8,
        ),

    pub config_string_to_boolean:
        unsafe extern "C" fn(text: *const u8) -> i32,

    pub config_option_reset:
        unsafe extern "C" fn(
            option: *mut t_config_option,
            run_callback: i32,
        ) -> i32,

    pub config_option_set:
        unsafe extern "C" fn(
            option: *mut t_config_option,
            value: *const u8,
            run_callback: i32,
        ) -> i32,

    pub config_option_set_null:
        unsafe extern "C" fn(
            option: *mut t_config_option,
            run_callback: i32,
        ) -> i32,

    pub config_option_unset:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_option_rename:
        unsafe extern "C" fn(option: *mut t_config_option, new_name: *const u8),

    pub config_option_get_string:
        unsafe extern "C" fn(
            option: *mut t_config_option,
            property: *const u8,
        ) -> *const u8,

    pub config_option_get_pointer:
        unsafe extern "C" fn(
            option: *mut t_config_option,
            property: *const u8,
        ) -> *mut c_void,

    pub config_option_is_null:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_option_default_is_null:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_boolean:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_boolean_default:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_integer:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_integer_default:
        unsafe extern "C" fn(option: *mut t_config_option) -> i32,

    pub config_string:
        unsafe extern "C" fn(option: *mut t_config_option) -> *const u8,

    pub config_string_default:
        unsafe extern "C" fn(option: *mut t_config_option) -> *const u8,

    pub config_color:
        unsafe extern "C" fn(option: *mut t_config_option) -> *const u8,

    pub config_color_default:
        unsafe extern "C" fn(option: *mut t_config_option) -> *const u8,

    pub config_write_option:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            option: *mut t_config_option,
        ) -> i32,

    pub config_write_line:
        unsafe extern "C" fn(
            config_file: *mut t_config_file,
            option_name: *const u8,
            value: *const u8,
            ...
        ) -> i32,

    pub config_write:
        unsafe extern "C" fn(config_file: *mut t_config_file) -> i32,

    pub config_read:
        unsafe extern "C" fn(config_file: *mut t_config_file) -> i32,

    pub config_reload:
        unsafe extern "C" fn(config_file: *mut t_config_file) -> i32,

    pub config_option_free:
        unsafe extern "C" fn(option: *mut t_config_option),

    pub config_section_free_options:
        unsafe extern "C" fn(section: *mut t_config_section),

    pub config_section_free:
        unsafe extern "C" fn(section: *mut t_config_section),

    pub config_free:
        unsafe extern "C" fn(config_file: *mut t_config_file),

    pub config_get:
        unsafe extern "C" fn(option_name: *const u8) -> *mut t_config_option,

    pub config_get_plugin:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            option_name: *const u8,
        ) -> *const u8,

    pub config_is_set_plugin:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            option_name: *const u8,
        ) -> i32,

    pub config_set_plugin:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            option_name: *const u8,
            value: *const u8,
        ) -> i32,

    pub config_set_desc_plugin:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            option_name: *const u8,
            description: *const u8,
        ),

    pub config_unset_plugin:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            option_name: *const u8,
        ) -> i32,

    /* key bindings */
    pub key_bind:
        unsafe extern "C" fn(
            context: *const u8,
            keys: *mut t_hashtable,
        ) -> i32,

    pub key_unbind:
        unsafe extern "C" fn(
            context: *const u8,
            key: *const u8,
        ) -> i32,

    /* display */
    pub prefix:
        unsafe extern "C" fn(prefix: *const u8) -> *const u8,

    pub color:
        unsafe extern "C" fn(color_name: *const u8) -> *const u8,

    pub printf_date_tags:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            date: time_t,
            tags: *const u8,
            message: *const u8,
            ...
        ),

    pub printf_y:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            y: i32,
            message: *const u8,
            ...
        ),

    pub log_printf:
        unsafe extern "C" fn(message: *const u8, ...),

    /* hooks */
    pub hook_command:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            command: *const u8,
            description: *const u8,
            args: *const u8,
            args_description: *const u8,
            completion: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    buffer: *mut t_gui_buffer,
                    argc: i32,
                    argv: *mut *mut u8,
                    argv_eol: *mut *mut u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_command_run:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            command: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    buffer: *mut t_gui_buffer,
                    command: *const u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_timer:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            interval: i64,
            align_second: i32,
            max_calls: i32,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    remaining_calls: i32,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_fd:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            fd: i32,
            flag_read: i32,
            flag_write: i32,
            flag_exception: i32,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    fd: i32,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_process:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            command: *const u8,
            timeout: i32,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    command: *const u8,
                    return_code: i32,
                    out: *const u8,
                    err: *const u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_process_hashtable:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            command: *const u8,
            options: *mut t_hashtable,
            timeout: i32,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    command: *const u8,
                    return_code: i32,
                    out: *const u8,
                    err: *const u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_connect:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            proxy: *const u8,
            address: *const u8,
            port: i32,
            ipv6: i32,
            retry: i32,
            gnutls_sess: *mut c_void,
            gnutls_cb: *mut c_void,
            gnutls_dhkey_size: i32,
            gnutls_priorities: *const u8,
            local_hostname: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    status: i32,
                    gnutls_rc: i32,
                    sock: i32,
                    error: *const u8,
                    ip_address: *const u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_line:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            buffer_type: *const u8,
            buffer_name: *const u8,
            tags: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    line: *mut t_hashtable,
                ) -> *mut t_hashtable,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_print:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            buffer: *mut t_gui_buffer,
            tags: *const u8,
            message: *const u8,
            strip_colors: i32,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    buffer: *mut t_gui_buffer,
                    date: time_t,
                    tags_count: i32,
                    tags: *mut *const u8,
                    displayed: i32,
                    highlight: i32,
                    prefix: *const u8,
                    message: *const u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_signal:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            signal: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    signal: *const u8,
                    type_data: *const u8,
                    signal_data: *mut c_void,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_signal_send:
        unsafe extern "C" fn(
            signal: *const u8,
            type_data: *const u8,
            signal_data: *mut c_void,
        ) -> i32,

    pub hook_hsignal:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            signal: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    signal: *const u8,
                    hashtable: *mut t_hashtable,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_hsignal_send:
        unsafe extern "C" fn(
            signal: *const u8,
            hashtable: *mut t_hashtable,
        ) -> i32,

    pub hook_config:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            option: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    option: *const u8,
                    value: *const u8,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_completion:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            completion_item: *const u8,
            description: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    completion_item: *const u8,
                    buffer: *mut t_gui_buffer,
                    completion: *mut t_gui_completion,
                ) -> i32,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_completion_get_string:
        unsafe extern "C" fn(
            completion: *mut t_gui_completion,
            property: *const u8,
        ) -> *const u8,

    pub hook_completion_list_add:
        unsafe extern "C" fn(
            completion: *mut t_gui_completion,
            word: *const u8,
            nick_completion: i32,
            where_: *const u8,
        ),

    pub hook_modifier:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            modifier: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    modifier: *const u8,
                    modifier_data: *const u8,
                    string: *const u8,
                ) -> *mut u8,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_modifier_exec:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            modifier: *const u8,
            modifier_data: *const u8,
            string: *const u8,
        ) -> *mut u8,

    pub hook_info:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            info_name: *const u8,
            description: *const u8,
            args_description: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    info_name: *const u8,
                    arguments: *const u8,
                ) -> *mut u8,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_info_hashtable:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            info_name: *const u8,
            description: *const u8,
            args_description: *const u8,
            output_description: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    info_name: *const u8,
                    hashtable: *mut t_hashtable,
                ) -> *mut t_hashtable,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_infolist:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            infolist_name: *const u8,
            description: *const u8,
            pointer_description: *const u8,
            args_description: *const u8,
            callback:
                unsafe extern "C" fn(
                    cb_pointer: *const c_void,
                    data: *mut c_void,
                    infolist_name: *const u8,
                    obj_pointer: *mut c_void,
                    arguments: *const u8,
                ) -> *mut t_infolist,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_hdata:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            hdata_name: *const u8,
            description: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    hdata_name: *const u8,
                ) -> *mut t_hdata,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_focus:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            area: *const u8,
            callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    info: *mut t_hashtable,
                ) -> *mut t_hashtable,
            callback_pointer: *const c_void,
            callback_data: *mut c_void,
        ) -> *mut t_hook,

    pub hook_set:
        unsafe extern "C" fn(
            hook: *mut t_hook,
            property: *const u8,
            value: *const u8,
        ),

    pub unhook:
        unsafe extern "C" fn(hook: *mut t_hook),

    pub unhook_all:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            subplugin: *const u8,
        ),

    /* buffers */
    pub buffer_new:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            name: *const u8,
            input_callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    buffer: *mut t_gui_buffer,
                    input_data: *const u8,
                ) -> i32,
            input_callback_pointer: *const c_void,
            input_callback_data: *mut c_void,
            close_callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    buffer: *mut t_gui_buffer,
                ) -> i32,
            close_callback_pointer: *const c_void,
            close_callback_data: *mut c_void,
        ) -> *mut t_gui_buffer,

    pub buffer_search:
        unsafe extern "C" fn(
            plugin: *const u8,
            name: *const u8,
        ) -> *mut t_gui_buffer,

    pub buffer_search_main:
        unsafe extern "C" fn() -> *mut t_gui_buffer,

    pub buffer_clear:
        unsafe extern "C" fn(buffer: *mut t_gui_buffer),

    pub buffer_close:
        unsafe extern "C" fn(buffer: *mut t_gui_buffer),

    pub buffer_merge:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            target_buffer: *mut t_gui_buffer
        ),

    pub buffer_unmerge:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            number: i32
        ),

    pub buffer_get_integer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            property: *const u8,
        ) -> i32,

    pub buffer_get_string:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            property: *const u8,
        ) -> *const u8,

    pub buffer_get_pointer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            property: *const u8,
        ) -> *mut c_void,

    pub buffer_set:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            property: *const u8,
            value: *const u8,
        ),

    pub buffer_set_pointer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            property: *const u8,
            pointer: *mut c_void,
        ),

    pub buffer_string_replace_local_var:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            string: *const u8,
        ) -> *mut u8,

    pub buffer_match_list:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            string: *const u8,
        ) -> i32,

    /* windows */
    pub window_search_with_buffer:
        unsafe extern "C" fn(buffer: *mut t_gui_buffer) -> *mut t_gui_window,

    pub window_get_integer:
        unsafe extern "C" fn(
            window: *mut t_gui_window,
            property: *const u8,
        ) -> i32,

    pub window_get_string:
        unsafe extern "C" fn(
            window: *mut t_gui_window,
            property: *const u8,
        ) -> *const u8,

    pub window_get_pointer:
        unsafe extern "C" fn(
            window: *mut t_gui_window,
            property: *const u8,
        ) -> *mut c_void,

    pub window_set_title:
        unsafe extern "C" fn(title: *const u8),

    /* nicklist */
    pub nicklist_add_group:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            parent_group: *mut t_gui_nick_group,
            name: *const u8,
            color: *const u8,
            visible: i32,
        ) -> *mut t_gui_nick_group,

    pub nicklist_search_group:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            from_group: *mut t_gui_nick_group,
            name: *const u8,
        ) -> *mut t_gui_nick_group,

    pub nicklist_add_nick:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut t_gui_nick_group,
            name: *const u8,
            color: *const u8,
            prefix: *const u8,
            prefix_color: *const u8,
            visible: i32,
        ) -> *mut t_gui_nick,

    pub nicklist_search_nick:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            from_group: *mut t_gui_nick_group,
            name: *const u8,
        ) -> *mut t_gui_nick,

    pub nicklist_remove_group:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut t_gui_nick_group
        ),

    pub nicklist_remove_nick:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            nick: *mut t_gui_nick
        ),

    pub nicklist_remove_all:
        unsafe extern "C" fn(buffer: *mut t_gui_buffer),

    pub nicklist_get_next_item:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut *mut t_gui_nick_group,
            nick: *mut *mut t_gui_nick,
        ),

    pub nicklist_group_get_integer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut t_gui_nick_group,
            property: *const u8,
        ) -> i32,

    pub nicklist_group_get_string:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut t_gui_nick_group,
            property: *const u8,
        ) -> *const u8,

    pub nicklist_group_get_pointer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut t_gui_nick_group,
            property: *const u8,
        ) -> *mut c_void,

    pub nicklist_group_set:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            group: *mut t_gui_nick_group,
            property: *const u8,
            value: *const u8,
        ),

    pub nicklist_nick_get_integer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            nick: *mut t_gui_nick,
            property: *const u8,
        ) -> i32,

    pub nicklist_nick_get_string:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            nick: *mut t_gui_nick,
            property: *const u8,
        ) -> *const u8,

    pub nicklist_nick_get_pointer:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            nick: *mut t_gui_nick,
            property: *const u8,
        ) -> *mut c_void,

    pub nicklist_nick_set:
        unsafe extern "C" fn(
            buffer: *mut t_gui_buffer,
            nick: *mut t_gui_nick,
            property: *const u8,
            value: *const u8,
        ),

    /* bars */
    pub bar_item_search:
        unsafe extern "C" fn(name: *const u8) -> *mut t_gui_bar_item,

    pub bar_item_new:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            name: *const u8,
            build_callback:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    item: *mut t_gui_bar_item,
                    window: *mut t_gui_window,
                    buffer: *mut t_gui_buffer,
                    extra_info: *mut t_hashtable,
                ) -> *mut u8,
            build_callback_pointer: *const c_void,
            build_callback_data: *mut c_void,
        ) -> *mut t_gui_bar_item,

    pub bar_item_update:
        unsafe extern "C" fn(name: *const u8),

    pub bar_item_remove:
        unsafe extern "C" fn(item: *mut t_gui_bar_item),

    pub bar_search:
        unsafe extern "C" fn(name: *const u8) -> *mut t_gui_bar,

    pub bar_new:
        unsafe extern "C" fn(
            name: *const u8,
            hidden: *const u8,
            priority: *const u8,
            type_: *const u8,
            condition: *const u8,
            position: *const u8,
            filling_top_bottom: *const u8,
            filling_left_right: *const u8,
            size: *const u8,
            size_max: *const u8,
            color_fg: *const u8,
            color_delim: *const u8,
            color_bg: *const u8,
            separator: *const u8,
            items: *const u8,
        ) -> *mut t_gui_bar,

    pub bar_set:
        unsafe extern "C" fn(
            bar: *mut t_gui_bar,
            property: *const u8,
            value: *const u8,
        ) -> i32,

    pub bar_update:
        unsafe extern "C" fn(name: *const u8),

    pub bar_remove:
        unsafe extern "C" fn(bar: *mut t_gui_bar),

    /* command */
    pub command:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            buffer: *mut t_gui_buffer,
            command: *const u8,
        ) -> i32,

    pub command_options:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            buffer: *mut t_gui_buffer,
            command: *const u8,
            options: *mut t_hashtable,
        ) -> i32,

    /* network */
    pub network_pass_proxy:
        unsafe extern "C" fn(
            proxy: *const u8,
            sock: i32,
            address: *const u8,
            port: i32,
        ) -> i32,

    pub network_connect_to:
        unsafe extern "C" fn(
            proxy: *const u8,
            address: *mut sockaddr,
            address_length: socklen_t,
        ) -> i32,

    /* infos */
    pub info_get:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            info_name: *const u8,
            arguments: *const u8,
        ) -> *mut u8,

    pub info_get_hashtable:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            info_name: *const u8,
            hashtable: *mut t_hashtable,
        ) -> *mut t_hashtable,

    /* infolists */
    pub infolist_new:
        unsafe extern "C" fn(plugin: *mut t_weechat_plugin) -> *mut t_infolist,

    pub infolist_new_item:
        unsafe extern "C" fn(infolist: *mut t_infolist) -> *mut t_infolist_item,

    pub infolist_new_var_integer:
        unsafe extern "C" fn(
            item: *mut t_infolist_item,
            name: *const u8,
            value: i32,
        ) -> *mut t_infolist_var,

    pub infolist_new_var_string:
        unsafe extern "C" fn(
            item: *mut t_infolist_item,
            name: *const u8,
            value: *const u8,
        ) -> *mut t_infolist_var,

    pub infolist_new_var_pointer:
        unsafe extern "C" fn(
            item: *mut t_infolist_item,
            name: *const u8,
            pointer: *mut c_void,
        ) -> *mut t_infolist_var,

    pub infolist_new_var_buffer:
        unsafe extern "C" fn(
            item: *mut t_infolist_item,
            name: *const u8,
            pointer: *mut c_void,
            size: i32,
        ) -> *mut t_infolist_var,

    pub infolist_new_var_time:
        unsafe extern "C" fn(
            item: *mut t_infolist_item,
            name: *const u8,
            time: time_t,
        ) -> *mut t_infolist_var,

    pub infolist_search_var:
        unsafe extern "C" fn(
            infolist: *mut t_infolist,
            name: *const u8,
        ) -> *mut t_infolist_var,

    pub infolist_get:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            infolist_name: *const u8,
            pointer: *mut c_void,
            arguments: *const u8,
        ) -> *mut t_infolist,

    pub infolist_next:
        unsafe extern "C" fn(infolist: *mut t_infolist) -> i32,

    pub infolist_prev:
        unsafe extern "C" fn(infolist: *mut t_infolist) -> i32,

    pub infolist_reset_item_cursor:
        unsafe extern "C" fn(infolist: *mut t_infolist),

    pub infolist_fields:
        unsafe extern "C" fn(infolist: *mut t_infolist) -> *const u8,

    pub infolist_integer:
        unsafe extern "C" fn(
            infolist: *mut t_infolist,
            var: *const u8,
        ) -> i32,

    pub infolist_string:
        unsafe extern "C" fn(
            infolist: *mut t_infolist,
            var: *const u8,
        ) -> *const u8,

    pub infolist_pointer:
        unsafe extern "C" fn(
            infolist: *mut t_infolist,
            var: *const u8,
        ) -> *mut c_void,

    pub infolist_buffer:
        unsafe extern "C" fn(
            infolist: *mut t_infolist,
            var: *const u8,
            size: *mut i32,
        ) -> *mut c_void,

    pub infolist_time:
        unsafe extern "C" fn(
            infolist: *mut t_infolist,
            var: *const u8,
        ) -> time_t,

    pub infolist_free:
        unsafe extern "C" fn(infolist: *mut t_infolist),

    /* hdata */
    pub hdata_new:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            hdata_name: *const u8,
            var_prev: *const u8,
            var_next: *const u8,
            create_allowed: i32,
            delete_allowed: i32,
            callback_update:
                unsafe extern "C" fn(
                    data: *mut c_void,
                    hdata: *mut t_hdata,
                    pointer: *mut c_void,
                    hashtable: *mut t_hashtable,
                ) -> i32,
            callback_update_data: *mut c_void,
        ) -> *mut t_hdata,

    pub hdata_new_var:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
            offset: i32,
            type_: i32,
            update_allowed: i32,
            array_size: *const u8,
            hdata_name: *const u8,
        ),

    pub hdata_new_list:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
            pointer: *mut c_void,
            flags: i32,
        ),

    pub hdata_get:
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin,
            hdata_name: *const u8,
        ) -> *mut t_hdata,

    pub hdata_get_var_offset:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
        ) -> i32,

    pub hdata_get_var_type:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
        ) -> i32,

    pub hdata_get_var_type_string:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
        ) -> *const u8,

    pub hdata_get_var_array_size:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> i32,

    pub hdata_get_var_array_size_string:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> *const u8,

    pub hdata_get_var_hdata:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
        ) -> *const u8,

    pub hdata_get_var:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> *mut c_void,

    pub hdata_get_var_at_offset:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            offset: i32,
        ) -> *mut c_void,

    pub hdata_get_list:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            name: *const u8,
        ) -> *mut c_void,

    pub hdata_check_pointer:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            list: *mut c_void,
            pointer: *mut c_void,
        ) -> i32,

    pub hdata_move:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            count: i32,
        ) -> *mut c_void,

    pub hdata_search:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            search: *const u8,
            move_: i32,
        ) -> *mut c_void,

    pub hdata_char:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> u8,

    pub hdata_integer:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> i32,

    pub hdata_long:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> i64,

    pub hdata_string:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> *const u8,

    pub hdata_pointer:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> *mut c_void,

    pub hdata_time:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> time_t,

    pub hdata_hashtable:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
        ) -> *mut t_hashtable,

    pub hdata_compare:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer1: *mut c_void,
            pointer2: *mut c_void,
            name: *const u8,
            case_sensitive: i32,
        ) -> i32,

    pub hdata_set:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            name: *const u8,
            value: *const u8,
        ) -> i32,

    pub hdata_update:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            pointer: *mut c_void,
            hashtable: *mut t_hashtable,
        ) -> i32,

    pub hdata_get_string:
        unsafe extern "C" fn(
            hdata: *mut t_hdata,
            property: *const u8,
        ) -> *const u8,

    /* upgrade */
    pub upgrade_new:
        unsafe extern "C" fn(
            filename: *const u8,
            callback_read:
                unsafe extern "C" fn(
                    pointer: *const c_void,
                    data: *mut c_void,
                    upgrade_file: *mut t_upgrade_file,
                    object_id: i32,
                    infolist: *mut t_infolist,
                ) -> i32,
            callback_read_pointer: *const c_void,
            callback_read_data: *mut c_void,
        ) -> *mut t_upgrade_file,

    pub upgrade_write_object:
        unsafe extern "C" fn(
            upgrade_file: *mut t_upgrade_file,
            object_id: i32,
            infolist: *mut t_infolist,
        ) -> i32,

    pub upgrade_read:
        unsafe extern "C" fn(upgrade_file: *mut t_upgrade_file) -> i32,

    pub upgrade_close:
        unsafe extern "C" fn(upgrade_file: *mut t_upgrade_file),
}

#[test]
fn test_layout_t_weechat_plugin() {
    assert_eq!(
        mem::size_of::<t_weechat_plugin>(),
        2424,
        concat!("Size of: ", stringify!(t_weechat_plugin))
    );
    assert_eq!(
        mem::align_of::<t_weechat_plugin>(),
        8,
        concat!("Alignment of ", stringify!(t_weechat_plugin))
    );
}

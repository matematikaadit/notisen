#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]

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

/* return codes for plugin functions */
pub const WEECHAT_RC_OK:     i32 = 0;
pub const WEECHAT_RC_OK_EAT: i32 = 1;
pub const WEECHAT_RC_ERROR:  i32 = -1;

/* flags for string_split function */
pub const WEECHAT_STRING_SPLIT_STRIP_LEFT:    i32 = (1 << 0)
pub const WEECHAT_STRING_SPLIT_STRIP_RIGHT:   i32 = (1 << 1)
pub const WEECHAT_STRING_SPLIT_COLLAPSE_SEPS: i32 = (1 << 2)
pub const WEECHAT_STRING_SPLIT_KEEP_EOL:      i32 = (1 << 3)

/* return codes for config read functions/callbacks */
pub const WEECHAT_CONFIG_READ_OK:             i32 =  0
pub const WEECHAT_CONFIG_READ_MEMORY_ERROR:   i32 = -1
pub const WEECHAT_CONFIG_READ_FILE_NOT_FOUND: i32 = -2

/* return codes for config write functions/callbacks */
pub const WEECHAT_CONFIG_WRITE_OK:           i32 =  0
pub const WEECHAT_CONFIG_WRITE_ERROR:        i32 = -1
pub const WEECHAT_CONFIG_WRITE_MEMORY_ERROR: i32 = -2

/* null value for option */
// #define WEECHAT_CONFIG_OPTION_NULL                 "null"
// still don't know how to translate this

/* return codes for config option set */
pub const WEECHAT_CONFIG_OPTION_SET_OK_CHANGED:       i32 =  2
pub const WEECHAT_CONFIG_OPTION_SET_OK_SAME_VALUE:    i32 =  1
pub const WEECHAT_CONFIG_OPTION_SET_ERROR:            i32 =  0
pub const WEECHAT_CONFIG_OPTION_SET_OPTION_NOT_FOUND: i32 = -1

/* return codes for config option unset */
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_NO_RESET: i32 =  0
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_RESET:    i32 =  1
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_REMOVED:  i32 =  2
pub const WEECHAT_CONFIG_OPTION_UNSET_ERROR:       i32 = -1

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
pub const WEECHAT_HDATA_OTHER:         i32 = 0
pub const WEECHAT_HDATA_CHAR:          i32 = 1
pub const WEECHAT_HDATA_INTEGER:       i32 = 2
pub const WEECHAT_HDATA_LONG:          i32 = 3
pub const WEECHAT_HDATA_STRING:        i32 = 4
pub const WEECHAT_HDATA_POINTER:       i32 = 5
pub const WEECHAT_HDATA_TIME:          i32 = 6
pub const WEECHAT_HDATA_HASHTABLE:     i32 = 7
pub const WEECHAT_HDATA_SHARED_STRING: i32 = 8

/* flags for hdata lists */
pub const WEECHAT_HDATA_LIST_CHECK_POINTERS: i32 = 1

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
pub const WEECHAT_HOOK_PROCESS_RUNNING: i32 = -1
pub const WEECHAT_HOOK_PROCESS_ERROR:   i32 = -2
pub const WEECHAT_HOOK_PROCESS_CHILD:   i32 = -3

/* connect status for connection hooked */
pub const WEECHAT_HOOK_CONNECT_OK:                     i32 =  0
pub const WEECHAT_HOOK_CONNECT_ADDRESS_NOT_FOUND:      i32 =  1
pub const WEECHAT_HOOK_CONNECT_IP_ADDRESS_NOT_FOUND:   i32 =  2
pub const WEECHAT_HOOK_CONNECT_CONNECTION_REFUSED:     i32 =  3
pub const WEECHAT_HOOK_CONNECT_PROXY_ERROR:            i32 =  4
pub const WEECHAT_HOOK_CONNECT_LOCAL_HOSTNAME_ERROR:   i32 =  5
pub const WEECHAT_HOOK_CONNECT_GNUTLS_INIT_ERROR:      i32 =  6
pub const WEECHAT_HOOK_CONNECT_GNUTLS_HANDSHAKE_ERROR: i32 =  7
pub const WEECHAT_HOOK_CONNECT_MEMORY_ERROR:           i32 =  8
pub const WEECHAT_HOOK_CONNECT_TIMEOUT:                i32 =  9
pub const WEECHAT_HOOK_CONNECT_SOCKET_ERROR:           i32 = 10

/* action for gnutls callback: verify or set certificate */
pub const WEECHAT_HOOK_CONNECT_GNUTLS_CB_VERIFY_CERT: i32 = 0
pub const WEECHAT_HOOK_CONNECT_GNUTLS_CB_SET_CERT:    i32 = 1

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

use core::ffi::c_void;
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
        unsafe extern "C" fn(
            plugin: *mut t_weechat_plugin
        ) -> *const u8,

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
        unsafe extern "C" fn(
            string: *const u8
        ) -> *const u8,
    pub ngettext:
        unsafe extern "C" fn(
            single *const u8,
            plural: *const u8,
            count: i32
        ) -> *const u8,
    pub strndup:
        unsafe extern "C" fn(
            string: *const u8,
            length: i32
        ) -> *mut u8,
    pub string_tolower:
        unsafe extern "C" fn(
            string: *mut u8
        ),
    pub string_toupper:
        unsafe extern "C" fn(
            string: *mut u8
        ),
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
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
    pub string_match:
        unsafe extern "C" fn(
            string: *const u8,
            mask: *const u8,
            case_sensitive: i32,
        ) -> i32,
    >,
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
        unsafe extern "C" fn(
            path: *const u8
        ) -> *mut u8,
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
        unsafe extern "C" fn(
            string: *const u8
        ) -> *mut u8,
    pub string_mask_to_regex:
        unsafe extern "C" fn(
            mask: *const u8
        ) -> *mut u8,
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
        unsafe extern "C" fn(
            split_string: *mut *mut u8
        ),
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
        unsafe extern "C" fn(
            split_command: *mut *mut u8
        ),
    pub string_format_size:
        unsafe extern "C" fn(
            size: u64
        ) -> *mut u8,
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
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
    pub string_input_for_buffer:
        unsafe extern "C" fn(
            string: *const u8,
        ) -> *const u8,
    pub string_eval_expression:
        unsafe extern "C" fn(
            expr: *const u8,
            pointers: *mut t_hashtable,
            extra_vars: *mut t_hashtable,
            options: *mut t_hashtable,
        ) -> *mut u8,
    pub string_dyn_alloc:
        unsafe extern "C" fn(
            size_alloc: i32
        ) -> *mut *mut u8,
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
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
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
        unsafe extern "C" fn(
            string: *const u8,
        ) -> *const u8,
    pub utf8_char_int:
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
    pub utf8_char_size:
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
    pub utf8_strlen:
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
    pub utf8_strnlen:
        unsafe extern "C" fn(
            string: *const u8,
            bytes: i32,
        ) -> i32,
    pub utf8_strlen_screen:
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
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
        unsafe extern "C" fn(
            string: *const u8
        ) -> i32,
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
        unsafe extern "C" fn(
            filename: *const u8,
        ) -> *mut u8,

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
        unsafe extern "C" fn(
            date: *const time_t
        ) -> *const  u8,
    pub util_version_number:
        unsafe extern "C" fn(
            version: *const u8
        ) -> i32,

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
        unsafe extern "C" fn(
            item: *mut t_weelist_item
        ) -> *mut c_void,
    pub list_size:
        unsafe extern "C" fn(
            weelist: *mut t_weelist
        ) -> i32,
    pub list_remove:
        unsafe extern "C" fn(
            weelist: *mut t_weelist,
            item: *mut t_weelist_item
        ),
    pub list_remove_all:
        unsafe extern "C" fn(
            weelist: *mut t_weelist
        ),
    pub list_free:
        unsafe extern "C" fn(
            weelist: *mut t_weelist
        ),

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
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist
        ) -> i32,
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
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist
        ) -> i32,
    pub arraylist_free:
        unsafe extern "C" fn(
            arraylist: *mut t_arraylist
        ),

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
            >,
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
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable
        ) -> *mut t_hashtable,
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
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable
        ),
    pub hashtable_free:
        unsafe extern "C" fn(
            hashtable: *mut t_hashtable
        ),

    // ~~~ PAUSE ~~~

    /* config files */
    struct t_config_file *(*config_new) (struct t_weechat_plugin *plugin,
                                         const char *name,
                                         int (*callback_reload)(const void *pointer,
                                                                void *data,
                                                                struct t_config_file *config_file),
                                         const void *callback_reload_pointer,
                                         void *callback_reload_data);
    struct t_config_section *(*config_new_section) (struct t_config_file *config_file,
                                                    const char *name,
                                                    int user_can_add_options,
                                                    int user_can_delete_options,
                                                    int (*callback_read)(const void *pointer,
                                                                         void *data,
                                                                         struct t_config_file *config_file,
                                                                         struct t_config_section *section,
                                                                         const char *option_name,
                                                                         const char *value),
                                                    const void *callback_read_pointer,
                                                    void *callback_read_data,
                                                    int (*callback_write)(const void *pointer,
                                                                          void *data,
                                                                          struct t_config_file *config_file,
                                                                          const char *section_name),
                                                    const void *callback_write_pointer,
                                                    void *callback_write_data,
                                                    int (*callback_write_default)(const void *pointer,
                                                                                  void *data,
                                                                                  struct t_config_file *config_file,
                                                                                  const char *section_name),
                                                    const void *callback_write_default_pointer,
                                                    void *callback_write_default_data,
                                                    int (*callback_create_option)(const void *pointer,
                                                                                  void *data,
                                                                                  struct t_config_file *config_file,
                                                                                  struct t_config_section *section,
                                                                                  const char *option_name,
                                                                                  const char *value),
                                                    const void *callback_create_option_pointer,
                                                    void *callback_create_option_data,
                                                    int (*callback_delete_option)(const void *pointer,
                                                                                  void *data,
                                                                                  struct t_config_file *config_file,
                                                                                  struct t_config_section *section,
                                                                                  struct t_config_option *option),
                                                    const void *callback_delete_option_pointer,
                                                    void *callback_delete_option_data);
    struct t_config_section *(*config_search_section) (struct t_config_file *config_file,
                                                       const char *section_name);
    struct t_config_option *(*config_new_option) (struct t_config_file *config_file,
                                                  struct t_config_section *section,
                                                  const char *name,
                                                  const char *type,
                                                  const char *description,
                                                  const char *string_values,
                                                  int min,
                                                  int max,
                                                  const char *default_value,
                                                  const char *value,
                                                  int null_value_allowed,
                                                  int (*callback_check_value)(const void *pointer,
                                                                              void *data,
                                                                              struct t_config_option *option,
                                                                              const char *value),
                                                  const void *callback_check_value_pointer,
                                                  void *callback_check_value_data,
                                                  void (*callback_change)(const void *pointer,
                                                                          void *data,
                                                                          struct t_config_option *option),
                                                  const void *callback_change_pointer,
                                                  void *callback_change_data,
                                                  void (*callback_delete)(const void *pointer,
                                                                          void *data,
                                                                          struct t_config_option *option),
                                                  const void *callback_delete_pointer,
                                                  void *callback_delete_data);
    struct t_config_option *(*config_search_option) (struct t_config_file *config_file,
                                                     struct t_config_section *section,
                                                     const char *option_name);
    void (*config_search_section_option) (struct t_config_file *config_file,
                                          struct t_config_section *section,
                                          const char *option_name,
                                          struct t_config_section **section_found,
                                          struct t_config_option **option_found);
    void (*config_search_with_string) (const char *option_name,
                                       struct t_config_file **config_file,
                                       struct t_config_section **section,
                                       struct t_config_option **option,
                                       char **pos_option_name);
    int (*config_string_to_boolean) (const char *text);
    int (*config_option_reset) (struct t_config_option *option,
                                int run_callback);
    int (*config_option_set) (struct t_config_option *option,
                              const char *value, int run_callback);
    int (*config_option_set_null) (struct t_config_option *option,
                                   int run_callback);
    int (*config_option_unset) (struct t_config_option *option);
    void (*config_option_rename) (struct t_config_option *option,
                                  const char *new_name);
    const char *(*config_option_get_string) (struct t_config_option *option,
                                             const char *property);
    void *(*config_option_get_pointer) (struct t_config_option *option,
                                        const char *property);
    int (*config_option_is_null) (struct t_config_option *option);
    int (*config_option_default_is_null) (struct t_config_option *option);
    int (*config_boolean) (struct t_config_option *option);
    int (*config_boolean_default) (struct t_config_option *option);
    int (*config_integer) (struct t_config_option *option);
    int (*config_integer_default) (struct t_config_option *option);
    const char *(*config_string) (struct t_config_option *option);
    const char *(*config_string_default) (struct t_config_option *option);
    const char *(*config_color) (struct t_config_option *option);
    const char *(*config_color_default) (struct t_config_option *option);
    int (*config_write_option) (struct t_config_file *config_file,
                                struct t_config_option *option);
    int (*config_write_line) (struct t_config_file *config_file,
                              const char *option_name,
                              const char *value, ...);
    int (*config_write) (struct t_config_file *config_file);
    int (*config_read) (struct t_config_file *config_file);
    int (*config_reload) (struct t_config_file *config_file);
    void (*config_option_free) (struct t_config_option *option);
    void (*config_section_free_options) (struct t_config_section *section);
    void (*config_section_free) (struct t_config_section *section);
    void (*config_free) (struct t_config_file *config_file);
    struct t_config_option *(*config_get) (const char *option_name);
    const char *(*config_get_plugin) (struct t_weechat_plugin *plugin,
                                      const char *option_name);
    int (*config_is_set_plugin) (struct t_weechat_plugin *plugin,
                                 const char *option_name);
    int (*config_set_plugin) (struct t_weechat_plugin *plugin,
                              const char *option_name, const char *value);
    void (*config_set_desc_plugin) (struct t_weechat_plugin *plugin,
                                    const char *option_name,
                                    const char *description);
    int (*config_unset_plugin) (struct t_weechat_plugin *plugin,
                                const char *option_name);

    /* key bindings */
    int (*key_bind) (const char *context, struct t_hashtable *keys);
    int (*key_unbind) (const char *context, const char *key);

    /* display */
    const char *(*prefix) (const char *prefix);
    const char *(*color) (const char *color_name);
    void (*printf_date_tags) (struct t_gui_buffer *buffer, time_t date,
                              const char *tags, const char *message, ...);
    void (*printf_y) (struct t_gui_buffer *buffer, int y,
                      const char *message, ...);
    void (*log_printf) (const char *message, ...);

    /* hooks */
    struct t_hook *(*hook_command) (struct t_weechat_plugin *plugin,
                                    const char *command,
                                    const char *description,
                                    const char *args,
                                    const char *args_description,
                                    const char *completion,
                                    int (*callback)(const void *pointer,
                                                    void *data,
                                                    struct t_gui_buffer *buffer,
                                                    int argc, char **argv,
                                                    char **argv_eol),
                                    const void *callback_pointer,
                                    void *callback_data);
    struct t_hook *(*hook_command_run) (struct t_weechat_plugin *plugin,
                                        const char *command,
                                        int (*callback)(const void *pointer,
                                                        void *data,
                                                        struct t_gui_buffer *buffer,
                                                        const char *command),
                                        const void *callback_pointer,
                                        void *callback_data);
    struct t_hook *(*hook_timer) (struct t_weechat_plugin *plugin,
                                  long interval,
                                  int align_second,
                                  int max_calls,
                                  int (*callback)(const void *pointer,
                                                  void *data,
                                                  int remaining_calls),
                                  const void *callback_pointer,
                                  void *callback_data);
    struct t_hook *(*hook_fd) (struct t_weechat_plugin *plugin,
                               int fd,
                               int flag_read,
                               int flag_write,
                               int flag_exception,
                               int (*callback)(const void *pointer,
                                               void *data,
                                               int fd),
                               const void *callback_pointer,
                               void *callback_data);
    struct t_hook *(*hook_process) (struct t_weechat_plugin *plugin,
                                    const char *command,
                                    int timeout,
                                    int (*callback)(const void *pointer,
                                                    void *data,
                                                    const char *command,
                                                    int return_code,
                                                    const char *out,
                                                    const char *err),
                                    const void *callback_pointer,
                                    void *callback_data);
    struct t_hook *(*hook_process_hashtable) (struct t_weechat_plugin *plugin,
                                              const char *command,
                                              struct t_hashtable *options,
                                              int timeout,
                                              int (*callback)(const void *pointer,
                                                              void *data,
                                                              const char *command,
                                                              int return_code,
                                                              const char *out,
                                                              const char *err),
                                              const void *callback_pointer,
                                              void *callback_data);
    struct t_hook *(*hook_connect) (struct t_weechat_plugin *plugin,
                                    const char *proxy,
                                    const char *address,
                                    int port,
                                    int ipv6,
                                    int retry,
                                    void *gnutls_sess, void *gnutls_cb,
                                    int gnutls_dhkey_size,
                                    const char *gnutls_priorities,
                                    const char *local_hostname,
                                    int (*callback)(const void *pointer,
                                                    void *data,
                                                    int status,
                                                    int gnutls_rc,
                                                    int sock,
                                                    const char *error,
                                                    const char *ip_address),
                                    const void *callback_pointer,
                                    void *callback_data);
    struct t_hook *(*hook_line) (struct t_weechat_plugin *plugin,
                                 const char *buffer_type,
                                 const char *buffer_name,
                                 const char *tags,
                                 struct t_hashtable *(*callback)(const void *pointer,
                                                                 void *data,
                                                                 struct t_hashtable *line),
                                 const void *callback_pointer,
                                 void *callback_data);
    struct t_hook *(*hook_print) (struct t_weechat_plugin *plugin,
                                  struct t_gui_buffer *buffer,
                                  const char *tags,
                                  const char *message,
                                  int strip_colors,
                                  int (*callback)(const void *pointer,
                                                  void *data,
                                                  struct t_gui_buffer *buffer,
                                                  time_t date,
                                                  int tags_count,
                                                  const char **tags,
                                                  int displayed,
                                                  int highlight,
                                                  const char *prefix,
                                                  const char *message),
                                  const void *callback_pointer,
                                  void *callback_data);
    struct t_hook *(*hook_signal) (struct t_weechat_plugin *plugin,
                                   const char *signal,
                                   int (*callback)(const void *pointer,
                                                   void *data,
                                                   const char *signal,
                                                   const char *type_data,
                                                   void *signal_data),
                                   const void *callback_pointer,
                                   void *callback_data);
    int (*hook_signal_send) (const char *signal, const char *type_data,
                             void *signal_data);
    struct t_hook *(*hook_hsignal) (struct t_weechat_plugin *plugin,
                                    const char *signal,
                                    int (*callback)(const void *pointer,
                                                    void *data,
                                                    const char *signal,
                                                    struct t_hashtable *hashtable),
                                    const void *callback_pointer,
                                    void *callback_data);
    int (*hook_hsignal_send) (const char *signal,
                              struct t_hashtable *hashtable);
    struct t_hook *(*hook_config) (struct t_weechat_plugin *plugin,
                                   const char *option,
                                   int (*callback)(const void *pointer,
                                                   void *data,
                                                   const char *option,
                                                   const char *value),
                                   const void *callback_pointer,
                                   void *callback_data);
    struct t_hook *(*hook_completion) (struct t_weechat_plugin *plugin,
                                       const char *completion_item,
                                       const char *description,
                                       int (*callback)(const void *pointer,
                                                       void *data,
                                                       const char *completion_item,
                                                       struct t_gui_buffer *buffer,
                                                       struct t_gui_completion *completion),
                                       const void *callback_pointer,
                                       void *callback_data);
    const char *(*hook_completion_get_string) (struct t_gui_completion *completion,
                                               const char *property);
    void (*hook_completion_list_add) (struct t_gui_completion *completion,
                                      const char *word,
                                      int nick_completion,
                                      const char *where);
    struct t_hook *(*hook_modifier) (struct t_weechat_plugin *plugin,
                                     const char *modifier,
                                     char *(*callback)(const void *pointer,
                                                       void *data,
                                                       const char *modifier,
                                                       const char *modifier_data,
                                                       const char *string),
                                     const void *callback_pointer,
                                     void *callback_data);
    char *(*hook_modifier_exec) (struct t_weechat_plugin *plugin,
                                 const char *modifier,
                                 const char *modifier_data,
                                 const char *string);
    struct t_hook *(*hook_info) (struct t_weechat_plugin *plugin,
                                 const char *info_name,
                                 const char *description,
                                 const char *args_description,
                                 char *(*callback)(const void *pointer,
                                                   void *data,
                                                   const char *info_name,
                                                   const char *arguments),
                                 const void *callback_pointer,
                                 void *callback_data);
    struct t_hook *(*hook_info_hashtable) (struct t_weechat_plugin *plugin,
                                           const char *info_name,
                                           const char *description,
                                           const char *args_description,
                                           const char *output_description,
                                           struct t_hashtable *(*callback)(const void *pointer,
                                                                           void *data,
                                                                           const char *info_name,
                                                                           struct t_hashtable *hashtable),
                                           const void *callback_pointer,
                                           void *callback_data);
    struct t_hook *(*hook_infolist) (struct t_weechat_plugin *plugin,
                                     const char *infolist_name,
                                     const char *description,
                                     const char *pointer_description,
                                     const char *args_description,
                                     struct t_infolist *(*callback)(const void *cb_pointer,
                                                                    void *data,
                                                                    const char *infolist_name,
                                                                    void *obj_pointer,
                                                                    const char *arguments),
                                     const void *callback_pointer,
                                     void *callback_data);
    struct t_hook *(*hook_hdata) (struct t_weechat_plugin *plugin,
                                  const char *hdata_name,
                                  const char *description,
                                  struct t_hdata *(*callback)(const void *pointer,
                                                              void *data,
                                                              const char *hdata_name),
                                  const void *callback_pointer,
                                  void *callback_data);
    struct t_hook *(*hook_focus) (struct t_weechat_plugin *plugin,
                                  const char *area,
                                  struct t_hashtable *(*callback)(const void *pointer,
                                                                  void *data,
                                                                  struct t_hashtable *info),
                                  const void *callback_pointer,
                                  void *callback_data);
    void (*hook_set) (struct t_hook *hook, const char *property,
                      const char *value);
    void (*unhook) (struct t_hook *hook);
    void (*unhook_all) (struct t_weechat_plugin *plugin,
                        const char *subplugin);

    /* buffers */
    struct t_gui_buffer *(*buffer_new) (struct t_weechat_plugin *plugin,
                                        const char *name,
                                        int (*input_callback)(const void *pointer,
                                                              void *data,
                                                              struct t_gui_buffer *buffer,
                                                              const char *input_data),
                                        const void *input_callback_pointer,
                                        void *input_callback_data,
                                        int (*close_callback)(const void *pointer,
                                                              void *data,
                                                              struct t_gui_buffer *buffer),
                                        const void *close_callback_pointer,
                                        void *close_callback_data);
    struct t_gui_buffer *(*buffer_search) (const char *plugin, const char *name);
    struct t_gui_buffer *(*buffer_search_main) ();
    void (*buffer_clear) (struct t_gui_buffer *buffer);
    void (*buffer_close) (struct t_gui_buffer *buffer);
    void (*buffer_merge) (struct t_gui_buffer *buffer,
                           struct t_gui_buffer *target_buffer);
    void (*buffer_unmerge) (struct t_gui_buffer *buffer, int number);
    int (*buffer_get_integer) (struct t_gui_buffer *buffer,
                               const char *property);
    const char *(*buffer_get_string) (struct t_gui_buffer *buffer,
                                      const char *property);
    void *(*buffer_get_pointer) (struct t_gui_buffer *buffer,
                                 const char *property);
    void (*buffer_set) (struct t_gui_buffer *buffer, const char *property,
                        const char *value);
    void (*buffer_set_pointer) (struct t_gui_buffer *buffer,
                                const char *property, void *pointer);
    char *(*buffer_string_replace_local_var) (struct t_gui_buffer *buffer,
                                              const char *string);
    int (*buffer_match_list) (struct t_gui_buffer *buffer, const char *string);

    /* windows */
    struct t_gui_window *(*window_search_with_buffer) (struct t_gui_buffer *buffer);
    int (*window_get_integer) (struct t_gui_window *window,
                               const char *property);
    const char *(*window_get_string) (struct t_gui_window *window,
                                      const char *property);
    void *(*window_get_pointer) (struct t_gui_window *window,
                                 const char *property);
    void (*window_set_title) (const char *title);

    /* nicklist */
    struct t_gui_nick_group *(*nicklist_add_group) (struct t_gui_buffer *buffer,
                                                    struct t_gui_nick_group *parent_group,
                                                    const char *name,
                                                    const char *color,
                                                    int visible);
    struct t_gui_nick_group *(*nicklist_search_group) (struct t_gui_buffer *buffer,
                                                       struct t_gui_nick_group *from_group,
                                                       const char *name);
    struct t_gui_nick *(*nicklist_add_nick) (struct t_gui_buffer *buffer,
                                             struct t_gui_nick_group *group,
                                             const char *name,
                                             const char *color,
                                             const char *prefix,
                                             const char *prefix_color,
                                             int visible);
    struct t_gui_nick *(*nicklist_search_nick) (struct t_gui_buffer *buffer,
                                                struct t_gui_nick_group *from_group,
                                                const char *name);
    void (*nicklist_remove_group) (struct t_gui_buffer *buffer,
                                   struct t_gui_nick_group *group);
    void (*nicklist_remove_nick) (struct t_gui_buffer *buffer,
                                  struct t_gui_nick *nick);
    void (*nicklist_remove_all) (struct t_gui_buffer *buffer);
    void (*nicklist_get_next_item) (struct t_gui_buffer *buffer,
                                    struct t_gui_nick_group **group,
                                    struct t_gui_nick **nick);
    int (*nicklist_group_get_integer) (struct t_gui_buffer *buffer,
                                       struct t_gui_nick_group *group,
                                       const char *property);
    const char *(*nicklist_group_get_string) (struct t_gui_buffer *buffer,
                                              struct t_gui_nick_group *group,
                                              const char *property);
    void *(*nicklist_group_get_pointer) (struct t_gui_buffer *buffer,
                                         struct t_gui_nick_group *group,
                                         const char *property);
    void (*nicklist_group_set) (struct t_gui_buffer *buffer,
                                struct t_gui_nick_group *group,
                                const char *property, const char *value);
    int (*nicklist_nick_get_integer) (struct t_gui_buffer *buffer,
                                      struct t_gui_nick *nick,
                                      const char *property);
    const char *(*nicklist_nick_get_string) (struct t_gui_buffer *buffer,
                                             struct t_gui_nick *nick,
                                             const char *property);
    void *(*nicklist_nick_get_pointer) (struct t_gui_buffer *buffer,
                                        struct t_gui_nick *nick,
                                        const char *property);
    void (*nicklist_nick_set) (struct t_gui_buffer *buffer,
                               struct t_gui_nick *nick,
                               const char *property, const char *value);

    /* bars */
    struct t_gui_bar_item *(*bar_item_search) (const char *name);
    struct t_gui_bar_item *(*bar_item_new) (struct t_weechat_plugin *plugin,
                                            const char *name,
                                            char *(*build_callback)(const void *pointer,
                                                                    void *data,
                                                                    struct t_gui_bar_item *item,
                                                                    struct t_gui_window *window,
                                                                    struct t_gui_buffer *buffer,
                                                                    struct t_hashtable *extra_info),
                                            const void *build_callback_pointer,
                                            void *build_callback_data);
    void (*bar_item_update) (const char *name);
    void (*bar_item_remove) (struct t_gui_bar_item *item);
    struct t_gui_bar *(*bar_search) (const char *name);
    struct t_gui_bar *(*bar_new) (const char *name,
                                  const char *hidden,
                                  const char *priority,
                                  const char *type,
                                  const char *condition,
                                  const char *position,
                                  const char *filling_top_bottom,
                                  const char *filling_left_right,
                                  const char *size,
                                  const char *size_max,
                                  const char *color_fg,
                                  const char *color_delim,
                                  const char *color_bg,
                                  const char *separator,
                                  const char *items);
    int (*bar_set) (struct t_gui_bar *bar, const char *property,
                    const char *value);
    void (*bar_update) (const char *name);
    void (*bar_remove) (struct t_gui_bar *bar);

    /* command */
    int (*command) (struct t_weechat_plugin *plugin,
                    struct t_gui_buffer *buffer, const char *command);
    int (*command_options) (struct t_weechat_plugin *plugin,
                            struct t_gui_buffer *buffer, const char *command,
                            struct t_hashtable *options);

    /* network */
    int (*network_pass_proxy) (const char *proxy, int sock,
                               const char *address, int port);
    int (*network_connect_to) (const char *proxy,
                               struct sockaddr *address,
                               socklen_t address_length);

    /* infos */
    char *(*info_get) (struct t_weechat_plugin *plugin, const char *info_name,
                       const char *arguments);
    struct t_hashtable *(*info_get_hashtable) (struct t_weechat_plugin *plugin,
                                               const char *info_name,
                                               struct t_hashtable *hashtable);

    /* infolists */
    struct t_infolist *(*infolist_new) (struct t_weechat_plugin *plugin);
    struct t_infolist_item *(*infolist_new_item) (struct t_infolist *infolist);
    struct t_infolist_var *(*infolist_new_var_integer) (struct t_infolist_item *item,
                                                        const char *name,
                                                        int value);
    struct t_infolist_var *(*infolist_new_var_string) (struct t_infolist_item *item,
                                                       const char *name,
                                                       const char *value);
    struct t_infolist_var *(*infolist_new_var_pointer) (struct t_infolist_item *item,
                                                        const char *name,
                                                        void *pointer);
    struct t_infolist_var *(*infolist_new_var_buffer) (struct t_infolist_item *item,
                                                       const char *name,
                                                       void *pointer,
                                                       int size);
    struct t_infolist_var *(*infolist_new_var_time) (struct t_infolist_item *item,
                                                     const char *name,
                                                     time_t time);
    struct t_infolist_var *(*infolist_search_var) (struct t_infolist *infolist,
                                                   const char *name);
    struct t_infolist *(*infolist_get) (struct t_weechat_plugin *plugin,
                                        const char *infolist_name,
                                        void *pointer,
                                        const char *arguments);
    int (*infolist_next) (struct t_infolist *infolist);
    int (*infolist_prev) (struct t_infolist *infolist);
    void (*infolist_reset_item_cursor) (struct t_infolist *infolist);
    const char *(*infolist_fields) (struct t_infolist *infolist);
    int (*infolist_integer) (struct t_infolist *infolist, const char *var);
    const char *(*infolist_string) (struct t_infolist *infolist, const char *var);
    void *(*infolist_pointer) (struct t_infolist *infolist, const char *var);
    void *(*infolist_buffer) (struct t_infolist *infolist, const char *var,
                              int *size);
    time_t (*infolist_time) (struct t_infolist *infolist, const char *var);
    void (*infolist_free) (struct t_infolist *infolist);

    /* hdata */
    struct t_hdata *(*hdata_new) (struct t_weechat_plugin *plugin,
                                  const char *hdata_name, const char *var_prev,
                                  const char *var_next,
                                  int create_allowed, int delete_allowed,
                                  int (*callback_update)(void *data,
                                                         struct t_hdata *hdata,
                                                         void *pointer,
                                                         struct t_hashtable *hashtable),
                                  void *callback_update_data);
    void (*hdata_new_var) (struct t_hdata *hdata, const char *name, int offset,
                           int type, int update_allowed, const char *array_size,
                           const char *hdata_name);
    void (*hdata_new_list) (struct t_hdata *hdata, const char *name,
                            void *pointer, int flags);
    struct t_hdata *(*hdata_get) (struct t_weechat_plugin *plugin,
                                  const char *hdata_name);
    int (*hdata_get_var_offset) (struct t_hdata *hdata, const char *name);
    int (*hdata_get_var_type) (struct t_hdata *hdata, const char *name);
    const char *(*hdata_get_var_type_string) (struct t_hdata *hdata,
                                              const char *name);
    int (*hdata_get_var_array_size) (struct t_hdata *hdata, void *pointer,
                                     const char *name);
    const char *(*hdata_get_var_array_size_string) (struct t_hdata *hdata,
                                                    void *pointer,
                                                    const char *name);
    const char *(*hdata_get_var_hdata) (struct t_hdata *hdata,
                                        const char *name);
    void *(*hdata_get_var) (struct t_hdata *hdata, void *pointer,
                            const char *name);
    void *(*hdata_get_var_at_offset) (struct t_hdata *hdata, void *pointer,
                                      int offset);
    void *(*hdata_get_list) (struct t_hdata *hdata, const char *name);
    int (*hdata_check_pointer) (struct t_hdata *hdata, void *list,
                                void *pointer);
    void *(*hdata_move) (struct t_hdata *hdata, void *pointer, int count);
    void *(*hdata_search) (struct t_hdata *hdata, void *pointer,
                           const char *search, int move);
    char (*hdata_char) (struct t_hdata *hdata, void *pointer,
                        const char *name);
    int (*hdata_integer) (struct t_hdata *hdata, void *pointer,
                          const char *name);
    long (*hdata_long) (struct t_hdata *hdata, void *pointer,
                        const char *name);
    const char *(*hdata_string) (struct t_hdata *hdata, void *pointer,
                                 const char *name);
    void *(*hdata_pointer) (struct t_hdata *hdata, void *pointer,
                            const char *name);
    time_t (*hdata_time) (struct t_hdata *hdata, void *pointer,
                          const char *name);
    struct t_hashtable *(*hdata_hashtable) (struct t_hdata *hdata,
                                            void *pointer, const char *name);
    int (*hdata_compare) (struct t_hdata *hdata,
                          void *pointer1, void *pointer2, const char *name,
                          int case_sensitive);
    int (*hdata_set) (struct t_hdata *hdata, void *pointer, const char *name,
                      const char *value);
    int (*hdata_update) (struct t_hdata *hdata, void *pointer,
                         struct t_hashtable *hashtable);
    const char *(*hdata_get_string) (struct t_hdata *hdata,
                                     const char *property);

    /* upgrade */
    struct t_upgrade_file *(*upgrade_new) (const char *filename,
                                           int (*callback_read)(const void *pointer,
                                                                void *data,
                                                                struct t_upgrade_file *upgrade_file,
                                                                int object_id,
                                                                struct t_infolist *infolist),
                                           const void *callback_read_pointer,
                                           void *callback_read_data);
    int (*upgrade_write_object) (struct t_upgrade_file *upgrade_file,
                                 int object_id,
                                 struct t_infolist *infolist);
    int (*upgrade_read) (struct t_upgrade_file *upgrade_file);
    void (*upgrade_close) (struct t_upgrade_file *upgrade_file);
};

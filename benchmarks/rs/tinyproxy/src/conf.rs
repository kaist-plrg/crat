use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type htab;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn sblist_free(l: *mut sblist);
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn htab_next(
        _: *mut htab,
        iterator: size_t,
        key: *mut *mut libc::c_char,
        v: *mut *mut htab_value,
    ) -> size_t;
    fn insert_acl(
        location: *mut libc::c_char,
        access_type: acl_access_t,
        access_list: *mut acl_list_t,
    ) -> libc::c_int;
    fn flush_access_list(access_list: acl_list_t);
    fn htab_destroy(_: *mut htab);
    fn anonymous_insert(conf: *mut config_s, s: *mut libc::c_char) -> libc::c_int;
    fn add_new_errorpage(
        _: *mut config_s,
        filepath: *mut libc::c_char,
        errornum: libc::c_uint,
    ) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn set_log_level(level: libc::c_int);
    fn reversepath_add(
        path: *const libc::c_char,
        url: *const libc::c_char,
        reversepath_list: *mut *mut reversepath,
    );
    fn free_reversepath_list(reverse: *mut reversepath);
    fn free_upstream_list(up: *mut upstream);
    fn upstream_add(
        host: *const libc::c_char,
        port: libc::c_int,
        domain: *mut libc::c_char,
        user: *const libc::c_char,
        pass: *const libc::c_char,
        type_0: proxy_type,
        upstream_list: *mut *mut upstream,
    ) -> upstream_build_error;
    fn upstream_build_error_string(_: upstream_build_error) -> *const libc::c_char;
    fn add_connect_port_allowed(port: libc::c_int, connect_ports: *mut *mut sblist);
    fn free_connect_ports_list(connect_ports: *mut sblist);
    fn basicauth_add(
        authlist: *mut sblist,
        user: *const libc::c_char,
        pass: *const libc::c_char,
    );
    fn config_directive_find(
        str: *const libc::c_char,
        len: size_t,
    ) -> *const config_directive_entry;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union htab_value {
    pub p: *mut libc::c_void,
    pub n: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_access_t = libc::c_uint;
pub const ACL_DENY: acl_access_t = 1;
pub const ACL_ALLOW: acl_access_t = 0;
pub type acl_list_t = *mut sblist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct upstream {
    pub next: *mut upstream,
    pub host: *mut libc::c_char,
    pub ua: C2RustUnnamed_2,
    pub pass: *mut libc::c_char,
    pub port: libc::c_int,
    pub target: hostspec,
    pub type_0: proxy_type,
}
pub type proxy_type = libc::c_uint;
pub const PT_SOCKS5: proxy_type = 3;
pub const PT_SOCKS4: proxy_type = 2;
pub const PT_HTTP: proxy_type = 1;
pub const PT_NONE: proxy_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostspec {
    pub type_0: hostspec_type,
    pub address: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: *mut libc::c_char,
    pub ip: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub network: [libc::c_uchar; 16],
    pub mask: [libc::c_uchar; 16],
}
pub type hostspec_type = libc::c_uint;
pub const HST_NUMERIC: hostspec_type = 2;
pub const HST_STRING: hostspec_type = 1;
pub const HST_NONE: hostspec_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub user: *mut libc::c_char,
    pub authstr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reversepath {
    pub next: *mut reversepath,
    pub path: *mut libc::c_char,
    pub url: *mut libc::c_char,
}
pub type config_directive = libc::c_uint;
pub const CD_loglevel: config_directive = 40;
pub const CD_upstream: config_directive = 39;
pub const CD_reversepath: config_directive = 38;
pub const CD_reversemagic: config_directive = 37;
pub const CD_reverseonly: config_directive = 36;
pub const CD_reversebaseurl: config_directive = 35;
pub const CD_filtercasesensitive: config_directive = 34;
pub const CD_filterdefaultdeny: config_directive = 33;
pub const CD_filterextended: config_directive = 32;
pub const CD_filtertype: config_directive = 31;
pub const CD_filterurls: config_directive = 30;
pub const CD_filter: config_directive = 29;
pub const CD_addheader: config_directive = 28;
pub const CD_errorfile: config_directive = 27;
pub const CD_basicauth: config_directive = 26;
pub const CD_bind: config_directive = 25;
pub const CD_deny: config_directive = 24;
pub const CD_allow: config_directive = 23;
pub const CD_listen: config_directive = 22;
pub const CD_group: config_directive = 21;
pub const CD_user: config_directive = 20;
pub const CD_connectport: config_directive = 19;
pub const CD_timeout: config_directive = 18;
pub const CD_maxrequestsperchild: config_directive = 17;
pub const CD_startservers: config_directive = 16;
pub const CD_minspareservers: config_directive = 15;
pub const CD_maxspareservers: config_directive = 14;
pub const CD_maxclients: config_directive = 13;
pub const CD_port: config_directive = 12;
pub const CD_disableviaheader: config_directive = 11;
pub const CD_bindsame: config_directive = 10;
pub const CD_syslog: config_directive = 9;
pub const CD_xtinyproxy: config_directive = 8;
pub const CD_stathost: config_directive = 7;
pub const CD_statfile: config_directive = 6;
pub const CD_defaulterrorfile: config_directive = 5;
pub const CD_viaproxyname: config_directive = 4;
pub const CD_anonymous: config_directive = 3;
pub const CD_pidfile: config_directive = 2;
pub const CD_logfile: config_directive = 1;
pub const CD_NIL: config_directive = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_directive_entry {
    pub name: *const libc::c_char,
    pub value: config_directive,
}
pub type CONFFILE_HANDLER = Option::<
    unsafe extern "C" fn(
        *mut config_s,
        *const libc::c_char,
        libc::c_ulong,
        *mut regmatch_t,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub re: *const libc::c_char,
    pub handler: CONFFILE_HANDLER,
    pub cre: *mut regex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_levels_s {
    pub string: *const libc::c_char,
    pub level: libc::c_int,
}
pub type upstream_build_error = libc::c_uint;
pub const UBE_NETMASK: upstream_build_error = 6;
pub const UBE_INVPARAMS: upstream_build_error = 5;
pub const UBE_INVHOST: upstream_build_error = 4;
pub const UBE_EDOMAIN: upstream_build_error = 3;
pub const UBE_USERLEN: upstream_build_error = 2;
pub const UBE_OOM: upstream_build_error = 1;
pub const UBE_SUCCESS: upstream_build_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub flag: libc::c_ushort,
    pub type_0: [libc::c_char; 8],
}
pub const FILTER_OPT_TYPE_FNMATCH: filter_options = 1024;
pub const FILTER_OPT_TYPE_BRE: filter_options = 256;
pub const FILTER_OPT_TYPE_ERE: filter_options = 512;
pub const FILTER_OPT_CASESENSITIVE: filter_options = 1;
pub const FILTER_OPT_DEFAULT_DENY: filter_options = 4;
pub const FILTER_OPT_URL: filter_options = 2;
pub type filter_options = libc::c_uint;
unsafe extern "C" fn handle_disabled_feature(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    fprintf(
        stderr,
        b"ERROR: accessing feature that was disabled at compiletime on line %lu\n\0"
            as *const u8 as *const libc::c_char,
        lineno,
    );
    return -(1 as libc::c_int);
}
pub static mut directives: [C2RustUnnamed_3; 41] = unsafe {
    [
        C2RustUnnamed_3 {
            re: 0 as *const libc::c_char,
            handler: None,
            cre: 0 as *const regex_t as *mut regex_t,
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_logfile
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_pidfile
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_anonymous
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_viaproxyname
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_defaulterrorfile
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_statfile
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_stathost
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_xtinyproxy
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_syslog
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_bindsame
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_disableviaheader
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_port
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_maxclients
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_obsolete
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_obsolete
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_obsolete
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_obsolete
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_timeout
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_connectport
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+([-a-z0-9._]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_user
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+([-a-z0-9._]+)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_group
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(((([0-9]{1,3})\\.){3}[0-9]{1,3})|(([0-9a-f:]{2,39})((%[^ \t\\/]{1,16})?)|([0-9a-f:]{0,29}:((([0-9]{1,3})\\.){3}[0-9]{1,3}))((%[^ \t\\/]{1,16})?)))[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_listen
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(((((([0-9]{1,3})\\.){3}[0-9]{1,3})(/[0-9]+)?)|((([0-9a-f:]{2,39})((%[^ \t\\/]{1,16})?)|([0-9a-f:]{0,29}:((([0-9]{1,3})\\.){3}[0-9]{1,3}))((%[^ \t\\/]{1,16})?))(/[0-9]+)?))|([-a-z0-9._]+))[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_allow
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(((((([0-9]{1,3})\\.){3}[0-9]{1,3})(/[0-9]+)?)|((([0-9a-f:]{2,39})((%[^ \t\\/]{1,16})?)|([0-9a-f:]{0,29}:((([0-9]{1,3})\\.){3}[0-9]{1,3}))((%[^ \t\\/]{1,16})?))(/[0-9]+)?))|([-a-z0-9._]+))[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_deny
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(((([0-9]{1,3})\\.){3}[0-9]{1,3})|(([0-9a-f:]{2,39})((%[^ \t\\/]{1,16})?)|([0-9a-f:]{0,29}:((([0-9]{1,3})\\.){3}[0-9]{1,3}))((%[^ \t\\/]{1,16})?)))[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_bind
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+([^:]*)[ \t]+([^@]*)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_basicauth
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(()[0-9]+)[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_errorfile
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]+\"([^\"]+)\"[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_addheader
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_filter
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_filterurls
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(bre|ere|fnmatch)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_filtertype
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_filterextended
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_filterdefaultdeny
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_filtercasesensitive
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_reversebaseurl
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_reverseonly
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(yes|on|no|off)[ \t]*$\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    handle_reversemagic
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+\"([^\"]+)\"([ \t]+\"([^\"]+)\")?[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_reversepath
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+((none)[ \t]+\"([^\"]+)\")|((http|socks4|socks5)[ \t]+(([^:]*):([^@]*)@)?(((([0-9]{1,3})\\.){3}[0-9]{1,3})|\\[((([0-9a-f:]{2,39})((%[^ \t\\/]{1,16})?)|([0-9a-f:]{0,29}:((([0-9]{1,3})\\.){3}[0-9]{1,3}))((%[^ \t\\/]{1,16})?)))\\]|([-a-z0-9._]+)):(()[0-9]+)([ \t]+\"([^\"]+)\")?)[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_upstream
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                re: b"^[ \t]*()[ \t]+(critical|error|warning|notice|connect|info)[ \t]*$\0"
                    as *const u8 as *const libc::c_char,
                handler: Some(
                    handle_loglevel
                        as unsafe extern "C" fn(
                            *mut config_s,
                            *const libc::c_char,
                            libc::c_ulong,
                            *mut regmatch_t,
                        ) -> libc::c_int,
                ),
                cre: 0 as *const regex_t as *mut regex_t,
            };
            init
        },
    ]
};
pub static mut ndirectives: libc::c_uint = 0;
unsafe extern "C" fn free_added_headers(mut add_headers: *mut sblist) {
    let mut i: size_t = 0;
    if add_headers.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*add_headers).count {
        let mut header: *mut http_header_t = sblist_get(add_headers, i)
            as *mut http_header_t;
        free((*header).name as *mut libc::c_void);
        (*header).name = 0 as *mut libc::c_char;
        free((*header).value as *mut libc::c_void);
        (*header).value = 0 as *mut libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    sblist_free(add_headers);
}
unsafe extern "C" fn stringlist_free(mut sl: *mut sblist) {
    let mut i: size_t = 0;
    let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !sl.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*sl).count {
            s = sblist_get(sl, i) as *mut *mut libc::c_char;
            if !s.is_null() {
                free(*s as *mut libc::c_void);
                *s = 0 as *mut libc::c_char;
            }
            i = i.wrapping_add(1);
            i;
        }
        sblist_free(sl);
    }
}
pub unsafe extern "C" fn free_config(mut conf: *mut config_s) {
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut htab_value = 0 as *mut htab_value;
    let mut it: size_t = 0;
    free((*conf).logf_name as *mut libc::c_void);
    (*conf).logf_name = 0 as *mut libc::c_char;
    free((*conf).stathost as *mut libc::c_void);
    (*conf).stathost = 0 as *mut libc::c_char;
    free((*conf).user as *mut libc::c_void);
    (*conf).user = 0 as *mut libc::c_char;
    free((*conf).group as *mut libc::c_void);
    (*conf).group = 0 as *mut libc::c_char;
    stringlist_free((*conf).basicauth_list);
    stringlist_free((*conf).listen_addrs);
    stringlist_free((*conf).bind_addrs);
    free((*conf).filter as *mut libc::c_void);
    (*conf).filter = 0 as *mut libc::c_char;
    free_reversepath_list((*conf).reversepath_list);
    free((*conf).reversebaseurl as *mut libc::c_void);
    (*conf).reversebaseurl = 0 as *mut libc::c_char;
    free_upstream_list((*conf).upstream_list);
    free((*conf).pidpath as *mut libc::c_void);
    (*conf).pidpath = 0 as *mut libc::c_char;
    free((*conf).via_proxy_name as *mut libc::c_void);
    (*conf).via_proxy_name = 0 as *mut libc::c_char;
    if !((*conf).errorpages).is_null() {
        it = 0 as libc::c_int as size_t;
        loop {
            it = htab_next((*conf).errorpages, it, &mut k, &mut v);
            if !(it != 0) {
                break;
            }
            free(k as *mut libc::c_void);
            k = 0 as *mut libc::c_char;
            free((*v).p);
            (*v).p = 0 as *mut libc::c_void;
        }
        htab_destroy((*conf).errorpages);
    }
    free_added_headers((*conf).add_headers);
    free((*conf).errorpage_undef as *mut libc::c_void);
    (*conf).errorpage_undef = 0 as *mut libc::c_char;
    free((*conf).statpage as *mut libc::c_void);
    (*conf).statpage = 0 as *mut libc::c_char;
    flush_access_list((*conf).access_list);
    free_connect_ports_list((*conf).connect_ports);
    if !((*conf).anonymous_map).is_null() {
        it = 0 as libc::c_int as size_t;
        loop {
            it = htab_next((*conf).anonymous_map, it, &mut k, &mut v);
            if !(it != 0) {
                break;
            }
            free(k as *mut libc::c_void);
            k = 0 as *mut libc::c_char;
        }
        htab_destroy((*conf).anonymous_map);
    }
    memset(
        conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<config_s>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn config_init() -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut r: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i != ndirectives {
        if (directives[i as usize].handler).is_none() {
            directives[i as usize]
                .handler = Some(
                handle_disabled_feature
                    as unsafe extern "C" fn(
                        *mut config_s,
                        *const libc::c_char,
                        libc::c_ulong,
                        *mut regmatch_t,
                    ) -> libc::c_int,
            );
        } else {
            directives[i as usize]
                .cre = malloc(::std::mem::size_of::<regex_t>() as libc::c_ulong)
                as *mut regex_t;
            if (directives[i as usize].cre).is_null() {
                return -(1 as libc::c_int);
            }
            r = regcomp(
                directives[i as usize].cre,
                directives[i as usize].re,
                1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int,
            ) as libc::c_uint;
            if r != 0 {
                return r as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    atexit(Some(config_free_regex as unsafe extern "C" fn() -> ()));
    return 0 as libc::c_int;
}
unsafe extern "C" fn config_free_regex() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ndirectives {
        if !(directives[i as usize].cre).is_null() {
            regfree(directives[i as usize].cre);
            free(directives[i as usize].cre as *mut libc::c_void);
            directives[i as usize].cre = 0 as *mut regex_t;
            directives[i as usize].cre = 0 as *mut regex_t;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn check_match(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut cd: config_directive,
) -> libc::c_int {
    let mut match_0: [regmatch_t; 33] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 33];
    let mut i: libc::c_uint = cd as libc::c_uint;
    if (directives[i as usize].cre).is_null() {
        return (Some(((*directives.as_mut_ptr().offset(i as isize)).handler).unwrap()))
            .unwrap()(conf, line, lineno, match_0.as_mut_ptr());
    }
    if regexec(
        directives[i as usize].cre,
        line,
        33 as libc::c_int as size_t,
        match_0.as_mut_ptr(),
        0 as libc::c_int,
    ) == 0
    {
        return (Some(((*directives.as_mut_ptr().offset(i as isize)).handler).unwrap()))
            .unwrap()(conf, line, lineno, match_0.as_mut_ptr());
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn config_parse(
    mut conf: *mut config_s,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 2048] = [0; 2048];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut e: *const config_directive_entry = 0 as *const config_directive_entry;
    let mut lineno: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
    while !(fgets(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        if !(buffer[0 as libc::c_int as usize] as libc::c_int == '#' as i32) {
            p = buffer.as_mut_ptr();
            while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                p = p.offset(1);
                p;
            }
            if !(*p == 0) {
                q = p;
                while *q as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(*q as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    q = q.offset(1);
                    q;
                }
                c = *q;
                *q = 0 as libc::c_int as libc::c_char;
                e = config_directive_find(p, strlen(p));
                *q = c;
                if e.is_null()
                    || (*e).value as libc::c_uint
                        == CD_NIL as libc::c_int as libc::c_uint
                    || check_match(conf, q, lineno, (*e).value) != 0
                {
                    fprintf(
                        stderr,
                        b"ERROR: Syntax error on line %lu\n\0" as *const u8
                            as *const libc::c_char,
                        lineno,
                    );
                    return 1 as libc::c_int;
                }
            }
        }
        lineno = lineno.wrapping_add(1);
        lineno;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn load_config_file(
    mut config_fname: *const libc::c_char,
    mut conf: *mut config_s,
) -> libc::c_int {
    let mut config_file: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    config_file = fopen(config_fname, b"r\0" as *const u8 as *const libc::c_char);
    if config_file.is_null() {
        fprintf(
            stderr,
            b"%s: Could not open config file \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            b"tinyproxy\0" as *const u8 as *const libc::c_char,
            config_fname,
        );
    } else if config_parse(conf, config_file) != 0 {
        fprintf(
            stderr,
            b"Unable to parse config file. Not starting.\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        ret = 0 as libc::c_int;
    }
    if !config_file.is_null() {
        fclose(config_file);
    }
    return ret;
}
unsafe extern "C" fn initialize_config_defaults(mut conf: *mut config_s) {
    memset(
        conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<config_s>() as libc::c_ulong,
    );
    (*conf).errorpages = 0 as *mut htab;
    (*conf).stathost = strdup(b"tinyproxy.stats\0" as *const u8 as *const libc::c_char);
    (*conf).idletimeout = (60 as libc::c_int * 10 as libc::c_int) as libc::c_uint;
    (*conf).logf_name = 0 as *mut libc::c_char;
    (*conf).pidpath = 0 as *mut libc::c_char;
    (*conf).maxclients = 100 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn reload_config_file(
    mut config_fname: *const libc::c_char,
    mut conf: *mut config_s,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    initialize_config_defaults(conf);
    ret = load_config_file(config_fname, conf);
    if !(ret != 0 as libc::c_int) {
        if (*conf).port == 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"tinyproxy: You MUST set a Port in the config file.\n\0" as *const u8
                    as *const libc::c_char,
            );
            ret = -(1 as libc::c_int);
        } else {
            if ((*conf).user).is_null() && geteuid() == 0 {
                log_message(
                    4 as libc::c_int,
                    b"You SHOULD set a UserName in the config file. Using current user instead.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if (*conf).idletimeout == 0 as libc::c_int as libc::c_uint {
                log_message(
                    4 as libc::c_int,
                    b"Invalid idle time setting. Only values greater than zero are allowed. Therefore setting idle timeout to %u seconds.\0"
                        as *const u8 as *const libc::c_char,
                    60 as libc::c_int * 10 as libc::c_int,
                );
                (*conf)
                    .idletimeout = (60 as libc::c_int * 10 as libc::c_int)
                    as libc::c_uint;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn get_string_arg(
    mut line: *const libc::c_char,
    mut match_0: *mut regmatch_t,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let len: libc::c_uint = ((*match_0).rm_eo - (*match_0).rm_so) as libc::c_uint;
    p = malloc(len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        as *mut libc::c_char;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    memcpy(
        p as *mut libc::c_void,
        line.offset((*match_0).rm_so as isize) as *const libc::c_void,
        len as libc::c_ulong,
    );
    *p.offset(len as isize) = '\0' as i32 as libc::c_char;
    return p;
}
unsafe extern "C" fn set_string_arg(
    mut var: *mut *mut libc::c_char,
    mut line: *const libc::c_char,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = get_string_arg(line, match_0);
    if arg.is_null() {
        return -(1 as libc::c_int);
    }
    if !(*var).is_null() {
        free(*var as *mut libc::c_void);
        *var = 0 as *mut libc::c_char;
    }
    *var = arg;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_bool_arg(
    mut line: *const libc::c_char,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut p: *const libc::c_char = line.offset((*match_0).rm_so as isize);
    if tolower(*p.offset(0 as libc::c_int as isize) as libc::c_int) == 'y' as i32
        || tolower(*p.offset(1 as libc::c_int as isize) as libc::c_int) == 'n' as i32
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn set_bool_arg(
    mut var: *mut libc::c_uint,
    mut line: *const libc::c_char,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    *var = get_bool_arg(line, match_0) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_long_arg(
    mut line: *const libc::c_char,
    mut match_0: *mut regmatch_t,
) -> libc::c_ulong {
    return strtoul(
        line.offset((*match_0).rm_so as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn set_int_arg(
    mut var: *mut libc::c_uint,
    mut line: *const libc::c_char,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    *var = get_long_arg(line, match_0) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_logfile(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).logf_name,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_pidfile(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).pidpath,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_anonymous(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if arg.is_null() {
        return -(1 as libc::c_int);
    }
    if anonymous_insert(conf, arg) < 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"line %lu: anonymous_insert() failed: '%s'\0" as *const u8
                as *const libc::c_char,
            lineno,
            arg,
        );
        free(arg as *mut libc::c_void);
        arg = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_viaproxyname(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut r: libc::c_int = set_string_arg(
        &mut (*conf).via_proxy_name,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if r != 0 {
        return r;
    }
    log_message(
        6 as libc::c_int,
        b"Setting \"Via\" header to '%s'\0" as *const u8 as *const libc::c_char,
        (*conf).via_proxy_name,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_disableviaheader(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut r: libc::c_int = set_bool_arg(
        &mut (*conf).disable_viaheader,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if r != 0 {
        return r;
    }
    log_message(
        6 as libc::c_int,
        b"Disabling transmission of the \"Via\" header.\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_defaulterrorfile(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).errorpage_undef,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_statfile(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).statpage,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_stathost(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut r: libc::c_int = set_string_arg(
        &mut (*conf).stathost,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if r != 0 {
        return r;
    }
    log_message(
        6 as libc::c_int,
        b"Stathost set to \"%s\"\0" as *const u8 as *const libc::c_char,
        (*conf).stathost,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_xtinyproxy(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_bool_arg(
        &mut (*conf).add_xtinyproxy,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_syslog(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_bool_arg(
        &mut (*conf).syslog,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_bindsame(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut r: libc::c_int = set_bool_arg(
        &mut (*conf).bindsame,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if r != 0 {
        return r;
    }
    log_message(
        6 as libc::c_int,
        b"Binding outgoing connection to incoming IP\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_port(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    set_int_arg(
        &mut (*conf).port,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if (*conf).port > 65535 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Bad port number (%d) supplied for Port.\n\0" as *const u8
                as *const libc::c_char,
            (*conf).port,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_maxclients(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    set_int_arg(
        &mut (*conf).maxclients,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_obsolete(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    fprintf(
        stderr,
        b"WARNING: obsolete config item on line %lu\n\0" as *const u8
            as *const libc::c_char,
        lineno,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_timeout(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_int_arg(
        &mut (*conf).idletimeout,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_connectport(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    add_connect_port_allowed(
        get_long_arg(line, &mut *match_0.offset(2 as libc::c_int as isize))
            as libc::c_int,
        &mut (*conf).connect_ports,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_user(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).user,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_group(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).group,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn warn_invalid_address(
    mut arg: *mut libc::c_char,
    mut lineno: libc::c_ulong,
) {
    log_message(
        4 as libc::c_int,
        b"line %lu: Invalid address %s\0" as *const u8 as *const libc::c_char,
        lineno,
        arg,
    );
}
unsafe extern "C" fn handle_allow(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if insert_acl(arg, ACL_ALLOW, &mut (*conf).access_list) < 0 as libc::c_int {
        warn_invalid_address(arg, lineno);
    }
    free(arg as *mut libc::c_void);
    arg = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_deny(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if insert_acl(arg, ACL_DENY, &mut (*conf).access_list) < 0 as libc::c_int {
        warn_invalid_address(arg, lineno);
    }
    free(arg as *mut libc::c_void);
    arg = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_bind(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if arg.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*conf).bind_addrs).is_null() {
        (*conf)
            .bind_addrs = sblist_new(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
        if ((*conf).bind_addrs).is_null() {
            log_message(
                4 as libc::c_int,
                b"line %lu: Could not create a list of bind addresses.\0" as *const u8
                    as *const libc::c_char,
                lineno,
                b"\0" as *const u8 as *const libc::c_char,
            );
            free(arg as *mut libc::c_void);
            arg = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
    }
    sblist_add(
        (*conf).bind_addrs,
        &mut arg as *mut *mut libc::c_char as *mut libc::c_void,
    );
    log_message(
        6 as libc::c_int,
        b"Added bind address [%s] for outgoing connections.\0" as *const u8
            as *const libc::c_char,
        arg,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_listen(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    if arg.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*conf).listen_addrs).is_null() {
        (*conf)
            .listen_addrs = sblist_new(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
        if ((*conf).listen_addrs).is_null() {
            log_message(
                4 as libc::c_int,
                b"line %lu: Could not create a list of listen addresses.\0" as *const u8
                    as *const libc::c_char,
                lineno,
                b"\0" as *const u8 as *const libc::c_char,
            );
            free(arg as *mut libc::c_void);
            arg = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
    }
    sblist_add(
        (*conf).listen_addrs,
        &mut arg as *mut *mut libc::c_char as *mut libc::c_void,
    );
    log_message(
        6 as libc::c_int,
        b"Added address [%s] to listen addresses.\0" as *const u8 as *const libc::c_char,
        arg,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_errorfile(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut err: libc::c_ulong = get_long_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    let mut page: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(4 as libc::c_int as isize),
    );
    if add_new_errorpage(conf, page, err as libc::c_uint) < 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"line %lu: add_new_errorpage() failed: '%s'\0" as *const u8
                as *const libc::c_char,
            lineno,
            page,
        );
        free(page as *mut libc::c_void);
        page = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_addheader(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut name: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    let mut value: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(3 as libc::c_int as isize),
    );
    let mut header: http_header_t = http_header_t {
        name: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
    };
    if ((*conf).add_headers).is_null() {
        (*conf)
            .add_headers = sblist_new(
            ::std::mem::size_of::<http_header_t>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
    }
    header.name = name;
    header.value = value;
    sblist_add(
        (*conf).add_headers,
        &mut header as *mut http_header_t as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
static mut log_levels: [log_levels_s; 6] = [
    {
        let mut init = log_levels_s {
            string: b"critical\0" as *const u8 as *const libc::c_char,
            level: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = log_levels_s {
            string: b"error\0" as *const u8 as *const libc::c_char,
            level: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = log_levels_s {
            string: b"warning\0" as *const u8 as *const libc::c_char,
            level: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = log_levels_s {
            string: b"notice\0" as *const u8 as *const libc::c_char,
            level: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = log_levels_s {
            string: b"connect\0" as *const u8 as *const libc::c_char,
            level: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = log_levels_s {
            string: b"info\0" as *const u8 as *const libc::c_char,
            level: 6 as libc::c_int,
        };
        init
    },
];
static mut nlevels: libc::c_uint = 0;
unsafe extern "C" fn handle_loglevel(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut arg: *mut libc::c_char = get_string_arg(
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i != nlevels {
        if strcasecmp(arg, log_levels[i as usize].string) == 0 {
            set_log_level(log_levels[i as usize].level);
            free(arg as *mut libc::c_void);
            arg = 0 as *mut libc::c_char;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(arg as *mut libc::c_void);
    arg = 0 as *mut libc::c_char;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn handle_basicauth(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    user = get_string_arg(line, &mut *match_0.offset(2 as libc::c_int as isize));
    if user.is_null() {
        return -(1 as libc::c_int);
    }
    pass = get_string_arg(line, &mut *match_0.offset(3 as libc::c_int as isize));
    if pass.is_null() {
        free(user as *mut libc::c_void);
        user = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    if ((*conf).basicauth_list).is_null() {
        (*conf)
            .basicauth_list = sblist_new(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
    }
    basicauth_add((*conf).basicauth_list, user, pass);
    free(user as *mut libc::c_void);
    user = 0 as *mut libc::c_char;
    free(pass as *mut libc::c_void);
    pass = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn warn_deprecated(
    mut arg: *const libc::c_char,
    mut lineno: libc::c_ulong,
) {
    log_message(
        4 as libc::c_int,
        b"line %lu: deprecated option %s\0" as *const u8 as *const libc::c_char,
        lineno,
        arg,
    );
}
unsafe extern "C" fn handle_filter(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).filter,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_filterurls(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    (*conf).filter_opts
        |= (get_bool_arg(line, &mut *match_0.offset(2 as libc::c_int as isize))
            * FILTER_OPT_URL as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_filterextended(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    warn_deprecated(
        b"FilterExtended, use FilterType\0" as *const u8 as *const libc::c_char,
        lineno,
    );
    (*conf).filter_opts
        |= (get_bool_arg(line, &mut *match_0.offset(2 as libc::c_int as isize))
            * FILTER_OPT_TYPE_ERE as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_filterdefaultdeny(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    (*conf).filter_opts
        |= (get_bool_arg(line, &mut *match_0.offset(2 as libc::c_int as isize))
            * FILTER_OPT_DEFAULT_DENY as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_filtercasesensitive(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    (*conf).filter_opts
        |= (get_bool_arg(line, &mut *match_0.offset(2 as libc::c_int as isize))
            * FILTER_OPT_CASESENSITIVE as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_filtertype(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    static mut ftmap: [C2RustUnnamed_4; 3] = unsafe {
        [
            {
                let mut init = C2RustUnnamed_4 {
                    flag: FILTER_OPT_TYPE_ERE as libc::c_int as libc::c_ushort,
                    type_0: *::std::mem::transmute::<
                        &[u8; 8],
                        &mut [libc::c_char; 8],
                    >(b"ere\0\0\0\0\0"),
                };
                init
            },
            {
                let mut init = C2RustUnnamed_4 {
                    flag: FILTER_OPT_TYPE_BRE as libc::c_int as libc::c_ushort,
                    type_0: *::std::mem::transmute::<
                        &[u8; 8],
                        &mut [libc::c_char; 8],
                    >(b"bre\0\0\0\0\0"),
                };
                init
            },
            {
                let mut init = C2RustUnnamed_4 {
                    flag: FILTER_OPT_TYPE_FNMATCH as libc::c_int as libc::c_ushort,
                    type_0: *::std::mem::transmute::<
                        &[u8; 8],
                        &mut [libc::c_char; 8],
                    >(b"fnmatch\0"),
                };
                init
            },
        ]
    };
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    type_0 = get_string_arg(line, &mut *match_0.offset(2 as libc::c_int as isize));
    if type_0.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_4; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong)
    {
        if strcasecmp((ftmap[i as usize].type_0).as_ptr(), type_0) == 0 {
            (*conf).filter_opts |= ftmap[i as usize].flag as libc::c_uint;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(type_0 as *mut libc::c_void);
    type_0 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_reverseonly(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_bool_arg(
        &mut (*conf).reverseonly,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_reversemagic(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_bool_arg(
        &mut (*conf).reversemagic,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_reversebaseurl(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    return set_string_arg(
        &mut (*conf).reversebaseurl,
        line,
        &mut *match_0.offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn handle_reversepath(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut arg1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg2: *mut libc::c_char = 0 as *mut libc::c_char;
    arg1 = get_string_arg(line, &mut *match_0.offset(2 as libc::c_int as isize));
    if arg1.is_null() {
        return -(1 as libc::c_int);
    }
    if (*match_0.offset(4 as libc::c_int as isize)).rm_so != -(1 as libc::c_int) {
        arg2 = get_string_arg(line, &mut *match_0.offset(4 as libc::c_int as isize));
        if arg2.is_null() {
            free(arg1 as *mut libc::c_void);
            arg1 = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
        reversepath_add(arg1, arg2, &mut (*conf).reversepath_list);
        free(arg1 as *mut libc::c_void);
        arg1 = 0 as *mut libc::c_char;
        free(arg2 as *mut libc::c_void);
        arg2 = 0 as *mut libc::c_char;
    } else {
        reversepath_add(0 as *const libc::c_char, arg1, &mut (*conf).reversepath_list);
        free(arg1 as *mut libc::c_void);
        arg1 = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pt_from_string(mut s: *const libc::c_char) -> proxy_type {
    static mut pt_map: [[libc::c_char; 7]; 4] = unsafe {
        [
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"none\0\0\0"),
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"http\0\0\0"),
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"socks4\0"),
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"socks5\0"),
        ]
    };
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[[libc::c_char; 7]; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
    {
        if strcmp((pt_map[i as usize]).as_ptr(), s) == 0 {
            return i as proxy_type;
        }
        i = i.wrapping_add(1);
        i;
    }
    return PT_NONE;
}
unsafe extern "C" fn handle_upstream(
    mut conf: *mut config_s,
    mut line: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut match_0: *mut regmatch_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    let mut mi: libc::c_int = 0;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pt: proxy_type = PT_NONE;
    let mut ube: upstream_build_error = UBE_SUCCESS;
    if (*match_0.offset(3 as libc::c_int as isize)).rm_so != -(1 as libc::c_int) {
        tmp = get_string_arg(line, &mut *match_0.offset(3 as libc::c_int as isize));
        if strcmp(tmp, b"none\0" as *const u8 as *const libc::c_char) == 0 {
            free(tmp as *mut libc::c_void);
            tmp = 0 as *mut libc::c_char;
            if (*match_0.offset(4 as libc::c_int as isize)).rm_so == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            domain = get_string_arg(
                line,
                &mut *match_0.offset(4 as libc::c_int as isize),
            );
            if domain.is_null() {
                return -(1 as libc::c_int);
            }
            ube = upstream_add(
                0 as *const libc::c_char,
                0 as libc::c_int,
                domain,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                PT_NONE,
                &mut (*conf).upstream_list,
            );
            free(domain as *mut libc::c_void);
            domain = 0 as *mut libc::c_char;
            current_block = 16924917904204750491;
        } else {
            current_block = 13109137661213826276;
        }
    } else {
        current_block = 13109137661213826276;
    }
    match current_block {
        13109137661213826276 => {
            mi = 6 as libc::c_int;
            tmp = get_string_arg(line, &mut *match_0.offset(mi as isize));
            pt = pt_from_string(tmp);
            free(tmp as *mut libc::c_void);
            tmp = 0 as *mut libc::c_char;
            mi += 2 as libc::c_int;
            if (*match_0.offset(mi as isize)).rm_so != -(1 as libc::c_int) {
                user = get_string_arg(line, &mut *match_0.offset(mi as isize));
            }
            mi += 1;
            mi;
            if (*match_0.offset(mi as isize)).rm_so != -(1 as libc::c_int) {
                pass = get_string_arg(line, &mut *match_0.offset(mi as isize));
            }
            mi += 1;
            mi;
            if (*match_0.offset((mi + 4 as libc::c_int) as isize)).rm_so
                != -(1 as libc::c_int)
            {
                ip = get_string_arg(
                    line,
                    &mut *match_0.offset((mi + 4 as libc::c_int) as isize),
                );
            } else {
                ip = get_string_arg(line, &mut *match_0.offset(mi as isize));
            }
            if ip.is_null() {
                return -(1 as libc::c_int);
            }
            mi += 16 as libc::c_int;
            port = get_long_arg(line, &mut *match_0.offset(mi as isize)) as libc::c_int;
            mi += 3 as libc::c_int;
            if (*match_0.offset(mi as isize)).rm_so != -(1 as libc::c_int) {
                domain = get_string_arg(line, &mut *match_0.offset(mi as isize));
            }
            ube = upstream_add(
                ip,
                port,
                domain,
                user,
                pass,
                pt,
                &mut (*conf).upstream_list,
            );
            free(user as *mut libc::c_void);
            user = 0 as *mut libc::c_char;
            free(pass as *mut libc::c_void);
            pass = 0 as *mut libc::c_char;
            free(domain as *mut libc::c_void);
            domain = 0 as *mut libc::c_char;
            free(ip as *mut libc::c_void);
            ip = 0 as *mut libc::c_char;
        }
        _ => {}
    }
    if ube as libc::c_uint != UBE_SUCCESS as libc::c_int as libc::c_uint {
        log_message(
            4 as libc::c_int,
            b"line %lu: %s\0" as *const u8 as *const libc::c_char,
            lineno,
            upstream_build_error_string(ube),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    ndirectives = (::std::mem::size_of::<[C2RustUnnamed_3; 41]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong)
        as libc::c_uint;
    nlevels = (::std::mem::size_of::<[log_levels_s; 6]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<log_levels_s>() as libc::c_ulong)
        as libc::c_uint;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

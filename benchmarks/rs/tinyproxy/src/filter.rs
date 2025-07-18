use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    pub type re_dfa_t;
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
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    static mut config: *mut config_s;
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
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn sblist_free(l: *mut sblist);
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_list_t = *mut sblist;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
pub type C2RustUnnamed_0 = libc::c_int;
pub const _REG_ERPAREN: C2RustUnnamed_0 = 16;
pub const _REG_ESIZE: C2RustUnnamed_0 = 15;
pub const _REG_EEND: C2RustUnnamed_0 = 14;
pub const _REG_BADRPT: C2RustUnnamed_0 = 13;
pub const _REG_ESPACE: C2RustUnnamed_0 = 12;
pub const _REG_ERANGE: C2RustUnnamed_0 = 11;
pub const _REG_BADBR: C2RustUnnamed_0 = 10;
pub const _REG_EBRACE: C2RustUnnamed_0 = 9;
pub const _REG_EPAREN: C2RustUnnamed_0 = 8;
pub const _REG_EBRACK: C2RustUnnamed_0 = 7;
pub const _REG_ESUBREG: C2RustUnnamed_0 = 6;
pub const _REG_EESCAPE: C2RustUnnamed_0 = 5;
pub const _REG_ECTYPE: C2RustUnnamed_0 = 4;
pub const _REG_ECOLLATE: C2RustUnnamed_0 = 3;
pub const _REG_BADPAT: C2RustUnnamed_0 = 2;
pub const _REG_NOMATCH: C2RustUnnamed_0 = 1;
pub const _REG_NOERROR: C2RustUnnamed_0 = 0;
pub const _REG_ENOSYS: C2RustUnnamed_0 = -1;
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
pub type filter_options = libc::c_uint;
pub const FILTER_OPT_TYPE_FNMATCH: filter_options = 1024;
pub const FILTER_OPT_TYPE_ERE: filter_options = 512;
pub const FILTER_OPT_TYPE_BRE: filter_options = 256;
pub const FILTER_OPT_DEFAULT_DENY: filter_options = 4;
pub const FILTER_OPT_URL: filter_options = 2;
pub const FILTER_OPT_CASESENSITIVE: filter_options = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_list {
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub cpatb: regex_t,
    pub pattern: *mut libc::c_char,
}
static mut err: libc::c_int = 0;
static mut fl: *mut sblist = 0 as *const sblist as *mut sblist;
static mut already_init: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn filter_init() {
    let mut current_block: u64;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut fe: filter_list = filter_list {
        u: C2RustUnnamed_1 {
            cpatb: regex_t {
                buffer: 0 as *mut re_dfa_t,
                allocated: 0,
                used: 0,
                syntax: 0,
                fastmap: 0 as *mut libc::c_char,
                translate: 0 as *mut libc::c_uchar,
                re_nsub: 0,
                can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
                c2rust_padding: [0; 7],
            },
        },
    };
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cflags: libc::c_int = 0;
    let mut lineno: libc::c_int = 0 as libc::c_int;
    if !fl.is_null() || already_init != 0 {
        return;
    }
    fd = fopen((*config).filter, b"r\0" as *const u8 as *const libc::c_char);
    if fd.is_null() {
        perror(b"filter file\0" as *const u8 as *const libc::c_char);
        exit(65 as libc::c_int);
    }
    cflags = (1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int;
    cflags
        |= 1 as libc::c_int
            * ((*config).filter_opts & FILTER_OPT_TYPE_ERE as libc::c_int as libc::c_uint
                != 0) as libc::c_int;
    cflags
        |= ((1 as libc::c_int) << 1 as libc::c_int)
            * ((*config).filter_opts
                & FILTER_OPT_CASESENSITIVE as libc::c_int as libc::c_uint == 0)
                as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 512 as libc::c_int, fd)).is_null() {
        lineno += 1;
        lineno;
        s = buf.as_mut_ptr();
        while *s as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            s = s.offset(1);
            s;
        }
        start = s;
        while *s != 0 {
            if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                break;
            }
            if *s as libc::c_int == '#' as i32 {
                if s == buf.as_mut_ptr()
                    || *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        != '\\' as i32
                {
                    break;
                }
            }
            s = s.offset(1);
            s;
        }
        *s = '\0' as i32 as libc::c_char;
        s = start;
        if *s as libc::c_int == '\0' as i32 {
            continue;
        }
        if fl.is_null() {
            fl = sblist_new(
                ::std::mem::size_of::<filter_list>() as libc::c_ulong,
                (4096 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<filter_list>() as libc::c_ulong),
            );
        }
        if (*config).filter_opts & FILTER_OPT_TYPE_FNMATCH as libc::c_int as libc::c_uint
            != 0
        {
            fe.u.pattern = strdup(s);
            if (fe.u.pattern).is_null() {
                current_block = 2891135413264362348;
            } else {
                current_block = 17788412896529399552;
            }
        } else {
            err = regcomp(&mut fe.u.cpatb, s, cflags);
            if err != 0 as libc::c_int {
                if err == _REG_ESPACE as libc::c_int {
                    current_block = 2891135413264362348;
                } else {
                    fprintf(
                        stderr,
                        b"Bad regex in %s: line %d - %s\n\0" as *const u8
                            as *const libc::c_char,
                        (*config).filter,
                        lineno,
                        s,
                    );
                    exit(65 as libc::c_int);
                }
            } else {
                current_block = 17788412896529399552;
            }
        }
        match current_block {
            17788412896529399552 => {
                if !(sblist_add(fl, &mut fe as *mut filter_list as *mut libc::c_void)
                    == 0)
                {
                    continue;
                }
            }
            _ => {}
        }
        fprintf(
            stderr,
            b"out of memory parsing filter file %s: line %d\n\0" as *const u8
                as *const libc::c_char,
            (*config).filter,
            lineno,
        );
        exit(65 as libc::c_int);
    }
    if ferror(fd) != 0 {
        perror(b"fgets\0" as *const u8 as *const libc::c_char);
        exit(65 as libc::c_int);
    }
    fclose(fd);
    already_init = 1 as libc::c_int;
}
pub unsafe extern "C" fn filter_destroy() {
    let mut p: *mut filter_list = 0 as *mut filter_list;
    let mut i: size_t = 0;
    if already_init != 0 {
        if !fl.is_null() {
            i = 0 as libc::c_int as size_t;
            while i < (*fl).count {
                p = sblist_get(fl, i) as *mut filter_list;
                if (*config).filter_opts
                    & FILTER_OPT_TYPE_FNMATCH as libc::c_int as libc::c_uint != 0
                {
                    free((*p).u.pattern as *mut libc::c_void);
                    (*p).u.pattern = 0 as *mut libc::c_char;
                } else {
                    regfree(&mut (*p).u.cpatb);
                }
                i = i.wrapping_add(1);
                i;
            }
            sblist_free(fl);
        }
        fl = 0 as *mut sblist;
        already_init = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn filter_reload() {
    if !((*config).filter).is_null() {
        log_message(
            5 as libc::c_int,
            b"Re-reading filter file.\0" as *const u8 as *const libc::c_char,
        );
        filter_destroy();
        filter_init();
    }
}
pub unsafe extern "C" fn filter_run(mut str: *const libc::c_char) -> libc::c_int {
    let mut p: *mut filter_list = 0 as *mut filter_list;
    let mut i: size_t = 0;
    let mut result: libc::c_int = 0;
    if !(fl.is_null() || already_init == 0) {
        i = 0 as libc::c_int as size_t;
        while i < (*fl).count {
            p = sblist_get(fl, i) as *mut filter_list;
            if (*config).filter_opts
                & FILTER_OPT_TYPE_FNMATCH as libc::c_int as libc::c_uint != 0
            {
                result = fnmatch((*p).u.pattern, str, 0 as libc::c_int);
            } else {
                result = regexec(
                    &mut (*p).u.cpatb,
                    str,
                    0 as libc::c_int as size_t,
                    0 as *mut regmatch_t,
                    0 as libc::c_int,
                );
            }
            if result == 0 as libc::c_int {
                if (*config).filter_opts
                    & FILTER_OPT_DEFAULT_DENY as libc::c_int as libc::c_uint == 0
                {
                    return 1 as libc::c_int
                } else {
                    return 0 as libc::c_int
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if (*config).filter_opts & FILTER_OPT_DEFAULT_DENY as libc::c_int as libc::c_uint
        == 0
    {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut program_invocation_short_name: *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn flockfile(__stream: *mut FILE);
    fn funlockfile(__stream: *mut FILE);
    static mut argp_program_bug_address: *const libc::c_char;
    static mut argp_err_exit_status: error_t;
    fn _argp_input(__argp: *const argp, __state: *const argp_state) -> *mut libc::c_void;
    fn _argp_fmtstream_update(__fs: argp_fmtstream_t);
    fn argp_make_fmtstream(
        __stream: *mut FILE,
        __lmargin: size_t,
        __rmargin: size_t,
        __wmargin: ssize_t,
    ) -> argp_fmtstream_t;
    fn argp_fmtstream_free(__fs: argp_fmtstream_t);
    fn argp_fmtstream_printf(
        __fs: argp_fmtstream_t,
        __fmt: *const libc::c_char,
        _: ...
    ) -> ssize_t;
    fn _argp_fmtstream_ensure(__fs: argp_fmtstream_t, __amount: size_t) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type error_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type va_list = __builtin_va_list;
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
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const libc::c_char,
    pub doc: *const libc::c_char,
    pub children: *const argp_child,
    pub help_filter: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> *mut libc::c_char,
    >,
    pub argp_domain: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: libc::c_int,
    pub header: *const libc::c_char,
    pub group: libc::c_int,
}
pub type argp_parser_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub next: libc::c_int,
    pub flags: libc::c_uint,
    pub arg_num: libc::c_uint,
    pub quoted: libc::c_int,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
pub type argp_fmtstream_t = *mut argp_fmtstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_fmtstream {
    pub stream: *mut FILE,
    pub lmargin: size_t,
    pub rmargin: size_t,
    pub wmargin: ssize_t,
    pub point_offs: size_t,
    pub point_col: ssize_t,
    pub buf: *mut libc::c_char,
    pub p: *mut libc::c_char,
    pub end: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol {
    pub entries: *mut hol_entry,
    pub num_entries: libc::c_uint,
    pub short_options: *mut libc::c_char,
    pub clusters: *mut hol_cluster,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol_cluster {
    pub header: *const libc::c_char,
    pub index: libc::c_int,
    pub group: libc::c_int,
    pub parent: *mut hol_cluster,
    pub argp: *const argp,
    pub depth: libc::c_int,
    pub next: *mut hol_cluster,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol_entry {
    pub opt: *const argp_option,
    pub num: libc::c_uint,
    pub short_options: *mut libc::c_char,
    pub group: libc::c_int,
    pub cluster: *mut hol_cluster,
    pub argp: *const argp,
    pub ord: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uparams {
    pub dup_args: libc::c_int,
    pub dup_args_note: libc::c_int,
    pub short_opt_col: libc::c_int,
    pub long_opt_col: libc::c_int,
    pub doc_opt_col: libc::c_int,
    pub opt_doc_col: libc::c_int,
    pub header_col: libc::c_int,
    pub usage_indent: libc::c_int,
    pub rmargin: libc::c_int,
    pub valid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol_help_state {
    pub prev_entry: *mut hol_entry,
    pub sep_groups: libc::c_int,
    pub suppressed_dup_arg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pentry_state {
    pub entry: *const hol_entry,
    pub stream: argp_fmtstream_t,
    pub hhstate: *mut hol_help_state,
    pub first: libc::c_int,
    pub state: *const argp_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uparam_name {
    pub name: [libc::c_char; 14],
    pub is_bool: bool,
    pub uparams_offs: libc::c_uchar,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn _option_is_end(mut __opt: *const argp_option) -> libc::c_int {
    return ((*__opt).key == 0 && ((*__opt).name).is_null() && ((*__opt).doc).is_null()
        && (*__opt).group == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _option_is_short(mut __opt: *const argp_option) -> libc::c_int {
    if (*__opt).flags & 0x8 as libc::c_int != 0 {
        return 0 as libc::c_int
    } else {
        let mut __key: libc::c_int = (*__opt).key;
        return (__key > 0 as libc::c_int
            && __key <= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            && *(*__ctype_b_loc()).offset(__key as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0)
            as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_write(
    mut __fs: argp_fmtstream_t,
    mut __str: *const libc::c_char,
    mut __len: size_t,
) -> size_t {
    if ((*__fs).p).offset(__len as isize) <= (*__fs).end
        || _argp_fmtstream_ensure(__fs, __len) != 0
    {
        memcpy((*__fs).p as *mut libc::c_void, __str as *const libc::c_void, __len);
        (*__fs).p = ((*__fs).p).offset(__len as isize);
        return __len;
    } else {
        return 0 as libc::c_int as size_t
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_puts(
    mut __fs: argp_fmtstream_t,
    mut __str: *const libc::c_char,
) -> libc::c_int {
    let mut __len: size_t = strlen(__str);
    if __len != 0 {
        let mut __wrote: size_t = argp_fmtstream_write(__fs, __str, __len);
        return if __wrote == __len { 0 as libc::c_int } else { -(1 as libc::c_int) };
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_putc(
    mut __fs: argp_fmtstream_t,
    mut __ch: libc::c_int,
) -> libc::c_int {
    if (*__fs).p < (*__fs).end
        || _argp_fmtstream_ensure(__fs, 1 as libc::c_int as size_t) != 0
    {
        let fresh1 = (*__fs).p;
        (*__fs).p = ((*__fs).p).offset(1);
        *fresh1 = __ch as libc::c_char;
        return *fresh1 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_set_lmargin(
    mut __fs: argp_fmtstream_t,
    mut __lmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).lmargin;
    (*__fs).lmargin = __lmargin;
    return __old;
}
#[inline]
unsafe extern "C" fn argp_fmtstream_set_wmargin(
    mut __fs: argp_fmtstream_t,
    mut __wmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).wmargin as size_t;
    (*__fs).wmargin = __wmargin as ssize_t;
    return __old;
}
#[inline]
unsafe extern "C" fn argp_fmtstream_point(mut __fs: argp_fmtstream_t) -> size_t {
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    return (if (*__fs).point_col >= 0 as libc::c_int as libc::c_long {
        (*__fs).point_col
    } else {
        0 as libc::c_int as libc::c_long
    }) as size_t;
}
static mut uparams: uparams = {
    let mut init = uparams {
        dup_args: 0 as libc::c_int,
        dup_args_note: 1 as libc::c_int,
        short_opt_col: 2 as libc::c_int,
        long_opt_col: 6 as libc::c_int,
        doc_opt_col: 2 as libc::c_int,
        opt_doc_col: 29 as libc::c_int,
        header_col: 1 as libc::c_int,
        usage_indent: 12 as libc::c_int,
        rmargin: 79 as libc::c_int,
        valid: 0,
    };
    init
};
static mut uparam_names: [uparam_name; 9] = unsafe {
    [
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"dup-args\0\0\0\0\0\0"),
                is_bool: 1 as libc::c_int != 0,
                uparams_offs: 0 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"dup-args-note\0"),
                is_bool: 1 as libc::c_int != 0,
                uparams_offs: 4 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"short-opt-col\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 8 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"long-opt-col\0\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 12 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"doc-opt-col\0\0\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 16 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"opt-doc-col\0\0\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 20 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"header-col\0\0\0\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 24 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"usage-indent\0\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 28 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"rmargin\0\0\0\0\0\0\0"),
                is_bool: 0 as libc::c_int != 0,
                uparams_offs: 32 as libc::c_ulong as libc::c_uchar,
            };
            init
        },
    ]
};
unsafe extern "C" fn validate_uparams(
    mut state: *const argp_state,
    mut upptr: *mut uparams,
) {
    let mut up: *const uparam_name = 0 as *const uparam_name;
    up = uparam_names.as_ptr();
    while up
        < uparam_names
            .as_ptr()
            .offset(
                (::std::mem::size_of::<[uparam_name; 9]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<uparam_name>() as libc::c_ulong)
                    as isize,
            )
    {
        if !((*up).is_bool as libc::c_int != 0
            || (*up).uparams_offs as libc::c_ulong == 32 as libc::c_ulong)
        {
            if *((upptr as *mut libc::c_char)
                .offset((*up).uparams_offs as libc::c_int as isize) as *mut libc::c_int)
                >= (*upptr).rmargin
            {
                argp_failure(
                    state,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        (*(*state).root_argp).argp_domain,
                        b"ARGP_HELP_FMT: %s value is less than or equal to %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"rmargin\0" as *const u8 as *const libc::c_char,
                    ((*up).name).as_ptr(),
                );
                return;
            }
        }
        up = up.offset(1);
        up;
    }
    uparams = *upptr;
    uparams.valid = 1 as libc::c_int;
}
unsafe extern "C" fn fill_in_uparams(mut state: *const argp_state) {
    let mut var: *const libc::c_char = getenv(
        b"ARGP_HELP_FMT\0" as *const u8 as *const libc::c_char,
    );
    let mut new_params: uparams = uparams;
    if !var.is_null() {
        while *var != 0 {
            while *(*__ctype_b_loc())
                .offset(*var as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                var = var.offset(1);
                var;
            }
            if *(*__ctype_b_loc()).offset(*var as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                let mut var_len: size_t = 0;
                let mut un: *const uparam_name = 0 as *const uparam_name;
                let mut unspec: libc::c_int = 0 as libc::c_int;
                let mut val: libc::c_int = 0 as libc::c_int;
                let mut arg_0: *const libc::c_char = var;
                while *(*__ctype_b_loc())
                    .offset(*arg_0 as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *arg_0 as libc::c_int == '-' as i32
                    || *arg_0 as libc::c_int == '_' as i32
                {
                    arg_0 = arg_0.offset(1);
                    arg_0;
                }
                var_len = arg_0.offset_from(var) as libc::c_long as size_t;
                while *(*__ctype_b_loc())
                    .offset(*arg_0 as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    arg_0 = arg_0.offset(1);
                    arg_0;
                }
                if *arg_0 as libc::c_int == '\0' as i32
                    || *arg_0 as libc::c_int == ',' as i32
                {
                    unspec = 1 as libc::c_int;
                } else if *arg_0 as libc::c_int == '=' as i32 {
                    arg_0 = arg_0.offset(1);
                    arg_0;
                    while *(*__ctype_b_loc())
                        .offset(*arg_0 as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        arg_0 = arg_0.offset(1);
                        arg_0;
                    }
                }
                if unspec != 0 {
                    if *var.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'n' as i32
                        && *var.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'o' as i32
                        && *var.offset(2 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                    {
                        val = 0 as libc::c_int;
                        var = var.offset(3 as libc::c_int as isize);
                        var_len = (var_len as libc::c_ulong)
                            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    } else {
                        val = 1 as libc::c_int;
                    }
                } else if *(*__ctype_b_loc())
                    .offset(*arg_0 as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    val = atoi(arg_0);
                    while *(*__ctype_b_loc())
                        .offset(*arg_0 as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        arg_0 = arg_0.offset(1);
                        arg_0;
                    }
                    while *(*__ctype_b_loc())
                        .offset(*arg_0 as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        arg_0 = arg_0.offset(1);
                        arg_0;
                    }
                }
                un = uparam_names.as_ptr();
                while un
                    < uparam_names
                        .as_ptr()
                        .offset(
                            (::std::mem::size_of::<[uparam_name; 9]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uparam_name>() as libc::c_ulong,
                                ) as isize,
                        )
                {
                    if strlen(((*un).name).as_ptr()) == var_len
                        && strncmp(var, ((*un).name).as_ptr(), var_len)
                            == 0 as libc::c_int
                    {
                        if unspec != 0 && !(*un).is_bool {
                            argp_failure(
                                state,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    if state.is_null() {
                                        0 as *const libc::c_char
                                    } else {
                                        (*(*state).root_argp).argp_domain
                                    },
                                    b"%.*s: ARGP_HELP_FMT parameter requires a value\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                var_len as libc::c_int,
                                var,
                            );
                        } else {
                            *((&mut new_params as *mut uparams as *mut libc::c_char)
                                .offset((*un).uparams_offs as libc::c_int as isize)
                                as *mut libc::c_int) = val;
                        }
                        break;
                    } else {
                        un = un.offset(1);
                        un;
                    }
                }
                if un
                    == uparam_names
                        .as_ptr()
                        .offset(
                            (::std::mem::size_of::<[uparam_name; 9]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uparam_name>() as libc::c_ulong,
                                ) as isize,
                        )
                {
                    argp_failure(
                        state,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            if state.is_null() {
                                0 as *const libc::c_char
                            } else {
                                (*(*state).root_argp).argp_domain
                            },
                            b"%.*s: Unknown ARGP_HELP_FMT parameter\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        var_len as libc::c_int,
                        var,
                    );
                }
                var = arg_0;
                if *var as libc::c_int == ',' as i32 {
                    var = var.offset(1);
                    var;
                }
            } else {
                if !(*var != 0) {
                    continue;
                }
                argp_failure(
                    state,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        if state.is_null() {
                            0 as *const libc::c_char
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        b"Garbage in ARGP_HELP_FMT: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    var,
                );
                break;
            }
        }
        validate_uparams(state, &mut new_params);
    }
}
unsafe extern "C" fn find_char(
    mut ch: libc::c_char,
    mut beg: *mut libc::c_char,
    mut end: *mut libc::c_char,
) -> libc::c_int {
    while beg < end {
        if *beg as libc::c_int == ch as libc::c_int {
            return 1 as libc::c_int
        } else {
            beg = beg.offset(1);
            beg;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_hol(
    mut argp: *const argp,
    mut cluster: *mut hol_cluster,
) -> *mut hol {
    let mut so: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *const argp_option = 0 as *const argp_option;
    let mut opts: *const argp_option = (*argp).options;
    let mut entry: *mut hol_entry = 0 as *mut hol_entry;
    let mut num_short_options: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hol: *mut hol = malloc(::std::mem::size_of::<hol>() as libc::c_ulong)
        as *mut hol;
    if !hol.is_null() {} else {
        __assert_fail(
            b"hol\0" as *const u8 as *const libc::c_char,
            b"argp-help.c\0" as *const u8 as *const libc::c_char,
            449 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                .as_ptr(),
        );
    }
    'c_8938: {
        if !hol.is_null() {} else {
            __assert_fail(
                b"hol\0" as *const u8 as *const libc::c_char,
                b"argp-help.c\0" as *const u8 as *const libc::c_char,
                449 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*hol).num_entries = 0 as libc::c_int as libc::c_uint;
    (*hol).clusters = 0 as *mut hol_cluster;
    if !opts.is_null() {
        let mut cur_group: libc::c_int = 0 as libc::c_int;
        if (*opts).flags & 0x4 as libc::c_int == 0 {} else {
            __assert_fail(
                b"! oalias (opts)\0" as *const u8 as *const libc::c_char,
                b"argp-help.c\0" as *const u8 as *const libc::c_char,
                459 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                    .as_ptr(),
            );
        }
        'c_8878: {
            if (*opts).flags & 0x4 as libc::c_int == 0 {} else {
                __assert_fail(
                    b"! oalias (opts)\0" as *const u8 as *const libc::c_char,
                    b"argp-help.c\0" as *const u8 as *const libc::c_char,
                    459 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 64],
                        &[libc::c_char; 64],
                    >(
                        b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        o = opts;
        while _option_is_end(o) == 0 {
            if (*o).flags & 0x4 as libc::c_int == 0 {
                (*hol).num_entries = ((*hol).num_entries).wrapping_add(1);
                (*hol).num_entries;
            }
            if _option_is_short(o) != 0 {
                num_short_options = num_short_options.wrapping_add(1);
                num_short_options;
            }
            o = o.offset(1);
            o;
        }
        (*hol)
            .entries = malloc(
            (::std::mem::size_of::<hol_entry>() as libc::c_ulong)
                .wrapping_mul((*hol).num_entries as libc::c_ulong),
        ) as *mut hol_entry;
        (*hol)
            .short_options = malloc(
            num_short_options.wrapping_add(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
        ) as *mut libc::c_char;
        if !((*hol).entries).is_null() && !((*hol).short_options).is_null() {} else {
            __assert_fail(
                b"hol->entries && hol->short_options\0" as *const u8
                    as *const libc::c_char,
                b"argp-help.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                    .as_ptr(),
            );
        }
        'c_8761: {
            if !((*hol).entries).is_null() && !((*hol).short_options).is_null() {} else {
                __assert_fail(
                    b"hol->entries && hol->short_options\0" as *const u8
                        as *const libc::c_char,
                    b"argp-help.c\0" as *const u8 as *const libc::c_char,
                    473 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 64],
                        &[libc::c_char; 64],
                    >(
                        b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if -(1 as libc::c_int) as size_t
            <= (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong
        {
            if (*hol).num_entries as libc::c_ulong
                <= (-(1 as libc::c_int) as size_t)
                    .wrapping_div(::std::mem::size_of::<hol_entry>() as libc::c_ulong)
            {} else {
                __assert_fail(
                    b"hol->num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                        as *const u8 as *const libc::c_char,
                    b"argp-help.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 64],
                        &[libc::c_char; 64],
                    >(
                        b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_8689: {
                if (*hol).num_entries as libc::c_ulong
                    <= (-(1 as libc::c_int) as size_t)
                        .wrapping_div(
                            ::std::mem::size_of::<hol_entry>() as libc::c_ulong,
                        )
                {} else {
                    __assert_fail(
                        b"hol->num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                            as *const u8 as *const libc::c_char,
                        b"argp-help.c\0" as *const u8 as *const libc::c_char,
                        475 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 64],
                            &[libc::c_char; 64],
                        >(
                            b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
        so = (*hol).short_options;
        o = opts;
        entry = (*hol).entries;
        while _option_is_end(o) == 0 {
            (*entry).opt = o;
            (*entry).num = 0 as libc::c_int as libc::c_uint;
            (*entry).short_options = so;
            cur_group = if (*o).group != 0 {
                (*o).group
            } else if ((*o).name).is_null() && (*o).key == 0 {
                cur_group + 1 as libc::c_int
            } else {
                cur_group
            };
            (*entry).group = cur_group;
            (*entry).cluster = cluster;
            (*entry).argp = argp;
            loop {
                (*entry).num = ((*entry).num).wrapping_add(1);
                (*entry).num;
                if _option_is_short(o) != 0
                    && find_char((*o).key as libc::c_char, (*hol).short_options, so) == 0
                {
                    let fresh2 = so;
                    so = so.offset(1);
                    *fresh2 = (*o).key as libc::c_char;
                }
                o = o.offset(1);
                o;
                if !(_option_is_end(o) == 0 && (*o).flags & 0x4 as libc::c_int != 0) {
                    break;
                }
            }
            entry = entry.offset(1);
            entry;
        }
        *so = '\0' as i32 as libc::c_char;
    }
    return hol;
}
unsafe extern "C" fn hol_add_cluster(
    mut hol: *mut hol,
    mut group: libc::c_int,
    mut header: *const libc::c_char,
    mut index: libc::c_int,
    mut parent: *mut hol_cluster,
    mut argp: *const argp,
) -> *mut hol_cluster {
    let mut cl: *mut hol_cluster = malloc(
        ::std::mem::size_of::<hol_cluster>() as libc::c_ulong,
    ) as *mut hol_cluster;
    if !cl.is_null() {
        (*cl).group = group;
        (*cl).header = header;
        (*cl).index = index;
        (*cl).parent = parent;
        (*cl).argp = argp;
        (*cl)
            .depth = if !parent.is_null() {
            (*parent).depth + 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        (*cl).next = (*hol).clusters;
        (*hol).clusters = cl;
    }
    return cl;
}
unsafe extern "C" fn hol_free(mut hol: *mut hol) {
    let mut cl: *mut hol_cluster = (*hol).clusters;
    while !cl.is_null() {
        let mut next: *mut hol_cluster = (*cl).next;
        rpl_free(cl as *mut libc::c_void);
        cl = next;
    }
    if (*hol).num_entries > 0 as libc::c_int as libc::c_uint {
        rpl_free((*hol).entries as *mut libc::c_void);
        rpl_free((*hol).short_options as *mut libc::c_void);
    }
    rpl_free(hol as *mut libc::c_void);
}
unsafe extern "C" fn hol_entry_short_iterate(
    mut entry: *const hol_entry,
    mut func: Option::<
        unsafe extern "C" fn(
            *const argp_option,
            *const argp_option,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut domain: *const libc::c_char,
    mut cookie: *mut libc::c_void,
) -> libc::c_int {
    let mut nopts: libc::c_uint = 0;
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut real: *const argp_option = (*entry).opt;
    let mut so: *mut libc::c_char = (*entry).short_options;
    opt = real;
    nopts = (*entry).num;
    while nopts > 0 as libc::c_int as libc::c_uint && val == 0 {
        if _option_is_short(opt) != 0 && *so as libc::c_int == (*opt).key {
            if (*opt).flags & 0x4 as libc::c_int == 0 {
                real = opt;
            }
            if (*opt).flags & 0x2 as libc::c_int == 0 {
                val = (Some(func.unwrap())).unwrap()(opt, real, domain, cookie);
            }
            so = so.offset(1);
            so;
        }
        opt = opt.offset(1);
        opt;
        nopts = nopts.wrapping_sub(1);
        nopts;
    }
    return val;
}
#[inline(always)]
unsafe extern "C" fn hol_entry_long_iterate(
    mut entry: *const hol_entry,
    mut func: Option::<
        unsafe extern "C" fn(
            *const argp_option,
            *const argp_option,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut domain: *const libc::c_char,
    mut cookie: *mut libc::c_void,
) -> libc::c_int {
    let mut nopts: libc::c_uint = 0;
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut real: *const argp_option = (*entry).opt;
    opt = real;
    nopts = (*entry).num;
    while nopts > 0 as libc::c_int as libc::c_uint && val == 0 {
        if !((*opt).name).is_null() {
            if (*opt).flags & 0x4 as libc::c_int == 0 {
                real = opt;
            }
            if (*opt).flags & 0x2 as libc::c_int == 0 {
                val = (Some(func.unwrap())).unwrap()(opt, real, domain, cookie);
            }
        }
        opt = opt.offset(1);
        opt;
        nopts = nopts.wrapping_sub(1);
        nopts;
    }
    return val;
}
unsafe extern "C" fn until_short(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const libc::c_char,
    mut cookie: *mut libc::c_void,
) -> libc::c_int {
    return if _option_is_short(opt) != 0 { (*opt).key } else { 0 as libc::c_int };
}
unsafe extern "C" fn hol_entry_first_short(mut entry: *const hol_entry) -> libc::c_char {
    return hol_entry_short_iterate(
        entry,
        Some(
            until_short
                as unsafe extern "C" fn(
                    *const argp_option,
                    *const argp_option,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        (*(*entry).argp).argp_domain,
        0 as *mut libc::c_void,
    ) as libc::c_char;
}
unsafe extern "C" fn hol_entry_first_long(
    mut entry: *const hol_entry,
) -> *const libc::c_char {
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut num: libc::c_uint = 0;
    opt = (*entry).opt;
    num = (*entry).num;
    while num > 0 as libc::c_int as libc::c_uint {
        if !((*opt).name).is_null() && (*opt).flags & 0x2 as libc::c_int == 0 {
            return (*opt).name;
        }
        opt = opt.offset(1);
        opt;
        num = num.wrapping_sub(1);
        num;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn hol_find_entry(
    mut hol: *mut hol,
    mut name: *const libc::c_char,
) -> *mut hol_entry {
    let mut entry: *mut hol_entry = (*hol).entries;
    let mut num_entries: libc::c_uint = (*hol).num_entries;
    loop {
        let fresh3 = num_entries;
        num_entries = num_entries.wrapping_sub(1);
        if !(fresh3 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        let mut opt: *const argp_option = (*entry).opt;
        let mut num_opts: libc::c_uint = (*entry).num;
        loop {
            let fresh4 = num_opts;
            num_opts = num_opts.wrapping_sub(1);
            if !(fresh4 > 0 as libc::c_int as libc::c_uint) {
                break;
            }
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as libc::c_int == 0
                && strcmp((*opt).name, name) == 0 as libc::c_int
            {
                return entry
            } else {
                opt = opt.offset(1);
                opt;
            }
        }
        entry = entry.offset(1);
        entry;
    }
    return 0 as *mut hol_entry;
}
unsafe extern "C" fn hol_set_group(
    mut hol: *mut hol,
    mut name: *const libc::c_char,
    mut group: libc::c_int,
) {
    let mut entry: *mut hol_entry = hol_find_entry(hol, name);
    if !entry.is_null() {
        (*entry).group = group;
    }
}
unsafe extern "C" fn group_cmp(
    mut group1: libc::c_int,
    mut group2: libc::c_int,
) -> libc::c_int {
    if group1 < 0 as libc::c_int && group2 < 0 as libc::c_int
        || group1 >= 0 as libc::c_int && group2 >= 0 as libc::c_int
    {
        return group1 - group2
    } else {
        return group2 - group1
    };
}
unsafe extern "C" fn hol_sibling_cluster_cmp(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> libc::c_int {
    let mut cmp: libc::c_int = group_cmp((*cl1).group, (*cl2).group);
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    return (*cl2).index - (*cl1).index;
}
unsafe extern "C" fn hol_cousin_cluster_cmp(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> libc::c_int {
    if (*cl1).parent == (*cl2).parent {
        return hol_sibling_cluster_cmp(cl1, cl2)
    } else {
        let mut cmp: libc::c_int = hol_cousin_cluster_cmp((*cl1).parent, (*cl2).parent);
        if cmp != 0 as libc::c_int {
            return cmp;
        }
        cmp = group_cmp((*cl1).group, (*cl2).group);
        if cmp != 0 as libc::c_int {
            return cmp;
        }
        return (*cl2).index - (*cl1).index;
    };
}
unsafe extern "C" fn hol_cluster_cmp(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> libc::c_int {
    if (*cl1).depth > (*cl2).depth {
        loop {
            cl1 = (*cl1).parent;
            if !((*cl1).depth > (*cl2).depth) {
                break;
            }
        }
        let mut cmp: libc::c_int = hol_cousin_cluster_cmp(cl1, cl2);
        if cmp != 0 as libc::c_int {
            return cmp;
        }
        return 1 as libc::c_int;
    } else if (*cl1).depth < (*cl2).depth {
        loop {
            cl2 = (*cl2).parent;
            if !((*cl1).depth < (*cl2).depth) {
                break;
            }
        }
        let mut cmp_0: libc::c_int = hol_cousin_cluster_cmp(cl1, cl2);
        if cmp_0 != 0 as libc::c_int {
            return cmp_0;
        }
        return -(1 as libc::c_int);
    } else {
        return hol_cousin_cluster_cmp(cl1, cl2)
    };
}
unsafe extern "C" fn hol_cluster_base(mut cl: *mut hol_cluster) -> *mut hol_cluster {
    while !((*cl).parent).is_null() {
        cl = (*cl).parent;
    }
    return cl;
}
unsafe extern "C" fn canon_doc_option(
    mut name: *mut *const libc::c_char,
) -> libc::c_int {
    let mut non_opt: libc::c_int = 0;
    while *(*__ctype_b_loc()).offset(**name as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *name = (*name).offset(1);
        *name;
    }
    non_opt = (**name as libc::c_int != '-' as i32) as libc::c_int;
    while **name as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(**name as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        *name = (*name).offset(1);
        *name;
    }
    return non_opt;
}
unsafe extern "C" fn hol_entry_cmp(
    mut entry1: *const hol_entry,
    mut entry2: *const hol_entry,
) -> libc::c_int {
    let mut group1: libc::c_int = if !((*entry1).cluster).is_null() {
        (*hol_cluster_base((*entry1).cluster)).group
    } else {
        (*entry1).group
    };
    let mut group2: libc::c_int = if !((*entry2).cluster).is_null() {
        (*hol_cluster_base((*entry2).cluster)).group
    } else {
        (*entry2).group
    };
    let mut cmp: libc::c_int = group_cmp(group1, group2);
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    cmp = ((*entry1).cluster != 0 as *mut libc::c_void as *mut hol_cluster)
        as libc::c_int
        - ((*entry2).cluster != 0 as *mut libc::c_void as *mut hol_cluster)
            as libc::c_int;
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if !((*entry1).cluster).is_null() {
        cmp = hol_cluster_cmp((*entry1).cluster, (*entry2).cluster);
        if cmp != 0 as libc::c_int {
            return cmp;
        }
    }
    cmp = group_cmp((*entry1).group, (*entry2).group);
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    let mut long1: *const libc::c_char = hol_entry_first_long(entry1);
    let mut long2: *const libc::c_char = hol_entry_first_long(entry2);
    let mut doc1: libc::c_int = if (*(*entry1).opt).flags & 0x8 as libc::c_int != 0 {
        (!long1.is_null() && canon_doc_option(&mut long1) != 0) as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut doc2: libc::c_int = if (*(*entry2).opt).flags & 0x8 as libc::c_int != 0 {
        (!long2.is_null() && canon_doc_option(&mut long2) != 0) as libc::c_int
    } else {
        0 as libc::c_int
    };
    cmp = doc1 - doc2;
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    let mut short1: libc::c_int = hol_entry_first_short(entry1) as libc::c_int;
    let mut short2: libc::c_int = hol_entry_first_short(entry2) as libc::c_int;
    let mut first1: libc::c_uchar = (if short1 != 0 {
        short1
    } else if !long1.is_null() {
        *long1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uchar;
    let mut first2: libc::c_uchar = (if short2 != 0 {
        short2
    } else if !long2.is_null() {
        *long2 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uchar;
    cmp = ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = first1 as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(first1 as libc::c_int);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(first1 as libc::c_int as isize);
        }
        __res
    })
        - ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = first2 as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(first2 as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(first2 as libc::c_int as isize);
            }
            __res
        });
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    cmp = first2 as libc::c_int - first1 as libc::c_int;
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    cmp = (short1 != 0 as libc::c_int) as libc::c_int
        - (short2 != 0 as libc::c_int) as libc::c_int;
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if short1 == 0 as libc::c_int {
        cmp = (long1 != 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int
            - (long2 != 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int;
        if cmp != 0 as libc::c_int {
            return cmp;
        }
        if !long1.is_null() {
            cmp = strcasecmp(long1, long2);
            if cmp != 0 as libc::c_int {
                return cmp;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hol_entry_qcmp(
    mut entry1_v: *const libc::c_void,
    mut entry2_v: *const libc::c_void,
) -> libc::c_int {
    return hol_entry_cmp(entry1_v as *const hol_entry, entry2_v as *const hol_entry);
}
unsafe extern "C" fn hol_sort(mut hol: *mut hol) {
    if (*hol).num_entries > 0 as libc::c_int as libc::c_uint {
        let mut i: libc::c_uint = 0;
        let mut e: *mut hol_entry = 0 as *mut hol_entry;
        i = 0 as libc::c_int as libc::c_uint;
        e = (*hol).entries;
        while i < (*hol).num_entries {
            (*e).ord = i;
            i = i.wrapping_add(1);
            i;
            e = e.offset(1);
            e;
        }
        qsort(
            (*hol).entries as *mut libc::c_void,
            (*hol).num_entries as size_t,
            ::std::mem::size_of::<hol_entry>() as libc::c_ulong,
            Some(
                hol_entry_qcmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn hol_append(mut hol: *mut hol, mut more: *mut hol) {
    let mut cl_end: *mut *mut hol_cluster = &mut (*hol).clusters;
    while !(*cl_end).is_null() {
        cl_end = &mut (**cl_end).next;
    }
    *cl_end = (*more).clusters;
    (*more).clusters = 0 as *mut hol_cluster;
    if (*more).num_entries > 0 as libc::c_int as libc::c_uint {
        if (*hol).num_entries == 0 as libc::c_int as libc::c_uint {
            (*hol).num_entries = (*more).num_entries;
            (*hol).entries = (*more).entries;
            (*hol).short_options = (*more).short_options;
            (*more).num_entries = 0 as libc::c_int as libc::c_uint;
        } else {
            let mut left: libc::c_uint = 0;
            let mut so: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut more_so: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut e: *mut hol_entry = 0 as *mut hol_entry;
            let mut num_entries: libc::c_uint = ((*hol).num_entries)
                .wrapping_add((*more).num_entries);
            let mut entries: *mut hol_entry = malloc(
                (num_entries as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<hol_entry>() as libc::c_ulong),
            ) as *mut hol_entry;
            let mut hol_so_len: libc::c_uint = strlen((*hol).short_options)
                as libc::c_uint;
            let mut short_options: *mut libc::c_char = malloc(
                (hol_so_len as libc::c_ulong)
                    .wrapping_add(strlen((*more).short_options))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if !entries.is_null() && !short_options.is_null() {} else {
                __assert_fail(
                    b"entries && short_options\0" as *const u8 as *const libc::c_char,
                    b"argp-help.c\0" as *const u8 as *const libc::c_char,
                    969 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"void hol_append(struct hol *, struct hol *)\0"))
                        .as_ptr(),
                );
            }
            'c_9532: {
                if !entries.is_null() && !short_options.is_null() {} else {
                    __assert_fail(
                        b"entries && short_options\0" as *const u8
                            as *const libc::c_char,
                        b"argp-help.c\0" as *const u8 as *const libc::c_char,
                        969 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"void hol_append(struct hol *, struct hol *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if -(1 as libc::c_int) as size_t
                <= (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as libc::c_ulong
            {
                if num_entries as libc::c_ulong
                    <= (-(1 as libc::c_int) as size_t)
                        .wrapping_div(
                            ::std::mem::size_of::<hol_entry>() as libc::c_ulong,
                        )
                {} else {
                    __assert_fail(
                        b"num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                            as *const u8 as *const libc::c_char,
                        b"argp-help.c\0" as *const u8 as *const libc::c_char,
                        971 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"void hol_append(struct hol *, struct hol *)\0"))
                            .as_ptr(),
                    );
                }
                'c_9470: {
                    if num_entries as libc::c_ulong
                        <= (-(1 as libc::c_int) as size_t)
                            .wrapping_div(
                                ::std::mem::size_of::<hol_entry>() as libc::c_ulong,
                            )
                    {} else {
                        __assert_fail(
                            b"num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                                as *const u8 as *const libc::c_char,
                            b"argp-help.c\0" as *const u8 as *const libc::c_char,
                            971 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 44],
                                &[libc::c_char; 44],
                            >(b"void hol_append(struct hol *, struct hol *)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            mempcpy(
                mempcpy(
                    entries as *mut libc::c_void,
                    (*hol).entries as *const libc::c_void,
                    ((*hol).num_entries as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<hol_entry>() as libc::c_ulong,
                        ),
                ),
                (*more).entries as *const libc::c_void,
                ((*more).num_entries as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<hol_entry>() as libc::c_ulong),
            );
            mempcpy(
                short_options as *mut libc::c_void,
                (*hol).short_options as *const libc::c_void,
                hol_so_len as libc::c_ulong,
            );
            e = entries;
            left = (*hol).num_entries;
            while left > 0 as libc::c_int as libc::c_uint {
                (*e)
                    .short_options = short_options
                    .offset(
                        ((*e).short_options).offset_from((*hol).short_options)
                            as libc::c_long as isize,
                    );
                e = e.offset(1);
                e;
                left = left.wrapping_sub(1);
                left;
            }
            so = short_options.offset(hol_so_len as isize);
            more_so = (*more).short_options;
            left = (*more).num_entries;
            while left > 0 as libc::c_int as libc::c_uint {
                let mut opts_left: libc::c_int = 0;
                let mut opt: *const argp_option = 0 as *const argp_option;
                (*e).short_options = so;
                opts_left = (*e).num as libc::c_int;
                opt = (*e).opt;
                while opts_left != 0 {
                    let mut ch: libc::c_int = *more_so as libc::c_int;
                    if _option_is_short(opt) != 0 && ch == (*opt).key {
                        if find_char(
                            ch as libc::c_char,
                            short_options,
                            short_options.offset(hol_so_len as isize),
                        ) == 0
                        {
                            let fresh5 = so;
                            so = so.offset(1);
                            *fresh5 = ch as libc::c_char;
                        }
                        more_so = more_so.offset(1);
                        more_so;
                    }
                    opt = opt.offset(1);
                    opt;
                    opts_left -= 1;
                    opts_left;
                }
                e = e.offset(1);
                e;
                left = left.wrapping_sub(1);
                left;
            }
            *so = '\0' as i32 as libc::c_char;
            rpl_free((*hol).entries as *mut libc::c_void);
            rpl_free((*hol).short_options as *mut libc::c_void);
            (*hol).entries = entries;
            (*hol).num_entries = num_entries;
            (*hol).short_options = short_options;
        }
    }
    hol_free(more);
}
unsafe extern "C" fn argp_hol(
    mut argp: *const argp,
    mut cluster: *mut hol_cluster,
) -> *mut hol {
    let mut child: *const argp_child = (*argp).children;
    let mut hol: *mut hol = make_hol(argp, cluster);
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let mut child_cluster: *mut hol_cluster = if (*child).group != 0
                || !((*child).header).is_null()
            {
                hol_add_cluster(
                    hol,
                    (*child).group,
                    (*child).header,
                    child.offset_from((*argp).children) as libc::c_long as libc::c_int,
                    cluster,
                    argp,
                )
            } else {
                cluster
            };
            hol_append(hol, argp_hol((*child).argp, child_cluster));
            child = child.offset(1);
            child;
        }
    }
    return hol;
}
unsafe extern "C" fn indent_to(mut stream: argp_fmtstream_t, mut col: libc::c_uint) {
    let mut needed: libc::c_int = (col as libc::c_ulong)
        .wrapping_sub(argp_fmtstream_point(stream)) as libc::c_int;
    loop {
        let fresh6 = needed;
        needed = needed - 1;
        if !(fresh6 > 0 as libc::c_int) {
            break;
        }
        argp_fmtstream_putc(stream, ' ' as i32);
    };
}
unsafe extern "C" fn space(mut stream: argp_fmtstream_t, mut ensure: size_t) {
    if (argp_fmtstream_point(stream)).wrapping_add(ensure) >= (*stream).rmargin {
        argp_fmtstream_putc(stream, '\n' as i32);
    } else {
        argp_fmtstream_putc(stream, ' ' as i32);
    };
}
unsafe extern "C" fn arg(
    mut real: *const argp_option,
    mut req_fmt: *const libc::c_char,
    mut opt_fmt: *const libc::c_char,
    mut domain: *const libc::c_char,
    mut stream: argp_fmtstream_t,
) {
    if !((*real).arg).is_null() {
        if (*real).flags & 0x1 as libc::c_int != 0 {
            argp_fmtstream_printf(
                stream,
                opt_fmt,
                dcgettext(domain, (*real).arg, 5 as libc::c_int),
            );
        } else {
            argp_fmtstream_printf(
                stream,
                req_fmt,
                dcgettext(domain, (*real).arg, 5 as libc::c_int),
            );
        }
    }
}
unsafe extern "C" fn filter_doc(
    mut doc: *const libc::c_char,
    mut key: libc::c_int,
    mut argp: *const argp,
    mut state: *const argp_state,
) -> *const libc::c_char {
    if !argp.is_null() && ((*argp).help_filter).is_some() {
        let mut input: *mut libc::c_void = _argp_input(argp, state);
        return (Some(((*argp).help_filter).unwrap())).unwrap()(key, doc, input);
    } else {
        return doc
    };
}
unsafe extern "C" fn print_header(
    mut str: *const libc::c_char,
    mut argp: *const argp,
    mut pest: *mut pentry_state,
) {
    let mut tstr: *const libc::c_char = if !str.is_null() {
        dcgettext((*argp).argp_domain, str, 5 as libc::c_int)
    } else {
        0 as *mut libc::c_char
    };
    let mut fstr: *const libc::c_char = filter_doc(
        tstr,
        0x2000003 as libc::c_int,
        argp,
        (*pest).state,
    );
    if !fstr.is_null() {
        if *fstr != 0 {
            if !((*(*pest).hhstate).prev_entry).is_null() {
                argp_fmtstream_putc((*pest).stream, '\n' as i32);
            }
            indent_to((*pest).stream, uparams.header_col as libc::c_uint);
            argp_fmtstream_set_lmargin((*pest).stream, uparams.header_col as size_t);
            argp_fmtstream_set_wmargin((*pest).stream, uparams.header_col as size_t);
            argp_fmtstream_puts((*pest).stream, fstr);
            argp_fmtstream_set_lmargin((*pest).stream, 0 as libc::c_int as size_t);
            argp_fmtstream_putc((*pest).stream, '\n' as i32);
        }
        (*(*pest).hhstate).sep_groups = 1 as libc::c_int;
    }
    if fstr != tstr {
        rpl_free(fstr as *mut libc::c_char as *mut libc::c_void);
    }
}
unsafe extern "C" fn hol_cluster_is_child(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> libc::c_int {
    while !cl1.is_null() && cl1 != cl2 {
        cl1 = (*cl1).parent;
    }
    return (cl1 == cl2) as libc::c_int;
}
unsafe extern "C" fn comma(mut col: libc::c_uint, mut pest: *mut pentry_state) {
    if (*pest).first != 0 {
        let mut pe: *const hol_entry = (*(*pest).hhstate).prev_entry;
        let mut cl: *const hol_cluster = (*(*pest).entry).cluster;
        if (*(*pest).hhstate).sep_groups != 0 && !pe.is_null()
            && (*(*pest).entry).group != (*pe).group
        {
            argp_fmtstream_putc((*pest).stream, '\n' as i32);
        }
        if !cl.is_null() && !((*cl).header).is_null()
            && *(*cl).header as libc::c_int != 0
            && (pe.is_null()
                || (*pe).cluster != cl as *mut hol_cluster
                    && hol_cluster_is_child((*pe).cluster, cl) == 0)
        {
            let mut old_wm: libc::c_int = (*(*pest).stream).wmargin as libc::c_int;
            print_header((*cl).header, (*cl).argp, pest);
            argp_fmtstream_set_wmargin((*pest).stream, old_wm as size_t);
        }
        (*pest).first = 0 as libc::c_int;
    } else {
        argp_fmtstream_puts((*pest).stream, b", \0" as *const u8 as *const libc::c_char);
    }
    indent_to((*pest).stream, col);
}
unsafe extern "C" fn hol_entry_help(
    mut entry: *mut hol_entry,
    mut state: *const argp_state,
    mut stream: argp_fmtstream_t,
    mut hhstate: *mut hol_help_state,
) {
    let mut current_block: u64;
    let mut num: libc::c_uint = 0;
    let mut real: *const argp_option = (*entry).opt;
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut so: *mut libc::c_char = (*entry).short_options;
    let mut have_long_opt: libc::c_int = 0 as libc::c_int;
    let mut old_lm: libc::c_int = argp_fmtstream_set_lmargin(
        stream,
        0 as libc::c_int as size_t,
    ) as libc::c_int;
    let mut old_wm: libc::c_int = (*stream).wmargin as libc::c_int;
    let mut pest: pentry_state = {
        let mut init = pentry_state {
            entry: entry,
            stream: stream,
            hhstate: hhstate,
            first: 1 as libc::c_int,
            state: state,
        };
        init
    };
    if (*real).flags & 0x8 as libc::c_int == 0 {
        opt = real;
        num = (*entry).num;
        while num > 0 as libc::c_int as libc::c_uint {
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as libc::c_int == 0 {
                have_long_opt = 1 as libc::c_int;
                break;
            } else {
                opt = opt.offset(1);
                opt;
                num = num.wrapping_sub(1);
                num;
            }
        }
    }
    argp_fmtstream_set_wmargin(stream, uparams.short_opt_col as size_t);
    opt = real;
    num = (*entry).num;
    while num > 0 as libc::c_int as libc::c_uint {
        if _option_is_short(opt) != 0 && (*opt).key == *so as libc::c_int {
            if (*opt).flags & 0x2 as libc::c_int == 0 {
                comma(uparams.short_opt_col as libc::c_uint, &mut pest);
                argp_fmtstream_putc(stream, '-' as i32);
                argp_fmtstream_putc(stream, *so as libc::c_int);
                if have_long_opt == 0 || uparams.dup_args != 0 {
                    arg(
                        real,
                        b" %s\0" as *const u8 as *const libc::c_char,
                        b"[%s]\0" as *const u8 as *const libc::c_char,
                        if state.is_null() {
                            0 as *const libc::c_char
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        stream,
                    );
                } else if !((*real).arg).is_null() {
                    (*hhstate).suppressed_dup_arg = 1 as libc::c_int;
                }
            }
            so = so.offset(1);
            so;
        }
        opt = opt.offset(1);
        opt;
        num = num.wrapping_sub(1);
        num;
    }
    if (*real).flags & 0x8 as libc::c_int != 0 {
        argp_fmtstream_set_wmargin(stream, uparams.doc_opt_col as size_t);
        opt = real;
        num = (*entry).num;
        while num > 0 as libc::c_int as libc::c_uint {
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as libc::c_int == 0 {
                comma(uparams.doc_opt_col as libc::c_uint, &mut pest);
                argp_fmtstream_puts(
                    stream,
                    dcgettext(
                        if state.is_null() {
                            0 as *const libc::c_char
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        (*opt).name,
                        5 as libc::c_int,
                    ),
                );
            }
            opt = opt.offset(1);
            opt;
            num = num.wrapping_sub(1);
            num;
        }
    } else {
        argp_fmtstream_set_wmargin(stream, uparams.long_opt_col as size_t);
        opt = real;
        num = (*entry).num;
        while num > 0 as libc::c_int as libc::c_uint {
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as libc::c_int == 0 {
                comma(uparams.long_opt_col as libc::c_uint, &mut pest);
                argp_fmtstream_printf(
                    stream,
                    b"--%s\0" as *const u8 as *const libc::c_char,
                    (*opt).name,
                );
                arg(
                    real,
                    b"=%s\0" as *const u8 as *const libc::c_char,
                    b"[=%s]\0" as *const u8 as *const libc::c_char,
                    if state.is_null() {
                        0 as *const libc::c_char
                    } else {
                        (*(*state).root_argp).argp_domain
                    },
                    stream,
                );
            }
            opt = opt.offset(1);
            opt;
            num = num.wrapping_sub(1);
            num;
        }
    }
    argp_fmtstream_set_lmargin(stream, 0 as libc::c_int as size_t);
    if pest.first != 0 {
        if _option_is_short(real) == 0 && ((*real).name).is_null() {
            print_header((*real).doc, (*entry).argp, &mut pest);
            current_block = 10095721787123848864;
        } else {
            current_block = 9777885629667343913;
        }
    } else {
        let mut tstr: *const libc::c_char = if !((*real).doc).is_null() {
            dcgettext(
                if state.is_null() {
                    0 as *const libc::c_char
                } else {
                    (*(*state).root_argp).argp_domain
                },
                (*real).doc,
                5 as libc::c_int,
            )
        } else {
            0 as *mut libc::c_char
        };
        let mut fstr: *const libc::c_char = filter_doc(
            tstr,
            (*real).key,
            (*entry).argp,
            state,
        );
        if !fstr.is_null() && *fstr as libc::c_int != 0 {
            let mut col: libc::c_uint = argp_fmtstream_point(stream) as libc::c_uint;
            argp_fmtstream_set_lmargin(stream, uparams.opt_doc_col as size_t);
            argp_fmtstream_set_wmargin(stream, uparams.opt_doc_col as size_t);
            if col > (uparams.opt_doc_col + 3 as libc::c_int) as libc::c_uint {
                argp_fmtstream_putc(stream, '\n' as i32);
            } else if col >= uparams.opt_doc_col as libc::c_uint {
                argp_fmtstream_puts(
                    stream,
                    b"   \0" as *const u8 as *const libc::c_char,
                );
            } else {
                indent_to(stream, uparams.opt_doc_col as libc::c_uint);
            }
            argp_fmtstream_puts(stream, fstr);
        }
        if !fstr.is_null() && fstr != tstr {
            rpl_free(fstr as *mut libc::c_char as *mut libc::c_void);
        }
        argp_fmtstream_set_lmargin(stream, 0 as libc::c_int as size_t);
        argp_fmtstream_putc(stream, '\n' as i32);
        current_block = 10095721787123848864;
    }
    match current_block {
        10095721787123848864 => {
            (*hhstate).prev_entry = entry;
        }
        _ => {}
    }
    argp_fmtstream_set_lmargin(stream, old_lm as size_t);
    argp_fmtstream_set_wmargin(stream, old_wm as size_t);
}
unsafe extern "C" fn hol_help(
    mut hol: *mut hol,
    mut state: *const argp_state,
    mut stream: argp_fmtstream_t,
) {
    let mut num: libc::c_uint = 0;
    let mut entry: *mut hol_entry = 0 as *mut hol_entry;
    let mut hhstate: hol_help_state = {
        let mut init = hol_help_state {
            prev_entry: 0 as *mut hol_entry,
            sep_groups: 0 as libc::c_int,
            suppressed_dup_arg: 0 as libc::c_int,
        };
        init
    };
    entry = (*hol).entries;
    num = (*hol).num_entries;
    while num > 0 as libc::c_int as libc::c_uint {
        hol_entry_help(entry, state, stream, &mut hhstate);
        entry = entry.offset(1);
        entry;
        num = num.wrapping_sub(1);
        num;
    }
    if hhstate.suppressed_dup_arg != 0 && uparams.dup_args_note != 0 {
        let mut tstr: *const libc::c_char = dcgettext(
            if state.is_null() {
                0 as *const libc::c_char
            } else {
                (*(*state).root_argp).argp_domain
            },
            b"Mandatory or optional arguments to long options are also mandatory or optional for any corresponding short options.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        let mut fstr: *const libc::c_char = filter_doc(
            tstr,
            0x2000005 as libc::c_int,
            if !state.is_null() { (*state).root_argp } else { 0 as *const argp },
            state,
        );
        if !fstr.is_null() && *fstr as libc::c_int != 0 {
            argp_fmtstream_putc(stream, '\n' as i32);
            argp_fmtstream_puts(stream, fstr);
            argp_fmtstream_putc(stream, '\n' as i32);
        }
        if !fstr.is_null() && fstr != tstr {
            rpl_free(fstr as *mut libc::c_char as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn add_argless_short_opt(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const libc::c_char,
    mut cookie: *mut libc::c_void,
) -> libc::c_int {
    let mut snao_end: *mut *mut libc::c_char = cookie as *mut *mut libc::c_char;
    if !(!((*opt).arg).is_null() || !((*real).arg).is_null())
        && ((*opt).flags | (*real).flags) & 0x10 as libc::c_int == 0
    {
        let fresh7 = *snao_end;
        *snao_end = (*snao_end).offset(1);
        *fresh7 = (*opt).key as libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage_argful_short_opt(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const libc::c_char,
    mut cookie: *mut libc::c_void,
) -> libc::c_int {
    let mut stream: argp_fmtstream_t = cookie as argp_fmtstream_t;
    let mut arg_0: *const libc::c_char = (*opt).arg;
    let mut flags: libc::c_int = (*opt).flags | (*real).flags;
    if arg_0.is_null() {
        arg_0 = (*real).arg;
    }
    if !arg_0.is_null() && flags & 0x10 as libc::c_int == 0 {
        arg_0 = dcgettext(domain, arg_0, 5 as libc::c_int);
        if flags & 0x1 as libc::c_int != 0 {
            argp_fmtstream_printf(
                stream,
                b" [-%c[%s]]\0" as *const u8 as *const libc::c_char,
                (*opt).key,
                arg_0,
            );
        } else {
            space(
                stream,
                (6 as libc::c_int as libc::c_ulong).wrapping_add(strlen(arg_0)),
            );
            argp_fmtstream_printf(
                stream,
                b"[-%c %s]\0" as *const u8 as *const libc::c_char,
                (*opt).key,
                arg_0,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage_long_opt(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const libc::c_char,
    mut cookie: *mut libc::c_void,
) -> libc::c_int {
    let mut stream: argp_fmtstream_t = cookie as argp_fmtstream_t;
    let mut arg_0: *const libc::c_char = (*opt).arg;
    let mut flags: libc::c_int = (*opt).flags | (*real).flags;
    if arg_0.is_null() {
        arg_0 = (*real).arg;
    }
    if flags & 0x10 as libc::c_int == 0 {
        if !arg_0.is_null() {
            arg_0 = dcgettext(domain, arg_0, 5 as libc::c_int);
            if flags & 0x1 as libc::c_int != 0 {
                argp_fmtstream_printf(
                    stream,
                    b" [--%s[=%s]]\0" as *const u8 as *const libc::c_char,
                    (*opt).name,
                    arg_0,
                );
            } else {
                argp_fmtstream_printf(
                    stream,
                    b" [--%s=%s]\0" as *const u8 as *const libc::c_char,
                    (*opt).name,
                    arg_0,
                );
            }
        } else {
            argp_fmtstream_printf(
                stream,
                b" [--%s]\0" as *const u8 as *const libc::c_char,
                (*opt).name,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hol_usage(mut hol: *mut hol, mut stream: argp_fmtstream_t) {
    if (*hol).num_entries > 0 as libc::c_int as libc::c_uint {
        let mut nentries: libc::c_uint = 0;
        let mut entry: *mut hol_entry = 0 as *mut hol_entry;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            (strlen((*hol).short_options))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut short_no_arg_opts: *mut libc::c_char = fresh8.leak().as_mut_ptr()
            as *mut libc::c_char;
        let mut snao_end: *mut libc::c_char = short_no_arg_opts;
        entry = (*hol).entries;
        nentries = (*hol).num_entries;
        while nentries > 0 as libc::c_int as libc::c_uint {
            hol_entry_short_iterate(
                entry,
                Some(
                    add_argless_short_opt
                        as unsafe extern "C" fn(
                            *const argp_option,
                            *const argp_option,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                (*(*entry).argp).argp_domain,
                &mut snao_end as *mut *mut libc::c_char as *mut libc::c_void,
            );
            entry = entry.offset(1);
            entry;
            nentries = nentries.wrapping_sub(1);
            nentries;
        }
        if snao_end > short_no_arg_opts {
            let fresh9 = snao_end;
            snao_end = snao_end.offset(1);
            *fresh9 = 0 as libc::c_int as libc::c_char;
            argp_fmtstream_printf(
                stream,
                b" [-%s]\0" as *const u8 as *const libc::c_char,
                short_no_arg_opts,
            );
        }
        entry = (*hol).entries;
        nentries = (*hol).num_entries;
        while nentries > 0 as libc::c_int as libc::c_uint {
            hol_entry_short_iterate(
                entry,
                Some(
                    usage_argful_short_opt
                        as unsafe extern "C" fn(
                            *const argp_option,
                            *const argp_option,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                (*(*entry).argp).argp_domain,
                stream as *mut libc::c_void,
            );
            entry = entry.offset(1);
            entry;
            nentries = nentries.wrapping_sub(1);
            nentries;
        }
        entry = (*hol).entries;
        nentries = (*hol).num_entries;
        while nentries > 0 as libc::c_int as libc::c_uint {
            hol_entry_long_iterate(
                entry,
                Some(
                    usage_long_opt
                        as unsafe extern "C" fn(
                            *const argp_option,
                            *const argp_option,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                (*(*entry).argp).argp_domain,
                stream as *mut libc::c_void,
            );
            entry = entry.offset(1);
            entry;
            nentries = nentries.wrapping_sub(1);
            nentries;
        }
    }
}
unsafe extern "C" fn argp_args_levels(mut argp: *const argp) -> size_t {
    let mut levels: size_t = 0 as libc::c_int as size_t;
    let mut child: *const argp_child = (*argp).children;
    if !((*argp).args_doc).is_null()
        && !(strchr((*argp).args_doc, '\n' as i32)).is_null()
    {
        levels = levels.wrapping_add(1);
        levels;
    }
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let fresh10 = child;
            child = child.offset(1);
            levels = (levels as libc::c_ulong)
                .wrapping_add(argp_args_levels((*fresh10).argp)) as size_t as size_t;
        }
    }
    return levels;
}
unsafe extern "C" fn argp_args_usage(
    mut argp: *const argp,
    mut state: *const argp_state,
    mut levels: *mut *mut libc::c_char,
    mut advance: libc::c_int,
    mut stream: argp_fmtstream_t,
) -> libc::c_int {
    let mut our_level: *mut libc::c_char = *levels;
    let mut multiple: libc::c_int = 0 as libc::c_int;
    let mut child: *const argp_child = (*argp).children;
    let mut tdoc: *const libc::c_char = if !((*argp).args_doc).is_null() {
        dcgettext((*argp).argp_domain, (*argp).args_doc, 5 as libc::c_int)
    } else {
        0 as *mut libc::c_char
    };
    let mut fdoc: *const libc::c_char = filter_doc(
        tdoc,
        0x2000006 as libc::c_int,
        argp,
        state,
    );
    let mut nl: *const libc::c_char = 0 as *const libc::c_char;
    if !fdoc.is_null() {
        let mut cp: *const libc::c_char = fdoc;
        nl = strchrnul(cp, '\n' as i32);
        if *nl as libc::c_int != '\0' as i32 {
            let mut i: libc::c_int = 0;
            multiple = 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < *our_level as libc::c_int {
                cp = nl.offset(1 as libc::c_int as isize);
                nl = strchrnul(cp, '\n' as i32);
                i += 1;
                i;
            }
            *levels = (*levels).offset(1);
            *levels;
        }
        space(
            stream,
            nl.offset(1 as libc::c_int as isize).offset_from(cp) as libc::c_long
                as size_t,
        );
        argp_fmtstream_write(stream, cp, nl.offset_from(cp) as libc::c_long as size_t);
    }
    if !fdoc.is_null() && fdoc != tdoc {
        rpl_free(fdoc as *mut libc::c_char as *mut libc::c_void);
    }
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let fresh11 = child;
            child = child.offset(1);
            advance = (argp_args_usage((*fresh11).argp, state, levels, advance, stream)
                == 0) as libc::c_int;
        }
    }
    if advance != 0 && multiple != 0 {
        if *nl != 0 {
            *our_level += 1;
            *our_level;
            advance = 0 as libc::c_int;
        } else if *our_level as libc::c_int > 0 as libc::c_int {
            *our_level = 0 as libc::c_int as libc::c_char;
        }
    }
    return (advance == 0) as libc::c_int;
}
unsafe extern "C" fn argp_doc(
    mut argp: *const argp,
    mut state: *const argp_state,
    mut post: libc::c_int,
    mut pre_blank: libc::c_int,
    mut first_only: libc::c_int,
    mut stream: argp_fmtstream_t,
) -> libc::c_int {
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut inp_text: *const libc::c_char = 0 as *const libc::c_char;
    let mut input: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut anything: libc::c_int = 0 as libc::c_int;
    let mut inp_text_limit: size_t = 0 as libc::c_int as size_t;
    let mut doc: *const libc::c_char = if !((*argp).doc).is_null() {
        dcgettext((*argp).argp_domain, (*argp).doc, 5 as libc::c_int)
    } else {
        0 as *mut libc::c_char
    };
    let mut child: *const argp_child = (*argp).children;
    if !doc.is_null() {
        let mut vt: *mut libc::c_char = strchr(doc, '\u{b}' as i32);
        inp_text = if post != 0 {
            (if !vt.is_null() {
                vt.offset(1 as libc::c_int as isize)
            } else {
                0 as *mut libc::c_char
            }) as *const libc::c_char
        } else {
            doc
        };
        inp_text_limit = (if post == 0 && !vt.is_null() {
            vt.offset_from(doc) as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        }) as size_t;
    } else {
        inp_text = 0 as *const libc::c_char;
    }
    if ((*argp).help_filter).is_some() {
        if inp_text_limit != 0 {
            inp_text = strndup(inp_text, inp_text_limit);
        }
        input = _argp_input(argp, state);
        text = (Some(((*argp).help_filter).unwrap()))
            .unwrap()(
            if post != 0 { 0x2000002 as libc::c_int } else { 0x2000001 as libc::c_int },
            inp_text,
            input,
        );
    } else {
        text = inp_text;
    }
    if !text.is_null() {
        if pre_blank != 0 {
            argp_fmtstream_putc(stream, '\n' as i32);
        }
        if text == inp_text && inp_text_limit != 0 {
            argp_fmtstream_write(stream, inp_text, inp_text_limit);
        } else {
            argp_fmtstream_puts(stream, text);
        }
        if argp_fmtstream_point(stream) > (*stream).lmargin {
            argp_fmtstream_putc(stream, '\n' as i32);
        }
        anything = 1 as libc::c_int;
    }
    if !text.is_null() && text != inp_text {
        rpl_free(text as *mut libc::c_char as *mut libc::c_void);
    }
    if !inp_text.is_null() && inp_text_limit != 0 && ((*argp).help_filter).is_some() {
        rpl_free(inp_text as *mut libc::c_char as *mut libc::c_void);
    }
    if post != 0 && ((*argp).help_filter).is_some() {
        text = (Some(((*argp).help_filter).unwrap()))
            .unwrap()(0x2000004 as libc::c_int, 0 as *const libc::c_char, input);
        if !text.is_null() {
            if anything != 0 || pre_blank != 0 {
                argp_fmtstream_putc(stream, '\n' as i32);
            }
            argp_fmtstream_puts(stream, text);
            rpl_free(text as *mut libc::c_char as *mut libc::c_void);
            if argp_fmtstream_point(stream) > (*stream).lmargin {
                argp_fmtstream_putc(stream, '\n' as i32);
            }
            anything = 1 as libc::c_int;
        }
    }
    if !child.is_null() {
        while !((*child).argp).is_null() && !(first_only != 0 && anything != 0) {
            let fresh12 = child;
            child = child.offset(1);
            anything
                |= argp_doc(
                    (*fresh12).argp,
                    state,
                    post,
                    (anything != 0 || pre_blank != 0) as libc::c_int,
                    first_only,
                    stream,
                );
        }
    }
    return anything;
}
unsafe extern "C" fn _help(
    mut argp: *const argp,
    mut state: *const argp_state,
    mut stream: *mut FILE,
    mut flags: libc::c_uint,
    mut name: *mut libc::c_char,
) {
    let mut anything: libc::c_int = 0 as libc::c_int;
    let mut hol: *mut hol = 0 as *mut hol;
    let mut fs: argp_fmtstream_t = 0 as *mut argp_fmtstream;
    if stream.is_null() {
        return;
    }
    flockfile(stream);
    if uparams.valid == 0 {
        fill_in_uparams(state);
    }
    fs = argp_make_fmtstream(
        stream,
        0 as libc::c_int as size_t,
        uparams.rmargin as size_t,
        0 as libc::c_int as ssize_t,
    );
    if fs.is_null() {
        funlockfile(stream);
        return;
    }
    if flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint
        != 0
    {
        hol = argp_hol(argp, 0 as *mut hol_cluster);
        hol_set_group(
            hol,
            b"help\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        hol_set_group(
            hol,
            b"version\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        hol_sort(hol);
    }
    if flags & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint != 0 {
        let mut first_pattern: libc::c_int = 1 as libc::c_int;
        let mut more_patterns: libc::c_int = 0;
        let mut num_pattern_levels: size_t = argp_args_levels(argp);
        let mut fresh13 = ::std::vec::from_elem(0, num_pattern_levels as usize);
        let mut pattern_levels: *mut libc::c_char = fresh13.leak().as_mut_ptr()
            as *mut libc::c_char;
        memset(
            pattern_levels as *mut libc::c_void,
            0 as libc::c_int,
            num_pattern_levels,
        );
        loop {
            let mut old_lm: libc::c_int = 0;
            let mut old_wm: libc::c_int = argp_fmtstream_set_wmargin(
                fs,
                uparams.usage_indent as size_t,
            ) as libc::c_int;
            let mut levels: *mut libc::c_char = pattern_levels;
            if first_pattern != 0 {
                argp_fmtstream_printf(
                    fs,
                    b"%s %s\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        (*argp).argp_domain,
                        b"Usage:\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
            } else {
                argp_fmtstream_printf(
                    fs,
                    b"%s %s\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        (*argp).argp_domain,
                        b"  or: \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
            }
            old_lm = argp_fmtstream_set_lmargin(fs, uparams.usage_indent as size_t)
                as libc::c_int;
            if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                if (*hol).num_entries > 0 as libc::c_int as libc::c_uint {
                    argp_fmtstream_puts(
                        fs,
                        dcgettext(
                            (*argp).argp_domain,
                            b" [OPTION...]\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            } else {
                hol_usage(hol, fs);
                flags |= 0x2 as libc::c_int as libc::c_uint;
            }
            more_patterns = argp_args_usage(
                argp,
                state,
                &mut levels,
                1 as libc::c_int,
                fs,
            );
            argp_fmtstream_set_wmargin(fs, old_wm as size_t);
            argp_fmtstream_set_lmargin(fs, old_lm as size_t);
            argp_fmtstream_putc(fs, '\n' as i32);
            anything = 1 as libc::c_int;
            first_pattern = 0 as libc::c_int;
            if !(more_patterns != 0) {
                break;
            }
        }
    }
    if flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        anything
            |= argp_doc(
                argp,
                state,
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
                fs,
            );
    }
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        argp_fmtstream_printf(
            fs,
            dcgettext(
                (*argp).argp_domain,
                b"Try '%s --help' or '%s --usage' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
            name,
        );
        anything = 1 as libc::c_int;
    }
    if flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        if (*hol).num_entries > 0 as libc::c_int as libc::c_uint {
            if anything != 0 {
                argp_fmtstream_putc(fs, '\n' as i32);
            }
            hol_help(hol, state, fs);
            anything = 1 as libc::c_int;
        }
    }
    if flags & 0x20 as libc::c_int as libc::c_uint != 0 {
        anything
            |= argp_doc(argp, state, 1 as libc::c_int, anything, 0 as libc::c_int, fs);
    }
    if flags & 0x40 as libc::c_int as libc::c_uint != 0
        && !argp_program_bug_address.is_null()
    {
        if anything != 0 {
            argp_fmtstream_putc(fs, '\n' as i32);
        }
        argp_fmtstream_printf(
            fs,
            dcgettext(
                (*argp).argp_domain,
                b"Report bugs to %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            argp_program_bug_address,
        );
        anything = 1 as libc::c_int;
    }
    funlockfile(stream);
    if !hol.is_null() {
        hol_free(hol);
    }
    argp_fmtstream_free(fs);
}
pub unsafe extern "C" fn argp_help(
    mut argp: *const argp,
    mut stream: *mut FILE,
    mut flags: libc::c_uint,
    mut name: *mut libc::c_char,
) {
    _help(argp, 0 as *const argp_state, stream, flags, name);
}
pub unsafe extern "C" fn argp_state_help(
    mut state: *const argp_state,
    mut stream: *mut FILE,
    mut flags: libc::c_uint,
) {
    if (state.is_null() || (*state).flags & 0x2 as libc::c_int as libc::c_uint == 0)
        && !stream.is_null()
    {
        if !state.is_null() && (*state).flags & 0x40 as libc::c_int as libc::c_uint != 0
        {
            flags |= 0x80 as libc::c_int as libc::c_uint;
        }
        _help(
            if !state.is_null() { (*state).root_argp } else { 0 as *const argp },
            state,
            stream,
            flags,
            if !state.is_null() { (*state).name } else { program_invocation_short_name },
        );
        if state.is_null() || (*state).flags & 0x20 as libc::c_int as libc::c_uint == 0 {
            if flags & 0x100 as libc::c_int as libc::c_uint != 0 {
                exit(argp_err_exit_status);
            }
            if flags & 0x200 as libc::c_int as libc::c_uint != 0 {
                exit(0 as libc::c_int);
            }
        }
    }
}
pub unsafe extern "C" fn argp_error(
    mut state: *const argp_state,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if state.is_null() || (*state).flags & 0x2 as libc::c_int as libc::c_uint == 0 {
        let mut stream: *mut FILE = if !state.is_null() {
            (*state).err_stream
        } else {
            stderr
        };
        if !stream.is_null() {
            let mut ap: ::std::ffi::VaListImpl;
            flockfile(stream);
            ap = args.clone();
            fputs_unlocked(
                if !state.is_null() {
                    (*state).name
                } else {
                    program_invocation_short_name
                },
                stream,
            );
            putc_unlocked(':' as i32, stream);
            putc_unlocked(' ' as i32, stream);
            vfprintf(stream, fmt, ap.as_va_list());
            putc_unlocked('\n' as i32, stream);
            argp_state_help(
                state,
                stream,
                (0x4 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint,
            );
            funlockfile(stream);
        }
    }
}
pub unsafe extern "C" fn argp_failure(
    mut state: *const argp_state,
    mut status: libc::c_int,
    mut errnum: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if state.is_null() || (*state).flags & 0x2 as libc::c_int as libc::c_uint == 0 {
        let mut stream: *mut FILE = if !state.is_null() {
            (*state).err_stream
        } else {
            stderr
        };
        if !stream.is_null() {
            flockfile(stream);
            fputs_unlocked(
                if !state.is_null() {
                    (*state).name
                } else {
                    program_invocation_short_name
                },
                stream,
            );
            if !fmt.is_null() {
                let mut ap: ::std::ffi::VaListImpl;
                ap = args.clone();
                putc_unlocked(':' as i32, stream);
                putc_unlocked(' ' as i32, stream);
                vfprintf(stream, fmt, ap.as_va_list());
            }
            if errnum != 0 {
                let mut buf: [libc::c_char; 200] = [0; 200];
                let mut s: *const libc::c_char = 0 as *const libc::c_char;
                putc_unlocked(':' as i32, stream);
                putc_unlocked(' ' as i32, stream);
                s = strerror_r(
                    errnum,
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong,
                );
                if s.is_null()
                    && {
                        s = strerror(errnum);
                        s.is_null()
                    }
                {
                    s = dcgettext(
                        (*(*state).root_argp).argp_domain,
                        b"Unknown system error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                }
                fputs_unlocked(s, stream);
            }
            putc_unlocked('\n' as i32, stream);
            funlockfile(stream);
            if status != 0
                && (state.is_null()
                    || (*state).flags & 0x20 as libc::c_int as libc::c_uint == 0)
            {
                exit(status);
            }
        }
    }
}

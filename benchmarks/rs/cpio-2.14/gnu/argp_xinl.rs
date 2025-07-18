use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn argp_state_help(
        __state: *const argp_state,
        __stream: *mut FILE,
        __flags: libc::c_uint,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type error_t = libc::c_int;
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
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_usage(mut __state: *const argp_state) {
    argp_state_help(
        __state,
        stderr,
        (0x2 as libc::c_int | 0x4 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint,
    );
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _option_is_short(mut __opt: *const argp_option) -> libc::c_int {
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
#[linkage = "external"]
pub unsafe extern "C" fn _option_is_end(mut __opt: *const argp_option) -> libc::c_int {
    return ((*__opt).key == 0 && ((*__opt).name).is_null() && ((*__opt).doc).is_null()
        && (*__opt).group == 0) as libc::c_int;
}

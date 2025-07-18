use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bc_struct;
    pub type dc_string;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut progname: *const libc::c_char;
    fn dc_out_num(_: dc_num, _: libc::c_int, _: dc_discard);
    fn dc_out_str(_: dc_str, _: dc_discard);
    fn dc_dup_num(_: dc_num) -> dc_data;
    fn dc_dup_str(_: dc_str) -> dc_data;
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
pub type dc_discard = libc::c_uint;
pub const DC_KEEP: dc_discard = 1;
pub const DC_TOSS: dc_discard = 0;
pub type dc_newline = libc::c_uint;
pub const DC_WITHNL: dc_newline = 1;
pub const DC_NONL: dc_newline = 0;
pub type dc_value_type = libc::c_uint;
pub const DC_STRING: dc_value_type = 2;
pub const DC_NUMBER: dc_value_type = 1;
pub const DC_UNINITIALIZED: dc_value_type = 0;
pub type dc_num = *mut bc_struct;
pub type dc_str = *mut dc_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_data {
    pub dc_type: dc_value_type,
    pub v: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub number: dc_num,
    pub string: dc_str,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub unsafe extern "C" fn dc_memfail() {
    fprintf(
        stderr,
        b"%s: out of memory\n\0" as *const u8 as *const libc::c_char,
        progname,
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn dc_malloc(mut len: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = malloc(len);
    if result.is_null() {
        dc_memfail();
    }
    return result;
}
pub unsafe extern "C" fn dc_show_id(
    mut fp: *mut FILE,
    mut id: libc::c_int,
    mut suffix: *const libc::c_char,
) {
    if *(*__ctype_b_loc()).offset(id as isize) as libc::c_int
        & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        fprintf(
            fp,
            b"'%c' (%#o)%s\0" as *const u8 as *const libc::c_char,
            id as libc::c_uint,
            id,
            suffix,
        );
    } else {
        fprintf(
            fp,
            b"%#o%s\0" as *const u8 as *const libc::c_char,
            id as libc::c_uint,
            suffix,
        );
    };
}
pub unsafe extern "C" fn dc_garbage(
    mut msg: *const libc::c_char,
    mut regid: libc::c_int,
) {
    if regid < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s: garbage %s\n\0" as *const u8 as *const libc::c_char,
            progname,
            msg,
        );
    } else {
        fprintf(
            stderr,
            b"%s:%s register \0" as *const u8 as *const libc::c_char,
            progname,
            msg,
        );
        dc_show_id(
            stderr,
            regid,
            b" is garbage\n\0" as *const u8 as *const libc::c_char,
        );
    }
    abort();
}
pub unsafe extern "C" fn dc_system(mut s: *const libc::c_char) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmpstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    p = strchr(s, '\n' as i32);
    if !p.is_null() {
        len = p.offset_from(s) as libc::c_long as size_t;
        tmpstr = dc_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        strncpy(tmpstr, s, len);
        *tmpstr.offset(len as isize) = '\0' as i32 as libc::c_char;
        system(tmpstr);
        free(tmpstr as *mut libc::c_void);
        return p.offset(1 as libc::c_int as isize);
    }
    system(s);
    return s.offset(strlen(s) as isize);
}
pub unsafe extern "C" fn dc_print(
    mut value: dc_data,
    mut obase: libc::c_int,
    mut newline_p: dc_newline,
    mut discard_p: dc_discard,
) {
    if value.dc_type as libc::c_uint == DC_NUMBER as libc::c_int as libc::c_uint {
        dc_out_num(value.v.number, obase, discard_p);
    } else if value.dc_type as libc::c_uint == DC_STRING as libc::c_int as libc::c_uint {
        dc_out_str(value.v.string, discard_p);
    } else {
        dc_garbage(
            b"in data being printed\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    if newline_p as libc::c_uint == DC_WITHNL as libc::c_int as libc::c_uint {
        putchar('\n' as i32);
    }
    fflush(stdout);
}
pub unsafe extern "C" fn dc_dup(mut value: dc_data) -> dc_data {
    if value.dc_type as libc::c_uint != DC_NUMBER as libc::c_int as libc::c_uint
        && value.dc_type as libc::c_uint != DC_STRING as libc::c_int as libc::c_uint
    {
        dc_garbage(
            b"in value being duplicated\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    if value.dc_type as libc::c_uint == DC_NUMBER as libc::c_int as libc::c_uint {
        return dc_dup_num(value.v.number);
    }
    return dc_dup_str(value.v.string);
}

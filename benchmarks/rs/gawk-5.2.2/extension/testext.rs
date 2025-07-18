use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getuid() -> __uid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type awk_bool = libc::c_uint;
pub const awk_true: awk_bool = 1;
pub const awk_false: awk_bool = 0;
pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input {
    pub name: *const libc::c_char,
    pub fd: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub get_record: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *mut awk_input,
            *mut libc::c_int,
            *mut *mut libc::c_char,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> libc::c_int,
    >,
    pub read_func: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option::<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input_parser {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_input_parser,
}
pub type awk_input_parser_t = awk_input_parser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const libc::c_char,
    pub mode: *const libc::c_char,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_ferror: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_fclose: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_wrapper {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_output_wrapper,
}
pub type awk_output_wrapper_t = awk_output_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_two_way_processor {
    pub name: *const libc::c_char,
    pub can_take_two_way: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut awk_input_buf_t,
            *mut awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub next: *mut awk_two_way_processor,
}
pub type awk_two_way_processor_t = awk_two_way_processor;
pub type C2RustUnnamed = libc::c_uint;
pub const GAWK_API_MINOR_VERSION: C2RustUnnamed = 2;
pub const GAWK_API_MAJOR_VERSION: C2RustUnnamed = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
pub type AWK_NUMBER_TYPE = libc::c_uint;
pub const AWK_NUMBER_TYPE_MPZ: AWK_NUMBER_TYPE = 2;
pub const AWK_NUMBER_TYPE_MPFR: AWK_NUMBER_TYPE = 1;
pub const AWK_NUMBER_TYPE_DOUBLE: AWK_NUMBER_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
pub type awk_valtype_t = libc::c_uint;
pub const AWK_BOOL: awk_valtype_t = 8;
pub const AWK_VALUE_COOKIE: awk_valtype_t = 7;
pub const AWK_SCALAR: awk_valtype_t = 6;
pub const AWK_ARRAY: awk_valtype_t = 5;
pub const AWK_STRNUM: awk_valtype_t = 4;
pub const AWK_REGEX: awk_valtype_t = 3;
pub const AWK_STRING: awk_valtype_t = 2;
pub const AWK_NUMBER: awk_valtype_t = 1;
pub const AWK_UNDEFINED: awk_valtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_element {
    pub next: *mut awk_element,
    pub flags: C2RustUnnamed_1,
    pub index: awk_value_t,
    pub value: awk_value_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const AWK_ELEMENT_DELETE: C2RustUnnamed_1 = 1;
pub const AWK_ELEMENT_DEFAULT: C2RustUnnamed_1 = 0;
pub type awk_element_t = awk_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_flat_array {
    pub opaque1: *mut libc::c_void,
    pub opaque2: *mut libc::c_void,
    pub count: size_t,
    pub elements: [awk_element_t; 1],
}
pub type awk_flat_array_t = awk_flat_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const libc::c_char,
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
pub type awk_ext_id_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gawk_api {
    pub major_version: libc::c_int,
    pub minor_version: libc::c_int,
    pub gmp_major_version: libc::c_int,
    pub gmp_minor_version: libc::c_int,
    pub mpfr_major_version: libc::c_int,
    pub mpfr_minor_version: libc::c_int,
    pub do_flags: [libc::c_int; 6],
    pub api_add_ext_func: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *mut awk_ext_func_t,
        ) -> awk_bool_t,
    >,
    pub api_register_input_parser: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_input_parser_t) -> (),
    >,
    pub api_register_output_wrapper: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_output_wrapper_t) -> (),
    >,
    pub api_register_two_way_processor: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_two_way_processor_t) -> (),
    >,
    pub api_awk_atexit: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
            *mut libc::c_void,
        ) -> (),
    >,
    pub api_register_ext_version: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
    >,
    pub api_fatal: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_warning: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_lintwarn: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_nonfatal: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_update_ERRNO_int: Option::<
        unsafe extern "C" fn(awk_ext_id_t, libc::c_int) -> (),
    >,
    pub api_update_ERRNO_string: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
    >,
    pub api_unset_ERRNO: Option::<unsafe extern "C" fn(awk_ext_id_t) -> ()>,
    pub api_get_argument: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            size_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_argument: Option::<
        unsafe extern "C" fn(awk_ext_id_t, size_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_sym_lookup: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *const libc::c_char,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *const libc::c_char,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_lookup_scalar: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_scalar_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update_scalar: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_scalar_t, *mut awk_value_t) -> awk_bool_t,
    >,
    pub api_create_value: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *mut awk_value_t,
            *mut awk_value_cookie_t,
        ) -> awk_bool_t,
    >,
    pub api_release_value: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_value_cookie_t) -> awk_bool_t,
    >,
    pub api_get_element_count: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *mut size_t) -> awk_bool_t,
    >,
    pub api_get_array_element: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_array_element: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            *const awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_del_array_element: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *const awk_value_t) -> awk_bool_t,
    >,
    pub api_create_array: Option::<unsafe extern "C" fn(awk_ext_id_t) -> awk_array_t>,
    pub api_clear_array: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_flatten_array_typed: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut *mut awk_flat_array_t,
            awk_valtype_t,
            awk_valtype_t,
        ) -> awk_bool_t,
    >,
    pub api_release_flattened_array: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut awk_flat_array_t,
        ) -> awk_bool_t,
    >,
    pub api_malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub api_calloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub api_realloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub api_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub api_get_mpfr: Option::<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_mpz: Option::<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_file: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            libc::c_int,
            *mut *const awk_input_buf_t,
            *mut *const awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub api_destroy_array: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
}
pub type gawk_api_t = gawk_api;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nval {
    pub name: *const libc::c_char,
    pub val: libc::c_double,
}
#[inline]
unsafe extern "C" fn r_make_string_type(
    mut api_0: *const gawk_api_t,
    mut ext_id_0: awk_ext_id_t,
    mut string: *const libc::c_char,
    mut length: size_t,
    mut duplicate: awk_bool_t,
    mut result: *mut awk_value_t,
    mut val_type: awk_valtype_t,
) -> *mut awk_value_t {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        result as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<awk_value_t>() as libc::c_ulong,
    );
    (*result).val_type = val_type;
    (*result).u.s.len = length;
    if duplicate as u64 != 0 {
        cp = ((*api_0).api_malloc)
            .unwrap()(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if cp.is_null() {
            ((*api_0).api_fatal)
                .unwrap()(
                ext_id_0,
                b"%s: malloc of %d bytes failed\0" as *const u8 as *const libc::c_char,
                b"r_make_string\0" as *const u8 as *const libc::c_char,
                length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        memcpy(cp as *mut libc::c_void, string as *const libc::c_void, length);
        *cp.offset(length as isize) = '\0' as i32 as libc::c_char;
        (*result).u.s.str_0 = cp;
    } else {
        (*result).u.s.str_0 = string as *mut libc::c_char;
    }
    return result;
}
#[inline]
unsafe extern "C" fn r_make_string(
    mut api_0: *const gawk_api_t,
    mut ext_id_0: awk_ext_id_t,
    mut string: *const libc::c_char,
    mut length: size_t,
    mut duplicate: awk_bool_t,
    mut result: *mut awk_value_t,
) -> *mut awk_value_t {
    return r_make_string_type(
        api_0,
        ext_id_0,
        string,
        length,
        duplicate,
        result,
        AWK_STRING,
    );
}
#[inline]
unsafe extern "C" fn make_null_string(mut result: *mut awk_value_t) -> *mut awk_value_t {
    memset(
        result as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<awk_value_t>() as libc::c_ulong,
    );
    (*result).val_type = AWK_UNDEFINED;
    return result;
}
#[inline]
unsafe extern "C" fn make_number(
    mut num: libc::c_double,
    mut result: *mut awk_value_t,
) -> *mut awk_value_t {
    (*result).val_type = AWK_NUMBER;
    (*result).u.n.d = num;
    (*result).u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
    return result;
}
static mut api: *const gawk_api_t = 0 as *const gawk_api_t;
static mut ext_id: awk_ext_id_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut ext_version: *const libc::c_char = b"testext extension: version 1.0\0"
    as *const u8 as *const libc::c_char;
pub static mut plugin_is_GPL_compatible: libc::c_int = 0;
unsafe extern "C" fn valrep2str(mut value: *const awk_value_t) -> *const libc::c_char {
    static mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut size: libc::c_int = 8192 as libc::c_int - 3 as libc::c_int;
    match (*value).val_type as libc::c_uint {
        0 => {
            strcpy(
                buf.as_mut_ptr(),
                b"<undefined>\0" as *const u8 as *const libc::c_char,
            );
        }
        5 => {
            strcpy(buf.as_mut_ptr(), b"<array>\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            strcpy(buf.as_mut_ptr(), b"<scalar>\0" as *const u8 as *const libc::c_char);
        }
        7 => {
            strcpy(
                buf.as_mut_ptr(),
                b"<value-cookie>\0" as *const u8 as *const libc::c_char,
            );
        }
        3 | 4 | 2 => {
            if (*value).u.s.len < size as libc::c_ulong {
                size = (*value).u.s.len as libc::c_int;
            }
            sprintf(
                buf.as_mut_ptr(),
                b"\"%.*s\"\0" as *const u8 as *const libc::c_char,
                size,
                (*value).u.s.str_0,
            );
        }
        8 => {
            if ((*value).u.s.len).wrapping_add(8 as libc::c_int as libc::c_ulong)
                < size as libc::c_ulong
            {
                size = (*value).u.s.len as libc::c_int;
            }
            sprintf(
                buf.as_mut_ptr(),
                b"<bool>: %.*s\0" as *const u8 as *const libc::c_char,
                size,
                (*value).u.s.str_0,
            );
        }
        1 => {
            sprintf(
                buf.as_mut_ptr(),
                b"%g\0" as *const u8 as *const libc::c_char,
                (*value).u.n.d,
            );
        }
        _ => {}
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn dump_array_and_delete(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut value2: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut value3: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut flat_array: *mut awk_flat_array_t = 0 as *mut awk_flat_array_t;
    let mut count: size_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    make_number(0.0f64, result);
    if nargs != 2 as libc::c_int {
        printf(
            b"dump_array_and_delete: nargs not right (%d should be 2)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut value) as u64 != 0
    {
        name = value.u.s.str_0;
        if ((*api).api_sym_lookup)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            name,
            AWK_ARRAY,
            &mut value2,
        ) as u64 != 0
        {
            printf(
                b"dump_array_and_delete: sym_lookup of %s passed\n\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            if ((*api).api_get_element_count).unwrap()(ext_id, value2.u.a, &mut count)
                as u64 == 0
            {
                printf(
                    b"dump_array_and_delete: get_element_count failed\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                printf(
                    b"dump_array_and_delete: incoming size is %lu\n\0" as *const u8
                        as *const libc::c_char,
                    count,
                );
                if ((*api).api_flatten_array_typed)
                    .unwrap()(
                    ext_id,
                    value2.u.a,
                    &mut flat_array,
                    AWK_STRING,
                    AWK_UNDEFINED,
                ) as u64 == 0
                {
                    printf(
                        b"dump_array_and_delete: could not flatten array\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*flat_array).count != count {
                    printf(
                        b"dump_array_and_delete: flat_array->count (%lu) != count (%lu)\n\0"
                            as *const u8 as *const libc::c_char,
                        (*flat_array).count,
                        count,
                    );
                } else if ((*api).api_get_argument)
                    .unwrap()(
                    ext_id,
                    1 as libc::c_int as size_t,
                    AWK_STRING,
                    &mut value3,
                ) as u64 == 0
                {
                    printf(
                        b"dump_array_and_delete: get_argument(1) failed\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    i = 0 as libc::c_int;
                    while (i as libc::c_ulong) < (*flat_array).count {
                        printf(
                            b"\t%s[\"%.*s\"] = %s\n\0" as *const u8
                                as *const libc::c_char,
                            name,
                            (*((*flat_array).elements).as_mut_ptr().offset(i as isize))
                                .index
                                .u
                                .s
                                .len as libc::c_int,
                            (*((*flat_array).elements).as_mut_ptr().offset(i as isize))
                                .index
                                .u
                                .s
                                .str_0,
                            valrep2str(
                                &mut (*((*flat_array).elements)
                                    .as_mut_ptr()
                                    .offset(i as isize))
                                    .value,
                            ),
                        );
                        if strcmp(
                            value3.u.s.str_0,
                            (*((*flat_array).elements).as_mut_ptr().offset(i as isize))
                                .index
                                .u
                                .s
                                .str_0,
                        ) == 0 as libc::c_int
                        {
                            let ref mut fresh0 = (*((*flat_array).elements)
                                .as_mut_ptr()
                                .offset(i as isize))
                                .flags;
                            *fresh0 = ::std::mem::transmute::<
                                libc::c_uint,
                                C2RustUnnamed_1,
                            >(
                                *fresh0 as libc::c_uint
                                    | AWK_ELEMENT_DELETE as libc::c_int as libc::c_uint,
                            );
                            printf(
                                b"dump_array_and_delete: marking element \"%s\" for deletion\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*((*flat_array).elements).as_mut_ptr().offset(i as isize))
                                    .index
                                    .u
                                    .s
                                    .str_0,
                            );
                        }
                        i += 1;
                        i;
                    }
                    if ((*api).api_release_flattened_array)
                        .unwrap()(ext_id, value2.u.a, flat_array) as u64 == 0
                    {
                        printf(
                            b"dump_array_and_delete: could not release flattened array\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else {
                        make_number(1.0f64, result);
                    }
                }
            }
        } else {
            printf(
                b"dump_array_and_delete: sym_lookup of %s failed\n\0" as *const u8
                    as *const libc::c_char,
                name,
            );
        }
    } else {
        printf(
            b"dump_array_and_delete: get_argument(0) failed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return result;
}
unsafe extern "C" fn try_modify_environ(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut newvalue: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut flat_array: *mut awk_flat_array_t = 0 as *mut awk_flat_array_t;
    let mut environ_array: awk_array_t = 0 as *mut libc::c_void;
    let mut count: size_t = 0;
    let mut i: libc::c_int = 0;
    make_number(0.0f64, result);
    if nargs != 0 as libc::c_int {
        printf(
            b"try_modify_environ: nargs not right (%d should be 0)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"ENVIRON\0" as *const u8 as *const libc::c_char,
        AWK_ARRAY,
        &mut value,
    ) as u64 != 0
    {
        printf(
            b"try_modify_environ: sym_lookup of ENVIRON passed\n\0" as *const u8
                as *const libc::c_char,
        );
        environ_array = value.u.a;
        if ((*api).api_get_element_count).unwrap()(ext_id, environ_array, &mut count)
            as u64 == 0
        {
            printf(
                b"try_modify_environ: get_element_count failed\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            r_make_string(
                api,
                ext_id,
                b"testext2\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as size_t,
                awk_true,
                &mut index,
            );
            r_make_string(
                api,
                ext_id,
                b"a value\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as size_t,
                awk_true,
                &mut value,
            );
            if ((*api).api_set_array_element)
                .unwrap()(ext_id, environ_array, &mut index, &mut newvalue) as u64 != 0
            {
                printf(
                    b"try_modify_environ: set_array_element of ENVIRON passed\n\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                printf(
                    b"try_modify_environ: set_array_element of ENVIRON failed\n\0"
                        as *const u8 as *const libc::c_char,
                );
                ((*api).api_free).unwrap()(index.u.s.str_0 as *mut libc::c_void);
                ((*api).api_free).unwrap()(value.u.s.str_0 as *mut libc::c_void);
            }
            if ((*api).api_flatten_array_typed)
                .unwrap()(
                ext_id,
                environ_array,
                &mut flat_array,
                AWK_STRING,
                AWK_UNDEFINED,
            ) as u64 == 0
            {
                printf(
                    b"try_modify_environ: could not flatten array\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else if (*flat_array).count != count {
                printf(
                    b"try_modify_environ: flat_array->count (%lu) != count (%lu)\n\0"
                        as *const u8 as *const libc::c_char,
                    (*flat_array).count,
                    count,
                );
            } else {
                i = 0 as libc::c_int;
                while (i as libc::c_ulong) < (*flat_array).count {
                    if strcmp(
                        b"testext\0" as *const u8 as *const libc::c_char,
                        (*((*flat_array).elements).as_mut_ptr().offset(i as isize))
                            .index
                            .u
                            .s
                            .str_0,
                    ) == 0 as libc::c_int
                    {
                        let ref mut fresh1 = (*((*flat_array).elements)
                            .as_mut_ptr()
                            .offset(i as isize))
                            .flags;
                        *fresh1 = ::std::mem::transmute::<
                            libc::c_uint,
                            C2RustUnnamed_1,
                        >(
                            *fresh1 as libc::c_uint
                                | AWK_ELEMENT_DELETE as libc::c_int as libc::c_uint,
                        );
                        printf(
                            b"try_modify_environ: marking element \"%s\" for deletion\n\0"
                                as *const u8 as *const libc::c_char,
                            (*((*flat_array).elements).as_mut_ptr().offset(i as isize))
                                .index
                                .u
                                .s
                                .str_0,
                        );
                    }
                    i += 1;
                    i;
                }
                if ((*api).api_release_flattened_array)
                    .unwrap()(ext_id, environ_array, flat_array) as u64 == 0
                {
                    printf(
                        b"try_modify_environ: could not release flattened array\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    make_number(1.0f64, result);
                }
            }
        }
    } else {
        printf(
            b"try_modify_environ: sym_lookup of ENVIRON failed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return result;
}
unsafe extern "C" fn var_test(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut value2: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut valp: *mut awk_value_t = 0 as *mut awk_value_t;
    make_number(0.0f64, result);
    if nargs != 1 as libc::c_int {
        printf(
            b"var_test: nargs not right (%d should be 1)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else {
        if ((*api).api_sym_lookup)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            b"PROCINFO\0" as *const u8 as *const libc::c_char,
            AWK_ARRAY,
            &mut value,
        ) as u64 != 0
        {
            printf(
                b"var_test: sym_lookup of PROCINFO passed - got a value!\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            printf(
                b"var_test: sym_lookup of PROCINFO failed - did not get a value\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if ((*api).api_sym_lookup)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            b"ARGC\0" as *const u8 as *const libc::c_char,
            AWK_NUMBER,
            &mut value,
        ) as u64 != 0
        {
            printf(
                b"var_test: sym_lookup of ARGC passed - got a value!\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            printf(
                b"var_test: sym_lookup of ARGC failed - did not get a value\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        value.u.n.d += 1.;
        value.u.n.d;
        if ((*api).api_sym_update)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            b"ARGC\0" as *const u8 as *const libc::c_char,
            &mut value,
        ) as u64 != 0
        {
            printf(
                b"var_test: sym_update of ARGC passed and should not have!\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            printf(
                b"var_test: sym_update of ARGC failed - correctly\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if ((*api).api_get_argument)
            .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut value) as u64
            != 0
        {
            if ((*api).api_sym_lookup)
                .unwrap()(
                ext_id,
                b"\0" as *const u8 as *const libc::c_char,
                value.u.s.str_0,
                AWK_STRING,
                &mut value2,
            ) as u64 != 0
            {
                valp = make_number(42.0f64, &mut value2);
                if ((*api).api_sym_update)
                    .unwrap()(
                    ext_id,
                    b"\0" as *const u8 as *const libc::c_char,
                    value.u.s.str_0,
                    valp,
                ) as u64 != 0
                {
                    printf(
                        b"var_test: sym_update(\"%s\") succeeded\n\0" as *const u8
                            as *const libc::c_char,
                        value.u.s.str_0,
                    );
                    make_number(1.0f64, result);
                } else {
                    printf(
                        b"var_test: sym_update(\"%s\") failed\n\0" as *const u8
                            as *const libc::c_char,
                        value.u.s.str_0,
                    );
                }
            } else {
                printf(
                    b"var_test: sym_lookup(\"%s\") failed\n\0" as *const u8
                        as *const libc::c_char,
                    value.u.s.str_0,
                );
            }
        } else {
            printf(
                b"var_test: get_argument() failed\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return result;
}
unsafe extern "C" fn test_errno(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    make_number(0.0f64, result);
    if nargs != 0 as libc::c_int {
        printf(
            b"test_errno: nargs not right (%d should be 0)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else {
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, 10 as libc::c_int);
        make_number(1.0f64, result);
    }
    return result;
}
unsafe extern "C" fn test_deferred(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut current_block: u64;
    let mut arr: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let seed: [nval; 2] = [
        {
            let mut init = nval {
                name: b"fubar\0" as *const u8 as *const libc::c_char,
                val: 9.0f64,
            };
            init
        },
        {
            let mut init = nval {
                name: b"rumpus\0" as *const u8 as *const libc::c_char,
                val: -5.0f64,
            };
            init
        },
    ];
    let mut sysval: [nval; 2] = [
        {
            let mut init = nval {
                name: b"uid\0" as *const u8 as *const libc::c_char,
                val: getuid() as libc::c_double,
            };
            init
        },
        {
            let mut init = nval {
                name: b"api_major\0" as *const u8 as *const libc::c_char,
                val: GAWK_API_MAJOR_VERSION as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut i: size_t = 0;
    make_number(0.0f64, result);
    if nargs != 0 as libc::c_int {
        printf(
            b"test_deferred: nargs not right (%d should be 0)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"PROCINFO\0" as *const u8 as *const libc::c_char,
        AWK_ARRAY,
        &mut arr,
    ) as u64 == 0
    {
        printf(
            b"test_deferred: %d: sym_lookup failed\n\0" as *const u8
                as *const libc::c_char,
            439 as libc::c_int,
        );
    } else {
        i = 0 as libc::c_int as size_t;
        loop {
            if !(i
                < (::std::mem::size_of::<[nval; 2]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<nval>() as libc::c_ulong))
            {
                current_block = 11050875288958768710;
                break;
            }
            r_make_string(
                api,
                ext_id,
                seed[i as usize].name,
                strlen(seed[i as usize].name),
                awk_true,
                &mut index,
            );
            make_number(seed[i as usize].val, &mut value);
            if ((*api).api_set_array_element)
                .unwrap()(ext_id, arr.u.a, &mut index, &mut value) as u64 == 0
            {
                printf(
                    b"test_deferred: %d: set_array_element(%s) failed\n\0" as *const u8
                        as *const libc::c_char,
                    447 as libc::c_int,
                    seed[i as usize].name,
                );
                current_block = 8458645229064041830;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        match current_block {
            8458645229064041830 => {}
            _ => {
                i = 0 as libc::c_int as size_t;
                loop {
                    if !(i
                        < (::std::mem::size_of::<[nval; 2]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<nval>() as libc::c_ulong,
                            ))
                    {
                        current_block = 7175849428784450219;
                        break;
                    }
                    r_make_string(
                        api,
                        ext_id,
                        seed[i as usize].name,
                        strlen(seed[i as usize].name),
                        awk_true,
                        &mut index,
                    );
                    make_null_string(&mut value);
                    if ((*api).api_get_array_element)
                        .unwrap()(ext_id, arr.u.a, &mut index, AWK_NUMBER, &mut value)
                        as u64 == 0
                    {
                        printf(
                            b"test_deferred: %d: get_array_element(%s) failed\n\0"
                                as *const u8 as *const libc::c_char,
                            457 as libc::c_int,
                            seed[i as usize].name,
                        );
                        current_block = 8458645229064041830;
                        break;
                    } else {
                        printf(
                            b"%s = %g\n\0" as *const u8 as *const libc::c_char,
                            seed[i as usize].name,
                            value.u.n.d,
                        );
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                match current_block {
                    8458645229064041830 => {}
                    _ => {
                        i = 0 as libc::c_int as size_t;
                        loop {
                            if !(i
                                < (::std::mem::size_of::<[nval; 2]>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<nval>() as libc::c_ulong,
                                    ))
                            {
                                current_block = 7172762164747879670;
                                break;
                            }
                            r_make_string(
                                api,
                                ext_id,
                                sysval[i as usize].name,
                                strlen(sysval[i as usize].name),
                                awk_true,
                                &mut index,
                            );
                            make_null_string(&mut value);
                            if ((*api).api_get_array_element)
                                .unwrap()(
                                ext_id,
                                arr.u.a,
                                &mut index,
                                AWK_NUMBER,
                                &mut value,
                            ) as u64 == 0
                            {
                                printf(
                                    b"test_deferred: %d: get_array_element(%s) failed\n\0"
                                        as *const u8 as *const libc::c_char,
                                    468 as libc::c_int,
                                    sysval[i as usize].name,
                                );
                                current_block = 8458645229064041830;
                                break;
                            } else {
                                printf(
                                    b"%s matches %d\n\0" as *const u8 as *const libc::c_char,
                                    sysval[i as usize].name,
                                    (value.u.n.d == sysval[i as usize].val) as libc::c_int,
                                );
                                i = i.wrapping_add(1);
                                i;
                            }
                        }
                        match current_block {
                            8458645229064041830 => {}
                            _ => {
                                make_number(1.0f64, result);
                            }
                        }
                    }
                }
            }
        }
    }
    return result;
}
unsafe extern "C" fn test_array_size(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut count: size_t = 0 as libc::c_int as size_t;
    make_number(0.0f64, result);
    if nargs != 1 as libc::c_int {
        printf(
            b"test_array_size: nargs not right (%d should be 1)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_ARRAY, &mut value) as u64 == 0
    {
        printf(
            b"test_array_size: get_argument failed\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if ((*api).api_get_element_count).unwrap()(ext_id, value.u.a, &mut count)
        as u64 == 0
    {
        printf(
            b"test_array_size: get_element_count failed\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(
            b"test_array_size: incoming size is %lu\n\0" as *const u8
                as *const libc::c_char,
            count,
        );
        if ((*api).api_clear_array).unwrap()(ext_id, value.u.a) as u64 == 0 {
            printf(
                b"test_array_size: clear_array failed\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            make_number(1.0f64, result);
        }
    }
    return result;
}
unsafe extern "C" fn test_array_elem(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut array: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut index2: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    make_number(0.0f64, result);
    if nargs != 2 as libc::c_int {
        printf(
            b"test_array_elem: nargs not right (%d should be 2)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_ARRAY, &mut array) as u64 == 0
    {
        printf(
            b"test_array_elem: get_argument 0 (array) failed\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_STRING, &mut index) as u64 == 0
    {
        printf(
            b"test_array_elem: get_argument 1 (index) failed\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        r_make_string(
            api,
            ext_id,
            index.u.s.str_0,
            index.u.s.len,
            awk_true,
            &mut index2,
        );
        if ((*api).api_get_array_element)
            .unwrap()(ext_id, array.u.a, &mut index2, AWK_UNDEFINED, &mut value) as u64
            == 0
        {
            printf(
                b"test_array_elem: get_array_element failed\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            printf(
                b"test_array_elem: a[\"%.*s\"] = %s\n\0" as *const u8
                    as *const libc::c_char,
                index.u.s.len as libc::c_int,
                index.u.s.str_0,
                valrep2str(&mut value),
            );
            make_number(42.0f64, &mut value);
            r_make_string(
                api,
                ext_id,
                index.u.s.str_0,
                index.u.s.len,
                awk_true,
                &mut index2,
            );
            if ((*api).api_set_array_element)
                .unwrap()(ext_id, array.u.a, &mut index2, &mut value) as u64 == 0
            {
                printf(
                    b"test_array_elem: set_array_element failed\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                r_make_string(
                    api,
                    ext_id,
                    b"5\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                    awk_true,
                    &mut index,
                );
                if ((*api).api_del_array_element).unwrap()(ext_id, array.u.a, &mut index)
                    as u64 == 0
                {
                    printf(
                        b"test_array_elem: del_array_element failed\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    r_make_string(
                        api,
                        ext_id,
                        b"7\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as size_t,
                        awk_true,
                        &mut index,
                    );
                    r_make_string(
                        api,
                        ext_id,
                        b"seven\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as size_t,
                        awk_true,
                        &mut value,
                    );
                    if ((*api).api_set_array_element)
                        .unwrap()(ext_id, array.u.a, &mut index, &mut value) as u64 == 0
                    {
                        printf(
                            b"test_array_elem: set_array_element failed\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        r_make_string(
                            api,
                            ext_id,
                            b"subarray\0" as *const u8 as *const libc::c_char,
                            8 as libc::c_int as size_t,
                            awk_true,
                            &mut index,
                        );
                        fill_in_array(&mut value);
                        if ((*api).api_set_array_element)
                            .unwrap()(ext_id, array.u.a, &mut index, &mut value) as u64
                            == 0
                        {
                            printf(
                                b"test_array_elem: set_array_element (subarray) failed\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        } else {
                            make_number(1.0f64, result);
                        }
                    }
                }
            }
        }
    }
    return result;
}
unsafe extern "C" fn test_array_param(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut new_array: awk_value_t = {
        let mut init = awk_value {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        };
        init
    };
    let mut arg0: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    make_number(0.0f64, result);
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_UNDEFINED, &mut arg0) as u64
        == 0
    {
        printf(
            b"test_array_param: could not get argument\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if arg0.val_type as libc::c_uint
        != AWK_UNDEFINED as libc::c_int as libc::c_uint
    {
        printf(
            b"test_array_param: argument is not undefined (%d)\n\0" as *const u8
                as *const libc::c_char,
            arg0.val_type as libc::c_uint,
        );
    } else {
        fill_in_array(&mut new_array);
        if ((*api).api_set_argument)
            .unwrap()(ext_id, 0 as libc::c_int as size_t, new_array.u.a) as u64 == 0
        {
            printf(
                b"test_array_param: could not change type of argument\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            make_number(1.0f64, result);
        }
    }
    return result;
}
unsafe extern "C" fn test_array_create(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut new_array: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut arg0: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    make_number(0.0f64, result);
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut arg0) as u64 == 0
    {
        printf(
            b"test_array_create: could not get argument\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if arg0.val_type as libc::c_uint != AWK_STRING as libc::c_int as libc::c_uint
    {
        printf(
            b"test_array_create: argument is not string (%d)\n\0" as *const u8
                as *const libc::c_char,
            arg0.val_type as libc::c_uint,
        );
    } else {
        new_array.val_type = AWK_ARRAY;
        new_array.u.a = ((*api).api_create_array).unwrap()(ext_id);
        if ((*api).api_sym_update)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            arg0.u.s.str_0,
            &mut new_array,
        ) as u64 == 0
        {
            printf(
                b"test_array_create: sym_update(\"%s\") failed!\n\0" as *const u8
                    as *const libc::c_char,
                arg0.u.s.str_0,
            );
        } else if populate_array(new_array.u.a) < 0 as libc::c_int {
            printf(
                b"test_array_create: populate(\"%s\") failed!\n\0" as *const u8
                    as *const libc::c_char,
                arg0.u.s.str_0,
            );
        } else {
            make_number(1.0f64, result);
        }
    }
    return result;
}
unsafe extern "C" fn print_do_lint(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    make_number(0.0f64, result);
    if nargs != 0 as libc::c_int {
        printf(
            b"print_do_lint: nargs not right (%d should be 0)\n\0" as *const u8
                as *const libc::c_char,
            nargs,
        );
    } else {
        printf(
            b"print_do_lint: lint = %d\n\0" as *const u8 as *const libc::c_char,
            (*api).do_flags[0 as libc::c_int as usize],
        );
        make_number(1.0f64, result);
    }
    return result;
}
unsafe extern "C" fn test_scalar(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut new_value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut new_value2: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut the_scalar: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    make_number(0.0f64, result);
    if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"the_scalar\0" as *const u8 as *const libc::c_char,
        AWK_SCALAR,
        &mut the_scalar,
    ) as u64 == 0
    {
        printf(
            b"test_scalar: could not get scalar cookie\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_UNDEFINED, &mut new_value)
        as u64 == 0
    {
        printf(
            b"test_scalar: could not get argument\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if new_value.val_type as libc::c_uint
        != AWK_STRING as libc::c_int as libc::c_uint
        && new_value.val_type as libc::c_uint
            != AWK_NUMBER as libc::c_int as libc::c_uint
    {
        printf(
            b"test_scalar: argument is not a scalar\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        if new_value.val_type as libc::c_uint
            == AWK_STRING as libc::c_int as libc::c_uint
        {
            r_make_string(
                api,
                ext_id,
                new_value.u.s.str_0,
                new_value.u.s.len,
                awk_true,
                &mut new_value2,
            );
        } else {
            new_value2 = new_value;
        }
        if ((*api).api_sym_update_scalar)
            .unwrap()(ext_id, the_scalar.u.scl, &mut new_value2) as u64 == 0
        {
            printf(
                b"test_scalar: could not update new_value2!\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            make_number(1.0f64, result);
        }
    }
    return result;
}
unsafe extern "C" fn test_scalar_reserved(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut new_value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut the_scalar: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    make_number(0.0f64, result);
    if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"ARGC\0" as *const u8 as *const libc::c_char,
        AWK_SCALAR,
        &mut the_scalar,
    ) as u64 != 0
    {
        printf(
            b"test_scalar_reserved: sym_lookup of ARGC passed - got a value!\n\0"
                as *const u8 as *const libc::c_char,
        );
        make_number(42.0f64, &mut new_value);
        if ((*api).api_sym_update_scalar)
            .unwrap()(ext_id, the_scalar.u.scl, &mut new_value) as u64 == 0
        {
            printf(
                b"test_scalar_reserved: could not update new_value2 for ARGC - pass\n\0"
                    as *const u8 as *const libc::c_char,
            );
            make_number(1.0f64, result);
        } else {
            printf(
                b"test_scalar_reserved: was able to update new_value2 for ARGC - fail\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else {
        printf(
            b"test_scalar_reserved: sym_lookup of ARGC failed - did not get a value\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return result;
}
unsafe extern "C" fn test_indirect_vars(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut name: *mut libc::c_char = b"NR\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    make_number(0.0f64, result);
    unlink(b"testexttmp.txt\0" as *const u8 as *const libc::c_char);
    if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        name,
        AWK_NUMBER,
        &mut value,
    ) as u64 != 0
    {
        printf(
            b"test_indirect_var: sym_lookup of %s passed\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
        printf(
            b"test_indirect_var: value of NR is %g\n\0" as *const u8
                as *const libc::c_char,
            value.u.n.d,
        );
        make_number(1.0f64, result);
    } else {
        printf(
            b"test_indirect_var: sym_lookup of %s failed\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
    }
    return result;
}
unsafe extern "C" fn test_get_file(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut filename: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut alias: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut fd: libc::c_int = 0;
    let mut ibuf: *const awk_input_buf_t = 0 as *const awk_input_buf_t;
    let mut obuf: *const awk_output_buf_t = 0 as *const awk_output_buf_t;
    if nargs != 2 as libc::c_int {
        printf(
            b"%s: nargs not right (%d should be 2)\n\0" as *const u8
                as *const libc::c_char,
            b"test_get_file\0" as *const u8 as *const libc::c_char,
            nargs,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut filename) as u64
        == 0
    {
        printf(
            b"%s: cannot get first arg\n\0" as *const u8 as *const libc::c_char,
            b"test_get_file\0" as *const u8 as *const libc::c_char,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_STRING, &mut alias) as u64 == 0
    {
        printf(
            b"%s: cannot get second arg\n\0" as *const u8 as *const libc::c_char,
            b"test_get_file\0" as *const u8 as *const libc::c_char,
        );
        return make_number(-1.0f64, result);
    }
    fd = open(filename.u.s.str_0, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        printf(
            b"%s: open(%s) failed\n\0" as *const u8 as *const libc::c_char,
            b"test_get_file\0" as *const u8 as *const libc::c_char,
            filename.u.s.str_0,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_file)
        .unwrap()(
        ext_id,
        alias.u.s.str_0,
        strlen(alias.u.s.str_0),
        b"<\0" as *const u8 as *const libc::c_char,
        fd,
        &mut ibuf,
        &mut obuf,
    ) as u64 == 0
    {
        printf(
            b"%s: get_file(%s) failed\n\0" as *const u8 as *const libc::c_char,
            b"test_get_file\0" as *const u8 as *const libc::c_char,
            alias.u.s.str_0,
        );
        return make_number(-1.0f64, result);
    }
    if ibuf.is_null() || (*ibuf).fd != fd {
        printf(
            b"%s: get_file(%s) returned fd %d instead of %d\n\0" as *const u8
                as *const libc::c_char,
            b"test_get_file\0" as *const u8 as *const libc::c_char,
            alias.u.s.str_0,
            if !ibuf.is_null() { (*ibuf).fd } else { -(1 as libc::c_int) },
            fd,
        );
        return make_number(-1.0f64, result);
    }
    return make_number(0.0f64, result);
}
unsafe extern "C" fn do_get_file(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut filename: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut filetype: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut fd: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut res: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut ibuf: *const awk_input_buf_t = 0 as *const awk_input_buf_t;
    let mut obuf: *const awk_output_buf_t = 0 as *const awk_output_buf_t;
    if nargs != 4 as libc::c_int {
        printf(
            b"%s: nargs not right (%d should be 4)\n\0" as *const u8
                as *const libc::c_char,
            b"get_file\0" as *const u8 as *const libc::c_char,
            nargs,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut filename) as u64
        == 0
    {
        printf(
            b"%s: cannot get first arg\n\0" as *const u8 as *const libc::c_char,
            b"get_file\0" as *const u8 as *const libc::c_char,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_STRING, &mut filetype) as u64
        == 0
    {
        printf(
            b"%s: cannot get second arg\n\0" as *const u8 as *const libc::c_char,
            b"get_file\0" as *const u8 as *const libc::c_char,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 2 as libc::c_int as size_t, AWK_NUMBER, &mut fd) as u64 == 0
    {
        printf(
            b"%s: cannot get third arg\n\0" as *const u8 as *const libc::c_char,
            b"get_file\0" as *const u8 as *const libc::c_char,
        );
        return make_number(-1.0f64, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 3 as libc::c_int as size_t, AWK_ARRAY, &mut res) as u64 == 0
    {
        printf(
            b"%s: cannot get fourth arg\n\0" as *const u8 as *const libc::c_char,
            b"get_file\0" as *const u8 as *const libc::c_char,
        );
        return make_number(-1.0f64, result);
    }
    ((*api).api_clear_array).unwrap()(ext_id, res.u.a);
    if ((*api).api_get_file)
        .unwrap()(
        ext_id,
        filename.u.s.str_0,
        strlen(filename.u.s.str_0),
        filetype.u.s.str_0,
        fd.u.n.d as libc::c_int,
        &mut ibuf,
        &mut obuf,
    ) as u64 == 0
    {
        printf(
            b"%s: get_file(%s, %s, %d) failed\n\0" as *const u8 as *const libc::c_char,
            b"get_file\0" as *const u8 as *const libc::c_char,
            filename.u.s.str_0,
            filetype.u.s.str_0,
            fd.u.n.d as libc::c_int,
        );
        return make_number(0.0f64, result);
    }
    if !ibuf.is_null() {
        let mut idx: awk_value_t = awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        };
        let mut val: awk_value_t = awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        };
        ((*api).api_set_array_element)
            .unwrap()(
            ext_id,
            res.u.a,
            r_make_string(
                api,
                ext_id,
                b"input\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as size_t,
                awk_true,
                &mut idx,
            ),
            make_number((*ibuf).fd as libc::c_double, &mut val),
        );
        if !((*ibuf).name).is_null() {
            ((*api).api_set_array_element)
                .unwrap()(
                ext_id,
                res.u.a,
                r_make_string(
                    api,
                    ext_id,
                    b"input_name\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as size_t,
                    awk_true,
                    &mut idx,
                ),
                r_make_string(
                    api,
                    ext_id,
                    (*ibuf).name,
                    strlen((*ibuf).name),
                    awk_true,
                    &mut val,
                ),
            );
        }
    }
    if !obuf.is_null() {
        let mut idx_0: awk_value_t = awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        };
        let mut val_0: awk_value_t = awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        };
        ((*api).api_set_array_element)
            .unwrap()(
            ext_id,
            res.u.a,
            r_make_string(
                api,
                ext_id,
                b"output\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
                awk_true,
                &mut idx_0,
            ),
            make_number(
                (if !((*obuf).fp).is_null() {
                    fileno((*obuf).fp)
                } else {
                    -(1 as libc::c_int)
                }) as libc::c_double,
                &mut val_0,
            ),
        );
        if !((*obuf).name).is_null() {
            ((*api).api_set_array_element)
                .unwrap()(
                ext_id,
                res.u.a,
                r_make_string(
                    api,
                    ext_id,
                    b"output_name\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as size_t,
                    awk_true,
                    &mut idx_0,
                ),
                r_make_string(
                    api,
                    ext_id,
                    (*obuf).name,
                    strlen((*obuf).name),
                    awk_true,
                    &mut val_0,
                ),
            );
        }
    }
    return make_number(1.0f64, result);
}
unsafe extern "C" fn populate_array(mut a_cookie: awk_array_t) -> libc::c_int {
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    r_make_string(
        api,
        ext_id,
        b"hello\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
        awk_true,
        &mut index,
    );
    r_make_string(
        api,
        ext_id,
        b"world\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
        awk_true,
        &mut value,
    );
    if ((*api).api_set_array_element).unwrap()(ext_id, a_cookie, &mut index, &mut value)
        as u64 == 0
    {
        printf(
            b"fill_in_array:%d: set_array_element failed\n\0" as *const u8
                as *const libc::c_char,
            1050 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    r_make_string(
        api,
        ext_id,
        b"answer\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
        awk_true,
        &mut index,
    );
    make_number(42.0f64, &mut value);
    if ((*api).api_set_array_element).unwrap()(ext_id, a_cookie, &mut index, &mut value)
        as u64 == 0
    {
        printf(
            b"fill_in_array:%d: set_array_element failed\n\0" as *const u8
                as *const libc::c_char,
            1057 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fill_in_array(mut new_array: *mut awk_value_t) {
    let mut a_cookie: awk_array_t = 0 as *mut libc::c_void;
    a_cookie = ((*api).api_create_array).unwrap()(ext_id);
    if populate_array(a_cookie) < 0 as libc::c_int {
        return;
    }
    (*new_array).val_type = AWK_ARRAY;
    (*new_array).u.a = a_cookie;
}
unsafe extern "C" fn create_new_array() {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    fill_in_array(&mut value);
    if ((*api).api_sym_update)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"new_array\0" as *const u8 as *const libc::c_char,
        &mut value,
    ) as u64 == 0
    {
        printf(
            b"create_new_array: sym_update(\"new_array\") failed!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn at_exit0(
    mut data: *mut libc::c_void,
    mut exit_status: libc::c_int,
) {
    printf(b"at_exit0 called (should be third):\0" as *const u8 as *const libc::c_char);
    if !data.is_null() {
        printf(b" data = %p,\0" as *const u8 as *const libc::c_char, data);
    } else {
        printf(b" data = NULL,\0" as *const u8 as *const libc::c_char);
    }
    printf(b" exit_status = %d\n\0" as *const u8 as *const libc::c_char, exit_status);
}
static mut data_for_1: libc::c_int = 0xdeadbeef as libc::c_uint as libc::c_int;
unsafe extern "C" fn at_exit1(
    mut data: *mut libc::c_void,
    mut exit_status: libc::c_int,
) {
    let mut data_p: *mut libc::c_int = data as *mut libc::c_int;
    printf(b"at_exit1 called (should be second):\0" as *const u8 as *const libc::c_char);
    if !data.is_null() {
        if data == &mut data_for_1 as *mut libc::c_int as *mut libc::c_void {
            printf(b" (data is & data_for_1),\0" as *const u8 as *const libc::c_char);
        } else {
            printf(
                b" (data is NOT & data_for_1),\0" as *const u8 as *const libc::c_char,
            );
        }
        printf(b" data value = %#x,\0" as *const u8 as *const libc::c_char, *data_p);
    } else {
        printf(b" data = NULL,\0" as *const u8 as *const libc::c_char);
    }
    printf(b" exit_status = %d\n\0" as *const u8 as *const libc::c_char, exit_status);
}
unsafe extern "C" fn at_exit2(
    mut data: *mut libc::c_void,
    mut exit_status: libc::c_int,
) {
    printf(b"at_exit2 called (should be first):\0" as *const u8 as *const libc::c_char);
    if !data.is_null() {
        printf(b" data = %p,\0" as *const u8 as *const libc::c_char, data);
    } else {
        printf(b" data = NULL,\0" as *const u8 as *const libc::c_char);
    }
    printf(b" exit_status = %d\n\0" as *const u8 as *const libc::c_char, exit_status);
}
unsafe extern "C" fn do_test_function(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    printf(b"test::test_function() called.\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    return make_number(0.0f64, result);
}
static mut func_table: [awk_ext_func_t; 15] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"dump_array_and_delete\0" as *const u8 as *const libc::c_char,
                function: Some(
                    dump_array_and_delete
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 2 as libc::c_int as size_t,
                min_required_args: 2 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"try_modify_environ\0" as *const u8 as *const libc::c_char,
                function: Some(
                    try_modify_environ
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as libc::c_int as size_t,
                min_required_args: 0 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"var_test\0" as *const u8 as *const libc::c_char,
                function: Some(
                    var_test
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as libc::c_int as size_t,
                min_required_args: 1 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_deferred\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_deferred
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as libc::c_int as size_t,
                min_required_args: 0 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_errno\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_errno
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as libc::c_int as size_t,
                min_required_args: 0 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_array_size\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_array_size
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as libc::c_int as size_t,
                min_required_args: 1 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_array_elem\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_array_elem
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 2 as libc::c_int as size_t,
                min_required_args: 2 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_array_param\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_array_param
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as libc::c_int as size_t,
                min_required_args: 1 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_array_create\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_array_create
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as libc::c_int as size_t,
                min_required_args: 1 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"print_do_lint\0" as *const u8 as *const libc::c_char,
                function: Some(
                    print_do_lint
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as libc::c_int as size_t,
                min_required_args: 0 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_scalar\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_scalar
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as libc::c_int as size_t,
                min_required_args: 1 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_scalar_reserved\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_scalar_reserved
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as libc::c_int as size_t,
                min_required_args: 0 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_indirect_vars\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_indirect_vars
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as libc::c_int as size_t,
                min_required_args: 0 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"test_get_file\0" as *const u8 as *const libc::c_char,
                function: Some(
                    test_get_file
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 2 as libc::c_int as size_t,
                min_required_args: 2 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"get_file\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_get_file
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 4 as libc::c_int as size_t,
                min_required_args: 4 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
static mut ns_test_func: awk_ext_func_t = unsafe {
    {
        let mut init = awk_ext_func {
            name: b"test_function\0" as *const u8 as *const libc::c_char,
            function: Some(
                do_test_function
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut awk_value_t,
                        *mut awk_ext_func,
                    ) -> *mut awk_value_t,
            ),
            max_expected_args: 0 as libc::c_int as size_t,
            min_required_args: 0 as libc::c_int as size_t,
            suppress_lint: awk_false,
            data: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn init_testext() -> awk_bool_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    static mut message: [libc::c_char; 13] = unsafe {
        *::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"hello, world\0")
    };
    static mut message2: [libc::c_char; 14] = unsafe {
        *::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"i am a scalar\0")
    };
    static mut message3: [libc::c_char; 18] = unsafe {
        *::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"in namespace test\0")
    };
    if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"TESTEXT_QUIET\0" as *const u8 as *const libc::c_char,
        AWK_NUMBER,
        &mut value,
    ) as u64 != 0
    {
        return awk_true;
    }
    ((*api).api_awk_atexit)
        .unwrap()(
        ext_id,
        Some(at_exit0 as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
        0 as *mut libc::c_void,
    );
    ((*api).api_awk_atexit)
        .unwrap()(
        ext_id,
        Some(at_exit1 as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
        &mut data_for_1 as *mut libc::c_int as *mut libc::c_void,
    );
    ((*api).api_awk_atexit)
        .unwrap()(
        ext_id,
        Some(at_exit2 as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
        0 as *mut libc::c_void,
    );
    if ((*api).api_sym_update)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"answer_num\0" as *const u8 as *const libc::c_char,
        make_number(42 as libc::c_int as libc::c_double, &mut value),
    ) as u64 == 0
    {
        printf(
            b"testext: sym_update(\"answer_num\") failed!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ((*api).api_sym_update)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"message_string\0" as *const u8 as *const libc::c_char,
        r_make_string(
            api,
            ext_id,
            message.as_ptr(),
            strlen(message.as_ptr()),
            awk_true,
            &mut value,
        ),
    ) as u64 == 0
    {
        printf(
            b"testext: sym_update(\"answer_num\") failed!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ((*api).api_sym_update)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"the_scalar\0" as *const u8 as *const libc::c_char,
        r_make_string(
            api,
            ext_id,
            message2.as_ptr(),
            strlen(message2.as_ptr()),
            awk_true,
            &mut value,
        ),
    ) as u64 == 0
    {
        printf(
            b"testext: sym_update(\"the_scalar\") failed!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    create_new_array();
    if ((*api).api_sym_update)
        .unwrap()(
        ext_id,
        b"test\0" as *const u8 as *const libc::c_char,
        b"testval\0" as *const u8 as *const libc::c_char,
        r_make_string(
            api,
            ext_id,
            message3.as_ptr(),
            strlen(message3.as_ptr()),
            awk_true,
            &mut value,
        ),
    ) as u64 == 0
    {
        printf(
            b"testext: sym_update_ns(\"test\", \"testval\") failed!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ((*api).api_add_ext_func)
        .unwrap()(
        ext_id,
        b"test\0" as *const u8 as *const libc::c_char,
        &mut ns_test_func,
    ) as u64 == 0
    {
        printf(
            b"testext: add_ext_func(\"test\", ns_test_func) failed!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return awk_true;
}
static mut init_func: Option::<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_testext as unsafe extern "C" fn() -> awk_bool_t)
};
pub unsafe extern "C" fn dl_load(
    api_p: *const gawk_api_t,
    mut id: awk_ext_id_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut errors: libc::c_int = 0 as libc::c_int;
    api = api_p;
    ext_id = id as *mut *mut libc::c_void as awk_ext_id_t;
    if (*api).major_version != GAWK_API_MAJOR_VERSION as libc::c_int
        || (*api).minor_version < GAWK_API_MINOR_VERSION as libc::c_int
    {
        fprintf(
            stderr,
            b"testext: version mismatch with gawk!\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"\tmy version (API %d.%d), gawk version (API %d.%d)\n\0" as *const u8
                as *const libc::c_char,
            GAWK_API_MAJOR_VERSION as libc::c_int,
            GAWK_API_MINOR_VERSION as libc::c_int,
            (*api).major_version,
            (*api).minor_version,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    j = (::std::mem::size_of::<[awk_ext_func_t; 15]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<awk_ext_func_t>() as libc::c_ulong);
    while i < j {
        if (func_table[i as usize].name).is_null() {
            break;
        }
        if ((*api).api_add_ext_func)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            &mut *func_table.as_mut_ptr().offset(i as isize),
        ) as u64 == 0
        {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                b"testext: could not add %s\0" as *const u8 as *const libc::c_char,
                func_table[i as usize].name,
            );
            errors += 1;
            errors;
        }
        i = i.wrapping_add(1);
        i;
    }
    if init_func.is_some() {
        if init_func.unwrap()() as u64 == 0 {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                b"testext: initialization function failed\0" as *const u8
                    as *const libc::c_char,
            );
            errors += 1;
            errors;
        }
    }
    if !ext_version.is_null() {
        ((*api).api_register_ext_version).unwrap()(ext_id, ext_version);
    }
    return (errors == 0 as libc::c_int) as libc::c_int;
}

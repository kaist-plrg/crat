use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
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
pub type value_storage = libc::c_int;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
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
static mut ext_version: *const libc::c_char = b"rwarray extension: version 2.1\0"
    as *const u8 as *const libc::c_char;
static mut init_func: Option::<unsafe extern "C" fn() -> awk_bool_t> = None;
pub static mut plugin_is_GPL_compatible: libc::c_int = 0;
unsafe extern "C" fn write_backend(
    mut result: *mut awk_value_t,
    mut array: awk_array_t,
    mut name: *const libc::c_char,
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
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut major: uint32_t = 4 as libc::c_int as uint32_t;
    let mut minor: uint32_t = 1 as libc::c_int as uint32_t;
    make_number(0.0f64, result);
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut filename) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: first argument is not a string\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        *__errno_location() = 22 as libc::c_int;
    } else {
        fp = fopen(filename.u.s.str_0, b"wb\0" as *const u8 as *const libc::c_char);
        if !fp.is_null() {
            if !(fwrite(
                b"awkrulz\n\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                strlen(b"awkrulz\n\0" as *const u8 as *const libc::c_char),
                fp,
            ) != strlen(b"awkrulz\n\0" as *const u8 as *const libc::c_char))
            {
                major = __bswap_32(major);
                if !(fwrite(
                    &mut major as *mut uint32_t as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                    fp,
                ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                {
                    minor = __bswap_32(minor);
                    if !(fwrite(
                        &mut minor as *mut uint32_t as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                        fp,
                    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    {
                        if write_array(fp, array) as u64 != 0 {
                            make_number(1.0f64, result);
                            fclose(fp);
                            return result;
                        }
                    }
                }
            }
        }
    }
    ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
    if !fp.is_null() {
        fclose(fp);
        unlink(filename.u.s.str_0);
    }
    return result;
}
unsafe extern "C" fn do_writea(
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
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_ARRAY, &mut array) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"writea: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        *__errno_location() = 22 as libc::c_int;
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        make_number(0.0f64, result);
        return result;
    }
    return write_backend(
        result,
        array.u.a,
        b"writea\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn do_writeall(
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
    if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"SYMTAB\0" as *const u8 as *const libc::c_char,
        AWK_ARRAY,
        &mut array,
    ) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"writeall: unable to find SYMTAB array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        *__errno_location() = 22 as libc::c_int;
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        make_number(0.0f64, result);
        return result;
    }
    return write_backend(
        result,
        array.u.a,
        b"writeall\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn write_array(
    mut fp: *mut FILE,
    mut array: awk_array_t,
) -> awk_bool_t {
    let mut i: uint32_t = 0;
    let mut count: uint32_t = 0;
    let mut flat_array: *mut awk_flat_array_t = 0 as *mut awk_flat_array_t;
    if ((*api).api_flatten_array_typed)
        .unwrap()(ext_id, array, &mut flat_array, AWK_STRING, AWK_UNDEFINED) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"write_array: could not flatten array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return awk_false;
    }
    count = __bswap_32((*flat_array).count as __uint32_t);
    if fwrite(
        &mut count as *mut uint32_t as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        return awk_false;
    }
    i = 0 as libc::c_int as uint32_t;
    while (i as libc::c_ulong) < (*flat_array).count {
        if write_elem(fp, &mut *((*flat_array).elements).as_mut_ptr().offset(i as isize))
            as u64 == 0
        {
            ((*api).api_release_flattened_array).unwrap()(ext_id, array, flat_array);
            return awk_false;
        }
        i = i.wrapping_add(1);
        i;
    }
    if ((*api).api_release_flattened_array).unwrap()(ext_id, array, flat_array) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"write_array: could not release flattened array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return awk_false;
    }
    return awk_true;
}
unsafe extern "C" fn write_elem(
    mut fp: *mut FILE,
    mut element: *mut awk_element_t,
) -> awk_bool_t {
    let mut indexval_len: uint32_t = 0;
    let mut write_count: ssize_t = 0;
    indexval_len = __bswap_32((*element).index.u.s.len as __uint32_t);
    if fwrite(
        &mut indexval_len as *mut uint32_t as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        return awk_false;
    }
    if (*element).index.u.s.len > 0 as libc::c_int as libc::c_ulong {
        write_count = fwrite(
            (*element).index.u.s.str_0 as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (*element).index.u.s.len,
            fp,
        ) as ssize_t;
        if write_count != (*element).index.u.s.len as ssize_t {
            return awk_false;
        }
    }
    return write_value(fp, &mut (*element).value);
}
unsafe extern "C" fn write_value(
    mut fp: *mut FILE,
    mut val: *mut awk_value_t,
) -> awk_bool_t {
    let mut code: uint32_t = 0;
    let mut len: uint32_t = 0;
    if (*val).val_type as libc::c_uint == AWK_ARRAY as libc::c_int as libc::c_uint {
        code = __bswap_32(5 as libc::c_int as __uint32_t);
        if fwrite(
            &mut code as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        return write_array(fp, (*val).u.a);
    }
    if (*val).val_type as libc::c_uint == AWK_NUMBER as libc::c_int as libc::c_uint {
        return write_number(fp, val);
    }
    match (*val).val_type as libc::c_uint {
        2 => {
            code = __bswap_32(1 as libc::c_int as __uint32_t);
        }
        4 => {
            code = __bswap_32(7 as libc::c_int as __uint32_t);
        }
        3 => {
            code = __bswap_32(6 as libc::c_int as __uint32_t);
        }
        8 => {
            code = __bswap_32(8 as libc::c_int as __uint32_t);
        }
        0 => {
            code = __bswap_32(20 as libc::c_int as __uint32_t);
        }
        _ => {
            code = __bswap_32(20 as libc::c_int as __uint32_t);
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"array value has unknown type %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*val).val_type as libc::c_uint,
            );
        }
    }
    if fwrite(
        &mut code as *mut uint32_t as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        return awk_false;
    }
    if code == __bswap_32(8 as libc::c_int as __uint32_t) {
        len = (if (*val).u.b as libc::c_uint == awk_true as libc::c_int as libc::c_uint {
            4 as libc::c_int
        } else {
            5 as libc::c_int
        }) as uint32_t;
        len = __bswap_32(len);
        let mut s: *const libc::c_char = if (*val).u.b as libc::c_uint
            == awk_true as libc::c_int as libc::c_uint
        {
            b"TRUE\0" as *const u8 as *const libc::c_char
        } else {
            b"FALSE\0" as *const u8 as *const libc::c_char
        };
        if fwrite(
            &mut len as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        if fwrite(
            s as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            strlen(s),
            fp,
        ) != strlen(s) as ssize_t as libc::c_ulong
        {
            return awk_false;
        }
    } else {
        len = __bswap_32((*val).u.s.len as __uint32_t);
        if fwrite(
            &mut len as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        if fwrite(
            (*val).u.s.str_0 as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (*val).u.s.len,
            fp,
        ) != (*val).u.s.len as ssize_t as libc::c_ulong
        {
            return awk_false;
        }
    }
    return awk_true;
}
unsafe extern "C" fn write_number(
    mut fp: *mut FILE,
    mut val: *mut awk_value_t,
) -> awk_bool_t {
    let mut len: uint32_t = 0;
    let mut code: uint32_t = 0;
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    if (*val).u.n.type_0 as libc::c_uint
        == AWK_NUMBER_TYPE_DOUBLE as libc::c_int as libc::c_uint
    {
        let mut network_order_len: uint32_t = 0;
        code = __bswap_32(2 as libc::c_int as __uint32_t);
        if fwrite(
            &mut code as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        sprintf(
            buffer.as_mut_ptr(),
            b"%.17g\0" as *const u8 as *const libc::c_char,
            (*val).u.n.d,
        );
        len = (strlen(buffer.as_mut_ptr()))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as uint32_t;
        network_order_len = __bswap_32(len);
        if fwrite(
            &mut network_order_len as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        if fwrite(
            buffer.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as libc::c_ulong,
            fp,
        ) != len as libc::c_ulong
        {
            return awk_false;
        }
    } else {
        ((*api).api_fatal)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"rwarray extension: received GMP/MPFR value but compiled without GMP/MPFR support.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return awk_true;
}
unsafe extern "C" fn free_value(mut v: *mut awk_value_t) {
    match (*v).val_type as libc::c_uint {
        5 => {
            ((*api).api_destroy_array).unwrap()(ext_id, (*v).u.a);
        }
        2 | 3 | 4 | 0 => {
            ((*api).api_free).unwrap()((*v).u.s.str_0 as *mut libc::c_void);
        }
        8 => {}
        1 => {
            match (*v).u.n.type_0 as libc::c_uint {
                0 => {}
                _ => {
                    ((*api).api_warning)
                        .unwrap()(
                        ext_id,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot free number with unknown type %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*v).u.n.type_0 as libc::c_uint,
                    );
                }
            }
        }
        _ => {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot free value with unhandled type %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*v).val_type as libc::c_uint,
            );
        }
    };
}
unsafe extern "C" fn do_poke(mut e: *mut awk_element_t) -> awk_bool_t {
    let mut t: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    if (*e).index.val_type as libc::c_uint != AWK_STRING as libc::c_int as libc::c_uint {
        return awk_false;
    }
    let mut p: *mut libc::c_char = strstr(
        (*e).index.u.s.str_0,
        b"::\0" as *const u8 as *const libc::c_char,
    );
    let mut ns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ident: *mut libc::c_char = 0 as *mut libc::c_char;
    if !p.is_null() {
        ns = (*e).index.u.s.str_0;
        ident = p.offset(2 as libc::c_int as isize);
        *p = '\0' as i32 as libc::c_char;
    } else {
        ns = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        ident = (*e).index.u.s.str_0;
    }
    if ((*api).api_sym_lookup).unwrap()(ext_id, ns, ident, AWK_UNDEFINED, &mut t)
        as libc::c_uint != 0
        && t.val_type as libc::c_uint != AWK_UNDEFINED as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    if ((*api).api_sym_update).unwrap()(ext_id, ns, ident, &mut (*e).value) as u64 == 0 {
        if *ns.offset(0 as libc::c_int as isize) != 0 {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"readall: unable to set %s::%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ns,
                ident,
            );
        } else {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"readall: unable to set %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ident,
            );
        }
        return awk_false;
    }
    return awk_true;
}
unsafe extern "C" fn read_global(
    mut fp: *mut FILE,
    mut unused: awk_array_t,
) -> awk_bool_t {
    let mut i: uint32_t = 0;
    let mut count: uint32_t = 0;
    let mut new_elem: awk_element_t = awk_element_t {
        next: 0 as *mut awk_element,
        flags: AWK_ELEMENT_DEFAULT,
        index: awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        },
        value: awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        },
    };
    let mut vs: value_storage = 0;
    if fread(
        &mut count as *mut uint32_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        return awk_false;
    }
    count = __bswap_32(count);
    i = 0 as libc::c_int as uint32_t;
    while i < count {
        if read_elem(fp, &mut new_elem, &mut vs) as u64 != 0 {
            if do_poke(&mut new_elem) as u64 == 0 {
                free_value(&mut new_elem.value);
            }
            if new_elem.index.u.s.len != 0 {
                ((*api).api_free)
                    .unwrap()(new_elem.index.u.s.str_0 as *mut libc::c_void);
            }
        } else {
            return awk_false
        }
        i = i.wrapping_add(1);
        i;
    }
    return awk_true;
}
unsafe extern "C" fn read_one(mut fp: *mut FILE, mut array: awk_array_t) -> awk_bool_t {
    if ((*api).api_clear_array).unwrap()(ext_id, array) as u64 == 0 {
        *__errno_location() = 12 as libc::c_int;
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"reada: clear_array failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return awk_false;
    }
    return read_array(fp, array);
}
unsafe extern "C" fn read_backend(
    mut result: *mut awk_value_t,
    mut array: awk_array_t,
    mut name: *const libc::c_char,
    mut func: Option::<unsafe extern "C" fn(*mut FILE, awk_array_t) -> awk_bool_t>,
) -> *mut awk_value_t {
    let mut current_block: u64;
    let mut filename: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut major: uint32_t = 0;
    let mut minor: uint32_t = 0;
    let mut magic_buf: [libc::c_char; 30] = [0; 30];
    make_number(0.0f64, result);
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut filename) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: first argument is not a string\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        *__errno_location() = 22 as libc::c_int;
        current_block = 15058651137498033851;
    } else {
        fp = fopen(filename.u.s.str_0, b"rb\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            current_block = 15058651137498033851;
        } else {
            memset(
                magic_buf.as_mut_ptr() as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            );
            if fread(
                magic_buf.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                strlen(b"awkrulz\n\0" as *const u8 as *const libc::c_char),
                fp,
            ) != strlen(b"awkrulz\n\0" as *const u8 as *const libc::c_char)
            {
                *__errno_location() = 9 as libc::c_int;
                current_block = 15058651137498033851;
            } else if strcmp(
                magic_buf.as_mut_ptr(),
                b"awkrulz\n\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                *__errno_location() = 9 as libc::c_int;
                current_block = 15058651137498033851;
            } else if fread(
                &mut major as *mut uint32_t as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                fp,
            ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
            {
                *__errno_location() = 9 as libc::c_int;
                current_block = 15058651137498033851;
            } else {
                major = __bswap_32(major);
                if major != 4 as libc::c_int as libc::c_uint {
                    *__errno_location() = 9 as libc::c_int;
                    current_block = 15058651137498033851;
                } else if fread(
                    &mut minor as *mut uint32_t as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                    fp,
                ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
                {
                    current_block = 15058651137498033851;
                } else {
                    minor = __bswap_32(minor);
                    if minor != 1 as libc::c_int as libc::c_uint {
                        *__errno_location() = 9 as libc::c_int;
                        current_block = 15058651137498033851;
                    } else if (Some(func.unwrap())).unwrap()(fp, array) as u64 != 0 {
                        make_number(1.0f64, result);
                        current_block = 788629987478170427;
                    } else {
                        current_block = 15058651137498033851;
                    }
                }
            }
        }
    }
    match current_block {
        15058651137498033851 => {
            ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        }
        _ => {}
    }
    if !fp.is_null() {
        fclose(fp);
    }
    return result;
}
unsafe extern "C" fn do_reada(
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
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_ARRAY, &mut array) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"reada: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        *__errno_location() = 22 as libc::c_int;
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        make_number(0.0f64, result);
        return result;
    }
    return read_backend(
        result,
        array.u.a,
        b"read\0" as *const u8 as *const libc::c_char,
        Some(read_one as unsafe extern "C" fn(*mut FILE, awk_array_t) -> awk_bool_t),
    );
}
unsafe extern "C" fn do_readall(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    return read_backend(
        result,
        0 as *mut libc::c_void,
        b"readall\0" as *const u8 as *const libc::c_char,
        Some(read_global as unsafe extern "C" fn(*mut FILE, awk_array_t) -> awk_bool_t),
    );
}
unsafe extern "C" fn read_array(
    mut fp: *mut FILE,
    mut array: awk_array_t,
) -> awk_bool_t {
    let mut i: uint32_t = 0;
    let mut count: uint32_t = 0;
    let mut new_elem: awk_element_t = awk_element_t {
        next: 0 as *mut awk_element,
        flags: AWK_ELEMENT_DEFAULT,
        index: awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        },
        value: awk_value_t {
            val_type: AWK_UNDEFINED,
            u: C2RustUnnamed_0 {
                s: awk_string_t {
                    str_0: 0 as *mut libc::c_char,
                    len: 0,
                },
            },
        },
    };
    let mut vs: value_storage = 0;
    if fread(
        &mut count as *mut uint32_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        return awk_false;
    }
    count = __bswap_32(count);
    i = 0 as libc::c_int as uint32_t;
    while i < count {
        if !(read_elem(fp, &mut new_elem, &mut vs) as u64 != 0) {
            break;
        }
        if ((*api).api_set_array_element)
            .unwrap()(ext_id, array, &mut new_elem.index, &mut new_elem.value) as u64
            == 0
        {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"read_array: set_array_element failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return awk_false;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i != count {
        return awk_false;
    }
    return awk_true;
}
unsafe extern "C" fn read_elem(
    mut fp: *mut FILE,
    mut element: *mut awk_element_t,
    mut vs: *mut value_storage,
) -> awk_bool_t {
    let mut index_len: uint32_t = 0;
    static mut buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buflen: uint32_t = 0;
    let mut ret: ssize_t = 0;
    ret = fread(
        &mut index_len as *mut uint32_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) as ssize_t;
    if ret as libc::c_ulong != ::std::mem::size_of::<uint32_t>() as libc::c_ulong {
        return awk_false;
    }
    index_len = __bswap_32(index_len);
    memset(
        element as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<awk_element_t>() as libc::c_ulong,
    );
    if index_len > 0 as libc::c_int as libc::c_uint {
        if buffer.is_null() {
            buffer = ((*api).api_malloc).unwrap()(index_len as size_t)
                as *mut libc::c_char;
            if buffer.is_null() {
                ((*api).api_fatal)
                    .unwrap()(
                    ext_id,
                    b"%s: malloc of %d bytes failed\0" as *const u8
                        as *const libc::c_char,
                    b"read_elem\0" as *const u8 as *const libc::c_char,
                    index_len,
                );
            }
            buflen = index_len;
        } else if buflen < index_len {
            let mut cp: *mut libc::c_char = ((*api).api_realloc)
                .unwrap()(buffer as *mut libc::c_void, index_len as size_t)
                as *mut libc::c_char;
            if cp.is_null() {
                return awk_false;
            }
            buffer = cp;
            buflen = index_len;
        }
        if fread(
            buffer as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            index_len as libc::c_ulong,
            fp,
        ) != index_len as ssize_t as libc::c_ulong
        {
            return awk_false;
        }
        r_make_string(
            api,
            ext_id,
            buffer,
            index_len as size_t,
            awk_true,
            &mut (*element).index,
        );
    } else {
        make_null_string(&mut (*element).index);
    }
    if read_value(fp, &mut (*element).value, &mut (*element).index, vs) as u64 == 0 {
        return awk_false;
    }
    return awk_true;
}
unsafe extern "C" fn read_value(
    mut fp: *mut FILE,
    mut value: *mut awk_value_t,
    mut idx: *mut awk_value_t,
    mut vs: *mut value_storage,
) -> awk_bool_t {
    let mut code: uint32_t = 0;
    let mut len: uint32_t = 0;
    if fread(
        &mut code as *mut uint32_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        fp,
    ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
    {
        return awk_false;
    }
    code = __bswap_32(code);
    if code == 5 as libc::c_int as libc::c_uint {
        let mut array: awk_array_t = ((*api).api_create_array).unwrap()(ext_id);
        if read_array(fp, array) as u64 == 0 {
            return awk_false;
        }
        (*value).val_type = AWK_ARRAY;
        (*value).u.a = array;
    } else if code == 2 as libc::c_int as libc::c_uint
        || code == 3 as libc::c_int as libc::c_uint
        || code == 4 as libc::c_int as libc::c_uint
    {
        return read_number(fp, value, code, vs)
    } else {
        if fread(
            &mut len as *mut uint32_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        len = __bswap_32(len);
        match code {
            1 => {
                (*value).val_type = AWK_STRING;
            }
            6 => {
                (*value).val_type = AWK_REGEX;
            }
            7 => {
                (*value).val_type = AWK_STRNUM;
            }
            20 => {
                (*value).val_type = AWK_UNDEFINED;
            }
            8 => {
                (*value).val_type = AWK_BOOL;
            }
            _ => {
                ((*api).api_warning)
                    .unwrap()(
                    ext_id,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"treating recovered value with unknown type code %d as a string\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    code,
                );
                (*value).val_type = AWK_STRING;
            }
        }
        (*value).u.s.len = len as size_t;
        (*value)
            .u
            .s
            .str_0 = ((*api).api_malloc)
            .unwrap()(len.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t)
            as *mut libc::c_char;
        if fread(
            (*value).u.s.str_0 as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as libc::c_ulong,
            fp,
        ) != len as ssize_t as libc::c_ulong
        {
            ((*api).api_free).unwrap()((*value).u.s.str_0 as *mut libc::c_void);
            return awk_false;
        }
        *((*value).u.s.str_0).offset(len as isize) = '\0' as i32 as libc::c_char;
        (*value).u.s.len = len as size_t;
        if code == 8 as libc::c_int as libc::c_uint {
            let mut val: bool = strcmp(
                (*value).u.s.str_0,
                b"TRUE\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int;
            ((*api).api_free).unwrap()((*value).u.s.str_0 as *mut libc::c_void);
            (*value).u.s.str_0 = 0 as *mut libc::c_char;
            (*value)
                .u
                .b = (if val as libc::c_int != 0 {
                awk_true as libc::c_int
            } else {
                awk_false as libc::c_int
            }) as awk_bool_t;
        }
    }
    return awk_true;
}
unsafe extern "C" fn read_number(
    mut fp: *mut FILE,
    mut value: *mut awk_value_t,
    mut code: uint32_t,
    mut vs: *mut value_storage,
) -> awk_bool_t {
    let mut len: uint32_t = 0;
    if code == 2 as libc::c_int as libc::c_uint {
        let mut buffer: [libc::c_char; 8192] = [0; 8192];
        let mut d: libc::c_double = 0.;
        if fread(
            &mut len as *mut uint32_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            fp,
        ) != ::std::mem::size_of::<uint32_t>() as libc::c_ulong
        {
            return awk_false;
        }
        len = __bswap_32(len);
        if fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as libc::c_ulong,
            fp,
        ) != len as libc::c_ulong
        {
            return awk_false;
        }
        sscanf(
            buffer.as_mut_ptr(),
            b"%lg\0" as *const u8 as *const libc::c_char,
            &mut d as *mut libc::c_double,
        );
        value = make_number(d, value);
    } else {
        ((*api).api_fatal)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"rwarray extension: GMP/MPFR value in file but compiled without GMP/MPFR support.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return awk_true;
}
static mut func_table: [awk_ext_func_t; 4] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"writea\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_writea
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
                name: b"reada\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_reada
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
                name: b"writeall\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_writeall
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
                name: b"readall\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_readall
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
    ]
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
            b"rwarray: version mismatch with gawk!\n\0" as *const u8
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
    j = (::std::mem::size_of::<[awk_ext_func_t; 4]>() as libc::c_ulong)
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
                b"rwarray: could not add %s\0" as *const u8 as *const libc::c_char,
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
                b"rwarray: initialization function failed\0" as *const u8
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

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
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
static mut api: *const gawk_api_t = 0 as *const gawk_api_t;
static mut ext_id: awk_ext_id_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut ext_version: *const libc::c_char = b"readfile extension: version 2.0\0"
    as *const u8 as *const libc::c_char;
static mut init_func: Option::<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> awk_bool_t>,
        Option::<unsafe extern "C" fn() -> awk_bool_t>,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> awk_bool_t,
                unsafe extern "C" fn() -> awk_bool_t,
            >(init_readfile),
        ),
    )
};
pub static mut plugin_is_GPL_compatible: libc::c_int = 0;
unsafe extern "C" fn read_file_to_buffer(
    mut fd: libc::c_int,
    mut sbuf: *const stat,
) -> *mut libc::c_char {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        != 0o100000 as libc::c_int as libc::c_uint
    {
        *__errno_location() = 22 as libc::c_int;
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        return 0 as *mut libc::c_char;
    }
    text = ((*api).api_malloc)
        .unwrap()(((*sbuf).st_size + 1 as libc::c_int as libc::c_long) as size_t)
        as *mut libc::c_char;
    if text.is_null() {
        ((*api).api_fatal)
            .unwrap()(
            ext_id,
            b"%s: malloc of %d bytes failed\0" as *const u8 as *const libc::c_char,
            b"do_readfile\0" as *const u8 as *const libc::c_char,
            (*sbuf).st_size + 1 as libc::c_int as libc::c_long,
        );
    }
    if read(fd, text as *mut libc::c_void, (*sbuf).st_size as size_t) != (*sbuf).st_size
    {
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        ((*api).api_free).unwrap()(text as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *text.offset((*sbuf).st_size as isize) = '\0' as i32 as libc::c_char;
    return text;
}
unsafe extern "C" fn do_readfile(
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
    let mut ret: libc::c_int = 0;
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    make_null_string(result);
    ((*api).api_unset_ERRNO).unwrap()(ext_id);
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut filename) as u64
        != 0
    {
        ret = stat(filename.u.s.str_0, &mut sbuf);
        if ret < 0 as libc::c_int {
            ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        } else {
            fd = open(filename.u.s.str_0, 0 as libc::c_int | 0 as libc::c_int);
            if fd < 0 as libc::c_int {
                ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
            } else {
                text = read_file_to_buffer(fd, &mut sbuf);
                if text.is_null() {
                    close(fd);
                } else {
                    close(fd);
                    r_make_string(
                        api,
                        ext_id,
                        text,
                        sbuf.st_size as size_t,
                        awk_false,
                        result,
                    );
                }
            }
        }
    } else if (*api).do_flags[0 as libc::c_int as usize] != 0 {
        ((*api).api_lintwarn)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"readfile: called with wrong kind of argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return result;
}
unsafe extern "C" fn readfile_get_record(
    mut out: *mut *mut libc::c_char,
    mut iobuf: *mut awk_input_buf_t,
    mut errcode: *mut libc::c_int,
    mut rt_start: *mut *mut libc::c_char,
    mut rt_len: *mut size_t,
    mut unused: *mut *const awk_fieldwidth_info_t,
) -> libc::c_int {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    if out.is_null() || iobuf.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*iobuf).opaque).is_null() {
        ((*api).api_free).unwrap()((*iobuf).opaque);
        (*iobuf).opaque = 0 as *mut libc::c_void;
        return -(1 as libc::c_int);
    }
    text = read_file_to_buffer((*iobuf).fd, &mut (*iobuf).sbuf);
    if text.is_null() {
        return -(1 as libc::c_int);
    }
    (*iobuf).opaque = text as *mut libc::c_void;
    *rt_start = 0 as *mut libc::c_char;
    *rt_len = 0 as libc::c_int as size_t;
    *out = text;
    return (*iobuf).sbuf.st_size as libc::c_int;
}
unsafe extern "C" fn readfile_can_take_file(
    mut iobuf: *const awk_input_buf_t,
) -> awk_bool_t {
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
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    if iobuf.is_null() {
        return awk_false;
    }
    if ((*api).api_sym_lookup)
        .unwrap()(
        ext_id,
        b"\0" as *const u8 as *const libc::c_char,
        b"PROCINFO\0" as *const u8 as *const libc::c_char,
        AWK_ARRAY,
        &mut array,
    ) as u64 == 0
    {
        return awk_false;
    }
    r_make_string(
        api,
        ext_id,
        b"readfile\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as size_t,
        awk_true,
        &mut index,
    );
    if ((*api).api_get_array_element)
        .unwrap()(ext_id, array.u.a, &mut index, AWK_UNDEFINED, &mut value) as u64 == 0
    {
        return awk_false;
    }
    return awk_true;
}
unsafe extern "C" fn readfile_take_control_of(
    mut iobuf: *mut awk_input_buf_t,
) -> awk_bool_t {
    if iobuf.is_null() {
        return awk_false;
    }
    (*iobuf)
        .get_record = Some(
        readfile_get_record
            as unsafe extern "C" fn(
                *mut *mut libc::c_char,
                *mut awk_input_buf_t,
                *mut libc::c_int,
                *mut *mut libc::c_char,
                *mut size_t,
                *mut *const awk_fieldwidth_info_t,
            ) -> libc::c_int,
    );
    return awk_true;
}
static mut readfile_parser: awk_input_parser_t = unsafe {
    {
        let mut init = awk_input_parser {
            name: b"readfile\0" as *const u8 as *const libc::c_char,
            can_take_file: Some(
                readfile_can_take_file
                    as unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
            ),
            take_control_of: Some(
                readfile_take_control_of
                    as unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
            ),
            next: 0 as *const awk_input_parser as *mut awk_input_parser,
        };
        init
    }
};
unsafe extern "C" fn init_readfile() -> awk_bool_t {
    ((*api).api_register_input_parser).unwrap()(ext_id, &mut readfile_parser);
    return awk_true;
}
static mut func_table: [awk_ext_func_t; 1] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"readfile\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_readfile
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
            b"readfile: version mismatch with gawk!\n\0" as *const u8
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
    j = (::std::mem::size_of::<[awk_ext_func_t; 1]>() as libc::c_ulong)
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
                b"readfile: could not add %s\0" as *const u8 as *const libc::c_char,
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
                b"readfile: initialization function failed\0" as *const u8
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

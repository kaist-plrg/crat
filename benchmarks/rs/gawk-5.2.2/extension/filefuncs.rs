use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn statvfs(__file: *const libc::c_char, __buf: *mut statvfs) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn gawk_fts_close(_: *mut FTS) -> libc::c_int;
    fn gawk_fts_open(
        _: *const *mut libc::c_char,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn gawk_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn gawk_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
    fn stack_empty() -> libc::c_int;
    fn stack_pop() -> *mut libc::c_void;
    fn stack_push(_: *mut libc::c_void) -> libc::c_int;
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
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
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
pub struct flagtab {
    pub name: *const libc::c_char,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_pathlen: libc::c_uint,
    pub fts_nitems: libc::c_uint,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_number: libc::c_longlong,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: libc::c_uint,
    pub fts_namelen: libc::c_uint,
    pub fts_ino: ino_t,
    pub fts_dev: dev_t,
    pub fts_nlink: libc::c_uint,
    pub fts_level: libc::c_int,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: *mut stat,
    pub fts_name: [libc::c_char; 1],
}
pub type FTSENT = _ftsent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftype_map {
    pub mask: libc::c_uint,
    pub type_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setuid_map {
    pub mask: libc::c_uint,
    pub index: libc::c_int,
    pub small_rep: libc::c_int,
    pub big_rep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_map {
    pub mask: libc::c_uint,
    pub rep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftype_map_0 {
    pub mask: libc::c_uint,
    pub charval: libc::c_int,
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
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
static mut init_func: Option::<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_filefuncs as unsafe extern "C" fn() -> awk_bool_t)
};
static mut ext_version: *const libc::c_char = b"filefuncs extension: version 1.0\0"
    as *const u8 as *const libc::c_char;
pub static mut plugin_is_GPL_compatible: libc::c_int = 0;
unsafe extern "C" fn do_chdir(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut newdir: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut newdir) as u64
        != 0
    {
        ret = chdir(newdir.u.s.str_0);
        if ret < 0 as libc::c_int {
            ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        }
    }
    return make_number(ret as libc::c_double, result);
}
unsafe extern "C" fn format_mode(mut fmode: libc::c_ulong) -> *mut libc::c_char {
    static mut outbuf: [libc::c_char; 12] = [0; 12];
    static mut ftype_map_0: [ftype_map_0; 7] = [
        {
            let mut init = ftype_map_0 {
                mask: 0o100000 as libc::c_int as libc::c_uint,
                charval: '-' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o60000 as libc::c_int as libc::c_uint,
                charval: 'b' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o20000 as libc::c_int as libc::c_uint,
                charval: 'c' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o40000 as libc::c_int as libc::c_uint,
                charval: 'd' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o140000 as libc::c_int as libc::c_uint,
                charval: 's' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o10000 as libc::c_int as libc::c_uint,
                charval: 'p' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o120000 as libc::c_int as libc::c_uint,
                charval: 'l' as i32,
            };
            init
        },
    ];
    static mut map: [mode_map; 9] = [
        {
            let mut init = mode_map {
                mask: 0o400 as libc::c_int as libc::c_uint,
                rep: 'r' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: 0o200 as libc::c_int as libc::c_uint,
                rep: 'w' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: 0o100 as libc::c_int as libc::c_uint,
                rep: 'x' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint,
                rep: 'r' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint,
                rep: 'w' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint,
                rep: 'x' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint,
                rep: 'r' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint,
                rep: 'w' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint,
                rep: 'x' as i32,
            };
            init
        },
    ];
    static mut setuid_map_0: [setuid_map; 3] = [
        {
            let mut init = setuid_map {
                mask: 0o4000 as libc::c_int as libc::c_uint,
                index: 3 as libc::c_int,
                small_rep: 's' as i32,
                big_rep: 'S' as i32,
            };
            init
        },
        {
            let mut init = setuid_map {
                mask: 0o2000 as libc::c_int as libc::c_uint,
                index: 6 as libc::c_int,
                small_rep: 's' as i32,
                big_rep: 'l' as i32,
            };
            init
        },
        {
            let mut init = setuid_map {
                mask: 0o1000 as libc::c_int as libc::c_uint,
                index: 9 as libc::c_int,
                small_rep: 't' as i32,
                big_rep: 'T' as i32,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    strcpy(outbuf.as_mut_ptr(), b"----------\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    k = (::std::mem::size_of::<[ftype_map_0; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<ftype_map_0>() as libc::c_ulong)
        as libc::c_int;
    while j < k {
        if fmode & 0o170000 as libc::c_int as libc::c_ulong
            == ftype_map_0[j as usize].mask as libc::c_ulong
        {
            outbuf[i as usize] = ftype_map_0[j as usize].charval as libc::c_char;
            break;
        } else {
            j += 1;
            j;
        }
    }
    j = 0 as libc::c_int;
    k = (::std::mem::size_of::<[mode_map; 9]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<mode_map>() as libc::c_ulong) as libc::c_int;
    while j < k {
        i += 1;
        i;
        if fmode & map[j as usize].mask as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            outbuf[i as usize] = map[j as usize].rep as libc::c_char;
        }
        j += 1;
        j;
    }
    i += 1;
    i;
    outbuf[i as usize] = '\0' as i32 as libc::c_char;
    j = 0 as libc::c_int;
    k = (::std::mem::size_of::<[setuid_map; 3]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<setuid_map>() as libc::c_ulong)
        as libc::c_int;
    while j < k {
        if fmode & setuid_map_0[j as usize].mask as libc::c_ulong != 0 {
            if outbuf[setuid_map_0[j as usize].index as usize] as libc::c_int
                == 'x' as i32
            {
                outbuf[setuid_map_0[j as usize].index
                    as usize] = setuid_map_0[j as usize].small_rep as libc::c_char;
            } else {
                outbuf[setuid_map_0[j as usize].index
                    as usize] = setuid_map_0[j as usize].big_rep as libc::c_char;
            }
        }
        j += 1;
        j;
    }
    return outbuf.as_mut_ptr();
}
unsafe extern "C" fn read_symlink(
    mut fname: *const libc::c_char,
    mut bufsize: size_t,
    mut linksize: *mut ssize_t,
) -> *mut libc::c_char {
    if bufsize != 0 {
        bufsize = (bufsize as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else {
        bufsize = (8192 as libc::c_int * 2 as libc::c_int) as size_t;
    }
    if bufsize
        > (if (-(1 as libc::c_int) as size_t)
            < 9223372036854775807 as libc::c_long as libc::c_ulong
        {
            -(1 as libc::c_int) as size_t
        } else {
            9223372036854775807 as libc::c_long as libc::c_ulong
        }) || bufsize < 2 as libc::c_int as libc::c_ulong
    {
        bufsize = if (-(1 as libc::c_int) as size_t)
            < 9223372036854775807 as libc::c_long as libc::c_ulong
        {
            -(1 as libc::c_int) as size_t
        } else {
            9223372036854775807 as libc::c_long as libc::c_ulong
        };
    }
    loop {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        buf = ((*api).api_malloc).unwrap()(bufsize) as *mut libc::c_char;
        if buf.is_null() {
            ((*api).api_fatal)
                .unwrap()(
                ext_id,
                b"%s: malloc of %d bytes failed\0" as *const u8 as *const libc::c_char,
                b"read_symlink\0" as *const u8 as *const libc::c_char,
                bufsize,
            );
        }
        *linksize = readlink(fname, buf, bufsize);
        if *linksize < 0 as libc::c_int as libc::c_long {
            if *__errno_location() != 34 as libc::c_int {
                ((*api).api_free).unwrap()(buf as *mut libc::c_void);
                return 0 as *mut libc::c_char;
            }
        } else if *linksize as size_t
            <= bufsize.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
            *buf.offset(*linksize as isize) = '\0' as i32 as libc::c_char;
            return buf;
        }
        ((*api).api_free).unwrap()(buf as *mut libc::c_void);
        if bufsize
            <= (if (-(1 as libc::c_int) as size_t)
                < 9223372036854775807 as libc::c_long as libc::c_ulong
            {
                -(1 as libc::c_int) as size_t
            } else {
                9223372036854775807 as libc::c_long as libc::c_ulong
            })
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            bufsize = (bufsize as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else if bufsize
            < (if (-(1 as libc::c_int) as size_t)
                < 9223372036854775807 as libc::c_long as libc::c_ulong
            {
                -(1 as libc::c_int) as size_t
            } else {
                9223372036854775807 as libc::c_long as libc::c_ulong
            })
        {
            bufsize = if (-(1 as libc::c_int) as size_t)
                < 9223372036854775807 as libc::c_long as libc::c_ulong
            {
                -(1 as libc::c_int) as size_t
            } else {
                9223372036854775807 as libc::c_long as libc::c_ulong
            };
        } else {
            return 0 as *mut libc::c_char
        }
    };
}
unsafe extern "C" fn device_blocksize() -> libc::c_int {
    return 512 as libc::c_int;
}
unsafe extern "C" fn array_set(
    mut array: awk_array_t,
    mut sub: *const libc::c_char,
    mut value: *mut awk_value_t,
) {
    let mut index: awk_value_t = awk_value_t {
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
        array,
        r_make_string(api, ext_id, sub, strlen(sub), awk_true, &mut index),
        value,
    );
}
unsafe extern "C" fn array_set_numeric(
    mut array: awk_array_t,
    mut sub: *const libc::c_char,
    mut num: libc::c_double,
) {
    let mut tmp: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    array_set(array, sub, make_number(num, &mut tmp));
}
unsafe extern "C" fn fill_stat_array(
    mut name: *const libc::c_char,
    mut array: awk_array_t,
    mut sbuf: *mut stat,
) -> libc::c_int {
    let mut pmode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *const libc::c_char = b"unknown\0" as *const u8
        as *const libc::c_char;
    let mut tmp: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    static mut ftype_map_0: [ftype_map; 7] = [
        {
            let mut init = ftype_map {
                mask: 0o100000 as libc::c_int as libc::c_uint,
                type_0: b"file\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o60000 as libc::c_int as libc::c_uint,
                type_0: b"blockdev\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o20000 as libc::c_int as libc::c_uint,
                type_0: b"chardev\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o40000 as libc::c_int as libc::c_uint,
                type_0: b"directory\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o140000 as libc::c_int as libc::c_uint,
                type_0: b"socket\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o10000 as libc::c_int as libc::c_uint,
                type_0: b"fifo\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o120000 as libc::c_int as libc::c_uint,
                type_0: b"symlink\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    ((*api).api_clear_array).unwrap()(ext_id, array);
    array_set(
        array,
        b"name\0" as *const u8 as *const libc::c_char,
        r_make_string(api, ext_id, name, strlen(name), awk_true, &mut tmp),
    );
    array_set_numeric(
        array,
        b"dev\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_dev as libc::c_double,
    );
    array_set_numeric(
        array,
        b"ino\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_ino as libc::c_double,
    );
    array_set_numeric(
        array,
        b"mode\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_mode as libc::c_double,
    );
    array_set_numeric(
        array,
        b"nlink\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_nlink as libc::c_double,
    );
    array_set_numeric(
        array,
        b"uid\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_uid as libc::c_double,
    );
    array_set_numeric(
        array,
        b"gid\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_gid as libc::c_double,
    );
    array_set_numeric(
        array,
        b"size\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_size as libc::c_double,
    );
    array_set_numeric(
        array,
        b"blocks\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_blocks as libc::c_double,
    );
    array_set_numeric(
        array,
        b"atime\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_atim.tv_sec as libc::c_double,
    );
    array_set_numeric(
        array,
        b"mtime\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_mtim.tv_sec as libc::c_double,
    );
    array_set_numeric(
        array,
        b"ctime\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_ctim.tv_sec as libc::c_double,
    );
    if (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
        || (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
    {
        array_set_numeric(
            array,
            b"rdev\0" as *const u8 as *const libc::c_char,
            (*sbuf).st_rdev as libc::c_double,
        );
        array_set_numeric(
            array,
            b"major\0" as *const u8 as *const libc::c_char,
            gnu_dev_major((*sbuf).st_rdev) as libc::c_double,
        );
        array_set_numeric(
            array,
            b"minor\0" as *const u8 as *const libc::c_char,
            gnu_dev_minor((*sbuf).st_rdev) as libc::c_double,
        );
    }
    array_set_numeric(
        array,
        b"blksize\0" as *const u8 as *const libc::c_char,
        (*sbuf).st_blksize as libc::c_double,
    );
    array_set_numeric(
        array,
        b"devbsize\0" as *const u8 as *const libc::c_char,
        device_blocksize() as libc::c_double,
    );
    pmode = format_mode((*sbuf).st_mode as libc::c_ulong);
    array_set(
        array,
        b"pmode\0" as *const u8 as *const libc::c_char,
        r_make_string(api, ext_id, pmode, strlen(pmode), awk_true, &mut tmp),
    );
    if (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut linksize: ssize_t = 0;
        buf = read_symlink(name, (*sbuf).st_size as size_t, &mut linksize);
        if !buf.is_null() {
            array_set(
                array,
                b"linkval\0" as *const u8 as *const libc::c_char,
                r_make_string(api, ext_id, buf, linksize as size_t, awk_false, &mut tmp),
            );
        } else {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"stat: unable to read symbolic link `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
    }
    type_0 = b"unknown\0" as *const u8 as *const libc::c_char;
    j = 0 as libc::c_int;
    k = (::std::mem::size_of::<[ftype_map; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<ftype_map>() as libc::c_ulong)
        as libc::c_int;
    while j < k {
        if (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == ftype_map_0[j as usize].mask
        {
            type_0 = ftype_map_0[j as usize].type_0;
            break;
        } else {
            j += 1;
            j;
        }
    }
    array_set(
        array,
        b"type\0" as *const u8 as *const libc::c_char,
        r_make_string(api, ext_id, type_0, strlen(type_0), awk_true, &mut tmp),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_stat(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut file_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut array_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut array: awk_array_t = 0 as *mut libc::c_void;
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
    let mut statfunc: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    > = Some(
        lstat as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    );
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut file_param) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"stat: first argument is not a string\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return make_number(-(1 as libc::c_int) as libc::c_double, result);
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_ARRAY, &mut array_param) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"stat: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return make_number(-(1 as libc::c_int) as libc::c_double, result);
    }
    if nargs == 3 as libc::c_int {
        statfunc = Some(
            stat as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        );
    }
    name = file_param.u.s.str_0;
    array = array_param.u.a;
    ((*api).api_clear_array).unwrap()(ext_id, array);
    ret = statfunc.unwrap()(name, &mut sbuf);
    if ret < 0 as libc::c_int {
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        return make_number(ret as libc::c_double, result);
    }
    ret = fill_stat_array(name, array, &mut sbuf);
    return make_number(ret as libc::c_double, result);
}
unsafe extern "C" fn do_statvfs(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut file_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut array_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut array: awk_array_t = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    let mut vfsbuf: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        __f_spare: [0; 6],
    };
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_STRING, &mut file_param) as u64
        == 0
        || ((*api).api_get_argument)
            .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_ARRAY, &mut array_param)
            as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"stat: bad parameters\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return make_number(-(1 as libc::c_int) as libc::c_double, result);
    }
    name = file_param.u.s.str_0;
    array = array_param.u.a;
    ((*api).api_clear_array).unwrap()(ext_id, array);
    ret = statvfs(name, &mut vfsbuf);
    if ret < 0 as libc::c_int {
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
        return make_number(ret as libc::c_double, result);
    }
    array_set_numeric(
        array,
        b"bsize\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_bsize as libc::c_double,
    );
    array_set_numeric(
        array,
        b"frsize\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_frsize as libc::c_double,
    );
    array_set_numeric(
        array,
        b"blocks\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_blocks as libc::c_double,
    );
    array_set_numeric(
        array,
        b"bfree\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_bfree as libc::c_double,
    );
    array_set_numeric(
        array,
        b"bavail\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_bavail as libc::c_double,
    );
    array_set_numeric(
        array,
        b"files\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_files as libc::c_double,
    );
    array_set_numeric(
        array,
        b"ffree\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_ffree as libc::c_double,
    );
    array_set_numeric(
        array,
        b"favail\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_favail as libc::c_double,
    );
    array_set_numeric(
        array,
        b"fsid\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_fsid as libc::c_double,
    );
    array_set_numeric(
        array,
        b"flag\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_flag as libc::c_double,
    );
    array_set_numeric(
        array,
        b"namemax\0" as *const u8 as *const libc::c_char,
        vfsbuf.f_namemax as libc::c_double,
    );
    return make_number(ret as libc::c_double, result);
}
unsafe extern "C" fn init_filefuncs() -> awk_bool_t {
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    static mut opentab: [flagtab; 8] = [
        {
            let mut init = flagtab {
                name: b"FTS_COMFOLLOW\0" as *const u8 as *const libc::c_char,
                value: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_LOGICAL\0" as *const u8 as *const libc::c_char,
                value: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_NOCHDIR\0" as *const u8 as *const libc::c_char,
                value: 0x4 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_PHYSICAL\0" as *const u8 as *const libc::c_char,
                value: 0x10 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_SEEDOT\0" as *const u8 as *const libc::c_char,
                value: 0x20 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_XDEV\0" as *const u8 as *const libc::c_char,
                value: 0x40 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_SKIP\0" as *const u8 as *const libc::c_char,
                value: 0x200 as libc::c_int,
            };
            init
        },
        {
            let mut init = flagtab {
                name: 0 as *const libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
    ];
    i = 0 as libc::c_int;
    while !(opentab[i as usize].name).is_null() {
        make_number(opentab[i as usize].value as libc::c_double, &mut value);
        if ((*api).api_sym_update)
            .unwrap()(
            ext_id,
            b"\0" as *const u8 as *const libc::c_char,
            opentab[i as usize].name,
            &mut value,
        ) as u64 == 0
        {
            ((*api).api_warning)
                .unwrap()(
                ext_id,
                dcgettext(
                    0 as *const libc::c_char,
                    b"fts init: could not create variable %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opentab[i as usize].name,
            );
            errors += 1;
            errors;
        }
        i += 1;
        i;
    }
    return (errors == 0 as libc::c_int) as libc::c_int as awk_bool_t;
}
static mut fts_errors: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn fill_stat_element(
    mut element_array: awk_array_t,
    mut name: *const libc::c_char,
    mut sbuf: *mut stat,
) {
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
    let mut stat_array: awk_array_t = 0 as *mut libc::c_void;
    stat_array = ((*api).api_create_array).unwrap()(ext_id);
    if stat_array.is_null() {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fill_stat_element: could not create array, out of memory\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fts_errors += 1;
        fts_errors;
        return;
    }
    fill_stat_array(name, stat_array, sbuf);
    r_make_string(
        api,
        ext_id,
        b"stat\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
        awk_true,
        &mut index,
    );
    value.val_type = AWK_ARRAY;
    value.u.a = stat_array;
    if ((*api).api_set_array_element)
        .unwrap()(ext_id, element_array, &mut index, &mut value) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fill_stat_element: could not set element\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fts_errors += 1;
        fts_errors;
    }
}
unsafe extern "C" fn fill_path_element(
    mut element_array: awk_array_t,
    mut path: *const libc::c_char,
) {
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
        b"path\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
        awk_true,
        &mut index,
    );
    r_make_string(api, ext_id, path, strlen(path), awk_true, &mut value);
    if ((*api).api_set_array_element)
        .unwrap()(ext_id, element_array, &mut index, &mut value) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fill_path_element: could not set element\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fts_errors += 1;
        fts_errors;
    }
}
unsafe extern "C" fn fill_error_element(
    mut element_array: awk_array_t,
    errcode: libc::c_int,
) {
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
    let mut err: *const libc::c_char = strerror(errcode);
    r_make_string(
        api,
        ext_id,
        b"error\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
        awk_true,
        &mut index,
    );
    r_make_string(api, ext_id, err, strlen(err), awk_true, &mut value);
    if ((*api).api_set_array_element)
        .unwrap()(ext_id, element_array, &mut index, &mut value) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fill_error_element: could not set element\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fts_errors += 1;
        fts_errors;
    }
}
unsafe extern "C" fn fill_default_elements(
    mut element_array: awk_array_t,
    fentry: *const FTSENT,
    mut bad_ret: awk_bool_t,
) {
    fill_path_element(element_array, (*fentry).fts_path);
    if bad_ret as u64 == 0 {
        fill_stat_element(
            element_array,
            ((*fentry).fts_name).as_ptr(),
            (*fentry).fts_statp,
        );
    }
    if bad_ret as libc::c_uint != 0 || (*fentry).fts_errno != 0 as libc::c_int {
        fill_error_element(element_array, (*fentry).fts_errno);
    }
}
unsafe extern "C" fn process(
    mut hierarchy: *mut FTS,
    mut destarray: awk_array_t,
    mut seedot: libc::c_int,
    mut skipset: libc::c_int,
) {
    let mut fentry: *mut FTSENT = 0 as *mut FTSENT;
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
    let mut element_array: awk_array_t = 0 as *mut libc::c_void;
    let mut newdir_array: awk_array_t = 0 as *mut libc::c_void;
    let mut dot_array: awk_array_t = 0 as *mut libc::c_void;
    let mut bad_ret: awk_bool_t = awk_false;
    loop {
        fentry = gawk_fts_read(hierarchy);
        if fentry.is_null() {
            break;
        }
        bad_ret = awk_false;
        let mut current_block_33: u64;
        match (*fentry).fts_info as libc::c_int {
            1 => {
                if skipset != 0 && (*fentry).fts_level == 0 as libc::c_int {
                    gawk_fts_set(hierarchy, fentry, 4 as libc::c_int);
                }
                newdir_array = ((*api).api_create_array).unwrap()(ext_id);
                if newdir_array.is_null() {
                    ((*api).api_warning)
                        .unwrap()(
                        ext_id,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fts-process: could not create array\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fts_errors += 1;
                    fts_errors;
                } else {
                    r_make_string(
                        api,
                        ext_id,
                        ((*fentry).fts_name).as_mut_ptr(),
                        (*fentry).fts_namelen as size_t,
                        awk_true,
                        &mut index,
                    );
                    value.val_type = AWK_ARRAY;
                    value.u.a = newdir_array;
                    if ((*api).api_set_array_element)
                        .unwrap()(ext_id, destarray, &mut index, &mut value) as u64 == 0
                    {
                        ((*api).api_warning)
                            .unwrap()(
                            ext_id,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fts-process: could not set element\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fts_errors += 1;
                        fts_errors;
                    } else {
                        newdir_array = value.u.a;
                        stack_push(destarray);
                        destarray = newdir_array;
                    }
                }
                current_block_33 = 3123434771885419771;
            }
            4 | 2 | 7 | 10 => {
                bad_ret = awk_true;
                current_block_33 = 11077987873240601907;
            }
            11 | 12 | 13 | 8 | 5 => {
                current_block_33 = 11077987873240601907;
            }
            6 => {
                dot_array = ((*api).api_create_array).unwrap()(ext_id);
                r_make_string(
                    api,
                    ext_id,
                    b".\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                    awk_true,
                    &mut index,
                );
                value.val_type = AWK_ARRAY;
                value.u.a = dot_array;
                if ((*api).api_set_array_element)
                    .unwrap()(ext_id, destarray, &mut index, &mut value) as u64 == 0
                {
                    ((*api).api_warning)
                        .unwrap()(
                        ext_id,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fts-process: could not set element\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fts_errors += 1;
                    fts_errors;
                } else {
                    fill_default_elements(dot_array, fentry, bad_ret);
                    if stack_empty() == 0 {
                        destarray = stack_pop();
                    }
                }
                current_block_33 = 3123434771885419771;
            }
            3 | _ => {
                current_block_33 = 3123434771885419771;
            }
        }
        match current_block_33 {
            11077987873240601907 => {
                if !(seedot != 0
                    && strcmp(
                        ((*fentry).fts_name).as_mut_ptr(),
                        b".\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int)
                {
                    element_array = ((*api).api_create_array).unwrap()(ext_id);
                    if element_array.is_null() {
                        ((*api).api_warning)
                            .unwrap()(
                            ext_id,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fts-process: could not create array\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fts_errors += 1;
                        fts_errors;
                    } else {
                        r_make_string(
                            api,
                            ext_id,
                            ((*fentry).fts_name).as_mut_ptr(),
                            (*fentry).fts_namelen as size_t,
                            awk_true,
                            &mut index,
                        );
                        value.val_type = AWK_ARRAY;
                        value.u.a = element_array;
                        if ((*api).api_set_array_element)
                            .unwrap()(ext_id, destarray, &mut index, &mut value) as u64
                            == 0
                        {
                            ((*api).api_warning)
                                .unwrap()(
                                ext_id,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"fts-process: could not set element\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            fts_errors += 1;
                            fts_errors;
                        } else {
                            fill_default_elements(element_array, fentry, bad_ret);
                        }
                    }
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn do_fts(
    mut nargs: libc::c_int,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut pathlist: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut flagval: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut dest: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    let mut path_array: *mut awk_flat_array_t = 0 as *mut awk_flat_array_t;
    let mut pathvector: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut hierarchy: *mut FTS = 0 as *mut FTS;
    let mut flags: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    static mut mask: libc::c_int = 0x1 as libc::c_int | 0x2 as libc::c_int
        | 0x4 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
        | 0x40 as libc::c_int | 0x200 as libc::c_int;
    fts_errors = 0 as libc::c_int;
    if nargs > 3 as libc::c_int {
        ((*api).api_lintwarn)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fts: called with incorrect number of arguments, expecting 3\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if ((*api).api_get_argument)
        .unwrap()(ext_id, 0 as libc::c_int as size_t, AWK_ARRAY, &mut pathlist) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fts: first argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, 22 as libc::c_int);
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 1 as libc::c_int as size_t, AWK_NUMBER, &mut flagval) as u64
        == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fts: second argument is not a number\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, 22 as libc::c_int);
    } else if ((*api).api_get_argument)
        .unwrap()(ext_id, 2 as libc::c_int as size_t, AWK_ARRAY, &mut dest) as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fts: third argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ((*api).api_update_ERRNO_int).unwrap()(ext_id, 22 as libc::c_int);
    } else if ((*api).api_flatten_array_typed)
        .unwrap()(ext_id, pathlist.u.a, &mut path_array, AWK_STRING, AWK_UNDEFINED)
        as u64 == 0
    {
        ((*api).api_warning)
            .unwrap()(
            ext_id,
            dcgettext(
                0 as *const libc::c_char,
                b"fts: could not flatten array\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        flags = flagval.u.n.d as libc::c_int;
        if flags & (0x10 as libc::c_int | 0x2 as libc::c_int) == 0 as libc::c_int
            || flags & (0x10 as libc::c_int | 0x2 as libc::c_int)
                == 0x10 as libc::c_int | 0x2 as libc::c_int
        {
            ((*api).api_update_ERRNO_int).unwrap()(ext_id, 22 as libc::c_int);
        } else {
            if flags & 0x8 as libc::c_int != 0 as libc::c_int {
                flags &= !(0x8 as libc::c_int);
                if (*api).do_flags[0 as libc::c_int as usize] != 0 {
                    ((*api).api_lintwarn)
                        .unwrap()(
                        ext_id,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fts: ignoring sneaky FTS_NOSTAT flag. nyah, nyah, nyah.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            flags &= mask;
            if flags & 0x200 as libc::c_int != 0 {
                flags |= 0x4 as libc::c_int;
            }
            count = ((*path_array).count)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            pathvector = ((*api).api_calloc)
                .unwrap()(
                1 as libc::c_int as size_t,
                count
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if pathvector.is_null() {
                ((*api).api_fatal)
                    .unwrap()(
                    ext_id,
                    b"%s: calloc of %d bytes failed\0" as *const u8
                        as *const libc::c_char,
                    b"do_fts\0" as *const u8 as *const libc::c_char,
                    count
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                );
            }
            count = count.wrapping_sub(1);
            count;
            i = 0 as libc::c_int as size_t;
            while i < count {
                let ref mut fresh0 = *pathvector.offset(i as isize);
                *fresh0 = (*((*path_array).elements).as_mut_ptr().offset(i as isize))
                    .value
                    .u
                    .s
                    .str_0;
                i = i.wrapping_add(1);
                i;
            }
            hierarchy = gawk_fts_open(pathvector, flags & !(0x200 as libc::c_int), None);
            if !hierarchy.is_null() {
                process(
                    hierarchy,
                    dest.u.a,
                    (flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                    (flags & 0x200 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                );
                gawk_fts_close(hierarchy);
                if fts_errors == 0 as libc::c_int {
                    ret = 0 as libc::c_int;
                }
            } else {
                ((*api).api_update_ERRNO_int).unwrap()(ext_id, *__errno_location());
            }
        }
    }
    if !pathvector.is_null() {
        ((*api).api_free).unwrap()(pathvector as *mut libc::c_void);
    }
    if !path_array.is_null() {
        ((*api).api_release_flattened_array).unwrap()(ext_id, pathlist.u.a, path_array);
    }
    return make_number(ret as libc::c_double, result);
}
static mut func_table: [awk_ext_func_t; 4] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"chdir\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_chdir
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
                name: b"stat\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_stat
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 3 as libc::c_int as size_t,
                min_required_args: 2 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"fts\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_fts
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 3 as libc::c_int as size_t,
                min_required_args: 3 as libc::c_int as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"statvfs\0" as *const u8 as *const libc::c_char,
                function: Some(
                    do_statvfs
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
            b"filefuncs: version mismatch with gawk!\n\0" as *const u8
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
                b"filefuncs: could not add %s\0" as *const u8 as *const libc::c_char,
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
                b"filefuncs: initialization function failed\0" as *const u8
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

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn getdtablesize() -> libc::c_int;
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
pub type two_way_proc_data_t = two_way_proc_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct two_way_proc_data {
    pub size: size_t,
    pub len: size_t,
    pub data: *mut libc::c_char,
    pub in_use: size_t,
}
static mut api: *const gawk_api_t = 0 as *const gawk_api_t;
static mut ext_id: awk_ext_id_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut ext_version: *const libc::c_char = b"revtwoway extension: version 1.0\0"
    as *const u8 as *const libc::c_char;
static mut init_func: Option::<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_revtwoway as unsafe extern "C" fn() -> awk_bool_t)
};
pub static mut plugin_is_GPL_compatible: libc::c_int = 0;
static mut max_fds: size_t = 0;
unsafe extern "C" fn close_two_proc_data(mut proc_data: *mut two_way_proc_data_t) {
    if (*proc_data).in_use > 1 as libc::c_int as libc::c_ulong {
        (*proc_data).in_use = ((*proc_data).in_use).wrapping_sub(1);
        (*proc_data).in_use;
        return;
    }
    ((*api).api_free).unwrap()((*proc_data).data as *mut libc::c_void);
    ((*api).api_free).unwrap()(proc_data as *mut libc::c_void);
}
unsafe extern "C" fn rev2way_get_record(
    mut out: *mut *mut libc::c_char,
    mut iobuf: *mut awk_input_buf_t,
    mut errcode: *mut libc::c_int,
    mut rt_start: *mut *mut libc::c_char,
    mut rt_len: *mut size_t,
    mut unused: *mut *const awk_fieldwidth_info_t,
) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut proc_data: *mut two_way_proc_data_t = 0 as *mut two_way_proc_data_t;
    if out.is_null() || iobuf.is_null() || ((*iobuf).opaque).is_null() {
        return -(1 as libc::c_int);
    }
    proc_data = (*iobuf).opaque as *mut two_way_proc_data_t;
    if (*proc_data).len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    *out = (*proc_data).data;
    len = (*proc_data).len as libc::c_int;
    (*proc_data).len = 0 as libc::c_int as size_t;
    *rt_len = 0 as libc::c_int as size_t;
    if *((*proc_data).data).offset((len - 1 as libc::c_int) as isize) as libc::c_int
        == '\n' as i32
    {
        while *((*proc_data).data).offset((len - 1 as libc::c_int) as isize)
            as libc::c_int == '\n' as i32
        {
            len -= 1;
            len;
            *rt_len = (*rt_len).wrapping_add(1);
            *rt_len;
        }
        *rt_start = ((*proc_data).data).offset(len as isize);
    }
    return len;
}
unsafe extern "C" fn rev2way_close(mut iobuf: *mut awk_input_buf_t) {
    let mut proc_data: *mut two_way_proc_data_t = 0 as *mut two_way_proc_data_t;
    if iobuf.is_null() || ((*iobuf).opaque).is_null() {
        return;
    }
    proc_data = (*iobuf).opaque as *mut two_way_proc_data_t;
    close_two_proc_data(proc_data);
    (*iobuf).fd = -(1 as libc::c_int);
}
unsafe extern "C" fn rev2way_fwrite(
    mut buf: *const libc::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> size_t {
    let mut proc_data: *mut two_way_proc_data_t = 0 as *mut two_way_proc_data_t;
    let mut amount: size_t = 0;
    let mut char_count: size_t = 0;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    if opaque.is_null() {
        return 0 as libc::c_int as size_t;
    }
    proc_data = opaque as *mut two_way_proc_data_t;
    amount = size.wrapping_mul(count);
    if amount > (*proc_data).size || (*proc_data).len > 0 as libc::c_int as libc::c_ulong
    {
        if ((*proc_data).data).is_null() {
            (*proc_data)
                .data = ((*api).api_malloc).unwrap()(amount) as *mut libc::c_char;
            if ((*proc_data).data).is_null() {
                ((*api).api_fatal)
                    .unwrap()(
                    ext_id,
                    b"%s: malloc of %d bytes failed\0" as *const u8
                        as *const libc::c_char,
                    b"rev2way_fwrite\0" as *const u8 as *const libc::c_char,
                    amount,
                );
            }
        } else {
            (*proc_data)
                .data = ((*api).api_realloc)
                .unwrap()(
                (*proc_data).data as *mut libc::c_void,
                ((*proc_data).size).wrapping_add(amount),
            ) as *mut libc::c_char;
            if ((*proc_data).data).is_null() {
                ((*api).api_fatal)
                    .unwrap()(
                    ext_id,
                    b"%s: realloc of %d bytes failed\0" as *const u8
                        as *const libc::c_char,
                    b"rev2way_fwrite\0" as *const u8 as *const libc::c_char,
                    ((*proc_data).size).wrapping_add(amount),
                );
            }
        }
        (*proc_data)
            .size = ((*proc_data).size as libc::c_ulong).wrapping_add(amount) as size_t
            as size_t;
    }
    src = (buf as *mut libc::c_char)
        .offset(amount as isize)
        .offset(-(1 as libc::c_int as isize));
    dest = ((*proc_data).data).offset((*proc_data).len as isize);
    char_count = amount;
    while char_count > 0 as libc::c_int as libc::c_ulong {
        let fresh0 = src;
        src = src.offset(-1);
        let fresh1 = dest;
        dest = dest.offset(1);
        *fresh1 = *fresh0;
        char_count = char_count.wrapping_sub(1);
        char_count;
    }
    (*proc_data)
        .len = ((*proc_data).len as libc::c_ulong).wrapping_add(amount) as size_t
        as size_t;
    return amount;
}
unsafe extern "C" fn rev2way_fflush(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn rev2way_ferror(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn rev2way_fclose(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut proc_data: *mut two_way_proc_data_t = 0 as *mut two_way_proc_data_t;
    if opaque.is_null() {
        return -(1 as libc::c_int);
    }
    proc_data = opaque as *mut two_way_proc_data_t;
    close_two_proc_data(proc_data);
    return 0 as libc::c_int;
}
unsafe extern "C" fn revtwoway_can_take_two_way(
    mut name: *const libc::c_char,
) -> awk_bool_t {
    return (!name.is_null()
        && strcmp(name, b"/magic/mirror\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int) as libc::c_int as awk_bool_t;
}
unsafe extern "C" fn revtwoway_take_control_of(
    mut name: *const libc::c_char,
    mut inbuf: *mut awk_input_buf_t,
    mut outbuf: *mut awk_output_buf_t,
) -> awk_bool_t {
    let mut proc_data: *mut two_way_proc_data_t = 0 as *mut two_way_proc_data_t;
    if inbuf.is_null() || outbuf.is_null() {
        return awk_false;
    }
    proc_data = ((*api).api_malloc)
        .unwrap()(::std::mem::size_of::<two_way_proc_data_t>() as libc::c_ulong)
        as *mut two_way_proc_data_t;
    if proc_data.is_null() {
        ((*api).api_fatal)
            .unwrap()(
            ext_id,
            b"%s: malloc of %d bytes failed\0" as *const u8 as *const libc::c_char,
            b"revtwoway_take_control_of\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<two_way_proc_data_t>() as libc::c_ulong,
        );
    }
    (*proc_data).in_use = 2 as libc::c_int as size_t;
    (*proc_data).size = 0 as libc::c_int as size_t;
    (*proc_data).len = 0 as libc::c_int as size_t;
    (*proc_data).data = 0 as *mut libc::c_char;
    if max_fds.wrapping_add(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        max_fds = getdtablesize() as size_t;
    }
    (*inbuf)
        .get_record = Some(
        rev2way_get_record
            as unsafe extern "C" fn(
                *mut *mut libc::c_char,
                *mut awk_input_buf_t,
                *mut libc::c_int,
                *mut *mut libc::c_char,
                *mut size_t,
                *mut *const awk_fieldwidth_info_t,
            ) -> libc::c_int,
    );
    (*inbuf)
        .close_func = Some(
        rev2way_close as unsafe extern "C" fn(*mut awk_input_buf_t) -> (),
    );
    (*inbuf).fd = max_fds as libc::c_int;
    (*inbuf).opaque = proc_data as *mut libc::c_void;
    let fresh2 = max_fds;
    max_fds = max_fds.wrapping_add(1);
    (*outbuf).fp = fresh2 as *mut FILE;
    (*outbuf).opaque = proc_data as *mut libc::c_void;
    (*outbuf)
        .gawk_fwrite = Some(
        rev2way_fwrite
            as unsafe extern "C" fn(
                *const libc::c_void,
                size_t,
                size_t,
                *mut FILE,
                *mut libc::c_void,
            ) -> size_t,
    );
    (*outbuf)
        .gawk_fflush = Some(
        rev2way_fflush
            as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    );
    (*outbuf)
        .gawk_ferror = Some(
        rev2way_ferror
            as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    );
    (*outbuf)
        .gawk_fclose = Some(
        rev2way_fclose
            as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    );
    (*outbuf).redirected = awk_true;
    return awk_true;
}
static mut two_way_processor: awk_two_way_processor_t = unsafe {
    {
        let mut init = awk_two_way_processor {
            name: b"revtwoway\0" as *const u8 as *const libc::c_char,
            can_take_two_way: Some(
                revtwoway_can_take_two_way
                    as unsafe extern "C" fn(*const libc::c_char) -> awk_bool_t,
            ),
            take_control_of: Some(
                revtwoway_take_control_of
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut awk_input_buf_t,
                        *mut awk_output_buf_t,
                    ) -> awk_bool_t,
            ),
            next: 0 as *const awk_two_way_processor as *mut awk_two_way_processor,
        };
        init
    }
};
unsafe extern "C" fn init_revtwoway() -> awk_bool_t {
    ((*api).api_register_two_way_processor).unwrap()(ext_id, &mut two_way_processor);
    max_fds = getdtablesize() as size_t;
    return awk_true;
}
static mut func_table: [awk_ext_func_t; 1] = [
    {
        let mut init = awk_ext_func {
            name: 0 as *const libc::c_char,
            function: None,
            max_expected_args: 0 as libc::c_int as size_t,
            min_required_args: 0 as libc::c_int as size_t,
            suppress_lint: awk_false,
            data: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    },
];
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
            b"revtwoway: version mismatch with gawk!\n\0" as *const u8
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
                b"revtwoway: could not add %s\0" as *const u8 as *const libc::c_char,
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
                b"revtwoway: initialization function failed\0" as *const u8
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

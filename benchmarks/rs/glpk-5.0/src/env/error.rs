use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn glp_vprintf(fmt: *const libc::c_char, arg: ::std::ffi::VaList);
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _glp_get_env_ptr() -> *mut ENV;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut libc::c_char,
    pub term_out: libc::c_int,
    pub term_hook: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: libc::c_int,
    pub err_file: *const libc::c_char,
    pub err_line: libc::c_int,
    pub err_hook: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut libc::c_char,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: libc::c_int,
    pub mem_cpeak: libc::c_int,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: libc::c_int,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
unsafe extern "C" fn errfunc(mut fmt: *const libc::c_char, mut args: ...) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut arg: ::std::ffi::VaListImpl;
    (*env).err_st = 1 as libc::c_int;
    (*env).term_out = 1 as libc::c_int;
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    glp_printf(
        b"Error detected in file %s at line %d\n\0" as *const u8 as *const libc::c_char,
        (*env).err_file,
        (*env).err_line,
    );
    if ((*env).err_hook).is_some() {
        ((*env).err_hook).unwrap()((*env).err_info);
    }
    abort();
}
pub unsafe extern "C" fn glp_error_(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> glp_errfunc {
    let mut env: *mut ENV = _glp_get_env_ptr();
    (*env).err_file = file;
    (*env).err_line = line;
    return Some(errfunc as unsafe extern "C" fn(*const libc::c_char, ...) -> ());
}
pub unsafe extern "C" fn glp_at_error() -> libc::c_int {
    let mut env: *mut ENV = _glp_get_env_ptr();
    return (*env).err_st;
}
pub unsafe extern "C" fn glp_assert_(
    mut expr: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    (glp_error_(file, line))
        .unwrap()(b"Assertion failed: %s\n\0" as *const u8 as *const libc::c_char, expr);
}
pub unsafe extern "C" fn glp_error_hook(
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut info: *mut libc::c_void,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if func.is_none() {
        (*env).err_hook = None;
        (*env).err_info = 0 as *mut libc::c_void;
    } else {
        (*env).err_hook = func;
        (*env).err_info = info;
    };
}
pub unsafe extern "C" fn _glp_put_err_msg(mut msg: *const libc::c_char) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut len: libc::c_int = 0;
    len = strlen(msg) as libc::c_int;
    if len >= 1024 as libc::c_int {
        len = 1024 as libc::c_int - 1 as libc::c_int;
    }
    memcpy(
        (*env).err_buf as *mut libc::c_void,
        msg as *const libc::c_void,
        len as libc::c_ulong,
    );
    if len > 0 as libc::c_int
        && *((*env).err_buf).offset((len - 1 as libc::c_int) as isize) as libc::c_int
            == '\n' as i32
    {
        len -= 1;
        len;
    }
    *((*env).err_buf).offset(len as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn _glp_get_err_msg() -> *const libc::c_char {
    let mut env: *mut ENV = _glp_get_env_ptr();
    return (*env).err_buf;
}

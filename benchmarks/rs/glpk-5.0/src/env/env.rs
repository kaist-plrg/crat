use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn abort() -> !;
    fn _glp_dlclose(h: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn _glp_tls_get_ptr() -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn _glp_tls_set_ptr(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
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
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub unsafe extern "C" fn glp_init_env() -> libc::c_int {
    let mut env: *mut ENV = 0 as *mut ENV;
    let mut ok: libc::c_int = 0;
    ok = (8 as libc::c_int == 8 as libc::c_int
        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            == 1 as libc::c_int as libc::c_ulong
        && ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
            == 2 as libc::c_int as libc::c_ulong
        && ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        && (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
            || ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                == 8 as libc::c_int as libc::c_ulong)) as libc::c_int;
    if ok == 0 {
        return 3 as libc::c_int;
    }
    if !(_glp_tls_get_ptr()).is_null() {
        return 1 as libc::c_int;
    }
    env = malloc(::std::mem::size_of::<ENV>() as libc::c_ulong) as *mut ENV;
    if env.is_null() {
        return 2 as libc::c_int;
    }
    memset(
        env as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ENV>() as libc::c_ulong,
    );
    (*env).self_0 = env;
    (*env).term_buf = malloc(4096 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if ((*env).term_buf).is_null() {
        free(env as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*env).term_out = 1 as libc::c_int;
    (*env).term_hook = None;
    (*env).term_info = 0 as *mut libc::c_void;
    (*env).tee_file = 0 as *mut FILE;
    (*env).err_st = 0 as libc::c_int;
    (*env).err_file = 0 as *const libc::c_char;
    (*env).err_line = 0 as libc::c_int;
    (*env).err_hook = None;
    (*env).err_info = 0 as *mut libc::c_void;
    (*env).err_buf = malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if ((*env).err_buf).is_null() {
        free((*env).term_buf as *mut libc::c_void);
        free(env as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    *((*env).err_buf).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*env).mem_limit = !(0 as libc::c_int as size_t);
    (*env).mem_ptr = 0 as *mut MBD;
    (*env).mem_cpeak = 0 as libc::c_int;
    (*env).mem_count = (*env).mem_cpeak;
    (*env).mem_tpeak = 0 as libc::c_int as size_t;
    (*env).mem_total = (*env).mem_tpeak;
    (*env).gmp_pool = 0 as *mut libc::c_void;
    (*env).gmp_size = 0 as libc::c_int;
    (*env).gmp_work = 0 as *mut libc::c_ushort;
    (*env).h_mysql = 0 as *mut libc::c_void;
    (*env).h_odbc = (*env).h_mysql;
    _glp_tls_set_ptr(env as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_get_env_ptr() -> *mut ENV {
    let mut env: *mut ENV = _glp_tls_get_ptr() as *mut ENV;
    if env.is_null() {
        if glp_init_env() != 0 as libc::c_int {
            fprintf(
                stderr,
                b"GLPK initialization failed\n\0" as *const u8 as *const libc::c_char,
            );
            fflush(stderr);
            abort();
        }
        env = _glp_tls_get_ptr() as *mut ENV;
    }
    if (*env).self_0 != env {
        fprintf(
            stderr,
            b"Invalid GLPK environment\n\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        abort();
    }
    return env;
}
pub unsafe extern "C" fn glp_version() -> *const libc::c_char {
    return b"5.0\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn glp_config(
    mut option: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if strcmp(option, b"TLS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        s = b"_Thread_local\0" as *const u8 as *const libc::c_char;
    } else if strcmp(option, b"ODBC_DLNAME\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        s = 0 as *const libc::c_char;
    } else if strcmp(option, b"MYSQL_DLNAME\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        s = 0 as *const libc::c_char;
    } else {
        s = 0 as *const libc::c_char;
    }
    return s;
}
pub unsafe extern "C" fn glp_free_env() -> libc::c_int {
    let mut env: *mut ENV = _glp_tls_get_ptr() as *mut ENV;
    let mut desc: *mut MBD = 0 as *mut MBD;
    if env.is_null() {
        return 1 as libc::c_int;
    }
    if (*env).self_0 != env {
        fprintf(
            stderr,
            b"Invalid GLPK environment\n\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        abort();
    }
    if !((*env).h_odbc).is_null() {
        _glp_dlclose((*env).h_odbc);
    }
    if !((*env).h_mysql).is_null() {
        _glp_dlclose((*env).h_mysql);
    }
    while !((*env).mem_ptr).is_null() {
        desc = (*env).mem_ptr;
        (*env).mem_ptr = (*desc).next;
        free(desc as *mut libc::c_void);
    }
    if !((*env).tee_file).is_null() {
        fclose((*env).tee_file);
    }
    (*env).self_0 = 0 as *mut ENV;
    free((*env).term_buf as *mut libc::c_void);
    free((*env).err_buf as *mut libc::c_void);
    free(env as *mut libc::c_void);
    _glp_tls_set_ptr(0 as *mut libc::c_void);
    return 0 as libc::c_int;
}

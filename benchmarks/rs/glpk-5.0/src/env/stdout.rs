use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub unsafe extern "C" fn glp_puts(mut s: *const libc::c_char) {
    let mut current_block: u64;
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).term_out == 0) {
        if ((*env).term_hook).is_some() {
            if ((*env).term_hook).unwrap()((*env).term_info, s) != 0 as libc::c_int {
                current_block = 16661367652742375680;
            } else {
                current_block = 10680521327981672866;
            }
        } else {
            current_block = 10680521327981672866;
        }
        match current_block {
            16661367652742375680 => {}
            _ => {
                fputs(s, stdout);
                fflush(stdout);
                if !((*env).tee_file).is_null() {
                    fputs(s, (*env).tee_file);
                    fflush((*env).tee_file);
                }
            }
        }
    }
}
pub unsafe extern "C" fn glp_printf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut arg: ::std::ffi::VaListImpl;
    if !((*env).term_out == 0) {
        arg = args.clone();
        vsprintf((*env).term_buf, fmt, arg.as_va_list());
        if strlen((*env).term_buf) < 4096 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"env/stdout.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void glp_printf(const char *, ...)\0"))
                    .as_ptr(),
            );
        }
        'c_4022: {
            if strlen((*env).term_buf) < 4096 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8
                        as *const libc::c_char,
                    b"env/stdout.c\0" as *const u8 as *const libc::c_char,
                    82 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"void glp_printf(const char *, ...)\0"))
                        .as_ptr(),
                );
            }
        };
        glp_puts((*env).term_buf);
    }
}
pub unsafe extern "C" fn glp_vprintf(
    mut fmt: *const libc::c_char,
    mut arg: ::std::ffi::VaList,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).term_out == 0) {
        vsprintf((*env).term_buf, fmt, arg.as_va_list());
        if strlen((*env).term_buf) < 4096 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"env/stdout.c\0" as *const u8 as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void glp_vprintf(const char *, struct __va_list_tag *)\0"))
                    .as_ptr(),
            );
        }
        'c_4131: {
            if strlen((*env).term_buf) < 4096 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8
                        as *const libc::c_char,
                    b"env/stdout.c\0" as *const u8 as *const libc::c_char,
                    112 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void glp_vprintf(const char *, struct __va_list_tag *)\0"))
                        .as_ptr(),
                );
            }
        };
        glp_puts((*env).term_buf);
    }
}
pub unsafe extern "C" fn glp_term_out(mut flag: libc::c_int) -> libc::c_int {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut old: libc::c_int = (*env).term_out;
    if !(flag == 1 as libc::c_int || flag == 0 as libc::c_int) {
        (glp_error_(
            b"env/stdout.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
        ))
            .unwrap()(
            b"glp_term_out: flag = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            flag,
        );
    }
    (*env).term_out = flag;
    return old;
}
pub unsafe extern "C" fn glp_term_hook(
    mut func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if func.is_none() {
        (*env).term_hook = None;
        (*env).term_info = 0 as *mut libc::c_void;
    } else {
        (*env).term_hook = func;
        (*env).term_info = info;
    };
}
pub unsafe extern "C" fn glp_open_tee(mut name: *const libc::c_char) -> libc::c_int {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).tee_file).is_null() {
        return 1 as libc::c_int;
    }
    (*env).tee_file = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if ((*env).tee_file).is_null() {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_close_tee() -> libc::c_int {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if ((*env).tee_file).is_null() {
        return 1 as libc::c_int;
    }
    fclose((*env).tee_file);
    (*env).tee_file = 0 as *mut FILE;
    return 0 as libc::c_int;
}

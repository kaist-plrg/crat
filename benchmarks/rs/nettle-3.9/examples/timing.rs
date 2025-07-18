use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn clock() -> clock_t;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
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
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = libc::c_long;
pub type va_list = __builtin_va_list;
pub type clockid_t = __clockid_t;
pub type __clockid_t = libc::c_int;
pub static mut cgt_start: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
unsafe extern "C" fn die(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    exit(1 as libc::c_int);
}
unsafe extern "C" fn cgt_works_p() -> libc::c_int {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    return (clock_gettime(2 as libc::c_int, &mut now) == 0 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn cgt_time_start() {
    if clock_gettime(2 as libc::c_int, &mut cgt_start) < 0 as libc::c_int {
        die(
            b"clock_gettime failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn cgt_time_end() -> libc::c_double {
    let mut end: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(2 as libc::c_int, &mut end) < 0 as libc::c_int {
        die(
            b"clock_gettime failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return (end.tv_sec - cgt_start.tv_sec) as libc::c_double
        + 1e-9f64 * (end.tv_nsec - cgt_start.tv_nsec) as libc::c_double;
}
static mut clock_start: clock_t = 0;
unsafe extern "C" fn clock_time_start() {
    clock_start = clock();
}
unsafe extern "C" fn clock_time_end() -> libc::c_double {
    return (clock() - clock_start) as libc::c_double
        / 1000000 as libc::c_int as __clock_t as libc::c_double;
}
pub static mut time_start: Option::<unsafe extern "C" fn() -> ()> = unsafe {
    Some(clock_time_start as unsafe extern "C" fn() -> ())
};
pub static mut time_end: Option::<unsafe extern "C" fn() -> libc::c_double> = unsafe {
    Some(clock_time_end as unsafe extern "C" fn() -> libc::c_double)
};
pub unsafe extern "C" fn time_init() {
    if cgt_works_p() != 0 {
        time_start = Some(cgt_time_start as unsafe extern "C" fn() -> ());
        time_end = Some(cgt_time_end as unsafe extern "C" fn() -> libc::c_double);
    } else {
        fprintf(
            stderr,
            b"clock_gettime not working, falling back to clock\n\0" as *const u8
                as *const libc::c_char,
        );
        time_start = Some(clock_time_start as unsafe extern "C" fn() -> ());
        time_end = Some(clock_time_end as unsafe extern "C" fn() -> libc::c_double);
    };
}

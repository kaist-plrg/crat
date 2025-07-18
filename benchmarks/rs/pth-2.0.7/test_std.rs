use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pth_st;
    pub type pth_attr_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn pth_init() -> libc::c_int;
    fn pth_kill() -> libc::c_int;
    fn pth_version() -> libc::c_long;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: libc::c_int, _: ...) -> libc::c_int;
    fn pth_attr_destroy(_: pth_attr_t) -> libc::c_int;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_yield(_: pth_t) -> libc::c_int;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> libc::c_int;
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
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
pub type C2RustUnnamed = libc::c_uint;
pub const PTH_ATTR_BOUND: C2RustUnnamed = 14;
pub const PTH_ATTR_EVENTS: C2RustUnnamed = 13;
pub const PTH_ATTR_STATE: C2RustUnnamed = 12;
pub const PTH_ATTR_START_ARG: C2RustUnnamed = 11;
pub const PTH_ATTR_START_FUNC: C2RustUnnamed = 10;
pub const PTH_ATTR_TIME_RAN: C2RustUnnamed = 9;
pub const PTH_ATTR_TIME_LAST: C2RustUnnamed = 8;
pub const PTH_ATTR_TIME_SPAWN: C2RustUnnamed = 7;
pub const PTH_ATTR_DISPATCHES: C2RustUnnamed = 6;
pub const PTH_ATTR_STACK_ADDR: C2RustUnnamed = 5;
pub const PTH_ATTR_STACK_SIZE: C2RustUnnamed = 4;
pub const PTH_ATTR_CANCEL_STATE: C2RustUnnamed = 3;
pub const PTH_ATTR_JOINABLE: C2RustUnnamed = 2;
pub const PTH_ATTR_NAME: C2RustUnnamed = 1;
pub const PTH_ATTR_PRIO: C2RustUnnamed = 0;
unsafe extern "C" fn t1_func(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_long = 0;
    val = arg as libc::c_long;
    if val != 123 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        val += 10 as libc::c_int as libc::c_long;
        pth_yield(0 as pth_t);
        i += 1;
        i;
    }
    return val as *mut libc::c_void;
}
unsafe extern "C" fn t2_func(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut val: libc::c_long = 0;
    let mut tid: pth_t = 0 as *mut pth_st;
    let mut rval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc: libc::c_int = 0;
    val = arg as libc::c_long;
    if val < 9 as libc::c_int as libc::c_long {
        val += 1;
        val;
        fprintf(
            stderr,
            b"Spawning thread %ld\n\0" as *const u8 as *const libc::c_char,
            val,
        );
        tid = pth_spawn(
            0 as pth_attr_t,
            Some(
                t2_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            val as *mut libc::c_void,
        );
        if tid.is_null() {
            fprintf(
                stderr,
                b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                    as *const libc::c_char,
                *__errno_location(),
            );
            exit(1 as libc::c_int);
        }
        rc = pth_join(tid, &mut rval);
        fprintf(
            stderr,
            b"Joined thread %ld\n\0" as *const u8 as *const libc::c_char,
            val,
        );
        if rc == 0 as libc::c_int {
            fprintf(
                stderr,
                b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                    as *const libc::c_char,
                *__errno_location(),
            );
            exit(1 as libc::c_int);
        }
        rval = (arg as libc::c_long * rval as libc::c_long) as *mut libc::c_void;
    } else {
        rval = arg;
    }
    return rval;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    fprintf(
        stderr,
        b"\n=== TESTING GLOBAL LIBRARY API ===\n\n\0" as *const u8 as *const libc::c_char,
    );
    let mut version: libc::c_int = 0;
    fprintf(stderr, b"Fetching library version\n\0" as *const u8 as *const libc::c_char);
    version = pth_version() as libc::c_int;
    if version == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    fprintf(stderr, b"version = 0x%X\n\0" as *const u8 as *const libc::c_char, version);
    fprintf(
        stderr,
        b"\n=== TESTING BASIC OPERATION ===\n\n\0" as *const u8 as *const libc::c_char,
    );
    let mut rc: libc::c_int = 0;
    fprintf(
        stderr,
        b"Initializing Pth system (spawns scheduler and main thread)\n\0" as *const u8
            as *const libc::c_char,
    );
    rc = pth_init();
    if rc == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"Killing Pth system for testing purposes\n\0" as *const u8
            as *const libc::c_char,
    );
    pth_kill();
    fprintf(
        stderr,
        b"Re-Initializing Pth system\n\0" as *const u8 as *const libc::c_char,
    );
    rc = pth_init();
    if rc == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"\n=== TESTING BASIC THREAD OPERATION ===\n\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut tid: pth_t = 0 as *mut pth_st;
    let mut val: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc_0: libc::c_int = 0;
    fprintf(
        stderr,
        b"Creating attribute object\n\0" as *const u8 as *const libc::c_char,
    );
    attr = pth_attr_new();
    if attr.is_null() {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    rc_0 = pth_attr_set(
        attr,
        PTH_ATTR_NAME as libc::c_int,
        b"test1\0" as *const u8 as *const libc::c_char,
    );
    if rc_0 == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    rc_0 = pth_attr_set(attr, PTH_ATTR_PRIO as libc::c_int, 5 as libc::c_int);
    if rc_0 == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    fprintf(stderr, b"Spawning thread\n\0" as *const u8 as *const libc::c_char);
    tid = pth_spawn(
        attr,
        Some(t1_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        123 as libc::c_int as *mut libc::c_void,
    );
    if tid.is_null() {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    pth_attr_destroy(attr);
    fprintf(stderr, b"Joining thread\n\0" as *const u8 as *const libc::c_char);
    rc_0 = pth_join(tid, &mut val);
    if rc_0 == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    if val != 1123 as libc::c_int as *mut libc::c_void {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"\n=== TESTING NESTED THREAD OPERATION ===\n\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut tid_0: pth_t = 0 as *mut pth_st;
    let mut val_0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc_1: libc::c_int = 0;
    fprintf(stderr, b"Spawning thread 1\n\0" as *const u8 as *const libc::c_char);
    tid_0 = pth_spawn(
        0 as pth_attr_t,
        Some(t2_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        1 as libc::c_int as *mut libc::c_void,
    );
    if tid_0.is_null() {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    rc_1 = pth_join(tid_0, &mut val_0);
    fprintf(stderr, b"Joined thread 1\n\0" as *const u8 as *const libc::c_char);
    if rc_1 == 0 as libc::c_int {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    if val_0
        != (1 as libc::c_int * 2 as libc::c_int * 3 as libc::c_int * 4 as libc::c_int
            * 5 as libc::c_int * 6 as libc::c_int * 7 as libc::c_int * 8 as libc::c_int
            * 9 as libc::c_int) as *mut libc::c_void
    {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
        exit(1 as libc::c_int);
    }
    pth_kill();
    fprintf(
        stderr,
        b"\nOK - ALL TESTS SUCCESSFULLY PASSED.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

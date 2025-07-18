use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pth_st;
    pub type pth_attr_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn pth_init() -> libc::c_int;
    fn pth_kill() -> libc::c_int;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: libc::c_int, _: ...) -> libc::c_int;
    fn pth_attr_destroy(_: pth_attr_t) -> libc::c_int;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_cancel(_: pth_t) -> libc::c_int;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> libc::c_int;
    fn pth_cleanup_push(
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pth_sleep(_: libc::c_uint) -> libc::c_uint;
    fn pth_sigmask(_: libc::c_int, _: *const sigset_t, _: *mut sigset_t) -> libc::c_int;
    fn pth_sigwait(_: *const sigset_t, _: *mut libc::c_int) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
static mut child1: pth_t = 0 as *const pth_st as *mut pth_st;
static mut child2: pth_t = 0 as *const pth_st as *mut pth_st;
unsafe extern "C" fn inthandler(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    fprintf(stderr, b"inthandler: enter\n\0" as *const u8 as *const libc::c_char);
    sigemptyset(&mut sigs);
    sigaddset(&mut sigs, 2 as libc::c_int);
    pth_sigmask(1 as libc::c_int, &mut sigs, 0 as *mut sigset_t);
    n = 0 as libc::c_int;
    while n < 3 as libc::c_int {
        pth_sigwait(&mut sigs, &mut sig);
        fprintf(
            stderr,
            b"inthandler: SIGINT received (#%d)\n\0" as *const u8 as *const libc::c_char,
            n,
        );
        n += 1;
        n;
    }
    fprintf(
        stderr,
        b"inthandler: cancelling child1 and child2\n\0" as *const u8
            as *const libc::c_char,
    );
    pth_cancel(child1);
    pth_cancel(child2);
    fprintf(stderr, b"inthandler: leave\n\0" as *const u8 as *const libc::c_char);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn child_cleanup(mut arg: *mut libc::c_void) {
    fprintf(
        stderr,
        b"%s: running cleanup\n\0" as *const u8 as *const libc::c_char,
        arg as *mut libc::c_char,
    );
}
unsafe extern "C" fn child(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut name: *mut libc::c_char = _arg as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    fprintf(stderr, b"%s: enter\n\0" as *const u8 as *const libc::c_char, name);
    pth_cleanup_push(
        Some(child_cleanup as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        name as *mut libc::c_void,
    );
    pth_sigmask(2 as libc::c_int, 0 as *const sigset_t, &mut sigs);
    sigaddset(&mut sigs, 2 as libc::c_int);
    if strcmp(name, b"child1\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        sigaddset(&mut sigs, 10 as libc::c_int);
        sigdelset(&mut sigs, 12 as libc::c_int);
    } else {
        sigdelset(&mut sigs, 10 as libc::c_int);
        sigaddset(&mut sigs, 12 as libc::c_int);
    }
    pth_sigmask(2 as libc::c_int, &mut sigs, 0 as *mut sigset_t);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pth_sigmask(2 as libc::c_int, 0 as *const sigset_t, &mut sigs);
        fprintf(
            stderr,
            b"%s: SIGUSR1: %s\n\0" as *const u8 as *const libc::c_char,
            name,
            if sigismember(&mut sigs, 10 as libc::c_int) != 0 {
                b"blocked\0" as *const u8 as *const libc::c_char
            } else {
                b"unblocked\0" as *const u8 as *const libc::c_char
            },
        );
        fprintf(
            stderr,
            b"%s: SIGUSR2: %s\n\0" as *const u8 as *const libc::c_char,
            name,
            if sigismember(&mut sigs, 12 as libc::c_int) != 0 {
                b"blocked\0" as *const u8 as *const libc::c_char
            } else {
                b"unblocked\0" as *const u8 as *const libc::c_char
            },
        );
        fprintf(
            stderr,
            b"%s: leave to scheduler\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        pth_sleep(1 as libc::c_int as libc::c_uint);
        fprintf(
            stderr,
            b"%s: reentered from scheduler\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"%s: leave\n\0" as *const u8 as *const libc::c_char, name);
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    pth_init();
    fprintf(
        stderr,
        b"This is TEST_SIG, a Pth test using signals.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Hit CTRL-C three times to stop this test.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"But only after all threads were terminated.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"main: init\n\0" as *const u8 as *const libc::c_char);
    pth_sigmask(2 as libc::c_int, 0 as *const sigset_t, &mut sigs);
    sigaddset(&mut sigs, 10 as libc::c_int);
    sigaddset(&mut sigs, 12 as libc::c_int);
    sigaddset(&mut sigs, 2 as libc::c_int);
    pth_sigmask(2 as libc::c_int, &mut sigs, 0 as *mut sigset_t);
    attr = pth_attr_new();
    pth_attr_set(
        attr,
        PTH_ATTR_NAME as libc::c_int,
        b"child1\0" as *const u8 as *const libc::c_char,
    );
    child1 = pth_spawn(
        attr,
        Some(child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"child1\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        attr,
        PTH_ATTR_NAME as libc::c_int,
        b"child2\0" as *const u8 as *const libc::c_char,
    );
    child2 = pth_spawn(
        attr,
        Some(child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"child2\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        attr,
        PTH_ATTR_NAME as libc::c_int,
        b"inthandler\0" as *const u8 as *const libc::c_char,
    );
    pth_spawn(
        attr,
        Some(inthandler as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"inthandler\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_destroy(attr);
    while pth_join(0 as pth_t, 0 as *mut *mut libc::c_void) != 0 {}
    fprintf(stderr, b"main: exit\n\0" as *const u8 as *const libc::c_char);
    pth_kill();
    return 0 as libc::c_int;
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

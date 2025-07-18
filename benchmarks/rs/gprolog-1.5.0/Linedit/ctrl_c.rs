use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
static mut ctrl_c_handler: Option::<unsafe extern "C" fn() -> PlLong> = None;
static mut from_callback: libc::c_int = 0;
static mut ret_val: PlLong = 0;
static mut inside_ctrl_c: libc::c_int = 0;
unsafe extern "C" fn Wrapper_Handler(mut sig: libc::c_int) {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut from_callback1: libc::c_int = 0;
    sigemptyset(&mut set);
    sigaddset(&mut set, sig);
    sigprocmask(1 as libc::c_int, &mut set, 0 as *mut sigset_t);
    if inside_ctrl_c != 0 {
        printf(
            b"Already in a Ctrl+C handler - ignored\n\0" as *const u8
                as *const libc::c_char,
        );
        fflush(stdout);
        ret_val = 0 as libc::c_int as PlLong;
    } else {
        inside_ctrl_c = from_callback;
        from_callback1 = from_callback;
        from_callback = 0 as libc::c_int;
        ret_val = ::std::mem::transmute::<
            _,
            fn(_) -> PlLong,
        >((Some(ctrl_c_handler.unwrap())).unwrap())(from_callback1);
    }
    inside_ctrl_c = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Emit_Ctrl_C() -> PlLong {
    from_callback = 1 as libc::c_int;
    kill(getpid(), 2 as libc::c_int);
    return ret_val;
}
pub unsafe extern "C" fn Pl_Install_Ctrl_C_Handler(
    mut handler: Option::<unsafe extern "C" fn(libc::c_int) -> PlLong>,
) {
    ctrl_c_handler = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(libc::c_int) -> PlLong>,
        Option::<unsafe extern "C" fn() -> PlLong>,
    >(handler);
    signal(
        2 as libc::c_int,
        Some(Wrapper_Handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}

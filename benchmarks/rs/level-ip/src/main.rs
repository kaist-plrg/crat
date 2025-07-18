use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn parse_cli(argc: libc::c_int, argv: *mut *mut libc::c_char);
    fn tun_init();
    fn free_tun();
    fn start_ipc_listener() -> *mut libc::c_void;
    fn timers_start() -> *mut libc::c_void;
    fn route_init();
    fn free_routes();
    fn netdev_init(_: *mut libc::c_char, _: *mut libc::c_char);
    fn netdev_rx_loop() -> *mut libc::c_void;
    fn free_netdev();
    fn arp_init();
    fn free_arp();
    fn abort_sockets();
    fn tcp_init();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
static mut threads: [pthread_t; 4] = [0; 4];
pub static mut running: libc::c_int = 1 as libc::c_int;
pub static mut mask: sigset_t = sigset_t { __val: [0; 16] };
unsafe extern "C" fn create_thread(
    mut id: pthread_t,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
) {
    if pthread_create(
        &mut *threads.as_mut_ptr().offset(id as isize),
        0 as *const pthread_attr_t,
        func,
        0 as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Could not create core thread\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn stop_stack_handler(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut err: libc::c_int = 0;
    let mut signo: libc::c_int = 0;
    loop {
        err = sigwait(&mut mask, &mut signo);
        if err != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Sigwait failed: %d\n\0" as *const u8 as *const libc::c_char,
                err,
            );
        }
        match signo {
            2 | 3 => {
                running = 0 as libc::c_int;
                pthread_cancel(threads[2 as libc::c_int as usize]);
                pthread_cancel(threads[0 as libc::c_int as usize]);
                pthread_cancel(threads[1 as libc::c_int as usize]);
                return 0 as *mut libc::c_void;
            }
            _ => {
                printf(
                    b"Unexpected signal %d\n\0" as *const u8 as *const libc::c_char,
                    signo,
                );
            }
        }
    };
}
unsafe extern "C" fn init_signals() {
    let mut err: libc::c_int = 0;
    sigemptyset(&mut mask);
    sigaddset(&mut mask, 2 as libc::c_int);
    sigaddset(&mut mask, 3 as libc::c_int);
    err = pthread_sigmask(0 as libc::c_int, &mut mask, 0 as *mut __sigset_t);
    if err != 0 as libc::c_int {
        fprintf(stderr, b"SIG_BLOCK error\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn init_stack() {
    tun_init();
    netdev_init(0 as *mut libc::c_char, 0 as *mut libc::c_char);
    route_init();
    arp_init();
    tcp_init();
}
unsafe extern "C" fn run_threads() {
    create_thread(
        0 as libc::c_int as pthread_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> *mut libc::c_void,
                    unsafe extern "C" fn() -> *mut libc::c_void,
                >(netdev_rx_loop),
            ),
        ),
    );
    create_thread(
        1 as libc::c_int as pthread_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> *mut libc::c_void,
                    unsafe extern "C" fn() -> *mut libc::c_void,
                >(timers_start),
            ),
        ),
    );
    create_thread(
        2 as libc::c_int as pthread_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> *mut libc::c_void,
                    unsafe extern "C" fn() -> *mut libc::c_void,
                >(start_ipc_listener),
            ),
        ),
    );
    create_thread(
        3 as libc::c_int as pthread_t,
        Some(
            stop_stack_handler
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn wait_for_threads() {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if pthread_join(threads[i as usize], 0 as *mut *mut libc::c_void)
            != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error when joining threads\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn free_stack() {
    abort_sockets();
    free_arp();
    free_routes();
    free_netdev();
    free_tun();
}
pub unsafe extern "C" fn init_security() {
    if prctl(24 as libc::c_int, 12 as libc::c_int) == -(1 as libc::c_int) {
        perror(
            b"Error on network admin capability drop\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if prctl(24 as libc::c_int, 8 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"Error on capability set drop\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    parse_cli(argc, argv);
    init_signals();
    init_stack();
    init_security();
    run_threads();
    wait_for_threads();
    free_stack();
    return 0;
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

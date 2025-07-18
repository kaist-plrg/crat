use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pth_uctx_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn pth_uctx_create(_: *mut pth_uctx_t) -> libc::c_int;
    fn pth_uctx_make(
        _: pth_uctx_t,
        _: *mut libc::c_char,
        _: size_t,
        _: *const sigset_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
        _: pth_uctx_t,
    ) -> libc::c_int;
    fn pth_uctx_switch(_: pth_uctx_t, _: pth_uctx_t) -> libc::c_int;
    fn pth_uctx_destroy(_: pth_uctx_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pth_uctx_t = *mut pth_uctx_st;
pub static mut uctx: [pth_uctx_t; 10] = [0 as *const pth_uctx_st
    as *mut pth_uctx_st; 10];
pub static mut worker_done: [libc::c_int; 10] = [0; 10];
unsafe extern "C" fn worker(mut ctx: *mut libc::c_void) {
    let mut n: libc::c_int = ctx as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    fprintf(stderr, b"worker #%d: enter\n\0" as *const u8 as *const libc::c_char, n);
    ::std::ptr::write_volatile(&mut i as *mut libc::c_int, 0 as libc::c_int);
    while i < 100 as libc::c_int {
        fprintf(
            stderr,
            b"worker #%d: working (step %d)\n\0" as *const u8 as *const libc::c_char,
            n,
            i,
        );
        pth_uctx_switch(uctx[n as usize], uctx[0 as libc::c_int as usize]);
        ::std::ptr::write_volatile(
            &mut i as *mut libc::c_int,
            ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int) + 1,
        );
        ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int);
    }
    ::std::ptr::write_volatile(
        &mut worker_done[n as usize] as *mut libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    fprintf(stderr, b"worker #%d: exit\n\0" as *const u8 as *const libc::c_char, n);
}
unsafe extern "C" fn test_working() {
    let mut i: libc::c_int = 0;
    let mut todo: libc::c_int = 0;
    fprintf(stderr, b"master: startup\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"master: create contexts\n\0" as *const u8 as *const libc::c_char);
    pth_uctx_create(
        &mut *uctx.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut pth_uctx_t
            as *mut pth_uctx_t,
    );
    ::std::ptr::write_volatile(
        &mut worker_done[0 as libc::c_int as usize] as *mut libc::c_int,
        0 as libc::c_int,
    );
    ::std::ptr::write_volatile(&mut i as *mut libc::c_int, 1 as libc::c_int);
    while i < 10 as libc::c_int {
        ::std::ptr::write_volatile(
            &mut worker_done[i as usize] as *mut libc::c_int,
            0 as libc::c_int,
        );
        pth_uctx_create(
            &mut *uctx.as_mut_ptr().offset(i as isize) as *mut pth_uctx_t
                as *mut pth_uctx_t,
        );
        pth_uctx_make(
            uctx[i as usize],
            0 as *mut libc::c_char,
            (32 as libc::c_int * 1024 as libc::c_int) as size_t,
            0 as *const sigset_t,
            Some(worker as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            i as *mut libc::c_void,
            uctx[0 as libc::c_int as usize],
        );
        ::std::ptr::write_volatile(
            &mut i as *mut libc::c_int,
            ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int) + 1,
        );
        ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int);
    }
    loop {
        ::std::ptr::write_volatile(&mut todo as *mut libc::c_int, 0 as libc::c_int);
        ::std::ptr::write_volatile(&mut i as *mut libc::c_int, 1 as libc::c_int);
        while i < 10 as libc::c_int {
            if worker_done[i as usize] == 0 {
                fprintf(
                    stderr,
                    b"master: switching to worker #%d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
                pth_uctx_switch(uctx[0 as libc::c_int as usize], uctx[i as usize]);
                fprintf(
                    stderr,
                    b"master: came back from worker #%d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
                ::std::ptr::write_volatile(
                    &mut todo as *mut libc::c_int,
                    1 as libc::c_int,
                );
            }
            ::std::ptr::write_volatile(
                &mut i as *mut libc::c_int,
                ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int) + 1,
            );
            ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int);
        }
        if !(todo != 0) {
            break;
        }
    }
    fprintf(stderr, b"master: destroy contexts\n\0" as *const u8 as *const libc::c_char);
    ::std::ptr::write_volatile(&mut i as *mut libc::c_int, 1 as libc::c_int);
    while i < 10 as libc::c_int {
        pth_uctx_destroy(uctx[i as usize]);
        ::std::ptr::write_volatile(
            &mut i as *mut libc::c_int,
            ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int) + 1,
        );
        ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int);
    }
    pth_uctx_destroy(uctx[0 as libc::c_int as usize]);
    fprintf(stderr, b"master: exit\n\0" as *const u8 as *const libc::c_char);
}
pub static mut stat_start: time_t = 0;
pub static mut stat_end: time_t = 0;
pub static mut stat_switched: libc::c_int = 0;
unsafe extern "C" fn dummy(mut ctx: *mut libc::c_void) {
    loop {
        ::std::ptr::write_volatile(
            &mut stat_switched as *mut libc::c_int,
            ::std::ptr::read_volatile::<
                libc::c_int,
            >(&stat_switched as *const libc::c_int) + 1,
        );
        ::std::ptr::read_volatile::<libc::c_int>(&stat_switched as *const libc::c_int);
        pth_uctx_switch(
            uctx[1 as libc::c_int as usize],
            uctx[0 as libc::c_int as usize],
        );
    };
}
unsafe extern "C" fn test_performance() {
    let mut i: libc::c_int = 0;
    pth_uctx_create(
        &mut *uctx.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut pth_uctx_t
            as *mut pth_uctx_t,
    );
    pth_uctx_create(
        &mut *uctx.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut pth_uctx_t
            as *mut pth_uctx_t,
    );
    pth_uctx_make(
        uctx[1 as libc::c_int as usize],
        0 as *mut libc::c_char,
        (32 as libc::c_int * 1024 as libc::c_int) as size_t,
        0 as *const sigset_t,
        Some(dummy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
        uctx[0 as libc::c_int as usize],
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Performing %d user-space context switches... be patient!\n\0" as *const u8
            as *const libc::c_char,
        10000000 as libc::c_int,
    );
    stat_start = time(0 as *mut time_t);
    ::std::ptr::write_volatile(&mut stat_switched as *mut libc::c_int, 0 as libc::c_int);
    ::std::ptr::write_volatile(&mut i as *mut libc::c_int, 0 as libc::c_int);
    while i < 10000000 as libc::c_int {
        ::std::ptr::write_volatile(
            &mut stat_switched as *mut libc::c_int,
            ::std::ptr::read_volatile::<
                libc::c_int,
            >(&stat_switched as *const libc::c_int) + 1,
        );
        ::std::ptr::read_volatile::<libc::c_int>(&stat_switched as *const libc::c_int);
        pth_uctx_switch(
            uctx[0 as libc::c_int as usize],
            uctx[1 as libc::c_int as usize],
        );
        ::std::ptr::write_volatile(
            &mut i as *mut libc::c_int,
            ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int) + 1,
        );
        ::std::ptr::read_volatile::<libc::c_int>(&i as *const libc::c_int);
    }
    stat_end = time(0 as *mut time_t);
    pth_uctx_destroy(uctx[0 as libc::c_int as usize]);
    pth_uctx_destroy(uctx[1 as libc::c_int as usize]);
    fprintf(
        stderr,
        b"We required %d seconds for performing the test, so this means we can\n\0"
            as *const u8 as *const libc::c_char,
        (stat_end - stat_start) as libc::c_int,
    );
    fprintf(
        stderr,
        b"perform %d user-space context switches per second on this platform.\n\0"
            as *const u8 as *const libc::c_char,
        10000000 as libc::c_int / (stat_end - stat_start) as libc::c_int,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    test_working();
    test_performance();
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

use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn pth_yield(_: pth_t) -> libc::c_int;
    fn pth_cancel(_: pth_t) -> libc::c_int;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> libc::c_int;
    fn pth_timeout(_: libc::c_long, _: libc::c_long) -> pth_time_t;
    fn pth_event(_: libc::c_ulong, _: ...) -> pth_event_t;
    fn pth_event_free(_: pth_event_t, _: libc::c_int) -> libc::c_int;
    fn pth_select_ev(
        _: libc::c_int,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut timeval,
        _: pth_event_t,
    ) -> libc::c_int;
    fn pth_sleep(_: libc::c_uint) -> libc::c_uint;
    fn pth_read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type pth_time_t = timeval;
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
pub type pth_event_t = *mut pth_event_st;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTH_FREE_ALL: C2RustUnnamed_0 = 1;
pub const PTH_FREE_THIS: C2RustUnnamed_0 = 0;
unsafe extern "C" fn ticker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut now: time_t = 0;
    fprintf(stderr, b"ticker: start\n\0" as *const u8 as *const libc::c_char);
    loop {
        pth_sleep(5 as libc::c_int as libc::c_uint);
        now = time(0 as *mut time_t);
        fprintf(
            stderr,
            b"ticker was woken up on %s\0" as *const u8 as *const libc::c_char,
            ctime(&mut now),
        );
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut evt: pth_event_t = 0 as *mut pth_event_st;
    let mut t_ticker: pth_t = 0 as *mut pth_st;
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut c: libc::c_char = 0;
    let mut n: libc::c_int = 0;
    pth_init();
    fprintf(
        stderr,
        b"This is TEST_SELECT, a Pth test using select.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Enter data. Hit CTRL-C to stop this test.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    t_attr = pth_attr_new();
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"ticker\0" as *const u8 as *const libc::c_char,
    );
    t_ticker = pth_spawn(
        t_attr,
        Some(ticker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pth_attr_destroy(t_attr);
    pth_yield(0 as pth_t);
    evt = 0 as pth_event_t;
    loop {
        if evt.is_null() {
            evt = pth_event(
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong,
                pth_timeout(
                    10 as libc::c_int as libc::c_long,
                    0 as libc::c_int as libc::c_long,
                ),
            );
        } else {
            evt = pth_event(
                ((1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong,
                evt,
                pth_timeout(
                    10 as libc::c_int as libc::c_long,
                    0 as libc::c_int as libc::c_long,
                ),
            );
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(rfds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
            fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
            fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        rfds
            .__fds_bits[(0 as libc::c_int
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << 0 as libc::c_int
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        n = pth_select_ev(
            0 as libc::c_int + 1 as libc::c_int,
            &mut rfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
            evt,
        );
        if n == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int {
            fprintf(
                stderr,
                b"main: timeout - repeating\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if !(rfds
                .__fds_bits[(0 as libc::c_int
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << 0 as libc::c_int
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long)
            {
                fprintf(
                    stderr,
                    b"main: Hmmmm... strange situation: bit not set\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            fprintf(
                stderr,
                b"main: select returned %d\n\0" as *const u8 as *const libc::c_char,
                n,
            );
            while pth_read(
                0 as libc::c_int,
                &mut c as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) > 0 as libc::c_int as libc::c_long
            {
                fprintf(
                    stderr,
                    b"main: read stdin '%c'\n\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
            }
        }
    };
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

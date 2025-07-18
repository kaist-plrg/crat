use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut optind: libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_error(fmt: *const libc::c_char, _: ...);
    fn usage(fp: *mut FILE, exit_code: libc::c_int);
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn tb_init() -> libc::c_int;
    fn tb_shutdown() -> libc::c_int;
    fn tb_width() -> libc::c_int;
    fn tb_height() -> libc::c_int;
    fn tb_clear() -> libc::c_int;
    fn tb_present() -> libc::c_int;
    fn tb_peek_event(event: *mut tb_event, timeout_ms: libc::c_int) -> libc::c_int;
    fn tb_printf(
        x: libc::c_int,
        y: libc::c_int,
        fg: uintattr_t,
        bg: uintattr_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut opt_pgrep_args: *mut libc::c_char;
    static mut done: libc::c_int;
    static mut opt_pid: pid_t;
    fn perror(__s: *const libc::c_char);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type clockid_t = __clockid_t;
pub type ptrdiff_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uintattr_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tb_event {
    pub type_0: uint8_t,
    pub mod_0: uint8_t,
    pub key: uint16_t,
    pub ch: uint32_t,
    pub w: int32_t,
    pub h: int32_t,
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
}
pub type func_entry_t = func_entry_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct func_entry_s {
    pub func: [libc::c_char; 256],
    pub count_excl: libc::c_ulong,
    pub count_incl: libc::c_ulong,
    pub total_count_excl: libc::c_ulong,
    pub total_count_incl: libc::c_ulong,
    pub percent_excl: libc::c_float,
    pub hh: UT_hash_handle,
}
static mut func_map: *mut func_entry_t = 0 as *const func_entry_t as *mut func_entry_t;
static mut func_list: *mut *mut func_entry_t = 0 as *const *mut func_entry_t
    as *mut *mut func_entry_t;
static mut func_list_len: size_t = 0 as libc::c_int as size_t;
static mut func_list_size: size_t = 0 as libc::c_int as size_t;
static mut buf: [libc::c_char; 512] = [0; 512];
static mut buf_len: size_t = 0 as libc::c_int as size_t;
static mut total_samp_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut samp_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut total_err_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut is_paused: libc::c_int = 0 as libc::c_int;
static mut phpspy_args: [libc::c_char; 512] = [0; 512];
pub unsafe extern "C" fn main_top(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut ttyfd: libc::c_int = 0;
    let mut outfd: libc::c_int = 0;
    let mut errfd: libc::c_int = 0;
    let mut maxfd: libc::c_int = 0;
    let mut pid: pid_t = 0;
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut last_display: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut event: tb_event = tb_event {
        type_0: 0,
        mod_0: 0,
        key: 0,
        ch: 0,
        w: 0,
        h: 0,
        x: 0,
        y: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    pid = -(1 as libc::c_int);
    ttyfd = pid;
    errfd = ttyfd;
    outfd = errfd;
    if opt_pid == -(1 as libc::c_int) && opt_pgrep_args.is_null() && optind >= argc {
        log_error(
            b"Expected pid (-p), pgrep (-P), or command\n\n\0" as *const u8
                as *const libc::c_char,
        );
        usage(stderr, 1 as libc::c_int);
        return 1 as libc::c_int;
    }
    filter_child_args(argc, argv);
    snprintf(
        phpspy_args.as_mut_ptr(),
        512 as libc::c_int as libc::c_ulong,
        b"phpspy \0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i < argc {
        snprintf(
            phpspy_args.as_mut_ptr().offset(strlen(phpspy_args.as_mut_ptr()) as isize),
            (512 as libc::c_int as libc::c_ulong)
                .wrapping_sub(strlen(phpspy_args.as_mut_ptr())),
            b"%s \0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        );
        i += 1;
        i;
    }
    ttyfd = open(b"/dev/tty\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if ttyfd < 0 as libc::c_int {
        perror(b"open\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if fork_child(argc, argv, &mut pid, &mut outfd, &mut errfd) < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    maxfd = if (if ttyfd > outfd { ttyfd } else { outfd }) > errfd {
        if ttyfd > outfd { ttyfd } else { outfd }
    } else {
        errfd
    };
    last_display.tv_sec = 0 as libc::c_int as __time_t;
    last_display.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    tb_init();
    while done == 0 {
        clock_gettime(1 as libc::c_int, &mut ts);
        if last_display.tv_sec == 0 as libc::c_int as libc::c_long
            || ts.tv_sec - last_display.tv_sec >= 1 as libc::c_int as libc::c_long
        {
            display();
            last_display = ts;
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(readfds.fds_bits)
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
        readfds
            .fds_bits[(ttyfd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << ttyfd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        readfds
            .fds_bits[(outfd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << outfd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        readfds
            .fds_bits[(errfd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << errfd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        timeout.tv_sec = 1 as libc::c_int as __time_t;
        timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
        rc = select(
            maxfd + 1 as libc::c_int,
            &mut readfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut timeout,
        );
        if rc < 0 as libc::c_int {
            if *__errno_location() == 4 as libc::c_int {
                tb_peek_event(&mut event, 0 as libc::c_int);
                last_display.tv_sec = 0 as libc::c_int as __time_t;
            } else {
                perror(b"select\0" as *const u8 as *const libc::c_char);
                break;
            }
        } else {
            if readfds
                .fds_bits[(outfd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << outfd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                read_child_out(outfd);
            }
            if readfds
                .fds_bits[(errfd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << errfd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                read_child_err(errfd);
            }
            if readfds
                .fds_bits[(ttyfd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << ttyfd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                event.type_0 = 0 as libc::c_int as uint8_t;
                tb_peek_event(&mut event, 0 as libc::c_int);
                handle_event(&mut event);
            }
        }
    }
    tb_shutdown();
    close(outfd);
    close(errfd);
    close(ttyfd);
    kill(pid, 15 as libc::c_int);
    waitpid(pid, 0 as *mut libc::c_int, 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fork_child(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pid: *mut pid_t,
    mut outfd: *mut libc::c_int,
    mut errfd: *mut libc::c_int,
) -> libc::c_int {
    let mut pout: [libc::c_int; 2] = [0; 2];
    let mut perr: [libc::c_int; 2] = [0; 2];
    if pipe(pout.as_mut_ptr()) < 0 as libc::c_int
        || pipe(perr.as_mut_ptr()) < 0 as libc::c_int
    {
        perror(b"pipe\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    *pid = fork();
    if *pid == 0 as libc::c_int {
        close(pout[0 as libc::c_int as usize]);
        dup2(pout[1 as libc::c_int as usize], 1 as libc::c_int);
        close(pout[1 as libc::c_int as usize]);
        close(perr[0 as libc::c_int as usize]);
        dup2(perr[1 as libc::c_int as usize], 2 as libc::c_int);
        close(perr[1 as libc::c_int as usize]);
        execvp(
            *argv.offset(0 as libc::c_int as isize),
            argv as *const *mut libc::c_char,
        );
        perror(b"execvp\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else if *pid < 0 as libc::c_int {
        perror(b"fork\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    close(pout[1 as libc::c_int as usize]);
    close(perr[1 as libc::c_int as usize]);
    *outfd = pout[0 as libc::c_int as usize];
    *errfd = perr[0 as libc::c_int as usize];
    return 0 as libc::c_int;
}
unsafe extern "C" fn filter_child_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"-o\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
            || strncmp(
                *argv.offset(i as isize),
                b"--output\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            *(*argv.offset(i as isize))
                .offset(1 as libc::c_int as isize) = '#' as i32 as libc::c_char;
            *(*argv.offset(i as isize))
                .offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        if strncmp(
            *argv.offset(i as isize),
            b"-1\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
            || strncmp(
                *argv.offset(i as isize),
                b"--single-line\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            || strncmp(
                *argv.offset(i as isize),
                b"-t\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            || strncmp(
                *argv.offset(i as isize),
                b"--top\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            *(*argv.offset(i as isize))
                .offset(1 as libc::c_int as isize) = '@' as i32 as libc::c_char;
            *(*argv.offset(i as isize))
                .offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn read_child_out(mut fd: libc::c_int) {
    let mut rem: size_t = 0;
    let mut line_len: size_t = 0;
    let mut nl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut read_rv: ssize_t = 0;
    rem = (512 as libc::c_int as libc::c_ulong).wrapping_sub(buf_len);
    if rem < 1 as libc::c_int as libc::c_ulong {
        buf_len = 0 as libc::c_int as size_t;
        rem = 512 as libc::c_int as size_t;
    }
    read_rv = read(
        fd,
        buf.as_mut_ptr().offset(buf_len as isize) as *mut libc::c_void,
        rem,
    );
    if read_rv < 0 as libc::c_int as libc::c_long {
        perror(b"read\0" as *const u8 as *const libc::c_char);
        return;
    } else if read_rv == 0 as libc::c_int as libc::c_long {
        done = 1 as libc::c_int;
        return;
    }
    buf_len = (buf_len as libc::c_ulong).wrapping_add(read_rv as libc::c_ulong) as size_t
        as size_t;
    loop {
        nl = memchr(buf.as_mut_ptr() as *const libc::c_void, '\n' as i32, buf_len)
            as *mut libc::c_char;
        if nl.is_null() {
            break;
        }
        line_len = nl.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t;
        handle_line(buf.as_mut_ptr(), line_len as libc::c_int);
        memmove(
            buf.as_mut_ptr() as *mut libc::c_void,
            nl.offset(1 as libc::c_int as isize) as *const libc::c_void,
            buf_len
                .wrapping_sub(line_len.wrapping_add(1 as libc::c_int as libc::c_ulong)),
        );
        buf_len = (buf_len as libc::c_ulong)
            .wrapping_sub(line_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    };
}
unsafe extern "C" fn read_child_err(mut fd: libc::c_int) {
    let mut buf_0: [libc::c_char; 512] = [0; 512];
    let mut nl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_pos: size_t = 0;
    let mut read_rv: ssize_t = 0;
    read_rv = read(
        fd,
        buf_0.as_mut_ptr() as *mut libc::c_void,
        512 as libc::c_int as size_t,
    );
    if read_rv < 0 as libc::c_int as libc::c_long {
        perror(b"read\0" as *const u8 as *const libc::c_char);
        return;
    } else if read_rv == 0 as libc::c_int as libc::c_long {
        done = 1 as libc::c_int;
        return;
    }
    buf_pos = 0 as libc::c_int as size_t;
    while read_rv > 0 as libc::c_int as libc::c_long
        && {
            nl = memchr(
                buf_0.as_mut_ptr().offset(buf_pos as isize) as *const libc::c_void,
                '\n' as i32,
                read_rv as libc::c_ulong,
            ) as *mut libc::c_char;
            !nl.is_null()
        }
    {
        total_err_count = total_err_count
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        buf_pos = (buf_pos as libc::c_ulong)
            .wrapping_add(
                (nl.offset_from(buf_0.as_mut_ptr()) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            ) as size_t as size_t;
        read_rv
            -= nl.offset_from(buf_0.as_mut_ptr()) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
    }
}
unsafe extern "C" fn handle_line(
    mut line: *mut libc::c_char,
    mut line_len: libc::c_int,
) {
    let mut frame_num: libc::c_ulong = 0;
    let mut func: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut func_len: size_t = 0;
    let mut func_el: *mut func_entry_t = 0 as *mut func_entry_t;
    if line_len < 3 as libc::c_int {
        return;
    }
    if *line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        return;
    }
    frame_num = strtoull(line, &mut func, 10 as libc::c_int) as libc::c_ulong;
    if frame_num == 0 as libc::c_int as libc::c_ulong
        && *line.offset(0 as libc::c_int as isize) as libc::c_int != '0' as i32
    {
        return;
    }
    if *func as libc::c_int != ' ' as i32 {
        return;
    }
    func = func.offset(1 as libc::c_int as isize);
    func_len = (line_len as libc::c_long - func.offset_from(line) as libc::c_long)
        as size_t;
    if func_len < 1 as libc::c_int as libc::c_ulong
        || func_len >= 256 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let mut _hf_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = func as *const libc::c_uchar;
    _hf_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = func_len as libc::c_uint;
    while _hj_k >= 12 as libc::c_uint {
        _hj_i = _hj_i
            .wrapping_add(
                (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j = _hj_j
            .wrapping_add(
                (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hf_hashv = _hf_hashv
            .wrapping_add(
                (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as libc::c_int;
        _hj_key = _hj_key.offset(12 as libc::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
    }
    _hf_hashv = _hf_hashv.wrapping_add(func_len as libc::c_uint);
    let mut current_block_63: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_63 = 2809122975166620242;
        }
        10 => {
            current_block_63 = 2809122975166620242;
        }
        9 => {
            current_block_63 = 11269667230946691030;
        }
        8 => {
            current_block_63 = 4885156827351265621;
        }
        7 => {
            current_block_63 = 4339799501325312145;
        }
        6 => {
            current_block_63 = 9080116173873817767;
        }
        5 => {
            current_block_63 = 5770439152249486583;
        }
        4 => {
            current_block_63 = 10460820564149014484;
        }
        3 => {
            current_block_63 = 1561421893062450361;
        }
        2 => {
            current_block_63 = 3907272753513137745;
        }
        1 => {
            current_block_63 = 18104191032431943897;
        }
        _ => {
            current_block_63 = 10095721787123848864;
        }
    }
    match current_block_63 {
        2809122975166620242 => {
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_63 = 11269667230946691030;
        }
        _ => {}
    }
    match current_block_63 {
        11269667230946691030 => {
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_63 = 4885156827351265621;
        }
        _ => {}
    }
    match current_block_63 {
        4885156827351265621 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_63 = 4339799501325312145;
        }
        _ => {}
    }
    match current_block_63 {
        4339799501325312145 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_63 = 9080116173873817767;
        }
        _ => {}
    }
    match current_block_63 {
        9080116173873817767 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_63 = 5770439152249486583;
        }
        _ => {}
    }
    match current_block_63 {
        5770439152249486583 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_63 = 10460820564149014484;
        }
        _ => {}
    }
    match current_block_63 {
        10460820564149014484 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_63 = 1561421893062450361;
        }
        _ => {}
    }
    match current_block_63 {
        1561421893062450361 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_63 = 3907272753513137745;
        }
        _ => {}
    }
    match current_block_63 {
        3907272753513137745 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_63 = 18104191032431943897;
        }
        _ => {}
    }
    match current_block_63 {
        18104191032431943897 => {
            _hj_i = _hj_i
                .wrapping_add(
                    *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 13 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as libc::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 13 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 12 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as libc::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 5 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 3 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as libc::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 15 as libc::c_int;
    func_el = 0 as *mut func_entry_t;
    if !func_map.is_null() {
        let mut _hf_bkt: libc::c_uint = 0;
        _hf_bkt = _hf_hashv
            & ((*(*func_map).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*func_map).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                .is_null()
            {
                func_el = ((*((*(*func_map).hh.tbl).buckets).offset(_hf_bkt as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*func_map).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut func_entry_t;
            } else {
                func_el = 0 as *mut func_entry_t;
            }
            while !func_el.is_null() {
                if (*func_el).hh.hashv == _hf_hashv
                    && (*func_el).hh.keylen as libc::c_ulong == func_len
                {
                    if memcmp((*func_el).hh.key, func as *const libc::c_void, func_len)
                        == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*func_el).hh.hh_next).is_null() {
                    func_el = ((*func_el).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*func_map).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut func_entry_t;
                } else {
                    func_el = 0 as *mut func_entry_t;
                }
            }
        }
    }
    if func_el.is_null() {
        func_el = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<func_entry_t>() as libc::c_ulong,
        ) as *mut func_entry_t;
        snprintf(
            ((*func_el).func).as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"%.*s\0" as *const u8 as *const libc::c_char,
            func_len as libc::c_int,
            func,
        );
        let mut _uthash_hastr_keylen: libc::c_uint = strlen(
            ((*func_el).func).as_mut_ptr(),
        ) as libc::c_uint;
        let mut _ha_hashv: libc::c_uint = 0;
        let mut _hj_i_0: libc::c_uint = 0;
        let mut _hj_j_0: libc::c_uint = 0;
        let mut _hj_k_0: libc::c_uint = 0;
        let mut _hj_key_0: *const libc::c_uchar = &mut *((*func_el).func)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_uchar;
        _ha_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j_0 = 0x9e3779b9 as libc::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = _uthash_hastr_keylen;
        while _hj_k_0 >= 12 as libc::c_uint {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(11 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as libc::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as libc::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as libc::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(_uthash_hastr_keylen);
        let mut current_block_179: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_179 = 18371625021971093623;
            }
            10 => {
                current_block_179 = 18371625021971093623;
            }
            9 => {
                current_block_179 = 15165650403497571337;
            }
            8 => {
                current_block_179 = 7266251373903986620;
            }
            7 => {
                current_block_179 = 887687178682759170;
            }
            6 => {
                current_block_179 = 2535692301906169108;
            }
            5 => {
                current_block_179 = 15867619591269763334;
            }
            4 => {
                current_block_179 = 972173813676647121;
            }
            3 => {
                current_block_179 = 1319635546938210828;
            }
            2 => {
                current_block_179 = 10115417614016365113;
            }
            1 => {
                current_block_179 = 13634390024171496190;
            }
            _ => {
                current_block_179 = 13000670339742628194;
            }
        }
        match current_block_179 {
            18371625021971093623 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_179 = 15165650403497571337;
            }
            _ => {}
        }
        match current_block_179 {
            15165650403497571337 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_179 = 7266251373903986620;
            }
            _ => {}
        }
        match current_block_179 {
            7266251373903986620 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_179 = 887687178682759170;
            }
            _ => {}
        }
        match current_block_179 {
            887687178682759170 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_179 = 2535692301906169108;
            }
            _ => {}
        }
        match current_block_179 {
            2535692301906169108 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_179 = 15867619591269763334;
            }
            _ => {}
        }
        match current_block_179 {
            15867619591269763334 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_179 = 972173813676647121;
            }
            _ => {}
        }
        match current_block_179 {
            972173813676647121 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_179 = 1319635546938210828;
            }
            _ => {}
        }
        match current_block_179 {
            1319635546938210828 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_179 = 10115417614016365113;
            }
            _ => {}
        }
        match current_block_179 {
            10115417614016365113 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_179 = 13634390024171496190;
            }
            _ => {}
        }
        match current_block_179 {
            13634390024171496190 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 13 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 13 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 12 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 5 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 3 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 15 as libc::c_int;
        (*func_el).hh.hashv = _ha_hashv;
        (*func_el)
            .hh
            .key = &mut *((*func_el).func).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_char as *mut libc::c_void;
        (*func_el).hh.keylen = _uthash_hastr_keylen;
        if func_map.is_null() {
            (*func_el).hh.next = 0 as *mut libc::c_void;
            (*func_el).hh.prev = 0 as *mut libc::c_void;
            (*func_el)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*func_el).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*func_el).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*func_el).hh.tbl).tail = &mut (*func_el).hh;
                (*(*func_el).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*func_el).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*func_el).hh.tbl)
                    .hho = (&mut (*func_el).hh as *mut UT_hash_handle
                    as *mut libc::c_char)
                    .offset_from(func_el as *mut libc::c_char) as libc::c_long;
                (*(*func_el).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*func_el).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*func_el).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*func_el).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            func_map = func_el;
        } else {
            (*func_el).hh.tbl = (*func_map).hh.tbl;
            (*func_el).hh.next = 0 as *mut libc::c_void;
            (*func_el)
                .hh
                .prev = ((*(*func_map).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*func_map).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*func_map).hh.tbl).tail).next = func_el as *mut libc::c_void;
            (*(*func_map).hh.tbl).tail = &mut (*func_el).hh;
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*func_map).hh.tbl)
            .num_items = ((*(*func_map).hh.tbl).num_items).wrapping_add(1);
        (*(*func_map).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*func_map).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*func_map).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*func_el).hh.hh_next = (*_ha_head).hh_head;
        (*func_el).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*func_el).hh;
        }
        (*_ha_head).hh_head = &mut (*func_el).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*func_el).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (2 as libc::c_ulong)
                    .wrapping_mul((*(*func_el).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (2 as libc::c_ulong)
                        .wrapping_mul((*(*func_el).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
                (*(*func_el).hh.tbl)
                    .ideal_chain_maxlen = ((*(*func_el).hh.tbl).num_items
                    >> ((*(*func_el).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*func_el).hh.tbl).num_items
                            & ((*(*func_el).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*func_el).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*func_el).hh.tbl).num_buckets {
                    _he_thh = (*((*(*func_el).hh.tbl).buckets)
                        .offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*func_el).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*func_el).hh.tbl).ideal_chain_maxlen
                        {
                            (*(*func_el).hh.tbl)
                                .nonideal_items = ((*(*func_el).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*func_el).hh.tbl).nonideal_items;
                            (*_he_newbkt)
                                .expand_mult = ((*_he_newbkt).count)
                                .wrapping_div((*(*func_el).hh.tbl).ideal_chain_maxlen);
                        }
                        (*_he_thh).hh_prev = 0 as *mut UT_hash_handle;
                        (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                        if !((*_he_newbkt).hh_head).is_null() {
                            (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                        }
                        (*_he_newbkt).hh_head = _he_thh;
                        _he_thh = _he_hh_nxt;
                    }
                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    _he_bkt_i;
                }
                free((*(*func_el).hh.tbl).buckets as *mut libc::c_void);
                (*(*func_el).hh.tbl)
                    .num_buckets = ((*(*func_el).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*func_el).hh.tbl)
                    .log2_num_buckets = ((*(*func_el).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*func_el).hh.tbl).log2_num_buckets;
                (*(*func_el).hh.tbl).buckets = _he_new_buckets;
                (*(*func_el).hh.tbl)
                    .ineff_expands = if (*(*func_el).hh.tbl).nonideal_items
                    > (*(*func_el).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*func_el).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*func_el).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*func_el).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        func_list_len = (func_list_len as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
        if func_list_len > func_list_size {
            func_list = realloc(
                func_list as *mut libc::c_void,
                (::std::mem::size_of::<*mut func_entry_t>() as libc::c_ulong)
                    .wrapping_mul(
                        func_list_size.wrapping_add(1024 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut *mut func_entry_t;
            func_list_size = (func_list_size as libc::c_ulong)
                .wrapping_add(1024 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        let ref mut fresh6 = *func_list
            .offset(
                func_list_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *fresh6 = func_el;
    }
    if frame_num == 0 as libc::c_int as libc::c_ulong {
        samp_count = samp_count.wrapping_add(1 as libc::c_int as libc::c_ulong);
        (*func_el)
            .count_excl = ((*func_el).count_excl)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    (*func_el)
        .count_incl = ((*func_el).count_incl)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn handle_event(mut event: *mut tb_event) {
    if (*event).type_0 as libc::c_int != 1 as libc::c_int {
        return
    } else if (*event).ch == 'q' as i32 as libc::c_uint {
        done = 1 as libc::c_int;
    } else if (*event).ch == 'p' as i32 as libc::c_uint {
        is_paused = 1 as libc::c_int - is_paused;
    }
}
unsafe extern "C" fn func_list_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut fa: *mut func_entry_t = 0 as *mut func_entry_t;
    let mut fb: *mut func_entry_t = 0 as *mut func_entry_t;
    fa = *(a as *mut *mut func_entry_t);
    fb = *(b as *mut *mut func_entry_t);
    if (*fb).count_excl == (*fa).count_excl {
        if (*fb).count_incl == (*fa).count_incl {
            if (*fb).total_count_excl == (*fa).total_count_excl {
                if (*fb).total_count_incl == (*fa).total_count_incl {
                    return 0 as libc::c_int;
                }
                return if (*fb).total_count_incl > (*fa).total_count_incl {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
            }
            return if (*fb).total_count_excl > (*fa).total_count_excl {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        return if (*fb).count_incl > (*fa).count_incl {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return if (*fb).count_excl > (*fa).count_excl {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn display() {
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut el: *mut func_entry_t = 0 as *mut func_entry_t;
    let mut i: size_t = 0;
    if func_list_len > 0 as libc::c_int as libc::c_ulong {
        qsort(
            func_list as *mut libc::c_void,
            func_list_len,
            ::std::mem::size_of::<*mut func_entry_t>() as libc::c_ulong,
            Some(
                func_list_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        i = 0 as libc::c_int as size_t;
        while i < func_list_len {
            let ref mut fresh7 = (**func_list.offset(i as isize)).total_count_excl;
            *fresh7 = (*fresh7)
                .wrapping_add((**func_list.offset(i as isize)).count_excl);
            let ref mut fresh8 = (**func_list.offset(i as isize)).total_count_incl;
            *fresh8 = (*fresh8)
                .wrapping_add((**func_list.offset(i as isize)).count_incl);
            (**func_list.offset(i as isize))
                .percent_excl = if samp_count < 1 as libc::c_int as libc::c_ulong {
                0.0f32
            } else {
                100.0f32 * (**func_list.offset(i as isize)).count_excl as libc::c_float
                    / samp_count as libc::c_float
            };
            i = i.wrapping_add(1);
            i;
        }
    }
    total_samp_count = total_samp_count.wrapping_add(samp_count);
    tb_clear();
    w = tb_width();
    h = tb_height();
    y = 0 as libc::c_int;
    let fresh9 = y;
    y = y + 1;
    tb_printf(
        0 as libc::c_int,
        fresh9,
        0x100 as libc::c_int as uintattr_t,
        0 as libc::c_int as uintattr_t,
        b"%s\0" as *const u8 as *const libc::c_char,
        phpspy_args.as_mut_ptr(),
    );
    let fresh10 = y;
    y = y + 1;
    tb_printf(
        0 as libc::c_int,
        fresh10,
        0 as libc::c_int as uintattr_t,
        0 as libc::c_int as uintattr_t,
        b"samp_count=%llu  err_count=%llu  func_count=%llu\0" as *const u8
            as *const libc::c_char,
        total_samp_count,
        total_err_count,
        func_list_len,
    );
    y += 1;
    y;
    tb_printf(
        0 as libc::c_int,
        y,
        (0x100 as libc::c_int | 0x400 as libc::c_int) as uintattr_t,
        0 as libc::c_int as uintattr_t,
        b"%-10s %-10s %-10s %-10s %-7s \0" as *const u8 as *const libc::c_char,
        b"tincl\0" as *const u8 as *const libc::c_char,
        b"texcl\0" as *const u8 as *const libc::c_char,
        b"incl\0" as *const u8 as *const libc::c_char,
        b"excl\0" as *const u8 as *const libc::c_char,
        b"excl%\0" as *const u8 as *const libc::c_char,
    );
    let fresh11 = y;
    y = y + 1;
    tb_printf(
        52 as libc::c_int,
        fresh11,
        (0x100 as libc::c_int | 0x400 as libc::c_int) as uintattr_t,
        0 as libc::c_int as uintattr_t,
        b"%-*s\0" as *const u8 as *const libc::c_char,
        w - 52 as libc::c_int,
        b"func\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while y < h && i < func_list_len {
        let fresh12 = i;
        i = i.wrapping_add(1);
        el = *func_list.offset(fresh12 as isize);
        let fresh13 = y;
        y = y + 1;
        tb_printf(
            0 as libc::c_int,
            fresh13,
            0 as libc::c_int as uintattr_t,
            0 as libc::c_int as uintattr_t,
            b"%-9llu  %-9llu  %-9llu  %-9llu  %-6.2f  %s\0" as *const u8
                as *const libc::c_char,
            (*el).total_count_incl,
            (*el).total_count_excl,
            (*el).count_incl,
            (*el).count_excl,
            (*el).percent_excl as libc::c_double,
            ((*el).func).as_mut_ptr(),
        );
    }
    i = 0 as libc::c_int as size_t;
    while i < func_list_len {
        (**func_list.offset(i as isize)).count_excl = 0 as libc::c_int as libc::c_ulong;
        (**func_list.offset(i as isize)).count_incl = 0 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    samp_count = 0 as libc::c_int as libc::c_ulong;
    if is_paused == 0 {
        tb_present();
    }
}

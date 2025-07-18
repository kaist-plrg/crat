use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: __off_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vdprintf(
        __fd: libc::c_int,
        __fmt: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut config: shairport_cfg;
    fn die(format: *mut libc::c_char, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
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
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
static mut lock_fd: libc::c_int = -(1 as libc::c_int);
static mut daemon_pipe: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
pub unsafe extern "C" fn daemon_init() {
    let mut ret: libc::c_int = 0;
    ret = pipe(daemon_pipe.as_mut_ptr());
    if ret < 0 as libc::c_int {
        die(
            b"couldn't create a pipe?!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    let mut pid: pid_t = fork();
    if pid < 0 as libc::c_int {
        die(
            b"failed to fork!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if pid != 0 {
        close(daemon_pipe[1 as libc::c_int as usize]);
        let mut buf: [libc::c_char; 64] = [0; 64];
        ret = read(
            daemon_pipe[0 as libc::c_int as usize],
            buf.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        ) as libc::c_int;
        if ret < 0 as libc::c_int {
            fprintf(
                stderr,
                b"Spawning the daemon failed.\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
            write(
                2 as libc::c_int,
                buf.as_mut_ptr() as *const libc::c_void,
                ret as size_t,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        } else {
            if (config.pidfile).is_null() {
                printf(b"%d\n\0" as *const u8 as *const libc::c_char, pid);
            }
            exit(0 as libc::c_int);
        }
    } else {
        close(daemon_pipe[0 as libc::c_int as usize]);
        if !(config.pidfile).is_null() {
            lock_fd = open(
                config.pidfile,
                0o2 as libc::c_int | 0o100 as libc::c_int,
                0o400 as libc::c_int | 0o200 as libc::c_int,
            );
            if lock_fd < 0 as libc::c_int {
                die(
                    b"Could not open pidfile\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            ret = lockf(lock_fd, 2 as libc::c_int, 0 as libc::c_int as __off_t);
            if ret < 0 as libc::c_int {
                die(
                    b"Could not lock pidfile. Is an other instance running ?\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            dprintf(lock_fd, b"%d\n\0" as *const u8 as *const libc::c_char, getpid());
        }
    };
}
pub unsafe extern "C" fn daemon_ready() {
    let mut ok: libc::c_char = 0 as libc::c_int as libc::c_char;
    write(
        daemon_pipe[1 as libc::c_int as usize],
        &mut ok as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    close(daemon_pipe[1 as libc::c_int as usize]);
    daemon_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
}
pub unsafe extern "C" fn daemon_fail(
    mut format: *const libc::c_char,
    mut arg: ::std::ffi::VaList,
) {
    if daemon_pipe[1 as libc::c_int as usize] > 0 as libc::c_int {
        vdprintf(daemon_pipe[1 as libc::c_int as usize], format, arg.as_va_list());
    }
}
pub unsafe extern "C" fn daemon_exit() {
    if lock_fd > 0 as libc::c_int {
        lockf(lock_fd, 0 as libc::c_int, 0 as libc::c_int as __off_t);
        close(lock_fd);
        unlink(config.pidfile);
        lock_fd = -(1 as libc::c_int);
    }
}

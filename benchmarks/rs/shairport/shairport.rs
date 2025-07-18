use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    static mut config: shairport_cfg;
    fn die(format: *mut libc::c_char, _: ...);
    static mut debuglev: libc::c_int;
    fn mdns_ls_backends();
    fn mdns_unregister();
    static mut mdns_pid: libc::c_int;
    fn audio_ls_outputs();
    fn audio_get_output(name: *mut libc::c_char) -> *mut audio_output;
    fn daemon_init();
    fn daemon_ready();
    fn daemon_exit();
    fn rtsp_listen_loop();
    fn rtsp_shutdown_stream();
    fn metadata_open();
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut version: *const libc::c_char = b"-dirty\0" as *const u8
    as *const libc::c_char;
static mut shutting_down: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn shairport_shutdown(mut retval: libc::c_int) {
    if shutting_down != 0 {
        return;
    }
    shutting_down = 1 as libc::c_int;
    printf(b"Shutting down...\n\0" as *const u8 as *const libc::c_char);
    mdns_unregister();
    rtsp_shutdown_stream();
    if !(config.output).is_null() {
        ((*config.output).deinit).unwrap()();
    }
    daemon_exit();
    exit(retval);
}
unsafe extern "C" fn sig_ignore(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {}
unsafe extern "C" fn sig_shutdown(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {
    shairport_shutdown(0 as libc::c_int);
}
unsafe extern "C" fn sig_child(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {
    let mut pid: pid_t = 0;
    loop {
        pid = waitpid(-(1 as libc::c_int), 0 as *mut libc::c_int, 1 as libc::c_int);
        if !(pid > 0 as libc::c_int) {
            break;
        }
        if pid == mdns_pid && shutting_down == 0 {
            die(
                b"MDNS child process died unexpectedly!\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn sig_logrotate(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {
    log_setup();
}
pub unsafe extern "C" fn usage(mut progname: *mut libc::c_char) {
    printf(b"Usage: %s [options...]\n\0" as *const u8 as *const libc::c_char, progname);
    printf(
        b"  or:  %s [options...] -- [audio output-specific options]\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Mandatory arguments to long options are mandatory for short options too.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    -h, --help          show this help\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -p, --port=PORT     set RTSP listening port\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -a, --name=NAME     set advertised name\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -k, --password=PW   require password to stream audio\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -b FILL             set how full the buffer must be before audio output\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        starts. This value is in frames; default %d\n\0"
            as *const u8 as *const libc::c_char,
        config.buffer_start_fill,
    );
    printf(
        b"    -d, --daemon        fork (daemonise). The PID of the child process is\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        written to stdout, unless a pidfile is used.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -P, --pidfile=FILE  write daemon's pid to FILE on startup.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        Has no effect if -d is not specified\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -l, --log=FILE      redirect shairport's standard output to FILE\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        If --error is not specified, it also redirects\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        error output to FILE\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -e, --error=FILE    redirect shairport's standard error output to FILE\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -B, --on-start=COMMAND  run a shell command when playback begins\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -E, --on-stop=COMMAND   run a shell command when playback ends\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -w, --wait-cmd          block while the shell command(s) run\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -M, --meta-dir=DIR      set a directory to write metadata and album cover art to\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -o, --output=BACKEND    select audio output method\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -m, --mdns=BACKEND      force the use of BACKEND to advertise the service\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                            if no mdns provider is specified,\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                            shairport tries them all until one works.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    mdns_ls_backends();
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    audio_ls_outputs();
}
pub unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    setenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    static mut long_options: [option; 15] = [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"daemon\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pidfile\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"log\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"error\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"port\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"name\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"password\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"output\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"on-start\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"on-stop\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"wait-cmd\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"meta-dir\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mdns\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut opt: libc::c_int = 0;
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"+hdvP:l:e:p:a:k:o:b:B:E:M:wm:\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            104 => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(0 as libc::c_int);
            }
            100 => {
                config.daemonise = 1 as libc::c_int;
            }
            118 => {
                debuglev += 1;
                debuglev;
            }
            112 => {
                config.port = atoi(optarg);
            }
            97 => {
                config.apname = optarg;
            }
            111 => {
                config.output_name = optarg;
            }
            107 => {
                config.password = optarg;
            }
            98 => {
                config.buffer_start_fill = atoi(optarg);
            }
            66 => {
                config.cmd_start = optarg;
            }
            69 => {
                config.cmd_stop = optarg;
            }
            119 => {
                config.cmd_blocking = 1 as libc::c_int;
            }
            77 => {
                config.meta_dir = optarg;
            }
            80 => {
                config.pidfile = optarg;
            }
            108 => {
                config.logfile = optarg;
            }
            101 => {
                config.errfile = optarg;
            }
            109 => {
                config.mdns_name = optarg;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
    }
    return optind;
}
pub unsafe extern "C" fn signal_setup() {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    sigfillset(&mut set);
    sigdelset(&mut set, 2 as libc::c_int);
    sigdelset(&mut set, 15 as libc::c_int);
    sigdelset(&mut set, 1 as libc::c_int);
    sigdelset(&mut set, 19 as libc::c_int);
    sigdelset(&mut set, 17 as libc::c_int);
    pthread_sigmask(0 as libc::c_int, &mut set, 0 as *mut __sigset_t);
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sa.sa_flags = 4 as libc::c_int;
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_ignore
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(10 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sa.sa_flags = 4 as libc::c_int | 0x10000000 as libc::c_int;
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_shutdown
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_logrotate
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_child
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(17 as libc::c_int, &mut sa, 0 as *mut sigaction);
}
pub unsafe extern "C" fn shairport_startup_complete() {
    if config.daemonise != 0 {
        daemon_ready();
    }
}
unsafe extern "C" fn log_setup() {
    if !(config.logfile).is_null() {
        let mut log_fd: libc::c_int = open(
            config.logfile,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if log_fd < 0 as libc::c_int {
            die(
                b"Could not open logfile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        dup2(log_fd, 1 as libc::c_int);
        setvbuf(
            stdout,
            0 as *mut libc::c_char,
            1 as libc::c_int,
            8192 as libc::c_int as size_t,
        );
        if (config.errfile).is_null() {
            dup2(log_fd, 2 as libc::c_int);
            setvbuf(
                stderr,
                0 as *mut libc::c_char,
                1 as libc::c_int,
                8192 as libc::c_int as size_t,
            );
        }
    }
    if !(config.errfile).is_null() {
        let mut err_fd: libc::c_int = open(
            config.errfile,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if err_fd < 0 as libc::c_int {
            die(
                b"Could not open logfile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        dup2(err_fd, 2 as libc::c_int);
        setvbuf(
            stderr,
            0 as *mut libc::c_char,
            1 as libc::c_int,
            8192 as libc::c_int as size_t,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"Starting Shairport %s\n\0" as *const u8 as *const libc::c_char, version);
    signal_setup();
    memset(
        &mut config as *mut shairport_cfg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<shairport_cfg>() as libc::c_ulong,
    );
    config.buffer_start_fill = 220 as libc::c_int;
    config.port = 5002 as libc::c_int;
    let mut hostname: [libc::c_char; 100] = [0; 100];
    gethostname(hostname.as_mut_ptr(), 100 as libc::c_int as size_t);
    config
        .apname = malloc((20 as libc::c_int + 100 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    snprintf(
        config.apname,
        (20 as libc::c_int + 100 as libc::c_int) as libc::c_ulong,
        b"Shairport on %s\0" as *const u8 as *const libc::c_char,
        hostname.as_mut_ptr(),
    );
    let mut audio_arg: libc::c_int = parse_options(argc, argv);
    if strlen(config.apname) > 50 as libc::c_int as libc::c_ulong {
        die(
            b"Supplied name too long (max 50 characters)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if config.daemonise != 0 {
        daemon_init();
    }
    log_setup();
    config.output = audio_get_output(config.output_name);
    if (config.output).is_null() {
        audio_ls_outputs();
        die(
            b"Invalid audio output specified!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    ((*config.output).init).unwrap()(argc - audio_arg, argv.offset(audio_arg as isize));
    let mut ap_md5: [uint8_t; 16] = [0; 16];
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, config.apname as *const libc::c_void, strlen(config.apname));
    MD5_Final(ap_md5.as_mut_ptr(), &mut ctx);
    memcpy(
        (config.hw_addr).as_mut_ptr() as *mut libc::c_void,
        ap_md5.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong,
    );
    if !(config.meta_dir).is_null() {
        metadata_open();
    }
    rtsp_listen_loop();
    shairport_shutdown(1 as libc::c_int);
    return 1 as libc::c_int;
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

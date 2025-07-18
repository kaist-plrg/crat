use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn __errno_location() -> *mut libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn geteuid() -> __uid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn is_anonymous_enabled(conf: *mut config_s) -> libc::c_short;
    fn anonymous_insert(conf: *mut config_s, s: *mut libc::c_char) -> libc::c_int;
    fn reload_config_file(
        config_fname: *const libc::c_char,
        conf: *mut config_s,
    ) -> libc::c_int;
    fn free_config(conf: *mut config_s);
    fn config_init() -> libc::c_int;
    fn set_signal_handler(
        signo: libc::c_int,
        func: Option::<signal_func>,
    ) -> Option::<signal_func>;
    fn makedaemon();
    fn filter_init();
    fn filter_destroy();
    fn child_listening_sockets(listen_addrs: *mut sblist, port: uint16_t) -> libc::c_int;
    fn child_close_sock();
    fn child_main_loop();
    fn child_kill_children(sig: libc::c_int);
    fn child_free_children();
    fn loop_records_init();
    fn loop_records_destroy();
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn setup_logging() -> libc::c_int;
    fn shutdown_logging();
    fn init_stats();
    fn pidfile_create(path: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_list_t = *mut sblist;
pub type signal_func = unsafe extern "C" fn(libc::c_int) -> ();
pub static mut config: *mut config_s = 0 as *const config_s as *mut config_s;
static mut configs: [config_s; 2] = [config_s {
    basicauth_list: 0 as *const sblist as *mut sblist,
    logf_name: 0 as *const libc::c_char as *mut libc::c_char,
    syslog: 0,
    port: 0,
    stathost: 0 as *const libc::c_char as *mut libc::c_char,
    quit: 0,
    maxclients: 0,
    user: 0 as *const libc::c_char as *mut libc::c_char,
    group: 0 as *const libc::c_char as *mut libc::c_char,
    listen_addrs: 0 as *const sblist as *mut sblist,
    filter: 0 as *const libc::c_char as *mut libc::c_char,
    filter_opts: 0,
    add_xtinyproxy: 0,
    reversepath_list: 0 as *const reversepath as *mut reversepath,
    reverseonly: 0,
    reversemagic: 0,
    reversebaseurl: 0 as *const libc::c_char as *mut libc::c_char,
    upstream_list: 0 as *const upstream as *mut upstream,
    pidpath: 0 as *const libc::c_char as *mut libc::c_char,
    idletimeout: 0,
    bind_addrs: 0 as *const sblist as *mut sblist,
    bindsame: 0,
    via_proxy_name: 0 as *const libc::c_char as *mut libc::c_char,
    disable_viaheader: 0,
    errorpages: 0 as *const htab as *mut htab,
    errorpage_undef: 0 as *const libc::c_char as *mut libc::c_char,
    statpage: 0 as *const libc::c_char as *mut libc::c_char,
    access_list: 0 as *const sblist as *mut sblist,
    connect_ports: 0 as *const sblist as *mut sblist,
    anonymous_map: 0 as *const htab as *mut htab,
    add_headers: 0 as *const sblist as *mut sblist,
}; 2];
static mut config_file: *const libc::c_char = 0 as *const libc::c_char;
pub static mut received_sighup: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn get_next_config() -> *mut config_s {
    if config
        == &mut *configs.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut config_s
    {
        return &mut *configs.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut config_s;
    }
    return &mut *configs.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut config_s;
}
unsafe extern "C" fn takesig(mut sig: libc::c_int) {
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    match sig {
        10 | 1 => {
            received_sighup = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
        }
        2 | 15 => {
            (*config).quit = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
        }
        17 => {
            loop {
                pid = waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int);
                if !(pid > 0 as libc::c_int) {
                    break;
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn display_version() {
    printf(
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        b"tinyproxy\0" as *const u8 as *const libc::c_char,
        b"1.11.2\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn display_usage() {
    let mut features: libc::c_int = 0 as libc::c_int;
    printf(
        b"Usage: %s [options]\n\0" as *const u8 as *const libc::c_char,
        b"tinyproxy\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"\nOptions are:\n  -d        Do not daemonize (run in foreground).\n  -c FILE   Use an alternate configuration file.\n  -h        Display this usage information.\n  -v        Display version information.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\nFeatures compiled in:\n\0" as *const u8 as *const libc::c_char);
    printf(b"    XTinyproxy header\n\0" as *const u8 as *const libc::c_char);
    features += 1;
    features;
    printf(b"    Filtering\n\0" as *const u8 as *const libc::c_char);
    features += 1;
    features;
    printf(b"    Transparent proxy support\n\0" as *const u8 as *const libc::c_char);
    features += 1;
    features;
    printf(b"    Reverse proxy support\n\0" as *const u8 as *const libc::c_char);
    features += 1;
    features;
    printf(b"    Upstream proxy support\n\0" as *const u8 as *const libc::c_char);
    features += 1;
    features;
    if 0 as libc::c_int == features {
        printf(b"    None\n\0" as *const u8 as *const libc::c_char);
    }
    printf(
        b"\nFor support and bug reporting instructions, please visit\n<https://tinyproxy.github.io/>.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn get_id(mut str: *mut libc::c_char) -> libc::c_int {
    let mut tstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return -(1 as libc::c_int);
    }
    tstr = str;
    while *tstr as libc::c_int != 0 as libc::c_int {
        if *(*__ctype_b_loc()).offset(*tstr as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            return -(1 as libc::c_int);
        }
        tstr = tstr.offset(1);
        tstr;
    }
    return atoi(str);
}
unsafe extern "C" fn change_user(mut program: *const libc::c_char) {
    if !((*config).group).is_null()
        && strlen((*config).group) > 0 as libc::c_int as libc::c_ulong
    {
        let mut gid: libc::c_int = get_id((*config).group);
        if gid < 0 as libc::c_int {
            let mut thisgroup: *mut group = getgrnam((*config).group);
            if thisgroup.is_null() {
                fprintf(
                    stderr,
                    b"%s: Unable to find group \"%s\".\n\0" as *const u8
                        as *const libc::c_char,
                    program,
                    (*config).group,
                );
                exit(67 as libc::c_int);
            }
            gid = (*thisgroup).gr_gid as libc::c_int;
        }
        if setgid(gid as __gid_t) < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Unable to change to group \"%s\".\n\0" as *const u8
                    as *const libc::c_char,
                program,
                (*config).group,
            );
            exit(77 as libc::c_int);
        }
        if setgroups(0 as libc::c_int as size_t, 0 as *const __gid_t) < 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"%s: Unable to drop supplementary groups.\n\0" as *const u8
                    as *const libc::c_char,
                program,
            );
            exit(77 as libc::c_int);
        }
        log_message(
            6 as libc::c_int,
            b"Now running as group \"%s\".\0" as *const u8 as *const libc::c_char,
            (*config).group,
        );
    }
    if !((*config).user).is_null()
        && strlen((*config).user) > 0 as libc::c_int as libc::c_ulong
    {
        let mut uid: libc::c_int = get_id((*config).user);
        if uid < 0 as libc::c_int {
            let mut thisuser: *mut passwd = getpwnam((*config).user);
            if thisuser.is_null() {
                fprintf(
                    stderr,
                    b"%s: Unable to find user \"%s\".\n\0" as *const u8
                        as *const libc::c_char,
                    program,
                    (*config).user,
                );
                exit(67 as libc::c_int);
            }
            uid = (*thisuser).pw_uid as libc::c_int;
        }
        if setuid(uid as __uid_t) < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Unable to change to user \"%s\".\n\0" as *const u8
                    as *const libc::c_char,
                program,
                (*config).user,
            );
            exit(77 as libc::c_int);
        }
        log_message(
            6 as libc::c_int,
            b"Now running as user \"%s\".\0" as *const u8 as *const libc::c_char,
            (*config).user,
        );
    }
}
pub unsafe extern "C" fn reload_config(mut reload_logging: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ret2: libc::c_int = 0;
    let mut c_next: *mut config_s = get_next_config();
    log_message(
        5 as libc::c_int,
        b"Reloading config file (%s)\0" as *const u8 as *const libc::c_char,
        config_file,
    );
    if reload_logging != 0 {
        shutdown_logging();
    }
    ret = reload_config_file(config_file, c_next);
    if ret == 0 as libc::c_int {
        if !config.is_null() {
            free_config(config);
        }
        config = c_next;
    }
    ret2 = if reload_logging != 0 { setup_logging() } else { 0 as libc::c_int };
    if ret != 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"Reloading config file failed!\0" as *const u8 as *const libc::c_char,
        );
    } else {
        log_message(
            5 as libc::c_int,
            b"Reloading config file finished\0" as *const u8 as *const libc::c_char,
        );
    }
    return if ret != 0 { ret } else { ret2 };
}
unsafe extern "C" fn setup_sig(
    mut sig: libc::c_int,
    mut sigh: Option::<signal_func>,
    mut signame: *const libc::c_char,
    mut argv0: *const libc::c_char,
) {
    if set_signal_handler(sig, sigh)
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        fprintf(
            stderr,
            b"%s: Could not set the \"%s\" signal.\n\0" as *const u8
                as *const libc::c_char,
            argv0,
            signame,
        );
        exit(71 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut daemonized: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    srand(time(0 as *mut time_t) as libc::c_uint);
    umask(0o177 as libc::c_int as __mode_t);
    log_message(
        5 as libc::c_int,
        b"Initializing tinyproxy ...\0" as *const u8 as *const libc::c_char,
    );
    if config_init() != 0 {
        fprintf(
            stderr,
            b"ERROR: config_init() failed\n\0" as *const u8 as *const libc::c_char,
        );
        exit(70 as libc::c_int);
    }
    config_file = b"/usr/local/etc/tinyproxy/tinyproxy.conf\0" as *const u8
        as *const libc::c_char;
    loop {
        opt = getopt(argc, argv, b"c:vdh\0" as *const u8 as *const libc::c_char);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            118 => {
                display_version();
                exit(0 as libc::c_int);
            }
            100 => {
                daemonized = 0 as libc::c_int;
            }
            99 => {
                config_file = optarg;
            }
            104 => {
                display_usage();
                exit(0 as libc::c_int);
            }
            _ => {
                display_usage();
                exit(64 as libc::c_int);
            }
        }
    }
    if reload_config(0 as libc::c_int) != 0 {
        exit(70 as libc::c_int);
    }
    init_stats();
    if is_anonymous_enabled(config) != 0 {
        anonymous_insert(
            config,
            strdup(b"Content-Length\0" as *const u8 as *const libc::c_char),
        );
        anonymous_insert(
            config,
            strdup(b"Content-Type\0" as *const u8 as *const libc::c_char),
        );
    }
    if daemonized == (0 as libc::c_int == 0) as libc::c_int {
        if (*config).syslog == 0 && ((*config).logf_name).is_null() {
            fprintf(
                stderr,
                b"WARNING: logging deactivated (can't log to stdout when daemonized)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        makedaemon();
    }
    setup_sig(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
        b"SIGPIPE\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    if !((*config).filter).is_null() {
        filter_init();
    }
    if child_listening_sockets((*config).listen_addrs, (*config).port as uint16_t)
        < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"%s: Could not create listening sockets.\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(71 as libc::c_int);
    }
    if !((*config).pidpath).is_null() {
        if pidfile_create((*config).pidpath) < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Could not create PID file.\n\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            exit(71 as libc::c_int);
        }
    }
    if geteuid() == 0 as libc::c_int as libc::c_uint {
        change_user(*argv.offset(0 as libc::c_int as isize));
    } else {
        log_message(
            6 as libc::c_int,
            b"Not running as root, so not changing UID/GID.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if setup_logging() != 0 {
        exit(70 as libc::c_int);
    }
    log_message(
        6 as libc::c_int,
        b"Setting the various signals.\0" as *const u8 as *const libc::c_char,
    );
    setup_sig(
        17 as libc::c_int,
        Some(takesig as unsafe extern "C" fn(libc::c_int) -> ()),
        b"SIGCHLD\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    setup_sig(
        15 as libc::c_int,
        Some(takesig as unsafe extern "C" fn(libc::c_int) -> ()),
        b"SIGTERM\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    setup_sig(
        2 as libc::c_int,
        Some(takesig as unsafe extern "C" fn(libc::c_int) -> ()),
        b"SIGINT\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    if daemonized != 0 {
        setup_sig(
            1 as libc::c_int,
            Some(takesig as unsafe extern "C" fn(libc::c_int) -> ()),
            b"SIGHUP\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
    }
    setup_sig(
        10 as libc::c_int,
        Some(takesig as unsafe extern "C" fn(libc::c_int) -> ()),
        b"SIGUSR1\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    loop_records_init();
    log_message(
        6 as libc::c_int,
        b"Starting main loop. Accepting connections.\0" as *const u8
            as *const libc::c_char,
    );
    child_main_loop();
    log_message(
        5 as libc::c_int,
        b"Shutting down.\0" as *const u8 as *const libc::c_char,
    );
    child_kill_children(15 as libc::c_int);
    child_close_sock();
    child_free_children();
    loop_records_destroy();
    if !((*config).pidpath).is_null() && unlink((*config).pidpath) < 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"Could not remove PID file \"%s\": %s.\0" as *const u8
                as *const libc::c_char,
            (*config).pidpath,
            strerror(*__errno_location()),
        );
    }
    if !((*config).filter).is_null() {
        filter_destroy();
    }
    free_config(config);
    shutdown_logging();
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

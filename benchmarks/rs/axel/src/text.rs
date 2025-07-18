use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn conf_init(conf: *mut conf_t) -> libc::c_int;
    fn conf_free(conf: *mut conf_t);
    fn search_makelist(results: *mut search_t, url: *mut libc::c_char) -> libc::c_int;
    fn search_getspeeds(results: *mut search_t, count: libc::c_int) -> libc::c_int;
    fn search_sortlist(results: *mut search_t, count: libc::c_int);
    fn axel_new(
        conf: *mut conf_t,
        count: libc::c_int,
        urls: *const search_t,
    ) -> *mut axel_t;
    fn axel_open(axel: *mut axel_t) -> libc::c_int;
    fn axel_start(axel: *mut axel_t);
    fn axel_do(axel: *mut axel_t);
    fn axel_close(axel: *mut axel_t);
    fn axel_gettime() -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type uint16_t = __uint16_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_t {
    pub next: *mut libc::c_void,
    pub text: [libc::c_char; 1024],
}
pub type url_t = message_t;
pub type if_t = message_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abuf_t {
    pub p: *mut libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_t {
    pub default_filename: [libc::c_char; 1024],
    pub http_proxy: [libc::c_char; 1024],
    pub no_proxy: [libc::c_char; 1024],
    pub num_connections: uint16_t,
    pub strip_cgi_parameters: libc::c_int,
    pub save_state_interval: libc::c_int,
    pub connection_timeout: libc::c_int,
    pub reconnect_delay: libc::c_int,
    pub max_redirect: libc::c_int,
    pub buffer_size: libc::c_int,
    pub max_speed: libc::c_ulonglong,
    pub verbose: libc::c_int,
    pub alternate_output: libc::c_int,
    pub insecure: libc::c_int,
    pub no_clobber: libc::c_int,
    pub percentage: libc::c_int,
    pub interfaces: *mut if_t,
    pub ai_family: sa_family_t,
    pub search_timeout: libc::c_int,
    pub search_threads: libc::c_int,
    pub search_amount: libc::c_int,
    pub search_top: libc::c_int,
    pub io_timeout: libc::c_uint,
    pub add_header_count: libc::c_int,
    pub add_header: [[libc::c_char; 1024]; 10],
}
pub type C2RustUnnamed = libc::c_uint;
pub const HDR_count_init: C2RustUnnamed = 1;
pub const HDR_USER_AGENT: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_t {
    pub fd: libc::c_int,
    pub ai_family: sa_family_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftp_t {
    pub cwd: [libc::c_char; 1024],
    pub message: *mut libc::c_char,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub data_tcp: tcp_t,
    pub proto: libc::c_int,
    pub ftp_mode: libc::c_int,
    pub local_if: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_t {
    pub host: [libc::c_char; 1024],
    pub auth: [libc::c_char; 1024],
    pub request: [abuf_t; 1],
    pub headers: [abuf_t; 1],
    pub port: libc::c_int,
    pub proto: libc::c_int,
    pub proxy: libc::c_int,
    pub proxy_auth: [libc::c_char; 1024],
    pub firstbyte: off_t,
    pub lastbyte: off_t,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub local_if: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_t {
    pub conf: *mut conf_t,
    pub proto: libc::c_int,
    pub port: libc::c_int,
    pub proxy: libc::c_int,
    pub host: [libc::c_char; 1024],
    pub dir: [libc::c_char; 1024],
    pub file: [libc::c_char; 1024],
    pub user: [libc::c_char; 1024],
    pub pass: [libc::c_char; 1024],
    pub output_filename: [libc::c_char; 1024],
    pub ftp: [ftp_t; 1],
    pub http: [http_t; 1],
    pub size: off_t,
    pub currentbyte: off_t,
    pub lastbyte: off_t,
    pub tcp: *mut tcp_t,
    pub enabled: bool,
    pub supported: bool,
    pub last_transfer: libc::c_int,
    pub message: *mut libc::c_char,
    pub local_if: *mut libc::c_char,
    pub state: bool,
    pub setup_thread: [pthread_t; 1],
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub url: [libc::c_char; 1024],
    pub speed_start_time: libc::c_double,
    pub speed: off_t,
    pub size: off_t,
    pub speed_thread: [pthread_t; 1],
    pub conf: *mut conf_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct axel_t {
    pub conn: *mut conn_t,
    pub conf: *mut conf_t,
    pub filename: [libc::c_char; 1024],
    pub start_time: libc::c_double,
    pub next_state: libc::c_int,
    pub finish_time: libc::c_int,
    pub bytes_done: off_t,
    pub start_byte: off_t,
    pub size: off_t,
    pub bytes_per_second: libc::c_longlong,
    pub delay_time: timespec,
    pub outfd: libc::c_int,
    pub ready: libc::c_int,
    pub message: *mut message_t,
    pub last_message: *mut message_t,
    pub url: *mut url_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn conf_hdr_make(
    mut dst: *mut libc::c_char,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) {
    snprintf(
        dst,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        k,
        v,
    );
}
pub static mut run: libc::c_int = 1 as libc::c_int;
static mut axel_options: [option; 20] = [
    {
        let mut init = option {
            name: b"max-speed\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"num-connections\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-redirect\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 256 as libc::c_int,
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
            name: b"search\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ipv4\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '4' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ipv6\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '6' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-proxy\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
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
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"alternate\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"percentage\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"insecure\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-clobber\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"header\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user-agent\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"timeout\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
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
unsafe extern "C" fn calc_percentage(mut cur: off_t, mut total: off_t) -> libc::c_uint {
    return ({
        let mut __a: libc::c_int = 100 as libc::c_int;
        let mut __b: libc::c_long = (100 as libc::c_int as libc::c_long * cur
            + total / 2 as libc::c_int as libc::c_long) / total;
        if (__a as libc::c_long) < __b { __a as libc::c_long } else { __b }
    }) as libc::c_uint;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hsize: [libc::c_char; 512] = [0; 512];
    let mut htime: [libc::c_char; 512] = [0; 512];
    let mut current_block: u64;
    let mut fn_0: [libc::c_char; 1024] = [0; 1024];
    let mut do_search: libc::c_int = 0 as libc::c_int;
    let mut search: *mut search_t = 0 as *mut search_t;
    let mut conf: [conf_t; 1] = [conf_t {
        default_filename: [0; 1024],
        http_proxy: [0; 1024],
        no_proxy: [0; 1024],
        num_connections: 0,
        strip_cgi_parameters: 0,
        save_state_interval: 0,
        connection_timeout: 0,
        reconnect_delay: 0,
        max_redirect: 0,
        buffer_size: 0,
        max_speed: 0,
        verbose: 0,
        alternate_output: 0,
        insecure: 0,
        no_clobber: 0,
        percentage: 0,
        interfaces: 0 as *mut if_t,
        ai_family: 0,
        search_timeout: 0,
        search_threads: 0,
        search_amount: 0,
        search_top: 0,
        io_timeout: 0,
        add_header_count: 0,
        add_header: [[0; 1024]; 10],
    }; 1];
    let mut axel: *mut axel_t = 0 as *mut axel_t;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    fn_0[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"axel\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"axel\0" as *const u8 as *const libc::c_char);
    if conf_init(conf.as_mut_ptr()) == 0 {
        return 1 as libc::c_int;
    }
    opterr = 0 as libc::c_int;
    j = -(1 as libc::c_int);
    loop {
        let mut option: libc::c_int = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"s:n:o:S::46NqvhVapkcH:U:T:\0" as *const u8 as *const libc::c_char,
            axel_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if option == -(1 as libc::c_int) {
            current_block = 7343950298149844727;
            break;
        }
        match option {
            85 => {
                conf_hdr_make(
                    ((*conf.as_mut_ptr())
                        .add_header[HDR_USER_AGENT as libc::c_int as usize])
                        .as_mut_ptr(),
                    b"User-Agent\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            72 => {
                if !((*conf.as_mut_ptr()).add_header_count < 10 as libc::c_int) {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Too many custom headers (-H)! Currently only %u custom headers can be appended.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        10 as libc::c_int - HDR_count_init as libc::c_int,
                    );
                    current_block = 15391250765399816830;
                    break;
                } else {
                    let ref mut fresh0 = (*conf.as_mut_ptr()).add_header_count;
                    let fresh1 = *fresh0;
                    *fresh0 = *fresh0 + 1;
                    strlcpy(
                        ((*conf.as_mut_ptr()).add_header[fresh1 as usize]).as_mut_ptr(),
                        optarg,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    );
                }
            }
            115 => {
                if !(sscanf(
                    optarg,
                    b"%llu\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr()).max_speed as *mut libc::c_ulonglong,
                ) == 0)
                {
                    continue;
                }
                print_help();
                current_block = 15391250765399816830;
                break;
            }
            110 => {
                if !(sscanf(
                    optarg,
                    b"%hu\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr()).num_connections as *mut uint16_t,
                ) == 0)
                {
                    continue;
                }
                print_help();
                current_block = 15391250765399816830;
                break;
            }
            256 => {
                if sscanf(
                    optarg,
                    b"%i\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr()).max_redirect as *mut libc::c_int,
                ) == 0
                {
                    print_help();
                    return 1 as libc::c_int;
                }
            }
            111 => {
                strlcpy(
                    fn_0.as_mut_ptr(),
                    optarg,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            83 => {
                do_search = 1 as libc::c_int;
                if optarg.is_null() {
                    continue;
                }
                if !(sscanf(
                    optarg,
                    b"%i\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr()).search_top as *mut libc::c_int,
                ) == 0)
                {
                    continue;
                }
                print_help();
                current_block = 15391250765399816830;
                break;
            }
            54 => {
                (*conf.as_mut_ptr()).ai_family = 10 as libc::c_int as sa_family_t;
            }
            52 => {
                (*conf.as_mut_ptr()).ai_family = 2 as libc::c_int as sa_family_t;
            }
            97 => {
                (*conf.as_mut_ptr()).alternate_output = 1 as libc::c_int;
            }
            112 => {
                (*conf.as_mut_ptr()).percentage = 1 as libc::c_int;
            }
            107 => {
                (*conf.as_mut_ptr()).insecure = 1 as libc::c_int;
            }
            99 => {
                (*conf.as_mut_ptr()).no_clobber = 1 as libc::c_int;
            }
            78 => {
                *((*conf.as_mut_ptr()).http_proxy)
                    .as_mut_ptr() = 0 as libc::c_int as libc::c_char;
            }
            104 => {
                print_help();
                ret = 0 as libc::c_int;
                current_block = 15391250765399816830;
                break;
            }
            118 => {
                if j == -(1 as libc::c_int) {
                    j = 1 as libc::c_int;
                } else {
                    j += 1;
                    j;
                }
            }
            86 => {
                print_version_info();
                ret = 0 as libc::c_int;
                current_block = 15391250765399816830;
                break;
            }
            113 => {
                close(1 as libc::c_int);
                (*conf.as_mut_ptr()).verbose = -(1 as libc::c_int);
                if !(open(
                    b"/dev/null\0" as *const u8 as *const libc::c_char,
                    0o1 as libc::c_int,
                ) != 1 as libc::c_int)
                {
                    continue;
                }
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Can't redirect stdout to /dev/null.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                current_block = 15391250765399816830;
                break;
            }
            84 => {
                (*conf.as_mut_ptr())
                    .io_timeout = strtoul(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                ) as libc::c_uint;
            }
            _ => {
                print_help();
                current_block = 15391250765399816830;
                break;
            }
        }
    }
    match current_block {
        7343950298149844727 => {
            if (*conf.as_mut_ptr()).verbose < 0 as libc::c_int {
                (*conf.as_mut_ptr()).alternate_output = 0 as libc::c_int;
                (*conf.as_mut_ptr()).percentage = 0 as libc::c_int;
            } else if j > -(1 as libc::c_int) {
                (*conf.as_mut_ptr()).verbose = j;
            }
            if ((*conf.as_mut_ptr()).num_connections as libc::c_int) < 1 as libc::c_int {
                print_help();
            } else {
                if (*conf.as_mut_ptr()).max_redirect < 0 as libc::c_int {
                    print_help();
                    return 1 as libc::c_int;
                }
                if argc - optind == 0 as libc::c_int {
                    print_help();
                } else {
                    if strcmp(
                        *argv.offset(optind as isize),
                        b"-\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        s = malloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
                        if s.is_null() {
                            current_block = 15391250765399816830;
                        } else if scanf(
                            b"%1024[^\n]s\0" as *const u8 as *const libc::c_char,
                            s,
                        ) != 1 as libc::c_int
                        {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Error when trying to read URL (Too long?).\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            free(s as *mut libc::c_void);
                            current_block = 15391250765399816830;
                        } else {
                            current_block = 10435735846551762309;
                        }
                    } else {
                        s = *argv.offset(optind as isize);
                        if strlen(s) > 1024 as libc::c_int as size_t {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Can't handle URLs of length over %zu\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                1024 as libc::c_int as size_t,
                            );
                            current_block = 15391250765399816830;
                        } else {
                            current_block = 10435735846551762309;
                        }
                    }
                    match current_block {
                        15391250765399816830 => {}
                        _ => {
                            if (*conf.as_mut_ptr()).percentage == 0 {
                                printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Initializing download: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    s,
                                );
                            }
                            if do_search != 0 {
                                search = calloc(
                                    ((*conf.as_mut_ptr()).search_amount + 1 as libc::c_int)
                                        as libc::c_ulong,
                                    ::std::mem::size_of::<search_t>() as libc::c_ulong,
                                ) as *mut search_t;
                                if search.is_null() {
                                    current_block = 15391250765399816830;
                                } else {
                                    let ref mut fresh2 = (*search
                                        .offset(0 as libc::c_int as isize))
                                        .conf;
                                    *fresh2 = conf.as_mut_ptr();
                                    if (*conf.as_mut_ptr()).verbose != 0 {
                                        printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Doing search...\n\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    }
                                    let mut i: libc::c_int = search_makelist(search, s);
                                    if i < 0 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"File not found\n\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        current_block = 15391250765399816830;
                                    } else {
                                        if (*conf.as_mut_ptr()).verbose != 0 {
                                            printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Testing speeds, this can take a while...\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                        }
                                        j = search_getspeeds(search, i);
                                        if j < 0 as libc::c_int {
                                            fprintf(
                                                stderr,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Speed testing failed\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            return 1 as libc::c_int;
                                        }
                                        search_sortlist(search, i);
                                        if (*conf.as_mut_ptr()).verbose != 0 {
                                            printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"%i usable servers found, will use these URLs:\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                j,
                                            );
                                            j = ({
                                                let mut __a: libc::c_int = j;
                                                let mut __b: libc::c_int = (*conf.as_mut_ptr()).search_top;
                                                if __a < __b { __a } else { __b }
                                            });
                                            printf(
                                                b"%-60s %15s\n\0" as *const u8 as *const libc::c_char,
                                                b"URL\0" as *const u8 as *const libc::c_char,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Speed\0" as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            i = 0 as libc::c_int;
                                            while i < j {
                                                printf(
                                                    b"%-70.70s %5jd\n\0" as *const u8 as *const libc::c_char,
                                                    ((*search.offset(i as isize)).url).as_mut_ptr(),
                                                    (*search.offset(i as isize)).speed,
                                                );
                                                i += 1;
                                                i;
                                            }
                                            printf(b"\n\0" as *const u8 as *const libc::c_char);
                                        }
                                        axel = axel_new(conf.as_mut_ptr(), j, search);
                                        free(search as *mut libc::c_void);
                                        if axel.is_null() || (*axel).ready == -(1 as libc::c_int) {
                                            print_messages(axel);
                                            current_block = 15450582732539524906;
                                        } else {
                                            current_block = 7639320476250304355;
                                        }
                                    }
                                }
                            } else {
                                search = calloc(
                                    (argc - optind) as libc::c_ulong,
                                    ::std::mem::size_of::<search_t>() as libc::c_ulong,
                                ) as *mut search_t;
                                if search.is_null() {
                                    current_block = 15391250765399816830;
                                } else {
                                    let mut i_0: libc::c_int = 0 as libc::c_int;
                                    while i_0 < argc - optind {
                                        strlcpy(
                                            ((*search.offset(i_0 as isize)).url).as_mut_ptr(),
                                            *argv.offset((optind + i_0) as isize),
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                        );
                                        i_0 += 1;
                                        i_0;
                                    }
                                    axel = axel_new(conf.as_mut_ptr(), argc - optind, search);
                                    free(search as *mut libc::c_void);
                                    if axel.is_null() || (*axel).ready == -(1 as libc::c_int) {
                                        print_messages(axel);
                                        current_block = 15450582732539524906;
                                    } else {
                                        current_block = 7639320476250304355;
                                    }
                                }
                            }
                            match current_block {
                                15391250765399816830 => {}
                                _ => {
                                    match current_block {
                                        7639320476250304355 => {
                                            print_messages(axel);
                                            if s != *argv.offset(optind as isize) {
                                                free(s as *mut libc::c_void);
                                            }
                                            if *fn_0.as_mut_ptr() != 0 {
                                                let mut buf: stat = stat {
                                                    st_dev: 0,
                                                    st_ino: 0,
                                                    st_nlink: 0,
                                                    st_mode: 0,
                                                    st_uid: 0,
                                                    st_gid: 0,
                                                    __pad0: 0,
                                                    st_rdev: 0,
                                                    st_size: 0,
                                                    st_blksize: 0,
                                                    st_blocks: 0,
                                                    st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                                                    st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                                                    st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                                                    __glibc_reserved: [0; 3],
                                                };
                                                if stat(fn_0.as_mut_ptr(), &mut buf) == 0 as libc::c_int {
                                                    if buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                                        == 0o40000 as libc::c_int as libc::c_uint
                                                    {
                                                        let mut fnlen: size_t = strlen(fn_0.as_mut_ptr());
                                                        let mut axelfnlen: size_t = strlen(
                                                            ((*axel).filename).as_mut_ptr(),
                                                        );
                                                        if fnlen
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                            .wrapping_add(axelfnlen)
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                            > 1024 as libc::c_int as size_t
                                                        {
                                                            fprintf(
                                                                stderr,
                                                                dcgettext(
                                                                    0 as *const libc::c_char,
                                                                    b"Filename too long!\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                ),
                                                            );
                                                            current_block = 15450582732539524906;
                                                        } else {
                                                            fn_0[fnlen as usize] = '/' as i32 as libc::c_char;
                                                            memcpy(
                                                                fn_0
                                                                    .as_mut_ptr()
                                                                    .offset(fnlen as isize)
                                                                    .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                                                ((*axel).filename).as_mut_ptr() as *const libc::c_void,
                                                                axelfnlen,
                                                            );
                                                            fn_0[fnlen
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                .wrapping_add(axelfnlen)
                                                                as usize] = '\0' as i32 as libc::c_char;
                                                            current_block = 15947798178928648489;
                                                        }
                                                    } else {
                                                        current_block = 15947798178928648489;
                                                    }
                                                } else {
                                                    current_block = 15947798178928648489;
                                                }
                                                match current_block {
                                                    15450582732539524906 => {}
                                                    _ => {
                                                        let mut statefn: [libc::c_char; 1027] = [0; 1027];
                                                        snprintf(
                                                            statefn.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 1027]>()
                                                                as libc::c_ulong,
                                                            b"%s.st\0" as *const u8 as *const libc::c_char,
                                                            fn_0.as_mut_ptr(),
                                                        );
                                                        if access(fn_0.as_mut_ptr(), 0 as libc::c_int)
                                                            == 0 as libc::c_int
                                                            && access(statefn.as_mut_ptr(), 0 as libc::c_int)
                                                                != 0 as libc::c_int
                                                        {
                                                            fprintf(
                                                                stderr,
                                                                dcgettext(
                                                                    0 as *const libc::c_char,
                                                                    b"No state file, cannot resume!\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                ),
                                                            );
                                                            current_block = 15450582732539524906;
                                                        } else {
                                                            if access(statefn.as_mut_ptr(), 0 as libc::c_int)
                                                                == 0 as libc::c_int
                                                                && access(fn_0.as_mut_ptr(), 0 as libc::c_int)
                                                                    != 0 as libc::c_int
                                                            {
                                                                printf(
                                                                    dcgettext(
                                                                        0 as *const libc::c_char,
                                                                        b"State file found, but no downloaded data. Starting from scratch.\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        5 as libc::c_int,
                                                                    ),
                                                                );
                                                                unlink(statefn.as_mut_ptr());
                                                            }
                                                            strlcpy(
                                                                ((*axel).filename).as_mut_ptr(),
                                                                fn_0.as_mut_ptr(),
                                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                    as libc::c_ulong,
                                                            );
                                                            current_block = 18221534353613080499;
                                                        }
                                                    }
                                                }
                                            } else {
                                                s = ((*axel).filename)
                                                    .as_mut_ptr()
                                                    .offset(strlen(((*axel).filename).as_mut_ptr()) as isize);
                                                let mut i_1: libc::c_int = 0 as libc::c_int;
                                                loop {
                                                    let mut statefn_0: [libc::c_char; 1027] = [0; 1027];
                                                    snprintf(
                                                        statefn_0.as_mut_ptr(),
                                                        ::std::mem::size_of::<[libc::c_char; 1027]>()
                                                            as libc::c_ulong,
                                                        b"%s.st\0" as *const u8 as *const libc::c_char,
                                                        ((*axel).filename).as_mut_ptr(),
                                                    );
                                                    let mut f_exists: libc::c_int = (access(
                                                        ((*axel).filename).as_mut_ptr(),
                                                        0 as libc::c_int,
                                                    ) == 0) as libc::c_int;
                                                    let mut st_exists: libc::c_int = (access(
                                                        statefn_0.as_mut_ptr(),
                                                        0 as libc::c_int,
                                                    ) == 0) as libc::c_int;
                                                    if f_exists != 0 {
                                                        if (*((*axel).conn).offset(0 as libc::c_int as isize))
                                                            .supported as libc::c_int != 0 && st_exists != 0
                                                        {
                                                            break;
                                                        }
                                                    } else if st_exists == 0 {
                                                        break;
                                                    }
                                                    snprintf(
                                                        s,
                                                        ((*axel).filename)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                    as libc::c_ulong as isize,
                                                            )
                                                            .offset_from(s) as libc::c_long as libc::c_ulong,
                                                        b".%i\0" as *const u8 as *const libc::c_char,
                                                        i_1,
                                                    );
                                                    i_1 += 1;
                                                    i_1;
                                                }
                                                current_block = 18221534353613080499;
                                            }
                                            match current_block {
                                                15450582732539524906 => {}
                                                _ => {
                                                    if axel_open(axel) == 0 {
                                                        print_messages(axel);
                                                    } else {
                                                        print_messages(axel);
                                                        axel_start(axel);
                                                        print_messages(axel);
                                                        if (*conf.as_mut_ptr()).alternate_output != 0
                                                            || (*conf.as_mut_ptr()).percentage != 0
                                                        {
                                                            putchar('\n' as i32);
                                                        } else if (*axel).bytes_done
                                                            > 0 as libc::c_int as libc::c_long
                                                        {
                                                            putchar('\n' as i32);
                                                            print_commas((*axel).bytes_done);
                                                            fflush(stdout);
                                                        }
                                                        (*axel).start_byte = (*axel).bytes_done;
                                                        signal(
                                                            2 as libc::c_int,
                                                            Some(stop as unsafe extern "C" fn(libc::c_int) -> ()),
                                                        );
                                                        signal(
                                                            15 as libc::c_int,
                                                            Some(stop as unsafe extern "C" fn(libc::c_int) -> ()),
                                                        );
                                                        while (*axel).ready == 0 && run != 0 {
                                                            let mut prev: off_t = 0;
                                                            prev = (*axel).bytes_done;
                                                            axel_do(axel);
                                                            if (*conf.as_mut_ptr()).percentage != 0 {
                                                                if ((*axel).message).is_null() && prev != (*axel).bytes_done
                                                                {
                                                                    printf(
                                                                        b"%u\n\0" as *const u8 as *const libc::c_char,
                                                                        calc_percentage((*axel).bytes_done, (*axel).size),
                                                                    );
                                                                }
                                                            } else if (*conf.as_mut_ptr()).alternate_output != 0 {
                                                                if ((*axel).message).is_null() && prev != (*axel).bytes_done
                                                                {
                                                                    print_alternate_output(axel);
                                                                }
                                                            } else if (*conf.as_mut_ptr()).verbose > -(1 as libc::c_int)
                                                            {
                                                                print_progress(
                                                                    (*axel).bytes_done,
                                                                    prev,
                                                                    (*axel).size,
                                                                    (*axel).bytes_per_second as libc::c_double
                                                                        / 1024 as libc::c_int as libc::c_double,
                                                                );
                                                            }
                                                            if !((*axel).message).is_null() {
                                                                if (*conf.as_mut_ptr()).alternate_output == 1 as libc::c_int
                                                                {
                                                                    fputs(
                                                                        b"\x1B[2K\r\0" as *const u8 as *const libc::c_char,
                                                                        stdout,
                                                                    );
                                                                } else {
                                                                    putchar('\n' as i32);
                                                                }
                                                                print_messages(axel);
                                                                if (*axel).ready == 0 {
                                                                    if (*conf.as_mut_ptr()).alternate_output != 1 as libc::c_int
                                                                    {
                                                                        print_commas((*axel).bytes_done);
                                                                    } else {
                                                                        print_alternate_output(axel);
                                                                    }
                                                                }
                                                            } else if (*axel).ready != 0 {
                                                                putchar('\n' as i32);
                                                            }
                                                            fflush(stdout);
                                                        }
                                                        hsize = [0; 512];
                                                        htime = [0; 512];
                                                        time_human(
                                                            htime.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                                                as libc::c_ulong,
                                                            (axel_gettime() - (*axel).start_time) as libc::c_uint,
                                                        );
                                                        axel_size_human(
                                                            hsize.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                                                as libc::c_ulong,
                                                            ((*axel).bytes_done - (*axel).start_byte) as size_t,
                                                        );
                                                        printf(
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"\nDownloaded %s in %s. (%.2f KB/s)\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            hsize.as_mut_ptr(),
                                                            htime.as_mut_ptr(),
                                                            (*axel).bytes_per_second as libc::c_double
                                                                / 1024 as libc::c_int as libc::c_double,
                                                        );
                                                        ret = if (*axel).ready != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            2 as libc::c_int
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    axel_close(axel);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    conf_free(conf.as_mut_ptr());
    return ret;
}
unsafe extern "C" fn stop(mut signal_0: libc::c_int) {
    run = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn log2i(mut x: libc::c_ulonglong) -> libc::c_uint {
    return (if x != 0 {
        (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(x.leading_zeros() as i32 as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_uint;
}
pub unsafe extern "C" fn axel_size_human(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut value: size_t,
) -> *mut libc::c_char {
    let mut fval: libc::c_double = value as libc::c_double;
    let oname: [*const libc::c_char; 5] = [
        b"\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Kilo\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ) as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Mega\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ) as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Giga\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ) as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Tera\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ) as *const libc::c_char,
    ];
    let order: libc::c_uint = ({
        let mut __a: libc::c_ulong = (::std::mem::size_of::<[*const libc::c_char; 5]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut __b: libc::c_uint = (log2i(fval as libc::c_ulonglong))
            .wrapping_div(10 as libc::c_int as libc::c_uint);
        if __a < __b as libc::c_ulong { __a } else { __b as libc::c_ulong }
    }) as libc::c_uint;
    fval
        /= ((1 as libc::c_int) << order.wrapping_mul(10 as libc::c_int as libc::c_uint))
            as libc::c_double;
    let mut ret: libc::c_int = snprintf(
        dst,
        len,
        dcgettext(
            0 as *const libc::c_char,
            b"%g %sbyte(s)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fval,
        oname[order as usize],
    );
    return if ret < 0 as libc::c_int { 0 as *mut libc::c_char } else { dst };
}
unsafe extern "C" fn time_human(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut value: libc::c_uint,
) -> *mut libc::c_char {
    let mut hh: libc::c_uint = 0;
    let mut mm: libc::c_uint = 0;
    let mut ss: libc::c_uint = 0;
    ss = value.wrapping_rem(60 as libc::c_int as libc::c_uint);
    mm = value
        .wrapping_div(60 as libc::c_int as libc::c_uint)
        .wrapping_rem(60 as libc::c_int as libc::c_uint);
    hh = value.wrapping_div(3600 as libc::c_int as libc::c_uint);
    let mut ret: libc::c_int = 0;
    if hh != 0 {
        ret = snprintf(
            dst,
            len,
            dcgettext(
                0 as *const libc::c_char,
                b"%i:%02i:%02i hour(s)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            hh,
            mm,
            ss,
        );
    } else if mm != 0 {
        ret = snprintf(
            dst,
            len,
            dcgettext(
                0 as *const libc::c_char,
                b"%i:%02i minute(s)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            mm,
            ss,
        );
    } else {
        ret = snprintf(
            dst,
            len,
            dcgettext(
                0 as *const libc::c_char,
                b"%i second(s)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ss,
        );
    }
    return if ret < 0 as libc::c_int { 0 as *mut libc::c_char } else { dst };
}
unsafe extern "C" fn print_commas(mut bytes_done: off_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    printf(b"       \0" as *const u8 as *const libc::c_char);
    j = (bytes_done / 1024 as libc::c_int as libc::c_long
        % 50 as libc::c_int as libc::c_long) as libc::c_int;
    if j == 0 as libc::c_int {
        j = 50 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < j {
        if i % 10 as libc::c_int == 0 as libc::c_int {
            putchar(' ' as i32);
        }
        putchar(',' as i32);
        i += 1;
        i;
    }
}
unsafe extern "C" fn print_progress(
    mut cur: off_t,
    mut prev: off_t,
    mut total: off_t,
    mut kbps: libc::c_double,
) {
    prev /= 1024 as libc::c_int as libc::c_long;
    cur /= 1024 as libc::c_int as libc::c_long;
    let mut print_speed: bool = prev > 0 as libc::c_int as libc::c_long;
    let mut i: off_t = prev;
    while i < cur {
        if i % 50 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long {
            if print_speed {
                printf(b"  [%6.1fKB/s]\0" as *const u8 as *const libc::c_char, kbps);
            }
            if total as libc::c_longlong == 9223372036854775807 as libc::c_longlong {
                printf(b"\n[ N/A]  \0" as *const u8 as *const libc::c_char);
            } else {
                printf(
                    b"\n[%3u%%]  \0" as *const u8 as *const libc::c_char,
                    calc_percentage(1024 as libc::c_int as libc::c_long * i, total),
                );
            }
        } else if i % 10 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            putchar(' ' as i32);
        }
        putchar('.' as i32);
        i += 1;
        i;
    }
}
unsafe extern "C" fn alt_id(mut n: libc::c_int) -> libc::c_char {
    let mut p: *const libc::c_char = b"09AZaz\0" as *const u8 as *const libc::c_char;
    while *p as libc::c_int != 0
        && n
            > *p.offset(1 as libc::c_int as isize) as libc::c_int
                - *p.offset(0 as libc::c_int as isize) as libc::c_int
    {
        n
            -= *p.offset(1 as libc::c_int as isize) as libc::c_int
                - *p.offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
    }
    return (if *p as libc::c_int != 0 { *p as libc::c_int + n } else { '*' as i32 })
        as libc::c_char;
}
unsafe extern "C" fn print_alternate_output_progress(
    mut axel: *mut axel_t,
    mut progress: *mut libc::c_char,
    mut width: libc::c_int,
    mut done: off_t,
    mut total: off_t,
    mut now: libc::c_double,
) {
    if width == 0 {
        width = 1 as libc::c_int;
    }
    if total == 0 {
        total = 1 as libc::c_int as off_t;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        let mut offset: libc::c_int = ((*((*axel).conn).offset(i as isize)).currentbyte
            * width as libc::c_long / total) as libc::c_int;
        if (*((*axel).conn).offset(i as isize)).currentbyte
            < (*((*axel).conn).offset(i as isize)).lastbyte
        {
            if now
                <= ((*((*axel).conn).offset(i as isize)).last_transfer
                    + (*(*axel).conf).connection_timeout / 2 as libc::c_int)
                    as libc::c_double
            {
                *progress.offset(offset as isize) = alt_id(i);
            } else {
                *progress.offset(offset as isize) = '#' as i32 as libc::c_char;
            }
        }
        memset(
            progress.offset(offset as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            ' ' as i32,
            ({
                let mut __a: libc::c_int = 0 as libc::c_int;
                let mut __b: libc::c_long = (*((*axel).conn).offset(i as isize)).lastbyte
                    * width as libc::c_long / total - offset as libc::c_long
                    - 1 as libc::c_int as libc::c_long;
                if __a as libc::c_long > __b { __a as libc::c_long } else { __b }
            }) as libc::c_ulong,
        );
        i += 1;
        i;
    }
    *progress.offset(width as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"\r[%3u%%] [%s\0" as *const u8 as *const libc::c_char,
        calc_percentage(done, total),
        progress,
    );
}
unsafe extern "C" fn print_alternate_output(mut axel: *mut axel_t) {
    let mut done: off_t = (*axel).bytes_done;
    let mut total: off_t = (*axel).size;
    let mut now: libc::c_double = axel_gettime();
    let mut width: libc::c_int = get_term_width();
    let mut progress: *mut libc::c_char = 0 as *mut libc::c_char;
    if width < 40 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Can't setup alternate output. Deactivating.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        (*(*axel).conf).alternate_output = 0 as libc::c_int;
        return;
    }
    width -= 30 as libc::c_int;
    progress = malloc((width + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    if progress.is_null() {
        return;
    }
    memset(progress as *mut libc::c_void, '.' as i32, width as libc::c_ulong);
    if total as libc::c_longlong != 9223372036854775807 as libc::c_longlong {
        print_alternate_output_progress(axel, progress, width, done, total, now);
    } else {
        *progress.offset(width as isize) = '\0' as i32 as libc::c_char;
        printf(b"\r[ N/A] [%s\0" as *const u8 as *const libc::c_char, progress);
    }
    if (*axel).bytes_per_second > 1048576 as libc::c_int as libc::c_longlong {
        printf(
            b"] [%6.1fMB/s]\0" as *const u8 as *const libc::c_char,
            (*axel).bytes_per_second as libc::c_double
                / (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double,
        );
    } else if (*axel).bytes_per_second > 1024 as libc::c_int as libc::c_longlong {
        printf(
            b"] [%6.1fKB/s]\0" as *const u8 as *const libc::c_char,
            (*axel).bytes_per_second as libc::c_double
                / 1024 as libc::c_int as libc::c_double,
        );
    } else {
        printf(
            b"] [%6.1fB/s]\0" as *const u8 as *const libc::c_char,
            (*axel).bytes_per_second as libc::c_double,
        );
    }
    if total as libc::c_longlong != 9223372036854775807 as libc::c_longlong
        && done < total
    {
        let mut seconds: libc::c_int = 0;
        let mut minutes: libc::c_int = 0;
        let mut hours: libc::c_int = 0;
        let mut days: libc::c_int = 0;
        seconds = ((*axel).finish_time as libc::c_double - now) as libc::c_int;
        minutes = seconds / 60 as libc::c_int;
        seconds -= minutes * 60 as libc::c_int;
        hours = minutes / 60 as libc::c_int;
        minutes -= hours * 60 as libc::c_int;
        days = hours / 24 as libc::c_int;
        hours -= days * 24 as libc::c_int;
        if days != 0 {
            printf(b" [%2dd%2d]\0" as *const u8 as *const libc::c_char, days, hours);
        } else if hours != 0 {
            printf(b" [%2dh%02d]\0" as *const u8 as *const libc::c_char, hours, minutes);
        } else {
            printf(
                b" [%02d:%02d]\0" as *const u8 as *const libc::c_char,
                minutes,
                seconds,
            );
        }
    }
    free(progress as *mut libc::c_void);
}
unsafe extern "C" fn get_term_width() -> libc::c_int {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    ioctl(
        1 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut w as *mut winsize,
    );
    return w.ws_col as libc::c_int;
}
unsafe extern "C" fn print_help() {
    print_version();
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: axel [options] url1 [url2] [url...]\n\n--max-speed=x\t\t-s x\tSpecify maximum speed (bytes per second)\n--num-connections=x\t-n x\tSpecify maximum number of connections\n--max-redirect=x\t\tSpecify maximum number of redirections\n--output=f\t\t-o f\tSpecify local output file\n--search[=n]\t\t-S[n]\tSearch for mirrors and download from n servers\n--ipv4\t\t\t-4\tUse the IPv4 protocol\n--ipv6\t\t\t-6\tUse the IPv6 protocol\n--header=x\t\t-H x\tAdd HTTP header string\n--user-agent=x\t\t-U x\tSet user agent\n--no-proxy\t\t-N\tJust don't use any proxy server\n--insecure\t\t-k\tDon't verify the SSL certificate\n--no-clobber\t\t-c\tSkip download if file already exists\n--quiet\t\t\t-q\tLeave stdout alone\n--verbose\t\t-v\tMore status information\n--alternate\t\t-a\tAlternate progress indicator\n--percentage\t\t-p\tPrint simple percentages instead of progress bar (0-100)\n--help\t\t\t-h\tThis information\n--timeout=x\t\t-T x\tSet I/O and connection timeout\n--version\t\t-V\tVersion information\n\nVisit https://github.com/axel-download-accelerator/axel/issues to report bugs\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn print_version() {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Axel %s (%s)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"2.17.11\0" as *const u8 as *const libc::c_char,
        b"linux-gnu\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn print_version_info() {
    print_version();
    printf(
        b"\nCopyright 2001-2007 Wilmer van der Gaast,\n\t  2007-2009 Giridhar Appaji Nag,\n\t  2008-2010 Philipp Hagemeister,\n\t  2015-2017 Joao Eriberto Mota Filho,\n\t  2016-2017 Stephen Thirlwall,\n\t  2017      Ismael Luceno,\n\t  2017      Antonio Quartulli,\n\t\t    %s\n%s\n\n\0"
            as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"and others.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Please, see the CREDITS file.\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn print_messages(mut axel: *mut axel_t) {
    let mut m: *mut message_t = 0 as *mut message_t;
    if axel.is_null() {
        return;
    }
    loop {
        m = (*axel).message;
        if m.is_null() {
            break;
        }
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, ((*m).text).as_mut_ptr());
        (*axel).message = (*m).next as *mut message_t;
        free(m as *mut libc::c_void);
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

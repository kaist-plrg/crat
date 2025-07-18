use ::libc;
use ::c2rust_bitfields;
use ::f128;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type lua_State;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut VERSION: *const libc::c_char;
    fn http_body_is_final(parser: *const http_parser) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn http_should_keep_alive(parser: *const http_parser) -> libc::c_int;
    fn http_parser_execute(
        parser: *mut http_parser,
        settings: *const http_parser_settings,
        data: *const libc::c_char,
        len: size_t,
    ) -> size_t;
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn __errno_location() -> *mut libc::c_int;
    fn ERR_print_errors_fp(fp: *mut FILE);
    fn stats_alloc(_: uint64_t) -> *mut stats;
    fn stats_record(_: *mut stats, _: uint64_t) -> libc::c_int;
    fn stats_correct(_: *mut stats, _: int64_t);
    fn stats_mean(_: *mut stats) -> f128::f128;
    fn stats_stdev(stats: *mut stats, _: f128::f128) -> f128::f128;
    fn stats_within_stdev(
        _: *mut stats,
        _: f128::f128,
        _: f128::f128,
        _: uint64_t,
    ) -> f128::f128;
    fn stats_percentile(_: *mut stats, _: f128::f128) -> uint64_t;
    fn aeCreateEventLoop(setsize: libc::c_int) -> *mut aeEventLoop;
    fn aeDeleteEventLoop(eventLoop: *mut aeEventLoop);
    fn aeStop(eventLoop: *mut aeEventLoop);
    fn aeCreateFileEvent(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        mask: libc::c_int,
        proc_0: Option::<aeFileProc>,
        clientData: *mut libc::c_void,
    ) -> libc::c_int;
    fn aeDeleteFileEvent(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        mask: libc::c_int,
    );
    fn aeCreateTimeEvent(
        eventLoop: *mut aeEventLoop,
        milliseconds: libc::c_longlong,
        proc_0: Option::<aeTimeProc>,
        clientData: *mut libc::c_void,
        finalizerProc: Option::<aeEventFinalizerProc>,
    ) -> libc::c_longlong;
    fn aeMain(eventLoop: *mut aeEventLoop);
    fn aeGetApiName() -> *mut libc::c_char;
    fn http_parser_init(parser: *mut http_parser, type_0: http_parser_type);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn script_want_response(L: *mut lua_State) -> bool;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn script_create(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> *mut lua_State;
    fn script_resolve(
        _: *mut lua_State,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
    ) -> bool;
    fn script_done(_: *mut lua_State, _: *mut stats, _: *mut stats);
    fn script_init(
        _: *mut lua_State,
        _: *mut thread,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    );
    fn script_delay(_: *mut lua_State) -> uint64_t;
    fn script_request(_: *mut lua_State, _: *mut *mut libc::c_char, _: *mut size_t);
    fn script_response(
        _: *mut lua_State,
        _: libc::c_int,
        _: *mut buffer,
        _: *mut buffer,
    );
    fn script_verify_request(L: *mut lua_State) -> size_t;
    fn script_is_static(_: *mut lua_State) -> bool;
    fn script_has_delay(L: *mut lua_State) -> bool;
    fn script_has_done(L: *mut lua_State) -> bool;
    fn script_summary(_: *mut lua_State, _: uint64_t, _: uint64_t, _: uint64_t);
    fn script_errors(_: *mut lua_State, _: *mut errors);
    fn script_parse_url(_: *mut libc::c_char, _: *mut http_parser_url) -> libc::c_int;
    fn buffer_append(_: *mut buffer, _: *const libc::c_char, _: size_t);
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sock_connect(_: *mut connection, _: *mut libc::c_char) -> status;
    fn sock_close(_: *mut connection) -> status;
    fn sock_read(_: *mut connection, _: *mut size_t) -> status;
    fn sock_write(
        _: *mut connection,
        _: *mut libc::c_char,
        _: size_t,
        _: *mut size_t,
    ) -> status;
    fn sock_readable(_: *mut connection) -> size_t;
    fn ssl_init() -> *mut SSL_CTX;
    fn ssl_connect(_: *mut connection, _: *mut libc::c_char) -> status;
    fn ssl_close(_: *mut connection) -> status;
    fn ssl_read(_: *mut connection, _: *mut size_t) -> status;
    fn ssl_write(
        _: *mut connection,
        _: *mut libc::c_char,
        _: size_t,
        _: *mut size_t,
    ) -> status;
    fn ssl_readable(_: *mut connection) -> size_t;
    fn format_binary(_: f128::f128) -> *mut libc::c_char;
    fn format_metric(_: f128::f128) -> *mut libc::c_char;
    fn format_time_us(_: f128::f128) -> *mut libc::c_char;
    fn format_time_s(_: f128::f128) -> *mut libc::c_char;
    fn scan_metric(_: *mut libc::c_char, _: *mut uint64_t) -> libc::c_int;
    fn scan_time(_: *mut libc::c_char, _: *mut uint64_t) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type pthread_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
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
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errors {
    pub connect: uint32_t,
    pub read: uint32_t,
    pub write: uint32_t,
    pub status: uint32_t,
    pub timeout: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub count: uint64_t,
    pub limit: uint64_t,
    pub min: uint64_t,
    pub max: uint64_t,
    pub data: [uint64_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub lastTime: time_t,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when_sec: libc::c_long,
    pub when_ms: libc::c_long,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub next: *mut aeTimeEvent,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "2..=9")]
    #[bitfield(name = "state", ty = "libc::c_uint", bits = "10..=16")]
    #[bitfield(name = "header_state", ty = "libc::c_uint", bits = "17..=23")]
    #[bitfield(name = "index", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "lenient_http_headers", ty = "libc::c_uint", bits = "31..=31")]
    pub type_0_flags_state_header_state_index_lenient_http_headers: [u8; 4],
    pub nread: uint32_t,
    pub content_length: uint64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    #[bitfield(name = "status_code", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "method", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "http_errno", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "upgrade", ty = "libc::c_uint", bits = "31..=31")]
    pub status_code_method_http_errno_upgrade: [u8; 4],
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_url: http_data_cb,
    pub on_status: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
    pub on_chunk_header: http_cb,
    pub on_chunk_complete: http_cb,
}
pub type http_cb = Option::<unsafe extern "C" fn(*mut http_parser) -> libc::c_int>;
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type http_parser_type = libc::c_uint;
pub const HTTP_BOTH: http_parser_type = 2;
pub const HTTP_RESPONSE: http_parser_type = 1;
pub const HTTP_REQUEST: http_parser_type = 0;
pub type http_parser_url_fields = libc::c_uint;
pub const UF_MAX: http_parser_url_fields = 7;
pub const UF_USERINFO: http_parser_url_fields = 6;
pub const UF_FRAGMENT: http_parser_url_fields = 5;
pub const UF_QUERY: http_parser_url_fields = 4;
pub const UF_PATH: http_parser_url_fields = 3;
pub const UF_PORT: http_parser_url_fields = 2;
pub const UF_HOST: http_parser_url_fields = 1;
pub const UF_SCHEMA: http_parser_url_fields = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_url {
    pub field_set: uint16_t,
    pub port: uint16_t,
    pub field_data: [C2RustUnnamed_0; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub off: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread {
    pub thread: pthread_t,
    pub loop_0: *mut aeEventLoop,
    pub addr: *mut addrinfo,
    pub connections: uint64_t,
    pub complete: uint64_t,
    pub requests: uint64_t,
    pub bytes: uint64_t,
    pub start: uint64_t,
    pub L: *mut lua_State,
    pub errors: errors,
    pub cs: *mut connection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub thread: *mut thread,
    pub parser: http_parser,
    pub state: C2RustUnnamed_1,
    pub fd: libc::c_int,
    pub ssl: *mut SSL,
    pub delayed: bool,
    pub start: uint64_t,
    pub request: *mut libc::c_char,
    pub length: size_t,
    pub written: size_t,
    pub pending: uint64_t,
    pub headers: buffer,
    pub body: buffer,
    pub buf: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub buffer: *mut libc::c_char,
    pub length: size_t,
    pub cursor: *mut libc::c_char,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const VALUE: C2RustUnnamed_1 = 1;
pub const FIELD: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_12,
    pub _timer: C2RustUnnamed_11,
    pub _rt: C2RustUnnamed_10,
    pub _sigchld: C2RustUnnamed_9,
    pub _sigfault: C2RustUnnamed_6,
    pub _sigpoll: C2RustUnnamed_5,
    pub _sigsys: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _addr_bnd: C2RustUnnamed_8,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_13,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type status = libc::c_uint;
pub const RETRY: status = 2;
pub const ERROR: status = 1;
pub const OK: status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock {
    pub connect: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_char) -> status,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut connection) -> status>,
    pub read: Option::<unsafe extern "C" fn(*mut connection, *mut size_t) -> status>,
    pub write: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            size_t,
            *mut size_t,
        ) -> status,
    >,
    pub readable: Option::<unsafe extern "C" fn(*mut connection) -> size_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub connections: uint64_t,
    pub duration: uint64_t,
    pub threads: uint64_t,
    pub timeout: uint64_t,
    pub pipeline: uint64_t,
    pub delay: bool,
    pub dynamic: bool,
    pub latency: bool,
    pub host: *mut libc::c_char,
    pub script: *mut libc::c_char,
    pub ctx: *mut SSL_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub latency: *mut stats,
    pub requests: *mut stats,
}
static mut cfg: config = config {
    connections: 0,
    duration: 0,
    threads: 0,
    timeout: 0,
    pipeline: 0,
    delay: false,
    dynamic: false,
    latency: false,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    script: 0 as *const libc::c_char as *mut libc::c_char,
    ctx: 0 as *const SSL_CTX as *mut SSL_CTX,
};
static mut statistics: C2RustUnnamed_14 = C2RustUnnamed_14 {
    latency: 0 as *const stats as *mut stats,
    requests: 0 as *const stats as *mut stats,
};
static mut sock: sock = unsafe {
    {
        let mut init = sock {
            connect: Some(
                sock_connect
                    as unsafe extern "C" fn(*mut connection, *mut libc::c_char) -> status,
            ),
            close: Some(sock_close as unsafe extern "C" fn(*mut connection) -> status),
            read: Some(
                sock_read as unsafe extern "C" fn(*mut connection, *mut size_t) -> status,
            ),
            write: Some(
                sock_write
                    as unsafe extern "C" fn(
                        *mut connection,
                        *mut libc::c_char,
                        size_t,
                        *mut size_t,
                    ) -> status,
            ),
            readable: Some(
                sock_readable as unsafe extern "C" fn(*mut connection) -> size_t,
            ),
        };
        init
    }
};
static mut parser_settings: http_parser_settings = unsafe {
    {
        let mut init = http_parser_settings {
            on_message_begin: None,
            on_url: None,
            on_status: None,
            on_header_field: None,
            on_header_value: None,
            on_headers_complete: None,
            on_body: None,
            on_message_complete: Some(
                response_complete
                    as unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
            ),
            on_chunk_header: None,
            on_chunk_complete: None,
        };
        init
    }
};
static mut stop: sig_atomic_t = 0 as libc::c_int;
unsafe extern "C" fn handler(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(&mut stop as *mut sig_atomic_t, 1 as libc::c_int);
}
unsafe extern "C" fn usage() {
    printf(
        b"Usage: wrk <options> <url>                            \n  Options:                                            \n    -c, --connections <N>  Connections to keep open   \n    -d, --duration    <T>  Duration of test           \n    -t, --threads     <N>  Number of threads to use   \n                                                      \n    -s, --script      <S>  Load Lua script file       \n    -H, --header      <H>  Add header to request      \n        --latency          Print latency statistics   \n        --timeout     <T>  Socket/request timeout     \n    -v, --version          Print version details      \n                                                      \n  Numeric arguments may include a SI unit (1k, 1M, 1G)\n  Time arguments may include a time unit (2s, 2m, 2h)\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut headers: *mut *mut libc::c_char = zmalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut parts: http_parser_url = {
        let mut init = http_parser_url {
            field_set: 0,
            port: 0,
            field_data: [C2RustUnnamed_0 { off: 0, len: 0 }; 7],
        };
        init
    };
    if parse_args(&mut cfg, &mut url, &mut parts, headers, argc, argv) != 0 {
        usage();
        exit(1 as libc::c_int);
    }
    let mut schema: *mut libc::c_char = copy_url_part(url, &mut parts, UF_SCHEMA);
    let mut host: *mut libc::c_char = copy_url_part(url, &mut parts, UF_HOST);
    let mut port: *mut libc::c_char = copy_url_part(url, &mut parts, UF_PORT);
    let mut service: *mut libc::c_char = if !port.is_null() { port } else { schema };
    if strncmp(
        b"https\0" as *const u8 as *const libc::c_char,
        schema,
        5 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        cfg.ctx = ssl_init();
        if (cfg.ctx).is_null() {
            fprintf(
                stderr,
                b"unable to initialize SSL\n\0" as *const u8 as *const libc::c_char,
            );
            ERR_print_errors_fp(stderr);
            exit(1 as libc::c_int);
        }
        sock
            .connect = Some(
            ssl_connect
                as unsafe extern "C" fn(*mut connection, *mut libc::c_char) -> status,
        );
        sock.close = Some(ssl_close as unsafe extern "C" fn(*mut connection) -> status);
        sock
            .read = Some(
            ssl_read as unsafe extern "C" fn(*mut connection, *mut size_t) -> status,
        );
        sock
            .write = Some(
            ssl_write
                as unsafe extern "C" fn(
                    *mut connection,
                    *mut libc::c_char,
                    size_t,
                    *mut size_t,
                ) -> status,
        );
        sock
            .readable = Some(
            ssl_readable as unsafe extern "C" fn(*mut connection) -> size_t,
        );
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    statistics
        .latency = stats_alloc(
        (cfg.timeout).wrapping_mul(1000 as libc::c_int as libc::c_ulong),
    );
    statistics.requests = stats_alloc(10000000 as libc::c_int as uint64_t);
    let mut threads: *mut thread = zcalloc(
        (cfg.threads).wrapping_mul(::std::mem::size_of::<thread>() as libc::c_ulong),
    ) as *mut thread;
    let mut L: *mut lua_State = script_create(cfg.script, url, headers);
    if !script_resolve(L, host, service) {
        let mut msg: *mut libc::c_char = strerror(*__errno_location());
        fprintf(
            stderr,
            b"unable to connect to %s:%s %s\n\0" as *const u8 as *const libc::c_char,
            host,
            service,
            msg,
        );
        exit(1 as libc::c_int);
    }
    cfg.host = host;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < cfg.threads {
        let mut t: *mut thread = &mut *threads.offset(i as isize) as *mut thread;
        (*t)
            .loop_0 = aeCreateEventLoop(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (cfg.connections).wrapping_mul(3 as libc::c_int as libc::c_ulong),
                ) as libc::c_int,
        );
        (*t).connections = (cfg.connections).wrapping_div(cfg.threads);
        (*t).L = script_create(cfg.script, url, headers);
        script_init(L, t, argc - optind, &mut *argv.offset(optind as isize));
        if i == 0 as libc::c_int as libc::c_ulong {
            cfg.pipeline = script_verify_request((*t).L);
            cfg.dynamic = !script_is_static((*t).L);
            cfg.delay = script_has_delay((*t).L);
            if script_want_response((*t).L) {
                parser_settings
                    .on_header_field = Some(
                    header_field
                        as unsafe extern "C" fn(
                            *mut http_parser,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                );
                parser_settings
                    .on_header_value = Some(
                    header_value
                        as unsafe extern "C" fn(
                            *mut http_parser,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                );
                parser_settings
                    .on_body = Some(
                    response_body
                        as unsafe extern "C" fn(
                            *mut http_parser,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                );
            }
        }
        if ((*t).loop_0).is_null()
            || pthread_create(
                &mut (*t).thread,
                0 as *const pthread_attr_t,
                Some(
                    thread_main
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                t as *mut libc::c_void,
            ) != 0
        {
            let mut msg_0: *mut libc::c_char = strerror(*__errno_location());
            fprintf(
                stderr,
                b"unable to create thread %lu: %s\n\0" as *const u8
                    as *const libc::c_char,
                i,
                msg_0,
            );
            exit(2 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut sa: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_13 {
                sa_handler: Some(handler as unsafe extern "C" fn(libc::c_int) -> ()),
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0 as libc::c_int,
            sa_restorer: None,
        };
        init
    };
    sigfillset(&mut sa.sa_mask);
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    let mut time: *mut libc::c_char = format_time_s(f128::f128::new(cfg.duration));
    printf(b"Running %s test @ %s\n\0" as *const u8 as *const libc::c_char, time, url);
    printf(
        b"  %lu threads and %lu connections\n\0" as *const u8 as *const libc::c_char,
        cfg.threads,
        cfg.connections,
    );
    let mut start: uint64_t = time_us();
    let mut complete: uint64_t = 0 as libc::c_int as uint64_t;
    let mut bytes: uint64_t = 0 as libc::c_int as uint64_t;
    let mut errors: errors = {
        let mut init = errors {
            connect: 0 as libc::c_int as uint32_t,
            read: 0,
            write: 0,
            status: 0,
            timeout: 0,
        };
        init
    };
    sleep(cfg.duration as libc::c_uint);
    ::std::ptr::write_volatile(&mut stop as *mut sig_atomic_t, 1 as libc::c_int);
    let mut i_0: uint64_t = 0 as libc::c_int as uint64_t;
    while i_0 < cfg.threads {
        let mut t_0: *mut thread = &mut *threads.offset(i_0 as isize) as *mut thread;
        pthread_join((*t_0).thread, 0 as *mut *mut libc::c_void);
        complete = (complete as libc::c_ulong).wrapping_add((*t_0).complete) as uint64_t
            as uint64_t;
        bytes = (bytes as libc::c_ulong).wrapping_add((*t_0).bytes) as uint64_t
            as uint64_t;
        errors
            .connect = (errors.connect as libc::c_uint)
            .wrapping_add((*t_0).errors.connect) as uint32_t as uint32_t;
        errors
            .read = (errors.read as libc::c_uint).wrapping_add((*t_0).errors.read)
            as uint32_t as uint32_t;
        errors
            .write = (errors.write as libc::c_uint).wrapping_add((*t_0).errors.write)
            as uint32_t as uint32_t;
        errors
            .timeout = (errors.timeout as libc::c_uint)
            .wrapping_add((*t_0).errors.timeout) as uint32_t as uint32_t;
        errors
            .status = (errors.status as libc::c_uint).wrapping_add((*t_0).errors.status)
            as uint32_t as uint32_t;
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    let mut runtime_us: uint64_t = (time_us()).wrapping_sub(start);
    let mut runtime_s: f128::f128 = f128::f128::new(
        runtime_us as libc::c_double / 1000000.0f64,
    );
    let mut req_per_s: f128::f128 = f128::f128::new(complete) / runtime_s;
    let mut bytes_per_s: f128::f128 = f128::f128::new(bytes) / runtime_s;
    if complete.wrapping_div(cfg.connections) > 0 as libc::c_int as libc::c_ulong {
        let mut interval: int64_t = runtime_us
            .wrapping_div(complete.wrapping_div(cfg.connections)) as int64_t;
        stats_correct(statistics.latency, interval);
    }
    print_stats_header();
    print_stats(
        b"Latency\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        statistics.latency,
        Some(format_time_us as unsafe extern "C" fn(f128::f128) -> *mut libc::c_char),
    );
    print_stats(
        b"Req/Sec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        statistics.requests,
        Some(format_metric as unsafe extern "C" fn(f128::f128) -> *mut libc::c_char),
    );
    if cfg.latency {
        print_stats_latency(statistics.latency);
    }
    let mut runtime_msg: *mut libc::c_char = format_time_us(f128::f128::new(runtime_us));
    printf(
        b"  %lu requests in %s, %sB read\n\0" as *const u8 as *const libc::c_char,
        complete,
        runtime_msg,
        format_binary(f128::f128::new(bytes)),
    );
    if errors.connect != 0 || errors.read != 0 || errors.write != 0
        || errors.timeout != 0
    {
        printf(
            b"  Socket errors: connect %d, read %d, write %d, timeout %d\n\0"
                as *const u8 as *const libc::c_char,
            errors.connect,
            errors.read,
            errors.write,
            errors.timeout,
        );
    }
    if errors.status != 0 {
        printf(
            b"  Non-2xx or 3xx responses: %d\n\0" as *const u8 as *const libc::c_char,
            errors.status,
        );
    }
    printf(b"Requests/sec: %9.2Lf\n\0" as *const u8 as *const libc::c_char, req_per_s);
    printf(
        b"Transfer/sec: %10sB\n\0" as *const u8 as *const libc::c_char,
        format_binary(bytes_per_s),
    );
    if script_has_done(L) {
        script_summary(L, runtime_us, complete, bytes);
        script_errors(L, &mut errors);
        script_done(L, statistics.latency, statistics.requests);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn thread_main(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut thread: *mut thread = arg as *mut thread;
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0 as libc::c_int as size_t;
    if !cfg.dynamic {
        script_request((*thread).L, &mut request, &mut length);
    }
    (*thread)
        .cs = zcalloc(
        ((*thread).connections)
            .wrapping_mul(::std::mem::size_of::<connection>() as libc::c_ulong),
    ) as *mut connection;
    let mut c: *mut connection = (*thread).cs;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*thread).connections {
        (*c).thread = thread;
        (*c).ssl = if !(cfg.ctx).is_null() { SSL_new(cfg.ctx) } else { 0 as *mut SSL };
        (*c).request = request;
        (*c).length = length;
        (*c).delayed = cfg.delay;
        connect_socket(thread, c);
        i = i.wrapping_add(1);
        i;
        c = c.offset(1);
        c;
    }
    let mut loop_0: *mut aeEventLoop = (*thread).loop_0;
    aeCreateTimeEvent(
        loop_0,
        100 as libc::c_int as libc::c_longlong,
        Some(
            record_rate
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_longlong,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        thread as *mut libc::c_void,
        None,
    );
    (*thread).start = time_us();
    aeMain(loop_0);
    aeDeleteEventLoop(loop_0);
    zfree((*thread).cs as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn connect_socket(
    mut thread: *mut thread,
    mut c: *mut connection,
) -> libc::c_int {
    let mut current_block: u64;
    let mut addr: *mut addrinfo = (*thread).addr;
    let mut loop_0: *mut aeEventLoop = (*thread).loop_0;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    fd = socket((*addr).ai_family, (*addr).ai_socktype, (*addr).ai_protocol);
    flags = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int);
    if connect(fd, (*addr).ai_addr, (*addr).ai_addrlen) == -(1 as libc::c_int) {
        if *__errno_location() != 115 as libc::c_int {
            current_block = 2178676349992130760;
        } else {
            current_block = 7095457783677275021;
        }
    } else {
        current_block = 7095457783677275021;
    }
    match current_block {
        7095457783677275021 => {
            flags = 1 as libc::c_int;
            setsockopt(
                fd,
                IPPROTO_TCP as libc::c_int,
                1 as libc::c_int,
                &mut flags as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
            flags = 1 as libc::c_int | 2 as libc::c_int;
            if aeCreateFileEvent(
                loop_0,
                fd,
                flags,
                Some(
                    socket_connected
                        as unsafe extern "C" fn(
                            *mut aeEventLoop,
                            libc::c_int,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> (),
                ),
                c as *mut libc::c_void,
            ) == 0 as libc::c_int
            {
                (*c).parser.data = c as *mut libc::c_void;
                (*c).fd = fd;
                return fd;
            }
        }
        _ => {}
    }
    (*thread).errors.connect = ((*thread).errors.connect).wrapping_add(1);
    (*thread).errors.connect;
    close(fd);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn reconnect_socket(
    mut thread: *mut thread,
    mut c: *mut connection,
) -> libc::c_int {
    aeDeleteFileEvent((*thread).loop_0, (*c).fd, 2 as libc::c_int | 1 as libc::c_int);
    (sock.close).unwrap()(c);
    close((*c).fd);
    return connect_socket(thread, c);
}
unsafe extern "C" fn record_rate(
    mut loop_0: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut thread: *mut thread = data as *mut thread;
    if (*thread).requests > 0 as libc::c_int as libc::c_ulong {
        let mut elapsed_ms: uint64_t = (time_us())
            .wrapping_sub((*thread).start)
            .wrapping_div(1000 as libc::c_int as libc::c_ulong);
        let mut requests: uint64_t = ((*thread).requests as libc::c_double
            / elapsed_ms as libc::c_double * 1000 as libc::c_int as libc::c_double)
            as uint64_t;
        stats_record(statistics.requests, requests);
        (*thread).requests = 0 as libc::c_int as uint64_t;
        (*thread).start = time_us();
    }
    if stop != 0 {
        aeStop(loop_0);
    }
    return 100 as libc::c_int;
}
unsafe extern "C" fn delay_request(
    mut loop_0: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut c: *mut connection = data as *mut connection;
    (*c).delayed = 0 as libc::c_int != 0;
    aeCreateFileEvent(
        loop_0,
        (*c).fd,
        2 as libc::c_int,
        Some(
            socket_writeable
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn header_field(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut connection = (*parser).data as *mut connection;
    if (*c).state as libc::c_uint == VALUE as libc::c_int as libc::c_uint {
        let fresh0 = (*c).headers.cursor;
        (*c).headers.cursor = ((*c).headers.cursor).offset(1);
        *fresh0 = '\0' as i32 as libc::c_char;
        (*c).state = FIELD;
    }
    buffer_append(&mut (*c).headers, at, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn header_value(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut connection = (*parser).data as *mut connection;
    if (*c).state as libc::c_uint == FIELD as libc::c_int as libc::c_uint {
        let fresh1 = (*c).headers.cursor;
        (*c).headers.cursor = ((*c).headers.cursor).offset(1);
        *fresh1 = '\0' as i32 as libc::c_char;
        (*c).state = VALUE;
    }
    buffer_append(&mut (*c).headers, at, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn response_body(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut connection = (*parser).data as *mut connection;
    buffer_append(&mut (*c).body, at, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn response_complete(mut parser: *mut http_parser) -> libc::c_int {
    let mut c: *mut connection = (*parser).data as *mut connection;
    let mut thread: *mut thread = (*c).thread;
    let mut now: uint64_t = time_us();
    let mut status: libc::c_int = (*parser).status_code() as libc::c_int;
    (*thread).complete = ((*thread).complete).wrapping_add(1);
    (*thread).complete;
    (*thread).requests = ((*thread).requests).wrapping_add(1);
    (*thread).requests;
    if status > 399 as libc::c_int {
        (*thread).errors.status = ((*thread).errors.status).wrapping_add(1);
        (*thread).errors.status;
    }
    if !((*c).headers.buffer).is_null() {
        let fresh2 = (*c).headers.cursor;
        (*c).headers.cursor = ((*c).headers.cursor).offset(1);
        *fresh2 = '\0' as i32 as libc::c_char;
        script_response((*thread).L, status, &mut (*c).headers, &mut (*c).body);
        (*c).state = FIELD;
    }
    (*c).pending = ((*c).pending).wrapping_sub(1);
    if (*c).pending == 0 as libc::c_int as libc::c_ulong {
        if stats_record(statistics.latency, now.wrapping_sub((*c).start)) == 0 {
            (*thread).errors.timeout = ((*thread).errors.timeout).wrapping_add(1);
            (*thread).errors.timeout;
        }
        (*c).delayed = cfg.delay;
        aeCreateFileEvent(
            (*thread).loop_0,
            (*c).fd,
            2 as libc::c_int,
            Some(
                socket_writeable
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
    }
    if http_should_keep_alive(parser) == 0 {
        reconnect_socket(thread, c);
    } else {
        http_parser_init(parser, HTTP_RESPONSE);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn socket_connected(
    mut loop_0: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut c: *mut connection = data as *mut connection;
    match (sock.connect).unwrap()(c, cfg.host) as libc::c_uint {
        1 => {
            (*(*c).thread)
                .errors
                .connect = ((*(*c).thread).errors.connect).wrapping_add(1);
            (*(*c).thread).errors.connect;
            reconnect_socket((*c).thread, c);
            return;
        }
        2 => return,
        0 | _ => {
            http_parser_init(&mut (*c).parser, HTTP_RESPONSE);
            (*c).written = 0 as libc::c_int as size_t;
            aeCreateFileEvent(
                (*(*c).thread).loop_0,
                fd,
                1 as libc::c_int,
                Some(
                    socket_readable
                        as unsafe extern "C" fn(
                            *mut aeEventLoop,
                            libc::c_int,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> (),
                ),
                c as *mut libc::c_void,
            );
            aeCreateFileEvent(
                (*(*c).thread).loop_0,
                fd,
                2 as libc::c_int,
                Some(
                    socket_writeable
                        as unsafe extern "C" fn(
                            *mut aeEventLoop,
                            libc::c_int,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> (),
                ),
                c as *mut libc::c_void,
            );
            return;
        }
    };
}
unsafe extern "C" fn socket_writeable(
    mut loop_0: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut c: *mut connection = data as *mut connection;
    let mut thread: *mut thread = (*c).thread;
    if (*c).delayed {
        let mut delay: uint64_t = script_delay((*thread).L);
        aeDeleteFileEvent(loop_0, fd, 2 as libc::c_int);
        aeCreateTimeEvent(
            loop_0,
            delay as libc::c_longlong,
            Some(
                delay_request
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_longlong,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            c as *mut libc::c_void,
            None,
        );
        return;
    }
    if (*c).written == 0 {
        if cfg.dynamic {
            script_request((*thread).L, &mut (*c).request, &mut (*c).length);
        }
        (*c).start = time_us();
        (*c).pending = cfg.pipeline;
    }
    let mut buf: *mut libc::c_char = ((*c).request).offset((*c).written as isize);
    let mut len: size_t = ((*c).length).wrapping_sub((*c).written);
    let mut n: size_t = 0;
    match (sock.write).unwrap()(c, buf, len, &mut n) as libc::c_uint {
        1 => {
            (*thread).errors.write = ((*thread).errors.write).wrapping_add(1);
            (*thread).errors.write;
            reconnect_socket(thread, c);
            return;
        }
        2 => return,
        0 | _ => {
            (*c)
                .written = ((*c).written as libc::c_ulong).wrapping_add(n) as size_t
                as size_t;
            if (*c).written == (*c).length {
                (*c).written = 0 as libc::c_int as size_t;
                aeDeleteFileEvent(loop_0, fd, 2 as libc::c_int);
            }
            return;
        }
    };
}
unsafe extern "C" fn socket_readable(
    mut loop_0: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut current_block: u64;
    let mut c: *mut connection = data as *mut connection;
    let mut n: size_t = 0;
    loop {
        match (sock.read).unwrap()(c, &mut n) as libc::c_uint {
            1 => {
                current_block = 15924859052522923879;
                break;
            }
            2 => return,
            0 | _ => {
                if http_parser_execute(
                    &mut (*c).parser,
                    &mut parser_settings,
                    ((*c).buf).as_mut_ptr(),
                    n,
                ) != n
                {
                    current_block = 15924859052522923879;
                    break;
                }
                if n == 0 as libc::c_int as libc::c_ulong
                    && http_body_is_final(&mut (*c).parser) == 0
                {
                    current_block = 15924859052522923879;
                    break;
                }
                (*(*c).thread)
                    .bytes = ((*(*c).thread).bytes as libc::c_ulong).wrapping_add(n)
                    as uint64_t as uint64_t;
                if !(n == 8192 as libc::c_int as libc::c_ulong
                    && (sock.readable).unwrap()(c) > 0 as libc::c_int as libc::c_ulong)
                {
                    current_block = 6937071982253665452;
                    break;
                }
            }
        }
    }
    match current_block {
        6937071982253665452 => return,
        _ => {
            (*(*c).thread).errors.read = ((*(*c).thread).errors.read).wrapping_add(1);
            (*(*c).thread).errors.read;
            reconnect_socket((*c).thread, c);
            return;
        }
    };
}
unsafe extern "C" fn time_us() -> uint64_t {
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut t, 0 as *mut libc::c_void);
    return (t.tv_sec * 1000000 as libc::c_int as libc::c_long + t.tv_usec) as uint64_t;
}
unsafe extern "C" fn copy_url_part(
    mut url: *mut libc::c_char,
    mut parts: *mut http_parser_url,
    mut field: http_parser_url_fields,
) -> *mut libc::c_char {
    let mut part: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*parts).field_set as libc::c_int & (1 as libc::c_int) << field as libc::c_uint
        != 0
    {
        let mut off: uint16_t = (*parts).field_data[field as usize].off;
        let mut len: uint16_t = (*parts).field_data[field as usize].len;
        part = zcalloc(
            (len as libc::c_ulong)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ),
        ) as *mut libc::c_char;
        memcpy(
            part as *mut libc::c_void,
            &mut *url.offset(off as isize) as *mut libc::c_char as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    return part;
}
static mut longopts: [option; 10] = [
    {
        let mut init = option {
            name: b"connections\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"duration\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"threads\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"script\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
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
            name: b"latency\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
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
            val: 'v' as i32,
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
unsafe extern "C" fn parse_args(
    mut cfg_0: *mut config,
    mut url: *mut *mut libc::c_char,
    mut parts: *mut http_parser_url,
    mut headers: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut header: *mut *mut libc::c_char = headers;
    let mut c: libc::c_int = 0;
    memset(
        cfg_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<config>() as libc::c_ulong,
    );
    (*cfg_0).threads = 2 as libc::c_int as uint64_t;
    (*cfg_0).connections = 10 as libc::c_int as uint64_t;
    (*cfg_0).duration = 10 as libc::c_int as uint64_t;
    (*cfg_0).timeout = 2000 as libc::c_int as uint64_t;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"t:c:d:s:H:T:Lrv?\0" as *const u8 as *const libc::c_char,
            longopts.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            116 => {
                if scan_metric(optarg, &mut (*cfg_0).threads) != 0 {
                    return -(1 as libc::c_int);
                }
            }
            99 => {
                if scan_metric(optarg, &mut (*cfg_0).connections) != 0 {
                    return -(1 as libc::c_int);
                }
            }
            100 => {
                if scan_time(optarg, &mut (*cfg_0).duration) != 0 {
                    return -(1 as libc::c_int);
                }
            }
            115 => {
                (*cfg_0).script = optarg;
            }
            72 => {
                let fresh3 = header;
                header = header.offset(1);
                *fresh3 = optarg;
            }
            76 => {
                (*cfg_0).latency = 1 as libc::c_int != 0;
            }
            84 => {
                if scan_time(optarg, &mut (*cfg_0).timeout) != 0 {
                    return -(1 as libc::c_int);
                }
                (*cfg_0)
                    .timeout = ((*cfg_0).timeout as libc::c_ulong)
                    .wrapping_mul(1000 as libc::c_int as libc::c_ulong) as uint64_t
                    as uint64_t;
            }
            118 => {
                printf(
                    b"wrk %s [%s] \0" as *const u8 as *const libc::c_char,
                    VERSION,
                    aeGetApiName(),
                );
                printf(
                    b"Copyright (C) 2012 Will Glozer\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            104 | 63 | 58 | _ => return -(1 as libc::c_int),
        }
    }
    if optind == argc || (*cfg_0).threads == 0 || (*cfg_0).duration == 0 {
        return -(1 as libc::c_int);
    }
    if script_parse_url(*argv.offset(optind as isize), parts) == 0 {
        fprintf(
            stderr,
            b"invalid URL: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(optind as isize),
        );
        return -(1 as libc::c_int);
    }
    if (*cfg_0).connections == 0 || (*cfg_0).connections < (*cfg_0).threads {
        fprintf(
            stderr,
            b"number of connections must be >= threads\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *url = *argv.offset(optind as isize);
    *header = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_stats_header() {
    printf(
        b"  Thread Stats%6s%11s%8s%12s\n\0" as *const u8 as *const libc::c_char,
        b"Avg\0" as *const u8 as *const libc::c_char,
        b"Stdev\0" as *const u8 as *const libc::c_char,
        b"Max\0" as *const u8 as *const libc::c_char,
        b"+/- Stdev\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn print_units(
    mut n: f128::f128,
    mut fmt: Option::<unsafe extern "C" fn(f128::f128) -> *mut libc::c_char>,
    mut width: libc::c_int,
) {
    let mut msg: *mut libc::c_char = fmt.unwrap()(n);
    let mut len: libc::c_int = strlen(msg) as libc::c_int;
    let mut pad: libc::c_int = 2 as libc::c_int;
    if *(*__ctype_b_loc())
        .offset(*msg.offset((len - 1 as libc::c_int) as isize) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        pad -= 1;
        pad;
    }
    if *(*__ctype_b_loc())
        .offset(*msg.offset((len - 2 as libc::c_int) as isize) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        pad -= 1;
        pad;
    }
    width -= pad;
    printf(
        b"%*.*s%.*s\0" as *const u8 as *const libc::c_char,
        width,
        width,
        msg,
        pad,
        b"  \0" as *const u8 as *const libc::c_char,
    );
    free(msg as *mut libc::c_void);
}
unsafe extern "C" fn print_stats(
    mut name: *mut libc::c_char,
    mut stats: *mut stats,
    mut fmt: Option::<unsafe extern "C" fn(f128::f128) -> *mut libc::c_char>,
) {
    let mut max: uint64_t = (*stats).max;
    let mut mean: f128::f128 = stats_mean(stats);
    let mut stdev: f128::f128 = stats_stdev(stats, mean);
    printf(b"    %-10s\0" as *const u8 as *const libc::c_char, name);
    print_units(mean, fmt, 8 as libc::c_int);
    print_units(stdev, fmt, 10 as libc::c_int);
    print_units(f128::f128::new(max), fmt, 9 as libc::c_int);
    printf(
        b"%8.2Lf%%\n\0" as *const u8 as *const libc::c_char,
        stats_within_stdev(stats, mean, stdev, 1 as libc::c_int as uint64_t),
    );
}
unsafe extern "C" fn print_stats_latency(mut stats: *mut stats) {
    let mut percentiles: [f128::f128; 4] = [
        f128::f128::new(50.0f64),
        f128::f128::new(75.0f64),
        f128::f128::new(90.0f64),
        f128::f128::new(99.0f64),
    ];
    printf(b"  Latency Distribution\n\0" as *const u8 as *const libc::c_char);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[f128::f128; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<f128::f128>() as libc::c_ulong)
    {
        let mut p: f128::f128 = percentiles[i as usize];
        let mut n: uint64_t = stats_percentile(stats, p);
        printf(b"%7.0Lf%%\0" as *const u8 as *const libc::c_char, p);
        print_units(
            f128::f128::new(n),
            Some(
                format_time_us as unsafe extern "C" fn(f128::f128) -> *mut libc::c_char,
            ),
            10 as libc::c_int,
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
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

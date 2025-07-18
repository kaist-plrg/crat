use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn clock() -> clock_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
    fn _ssr(a: K, b: K, c: K) -> K;
    fn getline_(s: *mut S, n: *mut I, f: *mut FILE) -> I;
    fn appender(s: *mut S, n: *mut I, t: S, k: I) -> I;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn cd(a: K) -> K;
    fn finally();
    static mut PG: I;
    static mut fError: I;
    static mut fer: I;
    static mut fer1: I;
    static mut fnci: I;
    fn boilerplate();
    fn show(a: K) -> K;
    fn printAtDepth(u: V, a: K, d: I, x: I, vdep: I, b: I);
    static mut IFP: [S; 3];
    static mut IFS: [S; 3];
    static mut offsetSSR: V;
    static mut offsetWhat: V;
    static mut offsetAt: V;
    static mut offsetDot: V;
    static mut offsetColon: V;
    fn _3m(x: K) -> K;
    fn _0m(a: K) -> K;
    fn colon_dyadic(a: K, b: K) -> K;
    fn dot(a: K, b: K) -> K;
    fn join(a: K, b: K) -> K;
    fn what(x: K, y: K) -> K;
    fn at(x: K, y: K) -> K;
    fn flip(a: K) -> K;
    fn eachpair() -> K;
    fn eachleft() -> K;
    fn eachright() -> K;
    fn each() -> K;
    fn scan() -> K;
    fn over() -> K;
    fn end() -> K;
    fn wd(s: S, n: libc::c_int) -> K;
    fn ex(a: K) -> K;
    fn X(s: S) -> K;
    fn DT_OFFSET(v: V) -> L;
    static mut LS: S;
    static mut NIL: K;
    static mut IPC_PORT: S;
    static mut HTTP_PORT: S;
    static mut d_: S;
    static mut SEED: I;
    static mut KTREE: K;
    static mut SYMBOLS: N;
    fn kerr(s: cS) -> K;
    static mut DT_SIZE: L;
    static mut DT_END_OFFSET: L;
    static mut DT_ADVERB_OFFSET: L;
    static mut DT_VERB_OFFSET: L;
    static mut DT_SPECIAL_VERB_OFFSET: L;
    static mut offsetOver: L;
    static mut offsetScan: L;
    static mut offsetEach: L;
    static mut offsetEachright: L;
    static mut offsetEachleft: L;
    static mut offsetEachpair: L;
    fn TABLE_END() -> K;
    fn init_genrand64(seed: libc::c_ulonglong);
    static mut cdp: [C; 0];
    fn _dot_t() -> K;
    fn newE(s: S, k: K) -> K;
    fn newEntry(s: S) -> K;
    fn Kd() -> K;
    static mut KONA_WHO: K;
    static mut KONA_PORT: K;
    static mut KONA_CLIENT: K;
    fn _n() -> K;
    static mut mUsed: F;
    static mut mMax: F;
    static mut mAlloc: F;
    static mut mMap: F;
    static mut fWksp: I;
    static mut fLoad: I;
    static mut lineA: S;
    static mut lineB: S;
    static mut errmsg: [C; 256];
    fn test() -> I;
    static mut fnc: S;
    static mut fncp: [V; 128];
    static mut fom: I;
    static mut fam: I;
    static mut fll: I;
    static mut cls: K;
    fn sp(k: S) -> S;
    fn Kn() -> K;
    fn newN() -> N;
    static mut offsetJoin: V;
    static mut offset3m: V;
    fn load(s: S) -> K;
    fn wipe_tape(i: I) -> I;
    fn kap(a: *mut K, v: V) -> K;
    fn pdafree(p: PDA);
    fn newK(t: I, n: I) -> K;
    fn parsedepth(p: PDA) -> I;
    fn complete(a: S, n: I, q: *mut PDA, marks: *mut I) -> I;
    fn read_tape(i: I, j: I, type_0: I) -> K;
    fn denameS(dir_string: S, t: S, create: I) -> *mut K;
    fn ninit() -> I;
    static mut CP: [M0; 1025];
    fn _h() -> K;
    fn _host(x: K) -> K;
    static mut HOST_IFACE: S;
    fn spn(s: S, n: I) -> S;
    fn Ks(x: S) -> K;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type clock_t = __clock_t;
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
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_0 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_0 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_0 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_0 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_0 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_0 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_0 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_0 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_0 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_0 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_0 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_0 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_0 = 236;
pub const _SC_IPV6: C2RustUnnamed_0 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_0 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_0 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_0 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_0 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_0 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_0 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_0 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_0 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_0 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_0 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_0 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_0 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_0 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_0 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_0 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_0 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_0 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_0 = 182;
pub const _SC_TRACE: C2RustUnnamed_0 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_0 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_0 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_0 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_0 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_0 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_0 = 175;
pub const _SC_STREAMS: C2RustUnnamed_0 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_0 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_0 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_0 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_0 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_0 = 169;
pub const _SC_2_PBS: C2RustUnnamed_0 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_0 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_0 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_0 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_0 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_0 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_0 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_0 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_0 = 160;
pub const _SC_SPAWN: C2RustUnnamed_0 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_0 = 158;
pub const _SC_SHELL: C2RustUnnamed_0 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_0 = 156;
pub const _SC_REGEXP: C2RustUnnamed_0 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_0 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_0 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_0 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_0 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_0 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_0 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_0 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_0 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_0 = 146;
pub const _SC_PIPE: C2RustUnnamed_0 = 145;
pub const _SC_FIFO: C2RustUnnamed_0 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_0 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_0 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_0 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_0 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_0 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_0 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_0 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_0 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_0 = 135;
pub const _SC_BASE: C2RustUnnamed_0 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_0 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_0 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_0 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_0 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_0 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_0 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_0 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_0 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_0 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_0 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_0 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_0 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_0 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_0 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_0 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_0 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_0 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_0 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_0 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_0 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_0 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_0 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_0 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_0 = 110;
pub const _SC_NZERO: C2RustUnnamed_0 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_0 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_0 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_0 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_0 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_0 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_0 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_0 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_0 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_0 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_0 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_0 = 98;
pub const _SC_2_UPE: C2RustUnnamed_0 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_0 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_0 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_0 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_0 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_0 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_0 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_0 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_0 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_0 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_0 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_0 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_0 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_0 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_0 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_0 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_0 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_0 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_0 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_0 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_0 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_0 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_0 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_0 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_0 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_0 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_0 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_0 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_0 = 68;
pub const _SC_THREADS: C2RustUnnamed_0 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_0 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_0 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_0 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_0 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_0 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_0 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_0 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_0 = 60;
pub const _SC_SELECT: C2RustUnnamed_0 = 59;
pub const _SC_POLL: C2RustUnnamed_0 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_0 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_0 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_0 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_0 = 54;
pub const _SC_PII: C2RustUnnamed_0 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_0 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_0 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_0 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_0 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_0 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_0 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_0 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_0 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_0 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_0 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_0 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_0 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_0 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_0 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_0 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_0 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_0 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_0 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_0 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_0 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_0 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_0 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_0 = 30;
pub const _SC_VERSION: C2RustUnnamed_0 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_0 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_0 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_0 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_0 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_0 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_0 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_0 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_0 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_0 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_0 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_0 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_0 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_0 = 16;
pub const _SC_FSYNC: C2RustUnnamed_0 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_0 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_0 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_0 = 12;
pub const _SC_TIMERS: C2RustUnnamed_0 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_0 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_0 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_0 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_0 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_0 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_0 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_0 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_0 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_0 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_1 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_1 = 0;
pub type L = libc::c_longlong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m1 {
    pub a: libc::c_char,
    pub b: libc::c_char,
    pub c: [libc::c_char; 5],
    pub d: libc::c_char,
    pub n: I,
}
pub type M1 = m1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m0 {
    pub m1: M1,
    pub r: I,
    pub k: K,
    pub a: I,
}
pub type M0 = m0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub k: V,
    pub b: I,
    pub c: [*mut node; 2],
}
pub type Node = node;
pub type N = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pda {
    pub i: I,
    pub s: I,
    pub n: I,
    pub c: S,
}
pub type Pda = pda;
pub type PDA = *mut Pda;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_11,
    pub _timer: C2RustUnnamed_10,
    pub _rt: C2RustUnnamed_9,
    pub _sigchld: C2RustUnnamed_8,
    pub _sigfault: C2RustUnnamed_5,
    pub _sigpoll: C2RustUnnamed_4,
    pub _sigsys: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _addr_bnd: C2RustUnnamed_7,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_12,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub static mut KONA_GSET: K = 0 as *const k0 as K;
pub static mut KONA_IDX: K = 0 as *const k0 as K;
pub static mut interrupted: sig_atomic_t = 0 as libc::c_int;
pub unsafe extern "C" fn oerr() -> I {
    return printf(
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        errmsg.as_mut_ptr(),
        b"error\0" as *const u8 as *const libc::c_char,
    ) as I;
}
pub static mut feci: I = 0 as libc::c_int as I;
pub static mut scrLim: I = 0 as libc::c_int as I;
pub static mut fCheck: I = 0 as libc::c_int as I;
static mut ofCheck: I = 0 as libc::c_int as I;
pub static mut fCmplt: I = 0 as libc::c_int as I;
pub static mut fbr: I = 0 as libc::c_int as I;
static mut flc: I = 0 as libc::c_int as I;
static mut lineC: [C; 100] = [0; 100];
static mut ofnc: [C; 2] = unsafe {
    *::std::mem::transmute::<&[u8; 2], &mut [C; 2]>(b" \0")
};
static mut ocr: I = 0 as libc::c_int as I;
pub unsafe extern "C" fn prompt(mut n: I) -> I {
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        printf(b">\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    printf(b"  \0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn wds(mut a: *mut K, mut f: *mut FILE) -> I {
    return wds_(a, f, 0 as libc::c_int as I);
}
pub unsafe extern "C" fn wds_(mut a: *mut K, mut f: *mut FILE, mut l: I) -> I {
    let mut current_block: u64;
    let mut s: S = 0 as S;
    let mut t: S = 0 as S;
    let mut b: I = 0 as libc::c_int as I;
    let mut c: I = 0 as libc::c_int as I;
    let mut m: I = 0 as libc::c_int as I;
    let mut n: I = 0 as libc::c_int as I;
    let mut v: I = 0 as libc::c_int as I;
    let mut z: K = 0 as K;
    let mut p: PDA = 0 as PDA;
    let mut o: I = (isatty(fileno(stdin)) != 0 && f == stdin) as libc::c_int as I;
    c = getline_(&mut s, &mut n, f);
    if !(-(1 as libc::c_int) as libc::c_longlong == c) {
        appender(&mut t, &mut m, s, n);
        loop {
            v = complete(t, m, &mut p, 0 as *mut I);
            if !(1 as libc::c_int as libc::c_longlong == v) {
                current_block = 13513818773234778473;
                break;
            }
            b = parsedepth(p);
            if o != 0 {
                prompt(b + l);
            }
            c = getline_(&mut s, &mut n, f);
            if -(1 as libc::c_int) as libc::c_longlong == c {
                current_block = 8359905410816565417;
                break;
            }
            appender(&mut t, &mut m, s, n);
        }
        match current_block {
            8359905410816565417 => {}
            _ => {
                match v {
                    2 => {
                        show(kerr(b"unmatched\0" as *const u8 as *const libc::c_char));
                    }
                    3 => {
                        show(kerr(b"nest\0" as *const u8 as *const libc::c_char));
                    }
                    _ => {
                        z = newK(
                            -(3 as libc::c_int) as I,
                            m - 1 as libc::c_int as libc::c_longlong,
                        );
                        strncpy(
                            ((*z).k).as_mut_ptr() as *mut C,
                            t as *const libc::c_char,
                            (m - 1 as libc::c_int as libc::c_longlong) as libc::c_ulong,
                        );
                    }
                }
            }
        }
    }
    free(s as *mut libc::c_void);
    free(t as *mut libc::c_void);
    if !p.is_null() {
        pdafree(p);
    }
    if (v != 0 || c == -(1 as libc::c_int) as libc::c_longlong) && !z.is_null() {
        cd(z);
        *a = 0 as K;
    } else {
        *a = z;
    }
    return if v != 0 { -v } else { c };
}
pub static mut KONA_ARGS: K = 0 as *const k0 as *mut k0;
unsafe extern "C" fn multihomeini(mut x: *mut S) {
    static mut port: [C; 65] = [0; 65];
    let mut s: S = *x;
    if s.is_null() {
        return;
    }
    let mut p: S = strchr(s as *const libc::c_char, ':' as i32);
    if p.is_null() {
        return;
    }
    strcpy(
        port.as_mut_ptr(),
        p.offset(1 as libc::c_int as isize) as *const libc::c_char,
    );
    HOST_IFACE = spn(s, p.offset_from(s) as libc::c_long as I);
    *x = port.as_mut_ptr();
    let mut h: K = Ks(HOST_IFACE);
    cd(KONA_CLIENT);
    KONA_CLIENT = _host(h);
    cd(h);
}
pub unsafe extern "C" fn args(mut n: libc::c_int, mut v: *mut S) -> I {
    let mut a: K = 0 as *mut k0;
    let mut k: K = 0 as *mut k0;
    let mut c: I = 0;
    let mut len: I = 0;
    let mut b: I = 1 as libc::c_int as I;
    KONA_ARGS = newK(0 as libc::c_int as I, n as I);
    if KONA_ARGS.is_null() {
        return 0 as libc::c_int as I;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n as I;
    while i < _i {
        len = strlen(*v.offset(i as isize) as *const libc::c_char) as I;
        a = newK(-(3 as libc::c_int) as I, len);
        if a.is_null() {
            cd(KONA_ARGS);
            return 0 as libc::c_int as I;
        }
        strncpy(
            ((*a).k).as_mut_ptr() as *mut C,
            *v.offset(i as isize) as *const libc::c_char,
            len as libc::c_ulong,
        );
        let ref mut fresh0 = *((*KONA_ARGS).k).as_mut_ptr().offset(i as isize);
        *fresh0 = a;
        i += 1;
        i;
    }
    loop {
        c = getopt(
            n,
            v as *const *mut libc::c_char,
            b":b:h:i:e:x:\0" as *const u8 as *const libc::c_char,
        ) as I;
        if !(-(1 as libc::c_int) as libc::c_longlong != c) {
            break;
        }
        match c {
            104 => {
                if !IPC_PORT.is_null() {
                    printf(
                        b"-i accepted, cannot also have -h\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    HTTP_PORT = optarg;
                }
            }
            105 => {
                if !HTTP_PORT.is_null() {
                    printf(
                        b"-h accepted, cannot also have -i\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    IPC_PORT = optarg;
                    *(((*KONA_PORT).k).as_mut_ptr()
                        as *mut I) = atol(IPC_PORT as *const libc::c_char) as I;
                }
            }
            98 => {
                b = 0 as libc::c_int as I;
            }
            101 => {
                cd(X(optarg));
                exit(0 as libc::c_int);
            }
            120 => {
                k = X(optarg);
                printAtDepth(
                    0 as V,
                    k,
                    0 as libc::c_int as I,
                    0 as libc::c_int as I,
                    0 as libc::c_int as I,
                    0 as libc::c_int as I,
                );
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                cd(k);
                exit(0 as libc::c_int);
            }
            58 | 63 => {
                printf(b"%c\nabort\0" as *const u8 as *const libc::c_char, optopt);
                exit(0 as libc::c_int);
            }
            _ => {}
        }
    }
    if b != 0 {
        boilerplate();
    }
    multihomeini(if !IPC_PORT.is_null() { &mut IPC_PORT } else { &mut HTTP_PORT });
    let mut h: S = getenv(b"KINIT\0" as *const u8 as *const libc::c_char);
    if !h.is_null() {
        load(h);
    }
    while optind < n {
        let fresh1 = optind;
        optind = optind + 1;
        load(*v.offset(fresh1 as isize));
    }
    prompt(0 as libc::c_int as I);
    return 0 as libc::c_int as I;
}
pub static mut KFIXED: K = 0 as *const k0 as *mut k0;
pub static mut execute_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub static mut khome: [C; 4097] = [0; 4097];
unsafe extern "C" fn khinit() {
    let mut n: I = 0;
    let mut h: S = 0 as *mut C;
    khome[0 as libc::c_int as usize] = 0 as libc::c_int as C;
    h = getenv(b"KHOME\0" as *const u8 as *const libc::c_char);
    if !h.is_null() {
        n = strlen(h as *const libc::c_char) as I;
        if n + 1 as libc::c_int as libc::c_longlong
            > 4096 as libc::c_int as libc::c_longlong
        {
            return;
        }
        strcpy(khome.as_mut_ptr(), h as *const libc::c_char);
        strcpy(
            khome.as_mut_ptr().offset(n as isize),
            b"/\0" as *const u8 as *const libc::c_char,
        );
    } else {
        h = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
        if !h.is_null() {
            n = strlen(h as *const libc::c_char) as I;
            if n + 3 as libc::c_int as libc::c_longlong
                > 4096 as libc::c_int as libc::c_longlong
            {
                return;
            }
            strcpy(khome.as_mut_ptr(), h as *const libc::c_char);
            strcpy(
                khome.as_mut_ptr().offset(n as isize),
                b"/k/\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
pub unsafe extern "C" fn kinit() -> I {
    atexit(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(finally),
            ),
        ),
    );
    PG = sysconf(_SC_PAGESIZE as libc::c_int) as I;
    if PG & PG - 1 as libc::c_int as libc::c_longlong != 0 {
        fprintf(
            stderr,
            b"%s:%u: %s\n\0" as *const u8 as *const libc::c_char,
            b"src/kc.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            b"Pagesize not power of 2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    ninit();
    let mut mta: pthread_mutexattr_t = pthread_mutexattr_t {
        __size: [0; 4],
    };
    pthread_mutexattr_init(&mut mta);
    pthread_mutexattr_settype(&mut mta, PTHREAD_MUTEX_RECURSIVE_NP as libc::c_int);
    pthread_mutex_init(&mut execute_mutex, &mut mta);
    pthread_mutexattr_destroy(&mut mta);
    DT_SIZE = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(TABLE_END),
            ),
        ),
    );
    DT_END_OFFSET = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(end),
            ),
        ),
    );
    DT_ADVERB_OFFSET = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(over),
            ),
        ),
    );
    DT_VERB_OFFSET = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K) -> K>,
            V,
        >(Some(flip as unsafe extern "C" fn(K) -> K)),
    );
    DT_SPECIAL_VERB_OFFSET = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K) -> K>,
            V,
        >(Some(_0m as unsafe extern "C" fn(K) -> K)),
    );
    offsetOver = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(over),
            ),
        ),
    );
    offsetScan = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(scan),
            ),
        ),
    );
    offsetEach = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(each),
            ),
        ),
    );
    offsetEachright = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(eachright),
            ),
        ),
    );
    offsetEachleft = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(eachleft),
            ),
        ),
    );
    offsetEachpair = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(eachpair),
            ),
        ),
    );
    offsetWhat = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K, K) -> K>,
            V,
        >(Some(what as unsafe extern "C" fn(K, K) -> K)),
    ) as V;
    offsetAt = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K, K) -> K>,
            V,
        >(Some(at as unsafe extern "C" fn(K, K) -> K)),
    ) as V;
    offsetDot = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K, K) -> K>,
            V,
        >(Some(dot as unsafe extern "C" fn(K, K) -> K)),
    ) as V;
    offsetColon = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K, K) -> K>,
            V,
        >(Some(colon_dyadic as unsafe extern "C" fn(K, K) -> K)),
    ) as V;
    offsetJoin = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K, K) -> K>,
            V,
        >(Some(join as unsafe extern "C" fn(K, K) -> K)),
    ) as V;
    offsetSSR = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K, K, K) -> K>,
            V,
        >(Some(_ssr as unsafe extern "C" fn(K, K, K) -> K)),
    ) as V;
    offset3m = DT_OFFSET(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(K) -> K>,
            V,
        >(Some(_3m as unsafe extern "C" fn(K) -> K)),
    ) as V;
    kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
    SYMBOLS = newN();
    seedPRNG(-(271828 as libc::c_int) as I);
    NIL = Kn();
    KFIXED = newK(0 as libc::c_int as I, 0 as libc::c_int as I);
    kap(&mut KFIXED, &mut NIL as *mut K as V);
    cd(NIL);
    d_ = sp(b".k\0" as *const u8 as *const libc::c_char as S);
    LS = sp(b"\0" as *const u8 as *const libc::c_char as S);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = 3 as libc::c_int as I;
    while i < _i {
        IFP[i as usize] = sp(IFS[i as usize]);
        i += 1;
        i;
    }
    test();
    KTREE = Kd();
    let mut x: K = newEntry(sp(b"k\0" as *const u8 as *const libc::c_char as S));
    kap(&mut KTREE, &mut x as *mut K as V);
    cd(x);
    x = newE(sp(b"t\0" as *const u8 as *const libc::c_char as S), _dot_t());
    kap(&mut KTREE, &mut x as *mut K as V);
    cd(x);
    KONA_WHO = newK(1 as libc::c_int as I, 1 as libc::c_int as I);
    *(((*KONA_WHO).k).as_mut_ptr() as *mut I) = 0 as libc::c_int as I;
    KONA_PORT = newK(1 as libc::c_int as I, 1 as libc::c_int as I);
    *(((*KONA_PORT).k).as_mut_ptr() as *mut I) = 0 as libc::c_int as I;
    KONA_GSET = _n();
    KONA_IDX = _n();
    KONA_CLIENT = _host(_h());
    khinit();
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn randomBits() -> I {
    let mut s: I = 0;
    let mut f: I = open(
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as I;
    let mut r: I = read(
        f as libc::c_int,
        &mut s as *mut I as *mut libc::c_void,
        ::std::mem::size_of::<I>() as libc::c_ulong,
    ) as I;
    if r == 0 {
        show(kerr(b"read\0" as *const u8 as *const libc::c_char));
    }
    r = close(f as libc::c_int) as I;
    if r != 0 {
        show(kerr(b"file\0" as *const u8 as *const libc::c_char));
    }
    return s;
}
pub unsafe extern "C" fn seedPRNG(mut s: I) {
    SEED = if s != 0 { s } else { randomBits() };
    init_genrand64(SEED as libc::c_ulonglong);
}
unsafe extern "C" fn nodeCount_(mut n: N) -> I {
    let mut l: I = 0 as libc::c_int as I;
    let mut r: I = 0 as libc::c_int as I;
    if !((*n).k).is_null() {
        if strlen((*n).k as S as *const libc::c_char) != 0 {
            printf(b"%s \0" as *const u8 as *const libc::c_char, (*n).k as S);
        } else {
            printf(b"(nil) \0" as *const u8 as *const libc::c_char);
        }
    }
    if !((*n).c[0 as libc::c_int as usize]).is_null() {
        l += nodeCount_((*n).c[0 as libc::c_int as usize]);
    }
    if !((*n).c[1 as libc::c_int as usize]).is_null() {
        r += nodeCount_((*n).c[1 as libc::c_int as usize]);
    }
    return 1 as libc::c_int as libc::c_longlong + l + r;
}
unsafe extern "C" fn nodeCount(mut n: N) -> I {
    return nodeCount_(n) - 1 as libc::c_int as libc::c_longlong;
}
pub unsafe extern "C" fn check() -> I {
    let mut ofCheck_0: I = fCheck;
    if !fnc.is_null() {
        *ofnc.as_mut_ptr() = *fnc;
    }
    kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
    fCheck += 1;
    prompt(fCheck);
    let mut a: S = 0 as S;
    let mut n: I = 0 as libc::c_int as I;
    let mut q: PDA = 0 as PDA;
    loop {
        line(stdin, &mut a, &mut n, &mut q);
        if fCheck == ofCheck_0 {
            return 0 as libc::c_int as I;
        }
    };
}
static mut fln: I = 0 as libc::c_int as I;
pub unsafe extern "C" fn lines(mut f: *mut FILE) -> I {
    let mut a: S = 0 as S;
    let mut n: I = 0 as libc::c_int as I;
    let mut p: PDA = 0 as PDA;
    fln = 1 as libc::c_int as I;
    while -(1 as libc::c_int) as libc::c_longlong != line(f, &mut a, &mut n, &mut p) {
        fln = 0 as libc::c_int as I;
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn line(
    mut f: *mut FILE,
    mut a: *mut S,
    mut n: *mut I,
    mut p: *mut PDA,
) -> I {
    let mut v: I = 0;
    let mut ptr: S = 0 as *mut C;
    let mut current_block: u64;
    let mut s: S = 0 as S;
    let mut b: I = 0 as libc::c_int as I;
    let mut c: I = 0 as libc::c_int as I;
    let mut m: I = 0 as libc::c_int as I;
    let mut o: I = 1 as libc::c_int as I;
    let mut q: I = 1 as libc::c_int as I;
    let mut k: K = 0 as *mut k0;
    let mut d: F = 0.;
    feci = 0 as libc::c_int as I;
    fer = feci;
    fbr = fer;
    fam = 1 as libc::c_int as I;
    c = getline_(&mut s, &mut m, f);
    if -(1 as libc::c_int) as libc::c_longlong == c {
        current_block = 2906900040509793395;
    } else {
        if fCheck != 0
            && 1 as libc::c_int as libc::c_ulong == strlen(s as *const libc::c_char)
            && *s.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
        {
            loop {
                if !(1 as libc::c_int as libc::c_ulong
                    == strlen(s as *const libc::c_char)
                    && *s.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\n' as i32)
                {
                    current_block = 14523784380283086299;
                    break;
                }
                prompt(b + fCheck);
                c = getline_(&mut s, &mut m, f);
                if -(1 as libc::c_int) as libc::c_longlong == c {
                    current_block = 2906900040509793395;
                    break;
                }
            }
        } else {
            current_block = 14523784380283086299;
        }
        match current_block {
            2906900040509793395 => {}
            _ => {
                if fln != 0
                    && (*s.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
                        && *s.offset(1 as libc::c_int as isize) as libc::c_int
                            == '!' as i32)
                {
                    current_block = 2906900040509793395;
                } else {
                    if fCheck != 0
                        && *s.offset(0 as libc::c_int as isize) as libc::c_int
                            == ':' as i32
                        && (strstr(
                            s as *const libc::c_char,
                            b"`\"\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null() && (!lineA.is_null() || flc != 0)
                    {
                        let mut i: I = 0;
                        let mut j: I = 0;
                        let mut jj: I = 0;
                        if !(*a).is_null() {
                            j = 0 as libc::c_int as I;
                            while j < 10 as libc::c_int as libc::c_longlong {
                                if *cdp.as_mut_ptr().offset(j as isize) as libc::c_int
                                    == *ofnc.as_mut_ptr() as libc::c_int
                                {
                                    break;
                                }
                                j += 1;
                                j;
                            }
                            i = 0 as libc::c_int as I;
                            while (i as libc::c_ulonglong)
                                < strlen(lineC.as_mut_ptr()) as libc::c_ulonglong
                            {
                                if lineC[i as usize] as libc::c_int
                                    == *cdp
                                        .as_mut_ptr()
                                        .offset((j + 1 as libc::c_int as libc::c_longlong) as isize)
                                        as libc::c_int
                                {
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            *n = 0 as libc::c_int as I;
                            appender(
                                a,
                                n,
                                lineC.as_mut_ptr(),
                                i + 1 as libc::c_int as libc::c_longlong,
                            );
                        } else {
                            let mut cfnc: I = 0 as libc::c_int as I;
                            i = 0 as libc::c_int as I;
                            while (i as libc::c_ulonglong)
                                < strlen(lineC.as_mut_ptr()) as libc::c_ulonglong
                            {
                                if lineC[i as usize] as libc::c_int == *fnc as libc::c_int {
                                    cfnc += 1;
                                    cfnc;
                                }
                                i += 1;
                                i;
                            }
                            let mut cprm: I = 0 as libc::c_int as I;
                            i = 0 as libc::c_int as I;
                            while *cdp.as_mut_ptr().offset(i as isize) as libc::c_int
                                != 'a' as i32
                            {
                                cprm += 1;
                                cprm;
                                i += 1;
                                i;
                            }
                            if cfnc != ocr
                                && !(cfnc == ocr
                                    && *cdp
                                        .as_mut_ptr()
                                        .offset(
                                            (cprm - 1 as libc::c_int as libc::c_longlong) as isize,
                                        ) as libc::c_int == *fnc as libc::c_int)
                            {
                                let mut cfl: I = 0 as libc::c_int as I;
                                let mut cfc: I = 0 as libc::c_int as I;
                                i = (strlen(lineC.as_mut_ptr()))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as I;
                                while i > 0 as libc::c_int as libc::c_longlong {
                                    if lineC[i as usize] as libc::c_int == *fnc as libc::c_int {
                                        cfl += 1;
                                        cfl;
                                    }
                                    if cfl == ocr {
                                        break;
                                    }
                                    i -= 1;
                                    i;
                                }
                                j = 0 as libc::c_int as I;
                                while j < 10 as libc::c_int as libc::c_longlong {
                                    if *cdp.as_mut_ptr().offset(j as isize) as libc::c_int
                                        == *fnc as libc::c_int
                                    {
                                        cfc += 1;
                                        cfc;
                                    }
                                    if cfc == ocr {
                                        break;
                                    }
                                    j += 1;
                                    j;
                                }
                                jj = i - 1 as libc::c_int as libc::c_longlong;
                                while jj > 0 as libc::c_int as libc::c_longlong {
                                    if lineC[jj as usize] as libc::c_int
                                        == *cdp
                                            .as_mut_ptr()
                                            .offset((j + 1 as libc::c_int as libc::c_longlong) as isize)
                                            as libc::c_int
                                    {
                                        break;
                                    }
                                    jj -= 1;
                                    jj;
                                }
                                appender(
                                    a,
                                    n,
                                    lineC.as_mut_ptr(),
                                    jj + 1 as libc::c_int as libc::c_longlong,
                                );
                            }
                        }
                        appender(
                            a,
                            n,
                            s.offset(1 as libc::c_int as isize),
                            (strlen(s as *const libc::c_char))
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as I,
                        );
                        d = clock() as F;
                        k = ex(wd(*a, *n as libc::c_int));
                        d = (clock() as libc::c_double - d)
                            / 1000000 as libc::c_int as __clock_t as libc::c_double;
                        fCheck -= 1;
                        fCheck;
                        q = 0 as libc::c_int as I;
                        current_block = 970667356130215337;
                    } else {
                        if *s.offset(0 as libc::c_int as isize) as libc::c_int
                            == '\\' as i32
                            && *s.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\n' as i32
                        {
                            if fCheck == 0 && fLoad != 0 {
                                c = -(1 as libc::c_int) as I;
                                current_block = 2906900040509793395;
                            } else {
                                if fCheck != 0 {
                                    fCheck -= 1;
                                    fCheck;
                                    return 0 as libc::c_int as I;
                                }
                                if !(*a).is_null() {
                                    current_block = 2906900040509793395;
                                } else {
                                    current_block = 6528285054092551010;
                                }
                            }
                        } else {
                            current_block = 6528285054092551010;
                        }
                        match current_block {
                            2906900040509793395 => {}
                            _ => {
                                if *s.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '\\' as i32
                                    && *s.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\\' as i32
                                    && 2 as libc::c_int as libc::c_ulong
                                        == strlen(s as *const libc::c_char)
                                {
                                    exit(0 as libc::c_int);
                                }
                                if flc != 0 {
                                    *n = 0 as libc::c_int as I;
                                }
                                appender(a, n, s, c);
                                v = complete(*a, *n, p, 0 as *mut I);
                                b = parsedepth(*p);
                                if v == 3 as libc::c_int as libc::c_longlong {
                                    show(kerr(b"nest\0" as *const u8 as *const libc::c_char));
                                    current_block = 2906900040509793395;
                                } else if v == 2 as libc::c_int as libc::c_longlong {
                                    show(
                                        kerr(b"unmatched\0" as *const u8 as *const libc::c_char),
                                    );
                                    b = 0 as libc::c_int as I;
                                    current_block = 2906900040509793395;
                                } else if v == 1 as libc::c_int as libc::c_longlong {
                                    fCmplt = 1 as libc::c_int as I;
                                    current_block = 2543471465276344483;
                                } else {
                                    if v == 0 as libc::c_int as libc::c_longlong {
                                        fCmplt = 0 as libc::c_int as I;
                                    }
                                    if !n.is_null()
                                        && '\n' as i32
                                            == *(*a)
                                                .offset(
                                                    (*n - 1 as libc::c_int as libc::c_longlong) as isize,
                                                ) as libc::c_int
                                    {
                                        *n -= 1;
                                        *(*a).offset(*n as isize) = 0 as libc::c_int as C;
                                    }
                                    if pthread_mutex_lock(&mut execute_mutex) != 0 {
                                        perror(
                                            b"Lock mutex in line()\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        abort();
                                    }
                                    d = clock() as F;
                                    k = ex(wd(*a, *n as libc::c_int));
                                    d = (clock() as libc::c_double - d)
                                        / 1000000 as libc::c_int as __clock_t as libc::c_double;
                                    if pthread_mutex_unlock(&mut execute_mutex) != 0 {
                                        perror(
                                            b"Unlock mutex in line()\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        abort();
                                    }
                                    if o != 0 && !k.is_null() {
                                        printf(
                                            b"Elapsed: %.7f\n\0" as *const u8 as *const libc::c_char,
                                            d,
                                        );
                                    }
                                    current_block = 970667356130215337;
                                }
                            }
                        }
                    }
                    match current_block {
                        2906900040509793395 => {}
                        2543471465276344483 => {}
                        _ => {
                            if o != 0 && fam != 0 && feci == 0 {
                                show(k);
                            }
                            cd(k);
                            current_block = 2906900040509793395;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        2906900040509793395 => {
            if fCheck != 0
                && (strlen(s as *const libc::c_char) == 0 as libc::c_int as libc::c_ulong
                    || (*s
                        .offset(
                            (strlen(s as *const libc::c_char))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int) < 0 as libc::c_int)
            {
                exit(0 as libc::c_int);
            }
            ptr = 0 as S;
            if !(strcmp(
                errmsg.as_mut_ptr(),
                b"value\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                if strcmp(
                    errmsg.as_mut_ptr(),
                    b"(nil)\0" as *const u8 as *const libc::c_char,
                ) != 0 && fer != -(1 as libc::c_int) as libc::c_longlong
                {
                    oerr();
                    let mut ctl: I = 0 as libc::c_int as I;
                    if fError != 0 {
                        if 2 as libc::c_int as libc::c_longlong == fError {
                            exit(1 as libc::c_int);
                        }
                        if !lineA.is_null() {
                            if !fnc.is_null() {
                                let mut cnt: I = 0 as libc::c_int as I;
                                let mut i_0: I = 0;
                                if strlen(fnc as *const libc::c_char)
                                    == 1 as libc::c_int as libc::c_ulong
                                {
                                    i_0 = 0 as libc::c_int as I;
                                    while (i_0 as libc::c_ulonglong)
                                        < strlen(lineA as *const libc::c_char) as libc::c_ulonglong
                                    {
                                        if *lineA.offset(i_0 as isize) as libc::c_int
                                            == *fnc as libc::c_int
                                        {
                                            cnt += 1;
                                            cnt;
                                        }
                                        i_0 += 1;
                                        i_0;
                                    }
                                } else {
                                    i_0 = 0 as libc::c_int as I;
                                    while (i_0 as libc::c_ulonglong)
                                        < (strlen(lineA as *const libc::c_char))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as libc::c_ulonglong
                                    {
                                        if *lineA.offset(i_0 as isize) as libc::c_int
                                            == *fnc.offset(0 as libc::c_int as isize) as libc::c_int
                                        {
                                            if *lineA
                                                .offset(
                                                    (i_0 + 1 as libc::c_int as libc::c_longlong) as isize,
                                                ) as libc::c_int
                                                == *fnc.offset(1 as libc::c_int as isize) as libc::c_int
                                            {
                                                ptr = &mut *lineA.offset(i_0 as isize) as *mut C;
                                                cnt += 1;
                                                cnt;
                                            }
                                        }
                                        i_0 += 1;
                                        i_0;
                                    }
                                }
                                if cnt == 1 as libc::c_int as libc::c_longlong {
                                    ctl = 1 as libc::c_int as I;
                                    printf(
                                        b"%s\n\0" as *const u8 as *const libc::c_char,
                                        lineA,
                                    );
                                    if ptr.is_null() {
                                        ptr = strchr(
                                            lineA as *const libc::c_char,
                                            *fnc as libc::c_int,
                                        );
                                    }
                                    let mut i_1: I = 0 as libc::c_int as I;
                                    let mut _i: I = ptr.offset_from(lineA) as libc::c_long as I;
                                    while i_1 < _i {
                                        printf(b" \0" as *const u8 as *const libc::c_char);
                                        i_1 += 1;
                                        i_1;
                                    }
                                    printf(b"^\n\0" as *const u8 as *const libc::c_char);
                                }
                                if cnt > 1 as libc::c_int as libc::c_longlong && fnci != 0
                                    && fnci < 127 as libc::c_int as libc::c_longlong
                                {
                                    ocr = 0 as libc::c_int as I;
                                    i_0 = 0 as libc::c_int as I;
                                    while i_0 < fnci {
                                        if fncp[i_0 as usize]
                                            == fncp[(fnci - 1 as libc::c_int as libc::c_longlong)
                                                as usize]
                                        {
                                            ocr += 1;
                                            ocr;
                                        }
                                        i_0 += 1;
                                        i_0;
                                    }
                                    printf(
                                        b"%s\n\0" as *const u8 as *const libc::c_char,
                                        lineA,
                                    );
                                    printf(
                                        b"at execution instance %lld of \"%s\"\n\0" as *const u8
                                            as *const libc::c_char,
                                        ocr,
                                        fnc,
                                    );
                                    fnci = 0 as libc::c_int as I;
                                }
                            }
                        }
                        if !lineB.is_null() && ctl == 0
                            && strcmp(
                                lineA as *const libc::c_char,
                                lineB as *const libc::c_char,
                            ) != 0
                        {
                            if !fnc.is_null() {
                                let mut cnt_0: I = 0 as libc::c_int as I;
                                let mut i_2: I = 0;
                                printf(
                                    b"%s\n\0" as *const u8 as *const libc::c_char,
                                    lineB,
                                );
                                i_2 = 0 as libc::c_int as I;
                                while (i_2 as libc::c_ulonglong)
                                    < strlen(lineB as *const libc::c_char) as libc::c_ulonglong
                                {
                                    if *lineB.offset(i_2 as isize) as libc::c_int
                                        == *fnc as libc::c_int
                                    {
                                        cnt_0 += 1;
                                        cnt_0;
                                    }
                                    i_2 += 1;
                                    i_2;
                                }
                                if cnt_0 == 1 as libc::c_int as libc::c_longlong {
                                    let mut ptr_0: S = strchr(
                                        lineB as *const libc::c_char,
                                        *fnc as libc::c_int,
                                    );
                                    let mut i_3: I = 0 as libc::c_int as I;
                                    let mut _i_0: I = ptr_0.offset_from(lineB) as libc::c_long
                                        as I;
                                    while i_3 < _i_0 {
                                        printf(b" \0" as *const u8 as *const libc::c_char);
                                        i_3 += 1;
                                        i_3;
                                    }
                                    printf(b"^\n\0" as *const u8 as *const libc::c_char);
                                }
                                if cnt_0 > 1 as libc::c_int as libc::c_longlong && fnci != 0
                                    && fnci < 127 as libc::c_int as libc::c_longlong
                                {
                                    ocr = 0 as libc::c_int as I;
                                    i_2 = 0 as libc::c_int as I;
                                    while i_2 < fnci {
                                        if fncp[i_2 as usize]
                                            == fncp[(fnci - 1 as libc::c_int as libc::c_longlong)
                                                as usize]
                                        {
                                            ocr += 1;
                                            ocr;
                                        }
                                        i_2 += 1;
                                        i_2;
                                    }
                                    printf(
                                        b"at execution instance %lld of %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        ocr,
                                        fnc,
                                    );
                                }
                            }
                        }
                        if !lineA.is_null() || !lineB.is_null() {
                            if flc == 0 {
                                let mut i_4: I = 0 as libc::c_int as I;
                                i_4 = 0 as libc::c_int as I;
                                while (i_4 as libc::c_ulonglong)
                                    < (1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(strlen(*a as *const libc::c_char))
                                        as libc::c_ulonglong
                                {
                                    lineC[i_4 as usize] = *(*a).offset(i_4 as isize);
                                    i_4 += 1;
                                    i_4;
                                }
                                flc = 1 as libc::c_int as I;
                            }
                            check();
                        }
                    }
                }
            }
            if !(*p).is_null() {
                pdafree(*p);
            }
            *p = 0 as PDA;
            free(*a as *mut libc::c_void);
            *a = 0 as S;
            *n = 0 as libc::c_int as I;
            free(s as *mut libc::c_void);
            s = 0 as S;
        }
        _ => {}
    }
    if fWksp != 0 {
        printf(
            b"used now : %lld (%lld %lld)\n\0" as *const u8 as *const libc::c_char,
            mUsed as I,
            mAlloc as I,
            mMap as I,
        );
        printf(b"max used : %lld\n\0" as *const u8 as *const libc::c_char, mMax as I);
        printf(b"symbols  : \0" as *const u8 as *const libc::c_char);
        let mut cnt_1: I = nodeCount(SYMBOLS);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"count    : %lld\n\0" as *const u8 as *const libc::c_char, cnt_1);
        fWksp = 0 as libc::c_int as I;
    }
    if o != 0 && fLoad == 0 && q != 0 {
        prompt(b + fCheck);
    }
    kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
    feci = 0 as libc::c_int as I;
    fom = feci;
    fnci = fom;
    fer1 = fnci;
    fer = fer1;
    fll = fer;
    if fCheck == 0 {
        lineB = 0 as S;
        lineA = lineB;
        fnc = lineA;
    }
    if !cls.is_null() {
        cd(cls);
        cls = 0 as K;
    }
    return c;
}
pub static mut tmr_ival: I = 0 as libc::c_int as I;
pub unsafe extern "C" fn timer_thread(mut arg: V) -> V {
    loop {
        if tmr_ival != 0 {
            let mut a: K = _n();
            let mut h: K = *denameS(
                b".\0" as *const u8 as *const libc::c_char as S,
                b".m.ts\0" as *const u8 as *const libc::c_char as S,
                0 as libc::c_int as I,
            );
            let mut z: K = 0 as K;
            if 6 as libc::c_int as libc::c_longlong != (*h).t {
                if pthread_mutex_lock(&mut execute_mutex) != 0 {
                    perror(
                        b"Lock mutex in timer_thread())\0" as *const u8
                            as *const libc::c_char,
                    );
                    abort();
                }
                z = at(h, a);
                if pthread_mutex_unlock(&mut execute_mutex) != 0 {
                    perror(
                        b"Unlock mutex in timer_thread())\0" as *const u8
                            as *const libc::c_char,
                    );
                    abort();
                }
            }
            if !z.is_null() {
                cd(z);
            }
            cd(a);
        }
        usleep(
            (if tmr_ival != 0 {
                1000 as libc::c_int as libc::c_longlong * tmr_ival
            } else {
                10000 as libc::c_int as libc::c_longlong
            }) as __useconds_t,
        );
    };
}
unsafe extern "C" fn handle_SIGINT(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(&mut interrupted as *mut sig_atomic_t, 1 as libc::c_int);
}
pub static mut master: fd_set = fd_set { __fds_bits: [0; 16] };
pub unsafe extern "C" fn attend() -> I {
    fer = 0 as libc::c_int as I;
    let mut a: S = 0 as S;
    let mut n: I = 0 as libc::c_int as I;
    let mut q: PDA = 0 as PDA;
    let mut read_fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmax: libc::c_int = fileno(stdin);
    let mut listener: libc::c_int = 0 as libc::c_int;
    let mut newfd: libc::c_int = 0;
    let mut remoteaddr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut addrlen: socklen_t = 0;
    let mut nbytes: libc::c_int = 0;
    let mut yes: I = 1 as libc::c_int as I;
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh2 = &mut __d0;
    let fresh3;
    let fresh4 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh5 = &mut __d1;
    let fresh6;
    let fresh7 = &mut *(master.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh2,
        fresh4) => fresh3, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh5,
        fresh7) => fresh6, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
    let mut __d0_0: libc::c_int = 0;
    let mut __d1_0: libc::c_int = 0;
    let fresh8 = &mut __d0_0;
    let fresh9;
    let fresh10 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh11 = &mut __d1_0;
    let fresh12;
    let fresh13 = &mut *(read_fds.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh8,
        fresh10) => fresh9, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh11,
        fresh13) => fresh12, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
    c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_12 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sa
        .__sigaction_handler
        .sa_handler = Some(handle_SIGINT as unsafe extern "C" fn(libc::c_int) -> ());
    sa.sa_flags = 0x10000000 as libc::c_int;
    sigemptyset(&mut sa.sa_mask);
    let mut res: I = sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction) as I;
    if res != 0 {
        show(kerr(b"sigaction\0" as *const u8 as *const libc::c_char));
        return -(1 as libc::c_int) as I;
    }
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 2 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int;
    master
        .__fds_bits[(fileno(stdin)
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fileno(stdin)
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    if !IPC_PORT.is_null() || !HTTP_PORT.is_null() {
        rv = getaddrinfo(
            HOST_IFACE as *const libc::c_char,
            (if !IPC_PORT.is_null() { IPC_PORT } else { HTTP_PORT })
                as *const libc::c_char,
            &mut hints,
            &mut ai,
        );
        if rv != 0 {
            fprintf(
                stderr,
                b"server: %s\n\0" as *const u8 as *const libc::c_char,
                gai_strerror(rv),
            );
            exit(1 as libc::c_int);
        }
        p = ai;
        while !p.is_null() {
            listener = socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
            if !(listener < 0 as libc::c_int) {
                if !(bind(listener, (*p).ai_addr, (*p).ai_addrlen) < 0 as libc::c_int) {
                    break;
                }
                if close(listener) != 0 {
                    show(kerr(b"file\0" as *const u8 as *const libc::c_char));
                }
            }
            p = (*p).ai_next;
        }
        if p.is_null() {
            fprintf(
                stderr,
                b"server: failed to bind\n\0" as *const u8 as *const libc::c_char,
            );
            exit(2 as libc::c_int);
        }
        freeaddrinfo(ai);
        if -(1 as libc::c_int) == listen(listener, 10 as libc::c_int) {
            perror(b"listen\0" as *const u8 as *const libc::c_char);
            exit(3 as libc::c_int);
        }
        master
            .__fds_bits[(listener
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << listener
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        fdmax = listener;
    }
    fln = 1 as libc::c_int as I;
    loop {
        scrLim = 0 as libc::c_int as I;
        read_fds = master;
        if -(1 as libc::c_int)
            == select(
                fdmax + 1 as libc::c_int,
                &mut read_fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                0 as *mut timeval,
            )
        {
            if *__errno_location() == 4 as libc::c_int {
                ::std::ptr::write_volatile(
                    &mut interrupted as *mut sig_atomic_t,
                    0 as libc::c_int,
                );
                *__errno_location() = 0 as libc::c_int;
            } else {
                perror(b"select\0" as *const u8 as *const libc::c_char);
                exit(4 as libc::c_int);
            }
        }
        i = 0 as libc::c_int;
        while i <= fdmax {
            if read_fds
                .__fds_bits[(i
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << i
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                if i == fileno(stdin) {
                    ofCheck = 0 as libc::c_int as I;
                    fCheck = ofCheck;
                    flc = fCheck;
                    nbytes = line(stdin, &mut a, &mut n, &mut q) as libc::c_int;
                    fln = 0 as libc::c_int as I;
                    if nbytes <= 0 as libc::c_int {
                        if IPC_PORT.is_null() && HTTP_PORT.is_null() {
                            exit(0 as libc::c_int);
                        } else {
                            master
                                .__fds_bits[(i
                                / (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as usize]
                                &= !(((1 as libc::c_ulong)
                                    << i
                                        % (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as __fd_mask);
                        }
                    }
                } else if i == listener {
                    addrlen = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong
                        as socklen_t;
                    newfd = accept(
                        listener,
                        &mut remoteaddr as *mut sockaddr_storage as *mut sockaddr,
                        &mut addrlen,
                    );
                    if newfd == -(1 as libc::c_int) {
                        perror(b"accept\0" as *const u8 as *const libc::c_char);
                    } else {
                        wipe_tape(newfd as I);
                        master
                            .__fds_bits[(newfd
                            / (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as usize]
                            |= ((1 as libc::c_ulong)
                                << newfd
                                    % (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as __fd_mask;
                        if newfd > fdmax {
                            fdmax = newfd;
                        }
                        setsockopt(
                            newfd,
                            IPPROTO_TCP as libc::c_int,
                            1 as libc::c_int,
                            &mut yes as *mut I as *const libc::c_void,
                            ::std::mem::size_of::<I>() as libc::c_ulong as socklen_t,
                        );
                        CP[newfd as usize]
                            .a = ntohl(
                            (*(&mut remoteaddr as *mut sockaddr_storage
                                as *mut sockaddr_in))
                                .sin_addr
                                .s_addr,
                        ) as I;
                    }
                } else if a.is_null() {
                    read_tape(i as I, i as I, 0 as libc::c_int as I);
                }
            }
            i += 1;
            i;
        }
    };
}

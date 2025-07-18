use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type bn_blinding_st;
    pub type engine_st;
    pub type ASN1_VALUE_st;
    pub type evp_pkey_ctx_st;
    pub type ec_key_st;
    pub type evp_pkey_asn1_method_st;
    pub type X509_POLICY_CACHE_st;
    pub type x509_crl_method_st;
    pub type stack_st_GENERAL_NAMES;
    pub type X509_POLICY_TREE_st;
    pub type ssl3_buf_freelist_st;
    pub type cert_st;
    pub type sess_cert_st;
    pub type ssl3_enc_method;
    pub type stack_st_OCSP_RESPID;
    pub type _pqueue;
    pub type ev_loop;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pause() -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn ASN1_STRING_to_UTF8(
        out: *mut *mut libc::c_uchar,
        in_0: *mut ASN1_STRING,
    ) -> libc::c_int;
    fn BIO_ctrl(
        bp: *mut BIO,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn BIO_read(b: *mut BIO, data: *mut libc::c_void, len: libc::c_int) -> libc::c_int;
    fn BIO_free(a: *mut BIO) -> libc::c_int;
    fn BIO_new(type_0: *mut BIO_METHOD) -> *mut BIO;
    fn BIO_new_file(
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> *mut BIO;
    fn BIO_s_file() -> *mut BIO_METHOD;
    fn BIO_test_flags(b: *const BIO, flags: libc::c_int) -> libc::c_int;
    fn SSLeay() -> libc::c_ulong;
    fn sk_num(_: *const _STACK) -> libc::c_int;
    fn sk_value(_: *const _STACK, _: libc::c_int) -> *mut libc::c_void;
    fn sk_pop_free(
        st: *mut _STACK,
        func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn RSA_free(r: *mut RSA);
    fn DH_free(dh: *mut DH);
    fn DH_size(dh: *const DH) -> libc::c_int;
    fn SSL_get_ex_data(ssl: *const SSL, idx: libc::c_int) -> *mut libc::c_void;
    fn SSL_set_ex_data(
        ssl: *mut SSL,
        idx: libc::c_int,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn SSL_set_SSL_CTX(ssl: *mut SSL, ctx: *mut SSL_CTX) -> *mut SSL_CTX;
    fn SSL_get1_session(ssl: *mut SSL) -> *mut SSL_SESSION;
    fn SSL_set_shutdown(ssl: *mut SSL, mode: libc::c_int);
    fn SSL_library_init() -> libc::c_int;
    fn SSL_set_accept_state(s: *mut SSL);
    fn SSL_set_connect_state(s: *mut SSL);
    fn SSL_do_handshake(s: *mut SSL) -> libc::c_int;
    fn TLSv1_client_method() -> *const SSL_METHOD;
    fn TLSv1_server_method() -> *const SSL_METHOD;
    fn SSLv23_client_method() -> *const SSL_METHOD;
    fn SSLv23_server_method() -> *const SSL_METHOD;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn SSL_CTX_callback_ctrl(
        _: *mut SSL_CTX,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_long;
    fn SSL_CTX_ctrl(
        ctx: *mut SSL_CTX,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_ctrl(
        ssl: *mut SSL,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_write(
        ssl: *mut SSL,
        buf: *const libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int) -> libc::c_int;
    fn SSL_free(ssl: *mut SSL);
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn SSL_CTX_use_RSAPrivateKey(ctx: *mut SSL_CTX, rsa: *mut RSA) -> libc::c_int;
    fn SSL_set_session(to: *mut SSL, session: *mut SSL_SESSION) -> libc::c_int;
    fn SSL_SESSION_free(ses: *mut SSL_SESSION);
    fn SSL_load_error_strings();
    fn SSL_CTX_use_certificate_chain_file(
        ctx: *mut SSL_CTX,
        file: *const libc::c_char,
    ) -> libc::c_int;
    fn SSL_get_rbio(s: *const SSL) -> *mut BIO;
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    fn PEM_read_bio_X509_AUX(
        bp: *mut BIO,
        x: *mut *mut X509,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut X509;
    fn PEM_read_bio_RSAPrivateKey(
        bp: *mut BIO,
        x: *mut *mut RSA,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut RSA;
    fn PEM_read_bio_DHparams(
        bp: *mut BIO,
        x: *mut *mut DH,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut DH;
    fn SSL_CTX_set_info_callback(
        ctx: *mut SSL_CTX,
        cb: Option::<unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> ()>,
    );
    fn SSL_get_servername(s: *const SSL, type_0: libc::c_int) -> *const libc::c_char;
    fn SSL_CTX_set_cipher_list(_: *mut SSL_CTX, str: *const libc::c_char) -> libc::c_int;
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    fn X509_get_subject_name(a: *mut X509) -> *mut X509_NAME;
    fn X509_get_ext_d2i(
        x: *mut X509,
        nid: libc::c_int,
        crit: *mut libc::c_int,
        idx: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn X509_NAME_get_entry(
        name: *mut X509_NAME,
        loc: libc::c_int,
    ) -> *mut X509_NAME_ENTRY;
    fn X509_NAME_get_index_by_NID(
        name: *mut X509_NAME,
        nid: libc::c_int,
        lastpos: libc::c_int,
    ) -> libc::c_int;
    fn GENERAL_NAME_free(a: *mut GENERAL_NAME);
    fn ERR_print_errors_fp(fp: *mut FILE);
    fn ENGINE_register_all_complete() -> libc::c_int;
    fn ENGINE_free(e: *mut ENGINE) -> libc::c_int;
    fn ENGINE_get_id(e: *const ENGINE) -> *const libc::c_char;
    fn ENGINE_init(e: *mut ENGINE) -> libc::c_int;
    fn ENGINE_finish(e: *mut ENGINE) -> libc::c_int;
    fn ENGINE_set_default(e: *mut ENGINE, flags: libc::c_uint) -> libc::c_int;
    fn ENGINE_by_id(id: *const libc::c_char) -> *mut ENGINE;
    fn ENGINE_load_builtin_engines();
    fn ev_default_loop(flags: libc::c_uint) -> *mut ev_loop;
    fn ev_run(loop_1: *mut ev_loop, flags: libc::c_int) -> libc::c_int;
    fn ev_io_start(loop_1: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_1: *mut ev_loop, w: *mut ev_io);
    fn ev_timer_stop(loop_1: *mut ev_loop, w: *mut ev_timer);
    fn ev_timer_start(loop_1: *mut ev_loop, w: *mut ev_timer);
    fn ringbuffer_init(rb: *mut ringbuffer);
    fn ringbuffer_read_next(
        rb: *mut ringbuffer,
        length: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn ringbuffer_read_skip(rb: *mut ringbuffer, length: libc::c_int);
    fn ringbuffer_read_pop(rb: *mut ringbuffer);
    fn ringbuffer_write_ptr(rb: *mut ringbuffer) -> *mut libc::c_char;
    fn ringbuffer_write_append(rb: *mut ringbuffer, length: libc::c_int);
    fn ringbuffer_is_empty(rb: *mut ringbuffer) -> libc::c_int;
    fn ringbuffer_is_full(rb: *mut ringbuffer) -> libc::c_int;
    fn config_new() -> *mut stud_config;
    fn config_parse_cli(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        cfg: *mut stud_config,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
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
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
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
pub type va_list = __builtin_va_list;
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
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st {
    pub num: libc::c_int,
    pub data: *mut *mut libc::c_char,
    pub sorted: libc::c_int,
    pub num_alloc: libc::c_int,
    pub comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
}
pub type _STACK = stack_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
}
pub type ASN1_INTEGER = asn1_string_st;
pub type ASN1_ENUMERATED = asn1_string_st;
pub type ASN1_BIT_STRING = asn1_string_st;
pub type ASN1_OCTET_STRING = asn1_string_st;
pub type ASN1_PRINTABLESTRING = asn1_string_st;
pub type ASN1_T61STRING = asn1_string_st;
pub type ASN1_IA5STRING = asn1_string_st;
pub type ASN1_GENERALSTRING = asn1_string_st;
pub type ASN1_UNIVERSALSTRING = asn1_string_st;
pub type ASN1_BMPSTRING = asn1_string_st;
pub type ASN1_UTCTIME = asn1_string_st;
pub type ASN1_TIME = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_VISIBLESTRING = asn1_string_st;
pub type ASN1_UTF8STRING = asn1_string_st;
pub type ASN1_STRING = asn1_string_st;
pub type ASN1_BOOLEAN = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type BN_BLINDING = bn_blinding_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_mont_ctx_st {
    pub ri: libc::c_int,
    pub RR: BIGNUM,
    pub N: BIGNUM,
    pub Ni: BIGNUM,
    pub n0: [libc::c_ulong; 2],
    pub flags: libc::c_int,
}
pub type BN_MONT_CTX = bn_mont_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_gencb_st {
    pub ver: libc::c_uint,
    pub arg: *mut libc::c_void,
    pub cb: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub cb_1: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub cb_2: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut BN_GENCB) -> libc::c_int,
    >,
}
pub type BN_GENCB = bn_gencb_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
}
pub type BUF_MEM = buf_mem_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_st {
    pub nid: libc::c_int,
    pub block_size: libc::c_int,
    pub key_len: libc::c_int,
    pub iv_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *const libc::c_uchar,
            *const libc::c_uchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub do_cipher: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *mut libc::c_uchar,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_CIPHER_CTX) -> libc::c_int>,
    pub ctx_size: libc::c_int,
    pub set_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub get_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub app_data: *mut libc::c_void,
}
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_ctx_st {
    pub cipher: *const EVP_CIPHER,
    pub engine: *mut ENGINE,
    pub encrypt: libc::c_int,
    pub buf_len: libc::c_int,
    pub oiv: [libc::c_uchar; 16],
    pub iv: [libc::c_uchar; 16],
    pub buf: [libc::c_uchar; 32],
    pub num: libc::c_int,
    pub app_data: *mut libc::c_void,
    pub key_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub cipher_data: *mut libc::c_void,
    pub final_used: libc::c_int,
    pub block_mask: libc::c_int,
    pub final_0: [libc::c_uchar; 32],
}
pub type ENGINE = engine_st;
pub type EVP_CIPHER = evp_cipher_st;
pub type ASN1_TYPE = asn1_type_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_type_st {
    pub type_0: libc::c_int,
    pub value: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub ptr: *mut libc::c_char,
    pub boolean: ASN1_BOOLEAN,
    pub asn1_string: *mut ASN1_STRING,
    pub object: *mut ASN1_OBJECT,
    pub integer: *mut ASN1_INTEGER,
    pub enumerated: *mut ASN1_ENUMERATED,
    pub bit_string: *mut ASN1_BIT_STRING,
    pub octet_string: *mut ASN1_OCTET_STRING,
    pub printablestring: *mut ASN1_PRINTABLESTRING,
    pub t61string: *mut ASN1_T61STRING,
    pub ia5string: *mut ASN1_IA5STRING,
    pub generalstring: *mut ASN1_GENERALSTRING,
    pub bmpstring: *mut ASN1_BMPSTRING,
    pub universalstring: *mut ASN1_UNIVERSALSTRING,
    pub utctime: *mut ASN1_UTCTIME,
    pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut ASN1_VISIBLESTRING,
    pub utf8string: *mut ASN1_UTF8STRING,
    pub set: *mut ASN1_STRING,
    pub sequence: *mut ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
pub type ASN1_VALUE = ASN1_VALUE_st;
pub type ASN1_OBJECT = asn1_object_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_object_st {
    pub sn: *const libc::c_char,
    pub ln: *const libc::c_char,
    pub nid: libc::c_int,
    pub length: libc::c_int,
    pub data: *const libc::c_uchar,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_st {
    pub type_0: libc::c_int,
    pub pkey_type: libc::c_int,
    pub md_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub final_0: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *mut libc::c_uchar) -> libc::c_int,
    >,
    pub copy: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const EVP_MD_CTX) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub required_pkey_type: [libc::c_int; 5],
    pub block_size: libc::c_int,
    pub ctx_size: libc::c_int,
    pub md_ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_MD_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
}
pub type EVP_MD_CTX = env_md_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_ctx_st {
    pub digest: *const EVP_MD,
    pub engine: *mut ENGINE,
    pub flags: libc::c_ulong,
    pub md_data: *mut libc::c_void,
    pub pctx: *mut EVP_PKEY_CTX,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
}
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type EVP_MD = env_md_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_pkey_st {
    pub type_0: libc::c_int,
    pub save_type: libc::c_int,
    pub references: libc::c_int,
    pub ameth: *const EVP_PKEY_ASN1_METHOD,
    pub engine: *mut ENGINE,
    pub pkey: C2RustUnnamed_15,
    pub save_parameters: libc::c_int,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_ATTRIBUTE {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub ptr: *mut libc::c_char,
    pub rsa: *mut rsa_st,
    pub dsa: *mut dsa_st,
    pub dh: *mut dh_st,
    pub ec: *mut ec_key_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_st {
    pub pad: libc::c_int,
    pub version: libc::c_int,
    pub p: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub length: libc::c_long,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub q: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub seed: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub counter: *mut BIGNUM,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DH_METHOD,
    pub engine: *mut ENGINE,
}
pub type DH_METHOD = dh_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_method {
    pub name: *const libc::c_char,
    pub generate_key: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub compute_key: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, *const BIGNUM, *mut DH) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *const DH,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub generate_params: Option::<
        unsafe extern "C" fn(
            *mut DH,
            libc::c_int,
            libc::c_int,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type DH = dh_st;
pub type CRYPTO_EX_DATA = crypto_ex_data_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crypto_ex_data_st {
    pub sk: *mut stack_st_void,
    pub dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_void {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub write_params: libc::c_int,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub kinv: *mut BIGNUM,
    pub r: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DSA_METHOD,
    pub engine: *mut ENGINE,
}
pub type DSA_METHOD = dsa_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_method {
    pub name: *const libc::c_char,
    pub dsa_do_sign: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_int, *mut DSA) -> *mut DSA_SIG,
    >,
    pub dsa_sign_setup: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BN_CTX,
            *mut *mut BIGNUM,
            *mut *mut BIGNUM,
        ) -> libc::c_int,
    >,
    pub dsa_do_verify: Option::<
        unsafe extern "C" fn(
            *const libc::c_uchar,
            libc::c_int,
            *mut DSA_SIG,
            *mut DSA,
        ) -> libc::c_int,
    >,
    pub dsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub dsa_paramgen: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            libc::c_int,
            *const libc::c_uchar,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_ulong,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
    pub dsa_keygen: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
}
pub type DSA = dsa_st;
pub type DSA_SIG = DSA_SIG_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSA_SIG_st {
    pub r: *mut BIGNUM,
    pub s: *mut BIGNUM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub meth: *const RSA_METHOD,
    pub engine: *mut ENGINE,
    pub n: *mut BIGNUM,
    pub e: *mut BIGNUM,
    pub d: *mut BIGNUM,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub dmp1: *mut BIGNUM,
    pub dmq1: *mut BIGNUM,
    pub iqmp: *mut BIGNUM,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub _method_mod_n: *mut BN_MONT_CTX,
    pub _method_mod_p: *mut BN_MONT_CTX,
    pub _method_mod_q: *mut BN_MONT_CTX,
    pub bignum_data: *mut libc::c_char,
    pub blinding: *mut BN_BLINDING,
    pub mt_blinding: *mut BN_BLINDING,
}
pub type RSA_METHOD = rsa_meth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_meth_st {
    pub name: *const libc::c_char,
    pub rsa_pub_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_pub_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *mut RSA,
            *mut BN_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub rsa_sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_keygen: Option::<
        unsafe extern "C" fn(
            *mut RSA,
            libc::c_int,
            *mut BIGNUM,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type RSA = rsa_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type EVP_PKEY = evp_pkey_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_st {
    pub cert_info: *mut X509_CINF,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub valid: libc::c_int,
    pub references: libc::c_int,
    pub name: *mut libc::c_char,
    pub ex_data: CRYPTO_EX_DATA,
    pub ex_pathlen: libc::c_long,
    pub ex_pcpathlen: libc::c_long,
    pub ex_flags: libc::c_ulong,
    pub ex_kusage: libc::c_ulong,
    pub ex_xkusage: libc::c_ulong,
    pub ex_nscert: libc::c_ulong,
    pub skid: *mut ASN1_OCTET_STRING,
    pub akid: *mut AUTHORITY_KEYID,
    pub policy_cache: *mut X509_POLICY_CACHE,
    pub crldp: *mut stack_st_DIST_POINT,
    pub altname: *mut stack_st_GENERAL_NAME,
    pub nc: *mut NAME_CONSTRAINTS,
    pub sha1_hash: [libc::c_uchar; 20],
    pub aux: *mut X509_CERT_AUX,
}
pub type X509_CERT_AUX = x509_cert_aux_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_cert_aux_st {
    pub trust: *mut stack_st_ASN1_OBJECT,
    pub reject: *mut stack_st_ASN1_OBJECT,
    pub alias: *mut ASN1_UTF8STRING,
    pub keyid: *mut ASN1_OCTET_STRING,
    pub other: *mut stack_st_X509_ALGOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_ALGOR {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_ASN1_OBJECT {
    pub stack: _STACK,
}
pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NAME_CONSTRAINTS_st {
    pub permittedSubtrees: *mut stack_st_GENERAL_SUBTREE,
    pub excludedSubtrees: *mut stack_st_GENERAL_SUBTREE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_GENERAL_SUBTREE {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_GENERAL_NAME {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_DIST_POINT {
    pub stack: _STACK,
}
pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUTHORITY_KEYID_st {
    pub keyid: *mut ASN1_OCTET_STRING,
    pub issuer: *mut GENERAL_NAMES,
    pub serial: *mut ASN1_INTEGER,
}
pub type GENERAL_NAMES = stack_st_GENERAL_NAME;
pub type X509_ALGOR = X509_algor_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_algor_st {
    pub algorithm: *mut ASN1_OBJECT,
    pub parameter: *mut ASN1_TYPE,
}
pub type X509_CINF = x509_cinf_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_cinf_st {
    pub version: *mut ASN1_INTEGER,
    pub serialNumber: *mut ASN1_INTEGER,
    pub signature: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub validity: *mut X509_VAL,
    pub subject: *mut X509_NAME,
    pub key: *mut X509_PUBKEY,
    pub issuerUID: *mut ASN1_BIT_STRING,
    pub subjectUID: *mut ASN1_BIT_STRING,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
pub type ASN1_ENCODING = ASN1_ENCODING_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ASN1_ENCODING_st {
    pub enc: *mut libc::c_uchar,
    pub len: libc::c_long,
    pub modified: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_EXTENSION {
    pub stack: _STACK,
}
pub type X509_PUBKEY = X509_pubkey_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_pubkey_st {
    pub algor: *mut X509_ALGOR,
    pub public_key: *mut ASN1_BIT_STRING,
    pub pkey: *mut EVP_PKEY,
}
pub type X509_NAME = X509_name_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_name_st {
    pub entries: *mut stack_st_X509_NAME_ENTRY,
    pub modified: libc::c_int,
    pub bytes: *mut BUF_MEM,
    pub canon_enc: *mut libc::c_uchar,
    pub canon_enclen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_NAME_ENTRY {
    pub stack: _STACK,
}
pub type X509_VAL = X509_val_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_val_st {
    pub notBefore: *mut ASN1_TIME,
    pub notAfter: *mut ASN1_TIME,
}
pub type X509 = x509_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_crl_st {
    pub crl: *mut X509_CRL_INFO,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub akid: *mut AUTHORITY_KEYID,
    pub idp: *mut ISSUING_DIST_POINT,
    pub idp_flags: libc::c_int,
    pub idp_reasons: libc::c_int,
    pub crl_number: *mut ASN1_INTEGER,
    pub base_crl_number: *mut ASN1_INTEGER,
    pub sha1_hash: [libc::c_uchar; 20],
    pub issuers: *mut stack_st_GENERAL_NAMES,
    pub meth: *const X509_CRL_METHOD,
    pub meth_data: *mut libc::c_void,
}
pub type X509_CRL_METHOD = x509_crl_method_st;
pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISSUING_DIST_POINT_st {
    pub distpoint: *mut DIST_POINT_NAME,
    pub onlyuser: libc::c_int,
    pub onlyCA: libc::c_int,
    pub onlysomereasons: *mut ASN1_BIT_STRING,
    pub indirectCRL: libc::c_int,
    pub onlyattr: libc::c_int,
}
pub type DIST_POINT_NAME = DIST_POINT_NAME_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DIST_POINT_NAME_st {
    pub type_0: libc::c_int,
    pub name: C2RustUnnamed_16,
    pub dpname: *mut X509_NAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub fullname: *mut GENERAL_NAMES,
    pub relativename: *mut stack_st_X509_NAME_ENTRY,
}
pub type X509_CRL_INFO = X509_crl_info_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_crl_info_st {
    pub version: *mut ASN1_INTEGER,
    pub sig_alg: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub lastUpdate: *mut ASN1_TIME,
    pub nextUpdate: *mut ASN1_TIME,
    pub revoked: *mut stack_st_X509_REVOKED,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_REVOKED {
    pub stack: _STACK,
}
pub type X509_CRL = X509_crl_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_store_st {
    pub cache: libc::c_int,
    pub objs: *mut stack_st_X509_OBJECT,
    pub get_cert_methods: *mut stack_st_X509_LOOKUP,
    pub param: *mut X509_VERIFY_PARAM,
    pub verify: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub verify_cb: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_issuer: Option::<
        unsafe extern "C" fn(
            *mut *mut X509,
            *mut X509_STORE_CTX,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_issued: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509, *mut X509) -> libc::c_int,
    >,
    pub check_revocation: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_crl: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_CRL) -> libc::c_int,
    >,
    pub cert_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub lookup_certs: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_NAME) -> *mut stack_st_X509,
    >,
    pub lookup_crls: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_NAME,
        ) -> *mut stack_st_X509_CRL,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
}
pub type X509_STORE_CTX = x509_store_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_store_ctx_st {
    pub ctx: *mut X509_STORE,
    pub current_method: libc::c_int,
    pub cert: *mut X509,
    pub untrusted: *mut stack_st_X509,
    pub crls: *mut stack_st_X509_CRL,
    pub param: *mut X509_VERIFY_PARAM,
    pub other_ctx: *mut libc::c_void,
    pub verify: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub verify_cb: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_issuer: Option::<
        unsafe extern "C" fn(
            *mut *mut X509,
            *mut X509_STORE_CTX,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_issued: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509, *mut X509) -> libc::c_int,
    >,
    pub check_revocation: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_crl: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_CRL) -> libc::c_int,
    >,
    pub cert_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_policy: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub lookup_certs: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_NAME) -> *mut stack_st_X509,
    >,
    pub lookup_crls: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_NAME,
        ) -> *mut stack_st_X509_CRL,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub valid: libc::c_int,
    pub last_untrusted: libc::c_int,
    pub chain: *mut stack_st_X509,
    pub tree: *mut X509_POLICY_TREE,
    pub explicit_policy: libc::c_int,
    pub error_depth: libc::c_int,
    pub error: libc::c_int,
    pub current_cert: *mut X509,
    pub current_issuer: *mut X509,
    pub current_crl: *mut X509_CRL,
    pub current_crl_score: libc::c_int,
    pub current_reasons: libc::c_uint,
    pub parent: *mut X509_STORE_CTX,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509 {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_CRL {
    pub stack: _STACK,
}
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_VERIFY_PARAM_st {
    pub name: *mut libc::c_char,
    pub check_time: time_t,
    pub inh_flags: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub purpose: libc::c_int,
    pub trust: libc::c_int,
    pub depth: libc::c_int,
    pub policies: *mut stack_st_ASN1_OBJECT,
}
pub type X509_STORE = x509_store_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_LOOKUP {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_OBJECT {
    pub stack: _STACK,
}
pub type BIO = bio_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_st {
    pub method: *mut BIO_METHOD,
    pub callback: Option::<
        unsafe extern "C" fn(
            *mut bio_st,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
            libc::c_long,
            libc::c_long,
        ) -> libc::c_long,
    >,
    pub cb_arg: *mut libc::c_char,
    pub init: libc::c_int,
    pub shutdown: libc::c_int,
    pub flags: libc::c_int,
    pub retry_reason: libc::c_int,
    pub num: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub next_bio: *mut bio_st,
    pub prev_bio: *mut bio_st,
    pub references: libc::c_int,
    pub num_read: libc::c_ulong,
    pub num_write: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type BIO_METHOD = bio_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub bwrite: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bread: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bputs: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char) -> libc::c_int,
    >,
    pub bgets: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub create: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub destroy: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            Option::<bio_info_cb>,
        ) -> libc::c_long,
    >,
}
pub type bio_info_cb = unsafe extern "C" fn(
    *mut bio_st,
    libc::c_int,
    *const libc::c_char,
    libc::c_int,
    libc::c_long,
    libc::c_long,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_st {
    pub version: libc::c_int,
    pub type_0: libc::c_int,
    pub method: *const SSL_METHOD,
    pub rbio: *mut BIO,
    pub wbio: *mut BIO,
    pub bbio: *mut BIO,
    pub rwstate: libc::c_int,
    pub in_handshake: libc::c_int,
    pub handshake_func: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub server: libc::c_int,
    pub new_session: libc::c_int,
    pub quiet_shutdown: libc::c_int,
    pub shutdown: libc::c_int,
    pub state: libc::c_int,
    pub rstate: libc::c_int,
    pub init_buf: *mut BUF_MEM,
    pub init_msg: *mut libc::c_void,
    pub init_num: libc::c_int,
    pub init_off: libc::c_int,
    pub packet: *mut libc::c_uchar,
    pub packet_length: libc::c_uint,
    pub s2: *mut ssl2_state_st,
    pub s3: *mut ssl3_state_st,
    pub d1: *mut dtls1_state_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
            size_t,
            *mut SSL,
            *mut libc::c_void,
        ) -> (),
    >,
    pub msg_callback_arg: *mut libc::c_void,
    pub hit: libc::c_int,
    pub param: *mut X509_VERIFY_PARAM,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub mac_flags: libc::c_int,
    pub enc_read_ctx: *mut EVP_CIPHER_CTX,
    pub read_hash: *mut EVP_MD_CTX,
    pub expand: *mut COMP_CTX,
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub cert: *mut cert_st,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub session: *mut SSL_SESSION,
    pub generate_session_id: GEN_SESSION_CB,
    pub verify_mode: libc::c_int,
    pub verify_callback: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub info_callback: Option::<
        unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> (),
    >,
    pub error: libc::c_int,
    pub error_code: libc::c_int,
    pub psk_client_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub psk_server_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub ctx: *mut SSL_CTX,
    pub debug: libc::c_int,
    pub verify_result: libc::c_long,
    pub ex_data: CRYPTO_EX_DATA,
    pub client_CA: *mut stack_st_X509_NAME,
    pub references: libc::c_int,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub first_packet: libc::c_int,
    pub client_version: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub tlsext_debug_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            libc::c_int,
            *mut libc::c_uchar,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub tlsext_debug_arg: *mut libc::c_void,
    pub tlsext_hostname: *mut libc::c_char,
    pub servername_done: libc::c_int,
    pub tlsext_status_type: libc::c_int,
    pub tlsext_status_expected: libc::c_int,
    pub tlsext_ocsp_ids: *mut stack_st_OCSP_RESPID,
    pub tlsext_ocsp_exts: *mut X509_EXTENSIONS,
    pub tlsext_ocsp_resp: *mut libc::c_uchar,
    pub tlsext_ocsp_resplen: libc::c_int,
    pub tlsext_ticket_expected: libc::c_int,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_opaque_prf_input: *mut libc::c_void,
    pub tlsext_opaque_prf_input_len: size_t,
    pub tlsext_session_ticket: *mut TLS_SESSION_TICKET_EXT,
    pub tls_session_ticket_ext_cb: tls_session_ticket_ext_cb_fn,
    pub tls_session_ticket_ext_cb_arg: *mut libc::c_void,
    pub tls_session_secret_cb: tls_session_secret_cb_fn,
    pub tls_session_secret_cb_arg: *mut libc::c_void,
    pub initial_ctx: *mut SSL_CTX,
    pub next_proto_negotiated: *mut libc::c_uchar,
    pub next_proto_negotiated_len: libc::c_uchar,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
    pub srtp_profile: *mut SRTP_PROTECTION_PROFILE,
    pub tlsext_heartbeat: libc::c_uint,
    pub tlsext_hb_pending: libc::c_uint,
    pub tlsext_hb_seq: libc::c_uint,
    pub renegotiate: libc::c_int,
    pub srp_ctx: SRP_CTX,
}
pub type SRP_CTX = srp_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srp_ctx_st {
    pub SRP_cb_arg: *mut libc::c_void,
    pub TLS_ext_srp_username_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub SRP_verify_param_callback: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void) -> libc::c_int,
    >,
    pub SRP_give_srp_client_pwd_callback: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void) -> *mut libc::c_char,
    >,
    pub login: *mut libc::c_char,
    pub N: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub s: *mut BIGNUM,
    pub B: *mut BIGNUM,
    pub A: *mut BIGNUM,
    pub a: *mut BIGNUM,
    pub b: *mut BIGNUM,
    pub v: *mut BIGNUM,
    pub info: *mut libc::c_char,
    pub strength: libc::c_int,
    pub srp_Mask: libc::c_ulong,
}
pub type SSL = ssl_st;
pub type SRTP_PROTECTION_PROFILE = srtp_protection_profile_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srtp_protection_profile_st {
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_SRTP_PROTECTION_PROFILE {
    pub stack: _STACK,
}
pub type SSL_CTX = ssl_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_ctx_st {
    pub method: *const SSL_METHOD,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub cert_store: *mut x509_store_st,
    pub sessions: *mut lhash_st_SSL_SESSION,
    pub session_cache_size: libc::c_ulong,
    pub session_cache_head: *mut ssl_session_st,
    pub session_cache_tail: *mut ssl_session_st,
    pub session_cache_mode: libc::c_int,
    pub session_timeout: libc::c_long,
    pub new_session_cb: Option::<
        unsafe extern "C" fn(*mut ssl_st, *mut SSL_SESSION) -> libc::c_int,
    >,
    pub remove_session_cb: Option::<
        unsafe extern "C" fn(*mut ssl_ctx_st, *mut SSL_SESSION) -> (),
    >,
    pub get_session_cb: Option::<
        unsafe extern "C" fn(
            *mut ssl_st,
            *mut libc::c_uchar,
            libc::c_int,
            *mut libc::c_int,
        ) -> *mut SSL_SESSION,
    >,
    pub stats: C2RustUnnamed_17,
    pub references: libc::c_int,
    pub app_verify_callback: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut libc::c_void) -> libc::c_int,
    >,
    pub app_verify_arg: *mut libc::c_void,
    pub default_passwd_callback: Option::<pem_password_cb>,
    pub default_passwd_callback_userdata: *mut libc::c_void,
    pub client_cert_cb: Option::<
        unsafe extern "C" fn(*mut SSL, *mut *mut X509, *mut *mut EVP_PKEY) -> libc::c_int,
    >,
    pub app_gen_cookie_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_uchar,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub app_verify_cookie_cb: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_uchar, libc::c_uint) -> libc::c_int,
    >,
    pub ex_data: CRYPTO_EX_DATA,
    pub rsa_md5: *const EVP_MD,
    pub md5: *const EVP_MD,
    pub sha1: *const EVP_MD,
    pub extra_certs: *mut stack_st_X509,
    pub comp_methods: *mut stack_st_SSL_COMP,
    pub info_callback: Option::<
        unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> (),
    >,
    pub client_CA: *mut stack_st_X509_NAME,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub cert: *mut cert_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
            size_t,
            *mut SSL,
            *mut libc::c_void,
        ) -> (),
    >,
    pub msg_callback_arg: *mut libc::c_void,
    pub verify_mode: libc::c_int,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub default_verify_callback: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub generate_session_id: GEN_SESSION_CB,
    pub param: *mut X509_VERIFY_PARAM,
    pub quiet_shutdown: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub client_cert_engine: *mut ENGINE,
    pub tlsext_servername_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tlsext_servername_arg: *mut libc::c_void,
    pub tlsext_tick_key_name: [libc::c_uchar; 16],
    pub tlsext_tick_hmac_key: [libc::c_uchar; 16],
    pub tlsext_tick_aes_key: [libc::c_uchar; 16],
    pub tlsext_ticket_key_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_uchar,
            *mut libc::c_uchar,
            *mut EVP_CIPHER_CTX,
            *mut HMAC_CTX,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tlsext_status_cb: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void) -> libc::c_int,
    >,
    pub tlsext_status_arg: *mut libc::c_void,
    pub tlsext_opaque_prf_input_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_void,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tlsext_opaque_prf_input_callback_arg: *mut libc::c_void,
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_client_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub psk_server_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub freelist_max_len: libc::c_uint,
    pub wbuf_freelist: *mut ssl3_buf_freelist_st,
    pub rbuf_freelist: *mut ssl3_buf_freelist_st,
    pub srp_ctx: SRP_CTX,
    pub next_protos_advertised_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut *const libc::c_uchar,
            *mut libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub next_protos_advertised_cb_arg: *mut libc::c_void,
    pub next_proto_select_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut *mut libc::c_uchar,
            *mut libc::c_uchar,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub next_proto_select_cb_arg: *mut libc::c_void,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
}
pub type HMAC_CTX = hmac_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_ctx_st {
    pub md: *const EVP_MD,
    pub md_ctx: EVP_MD_CTX,
    pub i_ctx: EVP_MD_CTX,
    pub o_ctx: EVP_MD_CTX,
    pub key_length: libc::c_uint,
    pub key: [libc::c_uchar; 128],
}
pub type GEN_SESSION_CB = Option::<
    unsafe extern "C" fn(
        *const SSL,
        *mut libc::c_uchar,
        *mut libc::c_uint,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_NAME {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_SSL_COMP {
    pub stack: _STACK,
}
pub type pem_password_cb = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub sess_connect: libc::c_int,
    pub sess_connect_renegotiate: libc::c_int,
    pub sess_connect_good: libc::c_int,
    pub sess_accept: libc::c_int,
    pub sess_accept_renegotiate: libc::c_int,
    pub sess_accept_good: libc::c_int,
    pub sess_miss: libc::c_int,
    pub sess_timeout: libc::c_int,
    pub sess_cache_full: libc::c_int,
    pub sess_hit: libc::c_int,
    pub sess_cb_hit: libc::c_int,
}
pub type SSL_SESSION = ssl_session_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_session_st {
    pub ssl_version: libc::c_int,
    pub key_arg_length: libc::c_uint,
    pub key_arg: [libc::c_uchar; 8],
    pub master_key_length: libc::c_int,
    pub master_key: [libc::c_uchar; 48],
    pub session_id_length: libc::c_uint,
    pub session_id: [libc::c_uchar; 32],
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_identity: *mut libc::c_char,
    pub not_resumable: libc::c_int,
    pub sess_cert: *mut sess_cert_st,
    pub peer: *mut X509,
    pub verify_result: libc::c_long,
    pub references: libc::c_int,
    pub timeout: libc::c_long,
    pub time: libc::c_long,
    pub compress_meth: libc::c_uint,
    pub cipher: *const SSL_CIPHER,
    pub cipher_id: libc::c_ulong,
    pub ciphers: *mut stack_st_SSL_CIPHER,
    pub ex_data: CRYPTO_EX_DATA,
    pub prev: *mut ssl_session_st,
    pub next: *mut ssl_session_st,
    pub tlsext_hostname: *mut libc::c_char,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_tick: *mut libc::c_uchar,
    pub tlsext_ticklen: size_t,
    pub tlsext_tick_lifetime_hint: libc::c_long,
    pub srp_username: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_SSL_CIPHER {
    pub stack: _STACK,
}
pub type SSL_CIPHER = ssl_cipher_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_cipher_st {
    pub valid: libc::c_int,
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
    pub algorithm_mkey: libc::c_ulong,
    pub algorithm_auth: libc::c_ulong,
    pub algorithm_enc: libc::c_ulong,
    pub algorithm_mac: libc::c_ulong,
    pub algorithm_ssl: libc::c_ulong,
    pub algo_strength: libc::c_ulong,
    pub algorithm2: libc::c_ulong,
    pub strength_bits: libc::c_int,
    pub alg_bits: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lhash_st_SSL_SESSION {
    pub dummy: libc::c_int,
}
pub type SSL_METHOD = ssl_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_method_st {
    pub version: libc::c_int,
    pub ssl_new: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_clear: Option::<unsafe extern "C" fn(*mut SSL) -> ()>,
    pub ssl_free: Option::<unsafe extern "C" fn(*mut SSL) -> ()>,
    pub ssl_accept: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_connect: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_read: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub ssl_peek: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub ssl_write: Option::<
        unsafe extern "C" fn(*mut SSL, *const libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub ssl_shutdown: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_renegotiate: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_renegotiate_check: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_get_message: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_long,
            *mut libc::c_int,
        ) -> libc::c_long,
    >,
    pub ssl_read_bytes: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            *mut libc::c_uchar,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub ssl_write_bytes: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            *const libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub ssl_dispatch_alert: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub ssl_ctx_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL_CTX,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub get_cipher_by_char: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> *const SSL_CIPHER,
    >,
    pub put_cipher_by_char: Option::<
        unsafe extern "C" fn(*const SSL_CIPHER, *mut libc::c_uchar) -> libc::c_int,
    >,
    pub ssl_pending: Option::<unsafe extern "C" fn(*const SSL) -> libc::c_int>,
    pub num_ciphers: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub get_cipher: Option::<unsafe extern "C" fn(libc::c_uint) -> *const SSL_CIPHER>,
    pub get_ssl_method: Option::<
        unsafe extern "C" fn(libc::c_int) -> *const ssl_method_st,
    >,
    pub get_timeout: Option::<unsafe extern "C" fn() -> libc::c_long>,
    pub ssl3_enc: *mut ssl3_enc_method,
    pub ssl_version: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub ssl_callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            Option::<unsafe extern "C" fn() -> ()>,
        ) -> libc::c_long,
    >,
    pub ssl_ctx_callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL_CTX,
            libc::c_int,
            Option::<unsafe extern "C" fn() -> ()>,
        ) -> libc::c_long,
    >,
}
pub type tls_session_secret_cb_fn = Option::<
    unsafe extern "C" fn(
        *mut SSL,
        *mut libc::c_void,
        *mut libc::c_int,
        *mut stack_st_SSL_CIPHER,
        *mut *mut SSL_CIPHER,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type tls_session_ticket_ext_cb_fn = Option::<
    unsafe extern "C" fn(
        *mut SSL,
        *const libc::c_uchar,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type TLS_SESSION_TICKET_EXT = tls_session_ticket_ext_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_session_ticket_ext_st {
    pub length: libc::c_ushort,
    pub data: *mut libc::c_void,
}
pub type X509_EXTENSIONS = stack_st_X509_EXTENSION;
pub type COMP_CTX = comp_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_ctx_st {
    pub meth: *mut COMP_METHOD,
    pub compress_in: libc::c_ulong,
    pub compress_out: libc::c_ulong,
    pub expand_in: libc::c_ulong,
    pub expand_out: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type COMP_METHOD = comp_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub init: Option::<unsafe extern "C" fn(*mut COMP_CTX) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut COMP_CTX) -> ()>,
    pub compress: Option::<
        unsafe extern "C" fn(
            *mut COMP_CTX,
            *mut libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub expand: Option::<
        unsafe extern "C" fn(
            *mut COMP_CTX,
            *mut libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub ctrl: Option::<unsafe extern "C" fn() -> libc::c_long>,
    pub callback_ctrl: Option::<unsafe extern "C" fn() -> libc::c_long>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_state_st {
    pub send_cookie: libc::c_uint,
    pub cookie: [libc::c_uchar; 256],
    pub rcvd_cookie: [libc::c_uchar; 256],
    pub cookie_len: libc::c_uint,
    pub r_epoch: libc::c_ushort,
    pub w_epoch: libc::c_ushort,
    pub bitmap: DTLS1_BITMAP,
    pub next_bitmap: DTLS1_BITMAP,
    pub handshake_write_seq: libc::c_ushort,
    pub next_handshake_write_seq: libc::c_ushort,
    pub handshake_read_seq: libc::c_ushort,
    pub last_write_sequence: [libc::c_uchar; 8],
    pub unprocessed_rcds: record_pqueue,
    pub processed_rcds: record_pqueue,
    pub buffered_messages: pqueue,
    pub sent_messages: pqueue,
    pub buffered_app_data: record_pqueue,
    pub listen: libc::c_uint,
    pub link_mtu: libc::c_uint,
    pub mtu: libc::c_uint,
    pub w_msg_hdr: hm_header_st,
    pub r_msg_hdr: hm_header_st,
    pub timeout: dtls1_timeout_st,
    pub next_timeout: timeval,
    pub timeout_duration: libc::c_ushort,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 12],
    pub handshake_fragment_len: libc::c_uint,
    pub retransmitting: libc::c_uint,
    pub change_cipher_spec_ok: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_timeout_st {
    pub read_timeouts: libc::c_uint,
    pub write_timeouts: libc::c_uint,
    pub num_alerts: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hm_header_st {
    pub type_0: libc::c_uchar,
    pub msg_len: libc::c_ulong,
    pub seq: libc::c_ushort,
    pub frag_off: libc::c_ulong,
    pub frag_len: libc::c_ulong,
    pub is_ccs: libc::c_uint,
    pub saved_retransmit_state: dtls1_retransmit_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_retransmit_state {
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub session: *mut SSL_SESSION,
    pub epoch: libc::c_ushort,
}
pub type record_pqueue = record_pqueue_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record_pqueue_st {
    pub epoch: libc::c_ushort,
    pub q: pqueue,
}
pub type pqueue = *mut _pqueue;
pub type DTLS1_BITMAP = dtls1_bitmap_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_bitmap_st {
    pub map: libc::c_ulong,
    pub max_seq_num: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl3_state_st {
    pub flags: libc::c_long,
    pub delay_buf_pop_ret: libc::c_int,
    pub read_sequence: [libc::c_uchar; 8],
    pub read_mac_secret_size: libc::c_int,
    pub read_mac_secret: [libc::c_uchar; 64],
    pub write_sequence: [libc::c_uchar; 8],
    pub write_mac_secret_size: libc::c_int,
    pub write_mac_secret: [libc::c_uchar; 64],
    pub server_random: [libc::c_uchar; 32],
    pub client_random: [libc::c_uchar; 32],
    pub need_empty_fragments: libc::c_int,
    pub empty_fragment_done: libc::c_int,
    pub init_extra: libc::c_int,
    pub rbuf: SSL3_BUFFER,
    pub wbuf: SSL3_BUFFER,
    pub rrec: SSL3_RECORD,
    pub wrec: SSL3_RECORD,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 4],
    pub handshake_fragment_len: libc::c_uint,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_type: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub handshake_buffer: *mut BIO,
    pub handshake_dgst: *mut *mut EVP_MD_CTX,
    pub change_cipher_spec: libc::c_int,
    pub warn_alert: libc::c_int,
    pub fatal_alert: libc::c_int,
    pub alert_dispatch: libc::c_int,
    pub send_alert: [libc::c_uchar; 2],
    pub renegotiate: libc::c_int,
    pub total_renegotiations: libc::c_int,
    pub num_renegotiations: libc::c_int,
    pub in_read_app_data: libc::c_int,
    pub client_opaque_prf_input: *mut libc::c_void,
    pub client_opaque_prf_input_len: size_t,
    pub server_opaque_prf_input: *mut libc::c_void,
    pub server_opaque_prf_input_len: size_t,
    pub tmp: C2RustUnnamed_18,
    pub previous_client_finished: [libc::c_uchar; 64],
    pub previous_client_finished_len: libc::c_uchar,
    pub previous_server_finished: [libc::c_uchar; 64],
    pub previous_server_finished_len: libc::c_uchar,
    pub send_connection_binding: libc::c_int,
    pub next_proto_neg_seen: libc::c_int,
    pub is_probably_safari: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub cert_verify_md: [libc::c_uchar; 128],
    pub finish_md: [libc::c_uchar; 128],
    pub finish_md_len: libc::c_int,
    pub peer_finish_md: [libc::c_uchar; 128],
    pub peer_finish_md_len: libc::c_int,
    pub message_size: libc::c_ulong,
    pub message_type: libc::c_int,
    pub new_cipher: *const SSL_CIPHER,
    pub dh: *mut DH,
    pub ecdh: *mut EC_KEY,
    pub next_state: libc::c_int,
    pub reuse_message: libc::c_int,
    pub cert_req: libc::c_int,
    pub ctype_num: libc::c_int,
    pub ctype: [libc::c_char; 9],
    pub ca_names: *mut stack_st_X509_NAME,
    pub use_rsa_tmp: libc::c_int,
    pub key_block_length: libc::c_int,
    pub key_block: *mut libc::c_uchar,
    pub new_sym_enc: *const EVP_CIPHER,
    pub new_hash: *const EVP_MD,
    pub new_mac_pkey_type: libc::c_int,
    pub new_mac_secret_size: libc::c_int,
    pub new_compression: *const SSL_COMP,
    pub cert_request: libc::c_int,
}
pub type SSL_COMP = ssl_comp_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_comp_st {
    pub id: libc::c_int,
    pub name: *const libc::c_char,
    pub method: *mut COMP_METHOD,
}
pub type EC_KEY = ec_key_st;
pub type SSL3_RECORD = ssl3_record_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl3_record_st {
    pub type_0: libc::c_int,
    pub length: libc::c_uint,
    pub off: libc::c_uint,
    pub data: *mut libc::c_uchar,
    pub input: *mut libc::c_uchar,
    pub comp: *mut libc::c_uchar,
    pub epoch: libc::c_ulong,
    pub seq_num: [libc::c_uchar; 8],
}
pub type SSL3_BUFFER = ssl3_buffer_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl3_buffer_st {
    pub buf: *mut libc::c_uchar,
    pub len: size_t,
    pub offset: libc::c_int,
    pub left: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl2_state_st {
    pub three_byte_header: libc::c_int,
    pub clear_text: libc::c_int,
    pub escape: libc::c_int,
    pub ssl2_rollback: libc::c_int,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub wpend_off: libc::c_int,
    pub wpend_len: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub rbuf_left: libc::c_int,
    pub rbuf_offs: libc::c_int,
    pub rbuf: *mut libc::c_uchar,
    pub wbuf: *mut libc::c_uchar,
    pub write_ptr: *mut libc::c_uchar,
    pub padding: libc::c_uint,
    pub rlength: libc::c_uint,
    pub ract_data_length: libc::c_int,
    pub wlength: libc::c_uint,
    pub wact_data_length: libc::c_int,
    pub ract_data: *mut libc::c_uchar,
    pub wact_data: *mut libc::c_uchar,
    pub mac_data: *mut libc::c_uchar,
    pub read_key: *mut libc::c_uchar,
    pub write_key: *mut libc::c_uchar,
    pub challenge_length: libc::c_uint,
    pub challenge: [libc::c_uchar; 32],
    pub conn_id_length: libc::c_uint,
    pub conn_id: [libc::c_uchar; 16],
    pub key_material_length: libc::c_uint,
    pub key_material: [libc::c_uchar; 48],
    pub read_sequence: libc::c_ulong,
    pub write_sequence: libc::c_ulong,
    pub tmp: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub conn_id_length: libc::c_uint,
    pub cert_type: libc::c_uint,
    pub cert_length: libc::c_uint,
    pub csl: libc::c_uint,
    pub clear: libc::c_uint,
    pub enc: libc::c_uint,
    pub ccl: [libc::c_uchar; 32],
    pub cipher_spec_length: libc::c_uint,
    pub session_id_length: libc::c_uint,
    pub clen: libc::c_uint,
    pub rlen: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_name_entry_st {
    pub object: *mut ASN1_OBJECT,
    pub value: *mut ASN1_STRING,
    pub set: libc::c_int,
    pub size: libc::c_int,
}
pub type X509_NAME_ENTRY = X509_name_entry_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otherName_st {
    pub type_id: *mut ASN1_OBJECT,
    pub value: *mut ASN1_TYPE,
}
pub type OTHERNAME = otherName_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EDIPartyName_st {
    pub nameAssigner: *mut ASN1_STRING,
    pub partyName: *mut ASN1_STRING,
}
pub type EDIPARTYNAME = EDIPartyName_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GENERAL_NAME_st {
    pub type_0: libc::c_int,
    pub d: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub ptr: *mut libc::c_char,
    pub otherName: *mut OTHERNAME,
    pub rfc822Name: *mut ASN1_IA5STRING,
    pub dNSName: *mut ASN1_IA5STRING,
    pub x400Address: *mut ASN1_TYPE,
    pub directoryName: *mut X509_NAME,
    pub ediPartyName: *mut EDIPARTYNAME,
    pub uniformResourceIdentifier: *mut ASN1_IA5STRING,
    pub iPAddress: *mut ASN1_OCTET_STRING,
    pub registeredID: *mut ASN1_OBJECT,
    pub ip: *mut ASN1_OCTET_STRING,
    pub dirn: *mut X509_NAME,
    pub ia5: *mut ASN1_IA5STRING,
    pub rid: *mut ASN1_OBJECT,
    pub other: *mut ASN1_TYPE,
}
pub type GENERAL_NAME = GENERAL_NAME_st;
pub type ev_tstamp = libc::c_double;
pub type C2RustUnnamed_21 = libc::c_int;
pub const EV_ERROR: C2RustUnnamed_21 = -2147483648;
pub const EV_CUSTOM: C2RustUnnamed_21 = 16777216;
pub const EV_ASYNC: C2RustUnnamed_21 = 524288;
pub const EV_CLEANUP: C2RustUnnamed_21 = 262144;
pub const EV_FORK: C2RustUnnamed_21 = 131072;
pub const EV_EMBED: C2RustUnnamed_21 = 65536;
pub const EV_CHECK: C2RustUnnamed_21 = 32768;
pub const EV_PREPARE: C2RustUnnamed_21 = 16384;
pub const EV_IDLE: C2RustUnnamed_21 = 8192;
pub const EV_STAT: C2RustUnnamed_21 = 4096;
pub const EV_CHILD: C2RustUnnamed_21 = 2048;
pub const EV_SIGNAL: C2RustUnnamed_21 = 1024;
pub const EV_PERIODIC: C2RustUnnamed_21 = 512;
pub const EV_TIMEOUT: C2RustUnnamed_21 = 256;
pub const EV_TIMER: C2RustUnnamed_21 = 256;
pub const EV_IO: C2RustUnnamed_21 = 1;
pub const EV__IOFDSET: C2RustUnnamed_21 = 128;
pub const EV_WRITE: C2RustUnnamed_21 = 2;
pub const EV_READ: C2RustUnnamed_21 = 1;
pub const EV_NONE: C2RustUnnamed_21 = 0;
pub const EV_UNDEF: C2RustUnnamed_21 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_list {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, libc::c_int) -> (),
    >,
    pub next: *mut ev_watcher_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_time {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_time, libc::c_int) -> (),
    >,
    pub at: ev_tstamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: libc::c_int,
    pub events: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_timer {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
    >,
    pub at: ev_tstamp,
    pub repeat: ev_tstamp,
}
pub type C2RustUnnamed_22 = libc::c_uint;
pub const EVFLAG_NOTIMERFD: C2RustUnnamed_22 = 8388608;
pub const EVFLAG_NOSIGMASK: C2RustUnnamed_22 = 4194304;
pub const EVFLAG_SIGNALFD: C2RustUnnamed_22 = 2097152;
pub const EVFLAG_NOSIGFD: C2RustUnnamed_22 = 0;
pub const EVFLAG_NOINOTIFY: C2RustUnnamed_22 = 1048576;
pub const EVFLAG_FORKCHECK: C2RustUnnamed_22 = 33554432;
pub const EVFLAG_NOENV: C2RustUnnamed_22 = 16777216;
pub const EVFLAG_AUTO: C2RustUnnamed_22 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufent {
    pub data: [libc::c_char; 32768],
    pub ptr: *mut libc::c_char,
    pub left: size_t,
    pub next: *mut bufent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuffer {
    pub slots: [bufent; 3],
    pub head: *mut bufent,
    pub tail: *mut bufent,
    pub used: size_t,
}
pub type ENC_TYPE = libc::c_uint;
pub const ENC_SSL: ENC_TYPE = 1;
pub const ENC_TLS: ENC_TYPE = 0;
pub type PROXY_MODE = libc::c_uint;
pub const SSL_CLIENT: PROXY_MODE = 1;
pub const SSL_SERVER: PROXY_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cert_files {
    pub CERT_FILE: *mut libc::c_char,
    pub NEXT: *mut cert_files,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __stud_config {
    pub ETYPE: ENC_TYPE,
    pub PMODE: PROXY_MODE,
    pub WRITE_IP_OCTET: libc::c_int,
    pub WRITE_PROXY_LINE: libc::c_int,
    pub PROXY_PROXY_LINE: libc::c_int,
    pub CHROOT: *mut libc::c_char,
    pub UID: uid_t,
    pub GID: gid_t,
    pub FRONT_IP: *mut libc::c_char,
    pub FRONT_PORT: *mut libc::c_char,
    pub BACK_IP: *mut libc::c_char,
    pub BACK_PORT: *mut libc::c_char,
    pub NCORES: libc::c_long,
    pub CERT_FILES: *mut cert_files,
    pub CIPHER_SUITE: *mut libc::c_char,
    pub ENGINE: *mut libc::c_char,
    pub BACKLOG: libc::c_int,
    pub QUIET: libc::c_int,
    pub SYSLOG: libc::c_int,
    pub SYSLOG_FACILITY: libc::c_int,
    pub TCP_KEEPALIVE_TIME: libc::c_int,
    pub DAEMONIZE: libc::c_int,
    pub PREFER_SERVER_CIPHERS: libc::c_int,
}
pub type stud_config = __stud_config;
pub type _SHUTDOWN_REQUESTOR = libc::c_uint;
pub const SHUTDOWN_SSL: _SHUTDOWN_REQUESTOR = 2;
pub const SHUTDOWN_CLEAR: _SHUTDOWN_REQUESTOR = 1;
pub const SHUTDOWN_HARD: _SHUTDOWN_REQUESTOR = 0;
pub type SHUTDOWN_REQUESTOR = _SHUTDOWN_REQUESTOR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctx_list {
    pub servername: *mut libc::c_char,
    pub ctx: *mut SSL_CTX,
    pub next: *mut ctx_list,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct proxystate {
    pub ring_ssl2clear: ringbuffer,
    pub ring_clear2ssl: ringbuffer,
    pub ev_r_ssl: ev_io,
    pub ev_w_ssl: ev_io,
    pub ev_r_handshake: ev_io,
    pub ev_w_handshake: ev_io,
    pub ev_w_connect: ev_io,
    pub ev_r_clear: ev_io,
    pub ev_w_clear: ev_io,
    pub ev_proxy: ev_io,
    pub fd_up: libc::c_int,
    pub fd_down: libc::c_int,
    #[bitfield(name = "want_shutdown", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "handshaked", ty = "libc::c_int", bits = "1..=1")]
    #[bitfield(name = "clear_connected", ty = "libc::c_int", bits = "2..=2")]
    #[bitfield(name = "renegotiation", ty = "libc::c_int", bits = "3..=3")]
    pub want_shutdown_handshaked_clear_connected_renegotiation: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub ssl: *mut SSL,
    pub remote_ip: sockaddr_storage,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn ev_loop(mut loop_1: *mut ev_loop, mut flags: libc::c_int) {
    ev_run(loop_1, flags);
}
static mut loop_0: *mut ev_loop = 0 as *const ev_loop as *mut ev_loop;
static mut backaddr: *mut addrinfo = 0 as *const addrinfo as *mut addrinfo;
static mut master_pid: pid_t = 0;
static mut listener: ev_io = ev_io {
    active: 0,
    pending: 0,
    priority: 0,
    data: 0 as *const libc::c_void as *mut libc::c_void,
    cb: None,
    next: 0 as *const ev_watcher_list as *mut ev_watcher_list,
    fd: 0,
    events: 0,
};
static mut listener_socket: libc::c_int = 0;
static mut child_num: libc::c_int = 0;
static mut child_pids: *mut pid_t = 0 as *const pid_t as *mut pid_t;
static mut default_ctx: *mut SSL_CTX = 0 as *const SSL_CTX as *mut SSL_CTX;
static mut client_session: *mut SSL_SESSION = 0 as *const SSL_SESSION
    as *mut SSL_SESSION;
pub static mut openssl_version: libc::c_long = 0;
pub static mut create_workers: libc::c_int = 0;
pub static mut CONFIG: *mut stud_config = 0 as *const stud_config as *mut stud_config;
static mut tcp_proxy_line: [libc::c_char; 128] = unsafe {
    *::std::mem::transmute::<
        &[u8; 128],
        &mut [libc::c_char; 128],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut sni_ctxs: *mut ctx_list = 0 as *const ctx_list as *mut ctx_list;
unsafe extern "C" fn setnonblocking(mut fd: libc::c_int) {
    let mut flag: libc::c_int = 1 as libc::c_int;
    if ioctl(fd, 0x5421 as libc::c_int as libc::c_ulong, &mut flag as *mut libc::c_int)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"ioctl(fd, FIONBIO, &flag) == 0\0" as *const u8 as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void setnonblocking(int)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn settcpkeepalive(mut fd: libc::c_int) {
    let mut optval: libc::c_int = 1 as libc::c_int;
    let mut optlen: socklen_t = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        as socklen_t;
    if setsockopt(
        fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut optval as *mut libc::c_int as *const libc::c_void,
        optlen,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error activating SO_KEEPALIVE on client socket: %s\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Error activating SO_KEEPALIVE on client socket: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    }
    optval = (*CONFIG).TCP_KEEPALIVE_TIME;
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    if setsockopt(
        fd,
        6 as libc::c_int,
        4 as libc::c_int,
        &mut optval as *mut libc::c_int as *const libc::c_void,
        optlen,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error setting TCP_KEEPIDLE on client socket: %s\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Error setting TCP_KEEPIDLE on client socket: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn fail(mut s: *const libc::c_char) {
    perror(s);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn die(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, fmt, args_0.as_va_list());
    exit(1 as libc::c_int);
}
unsafe extern "C" fn init_dh(
    mut ctx: *mut SSL_CTX,
    mut cert: *const libc::c_char,
) -> libc::c_int {
    let mut dh: *mut DH = 0 as *mut DH;
    let mut bio: *mut BIO = 0 as *mut BIO;
    if !cert.is_null() {} else {
        __assert_fail(
            b"cert\0" as *const u8 as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int init_dh(SSL_CTX *, const char *)\0"))
                .as_ptr(),
        );
    };
    bio = BIO_new_file(cert, b"r\0" as *const u8 as *const libc::c_char);
    if bio.is_null() {
        ERR_print_errors_fp(stderr);
        return -(1 as libc::c_int);
    }
    dh = PEM_read_bio_DHparams(bio, 0 as *mut *mut DH, None, 0 as *mut libc::c_void);
    BIO_free(bio);
    if dh.is_null() {
        fprintf(
            stderr,
            b"{core} Note: no DH parameters found in %s\n\0" as *const u8
                as *const libc::c_char,
            cert,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Note: no DH parameters found in %s\n\0" as *const u8
                    as *const libc::c_char,
                cert,
            );
        }
        return -(1 as libc::c_int);
    }
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} Using DH parameters from %s\n\0" as *const u8
                as *const libc::c_char,
            cert,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} Using DH parameters from %s\n\0" as *const u8
                as *const libc::c_char,
            cert,
        );
    }
    SSL_CTX_ctrl(
        ctx,
        3 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        dh as *mut libc::c_char as *mut libc::c_void,
    );
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} DH initialized with %d bit key\n\0" as *const u8
                as *const libc::c_char,
            8 as libc::c_int * DH_size(dh),
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} DH initialized with %d bit key\n\0" as *const u8
                as *const libc::c_char,
            8 as libc::c_int * DH_size(dh),
        );
    }
    DH_free(dh);
    let mut ecdh: *mut EC_KEY = 0 as *mut EC_KEY;
    ecdh = EC_KEY_new_by_curve_name(415 as libc::c_int);
    SSL_CTX_ctrl(
        ctx,
        4 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        ecdh as *mut libc::c_char as *mut libc::c_void,
    );
    EC_KEY_free(ecdh);
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} ECDH Initialized with NIST P-256\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} ECDH Initialized with NIST P-256\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn info_callback(
    mut ssl: *const SSL,
    mut where_0: libc::c_int,
    mut ret: libc::c_int,
) {
    if where_0 & 0x10 as libc::c_int != 0 {
        let mut ps: *mut proxystate = SSL_get_ex_data(ssl, 0 as libc::c_int)
            as *mut proxystate;
        if (*ps).handshaked() != 0 {
            (*ps).set_renegotiation(1 as libc::c_int);
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{core} SSL renegotiation asked by client\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{core} SSL renegotiation asked by client\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
}
pub unsafe extern "C" fn load_rsa_privatekey(
    mut ctx: *mut SSL_CTX,
    mut file: *const libc::c_char,
) -> *mut RSA {
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    bio = BIO_new_file(file, b"r\0" as *const u8 as *const libc::c_char);
    if bio.is_null() {
        ERR_print_errors_fp(stderr);
        return 0 as *mut RSA;
    }
    rsa = PEM_read_bio_RSAPrivateKey(
        bio,
        0 as *mut *mut RSA,
        (*ctx).default_passwd_callback,
        (*ctx).default_passwd_callback_userdata,
    );
    BIO_free(bio);
    return rsa;
}
pub unsafe extern "C" fn sni_switch_ctx(
    mut ssl: *mut SSL,
    mut al: *mut libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut servername: *const libc::c_char = 0 as *const libc::c_char;
    let mut cl: *const ctx_list = 0 as *const ctx_list;
    servername = SSL_get_servername(ssl, 0 as libc::c_int);
    if servername.is_null() {
        return 3 as libc::c_int;
    }
    cl = sni_ctxs;
    while !cl.is_null() {
        if strcasecmp(servername, (*cl).servername) == 0 as libc::c_int {
            SSL_set_SSL_CTX(ssl, (*cl).ctx);
            return 3 as libc::c_int;
        }
        cl = (*cl).next;
    }
    return 3 as libc::c_int;
}
pub unsafe extern "C" fn make_ctx(mut pemfile: *const libc::c_char) -> *mut SSL_CTX {
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    let mut ssloptions: libc::c_long = 0x1000000 as libc::c_long
        | 0x80000bff as libc::c_long | 0x10000 as libc::c_long;
    ssloptions |= 0x20000 as libc::c_long;
    if (*CONFIG).ETYPE as libc::c_uint == ENC_TLS as libc::c_int as libc::c_uint {
        ctx = SSL_CTX_new(
            if (*CONFIG).PMODE as libc::c_uint
                == SSL_CLIENT as libc::c_int as libc::c_uint
            {
                TLSv1_client_method()
            } else {
                TLSv1_server_method()
            },
        );
    } else if (*CONFIG).ETYPE as libc::c_uint == ENC_SSL as libc::c_int as libc::c_uint {
        ctx = SSL_CTX_new(
            if (*CONFIG).PMODE as libc::c_uint
                == SSL_CLIENT as libc::c_int as libc::c_uint
            {
                SSLv23_client_method()
            } else {
                SSLv23_server_method()
            },
        );
    } else {
        if (*CONFIG).ETYPE as libc::c_uint == ENC_TLS as libc::c_int as libc::c_uint
            || (*CONFIG).ETYPE as libc::c_uint == ENC_SSL as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"CONFIG->ETYPE == ENC_TLS || CONFIG->ETYPE == ENC_SSL\0" as *const u8
                    as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                607 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"SSL_CTX *make_ctx(const char *)\0"))
                    .as_ptr(),
            );
        };
        return 0 as *mut SSL_CTX;
    }
    SSL_CTX_ctrl(ctx, 32 as libc::c_int, ssloptions, 0 as *mut libc::c_void);
    SSL_CTX_set_info_callback(
        ctx,
        Some(
            info_callback
                as unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> (),
        ),
    );
    if !((*CONFIG).CIPHER_SUITE).is_null() {
        if SSL_CTX_set_cipher_list(ctx, (*CONFIG).CIPHER_SUITE) != 1 as libc::c_int {
            ERR_print_errors_fp(stderr);
        }
    }
    if (*CONFIG).PREFER_SERVER_CIPHERS != 0 {
        SSL_CTX_ctrl(
            ctx,
            32 as libc::c_int,
            0x400000 as libc::c_long,
            0 as *mut libc::c_void,
        );
    }
    if (*CONFIG).PMODE as libc::c_uint == SSL_CLIENT as libc::c_int as libc::c_uint {
        return ctx;
    }
    if SSL_CTX_use_certificate_chain_file(ctx, pemfile) <= 0 as libc::c_int {
        ERR_print_errors_fp(stderr);
        exit(1 as libc::c_int);
    }
    rsa = load_rsa_privatekey(ctx, pemfile);
    if rsa.is_null() {
        fprintf(
            stderr,
            b"Error loading rsa private key\n\0" as *const u8 as *const libc::c_char,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Error loading rsa private key\n\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    if SSL_CTX_use_RSAPrivateKey(ctx, rsa) <= 0 as libc::c_int {
        ERR_print_errors_fp(stderr);
        exit(1 as libc::c_int);
    }
    init_dh(ctx, pemfile);
    if SSL_CTX_callback_ctrl(
        ctx,
        53 as libc::c_int,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SSL,
                    *mut libc::c_int,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                sni_switch_ctx
                    as unsafe extern "C" fn(
                        *mut SSL,
                        *mut libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    ) == 0
    {
        fprintf(
            stderr,
            b"Error setting up SNI support\n\0" as *const u8 as *const libc::c_char,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Error setting up SNI support\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    RSA_free(rsa);
    return ctx;
}
pub unsafe extern "C" fn init_openssl() {
    SSL_library_init();
    SSL_load_error_strings();
    if !((*CONFIG).CERT_FILES).is_null() {} else {
        __assert_fail(
            b"CONFIG->CERT_FILES != NULL\0" as *const u8 as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            688 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"void init_openssl()\0"))
                .as_ptr(),
        );
    };
    default_ctx = make_ctx((*(*CONFIG).CERT_FILES).CERT_FILE);
    let mut cf: *mut cert_files = 0 as *mut cert_files;
    let mut i: libc::c_int = 0;
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut x509: *mut X509 = 0 as *mut X509;
    let mut f: *mut BIO = 0 as *mut BIO;
    let mut names: *mut stack_st_GENERAL_NAME = 0 as *mut stack_st_GENERAL_NAME;
    let mut name: *mut GENERAL_NAME = 0 as *mut GENERAL_NAME;
    cf = (*(*CONFIG).CERT_FILES).NEXT;
    while !cf.is_null() {
        ctx = make_ctx((*cf).CERT_FILE);
        f = BIO_new(BIO_s_file());
        if BIO_ctrl(
            f,
            108 as libc::c_int,
            (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_long,
            (*cf).CERT_FILE as *mut libc::c_void,
        ) == 0
        {
            fprintf(
                stderr,
                b"Could not read cert '%s'\n\0" as *const u8 as *const libc::c_char,
                (*cf).CERT_FILE,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"Could not read cert '%s'\n\0" as *const u8 as *const libc::c_char,
                    (*cf).CERT_FILE,
                );
            }
        }
        x509 = PEM_read_bio_X509_AUX(
            f,
            0 as *mut *mut X509,
            None,
            0 as *mut libc::c_void,
        );
        BIO_free(f);
        names = X509_get_ext_d2i(
            x509,
            85 as libc::c_int,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
        ) as *mut stack_st_GENERAL_NAME;
        i = 0 as libc::c_int;
        while i
            < sk_num(
                (if 1 as libc::c_int != 0 {
                    names
                } else {
                    0 as *mut stack_st_GENERAL_NAME
                }) as *mut _STACK,
            )
        {
            name = sk_value(
                (if 1 as libc::c_int != 0 {
                    names
                } else {
                    0 as *mut stack_st_GENERAL_NAME
                }) as *mut _STACK,
                i,
            ) as *mut GENERAL_NAME;
            if (*name).type_0 == 2 as libc::c_int {
                let mut cl: *mut ctx_list = 0 as *mut ctx_list;
                cl = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<ctx_list>() as libc::c_ulong,
                ) as *mut ctx_list;
                ASN1_STRING_to_UTF8(
                    &mut (*cl).servername as *mut *mut libc::c_char
                        as *mut *mut libc::c_uchar,
                    (*name).d.dNSName,
                );
                (*cl).ctx = ctx;
                (*cl).next = sni_ctxs;
                sni_ctxs = cl;
            }
            i += 1;
            i;
        }
        if sk_num(
            (if 1 as libc::c_int != 0 { names } else { 0 as *mut stack_st_GENERAL_NAME })
                as *mut _STACK,
        ) > 0 as libc::c_int
        {
            sk_pop_free(
                (if 1 as libc::c_int != 0 {
                    names
                } else {
                    0 as *mut stack_st_GENERAL_NAME
                }) as *mut _STACK,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut GENERAL_NAME) -> ()>,
                    Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                >(
                    if 1 as libc::c_int != 0 {
                        Some(
                            GENERAL_NAME_free
                                as unsafe extern "C" fn(*mut GENERAL_NAME) -> (),
                        )
                    } else {
                        None
                    },
                ),
            );
        } else {
            if !names.is_null() {
                sk_pop_free(
                    (if 1 as libc::c_int != 0 {
                        names
                    } else {
                        0 as *mut stack_st_GENERAL_NAME
                    }) as *mut _STACK,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut GENERAL_NAME) -> ()>,
                        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                    >(
                        if 1 as libc::c_int != 0 {
                            Some(
                                GENERAL_NAME_free
                                    as unsafe extern "C" fn(*mut GENERAL_NAME) -> (),
                            )
                        } else {
                            None
                        },
                    ),
                );
            }
            let mut x509_name: *mut X509_NAME = X509_get_subject_name(x509);
            i = X509_NAME_get_index_by_NID(
                x509_name,
                13 as libc::c_int,
                -(1 as libc::c_int),
            );
            if i < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Could not find Subject Alternative Names or a CN on cert %s\n\0"
                        as *const u8 as *const libc::c_char,
                    (*cf).CERT_FILE,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"Could not find Subject Alternative Names or a CN on cert %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*cf).CERT_FILE,
                    );
                }
            }
            let mut x509_entry: *mut X509_NAME_ENTRY = X509_NAME_get_entry(x509_name, i);
            let mut cl_0: *mut ctx_list = 0 as *mut ctx_list;
            cl_0 = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<ctx_list>() as libc::c_ulong,
            ) as *mut ctx_list;
            ASN1_STRING_to_UTF8(
                &mut (*cl_0).servername as *mut *mut libc::c_char
                    as *mut *mut libc::c_uchar,
                (*x509_entry).value,
            );
            (*cl_0).ctx = ctx;
            (*cl_0).next = sni_ctxs;
            sni_ctxs = cl_0;
        }
        cf = (*cf).NEXT;
    }
    if !((*CONFIG).ENGINE).is_null() {
        let mut e: *mut ENGINE = 0 as *mut ENGINE;
        ENGINE_load_builtin_engines();
        if strcmp((*CONFIG).ENGINE, b"auto\0" as *const u8 as *const libc::c_char) == 0 {
            ENGINE_register_all_complete();
        } else {
            e = ENGINE_by_id((*CONFIG).ENGINE);
            if e.is_null() || ENGINE_init(e) == 0
                || ENGINE_set_default(e, 0xffff as libc::c_int as libc::c_uint) == 0
            {
                ERR_print_errors_fp(stderr);
                exit(1 as libc::c_int);
            }
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{core} will use OpenSSL engine %s.\n\0" as *const u8
                        as *const libc::c_char,
                    ENGINE_get_id(e),
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{core} will use OpenSSL engine %s.\n\0" as *const u8
                        as *const libc::c_char,
                    ENGINE_get_id(e),
                );
            }
            ENGINE_finish(e);
            ENGINE_free(e);
        }
    }
}
unsafe extern "C" fn prepare_proxy_line(mut ai_addr: *mut sockaddr) {
    tcp_proxy_line[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    let mut tcp6_address_string: [libc::c_char; 46] = [0; 46];
    if (*ai_addr).sa_family as libc::c_int == 2 as libc::c_int {
        let mut addr: *mut sockaddr_in = ai_addr as *mut sockaddr_in;
        let mut res: size_t = snprintf(
            tcp_proxy_line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"PROXY %%s %%s %s %%hu %hu\r\n\0" as *const u8 as *const libc::c_char,
            inet_ntoa((*addr).sin_addr),
            __bswap_16((*addr).sin_port) as libc::c_int,
        ) as size_t;
        if res < ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {} else {
            __assert_fail(
                b"res < sizeof(tcp_proxy_line)\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                788 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void prepare_proxy_line(struct sockaddr *)\0"))
                    .as_ptr(),
            );
        };
    } else if (*ai_addr).sa_family as libc::c_int == 10 as libc::c_int {
        let mut addr_0: *mut sockaddr_in6 = ai_addr as *mut sockaddr_in6;
        inet_ntop(
            10 as libc::c_int,
            &mut (*addr_0).sin6_addr as *mut in6_addr as *const libc::c_void,
            tcp6_address_string.as_mut_ptr(),
            46 as libc::c_int as socklen_t,
        );
        let mut res_0: size_t = snprintf(
            tcp_proxy_line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"PROXY %%s %%s %s %%hu %hu\r\n\0" as *const u8 as *const libc::c_char,
            tcp6_address_string.as_mut_ptr(),
            __bswap_16((*addr_0).sin6_port) as libc::c_int,
        ) as size_t;
        if res_0 < ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"res < sizeof(tcp_proxy_line)\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                798 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void prepare_proxy_line(struct sockaddr *)\0"))
                    .as_ptr(),
            );
        };
    } else {
        fprintf(
            stderr,
            b"The --write-proxy mode is not implemented for this address family.\n\0"
                as *const u8 as *const libc::c_char,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"The --write-proxy mode is not implemented for this address family.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn create_main_socket() -> libc::c_int {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
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
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int | 0x20 as libc::c_int;
    let gai_err: libc::c_int = getaddrinfo(
        (*CONFIG).FRONT_IP,
        (*CONFIG).FRONT_PORT,
        &mut hints,
        &mut ai,
    );
    if gai_err != 0 as libc::c_int {
        fprintf(
            stderr,
            b"{getaddrinfo}: [%s]\n\0" as *const u8 as *const libc::c_char,
            gai_strerror(gai_err),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{getaddrinfo}: [%s]\n\0" as *const u8 as *const libc::c_char,
                gai_strerror(gai_err),
            );
        }
        exit(1 as libc::c_int);
    }
    let mut s: libc::c_int = socket(
        (*ai).ai_family,
        SOCK_STREAM as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if s == -(1 as libc::c_int) {
        fail(b"{socket: main}\0" as *const u8 as *const libc::c_char);
    }
    let mut t: libc::c_int = 1 as libc::c_int;
    setsockopt(
        s,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut t as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    setsockopt(
        s,
        1 as libc::c_int,
        15 as libc::c_int,
        &mut t as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    setnonblocking(s);
    if bind(
        s,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*ai).ai_addr,
        },
        (*ai).ai_addrlen,
    ) != 0
    {
        fail(b"{bind-socket}\0" as *const u8 as *const libc::c_char);
    }
    let mut timeout: libc::c_int = 1 as libc::c_int;
    setsockopt(
        s,
        IPPROTO_TCP as libc::c_int,
        9 as libc::c_int,
        &mut timeout as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    prepare_proxy_line((*ai).ai_addr);
    freeaddrinfo(ai);
    listen(s, (*CONFIG).BACKLOG);
    return s;
}
unsafe extern "C" fn create_back_socket() -> libc::c_int {
    let mut s: libc::c_int = socket(
        (*backaddr).ai_family,
        SOCK_STREAM as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    let mut flag: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = setsockopt(
        s,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut flag as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if ret == -(1 as libc::c_int) {
        perror(
            b"Couldn't setsockopt to backend (TCP_NODELAY)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    setnonblocking(s);
    return s;
}
unsafe extern "C" fn safe_enable_io(mut ps: *mut proxystate, mut w: *mut ev_io) {
    if (*ps).want_shutdown() == 0 {
        ev_io_start(loop_0, w);
    }
}
unsafe extern "C" fn shutdown_proxy(
    mut ps: *mut proxystate,
    mut req: SHUTDOWN_REQUESTOR,
) {
    if (*ps).want_shutdown() != 0
        || req as libc::c_uint == SHUTDOWN_HARD as libc::c_int as libc::c_uint
    {
        ev_io_stop(loop_0, &mut (*ps).ev_w_ssl);
        ev_io_stop(loop_0, &mut (*ps).ev_r_ssl);
        ev_io_stop(loop_0, &mut (*ps).ev_w_handshake);
        ev_io_stop(loop_0, &mut (*ps).ev_r_handshake);
        ev_io_stop(loop_0, &mut (*ps).ev_w_connect);
        ev_io_stop(loop_0, &mut (*ps).ev_w_clear);
        ev_io_stop(loop_0, &mut (*ps).ev_r_clear);
        ev_io_stop(loop_0, &mut (*ps).ev_proxy);
        close((*ps).fd_up);
        close((*ps).fd_down);
        SSL_set_shutdown((*ps).ssl, 1 as libc::c_int);
        SSL_free((*ps).ssl);
        free(ps as *mut libc::c_void);
    } else {
        (*ps).set_want_shutdown(1 as libc::c_int);
        if req as libc::c_uint == SHUTDOWN_CLEAR as libc::c_int as libc::c_uint
            && ringbuffer_is_empty(&mut (*ps).ring_clear2ssl) != 0
        {
            shutdown_proxy(ps, SHUTDOWN_HARD);
        } else if req as libc::c_uint == SHUTDOWN_SSL as libc::c_int as libc::c_uint
            && ringbuffer_is_empty(&mut (*ps).ring_ssl2clear) != 0
        {
            shutdown_proxy(ps, SHUTDOWN_HARD);
        }
    };
}
unsafe extern "C" fn handle_socket_errno(
    mut ps: *mut proxystate,
    mut backend: libc::c_int,
) {
    if *__errno_location() == 11 as libc::c_int
        || *__errno_location() == 11 as libc::c_int
        || *__errno_location() == 4 as libc::c_int
    {
        return;
    }
    if *__errno_location() == 104 as libc::c_int {
        fprintf(
            stderr,
            b"{%s} Connection reset by peer\n\0" as *const u8 as *const libc::c_char,
            if backend != 0 {
                b"backend\0" as *const u8 as *const libc::c_char
            } else {
                b"client\0" as *const u8 as *const libc::c_char
            },
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{%s} Connection reset by peer\n\0" as *const u8 as *const libc::c_char,
                if backend != 0 {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else if *__errno_location() == 110 as libc::c_int {
        fprintf(
            stderr,
            b"{%s} Connection to backend timed out\n\0" as *const u8
                as *const libc::c_char,
            if backend != 0 {
                b"backend\0" as *const u8 as *const libc::c_char
            } else {
                b"client\0" as *const u8 as *const libc::c_char
            },
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{%s} Connection to backend timed out\n\0" as *const u8
                    as *const libc::c_char,
                if backend != 0 {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else if *__errno_location() == 32 as libc::c_int {
        fprintf(
            stderr,
            b"{%s} Broken pipe to backend (EPIPE)\n\0" as *const u8
                as *const libc::c_char,
            if backend != 0 {
                b"backend\0" as *const u8 as *const libc::c_char
            } else {
                b"client\0" as *const u8 as *const libc::c_char
            },
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{%s} Broken pipe to backend (EPIPE)\n\0" as *const u8
                    as *const libc::c_char,
                if backend != 0 {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else {
        perror(b"{backend} [errno]\0" as *const u8 as *const libc::c_char);
    }
    shutdown_proxy(ps, SHUTDOWN_CLEAR);
}
unsafe extern "C" fn start_connect(mut ps: *mut proxystate) {
    let mut t: libc::c_int = 1 as libc::c_int;
    t = connect(
        (*ps).fd_down,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*backaddr).ai_addr,
        },
        (*backaddr).ai_addrlen,
    );
    if t == 0 as libc::c_int || *__errno_location() == 115 as libc::c_int
        || *__errno_location() == 4 as libc::c_int
    {
        ev_io_start(loop_0, &mut (*ps).ev_w_connect);
        return;
    }
    perror(b"{backend-connect}\0" as *const u8 as *const libc::c_char);
    shutdown_proxy(ps, SHUTDOWN_HARD);
}
unsafe extern "C" fn clear_read(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    if (*ps).want_shutdown() != 0 {
        ev_io_stop(loop_1, &mut (*ps).ev_r_clear);
        return;
    }
    let mut fd: libc::c_int = (*w).fd;
    let mut buf: *mut libc::c_char = ringbuffer_write_ptr(&mut (*ps).ring_clear2ssl);
    t = recv(
        fd,
        buf as *mut libc::c_void,
        (1024 as libc::c_int * 32 as libc::c_int) as size_t,
        0 as libc::c_int,
    ) as libc::c_int;
    if t > 0 as libc::c_int {
        ringbuffer_write_append(&mut (*ps).ring_clear2ssl, t);
        if ringbuffer_is_full(&mut (*ps).ring_clear2ssl) != 0 {
            ev_io_stop(loop_1, &mut (*ps).ev_r_clear);
        }
        if (*ps).handshaked() != 0 {
            safe_enable_io(ps, &mut (*ps).ev_w_ssl);
        }
    } else if t == 0 as libc::c_int {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{%s} Connection closed\n\0" as *const u8 as *const libc::c_char,
                if fd == (*ps).fd_down {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{%s} Connection closed\n\0" as *const u8 as *const libc::c_char,
                if fd == (*ps).fd_down {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
        }
        shutdown_proxy(ps, SHUTDOWN_CLEAR);
    } else {
        if t == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"t == -1\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                960 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void clear_read(struct ev_loop *, ev_io *, int)\0"))
                    .as_ptr(),
            );
        };
        handle_socket_errno(
            ps,
            if fd == (*ps).fd_down { 1 as libc::c_int } else { 0 as libc::c_int },
        );
    };
}
unsafe extern "C" fn clear_write(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    let mut fd: libc::c_int = (*w).fd;
    let mut sz: libc::c_int = 0;
    if ringbuffer_is_empty(&mut (*ps).ring_ssl2clear) == 0 {} else {
        __assert_fail(
            b"!ringbuffer_is_empty(&ps->ring_ssl2clear)\0" as *const u8
                as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void clear_write(struct ev_loop *, ev_io *, int)\0"))
                .as_ptr(),
        );
    };
    let mut next: *mut libc::c_char = ringbuffer_read_next(
        &mut (*ps).ring_ssl2clear,
        &mut sz,
    );
    t = send(fd, next as *const libc::c_void, sz as size_t, MSG_NOSIGNAL as libc::c_int)
        as libc::c_int;
    if t > 0 as libc::c_int {
        if t == sz {
            ringbuffer_read_pop(&mut (*ps).ring_ssl2clear);
            if (*ps).handshaked() != 0 {
                safe_enable_io(ps, &mut (*ps).ev_r_ssl);
            }
            if ringbuffer_is_empty(&mut (*ps).ring_ssl2clear) != 0 {
                if (*ps).want_shutdown() != 0 {
                    shutdown_proxy(ps, SHUTDOWN_HARD);
                    return;
                }
                ev_io_stop(loop_1, &mut (*ps).ev_w_clear);
            }
        } else {
            ringbuffer_read_skip(&mut (*ps).ring_ssl2clear, t);
        }
    } else {
        if t == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"t == -1\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                996 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void clear_write(struct ev_loop *, ev_io *, int)\0"))
                    .as_ptr(),
            );
        };
        handle_socket_errno(
            ps,
            if fd == (*ps).fd_down { 1 as libc::c_int } else { 0 as libc::c_int },
        );
    };
}
unsafe extern "C" fn handle_connect(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    t = connect(
        (*ps).fd_down,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*backaddr).ai_addr,
        },
        (*backaddr).ai_addrlen,
    );
    if t == 0 || *__errno_location() == 106 as libc::c_int || *__errno_location() == 0 {
        ev_io_stop(loop_1, &mut (*ps).ev_w_connect);
        if (*ps).clear_connected() == 0 {
            (*ps).set_clear_connected(1 as libc::c_int);
            if ringbuffer_is_full(&mut (*ps).ring_clear2ssl) == 0 {
                safe_enable_io(ps, &mut (*ps).ev_r_clear);
            }
            if ringbuffer_is_empty(&mut (*ps).ring_ssl2clear) == 0 {
                ev_io_start(loop_1, &mut (*ps).ev_w_clear);
            }
        } else {
            start_handshake(ps, 3 as libc::c_int);
        }
    } else if !(*__errno_location() == 115 as libc::c_int
        || *__errno_location() == 4 as libc::c_int
        || *__errno_location() == 114 as libc::c_int)
    {
        perror(b"{backend-connect}\0" as *const u8 as *const libc::c_char);
        shutdown_proxy(ps, SHUTDOWN_HARD);
    }
}
unsafe extern "C" fn start_handshake(mut ps: *mut proxystate, mut err: libc::c_int) {
    ev_io_stop(loop_0, &mut (*ps).ev_r_ssl);
    ev_io_stop(loop_0, &mut (*ps).ev_w_ssl);
    (*ps).set_handshaked(0 as libc::c_int);
    if err == 2 as libc::c_int {
        ev_io_start(loop_0, &mut (*ps).ev_r_handshake);
    } else if err == 3 as libc::c_int {
        ev_io_start(loop_0, &mut (*ps).ev_w_handshake);
    }
}
unsafe extern "C" fn end_handshake(mut ps: *mut proxystate) {
    let mut tcp6_address_string: [libc::c_char; 46] = [0; 46];
    let mut written: size_t = 0 as libc::c_int as size_t;
    ev_io_stop(loop_0, &mut (*ps).ev_r_handshake);
    ev_io_stop(loop_0, &mut (*ps).ev_w_handshake);
    if !((*(*ps).ssl).s3).is_null() {
        (*(*(*ps).ssl).s3).flags |= 0x1 as libc::c_int as libc::c_long;
    }
    (*ps).set_handshaked(1 as libc::c_int);
    if (*ps).clear_connected() == 0 {
        if (*CONFIG).WRITE_PROXY_LINE != 0 {
            let mut ring_pnt: *mut libc::c_char = ringbuffer_write_ptr(
                &mut (*ps).ring_ssl2clear,
            );
            if (*ps).remote_ip.ss_family as libc::c_int == 2 as libc::c_int
                || (*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int
            {} else {
                __assert_fail(
                    b"ps->remote_ip.ss_family == AF_INET || ps->remote_ip.ss_family == AF_INET6\0"
                        as *const u8 as *const libc::c_char,
                    b"stud.c\0" as *const u8 as *const libc::c_char,
                    1072 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"void end_handshake(proxystate *)\0"))
                        .as_ptr(),
                );
            };
            if (*ps).remote_ip.ss_family as libc::c_int == 2 as libc::c_int {
                let mut addr: *mut sockaddr_in = &mut (*ps).remote_ip
                    as *mut sockaddr_storage as *mut sockaddr_in;
                written = snprintf(
                    ring_pnt,
                    (1024 as libc::c_int * 32 as libc::c_int) as libc::c_ulong,
                    tcp_proxy_line.as_mut_ptr(),
                    b"TCP4\0" as *const u8 as *const libc::c_char,
                    inet_ntoa((*addr).sin_addr),
                    __bswap_16((*addr).sin_port) as libc::c_int,
                ) as size_t;
            } else if (*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int {
                let mut addr_0: *mut sockaddr_in6 = &mut (*ps).remote_ip
                    as *mut sockaddr_storage as *mut sockaddr_in6;
                inet_ntop(
                    10 as libc::c_int,
                    &mut (*addr_0).sin6_addr as *mut in6_addr as *const libc::c_void,
                    tcp6_address_string.as_mut_ptr(),
                    46 as libc::c_int as socklen_t,
                );
                written = snprintf(
                    ring_pnt,
                    (1024 as libc::c_int * 32 as libc::c_int) as libc::c_ulong,
                    tcp_proxy_line.as_mut_ptr(),
                    b"TCP6\0" as *const u8 as *const libc::c_char,
                    tcp6_address_string.as_mut_ptr(),
                    __bswap_16((*addr_0).sin6_port) as libc::c_int,
                ) as size_t;
            }
            ringbuffer_write_append(&mut (*ps).ring_ssl2clear, written as libc::c_int);
        } else if (*CONFIG).WRITE_IP_OCTET != 0 {
            let mut ring_pnt_0: *mut libc::c_char = ringbuffer_write_ptr(
                &mut (*ps).ring_ssl2clear,
            );
            if (*ps).remote_ip.ss_family as libc::c_int == 2 as libc::c_int
                || (*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int
            {} else {
                __assert_fail(
                    b"ps->remote_ip.ss_family == AF_INET || ps->remote_ip.ss_family == AF_INET6\0"
                        as *const u8 as *const libc::c_char,
                    b"stud.c\0" as *const u8 as *const libc::c_char,
                    1097 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"void end_handshake(proxystate *)\0"))
                        .as_ptr(),
                );
            };
            let fresh0 = ring_pnt_0;
            ring_pnt_0 = ring_pnt_0.offset(1);
            *fresh0 = (*ps).remote_ip.ss_family as libc::c_uchar as libc::c_char;
            if (*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int {
                memcpy(
                    ring_pnt_0 as *mut libc::c_void,
                    &mut (*(&mut (*ps).remote_ip as *mut sockaddr_storage
                        as *mut sockaddr_in6))
                        .sin6_addr
                        .__in6_u
                        .__u6_addr8 as *mut [uint8_t; 16] as *const libc::c_void,
                    16 as libc::c_uint as libc::c_ulong,
                );
                ringbuffer_write_append(
                    &mut (*ps).ring_ssl2clear,
                    (1 as libc::c_uint).wrapping_add(16 as libc::c_uint) as libc::c_int,
                );
            } else {
                memcpy(
                    ring_pnt_0 as *mut libc::c_void,
                    &mut (*(&mut (*ps).remote_ip as *mut sockaddr_storage
                        as *mut sockaddr_in))
                        .sin_addr
                        .s_addr as *mut in_addr_t as *const libc::c_void,
                    4 as libc::c_uint as libc::c_ulong,
                );
                ringbuffer_write_append(
                    &mut (*ps).ring_ssl2clear,
                    (1 as libc::c_uint).wrapping_add(4 as libc::c_uint) as libc::c_int,
                );
            }
        }
        start_connect(ps);
    } else if SSL_ctrl(
        (*ps).ssl,
        8 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_void,
    ) == 0
    {
        if !client_session.is_null() {
            SSL_SESSION_free(client_session);
        }
        client_session = SSL_get1_session((*ps).ssl);
    }
    if ringbuffer_is_full(&mut (*ps).ring_ssl2clear) == 0 {
        safe_enable_io(ps, &mut (*ps).ev_r_ssl);
    }
    if ringbuffer_is_empty(&mut (*ps).ring_clear2ssl) == 0 {
        ev_io_start(loop_0, &mut (*ps).ev_w_ssl);
    }
}
unsafe extern "C" fn client_proxy_proxy(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut proxy: *mut libc::c_char = tcp_proxy_line.as_mut_ptr();
    let mut end: *mut libc::c_char = tcp_proxy_line
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as isize);
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    let mut b: *mut BIO = SSL_get_rbio((*ps).ssl);
    while proxy != end
        && {
            t = BIO_read(b, proxy as *mut libc::c_void, 1 as libc::c_int);
            t == 1 as libc::c_int
        }
    {
        let fresh1 = proxy;
        proxy = proxy.offset(1);
        if *fresh1 as libc::c_int == '\n' as i32 {
            break;
        }
    }
    if proxy == end {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{client} Unexpectedly long PROXY line. Perhaps a malformed request?\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{client} Unexpectedly long PROXY line. Perhaps a malformed request?\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        shutdown_proxy(ps, SHUTDOWN_SSL);
    } else if t == 1 as libc::c_int {
        if ringbuffer_is_full(&mut (*ps).ring_ssl2clear) != 0 {
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{client} Error writing PROXY line\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{client} Error writing PROXY line\0" as *const u8
                        as *const libc::c_char,
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
            return;
        }
        let mut ring: *mut libc::c_char = ringbuffer_write_ptr(
            &mut (*ps).ring_ssl2clear,
        );
        memcpy(
            ring as *mut libc::c_void,
            tcp_proxy_line.as_mut_ptr() as *const libc::c_void,
            proxy.offset_from(tcp_proxy_line.as_mut_ptr()) as libc::c_long
                as libc::c_ulong,
        );
        ringbuffer_write_append(
            &mut (*ps).ring_ssl2clear,
            proxy.offset_from(tcp_proxy_line.as_mut_ptr()) as libc::c_long as libc::c_int,
        );
        if *proxy.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\n' as i32 {
            ev_io_stop(loop_1, &mut (*ps).ev_proxy);
            start_handshake(ps, 2 as libc::c_int);
        }
    } else if BIO_test_flags(b, 0x8 as libc::c_int) == 0 {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{client} Unexpected error reading PROXY line\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{client} Unexpected error reading PROXY line\0" as *const u8
                    as *const libc::c_char,
            );
        }
        shutdown_proxy(ps, SHUTDOWN_SSL);
    }
}
unsafe extern "C" fn client_handshake(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    t = SSL_do_handshake((*ps).ssl);
    if t == 1 as libc::c_int {
        end_handshake(ps);
    } else {
        let mut err: libc::c_int = SSL_get_error((*ps).ssl, t);
        if err == 2 as libc::c_int {
            ev_io_stop(loop_1, &mut (*ps).ev_w_handshake);
            ev_io_start(loop_1, &mut (*ps).ev_r_handshake);
        } else if err == 3 as libc::c_int {
            ev_io_stop(loop_1, &mut (*ps).ev_r_handshake);
            ev_io_start(loop_1, &mut (*ps).ev_w_handshake);
        } else if err == 6 as libc::c_int {
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{%s} Connection closed (in handshake)\n\0" as *const u8
                        as *const libc::c_char,
                    if (*w).fd == (*ps).fd_up {
                        b"client\0" as *const u8 as *const libc::c_char
                    } else {
                        b"backend\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{%s} Connection closed (in handshake)\n\0" as *const u8
                        as *const libc::c_char,
                    if (*w).fd == (*ps).fd_up {
                        b"client\0" as *const u8 as *const libc::c_char
                    } else {
                        b"backend\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
        } else {
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{%s} Unexpected SSL error (in handshake): %d\n\0" as *const u8
                        as *const libc::c_char,
                    if (*w).fd == (*ps).fd_up {
                        b"client\0" as *const u8 as *const libc::c_char
                    } else {
                        b"backend\0" as *const u8 as *const libc::c_char
                    },
                    err,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{%s} Unexpected SSL error (in handshake): %d\n\0" as *const u8
                        as *const libc::c_char,
                    if (*w).fd == (*ps).fd_up {
                        b"client\0" as *const u8 as *const libc::c_char
                    } else {
                        b"backend\0" as *const u8 as *const libc::c_char
                    },
                    err,
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
        }
    };
}
unsafe extern "C" fn handle_fatal_ssl_error(
    mut ps: *mut proxystate,
    mut err: libc::c_int,
    mut backend: libc::c_int,
) {
    if err == 6 as libc::c_int {
        fprintf(
            stderr,
            b"{%s} Connection closed (in data)\n\0" as *const u8 as *const libc::c_char,
            if backend != 0 {
                b"backend\0" as *const u8 as *const libc::c_char
            } else {
                b"client\0" as *const u8 as *const libc::c_char
            },
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{%s} Connection closed (in data)\n\0" as *const u8
                    as *const libc::c_char,
                if backend != 0 {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else if err == 5 as libc::c_int {
        if *__errno_location() == 0 as libc::c_int {
            fprintf(
                stderr,
                b"{%s} Connection closed (in data)\n\0" as *const u8
                    as *const libc::c_char,
                if backend != 0 {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"{%s} Connection closed (in data)\n\0" as *const u8
                        as *const libc::c_char,
                    if backend != 0 {
                        b"backend\0" as *const u8 as *const libc::c_char
                    } else {
                        b"client\0" as *const u8 as *const libc::c_char
                    },
                );
            }
        } else {
            perror(
                if backend != 0 {
                    b"{backend} [errno] \0" as *const u8 as *const libc::c_char
                } else {
                    b"{client} [errno] \0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else {
        fprintf(
            stderr,
            b"{%s} Unexpected SSL_read error: %d\n\0" as *const u8
                as *const libc::c_char,
            if backend != 0 {
                b"backend\0" as *const u8 as *const libc::c_char
            } else {
                b"client\0" as *const u8 as *const libc::c_char
            },
            err,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{%s} Unexpected SSL_read error: %d\n\0" as *const u8
                    as *const libc::c_char,
                if backend != 0 {
                    b"backend\0" as *const u8 as *const libc::c_char
                } else {
                    b"client\0" as *const u8 as *const libc::c_char
                },
                err,
            );
        }
    }
    shutdown_proxy(ps, SHUTDOWN_SSL);
}
unsafe extern "C" fn ssl_read(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    if (*ps).want_shutdown() != 0 {
        ev_io_stop(loop_1, &mut (*ps).ev_r_ssl);
        return;
    }
    let mut buf: *mut libc::c_char = ringbuffer_write_ptr(&mut (*ps).ring_ssl2clear);
    t = SSL_read(
        (*ps).ssl,
        buf as *mut libc::c_void,
        1024 as libc::c_int * 32 as libc::c_int,
    );
    if (*ps).renegotiation() != 0 {
        shutdown_proxy(ps, SHUTDOWN_SSL);
        return;
    }
    if t > 0 as libc::c_int {
        ringbuffer_write_append(&mut (*ps).ring_ssl2clear, t);
        if ringbuffer_is_full(&mut (*ps).ring_ssl2clear) != 0 {
            ev_io_stop(loop_1, &mut (*ps).ev_r_ssl);
        }
        if (*ps).clear_connected() != 0 {
            safe_enable_io(ps, &mut (*ps).ev_w_clear);
        }
    } else {
        let mut err: libc::c_int = SSL_get_error((*ps).ssl, t);
        if err == 3 as libc::c_int {
            start_handshake(ps, err);
        } else if !(err == 2 as libc::c_int) {
            handle_fatal_ssl_error(
                ps,
                err,
                if (*w).fd == (*ps).fd_up { 0 as libc::c_int } else { 1 as libc::c_int },
            );
        }
    };
}
unsafe extern "C" fn ssl_write(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut ps: *mut proxystate = (*w).data as *mut proxystate;
    if ringbuffer_is_empty(&mut (*ps).ring_clear2ssl) == 0 {} else {
        __assert_fail(
            b"!ringbuffer_is_empty(&ps->ring_clear2ssl)\0" as *const u8
                as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            1264 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void ssl_write(struct ev_loop *, ev_io *, int)\0"))
                .as_ptr(),
        );
    };
    let mut next: *mut libc::c_char = ringbuffer_read_next(
        &mut (*ps).ring_clear2ssl,
        &mut sz,
    );
    t = SSL_write((*ps).ssl, next as *const libc::c_void, sz);
    if t > 0 as libc::c_int {
        if t == sz {
            ringbuffer_read_pop(&mut (*ps).ring_clear2ssl);
            if (*ps).clear_connected() != 0 {
                safe_enable_io(ps, &mut (*ps).ev_r_clear);
            }
            if ringbuffer_is_empty(&mut (*ps).ring_clear2ssl) != 0 {
                if (*ps).want_shutdown() != 0 {
                    shutdown_proxy(ps, SHUTDOWN_HARD);
                    return;
                }
                ev_io_stop(loop_1, &mut (*ps).ev_w_ssl);
            }
        } else {
            ringbuffer_read_skip(&mut (*ps).ring_clear2ssl, t);
        }
    } else {
        let mut err: libc::c_int = SSL_get_error((*ps).ssl, t);
        if err == 2 as libc::c_int {
            start_handshake(ps, err);
        } else if !(err == 3 as libc::c_int) {
            handle_fatal_ssl_error(
                ps,
                err,
                if (*w).fd == (*ps).fd_up { 0 as libc::c_int } else { 1 as libc::c_int },
            );
        }
    };
}
unsafe extern "C" fn handle_accept(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sl: socklen_t = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong
        as socklen_t;
    let mut client: libc::c_int = accept(
        (*w).fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut addr as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut sl,
    );
    if client == -(1 as libc::c_int) {
        match *__errno_location() {
            24 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this process\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this process\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            23 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this system\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this system\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {
                if *__errno_location() == 4 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                {} else {
                    __assert_fail(
                        b"errno == EINTR || errno == EWOULDBLOCK || errno == EAGAIN\0"
                            as *const u8 as *const libc::c_char,
                        b"stud.c\0" as *const u8 as *const libc::c_char,
                        1315 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 51],
                            &[libc::c_char; 51],
                        >(b"void handle_accept(struct ev_loop *, ev_io *, int)\0"))
                            .as_ptr(),
                    );
                };
            }
        }
        return;
    }
    let mut flag: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = setsockopt(
        client,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut flag as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if ret == -(1 as libc::c_int) {
        perror(
            b"Couldn't setsockopt on client (TCP_NODELAY)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    setnonblocking(client);
    settcpkeepalive(client);
    let mut back: libc::c_int = create_back_socket();
    if back == -(1 as libc::c_int) {
        close(client);
        perror(b"{backend-socket}\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut ctx: *mut SSL_CTX = (*w).data as *mut SSL_CTX;
    let mut ssl: *mut SSL = SSL_new(ctx);
    let mut mode: libc::c_long = 0x1 as libc::c_long;
    mode |= 0x10 as libc::c_long;
    SSL_ctrl(ssl, 33 as libc::c_int, mode, 0 as *mut libc::c_void);
    SSL_set_accept_state(ssl);
    SSL_set_fd(ssl, client);
    let mut ps: *mut proxystate = malloc(
        ::std::mem::size_of::<proxystate>() as libc::c_ulong,
    ) as *mut proxystate;
    (*ps).fd_up = client;
    (*ps).fd_down = back;
    (*ps).ssl = ssl;
    (*ps).set_want_shutdown(0 as libc::c_int);
    (*ps).set_clear_connected(0 as libc::c_int);
    (*ps).set_handshaked(0 as libc::c_int);
    (*ps).set_renegotiation(0 as libc::c_int);
    (*ps).remote_ip = addr;
    ringbuffer_init(&mut (*ps).ring_clear2ssl);
    ringbuffer_init(&mut (*ps).ring_ssl2clear);
    let ref mut fresh2 = (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh2 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh2;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_ssl
        .cb = Some(
        ssl_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_ssl.fd = client;
    (*ps).ev_r_ssl.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh3 = (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh3 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh3;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_ssl
        .cb = Some(
        ssl_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_ssl.fd = client;
    (*ps).ev_w_ssl.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh4 = (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh4 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh4;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_handshake.fd = client;
    (*ps).ev_r_handshake.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh5 = (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh5 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh5;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_handshake.fd = client;
    (*ps).ev_w_handshake.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh6 = (*(&mut (*ps).ev_proxy as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh6 = 0 as libc::c_int;
    (*(&mut (*ps).ev_proxy as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh6;
    (*(&mut (*ps).ev_proxy as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_proxy
        .cb = Some(
        client_proxy_proxy
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_proxy as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_proxy.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_proxy.fd = client;
    (*ps).ev_proxy.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh7 = (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh7 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh7;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_connect
        .cb = Some(
        handle_connect
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_connect.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_connect.fd = back;
    (*ps).ev_w_connect.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh8 = (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh8 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh8;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_clear
        .cb = Some(
        clear_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_clear.fd = back;
    (*ps).ev_w_clear.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh9 = (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh9 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh9;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_clear
        .cb = Some(
        clear_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_clear.fd = back;
    (*ps).ev_r_clear.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    (*ps).ev_r_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_w_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_r_clear.data = ps as *mut libc::c_void;
    (*ps).ev_w_clear.data = ps as *mut libc::c_void;
    (*ps).ev_proxy.data = ps as *mut libc::c_void;
    (*ps).ev_w_connect.data = ps as *mut libc::c_void;
    (*ps).ev_r_handshake.data = ps as *mut libc::c_void;
    (*ps).ev_w_handshake.data = ps as *mut libc::c_void;
    SSL_set_ex_data(ssl, 0 as libc::c_int, ps as *mut libc::c_char as *mut libc::c_void);
    if (*CONFIG).PROXY_PROXY_LINE != 0 {
        ev_io_start(loop_1, &mut (*ps).ev_proxy);
    } else {
        start_handshake(ps, 2 as libc::c_int);
    };
}
unsafe extern "C" fn check_ppid(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_timer,
    mut revents: libc::c_int,
) {
    let mut ppid: pid_t = getppid();
    if ppid != master_pid {
        fprintf(
            stderr,
            b"{core} Process %d detected parent death, closing listener socket.\n\0"
                as *const u8 as *const libc::c_char,
            child_num,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Process %d detected parent death, closing listener socket.\n\0"
                    as *const u8 as *const libc::c_char,
                child_num,
            );
        }
        ev_timer_stop(loop_1, w);
        ev_io_stop(loop_1, &mut listener);
        close(listener_socket);
    }
}
unsafe extern "C" fn handle_clear_accept(
    mut loop_1: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sl: socklen_t = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong
        as socklen_t;
    let mut client: libc::c_int = accept(
        (*w).fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut addr as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut sl,
    );
    if client == -(1 as libc::c_int) {
        match *__errno_location() {
            24 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this process\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this process\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            23 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this system\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this system\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {
                if *__errno_location() == 4 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                {} else {
                    __assert_fail(
                        b"errno == EINTR || errno == EWOULDBLOCK || errno == EAGAIN\0"
                            as *const u8 as *const libc::c_char,
                        b"stud.c\0" as *const u8 as *const libc::c_char,
                        1432 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 57],
                            &[libc::c_char; 57],
                        >(b"void handle_clear_accept(struct ev_loop *, ev_io *, int)\0"))
                            .as_ptr(),
                    );
                };
            }
        }
        return;
    }
    let mut flag: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = setsockopt(
        client,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut flag as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if ret == -(1 as libc::c_int) {
        perror(
            b"Couldn't setsockopt on client (TCP_NODELAY)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    setnonblocking(client);
    settcpkeepalive(client);
    let mut back: libc::c_int = create_back_socket();
    if back == -(1 as libc::c_int) {
        close(client);
        perror(b"{backend-socket}\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut ctx: *mut SSL_CTX = (*w).data as *mut SSL_CTX;
    let mut ssl: *mut SSL = SSL_new(ctx);
    let mut mode: libc::c_long = 0x1 as libc::c_long;
    mode |= 0x10 as libc::c_long;
    SSL_ctrl(ssl, 33 as libc::c_int, mode, 0 as *mut libc::c_void);
    SSL_set_connect_state(ssl);
    SSL_set_fd(ssl, back);
    if !client_session.is_null() {
        SSL_set_session(ssl, client_session);
    }
    let mut ps: *mut proxystate = malloc(
        ::std::mem::size_of::<proxystate>() as libc::c_ulong,
    ) as *mut proxystate;
    (*ps).fd_up = client;
    (*ps).fd_down = back;
    (*ps).ssl = ssl;
    (*ps).set_want_shutdown(0 as libc::c_int);
    (*ps).set_clear_connected(1 as libc::c_int);
    (*ps).set_handshaked(0 as libc::c_int);
    (*ps).set_renegotiation(0 as libc::c_int);
    (*ps).remote_ip = addr;
    ringbuffer_init(&mut (*ps).ring_clear2ssl);
    ringbuffer_init(&mut (*ps).ring_ssl2clear);
    let ref mut fresh10 = (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh10 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh10;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_clear
        .cb = Some(
        clear_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_clear.fd = client;
    (*ps).ev_r_clear.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh11 = (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh11 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh11;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_clear
        .cb = Some(
        clear_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_clear.fd = client;
    (*ps).ev_w_clear.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh12 = (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh12 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh12;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_connect
        .cb = Some(
        handle_connect
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_connect.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_connect.fd = back;
    (*ps).ev_w_connect.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh13 = (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh13 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh13;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_handshake.fd = back;
    (*ps).ev_r_handshake.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh14 = (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh14 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh14;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_handshake.fd = back;
    (*ps).ev_w_handshake.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh15 = (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh15 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh15;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_ssl
        .cb = Some(
        ssl_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_ssl.fd = back;
    (*ps).ev_w_ssl.events = EV_WRITE as libc::c_int | EV__IOFDSET as libc::c_int;
    let ref mut fresh16 = (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh16 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh16;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_ssl
        .cb = Some(
        ssl_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_ssl.fd = back;
    (*ps).ev_r_ssl.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    (*ps).ev_r_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_w_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_r_clear.data = ps as *mut libc::c_void;
    (*ps).ev_w_clear.data = ps as *mut libc::c_void;
    (*ps).ev_w_connect.data = ps as *mut libc::c_void;
    (*ps).ev_r_handshake.data = ps as *mut libc::c_void;
    (*ps).ev_w_handshake.data = ps as *mut libc::c_void;
    SSL_set_ex_data(ssl, 0 as libc::c_int, ps as *mut libc::c_char as *mut libc::c_void);
    ev_io_start(loop_1, &mut (*ps).ev_r_clear);
    start_connect(ps);
}
unsafe extern "C" fn handle_connections() {
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} Process %d online\n\0" as *const u8 as *const libc::c_char,
            child_num,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} Process %d online\n\0" as *const u8 as *const libc::c_char,
            child_num,
        );
    }
    create_workers = 0 as libc::c_int;
    let mut cpus: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    libc::memset(
        &mut cpus as *mut cpu_set_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
    );
    let mut __cpu: size_t = child_num as size_t;
    if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong)
        < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
    {
        let ref mut fresh17 = *(cpus.__bits)
            .as_mut_ptr()
            .offset(
                __cpu
                    .wrapping_div(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    ) as isize,
            );
        *fresh17
            |= (1 as libc::c_int as __cpu_mask)
                << __cpu
                    .wrapping_rem(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    );
    } else {};
    let mut res: libc::c_int = sched_setaffinity(
        0 as libc::c_int,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut cpus,
    );
    if res == 0 {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{core} Successfully attached to CPU #%d\n\0" as *const u8
                    as *const libc::c_char,
                child_num,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{core} Successfully attached to CPU #%d\n\0" as *const u8
                    as *const libc::c_char,
                child_num,
            );
        }
    } else {
        fprintf(
            stderr,
            b"{core-warning} Unable to attach to CPU #%d; do you have that many cores?\n\0"
                as *const u8 as *const libc::c_char,
            child_num,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core-warning} Unable to attach to CPU #%d; do you have that many cores?\n\0"
                    as *const u8 as *const libc::c_char,
                child_num,
            );
        }
    }
    loop_0 = ev_default_loop(EVFLAG_AUTO as libc::c_int as libc::c_uint);
    let mut timer_ppid_check: ev_timer = ev_timer {
        active: 0,
        pending: 0,
        priority: 0,
        data: 0 as *mut libc::c_void,
        cb: None,
        at: 0.,
        repeat: 0.,
    };
    let ref mut fresh18 = (*(&mut timer_ppid_check as *mut ev_timer as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh18 = 0 as libc::c_int;
    (*(&mut timer_ppid_check as *mut ev_timer as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh18;
    (*(&mut timer_ppid_check as *mut ev_timer as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    timer_ppid_check
        .cb = Some(
        check_ppid
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut timer_ppid_check as *mut ev_timer as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut timer_ppid_check.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
            >,
        >() as libc::c_ulong,
    );
    (*(&mut timer_ppid_check as *mut ev_timer as *mut ev_watcher_time)).at = 1.0f64;
    timer_ppid_check.repeat = 1.0f64;
    ev_timer_start(loop_0, &mut timer_ppid_check);
    let ref mut fresh19 = (*(&mut listener as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    *fresh19 = 0 as libc::c_int;
    (*(&mut listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = *fresh19;
    (*(&mut listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    listener
        .cb = (if (*CONFIG).PMODE as libc::c_uint
        == SSL_CLIENT as libc::c_int as libc::c_uint
    {
        Some(
            handle_clear_accept
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
        )
    } else {
        Some(
            handle_accept
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
        )
    });
    memmove(
        &mut (*(&mut listener as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut listener.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    listener.fd = listener_socket;
    listener.events = EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
    listener.data = default_ctx as *mut libc::c_void;
    ev_io_start(loop_0, &mut listener);
    ev_loop(loop_0, 0 as libc::c_int);
    fprintf(
        stderr,
        b"{core} Child %d exiting.\n\0" as *const u8 as *const libc::c_char,
        child_num,
    );
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            3 as libc::c_int,
            b"{core} Child %d exiting.\n\0" as *const u8 as *const libc::c_char,
            child_num,
        );
    }
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn change_root() {
    if chroot((*CONFIG).CHROOT) == -(1 as libc::c_int) {
        fail(b"chroot\0" as *const u8 as *const libc::c_char);
    }
    if chdir(b"/\0" as *const u8 as *const libc::c_char) != 0 {
        fail(b"chdir\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn drop_privileges() {
    if setgid((*CONFIG).GID) != 0 {
        fail(b"setgid failed\0" as *const u8 as *const libc::c_char);
    }
    if setuid((*CONFIG).UID) != 0 {
        fail(b"setuid failed\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn init_globals() {
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
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0 as libc::c_int;
    let gai_err: libc::c_int = getaddrinfo(
        (*CONFIG).BACK_IP,
        (*CONFIG).BACK_PORT,
        &mut hints,
        &mut backaddr,
    );
    if gai_err != 0 as libc::c_int {
        fprintf(
            stderr,
            b"{getaddrinfo}: [%s]\0" as *const u8 as *const libc::c_char,
            gai_strerror(gai_err),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{getaddrinfo}: [%s]\0" as *const u8 as *const libc::c_char,
                gai_strerror(gai_err),
            );
        }
        exit(1 as libc::c_int);
    }
    child_pids = calloc(
        (*CONFIG).NCORES as libc::c_ulong,
        ::std::mem::size_of::<pid_t>() as libc::c_ulong,
    ) as *mut pid_t;
    if child_pids.is_null() {
        fail(b"calloc\0" as *const u8 as *const libc::c_char);
    }
    if (*CONFIG).SYSLOG != 0 {
        openlog(
            b"stud\0" as *const u8 as *const libc::c_char,
            0x2 as libc::c_int | 0x1 as libc::c_int | 0x8 as libc::c_int,
            (*CONFIG).SYSLOG_FACILITY,
        );
    }
}
pub unsafe extern "C" fn start_children(
    mut start_index: libc::c_int,
    mut count: libc::c_int,
) {
    if create_workers == 0 {
        return;
    }
    child_num = start_index;
    while child_num < start_index + count {
        let mut pid: libc::c_int = fork();
        if pid == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"{core} fork() failed: %s; Goodbye cruel world!\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"{core} fork() failed: %s; Goodbye cruel world!\n\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            exit(1 as libc::c_int);
        } else if pid == 0 as libc::c_int {
            handle_connections();
            exit(0 as libc::c_int);
        } else {
            *child_pids.offset(child_num as isize) = pid;
        }
        child_num += 1;
        child_num;
    }
}
pub unsafe extern "C" fn replace_child_with_pid(mut pid: pid_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < (*CONFIG).NCORES {
        if *child_pids.offset(i as isize) == pid {
            start_children(i, 1 as libc::c_int);
            return;
        }
        i += 1;
        i;
    }
    fprintf(
        stderr,
        b"Cannot find index for child pid %d\0" as *const u8 as *const libc::c_char,
        pid,
    );
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            3 as libc::c_int,
            b"Cannot find index for child pid %d\0" as *const u8 as *const libc::c_char,
            pid,
        );
    }
}
unsafe extern "C" fn do_wait(mut signo: libc::c_int) {
    let mut status: libc::c_int = 0;
    let mut pid: libc::c_int = wait(&mut status);
    if pid == -(1 as libc::c_int) {
        if *__errno_location() == 10 as libc::c_int {
            fprintf(
                stderr,
                b"{core} All children have exited! Restarting...\n\0" as *const u8
                    as *const libc::c_char,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"{core} All children have exited! Restarting...\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            start_children(0 as libc::c_int, (*CONFIG).NCORES as libc::c_int);
        } else if *__errno_location() == 4 as libc::c_int {
            fprintf(
                stderr,
                b"{core} Interrupted wait\n\0" as *const u8 as *const libc::c_char,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"{core} Interrupted wait\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            fail(b"wait\0" as *const u8 as *const libc::c_char);
        }
    } else if status & 0x7f as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"{core} Child %d exited with status %d. Replacing...\n\0" as *const u8
                as *const libc::c_char,
            pid,
            (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Child %d exited with status %d. Replacing...\n\0" as *const u8
                    as *const libc::c_char,
                pid,
                (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
            );
        }
        replace_child_with_pid(pid);
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"{core} Child %d was terminated by signal %d. Replacing...\n\0" as *const u8
                as *const libc::c_char,
            pid,
            status & 0x7f as libc::c_int,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Child %d was terminated by signal %d. Replacing...\n\0"
                    as *const u8 as *const libc::c_char,
                pid,
                status & 0x7f as libc::c_int,
            );
        }
        replace_child_with_pid(pid);
    }
}
unsafe extern "C" fn sigh_terminate(mut signo: libc::c_int) {
    create_workers = 0 as libc::c_int;
    if getpid() == master_pid {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{core} Received signal %d, shutting down.\n\0" as *const u8
                    as *const libc::c_char,
                signo,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{core} Received signal %d, shutting down.\n\0" as *const u8
                    as *const libc::c_char,
                signo,
            );
        }
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < (*CONFIG).NCORES {
            if *child_pids.offset(i as isize) > 1 as libc::c_int
                && kill(*child_pids.offset(i as isize), 15 as libc::c_int)
                    != 0 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"{core} Unable to send SIGTERM to worker pid %d: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    *child_pids.offset(i as isize),
                    strerror(*__errno_location()),
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{core} Unable to send SIGTERM to worker pid %d: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        *child_pids.offset(i as isize),
                        strerror(*__errno_location()),
                    );
                }
            }
            i += 1;
            i;
        }
    }
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn init_signals() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_12 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    if sigaction(13 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int {
        fail(b"sigaction - sigpipe\0" as *const u8 as *const libc::c_char);
    }
    act.sa_flags = 1 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(do_wait as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(17 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int {
        fail(b"sigaction - sigchld\0" as *const u8 as *const libc::c_char);
    }
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(sigh_terminate as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Unable to register SIGINT signal handler: %s\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to register SIGINT signal handler: %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    if sigaction(15 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Unable to register SIGTERM signal handler: %s\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to register SIGTERM signal handler: %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn daemonize() {
    if chdir(b"/\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Unable change directory to /: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable change directory to /: %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    let mut pid: pid_t = fork();
    if pid < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Unable to daemonize: fork failed: %s\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to daemonize: fork failed: %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    if pid != 0 as libc::c_int {
        printf(
            b"{core} Daemonized as pid %d.\n\0" as *const u8 as *const libc::c_char,
            pid,
        );
        exit(0 as libc::c_int);
    }
    fclose(stdin);
    fclose(stdout);
    fclose(stderr);
    stdin = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if stdin.is_null() {
        fprintf(
            stderr,
            b"Unable to reopen stdin to %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to reopen stdin to %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    stdout = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if stdout.is_null() {
        fprintf(
            stderr,
            b"Unable to reopen stdout to %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to reopen stdout to %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    stderr = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if stderr.is_null() {
        fprintf(
            stderr,
            b"Unable to reopen stderr to %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to reopen stderr to %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    let mut s: pid_t = setsid();
    if s < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Unable to create new session, setsid(2) failed: %s :: %d\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
            s,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Unable to create new session, setsid(2) failed: %s :: %d\n\0"
                    as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
                s,
            );
        }
        exit(1 as libc::c_int);
    }
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"Successfully daemonized as pid %d.\n\0" as *const u8
                as *const libc::c_char,
            getpid(),
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"Successfully daemonized as pid %d.\n\0" as *const u8
                as *const libc::c_char,
            getpid(),
        );
    }
}
pub unsafe extern "C" fn openssl_check_version() {
    openssl_version = SSLeay() as libc::c_long;
    if (openssl_version ^ 0x1000115f as libc::c_long) & !(0xff0 as libc::c_long) != 0 {
        fprintf(
            stderr,
            b"WARNING: {core} OpenSSL version mismatch; stud was compiled with %lx, now using %lx.\n\0"
                as *const u8 as *const libc::c_char,
            0x1000115f as libc::c_long as libc::c_ulong,
            openssl_version as libc::c_ulong,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"WARNING: {core} OpenSSL version mismatch; stud was compiled with %lx, now using %lx.\n\0"
                    as *const u8 as *const libc::c_char,
                0x1000115f as libc::c_long as libc::c_ulong,
                openssl_version as libc::c_ulong,
            );
        }
    }
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} Using OpenSSL version %lx.\n\0" as *const u8 as *const libc::c_char,
            openssl_version as libc::c_ulong,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} Using OpenSSL version %lx.\n\0" as *const u8 as *const libc::c_char,
            openssl_version as libc::c_ulong,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    CONFIG = config_new();
    config_parse_cli(argc, argv, CONFIG);
    create_workers = 1 as libc::c_int;
    openssl_check_version();
    init_signals();
    init_globals();
    listener_socket = create_main_socket();
    init_openssl();
    if !((*CONFIG).CHROOT).is_null()
        && *((*CONFIG).CHROOT).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        change_root();
    }
    if (*CONFIG).UID != 0 || (*CONFIG).GID != 0 {
        drop_privileges();
    }
    if (*CONFIG).DAEMONIZE != 0 {
        (*CONFIG).QUIET = 1 as libc::c_int;
        (*CONFIG).SYSLOG = 1 as libc::c_int;
        daemonize();
    }
    master_pid = getpid();
    start_children(0 as libc::c_int, (*CONFIG).NCORES as libc::c_int);
    loop {
        pause();
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

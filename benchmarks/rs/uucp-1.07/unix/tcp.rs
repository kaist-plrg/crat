use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_id(iid: libc::c_int);
    fn ulog_device(zdevice: *const libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    static mut afSignal: [sig_atomic_t; 5];
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fsysdep_conn_read(
        qconn: *mut sconnection,
        zbuf: *mut libc::c_char,
        pclen: *mut size_t,
        cmin: size_t,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> boolean;
    fn fsysdep_conn_write(
        qconn: *mut sconnection,
        zbuf: *const libc::c_char,
        clen: size_t,
    ) -> boolean;
    fn fsysdep_conn_io(
        qconn: *mut sconnection,
        zwrite: *const libc::c_char,
        pcwrite: *mut size_t,
        zread: *mut libc::c_char,
        pcread: *mut size_t,
    ) -> boolean;
    fn fsysdep_conn_chat(
        qconn: *mut sconnection,
        pzprog: *mut *mut libc::c_char,
    ) -> boolean;
    fn ixsfork() -> pid_t;
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn fsuser_perms(_: *mut uid_t, _: *mut gid_t) -> boolean;
    fn fsuucp_perms(_: libc::c_long, _: libc::c_long) -> boolean;
    fn fconn_dial_sequence(
        qconn: *mut sconnection,
        puuconf: pointer,
        pzdialer: *mut *mut libc::c_char,
        qsys: *const uuconf_system,
        zphone: *const libc::c_char,
        qdialer: *mut uuconf_dialer,
        ptdialerfound: *mut tdialerfound,
    ) -> boolean;
    fn usysdep_exit(fsuccess: boolean);
    fn __errno_location() -> *mut libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
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
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn getservbyname(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
    ) -> *mut servent;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type sig_atomic_t = __sig_atomic_t;
pub type pid_t = __pid_t;
pub type uid_t = __uid_t;
pub type pointer = *mut libc::c_void;
pub type gid_t = __gid_t;
pub type boolean = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_system {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_pzalias: *mut *mut libc::c_char,
    pub uuconf_qalternate: *mut uuconf_system,
    pub uuconf_zalternate: *mut libc::c_char,
    pub uuconf_fcall: libc::c_int,
    pub uuconf_fcalled: libc::c_int,
    pub uuconf_qtimegrade: *mut uuconf_timespan,
    pub uuconf_qcalltimegrade: *mut uuconf_timespan,
    pub uuconf_qcalledtimegrade: *mut uuconf_timespan,
    pub uuconf_cmax_retries: libc::c_int,
    pub uuconf_csuccess_wait: libc::c_int,
    pub uuconf_qcall_local_size: *mut uuconf_timespan,
    pub uuconf_qcall_remote_size: *mut uuconf_timespan,
    pub uuconf_qcalled_local_size: *mut uuconf_timespan,
    pub uuconf_qcalled_remote_size: *mut uuconf_timespan,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_ihighbaud: libc::c_long,
    pub uuconf_zport: *mut libc::c_char,
    pub uuconf_qport: *mut uuconf_port,
    pub uuconf_zphone: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zcall_login: *mut libc::c_char,
    pub uuconf_zcall_password: *mut libc::c_char,
    pub uuconf_zcalled_login: *mut libc::c_char,
    pub uuconf_fcallback: libc::c_int,
    pub uuconf_fsequence: libc::c_int,
    pub uuconf_zprotocols: *mut libc::c_char,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_scalled_chat: uuconf_chat,
    pub uuconf_zdebug: *mut libc::c_char,
    pub uuconf_zmax_remote_debug: *mut libc::c_char,
    pub uuconf_fsend_request: libc::c_int,
    pub uuconf_frec_request: libc::c_int,
    pub uuconf_fcall_transfer: libc::c_int,
    pub uuconf_fcalled_transfer: libc::c_int,
    pub uuconf_pzlocal_send: *mut *mut libc::c_char,
    pub uuconf_pzremote_send: *mut *mut libc::c_char,
    pub uuconf_pzlocal_receive: *mut *mut libc::c_char,
    pub uuconf_pzremote_receive: *mut *mut libc::c_char,
    pub uuconf_pzpath: *mut *mut libc::c_char,
    pub uuconf_pzcmds: *mut *mut libc::c_char,
    pub uuconf_cfree_space: libc::c_long,
    pub uuconf_pzforward_from: *mut *mut libc::c_char,
    pub uuconf_pzforward_to: *mut *mut libc::c_char,
    pub uuconf_zpubdir: *const libc::c_char,
    pub uuconf_zlocalname: *mut libc::c_char,
    pub uuconf_cmax_file_time: libc::c_long,
    pub uuconf_palloc: UUCONF_POINTER,
}
pub type UUCONF_POINTER = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_proto_param {
    pub uuconf_bproto: libc::c_int,
    pub uuconf_qentries: *mut uuconf_proto_param_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_proto_param_entry {
    pub uuconf_cargs: libc::c_int,
    pub uuconf_pzargs: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_port {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_ttype: uuconf_porttype,
    pub uuconf_zprotocols: *mut libc::c_char,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_zlockname: *mut libc::c_char,
    pub uuconf_palloc: UUCONF_POINTER,
    pub uuconf_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uuconf_sstdin: uuconf_stdin_port,
    pub uuconf_smodem: uuconf_modem_port,
    pub uuconf_sdirect: uuconf_direct_port,
    pub uuconf_stcp: uuconf_tcp_port,
    pub uuconf_stli: uuconf_tli_port,
    pub uuconf_spipe: uuconf_pipe_port,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_pipe_port {
    pub uuconf_pzcmd: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_tli_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_fstream: libc::c_int,
    pub uuconf_pzpush: *mut *mut libc::c_char,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
    pub uuconf_zservaddr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_tcp_port {
    pub uuconf_zport: *mut libc::c_char,
    pub uuconf_iversion: libc::c_int,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_direct_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_fhardflow: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_modem_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_zdial_device: *mut libc::c_char,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_ilowbaud: libc::c_long,
    pub uuconf_ihighbaud: libc::c_long,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_fhardflow: libc::c_int,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
    pub uuconf_qdialer: *mut uuconf_dialer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_dialer {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zdialtone: *mut libc::c_char,
    pub uuconf_zpause: *mut libc::c_char,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_ccarrier_wait: libc::c_int,
    pub uuconf_fdtr_toggle: libc::c_int,
    pub uuconf_fdtr_toggle_wait: libc::c_int,
    pub uuconf_scomplete: uuconf_chat,
    pub uuconf_sabort: uuconf_chat,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_palloc: UUCONF_POINTER,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_stdin_port {
    pub uuconf_idummy: libc::c_int,
}
pub type uuconf_porttype = libc::c_uint;
pub const UUCONF_PORTTYPE_PIPE: uuconf_porttype = 6;
pub const UUCONF_PORTTYPE_TLI: uuconf_porttype = 5;
pub const UUCONF_PORTTYPE_TCP: uuconf_porttype = 4;
pub const UUCONF_PORTTYPE_DIRECT: uuconf_porttype = 3;
pub const UUCONF_PORTTYPE_MODEM: uuconf_porttype = 2;
pub const UUCONF_PORTTYPE_STDIN: uuconf_porttype = 1;
pub const UUCONF_PORTTYPE_UNKNOWN: uuconf_porttype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
}
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sconnection {
    pub qcmds: *const sconncmds,
    pub psysdep: pointer,
    pub qport: *mut uuconf_port,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sconncmds {
    pub pufree: Option::<unsafe extern "C" fn(*mut sconnection) -> ()>,
    pub pflock: Option::<
        unsafe extern "C" fn(*mut sconnection, boolean, boolean) -> boolean,
    >,
    pub pfunlock: Option::<unsafe extern "C" fn(*mut sconnection) -> boolean>,
    pub pfopen: Option::<
        unsafe extern "C" fn(*mut sconnection, libc::c_long, boolean, boolean) -> boolean,
    >,
    pub pfclose: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            pointer,
            *mut uuconf_dialer,
            boolean,
        ) -> boolean,
    >,
    pub pfdial: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            pointer,
            *const uuconf_system,
            *const libc::c_char,
            *mut uuconf_dialer,
            *mut tdialerfound,
        ) -> boolean,
    >,
    pub pfread: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            *mut libc::c_char,
            *mut size_t,
            size_t,
            libc::c_int,
            boolean,
        ) -> boolean,
    >,
    pub pfwrite: Option::<
        unsafe extern "C" fn(*mut sconnection, *const libc::c_char, size_t) -> boolean,
    >,
    pub pfio: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            *const libc::c_char,
            *mut size_t,
            *mut libc::c_char,
            *mut size_t,
        ) -> boolean,
    >,
    pub pfbreak: Option::<unsafe extern "C" fn(*mut sconnection) -> boolean>,
    pub pfset: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            tparitysetting,
            tstripsetting,
            txonxoffsetting,
        ) -> boolean,
    >,
    pub pfcarrier: Option::<unsafe extern "C" fn(*mut sconnection, boolean) -> boolean>,
    pub pfchat: Option::<
        unsafe extern "C" fn(*mut sconnection, *mut *mut libc::c_char) -> boolean,
    >,
    pub pibaud: Option::<unsafe extern "C" fn(*mut sconnection) -> libc::c_long>,
}
pub type txonxoffsetting = libc::c_uint;
pub const XONXOFF_ON: txonxoffsetting = 2;
pub const XONXOFF_OFF: txonxoffsetting = 1;
pub const XONXOFF_DEFAULT: txonxoffsetting = 0;
pub type tstripsetting = libc::c_uint;
pub const STRIPSETTING_SEVENBITS: tstripsetting = 2;
pub const STRIPSETTING_EIGHTBITS: tstripsetting = 1;
pub const STRIPSETTING_DEFAULT: tstripsetting = 0;
pub type tparitysetting = libc::c_uint;
pub const PARITYSETTING_SPACE: tparitysetting = 5;
pub const PARITYSETTING_MARK: tparitysetting = 4;
pub const PARITYSETTING_ODD: tparitysetting = 3;
pub const PARITYSETTING_EVEN: tparitysetting = 2;
pub const PARITYSETTING_NONE: tparitysetting = 1;
pub const PARITYSETTING_DEFAULT: tparitysetting = 0;
pub type tdialerfound = libc::c_uint;
pub const DIALERFOUND_FREE: tdialerfound = 2;
pub const DIALERFOUND_TRUE: tdialerfound = 1;
pub const DIALERFOUND_FALSE: tdialerfound = 0;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type sterminal = termios;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssysdep_conn {
    pub o: libc::c_int,
    pub ord: libc::c_int,
    pub owr: libc::c_int,
    pub zdevice: *mut libc::c_char,
    pub iflags: libc::c_int,
    pub iwr_flags: libc::c_int,
    pub ohold: libc::c_int,
    pub fterminal: boolean,
    pub ftli: boolean,
    pub ibaud: libc::c_long,
    pub sorig: sterminal,
    pub snew: sterminal,
    pub ipid: pid_t,
}
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
pub type uint32_t = __uint32_t;
pub type in_port_t = uint16_t;
pub type uint16_t = __uint16_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
    pub s_name: *mut libc::c_char,
    pub s_aliases: *mut *mut libc::c_char,
    pub s_port: libc::c_int,
    pub s_proto: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub const SOCK_STREAM: __socket_type = 1;
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
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
pub static mut tcp_rcsid: [libc::c_char; 48] = unsafe {
    *::std::mem::transmute::<
        &[u8; 48],
        &[libc::c_char; 48],
    >(b"$Id: tcp.c,v 1.12 2002/03/05 19:10:42 ian Rel $\0")
};
static mut stcpcmds: sconncmds = unsafe {
    {
        let mut init = sconncmds {
            pufree: Some(utcp_free as unsafe extern "C" fn(*mut sconnection) -> ()),
            pflock: None,
            pfunlock: None,
            pfopen: Some(
                ftcp_open
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        libc::c_long,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfclose: Some(
                ftcp_close
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        pointer,
                        *mut uuconf_dialer,
                        boolean,
                    ) -> boolean,
            ),
            pfdial: Some(
                ftcp_dial
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        pointer,
                        *const uuconf_system,
                        *const libc::c_char,
                        *mut uuconf_dialer,
                        *mut tdialerfound,
                    ) -> boolean,
            ),
            pfread: Some(
                fsysdep_conn_read
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *mut libc::c_char,
                        *mut size_t,
                        size_t,
                        libc::c_int,
                        boolean,
                    ) -> boolean,
            ),
            pfwrite: Some(
                fsysdep_conn_write
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *const libc::c_char,
                        size_t,
                    ) -> boolean,
            ),
            pfio: Some(
                fsysdep_conn_io
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *const libc::c_char,
                        *mut size_t,
                        *mut libc::c_char,
                        *mut size_t,
                    ) -> boolean,
            ),
            pfbreak: None,
            pfset: None,
            pfcarrier: None,
            pfchat: Some(
                fsysdep_conn_chat
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *mut *mut libc::c_char,
                    ) -> boolean,
            ),
            pibaud: None,
        };
        init
    }
};
pub unsafe extern "C" fn fsysdep_tcp_init(mut qconn: *mut sconnection) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    q = xmalloc(::std::mem::size_of::<ssysdep_conn>() as libc::c_ulong)
        as *mut ssysdep_conn;
    (*q).o = -(1 as libc::c_int);
    (*q).ord = -(1 as libc::c_int);
    (*q).owr = -(1 as libc::c_int);
    (*q).zdevice = 0 as *mut libc::c_char;
    (*q).iflags = -(1 as libc::c_int);
    (*q).iwr_flags = -(1 as libc::c_int);
    (*q).fterminal = 0 as libc::c_int;
    (*q).ftli = 0 as libc::c_int;
    (*q).ibaud = 0 as libc::c_int as libc::c_long;
    (*qconn).psysdep = q as pointer;
    (*qconn).qcmds = &stcpcmds;
    return 1 as libc::c_int;
}
unsafe extern "C" fn utcp_free(mut qconn: *mut sconnection) {
    xfree((*qconn).psysdep);
}
unsafe extern "C" fn ftcp_set_hints(
    mut iversion: libc::c_int,
    mut qhints: *mut addrinfo,
) -> boolean {
    match iversion {
        0 => {
            (*qhints).ai_family = 0 as libc::c_int;
        }
        4 => {
            (*qhints).ai_family = 2 as libc::c_int;
        }
        6 => {
            (*qhints).ai_family = 10 as libc::c_int;
        }
        _ => {
            ulog(
                LOG_ERROR,
                b"Invalid IP version number %d\0" as *const u8 as *const libc::c_char,
                iversion,
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ftcp_set_flags(mut qsysdep: *mut ssysdep_conn) -> boolean {
    if fcntl(
        (*qsysdep).o,
        2 as libc::c_int,
        fcntl((*qsysdep).o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close((*qsysdep).o);
        (*qsysdep).o = -(1 as libc::c_int);
        return 0 as libc::c_int;
    }
    (*qsysdep).iflags = fcntl((*qsysdep).o, 3 as libc::c_int, 0 as libc::c_int);
    if (*qsysdep).iflags < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fcntl: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close((*qsysdep).o);
        (*qsysdep).o = -(1 as libc::c_int);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ftcp_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut zport: *const libc::c_char = 0 as *const libc::c_char;
    let mut ieuid: uid_t = 0;
    let mut iegid: gid_t = 0;
    let mut fswap: boolean = 0;
    let mut shints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut qres: *mut addrinfo = 0 as *mut addrinfo;
    let mut quse: *mut addrinfo = 0 as *mut addrinfo;
    let mut ierr: libc::c_int = 0;
    ulog_device(b"TCP\0" as *const u8 as *const libc::c_char);
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    (*qsysdep).o = -(1 as libc::c_int);
    (*qsysdep).ipid = getpid();
    if fwait == 0 {
        return 1 as libc::c_int;
    }
    zport = (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_zport;
    bzero(
        &mut shints as *mut addrinfo as pointer,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    if ftcp_set_hints(
        (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_iversion,
        &mut shints,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    shints.ai_socktype = SOCK_STREAM as libc::c_int;
    shints.ai_flags = 0x1 as libc::c_int;
    ierr = getaddrinfo(0 as *const libc::c_char, zport, &mut shints, &mut qres);
    if ierr == -(8 as libc::c_int)
        && strcmp(zport, b"uucp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        ierr = getaddrinfo(
            0 as *const libc::c_char,
            b"540\0" as *const u8 as *const libc::c_char,
            &mut shints,
            &mut qres,
        );
    }
    if ierr != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"getaddrinfo: %s\0" as *const u8 as *const libc::c_char,
            gai_strerror(ierr),
        );
        qres = 0 as *mut addrinfo;
        quse = 0 as *mut addrinfo;
    } else {
        quse = qres;
        while !quse.is_null() {
            (*qsysdep)
                .o = socket((*quse).ai_family, (*quse).ai_socktype, (*quse).ai_protocol);
            if (*qsysdep).o >= 0 as libc::c_int {
                break;
            }
            quse = (*quse).ai_next;
        }
    }
    if (*qsysdep).o < 0 as libc::c_int {
        if (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_iversion != 0 as libc::c_int
            && (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_iversion != 4 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"Could not get IPv6 socket\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        (*qsysdep)
            .o = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if (*qsysdep).o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"socket: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    if ftcp_set_flags(qsysdep) == 0 {
        return 0 as libc::c_int;
    }
    fswap = (geteuid() != 0 as libc::c_int as libc::c_uint) as libc::c_int;
    if fswap != 0 {
        if fsuser_perms(&mut ieuid, &mut iegid) == 0 {
            close((*qsysdep).o);
            (*qsysdep).o = -(1 as libc::c_int);
            if !qres.is_null() {
                freeaddrinfo(qres);
            }
            return 0 as libc::c_int;
        }
    }
    if !quse.is_null() {
        if bind((*qsysdep).o, (*quse).ai_addr, (*quse).ai_addrlen) < 0 as libc::c_int {
            if fswap != 0 {
                fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long);
            }
            ulog(
                LOG_FATAL,
                b"bind: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    } else {
        let mut sin: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        bzero(
            &mut sin as *mut sockaddr_in as pointer,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        sin.sin_family = 2 as libc::c_int as sa_family_t;
        sin.sin_port = itcp_port_number(zport) as in_port_t;
        sin.sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
        if bind(
            (*qsysdep).o,
            &mut sin as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        ) < 0 as libc::c_int
        {
            if fswap != 0 {
                fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long);
            }
            ulog(
                LOG_FATAL,
                b"bind: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    }
    if !qres.is_null() {
        freeaddrinfo(qres);
    }
    if fswap != 0 {
        if fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long) == 0 {
            ulog(
                LOG_FATAL,
                b"Could not swap back to UUCP user permissions\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if listen((*qsysdep).o, 5 as libc::c_int) < 0 as libc::c_int {
        ulog(
            LOG_FATAL,
            b"listen: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    while !(afSignal[0 as libc::c_int as usize] != 0
        || afSignal[1 as libc::c_int as usize] != 0
        || afSignal[2 as libc::c_int as usize] != 0
        || afSignal[3 as libc::c_int as usize] != 0
        || afSignal[4 as libc::c_int as usize] != 0)
    {
        let mut speer: sockaddr_storage = sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        };
        let mut clen: size_t = 0;
        let mut onew: libc::c_int = 0;
        let mut ipid: pid_t = 0;
        if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"ftcp_open: Waiting for connections\0" as *const u8
                    as *const libc::c_char,
            );
        }
        clen = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong;
        onew = accept(
            (*qsysdep).o,
            &mut speer as *mut sockaddr_storage as *mut sockaddr,
            &mut clen as *mut size_t as *mut socklen_t,
        );
        if onew < 0 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"accept: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"ftcp_open: Got connection; forking\0" as *const u8
                    as *const libc::c_char,
            );
        }
        ipid = ixsfork();
        if ipid < 0 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"fork: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if ipid == 0 as libc::c_int {
            close((*qsysdep).o);
            (*qsysdep).o = onew;
            ipid = ixsfork();
            if ipid < 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"fork: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                _exit(1 as libc::c_int);
            }
            if ipid != 0 as libc::c_int {
                _exit(0 as libc::c_int);
            }
            ulog_id(getpid());
            return 1 as libc::c_int;
        }
        close(onew);
        ixswait(ipid as libc::c_ulong, 0 as *mut libc::c_void as *const libc::c_char);
    }
    usysdep_exit(0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ftcp_close(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdialer: *mut uuconf_dialer,
    mut fsuccess: boolean,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut fret: boolean = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    fret = 1 as libc::c_int;
    if (*qsysdep).o >= 0 as libc::c_int && close((*qsysdep).o) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"close: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fret = 0 as libc::c_int;
    }
    (*qsysdep).o = -(1 as libc::c_int);
    if (*qsysdep).ipid != getpid() {
        fret = 0 as libc::c_int;
    }
    return fret;
}
unsafe extern "C" fn ftcp_dial(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qsys: *const uuconf_system,
    mut zphone: *const libc::c_char,
    mut qdialer: *mut uuconf_dialer,
    mut ptdialer: *mut tdialerfound,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut zhost: *const libc::c_char = 0 as *const libc::c_char;
    let mut zport: *const libc::c_char = 0 as *const libc::c_char;
    let mut pzdialer: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut shints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut qres: *mut addrinfo = 0 as *mut addrinfo;
    let mut quse: *mut addrinfo = 0 as *mut addrinfo;
    let mut ierr: libc::c_int = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    (*qsysdep).o = -(1 as libc::c_int);
    *ptdialer = DIALERFOUND_FALSE;
    zhost = zphone;
    if zhost.is_null() {
        if qsys.is_null() {
            ulog(
                LOG_ERROR,
                b"No address for TCP connection\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        zhost = (*qsys).uuconf_zname;
    }
    zport = (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_zport;
    bzero(
        &mut shints as *mut addrinfo as pointer,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    if ftcp_set_hints(
        (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_iversion,
        &mut shints,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    shints.ai_socktype = SOCK_STREAM as libc::c_int;
    ierr = getaddrinfo(zhost, zport, &mut shints, &mut qres);
    if ierr == -(8 as libc::c_int)
        && strcmp(zport, b"uucp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        ierr = getaddrinfo(
            zhost,
            b"540\0" as *const u8 as *const libc::c_char,
            &mut shints,
            &mut qres,
        );
    }
    if ierr != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"getaddrinfo: %s\0" as *const u8 as *const libc::c_char,
            gai_strerror(ierr),
        );
        qres = 0 as *mut addrinfo;
        quse = 0 as *mut addrinfo;
        ierr = 0 as libc::c_int;
    } else {
        ierr = 0 as libc::c_int;
        quse = qres;
        while !quse.is_null() {
            (*qsysdep)
                .o = socket((*quse).ai_family, (*quse).ai_socktype, (*quse).ai_protocol);
            if (*qsysdep).o >= 0 as libc::c_int {
                if connect((*qsysdep).o, (*quse).ai_addr, (*quse).ai_addrlen)
                    >= 0 as libc::c_int
                {
                    break;
                }
                ierr = *__errno_location();
                close((*qsysdep).o);
                (*qsysdep).o = -(1 as libc::c_int);
            }
            quse = (*quse).ai_next;
        }
        if !qres.is_null() {
            freeaddrinfo(qres);
        }
    }
    if (*qsysdep).o < 0 as libc::c_int {
        let mut qhost: *mut hostent = 0 as *mut hostent;
        let mut sin: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_iversion != 0 as libc::c_int
            && (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_iversion != 4 as libc::c_int
        {
            if ierr != 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"connect: %s\0" as *const u8 as *const libc::c_char,
                    strerror(ierr),
                );
            } else {
                ulog(
                    LOG_ERROR,
                    b"Could not get IPv6 address or socket\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int;
        }
        (*qsysdep)
            .o = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if (*qsysdep).o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"socket: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        *__errno_location() = 0 as libc::c_int;
        bzero(
            &mut sin as *mut sockaddr_in as pointer,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        qhost = gethostbyname(zhost as *mut libc::c_char);
        if !qhost.is_null() {
            sin.sin_family = (*qhost).h_addrtype as sa_family_t;
            memcpy(
                &mut sin.sin_addr.s_addr as *mut in_addr_t as *mut libc::c_void,
                *((*qhost).h_addr_list).offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                (*qhost).h_length as size_t,
            );
        } else {
            if *__errno_location() != 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"gethostbyname (%s): %s\0" as *const u8 as *const libc::c_char,
                    zhost,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            sin.sin_family = 2 as libc::c_int as sa_family_t;
            sin.sin_addr.s_addr = inet_addr(zhost as *mut libc::c_char);
            if sin.sin_addr.s_addr as libc::c_long == -(1 as libc::c_int) as libc::c_long
            {
                ulog(
                    LOG_ERROR,
                    b"%s: unknown host name\0" as *const u8 as *const libc::c_char,
                    zhost,
                );
                return 0 as libc::c_int;
            }
        }
        sin.sin_port = itcp_port_number(zport) as in_port_t;
        if connect(
            (*qsysdep).o,
            &mut sin as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        ) < 0 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"connect: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            close((*qsysdep).o);
            (*qsysdep).o = -(1 as libc::c_int);
            return 0 as libc::c_int;
        }
    }
    if ftcp_set_flags(qsysdep) == 0 {
        return 0 as libc::c_int;
    }
    pzdialer = (*(*qconn).qport).uuconf_u.uuconf_stcp.uuconf_pzdialer;
    if !pzdialer.is_null() && !(*pzdialer).is_null() {
        if fconn_dial_sequence(qconn, puuconf, pzdialer, qsys, zphone, qdialer, ptdialer)
            == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn itcp_port_number(mut zname: *const libc::c_char) -> libc::c_int {
    let mut fuucp: boolean = 0;
    static mut iuucp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut servent = 0 as *mut servent;
    fuucp = (strcmp(zname, b"uucp\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
    if fuucp != 0 && iuucp != 0 as libc::c_int {
        return iuucp;
    }
    i = strtol(zname as *mut libc::c_char, &mut zend, 10 as libc::c_int) as libc::c_int;
    if i != 0 as libc::c_int && *zend as libc::c_int == '\0' as i32 {
        return __bswap_16(i as __uint16_t) as libc::c_int;
    }
    q = getservbyname(
        zname as *mut libc::c_char,
        b"tcp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if q.is_null() {
        if fuucp != 0 {
            iuucp = __bswap_16(540 as libc::c_int as __uint16_t) as libc::c_int;
            return iuucp;
        }
        ulog(
            LOG_ERROR,
            b"getservbyname (%s): %s\0" as *const u8 as *const libc::c_char,
            zname,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if fuucp != 0 {
        iuucp = (*q).s_port;
    }
    return (*q).s_port;
}

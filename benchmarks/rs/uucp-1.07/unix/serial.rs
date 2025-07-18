use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_device(zdevice: *const libc::c_char);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    static mut afSignal: [sig_atomic_t; 5];
    fn uuconf_dialer_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdialer: *const libc::c_char,
        uuconf_qdialer: *mut uuconf_dialer,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn zsysdep_port_name(pftcp_port: *mut boolean) -> *const libc::c_char;
    fn fchat(
        qconn: *mut sconnection,
        puuconf: pointer,
        qchat: *const uuconf_chat,
        qsys: *const uuconf_system,
        qdialer: *const uuconf_dialer,
        zphone: *const libc::c_char,
        ftranslate: boolean,
        zport: *const libc::c_char,
        ibaud: libc::c_long,
    ) -> boolean;
    fn fmodem_dial(
        qconn: *mut sconnection,
        puuconf: pointer,
        qsys: *const uuconf_system,
        zphone: *const libc::c_char,
        qdialer: *mut uuconf_dialer,
        ptdialerfound: *mut tdialerfound,
    ) -> boolean;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn tcsendbreak(__fd: libc::c_int, __duration: libc::c_int) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn usset_signal(
        isig: libc::c_int,
        pfn: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        fforce: boolean,
        pfignored: *mut boolean,
    );
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn fsuucp_perms(_: libc::c_long, _: libc::c_long) -> boolean;
    fn fsuser_perms(_: *mut uid_t, _: *mut gid_t) -> boolean;
    fn ixsspawn(
        pazargs: *mut *const libc::c_char,
        aidescs: *mut libc::c_int,
        fkeepuid: boolean,
        fkeepenv: boolean,
        zchdir: *const libc::c_char,
        fnosigs: boolean,
        fshell: boolean,
        zpath: *const libc::c_char,
        zuu_machine: *const libc::c_char,
        zuu_user: *const libc::c_char,
    ) -> pid_t;
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn ussignal(isig: libc::c_int);
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn fsdo_unlock(_: *const libc::c_char, fspooldir: boolean) -> boolean;
    fn fsdo_lock(
        _: *const libc::c_char,
        fspooldir: boolean,
        pferr: *mut boolean,
    ) -> boolean;
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pid_t = __pid_t;
pub type uid_t = __uid_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type pointer = *mut libc::c_void;
pub type gid_t = __gid_t;
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
pub type sterminal = termios;
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
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub const IGNORE_CLOCAL: tclocal_setting = 2;
pub type tclocal_setting = libc::c_uint;
pub const CLEAR_CLOCAL: tclocal_setting = 1;
pub const SET_CLOCAL: tclocal_setting = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbaud_table {
    pub icode: baud_code,
    pub ibaud: libc::c_long,
}
pub type baud_code = speed_t;
pub static mut serial_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: serial.c,v 1.78 2002/03/05 19:10:42 ian Rel $\0")
};
static mut sstdincmds: sconncmds = unsafe {
    {
        let mut init = sconncmds {
            pufree: Some(usserial_free as unsafe extern "C" fn(*mut sconnection) -> ()),
            pflock: None,
            pfunlock: None,
            pfopen: Some(
                fsstdin_open
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        libc::c_long,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfclose: Some(
                fsstdin_close
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        pointer,
                        *mut uuconf_dialer,
                        boolean,
                    ) -> boolean,
            ),
            pfdial: None,
            pfread: Some(
                fsdouble_read
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
                fsdouble_write
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
            pfbreak: Some(
                fsstdin_break as unsafe extern "C" fn(*mut sconnection) -> boolean,
            ),
            pfset: Some(
                fsstdin_set
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        tparitysetting,
                        tstripsetting,
                        txonxoffsetting,
                    ) -> boolean,
            ),
            pfcarrier: None,
            pfchat: Some(
                fsdouble_chat
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *mut *mut libc::c_char,
                    ) -> boolean,
            ),
            pibaud: Some(
                isserial_baud as unsafe extern "C" fn(*mut sconnection) -> libc::c_long,
            ),
        };
        init
    }
};
static mut smodemcmds: sconncmds = unsafe {
    {
        let mut init = sconncmds {
            pufree: Some(usserial_free as unsafe extern "C" fn(*mut sconnection) -> ()),
            pflock: Some(
                fsserial_lock
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfunlock: Some(
                fsserial_unlock as unsafe extern "C" fn(*mut sconnection) -> boolean,
            ),
            pfopen: Some(
                fsmodem_open
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        libc::c_long,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfclose: Some(
                fsmodem_close
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        pointer,
                        *mut uuconf_dialer,
                        boolean,
                    ) -> boolean,
            ),
            pfdial: Some(
                fmodem_dial
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
            pfbreak: Some(
                fsserial_break as unsafe extern "C" fn(*mut sconnection) -> boolean,
            ),
            pfset: Some(
                fsserial_set
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        tparitysetting,
                        tstripsetting,
                        txonxoffsetting,
                    ) -> boolean,
            ),
            pfcarrier: Some(
                fsmodem_carrier
                    as unsafe extern "C" fn(*mut sconnection, boolean) -> boolean,
            ),
            pfchat: Some(
                fsysdep_conn_chat
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *mut *mut libc::c_char,
                    ) -> boolean,
            ),
            pibaud: Some(
                isserial_baud as unsafe extern "C" fn(*mut sconnection) -> libc::c_long,
            ),
        };
        init
    }
};
static mut sdirectcmds: sconncmds = unsafe {
    {
        let mut init = sconncmds {
            pufree: Some(usserial_free as unsafe extern "C" fn(*mut sconnection) -> ()),
            pflock: Some(
                fsserial_lock
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfunlock: Some(
                fsserial_unlock as unsafe extern "C" fn(*mut sconnection) -> boolean,
            ),
            pfopen: Some(
                fsdirect_open
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        libc::c_long,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfclose: Some(
                fsdirect_close
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        pointer,
                        *mut uuconf_dialer,
                        boolean,
                    ) -> boolean,
            ),
            pfdial: None,
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
            pfbreak: Some(
                fsserial_break as unsafe extern "C" fn(*mut sconnection) -> boolean,
            ),
            pfset: Some(
                fsserial_set
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        tparitysetting,
                        tstripsetting,
                        txonxoffsetting,
                    ) -> boolean,
            ),
            pfcarrier: None,
            pfchat: Some(
                fsysdep_conn_chat
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        *mut *mut libc::c_char,
                    ) -> boolean,
            ),
            pibaud: Some(
                isserial_baud as unsafe extern "C" fn(*mut sconnection) -> libc::c_long,
            ),
        };
        init
    }
};
static mut iSunblock: libc::c_int = 0o4000 as libc::c_int | 0o4000 as libc::c_int;
pub static mut fSalarm: sig_atomic_t = 0;
unsafe extern "C" fn usalarm(mut isig: libc::c_int) {
    ::std::ptr::write_volatile(&mut fSalarm as *mut sig_atomic_t, 1 as libc::c_int);
    alarm(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn isblocksigs() -> sigset_t {
    let mut sblock: sigset_t = sigset_t { __val: [0; 16] };
    let mut sold: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut sblock);
    sigaddset(&mut sblock, 2 as libc::c_int);
    sigaddset(&mut sblock, 3 as libc::c_int);
    sigaddset(&mut sblock, 15 as libc::c_int);
    sigaddset(&mut sblock, 13 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut sblock, &mut sold);
    return sold;
}
unsafe extern "C" fn fsserial_init(
    mut qconn: *mut sconnection,
    mut qcmds: *const sconncmds,
    mut zdevice: *const libc::c_char,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    q = xmalloc(::std::mem::size_of::<ssysdep_conn>() as libc::c_ulong)
        as *mut ssysdep_conn;
    if zdevice.is_null() && !((*qconn).qport).is_null()
        && (*(*qconn).qport).uuconf_ttype as libc::c_uint
            != UUCONF_PORTTYPE_STDIN as libc::c_int as libc::c_uint
    {
        zdevice = (*(*qconn).qport).uuconf_zname;
    }
    if zdevice.is_null() {
        (*q).zdevice = 0 as *mut libc::c_char;
    } else if *zdevice as libc::c_int == '/' as i32 {
        (*q).zdevice = zbufcpy(zdevice);
    } else {
        let mut clen: size_t = 0;
        clen = strlen(zdevice);
        (*q)
            .zdevice = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(clen),
        );
        memcpy(
            (*q).zdevice as *mut libc::c_void,
            b"/dev/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            ((*q).zdevice)
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            zdevice as *const libc::c_void,
            clen,
        );
        *((*q).zdevice)
            .offset(
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_add(clen)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    (*q).o = -(1 as libc::c_int);
    (*q).ord = -(1 as libc::c_int);
    (*q).owr = -(1 as libc::c_int);
    (*q).ftli = 0 as libc::c_int;
    (*qconn).psysdep = q as pointer;
    (*qconn).qcmds = qcmds;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_stdin_init(mut qconn: *mut sconnection) -> boolean {
    chmod(
        b"/dev/tty\0" as *const u8 as *const libc::c_char,
        (0o400 as libc::c_int | 0o200 as libc::c_int) as __mode_t,
    );
    return fsserial_init(
        qconn,
        &sstdincmds,
        0 as *mut libc::c_void as *const libc::c_char,
    );
}
pub unsafe extern "C" fn fsysdep_modem_init(mut qconn: *mut sconnection) -> boolean {
    return fsserial_init(
        qconn,
        &smodemcmds,
        (*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_zdevice,
    );
}
pub unsafe extern "C" fn fsysdep_direct_init(mut qconn: *mut sconnection) -> boolean {
    return fsserial_init(
        qconn,
        &sdirectcmds,
        (*(*qconn).qport).uuconf_u.uuconf_sdirect.uuconf_zdevice,
    );
}
unsafe extern "C" fn usserial_free(mut qconn: *mut sconnection) {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    ubuffree((*qsysdep).zdevice);
    xfree(qsysdep as pointer);
    (*qconn).psysdep = 0 as *mut libc::c_void;
}
unsafe extern "C" fn fsserial_lockfile(
    mut flok: boolean,
    mut qconn: *const sconnection,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    if ((*qconn).qport).is_null() {
        z = 0 as *const libc::c_char;
    } else {
        z = (*(*qconn).qport).uuconf_zlockname;
    }
    zalc = 0 as *mut libc::c_char;
    if z.is_null() {
        let mut zbase: *const libc::c_char = 0 as *const libc::c_char;
        let mut clen: size_t = 0;
        zbase = (strrchr((*qsysdep).zdevice, '/' as i32))
            .offset(1 as libc::c_int as isize);
        clen = strlen(zbase);
        zalc = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(clen),
        );
        memcpy(
            zalc as *mut libc::c_void,
            b"LCK..\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            zalc
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            zbase as *const libc::c_void,
            clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        z = zalc;
    }
    if flok != 0 {
        fret = fsdo_lock(z, 0 as libc::c_int, 0 as *mut libc::c_void as *mut boolean);
    } else {
        fret = fsdo_unlock(z, 0 as libc::c_int);
    }
    ubuffree(zalc);
    return fret;
}
unsafe extern "C" fn fsserial_lock(
    mut qconn: *mut sconnection,
    mut fin: boolean,
    mut fuser: boolean,
) -> boolean {
    if fsserial_lockfile(1 as libc::c_int, qconn) == 0 {
        return 0 as libc::c_int;
    }
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut iflag: libc::c_int = 0;
    let mut ieuid: uid_t = 0;
    let mut iegid: gid_t = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    if fin != 0 {
        iflag = 0 as libc::c_int;
    } else {
        iflag = iSunblock;
    }
    if fuser != 0 {
        if fsuser_perms(&mut ieuid, &mut iegid) == 0 {
            fsserial_lockfile(0 as libc::c_int, qconn);
            return 0 as libc::c_int;
        }
    }
    (*qsysdep).o = open((*qsysdep).zdevice, 0o2 as libc::c_int | iflag);
    if (*qsysdep).o < 0 as libc::c_int {
        if fin == 0 && iSunblock != 0o4000 as libc::c_int
            && *__errno_location() == 22 as libc::c_int
        {
            iSunblock = 0o4000 as libc::c_int;
            (*qsysdep)
                .o = open(
                (*qsysdep).zdevice,
                0o2 as libc::c_int | 0o4000 as libc::c_int,
            );
        }
        if (*qsysdep).o < 0 as libc::c_int {
            let mut ierr: libc::c_int = 0;
            ierr = *__errno_location();
            if fuser != 0 {
                fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long);
            }
            if ierr != 16 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                    (*qsysdep).zdevice,
                    strerror(ierr),
                );
            }
            fsserial_lockfile(0 as libc::c_int, qconn);
            return 0 as libc::c_int;
        }
    }
    if fuser != 0 {
        if fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long) == 0 {
            close((*qsysdep).o);
            (*qsysdep).o = -(1 as libc::c_int);
            fsserial_lockfile(0 as libc::c_int, qconn);
            return 0 as libc::c_int;
        }
    }
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
        ioctl(
            (*qsysdep).o,
            0x5422 as libc::c_int as libc::c_ulong,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        close((*qsysdep).o);
        (*qsysdep).o = -(1 as libc::c_int);
        fsserial_lockfile(0 as libc::c_int, qconn);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsserial_unlock(mut qconn: *mut sconnection) -> boolean {
    let mut fret: boolean = 0;
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    fret = 1 as libc::c_int;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    if (*qsysdep).o >= 0 as libc::c_int {
        ioctl(
            (*qsysdep).o,
            0x5422 as libc::c_int as libc::c_ulong,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        if close((*qsysdep).o) < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"close: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            fret = 0 as libc::c_int;
        }
        (*qsysdep).o = -(1 as libc::c_int);
    }
    if fsserial_lockfile(0 as libc::c_int, qconn) == 0 {
        fret = 0 as libc::c_int;
    }
    return fret;
}
static mut asSbaud_table: [sbaud_table; 31] = [
    {
        let mut init = sbaud_table {
            icode: 0o1 as libc::c_int as baud_code,
            ibaud: 50 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o2 as libc::c_int as baud_code,
            ibaud: 75 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o3 as libc::c_int as baud_code,
            ibaud: 110 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o4 as libc::c_int as baud_code,
            ibaud: 134 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o5 as libc::c_int as baud_code,
            ibaud: 150 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o6 as libc::c_int as baud_code,
            ibaud: 200 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o7 as libc::c_int as baud_code,
            ibaud: 300 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10 as libc::c_int as baud_code,
            ibaud: 600 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o11 as libc::c_int as baud_code,
            ibaud: 1200 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o12 as libc::c_int as baud_code,
            ibaud: 1800 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o13 as libc::c_int as baud_code,
            ibaud: 2400 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o14 as libc::c_int as baud_code,
            ibaud: 4800 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o15 as libc::c_int as baud_code,
            ibaud: 9600 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o16 as libc::c_int as baud_code,
            ibaud: 19200 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o17 as libc::c_int as baud_code,
            ibaud: 38400 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10001 as libc::c_int as baud_code,
            ibaud: 57600 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10002 as libc::c_int as baud_code,
            ibaud: 115200 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10003 as libc::c_int as baud_code,
            ibaud: 230400 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10004 as libc::c_int as baud_code,
            ibaud: 460800 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10005 as libc::c_int as baud_code,
            ibaud: 500000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10006 as libc::c_int as baud_code,
            ibaud: 576000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10007 as libc::c_int as baud_code,
            ibaud: 921600 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10010 as libc::c_int as baud_code,
            ibaud: 1000000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10011 as libc::c_int as baud_code,
            ibaud: 1152000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10012 as libc::c_int as baud_code,
            ibaud: 1500000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10013 as libc::c_int as baud_code,
            ibaud: 2000000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10014 as libc::c_int as baud_code,
            ibaud: 2500000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10015 as libc::c_int as baud_code,
            ibaud: 3000000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10016 as libc::c_int as baud_code,
            ibaud: 3500000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0o10017 as libc::c_int as baud_code,
            ibaud: 4000000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = sbaud_table {
            icode: 0 as libc::c_int as baud_code,
            ibaud: 0 as libc::c_int as libc::c_long,
        };
        init
    },
];
static mut cSmin: libc::c_int = 0;
unsafe extern "C" fn fsserial_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
    mut tlocal: tclocal_setting,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut ib: baud_code = 0;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    if !((*q).zdevice).is_null() {
        let mut z: *const libc::c_char = 0 as *const libc::c_char;
        if strncmp(
            (*q).zdevice,
            b"/dev/\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            z = ((*q).zdevice)
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize));
        } else {
            z = (*q).zdevice;
        }
        ulog_device(z);
    } else {
        let mut zport: *const libc::c_char = 0 as *const libc::c_char;
        let mut fdummy: boolean = 0;
        if !((*qconn).qport).is_null()
            && (*(*qconn).qport).uuconf_ttype as libc::c_uint
                != UUCONF_PORTTYPE_STDIN as libc::c_int as libc::c_uint
        {
            ulog(
                LOG_FATAL,
                b"fsserial_open: Can't happen\0" as *const u8 as *const libc::c_char,
            );
        }
        zport = zsysdep_port_name(&mut fdummy);
        if !zport.is_null() {
            ulog_device(zport);
        }
    }
    ib = 0 as libc::c_int as baud_code;
    if ibaud != 0 as libc::c_int as libc::c_long {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[sbaud_table; 31]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<sbaud_table>() as libc::c_ulong)
        {
            if asSbaud_table[i as usize].ibaud == ibaud {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i
            >= (::std::mem::size_of::<[sbaud_table; 31]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<sbaud_table>() as libc::c_ulong)
        {
            ulog(
                LOG_ERROR,
                b"Unsupported baud rate %ld\0" as *const u8 as *const libc::c_char,
                ibaud,
            );
            return 0 as libc::c_int;
        }
        ib = asSbaud_table[i as usize].icode;
    }
    if (*q).o < 0 as libc::c_int {
        let mut iflag: libc::c_int = 0;
        let mut ieuid: uid_t = 0;
        let mut iegid: gid_t = 0;
        if fwait != 0 {
            iflag = 0 as libc::c_int;
        } else {
            iflag = iSunblock;
        }
        if fuser != 0 {
            if fsuser_perms(&mut ieuid, &mut iegid) == 0 {
                return 0 as libc::c_int;
            }
        }
        (*q).o = open((*q).zdevice, 0o2 as libc::c_int | iflag);
        if (*q).o < 0 as libc::c_int {
            if fwait == 0 && iSunblock != 0o4000 as libc::c_int
                && *__errno_location() == 22 as libc::c_int
            {
                iSunblock = 0o4000 as libc::c_int;
                (*q).o = open((*q).zdevice, 0o2 as libc::c_int | 0o4000 as libc::c_int);
            }
            if (*q).o < 0 as libc::c_int {
                let mut ierr: libc::c_int = 0;
                ierr = *__errno_location();
                if fuser != 0 {
                    fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long);
                }
                ulog(
                    LOG_ERROR,
                    b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                    (*q).zdevice,
                    strerror(ierr),
                );
                return 0 as libc::c_int;
            }
        }
        if fuser != 0 {
            if fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long) == 0 {
                return 0 as libc::c_int;
            }
        }
        if fcntl(
            (*q).o,
            2 as libc::c_int,
            fcntl((*q).o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
        ) < 0 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    (*q).iflags = fcntl((*q).o, 3 as libc::c_int, 0 as libc::c_int);
    if (*q).iflags < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fcntl: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    (*q).iwr_flags = -(1 as libc::c_int);
    if fsblock(q, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    if !(tcgetattr((*q).o, &mut (*q).sorig) == 0 as libc::c_int) {
        (*q).fterminal = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    (*q).fterminal = 1 as libc::c_int;
    (*q).snew = (*q).sorig;
    if ibaud == 0 as libc::c_int as libc::c_long {
        ib = cfgetospeed(&mut (*q).snew);
    }
    (*q).snew.c_iflag
        &= !(0o2 as libc::c_int | 0o400 as libc::c_int | 0o1 as libc::c_int
            | 0o200 as libc::c_int | 0o4 as libc::c_int | 0o100 as libc::c_int
            | 0o20 as libc::c_int | 0o40 as libc::c_int | 0o10000 as libc::c_int
            | 0o2000 as libc::c_int | 0o10 as libc::c_int | 0o20000 as libc::c_int)
            as libc::c_uint;
    (*q).snew.c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
    (*q).snew.c_cflag
        &= !(0o60 as libc::c_int | 0o400 as libc::c_int | 0o1000 as libc::c_int)
            as libc::c_uint;
    (*q).snew.c_cflag
        |= (0o60 as libc::c_int | 0o200 as libc::c_int | 0o2000 as libc::c_int)
            as libc::c_uint;
    (*q).snew.c_lflag
        &= !(0o10 as libc::c_int | 0o20 as libc::c_int | 0o40 as libc::c_int
            | 0o100 as libc::c_int | 0o2 as libc::c_int | 0o100000 as libc::c_int
            | 0o1 as libc::c_int | 0o200 as libc::c_int | 0o400 as libc::c_int
            | 0o40000 as libc::c_int) as libc::c_uint;
    cSmin = 1 as libc::c_int;
    (*q).snew.c_cc[6 as libc::c_int as usize] = cSmin as cc_t;
    (*q).snew.c_cc[5 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    cfsetospeed(&mut (*q).snew, ib);
    cfsetispeed(&mut (*q).snew, ib);
    tcflush((*q).o, 0 as libc::c_int);
    match tlocal as libc::c_uint {
        0 => {
            (*q).snew.c_cflag |= 0o4000 as libc::c_int as libc::c_uint;
        }
        1 => {
            (*q).snew.c_cflag &= !(0o4000 as libc::c_int) as libc::c_uint;
        }
        2 | _ => {}
    }
    if !(tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).snew) == 0 as libc::c_int) {
        ulog(
            LOG_ERROR,
            b"Can't set terminal settings: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    ioctl((*q).o, 0x540e as libc::c_int as libc::c_ulong, 0 as libc::c_int);
    if ibaud != 0 as libc::c_int as libc::c_long {
        (*q).ibaud = ibaud;
    } else {
        let mut i_0: size_t = 0;
        (*q).ibaud = 1200 as libc::c_int as libc::c_long;
        i_0 = 0 as libc::c_int as size_t;
        while i_0
            < (::std::mem::size_of::<[sbaud_table; 31]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<sbaud_table>() as libc::c_ulong)
        {
            if asSbaud_table[i_0 as usize].icode == ib
                && asSbaud_table[i_0 as usize].ibaud != 0 as libc::c_int as libc::c_long
            {
                (*q).ibaud = asSbaud_table[i_0 as usize].ibaud;
                break;
            } else {
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        }
        if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fsserial_open: Baud rate is %ld\0" as *const u8 as *const libc::c_char,
                (*q).ibaud,
            );
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsstdin_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    (*q).ord = 0 as libc::c_int;
    (*q).owr = 1 as libc::c_int;
    (*q).o = (*q).ord;
    if fsserial_open(
        qconn,
        ibaud,
        fwait,
        fuser,
        IGNORE_CLOCAL as libc::c_int as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*q).iwr_flags = fcntl((*q).owr, 3 as libc::c_int, 0 as libc::c_int);
    if (*q).iwr_flags < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fcntl: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsmodem_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
) -> boolean {
    let mut qm: *mut uuconf_modem_port = 0 as *mut uuconf_modem_port;
    qm = &mut (*(*qconn).qport).uuconf_u.uuconf_smodem;
    if ibaud == 0 as libc::c_int as libc::c_long {
        ibaud = (*qm).uuconf_ibaud;
    }
    if fsserial_open(
        qconn,
        ibaud,
        fwait,
        fuser,
        (if fwait != 0 {
            CLEAR_CLOCAL as libc::c_int
        } else {
            SET_CLOCAL as libc::c_int
        }) as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if fwait != 0 && fsserial_hardflow(qconn, (*qm).uuconf_fhardflow) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsdirect_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
) -> boolean {
    let mut qd: *mut uuconf_direct_port = 0 as *mut uuconf_direct_port;
    qd = &mut (*(*qconn).qport).uuconf_u.uuconf_sdirect;
    if ibaud == 0 as libc::c_int as libc::c_long {
        ibaud = (*qd).uuconf_ibaud;
    }
    if fsserial_open(
        qconn,
        ibaud,
        fwait,
        fuser,
        (if (*qd).uuconf_fcarrier != 0 {
            CLEAR_CLOCAL as libc::c_int
        } else {
            SET_CLOCAL as libc::c_int
        }) as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return fsserial_hardflow(qconn, (*qd).uuconf_fhardflow);
}
unsafe extern "C" fn fsblock(mut qs: *mut ssysdep_conn, mut fblock: boolean) -> boolean {
    let mut iwant: libc::c_int = 0;
    let mut isys: libc::c_int = 0;
    if fblock != 0 {
        iwant = (*qs).iflags & !(0o4000 as libc::c_int | 0o4000 as libc::c_int);
    } else {
        iwant = (*qs).iflags | iSunblock;
    }
    if iwant == (*qs).iflags {
        return 1 as libc::c_int;
    }
    isys = fcntl((*qs).o, 4 as libc::c_int, iwant);
    if isys < 0 as libc::c_int {
        if fblock == 0 && iSunblock != 0o4000 as libc::c_int
            && *__errno_location() == 22 as libc::c_int
        {
            iSunblock = 0o4000 as libc::c_int;
            iwant = (*qs).iflags | 0o4000 as libc::c_int;
            isys = fcntl((*qs).o, 4 as libc::c_int, iwant);
        }
        if isys < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"fcntl: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    (*qs).iflags = iwant;
    if (*qs).iwr_flags >= 0 as libc::c_int && (*qs).ord != (*qs).owr {
        if fblock != 0 {
            iwant = (*qs).iwr_flags & !(0o4000 as libc::c_int | 0o4000 as libc::c_int);
        } else {
            iwant = (*qs).iwr_flags | iSunblock;
        }
        isys = fcntl((*qs).owr, 4 as libc::c_int, iwant);
        if isys < 0 as libc::c_int {
            if fblock == 0 && iSunblock != 0o4000 as libc::c_int
                && *__errno_location() == 22 as libc::c_int
            {
                iSunblock = 0o4000 as libc::c_int;
                iwant = (*qs).iwr_flags | 0o4000 as libc::c_int;
                isys = fcntl((*qs).owr, 4 as libc::c_int, iwant);
            }
            if isys < 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"fcntl: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
        }
        (*qs).iwr_flags = iwant;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsserial_close(mut q: *mut ssysdep_conn) -> boolean {
    if (*q).o >= 0 as libc::c_int {
        if (*q).fterminal != 0 {
            ::std::ptr::write_volatile(
                &mut fSalarm as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            usset_signal(
                14 as libc::c_int,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(libc::c_int) -> (),
                            unsafe extern "C" fn() -> (),
                        >(usalarm),
                    ),
                ),
                1 as libc::c_int,
                0 as *mut libc::c_void as *mut boolean,
            );
            alarm(30 as libc::c_int as libc::c_uint);
            tcsetattr((*q).o, 1 as libc::c_int, &mut (*q).sorig);
            usset_signal(
                14 as libc::c_int,
                ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t),
                1 as libc::c_int,
                0 as *mut libc::c_void as *mut boolean,
            );
            alarm(0 as libc::c_int as libc::c_uint);
            if fSalarm != 0 {
                tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).sorig);
            }
        }
        ioctl(
            (*q).o,
            0x5422 as libc::c_int as libc::c_ulong,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        close((*q).o);
        (*q).o = -(1 as libc::c_int);
        sleep(2 as libc::c_int as libc::c_uint);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsstdin_close(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdialer: *mut uuconf_dialer,
    mut fsuccess: boolean,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    close((*qsysdep).owr);
    close(2 as libc::c_int);
    (*qsysdep).o = (*qsysdep).ord;
    return fsserial_close(qsysdep);
}
unsafe extern "C" fn fsmodem_close(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdialer: *mut uuconf_dialer,
    mut fsuccess: boolean,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut fret: boolean = 0;
    let mut sdialer: uuconf_dialer = uuconf_dialer {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_schat: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zdialtone: 0 as *mut libc::c_char,
        uuconf_zpause: 0 as *mut libc::c_char,
        uuconf_fcarrier: 0,
        uuconf_ccarrier_wait: 0,
        uuconf_fdtr_toggle: 0,
        uuconf_fdtr_toggle_wait: 0,
        uuconf_scomplete: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_sabort: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_ireliable: 0,
        uuconf_palloc: 0 as *mut libc::c_void,
    };
    let mut qchat: *const uuconf_chat = 0 as *const uuconf_chat;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    fret = 1 as libc::c_int;
    if qdialer.is_null() {
        if !((*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_pzdialer).is_null() {
            let mut zdialer: *const libc::c_char = 0 as *const libc::c_char;
            let mut iuuconf: libc::c_int = 0;
            zdialer = *((*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_pzdialer)
                .offset(0 as libc::c_int as isize);
            iuuconf = uuconf_dialer_info(puuconf, zdialer, &mut sdialer);
            if iuuconf == 0 as libc::c_int {
                qdialer = &mut sdialer;
            } else {
                ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                fret = 0 as libc::c_int;
            }
        } else {
            qdialer = (*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_qdialer;
        }
    }
    qchat = 0 as *const uuconf_chat;
    if !qdialer.is_null() {
        if fsuccess != 0 {
            qchat = &mut (*qdialer).uuconf_scomplete;
        } else {
            qchat = &mut (*qdialer).uuconf_sabort;
        }
    }
    if !qchat.is_null()
        && (!((*qchat).uuconf_pzprogram).is_null()
            || !((*qchat).uuconf_pzchat).is_null())
    {
        let mut fsighup_ignored: boolean = 0;
        let mut smask: sigset_t = sigset_t { __val: [0; 16] };
        let mut i: libc::c_int = 0;
        let mut afhold: [sig_atomic_t; 5] = [0; 5];
        fsmodem_carrier(qconn, 0 as libc::c_int);
        usset_signal(
            1 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
            0 as libc::c_int,
            &mut fsighup_ignored,
        );
        smask = isblocksigs();
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            afhold[i as usize] = afSignal[i as usize];
            ::std::ptr::write_volatile(
                &mut afSignal[i as usize] as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            i += 1;
            i;
        }
        sigprocmask(
            2 as libc::c_int,
            &mut smask,
            0 as *mut libc::c_void as *mut sigset_t,
        );
        if fchat(
            qconn,
            puuconf,
            qchat,
            0 as *mut libc::c_void as *const uuconf_system,
            0 as *mut libc::c_void as *const uuconf_dialer,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as libc::c_int,
            (*(*qconn).qport).uuconf_zname,
            (*qsysdep).ibaud,
        ) == 0
        {
            fret = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            if afhold[i as usize] != 0 {
                ::std::ptr::write_volatile(
                    &mut afSignal[i as usize] as *mut sig_atomic_t,
                    1 as libc::c_int,
                );
            }
            i += 1;
            i;
        }
        if fsighup_ignored == 0 {
            usset_signal(
                1 as libc::c_int,
                Some(ussignal as unsafe extern "C" fn(libc::c_int) -> ()),
                1 as libc::c_int,
                0 as *mut libc::c_void as *mut boolean,
            );
        }
    }
    if !qdialer.is_null() && qdialer == &mut sdialer as *mut uuconf_dialer {
        uuconf_free_block(sdialer.uuconf_palloc);
    }
    if (*qsysdep).fterminal != 0 {
        cfsetospeed(&mut (*qsysdep).snew, 0 as libc::c_int as speed_t);
        ::std::ptr::write_volatile(&mut fSalarm as *mut sig_atomic_t, 0 as libc::c_int);
        usset_signal(
            14 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(usalarm),
                ),
            ),
            1 as libc::c_int,
            0 as *mut libc::c_void as *mut boolean,
        );
        alarm(30 as libc::c_int as libc::c_uint);
        tcsetattr((*qsysdep).o, 1 as libc::c_int, &mut (*qsysdep).snew);
        usset_signal(
            14 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
            1 as libc::c_int,
            0 as *mut libc::c_void as *mut boolean,
        );
        alarm(0 as libc::c_int as libc::c_uint);
        sleep(2 as libc::c_int as libc::c_uint);
    }
    if fsserial_close(qsysdep) == 0 {
        fret = 0 as libc::c_int;
    }
    return fret;
}
unsafe extern "C" fn fsdirect_close(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdialer: *mut uuconf_dialer,
    mut fsuccess: boolean,
) -> boolean {
    return fsserial_close((*qconn).psysdep as *mut ssysdep_conn);
}
pub unsafe extern "C" fn fsysdep_modem_begin_dial(
    mut qconn: *mut sconnection,
    mut qdial: *mut uuconf_dialer,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    if (*qdial).uuconf_fdtr_toggle != 0 {
        if (*qsysdep).fterminal != 0 {
            let mut sbaud: sterminal = sterminal {
                c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,
            };
            sbaud = (*qsysdep).snew;
            cfsetospeed(&mut sbaud, 0 as libc::c_int as speed_t);
            tcsetattr((*qsysdep).o, 1 as libc::c_int, &mut sbaud);
            sleep(2 as libc::c_int as libc::c_uint);
            tcsetattr((*qsysdep).o, 0 as libc::c_int, &mut (*qsysdep).snew);
        }
        if (*qdial).uuconf_fdtr_toggle_wait != 0 {
            sleep(2 as libc::c_int as libc::c_uint);
        }
    }
    if fsmodem_carrier(qconn, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    z = (*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_zdial_device;
    if !z.is_null() {
        let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut o: libc::c_int = 0;
        (*qsysdep).ohold = (*qsysdep).o;
        zfree = 0 as *mut libc::c_char;
        if *z as libc::c_int != '/' as i32 {
            zfree = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_add(strlen(z)),
            );
            sprintf(zfree, b"/dev/%s\0" as *const u8 as *const libc::c_char, z);
            z = zfree;
        }
        o = open(z as *mut libc::c_char, 0o2 as libc::c_int | 0o400 as libc::c_int);
        if o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                z,
                strerror(*__errno_location()),
            );
            ubuffree(zfree);
            return 0 as libc::c_int;
        }
        ubuffree(zfree);
        if fcntl(
            o,
            2 as libc::c_int,
            fcntl(o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
        ) < 0 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            close(o);
            return 0 as libc::c_int;
        }
        (*qsysdep).o = o;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsmodem_carrier(
    mut qconn: *mut sconnection,
    mut fcarrier: boolean,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut qm: *mut uuconf_modem_port = 0 as *mut uuconf_modem_port;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    if (*q).fterminal == 0 {
        return 1 as libc::c_int;
    }
    qm = &mut (*(*qconn).qport).uuconf_u.uuconf_smodem;
    if fcarrier != 0 {
        if (*qm).uuconf_fcarrier != 0 {
            (*q).snew.c_cflag &= !(0o4000 as libc::c_int) as libc::c_uint;
            if !(tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).snew) == 0 as libc::c_int)
            {
                ulog(
                    LOG_ERROR,
                    b"Can't clear CLOCAL: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
        }
        if fsserial_hardflow(qconn, (*qm).uuconf_fhardflow) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        if fsserial_hardflow(qconn, 0 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        (*q).snew.c_cflag |= 0o4000 as libc::c_int as libc::c_uint;
        if !(tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).snew) == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"Can't set CLOCAL: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsserial_hardflow(
    mut qconn: *mut sconnection,
    mut fhardflow: boolean,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    if (*q).fterminal == 0 {
        return 1 as libc::c_int;
    }
    if fhardflow != 0 {
        (*q).snew.c_cflag |= 0o20000000000 as libc::c_uint;
        if !(tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).snew) == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"Can't enable hardware flow control: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    } else {
        (*q).snew.c_cflag &= !(0o20000000000 as libc::c_uint);
        if !(tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).snew) == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"Can't disable hardware flow control: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_modem_end_dial(
    mut qconn: *mut sconnection,
    mut qdial: *mut uuconf_dialer,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    if !((*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_zdial_device).is_null() {
        close((*q).o);
        (*q).o = (*q).ohold;
    }
    if (*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_fcarrier != 0
        && (*qdial).uuconf_fcarrier != 0
    {
        if fsmodem_carrier(qconn, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        let mut onew: libc::c_int = 0;
        onew = open((*q).zdevice, 0o2 as libc::c_int);
        if onew >= 0 as libc::c_int {
            let mut fbad: boolean = 0;
            let mut iflags: libc::c_int = 0;
            fbad = 0 as libc::c_int;
            if fcntl(
                onew,
                2 as libc::c_int,
                fcntl(onew, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
            ) < 0 as libc::c_int
            {
                fbad = 1 as libc::c_int;
            }
            if fbad == 0 {
                iflags = fcntl(onew, 3 as libc::c_int, 0 as libc::c_int);
                if iflags < 0 as libc::c_int
                    || !(tcsetattr(onew, 0 as libc::c_int, &mut (*q).snew)
                        == 0 as libc::c_int)
                {
                    fbad = 1 as libc::c_int;
                }
            }
            if fbad != 0 {
                close(onew);
            } else {
                close((*q).o);
                (*q).o = onew;
                (*q).iflags = iflags;
            }
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_conn_read(
    mut qconn: *mut sconnection,
    mut zbuf: *mut libc::c_char,
    mut pclen: *mut size_t,
    mut cmin: size_t,
    mut ctimeout: libc::c_int,
    mut freport: boolean,
) -> boolean {
    let mut cwant: size_t = 0;
    let mut fret: boolean = 0;
    let q: *mut ssysdep_conn = (*qconn).psysdep as *mut ssysdep_conn;
    let mut cwouldblock: libc::c_int = 0;
    cwant = *pclen;
    *pclen = 0 as libc::c_int as size_t;
    if ctimeout <= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if fsblock(q, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    ::std::ptr::write_volatile(&mut fSalarm as *mut sig_atomic_t, 0 as libc::c_int);
    usset_signal(
        14 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(usalarm),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    alarm(ctimeout as libc::c_uint);
    fret = 0 as libc::c_int;
    cwouldblock = 0 as libc::c_int;
    loop {
        let mut cgot: libc::c_int = 0;
        if (*q).fterminal != 0 {
            let mut csetmin: libc::c_int = 0;
            if cmin < 127 as libc::c_int as libc::c_ulong {
                csetmin = cmin as libc::c_int;
            } else {
                csetmin = 127 as libc::c_int;
            }
            if csetmin != cSmin {
                (*q).snew.c_cc[6 as libc::c_int as usize] = csetmin as cc_t;
                while !(tcsetattr((*q).o, 0 as libc::c_int, &mut (*q).snew)
                    == 0 as libc::c_int)
                {
                    if *__errno_location() != 4 as libc::c_int
                        || (afSignal[0 as libc::c_int as usize] != 0
                            || afSignal[2 as libc::c_int as usize] != 0
                            || afSignal[3 as libc::c_int as usize] != 0
                            || afSignal[4 as libc::c_int as usize] != 0)
                    {
                        let mut ierr: libc::c_int = 0;
                        ierr = *__errno_location();
                        usset_signal(
                            14 as libc::c_int,
                            ::std::mem::transmute::<
                                libc::intptr_t,
                                __sighandler_t,
                            >(1 as libc::c_int as libc::intptr_t),
                            1 as libc::c_int,
                            0 as *mut libc::c_void as *mut boolean,
                        );
                        alarm(0 as libc::c_int as libc::c_uint);
                        ulog(
                            LOG_ERROR,
                            b"Can't set MIN for terminal: %s\0" as *const u8
                                as *const libc::c_char,
                            strerror(ierr),
                        );
                        return 0 as libc::c_int;
                    }
                    if fSalarm != 0 {
                        ulog(
                            LOG_ERROR,
                            b"Timed out when setting MIN to %d; retrying\0" as *const u8
                                as *const libc::c_char,
                            csetmin,
                        );
                        ::std::ptr::write_volatile(
                            &mut fSalarm as *mut sig_atomic_t,
                            0 as libc::c_int,
                        );
                        alarm(ctimeout as libc::c_uint);
                    }
                }
                cSmin = csetmin;
            }
        }
        if afSignal[0 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0
        {
            break;
        }
        if fSalarm != 0 {
            fret = 1 as libc::c_int;
            break;
        } else {
            cgot = read((*q).o, zbuf as *mut libc::c_void, cwant) as libc::c_int;
            if cgot < 0 as libc::c_int {
                if *__errno_location() == 4 as libc::c_int {
                    ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
                }
                if fSalarm != 0 {
                    fret = 1 as libc::c_int;
                    break;
                } else if afSignal[0 as libc::c_int as usize] != 0
                    || afSignal[2 as libc::c_int as usize] != 0
                    || afSignal[3 as libc::c_int as usize] != 0
                    || afSignal[4 as libc::c_int as usize] != 0
                {
                    break;
                }
            }
            if cgot > 0 as libc::c_int {
                cwouldblock = 0 as libc::c_int;
            } else if cgot < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int
            {
                cgot = 0 as libc::c_int;
            } else if cgot < 0 as libc::c_int
                && (*__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int)
                && cwouldblock < 2 as libc::c_int
            {
                cwouldblock += 1;
                cwouldblock;
                cgot = 0 as libc::c_int;
            } else {
                let mut ierr_0: libc::c_int = 0;
                ierr_0 = *__errno_location();
                usset_signal(
                    14 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                    1 as libc::c_int,
                    0 as *mut libc::c_void as *mut boolean,
                );
                alarm(0 as libc::c_int as libc::c_uint);
                if freport != 0 {
                    if cgot == 0 as libc::c_int {
                        ulog(
                            LOG_ERROR,
                            b"Line disconnected\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        ulog(
                            LOG_ERROR,
                            b"read: %s\0" as *const u8 as *const libc::c_char,
                            strerror(ierr_0),
                        );
                    }
                }
                return 0 as libc::c_int;
            }
            cwant = (cwant as libc::c_ulong).wrapping_sub(cgot as libc::c_ulong)
                as size_t as size_t;
            if cgot as size_t >= cmin {
                cmin = 0 as libc::c_int as size_t;
            } else {
                cmin = (cmin as libc::c_ulong).wrapping_sub(cgot as libc::c_ulong)
                    as size_t as size_t;
            }
            zbuf = zbuf.offset(cgot as isize);
            *pclen = (*pclen as libc::c_ulong).wrapping_add(cgot as libc::c_ulong)
                as size_t as size_t;
            if !(cmin == 0 as libc::c_int as libc::c_ulong) {
                continue;
            }
            fret = 1 as libc::c_int;
            break;
        }
    }
    usset_signal(
        14 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    alarm(0 as libc::c_int as libc::c_uint);
    return fret;
}
pub unsafe extern "C" fn fsdouble_read(
    mut qconn: *mut sconnection,
    mut zbuf: *mut libc::c_char,
    mut pclen: *mut size_t,
    mut cmin: size_t,
    mut ctimeout: libc::c_int,
    mut freport: boolean,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    (*qsysdep).o = (*qsysdep).ord;
    return fsysdep_conn_read(qconn, zbuf, pclen, cmin, ctimeout, freport);
}
pub unsafe extern "C" fn fsysdep_conn_write(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut cwrite: size_t,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut czero: libc::c_int = 0;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    if fsblock(q, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    czero = 0 as libc::c_int;
    while cwrite > 0 as libc::c_int as libc::c_ulong {
        let mut cdid: libc::c_int = 0;
        loop {
            if afSignal[0 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0
            {
                return 0 as libc::c_int;
            }
            cdid = write((*q).o, zwrite as *const libc::c_void, cwrite) as libc::c_int;
            if cdid >= 0 as libc::c_int {
                break;
            }
            if *__errno_location() != 4 as libc::c_int {
                break;
            }
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
        }
        if cdid < 0 as libc::c_int {
            if *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 61 as libc::c_int
            {
                ulog(
                    LOG_ERROR,
                    b"write: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            cdid = 0 as libc::c_int;
        }
        if cdid == 0 as libc::c_int {
            czero += 1;
            czero;
            if czero >= 10 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"Line disconnected\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
        } else {
            czero = 0 as libc::c_int;
            cwrite = (cwrite as libc::c_ulong).wrapping_sub(cdid as libc::c_ulong)
                as size_t as size_t;
            zwrite = zwrite.offset(cdid as isize);
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsdouble_write(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut cwrite: size_t,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    (*qsysdep).o = (*qsysdep).ord;
    if fsblock(qsysdep, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    (*qsysdep).o = (*qsysdep).owr;
    return fsysdep_conn_write(qconn, zwrite, cwrite);
}
pub unsafe extern "C" fn fsysdep_conn_io(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut pcwrite: *mut size_t,
    mut zread: *mut libc::c_char,
    mut pcread: *mut size_t,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut cwrite: size_t = 0;
    let mut cread: size_t = 0;
    let mut czero: libc::c_int = 0;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    cwrite = *pcwrite;
    *pcwrite = 0 as libc::c_int as size_t;
    cread = *pcread;
    *pcread = 0 as libc::c_int as size_t;
    czero = 0 as libc::c_int;
    loop {
        let mut cgot: libc::c_int = 0;
        let mut cdid: libc::c_int = 0;
        let mut cdo: size_t = 0;
        if (*q).ord >= 0 as libc::c_int {
            (*q).o = (*q).ord;
        }
        if fsblock(q, 0 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        loop {
            if afSignal[0 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0
            {
                return 0 as libc::c_int;
            }
            cgot = read((*q).o, zread as *mut libc::c_void, cread) as libc::c_int;
            if cgot >= 0 as libc::c_int {
                break;
            }
            if *__errno_location() != 4 as libc::c_int {
                break;
            }
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
        }
        if cgot < 0 as libc::c_int {
            if *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 61 as libc::c_int
            {
                ulog(
                    LOG_ERROR,
                    b"read: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            cgot = 0 as libc::c_int;
        }
        cread = (cread as libc::c_ulong).wrapping_sub(cgot as libc::c_ulong) as size_t
            as size_t;
        zread = zread.offset(cgot as isize);
        *pcread = (*pcread as libc::c_ulong).wrapping_add(cgot as libc::c_ulong)
            as size_t as size_t;
        if cread == 0 as libc::c_int as libc::c_ulong
            || cwrite == 0 as libc::c_int as libc::c_ulong
        {
            return 1 as libc::c_int;
        }
        cdo = cwrite;
        if (*q).owr >= 0 as libc::c_int {
            (*q).o = (*q).owr;
        }
        loop {
            if afSignal[0 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0
            {
                return 0 as libc::c_int;
            }
            cdid = write((*q).o, zwrite as *const libc::c_void, cdo) as libc::c_int;
            if cdid >= 0 as libc::c_int {
                break;
            }
            if *__errno_location() != 4 as libc::c_int {
                break;
            }
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
        }
        if cdid < 0 as libc::c_int {
            if *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 61 as libc::c_int
            {
                ulog(
                    LOG_ERROR,
                    b"write: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            cdid = 0 as libc::c_int;
        }
        if cdid > 0 as libc::c_int {
            cwrite = (cwrite as libc::c_ulong).wrapping_sub(cdid as libc::c_ulong)
                as size_t as size_t;
            zwrite = zwrite.offset(cdid as isize);
            *pcwrite = (*pcwrite as libc::c_ulong).wrapping_add(cdid as libc::c_ulong)
                as size_t as size_t;
            if cwrite == 0 as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int;
            }
            czero = 0 as libc::c_int;
        } else {
            let mut stime: timeval = timeval { tv_sec: 0, tv_usec: 0 };
            let mut smask: fd_set = fd_set { __fds_bits: [0; 16] };
            let mut c: libc::c_int = 0;
            if (*q).fterminal != 0 {
                let mut cwait: libc::c_ulong = 0;
                cwait = 1024 as libc::c_int as libc::c_ulong;
                if cwait > cread {
                    cwait = cread;
                }
                stime
                    .tv_sec = cwait
                    .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                    .wrapping_div((*q).ibaud as libc::c_ulong) as __time_t;
                stime
                    .tv_usec = cwait
                    .wrapping_mul(1000000 as libc::c_int as libc::c_ulong)
                    .wrapping_div((*q).ibaud as libc::c_ulong)
                    .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                    .wrapping_rem(1000000 as libc::c_int as libc::c_ulong)
                    as __suseconds_t;
            } else {
                stime.tv_sec = 1 as libc::c_int as __time_t;
                stime.tv_usec = 0 as libc::c_int as __suseconds_t;
            }
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh0 = &mut __d0;
            let fresh1;
            let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh3 = &mut __d1;
            let fresh4;
            let fresh5 = &mut *(smask.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                fresh4, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
            smask
                .__fds_bits[((*q).o
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << (*q).o
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if afSignal[0 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0
            {
                return 0 as libc::c_int;
            }
            if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fsysdep_conn_io: Calling select\0" as *const u8
                        as *const libc::c_char,
                );
            }
            c = select(
                (*q).o + 1 as libc::c_int,
                0 as *mut libc::c_void as *mut fd_set,
                &mut smask as *mut fd_set as pointer as *mut fd_set,
                0 as *mut libc::c_void as *mut fd_set,
                &mut stime,
            );
            if c < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
            } else if !(c >= 0 as libc::c_int) {
                let mut ierr: libc::c_int = 0;
                if (*q).ord >= 0 as libc::c_int {
                    (*q).o = (*q).ord;
                }
                if fsblock(q, 1 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fsysdep_conn_io: Blocking write\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if (*q).owr >= 0 as libc::c_int {
                    (*q).o = (*q).owr;
                }
                if afSignal[0 as libc::c_int as usize] != 0
                    || afSignal[2 as libc::c_int as usize] != 0
                    || afSignal[3 as libc::c_int as usize] != 0
                    || afSignal[4 as libc::c_int as usize] != 0
                {
                    return 0 as libc::c_int;
                }
                usset_signal(
                    14 as libc::c_int,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> ()>,
                        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
                    >(
                        Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn(libc::c_int) -> (),
                                unsafe extern "C" fn() -> (),
                            >(usalarm),
                        ),
                    ),
                    1 as libc::c_int,
                    0 as *mut libc::c_void as *mut boolean,
                );
                if (*q).fterminal != 0 {
                    alarm(
                        ((10240 as libc::c_int as libc::c_long / (*q).ibaud)
                            as libc::c_int + 1 as libc::c_int) as libc::c_uint,
                    );
                } else {
                    alarm(1 as libc::c_int as libc::c_uint);
                }
                cdid = write(
                    (*q).o,
                    zwrite as *const libc::c_void,
                    1 as libc::c_int as size_t,
                ) as libc::c_int;
                ierr = *__errno_location();
                usset_signal(
                    14 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                    1 as libc::c_int,
                    0 as *mut libc::c_void as *mut boolean,
                );
                alarm(0 as libc::c_int as libc::c_uint);
                if cdid < 0 as libc::c_int {
                    if ierr == 4 as libc::c_int {
                        ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
                    } else {
                        ulog(
                            LOG_ERROR,
                            b"write: %s\0" as *const u8 as *const libc::c_char,
                            strerror(ierr),
                        );
                        return 0 as libc::c_int;
                    }
                } else if cdid == 0 as libc::c_int {
                    czero += 1;
                    czero;
                    if czero >= 10 as libc::c_int {
                        ulog(
                            LOG_ERROR,
                            b"Line disconnected\0" as *const u8 as *const libc::c_char,
                        );
                        return 0 as libc::c_int;
                    }
                } else {
                    cwrite = (cwrite as libc::c_ulong)
                        .wrapping_sub(cdid as libc::c_ulong) as size_t as size_t;
                    zwrite = zwrite.offset(cdid as isize);
                    *pcwrite = (*pcwrite as libc::c_ulong)
                        .wrapping_add(cdid as libc::c_ulong) as size_t as size_t;
                    czero = 0 as libc::c_int;
                }
            }
        }
    };
}
unsafe extern "C" fn fsserial_break(mut qconn: *mut sconnection) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    return (tcsendbreak((*q).o, 0 as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn fsstdin_break(mut qconn: *mut sconnection) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    (*qsysdep).o = (*qsysdep).owr;
    return fsserial_break(qconn);
}
unsafe extern "C" fn fsserial_set(
    mut qconn: *mut sconnection,
    mut tparity: tparitysetting,
    mut tstrip: tstripsetting,
    mut txonxoff: txonxoffsetting,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut fchanged: boolean = 0;
    let mut fdo: boolean = 0;
    let mut iset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut iclear: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    if (*q).fterminal == 0 {
        return 1 as libc::c_int;
    }
    fchanged = 0 as libc::c_int;
    fdo = 0 as libc::c_int;
    match tparity as libc::c_uint {
        1 => {
            iset = 0o60 as libc::c_int as libc::c_uint;
            iclear = (0o400 as libc::c_int | 0o1000 as libc::c_int
                | 0o60 as libc::c_int & !(0o60 as libc::c_int)) as libc::c_uint;
            fdo = 1 as libc::c_int;
        }
        2 => {
            iset = (0o400 as libc::c_int | 0o40 as libc::c_int) as libc::c_uint;
            iclear = (0o1000 as libc::c_int
                | 0o60 as libc::c_int & !(0o40 as libc::c_int)) as libc::c_uint;
            fdo = 1 as libc::c_int;
        }
        3 => {
            iset = (0o400 as libc::c_int | 0o1000 as libc::c_int | 0o40 as libc::c_int)
                as libc::c_uint;
            iclear = (0o60 as libc::c_int & !(0o40 as libc::c_int)) as libc::c_uint;
            fdo = 1 as libc::c_int;
        }
        0 | 4 | 5 | _ => {}
    }
    if fdo != 0 {
        if (*q).snew.c_cflag & iset != iset
            || (*q).snew.c_cflag & iclear != 0 as libc::c_int as libc::c_uint
        {
            (*q).snew.c_cflag |= iset;
            (*q).snew.c_cflag &= !iclear;
            fchanged = 1 as libc::c_int;
        }
    }
    fdo = 0 as libc::c_int;
    match tstrip as libc::c_uint {
        1 => {
            iset = 0 as libc::c_int as libc::c_uint;
            iclear = 0o40 as libc::c_int as libc::c_uint;
            fdo = 1 as libc::c_int;
        }
        2 => {
            iset = 0o40 as libc::c_int as libc::c_uint;
            iclear = 0 as libc::c_int as libc::c_uint;
            fdo = 1 as libc::c_int;
        }
        0 | _ => {}
    }
    if fdo != 0 {
        if (*q).snew.c_iflag & iset != iset
            || (*q).snew.c_iflag & iclear != 0 as libc::c_int as libc::c_uint
        {
            (*q).snew.c_iflag |= iset;
            (*q).snew.c_iflag &= !iclear;
            fchanged = 1 as libc::c_int;
        }
    }
    fdo = 0 as libc::c_int;
    match txonxoff as libc::c_uint {
        1 => {
            iset = 0 as libc::c_int as libc::c_uint;
            iclear = (0o2000 as libc::c_int | 0o10000 as libc::c_int) as libc::c_uint;
            fdo = 1 as libc::c_int;
        }
        2 => {
            if (*q).snew.c_cflag & 0o20000000000 as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                iset = 0o2000 as libc::c_int as libc::c_uint;
                iclear = 0o10000 as libc::c_int as libc::c_uint;
                fdo = 1 as libc::c_int;
            } else {
                iset = (0o2000 as libc::c_int | 0o10000 as libc::c_int) as libc::c_uint;
                iclear = 0 as libc::c_int as libc::c_uint;
                fdo = 1 as libc::c_int;
            }
        }
        0 | _ => {}
    }
    if fdo != 0 {
        if (*q).snew.c_iflag & iset != iset
            || (*q).snew.c_iflag & iclear != 0 as libc::c_int as libc::c_uint
        {
            (*q).snew.c_iflag |= iset;
            (*q).snew.c_iflag &= !iclear;
            fchanged = 1 as libc::c_int;
        }
    }
    if fchanged != 0 {
        if !(tcsetattr((*q).o, 1 as libc::c_int, &mut (*q).snew) == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"Can't change terminal settings: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsstdin_set(
    mut qconn: *mut sconnection,
    mut tparity: tparitysetting,
    mut tstrip: tstripsetting,
    mut txonxoff: txonxoffsetting,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    (*qsysdep).o = (*qsysdep).ord;
    return fsserial_set(
        qconn,
        tparity as libc::c_uint,
        tstrip as libc::c_uint,
        txonxoff as libc::c_uint,
    );
}
unsafe extern "C" fn fsrun_chat(
    mut oread: libc::c_int,
    mut owrite: libc::c_int,
    mut pzprog: *mut *mut libc::c_char,
) -> boolean {
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut ipid: pid_t = 0;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: size_t = 0;
    aidescs[0 as libc::c_int as usize] = oread;
    aidescs[1 as libc::c_int as usize] = owrite;
    aidescs[2 as libc::c_int as usize] = -(2 as libc::c_int);
    ipid = ixsspawn(
        pzprog as *mut *const libc::c_char,
        aidescs.as_mut_ptr(),
        1 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if ipid < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ixsspawn (%s): %s\0" as *const u8 as *const libc::c_char,
            *pzprog.offset(0 as libc::c_int as isize),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    e = fdopen(
        aidescs[2 as libc::c_int as usize],
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if e.is_null() {
        ulog(
            LOG_ERROR,
            b"fdopen: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(aidescs[2 as libc::c_int as usize]);
        kill(ipid, 9 as libc::c_int);
        ixswait(ipid as libc::c_ulong, 0 as *mut libc::c_void as *const libc::c_char);
        return 0 as libc::c_int;
    }
    z = 0 as *mut libc::c_char;
    c = 0 as libc::c_int as size_t;
    while getline(&mut z, &mut c, e) > 0 as libc::c_int as libc::c_long {
        let mut clen: size_t = 0;
        clen = strlen(z);
        if *z.offset(clen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\n' as i32
        {
            *z
                .offset(
                    clen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        if *z as libc::c_int != '\0' as i32 {
            ulog(LOG_NORMAL, b"chat: %s\0" as *const u8 as *const libc::c_char, z);
        }
    }
    xfree(z as pointer);
    fclose(e);
    return (ixswait(
        ipid as libc::c_ulong,
        b"Chat program\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn fsdouble_chat(
    mut qconn: *mut sconnection,
    mut pzprog: *mut *mut libc::c_char,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut fret: boolean = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    fret = fsrun_chat((*qsysdep).ord, (*qsysdep).owr, pzprog);
    if (*qsysdep).fterminal != 0 {
        tcgetattr((*qsysdep).ord, &mut (*qsysdep).snew);
    }
    return fret;
}
pub unsafe extern "C" fn fsysdep_conn_chat(
    mut qconn: *mut sconnection,
    mut pzprog: *mut *mut libc::c_char,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut fret: boolean = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    fret = fsrun_chat((*qsysdep).o, (*qsysdep).o, pzprog);
    if (*qsysdep).fterminal != 0 {
        tcgetattr((*qsysdep).o, &mut (*qsysdep).snew);
    }
    return fret;
}
unsafe extern "C" fn isserial_baud(mut qconn: *mut sconnection) -> libc::c_long {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    return (*qsysdep).ibaud;
}

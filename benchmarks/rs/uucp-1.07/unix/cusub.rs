use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    static mut afSignal: [sig_atomic_t; 5];
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn pause() -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn usset_signal(
        isig: libc::c_int,
        pfn: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        fforce: boolean,
        pfignored: *mut boolean,
    );
    fn ixsfork() -> pid_t;
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
    static mut zCuvar_escape: *const libc::c_char;
    static mut zCuvar_eol: *const libc::c_char;
    fn fconn_write(
        qconn: *mut sconnection,
        zbuf: *const libc::c_char,
        cbytes: size_t,
    ) -> boolean;
    static mut abPrecbuf: [libc::c_char; 16384];
    static mut iPrecstart: libc::c_int;
    static mut iPrecend: libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type sig_atomic_t = __sig_atomic_t;
pub type pid_t = __pid_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type pointer = *mut libc::c_void;
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
pub type tshell_cmd = libc::c_uint;
pub const SHELL_STDIO_ON_PORT: tshell_cmd = 3;
pub const SHELL_STDIN_FROM_PORT: tshell_cmd = 2;
pub const SHELL_STDOUT_TO_PORT: tshell_cmd = 1;
pub const SHELL_NORMAL: tshell_cmd = 0;
pub static mut cusub_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: cusub.c,v 1.27 2002/03/05 19:10:42 ian Rel $\0")
};
static mut bSeof: libc::c_char = 0;
static mut bStstp: libc::c_char = 0;
unsafe extern "C" fn zsport_line(mut qport: *const uuconf_port) -> *const libc::c_char {
    let mut zline: *const libc::c_char = 0 as *const libc::c_char;
    if qport.is_null() {
        return 0 as *const libc::c_char;
    }
    match (*qport).uuconf_ttype as libc::c_uint {
        2 => {
            zline = (*qport).uuconf_u.uuconf_smodem.uuconf_zdevice;
        }
        3 => {
            zline = (*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice;
        }
        4 | 5 | 6 => return 0 as *const libc::c_char,
        1 | _ => return 0 as *const libc::c_char,
    }
    if zline.is_null() {
        zline = (*qport).uuconf_zname;
    }
    return zline;
}
pub unsafe extern "C" fn fsysdep_port_access(mut qport: *mut uuconf_port) -> boolean {
    let mut zline: *const libc::c_char = 0 as *const libc::c_char;
    let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    zline = zsport_line(qport);
    if zline.is_null() {
        return 1 as libc::c_int;
    }
    zfree = 0 as *mut libc::c_char;
    if *zline as libc::c_int != '/' as i32 {
        zfree = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(zline)),
        );
        sprintf(zfree, b"/dev/%s\0" as *const u8 as *const libc::c_char, zline);
        zline = zfree;
    }
    fret = (access(zline, 4 as libc::c_int | 2 as libc::c_int) == 0 as libc::c_int)
        as libc::c_int;
    ubuffree(zfree);
    return fret;
}
pub unsafe extern "C" fn fsysdep_port_is_line(
    mut qport: *mut uuconf_port,
    mut zline: *const libc::c_char,
) -> boolean {
    let mut zpline: *const libc::c_char = 0 as *const libc::c_char;
    let mut zfree1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfree2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    zpline = zsport_line(qport);
    if zpline.is_null() {
        return 0 as libc::c_int;
    }
    if strcmp(zline, zpline) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    zfree1 = 0 as *mut libc::c_char;
    zfree2 = 0 as *mut libc::c_char;
    if *zline as libc::c_int != '/' as i32 {
        zfree1 = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(zline)),
        );
        sprintf(zfree1, b"/dev/%s\0" as *const u8 as *const libc::c_char, zline);
        zline = zfree1;
    }
    if *zpline as libc::c_int != '/' as i32 {
        zfree2 = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(zpline)),
        );
        sprintf(zfree2, b"/dev/%s\0" as *const u8 as *const libc::c_char, zpline);
        zpline = zfree2;
    }
    fret = (strcmp(zline, zpline) == 0 as libc::c_int) as libc::c_int;
    ubuffree(zfree1);
    ubuffree(zfree2);
    return fret;
}
static mut iSchild: pid_t = 0;
static mut oSpipe: libc::c_int = 0;
pub unsafe extern "C" fn fsysdep_cu_init(mut qconn: *mut sconnection) -> boolean {
    let mut ai: [libc::c_int; 2] = [0; 2];
    while iPrecend != iPrecstart {
        let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_int = 0;
        z = abPrecbuf.as_mut_ptr().offset(iPrecstart as isize);
        if iPrecend > iPrecstart {
            c = iPrecend - iPrecstart;
        } else {
            c = 16384 as libc::c_int - iPrecstart;
        }
        iPrecstart = (iPrecstart + c) % 16384 as libc::c_int;
        while c > 0 as libc::c_int {
            let mut cwrote: libc::c_int = 0;
            cwrote = write(1 as libc::c_int, z as *const libc::c_void, c as size_t)
                as libc::c_int;
            if cwrote <= 0 as libc::c_int {
                if cwrote < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"write: %s\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                } else {
                    ulog(
                        LOG_ERROR,
                        b"Line disconnected\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            c -= cwrote;
            z = z.offset(cwrote as isize);
        }
    }
    if pipe(ai.as_mut_ptr()) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"pipe: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    ::std::ptr::write_volatile(&mut iSchild as *mut pid_t, ixsfork());
    if iSchild < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fork: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    if iSchild == 0 as libc::c_int {
        close(ai[0 as libc::c_int as usize]);
        uscu_child(qconn, ai[1 as libc::c_int as usize]);
    }
    close(ai[1 as libc::c_int as usize]);
    oSpipe = ai[0 as libc::c_int as usize];
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_cu(
    mut qconn: *mut sconnection,
    mut pbcmd: *mut libc::c_char,
    mut zlocalname: *const libc::c_char,
) -> boolean {
    let mut fstart: boolean = 0;
    let mut b: libc::c_char = 0;
    let mut c: libc::c_int = 0;
    fstart = 1 as libc::c_int;
    loop {
        c = read(
            0 as libc::c_int,
            &mut b as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if c <= 0 as libc::c_int {
            break;
        }
        if fstart != 0 && b as libc::c_int == *zCuvar_escape as libc::c_int
            && b as libc::c_int != '\0' as i32
        {
            c = cscu_escape(pbcmd, zlocalname);
            if c <= 0 as libc::c_int {
                break;
            }
            if *pbcmd as libc::c_int != b as libc::c_int {
                write(
                    1 as libc::c_int,
                    pbcmd as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                if *pbcmd as libc::c_int == bSeof as libc::c_int {
                    *pbcmd = '.' as i32 as libc::c_char;
                }
                if *pbcmd as libc::c_int == bStstp as libc::c_int {
                    *pbcmd = 'z' as i32 as libc::c_char;
                }
                return 1 as libc::c_int;
            }
        }
        if fconn_write(qconn, &mut b, 1 as libc::c_int as size_t) == 0 {
            return 0 as libc::c_int;
        }
        fstart = (strchr(zCuvar_eol, b as libc::c_int)
            != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    }
    if c < 0 as libc::c_int {
        if *__errno_location() != 4 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"read: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        } else {
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
        }
        return 0 as libc::c_int;
    }
    ulog(LOG_ERROR, b"End of file on terminal\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub static mut fScu_alarm: sig_atomic_t = 0;
unsafe extern "C" fn uscu_alarm(mut isig: libc::c_int) {
    ::std::ptr::write_volatile(&mut fScu_alarm as *mut sig_atomic_t, 1 as libc::c_int);
}
unsafe extern "C" fn cscu_escape(
    mut pbcmd: *mut libc::c_char,
    mut zlocalname: *const libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    write(
        1 as libc::c_int,
        zCuvar_escape as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    ::std::ptr::write_volatile(&mut fScu_alarm as *mut sig_atomic_t, 0 as libc::c_int);
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
                >(uscu_alarm),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    alarm(1 as libc::c_int as libc::c_uint);
    c = 0 as libc::c_int;
    loop {
        if fScu_alarm != 0 {
            let mut b: libc::c_char = 0;
            ::std::ptr::write_volatile(
                &mut fScu_alarm as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            b = '[' as i32 as libc::c_char;
            write(
                1 as libc::c_int,
                &mut b as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            write(
                1 as libc::c_int,
                zlocalname as *const libc::c_void,
                strlen(zlocalname),
            );
            b = ']' as i32 as libc::c_char;
            write(
                1 as libc::c_int,
                &mut b as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        if c <= 0 as libc::c_int {
            c = read(
                0 as libc::c_int,
                pbcmd as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        if c >= 0 as libc::c_int || *__errno_location() != 4 as libc::c_int {
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
            return c;
        }
    };
}
static mut iSsend_sig: sig_atomic_t = 0;
unsafe extern "C" fn uscu_alarm_kill(mut isig: libc::c_int) {
    kill(iSchild, iSsend_sig);
    alarm(1 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn fsysdep_cu_copy(mut fcopy: boolean) -> boolean {
    let mut ierr: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
                >(uscu_alarm_kill),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    if fcopy != 0 {
        ::std::ptr::write_volatile(
            &mut iSsend_sig as *mut sig_atomic_t,
            10 as libc::c_int,
        );
    } else {
        ::std::ptr::write_volatile(
            &mut iSsend_sig as *mut sig_atomic_t,
            12 as libc::c_int,
        );
    }
    uscu_alarm_kill(14 as libc::c_int);
    alarm(1 as libc::c_int as libc::c_uint);
    loop {
        let mut b: libc::c_char = 0;
        c = read(
            oSpipe,
            &mut b as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if c > 0 as libc::c_int {
            if iDebug & 0o1000 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fsysdep_cu_copy: Got '%d'\0" as *const u8 as *const libc::c_char,
                    b as libc::c_int,
                );
            }
        }
        if c < 0 as libc::c_int && *__errno_location() != 4 as libc::c_int
            || c == 0 as libc::c_int
            || c > 0 as libc::c_int
                && b as libc::c_int == (if fcopy != 0 { 'G' as i32 } else { 'S' as i32 })
        {
            break;
        }
    }
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
    if c > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if c == 0 as libc::c_int {
        ulog(LOG_ERROR, b"EOF on child pipe\0" as *const u8 as *const libc::c_char);
    } else {
        ulog(
            LOG_ERROR,
            b"read: %s\0" as *const u8 as *const libc::c_char,
            strerror(ierr),
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_cu_finish() -> boolean {
    close(oSpipe);
    if kill(iSchild, 15 as libc::c_int) < 0 as libc::c_int {
        if *__errno_location() != 3 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"kill: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
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
                >(uscu_alarm_kill),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    ::std::ptr::write_volatile(&mut iSsend_sig as *mut sig_atomic_t, 9 as libc::c_int);
    alarm(2 as libc::c_int as libc::c_uint);
    ixswait(iSchild as libc::c_ulong, b"child\0" as *const u8 as *const libc::c_char);
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
    return 1 as libc::c_int;
}
static mut iSchild_sig: sig_atomic_t = 0;
unsafe extern "C" fn uscu_child_handler(mut isig: libc::c_int) {
    ::std::ptr::write_volatile(&mut iSchild_sig as *mut sig_atomic_t, isig);
}
unsafe extern "C" fn uscu_child(mut qconn: *mut sconnection, mut opipe: libc::c_int) {
    let mut oport: libc::c_int = 0;
    let mut fstopped: boolean = 0;
    let mut fgot: boolean = 0;
    let mut cwrite: libc::c_int = 0;
    let mut abbuf: [libc::c_char; 1024] = [0; 1024];
    fgot = 0 as libc::c_int;
    if ((*qconn).qport).is_null() {
        oport = 0 as libc::c_int;
    } else {
        let mut current_block_6: u64;
        match (*(*qconn).qport).uuconf_ttype as libc::c_uint {
            6 => {
                fgot = 1 as libc::c_int;
                current_block_6 = 2114826022405636219;
            }
            1 => {
                current_block_6 = 2114826022405636219;
            }
            2 | 3 | 4 | 5 => {
                oport = (*((*qconn).psysdep as *mut ssysdep_conn)).o;
                current_block_6 = 17965632435239708295;
            }
            _ => {
                ulog(
                    LOG_FATAL,
                    b"uscu_child: Can't happen\0" as *const u8 as *const libc::c_char,
                );
                oport = -(1 as libc::c_int);
                current_block_6 = 17965632435239708295;
            }
        }
        match current_block_6 {
            2114826022405636219 => {
                oport = (*((*qconn).psysdep as *mut ssysdep_conn)).ord;
            }
            _ => {}
        }
    }
    fcntl(
        oport,
        4 as libc::c_int,
        fcntl(oport, 3 as libc::c_int, 0 as libc::c_int)
            & !(0o4000 as libc::c_int | 0o4000 as libc::c_int),
    );
    usset_signal(
        10 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(uscu_child_handler),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    usset_signal(
        12 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(uscu_child_handler),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    usset_signal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    usset_signal(
        3 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    usset_signal(
        13 as libc::c_int,
        None,
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    usset_signal(
        15 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(uscu_child_handler),
            ),
        ),
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
    fstopped = 0 as libc::c_int;
    ::std::ptr::write_volatile(&mut iSchild_sig as *mut sig_atomic_t, 0 as libc::c_int);
    cwrite = 0 as libc::c_int;
    loop {
        let mut isig: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        isig = iSchild_sig;
        ::std::ptr::write_volatile(
            &mut iSchild_sig as *mut sig_atomic_t,
            0 as libc::c_int,
        );
        if isig != 0 as libc::c_int {
            let mut b: libc::c_char = 0;
            if isig == 15 as libc::c_int {
                exit(0 as libc::c_int);
            }
            if isig == 10 as libc::c_int {
                fstopped = 0 as libc::c_int;
                b = 'G' as i32 as libc::c_char;
            } else {
                fstopped = 1 as libc::c_int;
                b = 'S' as i32 as libc::c_char;
                cwrite = 0 as libc::c_int;
            }
            c = write(
                opipe,
                &mut b as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if c < 0 as libc::c_int
                && (*__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 61 as libc::c_int)
            {
                c = 0 as libc::c_int;
            }
            if c <= 0 as libc::c_int {
                kill(getppid(), 1 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        if fstopped != 0 {
            pause();
        } else if cwrite > 0 as libc::c_int {
            let mut zbuf: *mut libc::c_char = 0 as *mut libc::c_char;
            zbuf = abbuf.as_mut_ptr();
            while cwrite > 0 as libc::c_int {
                c = write(
                    1 as libc::c_int,
                    zbuf as *const libc::c_void,
                    cwrite as size_t,
                ) as libc::c_int;
                if c < 0 as libc::c_int
                    && (*__errno_location() == 11 as libc::c_int
                        || *__errno_location() == 11 as libc::c_int
                        || *__errno_location() == 61 as libc::c_int)
                {
                    c = 0 as libc::c_int;
                }
                if c < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                    break;
                }
                if c <= 0 as libc::c_int {
                    kill(getppid(), 1 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                cwrite -= c;
                zbuf = zbuf.offset(c as isize);
            }
        } else {
            *__errno_location() = 0 as libc::c_int;
            c = read(
                oport,
                abbuf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            ) as libc::c_int;
            if c < 0 as libc::c_int
                && (*__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 61 as libc::c_int)
            {
                c = 0 as libc::c_int;
            }
            if c == 0 as libc::c_int && fgot != 0
                || c < 0 as libc::c_int && *__errno_location() != 4 as libc::c_int
            {
                kill(getppid(), 1 as libc::c_int);
                exit(0 as libc::c_int);
            }
            if c > 0 as libc::c_int {
                fgot = 1 as libc::c_int;
                cwrite = c;
            }
        }
    };
}
static mut fSterm: boolean = 0;
static mut fSlocalecho: boolean = 0;
static mut sSterm_orig: sterminal = sterminal {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut sSterm_new: sterminal = sterminal {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut fStstp_ignored: boolean = 0;
pub unsafe extern "C" fn fsysdep_terminal_raw(mut flocalecho: boolean) -> boolean {
    fSlocalecho = flocalecho;
    bSeof = '\u{4}' as i32 as libc::c_char;
    bStstp = '\u{1a}' as i32 as libc::c_char;
    if !(tcgetattr(0 as libc::c_int, &mut sSterm_orig) == 0 as libc::c_int) {
        fSterm = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    fSterm = 1 as libc::c_int;
    sSterm_new = sSterm_orig;
    bSeof = sSterm_new.c_cc[4 as libc::c_int as usize] as libc::c_char;
    bStstp = sSterm_new.c_cc[10 as libc::c_int as usize] as libc::c_char;
    if flocalecho == 0 {
        sSterm_new.c_lflag
            &= !(0o2 as libc::c_int | 0o100000 as libc::c_int | 0o1 as libc::c_int
                | 0o10 as libc::c_int | 0o20 as libc::c_int | 0o40 as libc::c_int
                | 0o100 as libc::c_int) as libc::c_uint;
    } else {
        sSterm_new.c_lflag
            &= !(0o2 as libc::c_int | 0o100000 as libc::c_int | 0o1 as libc::c_int)
                as libc::c_uint;
    }
    sSterm_new.c_iflag
        &= !(0o100 as libc::c_int | 0o200 as libc::c_int | 0o400 as libc::c_int
            | 0o2000 as libc::c_int | 0o10000 as libc::c_int) as libc::c_uint;
    sSterm_new.c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
    sSterm_new.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    sSterm_new.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    if !(tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut sSterm_new)
        == 0 as libc::c_int)
    {
        ulog(
            LOG_ERROR,
            b"Can't set terminal settings: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_terminal_restore() -> boolean {
    if fSterm == 0 {
        return 1 as libc::c_int;
    }
    if !(tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut sSterm_orig)
        == 0 as libc::c_int)
    {
        ulog(
            LOG_ERROR,
            b"Can't restore terminal: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_terminal_line(
    mut zprompt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut cbuf: size_t = 0 as libc::c_int as size_t;
    let mut zbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cgot: size_t = 0 as libc::c_int as size_t;
    if !zprompt.is_null() && *zprompt as libc::c_int != '\0' as i32 {
        write(1 as libc::c_int, zprompt as *const libc::c_void, strlen(zprompt));
    }
    ::std::ptr::write_volatile(
        &mut afSignal[1 as libc::c_int as usize] as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut afSignal[2 as libc::c_int as usize] as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    if fsysdep_terminal_restore() == 0 {
        return 0 as *mut libc::c_char;
    }
    cbuf = 0 as libc::c_int as size_t;
    zbuf = 0 as *mut libc::c_char;
    cgot = 0 as libc::c_int as size_t;
    loop {
        let mut b: libc::c_char = 0;
        let mut c: libc::c_int = 0;
        if afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
        {
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
            cgot = 0 as libc::c_int as size_t;
            break;
        } else {
            c = read(
                0 as libc::c_int,
                &mut b as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if c < 0 as libc::c_int {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                ulog(
                    LOG_ERROR,
                    b"read: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                fsysdep_terminal_raw(fSlocalecho);
                return 0 as *mut libc::c_char;
            } else {
                if c == 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"EOF on terminal\0" as *const u8 as *const libc::c_char,
                    );
                    fsysdep_terminal_raw(fSlocalecho);
                    return 0 as *mut libc::c_char;
                }
                if cgot >= cbuf {
                    let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
                    cbuf = (cbuf as libc::c_ulong)
                        .wrapping_add(64 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    znew = zbufalc(cbuf);
                    if !zbuf.is_null() {
                        memcpy(
                            znew as *mut libc::c_void,
                            zbuf as *const libc::c_void,
                            cgot,
                        );
                        ubuffree(zbuf);
                    }
                    zbuf = znew;
                }
                *zbuf.offset(cgot as isize) = b;
                cgot = cgot.wrapping_add(1);
                cgot;
                if b as libc::c_int == '\n' as i32 {
                    break;
                }
            }
        }
    }
    if cgot >= cbuf {
        let mut znew_0: *mut libc::c_char = 0 as *mut libc::c_char;
        cbuf = cbuf.wrapping_add(1);
        cbuf;
        znew_0 = zbufalc(cbuf);
        if !zbuf.is_null() {
            memcpy(znew_0 as *mut libc::c_void, zbuf as *const libc::c_void, cgot);
            ubuffree(zbuf);
        }
        zbuf = znew_0;
    }
    *zbuf.offset(cgot as isize) = '\0' as i32 as libc::c_char;
    if fsysdep_terminal_raw(fSlocalecho) == 0 {
        return 0 as *mut libc::c_char;
    }
    return zbuf;
}
pub unsafe extern "C" fn fsysdep_terminal_puts(
    mut zline: *const libc::c_char,
) -> boolean {
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zprint: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clen: size_t = 0;
    if zline.is_null() {
        zalc = zbufalc(2 as libc::c_int as size_t);
        clen = 0 as libc::c_int as size_t;
    } else {
        clen = strlen(zline);
        zalc = zbufalc(clen.wrapping_add(2 as libc::c_int as libc::c_ulong));
        memcpy(zalc as *mut libc::c_void, zline as *const libc::c_void, clen);
    }
    if fSterm != 0 {
        *zalc.offset(clen as isize) = '\r' as i32 as libc::c_char;
        clen = clen.wrapping_add(1);
        clen;
    }
    *zalc.offset(clen as isize) = '\n' as i32 as libc::c_char;
    clen = clen.wrapping_add(1);
    clen;
    zprint = zalc;
    while clen > 0 as libc::c_int as libc::c_ulong {
        let mut c: libc::c_int = 0;
        c = write(1 as libc::c_int, zprint as *const libc::c_void, clen) as libc::c_int;
        if c <= 0 as libc::c_int {
            ubuffree(zalc);
            ulog(
                LOG_ERROR,
                b"write: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        clen = (clen as libc::c_ulong).wrapping_sub(c as libc::c_ulong) as size_t
            as size_t;
        zprint = zprint.offset(c as isize);
    }
    ubuffree(zalc);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_terminal_signals(mut faccept: boolean) -> boolean {
    if faccept != 0 {
        sSterm_new.c_lflag |= 0o1 as libc::c_int as libc::c_uint;
    } else {
        sSterm_new.c_lflag &= !(0o1 as libc::c_int) as libc::c_uint;
    }
    if faccept != 0 {
        usset_signal(
            20 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
            0 as libc::c_int,
            &mut fStstp_ignored,
        );
    } else if fStstp_ignored == 0 {
        usset_signal(
            20 as libc::c_int,
            None,
            1 as libc::c_int,
            0 as *mut libc::c_void as *mut boolean,
        );
    }
    if !(tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut sSterm_new)
        == 0 as libc::c_int)
    {
        ulog(
            LOG_ERROR,
            b"Can't set terminal: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_shell(
    mut qconn: *mut sconnection,
    mut zcmd: *const libc::c_char,
    mut tcmd: tshell_cmd,
) -> boolean {
    let mut azargs: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    let mut oread: libc::c_int = 0;
    let mut owrite: libc::c_int = 0;
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut ipid: pid_t = 0;
    if tcmd as libc::c_uint != SHELL_NORMAL as libc::c_int as libc::c_uint {
        azargs[0 as libc::c_int
            as usize] = b"/bin/sh\0" as *const u8 as *const libc::c_char;
    } else {
        azargs[0 as libc::c_int
            as usize] = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
        if (azargs[0 as libc::c_int as usize]).is_null() {
            azargs[0 as libc::c_int
                as usize] = b"/bin/sh\0" as *const u8 as *const libc::c_char;
        }
    }
    if zcmd.is_null() || *zcmd as libc::c_int == '\0' as i32 {
        azargs[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else {
        azargs[1 as libc::c_int as usize] = b"-c\0" as *const u8 as *const libc::c_char;
        azargs[2 as libc::c_int as usize] = zcmd;
        azargs[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    }
    if ((*qconn).qport).is_null() {
        oread = 0 as libc::c_int;
        owrite = 1 as libc::c_int;
    } else {
        match (*(*qconn).qport).uuconf_ttype as libc::c_uint {
            1 | 6 => {
                oread = (*((*qconn).psysdep as *mut ssysdep_conn)).ord;
                owrite = (*((*qconn).psysdep as *mut ssysdep_conn)).owr;
            }
            2 | 3 | 4 | 5 => {
                owrite = (*((*qconn).psysdep as *mut ssysdep_conn)).o;
                oread = owrite;
            }
            _ => {
                owrite = -(1 as libc::c_int);
                oread = owrite;
            }
        }
    }
    aidescs[0 as libc::c_int as usize] = 0 as libc::c_int;
    aidescs[1 as libc::c_int as usize] = 1 as libc::c_int;
    aidescs[2 as libc::c_int as usize] = 2 as libc::c_int;
    if tcmd as libc::c_uint == SHELL_STDIN_FROM_PORT as libc::c_int as libc::c_uint
        || tcmd as libc::c_uint == SHELL_STDIO_ON_PORT as libc::c_int as libc::c_uint
    {
        aidescs[0 as libc::c_int as usize] = oread;
    }
    if tcmd as libc::c_uint == SHELL_STDOUT_TO_PORT as libc::c_int as libc::c_uint
        || tcmd as libc::c_uint == SHELL_STDIO_ON_PORT as libc::c_int as libc::c_uint
    {
        aidescs[1 as libc::c_int as usize] = owrite;
    }
    ipid = ixsspawn(
        azargs.as_mut_ptr(),
        aidescs.as_mut_ptr(),
        0 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if ipid < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ixsspawn (/bin/sh): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return (ixswait(
        ipid as libc::c_ulong,
        b"shell\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_chdir(mut zdir: *const libc::c_char) -> boolean {
    if zdir.is_null() || *zdir as libc::c_int == '\0' as i32 {
        zdir = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
        if zdir.is_null() {
            ulog(LOG_ERROR, b"HOME not defined\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
    }
    if chdir(zdir) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"chdir (%s): %s\0" as *const u8 as *const libc::c_char,
            zdir,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_suspend() -> boolean {
    return (kill(getpid(), 20 as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}

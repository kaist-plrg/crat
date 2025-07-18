use ::libc;
extern "C" {
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    fn usysdep_sleep(cseconds: libc::c_int);
    fn fsdouble_chat(qconn: *mut sconnection, pzprog: *mut *mut libc::c_char) -> boolean;
    fn fsysdep_conn_io(
        qconn: *mut sconnection,
        zwrite: *const libc::c_char,
        pcwrite: *mut size_t,
        zread: *mut libc::c_char,
        pcread: *mut size_t,
    ) -> boolean;
    fn fsdouble_write(
        qconn: *mut sconnection,
        zbuf: *const libc::c_char,
        clen: size_t,
    ) -> boolean;
    fn fsdouble_read(
        qconn: *mut sconnection,
        zbuf: *mut libc::c_char,
        pclen: *mut size_t,
        cmin: size_t,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> boolean;
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
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
pub static mut pipe_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: pipe.c,v 1.10 2002/03/05 19:10:42 ian Rel $\0")
};
static mut spipecmds: sconncmds = unsafe {
    {
        let mut init = sconncmds {
            pufree: Some(uspipe_free as unsafe extern "C" fn(*mut sconnection) -> ()),
            pflock: None,
            pfunlock: None,
            pfopen: Some(
                fspipe_open
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        libc::c_long,
                        boolean,
                        boolean,
                    ) -> boolean,
            ),
            pfclose: Some(
                fspipe_close
                    as unsafe extern "C" fn(
                        *mut sconnection,
                        pointer,
                        *mut uuconf_dialer,
                        boolean,
                    ) -> boolean,
            ),
            pfdial: Some(
                fspipe_dial
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
            pfbreak: None,
            pfset: None,
            pfcarrier: None,
            pfchat: Some(
                fsdouble_chat
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
pub unsafe extern "C" fn fsysdep_pipe_init(mut qconn: *mut sconnection) -> boolean {
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
    (*q).ipid = -(1 as libc::c_int);
    (*qconn).psysdep = q as pointer;
    (*qconn).qcmds = &spipecmds;
    return 1 as libc::c_int;
}
unsafe extern "C" fn uspipe_free(mut qconn: *mut sconnection) {
    xfree((*qconn).psysdep);
}
unsafe extern "C" fn fspipe_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
) -> boolean {
    if fwait != 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fspipe_close(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdialer: *mut uuconf_dialer,
    mut fsuccess: boolean,
) -> boolean {
    let mut qsysdep: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut fret: boolean = 0;
    qsysdep = (*qconn).psysdep as *mut ssysdep_conn;
    fret = 1 as libc::c_int;
    if (*qsysdep).ord >= 0 as libc::c_int && close((*qsysdep).ord) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fspipe_close: close read fd: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fret = 0 as libc::c_int;
    }
    if (*qsysdep).owr != (*qsysdep).ord && (*qsysdep).owr >= 0 as libc::c_int
        && close((*qsysdep).owr) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fspipe_close: close write fd: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fret = 0 as libc::c_int;
    }
    (*qsysdep).ord = -(1 as libc::c_int);
    (*qsysdep).owr = -(1 as libc::c_int);
    if (*qsysdep).ipid >= 0 as libc::c_int {
        if kill((*qsysdep).ipid, 1 as libc::c_int) == 0 as libc::c_int {
            usysdep_sleep(2 as libc::c_int);
        }
        if kill((*qsysdep).ipid, 13 as libc::c_int) == 0 as libc::c_int {
            usysdep_sleep(2 as libc::c_int);
        }
        if kill((*qsysdep).ipid, 9 as libc::c_int) < 0 as libc::c_int
            && *__errno_location() == 1 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"fspipe_close: Cannot kill child pid %lu: %s\0" as *const u8
                    as *const libc::c_char,
                (*qsysdep).ipid as libc::c_ulong,
                strerror(*__errno_location()),
            );
            fret = 0 as libc::c_int;
        } else {
            ixswait(
                (*qsysdep).ipid as libc::c_ulong,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
    }
    (*qsysdep).ipid = -(1 as libc::c_int);
    return fret;
}
unsafe extern "C" fn fspipe_dial(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qsys: *const uuconf_system,
    mut zphone: *const libc::c_char,
    mut qdialer: *mut uuconf_dialer,
    mut ptdialer: *mut tdialerfound,
) -> boolean {
    let mut q: *mut ssysdep_conn = 0 as *mut ssysdep_conn;
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut pzprog: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    q = (*qconn).psysdep as *mut ssysdep_conn;
    *ptdialer = DIALERFOUND_FALSE;
    pzprog = (*(*qconn).qport).uuconf_u.uuconf_spipe.uuconf_pzcmd
        as *mut *const libc::c_char;
    if pzprog.is_null() {
        ulog(
            LOG_ERROR,
            b"No command for pipe connection\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    aidescs[0 as libc::c_int as usize] = -(3 as libc::c_int);
    aidescs[1 as libc::c_int as usize] = -(2 as libc::c_int);
    aidescs[2 as libc::c_int as usize] = -(1 as libc::c_int);
    (*q)
        .ipid = ixsspawn(
        pzprog,
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
    if (*q).ipid < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ixsspawn (%s): %s\0" as *const u8 as *const libc::c_char,
            *pzprog.offset(0 as libc::c_int as isize),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    (*q).owr = aidescs[0 as libc::c_int as usize];
    (*q).ord = aidescs[1 as libc::c_int as usize];
    (*q).o = (*q).ord;
    (*q).iflags = fcntl((*q).ord, 3 as libc::c_int, 0 as libc::c_int);
    (*q).iwr_flags = fcntl((*q).owr, 3 as libc::c_int, 0 as libc::c_int);
    if (*q).iflags < 0 as libc::c_int || (*q).iwr_flags < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fspipe_dial: fcntl: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fspipe_close(qconn, puuconf, qdialer, 0 as libc::c_int);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

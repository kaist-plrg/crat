use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_device(zdevice: *const libc::c_char);
    fn udebug_buffer(zhdr: *const libc::c_char, zbuf: *const libc::c_char, clen: size_t);
    static mut afSignal: [sig_atomic_t; 5];
    static mut fLog_sighup: boolean;
    fn uuconf_dialer_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdialer: *const libc::c_char,
        uuconf_qdialer: *mut uuconf_dialer,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
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
    fn fsysdep_modem_begin_dial(
        qconn: *mut sconnection,
        qdial: *mut uuconf_dialer,
    ) -> boolean;
    fn fsysdep_modem_end_dial(
        qconn: *mut sconnection,
        qdial: *mut uuconf_dialer,
    ) -> boolean;
    fn fsysdep_stdin_init(qconn: *mut sconnection) -> boolean;
    fn fsysdep_modem_init(qconn: *mut sconnection) -> boolean;
    fn fsysdep_direct_init(qconn: *mut sconnection) -> boolean;
    fn fsysdep_tcp_init(qconn: *mut sconnection) -> boolean;
    fn fsysdep_pipe_init(qconn: *mut sconnection) -> boolean;
}
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
pub type sig_atomic_t = __sig_atomic_t;
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
pub static mut conn_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: conn.c,v 1.18 2002/03/05 19:10:41 ian Rel $\0")
};
pub unsafe extern "C" fn fconn_init(
    mut qport: *mut uuconf_port,
    mut qconn: *mut sconnection,
    mut ttype: uuconf_porttype,
) -> boolean {
    (*qconn).qport = qport;
    match if qport.is_null() {
        ttype as libc::c_uint
    } else {
        (*qport).uuconf_ttype as libc::c_uint
    } {
        1 => return fsysdep_stdin_init(qconn),
        2 => return fsysdep_modem_init(qconn),
        3 => return fsysdep_direct_init(qconn),
        4 => return fsysdep_tcp_init(qconn),
        6 => return fsysdep_pipe_init(qconn),
        _ => {
            ulog(
                LOG_ERROR,
                b"Unknown or unsupported port type\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn uconn_free(mut qconn: *mut sconnection) {
    (Some(((*(*qconn).qcmds).pufree).unwrap())).unwrap()(qconn);
}
pub unsafe extern "C" fn fconn_lock(
    mut qconn: *mut sconnection,
    mut fin: boolean,
    mut fuser: boolean,
) -> boolean {
    let mut pflock: Option::<
        unsafe extern "C" fn(*mut sconnection, boolean, boolean) -> boolean,
    > = None;
    pflock = (*(*qconn).qcmds).pflock;
    if pflock.is_none() {
        return 1 as libc::c_int;
    }
    return (Some(pflock.unwrap())).unwrap()(qconn, fin, fuser);
}
pub unsafe extern "C" fn fconn_unlock(mut qconn: *mut sconnection) -> boolean {
    let mut pfunlock: Option::<unsafe extern "C" fn(*mut sconnection) -> boolean> = None;
    pfunlock = (*(*qconn).qcmds).pfunlock;
    if pfunlock.is_none() {
        return 1 as libc::c_int;
    }
    return (Some(pfunlock.unwrap())).unwrap()(qconn);
}
pub unsafe extern "C" fn fconn_open(
    mut qconn: *mut sconnection,
    mut ibaud: libc::c_long,
    mut ihighbaud: libc::c_long,
    mut fwait: boolean,
    mut fuser: boolean,
) -> boolean {
    let mut fret: boolean = 0;
    if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        let mut abspeed: [libc::c_char; 20] = [0; 20];
        if ibaud == 0 as libc::c_int as libc::c_long {
            strcpy(
                abspeed.as_mut_ptr(),
                b"default speed\0" as *const u8 as *const libc::c_char,
            );
        } else {
            sprintf(
                abspeed.as_mut_ptr(),
                b"speed %ld\0" as *const u8 as *const libc::c_char,
                ibaud,
            );
        }
        if ((*qconn).qport).is_null() {
            ulog(
                LOG_DEBUG,
                b"fconn_open: Opening stdin port (%s)\0" as *const u8
                    as *const libc::c_char,
                abspeed.as_mut_ptr(),
            );
        } else if ((*(*qconn).qport).uuconf_zname).is_null() {
            ulog(
                LOG_DEBUG,
                b"fconn_open: Opening unnamed port (%s)\0" as *const u8
                    as *const libc::c_char,
                abspeed.as_mut_ptr(),
            );
        } else {
            ulog(
                LOG_DEBUG,
                b"fconn_open: Opening port %s (%s)\0" as *const u8
                    as *const libc::c_char,
                (*(*qconn).qport).uuconf_zname,
                abspeed.as_mut_ptr(),
            );
        }
    }
    if ihighbaud != 0 as libc::c_int as libc::c_long && !((*qconn).qport).is_null() {
        let mut qport: *mut uuconf_port = 0 as *mut uuconf_port;
        qport = (*qconn).qport;
        ibaud = ihighbaud;
        if (*qport).uuconf_ttype as libc::c_uint
            == UUCONF_PORTTYPE_MODEM as libc::c_int as libc::c_uint
        {
            if (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud
                != 0 as libc::c_int as libc::c_long
            {
                if (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud < ibaud {
                    ibaud = (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud;
                }
            } else if (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud
                != 0 as libc::c_int as libc::c_long
            {
                ibaud = (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud;
            }
        } else if (*qport).uuconf_ttype as libc::c_uint
            == UUCONF_PORTTYPE_DIRECT as libc::c_int as libc::c_uint
        {
            if (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud
                != 0 as libc::c_int as libc::c_long
            {
                ibaud = (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud;
            }
        }
    }
    if ((*qconn).qport).is_null() {
        ulog_device(b"stdin\0" as *const u8 as *const libc::c_char);
    } else {
        ulog_device((*(*qconn).qport).uuconf_zname);
    }
    fret = (Some(((*(*qconn).qcmds).pfopen).unwrap()))
        .unwrap()(qconn, ibaud, fwait, fuser);
    if fret == 0 {
        ulog_device(0 as *mut libc::c_void as *const libc::c_char);
    }
    return fret;
}
pub unsafe extern "C" fn fconn_close(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdialer: *mut uuconf_dialer,
    mut fsuccess: boolean,
) -> boolean {
    let mut fret: boolean = 0;
    if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fconn_close: Closing connection\0" as *const u8 as *const libc::c_char,
        );
    }
    fLog_sighup = 0 as libc::c_int;
    fret = (Some(((*(*qconn).qcmds).pfclose).unwrap()))
        .unwrap()(qconn, puuconf, qdialer, fsuccess);
    ::std::ptr::write_volatile(
        &mut afSignal[0 as libc::c_int as usize] as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
    fLog_sighup = 1 as libc::c_int;
    ulog_device(0 as *mut libc::c_void as *const libc::c_char);
    return fret;
}
pub unsafe extern "C" fn fconn_dial(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qsys: *const uuconf_system,
    mut zphone: *const libc::c_char,
    mut qdialer: *mut uuconf_dialer,
    mut ptdialerfound: *mut tdialerfound,
) -> boolean {
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
    let mut tfound: tdialerfound = DIALERFOUND_FALSE;
    let mut pfdial: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            pointer,
            *const uuconf_system,
            *const libc::c_char,
            *mut uuconf_dialer,
            *mut tdialerfound,
        ) -> boolean,
    > = None;
    if qdialer.is_null() {
        qdialer = &mut sdialer;
    }
    if ptdialerfound.is_null() {
        ptdialerfound = &mut tfound;
    }
    (*qdialer).uuconf_zname = 0 as *mut libc::c_char;
    *ptdialerfound = DIALERFOUND_FALSE;
    pfdial = (*(*qconn).qcmds).pfdial;
    if pfdial.is_none() {
        return 1 as libc::c_int;
    }
    return (Some(pfdial.unwrap()))
        .unwrap()(qconn, puuconf, qsys, zphone, qdialer, ptdialerfound);
}
pub unsafe extern "C" fn fconn_read(
    mut qconn: *mut sconnection,
    mut zbuf: *mut libc::c_char,
    mut pclen: *mut size_t,
    mut cmin: size_t,
    mut ctimeout: libc::c_int,
    mut freport: boolean,
) -> boolean {
    let mut fret: boolean = 0;
    fret = (Some(((*(*qconn).qcmds).pfread).unwrap()))
        .unwrap()(qconn, zbuf, pclen, cmin, ctimeout, freport);
    if iDebug & 0o1000 as libc::c_int != 0 as libc::c_int {
        udebug_buffer(
            b"fconn_read: Read\0" as *const u8 as *const libc::c_char,
            zbuf,
            *pclen,
        );
    } else if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fconn_read: Read %lu\0" as *const u8 as *const libc::c_char,
            *pclen,
        );
    }
    return fret;
}
pub unsafe extern "C" fn fconn_write(
    mut qconn: *mut sconnection,
    mut zbuf: *const libc::c_char,
    mut clen: size_t,
) -> boolean {
    if iDebug & 0o2000 as libc::c_int != 0 as libc::c_int {
        udebug_buffer(
            b"fconn_write: Writing\0" as *const u8 as *const libc::c_char,
            zbuf,
            clen,
        );
    } else if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fconn_write: Writing %lu\0" as *const u8 as *const libc::c_char,
            clen,
        );
    }
    return (Some(((*(*qconn).qcmds).pfwrite).unwrap())).unwrap()(qconn, zbuf, clen);
}
pub unsafe extern "C" fn fconn_io(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut pcwrite: *mut size_t,
    mut zread: *mut libc::c_char,
    mut pcread: *mut size_t,
) -> boolean {
    let mut fret: boolean = 0;
    let mut cwrite: size_t = *pcwrite;
    let mut cread: size_t = *pcread;
    if cread == 0 as libc::c_int as libc::c_ulong
        || cwrite == 0 as libc::c_int as libc::c_ulong
    {
        ulog(
            LOG_FATAL,
            b"fconn_io: cread %lu; cwrite %lu\0" as *const u8 as *const libc::c_char,
            cread,
            cwrite,
        );
    }
    if iDebug & 0o2000 as libc::c_int != 0 as libc::c_int {
        udebug_buffer(
            b"fconn_io: Writing\0" as *const u8 as *const libc::c_char,
            zwrite,
            cwrite,
        );
    }
    fret = (Some(((*(*qconn).qcmds).pfio).unwrap()))
        .unwrap()(qconn, zwrite, pcwrite, zread, pcread);
    if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fconn_io: Wrote %lu of %lu, read %lu of %lu\0" as *const u8
                as *const libc::c_char,
            *pcwrite,
            cwrite,
            *pcread,
            cread,
        );
    }
    if *pcread > 0 as libc::c_int as libc::c_ulong
        && iDebug & 0o1000 as libc::c_int != 0 as libc::c_int
    {
        udebug_buffer(
            b"fconn_io: Read\0" as *const u8 as *const libc::c_char,
            zread,
            *pcread,
        );
    }
    return fret;
}
pub unsafe extern "C" fn fconn_break(mut qconn: *mut sconnection) -> boolean {
    let mut pfbreak: Option::<unsafe extern "C" fn(*mut sconnection) -> boolean> = None;
    pfbreak = (*(*qconn).qcmds).pfbreak;
    if pfbreak.is_none() {
        return 1 as libc::c_int;
    }
    if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fconn_break: Sending break character\0" as *const u8 as *const libc::c_char,
        );
    }
    return (Some(pfbreak.unwrap())).unwrap()(qconn);
}
pub unsafe extern "C" fn fconn_set(
    mut qconn: *mut sconnection,
    mut tparity: tparitysetting,
    mut tstrip: tstripsetting,
    mut txonxoff: txonxoffsetting,
) -> boolean {
    let mut pfset: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            tparitysetting,
            tstripsetting,
            txonxoffsetting,
        ) -> boolean,
    > = None;
    pfset = (*(*qconn).qcmds).pfset;
    if pfset.is_none() {
        return 1 as libc::c_int;
    }
    if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fconn_set: Changing setting to %d, %d, %d\0" as *const u8
                as *const libc::c_char,
            tparity as libc::c_int,
            tstrip as libc::c_int,
            txonxoff as libc::c_int,
        );
    }
    return (Some(pfset.unwrap())).unwrap()(qconn, tparity, tstrip, txonxoff);
}
pub unsafe extern "C" fn fconn_carrier(
    mut qconn: *mut sconnection,
    mut fcarrier: boolean,
) -> boolean {
    let mut pfcarrier: Option::<
        unsafe extern "C" fn(*mut sconnection, boolean) -> boolean,
    > = None;
    pfcarrier = (*(*qconn).qcmds).pfcarrier;
    if pfcarrier.is_none() {
        return 1 as libc::c_int;
    }
    return (Some(pfcarrier.unwrap())).unwrap()(qconn, fcarrier);
}
pub unsafe extern "C" fn fconn_run_chat(
    mut qconn: *mut sconnection,
    mut pzprog: *mut *mut libc::c_char,
) -> boolean {
    return (Some(((*(*qconn).qcmds).pfchat).unwrap())).unwrap()(qconn, pzprog);
}
pub unsafe extern "C" fn iconn_baud(mut qconn: *mut sconnection) -> libc::c_long {
    let mut pibaud: Option::<unsafe extern "C" fn(*mut sconnection) -> libc::c_long> = None;
    pibaud = (*(*qconn).qcmds).pibaud;
    if pibaud.is_none() {
        return 0 as libc::c_int as libc::c_long;
    }
    return (Some(pibaud.unwrap())).unwrap()(qconn);
}
pub unsafe extern "C" fn fconn_dial_sequence(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut pzdialer: *mut *mut libc::c_char,
    mut qsys: *const uuconf_system,
    mut zphone: *const libc::c_char,
    mut qdialer: *mut uuconf_dialer,
    mut ptdialerfound: *mut tdialerfound,
) -> boolean {
    let mut zname: *const libc::c_char = 0 as *const libc::c_char;
    let mut ffirst: boolean = 0;
    let mut ffreefirst: boolean = 0;
    if ((*qconn).qport).is_null() {
        zname = 0 as *const libc::c_char;
    } else {
        zname = (*(*qconn).qport).uuconf_zname;
    }
    ffirst = 1 as libc::c_int;
    ffreefirst = 0 as libc::c_int;
    while !(*pzdialer).is_null() {
        let mut q: *mut uuconf_dialer = 0 as *mut uuconf_dialer;
        let mut s: uuconf_dialer = uuconf_dialer {
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
        let mut ztoken: *const libc::c_char = 0 as *const libc::c_char;
        let mut ftranslate: boolean = 0;
        if ffirst == 0 {
            q = &mut s;
        } else {
            q = qdialer;
        }
        if ffirst == 0
            || *ptdialerfound as libc::c_uint
                == DIALERFOUND_FALSE as libc::c_int as libc::c_uint
        {
            let mut iuuconf: libc::c_int = 0;
            iuuconf = uuconf_dialer_info(puuconf, *pzdialer, q);
            if iuuconf == 1 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"%s: Dialer not found\0" as *const u8 as *const libc::c_char,
                    *pzdialer,
                );
                if ffreefirst != 0 {
                    uuconf_free_block((*qdialer).uuconf_palloc);
                }
                return 0 as libc::c_int;
            } else if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                if ffreefirst != 0 {
                    uuconf_free_block((*qdialer).uuconf_palloc);
                }
                return 0 as libc::c_int;
            }
            if ffirst != 0 {
                *ptdialerfound = DIALERFOUND_FREE;
                ffreefirst = 1 as libc::c_int;
            }
        }
        pzdialer = pzdialer.offset(1);
        pzdialer;
        ztoken = *pzdialer;
        ftranslate = 0 as libc::c_int;
        if ztoken.is_null()
            || strcmp(ztoken, b"\\D\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            ztoken = zphone;
        } else if strcmp(ztoken, b"\\T\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ztoken = zphone;
            ftranslate = 1 as libc::c_int;
        }
        if fchat(
            qconn,
            puuconf,
            &mut (*q).uuconf_schat,
            qsys,
            q,
            ztoken,
            ftranslate,
            zname,
            iconn_baud(qconn),
        ) == 0
        {
            if q == &mut s as *mut uuconf_dialer {
                uuconf_free_block((*q).uuconf_palloc);
            }
            if ffreefirst != 0 {
                uuconf_free_block((*qdialer).uuconf_palloc);
            }
            return 0 as libc::c_int;
        }
        if ffirst != 0 {
            ffirst = 0 as libc::c_int;
        } else {
            uuconf_free_block((*q).uuconf_palloc);
        }
        if !(*pzdialer).is_null() {
            pzdialer = pzdialer.offset(1);
            pzdialer;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fmodem_dial(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qsys: *const uuconf_system,
    mut zphone: *const libc::c_char,
    mut qdialer: *mut uuconf_dialer,
    mut ptdialerfound: *mut tdialerfound,
) -> boolean {
    let mut pzdialer: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *ptdialerfound = DIALERFOUND_FALSE;
    pzdialer = (*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_pzdialer;
    if !pzdialer.is_null() && !(*pzdialer).is_null() {
        let mut iuuconf: libc::c_int = 0;
        let mut fret: boolean = 0;
        iuuconf = uuconf_dialer_info(puuconf, *pzdialer, qdialer);
        if iuuconf == 1 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"%s: Dialer not found\0" as *const u8 as *const libc::c_char,
                *pzdialer,
            );
            return 0 as libc::c_int;
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            return 0 as libc::c_int;
        }
        *ptdialerfound = DIALERFOUND_FREE;
        fret = (fsysdep_modem_begin_dial(qconn, qdialer) != 0
            && fconn_dial_sequence(
                qconn,
                puuconf,
                pzdialer,
                qsys,
                zphone,
                qdialer,
                ptdialerfound,
            ) != 0 && fsysdep_modem_end_dial(qconn, qdialer) != 0) as libc::c_int;
        if fret == 0 {
            uuconf_free_block((*qdialer).uuconf_palloc);
        }
        return fret;
    } else if !((*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_qdialer).is_null() {
        let mut q: *mut uuconf_dialer = 0 as *mut uuconf_dialer;
        let mut zname: *const libc::c_char = 0 as *const libc::c_char;
        q = (*(*qconn).qport).uuconf_u.uuconf_smodem.uuconf_qdialer;
        *qdialer = *q;
        *ptdialerfound = DIALERFOUND_TRUE;
        if ((*qconn).qport).is_null() {
            zname = 0 as *const libc::c_char;
        } else {
            zname = (*(*qconn).qport).uuconf_zname;
        }
        return (fsysdep_modem_begin_dial(qconn, q) != 0
            && fchat(
                qconn,
                puuconf,
                &mut (*q).uuconf_schat,
                qsys,
                q,
                zphone,
                0 as libc::c_int,
                zname,
                iconn_baud(qconn),
            ) != 0 && fsysdep_modem_end_dial(qconn, q) != 0) as libc::c_int;
    } else {
        ulog(LOG_ERROR, b"No dialer information\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    };
}

use ::libc;
extern "C" {
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn fsysdep_mail(
        zto: *const libc::c_char,
        zsubject: *const libc::c_char,
        cstrs: libc::c_int,
        paz: *mut *const libc::c_char,
    ) -> boolean;
    fn fconn_read(
        qconn: *mut sconnection,
        zbuf: *mut libc::c_char,
        pclen: *mut size_t,
        cmin: size_t,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> boolean;
    fn fconn_write(
        qconn: *mut sconnection,
        zbuf: *const libc::c_char,
        cbytes: size_t,
    ) -> boolean;
    fn fconn_io(
        qconn: *mut sconnection,
        zwrite: *const libc::c_char,
        pcwrite: *mut size_t,
        zread: *mut libc::c_char,
        pcread: *mut size_t,
    ) -> boolean;
}
pub type size_t = libc::c_ulong;
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
pub static mut prot_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: prot.c,v 1.33 2002/03/05 19:10:41 ian Rel $\0")
};
pub static mut abPrecbuf: [libc::c_char; 16384] = [0; 16384];
pub static mut iPrecstart: libc::c_int = 0;
pub static mut iPrecend: libc::c_int = 0;
pub unsafe extern "C" fn fsend_data(
    mut qconn: *mut sconnection,
    mut zsend: *const libc::c_char,
    mut csend: size_t,
    mut fdoread: boolean,
) -> boolean {
    if fdoread == 0 {
        return fconn_write(qconn, zsend, csend);
    }
    while csend > 0 as libc::c_int as libc::c_ulong {
        let mut crec: size_t = 0;
        let mut csent: size_t = 0;
        if iPrecend < iPrecstart {
            crec = (iPrecstart - iPrecend - 1 as libc::c_int) as size_t;
        } else {
            crec = (16384 as libc::c_int - iPrecend) as size_t;
            if iPrecstart == 0 as libc::c_int {
                crec = crec.wrapping_sub(1);
                crec;
            }
        }
        if crec == 0 as libc::c_int as libc::c_ulong {
            return fconn_write(qconn, zsend, csend);
        }
        csent = csend;
        if fconn_io(
            qconn,
            zsend,
            &mut csent,
            abPrecbuf.as_mut_ptr().offset(iPrecend as isize),
            &mut crec,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        csend = (csend as libc::c_ulong).wrapping_sub(csent) as size_t as size_t;
        zsend = zsend.offset(csent as isize);
        iPrecend = (iPrecend as libc::c_ulong)
            .wrapping_add(crec)
            .wrapping_rem(16384 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn freceive_data(
    mut qconn: *mut sconnection,
    mut cneed: size_t,
    mut pcrec: *mut size_t,
    mut ctimeout: libc::c_int,
    mut freport: boolean,
) -> boolean {
    if iPrecend < iPrecstart {
        *pcrec = (iPrecstart - iPrecend - 1 as libc::c_int) as size_t;
    } else {
        *pcrec = (16384 as libc::c_int - iPrecend) as size_t;
        if iPrecstart == 0 as libc::c_int {
            *pcrec = (*pcrec).wrapping_sub(1);
            *pcrec;
        }
    }
    if *pcrec == 0 as libc::c_int as libc::c_ulong {
        ulog(
            LOG_FATAL,
            b"freceive_data: No room in buffer\0" as *const u8 as *const libc::c_char,
        );
    }
    if *pcrec < cneed {
        cneed = *pcrec;
    }
    if fconn_read(
        qconn,
        abPrecbuf.as_mut_ptr().offset(iPrecend as isize),
        pcrec,
        cneed,
        ctimeout,
        freport,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    iPrecend = (iPrecend as libc::c_ulong)
        .wrapping_add(*pcrec)
        .wrapping_rem(16384 as libc::c_int as libc::c_ulong) as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn breceive_char(
    mut qconn: *mut sconnection,
    mut ctimeout: libc::c_int,
    mut freport: boolean,
) -> libc::c_int {
    let mut b: libc::c_char = 0;
    if iPrecstart == iPrecend {
        let mut crec: size_t = 0;
        if freceive_data(
            qconn,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            &mut crec,
            ctimeout,
            freport,
        ) == 0
        {
            return -(2 as libc::c_int);
        }
        if crec == 0 as libc::c_int as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    b = abPrecbuf[iPrecstart as usize];
    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
    return b as libc::c_uchar as libc::c_int;
}
pub unsafe extern "C" fn fmail_transfer(
    mut fsuccess: boolean,
    mut zuser: *const libc::c_char,
    mut zmail: *const libc::c_char,
    mut zwhy: *const libc::c_char,
    mut zfromfile: *const libc::c_char,
    mut zfromsys: *const libc::c_char,
    mut ztofile: *const libc::c_char,
    mut ztosys: *const libc::c_char,
    mut zsaved: *const libc::c_char,
) -> boolean {
    let mut zsendto: *const libc::c_char = 0 as *const libc::c_char;
    let mut az: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
    let mut i: libc::c_int = 0;
    if !zmail.is_null() && *zmail as libc::c_int != '\0' as i32 {
        zsendto = zmail;
    } else {
        zsendto = zuser;
    }
    i = 0 as libc::c_int;
    let fresh0 = i;
    i = i + 1;
    az[fresh0 as usize] = b"The file\n\t\0" as *const u8 as *const libc::c_char;
    if !zfromsys.is_null() {
        let fresh1 = i;
        i = i + 1;
        az[fresh1 as usize] = zfromsys;
        let fresh2 = i;
        i = i + 1;
        az[fresh2 as usize] = b"!\0" as *const u8 as *const libc::c_char;
    }
    let fresh3 = i;
    i = i + 1;
    az[fresh3 as usize] = zfromfile;
    if fsuccess != 0 {
        let fresh4 = i;
        i = i + 1;
        az[fresh4
            as usize] = b"\nwas successfully transferred to\n\t\0" as *const u8
            as *const libc::c_char;
    } else {
        let fresh5 = i;
        i = i + 1;
        az[fresh5
            as usize] = b"\ncould not be transferred to\n\t\0" as *const u8
            as *const libc::c_char;
    }
    if !ztosys.is_null() {
        let fresh6 = i;
        i = i + 1;
        az[fresh6 as usize] = ztosys;
        let fresh7 = i;
        i = i + 1;
        az[fresh7 as usize] = b"!\0" as *const u8 as *const libc::c_char;
    }
    let fresh8 = i;
    i = i + 1;
    az[fresh8 as usize] = ztofile;
    let fresh9 = i;
    i = i + 1;
    az[fresh9 as usize] = b"\nas requested by\n\t\0" as *const u8 as *const libc::c_char;
    let fresh10 = i;
    i = i + 1;
    az[fresh10 as usize] = zuser;
    if fsuccess == 0 {
        let fresh11 = i;
        i = i + 1;
        az[fresh11
            as usize] = b"\nfor the following reason:\n\t\0" as *const u8
            as *const libc::c_char;
        let fresh12 = i;
        i = i + 1;
        az[fresh12 as usize] = zwhy;
        let fresh13 = i;
        i = i + 1;
        az[fresh13 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
    }
    if !zsaved.is_null() {
        let fresh14 = i;
        i = i + 1;
        az[fresh14 as usize] = zsaved;
        let fresh15 = i;
        i = i + 1;
        az[fresh15 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
    }
    return fsysdep_mail(
        zsendto,
        if fsuccess != 0 {
            b"UUCP succeeded\0" as *const u8 as *const libc::c_char
        } else {
            b"UUCP failed\0" as *const u8 as *const libc::c_char
        },
        i,
        az.as_mut_ptr(),
    );
}

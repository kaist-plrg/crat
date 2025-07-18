use ::libc;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn udebug_buffer(zhdr: *const libc::c_char, zbuf: *const libc::c_char, clen: size_t);
    fn cdebug_char(z: *mut libc::c_char, ichar: libc::c_int) -> size_t;
    fn cescape(zbuf: *mut libc::c_char) -> size_t;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    static mut zLdevice: *mut libc::c_char;
    fn uuconf_callout(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *const uuconf_system,
        uuconf_pzlog: *mut *mut libc::c_char,
        uuconf_pzpass: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn uuconf_dialcode(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdial: *const libc::c_char,
        uuconf_pznum: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn fconn_write(
        qconn: *mut sconnection,
        zbuf: *const libc::c_char,
        cbytes: size_t,
    ) -> boolean;
    fn fconn_break(qconn: *mut sconnection) -> boolean;
    fn fconn_carrier(qconn: *mut sconnection, fcarrier: boolean) -> boolean;
    fn fconn_run_chat(
        qconn: *mut sconnection,
        pzprog: *mut *mut libc::c_char,
    ) -> boolean;
    fn breceive_char(
        qconn: *mut sconnection,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> libc::c_int;
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn usysdep_sleep(cseconds: libc::c_int);
    fn usysdep_pause();
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub uuconf_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub static mut chat_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: chat.c,v 1.50 2002/03/05 19:10:41 ian Rel $\0")
};
pub unsafe extern "C" fn fchat(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qchat: *const uuconf_chat,
    mut qsys: *const uuconf_system,
    mut qdial: *const uuconf_dialer,
    mut zphone: *const libc::c_char,
    mut ftranslate: boolean,
    mut zport: *const libc::c_char,
    mut ibaud: libc::c_long,
) -> boolean {
    let mut cstrings: libc::c_int = 0;
    let mut azstrings: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut aclens: *mut size_t = 0 as *mut size_t;
    let mut pzchat: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut zbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cbuflen: size_t = 0;
    let mut fret: boolean = 0;
    let mut i: libc::c_int = 0;
    if !((*qchat).uuconf_pzprogram).is_null() {
        if fcprogram(
            qconn,
            puuconf,
            (*qchat).uuconf_pzprogram,
            qsys,
            qdial,
            zphone,
            zport,
            ibaud,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if ((*qchat).uuconf_pzchat).is_null() {
        return 1 as libc::c_int;
    }
    if ((*qchat).uuconf_pzfail).is_null() {
        cstrings = 1 as libc::c_int;
        azstrings = xmalloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as *mut *mut libc::c_char;
        aclens = xmalloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
            as *mut size_t;
    } else {
        let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        cstrings = 1 as libc::c_int;
        pz = (*qchat).uuconf_pzfail;
        while !(*pz).is_null() {
            cstrings += 1;
            cstrings;
            pz = pz.offset(1);
            pz;
        }
        azstrings = xmalloc(
            (cstrings as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        aclens = xmalloc(
            (cstrings as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as *mut size_t;
        cstrings = 1 as libc::c_int;
        pz = (*qchat).uuconf_pzfail;
        while !(*pz).is_null() {
            let ref mut fresh0 = *azstrings.offset(cstrings as isize);
            *fresh0 = zbufcpy(*pz);
            *aclens
                .offset(
                    cstrings as isize,
                ) = cescape(*azstrings.offset(cstrings as isize));
            cstrings += 1;
            cstrings;
            pz = pz.offset(1);
            pz;
        }
    }
    cbuflen = 0 as libc::c_int as size_t;
    zbuf = 0 as *mut libc::c_char;
    fret = 1 as libc::c_int;
    pzchat = (*qchat).uuconf_pzchat;
    while !(*pzchat).is_null() {
        let mut clen: size_t = 0;
        loop {
            let mut ztimeout: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ctimeout: libc::c_int = 0;
            clen = strlen(*pzchat);
            if clen >= cbuflen {
                ubuffree(zbuf);
                zbuf = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
                cbuflen = clen;
            }
            memcpy(
                zbuf as *mut libc::c_void,
                *pzchat as *const libc::c_void,
                clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let ref mut fresh1 = *azstrings.offset(0 as libc::c_int as isize);
            *fresh1 = zbuf;
            if *(*azstrings.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                let ref mut fresh2 = *azstrings.offset(0 as libc::c_int as isize);
                *fresh2 = (*fresh2).offset(1);
                *fresh2;
            }
            ctimeout = (*qchat).uuconf_ctimeout;
            ztimeout = strrchr(
                *azstrings.offset(0 as libc::c_int as isize),
                '\\' as i32,
            );
            if !ztimeout.is_null()
                && *ztimeout.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'W' as i32
            {
                let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cval: libc::c_int = 0;
                cval = strtol(
                    ztimeout.offset(2 as libc::c_int as isize),
                    &mut zend,
                    10 as libc::c_int,
                ) as libc::c_int;
                if zend != ztimeout.offset(2 as libc::c_int as isize)
                    && *zend as libc::c_int == '\0' as i32
                {
                    ctimeout = cval;
                    *ztimeout = '\0' as i32 as libc::c_char;
                }
            }
            *aclens
                .offset(
                    0 as libc::c_int as isize,
                ) = cescape(*azstrings.offset(0 as libc::c_int as isize));
            if *aclens.offset(0 as libc::c_int as isize)
                == 0 as libc::c_int as libc::c_ulong
                || *aclens.offset(0 as libc::c_int as isize)
                    == 2 as libc::c_int as libc::c_ulong
                    && strcmp(
                        *azstrings.offset(0 as libc::c_int as isize),
                        b"\"\"\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
            {
                if (*pzchat.offset(1 as libc::c_int as isize)).is_null()
                    || *(*pzchat.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                {
                    break;
                }
            } else {
                let mut istr: libc::c_int = 0;
                istr = icexpect(
                    qconn,
                    cstrings,
                    azstrings,
                    aclens,
                    ctimeout,
                    (*qchat).uuconf_fstrip,
                );
                if istr == 0 as libc::c_int {
                    break;
                }
                if istr < -(1 as libc::c_int) {
                    fret = 0 as libc::c_int;
                    break;
                } else if istr > 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"Chat script failed: Got \"%s\"\0" as *const u8
                            as *const libc::c_char,
                        *((*qchat).uuconf_pzfail)
                            .offset((istr - 1 as libc::c_int) as isize),
                    );
                    fret = 0 as libc::c_int;
                    break;
                } else if (*pzchat.offset(1 as libc::c_int as isize)).is_null()
                    || *(*pzchat.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                {
                    ulog(
                        LOG_ERROR,
                        b"Timed out in chat script\0" as *const u8 as *const libc::c_char,
                    );
                    fret = 0 as libc::c_int;
                    break;
                }
            }
            pzchat = pzchat.offset(1);
            pzchat;
            if fcsend(
                qconn,
                puuconf,
                (*pzchat).offset(1 as libc::c_int as isize),
                qsys,
                qdial,
                zphone,
                ftranslate,
                (*qchat).uuconf_fstrip,
            ) == 0
            {
                fret = 0 as libc::c_int;
                break;
            } else {
                if (*pzchat.offset(1 as libc::c_int as isize)).is_null()
                    || *(*pzchat.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                {
                    break;
                }
                pzchat = pzchat.offset(1);
                pzchat;
            }
        }
        if fret == 0 {
            break;
        }
        loop {
            pzchat = pzchat.offset(1);
            pzchat;
            if !(!(*pzchat).is_null()
                && *(*pzchat).offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32)
            {
                break;
            }
        }
        if (*pzchat).is_null() {
            break;
        }
        if **pzchat as libc::c_int != '\0' as i32 {
            if fcsend(
                qconn,
                puuconf,
                *pzchat,
                qsys,
                qdial,
                zphone,
                ftranslate,
                (*qchat).uuconf_fstrip,
            ) == 0
            {
                fret = 0 as libc::c_int;
                break;
            }
        }
        pzchat = pzchat.offset(1);
        pzchat;
    }
    ubuffree(zbuf);
    i = 1 as libc::c_int;
    while i < cstrings {
        ubuffree(*azstrings.offset(i as isize));
        i += 1;
        i;
    }
    xfree(azstrings as pointer);
    xfree(aclens as pointer);
    return fret;
}
unsafe extern "C" fn icexpect(
    mut qconn: *mut sconnection,
    mut cstrings: libc::c_int,
    mut azstrings: *mut *mut libc::c_char,
    mut aclens: *mut size_t,
    mut ctimeout: libc::c_int,
    mut fstrip: boolean,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cmax: size_t = 0;
    let mut zhave: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chave: size_t = 0;
    let mut iendtime: libc::c_long = 0;
    let mut cchars: libc::c_int = 0;
    let mut iolddebug: libc::c_int = 0;
    cmax = *aclens.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < cstrings {
        if cmax < *aclens.offset(i as isize) {
            cmax = *aclens.offset(i as isize);
        }
        i += 1;
        i;
    }
    zhave = zbufalc(cmax);
    chave = 0 as libc::c_int as size_t;
    iendtime = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
        + ctimeout as libc::c_long;
    cchars = 0 as libc::c_int;
    iolddebug = iDebug;
    if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
        udebug_buffer(
            b"icexpect: Looking for\0" as *const u8 as *const libc::c_char,
            *azstrings.offset(0 as libc::c_int as isize),
            *aclens.offset(0 as libc::c_int as isize),
        );
        ulog(LOG_DEBUG_START, b"icexpect: Got \"\0" as *const u8 as *const libc::c_char);
        iDebug &= !(0o1000 as libc::c_int | 0o40 as libc::c_int);
    }
    loop {
        let mut bchar: libc::c_int = 0;
        if ctimeout <= 0 as libc::c_int {
            if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG_END,
                    b"\" (timed out)\0" as *const u8 as *const libc::c_char,
                );
                iDebug = iolddebug;
            }
            ubuffree(zhave);
            return -(1 as libc::c_int);
        }
        if chave >= cmax {
            let mut imove: size_t = 0;
            imove = 0 as libc::c_int as size_t;
            while imove < cmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                *zhave
                    .offset(
                        imove as isize,
                    ) = *zhave
                    .offset(
                        imove.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                imove = imove.wrapping_add(1);
                imove;
            }
            chave = chave.wrapping_sub(1);
            chave;
        }
        bchar = breceive_char(qconn, ctimeout, 1 as libc::c_int);
        if bchar < 0 as libc::c_int {
            if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG_END,
                    b"\" (%s)\0" as *const u8 as *const libc::c_char,
                    if bchar == -(1 as libc::c_int) {
                        b"timed out\0" as *const u8 as *const libc::c_char
                    } else {
                        b"error\0" as *const u8 as *const libc::c_char
                    },
                );
                iDebug = iolddebug;
            }
            ubuffree(zhave);
            return bchar;
        }
        if fstrip != 0 {
            bchar &= 0x7f as libc::c_int;
        }
        *zhave.offset(chave as isize) = bchar as libc::c_char;
        chave = chave.wrapping_add(1);
        chave;
        if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
            let mut ab: [libc::c_char; 5] = [0; 5];
            cchars += 1;
            cchars;
            if cchars > 60 as libc::c_int {
                ulog(LOG_DEBUG_END, b"\"\0" as *const u8 as *const libc::c_char);
                ulog(
                    LOG_DEBUG_START,
                    b"icexpect: Got \"\0" as *const u8 as *const libc::c_char,
                );
                cchars = 0 as libc::c_int;
            }
            cdebug_char(ab.as_mut_ptr(), bchar);
            ulog(
                LOG_DEBUG_CONTINUE,
                b"%s\0" as *const u8 as *const libc::c_char,
                ab.as_mut_ptr(),
            );
        }
        i = 0 as libc::c_int;
        while i < cstrings {
            if *aclens.offset(i as isize) <= chave
                && memcmp(
                    zhave
                        .offset(chave as isize)
                        .offset(-(*aclens.offset(i as isize) as isize))
                        as *const libc::c_void,
                    *azstrings.offset(i as isize) as *const libc::c_void,
                    *aclens.offset(i as isize),
                ) == 0 as libc::c_int
            {
                if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
                    if i == 0 as libc::c_int {
                        ulog(
                            LOG_DEBUG_END,
                            b"\" (found it)\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        ulog(LOG_DEBUG_END, b"\"\0" as *const u8 as *const libc::c_char);
                        udebug_buffer(
                            b"icexpect: Found\0" as *const u8 as *const libc::c_char,
                            *azstrings.offset(i as isize),
                            *aclens.offset(i as isize),
                        );
                    }
                    iDebug = iolddebug;
                }
                ubuffree(zhave);
                return i;
            }
            i += 1;
            i;
        }
        ctimeout = (iendtime
            - ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)) as libc::c_int;
    };
}
static mut cCsend_chars: size_t = 0;
static mut iColddebug: libc::c_int = 0;
unsafe extern "C" fn fcsend_debug(
    mut fquote: boolean,
    mut clen: size_t,
    mut zbuf: *const libc::c_char,
) -> boolean {
    let mut cwas: size_t = 0;
    if !(iDebug & 0o2 as libc::c_int != 0 as libc::c_int) {
        return 1 as libc::c_int;
    }
    cwas = cCsend_chars;
    if clen > 0 as libc::c_int as libc::c_ulong {
        cCsend_chars = (cCsend_chars as libc::c_ulong).wrapping_add(clen) as size_t
            as size_t;
    } else {
        cCsend_chars = (cCsend_chars as libc::c_ulong).wrapping_add(strlen(zbuf))
            as size_t as size_t;
    }
    if cCsend_chars > 60 as libc::c_int as libc::c_ulong
        && cwas > 10 as libc::c_int as libc::c_ulong
    {
        ulog(
            LOG_DEBUG_END,
            b"%s\0" as *const u8 as *const libc::c_char,
            if fquote != 0 {
                b"\"\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        fquote = 0 as libc::c_int;
        ulog(LOG_DEBUG_START, b"fcsend: Writing\0" as *const u8 as *const libc::c_char);
        cCsend_chars = 0 as libc::c_int as size_t;
    }
    if clen == 0 as libc::c_int as libc::c_ulong {
        ulog(
            LOG_DEBUG_CONTINUE,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            if fquote != 0 {
                b"\"\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            zbuf,
        );
        return 0 as libc::c_int;
    } else {
        let mut i: size_t = 0;
        if fquote == 0 {
            ulog(LOG_DEBUG_CONTINUE, b" \"\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < clen {
            let mut ab: [libc::c_char; 5] = [0; 5];
            cdebug_char(ab.as_mut_ptr(), *zbuf.offset(i as isize) as libc::c_int);
            ulog(
                LOG_DEBUG_CONTINUE,
                b"%s\0" as *const u8 as *const libc::c_char,
                ab.as_mut_ptr(),
            );
            i = i.wrapping_add(1);
            i;
        }
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn ucsend_debug_end(mut fquote: boolean, mut ferr: boolean) {
    if !(iDebug & 0o2 as libc::c_int != 0 as libc::c_int) {
        return;
    }
    if fquote != 0 {
        ulog(LOG_DEBUG_CONTINUE, b"\"\0" as *const u8 as *const libc::c_char);
    }
    if ferr != 0 {
        ulog(LOG_DEBUG_CONTINUE, b" (error)\0" as *const u8 as *const libc::c_char);
    }
    ulog(
        LOG_DEBUG_END,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    iDebug = iColddebug;
}
unsafe extern "C" fn fcsend(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut z: *const libc::c_char,
    mut qsys: *const uuconf_system,
    mut qdial: *const uuconf_dialer,
    mut zphone: *const libc::c_char,
    mut ftranslate: boolean,
    mut fstrip: boolean,
) -> boolean {
    let mut fnocr: boolean = 0;
    let mut pfwrite: Option::<
        unsafe extern "C" fn(*mut sconnection, *const libc::c_char, size_t) -> boolean,
    > = None;
    let mut zcallout_login: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zcallout_pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fquote: boolean = 0;
    if strcmp(z, b"\"\"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    fnocr = 0 as libc::c_int;
    pfwrite = Some(
        fconn_write
            as unsafe extern "C" fn(
                *mut sconnection,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    zcallout_login = 0 as *mut libc::c_char;
    zcallout_pass = 0 as *mut libc::c_char;
    if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
        ulog(LOG_DEBUG_START, b"fcsend: Writing\0" as *const u8 as *const libc::c_char);
        fquote = 0 as libc::c_int;
        cCsend_chars = 0 as libc::c_int as size_t;
        iColddebug = iDebug;
        iDebug &= !(0o2000 as libc::c_int | 0o40 as libc::c_int);
    }
    while *z as libc::c_int != '\0' as i32 {
        let mut zlook: *const libc::c_char = 0 as *const libc::c_char;
        let mut fsend: boolean = 0;
        let mut bsend: libc::c_char = 0;
        zlook = z
            .offset(
                strcspn(
                    z as *mut libc::c_char,
                    b"\\BE\0" as *const u8 as *const libc::c_char,
                ) as isize,
            );
        if zlook > z {
            let mut c: size_t = 0;
            c = zlook.offset_from(z) as libc::c_long as size_t;
            fquote = fcsend_debug(fquote, c, z);
            if (Some(pfwrite.unwrap())).unwrap()(qconn, z, c) == 0 {
                ucsend_debug_end(fquote, 1 as libc::c_int);
                return 0 as libc::c_int;
            }
        }
        if *zlook as libc::c_int == '\0' as i32 {
            break;
        }
        z = zlook;
        fsend = 0 as libc::c_int;
        match *z as libc::c_int {
            66 => {
                if strncmp(
                    z,
                    b"BREAK\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    fquote = fcsend_debug(
                        fquote,
                        0 as libc::c_int as size_t,
                        b"break\0" as *const u8 as *const libc::c_char,
                    );
                    if fconn_break(qconn) == 0 {
                        ucsend_debug_end(fquote, 1 as libc::c_int);
                        return 0 as libc::c_int;
                    }
                    fnocr = 1 as libc::c_int;
                    z = z.offset(5 as libc::c_int as isize);
                } else {
                    fsend = 1 as libc::c_int;
                    bsend = 'B' as i32 as libc::c_char;
                    z = z.offset(1);
                    z;
                }
            }
            69 => {
                if strncmp(
                    z,
                    b"EOT\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    fsend = 1 as libc::c_int;
                    bsend = '\u{4}' as i32 as libc::c_char;
                    fnocr = 1 as libc::c_int;
                    z = z.offset(3 as libc::c_int as isize);
                } else {
                    fsend = 1 as libc::c_int;
                    bsend = 'E' as i32 as libc::c_char;
                    z = z.offset(1);
                    z;
                }
            }
            92 => {
                z = z.offset(1);
                z;
                let mut current_block_206: u64;
                match *z as libc::c_int {
                    45 => {
                        fsend = 1 as libc::c_int;
                        bsend = '-' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    98 => {
                        fsend = 1 as libc::c_int;
                        bsend = '\u{8}' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    99 => {
                        fnocr = 1 as libc::c_int;
                        current_block_206 = 17648591037158480576;
                    }
                    100 => {
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"sleep\0" as *const u8 as *const libc::c_char,
                        );
                        usysdep_sleep(1 as libc::c_int);
                        current_block_206 = 17648591037158480576;
                    }
                    101 => {
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"echo-check-off\0" as *const u8 as *const libc::c_char,
                        );
                        pfwrite = Some(
                            fconn_write
                                as unsafe extern "C" fn(
                                    *mut sconnection,
                                    *const libc::c_char,
                                    size_t,
                                ) -> boolean,
                        );
                        current_block_206 = 17648591037158480576;
                    }
                    69 => {
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"echo-check-on\0" as *const u8 as *const libc::c_char,
                        );
                        if fstrip != 0 {
                            pfwrite = Some(
                                fcecho_send_strip
                                    as unsafe extern "C" fn(
                                        *mut sconnection,
                                        *const libc::c_char,
                                        size_t,
                                    ) -> boolean,
                            );
                        } else {
                            pfwrite = Some(
                                fcecho_send_nostrip
                                    as unsafe extern "C" fn(
                                        *mut sconnection,
                                        *const libc::c_char,
                                        size_t,
                                    ) -> boolean,
                            );
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    75 => {
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"break\0" as *const u8 as *const libc::c_char,
                        );
                        if fconn_break(qconn) == 0 {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    110 => {
                        fsend = 1 as libc::c_int;
                        bsend = '\n' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    78 => {
                        fsend = 1 as libc::c_int;
                        bsend = '\0' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    112 => {
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"pause\0" as *const u8 as *const libc::c_char,
                        );
                        usysdep_pause();
                        current_block_206 = 17648591037158480576;
                    }
                    114 => {
                        fsend = 1 as libc::c_int;
                        bsend = '\r' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    115 => {
                        fsend = 1 as libc::c_int;
                        bsend = ' ' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    116 => {
                        fsend = 1 as libc::c_int;
                        bsend = '\t' as i32 as libc::c_char;
                        current_block_206 = 17648591037158480576;
                    }
                    0 => {
                        z = z.offset(-1);
                        z;
                        current_block_206 = 4469190921125482710;
                    }
                    92 => {
                        current_block_206 = 4469190921125482710;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        fsend = 1 as libc::c_int;
                        bsend = (*z as libc::c_int - '0' as i32) as libc::c_char;
                        if *z.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *z.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '7' as i32
                        {
                            z = z.offset(1);
                            bsend = (8 as libc::c_int * bsend as libc::c_int
                                + *z as libc::c_int - '0' as i32) as libc::c_char;
                        }
                        if *z.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *z.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '7' as i32
                        {
                            z = z.offset(1);
                            bsend = (8 as libc::c_int * bsend as libc::c_int
                                + *z as libc::c_int - '0' as i32) as libc::c_char;
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    120 => {
                        fsend = 1 as libc::c_int;
                        bsend = 0 as libc::c_int as libc::c_char;
                        while *(*__ctype_b_loc())
                            .offset(
                                *z.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            if *(*__ctype_b_loc())
                                .offset(
                                    *z.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                z = z.offset(1);
                                bsend = (16 as libc::c_int * bsend as libc::c_int
                                    + *z as libc::c_int - '0' as i32) as libc::c_char;
                            } else if *(*__ctype_b_loc())
                                .offset(
                                    *z.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                z = z.offset(1);
                                bsend = (16 as libc::c_int * bsend as libc::c_int
                                    + *z as libc::c_int - 'A' as i32 + 10 as libc::c_int)
                                    as libc::c_char;
                            } else {
                                z = z.offset(1);
                                bsend = (16 as libc::c_int * bsend as libc::c_int
                                    + *z as libc::c_int - 'a' as i32 + 10 as libc::c_int)
                                    as libc::c_char;
                            }
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    76 => {
                        let mut zlog: *const libc::c_char = 0 as *const libc::c_char;
                        let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut clen: size_t = 0;
                        if qsys.is_null() {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            ulog(
                                LOG_ERROR,
                                b"Illegal use of \\L\0" as *const u8 as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        zlog = (*qsys).uuconf_zcall_login;
                        if zlog.is_null() {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            ulog(
                                LOG_ERROR,
                                b"No login defined\0" as *const u8 as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        if *zlog.offset(0 as libc::c_int as isize) as libc::c_int
                            == '*' as i32
                            && *zlog.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32
                        {
                            if zcallout_login.is_null() {
                                let mut iuuconf: libc::c_int = 0;
                                iuuconf = uuconf_callout(
                                    puuconf,
                                    qsys,
                                    &mut zcallout_login,
                                    &mut zcallout_pass,
                                );
                                if iuuconf == 1 as libc::c_int || zcallout_login.is_null() {
                                    ucsend_debug_end(fquote, 1 as libc::c_int);
                                    ulog(
                                        LOG_ERROR,
                                        b"No login defined\0" as *const u8 as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int;
                                } else if iuuconf != 0 as libc::c_int {
                                    ucsend_debug_end(fquote, 1 as libc::c_int);
                                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                                    return 0 as libc::c_int;
                                }
                            }
                            zlog = zcallout_login;
                        }
                        zcopy = zbufcpy(zlog);
                        clen = cescape(zcopy);
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"login\0" as *const u8 as *const libc::c_char,
                        );
                        fquote = fcsend_debug(fquote, clen, zcopy);
                        if (Some(pfwrite.unwrap())).unwrap()(qconn, zcopy, clen) == 0 {
                            ubuffree(zcopy);
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        ubuffree(zcopy);
                        current_block_206 = 17648591037158480576;
                    }
                    80 => {
                        let mut zpass: *const libc::c_char = 0 as *const libc::c_char;
                        let mut zcopy_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut clen_0: size_t = 0;
                        if qsys.is_null() {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            ulog(
                                LOG_ERROR,
                                b"Illegal use of \\P\0" as *const u8 as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        zpass = (*qsys).uuconf_zcall_password;
                        if zpass.is_null() {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            ulog(
                                LOG_ERROR,
                                b"No password defined\0" as *const u8 as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        if *zpass.offset(0 as libc::c_int as isize) as libc::c_int
                            == '*' as i32
                            && *zpass.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32
                        {
                            if zcallout_pass.is_null() {
                                let mut iuuconf_0: libc::c_int = 0;
                                iuuconf_0 = uuconf_callout(
                                    puuconf,
                                    qsys,
                                    &mut zcallout_login,
                                    &mut zcallout_pass,
                                );
                                if iuuconf_0 == 1 as libc::c_int || zcallout_pass.is_null()
                                {
                                    ucsend_debug_end(fquote, 1 as libc::c_int);
                                    ulog(
                                        LOG_ERROR,
                                        b"No password defined\0" as *const u8 as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int;
                                } else if iuuconf_0 != 0 as libc::c_int {
                                    ucsend_debug_end(fquote, 1 as libc::c_int);
                                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf_0);
                                    return 0 as libc::c_int;
                                }
                            }
                            zpass = zcallout_pass;
                        }
                        zcopy_0 = zbufcpy(zpass);
                        clen_0 = cescape(zcopy_0);
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"password\0" as *const u8 as *const libc::c_char,
                        );
                        fquote = fcsend_debug(fquote, clen_0, zcopy_0);
                        if (Some(pfwrite.unwrap())).unwrap()(qconn, zcopy_0, clen_0) == 0
                        {
                            ubuffree(zcopy_0);
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        ubuffree(zcopy_0);
                        current_block_206 = 17648591037158480576;
                    }
                    68 => {
                        if qdial.is_null() || zphone.is_null() {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            ulog(
                                LOG_ERROR,
                                b"Illegal use of \\D\0" as *const u8 as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"\\D\0" as *const u8 as *const libc::c_char,
                        );
                        if fcphone(
                            qconn,
                            puuconf,
                            qdial,
                            zphone,
                            pfwrite,
                            ftranslate,
                            &mut fquote,
                        ) == 0
                        {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    84 => {
                        if qdial.is_null() || zphone.is_null() {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            ulog(
                                LOG_ERROR,
                                b"Illegal use of \\T\0" as *const u8 as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"\\T\0" as *const u8 as *const libc::c_char,
                        );
                        if fcphone(
                            qconn,
                            puuconf,
                            qdial,
                            zphone,
                            pfwrite,
                            1 as libc::c_int,
                            &mut fquote,
                        ) == 0
                        {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    77 => {
                        fquote = fcsend_debug(
                            fquote,
                            0 as libc::c_int as size_t,
                            b"ignore-carrier\0" as *const u8 as *const libc::c_char,
                        );
                        if fconn_carrier(qconn, 0 as libc::c_int) == 0 {
                            ucsend_debug_end(fquote, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    109 => {
                        if qdial.is_null() || (*qdial).uuconf_fcarrier != 0 {
                            fquote = fcsend_debug(
                                fquote,
                                0 as libc::c_int as size_t,
                                b"need-carrier\0" as *const u8 as *const libc::c_char,
                            );
                            if fconn_carrier(qconn, 1 as libc::c_int) == 0 {
                                ucsend_debug_end(fquote, 1 as libc::c_int);
                                return 0 as libc::c_int;
                            }
                        }
                        current_block_206 = 17648591037158480576;
                    }
                    _ => {
                        ulog(
                            LOG_ERROR,
                            b"Unrecognized escape sequence \\%c in send string\0"
                                as *const u8 as *const libc::c_char,
                            *z as libc::c_int,
                        );
                        fsend = 1 as libc::c_int;
                        bsend = *z;
                        current_block_206 = 17648591037158480576;
                    }
                }
                match current_block_206 {
                    4469190921125482710 => {
                        fsend = 1 as libc::c_int;
                        bsend = '\\' as i32 as libc::c_char;
                    }
                    _ => {}
                }
                z = z.offset(1);
                z;
            }
            _ => {
                ulog(
                    LOG_FATAL,
                    b"fcsend: Can't happen\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if fsend != 0 {
            fquote = fcsend_debug(fquote, 1 as libc::c_int as size_t, &mut bsend);
            if (Some(pfwrite.unwrap()))
                .unwrap()(qconn, &mut bsend, 1 as libc::c_int as size_t) == 0
            {
                ucsend_debug_end(fquote, 1 as libc::c_int);
                return 0 as libc::c_int;
            }
        }
    }
    xfree(zcallout_login as pointer);
    xfree(zcallout_pass as pointer);
    if fnocr == 0 {
        let mut b: libc::c_char = 0;
        b = '\r' as i32 as libc::c_char;
        fquote = fcsend_debug(fquote, 1 as libc::c_int as size_t, &mut b);
        if fconn_write(qconn, &mut b, 1 as libc::c_int as size_t) == 0 {
            ucsend_debug_end(fquote, 1 as libc::c_int);
            return 0 as libc::c_int;
        }
    }
    ucsend_debug_end(fquote, 0 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn fcphone(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut qdial: *const uuconf_dialer,
    mut zphone: *const libc::c_char,
    mut pfwrite: Option::<
        unsafe extern "C" fn(*mut sconnection, *const libc::c_char, size_t) -> boolean,
    >,
    mut ftranslate: boolean,
    mut pfquote: *mut boolean,
) -> boolean {
    let mut zprefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut zsuffix: *const libc::c_char = 0 as *const libc::c_char;
    if ftranslate != 0 {
        if fctranslate(puuconf, zphone, &mut zprefix, &mut zsuffix) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        zprefix = zphone;
        zsuffix = 0 as *const libc::c_char;
    }
    while !zprefix.is_null() {
        loop {
            let mut z: *const libc::c_char = 0 as *const libc::c_char;
            let mut zstr: *const libc::c_char = 0 as *const libc::c_char;
            z = zprefix
                .offset(
                    strcspn(
                        zprefix as *mut libc::c_char,
                        b"=-\0" as *const u8 as *const libc::c_char,
                    ) as isize,
                );
            if z > zprefix {
                let mut clen: size_t = 0;
                clen = z.offset_from(zprefix) as libc::c_long as size_t;
                *pfquote = fcsend_debug(*pfquote, clen, zprefix);
                if (Some(pfwrite.unwrap())).unwrap()(qconn, zprefix, clen) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if *z as libc::c_int == '=' as i32 {
                zstr = (*qdial).uuconf_zdialtone;
            } else {
                if !(*z as libc::c_int == '-' as i32) {
                    break;
                }
                zstr = (*qdial).uuconf_zpause;
            }
            if !zstr.is_null() {
                *pfquote = fcsend_debug(*pfquote, strlen(zstr), zstr);
                if (Some(pfwrite.unwrap())).unwrap()(qconn, zstr, strlen(zstr)) == 0 {
                    return 0 as libc::c_int;
                }
            }
            zprefix = z.offset(1 as libc::c_int as isize);
        }
        zprefix = zsuffix;
        zsuffix = 0 as *const libc::c_char;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fctranslate(
    mut puuconf: pointer,
    mut zphone: *const libc::c_char,
    mut pzprefix: *mut *const libc::c_char,
    mut pzsuffix: *mut *const libc::c_char,
) -> boolean {
    let mut iuuconf: libc::c_int = 0;
    let mut zdialcode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
    let mut ztrans: *mut libc::c_char = 0 as *mut libc::c_char;
    *pzprefix = zphone;
    *pzsuffix = 0 as *const libc::c_char;
    zdialcode = zbufalc(
        (strlen(zphone)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    zfrom = zphone;
    zto = zdialcode;
    while *zfrom as libc::c_int != '\0' as i32
        && *(*__ctype_b_loc()).offset(*zfrom as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        let fresh3 = zfrom;
        zfrom = zfrom.offset(1);
        let fresh4 = zto;
        zto = zto.offset(1);
        *fresh4 = *fresh3;
    }
    *zto = '\0' as i32 as libc::c_char;
    if *zdialcode as libc::c_int == '\0' as i32 {
        ubuffree(zdialcode);
        return 1 as libc::c_int;
    }
    iuuconf = uuconf_dialcode(puuconf, zdialcode, &mut ztrans);
    ubuffree(zdialcode);
    if iuuconf == 1 as libc::c_int {
        return 1 as libc::c_int
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        return 0 as libc::c_int;
    } else {
        *pzprefix = ztrans;
        *pzsuffix = zfrom;
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn fcecho_send_strip(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut cwrite: size_t,
) -> boolean {
    return fcecho_send(qconn, zwrite, cwrite, 1 as libc::c_int);
}
unsafe extern "C" fn fcecho_send_nostrip(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut cwrite: size_t,
) -> boolean {
    return fcecho_send(qconn, zwrite, cwrite, 0 as libc::c_int);
}
unsafe extern "C" fn fcecho_send(
    mut qconn: *mut sconnection,
    mut zwrite: *const libc::c_char,
    mut cwrite: size_t,
    mut fstrip: boolean,
) -> boolean {
    let mut zend: *const libc::c_char = 0 as *const libc::c_char;
    zend = zwrite.offset(cwrite as isize);
    while zwrite < zend {
        let mut b: libc::c_int = 0;
        let mut bwrite: libc::c_char = 0;
        bwrite = *zwrite;
        if fconn_write(qconn, &mut bwrite, 1 as libc::c_int as size_t) == 0 {
            return 0 as libc::c_int;
        }
        if fstrip != 0 {
            bwrite = (bwrite as libc::c_int & 0x7f as libc::c_int) as libc::c_char;
        }
        loop {
            b = breceive_char(qconn, 5 as libc::c_int, 1 as libc::c_int);
            if b < 0 as libc::c_int {
                if b == -(1 as libc::c_int) {
                    ulog(
                        LOG_ERROR,
                        b"Character not echoed\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if fstrip != 0 {
                b &= 0x7f as libc::c_int;
            }
            if !(b != bwrite as libc::c_uchar as libc::c_int) {
                break;
            }
        }
        zwrite = zwrite.offset(1);
        zwrite;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fcprogram(
    mut qconn: *mut sconnection,
    mut puuconf: pointer,
    mut pzprogram: *mut *mut libc::c_char,
    mut qsys: *const uuconf_system,
    mut qdial: *const uuconf_dialer,
    mut zphone: *const libc::c_char,
    mut zport: *const libc::c_char,
    mut ibaud: libc::c_long,
) -> boolean {
    let mut cargs: size_t = 0;
    let mut pzpass: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pzarg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut zcallout_login: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zcallout_pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    cargs = 1 as libc::c_int as size_t;
    pz = pzprogram;
    while !(*pz).is_null() {
        cargs = cargs.wrapping_add(1);
        cargs;
        pz = pz.offset(1);
        pz;
    }
    pzpass = xmalloc(
        cargs.wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    zcallout_login = 0 as *mut libc::c_char;
    zcallout_pass = 0 as *mut libc::c_char;
    fret = 1 as libc::c_int;
    pz = pzprogram;
    pzarg = pzpass;
    while !(*pz).is_null() {
        let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
        let mut calc: size_t = 0;
        let mut clen: size_t = 0;
        let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
        if (strchr(*pz, '\\' as i32)).is_null() {
            *pzarg = zbufcpy(*pz);
        } else {
            *pzarg = 0 as *mut libc::c_char;
            zto = 0 as *mut libc::c_char;
            calc = 0 as libc::c_int as size_t;
            clen = 0 as libc::c_int as size_t;
            zfrom = *pz;
            while *zfrom as libc::c_int != '\0' as i32 {
                let mut zadd: *const libc::c_char = 0 as *const libc::c_char;
                let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cadd: size_t = 0;
                let mut abadd: [libc::c_char; 15] = [0; 15];
                if *zfrom as libc::c_int != '\\' as i32 {
                    if clen.wrapping_add(2 as libc::c_int as libc::c_ulong) > calc {
                        let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
                        calc = clen.wrapping_add(50 as libc::c_int as libc::c_ulong);
                        znew = zbufalc(calc);
                        memcpy(
                            znew as *mut libc::c_void,
                            *pzarg as *const libc::c_void,
                            clen,
                        );
                        ubuffree(*pzarg);
                        *pzarg = znew;
                        zto = znew.offset(clen as isize);
                    }
                    let fresh5 = zto;
                    zto = zto.offset(1);
                    *fresh5 = *zfrom;
                    clen = clen.wrapping_add(1);
                    clen;
                } else {
                    zfrom = zfrom.offset(1);
                    zfrom;
                    let mut current_block_94: u64;
                    match *zfrom as libc::c_int {
                        0 => {
                            zfrom = zfrom.offset(-1);
                            zfrom;
                            current_block_94 = 13475044908890067863;
                        }
                        92 => {
                            current_block_94 = 13475044908890067863;
                        }
                        76 => {
                            let mut zlog: *const libc::c_char = 0 as *const libc::c_char;
                            if qsys.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\L\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else {
                                zlog = (*qsys).uuconf_zcall_login;
                                if zlog.is_null() {
                                    ulog(
                                        LOG_ERROR,
                                        b"chat-program: No login defined\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    fret = 0 as libc::c_int;
                                } else {
                                    if *zlog.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '*' as i32
                                        && *zlog.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '\0' as i32
                                    {
                                        if zcallout_login.is_null() {
                                            let mut iuuconf: libc::c_int = 0;
                                            iuuconf = uuconf_callout(
                                                puuconf,
                                                qsys,
                                                &mut zcallout_login,
                                                &mut zcallout_pass,
                                            );
                                            if iuuconf == 1 as libc::c_int || zcallout_login.is_null() {
                                                ulog(
                                                    LOG_ERROR,
                                                    b"chat-program: No login defined\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                fret = 0 as libc::c_int;
                                                current_block_94 = 13201766686570145889;
                                            } else if iuuconf != 0 as libc::c_int {
                                                ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                                                fret = 0 as libc::c_int;
                                                current_block_94 = 13201766686570145889;
                                            } else {
                                                current_block_94 = 6717214610478484138;
                                            }
                                        } else {
                                            current_block_94 = 6717214610478484138;
                                        }
                                        match current_block_94 {
                                            13201766686570145889 => {}
                                            _ => {
                                                zlog = zcallout_login;
                                                current_block_94 = 6174974146017752131;
                                            }
                                        }
                                    } else {
                                        current_block_94 = 6174974146017752131;
                                    }
                                    match current_block_94 {
                                        13201766686570145889 => {}
                                        _ => {
                                            zfree = zbufcpy(zlog);
                                            cescape(zfree);
                                            zadd = zfree;
                                        }
                                    }
                                }
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        80 => {
                            let mut zpass: *const libc::c_char = 0
                                as *const libc::c_char;
                            if qsys.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\P\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else {
                                zpass = (*qsys).uuconf_zcall_password;
                                if zpass.is_null() {
                                    ulog(
                                        LOG_ERROR,
                                        b"chat-program: No password defined\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    fret = 0 as libc::c_int;
                                } else {
                                    if *zpass.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '*' as i32
                                        && *zpass.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '\0' as i32
                                    {
                                        if zcallout_pass.is_null() {
                                            let mut iuuconf_0: libc::c_int = 0;
                                            iuuconf_0 = uuconf_callout(
                                                puuconf,
                                                qsys,
                                                &mut zcallout_login,
                                                &mut zcallout_pass,
                                            );
                                            if iuuconf_0 == 1 as libc::c_int || zcallout_pass.is_null()
                                            {
                                                ulog(
                                                    LOG_ERROR,
                                                    b"chat-program: No password defined\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                fret = 0 as libc::c_int;
                                                current_block_94 = 13201766686570145889;
                                            } else if iuuconf_0 != 0 as libc::c_int {
                                                ulog_uuconf(LOG_ERROR, puuconf, iuuconf_0);
                                                fret = 0 as libc::c_int;
                                                current_block_94 = 13201766686570145889;
                                            } else {
                                                current_block_94 = 1134115459065347084;
                                            }
                                        } else {
                                            current_block_94 = 1134115459065347084;
                                        }
                                        match current_block_94 {
                                            13201766686570145889 => {}
                                            _ => {
                                                zpass = zcallout_pass;
                                                current_block_94 = 5372832139739605200;
                                            }
                                        }
                                    } else {
                                        current_block_94 = 5372832139739605200;
                                    }
                                    match current_block_94 {
                                        13201766686570145889 => {}
                                        _ => {
                                            zfree = zbufcpy(zpass);
                                            cescape(zfree);
                                            zadd = zfree;
                                        }
                                    }
                                }
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        68 => {
                            if qdial.is_null() || zphone.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\D\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else {
                                zadd = zphone;
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        84 => {
                            let mut zprefix: *const libc::c_char = 0
                                as *const libc::c_char;
                            let mut zsuffix: *const libc::c_char = 0
                                as *const libc::c_char;
                            if qdial.is_null() || zphone.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\T\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else if fctranslate(
                                puuconf,
                                zphone,
                                &mut zprefix,
                                &mut zsuffix,
                            ) == 0
                            {
                                fret = 0 as libc::c_int;
                            } else if zsuffix.is_null() {
                                zadd = zprefix;
                            } else {
                                let mut cprefix: size_t = 0;
                                cprefix = strlen(zprefix);
                                if clen
                                    .wrapping_add(cprefix)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) > calc
                                {
                                    let mut znew_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                    calc = clen
                                        .wrapping_add(cprefix)
                                        .wrapping_add(20 as libc::c_int as libc::c_ulong);
                                    znew_0 = zbufalc(calc);
                                    memcpy(
                                        znew_0 as *mut libc::c_void,
                                        *pzarg as *const libc::c_void,
                                        clen,
                                    );
                                    ubuffree(*pzarg);
                                    *pzarg = znew_0;
                                    zto = znew_0.offset(clen as isize);
                                }
                                memcpy(
                                    zto as *mut libc::c_void,
                                    zprefix as *const libc::c_void,
                                    cprefix,
                                );
                                zto = zto.offset(cprefix as isize);
                                clen = (clen as libc::c_ulong).wrapping_add(cprefix)
                                    as size_t as size_t;
                                zadd = zsuffix;
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        89 => {
                            if zLdevice.is_null() && zport.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\Y\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else {
                                zadd = zLdevice;
                                if zadd.is_null() {
                                    zadd = zport;
                                }
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        90 => {
                            if qsys.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\Z\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else {
                                zadd = (*qsys).uuconf_zname;
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        83 => {
                            if ibaud == 0 as libc::c_int as libc::c_long {
                                ulog(
                                    LOG_ERROR,
                                    b"chat-program: Illegal use of \\S\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fret = 0 as libc::c_int;
                            } else {
                                sprintf(
                                    abadd.as_mut_ptr(),
                                    b"%ld\0" as *const u8 as *const libc::c_char,
                                    ibaud,
                                );
                                zadd = abadd.as_mut_ptr();
                            }
                            current_block_94 = 13201766686570145889;
                        }
                        _ => {
                            ulog(
                                LOG_ERROR,
                                b"chat-program: Unrecognized escape sequence \\%c\0"
                                    as *const u8 as *const libc::c_char,
                                *zfrom as libc::c_int,
                            );
                            abadd[0 as libc::c_int as usize] = *zfrom;
                            abadd[1 as libc::c_int
                                as usize] = '\0' as i32 as libc::c_char;
                            zadd = abadd.as_mut_ptr();
                            current_block_94 = 13201766686570145889;
                        }
                    }
                    match current_block_94 {
                        13475044908890067863 => {
                            zadd = b"\\\0" as *const u8 as *const libc::c_char;
                        }
                        _ => {}
                    }
                    if fret == 0 {
                        break;
                    }
                    cadd = strlen(zadd);
                    if clen
                        .wrapping_add(cadd)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) > calc
                    {
                        let mut znew_1: *mut libc::c_char = 0 as *mut libc::c_char;
                        calc = clen
                            .wrapping_add(cadd)
                            .wrapping_add(20 as libc::c_int as libc::c_ulong);
                        znew_1 = zbufalc(calc);
                        memcpy(
                            znew_1 as *mut libc::c_void,
                            *pzarg as *const libc::c_void,
                            clen,
                        );
                        ubuffree(*pzarg);
                        *pzarg = znew_1;
                        zto = znew_1.offset(clen as isize);
                    }
                    memcpy(
                        zto as *mut libc::c_void,
                        zadd as *const libc::c_void,
                        cadd.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    zto = zto.offset(cadd as isize);
                    clen = (clen as libc::c_ulong).wrapping_add(cadd) as size_t
                        as size_t;
                    ubuffree(zfree);
                }
                zfrom = zfrom.offset(1);
                zfrom;
            }
            if fret == 0 {
                break;
            }
            let fresh6 = zto;
            zto = zto.offset(1);
            *fresh6 = '\0' as i32 as libc::c_char;
            clen = clen.wrapping_add(1);
            clen;
        }
        pz = pz.offset(1);
        pz;
        pzarg = pzarg.offset(1);
        pzarg;
    }
    *pzarg = 0 as *mut libc::c_char;
    if fret != 0 {
        fret = fconn_run_chat(qconn, pzpass);
    }
    pz = pzpass;
    while !(*pz).is_null() {
        ubuffree(*pz);
        pz = pz.offset(1);
        pz;
    }
    xfree(pzpass as pointer);
    xfree(zcallout_login as pointer);
    xfree(zcallout_pass as pointer);
    return fret;
}

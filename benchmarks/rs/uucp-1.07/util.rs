use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xfree(_: pointer);
    fn uuconf_system_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzsystems: *mut *mut *mut libc::c_char,
        uuconf_falias: libc::c_int,
    ) -> libc::c_int;
    fn uuconf_system_unknown(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    static mut cSysdep_max_name_len: size_t;
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_in_directory(
        zfile: *const libc::c_char,
        zdir: *const libc::c_char,
        fcheck: boolean,
        freadable: boolean,
        zuser: *const libc::c_char,
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
pub static mut util_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: util.c,v 1.11 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn funknown_system(
    mut puuconf: pointer,
    mut zsystem: *const libc::c_char,
    mut qsys: *mut uuconf_system,
) -> boolean {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iuuconf: libc::c_int = 0;
    if strlen(zsystem) <= cSysdep_max_name_len {
        z = zbufcpy(zsystem);
    } else {
        let mut pznames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut ffound: boolean = 0;
        z = zbufalc(
            cSysdep_max_name_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            z as *mut libc::c_void,
            zsystem as *const libc::c_void,
            cSysdep_max_name_len,
        );
        *z.offset(cSysdep_max_name_len as isize) = '\0' as i32 as libc::c_char;
        iuuconf = uuconf_system_names(puuconf, &mut pznames, 1 as libc::c_int);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        ffound = 0 as libc::c_int;
        pz = pznames;
        while !(*pz).is_null() {
            if strcmp(*pz, z) == 0 as libc::c_int {
                ffound = 1 as libc::c_int;
            }
            xfree(*pz as pointer);
            pz = pz.offset(1);
            pz;
        }
        xfree(pznames as pointer);
        if ffound != 0 {
            ubuffree(z);
            return 0 as libc::c_int;
        }
    }
    iuuconf = uuconf_system_unknown(puuconf, qsys);
    if iuuconf == 1 as libc::c_int {
        ubuffree(z);
        return 0 as libc::c_int;
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    while !qsys.is_null() {
        (*qsys).uuconf_zname = z;
        qsys = (*qsys).uuconf_qalternate;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zremove_local_sys(
    mut qlocalsys: *mut uuconf_system,
    mut z: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut clen: size_t = 0;
    let mut zexclam: *mut libc::c_char = 0 as *mut libc::c_char;
    clen = strlen((*qlocalsys).uuconf_zname);
    zexclam = strchr(z, '!' as i32);
    while !zexclam.is_null() {
        if !(z == zexclam
            || zexclam.offset_from(z) as libc::c_long as size_t == clen
                && strncmp(z, (*qlocalsys).uuconf_zname, clen) == 0 as libc::c_int)
        {
            if ((*qlocalsys).uuconf_pzalias).is_null() {
                break;
            }
            let mut pzal: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            pzal = (*qlocalsys).uuconf_pzalias;
            while !(*pzal).is_null() {
                if strlen(*pzal) == zexclam.offset_from(z) as libc::c_long as size_t
                    && strncmp(
                        z,
                        *pzal,
                        zexclam.offset_from(z) as libc::c_long as size_t,
                    ) == 0 as libc::c_int
                {
                    break;
                }
                pzal = pzal.offset(1);
                pzal;
            }
            if (*pzal).is_null() {
                break;
            }
        }
        z = zexclam.offset(1 as libc::c_int as isize);
        zexclam = strchr(z, '!' as i32);
    }
    return z;
}
pub unsafe extern "C" fn fin_directory_list(
    mut zfile: *const libc::c_char,
    mut pzdirs: *mut *mut libc::c_char,
    mut zpubdir: *const libc::c_char,
    mut fcheck: boolean,
    mut freadable: boolean,
    mut zuser: *const libc::c_char,
) -> boolean {
    let mut fmatch: boolean = 0;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    fmatch = 0 as libc::c_int;
    pz = pzdirs;
    while !(*pz).is_null() {
        let mut zuse: *mut libc::c_char = 0 as *mut libc::c_char;
        if *(*pz.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '!' as i32
        {
            zuse = zsysdep_local_file(
                (*pz).offset(1 as libc::c_int as isize),
                zpubdir,
                0 as *mut libc::c_void as *mut boolean,
            );
            if zuse.is_null() {
                return 0 as libc::c_int;
            }
            if fsysdep_in_directory(
                zfile,
                zuse,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void as *const libc::c_char,
            ) != 0
            {
                fmatch = 0 as libc::c_int;
            }
        } else {
            zuse = zsysdep_local_file(
                *pz,
                zpubdir,
                0 as *mut libc::c_void as *mut boolean,
            );
            if zuse.is_null() {
                return 0 as libc::c_int;
            }
            if fsysdep_in_directory(zfile, zuse, fcheck, freadable, zuser) != 0 {
                fmatch = 1 as libc::c_int;
            }
        }
        ubuffree(zuse);
        pz = pz.offset(1);
        pz;
    }
    return fmatch;
}

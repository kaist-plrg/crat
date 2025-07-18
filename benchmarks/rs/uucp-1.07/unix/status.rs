use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut azStatus: [*const libc::c_char; 0];
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xfree(_: pointer);
    fn esysdep_fopen(
        zfile: *const libc::c_char,
        fpublic: boolean,
        fappend: boolean,
        fmkdirs: boolean,
    ) -> *mut FILE;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type tstatus_type = libc::c_uint;
pub const STATUS_VALUES: tstatus_type = 8;
pub const STATUS_WRONG_TIME: tstatus_type = 7;
pub const STATUS_TALKING: tstatus_type = 6;
pub const STATUS_FAILED: tstatus_type = 5;
pub const STATUS_HANDSHAKE_FAILED: tstatus_type = 4;
pub const STATUS_LOGIN_FAILED: tstatus_type = 3;
pub const STATUS_DIAL_FAILED: tstatus_type = 2;
pub const STATUS_PORT_FAILED: tstatus_type = 1;
pub const STATUS_COMPLETE: tstatus_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sstatus {
    pub ttype: tstatus_type,
    pub cretries: libc::c_int,
    pub ilast: libc::c_long,
    pub cwait: libc::c_int,
    pub zstring: *mut libc::c_char,
}
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub unsafe extern "C" fn fsysdep_get_status(
    mut qsys: *const uuconf_system,
    mut qret: *mut sstatus,
    mut pfnone: *mut boolean,
) -> boolean {
    let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut znext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fbad: boolean = 0;
    let mut istat: libc::c_int = 0;
    if !pfnone.is_null() {
        *pfnone = 0 as libc::c_int;
    }
    zname = zsysdep_in_dir(
        b".Status\0" as *const u8 as *const libc::c_char,
        (*qsys).uuconf_zname,
    );
    e = fopen(zname, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        if *__errno_location() != 2 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
                zname,
                strerror(*__errno_location()),
            );
            ubuffree(zname);
            return 0 as libc::c_int;
        }
        zline = 0 as *mut libc::c_char;
    } else {
        let mut cline: size_t = 0;
        zline = 0 as *mut libc::c_char;
        cline = 0 as libc::c_int as size_t;
        if getline(&mut zline, &mut cline, e) <= 0 as libc::c_int as libc::c_long {
            xfree(zline as pointer);
            zline = 0 as *mut libc::c_char;
        }
        fclose(e);
    }
    if zline.is_null() {
        (*qret).ttype = STATUS_COMPLETE;
        (*qret).cretries = 0 as libc::c_int;
        (*qret).ilast = 0 as libc::c_int as libc::c_long;
        (*qret).cwait = 0 as libc::c_int;
        (*qret).zstring = 0 as *mut libc::c_char;
        if !pfnone.is_null() {
            *pfnone = 1 as libc::c_int;
        }
        ubuffree(zname);
        return 1 as libc::c_int;
    }
    fbad = 0 as libc::c_int;
    istat = strtol(zline, &mut zend, 10 as libc::c_int) as libc::c_int;
    if zend == zline {
        fbad = 1 as libc::c_int;
    }
    if istat < 0 as libc::c_int || istat >= STATUS_VALUES as libc::c_int {
        istat = STATUS_COMPLETE as libc::c_int;
    }
    (*qret).ttype = istat as tstatus_type;
    znext = zend;
    (*qret).cretries = strtol(znext, &mut zend, 10 as libc::c_int) as libc::c_int;
    if zend == znext {
        fbad = 1 as libc::c_int;
    }
    znext = zend;
    (*qret).ilast = strtol(znext, &mut zend, 10 as libc::c_int);
    if zend == znext {
        fbad = 1 as libc::c_int;
    }
    znext = zend;
    (*qret).cwait = strtol(znext, &mut zend, 10 as libc::c_int) as libc::c_int;
    if zend == znext {
        fbad = 1 as libc::c_int;
    }
    if fbad == 0 {
        znext = zend;
        while *(*__ctype_b_loc()).offset(*znext as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            znext = znext.offset(1);
            znext;
        }
        if *znext as libc::c_int == '\0' as i32 {
            (*qret).zstring = 0 as *mut libc::c_char;
        } else {
            if *znext as libc::c_int == '"' as i32 {
                znext = znext.offset(1);
                znext;
            }
            (*qret).zstring = zbufcpy(znext);
            zend = ((*qret).zstring).offset(strlen((*qret).zstring) as isize);
            while zend != (*qret).zstring && *zend as libc::c_int != ' ' as i32 {
                zend = zend.offset(-1);
                zend;
            }
            if zend != (*qret).zstring
                && *zend.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '"' as i32
            {
                zend = zend.offset(-1);
                zend;
            }
            if zend != (*qret).zstring {
                *zend = '\0' as i32 as libc::c_char;
            } else {
                ubuffree((*qret).zstring);
                (*qret).zstring = 0 as *mut libc::c_char;
            }
        }
    }
    xfree(zline as pointer);
    if fbad != 0 {
        ulog(
            LOG_ERROR,
            b"%s: Bad status file format\0" as *const u8 as *const libc::c_char,
            zname,
        );
        ubuffree(zname);
        return 0 as libc::c_int;
    }
    ubuffree(zname);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_set_status(
    mut qsys: *const uuconf_system,
    mut qset: *const sstatus,
) -> boolean {
    let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut istat: libc::c_int = 0;
    zname = zsysdep_in_dir(
        b".Status\0" as *const u8 as *const libc::c_char,
        (*qsys).uuconf_zname,
    );
    e = esysdep_fopen(zname, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    ubuffree(zname);
    if e.is_null() {
        return 0 as libc::c_int;
    }
    istat = (*qset).ttype as libc::c_int;
    fprintf(
        e,
        b"%d %d %ld %d \0" as *const u8 as *const libc::c_char,
        istat,
        (*qset).cretries,
        (*qset).ilast,
        (*qset).cwait,
    );
    fprintf(
        e,
        b"%s\0" as *const u8 as *const libc::c_char,
        *azStatus.as_mut_ptr().offset((*qset).ttype as libc::c_int as isize),
    );
    fprintf(e, b" %s\n\0" as *const u8 as *const libc::c_char, (*qsys).uuconf_zname);
    if fclose(e) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fclose: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

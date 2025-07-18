use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fcmd_needs_quotes(qcmd: *const scmd) -> boolean;
    fn uquote_cmd(qorig: *const scmd, qnew: *mut scmd);
    fn ufree_quoted_cmd(qcmd: *mut scmd);
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ubuffree(z: *mut libc::c_char);
    fn zsfind_file(
        zsimple: *const libc::c_char,
        zsystem: *const libc::c_char,
        bgrade: libc::c_int,
    ) -> *mut libc::c_char;
    fn zscmd_file(qsys: *const uuconf_system, bgrade: libc::c_int) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn zsfile_to_jobid(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        bgrade: libc::c_int,
    ) -> *mut libc::c_char;
    fn esysdep_fopen(
        zfile: *const libc::c_char,
        fpublic: boolean,
        fappend: boolean,
        fmkdirs: boolean,
    ) -> *mut FILE;
    fn fsysdep_sync(e: openfile_t, zmsg: *const libc::c_char) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type openfile_t = *mut FILE;
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
pub struct scmd {
    pub bcmd: libc::c_char,
    pub bgrade: libc::c_char,
    pub pseq: pointer,
    pub zfrom: *const libc::c_char,
    pub zto: *const libc::c_char,
    pub zuser: *const libc::c_char,
    pub zoptions: *const libc::c_char,
    pub ztemp: *const libc::c_char,
    pub imode: libc::c_uint,
    pub znotify: *const libc::c_char,
    pub cbytes: libc::c_long,
    pub zcmd: *const libc::c_char,
    pub ipos: libc::c_long,
}
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub unsafe extern "C" fn zsysdep_spool_commands(
    mut qsys: *const uuconf_system,
    mut bgrade: libc::c_int,
    mut ccmds: libc::c_int,
    mut pascmds: *const scmd,
    mut pftemp: *mut boolean,
) -> *mut libc::c_char {
    let mut abtempfile: [libc::c_char; 14] = [0; 14];
    let mut ztemp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut qcmd: *const scmd = 0 as *const scmd;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zjobid: *mut libc::c_char = 0 as *mut libc::c_char;
    if !pftemp.is_null() {
        *pftemp = 1 as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(bgrade as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        ulog(LOG_FATAL, b"Bad grade %d\0" as *const u8 as *const libc::c_char, bgrade);
    }
    sprintf(
        abtempfile.as_mut_ptr(),
        b"TMP%010lx\0" as *const u8 as *const libc::c_char,
        getpid() as libc::c_ulong,
    );
    ztemp = zsfind_file(abtempfile.as_mut_ptr(), (*qsys).uuconf_zname, bgrade);
    if ztemp.is_null() {
        return 0 as *mut libc::c_char;
    }
    e = esysdep_fopen(ztemp, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    if e.is_null() {
        ubuffree(ztemp);
        return 0 as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    qcmd = pascmds;
    while i < ccmds {
        let mut fquote: boolean = 0;
        let mut q: *const scmd = 0 as *const scmd;
        let mut squoted: scmd = scmd {
            bcmd: 0,
            bgrade: 0,
            pseq: 0 as *mut libc::c_void,
            zfrom: 0 as *const libc::c_char,
            zto: 0 as *const libc::c_char,
            zuser: 0 as *const libc::c_char,
            zoptions: 0 as *const libc::c_char,
            ztemp: 0 as *const libc::c_char,
            imode: 0,
            znotify: 0 as *const libc::c_char,
            cbytes: 0,
            zcmd: 0 as *const libc::c_char,
            ipos: 0,
        };
        fquote = fcmd_needs_quotes(qcmd);
        if fquote == 0 {
            q = qcmd;
        } else {
            uquote_cmd(qcmd, &mut squoted);
            q = &mut squoted;
        }
        match (*q).bcmd as libc::c_int {
            83 => {
                fprintf(
                    e,
                    b"S %s %s %s -%s %s 0%o %s\n\0" as *const u8 as *const libc::c_char,
                    (*q).zfrom,
                    (*q).zto,
                    (*q).zuser,
                    (*q).zoptions,
                    (*q).ztemp,
                    (*q).imode,
                    if ((*q).znotify).is_null() {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        (*q).znotify
                    },
                );
            }
            82 => {
                fprintf(
                    e,
                    b"R %s %s %s -%s\n\0" as *const u8 as *const libc::c_char,
                    (*q).zfrom,
                    (*q).zto,
                    (*q).zuser,
                    (*q).zoptions,
                );
            }
            88 => {
                fprintf(
                    e,
                    b"X %s %s %s -%s\n\0" as *const u8 as *const libc::c_char,
                    (*q).zfrom,
                    (*q).zto,
                    (*q).zuser,
                    (*q).zoptions,
                );
            }
            69 => {
                fprintf(
                    e,
                    b"E %s %s %s -%s %s 0%o %s 0 %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*q).zfrom,
                    (*q).zto,
                    (*q).zuser,
                    (*q).zoptions,
                    (*q).ztemp,
                    (*q).imode,
                    (*q).znotify,
                    (*q).zcmd,
                );
            }
            _ => {
                ulog(
                    LOG_ERROR,
                    b"zsysdep_spool_commands: Unrecognized type %d\0" as *const u8
                        as *const libc::c_char,
                    (*q).bcmd as libc::c_int,
                );
                fclose(e);
                remove(ztemp);
                ubuffree(ztemp);
                if !pftemp.is_null() {
                    *pftemp = 0 as libc::c_int;
                }
                return 0 as *mut libc::c_char;
            }
        }
        if fquote != 0 {
            ufree_quoted_cmd(&mut squoted);
        }
        i += 1;
        i;
        qcmd = qcmd.offset(1);
        qcmd;
    }
    if fsysdep_sync(e, ztemp) == 0 {
        fclose(e);
        remove(ztemp);
        ubuffree(ztemp);
        return 0 as *mut libc::c_char;
    }
    if fclose(e) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fclose: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        remove(ztemp);
        ubuffree(ztemp);
        return 0 as *mut libc::c_char;
    }
    loop {
        z = zscmd_file(qsys, bgrade);
        if z.is_null() {
            remove(ztemp);
            ubuffree(ztemp);
            return 0 as *mut libc::c_char;
        }
        if link(ztemp, z) >= 0 as libc::c_int {
            break;
        }
        if *__errno_location() != 17 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"link (%s, %s): %s\0" as *const u8 as *const libc::c_char,
                ztemp,
                z,
                strerror(*__errno_location()),
            );
            remove(ztemp);
            ubuffree(ztemp);
            ubuffree(z);
            return 0 as *mut libc::c_char;
        }
        ubuffree(z);
    }
    remove(ztemp);
    ubuffree(ztemp);
    zjobid = zsfile_to_jobid(qsys, z, bgrade);
    if zjobid.is_null() {
        remove(z);
    }
    ubuffree(z);
    return zjobid;
}

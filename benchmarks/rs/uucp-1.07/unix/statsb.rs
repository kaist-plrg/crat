use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
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
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn funknown_system(
        puuconf: pointer,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> boolean;
    fn fparse_cmd(zcmd: *mut libc::c_char, qcmd: *mut scmd) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    fn xfree(_: pointer);
    fn uuconf_system_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zsystem: *const libc::c_char,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    static mut zSlockdir: *const libc::c_char;
    fn zsjobid_to_file(
        zid: *const libc::c_char,
        pzsystem: *mut *mut libc::c_char,
        pbgrade: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn zsfind_file(
        zsimple: *const libc::c_char,
        zsystem: *const libc::c_char,
        bgrade: libc::c_int,
    ) -> *mut libc::c_char;
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
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
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn zsysdep_login_name() -> *const libc::c_char;
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn fsysdep_get_status(
        qsys: *const uuconf_system,
        qret: *mut sstatus,
        pfnone: *mut boolean,
    ) -> boolean;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn zsysdep_jobid(qsys: *const uuconf_system, pseq: pointer) -> *mut libc::c_char;
    fn fsysdep_privileged() -> boolean;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn time(__timer: *mut time_t) -> time_t;
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pointer = *mut libc::c_void;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub static mut statsb_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: statsb.c,v 1.20 2002/03/05 19:10:42 ian Rel $\0")
};
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn issettime(
    mut z: *const libc::c_char,
    mut inow: time_t,
) -> libc::c_int {
    let mut s: utimbuf = utimbuf { actime: 0, modtime: 0 };
    s.actime = inow;
    s.modtime = inow;
    return utime(z as *mut libc::c_char, &mut s);
}
pub unsafe extern "C" fn fsysdep_kill_job(
    mut puuconf: pointer,
    mut zid: *const libc::c_char,
) -> boolean {
    return fskill_or_rejuv(puuconf, zid, 1 as libc::c_int);
}
pub unsafe extern "C" fn fsysdep_rejuvenate_job(
    mut puuconf: pointer,
    mut zid: *const libc::c_char,
) -> boolean {
    return fskill_or_rejuv(puuconf, zid, 0 as libc::c_int);
}
unsafe extern "C" fn fskill_or_rejuv(
    mut puuconf: pointer,
    mut zid: *const libc::c_char,
    mut fkill: boolean,
) -> boolean {
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zsys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bgrade: libc::c_char = 0;
    let mut inow: time_t = 0 as libc::c_int as time_t;
    let mut iuuconf: libc::c_int = 0;
    let mut ssys: uuconf_system = uuconf_system {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_pzalias: 0 as *mut *mut libc::c_char,
        uuconf_qalternate: 0 as *mut uuconf_system,
        uuconf_zalternate: 0 as *mut libc::c_char,
        uuconf_fcall: 0,
        uuconf_fcalled: 0,
        uuconf_qtimegrade: 0 as *mut uuconf_timespan,
        uuconf_qcalltimegrade: 0 as *mut uuconf_timespan,
        uuconf_qcalledtimegrade: 0 as *mut uuconf_timespan,
        uuconf_cmax_retries: 0,
        uuconf_csuccess_wait: 0,
        uuconf_qcall_local_size: 0 as *mut uuconf_timespan,
        uuconf_qcall_remote_size: 0 as *mut uuconf_timespan,
        uuconf_qcalled_local_size: 0 as *mut uuconf_timespan,
        uuconf_qcalled_remote_size: 0 as *mut uuconf_timespan,
        uuconf_ibaud: 0,
        uuconf_ihighbaud: 0,
        uuconf_zport: 0 as *mut libc::c_char,
        uuconf_qport: 0 as *mut uuconf_port,
        uuconf_zphone: 0 as *mut libc::c_char,
        uuconf_schat: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zcall_login: 0 as *mut libc::c_char,
        uuconf_zcall_password: 0 as *mut libc::c_char,
        uuconf_zcalled_login: 0 as *mut libc::c_char,
        uuconf_fcallback: 0,
        uuconf_fsequence: 0,
        uuconf_zprotocols: 0 as *mut libc::c_char,
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_scalled_chat: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zdebug: 0 as *mut libc::c_char,
        uuconf_zmax_remote_debug: 0 as *mut libc::c_char,
        uuconf_fsend_request: 0,
        uuconf_frec_request: 0,
        uuconf_fcall_transfer: 0,
        uuconf_fcalled_transfer: 0,
        uuconf_pzlocal_send: 0 as *mut *mut libc::c_char,
        uuconf_pzremote_send: 0 as *mut *mut libc::c_char,
        uuconf_pzlocal_receive: 0 as *mut *mut libc::c_char,
        uuconf_pzremote_receive: 0 as *mut *mut libc::c_char,
        uuconf_pzpath: 0 as *mut *mut libc::c_char,
        uuconf_pzcmds: 0 as *mut *mut libc::c_char,
        uuconf_cfree_space: 0,
        uuconf_pzforward_from: 0 as *mut *mut libc::c_char,
        uuconf_pzforward_to: 0 as *mut *mut libc::c_char,
        uuconf_zpubdir: 0 as *const libc::c_char,
        uuconf_zlocalname: 0 as *mut libc::c_char,
        uuconf_cmax_file_time: 0,
        uuconf_palloc: 0 as *mut libc::c_void,
    };
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut fret: boolean = 0;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut isys: libc::c_int = 0;
    zfile = zsjobid_to_file(zid, &mut zsys, &mut bgrade);
    if zfile.is_null() {
        return 0 as libc::c_int;
    }
    if fkill == 0 {
        inow = time(0 as *mut libc::c_void as *mut time_t);
    }
    iuuconf = uuconf_system_info(puuconf, zsys, &mut ssys);
    if iuuconf == 1 as libc::c_int {
        if funknown_system(puuconf, zsys, &mut ssys) == 0 {
            ulog(
                LOG_ERROR,
                b"%s: Bad job id\0" as *const u8 as *const libc::c_char,
                zid,
            );
            ubuffree(zfile);
            ubuffree(zsys);
            return 0 as libc::c_int;
        }
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        ubuffree(zfile);
        ubuffree(zsys);
        return 0 as libc::c_int;
    }
    e = fopen(zfile, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"%s: Job not found\0" as *const u8 as *const libc::c_char,
                zid,
            );
        } else {
            ulog(
                LOG_ERROR,
                b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
                zfile,
                strerror(*__errno_location()),
            );
        }
        uuconf_free_block(ssys.uuconf_palloc);
        ubuffree(zfile);
        ubuffree(zsys);
        return 0 as libc::c_int;
    }
    fret = 1 as libc::c_int;
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    while getline(&mut zline, &mut cline, e) > 0 as libc::c_int as libc::c_long {
        let mut s: scmd = scmd {
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
        if fparse_cmd(zline, &mut s) == 0 {
            ulog(
                LOG_ERROR,
                b"Bad line in command file %s\0" as *const u8 as *const libc::c_char,
                zfile,
            );
            fret = 0 as libc::c_int;
        } else {
            if strcmp(s.zuser, zsysdep_login_name()) != 0 as libc::c_int
                && fsysdep_privileged() == 0
            {
                ulog(
                    LOG_ERROR,
                    b"%s: Not submitted by you\0" as *const u8 as *const libc::c_char,
                    zid,
                );
                xfree(zline as pointer);
                fclose(e);
                uuconf_free_block(ssys.uuconf_palloc);
                ubuffree(zfile);
                ubuffree(zsys);
                return 0 as libc::c_int;
            }
            if s.bcmd as libc::c_int == 'S' as i32 || s.bcmd as libc::c_int == 'E' as i32
            {
                let mut ztemp: *mut libc::c_char = 0 as *mut libc::c_char;
                ztemp = zsfind_file(s.ztemp, ssys.uuconf_zname, bgrade as libc::c_int);
                if ztemp.is_null() {
                    fret = 0 as libc::c_int;
                } else {
                    if fkill != 0 {
                        isys = remove(ztemp);
                    } else {
                        isys = issettime(ztemp, inow);
                    }
                    if isys != 0 as libc::c_int
                        && *__errno_location() != 2 as libc::c_int
                    {
                        ulog(
                            LOG_ERROR,
                            b"%s (%s): %s\0" as *const u8 as *const libc::c_char,
                            if fkill != 0 {
                                b"remove\0" as *const u8 as *const libc::c_char
                            } else {
                                b"utime\0" as *const u8 as *const libc::c_char
                            },
                            ztemp,
                            strerror(*__errno_location()),
                        );
                        fret = 0 as libc::c_int;
                    }
                    ubuffree(ztemp);
                }
            }
        }
    }
    xfree(zline as pointer);
    fclose(e);
    uuconf_free_block(ssys.uuconf_palloc);
    ubuffree(zsys);
    if fkill != 0 {
        isys = remove(zfile);
    } else {
        isys = issettime(zfile, inow);
    }
    if isys != 0 as libc::c_int && *__errno_location() != 2 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"%s (%s): %s\0" as *const u8 as *const libc::c_char,
            if fkill != 0 {
                b"remove\0" as *const u8 as *const libc::c_char
            } else {
                b"utime\0" as *const u8 as *const libc::c_char
            },
            zfile,
            strerror(*__errno_location()),
        );
        fret = 0 as libc::c_int;
    }
    ubuffree(zfile);
    return fret;
}
pub unsafe extern "C" fn ixsysdep_work_time(
    mut qsys: *const uuconf_system,
    mut pseq: pointer,
) -> libc::c_long {
    let mut zjobid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iret: libc::c_long = 0;
    zjobid = zsysdep_jobid(qsys, pseq);
    zfile = zsjobid_to_file(
        zjobid,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if zfile.is_null() {
        return 0 as libc::c_int as libc::c_long;
    }
    ubuffree(zjobid);
    iret = ixsysdep_file_time(zfile);
    ubuffree(zfile);
    return iret;
}
pub unsafe extern "C" fn ixsysdep_file_time(
    mut zfile: *const libc::c_char,
) -> libc::c_long {
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(zfile as *mut libc::c_char, &mut s) < 0 as libc::c_int {
        if *__errno_location() != 2 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"stat (%s): %s\0" as *const u8 as *const libc::c_char,
                zfile,
                strerror(*__errno_location()),
            );
        }
        return ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    }
    return s.st_mtim.tv_sec;
}
pub unsafe extern "C" fn fsysdep_touch_file(mut zfile: *const libc::c_char) -> boolean {
    if issettime(zfile, time(0 as *mut libc::c_void as *mut time_t)) != 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"utime (%s): %s\0" as *const u8 as *const libc::c_char,
            zfile,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_all_status_init(mut phold: *mut pointer) -> boolean {
    let mut qdir: *mut DIR = 0 as *mut DIR;
    qdir = opendir(
        b".Status\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if qdir.is_null() {
        ulog(
            LOG_ERROR,
            b"opendir (.Status): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    *phold = qdir as pointer;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_all_status(
    mut phold: pointer,
    mut pferr: *mut boolean,
    mut qstat: *mut sstatus,
) -> *mut libc::c_char {
    let mut qdir: *mut DIR = phold as *mut DIR;
    let mut qentry: *mut dirent = 0 as *mut dirent;
    loop {
        *__errno_location() = 0 as libc::c_int;
        qentry = readdir(qdir);
        if qentry.is_null() {
            if *__errno_location() == 0 as libc::c_int {
                *pferr = 0 as libc::c_int;
            } else {
                ulog(
                    LOG_ERROR,
                    b"readdir: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                *pferr = 1 as libc::c_int;
            }
            return 0 as *mut libc::c_char;
        }
        if (*qentry).d_name[0 as libc::c_int as usize] as libc::c_int != '.' as i32 {
            let mut ssys: uuconf_system = uuconf_system {
                uuconf_zname: 0 as *mut libc::c_char,
                uuconf_pzalias: 0 as *mut *mut libc::c_char,
                uuconf_qalternate: 0 as *mut uuconf_system,
                uuconf_zalternate: 0 as *mut libc::c_char,
                uuconf_fcall: 0,
                uuconf_fcalled: 0,
                uuconf_qtimegrade: 0 as *mut uuconf_timespan,
                uuconf_qcalltimegrade: 0 as *mut uuconf_timespan,
                uuconf_qcalledtimegrade: 0 as *mut uuconf_timespan,
                uuconf_cmax_retries: 0,
                uuconf_csuccess_wait: 0,
                uuconf_qcall_local_size: 0 as *mut uuconf_timespan,
                uuconf_qcall_remote_size: 0 as *mut uuconf_timespan,
                uuconf_qcalled_local_size: 0 as *mut uuconf_timespan,
                uuconf_qcalled_remote_size: 0 as *mut uuconf_timespan,
                uuconf_ibaud: 0,
                uuconf_ihighbaud: 0,
                uuconf_zport: 0 as *mut libc::c_char,
                uuconf_qport: 0 as *mut uuconf_port,
                uuconf_zphone: 0 as *mut libc::c_char,
                uuconf_schat: uuconf_chat {
                    uuconf_pzchat: 0 as *mut *mut libc::c_char,
                    uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                    uuconf_ctimeout: 0,
                    uuconf_pzfail: 0 as *mut *mut libc::c_char,
                    uuconf_fstrip: 0,
                },
                uuconf_zcall_login: 0 as *mut libc::c_char,
                uuconf_zcall_password: 0 as *mut libc::c_char,
                uuconf_zcalled_login: 0 as *mut libc::c_char,
                uuconf_fcallback: 0,
                uuconf_fsequence: 0,
                uuconf_zprotocols: 0 as *mut libc::c_char,
                uuconf_qproto_params: 0 as *mut uuconf_proto_param,
                uuconf_scalled_chat: uuconf_chat {
                    uuconf_pzchat: 0 as *mut *mut libc::c_char,
                    uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                    uuconf_ctimeout: 0,
                    uuconf_pzfail: 0 as *mut *mut libc::c_char,
                    uuconf_fstrip: 0,
                },
                uuconf_zdebug: 0 as *mut libc::c_char,
                uuconf_zmax_remote_debug: 0 as *mut libc::c_char,
                uuconf_fsend_request: 0,
                uuconf_frec_request: 0,
                uuconf_fcall_transfer: 0,
                uuconf_fcalled_transfer: 0,
                uuconf_pzlocal_send: 0 as *mut *mut libc::c_char,
                uuconf_pzremote_send: 0 as *mut *mut libc::c_char,
                uuconf_pzlocal_receive: 0 as *mut *mut libc::c_char,
                uuconf_pzremote_receive: 0 as *mut *mut libc::c_char,
                uuconf_pzpath: 0 as *mut *mut libc::c_char,
                uuconf_pzcmds: 0 as *mut *mut libc::c_char,
                uuconf_cfree_space: 0,
                uuconf_pzforward_from: 0 as *mut *mut libc::c_char,
                uuconf_pzforward_to: 0 as *mut *mut libc::c_char,
                uuconf_zpubdir: 0 as *const libc::c_char,
                uuconf_zlocalname: 0 as *mut libc::c_char,
                uuconf_cmax_file_time: 0,
                uuconf_palloc: 0 as *mut libc::c_void,
            };
            ssys.uuconf_zname = ((*qentry).d_name).as_mut_ptr();
            if fsysdep_get_status(
                &mut ssys,
                qstat,
                0 as *mut libc::c_void as *mut boolean,
            ) != 0
            {
                return zbufcpy(((*qentry).d_name).as_mut_ptr());
            }
        }
    };
}
pub unsafe extern "C" fn usysdep_all_status_free(mut phold: pointer) {
    let mut qdir: *mut DIR = phold as *mut DIR;
    closedir(qdir);
}
pub unsafe extern "C" fn fsysdep_lock_status() -> boolean {
    let mut qdir: *mut DIR = 0 as *mut DIR;
    let mut qentry: *mut dirent = 0 as *mut dirent;
    let mut calc: libc::c_int = 0;
    let mut pai: *mut pid_t = 0 as *mut pid_t;
    let mut cgot: libc::c_int = 0;
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ztok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cargs: libc::c_int = 0;
    let mut iarg: libc::c_int = 0;
    let mut pazargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    qdir = opendir(zSlockdir as *mut libc::c_char);
    if qdir.is_null() {
        ulog(
            LOG_ERROR,
            b"opendir (%s): %s\0" as *const u8 as *const libc::c_char,
            zSlockdir,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    calc = 0 as libc::c_int;
    pai = 0 as *mut pid_t;
    cgot = 0 as libc::c_int;
    loop {
        qentry = readdir(qdir);
        if qentry.is_null() {
            break;
        }
        let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut o: libc::c_int = 0;
        let mut ab: [libc::c_char; 12] = [0; 12];
        let mut cread: libc::c_int = 0;
        let mut ierr: libc::c_int = 0;
        let mut ipid: pid_t = 0;
        let mut icheck: libc::c_int = 0;
        if strncmp(
            ((*qentry).d_name).as_mut_ptr(),
            b"LCK..\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0 as libc::c_int
        {
            continue;
        }
        zname = zsysdep_in_dir(zSlockdir, ((*qentry).d_name).as_mut_ptr());
        o = open(zname, 0 as libc::c_int | 0o400 as libc::c_int, 0 as libc::c_int);
        if o < 0 as libc::c_int {
            if *__errno_location() != 2 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                    zname,
                    strerror(*__errno_location()),
                );
            }
            ubuffree(zname);
        } else {
            cread = read(
                o,
                ab.as_mut_ptr() as *mut libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
            ierr = *__errno_location();
            close(o);
            if cread < 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"read %s: %s\0" as *const u8 as *const libc::c_char,
                    zname,
                    strerror(ierr),
                );
                ubuffree(zname);
            } else {
                ubuffree(zname);
                ab[cread as usize] = '\0' as i32 as libc::c_char;
                ipid = strtol(
                    ab.as_mut_ptr(),
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as pid_t;
                printf(
                    b"%s: %ld\n\0" as *const u8 as *const libc::c_char,
                    ((*qentry).d_name).as_mut_ptr(),
                    ipid as libc::c_long,
                );
                icheck = 0 as libc::c_int;
                while icheck < cgot {
                    if *pai.offset(icheck as isize) == ipid {
                        break;
                    }
                    icheck += 1;
                    icheck;
                }
                if icheck < cgot {
                    continue;
                }
                if cgot >= calc {
                    calc += 10 as libc::c_int;
                    pai = xrealloc(
                        pai as pointer,
                        (calc as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<pid_t>() as libc::c_ulong,
                            ),
                    ) as *mut pid_t;
                }
                *pai.offset(cgot as isize) = ipid;
                cgot += 1;
                cgot;
            }
        }
    }
    if cgot == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    aidescs[0 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[1 as libc::c_int as usize] = 1 as libc::c_int;
    aidescs[2 as libc::c_int as usize] = 2 as libc::c_int;
    zcopy = zbufcpy(b"/bin/ps -lp\0" as *const u8 as *const libc::c_char);
    cargs = 0 as libc::c_int;
    ztok = strtok(zcopy, b" \t\0" as *const u8 as *const libc::c_char);
    while !ztok.is_null() {
        cargs += 1;
        cargs;
        ztok = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
    }
    pazargs = xmalloc(
        ((cargs + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    memcpy(
        zcopy as *mut libc::c_void,
        b"/bin/ps -lp\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    ztok = strtok(zcopy, b" \t\0" as *const u8 as *const libc::c_char);
    iarg = 0 as libc::c_int;
    while !ztok.is_null() {
        let ref mut fresh0 = *pazargs.offset(iarg as isize);
        *fresh0 = ztok;
        ztok = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
        iarg += 1;
        iarg;
    }
    let ref mut fresh1 = *pazargs.offset(iarg as isize);
    *fresh1 = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut zlast: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zset: *mut libc::c_char = 0 as *mut libc::c_char;
    zlast = *pazargs.offset((cargs - 1 as libc::c_int) as isize);
    zset = zbufalc((strlen(zlast)).wrapping_add(20 as libc::c_int as libc::c_ulong));
    i = 0 as libc::c_int;
    while i < cgot {
        let mut ipid_0: pid_t = 0;
        sprintf(
            zset,
            b"%s%ld\0" as *const u8 as *const libc::c_char,
            zlast,
            *pai.offset(i as isize) as libc::c_long,
        );
        let ref mut fresh2 = *pazargs.offset((cargs - 1 as libc::c_int) as isize);
        *fresh2 = zset;
        ipid_0 = ixsspawn(
            pazargs as *mut *const libc::c_char,
            aidescs.as_mut_ptr(),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_char,
        );
        if ipid_0 < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"ixsspawn: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        } else {
            ixswait(
                ipid_0 as libc::c_ulong,
                b"/bin/ps -lp\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    ubuffree(zset);
    ubuffree(zcopy);
    xfree(pazargs as pointer);
    return 1 as libc::c_int;
}

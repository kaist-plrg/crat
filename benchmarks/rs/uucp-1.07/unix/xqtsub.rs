use ::libc;
extern "C" {
    pub type __dirstream;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn fcopy_file(
        zfrom: *const libc::c_char,
        zto: *const libc::c_char,
        fpublic: boolean,
        fmkdirs: boolean,
        fsignals: boolean,
    ) -> boolean;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
    fn fsysdep_rmdir(zdir: *const libc::c_char) -> boolean;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fsdo_lock(
        _: *const libc::c_char,
        fspooldir: boolean,
        pferr: *mut boolean,
    ) -> boolean;
    fn fsdo_unlock(_: *const libc::c_char, fspooldir: boolean) -> boolean;
    fn zstemp_file(qsys: *const uuconf_system) -> *mut libc::c_char;
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
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type mode_t = __mode_t;
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
pub static mut xqtsub_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: xqtsub.c,v 1.24 2002/03/05 19:10:42 ian Rel $\0")
};
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn zsysdep_find_command(
    mut zcmd: *const libc::c_char,
    mut pzcmds: *mut *mut libc::c_char,
    mut pzpath: *mut *mut libc::c_char,
    mut pferr: *mut boolean,
) -> *mut libc::c_char {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
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
    *pferr = 0 as libc::c_int;
    pz = pzcmds;
    while !(*pz).is_null() {
        let mut zslash: *mut libc::c_char = 0 as *mut libc::c_char;
        if strcmp(*pz, b"ALL\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            break;
        }
        zslash = strrchr(*pz, '/' as i32);
        if !zslash.is_null() {
            zslash = zslash.offset(1);
            zslash;
        } else {
            zslash = *pz;
        }
        if strcmp(zslash, zcmd) == 0 as libc::c_int
            || strcmp(*pz, zcmd) == 0 as libc::c_int
        {
            if **pz as libc::c_int == '/' as i32 {
                if stat(*pz, &mut s) != 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        *pz,
                        strerror(*__errno_location()),
                    );
                    *pferr = 1 as libc::c_int;
                    return 0 as *mut libc::c_char;
                }
                return zbufcpy(*pz);
            }
            break;
        } else {
            pz = pz.offset(1);
            pz;
        }
    }
    if (*pz).is_null() {
        return 0 as *mut libc::c_char;
    }
    pz = pzpath;
    while !(*pz).is_null() {
        let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
        zname = zsysdep_in_dir(*pz, zcmd);
        if stat(zname, &mut s) == 0 as libc::c_int {
            return zname;
        }
        pz = pz.offset(1);
        pz;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn zsysdep_xqt_local_file(
    mut qsys: *const uuconf_system,
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    if *zfile as libc::c_int != '~' as i32 {
        return 0 as *mut libc::c_char;
    }
    if *zfile.offset(1 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        let mut clen: size_t = 0;
        let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
        clen = strlen(zfile);
        zret = zbufalc(clen);
        memcpy(
            zret as *mut libc::c_void,
            zfile.offset(1 as libc::c_int as isize) as *const libc::c_void,
            clen,
        );
        return zret;
    }
    return zsysdep_local_file(
        zfile,
        (*qsys).uuconf_zpubdir,
        0 as *mut libc::c_void as *mut boolean,
    );
}
pub unsafe extern "C" fn fsysdep_execute(
    mut qsys: *const uuconf_system,
    mut zuser: *const libc::c_char,
    mut pazargs: *mut *const libc::c_char,
    mut zfullcmd: *const libc::c_char,
    mut zinput: *const libc::c_char,
    mut zoutput: *const libc::c_char,
    mut fshell: boolean,
    mut iseq: libc::c_int,
    mut pzerror: *mut *mut libc::c_char,
    mut pftemp: *mut boolean,
) -> boolean {
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut ferr: boolean = 0;
    let mut ipid: pid_t = 0;
    let mut ierr: libc::c_int = 0;
    let mut abxqtdir: [libc::c_char; 12] = [0; 12];
    let mut zxqtdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut istat: libc::c_int = 0;
    let mut zpath: *mut libc::c_char = 0 as *mut libc::c_char;
    *pzerror = 0 as *mut libc::c_char;
    *pftemp = 0 as libc::c_int;
    aidescs[0 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[1 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[2 as libc::c_int as usize] = -(1 as libc::c_int);
    ferr = 0 as libc::c_int;
    if !zinput.is_null() {
        aidescs[0 as libc::c_int
            as usize] = open(
            zinput as *mut libc::c_char,
            0 as libc::c_int | 0o400 as libc::c_int,
            0 as libc::c_int,
        );
        if aidescs[0 as libc::c_int as usize] < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                zinput,
                strerror(*__errno_location()),
            );
            ferr = 1 as libc::c_int;
        } else if fcntl(
            aidescs[0 as libc::c_int as usize],
            2 as libc::c_int,
            fcntl(aidescs[0 as libc::c_int as usize], 1 as libc::c_int, 0 as libc::c_int)
                | 1 as libc::c_int,
        ) < 0 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            ferr = 1 as libc::c_int;
        }
    }
    if ferr == 0 && !zoutput.is_null() {
        aidescs[1 as libc::c_int
            as usize] = creat(
            zoutput as *mut libc::c_char,
            (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
        );
        if aidescs[1 as libc::c_int as usize] < 0 as libc::c_int {
            if *__errno_location() == 2 as libc::c_int
                && *zoutput.offset(0 as libc::c_int as isize) as libc::c_int
                    != '/' as i32
            {
                if fsysdep_make_dirs(zoutput, 0 as libc::c_int) == 0 {
                    *pftemp = 1 as libc::c_int;
                    ferr = 1 as libc::c_int;
                } else {
                    aidescs[1 as libc::c_int
                        as usize] = creat(
                        zoutput as *mut libc::c_char,
                        (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
                    );
                }
            }
            if ferr == 0 && aidescs[1 as libc::c_int as usize] < 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"creat (%s): %s\0" as *const u8 as *const libc::c_char,
                    zoutput,
                    strerror(*__errno_location()),
                );
                *pftemp = 1 as libc::c_int;
                ferr = 1 as libc::c_int;
            }
        }
        if ferr == 0
            && fcntl(
                aidescs[1 as libc::c_int as usize],
                2 as libc::c_int,
                fcntl(
                    aidescs[1 as libc::c_int as usize],
                    1 as libc::c_int,
                    0 as libc::c_int,
                ) | 1 as libc::c_int,
            ) < 0 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            ferr = 1 as libc::c_int;
        }
    }
    if ferr == 0 {
        *pzerror = zstemp_file(qsys);
        aidescs[2 as libc::c_int
            as usize] = creat(
            *pzerror,
            (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
        );
        if aidescs[2 as libc::c_int as usize] < 0 as libc::c_int {
            if *__errno_location() == 2 as libc::c_int {
                if fsysdep_make_dirs(*pzerror, 0 as libc::c_int) == 0 {
                    *pftemp = 1 as libc::c_int;
                    ferr = 1 as libc::c_int;
                } else {
                    aidescs[2 as libc::c_int
                        as usize] = creat(
                        *pzerror,
                        (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
                    );
                }
            }
            if ferr == 0 && aidescs[2 as libc::c_int as usize] < 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"creat (%s): %s\0" as *const u8 as *const libc::c_char,
                    *pzerror,
                    strerror(*__errno_location()),
                );
                *pftemp = 1 as libc::c_int;
                ferr = 1 as libc::c_int;
            }
        }
        if ferr == 0
            && fcntl(
                aidescs[2 as libc::c_int as usize],
                2 as libc::c_int,
                fcntl(
                    aidescs[2 as libc::c_int as usize],
                    1 as libc::c_int,
                    0 as libc::c_int,
                ) | 1 as libc::c_int,
            ) < 0 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            ferr = 1 as libc::c_int;
        }
    }
    if iseq == 0 as libc::c_int {
        zxqtdir = b".Xqtdir\0" as *const u8 as *const libc::c_char;
    } else {
        sprintf(
            abxqtdir.as_mut_ptr(),
            b"%s%04d\0" as *const u8 as *const libc::c_char,
            b".Xqtdir\0" as *const u8 as *const libc::c_char,
            iseq,
        );
        zxqtdir = abxqtdir.as_mut_ptr();
    }
    if ferr != 0 {
        if aidescs[0 as libc::c_int as usize] != -(1 as libc::c_int) {
            close(aidescs[0 as libc::c_int as usize]);
        }
        if aidescs[1 as libc::c_int as usize] != -(1 as libc::c_int) {
            close(aidescs[1 as libc::c_int as usize]);
        }
        if aidescs[2 as libc::c_int as usize] != -(1 as libc::c_int) {
            close(aidescs[2 as libc::c_int as usize]);
        }
        ubuffree(*pzerror);
        return 0 as libc::c_int;
    }
    fshell = 0 as libc::c_int;
    if ((*qsys).uuconf_pzpath).is_null() {
        zpath = 0 as *mut libc::c_char;
    } else {
        let mut c: size_t = 0;
        let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        c = 0 as libc::c_int as size_t;
        pz = (*qsys).uuconf_pzpath;
        while !(*pz).is_null() {
            c = (c as libc::c_ulong)
                .wrapping_add(
                    (strlen(*pz)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            pz = pz.offset(1);
            pz;
        }
        zpath = zbufalc(c);
        *zpath = '\0' as i32 as libc::c_char;
        pz = (*qsys).uuconf_pzpath;
        while !(*pz).is_null() {
            strcat(zpath, *pz);
            if !(*pz.offset(1 as libc::c_int as isize)).is_null() {
                strcat(zpath, b":\0" as *const u8 as *const libc::c_char);
            }
            pz = pz.offset(1);
            pz;
        }
    }
    ipid = ixsspawn(
        pazargs,
        aidescs.as_mut_ptr(),
        1 as libc::c_int,
        0 as libc::c_int,
        zxqtdir,
        1 as libc::c_int,
        (fshell == 0) as libc::c_int,
        zpath,
        (*qsys).uuconf_zname,
        zuser,
    );
    ierr = *__errno_location();
    ubuffree(zpath);
    if aidescs[0 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(aidescs[0 as libc::c_int as usize]);
    }
    if aidescs[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(aidescs[1 as libc::c_int as usize]);
    }
    if aidescs[2 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(aidescs[2 as libc::c_int as usize]);
    }
    if ipid < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ixsspawn: %s\0" as *const u8 as *const libc::c_char,
            strerror(ierr),
        );
        *pftemp = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    istat = ixswait(
        ipid as libc::c_ulong,
        b"Execution\0" as *const u8 as *const libc::c_char,
    );
    if istat == 75 as libc::c_int {
        *pftemp = 1 as libc::c_int;
    }
    return (istat == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn ixsysdep_lock_uuxqt(
    mut zcmd: *const libc::c_char,
    mut cmaxuuxqts: libc::c_int,
) -> libc::c_int {
    let mut ab: [libc::c_char; 13] = [0; 13];
    let mut i: libc::c_int = 0;
    if cmaxuuxqts <= 0 as libc::c_int || cmaxuuxqts >= 10000 as libc::c_int {
        cmaxuuxqts = 9999 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < cmaxuuxqts {
        sprintf(ab.as_mut_ptr(), b"LCK.XQT.%d\0" as *const u8 as *const libc::c_char, i);
        if fsdo_lock(
            ab.as_mut_ptr(),
            1 as libc::c_int,
            0 as *mut libc::c_void as *mut boolean,
        ) != 0
        {
            break;
        }
        i += 1;
        i;
    }
    if i >= cmaxuuxqts {
        return -(1 as libc::c_int);
    }
    if !zcmd.is_null() {
        let mut abcmd: [libc::c_char; 14] = [0; 14];
        sprintf(
            abcmd.as_mut_ptr(),
            b"LXQ.%.9s\0" as *const u8 as *const libc::c_char,
            zcmd,
        );
        abcmd[strcspn(abcmd.as_mut_ptr(), b" \t/\0" as *const u8 as *const libc::c_char)
            as usize] = '\0' as i32 as libc::c_char;
        if fsdo_lock(
            abcmd.as_mut_ptr(),
            1 as libc::c_int,
            0 as *mut libc::c_void as *mut boolean,
        ) == 0
        {
            fsdo_unlock(ab.as_mut_ptr(), 1 as libc::c_int);
            return -(1 as libc::c_int);
        }
    }
    return i;
}
pub unsafe extern "C" fn fsysdep_unlock_uuxqt(
    mut iseq: libc::c_int,
    mut zcmd: *const libc::c_char,
    mut cmaxuuxqts: libc::c_int,
) -> boolean {
    let mut ab: [libc::c_char; 13] = [0; 13];
    let mut fret: boolean = 0;
    fret = 1 as libc::c_int;
    sprintf(ab.as_mut_ptr(), b"LCK.XQT.%d\0" as *const u8 as *const libc::c_char, iseq);
    if fsdo_unlock(ab.as_mut_ptr(), 1 as libc::c_int) == 0 {
        fret = 0 as libc::c_int;
    }
    if !zcmd.is_null() {
        let mut abcmd: [libc::c_char; 14] = [0; 14];
        sprintf(
            abcmd.as_mut_ptr(),
            b"LXQ.%.9s\0" as *const u8 as *const libc::c_char,
            zcmd,
        );
        abcmd[strcspn(abcmd.as_mut_ptr(), b" \t/\0" as *const u8 as *const libc::c_char)
            as usize] = '\0' as i32 as libc::c_char;
        if fsdo_unlock(abcmd.as_mut_ptr(), 1 as libc::c_int) == 0 {
            fret = 0 as libc::c_int;
        }
    }
    return fret;
}
pub unsafe extern "C" fn fsysdep_uuxqt_locked(mut zcmd: *const libc::c_char) -> boolean {
    let mut ab: [libc::c_char; 14] = [0; 14];
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
    sprintf(ab.as_mut_ptr(), b"LXQ.%.9s\0" as *const u8 as *const libc::c_char, zcmd);
    return (stat(ab.as_mut_ptr(), &mut s) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_lock_uuxqt_file(
    mut zfile: *const libc::c_char,
) -> boolean {
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    zcopy = zbufcpy(zfile);
    z = strrchr(zcopy, '/' as i32);
    if z.is_null() {
        *zcopy = 'L' as i32 as libc::c_char;
    } else {
        *z.offset(1 as libc::c_int as isize) = 'L' as i32 as libc::c_char;
    }
    fret = fsdo_lock(zcopy, 1 as libc::c_int, 0 as *mut libc::c_void as *mut boolean);
    ubuffree(zcopy);
    return fret;
}
pub unsafe extern "C" fn fsysdep_unlock_uuxqt_file(
    mut zfile: *const libc::c_char,
) -> boolean {
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    zcopy = zbufcpy(zfile);
    z = strrchr(zcopy, '/' as i32);
    if z.is_null() {
        *zcopy = 'L' as i32 as libc::c_char;
    } else {
        *z.offset(1 as libc::c_int as isize) = 'L' as i32 as libc::c_char;
    }
    fret = fsdo_unlock(zcopy, 1 as libc::c_int);
    ubuffree(zcopy);
    return fret;
}
pub unsafe extern "C" fn fsysdep_lock_uuxqt_dir(mut iseq: libc::c_int) -> boolean {
    let mut zxqtdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut abxqtdir: [libc::c_char; 12] = [0; 12];
    if iseq == 0 as libc::c_int {
        zxqtdir = b".Xqtdir\0" as *const u8 as *const libc::c_char;
    } else {
        sprintf(
            abxqtdir.as_mut_ptr(),
            b"%s%04d\0" as *const u8 as *const libc::c_char,
            b".Xqtdir\0" as *const u8 as *const libc::c_char,
            iseq,
        );
        zxqtdir = abxqtdir.as_mut_ptr();
    }
    if mkdir(
        zxqtdir,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) as __mode_t,
    ) < 0 as libc::c_int && *__errno_location() != 17 as libc::c_int
        && *__errno_location() != 21 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"mkdir (%s): %s\0" as *const u8 as *const libc::c_char,
            zxqtdir,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return fclean_uuxqt_dir(zxqtdir);
}
pub unsafe extern "C" fn fsysdep_unlock_uuxqt_dir(mut iseq: libc::c_int) -> boolean {
    let mut zxqtdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut abxqtdir: [libc::c_char; 12] = [0; 12];
    if iseq == 0 as libc::c_int {
        zxqtdir = b".Xqtdir\0" as *const u8 as *const libc::c_char;
    } else {
        sprintf(
            abxqtdir.as_mut_ptr(),
            b"%s%04d\0" as *const u8 as *const libc::c_char,
            b".Xqtdir\0" as *const u8 as *const libc::c_char,
            iseq,
        );
        zxqtdir = abxqtdir.as_mut_ptr();
    }
    return fclean_uuxqt_dir(zxqtdir);
}
unsafe extern "C" fn fclean_uuxqt_dir(mut zxqtdir: *const libc::c_char) -> boolean {
    let mut qdir: *mut DIR = 0 as *mut DIR;
    qdir = opendir(zxqtdir as *mut libc::c_char);
    if !qdir.is_null() {
        let mut qentry: *mut dirent = 0 as *mut dirent;
        loop {
            qentry = readdir(qdir);
            if qentry.is_null() {
                break;
            }
            let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
            if strcmp(
                ((*qentry).d_name).as_mut_ptr(),
                b".\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    ((*qentry).d_name).as_mut_ptr(),
                    b"..\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                continue;
            }
            z = zsysdep_in_dir(zxqtdir, ((*qentry).d_name).as_mut_ptr());
            if remove(z) < 0 as libc::c_int {
                let mut ierr: libc::c_int = 0;
                ierr = *__errno_location();
                if fsysdep_directory(z) == 0 {
                    ulog(
                        LOG_ERROR,
                        b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
                        z,
                        strerror(ierr),
                    );
                } else {
                    fsysdep_rmdir(z);
                }
            }
            ubuffree(z);
        }
        closedir(qdir);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_copy_uuxqt_files(
    mut cfiles: libc::c_int,
    mut pzfrom: *const *const libc::c_char,
    mut pzto: *const *const libc::c_char,
    mut iseq: libc::c_int,
    mut pzinput: *mut *mut libc::c_char,
) -> boolean {
    let mut zinput: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zxqtdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut abxqtdir: [libc::c_char; 12] = [0; 12];
    let mut i: libc::c_int = 0;
    if pzinput.is_null() {
        zinput = 0 as *mut libc::c_char;
    } else {
        zinput = *pzinput;
    }
    if iseq == 0 as libc::c_int {
        zxqtdir = b".Xqtdir\0" as *const u8 as *const libc::c_char;
    } else {
        sprintf(
            abxqtdir.as_mut_ptr(),
            b"%s%04d\0" as *const u8 as *const libc::c_char,
            b".Xqtdir\0" as *const u8 as *const libc::c_char,
            iseq,
        );
        zxqtdir = abxqtdir.as_mut_ptr();
    }
    i = 0 as libc::c_int;
    while i < cfiles {
        let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
        let mut zto: *const libc::c_char = 0 as *const libc::c_char;
        let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
        if !(*pzto.offset(i as isize)).is_null() {
            zfree = zsysdep_in_dir(zxqtdir, *pzto.offset(i as isize));
            zfrom = *pzfrom.offset(i as isize);
            zto = zfree;
            if !zinput.is_null() && strcmp(zinput, zfrom) == 0 as libc::c_int {
                *pzinput = zbufcpy(zto);
                zinput = 0 as *mut libc::c_char;
            }
            if link(zfrom, zto) < 0 as libc::c_int {
                if *__errno_location() != 18 as libc::c_int
                    && *__errno_location() != 17 as libc::c_int
                    && *__errno_location() != 31 as libc::c_int
                {
                    ulog(
                        LOG_ERROR,
                        b"link (%s, %s): %s\0" as *const u8 as *const libc::c_char,
                        zfrom,
                        zto,
                        strerror(*__errno_location()),
                    );
                    ubuffree(zfree);
                    return 0 as libc::c_int;
                }
                if fcopy_file(
                    zfrom,
                    zto,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    ubuffree(zfree);
                    return 0 as libc::c_int;
                }
            }
            chmod(
                zto,
                (0o400 as libc::c_int | 0o200 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as __mode_t,
            );
            ubuffree(zfree);
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}

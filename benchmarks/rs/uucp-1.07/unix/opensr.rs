use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fsuser_access(
        _: *const stat,
        imode: libc::c_int,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn zsappend3(
        zdir1: *const libc::c_char,
        zdir2: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn zstemp_file(qsys: *const uuconf_system) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type mode_t = __mode_t;
pub type time_t = __time_t;
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
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub unsafe extern "C" fn esysdep_open_send(
    mut qsys: *const uuconf_system,
    mut zfile: *const libc::c_char,
    mut fcheck: boolean,
    mut zuser: *const libc::c_char,
) -> openfile_t {
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
    let mut e: openfile_t = 0 as *mut FILE;
    let mut o: libc::c_int = 0;
    if fsysdep_directory(zfile) != 0 {
        ulog(
            LOG_ERROR,
            b"%s: is a directory\0" as *const u8 as *const libc::c_char,
            zfile,
        );
        return 0 as *mut libc::c_void as *mut FILE;
    }
    e = fopen(zfile, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        ulog(
            LOG_ERROR,
            b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
            zfile,
            strerror(*__errno_location()),
        );
        return 0 as openfile_t;
    }
    o = fileno(e);
    if fcntl(
        o,
        2 as libc::c_int,
        fcntl(o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fclose(e);
        return 0 as *mut libc::c_void as *mut FILE;
    }
    if fstat(o, &mut s) == -(1 as libc::c_int) {
        ulog(
            LOG_ERROR,
            b"fstat: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        s.st_mode = 0o666 as libc::c_int as __mode_t;
    }
    if fcheck != 0 {
        if fsuser_access(&mut s, 4 as libc::c_int, zuser) == 0 {
            ulog(
                LOG_ERROR,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                zfile,
                strerror(13 as libc::c_int),
            );
            fclose(e);
            return 0 as *mut libc::c_void as *mut FILE;
        }
    }
    return e;
}
pub unsafe extern "C" fn zsysdep_receive_temp(
    mut qsys: *const uuconf_system,
    mut zto: *const libc::c_char,
    mut ztemp: *const libc::c_char,
    mut frestart: boolean,
) -> *mut libc::c_char {
    if frestart != 0 && !ztemp.is_null() && *ztemp as libc::c_int == 'D' as i32
        && strcmp(ztemp, b"D.0\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        return zsappend3(
            b".Temp\0" as *const u8 as *const libc::c_char,
            (*qsys).uuconf_zname,
            ztemp,
        )
    } else {
        return zstemp_file(qsys)
    };
}
pub unsafe extern "C" fn esysdep_open_receive(
    mut qsys: *const uuconf_system,
    mut zto: *const libc::c_char,
    mut ztemp: *const libc::c_char,
    mut zreceive: *const libc::c_char,
    mut pcrestart: *mut libc::c_long,
) -> openfile_t {
    let mut o: libc::c_int = 0;
    let mut e: openfile_t = 0 as *mut FILE;
    o = -(1 as libc::c_int);
    if !pcrestart.is_null() {
        *pcrestart = -(1 as libc::c_int) as libc::c_long;
    }
    if !pcrestart.is_null() && !ztemp.is_null() && *ztemp as libc::c_int == 'D' as i32
        && strcmp(ztemp, b"D.0\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        o = open(zreceive as *mut libc::c_char, 0o1 as libc::c_int);
        if o >= 0 as libc::c_int {
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
            if fstat(o, &mut s) < 0 as libc::c_int
                || (s.st_mtim.tv_sec
                    + 7 as libc::c_int as libc::c_long
                        * 24 as libc::c_int as libc::c_long
                        * 60 as libc::c_int as libc::c_long
                        * 60 as libc::c_int as libc::c_long)
                    < time(0 as *mut libc::c_void as *mut time_t)
            {
                close(o);
                o = -(1 as libc::c_int);
            } else {
                if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"esysdep_open_receive: Reusing %s\0" as *const u8
                            as *const libc::c_char,
                        zreceive,
                    );
                }
                *pcrestart = s.st_size;
            }
        }
    }
    if o < 0 as libc::c_int {
        o = creat(
            zreceive as *mut libc::c_char,
            (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
        );
    }
    if o < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            if fsysdep_make_dirs(zreceive, 0 as libc::c_int) == 0 {
                return 0 as *mut libc::c_void as *mut FILE;
            }
            o = creat(
                zreceive as *mut libc::c_char,
                (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
            );
        }
        if o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"creat (%s): %s\0" as *const u8 as *const libc::c_char,
                zreceive,
                strerror(*__errno_location()),
            );
            return 0 as *mut libc::c_void as *mut FILE;
        }
    }
    if fcntl(
        o,
        2 as libc::c_int,
        fcntl(o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(o);
        remove(zreceive);
        return 0 as *mut libc::c_void as *mut FILE;
    }
    e = fdopen(o, b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if e.is_null() {
        ulog(
            LOG_ERROR,
            b"fdopen (%s): %s\0" as *const u8 as *const libc::c_char,
            zreceive,
            strerror(*__errno_location()),
        );
        close(o);
        remove(zreceive);
        return 0 as *mut libc::c_void as *mut FILE;
    }
    return e;
}

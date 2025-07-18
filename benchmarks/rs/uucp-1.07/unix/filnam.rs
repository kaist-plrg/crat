use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ubuffree(z: *mut libc::c_char);
    static mut afSignal: [sig_atomic_t; 5];
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    static mut zSlocalname: *const libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fsdo_unlock(_: *const libc::c_char, fspooldir: boolean) -> boolean;
    fn fsdo_lock(
        _: *const libc::c_char,
        fspooldir: boolean,
        pferr: *mut boolean,
    ) -> boolean;
    fn zsfind_file(
        zsimple: *const libc::c_char,
        zsystem: *const libc::c_char,
        bgrade: libc::c_int,
    ) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn fsysdep_file_exists(zfile: *const libc::c_char) -> boolean;
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type sig_atomic_t = __sig_atomic_t;
pub type pid_t = __pid_t;
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
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
unsafe extern "C" fn fscmd_seq(
    mut zsystem: *const libc::c_char,
    mut zseq: *mut libc::c_char,
) -> boolean {
    let mut cdelay: libc::c_int = 0;
    let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: libc::c_int = 0;
    let mut flockfile: boolean = 0;
    let mut i: libc::c_int = 0;
    let mut fret: boolean = 0;
    cdelay = 5 as libc::c_int;
    zfree = 0 as *mut libc::c_char;
    zfree = zsysdep_in_dir(zsystem, b"SEQF\0" as *const u8 as *const libc::c_char);
    zfile = zfree;
    o = open(
        zfile as *mut libc::c_char,
        0o2 as libc::c_int | 0o100 as libc::c_int | 0o400 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if o < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            if fsysdep_make_dirs(zfile, 0 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            o = open(
                zfile as *mut libc::c_char,
                0o2 as libc::c_int | 0o100 as libc::c_int | 0o400 as libc::c_int,
                0o400 as libc::c_int | 0o200 as libc::c_int,
            );
        }
        if o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                zfile,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    let mut slock: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    flockfile = 0 as libc::c_int;
    slock.l_type = 1 as libc::c_int as libc::c_short;
    slock.l_whence = 0 as libc::c_int as libc::c_short;
    slock.l_start = 0 as libc::c_int as __off_t;
    slock.l_len = 0 as libc::c_int as __off_t;
    while fcntl(o, 7 as libc::c_int, &mut slock as *mut flock) == -(1 as libc::c_int) {
        let mut fagain: boolean = 0;
        if *__errno_location() == 22 as libc::c_int {
            let mut ferr: boolean = 0;
            while fsdo_lock(
                b"LCK..SEQ\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                &mut ferr,
            ) == 0
            {
                if ferr != 0
                    || (afSignal[0 as libc::c_int as usize] != 0
                        || afSignal[1 as libc::c_int as usize] != 0
                        || afSignal[2 as libc::c_int as usize] != 0
                        || afSignal[3 as libc::c_int as usize] != 0
                        || afSignal[4 as libc::c_int as usize] != 0)
                {
                    close(o);
                    return 0 as libc::c_int;
                }
                sleep(cdelay as libc::c_uint);
                if cdelay < 60 as libc::c_int {
                    cdelay += 1;
                    cdelay;
                }
            }
            flockfile = 1 as libc::c_int;
            break;
        } else {
            fagain = 0 as libc::c_int;
            if *__errno_location() == 12 as libc::c_int {
                fagain = 1 as libc::c_int;
            }
            if *__errno_location() == 37 as libc::c_int {
                fagain = 1 as libc::c_int;
            }
            if *__errno_location() == 28 as libc::c_int {
                fagain = 1 as libc::c_int;
            }
            if fagain != 0 {
                sleep(cdelay as libc::c_uint);
                if cdelay < 60 as libc::c_int {
                    cdelay += 1;
                    cdelay;
                }
            } else {
                ulog(
                    LOG_ERROR,
                    b"Locking %s: %s\0" as *const u8 as *const libc::c_char,
                    zfile,
                    strerror(*__errno_location()),
                );
                close(o);
                return 0 as libc::c_int;
            }
        }
    }
    if read(o, zseq as *mut libc::c_void, 4 as libc::c_int as size_t)
        != 4 as libc::c_int as libc::c_long
    {
        strcpy(zseq, b"0000\0" as *const u8 as *const libc::c_char);
    }
    *zseq.offset(4 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    i = 4 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut zdig: *const libc::c_char = 0 as *const libc::c_char;
        zdig = strchr(
            b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8
                as *const libc::c_char,
            *zseq.offset(i as isize) as libc::c_int,
        );
        if zdig.is_null()
            || *zdig.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            || *zdig.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            *zseq.offset(i as isize) = '0' as i32 as libc::c_char;
            i -= 1;
            i;
        } else {
            *zseq.offset(i as isize) = *zdig.offset(1 as libc::c_int as isize);
            break;
        }
    }
    fret = 1 as libc::c_int;
    if lseek(o, 0 as libc::c_int as off_t, 0 as libc::c_int)
        < 0 as libc::c_int as libc::c_long
        || write(o, zseq as *const libc::c_void, 4 as libc::c_int as size_t)
            != 4 as libc::c_int as libc::c_long || close(o) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"lseek or write or close %s: %s\0" as *const u8 as *const libc::c_char,
            zfile,
            strerror(*__errno_location()),
        );
        close(o);
        fret = 0 as libc::c_int;
    }
    if flockfile != 0 {
        fsdo_unlock(b"LCK..SEQ\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    }
    ubuffree(zfree);
    return fret;
}
unsafe extern "C" fn zsfile_name(
    mut btype: libc::c_int,
    mut zsystem: *const libc::c_char,
    mut zlocalname: *const libc::c_char,
    mut bgrade: libc::c_int,
    mut fxqt: boolean,
    mut ztname: *mut libc::c_char,
    mut zdname: *mut libc::c_char,
    mut zxname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut abseq: [libc::c_char; 5] = [0; 5];
    let mut absimple: [libc::c_char; 15] = [0; 15];
    let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
    if zlocalname.is_null() {
        zlocalname = zSlocalname;
    }
    loop {
        if fscmd_seq(zsystem, abseq.as_mut_ptr()) == 0 {
            return 0 as *mut libc::c_char;
        }
        if btype == 'C' as i32 {
            sprintf(
                absimple.as_mut_ptr(),
                b"C.%c%s\0" as *const u8 as *const libc::c_char,
                bgrade,
                abseq.as_mut_ptr(),
            );
        } else if btype == 'D' as i32 {
            if fxqt != 0 {
                sprintf(
                    absimple.as_mut_ptr(),
                    b"D.X%s\0" as *const u8 as *const libc::c_char,
                    abseq.as_mut_ptr(),
                );
            } else {
                sprintf(
                    absimple.as_mut_ptr(),
                    b"D.%s\0" as *const u8 as *const libc::c_char,
                    abseq.as_mut_ptr(),
                );
            }
        } else {
            ulog(
                LOG_FATAL,
                b"zsfile_name: Can't happen\0" as *const u8 as *const libc::c_char,
            );
        }
        zname = zsfind_file(absimple.as_mut_ptr(), zsystem, bgrade);
        if zname.is_null() {
            return 0 as *mut libc::c_char;
        }
        if fsysdep_file_exists(zname) == 0 {
            break;
        }
        ubuffree(zname);
    }
    if !ztname.is_null() {
        strcpy(ztname, absimple.as_mut_ptr());
    }
    if !zdname.is_null() {
        sprintf(
            zdname,
            b"D.%.7s%c%s\0" as *const u8 as *const libc::c_char,
            zlocalname,
            bgrade,
            abseq.as_mut_ptr(),
        );
    }
    if !zxname.is_null() {
        sprintf(
            zxname,
            b"X.%.7s%c%s\0" as *const u8 as *const libc::c_char,
            zlocalname,
            bgrade,
            abseq.as_mut_ptr(),
        );
    }
    return zname;
}
pub unsafe extern "C" fn zsysdep_data_file_name(
    mut qsys: *const uuconf_system,
    mut zlocalname: *const libc::c_char,
    mut bgrade: libc::c_int,
    mut fxqt: boolean,
    mut ztname: *mut libc::c_char,
    mut zdname: *mut libc::c_char,
    mut zxname: *mut libc::c_char,
) -> *mut libc::c_char {
    return zsfile_name(
        'D' as i32,
        (*qsys).uuconf_zname,
        zlocalname,
        bgrade,
        fxqt,
        ztname,
        zdname,
        zxname,
    );
}
unsafe extern "C" fn usput62(
    mut i: libc::c_long,
    mut z: *mut libc::c_char,
    mut c: libc::c_int,
) {
    c -= 1;
    c;
    while c >= 0 as libc::c_int {
        let mut d: libc::c_int = 0;
        d = (i % 62 as libc::c_int as libc::c_long) as libc::c_int;
        i /= 62 as libc::c_int as libc::c_long;
        if d < 26 as libc::c_int {
            *z.offset(c as isize) = ('A' as i32 + d) as libc::c_char;
        } else if d < 52 as libc::c_int {
            *z.offset(c as isize) = ('a' as i32 + d - 26 as libc::c_int) as libc::c_char;
        } else {
            *z.offset(c as isize) = ('0' as i32 + d - 52 as libc::c_int) as libc::c_char;
        }
        c -= 1;
        c;
    }
}
pub unsafe extern "C" fn zscmd_file(
    mut qsys: *const uuconf_system,
    mut bgrade: libc::c_int,
) -> *mut libc::c_char {
    let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut isecs: libc::c_long = 0;
    let mut imicros: libc::c_long = 0;
    let mut ipid: pid_t = 0;
    isecs = ixsysdep_time(&mut imicros);
    ipid = getpid();
    isecs
        %= 24 as libc::c_int as libc::c_long * 60 as libc::c_int as libc::c_long
            * 60 as libc::c_int as libc::c_long;
    imicros %= 1000000 as libc::c_int as libc::c_long;
    imicros /= 5 as libc::c_int as libc::c_long;
    loop {
        let mut ab: [libc::c_char; 15] = [0; 15];
        ab[0 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
        ab[1 as libc::c_int as usize] = '.' as i32 as libc::c_char;
        ab[2 as libc::c_int as usize] = bgrade as libc::c_char;
        usput62(
            isecs,
            ab.as_mut_ptr().offset(3 as libc::c_int as isize),
            3 as libc::c_int,
        );
        usput62(
            imicros,
            ab.as_mut_ptr().offset(6 as libc::c_int as isize),
            3 as libc::c_int,
        );
        usput62(
            ipid as libc::c_long,
            ab.as_mut_ptr().offset(9 as libc::c_int as isize),
            5 as libc::c_int,
        );
        ab[14 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        zname = zsfind_file(ab.as_mut_ptr(), (*qsys).uuconf_zname, bgrade);
        if zname.is_null() {
            return 0 as *mut libc::c_char;
        }
        if fsysdep_file_exists(zname) == 0 {
            break;
        }
        ubuffree(zname);
        if imicros == 0 as libc::c_int as libc::c_long {
            imicros = 62 as libc::c_int as libc::c_long
                * 62 as libc::c_int as libc::c_long * 62 as libc::c_int as libc::c_long;
            if isecs == 0 as libc::c_int as libc::c_long {
                isecs = 62 as libc::c_int as libc::c_long
                    * 62 as libc::c_int as libc::c_long
                    * 62 as libc::c_int as libc::c_long;
            }
            isecs -= 1;
            isecs;
        }
        imicros -= 1;
        imicros;
    }
    return zname;
}
pub unsafe extern "C" fn zsysdep_xqt_file_name() -> *mut libc::c_char {
    let mut abseq: [libc::c_char; 5] = [0; 5];
    let mut absx: [libc::c_char; 15] = [0; 15];
    let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        if fscmd_seq(zSlocalname, abseq.as_mut_ptr()) == 0 {
            return 0 as *mut libc::c_char;
        }
        sprintf(
            absx.as_mut_ptr(),
            b"X.%.7sX%s\0" as *const u8 as *const libc::c_char,
            zSlocalname,
            abseq.as_mut_ptr(),
        );
        zname = zsfind_file(absx.as_mut_ptr(), zSlocalname, -(1 as libc::c_int));
        if zname.is_null() {
            return 0 as *mut libc::c_char;
        }
        if fsysdep_file_exists(zname) == 0 {
            break;
        }
        ubuffree(zname);
    }
    return zname;
}

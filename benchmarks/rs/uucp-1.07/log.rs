use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut iDebug: libc::c_int;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn uuconf_logfile(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzlog: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_statsfile(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzstats: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debugfile(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_error_string(
        uuconf_pglobal: *mut libc::c_void,
        ierror: libc::c_int,
        zbuf: *mut libc::c_char,
        cbuf: UUCONF_SIZE_T,
    ) -> libc::c_int;
    fn usysdep_exit(fsuccess: boolean);
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn usysdep_localtime(itime: libc::c_long, q: *mut tm);
    fn esysdep_fopen(
        zfile: *const libc::c_char,
        fpublic: boolean,
        fappend: boolean,
        fmkdirs: boolean,
    ) -> *mut FILE;
    fn zsysdep_base_name(zfile: *const libc::c_char) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type UUCONF_SIZE_T = size_t;
pub static mut log_rcsid: [libc::c_char; 48] = unsafe {
    *::std::mem::transmute::<
        &[u8; 48],
        &[libc::c_char; 48],
    >(b"$Id: log.c,v 1.65 2002/03/05 19:10:41 ian Rel $\0")
};
pub static mut zProgram: *const libc::c_char = 0 as *const libc::c_char;
static mut zLogfile: *const libc::c_char = 0 as *const libc::c_char;
static mut pfLfatal: Option::<unsafe extern "C" fn() -> ()> = None;
static mut fLfile: boolean = 0;
static mut iLid: libc::c_int = 0;
static mut zLuser: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zLsystem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut zLdevice: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut eLlog: *mut FILE = 0 as *const FILE as *mut FILE;
static mut fLlog_tried: boolean = 0;
static mut zLdebugfile: *const libc::c_char = 0 as *const libc::c_char;
static mut eLdebug: *mut FILE = 0 as *const FILE as *mut FILE;
static mut fLdebug_tried: boolean = 0;
static mut zLstatsfile: *const libc::c_char = 0 as *const libc::c_char;
static mut eLstats: *mut FILE = 0 as *const FILE as *mut FILE;
static mut fLstats_tried: boolean = 0;
pub static mut afSignal: [sig_atomic_t; 5] = [0; 5];
pub static mut afLog_signal: [sig_atomic_t; 5] = [0; 5];
pub static mut fLog_sighup: boolean = 1 as libc::c_int;
static mut azSignal_names: [*const libc::c_char; 5] = [
    b"hangup\0" as *const u8 as *const libc::c_char,
    b"interrupt\0" as *const u8 as *const libc::c_char,
    b"quit\0" as *const u8 as *const libc::c_char,
    b"termination\0" as *const u8 as *const libc::c_char,
    b"SIGPIPE\0" as *const u8 as *const libc::c_char,
];
pub static mut pfLstart: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pfLend: Option::<unsafe extern "C" fn() -> ()> = None;
pub unsafe extern "C" fn ulog_fatal_fn(mut pfn: Option::<unsafe extern "C" fn() -> ()>) {
    pfLfatal = pfn;
}
pub unsafe extern "C" fn ulog_to_file(mut puuconf: pointer, mut ffile: boolean) {
    let mut iuuconf: libc::c_int = 0;
    iuuconf = uuconf_logfile(puuconf, &mut zLogfile);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_debugfile(puuconf, &mut zLdebugfile);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_statsfile(puuconf, &mut zLstatsfile);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    fLfile = ffile;
}
pub unsafe extern "C" fn ulog_id(mut i: libc::c_int) {
    iLid = i;
}
pub unsafe extern "C" fn ulog_user(mut zuser: *const libc::c_char) {
    ubuffree(zLuser);
    zLuser = zbufcpy(zuser);
}
pub unsafe extern "C" fn ulog_system(mut zsystem: *const libc::c_char) {
    if zsystem.is_null() || zLsystem.is_null()
        || strcmp(zsystem, zLsystem) != 0 as libc::c_int
    {
        ubuffree(zLsystem);
        zLsystem = zbufcpy(zsystem);
    }
}
pub unsafe extern "C" fn ulog_device(mut zdevice: *const libc::c_char) {
    ubuffree(zLdevice);
    zLdevice = zbufcpy(zdevice);
}
#[inline]
unsafe extern "C" fn zstpcpy(
    mut zto: *mut libc::c_char,
    mut zfrom: *const libc::c_char,
) -> *mut libc::c_char {
    loop {
        let fresh0 = zfrom;
        zfrom = zfrom.offset(1);
        let fresh1 = zto;
        zto = zto.offset(1);
        *fresh1 = *fresh0;
        if !(*fresh1 as libc::c_int != '\0' as i32) {
            break;
        }
    }
    return zto.offset(-(1 as libc::c_int as isize));
}
pub unsafe extern "C" fn ulog(
    mut ttype: tlog,
    mut zmsg: *const libc::c_char,
    mut args: ...
) {
    let mut parg: ::std::ffi::VaListImpl;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut edebug: *mut FILE = 0 as *mut FILE;
    let mut fstart: boolean = 0;
    let mut fend: boolean = 0;
    let mut zhdr: *const libc::c_char = 0 as *const libc::c_char;
    let mut zprefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zformat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut fdoing_sigs: boolean = 0;
    if fdoing_sigs == 0 {
        let mut isig: libc::c_int = 0;
        fdoing_sigs = 1 as libc::c_int;
        isig = 0 as libc::c_int;
        while isig < 5 as libc::c_int {
            if afLog_signal[isig as usize] != 0 {
                ::std::ptr::write_volatile(
                    &mut afLog_signal[isig as usize] as *mut sig_atomic_t,
                    0 as libc::c_int,
                );
                if isig != 0 as libc::c_int && isig != 1 as libc::c_int
                    || fLog_sighup != 0
                {
                    ulog(
                        LOG_ERROR,
                        b"Got %s signal\0" as *const u8 as *const libc::c_char,
                        azSignal_names[isig as usize],
                    );
                }
            }
            isig += 1;
            isig;
        }
        fdoing_sigs = 0 as libc::c_int;
    }
    if fLfile != 0 && eLdebug.is_null() && fLdebug_tried == 0
        && iDebug != 0 as libc::c_int
    {
        fLdebug_tried = 1 as libc::c_int;
        eLdebug = esysdep_fopen(
            zLdebugfile,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        );
    }
    if fLfile == 0 {
        e = stderr;
    } else if ttype as libc::c_int >= LOG_DEBUG as libc::c_int {
        e = eLdebug;
        if e.is_null() {
            return;
        }
    } else {
        if eLlog.is_null() && fLlog_tried == 0 {
            let mut zprint: *const libc::c_char = 0 as *const libc::c_char;
            fLlog_tried = 1 as libc::c_int;
            eLlog = esysdep_fopen(
                zLogfile,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            zprint = zLogfile;
            if eLlog.is_null() {
                fprintf(
                    stderr,
                    b"%s: %s: can not open log file: %s\n\0" as *const u8
                        as *const libc::c_char,
                    zProgram,
                    zprint,
                    strerror(*__errno_location()),
                );
                if pfLfatal.is_some() {
                    (Some(pfLfatal.unwrap())).unwrap()();
                }
                usysdep_exit(0 as libc::c_int);
            }
        }
        e = eLlog;
        if e.is_null() {
            return;
        }
    }
    if zmsg.is_null() {
        return;
    }
    if pfLstart.is_some() {
        (Some(pfLstart.unwrap())).unwrap()();
    }
    edebug = 0 as *mut FILE;
    if (ttype as libc::c_int) < LOG_DEBUG as libc::c_int {
        edebug = eLdebug;
    }
    fstart = 1 as libc::c_int;
    fend = 1 as libc::c_int;
    match ttype as libc::c_uint {
        0 => {
            zhdr = b"\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            zhdr = b"ERROR: \0" as *const u8 as *const libc::c_char;
        }
        2 => {
            zhdr = b"FATAL: \0" as *const u8 as *const libc::c_char;
        }
        3 => {
            zhdr = b"DEBUG: \0" as *const u8 as *const libc::c_char;
        }
        4 => {
            zhdr = b"DEBUG: \0" as *const u8 as *const libc::c_char;
            fend = 0 as libc::c_int;
        }
        5 => {
            zhdr = 0 as *const libc::c_char;
            fstart = 0 as libc::c_int;
            fend = 0 as libc::c_int;
        }
        6 => {
            zhdr = 0 as *const libc::c_char;
            fstart = 0 as libc::c_int;
        }
        _ => {
            zhdr = b"???: \0" as *const u8 as *const libc::c_char;
        }
    }
    if fstart == 0 {
        zprefix = zbufcpy(b"\0" as *const u8 as *const libc::c_char);
    } else if fLfile == 0 {
        zprefix = zbufalc(
            (strlen(zProgram)).wrapping_add(3 as libc::c_int as libc::c_ulong),
        );
        sprintf(zprefix, b"%s: \0" as *const u8 as *const libc::c_char, zProgram);
    } else {
        zprefix = zbufalc(
            (strlen(zProgram))
                .wrapping_add(
                    (if zLsystem.is_null() {
                        1 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(zLsystem)
                    }),
                )
                .wrapping_add(
                    (if zLuser.is_null() {
                        4 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(zLuser)
                    }),
                )
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong,
                )
                .wrapping_add(strlen(zhdr))
                .wrapping_add(100 as libc::c_int as libc::c_ulong),
        );
        zset = zprefix;
        let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
        zbase = zsysdep_base_name(zProgram);
        if zbase.is_null() {
            zbase = zbufcpy(zProgram);
        }
        zset = zstpcpy(zset, zbase);
        let fresh2 = zset;
        zset = zset.offset(1);
        *fresh2 = ' ' as i32 as libc::c_char;
        ubuffree(zbase);
        zset = zstpcpy(
            zset,
            if zLsystem.is_null() {
                b"-\0" as *const u8 as *const libc::c_char
            } else {
                zLsystem as *const libc::c_char
            },
        );
        let fresh3 = zset;
        zset = zset.offset(1);
        *fresh3 = ' ' as i32 as libc::c_char;
        zset = zstpcpy(
            zset,
            if zLuser.is_null() {
                b"-\0" as *const u8 as *const libc::c_char
            } else {
                zLuser as *const libc::c_char
            },
        );
        let fresh4 = zset;
        zset = zset.offset(1);
        *fresh4 = ' ' as i32 as libc::c_char;
        let fresh5 = zset;
        zset = zset.offset(1);
        *fresh5 = '(' as i32 as libc::c_char;
        zset = zstpcpy(zset, zldate_and_time());
        if iLid != 0 as libc::c_int {
            sprintf(zset, b" %d\0" as *const u8 as *const libc::c_char, iLid);
            zset = zset.offset(strlen(zset) as isize);
        }
        let fresh6 = zset;
        zset = zset.offset(1);
        *fresh6 = ')' as i32 as libc::c_char;
        let fresh7 = zset;
        zset = zset.offset(1);
        *fresh7 = ' ' as i32 as libc::c_char;
        strcpy(zset, zhdr);
    }
    zformat = zbufalc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(strlen(zprefix))
            .wrapping_add(strlen(zmsg))
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    );
    zset = zformat;
    zfrom = zprefix;
    while *zfrom as libc::c_int != '\0' as i32 {
        if *zfrom as libc::c_int == '%' as i32 {
            let fresh8 = zset;
            zset = zset.offset(1);
            *fresh8 = '%' as i32 as libc::c_char;
        }
        let fresh9 = zfrom;
        zfrom = zfrom.offset(1);
        let fresh10 = zset;
        zset = zset.offset(1);
        *fresh10 = *fresh9;
    }
    ubuffree(zprefix);
    zset = zstpcpy(zset, zmsg);
    if fend != 0 {
        let fresh11 = zset;
        zset = zset.offset(1);
        *fresh11 = '\n' as i32 as libc::c_char;
        *zset = '\0' as i32 as libc::c_char;
    }
    parg = args.clone();
    vfprintf(e, zformat, parg.as_va_list());
    if !edebug.is_null() {
        parg = args.clone();
        vfprintf(edebug, zformat, parg.as_va_list());
    }
    ubuffree(zformat);
    fflush(e);
    if !edebug.is_null() {
        fflush(edebug);
    }
    if pfLend.is_some() {
        (Some(pfLend.unwrap())).unwrap()();
    }
    if ttype as libc::c_uint == LOG_FATAL as libc::c_int as libc::c_uint {
        if pfLfatal.is_some() {
            (Some(pfLfatal.unwrap())).unwrap()();
        }
        usysdep_exit(0 as libc::c_int);
    }
}
pub unsafe extern "C" fn ulog_uuconf(
    mut ttype: tlog,
    mut puuconf: pointer,
    mut iuuconf: libc::c_int,
) {
    let mut ab: [libc::c_char; 512] = [0; 512];
    uuconf_error_string(
        puuconf,
        iuuconf,
        ab.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    ulog(ttype, b"%s\0" as *const u8 as *const libc::c_char, ab.as_mut_ptr());
}
pub unsafe extern "C" fn ulog_close() {
    ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
    if !eLlog.is_null() {
        fclose(eLlog);
        eLlog = 0 as *mut FILE;
        fLlog_tried = 0 as libc::c_int;
    }
    if !eLdebug.is_null() {
        fclose(eLdebug);
        eLdebug = 0 as *mut FILE;
        fLdebug_tried = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn ustats(
    mut fsucceeded: boolean,
    mut zuser: *const libc::c_char,
    mut zsystem: *const libc::c_char,
    mut fsent: boolean,
    mut cbytes: libc::c_long,
    mut csecs: libc::c_long,
    mut cmicros: libc::c_long,
    mut fcaller: boolean,
) {
    let mut cbps: libc::c_long = 0;
    if cmicros < 0 as libc::c_int as libc::c_long {
        csecs -= -cmicros / 1000000 as libc::c_long + 1 as libc::c_int as libc::c_long;
        cmicros = 1000000 as libc::c_long - -cmicros % 1000000 as libc::c_long;
    }
    if cmicros >= 1000000 as libc::c_long {
        csecs += cmicros / 10000000 as libc::c_long;
        cmicros = cmicros % 1000000 as libc::c_long;
    }
    if csecs == 0 as libc::c_int as libc::c_long
        && cmicros < 1000 as libc::c_int as libc::c_long
    {
        cbps = 0 as libc::c_int as libc::c_long;
    } else {
        let mut cmillis: libc::c_long = 0;
        let mut cdiv: libc::c_long = 0;
        let mut crem: libc::c_long = 0;
        cmillis = csecs * 1000 as libc::c_int as libc::c_long
            + cmicros / 1000 as libc::c_int as libc::c_long;
        cdiv = cbytes / cmillis * 1000 as libc::c_int as libc::c_long;
        crem = cbytes % cmillis * 1000 as libc::c_int as libc::c_long;
        cbps = cdiv + crem / cmillis;
        if cmillis < 0 as libc::c_int as libc::c_long
            || cdiv < 0 as libc::c_int as libc::c_long
            || crem < 0 as libc::c_int as libc::c_long
            || cbps < 0 as libc::c_int as libc::c_long
        {
            cbps = cbytes
                / (csecs
                    + (if cmicros > 500000 as libc::c_long {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_long);
        }
    }
    if eLstats.is_null() {
        if fLstats_tried != 0 {
            return;
        }
        fLstats_tried = 1 as libc::c_int;
        eLstats = esysdep_fopen(
            zLstatsfile,
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        );
        if eLstats.is_null() {
            return;
        }
    }
    fprintf(
        eLstats,
        b"%s %s (%s) %s%s %ld bytes in %ld.%03ld seconds (%ld bytes/sec) on port %s\n\0"
            as *const u8 as *const libc::c_char,
        zuser,
        zsystem,
        zldate_and_time(),
        if fsucceeded != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"failed after \0" as *const u8 as *const libc::c_char
        },
        if fsent != 0 {
            b"sent\0" as *const u8 as *const libc::c_char
        } else {
            b"received\0" as *const u8 as *const libc::c_char
        },
        cbytes,
        csecs,
        cmicros / 1000 as libc::c_int as libc::c_long,
        cbps,
        if zLdevice.is_null() {
            b"unknown\0" as *const u8 as *const libc::c_char
        } else {
            zLdevice as *const libc::c_char
        },
    );
    fflush(eLstats);
}
pub unsafe extern "C" fn ustats_close() {
    if !eLstats.is_null() {
        if fclose(eLstats) != 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"fclose: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        eLstats = 0 as *mut FILE;
        fLstats_tried = 0 as libc::c_int;
    }
}
unsafe extern "C" fn zldate_and_time() -> *const libc::c_char {
    let mut isecs: libc::c_long = 0;
    let mut imicros: libc::c_long = 0;
    let mut s: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    static mut ab: [libc::c_char; 23] = [0; 23];
    isecs = ixsysdep_time(&mut imicros);
    usysdep_localtime(isecs, &mut s);
    sprintf(
        ab.as_mut_ptr(),
        b"%04d-%02d-%02d %02d:%02d:%02d.%02d\0" as *const u8 as *const libc::c_char,
        s.tm_year + 1900 as libc::c_int,
        s.tm_mon + 1 as libc::c_int,
        s.tm_mday,
        s.tm_hour,
        s.tm_min,
        s.tm_sec,
        (imicros / 10000 as libc::c_int as libc::c_long) as libc::c_int,
    );
    return ab.as_mut_ptr();
}

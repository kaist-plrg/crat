use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    static mut iDebug: libc::c_int;
    fn fparse_cmd(zcmd: *mut libc::c_char, qcmd: *mut scmd) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    fn xfree(_: pointer);
    fn uuconf_grade_cmp(uuconf_b1: libc::c_int, uuconf_b2: libc::c_int) -> libc::c_int;
    fn fsysdep_file_exists(zfile: *const libc::c_char) -> boolean;
    fn zsysdep_spool_file_name(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        pseq: pointer,
    ) -> *mut libc::c_char;
    fn fsysdep_move_file(
        zorig: *const libc::c_char,
        zto: *const libc::c_char,
        fmkdirs: boolean,
        fpublic: boolean,
        fcheck: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut zSspooldir: *const libc::c_char;
    fn zsfile_to_jobid(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        bgrade: libc::c_int,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type size_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
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
pub type constpointer = *const libc::c_void;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssfilename {
    pub zfile: *mut libc::c_char,
    pub bgrade: libc::c_char,
    pub bdummy: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssline {
    pub zline: *mut libc::c_char,
    pub qfile: *mut ssfile,
    pub ztemp: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssfile {
    pub zfile: *mut libc::c_char,
    pub bgrade: libc::c_char,
    pub bdummy: libc::c_char,
    pub clines: libc::c_int,
    pub cdid: libc::c_int,
    pub aslines: [ssline; 10],
}
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *mut libc::c_void;
        __comparison = (Some(__compar.unwrap())).unwrap()(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
pub static mut work_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: work.c,v 1.24 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asSwork_files: *mut ssfilename = 0 as *const ssfilename as *mut ssfilename;
static mut cSwork_files: size_t = 0;
static mut iSwork_file: size_t = 0;
static mut qSwork_file: *mut ssfile = 0 as *const ssfile as *mut ssfile;
unsafe extern "C" fn zswork_directory(
    mut zsystem: *const libc::c_char,
) -> *mut libc::c_char {
    return zsysdep_in_dir(zsystem, b"C.\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fswork_file(
    mut zsystem: *const libc::c_char,
    mut zfile: *const libc::c_char,
    mut pbgrade: *mut libc::c_char,
) -> boolean {
    *pbgrade = *zfile.offset(2 as libc::c_int as isize);
    return (*zfile.offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32
        && *zfile.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *zfile.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32)
        as libc::c_int;
}
unsafe extern "C" fn iswork_cmp(
    mut pkey: constpointer,
    mut pdatum: constpointer,
) -> libc::c_int {
    let mut qkey: *const ssfilename = pkey as *const ssfilename;
    let mut qdatum: *const ssfilename = pdatum as *const ssfilename;
    return strcmp((*qkey).zfile, (*qdatum).zfile);
}
pub unsafe extern "C" fn fsysdep_has_work(mut qsys: *const uuconf_system) -> boolean {
    let mut zdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qdir: *mut DIR = 0 as *mut DIR;
    let mut qentry: *mut dirent = 0 as *mut dirent;
    zdir = zswork_directory((*qsys).uuconf_zname);
    if zdir.is_null() {
        return 0 as libc::c_int;
    }
    qdir = opendir(zdir);
    if qdir.is_null() {
        ubuffree(zdir);
        return 0 as libc::c_int;
    }
    loop {
        qentry = readdir(qdir);
        if qentry.is_null() {
            break;
        }
        let mut bgrade: libc::c_char = 0;
        if fswork_file(
            (*qsys).uuconf_zname,
            ((*qentry).d_name).as_mut_ptr(),
            &mut bgrade,
        ) != 0
        {
            closedir(qdir);
            ubuffree(zdir);
            return 1 as libc::c_int;
        }
    }
    closedir(qdir);
    ubuffree(zdir);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_get_work_init(
    mut qsys: *const uuconf_system,
    mut bgrade: libc::c_int,
    mut cmax: libc::c_uint,
) -> boolean {
    let mut zdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qdir: *mut DIR = 0 as *mut DIR;
    let mut qentry: *mut dirent = 0 as *mut dirent;
    let mut chad: size_t = 0;
    let mut callocated: size_t = 0;
    zdir = zswork_directory((*qsys).uuconf_zname);
    if zdir.is_null() {
        return 0 as libc::c_int;
    }
    qdir = opendir(zdir);
    if qdir.is_null() {
        let mut fret: boolean = 0;
        if *__errno_location() == 2 as libc::c_int {
            fret = 1 as libc::c_int;
        } else {
            ulog(
                LOG_ERROR,
                b"opendir (%s): %s\0" as *const u8 as *const libc::c_char,
                zdir,
                strerror(*__errno_location()),
            );
            fret = 0 as libc::c_int;
        }
        ubuffree(zdir);
        return fret;
    }
    chad = cSwork_files;
    callocated = cSwork_files;
    if chad > 0 as libc::c_int as libc::c_ulong {
        qsort(
            asSwork_files as pointer,
            chad,
            ::std::mem::size_of::<ssfilename>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(constpointer, constpointer) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(iswork_cmp),
                ),
            ),
        );
    }
    loop {
        qentry = readdir(qdir);
        if qentry.is_null() {
            break;
        }
        let mut bfilegrade: libc::c_char = 0;
        let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut slook: ssfilename = ssfilename {
            zfile: 0 as *mut libc::c_char,
            bgrade: 0,
            bdummy: 0,
        };
        zname = zbufcpy(((*qentry).d_name).as_mut_ptr());
        slook.zfile = zname;
        if fswork_file(
            (*qsys).uuconf_zname,
            ((*qentry).d_name).as_mut_ptr(),
            &mut bfilegrade,
        ) == 0 || uuconf_grade_cmp(bgrade, bfilegrade as libc::c_int) < 0 as libc::c_int
            || !asSwork_files.is_null()
                && !(bsearch(
                    &mut slook as *mut ssfilename as pointer as *const libc::c_void,
                    asSwork_files as pointer as *const libc::c_void,
                    chad,
                    ::std::mem::size_of::<ssfilename>() as libc::c_ulong,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> libc::c_int>,
                        __compar_fn_t,
                    >(
                        Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn(
                                    constpointer,
                                    constpointer,
                                ) -> libc::c_int,
                                unsafe extern "C" fn() -> libc::c_int,
                            >(iswork_cmp),
                        ),
                    ),
                ))
                    .is_null()
        {
            ubuffree(zname);
        } else {
            if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fsysdep_get_work_init: Found %s\0" as *const u8
                        as *const libc::c_char,
                    zname,
                );
            }
            if cSwork_files >= callocated {
                callocated = (callocated as libc::c_ulong)
                    .wrapping_add(10 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
                asSwork_files = xrealloc(
                    asSwork_files as pointer,
                    callocated
                        .wrapping_mul(
                            ::std::mem::size_of::<ssfilename>() as libc::c_ulong,
                        ),
                ) as *mut ssfilename;
            }
            let ref mut fresh0 = (*asSwork_files.offset(cSwork_files as isize)).zfile;
            *fresh0 = zname;
            (*asSwork_files.offset(cSwork_files as isize)).bgrade = bfilegrade;
            cSwork_files = cSwork_files.wrapping_add(1);
            cSwork_files;
            if cmax != 0 as libc::c_int as libc::c_uint
                && cSwork_files.wrapping_sub(chad) > cmax as libc::c_ulong
            {
                break;
            }
        }
    }
    closedir(qdir);
    ubuffree(zdir);
    if cSwork_files > iSwork_file {
        qsort(
            asSwork_files.offset(iSwork_file as isize) as pointer,
            cSwork_files.wrapping_sub(iSwork_file),
            ::std::mem::size_of::<ssfilename>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(constpointer, constpointer) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(iswork_cmp),
                ),
            ),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsysdep_get_work(
    mut qsys: *const uuconf_system,
    mut bgrade: libc::c_int,
    mut cmax: libc::c_uint,
    mut qcmd: *mut scmd,
) -> boolean {
    let mut zdir: *mut libc::c_char = 0 as *mut libc::c_char;
    if !qSwork_file.is_null() && (*qSwork_file).cdid >= (*qSwork_file).clines {
        qSwork_file = 0 as *mut ssfile;
    }
    if asSwork_files.is_null() {
        (*qcmd).bcmd = 'H' as i32 as libc::c_char;
        return 1 as libc::c_int;
    }
    zdir = 0 as *mut libc::c_char;
    loop {
        while qSwork_file.is_null() {
            let mut e: *mut FILE = 0 as *mut FILE;
            let mut qfile: *mut ssfile = 0 as *mut ssfile;
            let mut iline: libc::c_int = 0;
            let mut callocated: libc::c_int = 0;
            let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cline: size_t = 0;
            let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut bfilegrade: libc::c_char = 0;
            loop {
                if iSwork_file >= cSwork_files {
                    (*qcmd).bcmd = 'H' as i32 as libc::c_char;
                    ubuffree(zdir);
                    return 1 as libc::c_int;
                }
                if zdir.is_null() {
                    zdir = zswork_directory((*qsys).uuconf_zname);
                    if zdir.is_null() {
                        return 0 as libc::c_int;
                    }
                }
                zname = zsysdep_in_dir(
                    zdir,
                    (*asSwork_files.offset(iSwork_file as isize)).zfile,
                );
                bfilegrade = (*asSwork_files.offset(iSwork_file as isize)).bgrade;
                iSwork_file = iSwork_file.wrapping_add(1);
                iSwork_file;
                e = fopen(zname, b"r\0" as *const u8 as *const libc::c_char);
                if e.is_null() {
                    ulog(
                        LOG_ERROR,
                        b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
                        zname,
                        strerror(*__errno_location()),
                    );
                    ubuffree(zname);
                }
                if !e.is_null() {
                    break;
                }
            }
            qfile = xmalloc(::std::mem::size_of::<ssfile>() as libc::c_ulong)
                as *mut ssfile;
            callocated = 10 as libc::c_int;
            iline = 0 as libc::c_int;
            zline = 0 as *mut libc::c_char;
            cline = 0 as libc::c_int as size_t;
            while getline(&mut zline, &mut cline, e) > 0 as libc::c_int as libc::c_long {
                if iline >= callocated {
                    qfile = xrealloc(
                        qfile as pointer,
                        (::std::mem::size_of::<ssfile>() as libc::c_ulong)
                            .wrapping_add(
                                (callocated as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<ssline>() as libc::c_ulong,
                                    ),
                            ),
                    ) as *mut ssfile;
                    callocated += 10 as libc::c_int;
                }
                (*qfile).aslines[iline as usize].zline = zbufcpy(zline);
                (*qfile).aslines[iline as usize].qfile = 0 as *mut ssfile;
                (*qfile).aslines[iline as usize].ztemp = 0 as *mut libc::c_char;
                iline += 1;
                iline;
            }
            xfree(zline as pointer);
            if fclose(e) != 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"fclose: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            if iline == 0 as libc::c_int {
                (*qfile)
                    .aslines[0 as libc::c_int as usize]
                    .zline = zbufcpy(b"P\0" as *const u8 as *const libc::c_char);
                (*qfile).aslines[0 as libc::c_int as usize].qfile = 0 as *mut ssfile;
                (*qfile)
                    .aslines[0 as libc::c_int as usize]
                    .ztemp = 0 as *mut libc::c_char;
                iline = 1 as libc::c_int;
            }
            (*qfile).zfile = zname;
            (*qfile).bgrade = bfilegrade;
            (*qfile).clines = iline;
            (*qfile).cdid = 0 as libc::c_int;
            qSwork_file = qfile;
        }
        loop {
            let mut iline_0: libc::c_int = 0;
            if (*qSwork_file).cdid >= (*qSwork_file).clines {
                qSwork_file = 0 as *mut ssfile;
                break;
            } else {
                iline_0 = (*qSwork_file).cdid;
                (*qSwork_file).cdid += 1;
                (*qSwork_file).cdid;
                if fparse_cmd((*qSwork_file).aslines[iline_0 as usize].zline, qcmd) == 0
                {
                    ulog(
                        LOG_ERROR,
                        b"Bad line in command file %s\0" as *const u8
                            as *const libc::c_char,
                        (*qSwork_file).zfile,
                    );
                    ubuffree((*qSwork_file).aslines[iline_0 as usize].zline);
                    (*qSwork_file)
                        .aslines[iline_0 as usize]
                        .zline = 0 as *mut libc::c_char;
                } else {
                    (*qcmd).bgrade = (*qSwork_file).bgrade;
                    (*qSwork_file).aslines[iline_0 as usize].qfile = qSwork_file;
                    (*qcmd)
                        .pseq = &mut *((*qSwork_file).aslines)
                        .as_mut_ptr()
                        .offset(iline_0 as isize) as *mut ssline as pointer;
                    if (*qcmd).bcmd as libc::c_int == 'S' as i32
                        || (*qcmd).bcmd as libc::c_int == 'E' as i32
                    {
                        let mut zreal: *mut libc::c_char = 0 as *mut libc::c_char;
                        zreal = zsysdep_spool_file_name(
                            qsys,
                            (*qcmd).ztemp,
                            (*qcmd).pseq,
                        );
                        if zreal.is_null() {
                            ubuffree((*qSwork_file).aslines[iline_0 as usize].zline);
                            (*qSwork_file)
                                .aslines[iline_0 as usize]
                                .zline = 0 as *mut libc::c_char;
                            ubuffree(zdir);
                            return 0 as libc::c_int;
                        }
                        (*qSwork_file).aslines[iline_0 as usize].ztemp = zreal;
                    }
                    ubuffree(zdir);
                    return 1 as libc::c_int;
                }
            }
        }
    };
}
pub unsafe extern "C" fn fsysdep_did_work(mut pseq: pointer) -> boolean {
    let mut qfile: *mut ssfile = 0 as *mut ssfile;
    let mut qline: *mut ssline = 0 as *mut ssline;
    let mut i: libc::c_int = 0;
    qline = pseq as *mut ssline;
    ubuffree((*qline).zline);
    (*qline).zline = 0 as *mut libc::c_char;
    qfile = (*qline).qfile;
    (*qline).qfile = 0 as *mut ssfile;
    if !((*qline).ztemp).is_null() {
        remove((*qline).ztemp);
        ubuffree((*qline).ztemp);
        (*qline).ztemp = 0 as *mut libc::c_char;
    }
    if (*qfile).cdid < (*qfile).clines {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*qfile).clines {
        if !((*qfile).aslines[i as usize].qfile).is_null() {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if remove((*qfile).zfile) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
            (*qfile).zfile,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    ubuffree((*qfile).zfile);
    xfree(qfile as pointer);
    if qfile == qSwork_file {
        qSwork_file = 0 as *mut ssfile;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn usysdep_get_work_free(mut qsys: *const uuconf_system) {
    if !asSwork_files.is_null() {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < cSwork_files {
            ubuffree(
                (*asSwork_files.offset(i as isize)).zfile as pointer as *mut libc::c_char,
            );
            i = i.wrapping_add(1);
            i;
        }
        xfree(asSwork_files as pointer);
        asSwork_files = 0 as *mut ssfilename;
        cSwork_files = 0 as libc::c_int as size_t;
        iSwork_file = 0 as libc::c_int as size_t;
    }
    if !qSwork_file.is_null() {
        let mut i_0: libc::c_int = 0;
        ubuffree((*qSwork_file).zfile);
        i_0 = 0 as libc::c_int;
        while i_0 < (*qSwork_file).cdid {
            ubuffree((*qSwork_file).aslines[i_0 as usize].zline);
            ubuffree((*qSwork_file).aslines[i_0 as usize].ztemp);
            i_0 += 1;
            i_0;
        }
        i_0 = (*qSwork_file).cdid;
        while i_0 < (*qSwork_file).clines {
            ubuffree((*qSwork_file).aslines[i_0 as usize].zline);
            i_0 += 1;
            i_0;
        }
        xfree(qSwork_file as pointer);
        qSwork_file = 0 as *mut ssfile;
    }
}
pub unsafe extern "C" fn zsysdep_save_temp_file(
    mut pseq: pointer,
) -> *const libc::c_char {
    let mut qline: *mut ssline = pseq as *mut ssline;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zslash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwant: size_t = 0;
    static mut zbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut cbuf: size_t = 0;
    if fsysdep_file_exists((*qline).ztemp) == 0 {
        return 0 as *const libc::c_char;
    }
    zslash = strrchr((*qline).ztemp, '/' as i32);
    if zslash.is_null() {
        zslash = (*qline).ztemp;
    } else {
        zslash = zslash.offset(1);
        zslash;
    }
    zto = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_add(strlen(zslash)),
    );
    sprintf(
        zto,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        b".Preserve\0" as *const u8 as *const libc::c_char,
        zslash,
    );
    if fsysdep_move_file(
        (*qline).ztemp,
        zto,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
    ) == 0
    {
        ubuffree(zto);
        return b"Could not move file to preservation directory\0" as *const u8
            as *const libc::c_char;
    }
    cwant = (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
        .wrapping_add(strlen(zSspooldir))
        .wrapping_add(strlen(zto));
    if cwant > cbuf {
        ubuffree(zbuf);
        zbuf = zbufalc(cwant);
        cbuf = cwant;
    }
    sprintf(
        zbuf,
        b"File saved as\n\t%s/%s\0" as *const u8 as *const libc::c_char,
        zSspooldir,
        zto,
    );
    ubuffree(zto);
    return zbuf;
}
pub unsafe extern "C" fn zsysdep_jobid(
    mut qsys: *const uuconf_system,
    mut pseq: pointer,
) -> *mut libc::c_char {
    return zsfile_to_jobid(qsys, (*(*(pseq as *mut ssline)).qfile).zfile, bsgrade(pseq));
}
pub unsafe extern "C" fn bsgrade(mut pseq: pointer) -> libc::c_int {
    let mut zfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut bgrade: libc::c_char = 0;
    if pseq.is_null() {
        return -(1 as libc::c_int);
    }
    zfile = (*(*(pseq as *mut ssline)).qfile).zfile;
    bgrade = *(strrchr(zfile, '/' as i32)).offset(3 as libc::c_int as isize);
    return bgrade as libc::c_int;
}

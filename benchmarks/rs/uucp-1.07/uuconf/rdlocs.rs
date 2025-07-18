use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn uuconf_cmd_line(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_z: *mut libc::c_char,
        uuconf_qtab: *const uuconf_cmdtab,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_pfiunknownfn: uuconf_cmdtabfn,
        uuconf_iflags: libc::c_int,
        pblock: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_getline(
        qglobal: *mut sglobal,
        _: *mut *mut libc::c_char,
        _: *mut size_t,
        _: *mut FILE,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type UUCONF_POINTER = *mut libc::c_void;
pub type UUCONF_SIZE_T = size_t;
pub type uuconf_cmdtabfn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::c_int,
        *mut *mut libc::c_char,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_cmdtab {
    pub uuconf_zcmd: *const libc::c_char,
    pub uuconf_itype: libc::c_int,
    pub uuconf_pvar: UUCONF_POINTER,
    pub uuconf_pifn: uuconf_cmdtabfn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sglobal {
    pub qprocess: *mut sprocess,
    pub pblock: pointer,
    pub ierrno: libc::c_int,
    pub zfilename: *const libc::c_char,
    pub ilineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sprocess {
    pub zlocalname: *const libc::c_char,
    pub zspooldir: *const libc::c_char,
    pub zpubdir: *const libc::c_char,
    pub zlockdir: *const libc::c_char,
    pub zlogfile: *const libc::c_char,
    pub zstatsfile: *const libc::c_char,
    pub zdebugfile: *const libc::c_char,
    pub zdebug: *const libc::c_char,
    pub fstrip_login: boolean,
    pub fstrip_proto: boolean,
    pub cmaxuuxqts: libc::c_int,
    pub zrunuuxqt: *const libc::c_char,
    pub fv2: boolean,
    pub fhdb: boolean,
    pub pzdialcodefiles: *mut *mut libc::c_char,
    pub pztimetables: *mut *mut libc::c_char,
    pub zconfigfile: *mut libc::c_char,
    pub pzsysfiles: *mut *mut libc::c_char,
    pub pzportfiles: *mut *mut libc::c_char,
    pub pzdialfiles: *mut *mut libc::c_char,
    pub pzpwdfiles: *mut *mut libc::c_char,
    pub pzcallfiles: *mut *mut libc::c_char,
    pub qunknown: *mut sunknown,
    pub fread_syslocs: boolean,
    pub qsyslocs: *mut stsysloc,
    pub qvalidate: *mut svalidate,
    pub fuses_myname: boolean,
    pub zv2systems: *mut libc::c_char,
    pub zv2devices: *mut libc::c_char,
    pub zv2userfile: *mut libc::c_char,
    pub zv2cmds: *mut libc::c_char,
    pub pzhdb_systems: *mut *mut libc::c_char,
    pub pzhdb_devices: *mut *mut libc::c_char,
    pub pzhdb_dialers: *mut *mut libc::c_char,
    pub fhdb_read_permissions: boolean,
    pub qhdb_permissions: *mut shpermissions,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shpermissions {
    pub qnext: *mut shpermissions,
    pub pzlogname: *mut *mut libc::c_char,
    pub pzmachine: *mut *mut libc::c_char,
    pub frequest: libc::c_int,
    pub fsendfiles: libc::c_int,
    pub pzread: *mut *mut libc::c_char,
    pub pzwrite: *mut *mut libc::c_char,
    pub fcallback: libc::c_int,
    pub pzcommands: *mut *mut libc::c_char,
    pub pzvalidate: *mut *mut libc::c_char,
    pub zmyname: *mut libc::c_char,
    pub zpubdir: *const libc::c_char,
    pub pzalias: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svalidate {
    pub qnext: *mut svalidate,
    pub zlogname: *const libc::c_char,
    pub pzmachines: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stsysloc {
    pub qnext: *mut stsysloc,
    pub zname: *const libc::c_char,
    pub falias: boolean,
    pub zfile: *const libc::c_char,
    pub e: *mut FILE,
    pub iloc: libc::c_long,
    pub ilineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sunknown {
    pub qnext: *mut sunknown,
    pub ilineno: libc::c_int,
    pub cargs: libc::c_int,
    pub pzargs: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sinfo {
    pub zname: *const libc::c_char,
    pub e: *mut FILE,
    pub qlocs: *mut stsysloc,
    pub qvals: *mut svalidate,
}
pub static mut _uuconf_rdlocs_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: rdlocs.c,v 1.10 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asTcmds: [uuconf_cmdtab; 5] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"system\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 2 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    itsystem
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"alias\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 2 as libc::c_int,
                uuconf_pvar: asTcmds.as_ptr() as pointer,
                uuconf_pifn: Some(
                    itsystem
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"called-login\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    itcalled_login
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"myname\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 2 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    itmyname
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: 0 as *const libc::c_char,
                uuconf_itype: 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn _uuconf_iread_locations(
    mut qglobal: *mut sglobal,
) -> libc::c_int {
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut si: sinfo = sinfo {
        zname: 0 as *const libc::c_char,
        e: 0 as *mut FILE,
        qlocs: 0 as *mut stsysloc,
        qvals: 0 as *mut svalidate,
    };
    let mut iret: libc::c_int = 0;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*(*qglobal).qprocess).fread_syslocs != 0 {
        return 0 as libc::c_int;
    }
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    si.qlocs = 0 as *mut stsysloc;
    si.qvals = 0 as *mut svalidate;
    iret = 0 as libc::c_int;
    pz = (*(*qglobal).qprocess).pzsysfiles;
    while !(*pz).is_null() {
        let mut e: *mut FILE = 0 as *mut FILE;
        let mut cchars: libc::c_int = 0;
        (*qglobal).ilineno = 0 as libc::c_int;
        e = fopen(*pz, b"r\0" as *const u8 as *const libc::c_char);
        if e.is_null() {
            if !(*__errno_location() == 2 as libc::c_int) {
                (*qglobal).ierrno = *__errno_location();
                iret = 2 as libc::c_int | 0x100 as libc::c_int;
                break;
            }
        } else {
            let mut cle_i: libc::c_int = fileno(e);
            fcntl(
                cle_i,
                2 as libc::c_int,
                fcntl(cle_i, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
            );
            si.zname = *pz;
            si.e = e;
            loop {
                cchars = _uuconf_getline(qglobal, &mut zline, &mut cline, e);
                if !(cchars > 0 as libc::c_int) {
                    break;
                }
                let mut zcmd: *mut libc::c_char = 0 as *mut libc::c_char;
                (*qglobal).ilineno += 1;
                (*qglobal).ilineno;
                zcmd = zline
                    .offset(
                        strspn(zline, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
                if !(strncasecmp(
                    zcmd,
                    b"system\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                    || strncasecmp(
                        zcmd,
                        b"alias\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int
                    || strncasecmp(
                        zcmd,
                        b"called-login\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int
                    || strncasecmp(
                        zcmd,
                        b"myname\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int)
                {
                    continue;
                }
                iret = uuconf_cmd_line(
                    qglobal as pointer,
                    zline,
                    asTcmds.as_ptr(),
                    &mut si as *mut sinfo as pointer,
                    ::std::mem::transmute::<
                        *mut libc::c_void,
                        uuconf_cmdtabfn,
                    >(0 as *mut libc::c_void),
                    0 as libc::c_int,
                    (*qglobal).pblock,
                );
                if iret & 0x800 as libc::c_int != 0 as libc::c_int {
                    iret &= !(0x800 as libc::c_int);
                    zline = 0 as *mut libc::c_char;
                    cline = 0 as libc::c_int as size_t;
                }
                if !(iret != 0 as libc::c_int) {
                    continue;
                }
                iret &= !(0x1000 as libc::c_int);
                break;
            }
            if iret != 0 as libc::c_int {
                break;
            }
        }
        pz = pz.offset(1);
        pz;
    }
    if !zline.is_null() {
        free(zline as pointer);
    }
    if iret != 0 as libc::c_int {
        (*qglobal).zfilename = *pz;
        iret |= 0x200 as libc::c_int | 0x400 as libc::c_int;
        if iret & 0xff as libc::c_int != 4 as libc::c_int {
            (*(*qglobal).qprocess).fread_syslocs = 1 as libc::c_int;
        }
    } else {
        (*(*qglobal).qprocess).qsyslocs = si.qlocs;
        (*(*qglobal).qprocess).qvalidate = si.qvals;
        (*(*qglobal).qprocess).fread_syslocs = 1 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn itsystem(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut q: *mut stsysloc = 0 as *mut stsysloc;
    let mut csize: size_t = 0;
    q = uuconf_malloc(
        (*qglobal).pblock,
        ::std::mem::size_of::<stsysloc>() as libc::c_ulong,
    ) as *mut stsysloc;
    if q.is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    csize = (strlen(*argv.offset(1 as libc::c_int as isize)))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*q).zname = uuconf_malloc((*qglobal).pblock, csize) as *const libc::c_char;
    if ((*q).zname).is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    (*q).qnext = (*qinfo).qlocs;
    memcpy(
        (*q).zname as pointer,
        *argv.offset(1 as libc::c_int as isize) as pointer as *const libc::c_void,
        csize,
    );
    (*q).falias = (pvar != 0 as *mut libc::c_void) as libc::c_int;
    (*q).zfile = (*qinfo).zname;
    (*q).e = (*qinfo).e;
    (*q).iloc = ftell((*qinfo).e);
    (*q).ilineno = (*qglobal).ilineno;
    (*qinfo).qlocs = q;
    return 0 as libc::c_int;
}
unsafe extern "C" fn itcalled_login(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut qval: *mut svalidate = 0 as *mut svalidate;
    let mut i: libc::c_int = 0;
    if argc <= 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    qval = (*qinfo).qvals;
    while !qval.is_null() {
        if strcmp(*argv.offset(1 as libc::c_int as isize), (*qval).zlogname)
            == 0 as libc::c_int
        {
            break;
        }
        qval = (*qval).qnext;
    }
    if qval.is_null() {
        qval = uuconf_malloc(
            (*qglobal).pblock,
            ::std::mem::size_of::<svalidate>() as libc::c_ulong,
        ) as *mut svalidate;
        if qval.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
        }
        (*qval).qnext = (*qinfo).qvals;
        (*qval).zlogname = *argv.offset(1 as libc::c_int as isize);
        (*qval).pzmachines = 0 as *mut *mut libc::c_char;
        (*qinfo).qvals = qval;
    }
    i = 2 as libc::c_int;
    while i < argc {
        let mut iret: libc::c_int = 0;
        iret = _uuconf_iadd_string(
            qglobal,
            *argv.offset(i as isize),
            0 as libc::c_int,
            1 as libc::c_int,
            &mut (*qval).pzmachines,
            (*qglobal).pblock,
        );
        if iret != 0 as libc::c_int {
            return iret | 0x1000 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn itmyname(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    (*(*qglobal).qprocess).fuses_myname = 1 as libc::c_int;
    return 0 as libc::c_int;
}

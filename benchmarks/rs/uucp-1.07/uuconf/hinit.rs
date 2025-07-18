use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn _uuconf_istrsplit(
        zline: *mut libc::c_char,
        bsep: libc::c_int,
        ppzsplit: *mut *mut *mut libc::c_char,
        csplit: *mut size_t,
    ) -> libc::c_int;
    fn _uuconf_getline(
        qglobal: *mut sglobal,
        _: *mut *mut libc::c_char,
        _: *mut size_t,
        _: *mut FILE,
    ) -> libc::c_int;
    fn uuconf_free(uuconf_pblock: *mut libc::c_void, uuconf_pfree: *mut libc::c_void);
    fn _uuconf_iinit_global(pqglobal: *mut *mut sglobal) -> libc::c_int;
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
pub type UUCONF_SIZE_T = size_t;
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
pub struct sglobal {
    pub qprocess: *mut sprocess,
    pub pblock: pointer,
    pub ierrno: libc::c_int,
    pub zfilename: *const libc::c_char,
    pub ilineno: libc::c_int,
}
pub static mut _uuconf_hinit_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: hinit.c,v 1.9 2002/03/05 19:10:42 ian Rel $\0")
};
static mut abHoldconfiglib: [libc::c_char; 14] = unsafe {
    *::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"/usr/lib/uucp\0")
};
pub unsafe extern "C" fn uuconf_hdb_init(
    mut ppglobal: *mut pointer,
    mut zprogram: *const libc::c_char,
) -> libc::c_int {
    let mut pqglobal: *mut *mut sglobal = ppglobal as *mut *mut sglobal;
    let mut iret: libc::c_int = 0;
    let mut qglobal: *mut sglobal = 0 as *mut sglobal;
    let mut pblock: pointer = 0 as *mut libc::c_void;
    let mut abdialcodes: [libc::c_char; 24] = [0; 24];
    let mut zsys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut FILE = 0 as *mut FILE;
    if (*pqglobal).is_null() {
        iret = _uuconf_iinit_global(pqglobal);
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    qglobal = *pqglobal;
    pblock = (*qglobal).pblock;
    if zprogram.is_null()
        || strcmp(zprogram, b"uucp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        zprogram = b"uucico\0" as *const u8 as *const libc::c_char;
    }
    memcpy(
        abdialcodes.as_mut_ptr() as pointer,
        abHoldconfiglib.as_ptr() as pointer as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        abdialcodes
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize)) as pointer,
        b"/Dialcodes\0" as *const u8 as *const libc::c_char as pointer
            as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
    );
    iret = _uuconf_iadd_string(
        qglobal,
        abdialcodes.as_mut_ptr(),
        1 as libc::c_int,
        0 as libc::c_int,
        &mut (*(*qglobal).qprocess).pzdialcodefiles,
        pblock,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    zsys = uuconf_malloc(
        pblock,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if zsys.is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int;
    }
    memcpy(
        zsys as pointer,
        abHoldconfiglib.as_ptr() as pointer as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        zsys
            .offset(
                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize)) as pointer,
        b"/Sysfiles\0" as *const u8 as *const libc::c_char as pointer
            as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
    );
    iret = 0 as libc::c_int;
    e = fopen(zsys, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        uuconf_free(pblock, zsys as *mut libc::c_void);
    } else {
        let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cline: size_t = 0;
        let mut pzargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut cargs: size_t = 0;
        let mut pzcolon: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut ccolon: size_t = 0;
        let mut cchars: libc::c_int = 0;
        zline = 0 as *mut libc::c_char;
        cline = 0 as libc::c_int as size_t;
        pzargs = 0 as *mut *mut libc::c_char;
        cargs = 0 as libc::c_int as size_t;
        pzcolon = 0 as *mut *mut libc::c_char;
        ccolon = 0 as libc::c_int as size_t;
        (*qglobal).ilineno = 0 as libc::c_int;
        while iret == 0 as libc::c_int
            && {
                cchars = _uuconf_getline(qglobal, &mut zline, &mut cline, e);
                cchars > 0 as libc::c_int
            }
        {
            let mut ctypes: libc::c_int = 0;
            let mut cnames: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            (*qglobal).ilineno += 1;
            (*qglobal).ilineno;
            cchars -= 1;
            cchars;
            if *zline.offset(cchars as isize) as libc::c_int == '\n' as i32 {
                *zline.offset(cchars as isize) = '\0' as i32 as libc::c_char;
            }
            if *zline.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
                continue;
            }
            ctypes = _uuconf_istrsplit(zline, '\0' as i32, &mut pzargs, &mut cargs);
            if ctypes < 0 as libc::c_int {
                (*qglobal).ierrno = *__errno_location();
                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                break;
            } else {
                if ctypes == 0 as libc::c_int {
                    continue;
                }
                if strncmp(
                    *pzargs.offset(0 as libc::c_int as isize),
                    b"service=\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int
                {
                    iret = 5 as libc::c_int;
                    break;
                } else {
                    let ref mut fresh0 = *pzargs.offset(0 as libc::c_int as isize);
                    *fresh0 = (*fresh0)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    cnames = _uuconf_istrsplit(
                        *pzargs.offset(0 as libc::c_int as isize),
                        ':' as i32,
                        &mut pzcolon,
                        &mut ccolon,
                    );
                    if cnames < 0 as libc::c_int {
                        (*qglobal).ierrno = *__errno_location();
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    } else {
                        i = 0 as libc::c_int;
                        while i < cnames {
                            if strcmp(zprogram, *pzcolon.offset(i as isize))
                                == 0 as libc::c_int
                            {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        if i >= cnames {
                            continue;
                        }
                        i = 1 as libc::c_int;
                        while i < ctypes && iret == 0 as libc::c_int {
                            let mut ppz: *mut *mut *mut libc::c_char = 0
                                as *mut *mut *mut libc::c_char;
                            let mut cfiles: libc::c_int = 0;
                            let mut ifile: libc::c_int = 0;
                            if strncmp(
                                *pzargs.offset(i as isize),
                                b"systems=\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                            {
                                ppz = &mut (*(*qglobal).qprocess).pzhdb_systems;
                                let ref mut fresh1 = *pzargs.offset(i as isize);
                                *fresh1 = (*fresh1)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 9]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    );
                            } else if strncmp(
                                *pzargs.offset(i as isize),
                                b"devices=\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                            {
                                ppz = &mut (*(*qglobal).qprocess).pzhdb_devices;
                                let ref mut fresh2 = *pzargs.offset(i as isize);
                                *fresh2 = (*fresh2)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 9]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    );
                            } else if strncmp(
                                *pzargs.offset(i as isize),
                                b"dialers=\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                            {
                                ppz = &mut (*(*qglobal).qprocess).pzhdb_dialers;
                                let ref mut fresh3 = *pzargs.offset(i as isize);
                                *fresh3 = (*fresh3)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 9]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    );
                            } else {
                                iret = 5 as libc::c_int;
                                break;
                            }
                            cfiles = _uuconf_istrsplit(
                                *pzargs.offset(i as isize),
                                ':' as i32,
                                &mut pzcolon,
                                &mut ccolon,
                            );
                            if cfiles < 0 as libc::c_int {
                                (*qglobal).ierrno = *__errno_location();
                                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                                break;
                            } else {
                                ifile = 0 as libc::c_int;
                                while ifile < cfiles && iret == 0 as libc::c_int {
                                    if *(*pzcolon.offset(ifile as isize))
                                        .offset(0 as libc::c_int as isize) as libc::c_int
                                        == '/' as i32
                                    {
                                        iret = _uuconf_iadd_string(
                                            qglobal,
                                            *pzcolon.offset(ifile as isize),
                                            1 as libc::c_int,
                                            0 as libc::c_int,
                                            ppz,
                                            pblock,
                                        );
                                    } else {
                                        let mut zdir: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut clen: size_t = 0;
                                        clen = strlen(*pzcolon.offset(ifile as isize));
                                        zdir = uuconf_malloc(
                                            pblock,
                                            (::std::mem::size_of::<[libc::c_char; 14]>()
                                                as libc::c_ulong)
                                                .wrapping_add(
                                                    ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                                                )
                                                .wrapping_add(clen)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ) as *mut libc::c_char;
                                        if zdir.is_null() {
                                            (*qglobal).ierrno = *__errno_location();
                                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                                            break;
                                        } else {
                                            memcpy(
                                                zdir as pointer,
                                                abHoldconfiglib.as_ptr() as pointer as *const libc::c_void,
                                                (::std::mem::size_of::<[libc::c_char; 14]>()
                                                    as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                            );
                                            memcpy(
                                                zdir
                                                    .offset(
                                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                                            as isize,
                                                    )
                                                    .offset(-(1 as libc::c_int as isize)) as pointer,
                                                b"/\0" as *const u8 as *const libc::c_char
                                                    as *const libc::c_void,
                                                (::std::mem::size_of::<[libc::c_char; 2]>()
                                                    as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                            );
                                            memcpy(
                                                zdir
                                                    .offset(
                                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                                            as isize,
                                                    )
                                                    .offset(-(1 as libc::c_int as isize))
                                                    .offset(
                                                        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                                            as isize,
                                                    )
                                                    .offset(-(1 as libc::c_int as isize)) as pointer,
                                                *pzcolon.offset(ifile as isize) as pointer
                                                    as *const libc::c_void,
                                                clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                            iret = _uuconf_iadd_string(
                                                qglobal,
                                                zdir,
                                                0 as libc::c_int,
                                                0 as libc::c_int,
                                                ppz,
                                                pblock,
                                            );
                                        }
                                    }
                                    ifile += 1;
                                    ifile;
                                }
                                i += 1;
                                i;
                            }
                        }
                    }
                }
            }
        }
        fclose(e);
        if !zline.is_null() {
            free(zline as pointer);
        }
        if !pzargs.is_null() {
            free(pzargs as pointer);
        }
        if !pzcolon.is_null() {
            free(pzcolon as pointer);
        }
        if iret != 0 as libc::c_int {
            (*qglobal).zfilename = zsys;
            return iret | 0x200 as libc::c_int | 0x400 as libc::c_int;
        }
    }
    if ((*(*qglobal).qprocess).pzhdb_systems).is_null() {
        let mut ab: [libc::c_char; 22] = [0; 22];
        memcpy(
            ab.as_mut_ptr() as pointer,
            abHoldconfiglib.as_ptr() as pointer as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            ab
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)) as pointer,
            b"/Systems\0" as *const u8 as *const libc::c_char as pointer
                as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        );
        iret = _uuconf_iadd_string(
            qglobal,
            ab.as_mut_ptr(),
            1 as libc::c_int,
            0 as libc::c_int,
            &mut (*(*qglobal).qprocess).pzhdb_systems,
            pblock,
        );
    }
    if ((*(*qglobal).qprocess).pzhdb_devices).is_null() && iret == 0 as libc::c_int {
        let mut ab_0: [libc::c_char; 22] = [0; 22];
        memcpy(
            ab_0.as_mut_ptr() as pointer,
            abHoldconfiglib.as_ptr() as pointer as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            ab_0
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)) as pointer,
            b"/Devices\0" as *const u8 as *const libc::c_char as pointer
                as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        );
        iret = _uuconf_iadd_string(
            qglobal,
            ab_0.as_mut_ptr(),
            1 as libc::c_int,
            0 as libc::c_int,
            &mut (*(*qglobal).qprocess).pzhdb_devices,
            pblock,
        );
    }
    if ((*(*qglobal).qprocess).pzhdb_dialers).is_null() && iret == 0 as libc::c_int {
        let mut ab_1: [libc::c_char; 22] = [0; 22];
        memcpy(
            ab_1.as_mut_ptr() as pointer,
            abHoldconfiglib.as_ptr() as pointer as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            ab_1
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)) as pointer,
            b"/Dialers\0" as *const u8 as *const libc::c_char as pointer
                as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        );
        iret = _uuconf_iadd_string(
            qglobal,
            ab_1.as_mut_ptr(),
            1 as libc::c_int,
            0 as libc::c_int,
            &mut (*(*qglobal).qprocess).pzhdb_dialers,
            pblock,
        );
    }
    return iret;
}

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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn _uuconf_ucmdtab_base(
        qoff: *const cmdtab_offset,
        celes: size_t,
        pbase: *mut libc::c_char,
        qset: *mut uuconf_cmdtab,
    );
    fn _uuconf_getline(
        qglobal: *mut sglobal,
        _: *mut *mut libc::c_char,
        _: *mut size_t,
        _: *mut FILE,
    ) -> libc::c_int;
    fn _uuconf_istrsplit(
        zline: *mut libc::c_char,
        bsep: libc::c_int,
        ppzsplit: *mut *mut *mut libc::c_char,
        csplit: *mut size_t,
    ) -> libc::c_int;
    fn uuconf_cmd_args(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_cargs: libc::c_int,
        uuconf_pzargs: *mut *mut libc::c_char,
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
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_free(uuconf_pblock: *mut libc::c_void, uuconf_pfree: *mut libc::c_void);
    static mut _uuconf_unset: *mut libc::c_char;
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
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
pub struct cmdtab_offset {
    pub zcmd: *const libc::c_char,
    pub itype: libc::c_int,
    pub ioff: size_t,
    pub pifn: uuconf_cmdtabfn,
}
pub static mut _uuconf_rdperm_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: rdperm.c,v 1.12 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asHperm_cmds: [cmdtab_offset; 15] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"NOREAD\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"NOWRITE\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"LOGNAME\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 8 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"MACHINE\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 16 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"REQUEST\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 24 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"SENDFILES\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 28 as libc::c_ulong,
                pifn: Some(
                    ihsendfiles
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
            let mut init = cmdtab_offset {
                zcmd: b"READ\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 32 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"WRITE\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 40 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"CALLBACK\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 48 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"COMMANDS\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"VALIDATE\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 64 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: b"MYNAME\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 72 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"PUBDIR\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 80 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"ALIAS\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 88 as libc::c_ulong,
                pifn: Some(
                    ihcolon
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
            let mut init = cmdtab_offset {
                zcmd: 0 as *const libc::c_char,
                itype: 0 as libc::c_int,
                ioff: 0 as libc::c_int as size_t,
                pifn: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn _uuconf_ihread_permissions(
    mut qglobal: *mut sglobal,
) -> libc::c_int {
    let mut zperm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut iret: libc::c_int = 0;
    let mut as_0: [uuconf_cmdtab; 15] = [uuconf_cmdtab {
        uuconf_zcmd: 0 as *const libc::c_char,
        uuconf_itype: 0,
        uuconf_pvar: 0 as *mut libc::c_void,
        uuconf_pifn: None,
    }; 15];
    let mut pznoread: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pznowrite: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut shperm: shpermissions = shpermissions {
        qnext: 0 as *mut shpermissions,
        pzlogname: 0 as *mut *mut libc::c_char,
        pzmachine: 0 as *mut *mut libc::c_char,
        frequest: 0,
        fsendfiles: 0,
        pzread: 0 as *mut *mut libc::c_char,
        pzwrite: 0 as *mut *mut libc::c_char,
        fcallback: 0,
        pzcommands: 0 as *mut *mut libc::c_char,
        pzvalidate: 0 as *mut *mut libc::c_char,
        zmyname: 0 as *mut libc::c_char,
        zpubdir: 0 as *const libc::c_char,
        pzalias: 0 as *mut *mut libc::c_char,
    };
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csplit: size_t = 0;
    let mut cchars: libc::c_int = 0;
    let mut qlist: *mut shpermissions = 0 as *mut shpermissions;
    let mut pq: *mut *mut shpermissions = 0 as *mut *mut shpermissions;
    if (*(*qglobal).qprocess).fhdb_read_permissions != 0 {
        return 0 as libc::c_int;
    }
    zperm = uuconf_malloc(
        (*qglobal).pblock,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if zperm.is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int;
    }
    memcpy(
        zperm as pointer,
        b"/usr/lib/uucp\0" as *const u8 as *const libc::c_char as pointer
            as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        zperm
            .offset(
                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize)) as pointer,
        b"/Permissions\0" as *const u8 as *const libc::c_char as pointer
            as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
    );
    e = fopen(zperm, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        uuconf_free((*qglobal).pblock, zperm as *mut libc::c_void);
        (*(*qglobal).qprocess).fhdb_read_permissions = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    _uuconf_ucmdtab_base(
        asHperm_cmds.as_ptr(),
        (::std::mem::size_of::<[cmdtab_offset; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong),
        &mut shperm as *mut shpermissions as *mut libc::c_char,
        as_0.as_mut_ptr(),
    );
    as_0[0 as libc::c_int as usize]
        .uuconf_pvar = &mut pznoread as *mut *mut *mut libc::c_char as pointer;
    as_0[1 as libc::c_int as usize]
        .uuconf_pvar = &mut pznowrite as *mut *mut *mut libc::c_char as pointer;
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    pzsplit = 0 as *mut *mut libc::c_char;
    csplit = 0 as libc::c_int as size_t;
    qlist = 0 as *mut shpermissions;
    pq = &mut qlist;
    (*qglobal).ilineno = 0 as libc::c_int;
    iret = 0 as libc::c_int;
    loop {
        cchars = _uuconf_getline(qglobal, &mut zline, &mut cline, e);
        if !(cchars > 0 as libc::c_int) {
            break;
        }
        let mut centries: libc::c_int = 0;
        let mut qnew: *mut shpermissions = 0 as *mut shpermissions;
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
        centries = _uuconf_istrsplit(zline, '\0' as i32, &mut pzsplit, &mut csplit);
        if centries < 0 as libc::c_int {
            (*qglobal).ierrno = *__errno_location();
            iret = 4 as libc::c_int | 0x100 as libc::c_int;
            break;
        } else {
            if centries == 0 as libc::c_int {
                continue;
            }
            shperm.pzlogname = &mut _uuconf_unset as *mut *mut libc::c_char;
            shperm.pzmachine = &mut _uuconf_unset as *mut *mut libc::c_char;
            shperm.frequest = -(1 as libc::c_int);
            shperm.fsendfiles = -(1 as libc::c_int);
            shperm.pzread = &mut _uuconf_unset as *mut *mut libc::c_char;
            shperm.pzwrite = &mut _uuconf_unset as *mut *mut libc::c_char;
            shperm.fcallback = -(1 as libc::c_int);
            shperm.pzcommands = &mut _uuconf_unset as *mut *mut libc::c_char;
            shperm.pzvalidate = &mut _uuconf_unset as *mut *mut libc::c_char;
            shperm
                .zmyname = &mut _uuconf_unset as *mut *mut libc::c_char
                as *mut libc::c_char;
            shperm
                .zpubdir = &mut _uuconf_unset as *mut *mut libc::c_char
                as *mut libc::c_char;
            shperm.pzalias = &mut _uuconf_unset as *mut *mut libc::c_char;
            pznoread = &mut _uuconf_unset as *mut *mut libc::c_char;
            pznowrite = &mut _uuconf_unset as *mut *mut libc::c_char;
            i = 0 as libc::c_int;
            while i < centries {
                let mut zeq: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut azargs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
                zeq = strchr(*pzsplit.offset(i as isize), '=' as i32);
                if zeq.is_null() {
                    iret = 5 as libc::c_int;
                    (*(*qglobal).qprocess).fhdb_read_permissions = 1 as libc::c_int;
                    break;
                } else {
                    *zeq = '\0' as i32 as libc::c_char;
                    azargs[0 as libc::c_int as usize] = *pzsplit.offset(i as isize);
                    azargs[1 as libc::c_int
                        as usize] = zeq.offset(1 as libc::c_int as isize);
                    iret = uuconf_cmd_args(
                        qglobal as *mut libc::c_void,
                        2 as libc::c_int,
                        azargs.as_mut_ptr(),
                        as_0.as_mut_ptr(),
                        0 as *mut libc::c_void,
                        Some(
                            ihunknownperm
                                as unsafe extern "C" fn(
                                    pointer,
                                    libc::c_int,
                                    *mut *mut libc::c_char,
                                    pointer,
                                    pointer,
                                ) -> libc::c_int,
                        ),
                        0 as libc::c_int,
                        (*qglobal).pblock,
                    );
                    if iret & 0x800 as libc::c_int != 0 as libc::c_int {
                        iret &= !(0x800 as libc::c_int);
                        if uuconf_add_block(
                            (*qglobal).pblock,
                            zline as *mut libc::c_void,
                        ) != 0 as libc::c_int
                        {
                            (*qglobal).ierrno = *__errno_location();
                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            zline = 0 as *mut libc::c_char;
                            cline = 0 as libc::c_int as size_t;
                        }
                    }
                    if iret & 0x1000 as libc::c_int != 0 as libc::c_int {
                        iret &= !(0x1000 as libc::c_int);
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
            }
            if iret != 0 as libc::c_int {
                break;
            }
            if shperm.pzmachine == &mut _uuconf_unset as *mut *mut libc::c_char
                && shperm.pzlogname == &mut _uuconf_unset as *mut *mut libc::c_char
            {
                iret = 5 as libc::c_int;
                (*(*qglobal).qprocess).fhdb_read_permissions = 1 as libc::c_int;
                break;
            } else {
                if !pznoread.is_null() {
                    iret = ihadd_norw(qglobal, &mut shperm.pzread, pznoread);
                    if iret != 0 as libc::c_int {
                        break;
                    }
                    uuconf_free((*qglobal).pblock, pznoread as *mut libc::c_void);
                }
                if !pznowrite.is_null() {
                    iret = ihadd_norw(qglobal, &mut shperm.pzwrite, pznowrite);
                    if iret != 0 as libc::c_int {
                        break;
                    }
                    uuconf_free((*qglobal).pblock, pznowrite as *mut libc::c_void);
                }
                qnew = uuconf_malloc(
                    (*qglobal).pblock,
                    ::std::mem::size_of::<shpermissions>() as libc::c_ulong,
                ) as *mut shpermissions;
                if qnew.is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    iret = 4 as libc::c_int | 0x100 as libc::c_int;
                    break;
                } else {
                    *qnew = shperm;
                    *pq = qnew;
                    pq = &mut (*qnew).qnext;
                    *pq = 0 as *mut shpermissions;
                }
            }
        }
    }
    fclose(e);
    if !zline.is_null() {
        free(zline as pointer);
    }
    if !pzsplit.is_null() {
        free(pzsplit as pointer);
    }
    if iret == 0 as libc::c_int {
        (*(*qglobal).qprocess).qhdb_permissions = qlist;
        (*(*qglobal).qprocess).fhdb_read_permissions = 1 as libc::c_int;
    } else {
        (*qglobal).zfilename = zperm;
        iret |= 0x200 as libc::c_int | 0x400 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn ihcolon(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut ppz: *mut *mut *mut libc::c_char = pvar as *mut *mut *mut libc::c_char;
    let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csplit: size_t = 0;
    let mut centries: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iret: libc::c_int = 0;
    *ppz = 0 as *mut *mut libc::c_char;
    pzsplit = 0 as *mut *mut libc::c_char;
    csplit = 0 as libc::c_int as size_t;
    centries = _uuconf_istrsplit(
        *argv.offset(1 as libc::c_int as isize),
        ':' as i32,
        &mut pzsplit,
        &mut csplit,
    );
    if centries < 0 as libc::c_int {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    if centries == 0 as libc::c_int {
        if !pzsplit.is_null() {
            free(pzsplit as pointer);
        }
        return 0 as libc::c_int;
    }
    iret = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < centries {
        iret = _uuconf_iadd_string(
            qglobal,
            *pzsplit.offset(i as isize),
            0 as libc::c_int,
            0 as libc::c_int,
            ppz,
            (*qglobal).pblock,
        );
        if iret != 0 as libc::c_int {
            iret |= 0x1000 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    free(pzsplit as pointer);
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn ihsendfiles(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut pi: *mut libc::c_int = pvar as *mut libc::c_int;
    match *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int
    {
        67 | 99 | 78 | 110 => {
            *pi = 0 as libc::c_int;
        }
        89 | 121 => {
            *pi = 1 as libc::c_int;
        }
        _ => return 5 as libc::c_int | 0x1000 as libc::c_int,
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ihunknownperm(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    return 5 as libc::c_int | 0x1000 as libc::c_int;
}
unsafe extern "C" fn ihadd_norw(
    mut qglobal: *mut sglobal,
    mut ppz: *mut *mut *mut libc::c_char,
    mut pzno: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if pzno == &mut _uuconf_unset as *mut *mut libc::c_char {
        return 0 as libc::c_int;
    }
    pz = pzno;
    while !(*pz).is_null() {
        let mut csize: size_t = 0;
        let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut iret: libc::c_int = 0;
        if **pz as libc::c_int != '\0' as i32 {
            csize = (strlen(*pz)).wrapping_add(1 as libc::c_int as libc::c_ulong);
            znew = uuconf_malloc(
                (*qglobal).pblock,
                csize.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if znew.is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            *znew.offset(0 as libc::c_int as isize) = '!' as i32 as libc::c_char;
            memcpy(
                znew.offset(1 as libc::c_int as isize) as pointer,
                *pz as pointer as *const libc::c_void,
                csize,
            );
            iret = _uuconf_iadd_string(
                qglobal,
                znew,
                0 as libc::c_int,
                0 as libc::c_int,
                ppz,
                (*qglobal).pblock,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        pz = pz.offset(1);
        pz;
    }
    return 0 as libc::c_int;
}

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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn uuconf_cmd_file(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_e: *mut FILE,
        uuconf_qtab: *const uuconf_cmdtab,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_pfiunknownfn: uuconf_cmdtabfn,
        uuconf_iflags: libc::c_int,
        pblock: *mut libc::c_void,
    ) -> libc::c_int;
    fn _uuconf_itimetable(
        pglobal: pointer,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pvar: pointer,
        pinfo: pointer,
    ) -> libc::c_int;
    fn _uuconf_iinit_global(pqglobal: *mut *mut sglobal) -> libc::c_int;
    fn _uuconf_ucmdtab_base(
        qoff: *const cmdtab_offset,
        celes: size_t,
        pbase: *mut libc::c_char,
        qset: *mut uuconf_cmdtab,
    );
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
pub struct sinfo {
    pub zname: *const libc::c_char,
    pub qcmds: *mut uuconf_cmdtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdtab_offset {
    pub zcmd: *const libc::c_char,
    pub itype: libc::c_int,
    pub ioff: size_t,
    pub pifn: uuconf_cmdtabfn,
}
pub static mut _uuconf_tinit_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: tinit.c,v 1.16 2002/03/05 19:10:43 ian Rel $\0")
};
static mut asCmds: [cmdtab_offset; 26] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"nodename\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 0 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"hostname\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 0 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"uuname\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 0 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"spool\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 8 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"pubdir\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 16 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"lockdir\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 24 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"logfile\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 32 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"statfile\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 40 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"debugfile\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 48 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"debug\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: Some(
                    itdebug
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
                zcmd: b"strip-login\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 64 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"strip-proto\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 68 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"max-uuxqts\0" as *const u8 as *const libc::c_char,
                itype: 0x22 as libc::c_int,
                ioff: 72 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"run-uuxqt\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 80 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"sysfile\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 120 as libc::c_ulong,
                pifn: Some(
                    itaddfile
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
                zcmd: b"portfile\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 128 as libc::c_ulong,
                pifn: Some(
                    itaddfile
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
                zcmd: b"dialfile\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 136 as libc::c_ulong,
                pifn: Some(
                    itaddfile
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
                zcmd: b"dialcodefile\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 96 as libc::c_ulong,
                pifn: Some(
                    itaddfile
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
                zcmd: b"callfile\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 152 as libc::c_ulong,
                pifn: Some(
                    itaddfile
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
                zcmd: b"passwdfile\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 144 as libc::c_ulong,
                pifn: Some(
                    itaddfile
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
                zcmd: b"unknown\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int,
                ioff: 160 as libc::c_ulong,
                pifn: Some(
                    itunknown
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
                zcmd: b"v2-files\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 88 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"hdb-files\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 92 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"bnu-files\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 92 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"timetable\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 104 as libc::c_ulong,
                pifn: Some(
                    _uuconf_itimetable
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
pub unsafe extern "C" fn uuconf_taylor_init(
    mut ppglobal: *mut pointer,
    mut zprogram: *const libc::c_char,
    mut zname: *const libc::c_char,
) -> libc::c_int {
    let mut pqglobal: *mut *mut sglobal = ppglobal as *mut *mut sglobal;
    let mut iret: libc::c_int = 0;
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qglobal: *mut sglobal = 0 as *mut sglobal;
    let mut fdefault: boolean = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut si: sinfo = sinfo {
        zname: 0 as *const libc::c_char,
        qcmds: 0 as *mut uuconf_cmdtab,
    };
    if (*pqglobal).is_null() {
        iret = _uuconf_iinit_global(pqglobal);
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    qglobal = *pqglobal;
    if !zname.is_null() {
        let mut csize: size_t = 0;
        csize = (strlen(zname)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        zcopy = uuconf_malloc((*qglobal).pblock, csize) as *mut libc::c_char;
        if zcopy.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int;
        }
        memcpy(zcopy as pointer, zname as pointer as *const libc::c_void, csize);
        fdefault = 0 as libc::c_int;
    } else {
        zcopy = uuconf_malloc(
            (*qglobal).pblock,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if zcopy.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int;
        }
        memcpy(
            zcopy as pointer,
            b"/usr/conf/uucp\0" as *const u8 as *const libc::c_char as pointer
                as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            zcopy
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)) as pointer,
            b"/config\0" as *const u8 as *const libc::c_char as pointer
                as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        fdefault = 1 as libc::c_int;
    }
    (*(*qglobal).qprocess).zconfigfile = zcopy;
    e = fopen(zcopy, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        if fdefault == 0 {
            (*qglobal).ierrno = *__errno_location();
            (*qglobal).zfilename = zcopy;
            return 2 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int;
        }
    } else {
        let mut as_0: [uuconf_cmdtab; 26] = [uuconf_cmdtab {
            uuconf_zcmd: 0 as *const libc::c_char,
            uuconf_itype: 0,
            uuconf_pvar: 0 as *mut libc::c_void,
            uuconf_pifn: None,
        }; 26];
        _uuconf_ucmdtab_base(
            asCmds.as_ptr(),
            (::std::mem::size_of::<[cmdtab_offset; 26]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong),
            (*qglobal).qprocess as *mut libc::c_char,
            as_0.as_mut_ptr(),
        );
        if zprogram.is_null() {
            zprogram = b"uucp\0" as *const u8 as *const libc::c_char;
        }
        si.zname = zprogram;
        si.qcmds = as_0.as_mut_ptr();
        iret = uuconf_cmd_file(
            qglobal as *mut libc::c_void,
            e,
            as_0.as_mut_ptr(),
            &mut si as *mut sinfo as pointer,
            Some(
                itprogram
                    as unsafe extern "C" fn(
                        pointer,
                        libc::c_int,
                        *mut *mut libc::c_char,
                        pointer,
                        pointer,
                    ) -> libc::c_int,
            ),
            0x2 as libc::c_int,
            (*qglobal).pblock,
        );
        fclose(e);
        if iret != 0 as libc::c_int {
            (*qglobal).zfilename = zcopy;
            return iret | 0x200 as libc::c_int;
        }
    }
    iret = itset_default(
        qglobal,
        &mut (*(*qglobal).qprocess).pzsysfiles,
        b"/sys\0" as *const u8 as *const libc::c_char,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    iret = itset_default(
        qglobal,
        &mut (*(*qglobal).qprocess).pzportfiles,
        b"/port\0" as *const u8 as *const libc::c_char,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    iret = itset_default(
        qglobal,
        &mut (*(*qglobal).qprocess).pzdialfiles,
        b"/dial\0" as *const u8 as *const libc::c_char,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    iret = itset_default(
        qglobal,
        &mut (*(*qglobal).qprocess).pzdialcodefiles,
        b"/dialcode\0" as *const u8 as *const libc::c_char,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    iret = itset_default(
        qglobal,
        &mut (*(*qglobal).qprocess).pzpwdfiles,
        b"/passwd\0" as *const u8 as *const libc::c_char,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    iret = itset_default(
        qglobal,
        &mut (*(*qglobal).qprocess).pzcallfiles,
        b"/call\0" as *const u8 as *const libc::c_char,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn itdebug(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pzdebug: *mut *mut libc::c_char = pvar as *mut *mut libc::c_char;
    return _uuconf_idebug_cmd(qglobal, pzdebug, argc, argv, (*qglobal).pblock);
}
unsafe extern "C" fn itaddfile(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut ppz: *mut *mut *mut libc::c_char = pvar as *mut *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut iret: libc::c_int = 0;
    if argc == 1 as libc::c_int {
        iret = _uuconf_iadd_string(
            qglobal,
            0 as *mut libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            ppz,
            (*qglobal).pblock,
        );
        if iret != 0 as libc::c_int {
            return iret;
        }
    } else {
        i = 1 as libc::c_int;
        while i < argc {
            let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut fallocated: boolean = 0;
            if **argv.offset(i as isize) as libc::c_int == '/' as i32 {
                z = *argv.offset(i as isize);
                fallocated = 0 as libc::c_int;
            } else {
                let mut abs_cdir: size_t = 0;
                let mut abs_cfile: size_t = 0;
                let mut abs_zret: *mut libc::c_char = 0 as *mut libc::c_char;
                abs_cdir = strlen(
                    b"/usr/conf/uucp\0" as *const u8 as *const libc::c_char,
                );
                abs_cfile = strlen(*argv.offset(i as isize));
                abs_zret = uuconf_malloc(
                    (*qglobal).pblock,
                    abs_cdir
                        .wrapping_add(abs_cfile)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                z = abs_zret;
                fallocated = 1 as libc::c_int;
                if !abs_zret.is_null() {
                    memcpy(
                        abs_zret as pointer,
                        b"/usr/conf/uucp\0" as *const u8 as *const libc::c_char
                            as pointer as *const libc::c_void,
                        abs_cdir,
                    );
                    *abs_zret.offset(abs_cdir as isize) = '/' as i32 as libc::c_char;
                    memcpy(
                        abs_zret
                            .offset(abs_cdir as isize)
                            .offset(1 as libc::c_int as isize) as pointer,
                        *argv.offset(i as isize) as pointer as *const libc::c_void,
                        abs_cfile.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                }
            }
            if z.is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                z,
                (fallocated == 0) as libc::c_int,
                0 as libc::c_int,
                ppz,
                (*qglobal).pblock,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn itunknown(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pq: *mut *mut sunknown = pvar as *mut *mut sunknown;
    let mut q: *mut sunknown = 0 as *mut sunknown;
    q = uuconf_malloc(
        (*qglobal).pblock,
        ::std::mem::size_of::<sunknown>() as libc::c_ulong,
    ) as *mut sunknown;
    if q.is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    (*q).qnext = 0 as *mut sunknown;
    (*q).ilineno = (*qglobal).ilineno;
    (*q).cargs = argc - 1 as libc::c_int;
    (*q)
        .pzargs = uuconf_malloc(
        (*qglobal).pblock,
        ((argc - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if ((*q).pzargs).is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    memcpy(
        (*q).pzargs as pointer,
        argv.offset(1 as libc::c_int as isize) as pointer as *const libc::c_void,
        ((argc - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    while !(*pq).is_null() {
        pq = &mut (**pq).qnext;
    }
    *pq = q;
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn itprogram(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    if argc <= 1 as libc::c_int
        || strcasecmp((*qinfo).zname, *argv.offset(0 as libc::c_int as isize))
            != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return uuconf_cmd_args(
        pglobal,
        argc - 1 as libc::c_int,
        argv.offset(1 as libc::c_int as isize),
        (*qinfo).qcmds,
        0 as *mut libc::c_void,
        ::std::mem::transmute::<
            *mut libc::c_void,
            uuconf_cmdtabfn,
        >(0 as *mut libc::c_void),
        0 as libc::c_int,
        (*qglobal).pblock,
    );
}
unsafe extern "C" fn itset_default(
    mut qglobal: *mut sglobal,
    mut ppzvar: *mut *mut *mut libc::c_char,
    mut zfile: *const libc::c_char,
) -> libc::c_int {
    let mut clen: size_t = 0;
    let mut zadd: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*ppzvar).is_null() {
        return 0 as libc::c_int;
    }
    clen = strlen(zfile);
    zadd = uuconf_malloc(
        (*qglobal).pblock,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong).wrapping_add(clen),
    ) as *mut libc::c_char;
    if zadd.is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int;
    }
    memcpy(
        zadd as pointer,
        b"/usr/conf/uucp\0" as *const u8 as *const libc::c_char as pointer
            as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        zadd
            .offset(
                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize)) as pointer,
        zfile as pointer as *const libc::c_void,
        clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return _uuconf_iadd_string(
        qglobal,
        zadd,
        0 as libc::c_int,
        0 as libc::c_int,
        ppzvar,
        (*qglobal).pblock,
    );
}
pub unsafe extern "C" fn _uuconf_idebug_cmd(
    mut qglobal: *mut sglobal,
    mut pzdebug: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pblock: pointer,
) -> libc::c_int {
    if argc == 1 as libc::c_int {
        *pzdebug = 0 as *mut libc::c_char;
        return 0 as libc::c_int;
    } else if argc == 2 as libc::c_int {
        *pzdebug = *argv.offset(1 as libc::c_int as isize);
        return 0x800 as libc::c_int;
    } else {
        let mut cdebug: size_t = 0;
        let mut i: libc::c_int = 0;
        let mut zdebug: *mut libc::c_char = 0 as *mut libc::c_char;
        cdebug = 0 as libc::c_int as size_t;
        i = 1 as libc::c_int;
        while i < argc {
            cdebug = (cdebug as libc::c_ulong)
                .wrapping_add(
                    (strlen(*argv.offset(i as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            i += 1;
            i;
        }
        zdebug = uuconf_malloc(pblock, cdebug) as *mut libc::c_char;
        if zdebug.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
        }
        cdebug = 0 as libc::c_int as size_t;
        i = 1 as libc::c_int;
        while i < argc {
            let mut clen: size_t = 0;
            clen = strlen(*argv.offset(i as isize));
            memcpy(
                zdebug.offset(cdebug as isize) as *mut libc::c_void,
                *argv.offset(i as isize) as *const libc::c_void,
                clen,
            );
            *zdebug
                .offset(cdebug.wrapping_add(clen) as isize) = ' ' as i32 as libc::c_char;
            cdebug = (cdebug as libc::c_ulong)
                .wrapping_add(clen.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            i += 1;
            i;
        }
        *zdebug
            .offset(
                cdebug.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        *pzdebug = zdebug;
        return 0 as libc::c_int;
    };
}

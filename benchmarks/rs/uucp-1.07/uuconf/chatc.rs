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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
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
    fn _uuconf_ucmdtab_base(
        qoff: *const cmdtab_offset,
        celes: size_t,
        pbase: *mut libc::c_char,
        qset: *mut uuconf_cmdtab,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
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
pub static mut _uuconf_chatc_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: chatc.c,v 1.10 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asChat_cmds: [cmdtab_offset; 6] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"chat\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int,
                ioff: 0 as libc::c_ulong,
                pifn: Some(
                    icchat
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
                zcmd: b"chat-program\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 8 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"chat-timeout\0" as *const u8 as *const libc::c_char,
                itype: 0x22 as libc::c_int,
                ioff: 16 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"chat-fail\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 24 as libc::c_ulong,
                pifn: Some(
                    icchat_fail
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
                zcmd: b"chat-seven-bit\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 32 as libc::c_ulong,
                pifn: None,
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
pub unsafe extern "C" fn _uuconf_ichat_cmd(
    mut qglobal: *mut sglobal,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut qchat: *mut uuconf_chat,
    mut pblock: pointer,
) -> libc::c_int {
    let mut zchat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut as_0: [uuconf_cmdtab; 6] = [uuconf_cmdtab {
        uuconf_zcmd: 0 as *const libc::c_char,
        uuconf_itype: 0,
        uuconf_pvar: 0 as *mut libc::c_void,
        uuconf_pifn: None,
    }; 6];
    let mut iret: libc::c_int = 0;
    zchat = *argv.offset(0 as libc::c_int as isize);
    while *zchat as libc::c_int != '\0' as i32 {
        if (*zchat as libc::c_int == 'c' as i32 || *zchat as libc::c_int == 'C' as i32)
            && strncasecmp(
                zchat,
                b"chat\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            break;
        }
        zchat = zchat.offset(1);
        zchat;
    }
    if *zchat as libc::c_int == '\0' as i32 {
        return 5 as libc::c_int;
    }
    let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
    *fresh0 = zchat;
    _uuconf_ucmdtab_base(
        asChat_cmds.as_ptr(),
        (::std::mem::size_of::<[cmdtab_offset; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong),
        qchat as *mut libc::c_char,
        as_0.as_mut_ptr(),
    );
    iret = uuconf_cmd_args(
        qglobal as pointer,
        argc,
        argv,
        as_0.as_mut_ptr(),
        pblock,
        Some(
            icunknown
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0 as libc::c_int,
        pblock,
    );
    if !((*qchat).uuconf_pzprogram).is_null()
        && (*((*qchat).uuconf_pzprogram).offset(0 as libc::c_int as isize)).is_null()
    {
        (*qchat).uuconf_pzprogram = 0 as *mut *mut libc::c_char;
    }
    return iret & !(0x1000 as libc::c_int);
}
unsafe extern "C" fn icchat(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut ppz: *mut *mut *mut libc::c_char = pvar as *mut *mut *mut libc::c_char;
    let mut pblock: pointer = pinfo;
    let mut i: libc::c_int = 0;
    *ppz = 0 as *mut *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zdash: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut iret: libc::c_int = 0;
        z = *argv.offset(i as isize);
        zdash = strchr(z, '-' as i32);
        while !zdash.is_null() {
            *zdash = '\0' as i32 as libc::c_char;
            iret = _uuconf_iadd_string(
                qglobal,
                z,
                1 as libc::c_int,
                0 as libc::c_int,
                ppz,
                pblock,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            *zdash = '-' as i32 as libc::c_char;
            z = zdash;
            zdash = strchr(z.offset(1 as libc::c_int as isize), '-' as i32);
        }
        iret = _uuconf_iadd_string(
            qglobal,
            z,
            0 as libc::c_int,
            0 as libc::c_int,
            ppz,
            pblock,
        );
        if iret != 0 as libc::c_int {
            return iret;
        }
        if (i + 1 as libc::c_int) < argc {
            if *(*argv.offset((i + 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
            {
                iret = _uuconf_iadd_string(
                    qglobal,
                    *argv.offset((i + 1 as libc::c_int) as isize),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    ppz,
                    pblock,
                );
            } else {
                let mut clen: size_t = 0;
                clen = strlen(*argv.offset((i + 1 as libc::c_int) as isize));
                z = uuconf_malloc(
                    pblock,
                    clen.wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if z.is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    return 4 as libc::c_int | 0x100 as libc::c_int;
                }
                *z.offset(0 as libc::c_int as isize) = '\\' as i32 as libc::c_char;
                memcpy(
                    z.offset(1 as libc::c_int as isize) as pointer,
                    *argv.offset((i + 1 as libc::c_int) as isize) as pointer
                        as *const libc::c_void,
                    clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                iret = _uuconf_iadd_string(
                    qglobal,
                    z,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    ppz,
                    pblock,
                );
            }
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        i += 2 as libc::c_int;
    }
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn icchat_fail(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut ppz: *mut *mut *mut libc::c_char = pvar as *mut *mut *mut libc::c_char;
    let mut pblock: pointer = pinfo;
    return _uuconf_iadd_string(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        1 as libc::c_int,
        0 as libc::c_int,
        ppz,
        pblock,
    );
}
unsafe extern "C" fn icunknown(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    return 5 as libc::c_int;
}

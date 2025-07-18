use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_iboolean(
        qglobal: *mut sglobal,
        zval: *const libc::c_char,
        pi: *mut libc::c_int,
    ) -> libc::c_int;
    fn _uuconf_iint(
        qglobal: *mut sglobal,
        zval: *const libc::c_char,
        p: pointer,
        fint: boolean,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
pub const _ISupper: C2RustUnnamed = 256;
pub const _ISlower: C2RustUnnamed = 512;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub static mut _uuconf_cmdarg_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: cmdarg.c,v 1.7 2002/03/05 19:10:42 ian Rel $\0")
};
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn uuconf_cmd_args(
    mut pglobal: pointer,
    mut cargs: libc::c_int,
    mut pzargs: *mut *mut libc::c_char,
    mut qtab: *const uuconf_cmdtab,
    mut pinfo: pointer,
    mut pfiunknown: Option::<
        unsafe extern "C" fn(
            pointer,
            libc::c_int,
            *mut *mut libc::c_char,
            pointer,
            pointer,
        ) -> libc::c_int,
    >,
    mut iflags: libc::c_int,
    mut pblock: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut bfirstu: libc::c_int = 0;
    let mut bfirstl: libc::c_int = 0;
    let mut pficmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    > = None;
    let mut q: *const uuconf_cmdtab = 0 as *const uuconf_cmdtab;
    let mut itype: libc::c_int = 0;
    let mut callowed: libc::c_int = 0;
    bfirstl = *(*pzargs.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) as libc::c_int;
    bfirstu = bfirstl;
    if iflags & 0x1 as libc::c_int != 0 as libc::c_int {
        pficmp = Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    } else {
        if *(*__ctype_b_loc()).offset(bfirstu as isize) as libc::c_int
            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            bfirstu = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = bfirstu;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(bfirstu);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(bfirstu as isize);
                }
                __res
            });
        }
        if *(*__ctype_b_loc()).offset(bfirstl as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            bfirstl = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = bfirstl;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(bfirstl);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(bfirstl as isize);
                }
                __res
            });
        }
        pficmp = Some(
            strcasecmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    }
    itype = 0 as libc::c_int;
    q = qtab;
    while !((*q).uuconf_zcmd).is_null() {
        let mut bfirst: libc::c_int = 0;
        bfirst = *((*q).uuconf_zcmd).offset(0 as libc::c_int as isize) as libc::c_int;
        if !(bfirst != bfirstu && bfirst != bfirstl) {
            itype = (*q).uuconf_itype & 0x70 as libc::c_int;
            if itype != 0x70 as libc::c_int {
                if (Some(pficmp.unwrap()))
                    .unwrap()(
                    (*q).uuconf_zcmd,
                    *pzargs.offset(0 as libc::c_int as isize),
                ) == 0 as libc::c_int
                {
                    break;
                }
            } else {
                let mut clen: size_t = 0;
                clen = strlen((*q).uuconf_zcmd);
                if iflags & 0x1 as libc::c_int != 0 as libc::c_int {
                    if strncmp(
                        (*q).uuconf_zcmd,
                        *pzargs.offset(0 as libc::c_int as isize),
                        clen,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                } else if strncasecmp(
                    (*q).uuconf_zcmd,
                    *pzargs.offset(0 as libc::c_int as isize),
                    clen,
                ) == 0 as libc::c_int
                {
                    break;
                }
            }
        }
        q = q.offset(1);
        q;
    }
    if ((*q).uuconf_zcmd).is_null() {
        if pfiunknown.is_none() {
            return 0 as libc::c_int;
        }
        return (Some(pfiunknown.unwrap()))
            .unwrap()(pglobal, cargs, pzargs, 0 as *mut libc::c_void, pinfo);
    }
    callowed = (*q).uuconf_itype & 0xf as libc::c_int;
    if callowed != 0 as libc::c_int && callowed != cargs {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    match itype {
        64 => {
            if cargs == 1 as libc::c_int {
                let ref mut fresh0 = *((*q).uuconf_pvar as *mut *mut libc::c_char);
                *fresh0 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else if cargs == 2 as libc::c_int {
                let ref mut fresh1 = *((*q).uuconf_pvar as *mut *mut libc::c_char);
                *fresh1 = *pzargs.offset(1 as libc::c_int as isize);
            } else {
                return 5 as libc::c_int | 0x1000 as libc::c_int
            }
            return 0x800 as libc::c_int;
        }
        32 => {
            return _uuconf_iint(
                qglobal,
                *pzargs.offset(1 as libc::c_int as isize),
                (*q).uuconf_pvar,
                1 as libc::c_int,
            );
        }
        48 => {
            return _uuconf_iint(
                qglobal,
                *pzargs.offset(1 as libc::c_int as isize),
                (*q).uuconf_pvar,
                0 as libc::c_int,
            );
        }
        16 => {
            return _uuconf_iboolean(
                qglobal,
                *pzargs.offset(1 as libc::c_int as isize),
                (*q).uuconf_pvar as *mut libc::c_int,
            );
        }
        80 => {
            if cargs == 1 as libc::c_int {
                let mut ppz: *mut *mut *mut libc::c_char = (*q).uuconf_pvar
                    as *mut *mut *mut libc::c_char;
                let mut iret: libc::c_int = 0;
                *ppz = 0 as *mut *mut libc::c_char;
                iret = _uuconf_iadd_string(
                    qglobal,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    ppz,
                    pblock,
                );
                if iret != 0 as libc::c_int {
                    return iret | 0x1000 as libc::c_int;
                }
                return 0 as libc::c_int;
            } else {
                let mut ppz_0: *mut *mut *mut libc::c_char = (*q).uuconf_pvar
                    as *mut *mut *mut libc::c_char;
                let mut i: libc::c_int = 0;
                *ppz_0 = 0 as *mut *mut libc::c_char;
                i = 1 as libc::c_int;
                while i < cargs {
                    let mut iret_0: libc::c_int = 0;
                    iret_0 = _uuconf_iadd_string(
                        qglobal,
                        *pzargs.offset(i as isize),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        ppz_0,
                        pblock,
                    );
                    if iret_0 != 0 as libc::c_int {
                        *ppz_0 = 0 as *mut *mut libc::c_char;
                        return iret_0 | 0x1000 as libc::c_int;
                    }
                    i += 1;
                    i;
                }
                return 0x800 as libc::c_int;
            }
        }
        96 | 112 => {
            return (Some(((*q).uuconf_pifn).unwrap()))
                .unwrap()(pglobal, cargs, pzargs, (*q).uuconf_pvar, pinfo);
        }
        _ => return 5 as libc::c_int | 0x1000 as libc::c_int,
    };
}

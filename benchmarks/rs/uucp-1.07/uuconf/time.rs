use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    static mut _uuconf_unset: *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
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
pub type UUCONF_SIZE_T = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
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
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub zname: *const libc::c_char,
    pub imin: libc::c_int,
    pub imax: libc::c_int,
}
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub static mut _uuconf_time_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: time.c,v 1.14 2002/03/05 19:10:43 ian Rel $\0")
};
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut asTdays: [C2RustUnnamed; 12] = [
    {
        let mut init = C2RustUnnamed {
            zname: b"any\0" as *const u8 as *const libc::c_char,
            imin: 0 as libc::c_int,
            imax: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"wk\0" as *const u8 as *const libc::c_char,
            imin: 1 as libc::c_int,
            imax: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"su\0" as *const u8 as *const libc::c_char,
            imin: 0 as libc::c_int,
            imax: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"mo\0" as *const u8 as *const libc::c_char,
            imin: 1 as libc::c_int,
            imax: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"tu\0" as *const u8 as *const libc::c_char,
            imin: 2 as libc::c_int,
            imax: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"we\0" as *const u8 as *const libc::c_char,
            imin: 3 as libc::c_int,
            imax: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"th\0" as *const u8 as *const libc::c_char,
            imin: 4 as libc::c_int,
            imax: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"fr\0" as *const u8 as *const libc::c_char,
            imin: 5 as libc::c_int,
            imax: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"sa\0" as *const u8 as *const libc::c_char,
            imin: 6 as libc::c_int,
            imax: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"never\0" as *const u8 as *const libc::c_char,
            imin: -(1 as libc::c_int),
            imax: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: b"none\0" as *const u8 as *const libc::c_char,
            imin: -(1 as libc::c_int),
            imax: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            zname: 0 as *const libc::c_char,
            imin: 0 as libc::c_int,
            imax: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn _uuconf_itime_parse(
    mut qglobal: *mut sglobal,
    mut ztime: *mut libc::c_char,
    mut ival: libc::c_long,
    mut cretry: libc::c_int,
    mut picmp: Option::<unsafe extern "C" fn(libc::c_long, libc::c_long) -> libc::c_int>,
    mut pqspan: *mut *mut uuconf_timespan,
    mut pblock: pointer,
) -> libc::c_int {
    let mut qlist: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
    let mut bfirst: libc::c_char = 0;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    qlist = *pqspan;
    if qlist == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan {
        qlist = 0 as *mut uuconf_timespan;
    }
    loop {
        let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut zfound: *mut libc::c_char = 0 as *mut libc::c_char;
        bfirst = *ztime;
        if *(*__ctype_b_loc()).offset(bfirst as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            bfirst = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = bfirst as libc::c_uchar
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(bfirst as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(bfirst as libc::c_uchar as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
        }
        zfound = 0 as *mut libc::c_char;
        pz = (*(*qglobal).qprocess).pztimetables;
        while !(*pz).is_null() {
            if (bfirst as libc::c_int
                == *(*pz).offset(0 as libc::c_int as isize) as libc::c_int
                || *(*__ctype_b_loc())
                    .offset(
                        *(*pz).offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && bfirst as libc::c_int
                        == ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = *(*pz)
                                        .offset(0 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower(
                                        *(*pz).offset(0 as libc::c_int as isize) as libc::c_uchar
                                            as libc::c_int,
                                    );
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(
                                        *(*pz).offset(0 as libc::c_int as isize) as libc::c_uchar
                                            as libc::c_int as isize,
                                    );
                            }
                            __res
                        })) && strcasecmp(ztime, *pz) == 0 as libc::c_int
            {
                zfound = *pz.offset(1 as libc::c_int as isize);
            }
            pz = pz.offset(2 as libc::c_int as isize);
        }
        if zfound.is_null() {
            break;
        }
        ztime = zfound;
    }
    z = ztime;
    while *z as libc::c_int != '\0' as i32 {
        let mut iday: libc::c_int = 0;
        let mut afday: [boolean; 7] = [0; 7];
        let mut istart: libc::c_int = 0;
        let mut iend: libc::c_int = 0;
        if *z as libc::c_int == ',' as i32 || *z as libc::c_int == '|' as i32 {
            z = z.offset(1);
            z;
        }
        if *z as libc::c_int == '\0' as i32 || *z as libc::c_int == ';' as i32 {
            break;
        }
        iday = 0 as libc::c_int;
        while iday < 7 as libc::c_int {
            afday[iday as usize] = 0 as libc::c_int;
            iday += 1;
            iday;
        }
        loop {
            bfirst = *z;
            if *(*__ctype_b_loc())
                .offset(bfirst as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                bfirst = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = bfirst as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(bfirst as libc::c_uchar as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(bfirst as libc::c_uchar as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
            }
            iday = 0 as libc::c_int;
            while !(asTdays[iday as usize].zname).is_null() {
                let mut clen: size_t = 0;
                if !(bfirst as libc::c_int
                    != *(asTdays[iday as usize].zname).offset(0 as libc::c_int as isize)
                        as libc::c_int)
                {
                    clen = strlen(asTdays[iday as usize].zname);
                    if strncasecmp(z, asTdays[iday as usize].zname, clen)
                        == 0 as libc::c_int
                    {
                        let mut iset: libc::c_int = 0;
                        iset = asTdays[iday as usize].imin;
                        while iset <= asTdays[iday as usize].imax {
                            afday[iset as usize] = 1 as libc::c_int;
                            iset += 1;
                            iset;
                        }
                        z = z.offset(clen as isize);
                        break;
                    }
                }
                iday += 1;
                iday;
            }
            if (asTdays[iday as usize].zname).is_null() {
                return 5 as libc::c_int;
            }
            if !(*(*__ctype_b_loc()).offset(*z as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
            {
                break;
            }
        }
        if *(*__ctype_b_loc()).offset(*z as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            istart = 0 as libc::c_int;
            iend = 24 as libc::c_int * 60 as libc::c_int;
        } else {
            let mut zendnum: *mut libc::c_char = 0 as *mut libc::c_char;
            istart = strtol(z as *mut libc::c_char, &mut zendnum, 10 as libc::c_int)
                as libc::c_int;
            if *zendnum as libc::c_int != '-' as i32
                || *(*__ctype_b_loc())
                    .offset(
                        *zendnum.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                return 5 as libc::c_int;
            }
            z = zendnum.offset(1 as libc::c_int as isize);
            iend = strtol(z as *mut libc::c_char, &mut zendnum, 10 as libc::c_int)
                as libc::c_int;
            z = zendnum;
            istart = istart / 100 as libc::c_int * 60 as libc::c_int
                + istart % 100 as libc::c_int;
            iend = iend / 100 as libc::c_int * 60 as libc::c_int
                + iend % 100 as libc::c_int;
        }
        iday = 0 as libc::c_int;
        while iday < 7 as libc::c_int {
            if afday[iday as usize] != 0 {
                let mut iminute: libc::c_int = 0;
                let mut iret: libc::c_int = 0;
                iminute = iday * 24 as libc::c_int * 60 as libc::c_int;
                if istart < iend {
                    iret = itadd_span(
                        qglobal,
                        iminute + istart,
                        iminute + iend,
                        ival,
                        cretry,
                        picmp,
                        &mut qlist,
                        pblock,
                    );
                } else {
                    iret = itadd_span(
                        qglobal,
                        iminute,
                        iminute + iend,
                        ival,
                        cretry,
                        picmp,
                        &mut qlist,
                        pblock,
                    );
                    if iret == 0 as libc::c_int {
                        iret = itadd_span(
                            qglobal,
                            iminute + istart,
                            iminute + 24 as libc::c_int * 60 as libc::c_int,
                            ival,
                            cretry,
                            picmp,
                            &mut qlist,
                            pblock,
                        );
                    }
                }
                if iret != 0 as libc::c_int {
                    return iret;
                }
            }
            iday += 1;
            iday;
        }
    }
    *pqspan = qlist;
    return 0 as libc::c_int;
}
unsafe extern "C" fn itadd_span(
    mut qglobal: *mut sglobal,
    mut istart: libc::c_int,
    mut iend: libc::c_int,
    mut ival: libc::c_long,
    mut cretry: libc::c_int,
    mut picmp: Option::<unsafe extern "C" fn(libc::c_long, libc::c_long) -> libc::c_int>,
    mut pqspan: *mut *mut uuconf_timespan,
    mut pblock: pointer,
) -> libc::c_int {
    let mut pq: *mut *mut uuconf_timespan = 0 as *mut *mut uuconf_timespan;
    let mut iret: libc::c_int = 0;
    pq = pqspan;
    while !(*pq).is_null() {
        let mut icmp: libc::c_int = 0;
        if iend <= (**pq).uuconf_istart {
            if iend == (**pq).uuconf_istart && cretry == (**pq).uuconf_cretry
                && (Some(picmp.unwrap())).unwrap()(ival, (**pq).uuconf_ival)
                    == 0 as libc::c_int
            {
                (**pq).uuconf_istart = istart;
                return 0 as libc::c_int;
            }
            break;
        } else {
            if (**pq).uuconf_iend <= istart {
                if (**pq).uuconf_iend == istart && (**pq).uuconf_cretry == cretry
                    && (((**pq).uuconf_qnext).is_null()
                        || iend <= (*(**pq).uuconf_qnext).uuconf_istart)
                    && (Some(picmp.unwrap())).unwrap()(ival, (**pq).uuconf_ival)
                        == 0 as libc::c_int
                {
                    (**pq).uuconf_iend = iend;
                    return 0 as libc::c_int;
                }
            } else {
                icmp = (Some(picmp.unwrap())).unwrap()(ival, (**pq).uuconf_ival);
                if icmp == 0 as libc::c_int {
                    if istart < (**pq).uuconf_istart {
                        (**pq).uuconf_istart = istart;
                    }
                    if (**pq).uuconf_iend >= iend {
                        return 0 as libc::c_int;
                    }
                    if ((**pq).uuconf_qnext).is_null()
                        || iend <= (*(**pq).uuconf_qnext).uuconf_istart
                    {
                        (**pq).uuconf_iend = iend;
                        return 0 as libc::c_int;
                    }
                    (**pq).uuconf_iend = (*(**pq).uuconf_qnext).uuconf_istart;
                    istart = (**pq).uuconf_iend;
                } else if icmp < 0 as libc::c_int {
                    if (**pq).uuconf_istart < istart {
                        iret = itnew(
                            qglobal,
                            pq,
                            *pq,
                            (**pq).uuconf_istart,
                            istart,
                            (**pq).uuconf_ival,
                            (**pq).uuconf_cretry,
                            pblock,
                        );
                        if iret != 0 as libc::c_int {
                            return iret;
                        }
                        pq = &mut (**pq).uuconf_qnext;
                    }
                    if iend < (**pq).uuconf_iend {
                        iret = itnew(
                            qglobal,
                            &mut (**pq).uuconf_qnext,
                            (**pq).uuconf_qnext,
                            iend,
                            (**pq).uuconf_iend,
                            (**pq).uuconf_ival,
                            (**pq).uuconf_cretry,
                            pblock,
                        );
                        if iret != 0 as libc::c_int {
                            return iret;
                        }
                    }
                    (**pq).uuconf_ival = ival;
                    (**pq).uuconf_istart = istart;
                    (**pq).uuconf_cretry = cretry;
                    if ((**pq).uuconf_qnext).is_null()
                        || iend <= (*(**pq).uuconf_qnext).uuconf_istart
                    {
                        (**pq).uuconf_iend = iend;
                        return 0 as libc::c_int;
                    }
                    (**pq).uuconf_iend = (*(**pq).uuconf_qnext).uuconf_istart;
                    istart = (**pq).uuconf_iend;
                } else {
                    if istart < (**pq).uuconf_istart {
                        iret = itnew(
                            qglobal,
                            pq,
                            *pq,
                            istart,
                            (**pq).uuconf_istart,
                            ival,
                            cretry,
                            pblock,
                        );
                        if iret != 0 as libc::c_int {
                            return iret;
                        }
                        pq = &mut (**pq).uuconf_qnext;
                    }
                    if iend <= (**pq).uuconf_iend {
                        return 0 as libc::c_int;
                    }
                    istart = (**pq).uuconf_iend;
                }
            }
            pq = &mut (**pq).uuconf_qnext;
        }
    }
    return itnew(qglobal, pq, *pq, istart, iend, ival, cretry, pblock);
}
unsafe extern "C" fn itnew(
    mut qglobal: *mut sglobal,
    mut pqset: *mut *mut uuconf_timespan,
    mut qnext: *mut uuconf_timespan,
    mut istart: libc::c_int,
    mut iend: libc::c_int,
    mut ival: libc::c_long,
    mut cretry: libc::c_int,
    mut pblock: pointer,
) -> libc::c_int {
    let mut qnew: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
    qnew = uuconf_malloc(
        pblock,
        ::std::mem::size_of::<uuconf_timespan>() as libc::c_ulong,
    ) as *mut uuconf_timespan;
    if qnew.is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int;
    }
    (*qnew).uuconf_qnext = qnext;
    (*qnew).uuconf_istart = istart;
    (*qnew).uuconf_iend = iend;
    (*qnew).uuconf_ival = ival;
    (*qnew).uuconf_cretry = cretry;
    *pqset = qnew;
    return 0 as libc::c_int;
}

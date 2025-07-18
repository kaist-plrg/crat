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
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn uuconf_free(uuconf_pblock: *mut libc::c_void, uuconf_pfree: *mut libc::c_void);
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
pub static mut _uuconf_paramc_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: paramc.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_iadd_proto_param(
    mut qglobal: *mut sglobal,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pqparam: *mut *mut uuconf_proto_param,
    mut pblock: pointer,
) -> libc::c_int {
    let mut q: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
    let mut c: size_t = 0;
    let mut qentry: *mut uuconf_proto_param_entry = 0 as *mut uuconf_proto_param_entry;
    if argc < 2 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    if *(*argv.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
        as libc::c_int != '\0' as i32
    {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    if (*pqparam).is_null() {
        *pqparam = uuconf_malloc(
            pblock,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<uuconf_proto_param>() as libc::c_ulong,
                ),
        ) as *mut uuconf_proto_param;
        if (*pqparam).is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
        }
        (*(*pqparam).offset(1 as libc::c_int as isize)).uuconf_bproto = '\0' as i32;
        q = *pqparam;
        (*q)
            .uuconf_bproto = *(*argv.offset(0 as libc::c_int as isize))
            .offset(0 as libc::c_int as isize) as libc::c_int;
        (*q).uuconf_qentries = 0 as *mut uuconf_proto_param_entry;
    } else {
        c = 0 as libc::c_int as size_t;
        q = *pqparam;
        while (*q).uuconf_bproto != '\0' as i32 {
            if (*q).uuconf_bproto
                == *(*argv.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int
            {
                break;
            }
            c = c.wrapping_add(1);
            c;
            q = q.offset(1);
            q;
        }
        if (*q).uuconf_bproto == '\0' as i32 {
            let mut qnew: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
            qnew = uuconf_malloc(
                pblock,
                c
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<uuconf_proto_param>() as libc::c_ulong,
                    ),
            ) as *mut uuconf_proto_param;
            if qnew.is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
            }
            memcpy(
                qnew as pointer,
                *pqparam as pointer as *const libc::c_void,
                c
                    .wrapping_mul(
                        ::std::mem::size_of::<uuconf_proto_param>() as libc::c_ulong,
                    ),
            );
            (*qnew.offset(c.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .uuconf_bproto = '\0' as i32;
            uuconf_free(pblock, *pqparam as *mut libc::c_void);
            *pqparam = qnew;
            q = qnew.offset(c as isize);
            (*q)
                .uuconf_bproto = *(*argv.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int;
            (*q).uuconf_qentries = 0 as *mut uuconf_proto_param_entry;
        }
    }
    if ((*q).uuconf_qentries).is_null() {
        qentry = uuconf_malloc(
            pblock,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<uuconf_proto_param_entry>() as libc::c_ulong,
                ),
        ) as *mut uuconf_proto_param_entry;
        if qentry.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
        }
        (*qentry.offset(1 as libc::c_int as isize)).uuconf_cargs = 0 as libc::c_int;
        (*q).uuconf_qentries = qentry;
    } else {
        let mut qnewent: *mut uuconf_proto_param_entry = 0
            as *mut uuconf_proto_param_entry;
        c = 0 as libc::c_int as size_t;
        qentry = (*q).uuconf_qentries;
        while (*qentry).uuconf_cargs != 0 as libc::c_int {
            c = c.wrapping_add(1);
            c;
            qentry = qentry.offset(1);
            qentry;
        }
        qnewent = uuconf_malloc(
            pblock,
            c
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<uuconf_proto_param_entry>() as libc::c_ulong,
                ),
        ) as *mut uuconf_proto_param_entry;
        if qnewent.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
        }
        memcpy(
            qnewent as pointer,
            (*q).uuconf_qentries as pointer as *const libc::c_void,
            c
                .wrapping_mul(
                    ::std::mem::size_of::<uuconf_proto_param_entry>() as libc::c_ulong,
                ),
        );
        (*qnewent.offset(c.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
            .uuconf_cargs = 0 as libc::c_int;
        uuconf_free(pblock, (*q).uuconf_qentries as *mut libc::c_void);
        (*q).uuconf_qentries = qnewent;
        qentry = qnewent.offset(c as isize);
    }
    (*qentry).uuconf_cargs = argc - 1 as libc::c_int;
    (*qentry)
        .uuconf_pzargs = uuconf_malloc(
        pblock,
        ((argc - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if ((*qentry).uuconf_pzargs).is_null() {
        (*qglobal).ierrno = *__errno_location();
        (*qentry).uuconf_cargs = 0 as libc::c_int;
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    memcpy(
        (*qentry).uuconf_pzargs as pointer,
        argv.offset(1 as libc::c_int as isize) as pointer as *const libc::c_void,
        ((argc - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    return 0x800 as libc::c_int;
}

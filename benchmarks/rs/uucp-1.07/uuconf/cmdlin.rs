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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn _uuconf_istrsplit(
        zline: *mut libc::c_char,
        bsep: libc::c_int,
        ppzsplit: *mut *mut *mut libc::c_char,
        csplit: *mut size_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub const _ISspace: C2RustUnnamed = 8192;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub static mut _uuconf_cmdlin_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: cmdlin.c,v 1.7 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_cmd_line(
    mut pglobal: pointer,
    mut zline: *mut libc::c_char,
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
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cargs: libc::c_int = 0;
    let mut azargs: [*mut libc::c_char; 16] = [0 as *mut libc::c_char; 16];
    let mut pzargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut iret: libc::c_int = 0;
    if iflags & 0x4 as libc::c_int == 0 as libc::c_int {
        z = zline;
        loop {
            z = strchr(z, '#' as i32);
            if z.is_null() {
                break;
            }
            if z == zline
                || *z.offset(-(1 as libc::c_int as isize)) as libc::c_int != '\\' as i32
            {
                *z = '\0' as i32 as libc::c_char;
                break;
            } else {
                loop {
                    let ref mut fresh0 = *z.offset(-(1 as libc::c_int as isize));
                    *fresh0 = *z;
                    if !(*fresh0 as libc::c_int != '\0' as i32) {
                        break;
                    }
                    z = z.offset(1);
                    z;
                }
            }
        }
    }
    z = zline;
    cargs = 0 as libc::c_int;
    pzargs = azargs.as_mut_ptr();
    loop {
        while *z as libc::c_int != '\0' as i32
            && *(*__ctype_b_loc()).offset(*z as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            z = z.offset(1);
            z;
        }
        if *z as libc::c_int == '\0' as i32 {
            break;
        }
        if cargs >= 16 as libc::c_int {
            let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut csplit: size_t = 0;
            let mut cmore: libc::c_int = 0;
            pzsplit = 0 as *mut *mut libc::c_char;
            csplit = 0 as libc::c_int as size_t;
            cmore = _uuconf_istrsplit(z, '\0' as i32, &mut pzsplit, &mut csplit);
            if cmore < 0 as libc::c_int {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            pzargs = malloc(
                ((cmore + 16 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if pzargs.is_null() {
                (*qglobal).ierrno = *__errno_location();
                free(pzsplit as pointer);
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            memcpy(
                pzargs as pointer,
                azargs.as_mut_ptr() as pointer as *const libc::c_void,
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            );
            memcpy(
                pzargs.offset(16 as libc::c_int as isize) as pointer,
                pzsplit as pointer as *const libc::c_void,
                (cmore as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            );
            cargs = cmore + 16 as libc::c_int;
            free(pzsplit as pointer);
            break;
        } else {
            azargs[cargs as usize] = z;
            cargs += 1;
            cargs;
            while *z as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(*z as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                z = z.offset(1);
                z;
            }
            if *z as libc::c_int == '\0' as i32 {
                break;
            }
            let fresh1 = z;
            z = z.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
        }
    }
    if cargs <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    iret = uuconf_cmd_args(
        pglobal,
        cargs,
        pzargs,
        qtab,
        pinfo,
        pfiunknown,
        iflags,
        pblock,
    );
    if pzargs != azargs.as_mut_ptr() {
        free(pzargs as pointer);
    }
    return iret;
}

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn free(__ptr: *mut libc::c_void);
    fn _uuconf_getline(
        qglobal: *mut sglobal,
        _: *mut *mut libc::c_char,
        _: *mut size_t,
        _: *mut FILE,
    ) -> libc::c_int;
    fn uuconf_cmd_line(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_z: *mut libc::c_char,
        uuconf_qtab: *const uuconf_cmdtab,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_pfiunknownfn: uuconf_cmdtabfn,
        uuconf_iflags: libc::c_int,
        pblock: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub static mut _uuconf_cmdfil_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: cmdfil.c,v 1.7 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_cmd_file(
    mut pglobal: pointer,
    mut e: *mut FILE,
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
    let mut fcont: boolean = 0;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut iret: libc::c_int = 0;
    fcont = (iflags & 0x2 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    iret = 0 as libc::c_int;
    (*qglobal).ilineno = 0 as libc::c_int;
    while (if fcont != 0 {
        _uuconf_getline(qglobal, &mut zline, &mut cline, e) as libc::c_long
    } else {
        getline(&mut zline, &mut cline, e)
    }) > 0 as libc::c_int as libc::c_long
    {
        (*qglobal).ilineno += 1;
        (*qglobal).ilineno;
        iret = uuconf_cmd_line(pglobal, zline, qtab, pinfo, pfiunknown, iflags, pblock);
        if iret & 0x800 as libc::c_int != 0 as libc::c_int {
            iret &= !(0x800 as libc::c_int);
            if !pblock.is_null() {
                if uuconf_add_block(pblock, zline as *mut libc::c_void)
                    != 0 as libc::c_int
                {
                    (*qglobal).ierrno = *__errno_location();
                    iret = 4 as libc::c_int | 0x100 as libc::c_int
                        | 0x400 as libc::c_int;
                    break;
                }
            }
            zline = 0 as *mut libc::c_char;
            cline = 0 as libc::c_int as size_t;
        }
        if iret & 0x1000 as libc::c_int != 0 as libc::c_int {
            iret &= !(0x1000 as libc::c_int);
            if iret != 0 as libc::c_int {
                iret |= 0x400 as libc::c_int;
            }
            break;
        } else {
            iret = 0 as libc::c_int;
        }
    }
    if !zline.is_null() {
        free(zline as pointer);
    }
    return iret;
}

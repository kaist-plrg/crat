use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(__ptr: *mut libc::c_void);
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn uuconf_taylor_dialer_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzdialers: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
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
pub static mut _uuconf_dnams_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: dnams.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_dialer_names(
    mut pglobal: pointer,
    mut ppzdialers: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pztaylor: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pzhdb: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut iret: libc::c_int = 0;
    *ppzdialers = 0 as *mut *mut libc::c_char;
    pztaylor = 0 as *mut *mut libc::c_char;
    pzhdb = 0 as *mut *mut libc::c_char;
    iret = uuconf_taylor_dialer_names(pglobal, &mut pztaylor);
    if iret != 0 as libc::c_int {
        return iret;
    }
    if pzhdb.is_null() {
        *ppzdialers = pztaylor;
    } else if pztaylor.is_null() {
        *ppzdialers = pzhdb;
    } else {
        let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        iret = 0 as libc::c_int;
        pz = pztaylor;
        while !(*pz).is_null() {
            iret = _uuconf_iadd_string(
                qglobal,
                *pz,
                0 as libc::c_int,
                1 as libc::c_int,
                ppzdialers,
                0 as *mut libc::c_void,
            );
            if iret != 0 as libc::c_int {
                break;
            }
            pz = pz.offset(1);
            pz;
        }
        if iret == 0 as libc::c_int {
            pz = pzhdb;
            while !(*pz).is_null() {
                iret = _uuconf_iadd_string(
                    qglobal,
                    *pz,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    ppzdialers,
                    0 as *mut libc::c_void,
                );
                if iret != 0 as libc::c_int {
                    break;
                }
                pz = pz.offset(1);
                pz;
            }
        }
        if !pztaylor.is_null() {
            free(pztaylor as pointer);
        }
        if !pzhdb.is_null() {
            free(pzhdb as pointer);
        }
    }
    if iret == 0 as libc::c_int && (*ppzdialers).is_null() {
        iret = _uuconf_iadd_string(
            qglobal,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            ppzdialers,
            0 as *mut libc::c_void,
        );
    }
    return iret;
}

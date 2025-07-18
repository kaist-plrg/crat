use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn _uuconf_ichat_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qchat: *mut uuconf_chat,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_uclear_dialer(qdialer: *mut uuconf_dialer);
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_malloc_block() -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
}
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
pub struct uuconf_dialer {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zdialtone: *mut libc::c_char,
    pub uuconf_zpause: *mut libc::c_char,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_ccarrier_wait: libc::c_int,
    pub uuconf_fdtr_toggle: libc::c_int,
    pub uuconf_fdtr_toggle_wait: libc::c_int,
    pub uuconf_scomplete: uuconf_chat,
    pub uuconf_sabort: uuconf_chat,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_palloc: UUCONF_POINTER,
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
pub const _ISspace: C2RustUnnamed = 8192;
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
pub static mut _uuconf_hdial_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: hdial.c,v 1.7 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_hdb_dialer_info(
    mut pglobal: pointer,
    mut zname: *const libc::c_char,
    mut qdialer: *mut uuconf_dialer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csplit: size_t = 0;
    let mut iret: libc::c_int = 0;
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    pzsplit = 0 as *mut *mut libc::c_char;
    csplit = 0 as libc::c_int as size_t;
    iret = 1 as libc::c_int;
    pz = (*(*qglobal).qprocess).pzhdb_dialers;
    while !(*pz).is_null() {
        let mut e: *mut FILE = 0 as *mut FILE;
        let mut cchars: libc::c_int = 0;
        (*qglobal).ilineno = 0 as libc::c_int;
        e = fopen(*pz, b"r\0" as *const u8 as *const libc::c_char);
        if e.is_null() {
            if !(*__errno_location() == 2 as libc::c_int) {
                (*qglobal).ierrno = *__errno_location();
                iret = 2 as libc::c_int | 0x100 as libc::c_int;
                break;
            }
        } else {
            loop {
                cchars = _uuconf_getline(qglobal, &mut zline, &mut cline, e);
                if !(cchars > 0 as libc::c_int) {
                    break;
                }
                let mut ctoks: libc::c_int = 0;
                let mut pblock: pointer = 0 as *mut libc::c_void;
                (*qglobal).ilineno += 1;
                (*qglobal).ilineno;
                cchars -= 1;
                cchars;
                if *zline.offset(cchars as isize) as libc::c_int == '\n' as i32 {
                    *zline.offset(cchars as isize) = '\0' as i32 as libc::c_char;
                }
                if *(*__ctype_b_loc())
                    .offset(
                        *zline.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *zline.offset(0 as libc::c_int as isize) as libc::c_int
                        == '#' as i32
                {
                    continue;
                }
                ctoks = _uuconf_istrsplit(zline, '\0' as i32, &mut pzsplit, &mut csplit);
                if ctoks < 0 as libc::c_int {
                    (*qglobal).ierrno = *__errno_location();
                    iret = 4 as libc::c_int | 0x100 as libc::c_int;
                    break;
                } else {
                    if ctoks < 1 as libc::c_int {
                        continue;
                    }
                    if strcmp(zname, *pzsplit.offset(0 as libc::c_int as isize))
                        != 0 as libc::c_int
                    {
                        continue;
                    }
                    pblock = uuconf_malloc_block();
                    if pblock.is_null() {
                        (*qglobal).ierrno = *__errno_location();
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    } else if uuconf_add_block(pblock, zline as *mut libc::c_void)
                        != 0 as libc::c_int
                    {
                        (*qglobal).ierrno = *__errno_location();
                        uuconf_free_block(pblock);
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    } else {
                        zline = 0 as *mut libc::c_char;
                        _uuconf_uclear_dialer(qdialer);
                        (*qdialer)
                            .uuconf_zname = *pzsplit.offset(0 as libc::c_int as isize);
                        (*qdialer).uuconf_palloc = pblock;
                        if ctoks > 1 as libc::c_int {
                            if strcmp(
                                *pzsplit.offset(1 as libc::c_int as isize),
                                b"\"\"\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                            {
                                let mut zsubs: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut bnext: libc::c_char = 0;
                                zsubs = *pzsplit.offset(1 as libc::c_int as isize);
                                bnext = *zsubs;
                                while bnext as libc::c_int != '\0' as i32 {
                                    if bnext as libc::c_int == '=' as i32 {
                                        (*qdialer)
                                            .uuconf_zdialtone = zsubs.offset(1 as libc::c_int as isize);
                                    } else if bnext as libc::c_int == '-' as i32 {
                                        (*qdialer)
                                            .uuconf_zpause = zsubs.offset(1 as libc::c_int as isize);
                                    }
                                    if *zsubs.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\0' as i32
                                    {
                                        break;
                                    }
                                    zsubs = zsubs.offset(2 as libc::c_int as isize);
                                    bnext = *zsubs;
                                    *zsubs = '\0' as i32 as libc::c_char;
                                }
                            }
                            if ctoks > 2 as libc::c_int {
                                let ref mut fresh0 = *pzsplit
                                    .offset(1 as libc::c_int as isize);
                                *fresh0 = b"chat\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                                iret = _uuconf_ichat_cmd(
                                    qglobal,
                                    ctoks - 1 as libc::c_int,
                                    pzsplit.offset(1 as libc::c_int as isize),
                                    &mut (*qdialer).uuconf_schat,
                                    pblock,
                                );
                                iret &= !(0x800 as libc::c_int);
                                if iret != 0 as libc::c_int {
                                    uuconf_free_block(pblock);
                                    break;
                                }
                            }
                        }
                        iret = 0 as libc::c_int;
                        break;
                    }
                }
            }
            fclose(e);
            if iret != 1 as libc::c_int {
                break;
            }
        }
        pz = pz.offset(1);
        pz;
    }
    if !zline.is_null() {
        free(zline as pointer);
    }
    if !pzsplit.is_null() {
        free(pzsplit as pointer);
    }
    if iret != 0 as libc::c_int && iret != 1 as libc::c_int {
        (*qglobal).zfilename = *pz;
        iret |= 0x200 as libc::c_int | 0x400 as libc::c_int;
    }
    return iret;
}

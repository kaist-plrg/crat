use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_malloc_block() -> *mut libc::c_void;
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn _uuconf_uclear_port(qport: *mut uuconf_port);
    fn _uuconf_istrsplit(
        zline: *mut libc::c_char,
        bsep: libc::c_int,
        ppzsplit: *mut *mut *mut libc::c_char,
        csplit: *mut size_t,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub struct uuconf_port {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_ttype: uuconf_porttype,
    pub uuconf_zprotocols: *mut libc::c_char,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_zlockname: *mut libc::c_char,
    pub uuconf_palloc: UUCONF_POINTER,
    pub uuconf_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uuconf_sstdin: uuconf_stdin_port,
    pub uuconf_smodem: uuconf_modem_port,
    pub uuconf_sdirect: uuconf_direct_port,
    pub uuconf_stcp: uuconf_tcp_port,
    pub uuconf_stli: uuconf_tli_port,
    pub uuconf_spipe: uuconf_pipe_port,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_pipe_port {
    pub uuconf_pzcmd: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_tli_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_fstream: libc::c_int,
    pub uuconf_pzpush: *mut *mut libc::c_char,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
    pub uuconf_zservaddr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_tcp_port {
    pub uuconf_zport: *mut libc::c_char,
    pub uuconf_iversion: libc::c_int,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_direct_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_fhardflow: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_modem_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_zdial_device: *mut libc::c_char,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_ilowbaud: libc::c_long,
    pub uuconf_ihighbaud: libc::c_long,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_fhardflow: libc::c_int,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
    pub uuconf_qdialer: *mut uuconf_dialer,
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
pub struct uuconf_stdin_port {
    pub uuconf_idummy: libc::c_int,
}
pub type uuconf_porttype = libc::c_uint;
pub const UUCONF_PORTTYPE_PIPE: uuconf_porttype = 6;
pub const UUCONF_PORTTYPE_TLI: uuconf_porttype = 5;
pub const UUCONF_PORTTYPE_TCP: uuconf_porttype = 4;
pub const UUCONF_PORTTYPE_DIRECT: uuconf_porttype = 3;
pub const UUCONF_PORTTYPE_MODEM: uuconf_porttype = 2;
pub const UUCONF_PORTTYPE_STDIN: uuconf_porttype = 1;
pub const UUCONF_PORTTYPE_UNKNOWN: uuconf_porttype = 0;
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
pub static mut _uuconf_vport_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: vport.c,v 1.12 2002/03/05 19:10:43 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_v2_find_port(
    mut pglobal: pointer,
    mut zname: *const libc::c_char,
    mut ibaud: libc::c_long,
    mut ihighbaud: libc::c_long,
    mut pifn: Option::<unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int>,
    mut pinfo: pointer,
    mut qport: *mut uuconf_port,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csplit: size_t = 0;
    let mut iret: libc::c_int = 0;
    let mut cchars: libc::c_int = 0;
    e = fopen(
        (*(*qglobal).qprocess).zv2devices,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if e.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            return 1 as libc::c_int;
        }
        (*qglobal).ierrno = *__errno_location();
        (*qglobal).zfilename = (*(*qglobal).qprocess).zv2devices;
        return 2 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int;
    }
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    pzsplit = 0 as *mut *mut libc::c_char;
    csplit = 0 as libc::c_int as size_t;
    iret = 1 as libc::c_int;
    (*qglobal).ilineno = 0 as libc::c_int;
    loop {
        cchars = getline(&mut zline, &mut cline, e) as libc::c_int;
        if !(cchars > 0 as libc::c_int) {
            break;
        }
        let mut ctoks: libc::c_int = 0;
        let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ilow: libc::c_long = 0;
        let mut ihigh: libc::c_long = 0;
        let mut pblock: pointer = 0 as *mut libc::c_void;
        (*qglobal).ilineno += 1;
        (*qglobal).ilineno;
        iret = 1 as libc::c_int;
        cchars -= 1;
        cchars;
        if *zline.offset(cchars as isize) as libc::c_int == '\n' as i32 {
            *zline.offset(cchars as isize) = '\0' as i32 as libc::c_char;
        }
        *zline
            .offset(
                strcspn(zline, b"#\0" as *const u8 as *const libc::c_char) as isize,
            ) = '\0' as i32 as libc::c_char;
        ctoks = _uuconf_istrsplit(zline, '\0' as i32, &mut pzsplit, &mut csplit);
        if ctoks < 0 as libc::c_int {
            (*qglobal).ierrno = *__errno_location();
            iret = 4 as libc::c_int | 0x100 as libc::c_int;
            break;
        } else {
            if ctoks < 4 as libc::c_int {
                continue;
            }
            if !zname.is_null()
                && strcmp(*pzsplit.offset(0 as libc::c_int as isize), zname)
                    != 0 as libc::c_int
            {
                continue;
            }
            ilow = strtol(
                *pzsplit.offset(3 as libc::c_int as isize),
                &mut zend,
                10 as libc::c_int,
            );
            if *zend as libc::c_int == '-' as i32 {
                ihigh = strtol(
                    zend.offset(1 as libc::c_int as isize),
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
            } else {
                ihigh = ilow;
            }
            if ibaud != 0 as libc::c_int as libc::c_long
                && ilow != 0 as libc::c_int as libc::c_long
                && (ilow > ibaud || ihigh < ibaud)
            {
                continue;
            }
            pblock = 0 as *mut libc::c_void;
            _uuconf_uclear_port(qport);
            (*qport).uuconf_zname = *pzsplit.offset(0 as libc::c_int as isize);
            if strcmp(
                *pzsplit.offset(0 as libc::c_int as isize),
                b"DIR\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*qport).uuconf_ttype = UUCONF_PORTTYPE_DIRECT;
                (*qport)
                    .uuconf_u
                    .uuconf_sdirect
                    .uuconf_zdevice = *pzsplit.offset(1 as libc::c_int as isize);
                (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud = ilow;
                (*qport).uuconf_u.uuconf_sdirect.uuconf_fcarrier = 0 as libc::c_int;
                (*qport).uuconf_u.uuconf_sdirect.uuconf_fhardflow = 1 as libc::c_int;
            } else {
                (*qport).uuconf_ttype = UUCONF_PORTTYPE_MODEM;
                (*qport)
                    .uuconf_u
                    .uuconf_smodem
                    .uuconf_zdevice = *pzsplit.offset(1 as libc::c_int as isize);
                if strcmp(
                    *pzsplit.offset(2 as libc::c_int as isize),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    (*qport)
                        .uuconf_u
                        .uuconf_smodem
                        .uuconf_zdial_device = *pzsplit
                        .offset(2 as libc::c_int as isize);
                } else {
                    (*qport)
                        .uuconf_u
                        .uuconf_smodem
                        .uuconf_zdial_device = 0 as *mut libc::c_char;
                }
                if ilow == ihigh {
                    (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud = ilow;
                    (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud = 0 as libc::c_long;
                    (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud = 0 as libc::c_long;
                } else {
                    (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud = 0 as libc::c_long;
                    (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud = ilow;
                    (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud = ihigh;
                }
                (*qport).uuconf_u.uuconf_smodem.uuconf_fcarrier = 1 as libc::c_int;
                (*qport).uuconf_u.uuconf_smodem.uuconf_fhardflow = 1 as libc::c_int;
                if ctoks < 5 as libc::c_int {
                    (*qport)
                        .uuconf_u
                        .uuconf_smodem
                        .uuconf_pzdialer = 0 as *mut *mut libc::c_char;
                } else {
                    let mut c: size_t = 0;
                    let mut pzd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                    pblock = uuconf_malloc_block();
                    if pblock.is_null() {
                        (*qglobal).ierrno = *__errno_location();
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    } else {
                        c = ((ctoks - 4 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            );
                        pzd = uuconf_malloc(
                            pblock,
                            c
                                .wrapping_add(
                                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                        ) as *mut *mut libc::c_char;
                        if pzd.is_null() {
                            (*qglobal).ierrno = *__errno_location();
                            uuconf_free_block(pblock);
                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            memcpy(
                                pzd as pointer,
                                pzsplit.offset(4 as libc::c_int as isize) as pointer
                                    as *const libc::c_void,
                                c,
                            );
                            let ref mut fresh0 = *pzd
                                .offset((ctoks - 4 as libc::c_int) as isize);
                            *fresh0 = 0 as *mut libc::c_char;
                            (*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer = pzd;
                        }
                    }
                }
                (*qport).uuconf_u.uuconf_smodem.uuconf_qdialer = 0 as *mut uuconf_dialer;
            }
            if pifn.is_some() {
                iret = (Some(pifn.unwrap())).unwrap()(qport, pinfo);
                if iret != 0 as libc::c_int {
                    if !pblock.is_null() {
                        uuconf_free_block(pblock);
                    }
                    if iret != 1 as libc::c_int {
                        break;
                    } else {
                        continue;
                    }
                }
            }
            if pblock.is_null() {
                pblock = uuconf_malloc_block();
                if pblock.is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    iret = 4 as libc::c_int | 0x100 as libc::c_int;
                    break;
                }
            }
            if uuconf_add_block(pblock, zline as *mut libc::c_void) != 0 as libc::c_int {
                (*qglobal).ierrno = *__errno_location();
                uuconf_free_block(pblock);
                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                break;
            } else {
                zline = 0 as *mut libc::c_char;
                (*qport).uuconf_palloc = pblock;
                break;
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
    if iret != 0 as libc::c_int && iret != 1 as libc::c_int {
        (*qglobal).zfilename = (*(*qglobal).qprocess).zv2devices;
        iret |= 0x200 as libc::c_int | 0x400 as libc::c_int;
    }
    return iret;
}

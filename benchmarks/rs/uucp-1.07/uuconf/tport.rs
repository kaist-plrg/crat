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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
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
    fn uuconf_malloc_block() -> *mut libc::c_void;
    fn _uuconf_uclear_port(qport: *mut uuconf_port);
    fn _uuconf_iport_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qport: *mut uuconf_port,
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
pub static mut _uuconf_tport_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: tport.c,v 1.12 2002/03/05 19:10:43 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_taylor_find_port(
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
    let mut pblock: pointer = 0 as *mut libc::c_void;
    let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iret: libc::c_int = 0;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if ihighbaud == 0 as libc::c_long {
        ihighbaud = ibaud;
    }
    e = 0 as *mut FILE;
    pblock = 0 as *mut libc::c_void;
    zfree = 0 as *mut libc::c_char;
    iret = 1 as libc::c_int;
    pz = (*(*qglobal).qprocess).pzportfiles;
    while !(*pz).is_null() {
        let mut as_0: [uuconf_cmdtab; 2] = [uuconf_cmdtab {
            uuconf_zcmd: 0 as *const libc::c_char,
            uuconf_itype: 0,
            uuconf_pvar: 0 as *mut libc::c_void,
            uuconf_pifn: None,
        }; 2];
        let mut zport: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sdefault: uuconf_port = uuconf_port {
            uuconf_zname: 0 as *mut libc::c_char,
            uuconf_ttype: UUCONF_PORTTYPE_UNKNOWN,
            uuconf_zprotocols: 0 as *mut libc::c_char,
            uuconf_qproto_params: 0 as *mut uuconf_proto_param,
            uuconf_ireliable: 0,
            uuconf_zlockname: 0 as *mut libc::c_char,
            uuconf_palloc: 0 as *mut libc::c_void,
            uuconf_u: C2RustUnnamed {
                uuconf_sstdin: uuconf_stdin_port {
                    uuconf_idummy: 0,
                },
            },
        };
        let mut ilineno: libc::c_int = 0;
        e = fopen(*pz, b"r\0" as *const u8 as *const libc::c_char);
        if e.is_null() {
            if !(*__errno_location() == 2 as libc::c_int) {
                (*qglobal).ierrno = *__errno_location();
                iret = 2 as libc::c_int | 0x100 as libc::c_int;
                break;
            }
        } else {
            (*qglobal).ilineno = 0 as libc::c_int;
            as_0[0 as libc::c_int as usize]
                .uuconf_zcmd = b"port\0" as *const u8 as *const libc::c_char;
            as_0[0 as libc::c_int as usize]
                .uuconf_itype = 0x60 as libc::c_int | 2 as libc::c_int;
            as_0[0 as libc::c_int as usize]
                .uuconf_pvar = &mut zport as *mut *mut libc::c_char as pointer;
            as_0[0 as libc::c_int as usize]
                .uuconf_pifn = Some(
                ipport
                    as unsafe extern "C" fn(
                        pointer,
                        libc::c_int,
                        *mut *mut libc::c_char,
                        pointer,
                        pointer,
                    ) -> libc::c_int,
            );
            as_0[1 as libc::c_int as usize].uuconf_zcmd = 0 as *const libc::c_char;
            pblock = uuconf_malloc_block();
            if pblock.is_null() {
                (*qglobal).ierrno = *__errno_location();
                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                break;
            } else {
                _uuconf_uclear_port(&mut sdefault);
                sdefault.uuconf_palloc = pblock;
                zport = 0 as *mut libc::c_char;
                iret = uuconf_cmd_file(
                    pglobal,
                    e,
                    as_0.as_mut_ptr(),
                    &mut sdefault as *mut uuconf_port as pointer,
                    Some(
                        ipunknown
                            as unsafe extern "C" fn(
                                pointer,
                                libc::c_int,
                                *mut *mut libc::c_char,
                                pointer,
                                pointer,
                            ) -> libc::c_int,
                    ),
                    0x2 as libc::c_int,
                    pblock,
                );
                if iret != 0 as libc::c_int {
                    zfree = zport;
                    break;
                } else {
                    iret = 1 as libc::c_int;
                    while !zport.is_null() {
                        let mut piunknown: uuconf_cmdtabfn = None;
                        let mut fmatch: boolean = 0;
                        if zname.is_null() || strcmp(zname, zport) == 0 as libc::c_int {
                            piunknown = Some(
                                ipunknown
                                    as unsafe extern "C" fn(
                                        pointer,
                                        libc::c_int,
                                        *mut *mut libc::c_char,
                                        pointer,
                                        pointer,
                                    ) -> libc::c_int,
                            );
                            *qport = sdefault;
                            (*qport).uuconf_zname = zport;
                            zfree = zport;
                            fmatch = 1 as libc::c_int;
                        } else {
                            piunknown = None;
                            free(zport as pointer);
                            fmatch = 0 as libc::c_int;
                        }
                        zport = 0 as *mut libc::c_char;
                        ilineno = (*qglobal).ilineno;
                        iret = uuconf_cmd_file(
                            pglobal,
                            e,
                            as_0.as_mut_ptr(),
                            qport as pointer,
                            piunknown,
                            0x2 as libc::c_int,
                            pblock,
                        );
                        (*qglobal).ilineno += ilineno;
                        if iret != 0 as libc::c_int {
                            break;
                        }
                        iret = 1 as libc::c_int;
                        if fmatch != 0 {
                            if ibaud != 0 as libc::c_int as libc::c_long {
                                if (*qport).uuconf_ttype as libc::c_uint
                                    == UUCONF_PORTTYPE_MODEM as libc::c_int as libc::c_uint
                                {
                                    let mut imbaud: libc::c_long = 0;
                                    let mut imhigh: libc::c_long = 0;
                                    let mut imlow: libc::c_long = 0;
                                    imbaud = (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud;
                                    imhigh = (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud;
                                    imlow = (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud;
                                    if !(imbaud == 0 as libc::c_int as libc::c_long
                                        && imlow == 0 as libc::c_int as libc::c_long)
                                    {
                                        if !(ibaud <= imbaud && imbaud <= ihighbaud) {
                                            if !(imlow != 0 as libc::c_int as libc::c_long
                                                && imlow <= ihighbaud && imhigh >= ibaud)
                                            {
                                                fmatch = 0 as libc::c_int;
                                            }
                                        }
                                    }
                                } else if (*qport).uuconf_ttype as libc::c_uint
                                    == UUCONF_PORTTYPE_DIRECT as libc::c_int as libc::c_uint
                                {
                                    let mut idbaud: libc::c_long = 0;
                                    idbaud = (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud;
                                    if idbaud != 0 as libc::c_int as libc::c_long
                                        && idbaud != ibaud
                                    {
                                        fmatch = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                        if fmatch != 0 {
                            if pifn.is_some() {
                                iret = (Some(pifn.unwrap())).unwrap()(qport, pinfo);
                                if iret == 1 as libc::c_int {
                                    fmatch = 0 as libc::c_int;
                                } else if iret != 0 as libc::c_int {
                                    break;
                                }
                            }
                        }
                        if fmatch != 0 {
                            if uuconf_add_block(pblock, zfree as *mut libc::c_void)
                                == 0 as libc::c_int
                            {
                                zfree = 0 as *mut libc::c_char;
                                iret = 0 as libc::c_int;
                            } else {
                                (*qglobal).ierrno = *__errno_location();
                                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            }
                            break;
                        } else if !zfree.is_null() {
                            free(zfree as pointer);
                            zfree = 0 as *mut libc::c_char;
                        }
                    }
                    fclose(e);
                    e = 0 as *mut FILE;
                    if iret != 1 as libc::c_int {
                        break;
                    }
                    uuconf_free_block(pblock);
                    pblock = 0 as *mut libc::c_void;
                }
            }
        }
        pz = pz.offset(1);
        pz;
    }
    if !e.is_null() {
        fclose(e);
    }
    if !zfree.is_null() {
        free(zfree as pointer);
    }
    if iret != 0 as libc::c_int && !pblock.is_null() {
        uuconf_free_block(pblock);
    }
    if iret != 0 as libc::c_int && iret != 1 as libc::c_int {
        (*qglobal).zfilename = *pz;
        iret |= 0x200 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn ipport(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pz: *mut *mut libc::c_char = pvar as *mut *mut libc::c_char;
    let mut csize: size_t = 0;
    csize = (strlen(*argv.offset(1 as libc::c_int as isize)))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    *pz = malloc(csize) as *mut libc::c_char;
    if (*pz).is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
    }
    memcpy(
        *pz as pointer,
        *argv.offset(1 as libc::c_int as isize) as pointer as *const libc::c_void,
        csize,
    );
    return 0x1000 as libc::c_int;
}
unsafe extern "C" fn ipunknown(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qport: *mut uuconf_port = pinfo as *mut uuconf_port;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iport_cmd(qglobal, argc, argv, qport);
    if iret & 0xff as libc::c_int != 0 as libc::c_int {
        iret |= 0x1000 as libc::c_int;
    }
    return iret;
}

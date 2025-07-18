use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn _uuconf_ucmdtab_base(
        qoff: *const cmdtab_offset,
        celes: size_t,
        pbase: *mut libc::c_char,
        qset: *mut uuconf_cmdtab,
    );
    fn _uuconf_iboolean(
        qglobal: *mut sglobal,
        zval: *const libc::c_char,
        pi: *mut libc::c_int,
    ) -> libc::c_int;
    fn _uuconf_ichat_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qchat: *mut uuconf_chat,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_iadd_proto_param(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pq: *mut *mut uuconf_proto_param,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_iseven_bit(
        pglobal: pointer,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pvar: pointer,
        pinfo: pointer,
    ) -> libc::c_int;
    fn _uuconf_ireliable(
        pglobal: pointer,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pvar: pointer,
        pinfo: pointer,
    ) -> libc::c_int;
    fn _uuconf_ihalf_duplex(
        pglobal: pointer,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pvar: pointer,
        pinfo: pointer,
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
pub static mut _uuconf_tdialc_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: tdialc.c,v 1.9 2002/03/05 19:10:43 ian Rel $\0")
};
static mut asDialer_cmds: [cmdtab_offset; 15] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"chat\0" as *const u8 as *const libc::c_char,
                itype: 0x70 as libc::c_int | 0 as libc::c_int,
                ioff: 8 as libc::c_ulong,
                pifn: Some(
                    idchat
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
                zcmd: b"dialtone\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 48 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"pause\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"carrier\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 64 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"carrier-wait\0" as *const u8 as *const libc::c_char,
                itype: 0x22 as libc::c_int,
                ioff: 68 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"dtr-toggle\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iddtr_toggle
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
                zcmd: b"complete\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 80 as libc::c_ulong,
                pifn: Some(
                    idcomplete
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
                zcmd: b"complete-chat\0" as *const u8 as *const libc::c_char,
                itype: 0x70 as libc::c_int,
                ioff: 80 as libc::c_ulong,
                pifn: Some(
                    idchat
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
                zcmd: b"abort\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 120 as libc::c_ulong,
                pifn: Some(
                    idcomplete
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
                zcmd: b"abort-chat\0" as *const u8 as *const libc::c_char,
                itype: 0x70 as libc::c_int,
                ioff: 120 as libc::c_ulong,
                pifn: Some(
                    idchat
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
                zcmd: b"protocol-parameter\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 160 as libc::c_ulong,
                pifn: Some(
                    idproto_param
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
                zcmd: b"seven-bit\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 168 as libc::c_ulong,
                pifn: Some(
                    _uuconf_iseven_bit
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
                zcmd: b"reliable\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 168 as libc::c_ulong,
                pifn: Some(
                    _uuconf_ireliable
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
                zcmd: b"half-duplex\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: 168 as libc::c_ulong,
                pifn: Some(
                    _uuconf_ihalf_duplex
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
                zcmd: 0 as *const libc::c_char,
                itype: 0 as libc::c_int,
                ioff: 0 as libc::c_int as size_t,
                pifn: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn _uuconf_idialer_cmd(
    mut qglobal: *mut sglobal,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut qdialer: *mut uuconf_dialer,
) -> libc::c_int {
    let mut as_0: [uuconf_cmdtab; 15] = [uuconf_cmdtab {
        uuconf_zcmd: 0 as *const libc::c_char,
        uuconf_itype: 0,
        uuconf_pvar: 0 as *mut libc::c_void,
        uuconf_pifn: None,
    }; 15];
    let mut iret: libc::c_int = 0;
    _uuconf_ucmdtab_base(
        asDialer_cmds.as_ptr(),
        (::std::mem::size_of::<[cmdtab_offset; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong),
        qdialer as *mut libc::c_char,
        as_0.as_mut_ptr(),
    );
    iret = uuconf_cmd_args(
        qglobal as pointer,
        argc,
        argv,
        as_0.as_mut_ptr(),
        qdialer as pointer,
        Some(
            idcunknown
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0 as libc::c_int,
        (*qdialer).uuconf_palloc,
    );
    return iret & !(0x1000 as libc::c_int);
}
unsafe extern "C" fn idchat(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qchat: *mut uuconf_chat = pvar as *mut uuconf_chat;
    let mut qdialer: *mut uuconf_dialer = pinfo as *mut uuconf_dialer;
    return _uuconf_ichat_cmd(qglobal, argc, argv, qchat, (*qdialer).uuconf_palloc);
}
unsafe extern "C" fn iddtr_toggle(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qdialer: *mut uuconf_dialer = pinfo as *mut uuconf_dialer;
    let mut iret: libc::c_int = 0;
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    iret = _uuconf_iboolean(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut (*qdialer).uuconf_fdtr_toggle,
    );
    if iret & !(0x800 as libc::c_int) != 0 as libc::c_int {
        return iret;
    }
    if argc < 3 as libc::c_int {
        return iret;
    }
    iret
        |= _uuconf_iboolean(
            qglobal,
            *argv.offset(2 as libc::c_int as isize),
            &mut (*qdialer).uuconf_fdtr_toggle_wait,
        );
    return iret;
}
unsafe extern "C" fn idcomplete(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qchat: *mut uuconf_chat = pvar as *mut uuconf_chat;
    let mut qdialer: *mut uuconf_dialer = pinfo as *mut uuconf_dialer;
    let mut azargs: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    azargs[0 as libc::c_int
        as usize] = b"complete-chat\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    azargs[1 as libc::c_int
        as usize] = b"\"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    azargs[2 as libc::c_int as usize] = *argv.offset(1 as libc::c_int as isize);
    return _uuconf_ichat_cmd(
        qglobal,
        3 as libc::c_int,
        azargs.as_mut_ptr(),
        qchat,
        (*qdialer).uuconf_palloc,
    );
}
unsafe extern "C" fn idproto_param(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pqparam: *mut *mut uuconf_proto_param = pvar as *mut *mut uuconf_proto_param;
    let mut qdialer: *mut uuconf_dialer = pinfo as *mut uuconf_dialer;
    return _uuconf_iadd_proto_param(
        qglobal,
        argc - 1 as libc::c_int,
        argv.offset(1 as libc::c_int as isize),
        pqparam,
        (*qdialer).uuconf_palloc,
    );
}
unsafe extern "C" fn idcunknown(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    return 5 as libc::c_int | 0x1000 as libc::c_int;
}

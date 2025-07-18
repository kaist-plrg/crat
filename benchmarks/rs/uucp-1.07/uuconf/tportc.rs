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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn _uuconf_ucmdtab_base(
        qoff: *const cmdtab_offset,
        celes: size_t,
        pbase: *mut libc::c_char,
        qset: *mut uuconf_cmdtab,
    );
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
    fn _uuconf_iint(
        qglobal: *mut sglobal,
        zval: *const libc::c_char,
        p: pointer,
        fint: boolean,
    ) -> libc::c_int;
    fn _uuconf_uclear_dialer(qdialer: *mut uuconf_dialer);
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_idialer_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qdialer: *mut uuconf_dialer,
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
pub static mut _uuconf_tportc_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: tportc.c,v 1.18 2002/03/05 19:10:43 ian Rel $\0")
};
static mut azPtype_names: [*const libc::c_char; 7] = [
    0 as *const libc::c_char,
    b"stdin\0" as *const u8 as *const libc::c_char,
    b"modem\0" as *const u8 as *const libc::c_char,
    b"direct\0" as *const u8 as *const libc::c_char,
    b"tcp\0" as *const u8 as *const libc::c_char,
    b"tli\0" as *const u8 as *const libc::c_char,
    b"pipe\0" as *const u8 as *const libc::c_char,
];
static mut asPort_cmds: [cmdtab_offset; 7] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"protocol\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 16 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"protocol-parameter\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 24 as libc::c_ulong,
                pifn: Some(
                    ipproto_param
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
                ioff: 32 as libc::c_ulong,
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
                ioff: 32 as libc::c_ulong,
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
                ioff: 32 as libc::c_ulong,
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
                zcmd: b"lockname\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 40 as libc::c_ulong,
                pifn: None,
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
static mut asPstdin_cmds: [cmdtab_offset; 1] = [
    {
        let mut init = cmdtab_offset {
            zcmd: 0 as *const libc::c_char,
            itype: 0 as libc::c_int,
            ioff: 0 as libc::c_int as size_t,
            pifn: None,
        };
        init
    },
];
static mut asPmodem_cmds: [cmdtab_offset; 11] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"device\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"baud\0" as *const u8 as *const libc::c_char,
                itype: 0x32 as libc::c_int,
                ioff: 72 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"speed\0" as *const u8 as *const libc::c_char,
                itype: 0x32 as libc::c_int,
                ioff: 72 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"baud-range\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: Some(
                    ipbaud_range
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
                zcmd: b"speed-range\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: Some(
                    ipbaud_range
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
                zcmd: b"carrier\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 96 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"hardflow\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 100 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"dial-device\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 64 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"dialer\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: Some(
                    ipdialer
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
                zcmd: b"dialer-sequence\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 104 as libc::c_ulong,
                pifn: None,
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
static mut asPdirect_cmds: [cmdtab_offset; 6] = [
    {
        let mut init = cmdtab_offset {
            zcmd: b"device\0" as *const u8 as *const libc::c_char,
            itype: 0x40 as libc::c_int,
            ioff: 56 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"baud\0" as *const u8 as *const libc::c_char,
            itype: 0x32 as libc::c_int,
            ioff: 64 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"speed\0" as *const u8 as *const libc::c_char,
            itype: 0x32 as libc::c_int,
            ioff: 64 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"carrier\0" as *const u8 as *const libc::c_char,
            itype: 0x12 as libc::c_int,
            ioff: 72 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"hardflow\0" as *const u8 as *const libc::c_char,
            itype: 0x12 as libc::c_int,
            ioff: 76 as libc::c_ulong,
            pifn: None,
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
];
static mut asPtcp_cmds: [cmdtab_offset; 4] = [
    {
        let mut init = cmdtab_offset {
            zcmd: b"service\0" as *const u8 as *const libc::c_char,
            itype: 0x40 as libc::c_int,
            ioff: 56 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"version\0" as *const u8 as *const libc::c_char,
            itype: 0x22 as libc::c_int,
            ioff: 64 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"dialer-sequence\0" as *const u8 as *const libc::c_char,
            itype: 0x50 as libc::c_int,
            ioff: 72 as libc::c_ulong,
            pifn: None,
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
];
static mut asPtli_cmds: [cmdtab_offset; 6] = [
    {
        let mut init = cmdtab_offset {
            zcmd: b"device\0" as *const u8 as *const libc::c_char,
            itype: 0x40 as libc::c_int,
            ioff: 56 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"stream\0" as *const u8 as *const libc::c_char,
            itype: 0x12 as libc::c_int,
            ioff: 64 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"push\0" as *const u8 as *const libc::c_char,
            itype: 0x50 as libc::c_int,
            ioff: 72 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"dialer-sequence\0" as *const u8 as *const libc::c_char,
            itype: 0x50 as libc::c_int,
            ioff: 80 as libc::c_ulong,
            pifn: None,
        };
        init
    },
    {
        let mut init = cmdtab_offset {
            zcmd: b"server-address\0" as *const u8 as *const libc::c_char,
            itype: 0x40 as libc::c_int,
            ioff: 88 as libc::c_ulong,
            pifn: None,
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
];
static mut asPpipe_cmds: [cmdtab_offset; 2] = [
    {
        let mut init = cmdtab_offset {
            zcmd: b"command\0" as *const u8 as *const libc::c_char,
            itype: 0x50 as libc::c_int,
            ioff: 56 as libc::c_ulong,
            pifn: None,
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
];
pub unsafe extern "C" fn _uuconf_iport_cmd(
    mut qglobal: *mut sglobal,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut qport: *mut uuconf_port,
) -> libc::c_int {
    let mut fgottype: boolean = 0;
    let mut qcmds: *const cmdtab_offset = 0 as *const cmdtab_offset;
    let mut ccmds: size_t = 0;
    let mut as_0: [uuconf_cmdtab; 11] = [uuconf_cmdtab {
        uuconf_zcmd: 0 as *const libc::c_char,
        uuconf_itype: 0,
        uuconf_pvar: 0 as *mut libc::c_void,
        uuconf_pifn: None,
    }; 11];
    let mut i: size_t = 0;
    let mut iret: libc::c_int = 0;
    fgottype = (strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"type\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    if fgottype != 0
        || (*qport).uuconf_ttype as libc::c_uint
            == UUCONF_PORTTYPE_UNKNOWN as libc::c_int as libc::c_uint
    {
        let mut ttype: uuconf_porttype = UUCONF_PORTTYPE_UNKNOWN;
        if fgottype == 0 {
            ttype = UUCONF_PORTTYPE_MODEM;
        } else {
            if argc != 2 as libc::c_int {
                return 5 as libc::c_int;
            }
            i = 0 as libc::c_int as size_t;
            while i
                < (::std::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    )
            {
                if !(azPtype_names[i as usize]).is_null()
                    && strcasecmp(
                        *argv.offset(1 as libc::c_int as isize),
                        azPtype_names[i as usize],
                    ) == 0 as libc::c_int
                {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            if i
                >= (::std::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    )
            {
                return 5 as libc::c_int;
            }
            ttype = i as uuconf_porttype;
        }
        (*qport).uuconf_ttype = ttype;
        match ttype as libc::c_uint {
            2 => {
                (*qport).uuconf_u.uuconf_smodem.uuconf_zdevice = 0 as *mut libc::c_char;
                (*qport)
                    .uuconf_u
                    .uuconf_smodem
                    .uuconf_zdial_device = 0 as *mut libc::c_char;
                (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud = 0 as libc::c_long;
                (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud = 0 as libc::c_long;
                (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud = 0 as libc::c_long;
                (*qport).uuconf_u.uuconf_smodem.uuconf_fcarrier = 1 as libc::c_int;
                (*qport).uuconf_u.uuconf_smodem.uuconf_fhardflow = 1 as libc::c_int;
                (*qport)
                    .uuconf_u
                    .uuconf_smodem
                    .uuconf_pzdialer = 0 as *mut *mut libc::c_char;
                (*qport).uuconf_u.uuconf_smodem.uuconf_qdialer = 0 as *mut uuconf_dialer;
            }
            3 => {
                (*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice = 0 as *mut libc::c_char;
                (*qport)
                    .uuconf_u
                    .uuconf_sdirect
                    .uuconf_ibaud = -(1 as libc::c_int) as libc::c_long;
                (*qport).uuconf_u.uuconf_sdirect.uuconf_fcarrier = 0 as libc::c_int;
                (*qport).uuconf_u.uuconf_sdirect.uuconf_fhardflow = 1 as libc::c_int;
            }
            4 => {
                (*qport)
                    .uuconf_u
                    .uuconf_stcp
                    .uuconf_zport = b"uucp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                (*qport).uuconf_u.uuconf_stcp.uuconf_iversion = 0 as libc::c_int;
                (*qport)
                    .uuconf_u
                    .uuconf_stcp
                    .uuconf_pzdialer = 0 as *mut *mut libc::c_char;
                (*qport)
                    .uuconf_ireliable = 0o1 as libc::c_int | 0o10 as libc::c_int
                    | 0o4 as libc::c_int | 0o2 as libc::c_int | 0o20 as libc::c_int;
            }
            5 => {
                (*qport).uuconf_u.uuconf_stli.uuconf_zdevice = 0 as *mut libc::c_char;
                (*qport).uuconf_u.uuconf_stli.uuconf_fstream = 0 as libc::c_int;
                (*qport)
                    .uuconf_u
                    .uuconf_stli
                    .uuconf_pzpush = 0 as *mut *mut libc::c_char;
                (*qport)
                    .uuconf_u
                    .uuconf_stli
                    .uuconf_pzdialer = 0 as *mut *mut libc::c_char;
                (*qport).uuconf_u.uuconf_stli.uuconf_zservaddr = 0 as *mut libc::c_char;
                (*qport)
                    .uuconf_ireliable = 0o1 as libc::c_int | 0o10 as libc::c_int
                    | 0o4 as libc::c_int | 0o2 as libc::c_int | 0o20 as libc::c_int;
            }
            6 => {
                (*qport)
                    .uuconf_u
                    .uuconf_spipe
                    .uuconf_pzcmd = 0 as *mut *mut libc::c_char;
            }
            1 | _ => {}
        }
        if fgottype != 0 {
            return 0 as libc::c_int;
        }
    }
    qcmds = asPort_cmds.as_ptr();
    ccmds = (::std::mem::size_of::<[cmdtab_offset; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[cmdtab_offset; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if strcasecmp(
            *argv.offset(0 as libc::c_int as isize),
            asPort_cmds[i as usize].zcmd,
        ) == 0 as libc::c_int
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i
        >= (::std::mem::size_of::<[cmdtab_offset; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        match (*qport).uuconf_ttype as libc::c_uint {
            1 => {
                qcmds = asPstdin_cmds.as_ptr();
                ccmds = (::std::mem::size_of::<[cmdtab_offset; 1]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong,
                    );
            }
            2 => {
                qcmds = asPmodem_cmds.as_ptr();
                ccmds = (::std::mem::size_of::<[cmdtab_offset; 11]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong,
                    );
            }
            3 => {
                qcmds = asPdirect_cmds.as_ptr();
                ccmds = (::std::mem::size_of::<[cmdtab_offset; 6]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong,
                    );
            }
            4 => {
                qcmds = asPtcp_cmds.as_ptr();
                ccmds = (::std::mem::size_of::<[cmdtab_offset; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong,
                    );
            }
            5 => {
                qcmds = asPtli_cmds.as_ptr();
                ccmds = (::std::mem::size_of::<[cmdtab_offset; 6]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong,
                    );
            }
            6 => {
                qcmds = asPpipe_cmds.as_ptr();
                ccmds = (::std::mem::size_of::<[cmdtab_offset; 2]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong,
                    );
            }
            _ => return 5 as libc::c_int,
        }
    }
    _uuconf_ucmdtab_base(qcmds, ccmds, qport as *mut libc::c_char, as_0.as_mut_ptr());
    iret = uuconf_cmd_args(
        qglobal as pointer,
        argc,
        argv,
        as_0.as_mut_ptr(),
        qport as pointer,
        Some(
            ipcunknown
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0 as libc::c_int,
        (*qport).uuconf_palloc,
    );
    return iret & !(0x1000 as libc::c_int);
}
unsafe extern "C" fn ipproto_param(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pqparam: *mut *mut uuconf_proto_param = pvar as *mut *mut uuconf_proto_param;
    let mut qport: *mut uuconf_port = pinfo as *mut uuconf_port;
    return _uuconf_iadd_proto_param(
        qglobal,
        argc - 1 as libc::c_int,
        argv.offset(1 as libc::c_int as isize),
        pqparam,
        (*qport).uuconf_palloc,
    );
}
unsafe extern "C" fn ipbaud_range(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qmodem: *mut uuconf_modem_port = pvar as *mut uuconf_modem_port;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iint(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut (*qmodem).uuconf_ilowbaud as *mut libc::c_long as pointer,
        0 as libc::c_int,
    );
    if iret & !(0x800 as libc::c_int) != 0 as libc::c_int {
        return iret;
    }
    iret
        |= _uuconf_iint(
            qglobal,
            *argv.offset(2 as libc::c_int as isize),
            &mut (*qmodem).uuconf_ihighbaud as *mut libc::c_long as pointer,
            0 as libc::c_int,
        );
    return iret;
}
unsafe extern "C" fn ipdialer(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qmodem: *mut uuconf_modem_port = pvar as *mut uuconf_modem_port;
    let mut qport: *mut uuconf_port = pinfo as *mut uuconf_port;
    let mut iret: libc::c_int = 0;
    if argc < 2 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    if argc > 2 as libc::c_int {
        if ((*qmodem).uuconf_qdialer).is_null() {
            let mut qnew: *mut uuconf_dialer = 0 as *mut uuconf_dialer;
            let mut clen: size_t = 0;
            qnew = uuconf_malloc(
                (*qport).uuconf_palloc,
                ::std::mem::size_of::<uuconf_dialer>() as libc::c_ulong,
            ) as *mut uuconf_dialer;
            if qnew.is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
            }
            _uuconf_uclear_dialer(qnew);
            if ((*qport).uuconf_zname).is_null() {
                (*qnew)
                    .uuconf_zname = b"default port file dialer\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            } else {
                clen = strlen((*qport).uuconf_zname);
                (*qnew)
                    .uuconf_zname = uuconf_malloc(
                    (*qport).uuconf_palloc,
                    clen
                        .wrapping_add(
                            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                if ((*qnew).uuconf_zname).is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    return 4 as libc::c_int | 0x100 as libc::c_int
                        | 0x1000 as libc::c_int;
                }
                memcpy(
                    (*qnew).uuconf_zname as pointer,
                    (*qport).uuconf_zname as pointer as *const libc::c_void,
                    clen,
                );
                memcpy(
                    ((*qnew).uuconf_zname).offset(clen as isize) as pointer,
                    b" dialer\0" as *const u8 as *const libc::c_char as pointer
                        as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                );
            }
            (*qnew).uuconf_palloc = (*qport).uuconf_palloc;
            (*qmodem).uuconf_qdialer = qnew;
        }
        iret = _uuconf_idialer_cmd(
            qglobal,
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
            (*qmodem).uuconf_qdialer,
        );
        if iret & !(0x800 as libc::c_int) != 0 as libc::c_int {
            iret |= 0x1000 as libc::c_int;
        }
        return iret;
    } else {
        (*qmodem).uuconf_pzdialer = 0 as *mut *mut libc::c_char;
        iret = _uuconf_iadd_string(
            qglobal,
            *argv.offset(1 as libc::c_int as isize),
            1 as libc::c_int,
            0 as libc::c_int,
            &mut (*qmodem).uuconf_pzdialer,
            (*qport).uuconf_palloc,
        );
        if iret != 0 as libc::c_int {
            iret |= 0x1000 as libc::c_int;
        }
        return iret;
    };
}
unsafe extern "C" fn ipcunknown(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    return 5 as libc::c_int | 0x1000 as libc::c_int;
}

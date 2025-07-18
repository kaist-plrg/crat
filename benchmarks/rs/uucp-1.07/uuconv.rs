use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    static mut _uuconf_unset: *mut libc::c_char;
    fn uuconf_error_string(
        uuconf_pglobal: *mut libc::c_void,
        ierror: libc::c_int,
        zbuf: *mut libc::c_char,
        cbuf: UUCONF_SIZE_T,
    ) -> libc::c_int;
    fn uuconf_hdb_dialer_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdialer: *const libc::c_char,
        uuconf_qdialer: *mut uuconf_dialer,
    ) -> libc::c_int;
    fn uuconf_taylor_dialer_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdialer: *const libc::c_char,
        uuconf_qdialer: *mut uuconf_dialer,
    ) -> libc::c_int;
    fn uuconf_hdb_dialer_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzdialers: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn uuconf_taylor_dialer_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzdialers: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn uuconf_hdb_find_port(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zname: *const libc::c_char,
        uuconf_ibaud: libc::c_long,
        uuconf_ihighbaud: libc::c_long,
        uuconf_pifn: Option::<
            unsafe extern "C" fn(*mut uuconf_port, *mut libc::c_void) -> libc::c_int,
        >,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_qport: *mut uuconf_port,
    ) -> libc::c_int;
    fn uuconf_v2_find_port(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zname: *const libc::c_char,
        uuconf_ibaud: libc::c_long,
        uuconf_ihighbaud: libc::c_long,
        uuconf_pifn: Option::<
            unsafe extern "C" fn(*mut uuconf_port, *mut libc::c_void) -> libc::c_int,
        >,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_qport: *mut uuconf_port,
    ) -> libc::c_int;
    fn uuconf_taylor_find_port(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zname: *const libc::c_char,
        uuconf_ibaud: libc::c_long,
        uuconf_ihighbaud: libc::c_long,
        uuconf_pifn: Option::<
            unsafe extern "C" fn(*mut uuconf_port, *mut libc::c_void) -> libc::c_int,
        >,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_qport: *mut uuconf_port,
    ) -> libc::c_int;
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_ihdb_system_internal(
        qglobal: *mut sglobal,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn _uuconf_iv2_system_internal(
        qglobal: *mut sglobal,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn _uuconf_itaylor_system_internal(
        qglobal: *mut sglobal,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_hdb_system_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzsystems: *mut *mut *mut libc::c_char,
        uuconf_falias: libc::c_int,
    ) -> libc::c_int;
    fn uuconf_v2_system_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzsystems: *mut *mut *mut libc::c_char,
        uuconf_falias: libc::c_int,
    ) -> libc::c_int;
    fn uuconf_taylor_system_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzsystems: *mut *mut *mut libc::c_char,
        uuconf_falias: libc::c_int,
    ) -> libc::c_int;
    fn uuconf_hdb_init(
        uuconf_ppglobal: *mut *mut libc::c_void,
        uuconf_zprogram: *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_v2_init(uuconf_ppglobal: *mut *mut libc::c_void) -> libc::c_int;
    fn uuconf_taylor_init(
        uuconf_pglobal: *mut *mut libc::c_void,
        uuconf_zprogram: *const libc::c_char,
        uuconf_zname: *const libc::c_char,
    ) -> libc::c_int;
    static mut gnu_optarg: *mut libc::c_char;
    static mut gnu_optind: libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
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
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
}
pub const CONFIG_HDB: tconfig = 2;
pub const CONFIG_TAYLOR: tconfig = 0;
pub type tconfig = libc::c_uint;
pub const CONFIG_V2: tconfig = 1;
pub type UUCONF_SIZE_T = size_t;
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
pub struct uuconf_system {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_pzalias: *mut *mut libc::c_char,
    pub uuconf_qalternate: *mut uuconf_system,
    pub uuconf_zalternate: *mut libc::c_char,
    pub uuconf_fcall: libc::c_int,
    pub uuconf_fcalled: libc::c_int,
    pub uuconf_qtimegrade: *mut uuconf_timespan,
    pub uuconf_qcalltimegrade: *mut uuconf_timespan,
    pub uuconf_qcalledtimegrade: *mut uuconf_timespan,
    pub uuconf_cmax_retries: libc::c_int,
    pub uuconf_csuccess_wait: libc::c_int,
    pub uuconf_qcall_local_size: *mut uuconf_timespan,
    pub uuconf_qcall_remote_size: *mut uuconf_timespan,
    pub uuconf_qcalled_local_size: *mut uuconf_timespan,
    pub uuconf_qcalled_remote_size: *mut uuconf_timespan,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_ihighbaud: libc::c_long,
    pub uuconf_zport: *mut libc::c_char,
    pub uuconf_qport: *mut uuconf_port,
    pub uuconf_zphone: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zcall_login: *mut libc::c_char,
    pub uuconf_zcall_password: *mut libc::c_char,
    pub uuconf_zcalled_login: *mut libc::c_char,
    pub uuconf_fcallback: libc::c_int,
    pub uuconf_fsequence: libc::c_int,
    pub uuconf_zprotocols: *mut libc::c_char,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_scalled_chat: uuconf_chat,
    pub uuconf_zdebug: *mut libc::c_char,
    pub uuconf_zmax_remote_debug: *mut libc::c_char,
    pub uuconf_fsend_request: libc::c_int,
    pub uuconf_frec_request: libc::c_int,
    pub uuconf_fcall_transfer: libc::c_int,
    pub uuconf_fcalled_transfer: libc::c_int,
    pub uuconf_pzlocal_send: *mut *mut libc::c_char,
    pub uuconf_pzremote_send: *mut *mut libc::c_char,
    pub uuconf_pzlocal_receive: *mut *mut libc::c_char,
    pub uuconf_pzremote_receive: *mut *mut libc::c_char,
    pub uuconf_pzpath: *mut *mut libc::c_char,
    pub uuconf_pzcmds: *mut *mut libc::c_char,
    pub uuconf_cfree_space: libc::c_long,
    pub uuconf_pzforward_from: *mut *mut libc::c_char,
    pub uuconf_pzforward_to: *mut *mut libc::c_char,
    pub uuconf_zpubdir: *const libc::c_char,
    pub uuconf_zlocalname: *mut libc::c_char,
    pub uuconf_cmax_file_time: libc::c_long,
    pub uuconf_palloc: UUCONF_POINTER,
}
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const no_argument: _argtype = 0;
pub const required_argument: _argtype = 1;
pub type _argtype = libc::c_uint;
pub const optional_argument: _argtype = 2;
pub static mut uuconv_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: uuconv.c,v 1.30 2002/03/05 19:10:42 ian Rel $\0")
};
pub static mut zProgram: *const libc::c_char = 0 as *const libc::c_char;
static mut qVperms: *mut shpermissions = 0 as *const shpermissions as *mut shpermissions;
static mut asVlongopts: [option; 8] = [
    {
        let mut init = option {
            name: b"input\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"program\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"config\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut zinput: *const libc::c_char = 0 as *const libc::c_char;
    let mut zoutput: *const libc::c_char = 0 as *const libc::c_char;
    let mut zprogram: *const libc::c_char = 0 as *const libc::c_char;
    let mut iopt: libc::c_int = 0;
    let mut tinput: tconfig = CONFIG_TAYLOR;
    let mut toutput: tconfig = CONFIG_TAYLOR;
    let mut iret: libc::c_int = 0;
    let mut pinput: pointer = 0 as *mut libc::c_void;
    zProgram = *argv.offset(0 as libc::c_int as isize);
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"i:I:o:p:vx:\0" as *const u8 as *const libc::c_char,
            asVlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            105 => {
                zinput = gnu_optarg;
            }
            111 => {
                zoutput = gnu_optarg;
            }
            112 => {
                zprogram = gnu_optarg;
            }
            73 => {
                zconfig = gnu_optarg;
            }
            120 => {}
            118 => {
                printf(
                    b"uuconv (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
                    b"1.07\0" as *const u8 as *const libc::c_char,
                );
                printf(
                    b"Copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"This program is free software; you may redistribute it under the terms of\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"the GNU General Public LIcense.  This program has ABSOLUTELY NO WARRANTY.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            1 => {
                uvhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                uvusage();
            }
        }
    }
    if gnu_optind != argc || zinput.is_null() || zoutput.is_null() {
        uvusage();
    }
    if strcasecmp(zinput, b"taylor\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        tinput = CONFIG_TAYLOR;
    } else if strcasecmp(zinput, b"v2\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        tinput = CONFIG_V2;
    } else if strcasecmp(zinput, b"hdb\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        tinput = CONFIG_HDB;
    } else {
        uvusage();
        tinput = CONFIG_TAYLOR;
    }
    if strcasecmp(zoutput, b"taylor\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        toutput = CONFIG_TAYLOR;
    } else if strcasecmp(zoutput, b"v2\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        toutput = CONFIG_V2;
    } else if strcasecmp(zoutput, b"hdb\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        toutput = CONFIG_HDB;
    } else {
        uvusage();
        toutput = CONFIG_TAYLOR;
    }
    if tinput as libc::c_uint == toutput as libc::c_uint {
        uvusage();
    }
    iret = 0 as libc::c_int;
    pinput = 0 as *mut libc::c_void;
    match tinput as libc::c_uint {
        0 => {
            iret = uuconf_taylor_init(&mut pinput, zprogram, zconfig);
        }
        1 => {
            iret = uuconf_v2_init(&mut pinput);
        }
        2 => {
            iret = uuconf_hdb_init(&mut pinput, zprogram);
        }
        _ => {}
    }
    if iret != 0 as libc::c_int {
        uvuuconf_error(pinput, iret);
        exit(1 as libc::c_int);
    }
    let mut pzsystems: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut zsys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abtaylor: [libc::c_char; 6] = [0; 6];
    let mut abv2: [libc::c_char; 8] = [0; 8];
    let mut abhdb: [libc::c_char; 10] = [0; 10];
    let mut esys: *mut FILE = 0 as *mut FILE;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    match tinput as libc::c_uint {
        0 => {
            iret = uuconf_taylor_system_names(pinput, &mut pzsystems, 0 as libc::c_int);
        }
        1 => {
            iret = uuconf_v2_system_names(pinput, &mut pzsystems, 0 as libc::c_int);
        }
        2 => {
            iret = uuconf_hdb_system_names(pinput, &mut pzsystems, 0 as libc::c_int);
        }
        _ => {}
    }
    if iret != 0 as libc::c_int {
        uvuuconf_error(pinput, iret);
    } else {
        match toutput as libc::c_uint {
            1 => {
                sprintf(
                    abv2.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    b".\0" as *const u8 as *const libc::c_char,
                    b"/L.sys\0" as *const u8 as *const libc::c_char,
                );
                zsys = abv2.as_mut_ptr();
            }
            2 => {
                sprintf(
                    abhdb.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    b".\0" as *const u8 as *const libc::c_char,
                    b"/Systems\0" as *const u8 as *const libc::c_char,
                );
                zsys = abhdb.as_mut_ptr();
            }
            0 | _ => {
                sprintf(
                    abtaylor.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    b".\0" as *const u8 as *const libc::c_char,
                    b"/sys\0" as *const u8 as *const libc::c_char,
                );
                zsys = abtaylor.as_mut_ptr();
            }
        }
        esys = fopen(zsys, b"w\0" as *const u8 as *const libc::c_char);
        if esys.is_null() {
            fprintf(stderr, b"uuchk:%s: \0" as *const u8 as *const libc::c_char, zsys);
            perror(b"fopen\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        fprintf(
            esys,
            b"# %s file automatically generated by uuconv.\n\0" as *const u8
                as *const libc::c_char,
            zsys,
        );
        pz = pzsystems;
        while !(*pz).is_null() {
            let mut ssys: uuconf_system = uuconf_system {
                uuconf_zname: 0 as *mut libc::c_char,
                uuconf_pzalias: 0 as *mut *mut libc::c_char,
                uuconf_qalternate: 0 as *mut uuconf_system,
                uuconf_zalternate: 0 as *mut libc::c_char,
                uuconf_fcall: 0,
                uuconf_fcalled: 0,
                uuconf_qtimegrade: 0 as *mut uuconf_timespan,
                uuconf_qcalltimegrade: 0 as *mut uuconf_timespan,
                uuconf_qcalledtimegrade: 0 as *mut uuconf_timespan,
                uuconf_cmax_retries: 0,
                uuconf_csuccess_wait: 0,
                uuconf_qcall_local_size: 0 as *mut uuconf_timespan,
                uuconf_qcall_remote_size: 0 as *mut uuconf_timespan,
                uuconf_qcalled_local_size: 0 as *mut uuconf_timespan,
                uuconf_qcalled_remote_size: 0 as *mut uuconf_timespan,
                uuconf_ibaud: 0,
                uuconf_ihighbaud: 0,
                uuconf_zport: 0 as *mut libc::c_char,
                uuconf_qport: 0 as *mut uuconf_port,
                uuconf_zphone: 0 as *mut libc::c_char,
                uuconf_schat: uuconf_chat {
                    uuconf_pzchat: 0 as *mut *mut libc::c_char,
                    uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                    uuconf_ctimeout: 0,
                    uuconf_pzfail: 0 as *mut *mut libc::c_char,
                    uuconf_fstrip: 0,
                },
                uuconf_zcall_login: 0 as *mut libc::c_char,
                uuconf_zcall_password: 0 as *mut libc::c_char,
                uuconf_zcalled_login: 0 as *mut libc::c_char,
                uuconf_fcallback: 0,
                uuconf_fsequence: 0,
                uuconf_zprotocols: 0 as *mut libc::c_char,
                uuconf_qproto_params: 0 as *mut uuconf_proto_param,
                uuconf_scalled_chat: uuconf_chat {
                    uuconf_pzchat: 0 as *mut *mut libc::c_char,
                    uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                    uuconf_ctimeout: 0,
                    uuconf_pzfail: 0 as *mut *mut libc::c_char,
                    uuconf_fstrip: 0,
                },
                uuconf_zdebug: 0 as *mut libc::c_char,
                uuconf_zmax_remote_debug: 0 as *mut libc::c_char,
                uuconf_fsend_request: 0,
                uuconf_frec_request: 0,
                uuconf_fcall_transfer: 0,
                uuconf_fcalled_transfer: 0,
                uuconf_pzlocal_send: 0 as *mut *mut libc::c_char,
                uuconf_pzremote_send: 0 as *mut *mut libc::c_char,
                uuconf_pzlocal_receive: 0 as *mut *mut libc::c_char,
                uuconf_pzremote_receive: 0 as *mut *mut libc::c_char,
                uuconf_pzpath: 0 as *mut *mut libc::c_char,
                uuconf_pzcmds: 0 as *mut *mut libc::c_char,
                uuconf_cfree_space: 0,
                uuconf_pzforward_from: 0 as *mut *mut libc::c_char,
                uuconf_pzforward_to: 0 as *mut *mut libc::c_char,
                uuconf_zpubdir: 0 as *const libc::c_char,
                uuconf_zlocalname: 0 as *mut libc::c_char,
                uuconf_cmax_file_time: 0,
                uuconf_palloc: 0 as *mut libc::c_void,
            };
            match tinput as libc::c_uint {
                0 => {
                    iret = _uuconf_itaylor_system_internal(
                        pinput as *mut sglobal,
                        *pz,
                        &mut ssys,
                    );
                }
                1 => {
                    iret = _uuconf_iv2_system_internal(
                        pinput as *mut sglobal,
                        *pz,
                        &mut ssys,
                    );
                }
                2 => {
                    iret = _uuconf_ihdb_system_internal(
                        pinput as *mut sglobal,
                        *pz,
                        &mut ssys,
                    );
                }
                _ => {}
            }
            if iret != 0 as libc::c_int {
                uvuuconf_error(pinput, iret);
            } else {
                match toutput as libc::c_uint {
                    0 => {
                        uvwrite_taylor_system(esys, &mut ssys);
                    }
                    1 => {
                        uvwrite_v2_system(esys, &mut ssys);
                    }
                    2 => {
                        uvwrite_hdb_system(esys, &mut ssys);
                    }
                    _ => {}
                }
                if toutput as libc::c_uint != CONFIG_HDB as libc::c_int as libc::c_uint {
                    uuconf_free_block(ssys.uuconf_palloc);
                }
            }
            pz = pz.offset(1);
            pz;
        }
        if toutput as libc::c_uint == CONFIG_HDB as libc::c_int as libc::c_uint {
            uvwrite_perms();
        }
        if ferror(esys) != 0 || fclose(esys) == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"uuchk:%s: error during output\n\0" as *const u8 as *const libc::c_char,
                zsys,
            );
            exit(1 as libc::c_int);
        }
    }
    let mut zport: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abtaylor_0: [libc::c_char; 7] = [0; 7];
    let mut abv2_0: [libc::c_char; 12] = [0; 12];
    let mut abhdb_0: [libc::c_char; 10] = [0; 10];
    let mut eport: *mut FILE = 0 as *mut FILE;
    let mut piportfn: Option::<
        unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
    > = None;
    let mut sport: uuconf_port = uuconf_port {
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
    match toutput as libc::c_uint {
        1 => {
            sprintf(
                abv2_0.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                b".\0" as *const u8 as *const libc::c_char,
                b"/L-devices\0" as *const u8 as *const libc::c_char,
            );
            zport = abv2_0.as_mut_ptr();
            piportfn = Some(
                ivwrite_v2_port
                    as unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
            );
        }
        2 => {
            sprintf(
                abhdb_0.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                b".\0" as *const u8 as *const libc::c_char,
                b"/Devices\0" as *const u8 as *const libc::c_char,
            );
            zport = abhdb_0.as_mut_ptr();
            piportfn = Some(
                ivwrite_hdb_port
                    as unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
            );
        }
        0 | _ => {
            sprintf(
                abtaylor_0.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                b".\0" as *const u8 as *const libc::c_char,
                b"/port\0" as *const u8 as *const libc::c_char,
            );
            zport = abtaylor_0.as_mut_ptr();
            piportfn = Some(
                ivwrite_taylor_port
                    as unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
            );
        }
    }
    eport = fopen(zport, b"w\0" as *const u8 as *const libc::c_char);
    if eport.is_null() {
        fprintf(stderr, b"uuchk:%s: \0" as *const u8 as *const libc::c_char, zport);
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    fprintf(
        eport,
        b"# %s file automatically generated by uuconv.\n\0" as *const u8
            as *const libc::c_char,
        zport,
    );
    match tinput as libc::c_uint {
        0 => {
            iret = uuconf_taylor_find_port(
                pinput,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_long,
                0 as libc::c_long,
                piportfn,
                eport as pointer,
                &mut sport,
            );
        }
        1 => {
            iret = uuconf_v2_find_port(
                pinput,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_long,
                0 as libc::c_long,
                piportfn,
                eport as pointer,
                &mut sport,
            );
        }
        2 => {
            iret = uuconf_hdb_find_port(
                pinput,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_long,
                0 as libc::c_long,
                piportfn,
                eport as pointer,
                &mut sport,
            );
        }
        _ => {}
    }
    if iret != 1 as libc::c_int {
        uvuuconf_error(pinput, iret);
    }
    if ferror(eport) != 0 || fclose(eport) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"uuchk:%s: error during output\n\0" as *const u8 as *const libc::c_char,
            zport,
        );
        exit(1 as libc::c_int);
    }
    if tinput as libc::c_uint != CONFIG_V2 as libc::c_int as libc::c_uint
        && toutput as libc::c_uint != CONFIG_V2 as libc::c_int as libc::c_uint
    {
        let mut pzdialers: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut zdialer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut abtaylor_1: [libc::c_char; 7] = [0; 7];
        let mut abhdb_1: [libc::c_char; 10] = [0; 10];
        let mut edialer: *mut FILE = 0 as *mut FILE;
        let mut pz_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        match tinput as libc::c_uint {
            2 => {
                iret = uuconf_hdb_dialer_names(pinput, &mut pzdialers);
            }
            0 | _ => {
                iret = uuconf_taylor_dialer_names(pinput, &mut pzdialers);
            }
        }
        if iret != 0 as libc::c_int {
            uvuuconf_error(pinput, iret);
        } else {
            match toutput as libc::c_uint {
                2 => {
                    sprintf(
                        abhdb_1.as_mut_ptr(),
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        b".\0" as *const u8 as *const libc::c_char,
                        b"/Dialers\0" as *const u8 as *const libc::c_char,
                    );
                    zdialer = abhdb_1.as_mut_ptr();
                }
                0 | _ => {
                    sprintf(
                        abtaylor_1.as_mut_ptr(),
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        b".\0" as *const u8 as *const libc::c_char,
                        b"/dial\0" as *const u8 as *const libc::c_char,
                    );
                    zdialer = abtaylor_1.as_mut_ptr();
                }
            }
            edialer = fopen(zdialer, b"w\0" as *const u8 as *const libc::c_char);
            if edialer.is_null() {
                fprintf(
                    stderr,
                    b"uuchk:%s: \0" as *const u8 as *const libc::c_char,
                    zdialer,
                );
                perror(b"fopen\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            fprintf(
                edialer,
                b"# %s file automatically generated by uuconv.\n\0" as *const u8
                    as *const libc::c_char,
                zdialer,
            );
            pz_0 = pzdialers;
            while !(*pz_0).is_null() {
                let mut sdialer: uuconf_dialer = uuconf_dialer {
                    uuconf_zname: 0 as *mut libc::c_char,
                    uuconf_schat: uuconf_chat {
                        uuconf_pzchat: 0 as *mut *mut libc::c_char,
                        uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                        uuconf_ctimeout: 0,
                        uuconf_pzfail: 0 as *mut *mut libc::c_char,
                        uuconf_fstrip: 0,
                    },
                    uuconf_zdialtone: 0 as *mut libc::c_char,
                    uuconf_zpause: 0 as *mut libc::c_char,
                    uuconf_fcarrier: 0,
                    uuconf_ccarrier_wait: 0,
                    uuconf_fdtr_toggle: 0,
                    uuconf_fdtr_toggle_wait: 0,
                    uuconf_scomplete: uuconf_chat {
                        uuconf_pzchat: 0 as *mut *mut libc::c_char,
                        uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                        uuconf_ctimeout: 0,
                        uuconf_pzfail: 0 as *mut *mut libc::c_char,
                        uuconf_fstrip: 0,
                    },
                    uuconf_sabort: uuconf_chat {
                        uuconf_pzchat: 0 as *mut *mut libc::c_char,
                        uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                        uuconf_ctimeout: 0,
                        uuconf_pzfail: 0 as *mut *mut libc::c_char,
                        uuconf_fstrip: 0,
                    },
                    uuconf_qproto_params: 0 as *mut uuconf_proto_param,
                    uuconf_ireliable: 0,
                    uuconf_palloc: 0 as *mut libc::c_void,
                };
                match tinput as libc::c_uint {
                    2 => {
                        iret = uuconf_hdb_dialer_info(pinput, *pz_0, &mut sdialer);
                    }
                    0 | _ => {
                        iret = uuconf_taylor_dialer_info(pinput, *pz_0, &mut sdialer);
                    }
                }
                if iret != 0 as libc::c_int {
                    uvuuconf_error(pinput, iret);
                } else {
                    match toutput as libc::c_uint {
                        2 => {
                            uvwrite_hdb_dialer(edialer, &mut sdialer);
                        }
                        0 | _ => {
                            fprintf(
                                edialer,
                                b"# Start of dialer %s\n\0" as *const u8
                                    as *const libc::c_char,
                                sdialer.uuconf_zname,
                            );
                            fprintf(
                                edialer,
                                b"dialer %s\n\0" as *const u8 as *const libc::c_char,
                                sdialer.uuconf_zname,
                            );
                            uvwrite_taylor_dialer(
                                edialer,
                                &mut sdialer,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    uuconf_free_block(sdialer.uuconf_palloc);
                }
                pz_0 = pz_0.offset(1);
                pz_0;
            }
            if ferror(edialer) != 0 || fclose(edialer) == -(1 as libc::c_int) {
                fprintf(
                    stderr,
                    b"uuchk:%s: error during output\n\0" as *const u8
                        as *const libc::c_char,
                    zdialer,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn uvusage() {
    fprintf(
        stderr,
        b"Usage: %s -i input-type -o output-type [-p program]\n\0" as *const u8
            as *const libc::c_char,
        zProgram,
    );
    fprintf(
        stderr,
        b"Use %s --help for help\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn uvhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Converts UUCP configuration files from one format to another.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Usage: %s -i input -o output [-p program] [-I file]\n\0" as *const u8
            as *const libc::c_char,
        zProgram,
    );
    printf(
        b" -i,--input input: Set input type (one of taylor, v2, hdb)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -o,--output output: Set output type (one of taylor, v2, hdb)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -p,--program program: Program to convert (e.g., uucp or cu)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -I,--config file: Set Taylor UUCP configuration file to use\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -v,--version: Print version and exit\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b" --help: Print help and exit\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Report bugs to taylor-uucp@gnu.org\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn uvwrite_time(mut e: *mut FILE, mut qtime: *mut uuconf_timespan) {
    if qtime.is_null() {
        fprintf(e, b"Never\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*qtime).uuconf_istart == 0 as libc::c_int
        && (*qtime).uuconf_iend
            == 7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
    {
        fprintf(e, b"Any\0" as *const u8 as *const libc::c_char);
        return;
    }
    while !qtime.is_null() {
        let mut idaystart: libc::c_int = 0;
        let mut idayend: libc::c_int = 0;
        let mut ihourstart: libc::c_int = 0;
        let mut ihourend: libc::c_int = 0;
        let mut iminutestart: libc::c_int = 0;
        let mut iminuteend: libc::c_int = 0;
        let zdays: *const libc::c_char = b"Su\0Mo\0Tu\0We\0Th\0Fr\0Sa\0" as *const u8
            as *const libc::c_char;
        idaystart = (*qtime).uuconf_istart / (24 as libc::c_int * 60 as libc::c_int);
        ihourstart = (*qtime).uuconf_istart % (24 as libc::c_int * 60 as libc::c_int)
            / 60 as libc::c_int;
        iminutestart = (*qtime).uuconf_istart % 60 as libc::c_int;
        if (*qtime).uuconf_iend
            >= 7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
        {
            (*qtime)
                .uuconf_iend = 7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                - 1 as libc::c_int;
        }
        idayend = (*qtime).uuconf_iend / (24 as libc::c_int * 60 as libc::c_int);
        ihourend = (*qtime).uuconf_iend % (24 as libc::c_int * 60 as libc::c_int)
            / 60 as libc::c_int;
        iminuteend = (*qtime).uuconf_iend % 60 as libc::c_int;
        if ihourend == 0 as libc::c_int && iminuteend == 0 as libc::c_int {
            idayend -= 1;
            idayend;
        }
        if idaystart == idayend {
            fprintf(
                e,
                b"%s%02d%02d-%02d%02d\0" as *const u8 as *const libc::c_char,
                zdays.offset((idaystart * 3 as libc::c_int) as isize),
                ihourstart,
                iminutestart,
                ihourend,
                iminuteend,
            );
        } else {
            let mut i: libc::c_int = 0;
            fprintf(
                e,
                b"%s%02d%02d-0000\0" as *const u8 as *const libc::c_char,
                zdays.offset((idaystart * 3 as libc::c_int) as isize),
                ihourstart,
                iminutestart,
            );
            i = idaystart + 1 as libc::c_int;
            while i < idayend {
                fprintf(
                    e,
                    b",%s\0" as *const u8 as *const libc::c_char,
                    zdays.offset((i * 3 as libc::c_int) as isize),
                );
                i += 1;
                i;
            }
            if ihourend != 0 as libc::c_int || iminuteend != 0 as libc::c_int {
                fprintf(
                    e,
                    b",%s0000-%02d%02d\0" as *const u8 as *const libc::c_char,
                    zdays.offset((idayend * 3 as libc::c_int) as isize),
                    ihourend,
                    iminuteend,
                );
            }
        }
        if !((*qtime).uuconf_qnext).is_null() {
            fprintf(e, b",\0" as *const u8 as *const libc::c_char);
        }
        qtime = (*qtime).uuconf_qnext;
    }
}
unsafe extern "C" fn uvwrite_string(
    mut e: *mut FILE,
    mut zarg: *const libc::c_char,
    mut zcmd: *const libc::c_char,
) {
    if zarg != &mut _uuconf_unset as *mut *mut libc::c_char as *const libc::c_char {
        fprintf(
            e,
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            zcmd,
            if zarg.is_null() { b"\0" as *const u8 as *const libc::c_char } else { zarg },
        );
    }
}
unsafe extern "C" fn uvwrite_size(
    mut e: *mut FILE,
    mut qtime: *mut uuconf_timespan,
    mut zcmd: *const libc::c_char,
) {
    if qtime != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan {
        while !qtime.is_null() {
            fprintf(
                e,
                b"%s %ld\0" as *const u8 as *const libc::c_char,
                zcmd,
                (*qtime).uuconf_ival,
            );
            uvwrite_time(e, qtime);
            fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
            qtime = (*qtime).uuconf_qnext;
        }
    }
}
unsafe extern "C" fn uvwrite_boolean(
    mut e: *mut FILE,
    mut fval: libc::c_int,
    mut zcmd: *const libc::c_char,
) {
    if fval >= 0 as libc::c_int {
        fprintf(
            e,
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            zcmd,
            if fval > 0 as libc::c_int {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    }
}
unsafe extern "C" fn uvwrite_string_array(
    mut e: *mut FILE,
    mut pz: *mut *mut libc::c_char,
    mut zcmd: *const libc::c_char,
) {
    if pz != &mut _uuconf_unset as *mut *mut libc::c_char {
        fprintf(e, b"%s\0" as *const u8 as *const libc::c_char, zcmd);
        if !pz.is_null() {
            while !(*pz).is_null() {
                fprintf(e, b" %s\0" as *const u8 as *const libc::c_char, *pz);
                pz = pz.offset(1);
                pz;
            }
        }
        fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn uvwrite_chat_script(
    mut e: *mut FILE,
    mut pzarg: *mut *mut libc::c_char,
) {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if pzarg.is_null() || pzarg == &mut _uuconf_unset as *mut *mut libc::c_char {
        return;
    }
    pz = pzarg;
    while !(*pz).is_null() {
        if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
            && pz != pzarg
        {
            fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, *pz);
        pz = pz.offset(1);
        pz;
    }
}
unsafe extern "C" fn uvwrite_chat(
    mut e: *mut FILE,
    mut q: *const uuconf_chat,
    mut qlast: *const uuconf_chat,
    mut zprefix: *const libc::c_char,
    mut fforce: boolean,
) {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ab: [libc::c_char; 100] = [0; 100];
    if (*q).uuconf_pzchat != &mut _uuconf_unset as *mut *mut libc::c_char
        && (if qlast.is_null() {
            (fforce != 0 || !((*q).uuconf_pzchat).is_null()) as libc::c_int
        } else {
            ((*qlast).uuconf_pzchat != (*q).uuconf_pzchat) as libc::c_int
        }) != 0
    {
        fprintf(e, b"%schat \0" as *const u8 as *const libc::c_char, zprefix);
        uvwrite_chat_script(e, (*q).uuconf_pzchat);
        fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if (*q).uuconf_pzprogram != &mut _uuconf_unset as *mut *mut libc::c_char
        && (if qlast.is_null() {
            ((*q).uuconf_pzprogram != 0 as *mut libc::c_void as *mut *mut libc::c_char)
                as libc::c_int
        } else {
            ((*qlast).uuconf_pzprogram != (*q).uuconf_pzprogram) as libc::c_int
        }) != 0
    {
        sprintf(
            ab.as_mut_ptr(),
            b"%schat-program\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_string_array(e, (*q).uuconf_pzprogram, ab.as_mut_ptr());
    }
    if (*q).uuconf_ctimeout >= 0 as libc::c_int
        && (qlast.is_null() || (*qlast).uuconf_ctimeout != (*q).uuconf_ctimeout)
    {
        fprintf(
            e,
            b"%schat-timeout %d\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            (*q).uuconf_ctimeout,
        );
    }
    if !((*q).uuconf_pzfail).is_null()
        && (*q).uuconf_pzfail != &mut _uuconf_unset as *mut *mut libc::c_char
        && (qlast.is_null() || (*qlast).uuconf_pzfail != (*q).uuconf_pzfail)
    {
        pz = (*q).uuconf_pzfail;
        while !(*pz).is_null() {
            fprintf(
                e,
                b"%schat-fail %s\n\0" as *const u8 as *const libc::c_char,
                zprefix,
                *pz,
            );
            pz = pz.offset(1);
            pz;
        }
    }
    if qlast.is_null() || (*qlast).uuconf_fstrip != (*q).uuconf_fstrip {
        sprintf(
            ab.as_mut_ptr(),
            b"%schat-strip\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(e, (*q).uuconf_fstrip, ab.as_mut_ptr());
    }
}
unsafe extern "C" fn uvwrite_proto_params(
    mut e: *mut FILE,
    mut qparams: *const uuconf_proto_param,
    mut zprefix: *const libc::c_char,
) {
    let mut qp: *const uuconf_proto_param = 0 as *const uuconf_proto_param;
    if qparams.is_null()
        || qparams
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_proto_param
                as *const uuconf_proto_param
    {
        return;
    }
    qp = qparams;
    while (*qp).uuconf_bproto != '\0' as i32 {
        let mut qe: *const uuconf_proto_param_entry = 0
            as *const uuconf_proto_param_entry;
        qe = (*qp).uuconf_qentries;
        while (*qe).uuconf_cargs > 0 as libc::c_int {
            let mut i: libc::c_int = 0;
            fprintf(
                e,
                b"%sprotocol-parameter %c\0" as *const u8 as *const libc::c_char,
                zprefix,
                (*qp).uuconf_bproto,
            );
            i = 0 as libc::c_int;
            while i < (*qe).uuconf_cargs {
                fprintf(
                    e,
                    b" %s\0" as *const u8 as *const libc::c_char,
                    *((*qe).uuconf_pzargs).offset(i as isize),
                );
                i += 1;
                i;
            }
            fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
            qe = qe.offset(1);
            qe;
        }
        qp = qp.offset(1);
        qp;
    }
}
unsafe extern "C" fn uvwrite_taylor_system(
    mut e: *mut FILE,
    mut q: *const uuconf_system,
) {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut qlast: *const uuconf_system = 0 as *const uuconf_system;
    fprintf(
        e,
        b"# Start of system %s\n\0" as *const u8 as *const libc::c_char,
        (*q).uuconf_zname,
    );
    fprintf(e, b"system %s\n\0" as *const u8 as *const libc::c_char, (*q).uuconf_zname);
    if !((*q).uuconf_pzalias).is_null()
        && (*q).uuconf_pzalias != &mut _uuconf_unset as *mut *mut libc::c_char
    {
        pz = (*q).uuconf_pzalias;
        while !(*pz).is_null() {
            uvwrite_string(e, *pz, b"alias\0" as *const u8 as *const libc::c_char);
            pz = pz.offset(1);
            pz;
        }
    }
    qlast = 0 as *const uuconf_system;
    while !q.is_null() {
        let mut qtime: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
        if !qlast.is_null() {
            fprintf(e, b"alternate\0" as *const u8 as *const libc::c_char);
            if (*q).uuconf_zalternate
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                && !((*q).uuconf_zalternate).is_null()
            {
                fprintf(
                    e,
                    b" %s\0" as *const u8 as *const libc::c_char,
                    (*q).uuconf_zalternate,
                );
            }
            fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
        }
        if (qlast.is_null() || (*qlast).uuconf_qtimegrade != (*q).uuconf_qtimegrade)
            && (*q).uuconf_qtimegrade
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            if ((*q).uuconf_qtimegrade).is_null() {
                fprintf(e, b"time never\n\0" as *const u8 as *const libc::c_char);
            } else {
                qtime = (*q).uuconf_qtimegrade;
                while !qtime.is_null() {
                    if (*qtime).uuconf_ival as libc::c_char as libc::c_int == 'z' as i32
                    {
                        fprintf(e, b"time \0" as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(
                            e,
                            b"timegrade %c \0" as *const u8 as *const libc::c_char,
                            (*qtime).uuconf_ival as libc::c_char as libc::c_int,
                        );
                    }
                    uvwrite_time(e, qtime);
                    if (*qtime).uuconf_cretry != 0 as libc::c_int {
                        fprintf(
                            e,
                            b" %d\0" as *const u8 as *const libc::c_char,
                            (*qtime).uuconf_cretry,
                        );
                    }
                    fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
                    qtime = (*qtime).uuconf_qnext;
                }
            }
        }
        if (qlast.is_null()
            || (*qlast).uuconf_qcalltimegrade != (*q).uuconf_qcalltimegrade)
            && (*q).uuconf_qcalltimegrade
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            qtime = (*q).uuconf_qcalltimegrade;
            while !qtime.is_null() {
                fprintf(
                    e,
                    b"call-timegrade %c \0" as *const u8 as *const libc::c_char,
                    (*qtime).uuconf_ival as libc::c_char as libc::c_int,
                );
                uvwrite_time(e, qtime);
                fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
                qtime = (*qtime).uuconf_qnext;
            }
        }
        if (qlast.is_null()
            || (*qlast).uuconf_qcalledtimegrade != (*q).uuconf_qcalledtimegrade)
            && (*q).uuconf_qcalledtimegrade
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            qtime = (*q).uuconf_qcalledtimegrade;
            while !qtime.is_null() {
                fprintf(
                    e,
                    b"called-timegrade %c \0" as *const u8 as *const libc::c_char,
                    (*qtime).uuconf_ival as libc::c_char as libc::c_int,
                );
                uvwrite_time(e, qtime);
                fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
                qtime = (*qtime).uuconf_qnext;
            }
        }
        if qlast.is_null()
            || (*qlast).uuconf_qcall_local_size != (*q).uuconf_qcall_local_size
        {
            uvwrite_size(
                e,
                (*q).uuconf_qcall_local_size,
                b"call-local-size\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_qcall_remote_size != (*q).uuconf_qcall_remote_size
        {
            uvwrite_size(
                e,
                (*q).uuconf_qcall_remote_size,
                b"call-remote-size\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_qcalled_local_size != (*q).uuconf_qcalled_local_size
        {
            uvwrite_size(
                e,
                (*q).uuconf_qcalled_local_size,
                b"called-local-size\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_qcalled_remote_size != (*q).uuconf_qcalled_remote_size
        {
            uvwrite_size(
                e,
                (*q).uuconf_qcalled_remote_size,
                b"called-remote-size\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_ibaud != (*q).uuconf_ibaud
            || (qlast.is_null() || (*qlast).uuconf_ihighbaud != (*q).uuconf_ihighbaud)
        {
            if (*q).uuconf_ibaud >= 0 as libc::c_int as libc::c_long {
                if (*q).uuconf_ihighbaud > 0 as libc::c_int as libc::c_long {
                    fprintf(
                        e,
                        b"baud-range %ld %ld\n\0" as *const u8 as *const libc::c_char,
                        (*q).uuconf_ibaud,
                        (*q).uuconf_ihighbaud,
                    );
                } else {
                    fprintf(
                        e,
                        b"baud %ld\n\0" as *const u8 as *const libc::c_char,
                        (*q).uuconf_ibaud,
                    );
                }
            }
        }
        if qlast.is_null() || (*qlast).uuconf_zport != (*q).uuconf_zport
            || (qlast.is_null() || (*qlast).uuconf_qport != (*q).uuconf_qport)
        {
            if !((*q).uuconf_zport).is_null()
                && (*q).uuconf_zport
                    != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
            {
                uvwrite_string(
                    e,
                    (*q).uuconf_zport,
                    b"port\0" as *const u8 as *const libc::c_char,
                );
            } else if !((*q).uuconf_qport).is_null()
                && (*q).uuconf_qport
                    != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
            {
                uvwrite_taylor_port(
                    e,
                    (*q).uuconf_qport,
                    b"port \0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if qlast.is_null() || (*qlast).uuconf_zphone != (*q).uuconf_zphone {
            let mut zcmd: *const libc::c_char = 0 as *const libc::c_char;
            if !((*q).uuconf_qport).is_null()
                && (*q).uuconf_qport
                    != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
                && ((*(*q).uuconf_qport).uuconf_ttype as libc::c_uint
                    == UUCONF_PORTTYPE_TCP as libc::c_int as libc::c_uint
                    || (*(*q).uuconf_qport).uuconf_ttype as libc::c_uint
                        == UUCONF_PORTTYPE_TLI as libc::c_int as libc::c_uint)
            {
                zcmd = b"address\0" as *const u8 as *const libc::c_char;
            } else {
                zcmd = b"phone\0" as *const u8 as *const libc::c_char;
            }
            uvwrite_string(e, (*q).uuconf_zphone, zcmd);
        }
        uvwrite_chat(
            e,
            &(*q).uuconf_schat,
            if qlast.is_null() {
                0 as *mut libc::c_void as *mut uuconf_chat as *const uuconf_chat
            } else {
                &(*qlast).uuconf_schat
            },
            b"\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if qlast.is_null() || (*qlast).uuconf_zcall_login != (*q).uuconf_zcall_login {
            uvwrite_string(
                e,
                (*q).uuconf_zcall_login,
                b"call-login\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_zcall_password != (*q).uuconf_zcall_password
        {
            uvwrite_string(
                e,
                (*q).uuconf_zcall_password,
                b"call-password\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_zcalled_login != (*q).uuconf_zcalled_login
        {
            uvwrite_string(
                e,
                (*q).uuconf_zcalled_login,
                b"called-login\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_fcallback != (*q).uuconf_fcallback {
            uvwrite_boolean(
                e,
                (*q).uuconf_fcallback,
                b"callback\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_fsequence != (*q).uuconf_fsequence {
            uvwrite_boolean(
                e,
                (*q).uuconf_fsequence,
                b"sequence\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_zprotocols != (*q).uuconf_zprotocols {
            uvwrite_string(
                e,
                (*q).uuconf_zprotocols,
                b"protocol\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_qproto_params != (*q).uuconf_qproto_params
        {
            uvwrite_proto_params(
                e,
                (*q).uuconf_qproto_params,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        uvwrite_chat(
            e,
            &(*q).uuconf_scalled_chat,
            if qlast.is_null() {
                0 as *mut libc::c_void as *mut uuconf_chat as *const uuconf_chat
            } else {
                &(*qlast).uuconf_scalled_chat
            },
            b"called-\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        if qlast.is_null() || (*qlast).uuconf_zdebug != (*q).uuconf_zdebug {
            uvwrite_string(
                e,
                (*q).uuconf_zdebug,
                b"debug\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_zmax_remote_debug != (*q).uuconf_zmax_remote_debug
        {
            uvwrite_string(
                e,
                (*q).uuconf_zmax_remote_debug,
                b"max-remote-debug\0" as *const u8 as *const libc::c_char,
            );
        }
        if (qlast.is_null() || (*qlast).uuconf_fsend_request != (*q).uuconf_fsend_request
            || (qlast.is_null()
                || (*qlast).uuconf_frec_request != (*q).uuconf_frec_request))
            && ((*q).uuconf_fsend_request >= 0 as libc::c_int
                || (*q).uuconf_frec_request >= 0 as libc::c_int)
        {
            if (*q).uuconf_fsend_request >= 0 as libc::c_int
                && (if (*q).uuconf_fsend_request > 0 as libc::c_int {
                    ((*q).uuconf_frec_request > 0 as libc::c_int) as libc::c_int
                } else {
                    ((*q).uuconf_frec_request == 0 as libc::c_int) as libc::c_int
                }) != 0
            {
                uvwrite_boolean(
                    e,
                    (*q).uuconf_fsend_request,
                    b"request\0" as *const u8 as *const libc::c_char,
                );
            } else {
                uvwrite_boolean(
                    e,
                    (*q).uuconf_fsend_request,
                    b"send-request\0" as *const u8 as *const libc::c_char,
                );
                uvwrite_boolean(
                    e,
                    (*q).uuconf_frec_request,
                    b"receive-request\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if (qlast.is_null()
            || (*qlast).uuconf_fcall_transfer != (*q).uuconf_fcall_transfer
            || (qlast.is_null()
                || (*qlast).uuconf_fcalled_transfer != (*q).uuconf_fcalled_transfer))
            && ((*q).uuconf_fcall_transfer >= 0 as libc::c_int
                || (*q).uuconf_fcalled_transfer >= 0 as libc::c_int)
        {
            if (*q).uuconf_fcall_transfer >= 0 as libc::c_int
                && (if (*q).uuconf_fcall_transfer > 0 as libc::c_int {
                    ((*q).uuconf_fcalled_transfer > 0 as libc::c_int) as libc::c_int
                } else {
                    ((*q).uuconf_fcalled_transfer == 0 as libc::c_int) as libc::c_int
                }) != 0
            {
                uvwrite_boolean(
                    e,
                    (*q).uuconf_fcall_transfer,
                    b"transfer\0" as *const u8 as *const libc::c_char,
                );
            } else {
                uvwrite_boolean(
                    e,
                    (*q).uuconf_fcall_transfer,
                    b"call-transfer\0" as *const u8 as *const libc::c_char,
                );
                uvwrite_boolean(
                    e,
                    (*q).uuconf_fcalled_transfer,
                    b"called-transfer\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if qlast.is_null() || (*qlast).uuconf_pzlocal_send != (*q).uuconf_pzlocal_send {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzlocal_send,
                b"local-send\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_pzremote_send != (*q).uuconf_pzremote_send
        {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzremote_send,
                b"remote-send\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_pzlocal_receive != (*q).uuconf_pzlocal_receive
        {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzlocal_receive,
                b"local-receive\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_pzremote_receive != (*q).uuconf_pzremote_receive
        {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzremote_receive,
                b"remote-receive\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_pzpath != (*q).uuconf_pzpath {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzpath,
                b"command-path\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_pzcmds != (*q).uuconf_pzcmds {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzcmds,
                b"commands\0" as *const u8 as *const libc::c_char,
            );
        }
        if (qlast.is_null() || (*qlast).uuconf_cfree_space != (*q).uuconf_cfree_space)
            && (*q).uuconf_cfree_space >= 0 as libc::c_int as libc::c_long
        {
            fprintf(
                e,
                b"free-space %ld\n\0" as *const u8 as *const libc::c_char,
                (*q).uuconf_cfree_space,
            );
        }
        if qlast.is_null()
            || (*qlast).uuconf_pzforward_from != (*q).uuconf_pzforward_from
        {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzforward_from,
                b"forward-from\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_pzforward_to != (*q).uuconf_pzforward_to {
            uvwrite_string_array(
                e,
                (*q).uuconf_pzforward_to,
                b"forward-to\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_zpubdir != (*q).uuconf_zpubdir {
            uvwrite_string(
                e,
                (*q).uuconf_zpubdir,
                b"pubdir\0" as *const u8 as *const libc::c_char,
            );
        }
        if qlast.is_null() || (*qlast).uuconf_zlocalname != (*q).uuconf_zlocalname {
            uvwrite_string(
                e,
                (*q).uuconf_zlocalname,
                b"myname\0" as *const u8 as *const libc::c_char,
            );
        }
        qlast = q;
        q = (*q).uuconf_qalternate;
    }
}
unsafe extern "C" fn uvwrite_v2_system(mut e: *mut FILE, mut q: *const uuconf_system) {
    while !q.is_null() {
        fprintf(e, b"%s\0" as *const u8 as *const libc::c_char, (*q).uuconf_zname);
        if (*q).uuconf_qtimegrade
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            fprintf(e, b" \0" as *const u8 as *const libc::c_char);
            uvwrite_time(e, (*q).uuconf_qtimegrade);
            if (*q).uuconf_zport
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                || (*q).uuconf_qport
                    != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
            {
                let mut qp: *mut uuconf_port = 0 as *mut uuconf_port;
                let mut ftcp: boolean = 0;
                qp = (*q).uuconf_qport;
                ftcp = (qp
                    != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
                    && !qp.is_null()
                    && (*qp).uuconf_ttype as libc::c_uint
                        == UUCONF_PORTTYPE_TCP as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if ftcp != 0
                    || !((*q).uuconf_zport).is_null()
                        && (*q).uuconf_zport
                            != &mut _uuconf_unset as *mut *mut libc::c_char
                                as *mut libc::c_char
                {
                    if ftcp != 0 {
                        fprintf(e, b" TCP\0" as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(
                            e,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            (*q).uuconf_zport,
                        );
                    }
                    if ftcp != 0 || (*q).uuconf_ibaud >= 0 as libc::c_int as libc::c_long
                    {
                        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
                        if ftcp != 0 {
                            let mut zport: *const libc::c_char = 0
                                as *const libc::c_char;
                            zport = (*qp).uuconf_u.uuconf_stcp.uuconf_zport;
                            if zport.is_null() {
                                zport = b"uucp\0" as *const u8 as *const libc::c_char;
                            }
                            fprintf(
                                e,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                zport,
                            );
                        } else {
                            fprintf(
                                e,
                                b"%ld\0" as *const u8 as *const libc::c_char,
                                (*q).uuconf_ibaud,
                            );
                        }
                        if (*q).uuconf_zphone
                            != &mut _uuconf_unset as *mut *mut libc::c_char
                                as *mut libc::c_char && !((*q).uuconf_zphone).is_null()
                        {
                            let mut pzc: *mut *mut libc::c_char = 0
                                as *mut *mut libc::c_char;
                            fprintf(
                                e,
                                b" %s\0" as *const u8 as *const libc::c_char,
                                (*q).uuconf_zphone,
                            );
                            pzc = (*q).uuconf_schat.uuconf_pzchat;
                            if pzc != &mut _uuconf_unset as *mut *mut libc::c_char
                                && !pzc.is_null()
                            {
                                fprintf(e, b" \0" as *const u8 as *const libc::c_char);
                                uvwrite_chat_script(e, pzc);
                            }
                        }
                    }
                }
            }
        }
        fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
        q = (*q).uuconf_qalternate;
    }
}
unsafe extern "C" fn uvwrite_hdb_system(
    mut e: *mut FILE,
    mut qsys: *const uuconf_system,
) {
    let mut q: *const uuconf_system = 0 as *const uuconf_system;
    let mut sperm: shpermissions = shpermissions {
        qnext: 0 as *mut shpermissions,
        pzlogname: 0 as *mut *mut libc::c_char,
        pzmachine: 0 as *mut *mut libc::c_char,
        frequest: 0,
        fsendfiles: 0,
        pzread: 0 as *mut *mut libc::c_char,
        pzwrite: 0 as *mut *mut libc::c_char,
        fcallback: 0,
        pzcommands: 0 as *mut *mut libc::c_char,
        pzvalidate: 0 as *mut *mut libc::c_char,
        zmyname: 0 as *mut libc::c_char,
        zpubdir: 0 as *const libc::c_char,
        pzalias: 0 as *mut *mut libc::c_char,
    };
    let mut azmachine: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut azlogname: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    q = qsys;
    while !q.is_null() {
        if (*q).uuconf_fcall != 0 {
            fprintf(e, b"%s\0" as *const u8 as *const libc::c_char, (*q).uuconf_zname);
            if (*q).uuconf_qtimegrade
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
            {
                let mut zport: *const libc::c_char = 0 as *const libc::c_char;
                fprintf(e, b" \0" as *const u8 as *const libc::c_char);
                uvwrite_time(e, (*q).uuconf_qtimegrade);
                zport = (*q).uuconf_zport;
                if !((*q).uuconf_qport).is_null()
                    && (*q).uuconf_qport
                        != &mut _uuconf_unset as *mut *mut libc::c_char
                            as *mut uuconf_port
                    && (*(*q).uuconf_qport).uuconf_ttype as libc::c_uint
                        == UUCONF_PORTTYPE_TCP as libc::c_int as libc::c_uint
                {
                    zport = b"TCP\0" as *const u8 as *const libc::c_char;
                }
                if !zport.is_null()
                    && zport
                        != &mut _uuconf_unset as *mut *mut libc::c_char
                            as *mut libc::c_char as *const libc::c_char
                {
                    fprintf(e, b" %s\0" as *const u8 as *const libc::c_char, zport);
                    if (*q).uuconf_zprotocols
                        != &mut _uuconf_unset as *mut *mut libc::c_char
                            as *mut libc::c_char && !((*q).uuconf_zprotocols).is_null()
                    {
                        fprintf(
                            e,
                            b",%s\0" as *const u8 as *const libc::c_char,
                            (*q).uuconf_zprotocols,
                        );
                    }
                    if (*q).uuconf_ibaud >= 0 as libc::c_int as libc::c_long
                        || (*q).uuconf_zphone
                            != &mut _uuconf_unset as *mut *mut libc::c_char
                                as *mut libc::c_char
                    {
                        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
                        if (*q).uuconf_ibaud < 0 as libc::c_int as libc::c_long {
                            fprintf(e, b"Any\0" as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(
                                e,
                                b"%ld\0" as *const u8 as *const libc::c_char,
                                (*q).uuconf_ibaud,
                            );
                            if (*q).uuconf_ihighbaud >= 0 as libc::c_int as libc::c_long
                            {
                                fprintf(
                                    e,
                                    b"-%ld\0" as *const u8 as *const libc::c_char,
                                    (*q).uuconf_ihighbaud,
                                );
                            }
                        }
                        if (*q).uuconf_zphone
                            != &mut _uuconf_unset as *mut *mut libc::c_char
                                as *mut libc::c_char && !((*q).uuconf_zphone).is_null()
                        {
                            let mut pzc: *mut *mut libc::c_char = 0
                                as *mut *mut libc::c_char;
                            fprintf(
                                e,
                                b" %s\0" as *const u8 as *const libc::c_char,
                                (*q).uuconf_zphone,
                            );
                            pzc = (*q).uuconf_schat.uuconf_pzchat;
                            if pzc != &mut _uuconf_unset as *mut *mut libc::c_char
                                && !pzc.is_null()
                            {
                                fprintf(e, b" \0" as *const u8 as *const libc::c_char);
                                uvwrite_chat_script(e, pzc);
                            }
                        }
                    }
                }
            }
            fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
        }
        q = (*q).uuconf_qalternate;
    }
    q = qsys;
    while !q.is_null() {
        if (*q).uuconf_fcall != 0 {
            break;
        }
        q = (*q).uuconf_qalternate;
    }
    if !q.is_null() {
        sperm.qnext = 0 as *mut shpermissions;
        sperm.pzlogname = 0 as *mut *mut libc::c_char;
        sperm.pzmachine = 0 as *mut *mut libc::c_char;
        sperm.frequest = -(1 as libc::c_int);
        sperm.fsendfiles = -(1 as libc::c_int);
        sperm.pzread = 0 as *mut *mut libc::c_char;
        sperm.pzwrite = 0 as *mut *mut libc::c_char;
        sperm.fcallback = -(1 as libc::c_int);
        sperm.pzcommands = 0 as *mut *mut libc::c_char;
        sperm.pzvalidate = 0 as *mut *mut libc::c_char;
        sperm.zmyname = 0 as *mut libc::c_char;
        sperm.zpubdir = 0 as *const libc::c_char;
        sperm.pzalias = 0 as *mut *mut libc::c_char;
        azmachine[0 as libc::c_int as usize] = (*q).uuconf_zname;
        azmachine[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
        sperm.pzmachine = azmachine.as_mut_ptr();
        if (*q).uuconf_fsend_request >= 0 as libc::c_int {
            sperm.frequest = (*q).uuconf_fsend_request;
        }
        if (*q).uuconf_pzremote_send != &mut _uuconf_unset as *mut *mut libc::c_char
            && !((*q).uuconf_pzremote_send).is_null()
        {
            sperm.pzread = (*q).uuconf_pzremote_send;
        }
        if (*q).uuconf_pzremote_receive != &mut _uuconf_unset as *mut *mut libc::c_char
            && !((*q).uuconf_pzremote_receive).is_null()
        {
            sperm.pzwrite = (*q).uuconf_pzremote_receive;
        }
        if (*q).uuconf_pzcmds != &mut _uuconf_unset as *mut *mut libc::c_char
            && !((*q).uuconf_pzcmds).is_null()
        {
            sperm.pzcommands = (*q).uuconf_pzcmds;
        }
        if (*q).uuconf_zlocalname
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
            && !((*q).uuconf_zlocalname).is_null()
        {
            sperm.zmyname = (*q).uuconf_zlocalname;
        }
        if (*q).uuconf_zpubdir
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                as *const libc::c_char && !((*q).uuconf_zpubdir).is_null()
        {
            sperm.zpubdir = (*q).uuconf_zpubdir;
        }
        if (*q).uuconf_pzalias != &mut _uuconf_unset as *mut *mut libc::c_char
            && !((*q).uuconf_pzalias).is_null()
        {
            sperm.pzalias = (*q).uuconf_pzalias;
        }
        if (*q).uuconf_fcalled != 0
            && (*q).uuconf_zcalled_login
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
            && !((*q).uuconf_zcalled_login).is_null()
        {
            azlogname[0 as libc::c_int as usize] = (*q).uuconf_zcalled_login;
            azlogname[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
            sperm.pzlogname = azlogname.as_mut_ptr();
            if (*q).uuconf_fcalled_transfer >= 0 as libc::c_int {
                sperm.fsendfiles = (*q).uuconf_fcalled_transfer;
            }
            if (*q).uuconf_fcallback >= 0 as libc::c_int {
                sperm.fcallback = (*q).uuconf_fcallback;
            }
            sperm.pzvalidate = azmachine.as_mut_ptr();
        }
        uvadd_perm(&mut sperm);
    }
    q = qsys;
    while !q.is_null() {
        if !((*q).uuconf_fcalled == 0 || (*q).uuconf_fcall != 0) {
            sperm.qnext = 0 as *mut shpermissions;
            sperm.pzlogname = 0 as *mut *mut libc::c_char;
            sperm.pzmachine = 0 as *mut *mut libc::c_char;
            sperm.frequest = -(1 as libc::c_int);
            sperm.fsendfiles = -(1 as libc::c_int);
            sperm.pzread = 0 as *mut *mut libc::c_char;
            sperm.pzwrite = 0 as *mut *mut libc::c_char;
            sperm.fcallback = -(1 as libc::c_int);
            sperm.pzcommands = 0 as *mut *mut libc::c_char;
            sperm.pzvalidate = 0 as *mut *mut libc::c_char;
            sperm.zmyname = 0 as *mut libc::c_char;
            sperm.zpubdir = 0 as *const libc::c_char;
            sperm.pzalias = 0 as *mut *mut libc::c_char;
            if (*q).uuconf_zcalled_login
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                && !((*q).uuconf_zcalled_login).is_null()
            {
                azlogname[0 as libc::c_int as usize] = (*q).uuconf_zcalled_login;
            } else {
                azlogname[0 as libc::c_int
                    as usize] = b"OTHER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            azlogname[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
            sperm.pzlogname = azlogname.as_mut_ptr();
            if (*q).uuconf_fsend_request >= 0 as libc::c_int {
                sperm.frequest = (*q).uuconf_fsend_request;
            }
            if (*q).uuconf_fcalled_transfer >= 0 as libc::c_int {
                sperm.fsendfiles = (*q).uuconf_fcalled_transfer;
            }
            if (*q).uuconf_pzremote_send != &mut _uuconf_unset as *mut *mut libc::c_char
                && !((*q).uuconf_pzremote_send).is_null()
            {
                sperm.pzread = (*q).uuconf_pzremote_send;
            }
            if (*q).uuconf_pzremote_receive
                != &mut _uuconf_unset as *mut *mut libc::c_char
                && !((*q).uuconf_pzremote_receive).is_null()
            {
                sperm.pzwrite = (*q).uuconf_pzremote_receive;
            }
            if (*q).uuconf_fcallback >= 0 as libc::c_int {
                sperm.fcallback = (*q).uuconf_fcallback;
            }
            if (*q).uuconf_zlocalname
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                && !((*q).uuconf_zlocalname).is_null()
            {
                sperm.zmyname = (*q).uuconf_zlocalname;
            }
            if (*q).uuconf_zpubdir
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                    as *const libc::c_char && !((*q).uuconf_zpubdir).is_null()
            {
                sperm.zpubdir = (*q).uuconf_zpubdir;
            }
            uvadd_perm(&mut sperm);
        }
        q = (*q).uuconf_qalternate;
    }
}
unsafe extern "C" fn fvperm_string_cmp(
    mut z1: *const libc::c_char,
    mut z2: *const libc::c_char,
) -> boolean {
    if z1.is_null() {
        return (z2 == 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int;
    }
    if z2.is_null() {
        return 0 as libc::c_int;
    }
    return (strcmp(z1, z2) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn fvperm_array_cmp(
    mut pz1: *mut *const libc::c_char,
    mut pz2: *mut *const libc::c_char,
) -> boolean {
    if pz1.is_null() {
        return (pz2 == 0 as *mut libc::c_void as *mut *const libc::c_char)
            as libc::c_int;
    }
    if pz2.is_null() {
        return 0 as libc::c_int;
    }
    while !(*pz1).is_null() && !(*pz2).is_null() {
        if strcmp(*pz1, *pz2) != 0 as libc::c_int {
            break;
        }
        pz1 = pz1.offset(1);
        pz1;
        pz2 = pz2.offset(1);
        pz2;
    }
    return ((*pz1).is_null() && (*pz2).is_null()) as libc::c_int;
}
unsafe extern "C" fn uvadd_perm(mut qadd: *mut shpermissions) {
    let mut qlook: *mut shpermissions = 0 as *mut shpermissions;
    let mut qnew: *mut shpermissions = 0 as *mut shpermissions;
    let mut iret: libc::c_int = 0;
    if ((*qadd).pzlogname).is_null() && (*qadd).frequest < 0 as libc::c_int
        && (*qadd).fsendfiles < 0 as libc::c_int && ((*qadd).pzread).is_null()
        && ((*qadd).pzwrite).is_null() && (*qadd).fcallback < 0 as libc::c_int
        && ((*qadd).pzcommands).is_null() && ((*qadd).pzvalidate).is_null()
        && ((*qadd).zmyname).is_null() && ((*qadd).zpubdir).is_null()
        && ((*qadd).pzalias).is_null()
    {
        return;
    }
    qlook = qVperms;
    while !qlook.is_null() {
        if !(if ((*qadd).pzlogname).is_null() {
            ((*qlook).pzlogname != 0 as *mut libc::c_void as *mut *mut libc::c_char)
                as libc::c_int
        } else {
            ((*qlook).pzlogname == 0 as *mut libc::c_void as *mut *mut libc::c_char)
                as libc::c_int
        } != 0)
        {
            if !(if ((*qadd).pzmachine).is_null() {
                ((*qlook).pzmachine != 0 as *mut libc::c_void as *mut *mut libc::c_char)
                    as libc::c_int
            } else {
                ((*qlook).pzmachine == 0 as *mut libc::c_void as *mut *mut libc::c_char)
                    as libc::c_int
            } != 0)
            {
                if !((*qadd).frequest != (*qlook).frequest
                    || (*qadd).fsendfiles != (*qlook).fsendfiles
                    || (*qadd).fcallback != (*qlook).fcallback)
                {
                    if !(fvperm_string_cmp((*qadd).zmyname, (*qlook).zmyname) == 0
                        || fvperm_string_cmp((*qadd).zpubdir, (*qlook).zpubdir) == 0)
                    {
                        if !(fvperm_array_cmp(
                            (*qadd).pzread as *mut *const libc::c_char,
                            (*qlook).pzread as *mut *const libc::c_char,
                        ) == 0
                            || fvperm_array_cmp(
                                (*qadd).pzwrite as *mut *const libc::c_char,
                                (*qlook).pzwrite as *mut *const libc::c_char,
                            ) == 0
                            || fvperm_array_cmp(
                                (*qadd).pzcommands as *mut *const libc::c_char,
                                (*qlook).pzcommands as *mut *const libc::c_char,
                            ) == 0)
                        {
                            if !((*qadd).pzmachine).is_null() {
                                iret = _uuconf_iadd_string(
                                    0 as *mut libc::c_void as *mut sglobal,
                                    *((*qadd).pzmachine).offset(0 as libc::c_int as isize),
                                    0 as libc::c_int,
                                    1 as libc::c_int,
                                    &mut (*qlook).pzmachine,
                                    0 as *mut libc::c_void,
                                );
                                if iret != 0 as libc::c_int {
                                    uvuuconf_error(0 as *mut libc::c_void, iret);
                                }
                            }
                            if !((*qadd).pzlogname).is_null() {
                                iret = _uuconf_iadd_string(
                                    0 as *mut libc::c_void as *mut sglobal,
                                    *((*qadd).pzlogname).offset(0 as libc::c_int as isize),
                                    0 as libc::c_int,
                                    1 as libc::c_int,
                                    &mut (*qlook).pzlogname,
                                    0 as *mut libc::c_void,
                                );
                                if iret != 0 as libc::c_int {
                                    uvuuconf_error(0 as *mut libc::c_void, iret);
                                }
                            }
                            if !((*qadd).pzalias).is_null() {
                                let mut pz: *mut *mut libc::c_char = 0
                                    as *mut *mut libc::c_char;
                                pz = (*qadd).pzalias;
                                while !(*pz).is_null() {
                                    iret = _uuconf_iadd_string(
                                        0 as *mut libc::c_void as *mut sglobal,
                                        *pz,
                                        0 as libc::c_int,
                                        1 as libc::c_int,
                                        &mut (*qlook).pzalias,
                                        0 as *mut libc::c_void,
                                    );
                                    if iret != 0 as libc::c_int {
                                        uvuuconf_error(0 as *mut libc::c_void, iret);
                                    }
                                    pz = pz.offset(1);
                                    pz;
                                }
                            }
                            return;
                        }
                    }
                }
            }
        }
        qlook = (*qlook).qnext;
    }
    qnew = malloc(::std::mem::size_of::<shpermissions>() as libc::c_ulong)
        as *mut shpermissions;
    if qnew.is_null() {
        uvuuconf_error(0 as *mut libc::c_void, 4 as libc::c_int);
    }
    *qnew = *qadd;
    if !((*qadd).pzmachine).is_null() {
        (*qnew).pzmachine = 0 as *mut *mut libc::c_char;
        iret = _uuconf_iadd_string(
            0 as *mut libc::c_void as *mut sglobal,
            *((*qadd).pzmachine).offset(0 as libc::c_int as isize),
            0 as libc::c_int,
            0 as libc::c_int,
            &mut (*qnew).pzmachine,
            0 as *mut libc::c_void,
        );
        if iret != 0 as libc::c_int {
            uvuuconf_error(0 as *mut libc::c_void, iret);
        }
    }
    if !((*qadd).pzlogname).is_null() {
        (*qnew).pzlogname = 0 as *mut *mut libc::c_char;
        iret = _uuconf_iadd_string(
            0 as *mut libc::c_void as *mut sglobal,
            *((*qadd).pzlogname).offset(0 as libc::c_int as isize),
            0 as libc::c_int,
            0 as libc::c_int,
            &mut (*qnew).pzlogname,
            0 as *mut libc::c_void,
        );
        if iret != 0 as libc::c_int {
            uvuuconf_error(0 as *mut libc::c_void, iret);
        }
    }
    if !((*qadd).pzvalidate).is_null() {
        (*qnew).pzvalidate = (*qnew).pzmachine;
    }
    (*qnew).qnext = qVperms;
    qVperms = qnew;
}
unsafe extern "C" fn uvwrite_perms() {
    let mut ab: [libc::c_char; 14] = [0; 14];
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut q: *mut shpermissions = 0 as *mut shpermissions;
    sprintf(
        ab.as_mut_ptr(),
        b"%s%s\0" as *const u8 as *const libc::c_char,
        b".\0" as *const u8 as *const libc::c_char,
        b"/Permissions\0" as *const u8 as *const libc::c_char,
    );
    e = fopen(ab.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        fprintf(
            stderr,
            b"uuchk:%s: \0" as *const u8 as *const libc::c_char,
            ab.as_mut_ptr(),
        );
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    fprintf(
        e,
        b"# Permissions file automatically generated by uuconv.\n\0" as *const u8
            as *const libc::c_char,
    );
    q = qVperms;
    while !q.is_null() {
        let mut ccol: size_t = 0;
        ccol = 0 as libc::c_int as size_t;
        uvwrite_perm_array(
            e,
            (*q).pzlogname as *mut *const libc::c_char,
            b"LOGNAME\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_array(
            e,
            (*q).pzmachine as *mut *const libc::c_char,
            b"MACHINE\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_boolean(
            e,
            (*q).frequest,
            b"REQUEST\0" as *const u8 as *const libc::c_char,
            &mut ccol,
            0 as libc::c_int,
        );
        uvwrite_perm_boolean(
            e,
            (*q).fsendfiles,
            b"SENDFILES\0" as *const u8 as *const libc::c_char,
            &mut ccol,
            1 as libc::c_int,
        );
        uvwrite_perm_rw_array(
            e,
            (*q).pzread as *mut *const libc::c_char,
            b"READ\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_rw_array(
            e,
            (*q).pzwrite as *mut *const libc::c_char,
            b"WRITE\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_boolean(
            e,
            (*q).fcallback,
            b"CALLBACK\0" as *const u8 as *const libc::c_char,
            &mut ccol,
            0 as libc::c_int,
        );
        uvwrite_perm_array(
            e,
            (*q).pzcommands as *mut *const libc::c_char,
            b"COMMANDS\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_array(
            e,
            (*q).pzvalidate as *mut *const libc::c_char,
            b"VALIDATE\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_string(
            e,
            (*q).zmyname,
            b"MYNAME\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_string(
            e,
            (*q).zpubdir,
            b"PUBDIR\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        uvwrite_perm_array(
            e,
            (*q).pzalias as *mut *const libc::c_char,
            b"ALIAS\0" as *const u8 as *const libc::c_char,
            &mut ccol,
        );
        fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
        q = (*q).qnext;
    }
    if ferror(e) != 0 || fclose(e) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"uuchk:%s: error during output\n\0" as *const u8 as *const libc::c_char,
            b"/Permissions\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn uvwrite_perm_array(
    mut e: *mut FILE,
    mut pzarg: *mut *const libc::c_char,
    mut zcmd: *const libc::c_char,
    mut pccol: *mut size_t,
) {
    let mut c: size_t = 0;
    let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if pzarg.is_null() {
        return;
    }
    c = (strlen(zcmd)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    pz = pzarg;
    while !(*pz).is_null() {
        c = (c as libc::c_ulong)
            .wrapping_add((strlen(*pz)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        pz = pz.offset(1);
        pz;
    }
    if *pccol > 20 as libc::c_int as libc::c_ulong
        && c.wrapping_add(*pccol) > 75 as libc::c_int as libc::c_ulong
    {
        fprintf(e, b" \\\n\0" as *const u8 as *const libc::c_char);
        *pccol = c.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    } else {
        if *pccol != 0 as libc::c_int as libc::c_ulong {
            fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        }
        *pccol = (*pccol as libc::c_ulong).wrapping_add(c) as size_t as size_t;
    }
    fprintf(e, b"%s=\0" as *const u8 as *const libc::c_char, zcmd);
    pz = pzarg;
    while !(*pz).is_null() {
        if pz != pzarg {
            fprintf(e, b":\0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, b"%s\0" as *const u8 as *const libc::c_char, *pz);
        pz = pz.offset(1);
        pz;
    }
}
unsafe extern "C" fn uvwrite_perm_boolean(
    mut e: *mut FILE,
    mut f: libc::c_int,
    mut zcmd: *const libc::c_char,
    mut pccol: *mut size_t,
    mut fsendfiles: boolean,
) {
    let mut az: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    if f < 0 as libc::c_int {
        return;
    }
    if f != 0 {
        az[0 as libc::c_int as usize] = b"yes\0" as *const u8 as *const libc::c_char;
    } else {
        az[0 as libc::c_int
            as usize] = if fsendfiles != 0 {
            b"call\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        };
    }
    az[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    uvwrite_perm_array(e, az.as_mut_ptr(), zcmd, pccol);
}
unsafe extern "C" fn uvwrite_perm_rw_array(
    mut e: *mut FILE,
    mut pzarg: *mut *const libc::c_char,
    mut zcmd: *const libc::c_char,
    mut pccol: *mut size_t,
) {
    let mut c: size_t = 0;
    let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut pzcopy: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut pzset: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if pzarg.is_null() {
        return;
    }
    c = 0 as libc::c_int as size_t;
    pz = pzarg;
    while !(*pz).is_null() {
        c = c.wrapping_add(1);
        c;
        pz = pz.offset(1);
        pz;
    }
    pzcopy = malloc(
        c
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    if pzcopy.is_null() {
        uvuuconf_error(0 as *mut libc::c_void, 4 as libc::c_int);
    }
    pzset = pzcopy;
    pz = pzarg;
    while !(*pz).is_null() {
        if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32 {
            let fresh0 = pzset;
            pzset = pzset.offset(1);
            *fresh0 = *pz;
        }
        pz = pz.offset(1);
        pz;
    }
    *pzset = 0 as *const libc::c_char;
    if pzset != pzcopy {
        uvwrite_perm_array(e, pzcopy, zcmd, pccol);
    }
    pzset = pzcopy;
    pz = pzarg;
    while !(*pz).is_null() {
        if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 {
            let fresh1 = pzset;
            pzset = pzset.offset(1);
            *fresh1 = *pz;
        }
        pz = pz.offset(1);
        pz;
    }
    *pzset = 0 as *const libc::c_char;
    if pzset != pzcopy {
        let mut ab: [libc::c_char; 20] = [0; 20];
        sprintf(ab.as_mut_ptr(), b"NO%s\0" as *const u8 as *const libc::c_char, zcmd);
        uvwrite_perm_array(e, pzcopy, ab.as_mut_ptr(), pccol);
    }
}
unsafe extern "C" fn uvwrite_perm_string(
    mut e: *mut FILE,
    mut z: *const libc::c_char,
    mut zcmd: *const libc::c_char,
    mut pccol: *mut size_t,
) {
    let mut az: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    if z.is_null() {
        return;
    }
    az[0 as libc::c_int as usize] = z;
    az[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    uvwrite_perm_array(e, az.as_mut_ptr(), zcmd, pccol);
}
unsafe extern "C" fn ivwrite_taylor_port(
    mut qport: *mut uuconf_port,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut e: *mut FILE = pinfo as *mut FILE;
    fprintf(
        e,
        b"port %s\n\0" as *const u8 as *const libc::c_char,
        (*qport).uuconf_zname,
    );
    uvwrite_taylor_port(e, qport, b"\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn uvwrite_taylor_port(
    mut e: *mut FILE,
    mut qport: *mut uuconf_port,
    mut zprefix: *const libc::c_char,
) {
    let mut ztype: *const libc::c_char = 0 as *const libc::c_char;
    let mut ab: [libc::c_char; 100] = [0; 100];
    match (*qport).uuconf_ttype as libc::c_uint {
        1 => {
            ztype = b"stdin\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            ztype = b"modem\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            ztype = b"direct\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            ztype = b"tcp\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            ztype = b"tli\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            ztype = b"pipe\0" as *const u8 as *const libc::c_char;
        }
        0 | _ => {
            fprintf(
                stderr,
                b"uuconv: Bad port type\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    fprintf(e, b"%stype %s\n\0" as *const u8 as *const libc::c_char, zprefix, ztype);
    if !((*qport).uuconf_zprotocols).is_null() {
        fprintf(
            e,
            b"%sprotocol %s\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            (*qport).uuconf_zprotocols,
        );
    }
    if !((*qport).uuconf_qproto_params).is_null() {
        uvwrite_proto_params(e, (*qport).uuconf_qproto_params, zprefix);
    }
    if (*qport).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int {
        sprintf(
            ab.as_mut_ptr(),
            b"%sseven-bit\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(
            e,
            ((*qport).uuconf_ireliable & 0o2 as libc::c_int == 0 as libc::c_int)
                as libc::c_int,
            ab.as_mut_ptr(),
        );
        sprintf(
            ab.as_mut_ptr(),
            b"%sreliable\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(
            e,
            ((*qport).uuconf_ireliable & 0o4 as libc::c_int != 0 as libc::c_int)
                as libc::c_int,
            ab.as_mut_ptr(),
        );
        sprintf(
            ab.as_mut_ptr(),
            b"%shalf-duplex\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(
            e,
            ((*qport).uuconf_ireliable & 0o20 as libc::c_int == 0 as libc::c_int)
                as libc::c_int,
            ab.as_mut_ptr(),
        );
    }
    if !((*qport).uuconf_zlockname).is_null() {
        fprintf(
            e,
            b"%slockname %s\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            (*qport).uuconf_zlockname,
        );
    }
    match (*qport).uuconf_ttype as libc::c_uint {
        2 => {
            let mut qm: *mut uuconf_modem_port = 0 as *mut uuconf_modem_port;
            qm = &mut (*qport).uuconf_u.uuconf_smodem;
            if !((*qm).uuconf_zdevice).is_null() {
                fprintf(
                    e,
                    b"%sdevice %s\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qm).uuconf_zdevice,
                );
            }
            if !((*qm).uuconf_zdial_device).is_null() {
                fprintf(
                    e,
                    b"%sdial-device %s\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qm).uuconf_zdial_device,
                );
            }
            if (*qm).uuconf_ibaud != 0 as libc::c_int as libc::c_long {
                fprintf(
                    e,
                    b"%sbaud %ld\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qm).uuconf_ibaud,
                );
            }
            if (*qm).uuconf_ilowbaud != 0 as libc::c_int as libc::c_long {
                fprintf(
                    e,
                    b"%sbaud-range %ld %ld\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qm).uuconf_ilowbaud,
                    (*qm).uuconf_ihighbaud,
                );
            }
            if (*qm).uuconf_fcarrier == 0 {
                fprintf(
                    e,
                    b"%scarrier false\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
            }
            if (*qm).uuconf_fhardflow == 0 {
                fprintf(
                    e,
                    b"%shardflow false\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
            }
            if !((*qm).uuconf_pzdialer).is_null() {
                if (*((*qm).uuconf_pzdialer).offset(1 as libc::c_int as isize)).is_null()
                {
                    fprintf(
                        e,
                        b"%sdialer %s\n\0" as *const u8 as *const libc::c_char,
                        zprefix,
                        *((*qm).uuconf_pzdialer).offset(0 as libc::c_int as isize),
                    );
                } else {
                    sprintf(
                        ab.as_mut_ptr(),
                        b"%sdialer-sequence\0" as *const u8 as *const libc::c_char,
                        zprefix,
                    );
                    uvwrite_string_array(e, (*qm).uuconf_pzdialer, ab.as_mut_ptr());
                }
            }
            if !((*qm).uuconf_qdialer).is_null() {
                sprintf(
                    ab.as_mut_ptr(),
                    b"%sdialer \0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
                uvwrite_taylor_dialer(e, (*qm).uuconf_qdialer, ab.as_mut_ptr());
            }
        }
        3 => {
            let mut qd: *mut uuconf_direct_port = 0 as *mut uuconf_direct_port;
            qd = &mut (*qport).uuconf_u.uuconf_sdirect;
            if !((*qd).uuconf_zdevice).is_null() {
                fprintf(
                    e,
                    b"%sdevice %s\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qd).uuconf_zdevice,
                );
            }
            if (*qd).uuconf_ibaud != 0 as libc::c_int as libc::c_long {
                fprintf(
                    e,
                    b"%sbaud %ld\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qd).uuconf_ibaud,
                );
            }
            if (*qd).uuconf_fcarrier != 0 {
                fprintf(
                    e,
                    b"%scarrier true\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
            }
            if (*qd).uuconf_fhardflow == 0 {
                fprintf(
                    e,
                    b"%shardflow false\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
            }
        }
        4 => {
            if !((*qport).uuconf_u.uuconf_stcp.uuconf_zport).is_null() {
                fprintf(
                    e,
                    b"%sservice %s\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qport).uuconf_u.uuconf_stcp.uuconf_zport,
                );
            }
            if !((*qport).uuconf_u.uuconf_stcp.uuconf_pzdialer).is_null() {
                sprintf(
                    ab.as_mut_ptr(),
                    b"%sdialer-sequence\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
                uvwrite_string_array(
                    e,
                    (*qport).uuconf_u.uuconf_stcp.uuconf_pzdialer,
                    ab.as_mut_ptr(),
                );
            }
        }
        5 => {
            let mut qt: *mut uuconf_tli_port = 0 as *mut uuconf_tli_port;
            qt = &mut (*qport).uuconf_u.uuconf_stli;
            if !((*qt).uuconf_zdevice).is_null() {
                fprintf(
                    e,
                    b"%sdevice %s\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qt).uuconf_zdevice,
                );
            }
            sprintf(
                ab.as_mut_ptr(),
                b"%sstream\0" as *const u8 as *const libc::c_char,
                zprefix,
            );
            uvwrite_boolean(e, (*qt).uuconf_fstream, ab.as_mut_ptr());
            if !((*qt).uuconf_pzpush).is_null() {
                sprintf(
                    ab.as_mut_ptr(),
                    b"%spush\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
                uvwrite_string_array(e, (*qt).uuconf_pzpush, ab.as_mut_ptr());
            }
            if !((*qt).uuconf_pzdialer).is_null() {
                sprintf(
                    ab.as_mut_ptr(),
                    b"%sdialer-sequence\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
                uvwrite_string_array(e, (*qt).uuconf_pzdialer, ab.as_mut_ptr());
            }
            if !((*qt).uuconf_zservaddr).is_null() {
                fprintf(
                    e,
                    b"%sserver-address %s\n\0" as *const u8 as *const libc::c_char,
                    zprefix,
                    (*qt).uuconf_zservaddr,
                );
            }
        }
        6 => {
            let mut qp: *mut uuconf_pipe_port = 0 as *mut uuconf_pipe_port;
            qp = &mut (*qport).uuconf_u.uuconf_spipe;
            if !((*qp).uuconf_pzcmd).is_null() {
                sprintf(
                    ab.as_mut_ptr(),
                    b"%scommad\0" as *const u8 as *const libc::c_char,
                    zprefix,
                );
                uvwrite_string_array(e, (*qp).uuconf_pzcmd, ab.as_mut_ptr());
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn ivwrite_v2_port(
    mut qport: *mut uuconf_port,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut e: *mut FILE = pinfo as *mut FILE;
    if (*qport).uuconf_ttype as libc::c_uint
        == UUCONF_PORTTYPE_DIRECT as libc::c_int as libc::c_uint
    {
        fprintf(
            e,
            b"DIR %s - %ld direct\0" as *const u8 as *const libc::c_char,
            (*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice,
            (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud,
        );
    } else if (*qport).uuconf_ttype as libc::c_uint
        == UUCONF_PORTTYPE_MODEM as libc::c_int as libc::c_uint
    {
        fprintf(
            e,
            b"%s %s \0" as *const u8 as *const libc::c_char,
            (*qport).uuconf_zname,
            (*qport).uuconf_u.uuconf_smodem.uuconf_zdevice,
        );
        if !((*qport).uuconf_u.uuconf_smodem.uuconf_zdial_device).is_null() {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_zdial_device,
            );
        } else {
            fprintf(e, b"-\0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud != 0 as libc::c_long {
            fprintf(
                e,
                b"%ld-%ld\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud,
                (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud,
            );
        } else if (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud != 0 as libc::c_long {
            fprintf(
                e,
                b"%ld\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud,
            );
        } else {
            fprintf(e, b"Any\0" as *const u8 as *const libc::c_char);
        }
        if !((*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer).is_null() {
            fprintf(
                e,
                b" %s\0" as *const u8 as *const libc::c_char,
                *((*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer)
                    .offset(0 as libc::c_int as isize),
            );
        }
    } else {
        fprintf(
            e,
            b"# Ignoring port %s with unsupported type\0" as *const u8
                as *const libc::c_char,
            (*qport).uuconf_zname,
        );
    }
    fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ivwrite_hdb_port(
    mut qport: *mut uuconf_port,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut e: *mut FILE = pinfo as *mut FILE;
    if (*qport).uuconf_ttype as libc::c_uint
        == UUCONF_PORTTYPE_DIRECT as libc::c_int as libc::c_uint
    {
        fprintf(e, b"Direct\0" as *const u8 as *const libc::c_char);
        if !((*qport).uuconf_zprotocols).is_null() {
            fprintf(
                e,
                b",%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zprotocols,
            );
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if !((*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice).is_null() {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice,
            );
        } else {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zname,
            );
        }
        fprintf(
            e,
            b" - %ld\0" as *const u8 as *const libc::c_char,
            (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud,
        );
    } else if (*qport).uuconf_ttype as libc::c_uint
        == UUCONF_PORTTYPE_MODEM as libc::c_int as libc::c_uint
    {
        fprintf(e, b"%s\0" as *const u8 as *const libc::c_char, (*qport).uuconf_zname);
        if !((*qport).uuconf_zprotocols).is_null() {
            fprintf(
                e,
                b",%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zprotocols,
            );
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if !((*qport).uuconf_u.uuconf_smodem.uuconf_zdevice).is_null() {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_zdevice,
            );
        } else {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zname,
            );
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if !((*qport).uuconf_u.uuconf_smodem.uuconf_zdial_device).is_null() {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_zdial_device,
            );
        } else {
            fprintf(e, b"-\0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud != 0 as libc::c_long {
            fprintf(
                e,
                b"%ld-%ld\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_ilowbaud,
                (*qport).uuconf_u.uuconf_smodem.uuconf_ihighbaud,
            );
        } else if (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud != 0 as libc::c_long {
            fprintf(
                e,
                b"%ld\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_ibaud,
            );
        } else {
            fprintf(e, b"Any\0" as *const u8 as *const libc::c_char);
        }
        if !((*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer).is_null() {
            let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            pz = (*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer;
            while !(*pz).is_null() {
                fprintf(e, b" %s\0" as *const u8 as *const libc::c_char, *pz);
                pz = pz.offset(1);
                pz;
            }
        }
    } else if (*qport).uuconf_ttype as libc::c_uint
        == UUCONF_PORTTYPE_TCP as libc::c_int as libc::c_uint
    {
        let mut pz_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        fprintf(e, b"TCP\0" as *const u8 as *const libc::c_char);
        if !((*qport).uuconf_zprotocols).is_null() {
            fprintf(
                e,
                b",%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zprotocols,
            );
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if ((*qport).uuconf_u.uuconf_stcp.uuconf_zport).is_null() {
            fprintf(e, b"uucp\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_stcp.uuconf_zport,
            );
        }
        fprintf(e, b" - -\0" as *const u8 as *const libc::c_char);
        pz_0 = (*qport).uuconf_u.uuconf_stcp.uuconf_pzdialer;
        if !pz_0.is_null() {
            while !(*pz_0).is_null() {
                fprintf(e, b" %s\0" as *const u8 as *const libc::c_char, *pz_0);
                pz_0 = pz_0.offset(1);
                pz_0;
            }
        }
    } else if (*qport).uuconf_ttype as libc::c_uint
        == UUCONF_PORTTYPE_TLI as libc::c_int as libc::c_uint
    {
        let mut pz_1: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        fprintf(e, b"%s\0" as *const u8 as *const libc::c_char, (*qport).uuconf_zname);
        if !((*qport).uuconf_zprotocols).is_null() {
            fprintf(
                e,
                b",%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zprotocols,
            );
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        if !((*qport).uuconf_u.uuconf_stli.uuconf_zdevice).is_null() {
            fprintf(
                e,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_smodem.uuconf_zdevice,
            );
        } else {
            fprintf(e, b"-\0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, b" - -\0" as *const u8 as *const libc::c_char);
        pz_1 = (*qport).uuconf_u.uuconf_stli.uuconf_pzdialer;
        if pz_1.is_null() || (*pz_1).is_null()
            || strcmp(*pz_1, b"TLI\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
                && strcmp(*pz_1, b"TLIS\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
        {
            fprintf(
                e,
                b" TLI%s \\D\0" as *const u8 as *const libc::c_char,
                if (*qport).uuconf_u.uuconf_stli.uuconf_fstream != 0 {
                    b"S\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if !pz_1.is_null() {
            while !(*pz_1).is_null() {
                fprintf(e, b" %s\0" as *const u8 as *const libc::c_char, *pz_1);
                pz_1 = pz_1.offset(1);
                pz_1;
            }
        }
    } else {
        fprintf(
            e,
            b"# Ignoring port %s with unsupported type\0" as *const u8
                as *const libc::c_char,
            (*qport).uuconf_zname,
        );
    }
    fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn uvwrite_taylor_dialer(
    mut e: *mut FILE,
    mut qdialer: *mut uuconf_dialer,
    mut zprefix: *const libc::c_char,
) {
    let mut ab: [libc::c_char; 100] = [0; 100];
    if (*qdialer).uuconf_schat.uuconf_ctimeout == 60 as libc::c_int {
        (*qdialer).uuconf_schat.uuconf_ctimeout = -(1 as libc::c_int);
    }
    if (*qdialer).uuconf_schat.uuconf_fstrip != 0 {
        (*qdialer).uuconf_schat.uuconf_fstrip = -(1 as libc::c_int);
    }
    if (*qdialer).uuconf_scomplete.uuconf_ctimeout == 60 as libc::c_int {
        (*qdialer).uuconf_scomplete.uuconf_ctimeout = -(1 as libc::c_int);
    }
    if (*qdialer).uuconf_scomplete.uuconf_fstrip != 0 {
        (*qdialer).uuconf_scomplete.uuconf_fstrip = -(1 as libc::c_int);
    }
    if (*qdialer).uuconf_sabort.uuconf_ctimeout == 60 as libc::c_int {
        (*qdialer).uuconf_sabort.uuconf_ctimeout = -(1 as libc::c_int);
    }
    if (*qdialer).uuconf_sabort.uuconf_fstrip != 0 {
        (*qdialer).uuconf_sabort.uuconf_fstrip = -(1 as libc::c_int);
    }
    uvwrite_chat(
        e,
        &mut (*qdialer).uuconf_schat,
        0 as *mut libc::c_void as *mut uuconf_chat,
        zprefix,
        0 as libc::c_int,
    );
    if !((*qdialer).uuconf_zdialtone).is_null()
        && strcmp(
            (*qdialer).uuconf_zdialtone,
            b",\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        fprintf(
            e,
            b"%sdialtone %s\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            (*qdialer).uuconf_zdialtone,
        );
    }
    if !((*qdialer).uuconf_zpause).is_null()
        && strcmp((*qdialer).uuconf_zpause, b",\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        fprintf(
            e,
            b"%spause %s\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            (*qdialer).uuconf_zpause,
        );
    }
    if (*qdialer).uuconf_fcarrier == 0 {
        fprintf(e, b"%scarrier false\n\0" as *const u8 as *const libc::c_char, zprefix);
    }
    if (*qdialer).uuconf_ccarrier_wait != 60 as libc::c_int {
        fprintf(
            e,
            b"%scarrier-wait %d\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            (*qdialer).uuconf_ccarrier_wait,
        );
    }
    if (*qdialer).uuconf_fdtr_toggle != 0 {
        fprintf(
            e,
            b"%sdtr-toggle %s %s\n\0" as *const u8 as *const libc::c_char,
            zprefix,
            if (*qdialer).uuconf_fdtr_toggle != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            if (*qdialer).uuconf_fdtr_toggle_wait != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    }
    sprintf(
        ab.as_mut_ptr(),
        b"%scomplete-\0" as *const u8 as *const libc::c_char,
        zprefix,
    );
    uvwrite_chat(
        e,
        &mut (*qdialer).uuconf_scomplete,
        0 as *mut libc::c_void as *mut uuconf_chat,
        ab.as_mut_ptr(),
        0 as libc::c_int,
    );
    sprintf(ab.as_mut_ptr(), b"%sabort-\0" as *const u8 as *const libc::c_char, zprefix);
    uvwrite_chat(
        e,
        &mut (*qdialer).uuconf_sabort,
        0 as *mut libc::c_void as *mut uuconf_chat,
        ab.as_mut_ptr(),
        0 as libc::c_int,
    );
    if !((*qdialer).uuconf_qproto_params).is_null() {
        uvwrite_proto_params(e, (*qdialer).uuconf_qproto_params, zprefix);
    }
    if (*qdialer).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int {
        sprintf(
            ab.as_mut_ptr(),
            b"%sseven-bit\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(
            e,
            ((*qdialer).uuconf_ireliable & 0o2 as libc::c_int == 0 as libc::c_int)
                as libc::c_int,
            ab.as_mut_ptr(),
        );
        sprintf(
            ab.as_mut_ptr(),
            b"%sreliable\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(
            e,
            ((*qdialer).uuconf_ireliable & 0o4 as libc::c_int != 0 as libc::c_int)
                as libc::c_int,
            ab.as_mut_ptr(),
        );
        sprintf(
            ab.as_mut_ptr(),
            b"%shalf-duplex\0" as *const u8 as *const libc::c_char,
            zprefix,
        );
        uvwrite_boolean(
            e,
            ((*qdialer).uuconf_ireliable & 0o20 as libc::c_int == 0 as libc::c_int)
                as libc::c_int,
            ab.as_mut_ptr(),
        );
    }
}
unsafe extern "C" fn uvwrite_hdb_dialer(
    mut e: *mut FILE,
    mut qdialer: *mut uuconf_dialer,
) {
    fprintf(e, b"%s \0" as *const u8 as *const libc::c_char, (*qdialer).uuconf_zname);
    if !((*qdialer).uuconf_zdialtone).is_null() {
        fprintf(
            e,
            b"=%c\0" as *const u8 as *const libc::c_char,
            *((*qdialer).uuconf_zdialtone).offset(0 as libc::c_int as isize)
                as libc::c_int,
        );
    }
    if !((*qdialer).uuconf_zpause).is_null() {
        fprintf(
            e,
            b"-%c\0" as *const u8 as *const libc::c_char,
            *((*qdialer).uuconf_zpause).offset(0 as libc::c_int as isize) as libc::c_int,
        );
    }
    if !((*qdialer).uuconf_schat.uuconf_pzchat).is_null() {
        if ((*qdialer).uuconf_zdialtone).is_null()
            && ((*qdialer).uuconf_zpause).is_null()
        {
            fprintf(e, b"\"\"\0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, b" \0" as *const u8 as *const libc::c_char);
        uvwrite_chat_script(e, (*qdialer).uuconf_schat.uuconf_pzchat);
    }
    fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn uvuuconf_error(mut puuconf: pointer, mut iret: libc::c_int) {
    let mut ab: [libc::c_char; 512] = [0; 512];
    uuconf_error_string(
        puuconf,
        iret,
        ab.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if iret & 0x200 as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"uuconv: %s\n\0" as *const u8 as *const libc::c_char,
            ab.as_mut_ptr(),
        );
    } else {
        fprintf(
            stderr,
            b"uuconv:%s\n\0" as *const u8 as *const libc::c_char,
            ab.as_mut_ptr(),
        );
    }
    if iret & 0xff as libc::c_int != 2 as libc::c_int {
        exit(1 as libc::c_int);
    }
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

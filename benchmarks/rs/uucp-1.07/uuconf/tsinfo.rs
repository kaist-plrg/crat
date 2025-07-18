use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn rewind(__stream: *mut FILE);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _uuconf_ucmdtab_base(
        qoff: *const cmdtab_offset,
        celes: size_t,
        pbase: *mut libc::c_char,
        qset: *mut uuconf_cmdtab,
    );
    fn _uuconf_isystem_basic_default(
        qglobal: *mut sglobal,
        qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn _uuconf_uclear_system(qsys: *mut uuconf_system);
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn _uuconf_isystem_default(
        qglobal: *mut sglobal,
        q: *mut uuconf_system,
        qdefault: *mut uuconf_system,
        faddalternates: boolean,
    ) -> libc::c_int;
    static mut _uuconf_unset: *mut libc::c_char;
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
    fn uuconf_malloc_block() -> *mut libc::c_void;
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_iboolean(
        qglobal: *mut sglobal,
        zval: *const libc::c_char,
        pi: *mut libc::c_int,
    ) -> libc::c_int;
    fn _uuconf_idebug_cmd(
        qglobal: *mut sglobal,
        pzdebug: *mut *mut libc::c_char,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pblock: pointer,
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
    fn _uuconf_iread_locations(qglobal: *mut sglobal) -> libc::c_int;
    fn _uuconf_iport_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qport: *mut uuconf_port,
    ) -> libc::c_int;
    fn _uuconf_uclear_port(qport: *mut uuconf_port);
    fn _uuconf_iint(
        qglobal: *mut sglobal,
        zval: *const libc::c_char,
        p: pointer,
        fint: boolean,
    ) -> libc::c_int;
    fn _uuconf_itimetable(
        pglobal: pointer,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        pvar: pointer,
        pinfo: pointer,
    ) -> libc::c_int;
    fn _uuconf_itime_parse(
        qglobal: *mut sglobal,
        ztime: *mut libc::c_char,
        ival: libc::c_long,
        cretry: libc::c_int,
        picmp: Option::<unsafe extern "C" fn(libc::c_long, libc::c_long) -> libc::c_int>,
        pqspan: *mut *mut uuconf_timespan,
        pblock: pointer,
    ) -> libc::c_int;
    fn _uuconf_itime_grade_cmp(_: libc::c_long, _: libc::c_long) -> libc::c_int;
    fn uuconf_cmd_file(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_e: *mut FILE,
        uuconf_qtab: *const uuconf_cmdtab,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_pfiunknownfn: uuconf_cmdtabfn,
        uuconf_iflags: libc::c_int,
        pblock: *mut libc::c_void,
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
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sinfo {
    pub qsys: *mut uuconf_system,
    pub falternates: boolean,
    pub salternate: uuconf_system,
    pub fdefault_alternates: libc::c_int,
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
pub struct cmdtab_offset {
    pub zcmd: *const libc::c_char,
    pub itype: libc::c_int,
    pub ioff: size_t,
    pub pifn: uuconf_cmdtabfn,
}
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub static mut _uuconf_tsinfo_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: tsinfo.c,v 1.21 2002/03/05 19:10:43 ian Rel $\0")
};
static mut asIcmds: [cmdtab_offset; 53] = unsafe {
    [
        {
            let mut init = cmdtab_offset {
                zcmd: b"system\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iisystem
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
                zcmd: b"alias\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iialias
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
                zcmd: b"alternate\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iialternate
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
                zcmd: b"default-alternates\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iidefault_alternates
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
                zcmd: b"time\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 40 as libc::c_ulong,
                pifn: Some(
                    iitime
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
                zcmd: b"timegrade\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 40 as libc::c_ulong,
                pifn: Some(
                    iitimegrade
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
                zcmd: b"max-retries\0" as *const u8 as *const libc::c_char,
                itype: 0x22 as libc::c_int,
                ioff: 64 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"success-wait\0" as *const u8 as *const libc::c_char,
                itype: 0x22 as libc::c_int,
                ioff: 68 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"call-timegrade\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 48 as libc::c_ulong,
                pifn: Some(
                    iitimegrade
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
                zcmd: b"called-timegrade\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 56 as libc::c_ulong,
                pifn: Some(
                    iitimegrade
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
                zcmd: b"call-local-size\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 72 as libc::c_ulong,
                pifn: Some(
                    iisize
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
                zcmd: b"call-remote-size\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 80 as libc::c_ulong,
                pifn: Some(
                    iisize
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
                zcmd: b"called-local-size\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 88 as libc::c_ulong,
                pifn: Some(
                    iisize
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
                zcmd: b"called-remote-size\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 96 as libc::c_ulong,
                pifn: Some(
                    iisize
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
                zcmd: b"timetable\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    _uuconf_itimetable
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
                zcmd: b"baud\0" as *const u8 as *const libc::c_char,
                itype: 0x32 as libc::c_int,
                ioff: 104 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"speed\0" as *const u8 as *const libc::c_char,
                itype: 0x32 as libc::c_int,
                ioff: 104 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"baud-range\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 3 as libc::c_int,
                ioff: 0 as libc::c_int as size_t,
                pifn: Some(
                    iibaud_range
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
                ioff: 0 as libc::c_int as size_t,
                pifn: Some(
                    iibaud_range
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
                zcmd: b"port\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iiport
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
                zcmd: b"phone\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 136 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"address\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 136 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"chat\0" as *const u8 as *const libc::c_char,
                itype: 0x70 as libc::c_int | 0 as libc::c_int,
                ioff: 144 as libc::c_ulong,
                pifn: Some(
                    iichat
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
                zcmd: b"call-login\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 184 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"call-password\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 192 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"called-login\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 200 as libc::c_ulong,
                pifn: Some(
                    iicalled_login
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
                zcmd: b"callback\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 208 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"sequence\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 212 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"protocol\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 216 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"protocol-parameter\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 224 as libc::c_ulong,
                pifn: Some(
                    iiproto_param
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
                zcmd: b"called-chat\0" as *const u8 as *const libc::c_char,
                itype: 0x70 as libc::c_int | 0 as libc::c_int,
                ioff: 232 as libc::c_ulong,
                pifn: Some(
                    iichat
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
                zcmd: b"debug\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: 272 as libc::c_ulong,
                pifn: Some(
                    iidebug
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
                zcmd: b"max-remote-debug\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 280 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"send-request\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 288 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"receive-request\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 292 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"request\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iirequest
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
                zcmd: b"call-transfer\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 296 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"called-transfer\0" as *const u8 as *const libc::c_char,
                itype: 0x12 as libc::c_int,
                ioff: 300 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"transfer\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 2 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iitransfer
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
                zcmd: b"local-send\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 304 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"remote-send\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 312 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"local-receive\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 320 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"remote-receive\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 328 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"command-path\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 336 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"commands\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 344 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"free-space\0" as *const u8 as *const libc::c_char,
                itype: 0x32 as libc::c_int,
                ioff: 352 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"forward-from\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 360 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"forward-to\0" as *const u8 as *const libc::c_char,
                itype: 0x50 as libc::c_int,
                ioff: 368 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"forward\0" as *const u8 as *const libc::c_char,
                itype: 0x60 as libc::c_int | 0 as libc::c_int,
                ioff: -(1 as libc::c_int) as size_t,
                pifn: Some(
                    iiforward
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
                zcmd: b"pubdir\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 376 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"myname\0" as *const u8 as *const libc::c_char,
                itype: 0x40 as libc::c_int,
                ioff: 384 as libc::c_ulong,
                pifn: None,
            };
            init
        },
        {
            let mut init = cmdtab_offset {
                zcmd: b"max-file-time\0" as *const u8 as *const libc::c_char,
                itype: 0x32 as libc::c_int,
                ioff: 392 as libc::c_ulong,
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
pub unsafe extern "C" fn _uuconf_itaylor_system_internal(
    mut qglobal: *mut sglobal,
    mut zsystem: *const libc::c_char,
    mut qsys: *mut uuconf_system,
) -> libc::c_int {
    let mut iret: libc::c_int = 0;
    let mut qloc: *mut stsysloc = 0 as *mut stsysloc;
    let mut as_0: [uuconf_cmdtab; 53] = [uuconf_cmdtab {
        uuconf_zcmd: 0 as *const libc::c_char,
        uuconf_itype: 0,
        uuconf_pvar: 0 as *mut libc::c_void,
        uuconf_pifn: None,
    }; 53];
    let mut si: sinfo = sinfo {
        qsys: 0 as *mut uuconf_system,
        falternates: 0,
        salternate: uuconf_system {
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
        },
        fdefault_alternates: 0,
    };
    let mut sdefaults: uuconf_system = uuconf_system {
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
    if (*(*qglobal).qprocess).fread_syslocs == 0 {
        iret = _uuconf_iread_locations(qglobal);
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    qloc = (*(*qglobal).qprocess).qsyslocs;
    while !qloc.is_null() {
        if *((*qloc).zname).offset(0 as libc::c_int as isize) as libc::c_int
            == *zsystem.offset(0 as libc::c_int as isize) as libc::c_int
            && strcmp((*qloc).zname, zsystem) == 0 as libc::c_int
        {
            break;
        }
        qloc = (*qloc).qnext;
    }
    if qloc.is_null() {
        return 1 as libc::c_int;
    }
    while (*qloc).falias != 0 {
        qloc = (*qloc).qnext;
        if qloc.is_null() {
            return 1 as libc::c_int;
        }
    }
    _uuconf_ucmdtab_base(
        asIcmds.as_ptr(),
        (::std::mem::size_of::<[cmdtab_offset; 53]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong),
        qsys as *mut libc::c_char,
        as_0.as_mut_ptr(),
    );
    rewind((*qloc).e);
    _uuconf_uclear_system(qsys);
    si.qsys = qsys;
    si.falternates = 0 as libc::c_int;
    si.fdefault_alternates = 1 as libc::c_int;
    (*qsys).uuconf_palloc = uuconf_malloc_block();
    if ((*qsys).uuconf_palloc).is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int;
    }
    iret = uuconf_cmd_file(
        qglobal as pointer,
        (*qloc).e,
        as_0.as_mut_ptr(),
        &mut si as *mut sinfo as pointer,
        Some(
            iiunknown
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0x2 as libc::c_int,
        (*qsys).uuconf_palloc,
    );
    if iret != 0 as libc::c_int {
        (*qglobal).zfilename = (*qloc).zfile;
        return iret | 0x200 as libc::c_int;
    }
    if si.falternates == 0 {
        uiset_call(qsys);
    } else {
        iret = iialternate(
            qglobal as pointer,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            0 as *mut libc::c_void,
            &mut si as *mut sinfo as pointer,
        );
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    sdefaults = *qsys;
    if fseek((*qloc).e, (*qloc).iloc, 0 as libc::c_int) != 0 as libc::c_int {
        (*qglobal).ierrno = *__errno_location();
        (*qglobal).zfilename = (*qloc).zfile;
        return 3 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int;
    }
    _uuconf_uclear_system(qsys);
    (*qsys).uuconf_zname = (*qloc).zname as *mut libc::c_char;
    (*qsys).uuconf_palloc = sdefaults.uuconf_palloc;
    si.falternates = 0 as libc::c_int;
    iret = uuconf_cmd_file(
        qglobal as *mut libc::c_void,
        (*qloc).e,
        as_0.as_mut_ptr(),
        &mut si as *mut sinfo as pointer,
        Some(
            iiunknown
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0x2 as libc::c_int,
        (*qsys).uuconf_palloc,
    );
    (*qglobal).ilineno += (*qloc).ilineno;
    if iret == 0 as libc::c_int {
        if si.falternates == 0 {
            uiset_call(qsys);
        } else {
            iret = iialternate(
                qglobal as pointer,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void,
                &mut si as *mut sinfo as pointer,
            );
        }
    }
    if iret == 0 as libc::c_int {
        iret = _uuconf_isystem_default(
            qglobal,
            qsys,
            &mut sdefaults,
            si.fdefault_alternates,
        );
    }
    if iret == 0 as libc::c_int {
        (*qsys).uuconf_fcalled = 1 as libc::c_int;
        if (*qsys).uuconf_zport
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
            || (*qsys).uuconf_qport
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
            || (*qsys).uuconf_ibaud >= 0 as libc::c_int as libc::c_long
            || (*qsys).uuconf_zphone
                != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qsys).uuconf_fcall = 1 as libc::c_int;
        }
    }
    if iret != 0 as libc::c_int {
        (*qglobal).zfilename = (*qloc).zfile;
        iret |= 0x200 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn uiset_call(mut qsys: *mut uuconf_system) {
    (*qsys)
        .uuconf_fcall = ((*qsys).uuconf_qtimegrade
        != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        || (*qsys).uuconf_zport
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        || (*qsys).uuconf_qport
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
        || (*qsys).uuconf_ibaud >= 0 as libc::c_int as libc::c_long
        || (*qsys).uuconf_zphone
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        || (*qsys).uuconf_schat.uuconf_pzchat
            != &mut _uuconf_unset as *mut *mut libc::c_char
        || (*qsys).uuconf_schat.uuconf_pzprogram
            != &mut _uuconf_unset as *mut *mut libc::c_char) as libc::c_int;
    (*qsys)
        .uuconf_fcalled = ((*qsys).uuconf_zcalled_login
        != &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char)
        as libc::c_int;
}
unsafe extern "C" fn iisystem(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    return 0x1000 as libc::c_int;
}
unsafe extern "C" fn iialias(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iadd_string(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        1 as libc::c_int,
        0 as libc::c_int,
        &mut (*(*qinfo).qsys).uuconf_pzalias,
        (*(*qinfo).qsys).uuconf_palloc,
    );
    if iret != 0 as libc::c_int {
        iret |= 0x1000 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn iialternate(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    uiset_call((*qinfo).qsys);
    if (*qinfo).falternates == 0 {
        (*qinfo).salternate = *(*qinfo).qsys;
        (*qinfo).falternates = 1 as libc::c_int;
    } else {
        let mut iret: libc::c_int = 0;
        let mut qnew: *mut uuconf_system = 0 as *mut uuconf_system;
        let mut pq: *mut *mut uuconf_system = 0 as *mut *mut uuconf_system;
        iret = _uuconf_isystem_default(
            qglobal,
            (*qinfo).qsys,
            &mut (*qinfo).salternate,
            0 as libc::c_int,
        );
        if iret != 0 as libc::c_int {
            return iret | 0x1000 as libc::c_int;
        }
        qnew = uuconf_malloc(
            (*(*qinfo).qsys).uuconf_palloc,
            ::std::mem::size_of::<uuconf_system>() as libc::c_ulong,
        ) as *mut uuconf_system;
        if qnew.is_null() {
            (*qglobal).ierrno = *__errno_location();
            return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
        }
        *qnew = *(*qinfo).qsys;
        pq = &mut (*qinfo).salternate.uuconf_qalternate;
        while !(*pq).is_null() {
            pq = &mut (**pq).uuconf_qalternate;
        }
        *pq = qnew;
    }
    if argc == 0 as libc::c_int {
        *(*qinfo).qsys = (*qinfo).salternate;
    } else {
        _uuconf_uclear_system((*qinfo).qsys);
        (*(*qinfo).qsys).uuconf_zname = (*qinfo).salternate.uuconf_zname;
        (*(*qinfo).qsys).uuconf_palloc = (*qinfo).salternate.uuconf_palloc;
        if argc > 1 as libc::c_int {
            (*(*qinfo).qsys).uuconf_zalternate = *argv.offset(1 as libc::c_int as isize);
            return 0x800 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iidefault_alternates(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    return _uuconf_iboolean(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut (*qinfo).fdefault_alternates,
    );
}
unsafe extern "C" fn iitime(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut aznew: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    let mut ab: [libc::c_char; 2] = [0; 2];
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    aznew[0 as libc::c_int as usize] = *argv.offset(0 as libc::c_int as isize);
    ab[0 as libc::c_int as usize] = 'z' as i32 as libc::c_char;
    ab[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    aznew[1 as libc::c_int as usize] = ab.as_mut_ptr();
    aznew[2 as libc::c_int as usize] = *argv.offset(1 as libc::c_int as isize);
    if argc > 2 as libc::c_int {
        aznew[3 as libc::c_int as usize] = *argv.offset(2 as libc::c_int as isize);
    }
    return iitimegrade(
        pglobal,
        argc + 1 as libc::c_int,
        aznew.as_mut_ptr(),
        pvar,
        pinfo,
    );
}
unsafe extern "C" fn iitimegrade(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pqspan: *mut *mut uuconf_timespan = pvar as *mut *mut uuconf_timespan;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut cretry: libc::c_int = 0;
    let mut iret: libc::c_int = 0;
    if argc < 3 as libc::c_int || argc > 4 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    if *(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
        as libc::c_int != '\0' as i32
        || *(*__ctype_b_loc())
            .offset(
                *(*argv.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    if argc == 3 as libc::c_int {
        cretry = 0 as libc::c_int;
    } else {
        iret = _uuconf_iint(
            qglobal,
            *argv.offset(3 as libc::c_int as isize),
            &mut cretry as *mut libc::c_int as pointer,
            1 as libc::c_int,
        );
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    iret = _uuconf_itime_parse(
        qglobal,
        *argv.offset(2 as libc::c_int as isize),
        *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_long,
        cretry,
        Some(
            _uuconf_itime_grade_cmp
                as unsafe extern "C" fn(libc::c_long, libc::c_long) -> libc::c_int,
        ),
        pqspan,
        (*(*qinfo).qsys).uuconf_palloc,
    );
    if iret != 0 as libc::c_int {
        iret |= 0x1000 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn iibaud_range(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qsys: *mut uuconf_system = pvar as *mut uuconf_system;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iint(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut (*qsys).uuconf_ibaud as *mut libc::c_long as pointer,
        0 as libc::c_int,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    return _uuconf_iint(
        qglobal,
        *argv.offset(2 as libc::c_int as isize),
        &mut (*qsys).uuconf_ihighbaud as *mut libc::c_long as pointer,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn iisize(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pqspan: *mut *mut uuconf_timespan = pvar as *mut *mut uuconf_timespan;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut ival: libc::c_long = 0;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iint(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut ival as *mut libc::c_long as pointer,
        0 as libc::c_int,
    );
    if iret != 0 as libc::c_int {
        return iret;
    }
    iret = _uuconf_itime_parse(
        qglobal,
        *argv.offset(2 as libc::c_int as isize),
        ival,
        0 as libc::c_int,
        Some(
            iisizecmp as unsafe extern "C" fn(libc::c_long, libc::c_long) -> libc::c_int,
        ),
        pqspan,
        (*(*qinfo).qsys).uuconf_palloc,
    );
    if iret != 0 as libc::c_int {
        iret |= 0x1000 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn iisizecmp(
    mut i1: libc::c_long,
    mut i2: libc::c_long,
) -> libc::c_int {
    if i1 < i2 {
        return -(1 as libc::c_int)
    } else if i1 == i2 {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn iiport(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    if argc < 2 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int
    } else if argc == 2 as libc::c_int {
        (*(*qinfo).qsys).uuconf_zport = *argv.offset(1 as libc::c_int as isize);
        return 0x800 as libc::c_int;
    } else {
        let mut iret: libc::c_int = 0;
        if (*(*qinfo).qsys).uuconf_qport
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
        {
            let mut qnew: *mut uuconf_port = 0 as *mut uuconf_port;
            qnew = uuconf_malloc(
                (*(*qinfo).qsys).uuconf_palloc,
                ::std::mem::size_of::<uuconf_port>() as libc::c_ulong,
            ) as *mut uuconf_port;
            if qnew.is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int | 0x1000 as libc::c_int;
            }
            _uuconf_uclear_port(qnew);
            if ((*(*qinfo).qsys).uuconf_zname).is_null() {
                (*qnew)
                    .uuconf_zname = b"default system file port\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            } else {
                let mut zname: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut clen: size_t = 0;
                clen = strlen((*(*qinfo).qsys).uuconf_zname);
                zname = uuconf_malloc(
                    (*(*qinfo).qsys).uuconf_palloc,
                    clen
                        .wrapping_add(
                            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                if zname.is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    return 4 as libc::c_int | 0x100 as libc::c_int
                        | 0x1000 as libc::c_int;
                }
                memcpy(
                    zname as pointer,
                    b"system \0" as *const u8 as *const libc::c_char as pointer
                        as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                memcpy(
                    zname
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(-(1 as libc::c_int as isize)) as pointer,
                    (*(*qinfo).qsys).uuconf_zname as pointer as *const libc::c_void,
                    clen,
                );
                memcpy(
                    zname
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(-(1 as libc::c_int as isize))
                        .offset(clen as isize) as pointer,
                    b" port\0" as *const u8 as *const libc::c_char as pointer
                        as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                );
                (*qnew).uuconf_zname = zname;
            }
            (*qnew).uuconf_palloc = (*(*qinfo).qsys).uuconf_palloc;
            (*(*qinfo).qsys).uuconf_qport = qnew;
        }
        iret = _uuconf_iport_cmd(
            qglobal,
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
            (*(*qinfo).qsys).uuconf_qport,
        );
        if iret & 0xff as libc::c_int != 0 as libc::c_int {
            iret |= 0x1000 as libc::c_int;
        }
        return iret;
    };
}
unsafe extern "C" fn iichat(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut qchat: *mut uuconf_chat = pvar as *mut uuconf_chat;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_ichat_cmd(qglobal, argc, argv, qchat, (*(*qinfo).qsys).uuconf_palloc);
    if iret & 0xff as libc::c_int != 0 as libc::c_int {
        iret |= 0x1000 as libc::c_int;
    }
    return iret;
}
unsafe extern "C" fn iidebug(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut pzdebug: *mut *mut libc::c_char = pvar as *mut *mut libc::c_char;
    return _uuconf_idebug_cmd(
        qglobal,
        pzdebug,
        argc,
        argv,
        (*(*qinfo).qsys).uuconf_palloc,
    );
}
unsafe extern "C" fn iicalled_login(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut pz: *mut *mut libc::c_char = pvar as *mut *mut libc::c_char;
    if argc < 2 as libc::c_int {
        return 5 as libc::c_int | 0x1000 as libc::c_int;
    }
    *pz = *argv.offset(1 as libc::c_int as isize);
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn iiproto_param(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut pqparam: *mut *mut uuconf_proto_param = pvar as *mut *mut uuconf_proto_param;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    if *pqparam
        == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_proto_param
    {
        *pqparam = 0 as *mut uuconf_proto_param;
    }
    return _uuconf_iadd_proto_param(
        qglobal,
        argc - 1 as libc::c_int,
        argv.offset(1 as libc::c_int as isize),
        pqparam,
        (*(*qinfo).qsys).uuconf_palloc,
    );
}
unsafe extern "C" fn iirequest(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iboolean(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut (*(*qinfo).qsys).uuconf_fsend_request,
    );
    if iret & 0xff as libc::c_int == 0 as libc::c_int {
        (*(*qinfo).qsys).uuconf_frec_request = (*(*qinfo).qsys).uuconf_fsend_request;
    }
    return iret;
}
unsafe extern "C" fn iitransfer(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut iret: libc::c_int = 0;
    iret = _uuconf_iboolean(
        qglobal,
        *argv.offset(1 as libc::c_int as isize),
        &mut (*(*qinfo).qsys).uuconf_fcall_transfer,
    );
    if iret & 0xff as libc::c_int == 0 as libc::c_int {
        (*(*qinfo).qsys)
            .uuconf_fcalled_transfer = (*(*qinfo).qsys).uuconf_fcall_transfer;
    }
    return iret;
}
unsafe extern "C" fn iiforward(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut qinfo: *mut sinfo = pinfo as *mut sinfo;
    let mut qsys: *mut uuconf_system = 0 as *mut uuconf_system;
    let mut i: libc::c_int = 0;
    let mut iret: libc::c_int = 0;
    qsys = (*qinfo).qsys;
    (*qsys).uuconf_pzforward_from = 0 as *mut *mut libc::c_char;
    (*qsys).uuconf_pzforward_to = 0 as *mut *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        iret = _uuconf_iadd_string(
            qglobal,
            *argv.offset(i as isize),
            0 as libc::c_int,
            0 as libc::c_int,
            &mut (*qsys).uuconf_pzforward_to,
            (*qsys).uuconf_palloc,
        );
        if iret != 0 as libc::c_int {
            return iret | 0x800 as libc::c_int | 0x1000 as libc::c_int;
        }
        iret = _uuconf_iadd_string(
            qglobal,
            *argv.offset(i as isize),
            0 as libc::c_int,
            0 as libc::c_int,
            &mut (*qsys).uuconf_pzforward_from,
            (*qsys).uuconf_palloc,
        );
        if iret != 0 as libc::c_int {
            return iret | 0x800 as libc::c_int | 0x1000 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn iiunknown(
    mut pglobal: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    return 5 as libc::c_int | 0x1000 as libc::c_int;
}
pub unsafe extern "C" fn uuconf_taylor_system_unknown(
    mut pglobal: pointer,
    mut qsys: *mut uuconf_system,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut as_0: [uuconf_cmdtab; 53] = [uuconf_cmdtab {
        uuconf_zcmd: 0 as *const libc::c_char,
        uuconf_itype: 0,
        uuconf_pvar: 0 as *mut libc::c_void,
        uuconf_pifn: None,
    }; 53];
    let mut si: sinfo = sinfo {
        qsys: 0 as *mut uuconf_system,
        falternates: 0,
        salternate: uuconf_system {
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
        },
        fdefault_alternates: 0,
    };
    let mut q: *mut sunknown = 0 as *mut sunknown;
    let mut iret: libc::c_int = 0;
    if ((*(*qglobal).qprocess).qunknown).is_null() {
        return 1 as libc::c_int;
    }
    _uuconf_ucmdtab_base(
        asIcmds.as_ptr(),
        (::std::mem::size_of::<[cmdtab_offset; 53]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdtab_offset>() as libc::c_ulong),
        qsys as *mut libc::c_char,
        as_0.as_mut_ptr(),
    );
    _uuconf_uclear_system(qsys);
    si.qsys = qsys;
    si.falternates = 0 as libc::c_int;
    si.fdefault_alternates = 1 as libc::c_int;
    (*qsys).uuconf_palloc = uuconf_malloc_block();
    if ((*qsys).uuconf_palloc).is_null() {
        (*qglobal).ierrno = *__errno_location();
        return 4 as libc::c_int | 0x100 as libc::c_int;
    }
    q = (*(*qglobal).qprocess).qunknown;
    while !q.is_null() {
        iret = uuconf_cmd_args(
            pglobal,
            (*q).cargs,
            (*q).pzargs,
            as_0.as_mut_ptr(),
            &mut si as *mut sinfo as pointer,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                uuconf_cmdtabfn,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(iiunknown),
                ),
            ),
            0x2 as libc::c_int,
            (*qsys).uuconf_palloc,
        );
        iret &= !(0x800 as libc::c_int);
        if iret & 0xff as libc::c_int != 0 as libc::c_int {
            (*qglobal).zfilename = (*(*qglobal).qprocess).zconfigfile;
            (*qglobal).ilineno = (*q).ilineno;
            return iret & !(0x1000 as libc::c_int) | 0x200 as libc::c_int
                | 0x400 as libc::c_int;
        }
        if iret & 0x1000 as libc::c_int != 0 as libc::c_int {
            break;
        }
        q = (*q).qnext;
    }
    if si.falternates == 0 {
        uiset_call(qsys);
    } else {
        iret = iialternate(
            pglobal,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            0 as *mut libc::c_void,
            &mut si as *mut sinfo as pointer,
        );
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    (*qsys).uuconf_fcalled = 1 as libc::c_int;
    return _uuconf_isystem_basic_default(qglobal, qsys);
}

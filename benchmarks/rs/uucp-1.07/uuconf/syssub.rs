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
    fn free(__ptr: *mut libc::c_void);
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn uuconf_free(uuconf_pblock: *mut libc::c_void, uuconf_pfree: *mut libc::c_void);
    static mut _uuconf_unset: *mut libc::c_char;
    fn _uuconf_pmalloc_block_merge(_: pointer, _: pointer) -> pointer;
    fn _uuconf_istrsplit(
        zline: *mut libc::c_char,
        bsep: libc::c_int,
        ppzsplit: *mut *mut *mut libc::c_char,
        csplit: *mut size_t,
    ) -> libc::c_int;
    fn _uuconf_iadd_string(
        qglobal: *mut sglobal,
        zadd: *mut libc::c_char,
        fcopy: boolean,
        fdupcheck: boolean,
        ppzstrings: *mut *mut *mut libc::c_char,
        pblock: pointer,
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
pub static mut _uuconf_syssub_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: syssub.c,v 1.17 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_uclear_system(mut q: *mut uuconf_system) {
    (*q)
        .uuconf_zname = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zalternate = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zdebug = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zmax_remote_debug = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zphone = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zcall_login = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zcall_password = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zcalled_login = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zprotocols = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zpubdir = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q)
        .uuconf_zlocalname = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q).uuconf_pzalias = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzlocal_send = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzremote_send = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzlocal_receive = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzremote_receive = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzpath = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzcmds = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzforward_from = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_pzforward_to = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_schat.uuconf_pzchat = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_schat.uuconf_pzprogram = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q).uuconf_schat.uuconf_pzfail = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q)
        .uuconf_scalled_chat
        .uuconf_pzchat = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q)
        .uuconf_scalled_chat
        .uuconf_pzprogram = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q)
        .uuconf_scalled_chat
        .uuconf_pzfail = &mut _uuconf_unset as *mut *mut libc::c_char;
    (*q)
        .uuconf_qtimegrade = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q)
        .uuconf_qcalltimegrade = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q)
        .uuconf_qcalledtimegrade = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q)
        .uuconf_qcall_local_size = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q)
        .uuconf_qcall_remote_size = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q)
        .uuconf_qcalled_local_size = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q)
        .uuconf_qcalled_remote_size = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_timespan;
    (*q).uuconf_fcall = -(1 as libc::c_int);
    (*q).uuconf_fcalled = -(1 as libc::c_int);
    (*q).uuconf_fcallback = -(1 as libc::c_int);
    (*q).uuconf_fsequence = -(1 as libc::c_int);
    (*q).uuconf_fsend_request = -(1 as libc::c_int);
    (*q).uuconf_frec_request = -(1 as libc::c_int);
    (*q).uuconf_fcall_transfer = -(1 as libc::c_int);
    (*q).uuconf_fcalled_transfer = -(1 as libc::c_int);
    (*q).uuconf_schat.uuconf_fstrip = -(1 as libc::c_int);
    (*q).uuconf_scalled_chat.uuconf_fstrip = -(1 as libc::c_int);
    (*q).uuconf_cmax_retries = -(1 as libc::c_int);
    (*q).uuconf_csuccess_wait = -(1 as libc::c_int);
    (*q).uuconf_ibaud = -(1 as libc::c_int) as libc::c_long;
    (*q).uuconf_ihighbaud = -(1 as libc::c_int) as libc::c_long;
    (*q).uuconf_cfree_space = -(1 as libc::c_int) as libc::c_long;
    (*q).uuconf_schat.uuconf_ctimeout = -(1 as libc::c_int);
    (*q).uuconf_scalled_chat.uuconf_ctimeout = -(1 as libc::c_int);
    (*q).uuconf_cmax_file_time = -(1 as libc::c_int) as libc::c_long;
    (*q).uuconf_qalternate = 0 as *mut uuconf_system;
    (*q)
        .uuconf_zport = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut libc::c_char;
    (*q).uuconf_qport = &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port;
    (*q)
        .uuconf_qproto_params = &mut _uuconf_unset as *mut *mut libc::c_char
        as *mut uuconf_proto_param;
    (*q).uuconf_palloc = 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn _uuconf_isystem_default(
    mut qglobal: *mut sglobal,
    mut qset: *mut uuconf_system,
    mut qdefault: *mut uuconf_system,
    mut faddalternates: boolean,
) -> libc::c_int {
    let mut qalt: *mut uuconf_system = 0 as *mut uuconf_system;
    if (*qset).uuconf_palloc != (*qdefault).uuconf_palloc {
        (*qset)
            .uuconf_palloc = _uuconf_pmalloc_block_merge(
            (*qset).uuconf_palloc,
            (*qdefault).uuconf_palloc,
        );
    }
    if faddalternates != 0 {
        let mut pq: *mut *mut uuconf_system = 0 as *mut *mut uuconf_system;
        let mut qdef: *mut uuconf_system = 0 as *mut uuconf_system;
        qdef = qdefault;
        pq = &mut qset;
        while !qdef.is_null() {
            if (*pq).is_null() {
                *pq = uuconf_malloc(
                    (*qset).uuconf_palloc,
                    ::std::mem::size_of::<uuconf_system>() as libc::c_ulong,
                ) as *mut uuconf_system;
                if (*pq).is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    return 4 as libc::c_int | 0x100 as libc::c_int;
                }
                **pq = *qset;
                (**pq).uuconf_qalternate = 0 as *mut uuconf_system;
            }
            qdef = (*qdef).uuconf_qalternate;
            pq = &mut (**pq).uuconf_qalternate;
        }
    }
    qalt = qset;
    while !qalt.is_null() {
        if (*qalt).uuconf_zname
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zname = (*qdefault).uuconf_zname;
        }
        if (*qalt).uuconf_zalternate
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zalternate = (*qdefault).uuconf_zalternate;
        }
        if (*qalt).uuconf_zdebug
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zdebug = (*qdefault).uuconf_zdebug;
        }
        if (*qalt).uuconf_zmax_remote_debug
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zmax_remote_debug = (*qdefault).uuconf_zmax_remote_debug;
        }
        if (*qalt).uuconf_zphone
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zphone = (*qdefault).uuconf_zphone;
        }
        if (*qalt).uuconf_zcall_login
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zcall_login = (*qdefault).uuconf_zcall_login;
        }
        if (*qalt).uuconf_zcall_password
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zcall_password = (*qdefault).uuconf_zcall_password;
        }
        if (*qalt).uuconf_zcalled_login
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zcalled_login = (*qdefault).uuconf_zcalled_login;
        }
        if (*qalt).uuconf_zprotocols
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zprotocols = (*qdefault).uuconf_zprotocols;
        }
        if (*qalt).uuconf_zpubdir
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                as *const libc::c_char
        {
            (*qalt).uuconf_zpubdir = (*qdefault).uuconf_zpubdir;
        }
        if (*qalt).uuconf_zlocalname
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*qalt).uuconf_zlocalname = (*qdefault).uuconf_zlocalname;
        }
        if (*qalt).uuconf_pzalias == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*qalt).uuconf_pzalias = (*qdefault).uuconf_pzalias;
        }
        if (*qalt).uuconf_pzlocal_send == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*qalt).uuconf_pzlocal_send = (*qdefault).uuconf_pzlocal_send;
        }
        if (*qalt).uuconf_pzremote_send == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*qalt).uuconf_pzremote_send = (*qdefault).uuconf_pzremote_send;
        }
        if (*qalt).uuconf_pzlocal_receive == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt).uuconf_pzlocal_receive = (*qdefault).uuconf_pzlocal_receive;
        }
        if (*qalt).uuconf_pzremote_receive
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt).uuconf_pzremote_receive = (*qdefault).uuconf_pzremote_receive;
        }
        if (*qalt).uuconf_pzpath == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*qalt).uuconf_pzpath = (*qdefault).uuconf_pzpath;
        }
        if (*qalt).uuconf_pzcmds == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*qalt).uuconf_pzcmds = (*qdefault).uuconf_pzcmds;
        }
        if (*qalt).uuconf_pzforward_from == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt).uuconf_pzforward_from = (*qdefault).uuconf_pzforward_from;
        }
        if (*qalt).uuconf_pzforward_to == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*qalt).uuconf_pzforward_to = (*qdefault).uuconf_pzforward_to;
        }
        if (*qalt).uuconf_schat.uuconf_pzchat
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt).uuconf_schat.uuconf_pzchat = (*qdefault).uuconf_schat.uuconf_pzchat;
        }
        if (*qalt).uuconf_schat.uuconf_pzprogram
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt)
                .uuconf_schat
                .uuconf_pzprogram = (*qdefault).uuconf_schat.uuconf_pzprogram;
        }
        if (*qalt).uuconf_schat.uuconf_pzfail
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt).uuconf_schat.uuconf_pzfail = (*qdefault).uuconf_schat.uuconf_pzfail;
        }
        if (*qalt).uuconf_scalled_chat.uuconf_pzchat
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt)
                .uuconf_scalled_chat
                .uuconf_pzchat = (*qdefault).uuconf_scalled_chat.uuconf_pzchat;
        }
        if (*qalt).uuconf_scalled_chat.uuconf_pzprogram
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt)
                .uuconf_scalled_chat
                .uuconf_pzprogram = (*qdefault).uuconf_scalled_chat.uuconf_pzprogram;
        }
        if (*qalt).uuconf_scalled_chat.uuconf_pzfail
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*qalt)
                .uuconf_scalled_chat
                .uuconf_pzfail = (*qdefault).uuconf_scalled_chat.uuconf_pzfail;
        }
        if (*qalt).uuconf_qtimegrade
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qtimegrade = (*qdefault).uuconf_qtimegrade;
        }
        if (*qalt).uuconf_qcalltimegrade
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qcalltimegrade = (*qdefault).uuconf_qcalltimegrade;
        }
        if (*qalt).uuconf_qcalledtimegrade
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qcalledtimegrade = (*qdefault).uuconf_qcalledtimegrade;
        }
        if (*qalt).uuconf_qcall_local_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qcall_local_size = (*qdefault).uuconf_qcall_local_size;
        }
        if (*qalt).uuconf_qcall_remote_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qcall_remote_size = (*qdefault).uuconf_qcall_remote_size;
        }
        if (*qalt).uuconf_qcalled_local_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qcalled_local_size = (*qdefault).uuconf_qcalled_local_size;
        }
        if (*qalt).uuconf_qcalled_remote_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*qalt).uuconf_qcalled_remote_size = (*qdefault).uuconf_qcalled_remote_size;
        }
        if (*qalt).uuconf_fcall < 0 as libc::c_int {
            (*qalt).uuconf_fcall = (*qdefault).uuconf_fcall;
        }
        if (*qalt).uuconf_fcalled < 0 as libc::c_int {
            (*qalt).uuconf_fcalled = (*qdefault).uuconf_fcalled;
        }
        if (*qalt).uuconf_fcallback < 0 as libc::c_int {
            (*qalt).uuconf_fcallback = (*qdefault).uuconf_fcallback;
        }
        if (*qalt).uuconf_fsequence < 0 as libc::c_int {
            (*qalt).uuconf_fsequence = (*qdefault).uuconf_fsequence;
        }
        if (*qalt).uuconf_fsend_request < 0 as libc::c_int {
            (*qalt).uuconf_fsend_request = (*qdefault).uuconf_fsend_request;
        }
        if (*qalt).uuconf_frec_request < 0 as libc::c_int {
            (*qalt).uuconf_frec_request = (*qdefault).uuconf_frec_request;
        }
        if (*qalt).uuconf_fcall_transfer < 0 as libc::c_int {
            (*qalt).uuconf_fcall_transfer = (*qdefault).uuconf_fcall_transfer;
        }
        if (*qalt).uuconf_fcalled_transfer < 0 as libc::c_int {
            (*qalt).uuconf_fcalled_transfer = (*qdefault).uuconf_fcalled_transfer;
        }
        if (*qalt).uuconf_schat.uuconf_fstrip < 0 as libc::c_int {
            (*qalt).uuconf_schat.uuconf_fstrip = (*qdefault).uuconf_schat.uuconf_fstrip;
        }
        if (*qalt).uuconf_scalled_chat.uuconf_fstrip < 0 as libc::c_int {
            (*qalt)
                .uuconf_scalled_chat
                .uuconf_fstrip = (*qdefault).uuconf_scalled_chat.uuconf_fstrip;
        }
        if (*qalt).uuconf_cmax_retries < 0 as libc::c_int {
            (*qalt).uuconf_cmax_retries = (*qdefault).uuconf_cmax_retries;
        }
        if (*qalt).uuconf_csuccess_wait < 0 as libc::c_int {
            (*qalt).uuconf_csuccess_wait = (*qdefault).uuconf_csuccess_wait;
        }
        if (*qalt).uuconf_ibaud < 0 as libc::c_int as libc::c_long {
            (*qalt).uuconf_ibaud = (*qdefault).uuconf_ibaud;
        }
        if (*qalt).uuconf_ihighbaud < 0 as libc::c_int as libc::c_long {
            (*qalt).uuconf_ihighbaud = (*qdefault).uuconf_ihighbaud;
        }
        if (*qalt).uuconf_cfree_space < 0 as libc::c_int as libc::c_long {
            (*qalt).uuconf_cfree_space = (*qdefault).uuconf_cfree_space;
        }
        if (*qalt).uuconf_schat.uuconf_ctimeout < 0 as libc::c_int {
            (*qalt)
                .uuconf_schat
                .uuconf_ctimeout = (*qdefault).uuconf_schat.uuconf_ctimeout;
        }
        if (*qalt).uuconf_scalled_chat.uuconf_ctimeout < 0 as libc::c_int {
            (*qalt)
                .uuconf_scalled_chat
                .uuconf_ctimeout = (*qdefault).uuconf_scalled_chat.uuconf_ctimeout;
        }
        if (*qalt).uuconf_cmax_file_time < 0 as libc::c_int as libc::c_long {
            (*qalt).uuconf_cmax_file_time = (*qdefault).uuconf_cmax_file_time;
        }
        if (*qalt).uuconf_zport
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
            && (*qalt).uuconf_qport
                == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
        {
            (*qalt).uuconf_zport = (*qdefault).uuconf_zport;
        }
        if (*qalt).uuconf_qport
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
        {
            (*qalt).uuconf_qport = (*qdefault).uuconf_qport;
        }
        if (*qalt).uuconf_qproto_params
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_proto_param
        {
            (*qalt).uuconf_qproto_params = (*qdefault).uuconf_qproto_params;
        } else if !((*qdefault).uuconf_qproto_params).is_null() {
            let mut cnew: libc::c_int = 0;
            let mut ca: libc::c_int = 0;
            let mut qd: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
            let mut qa: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
            ca = 0 as libc::c_int;
            cnew = 0 as libc::c_int;
            qd = (*qdefault).uuconf_qproto_params;
            while (*qd).uuconf_bproto != '\0' as i32 {
                let mut c: libc::c_int = 0;
                c = 0 as libc::c_int;
                qa = (*qalt).uuconf_qproto_params;
                while (*qa).uuconf_bproto != '\0' as i32
                    && (*qa).uuconf_bproto != (*qd).uuconf_bproto
                {
                    c += 1;
                    c;
                    qa = qa.offset(1);
                    qa;
                }
                if (*qa).uuconf_bproto == '\0' as i32 {
                    cnew += 1;
                    cnew;
                    ca = c;
                }
                qd = qd.offset(1);
                qd;
            }
            if cnew > 0 as libc::c_int {
                let mut qnew: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
                qnew = uuconf_malloc(
                    (*qset).uuconf_palloc,
                    ((ca + cnew + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uuconf_proto_param>() as libc::c_ulong,
                        ),
                ) as *mut uuconf_proto_param;
                if qnew.is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    return 4 as libc::c_int | 0x100 as libc::c_int;
                }
                memcpy(
                    qnew as pointer,
                    (*qalt).uuconf_qproto_params as pointer as *const libc::c_void,
                    (ca as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uuconf_proto_param>() as libc::c_ulong,
                        ),
                );
                cnew = 0 as libc::c_int;
                qd = (*qdefault).uuconf_qproto_params;
                while (*qd).uuconf_bproto != '\0' as i32 {
                    qa = (*qalt).uuconf_qproto_params;
                    while (*qa).uuconf_bproto != '\0' as i32
                        && (*qa).uuconf_bproto != (*qd).uuconf_bproto
                    {
                        qa = qa.offset(1);
                        qa;
                    }
                    if (*qa).uuconf_bproto == '\0' as i32 {
                        *qnew.offset((ca + cnew) as isize) = *qd;
                        cnew += 1;
                        cnew;
                    }
                    qd = qd.offset(1);
                    qd;
                }
                (*qnew.offset((ca + cnew) as isize)).uuconf_bproto = '\0' as i32;
                uuconf_free(
                    (*qset).uuconf_palloc,
                    (*qalt).uuconf_qproto_params as *mut libc::c_void,
                );
                (*qalt).uuconf_qproto_params = qnew;
            }
        }
        if !((*qdefault).uuconf_qalternate).is_null() {
            qdefault = (*qdefault).uuconf_qalternate;
        }
        qalt = (*qalt).uuconf_qalternate;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _uuconf_isystem_basic_default(
    mut qglobal: *mut sglobal,
    mut q: *mut uuconf_system,
) -> libc::c_int {
    let mut iret: libc::c_int = 0;
    iret = 0 as libc::c_int;
    while !q.is_null() && iret == 0 as libc::c_int {
        if (*q).uuconf_cmax_retries < 0 as libc::c_int {
            (*q).uuconf_cmax_retries = 26 as libc::c_int;
        }
        if (*q).uuconf_schat.uuconf_pzchat
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_schat.uuconf_pzchat = 0 as *mut *mut libc::c_char;
            iret = _uuconf_iadd_string(
                qglobal,
                b"\"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"\\r\\c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"ogin:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"-BREAK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"-ogin:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"-BREAK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"-ogin:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"\\L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"word:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
            iret = _uuconf_iadd_string(
                qglobal,
                b"\\P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_schat.uuconf_pzchat,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        if (*q).uuconf_schat.uuconf_ctimeout < 0 as libc::c_int {
            (*q).uuconf_schat.uuconf_ctimeout = 10 as libc::c_int;
        }
        if (*q).uuconf_schat.uuconf_fstrip < 0 as libc::c_int {
            (*q).uuconf_schat.uuconf_fstrip = 1 as libc::c_int;
        }
        if (*q).uuconf_scalled_chat.uuconf_ctimeout < 0 as libc::c_int {
            (*q).uuconf_scalled_chat.uuconf_ctimeout = 60 as libc::c_int;
        }
        if (*q).uuconf_scalled_chat.uuconf_fstrip < 0 as libc::c_int {
            (*q).uuconf_scalled_chat.uuconf_fstrip = 1 as libc::c_int;
        }
        if (*q).uuconf_fsend_request < 0 as libc::c_int {
            (*q).uuconf_fsend_request = 1 as libc::c_int;
        }
        if (*q).uuconf_frec_request < 0 as libc::c_int {
            (*q).uuconf_frec_request = 1 as libc::c_int;
        }
        if (*q).uuconf_fcall_transfer < 0 as libc::c_int {
            (*q).uuconf_fcall_transfer = 1 as libc::c_int;
        }
        if (*q).uuconf_fcalled_transfer < 0 as libc::c_int {
            (*q).uuconf_fcalled_transfer = 1 as libc::c_int;
        }
        if (*q).uuconf_pzlocal_send == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzlocal_send = 0 as *mut *mut libc::c_char;
            iret = _uuconf_iadd_string(
                qglobal,
                b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_pzlocal_send,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        if (*q).uuconf_pzremote_send == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzremote_send = 0 as *mut *mut libc::c_char;
            iret = _uuconf_iadd_string(
                qglobal,
                b"~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_pzremote_send,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        if (*q).uuconf_pzlocal_receive == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzlocal_receive = 0 as *mut *mut libc::c_char;
            iret = _uuconf_iadd_string(
                qglobal,
                b"~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_pzlocal_receive,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        if (*q).uuconf_pzremote_receive == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzremote_receive = 0 as *mut *mut libc::c_char;
            iret = _uuconf_iadd_string(
                qglobal,
                b"~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*q).uuconf_pzremote_receive,
                (*q).uuconf_palloc,
            );
            if iret != 0 as libc::c_int {
                return iret;
            }
        }
        if (*q).uuconf_pzpath == &mut _uuconf_unset as *mut *mut libc::c_char {
            let mut zdup: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut csplit: size_t = 0;
            let mut c: libc::c_int = 0;
            zdup = uuconf_malloc(
                (*q).uuconf_palloc,
                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong,
            ) as *mut libc::c_char;
            if zdup.is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            memcpy(
                zdup as pointer,
                b"/bin /usr/bin /usr/local/bin\0" as *const u8 as *const libc::c_char
                    as pointer as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong,
            );
            pz = 0 as *mut *mut libc::c_char;
            csplit = 0 as libc::c_int as size_t;
            c = _uuconf_istrsplit(zdup, '\0' as i32, &mut pz, &mut csplit);
            if c < 0 as libc::c_int {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            (*q)
                .uuconf_pzpath = uuconf_malloc(
                (*q).uuconf_palloc,
                ((c + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if ((*q).uuconf_pzpath).is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            memcpy(
                (*q).uuconf_pzpath as pointer,
                pz as pointer as *const libc::c_void,
                (c as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            );
            let ref mut fresh0 = *((*q).uuconf_pzpath).offset(c as isize);
            *fresh0 = 0 as *mut libc::c_char;
            free(pz as pointer);
        }
        if (*q).uuconf_pzcmds == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q)
                .uuconf_pzcmds = uuconf_malloc(
                (*q).uuconf_palloc,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if ((*q).uuconf_pzcmds).is_null() {
                (*qglobal).ierrno = *__errno_location();
                return 4 as libc::c_int | 0x100 as libc::c_int;
            }
            let ref mut fresh1 = *((*q).uuconf_pzcmds).offset(0 as libc::c_int as isize);
            *fresh1 = b"rnews\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let ref mut fresh2 = *((*q).uuconf_pzcmds).offset(1 as libc::c_int as isize);
            *fresh2 = b"rmail\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let ref mut fresh3 = *((*q).uuconf_pzcmds).offset(2 as libc::c_int as isize);
            *fresh3 = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_cfree_space < 0 as libc::c_int as libc::c_long {
            (*q).uuconf_cfree_space = 50000 as libc::c_int as libc::c_long;
        }
        if (*q).uuconf_zpubdir
            == &mut _uuconf_unset as *mut *mut libc::c_char as *const libc::c_char
        {
            (*q).uuconf_zpubdir = (*(*qglobal).qprocess).zpubdir;
        }
        if (*q).uuconf_zname
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zname = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zalternate
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zalternate = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zdebug
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zdebug = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zmax_remote_debug
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zmax_remote_debug = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zphone
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zphone = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zcall_login
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zcall_login = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zcall_password
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zcall_password = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zcalled_login
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zcalled_login = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zprotocols
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zprotocols = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_zpubdir
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
                as *const libc::c_char
        {
            (*q).uuconf_zpubdir = 0 as *const libc::c_char;
        }
        if (*q).uuconf_zlocalname
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zlocalname = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_pzalias == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzalias = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzlocal_send == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzlocal_send = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzremote_send == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzremote_send = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzlocal_receive == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzlocal_receive = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzremote_receive == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzremote_receive = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzpath == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzpath = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzcmds == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzcmds = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzforward_from == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzforward_from = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_pzforward_to == &mut _uuconf_unset as *mut *mut libc::c_char {
            (*q).uuconf_pzforward_to = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_schat.uuconf_pzchat
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_schat.uuconf_pzchat = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_schat.uuconf_pzprogram
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_schat.uuconf_pzprogram = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_schat.uuconf_pzfail
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_schat.uuconf_pzfail = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_scalled_chat.uuconf_pzchat
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_scalled_chat.uuconf_pzchat = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_scalled_chat.uuconf_pzprogram
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_scalled_chat.uuconf_pzprogram = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_scalled_chat.uuconf_pzfail
            == &mut _uuconf_unset as *mut *mut libc::c_char
        {
            (*q).uuconf_scalled_chat.uuconf_pzfail = 0 as *mut *mut libc::c_char;
        }
        if (*q).uuconf_qtimegrade
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qtimegrade = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_qcalltimegrade
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qcalltimegrade = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_qcalledtimegrade
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qcalledtimegrade = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_qcall_local_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qcall_local_size = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_qcall_remote_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qcall_remote_size = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_qcalled_local_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qcalled_local_size = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_qcalled_remote_size
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        {
            (*q).uuconf_qcalled_remote_size = 0 as *mut uuconf_timespan;
        }
        if (*q).uuconf_fcall < 0 as libc::c_int {
            (*q).uuconf_fcall = 0 as libc::c_int;
        }
        if (*q).uuconf_fcalled < 0 as libc::c_int {
            (*q).uuconf_fcalled = 0 as libc::c_int;
        }
        if (*q).uuconf_fcallback < 0 as libc::c_int {
            (*q).uuconf_fcallback = 0 as libc::c_int;
        }
        if (*q).uuconf_fsequence < 0 as libc::c_int {
            (*q).uuconf_fsequence = 0 as libc::c_int;
        }
        if (*q).uuconf_fsend_request < 0 as libc::c_int {
            (*q).uuconf_fsend_request = 0 as libc::c_int;
        }
        if (*q).uuconf_frec_request < 0 as libc::c_int {
            (*q).uuconf_frec_request = 0 as libc::c_int;
        }
        if (*q).uuconf_fcall_transfer < 0 as libc::c_int {
            (*q).uuconf_fcall_transfer = 0 as libc::c_int;
        }
        if (*q).uuconf_fcalled_transfer < 0 as libc::c_int {
            (*q).uuconf_fcalled_transfer = 0 as libc::c_int;
        }
        if (*q).uuconf_schat.uuconf_fstrip < 0 as libc::c_int {
            (*q).uuconf_schat.uuconf_fstrip = 0 as libc::c_int;
        }
        if (*q).uuconf_scalled_chat.uuconf_fstrip < 0 as libc::c_int {
            (*q).uuconf_scalled_chat.uuconf_fstrip = 0 as libc::c_int;
        }
        if (*q).uuconf_cmax_retries < 0 as libc::c_int {
            (*q).uuconf_cmax_retries = 0 as libc::c_int;
        }
        if (*q).uuconf_csuccess_wait < 0 as libc::c_int {
            (*q).uuconf_csuccess_wait = 0 as libc::c_int;
        }
        if (*q).uuconf_ibaud < 0 as libc::c_int as libc::c_long {
            (*q).uuconf_ibaud = 0 as libc::c_int as libc::c_long;
        }
        if (*q).uuconf_ihighbaud < 0 as libc::c_int as libc::c_long {
            (*q).uuconf_ihighbaud = 0 as libc::c_int as libc::c_long;
        }
        if (*q).uuconf_cfree_space < 0 as libc::c_int as libc::c_long {
            (*q).uuconf_cfree_space = 0 as libc::c_int as libc::c_long;
        }
        if (*q).uuconf_schat.uuconf_ctimeout < 0 as libc::c_int {
            (*q).uuconf_schat.uuconf_ctimeout = 0 as libc::c_int;
        }
        if (*q).uuconf_scalled_chat.uuconf_ctimeout < 0 as libc::c_int {
            (*q).uuconf_scalled_chat.uuconf_ctimeout = 0 as libc::c_int;
        }
        if (*q).uuconf_cmax_file_time < 0 as libc::c_int as libc::c_long {
            (*q).uuconf_cmax_file_time = 0 as libc::c_int as libc::c_long;
        }
        if (*q).uuconf_zport
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut libc::c_char
        {
            (*q).uuconf_zport = 0 as *mut libc::c_char;
        }
        if (*q).uuconf_qport
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_port
        {
            (*q).uuconf_qport = 0 as *mut uuconf_port;
        }
        if (*q).uuconf_qproto_params
            == &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_proto_param
        {
            (*q).uuconf_qproto_params = 0 as *mut uuconf_proto_param;
        }
        q = (*q).uuconf_qalternate;
    }
    return iret;
}

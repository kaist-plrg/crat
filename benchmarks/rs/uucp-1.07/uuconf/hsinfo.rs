use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn _uuconf_ihread_permissions(qglobal: *mut sglobal) -> libc::c_int;
    fn _uuconf_getline(
        qglobal: *mut sglobal,
        _: *mut *mut libc::c_char,
        _: *mut size_t,
        _: *mut FILE,
    ) -> libc::c_int;
    fn _uuconf_uclear_system(qsys: *mut uuconf_system);
    fn _uuconf_istrsplit(
        zline: *mut libc::c_char,
        bsep: libc::c_int,
        ppzsplit: *mut *mut *mut libc::c_char,
        csplit: *mut size_t,
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
    fn uuconf_malloc_block() -> *mut libc::c_void;
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
    ) -> libc::c_int;
    static mut _uuconf_unset: *mut libc::c_char;
    fn _uuconf_ichat_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qchat: *mut uuconf_chat,
        pblock: pointer,
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
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub static mut _uuconf_hsinfo_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: hsinfo.c,v 1.17 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_ihdb_system_internal(
    mut qglobal: *mut sglobal,
    mut zsystem: *const libc::c_char,
    mut qsys: *mut uuconf_system,
) -> libc::c_int {
    let mut iret: libc::c_int = 0;
    let mut qperm: *mut shpermissions = 0 as *mut shpermissions;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csplit: size_t = 0;
    let mut pzcomma: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ccomma: size_t = 0;
    let mut pblock: pointer = 0 as *mut libc::c_void;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ffound_machine: boolean = 0;
    let mut ffound_login: boolean = 0;
    let mut qother_machine: *mut shpermissions = 0 as *mut shpermissions;
    let mut qalt: *mut uuconf_system = 0 as *mut uuconf_system;
    if (*(*qglobal).qprocess).fhdb_read_permissions == 0 {
        iret = _uuconf_ihread_permissions(qglobal);
        if iret != 0 as libc::c_int {
            return iret;
        }
    }
    qperm = (*(*qglobal).qprocess).qhdb_permissions;
    while !qperm.is_null() {
        if !(((*qperm).pzalias).is_null() || ((*qperm).pzmachine).is_null()
            || (*qperm).pzalias == &mut _uuconf_unset as *mut *mut libc::c_char
            || (*qperm).pzmachine == &mut _uuconf_unset as *mut *mut libc::c_char)
        {
            pz = (*qperm).pzalias;
            while !(*pz).is_null() {
                if strcmp(*pz, zsystem) == 0 as libc::c_int {
                    zsystem = *((*qperm).pzmachine).offset(0 as libc::c_int as isize);
                    break;
                } else {
                    pz = pz.offset(1);
                    pz;
                }
            }
            if !(*pz).is_null() {
                break;
            }
        }
        qperm = (*qperm).qnext;
    }
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    pzsplit = 0 as *mut *mut libc::c_char;
    csplit = 0 as libc::c_int as size_t;
    pzcomma = 0 as *mut *mut libc::c_char;
    ccomma = 0 as libc::c_int as size_t;
    pblock = 0 as *mut libc::c_void;
    iret = 0 as libc::c_int;
    pz = (*(*qglobal).qprocess).pzhdb_systems;
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
                let mut ctimes: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                let mut qset: *mut uuconf_system = 0 as *mut uuconf_system;
                let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut zretry: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cretry: libc::c_int = 0;
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
                    if ctoks < 1 as libc::c_int
                        || strcmp(zsystem, *pzsplit.offset(0 as libc::c_int as isize))
                            != 0 as libc::c_int
                    {
                        continue;
                    }
                    if pblock.is_null() {
                        pblock = uuconf_malloc_block();
                        if pblock.is_null() {
                            (*qglobal).ierrno = *__errno_location();
                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            _uuconf_uclear_system(qsys);
                            (*qsys).uuconf_palloc = pblock;
                            qset = qsys;
                        }
                    } else {
                        let mut pq: *mut *mut uuconf_system = 0
                            as *mut *mut uuconf_system;
                        qset = uuconf_malloc(
                            pblock,
                            ::std::mem::size_of::<uuconf_system>() as libc::c_ulong,
                        ) as *mut uuconf_system;
                        if qset.is_null() {
                            (*qglobal).ierrno = *__errno_location();
                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            _uuconf_uclear_system(qset);
                            pq = &mut (*qsys).uuconf_qalternate;
                            while !(*pq).is_null() {
                                pq = &mut (**pq).uuconf_qalternate;
                            }
                            *pq = qset;
                        }
                    }
                    if uuconf_add_block(pblock, zline as *mut libc::c_void)
                        != 0 as libc::c_int
                    {
                        (*qglobal).ierrno = *__errno_location();
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    } else {
                        zline = 0 as *mut libc::c_char;
                        cline = 0 as libc::c_int as size_t;
                        (*qset)
                            .uuconf_zname = *pzsplit.offset(0 as libc::c_int as isize);
                        (*qset).uuconf_fcall = 1 as libc::c_int;
                        (*qset).uuconf_fcalled = 0 as libc::c_int;
                        if ctoks < 2 as libc::c_int {
                            continue;
                        }
                        zretry = strchr(
                            *pzsplit.offset(1 as libc::c_int as isize),
                            ';' as i32,
                        );
                        if zretry.is_null() {
                            cretry = 0 as libc::c_int;
                        } else {
                            *zretry = '\0' as i32 as libc::c_char;
                            cretry = strtol(
                                zretry.offset(1 as libc::c_int as isize),
                                0 as *mut libc::c_void as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            ) as libc::c_int;
                        }
                        ctimes = _uuconf_istrsplit(
                            *pzsplit.offset(1 as libc::c_int as isize),
                            ',' as i32,
                            &mut pzcomma,
                            &mut ccomma,
                        );
                        if ctimes < 0 as libc::c_int {
                            (*qglobal).ierrno = *__errno_location();
                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            i = 0 as libc::c_int;
                            while i < ctimes {
                                let mut zslash: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut bgrade: libc::c_char = 0;
                                z = *pzcomma.offset(i as isize);
                                zslash = strchr(z, '/' as i32);
                                if zslash.is_null() {
                                    bgrade = 'z' as i32 as libc::c_char;
                                } else {
                                    *zslash = '\0' as i32 as libc::c_char;
                                    bgrade = *zslash.offset(1 as libc::c_int as isize);
                                    if *(*__ctype_b_loc())
                                        .offset(bgrade as libc::c_uchar as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                        == 0
                                    {
                                        bgrade = 'z' as i32 as libc::c_char;
                                    }
                                }
                                iret = _uuconf_itime_parse(
                                    qglobal,
                                    z,
                                    bgrade as libc::c_long,
                                    cretry,
                                    Some(
                                        _uuconf_itime_grade_cmp
                                            as unsafe extern "C" fn(
                                                libc::c_long,
                                                libc::c_long,
                                            ) -> libc::c_int,
                                    ),
                                    &mut (*qset).uuconf_qtimegrade,
                                    pblock,
                                );
                                if iret == 5 as libc::c_int {
                                    iret = 0 as libc::c_int;
                                }
                                if iret != 0 as libc::c_int {
                                    break;
                                }
                                if bgrade as libc::c_int != 'z' as i32 {
                                    (*qset).uuconf_qcalltimegrade = (*qset).uuconf_qtimegrade;
                                }
                                i += 1;
                                i;
                            }
                            if iret != 0 as libc::c_int {
                                break;
                            }
                            if ctoks < 3 as libc::c_int {
                                continue;
                            }
                            (*qset)
                                .uuconf_zport = *pzsplit.offset(2 as libc::c_int as isize);
                            z = strchr(
                                *pzsplit.offset(2 as libc::c_int as isize),
                                ',' as i32,
                            );
                            if !z.is_null() {
                                (*qset)
                                    .uuconf_zprotocols = z.offset(1 as libc::c_int as isize);
                                *z = '\0' as i32 as libc::c_char;
                            }
                            if ctoks < 4 as libc::c_int {
                                continue;
                            }
                            z = *pzsplit.offset(3 as libc::c_int as isize);
                            if strcasecmp(
                                z,
                                b"Any\0" as *const u8 as *const libc::c_char,
                            ) != 0 as libc::c_int
                                && strcmp(z, b"-\0" as *const u8 as *const libc::c_char)
                                    != 0 as libc::c_int
                            {
                                let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
                                while *z as libc::c_int != '\0' as i32
                                    && *(*__ctype_b_loc())
                                        .offset(*z as libc::c_uchar as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                        == 0
                                {
                                    z = z.offset(1);
                                    z;
                                }
                                (*qset)
                                    .uuconf_ibaud = strtol(z, &mut zend, 10 as libc::c_int);
                                if *zend as libc::c_int == '-' as i32 {
                                    (*qset)
                                        .uuconf_ihighbaud = strtol(
                                        zend.offset(1 as libc::c_int as isize),
                                        0 as *mut libc::c_void as *mut *mut libc::c_char,
                                        10 as libc::c_int,
                                    );
                                }
                                if z != *pzsplit.offset(3 as libc::c_int as isize) {
                                    let mut cport: size_t = 0;
                                    let mut cclass: size_t = 0;
                                    cport = strlen(*pzsplit.offset(2 as libc::c_int as isize));
                                    cclass = z
                                        .offset_from(*pzsplit.offset(3 as libc::c_int as isize))
                                        as libc::c_long as size_t;
                                    (*qset)
                                        .uuconf_zport = uuconf_malloc(
                                        pblock,
                                        cport
                                            .wrapping_add(cclass)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as *mut libc::c_char;
                                    if ((*qset).uuconf_zport).is_null() {
                                        (*qglobal).ierrno = *__errno_location();
                                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                                        break;
                                    } else {
                                        memcpy(
                                            (*qset).uuconf_zport as pointer,
                                            *pzsplit.offset(2 as libc::c_int as isize) as pointer
                                                as *const libc::c_void,
                                            cport,
                                        );
                                        memcpy(
                                            ((*qset).uuconf_zport).offset(cport as isize) as pointer,
                                            *pzsplit.offset(3 as libc::c_int as isize) as pointer
                                                as *const libc::c_void,
                                            cclass,
                                        );
                                        *((*qset).uuconf_zport)
                                            .offset(
                                                cport.wrapping_add(cclass) as isize,
                                            ) = '\0' as i32 as libc::c_char;
                                    }
                                }
                            }
                            if ctoks < 5 as libc::c_int {
                                continue;
                            }
                            (*qset)
                                .uuconf_zphone = *pzsplit.offset(4 as libc::c_int as isize);
                            if ctoks < 6 as libc::c_int {
                                continue;
                            }
                            let ref mut fresh0 = *pzsplit
                                .offset(4 as libc::c_int as isize);
                            *fresh0 = b"chat\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            iret = _uuconf_ichat_cmd(
                                qglobal,
                                ctoks - 4 as libc::c_int,
                                pzsplit.offset(4 as libc::c_int as isize),
                                &mut (*qset).uuconf_schat,
                                pblock,
                            );
                            iret &= !(0x800 as libc::c_int);
                            if iret != 0 as libc::c_int {
                                break;
                            }
                        }
                    }
                }
            }
            fclose(e);
            if iret != 0 as libc::c_int {
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
    if !pzcomma.is_null() {
        free(pzcomma as pointer);
    }
    if iret != 0 as libc::c_int {
        (*qglobal).zfilename = *pz;
        return iret | 0x200 as libc::c_int | 0x400 as libc::c_int;
    }
    if pblock.is_null() {
        return 1 as libc::c_int;
    }
    ffound_machine = 0 as libc::c_int;
    ffound_login = 0 as libc::c_int;
    qother_machine = 0 as *mut shpermissions;
    qperm = (*(*qglobal).qprocess).qhdb_permissions;
    while !qperm.is_null() {
        let mut fmachine: boolean = 0;
        if qother_machine.is_null() && !((*qperm).pzmachine).is_null()
            && (*qperm).pzmachine != &mut _uuconf_unset as *mut *mut libc::c_char
            && *(*((*qperm).pzmachine).offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == 'O' as i32
            && strcmp(
                *((*qperm).pzmachine).offset(0 as libc::c_int as isize),
                b"OTHER\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            qother_machine = qperm;
        }
        fmachine = 0 as libc::c_int;
        if ffound_machine == 0 && !((*qperm).pzmachine).is_null()
            && (*qperm).pzmachine != &mut _uuconf_unset as *mut *mut libc::c_char
        {
            pz = (*qperm).pzmachine;
            while !(*pz).is_null() {
                if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int
                    == *zsystem.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp(*pz, zsystem) == 0 as libc::c_int
                {
                    qalt = qsys;
                    while !qalt.is_null() {
                        if (*qalt).uuconf_fcall != 0 {
                            iret = ihadd_machine_perm(qglobal, qalt, qperm);
                            if iret != 0 as libc::c_int {
                                return iret;
                            }
                        }
                        qalt = (*qalt).uuconf_qalternate;
                    }
                    fmachine = 1 as libc::c_int;
                    ffound_machine = 1 as libc::c_int;
                    break;
                } else {
                    pz = pz.offset(1);
                    pz;
                }
            }
        }
        if !((*qperm).pzlogname).is_null()
            && (*qperm).pzlogname != &mut _uuconf_unset as *mut *mut libc::c_char
            && !((*qperm).pzvalidate).is_null()
            && (*qperm).pzvalidate != &mut _uuconf_unset as *mut *mut libc::c_char
        {
            pz = (*qperm).pzvalidate;
            while !(*pz).is_null() {
                if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int
                    == *zsystem.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp(*pz, zsystem) == 0 as libc::c_int
                {
                    break;
                }
                pz = pz.offset(1);
                pz;
            }
            if !(*pz).is_null() {
                pz = (*qperm).pzlogname;
                while !(*pz).is_null() {
                    if fmachine != 0
                        && (((*qsys).uuconf_zcalled_login).is_null()
                            || (*qsys).uuconf_zcalled_login
                                == &mut _uuconf_unset as *mut *mut libc::c_char
                                    as *mut libc::c_char)
                    {
                        (*qsys).uuconf_zcalled_login = *pz;
                        iret = ihadd_logname_perm(qglobal, qsys, qperm);
                    } else {
                        let mut qnew: *mut uuconf_system = 0 as *mut uuconf_system;
                        let mut pq_0: *mut *mut uuconf_system = 0
                            as *mut *mut uuconf_system;
                        qnew = uuconf_malloc(
                            pblock,
                            ::std::mem::size_of::<uuconf_system>() as libc::c_ulong,
                        ) as *mut uuconf_system;
                        if qnew.is_null() {
                            (*qglobal).ierrno = *__errno_location();
                            return 4 as libc::c_int | 0x100 as libc::c_int;
                        }
                        *qnew = *qsys;
                        (*qnew).uuconf_qalternate = 0 as *mut uuconf_system;
                        pq_0 = &mut (*qsys).uuconf_qalternate;
                        while !(*pq_0).is_null() {
                            pq_0 = &mut (**pq_0).uuconf_qalternate;
                        }
                        *pq_0 = qnew;
                        (*qnew).uuconf_zcalled_login = *pz;
                        (*qnew).uuconf_fcall = 0 as libc::c_int;
                        iret = ihadd_logname_perm(qglobal, qnew, qperm);
                    }
                    if iret != 0 as libc::c_int {
                        return iret;
                    }
                    pz = pz.offset(1);
                    pz;
                }
                ffound_login = 1 as libc::c_int;
            }
        }
        qperm = (*qperm).qnext;
    }
    if ffound_machine == 0 && !qother_machine.is_null() {
        qalt = qsys;
        while !qalt.is_null() {
            if (*qalt).uuconf_fcall != 0 {
                iret = ihadd_machine_perm(qglobal, qalt, qother_machine);
                if iret != 0 as libc::c_int {
                    return iret;
                }
            }
            qalt = (*qalt).uuconf_qalternate;
        }
    }
    if ffound_login == 0 {
        qperm = (*(*qglobal).qprocess).qhdb_permissions;
        while !qperm.is_null() {
            if !(((*qperm).pzlogname).is_null()
                || (*qperm).pzlogname == &mut _uuconf_unset as *mut *mut libc::c_char)
            {
                pz = (*qperm).pzlogname;
                while !(*pz).is_null() {
                    let mut qnew_0: *mut uuconf_system = 0 as *mut uuconf_system;
                    let mut pq_1: *mut *mut uuconf_system = 0 as *mut *mut uuconf_system;
                    qnew_0 = uuconf_malloc(
                        pblock,
                        ::std::mem::size_of::<uuconf_system>() as libc::c_ulong,
                    ) as *mut uuconf_system;
                    if qnew_0.is_null() {
                        (*qglobal).ierrno = *__errno_location();
                        return 4 as libc::c_int | 0x100 as libc::c_int;
                    }
                    *qnew_0 = *qsys;
                    (*qnew_0).uuconf_qalternate = 0 as *mut uuconf_system;
                    pq_1 = &mut (*qsys).uuconf_qalternate;
                    while !(*pq_1).is_null() {
                        pq_1 = &mut (**pq_1).uuconf_qalternate;
                    }
                    *pq_1 = qnew_0;
                    if strcmp(*pz, b"OTHER\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        (*qnew_0)
                            .uuconf_zcalled_login = b"ANY\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    } else {
                        (*qnew_0).uuconf_zcalled_login = *pz;
                    }
                    (*qnew_0).uuconf_fcall = 0 as libc::c_int;
                    iret = ihadd_logname_perm(qglobal, qnew_0, qperm);
                    if iret != 0 as libc::c_int {
                        return iret;
                    }
                    pz = pz.offset(1);
                    pz;
                }
            }
            qperm = (*qperm).qnext;
        }
    }
    qalt = qsys;
    while !qalt.is_null() {
        iret = _uuconf_iadd_string(
            qglobal,
            b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            &mut (*qalt).uuconf_pzlocal_receive,
            pblock,
        );
        if iret != 0 as libc::c_int {
            return iret;
        }
        qalt = (*qalt).uuconf_qalternate;
    }
    if !((*qsys).uuconf_qtimegrade).is_null()
        && (*qsys).uuconf_qtimegrade
            != &mut _uuconf_unset as *mut *mut libc::c_char as *mut uuconf_timespan
        && (*(*qsys).uuconf_qtimegrade).uuconf_cretry > 0 as libc::c_int
    {
        (*qsys).uuconf_cmax_retries = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ihadd_machine_perm(
    mut qglobal: *mut sglobal,
    mut qsys: *mut uuconf_system,
    mut qperm: *mut shpermissions,
) -> libc::c_int {
    if (*qperm).frequest >= 0 as libc::c_int {
        (*qsys).uuconf_fsend_request = (*qperm).frequest;
    } else {
        (*qsys).uuconf_fsend_request = 0 as libc::c_int;
    }
    (*qsys).uuconf_pzremote_send = (*qperm).pzread;
    (*qsys).uuconf_pzremote_receive = (*qperm).pzwrite;
    (*qsys).uuconf_pzcmds = (*qperm).pzcommands;
    (*qsys).uuconf_zlocalname = (*qperm).zmyname;
    (*qsys).uuconf_zpubdir = (*qperm).zpubdir;
    (*qsys).uuconf_pzalias = (*qperm).pzalias;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ihadd_logname_perm(
    mut qglobal: *mut sglobal,
    mut qsys: *mut uuconf_system,
    mut qperm: *mut shpermissions,
) -> libc::c_int {
    (*qsys).uuconf_fcalled = 1 as libc::c_int;
    if (*qperm).frequest >= 0 as libc::c_int {
        (*qsys).uuconf_fsend_request = (*qperm).frequest;
    } else {
        (*qsys).uuconf_fsend_request = 0 as libc::c_int;
    }
    (*qsys).uuconf_fcalled_transfer = (*qperm).fsendfiles;
    (*qsys).uuconf_pzremote_send = (*qperm).pzread;
    (*qsys).uuconf_pzremote_receive = (*qperm).pzwrite;
    (*qsys).uuconf_fcallback = (*qperm).fcallback;
    (*qsys).uuconf_zlocalname = (*qperm).zmyname;
    (*qsys).uuconf_zpubdir = (*qperm).zpubdir;
    return 0 as libc::c_int;
}

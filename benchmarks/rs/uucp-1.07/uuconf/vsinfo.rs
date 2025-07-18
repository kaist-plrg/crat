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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn _uuconf_getline(
        qglobal: *mut sglobal,
        _: *mut *mut libc::c_char,
        _: *mut size_t,
        _: *mut FILE,
    ) -> libc::c_int;
    fn _uuconf_uclear_system(qsys: *mut uuconf_system);
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
    fn _uuconf_uclear_port(qport: *mut uuconf_port);
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
    fn _uuconf_ichat_cmd(
        qglobal: *mut sglobal,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        qchat: *mut uuconf_chat,
        pblock: pointer,
    ) -> libc::c_int;
    static mut _uuconf_unset: *mut libc::c_char;
    fn uuconf_add_block(
        uuconf_pblock: *mut libc::c_void,
        uuconf_padd: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
    fn uuconf_malloc_block() -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub static mut _uuconf_vsinfo_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: vsinfo.c,v 1.17 2002/03/05 19:10:43 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_iv2_system_internal(
    mut qglobal: *mut sglobal,
    mut zsystem: *const libc::c_char,
    mut qsys: *mut uuconf_system,
) -> libc::c_int {
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    let mut pzsplit: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csplit: size_t = 0;
    let mut pzcomma: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ccomma: size_t = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut cchars: libc::c_int = 0;
    let mut pblock: pointer = 0 as *mut libc::c_void;
    let mut iret: libc::c_int = 0;
    e = fopen(
        (*(*qglobal).qprocess).zv2systems,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if e.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            return 1 as libc::c_int;
        }
        (*qglobal).ierrno = *__errno_location();
        (*qglobal).zfilename = (*(*qglobal).qprocess).zv2systems;
        return 2 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int;
    }
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    pzsplit = 0 as *mut *mut libc::c_char;
    csplit = 0 as libc::c_int as size_t;
    pzcomma = 0 as *mut *mut libc::c_char;
    ccomma = 0 as libc::c_int as size_t;
    pblock = 0 as *mut libc::c_void;
    iret = 0 as libc::c_int;
    (*qglobal).ilineno = 0 as libc::c_int;
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
                let mut pq: *mut *mut uuconf_system = 0 as *mut *mut uuconf_system;
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
            if uuconf_add_block(pblock, zline as *mut libc::c_void) != 0 as libc::c_int {
                (*qglobal).ierrno = *__errno_location();
                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                break;
            } else {
                zline = 0 as *mut libc::c_char;
                cline = 0 as libc::c_int as size_t;
                (*qset).uuconf_zname = *pzsplit.offset(0 as libc::c_int as isize);
                (*qset).uuconf_fcall = 1 as libc::c_int;
                (*qset).uuconf_fcalled = 1 as libc::c_int;
                if ctoks < 2 as libc::c_int {
                    continue;
                }
                zretry = strchr(*pzsplit.offset(1 as libc::c_int as isize), ';' as i32);
                if zretry.is_null() {
                    cretry = 55 as libc::c_int;
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
                    (*qset).uuconf_zport = *pzsplit.offset(2 as libc::c_int as isize);
                    z = strchr(*pzsplit.offset(2 as libc::c_int as isize), ',' as i32);
                    if !z.is_null() {
                        (*qset).uuconf_zprotocols = z.offset(1 as libc::c_int as isize);
                        *z = '\0' as i32 as libc::c_char;
                    }
                    if strcmp(
                        (*qset).uuconf_zport,
                        b"TCP\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        (*qset).uuconf_zport = 0 as *mut libc::c_char;
                        (*qset)
                            .uuconf_qport = uuconf_malloc(
                            pblock,
                            ::std::mem::size_of::<uuconf_port>() as libc::c_ulong,
                        ) as *mut uuconf_port;
                        if ((*qset).uuconf_qport).is_null() {
                            (*qglobal).ierrno = *__errno_location();
                            iret = 4 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            _uuconf_uclear_port((*qset).uuconf_qport);
                            (*(*qset).uuconf_qport)
                                .uuconf_zname = b"TCP\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            (*(*qset).uuconf_qport).uuconf_ttype = UUCONF_PORTTYPE_TCP;
                            (*(*qset).uuconf_qport)
                                .uuconf_ireliable = 0o10 as libc::c_int | 0o4 as libc::c_int
                                | 0o2 as libc::c_int | 0o20 as libc::c_int
                                | 0o1 as libc::c_int;
                            if ctoks < 4 as libc::c_int {
                                (*(*qset).uuconf_qport)
                                    .uuconf_u
                                    .uuconf_stcp
                                    .uuconf_zport = b"uucp\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                            } else {
                                (*(*qset).uuconf_qport)
                                    .uuconf_u
                                    .uuconf_stcp
                                    .uuconf_zport = *pzsplit.offset(3 as libc::c_int as isize);
                            }
                            (*(*qset).uuconf_qport)
                                .uuconf_u
                                .uuconf_stcp
                                .uuconf_iversion = 0 as libc::c_int;
                            (*(*qset).uuconf_qport)
                                .uuconf_u
                                .uuconf_stcp
                                .uuconf_pzdialer = 0 as *mut *mut libc::c_char;
                        }
                    }
                    if ctoks < 4 as libc::c_int {
                        continue;
                    }
                    (*qset)
                        .uuconf_ibaud = strtol(
                        *pzsplit.offset(3 as libc::c_int as isize),
                        0 as *mut libc::c_void as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    if ctoks < 5 as libc::c_int {
                        continue;
                    }
                    (*qset).uuconf_zphone = *pzsplit.offset(4 as libc::c_int as isize);
                    if ctoks < 6 as libc::c_int {
                        continue;
                    }
                    let ref mut fresh0 = *pzsplit.offset(4 as libc::c_int as isize);
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
    if !pzcomma.is_null() {
        free(pzcomma as pointer);
    }
    if iret != 0 as libc::c_int {
        if !zline.is_null() {
            free(zline as pointer);
        }
        if !pzsplit.is_null() {
            free(pzsplit as pointer);
        }
        (*qglobal).zfilename = (*(*qglobal).qprocess).zv2systems;
        return iret | 0x200 as libc::c_int | 0x400 as libc::c_int;
    }
    if pblock.is_null() {
        if !zline.is_null() {
            free(zline as pointer);
        }
        if !pzsplit.is_null() {
            free(pzsplit as pointer);
        }
        return 1 as libc::c_int;
    }
    e = fopen(
        (*(*qglobal).qprocess).zv2userfile,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !e.is_null() {
        let mut pzlocal: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut pzremote: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut fdefault_callback: boolean = 0;
        let mut zdefault_login: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut q: *mut uuconf_system = 0 as *mut uuconf_system;
        pzlocal = 0 as *mut *mut libc::c_char;
        pzremote = 0 as *mut *mut libc::c_char;
        fdefault_callback = 0 as libc::c_int;
        zdefault_login = 0 as *mut libc::c_char;
        (*qglobal).ilineno = 0 as libc::c_int;
        loop {
            cchars = getline(&mut zline, &mut cline, e) as libc::c_int;
            if !(cchars > 0 as libc::c_int) {
                break;
            }
            let mut ctoks_0: libc::c_int = 0;
            let mut zcomma: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut fcallback: boolean = 0;
            let mut pzlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut pznew: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            (*qglobal).ilineno += 1;
            (*qglobal).ilineno;
            cchars -= 1;
            cchars;
            if *zline.offset(cchars as isize) as libc::c_int == '\n' as i32 {
                *zline.offset(cchars as isize) = '\0' as i32 as libc::c_char;
            }
            *zline
                .offset(
                    strcspn(zline, b"#\0" as *const u8 as *const libc::c_char) as isize,
                ) = '\0' as i32 as libc::c_char;
            ctoks_0 = _uuconf_istrsplit(zline, '\0' as i32, &mut pzsplit, &mut csplit);
            if ctoks_0 < 0 as libc::c_int {
                (*qglobal).ierrno = *__errno_location();
                iret = 4 as libc::c_int | 0x100 as libc::c_int;
                break;
            } else {
                if ctoks_0 == 0 as libc::c_int {
                    continue;
                }
                zcomma = strchr(*pzsplit.offset(0 as libc::c_int as isize), ',' as i32);
                if zcomma.is_null() {
                    continue;
                }
                let fresh1 = zcomma;
                zcomma = zcomma.offset(1);
                *fresh1 = '\0' as i32 as libc::c_char;
                fcallback = 0 as libc::c_int;
                pzlist = pzsplit.offset(1 as libc::c_int as isize);
                ctoks_0 -= 1;
                ctoks_0;
                if ctoks_0 > 0 as libc::c_int
                    && *(*pzsplit.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == 'c' as i32
                    && *(*pzsplit.offset(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    fcallback = 1 as libc::c_int;
                    pzlist = pzsplit.offset(2 as libc::c_int as isize);
                    ctoks_0 -= 1;
                    ctoks_0;
                }
                if (*(*pzsplit.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
                    || !pzlocal.is_null())
                    && (*zcomma.offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32 || !pzremote.is_null())
                    && strcmp(zcomma, zsystem) != 0 as libc::c_int
                {
                    continue;
                }
                pznew = uuconf_malloc(
                    pblock,
                    ((ctoks_0 + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                if pznew.is_null() {
                    (*qglobal).ierrno = *__errno_location();
                    iret = 4 as libc::c_int | 0x100 as libc::c_int;
                    break;
                } else {
                    memcpy(
                        pznew as pointer,
                        pzlist as pointer as *const libc::c_void,
                        (ctoks_0 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    );
                    let ref mut fresh2 = *pznew.offset(ctoks_0 as isize);
                    *fresh2 = 0 as *mut libc::c_char;
                    if uuconf_add_block(pblock, zline as *mut libc::c_void)
                        != 0 as libc::c_int
                    {
                        (*qglobal).ierrno = *__errno_location();
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    } else {
                        zline = 0 as *mut libc::c_char;
                        cline = 0 as libc::c_int as size_t;
                        if *(*pzsplit.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        {
                            pzlocal = pznew;
                            fdefault_callback = fcallback;
                        } else if *zcomma.offset(0 as libc::c_int as isize)
                            as libc::c_int == '\0' as i32
                        {
                            pzremote = pznew;
                            zdefault_login = *pzsplit.offset(0 as libc::c_int as isize);
                        } else {
                            q = qsys;
                            while !q.is_null() {
                                (*q)
                                    .uuconf_zcalled_login = *pzsplit
                                    .offset(0 as libc::c_int as isize);
                                (*q).uuconf_fcallback = fcallback;
                                (*q).uuconf_pzlocal_send = pznew;
                                (*q).uuconf_pzlocal_receive = pznew;
                                (*q).uuconf_pzremote_send = pznew;
                                (*q).uuconf_pzremote_receive = pznew;
                                q = (*q).uuconf_qalternate;
                            }
                            break;
                        }
                    }
                }
            }
        }
        fclose(e);
        if iret != 0 as libc::c_int {
            if !zline.is_null() {
                free(zline as pointer);
            }
            if !pzsplit.is_null() {
                free(pzsplit as pointer);
            }
            (*qglobal).zfilename = (*(*qglobal).qprocess).zv2userfile;
            return iret | 0x200 as libc::c_int | 0x400 as libc::c_int;
        }
        if (*qsys).uuconf_pzlocal_send == &mut _uuconf_unset as *mut *mut libc::c_char
            && !pzlocal.is_null()
        {
            q = qsys;
            while !q.is_null() {
                (*q).uuconf_fcallback = fdefault_callback;
                (*q).uuconf_pzlocal_send = pzlocal;
                (*q).uuconf_pzlocal_receive = pzlocal;
                q = (*q).uuconf_qalternate;
            }
        }
        if (*qsys).uuconf_pzremote_send == &mut _uuconf_unset as *mut *mut libc::c_char
            && !pzremote.is_null()
        {
            q = qsys;
            while !q.is_null() {
                (*q).uuconf_zcalled_login = zdefault_login;
                (*q).uuconf_pzremote_send = pzremote;
                (*q).uuconf_pzremote_receive = pzremote;
                q = (*q).uuconf_qalternate;
            }
        }
    }
    e = fopen(
        (*(*qglobal).qprocess).zv2cmds,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !e.is_null() {
        (*qglobal).ilineno = 0 as libc::c_int;
        if getline(&mut zline, &mut cline, e) > 0 as libc::c_int as libc::c_long {
            (*qglobal).ilineno += 1;
            (*qglobal).ilineno;
            *zline
                .offset(
                    strcspn(zline, b"#\n\0" as *const u8 as *const libc::c_char) as isize,
                ) = '\0' as i32 as libc::c_char;
            while *zline as libc::c_int == '\0' as i32 {
                if getline(&mut zline, &mut cline, e) <= 0 as libc::c_int as libc::c_long
                {
                    if !zline.is_null() {
                        free(zline as pointer);
                        zline = 0 as *mut libc::c_char;
                    }
                } else {
                    (*qglobal).ilineno += 1;
                    (*qglobal).ilineno;
                    *zline
                        .offset(
                            strcspn(zline, b"#\n\0" as *const u8 as *const libc::c_char)
                                as isize,
                        ) = '\0' as i32 as libc::c_char;
                }
            }
            if !zline.is_null()
                && strncmp(
                    zline,
                    b"PATH=\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
            {
                let mut ctoks_1: libc::c_int = 0;
                let mut pznew_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                zline = zline
                    .offset(
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                ctoks_1 = _uuconf_istrsplit(
                    zline,
                    ':' as i32,
                    &mut pzsplit,
                    &mut csplit,
                );
                if ctoks_1 < 0 as libc::c_int {
                    (*qglobal).ierrno = *__errno_location();
                    iret = 4 as libc::c_int | 0x100 as libc::c_int;
                }
                pznew_0 = 0 as *mut *mut libc::c_char;
                if iret == 0 as libc::c_int {
                    pznew_0 = uuconf_malloc(
                        pblock,
                        ((ctoks_1 + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                    if pznew_0.is_null() {
                        iret = 4 as libc::c_int | 0x100 as libc::c_int;
                    }
                }
                if iret == 0 as libc::c_int {
                    memcpy(
                        pznew_0 as pointer,
                        pzsplit as pointer as *const libc::c_void,
                        (ctoks_1 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    );
                    let ref mut fresh3 = *pznew_0.offset(ctoks_1 as isize);
                    *fresh3 = 0 as *mut libc::c_char;
                    (*qsys).uuconf_pzpath = pznew_0;
                    zline = 0 as *mut libc::c_char;
                    cline = 0 as libc::c_int as size_t;
                }
                if getline(&mut zline, &mut cline, e) < 0 as libc::c_int as libc::c_long
                {
                    if !zline.is_null() {
                        free(zline as pointer);
                        zline = 0 as *mut libc::c_char;
                    }
                } else {
                    (*qglobal).ilineno += 1;
                    (*qglobal).ilineno;
                }
            }
        }
        if iret == 0 as libc::c_int && !zline.is_null() {
            loop {
                *zline
                    .offset(
                        strcspn(zline, b"#,\n\0" as *const u8 as *const libc::c_char)
                            as isize,
                    ) = '\0' as i32 as libc::c_char;
                if *zline as libc::c_int != '\0' as i32 {
                    iret = _uuconf_iadd_string(
                        qglobal,
                        zline,
                        1 as libc::c_int,
                        0 as libc::c_int,
                        &mut (*qsys).uuconf_pzcmds,
                        pblock,
                    );
                    if iret != 0 as libc::c_int {
                        break;
                    }
                }
                if getline(&mut zline, &mut cline, e) < 0 as libc::c_int as libc::c_long
                {
                    break;
                }
                (*qglobal).ilineno += 1;
                (*qglobal).ilineno;
            }
        }
        fclose(e);
        if iret != 0 as libc::c_int {
            (*qglobal).zfilename = (*(*qglobal).qprocess).zv2cmds;
            iret |= 0x200 as libc::c_int | 0x400 as libc::c_int;
        }
    }
    if !zline.is_null() {
        free(zline as pointer);
    }
    if !pzsplit.is_null() {
        free(pzsplit as pointer);
    }
    return iret;
}

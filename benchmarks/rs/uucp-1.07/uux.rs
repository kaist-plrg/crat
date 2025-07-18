use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdin: *mut FILE;
    static mut iDebug: libc::c_int;
    fn funknown_system(
        puuconf: pointer,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> boolean;
    fn zremove_local_sys(
        qlocalsys: *mut uuconf_system,
        z: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn fin_directory_list(
        zfile: *const libc::c_char,
        pzdirs: *mut *mut libc::c_char,
        zpubdir: *const libc::c_char,
        fcheck: boolean,
        freadable: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn zquote_cmd_string(
        zorig: *const libc::c_char,
        fbackslashonly: boolean,
    ) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_fatal_fn(pfn: Option::<unsafe extern "C" fn() -> ()>);
    fn ulog_to_file(puuconf: pointer, ffile: boolean);
    fn ulog_system(zsystem: *const libc::c_char);
    fn ulog_user(zuser: *const libc::c_char);
    fn ulog_close();
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn fcopy_open_file(
        efrom: openfile_t,
        zto: *const libc::c_char,
        fpublic: boolean,
        fmkdirs: boolean,
        fsignals: boolean,
    ) -> boolean;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    static mut zProgram: *const libc::c_char;
    static mut afSignal: [sig_atomic_t; 5];
    fn uuconf_init(
        uuconf_ppglobal: *mut *mut libc::c_void,
        uuconf_zprogram: *const libc::c_char,
        uuconf_zname: *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_system_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zsystem: *const libc::c_char,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_system_local(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debuglevel(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn zsysdep_localname() -> *const libc::c_char;
    fn zsysdep_login_name() -> *const libc::c_char;
    fn usysdep_signal(isig: libc::c_int);
    fn fsysdep_link(
        zfrom: *const libc::c_char,
        zto: *const libc::c_char,
        pfworked: *mut boolean,
    ) -> boolean;
    fn fsysdep_run(
        ffork: boolean,
        zprogram: *const libc::c_char,
        zarg1: *const libc::c_char,
        zarg2: *const libc::c_char,
    ) -> boolean;
    fn esysdep_fopen(
        zfile: *const libc::c_char,
        fpublic: boolean,
        fappend: boolean,
        fmkdirs: boolean,
    ) -> *mut FILE;
    fn esysdep_user_fopen(
        zfile: *const libc::c_char,
        frd: boolean,
        fbinary: boolean,
    ) -> openfile_t;
    fn fsysdep_sync(e: openfile_t, zmsg: *const libc::c_char) -> boolean;
    fn zsysdep_spool_commands(
        qsys: *const uuconf_system,
        bgrade: libc::c_int,
        ccmds: libc::c_int,
        pascmds: *const scmd,
        pftemp: *mut boolean,
    ) -> *mut libc::c_char;
    fn zsysdep_data_file_name(
        qsys: *const uuconf_system,
        zlocalname: *const libc::c_char,
        bgrade: libc::c_int,
        fxqt: boolean,
        ztname: *mut libc::c_char,
        zdname: *mut libc::c_char,
        zxname: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn zsysdep_xqt_file_name() -> *mut libc::c_char;
    fn zsysdep_local_file_cwd(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn zsysdep_add_cwd(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn fsysdep_needs_cwd(zfile: *const libc::c_char) -> boolean;
    fn zsysdep_base_name(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn fsysdep_access(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_daemon_access(zfile: *const libc::c_char) -> boolean;
    static mut gnu_optarg: *mut libc::c_char;
    static mut gnu_optind: libc::c_int;
    static mut gnu_opterr: libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type openfile_t = *mut FILE;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
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
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scmd {
    pub bcmd: libc::c_char,
    pub bgrade: libc::c_char,
    pub pseq: pointer,
    pub zfrom: *const libc::c_char,
    pub zto: *const libc::c_char,
    pub zuser: *const libc::c_char,
    pub zoptions: *const libc::c_char,
    pub ztemp: *const libc::c_char,
    pub imode: libc::c_uint,
    pub znotify: *const libc::c_char,
    pub cbytes: libc::c_long,
    pub zcmd: *const libc::c_char,
    pub ipos: libc::c_long,
}
pub const _ISalnum: C2RustUnnamed_0 = 8;
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
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
pub static mut uux_rcsid: [libc::c_char; 48] = unsafe {
    *::std::mem::transmute::<
        &[u8; 48],
        &[libc::c_char; 48],
    >(b"$Id: uux.c,v 1.89 2002/03/05 19:10:42 ian Rel $\0")
};
static mut fXquote: boolean = 0;
static mut fXquote_output: boolean = 0;
static mut fXxqtlocal: boolean = 0;
static mut sXxqtsys: uuconf_system = uuconf_system {
    uuconf_zname: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_pzalias: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_qalternate: 0 as *const uuconf_system as *mut uuconf_system,
    uuconf_zalternate: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_fcall: 0,
    uuconf_fcalled: 0,
    uuconf_qtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalltimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalledtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_cmax_retries: 0,
    uuconf_csuccess_wait: 0,
    uuconf_qcall_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcall_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalled_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalled_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_ibaud: 0,
    uuconf_ihighbaud: 0,
    uuconf_zport: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_qport: 0 as *const uuconf_port as *mut uuconf_port,
    uuconf_zphone: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_schat: uuconf_chat {
        uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_ctimeout: 0,
        uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_fstrip: 0,
    },
    uuconf_zcall_login: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_zcall_password: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_zcalled_login: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_fcallback: 0,
    uuconf_fsequence: 0,
    uuconf_zprotocols: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_qproto_params: 0 as *const uuconf_proto_param as *mut uuconf_proto_param,
    uuconf_scalled_chat: uuconf_chat {
        uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_ctimeout: 0,
        uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_fstrip: 0,
    },
    uuconf_zdebug: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_zmax_remote_debug: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_fsend_request: 0,
    uuconf_frec_request: 0,
    uuconf_fcall_transfer: 0,
    uuconf_fcalled_transfer: 0,
    uuconf_pzlocal_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzremote_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzlocal_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzremote_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzpath: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzcmds: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_cfree_space: 0,
    uuconf_pzforward_from: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzforward_to: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_zpubdir: 0 as *const libc::c_char,
    uuconf_zlocalname: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_cmax_file_time: 0,
    uuconf_palloc: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut zXxqtloc: *const libc::c_char = 0 as *const libc::c_char;
static mut bXgrade: libc::c_char = 'N' as i32 as libc::c_char;
static mut abXxqt_tname: [libc::c_char; 15] = [0; 15];
static mut abXxqt_xname: [libc::c_char; 15] = [0; 15];
static mut eXxqt_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut pasXcmds: *mut scmd = 0 as *const scmd as *mut scmd;
static mut cXcmds: libc::c_int = 0;
static mut eXclose: *mut FILE = 0 as *const FILE as *mut FILE;
static mut zXnames: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut asXlongopts: [option; 17] = [
    {
        let mut init = option {
            name: b"requestor\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"return-stdin\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nocopy\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"copy\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"grade\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"jobid\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'j' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"link\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"notification\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdin\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nouucico\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"status\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"noexpand\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
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
    let mut zrequestor: *const libc::c_char = 0 as *const libc::c_char;
    let mut fretstdin: boolean = 0 as libc::c_int;
    let mut fcopy: boolean = 0 as libc::c_int;
    let mut fdontcopy: boolean = 0 as libc::c_int;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut fjobid: boolean = 0 as libc::c_int;
    let mut flink: boolean = 0 as libc::c_int;
    let mut fno_ack: boolean = 0 as libc::c_int;
    let mut fread_stdin: boolean = 0 as libc::c_int;
    let mut fuucico: boolean = 1 as libc::c_int;
    let mut zstatus_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut fexpand: boolean = 1 as libc::c_int;
    let mut ferror_ack: boolean = 0 as libc::c_int;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut zlocalname: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut clen: size_t = 0;
    let mut zargs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zsys: *const libc::c_char = 0 as *const libc::c_char;
    let mut zexclam: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fgetcwd: boolean = 0;
    let mut zuser: *const libc::c_char = 0 as *const libc::c_char;
    let mut zforward: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pzargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut calloc_args: libc::c_int = 0;
    let mut cargs: libc::c_int = 0;
    let mut zinput_from: *const libc::c_char = 0 as *const libc::c_char;
    let mut zinput_to: *const libc::c_char = 0 as *const libc::c_char;
    let mut zinput_temp: *const libc::c_char = 0 as *const libc::c_char;
    let mut finputcopied: boolean = 0;
    let mut zcall_system: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fcall_any: boolean = 0;
    let mut slocalsys: uuconf_system = uuconf_system {
        uuconf_zname: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_pzalias: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_qalternate: 0 as *const uuconf_system as *mut uuconf_system,
        uuconf_zalternate: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_fcall: 0,
        uuconf_fcalled: 0,
        uuconf_qtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalltimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalledtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_cmax_retries: 0,
        uuconf_csuccess_wait: 0,
        uuconf_qcall_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcall_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalled_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalled_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_ibaud: 0,
        uuconf_ihighbaud: 0,
        uuconf_zport: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_qport: 0 as *const uuconf_port as *mut uuconf_port,
        uuconf_zphone: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_schat: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zcall_login: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_zcall_password: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_zcalled_login: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_fcallback: 0,
        uuconf_fsequence: 0,
        uuconf_zprotocols: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_qproto_params: 0 as *const uuconf_proto_param as *mut uuconf_proto_param,
        uuconf_scalled_chat: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zdebug: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_zmax_remote_debug: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_fsend_request: 0,
        uuconf_frec_request: 0,
        uuconf_fcall_transfer: 0,
        uuconf_fcalled_transfer: 0,
        uuconf_pzlocal_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzremote_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzlocal_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzremote_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzpath: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzcmds: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_cfree_space: 0,
        uuconf_pzforward_from: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzforward_to: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_zpubdir: 0 as *const libc::c_char,
        uuconf_zlocalname: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_cmax_file_time: 0,
        uuconf_palloc: 0 as *const libc::c_void as *mut libc::c_void,
    };
    let mut fneedshell: boolean = 0;
    let mut zfullcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fpoll: boolean = 0;
    let mut aboptions: [libc::c_char; 10] = [0; 10];
    zProgram = *argv.offset(0 as libc::c_int as isize);
    ulog_fatal_fn(Some(uxfatal as unsafe extern "C" fn() -> ()));
    gnu_opterr = 0 as libc::c_int;
    loop {
        while getopt_long(
            argc,
            argv,
            b"+a:bcCg:I:jlnprs:Wvx:z\0" as *const u8 as *const libc::c_char,
            asXlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        ) != -(1 as libc::c_int)
        {}
        if gnu_optind >= argc
            || strcmp(
                *argv.offset(gnu_optind as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
        {
            break;
        }
        let ref mut fresh0 = *argv.offset(gnu_optind as isize);
        *fresh0 = zbufcpy(b"-p\0" as *const u8 as *const libc::c_char);
        gnu_optind = 0 as libc::c_int;
    }
    gnu_opterr = 1 as libc::c_int;
    gnu_optind = 0 as libc::c_int;
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"+a:bcCg:I:jlnprs:Wvx:z\0" as *const u8 as *const libc::c_char,
            asXlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            97 => {
                zrequestor = gnu_optarg;
            }
            98 => {
                fretstdin = 1 as libc::c_int;
            }
            99 => {
                fcopy = 0 as libc::c_int;
                fdontcopy = 1 as libc::c_int;
            }
            67 => {
                fcopy = 1 as libc::c_int;
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            106 => {
                fjobid = 1 as libc::c_int;
            }
            103 => {
                bXgrade = *gnu_optarg.offset(0 as libc::c_int as isize);
            }
            108 => {
                flink = 1 as libc::c_int;
            }
            110 => {
                fno_ack = 1 as libc::c_int;
            }
            112 => {
                fread_stdin = 1 as libc::c_int;
            }
            114 => {
                fuucico = 0 as libc::c_int;
            }
            115 => {
                zstatus_file = gnu_optarg;
            }
            87 => {
                fexpand = 0 as libc::c_int;
            }
            120 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            122 => {
                ferror_ack = 1 as libc::c_int;
            }
            2 => {
                if *gnu_optarg as libc::c_int == 't' as i32
                    || *gnu_optarg as libc::c_int == 'T' as i32
                    || *gnu_optarg as libc::c_int == 'y' as i32
                    || *gnu_optarg as libc::c_int == 'Y' as i32
                    || *gnu_optarg as libc::c_int == 'e' as i32
                    || *gnu_optarg as libc::c_int == 'E' as i32
                {
                    ferror_ack = 1 as libc::c_int;
                    fno_ack = 0 as libc::c_int;
                } else if *gnu_optarg as libc::c_int == 'f' as i32
                    || *gnu_optarg as libc::c_int == 'F' as i32
                    || *gnu_optarg as libc::c_int == 'n' as i32
                    || *gnu_optarg as libc::c_int == 'N' as i32
                {
                    ferror_ack = 0 as libc::c_int;
                    fno_ack = 1 as libc::c_int;
                }
            }
            118 => {
                printf(
                    b"uux (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
                uxhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                uxusage();
            }
        }
    }
    if *(*__ctype_b_loc()).offset(bXgrade as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
        || ((bXgrade as libc::c_int) < '0' as i32 || bXgrade as libc::c_int > '9' as i32)
            && ((bXgrade as libc::c_int) < 'a' as i32
                || bXgrade as libc::c_int > 'z' as i32)
            && ((bXgrade as libc::c_int) < 'A' as i32
                || bXgrade as libc::c_int > 'Z' as i32)
    {
        ulog(LOG_ERROR, b"Ignoring illegal grade\0" as *const u8 as *const libc::c_char);
        bXgrade = 'N' as i32 as libc::c_char;
    }
    if !zrequestor.is_null()
        && *zrequestor
            .offset(
                strcspn(zrequestor, b" \t\n\0" as *const u8 as *const libc::c_char)
                    as isize,
            ) as libc::c_int != '\0' as i32
        || !zstatus_file.is_null()
            && *zstatus_file
                .offset(
                    strcspn(zstatus_file, b" \t\n\0" as *const u8 as *const libc::c_char)
                        as isize,
                ) as libc::c_int != '\0' as i32
    {
        fXquote = 1 as libc::c_int;
    }
    if gnu_optind == argc {
        uxusage();
    }
    iuuconf = uuconf_init(
        &mut puuconf,
        0 as *mut libc::c_void as *const libc::c_char,
        zconfig,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    let mut zdebug: *const libc::c_char = 0 as *const libc::c_char;
    iuuconf = uuconf_debuglevel(puuconf, &mut zdebug);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    if !zdebug.is_null() {
        iDebug |= idebug_parse(zdebug);
    }
    clen = 1 as libc::c_int as size_t;
    i = gnu_optind;
    while i < argc {
        clen = (clen as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        i += 1;
        i;
    }
    zargs = zbufalc(clen);
    *zargs = '\0' as i32 as libc::c_char;
    i = gnu_optind;
    while i < argc {
        strcat(zargs, *argv.offset(i as isize));
        strcat(zargs, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    clen = strcspn(zargs, b";&*|<> \t\0" as *const u8 as *const libc::c_char);
    zcmd = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
    strncpy(zcmd, zargs, clen);
    *zcmd.offset(clen as isize) = '\0' as i32 as libc::c_char;
    zargs = zargs.offset(clen as isize);
    calloc_args = 10 as libc::c_int;
    pzargs = xmalloc(
        (calloc_args as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    cargs = 0 as libc::c_int;
    zarg = strtok(zargs, b" \t\0" as *const u8 as *const libc::c_char);
    while !zarg.is_null() {
        while *zarg as libc::c_int != '\0' as i32 {
            if cargs + 1 as libc::c_int >= calloc_args {
                calloc_args += 10 as libc::c_int;
                pzargs = xrealloc(
                    pzargs as pointer,
                    (calloc_args as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
            }
            if *zarg as libc::c_int == '(' as i32 {
                clen = strlen(zarg);
            } else {
                clen = strcspn(zarg, b";&*|<> \t\0" as *const u8 as *const libc::c_char);
            }
            if clen > 0 as libc::c_int as libc::c_ulong {
                let ref mut fresh1 = *pzargs.offset(cargs as isize);
                *fresh1 = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
                memcpy(
                    *pzargs.offset(cargs as isize) as *mut libc::c_void,
                    zarg as *const libc::c_void,
                    clen,
                );
                *(*pzargs.offset(cargs as isize))
                    .offset(clen as isize) = '\0' as i32 as libc::c_char;
                cargs += 1;
                cargs;
                zarg = zarg.offset(clen as isize);
            }
            if *zarg as libc::c_int != '\0' as i32 {
                clen = strspn(zarg, b";&*| \t\0" as *const u8 as *const libc::c_char);
                if clen == 0 as libc::c_int as libc::c_ulong {
                    clen = 1 as libc::c_int as size_t;
                }
                let ref mut fresh2 = *pzargs.offset(cargs as isize);
                *fresh2 = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
                memcpy(
                    *pzargs.offset(cargs as isize) as *mut libc::c_void,
                    zarg as *const libc::c_void,
                    clen,
                );
                *(*pzargs.offset(cargs as isize))
                    .offset(clen as isize) = '\0' as i32 as libc::c_char;
                cargs += 1;
                cargs;
                zarg = zarg.offset(clen as isize);
            }
        }
        zarg = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
    }
    fgetcwd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < cargs {
        if !(*(*pzargs.offset(i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '(' as i32)
        {
            zexclam = strrchr(*pzargs.offset(i as isize), '!' as i32);
            if !zexclam.is_null()
                && fsysdep_needs_cwd(zexclam.offset(1 as libc::c_int as isize)) != 0
            {
                fgetcwd = 1 as libc::c_int;
                break;
            } else if (*(*pzargs.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '<' as i32
                || *(*pzargs.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '>' as i32) && (i + 1 as libc::c_int) < cargs
                && (strchr(*pzargs.offset((i + 1 as libc::c_int) as isize), '!' as i32))
                    .is_null()
                && fsysdep_needs_cwd(*pzargs.offset((i + 1 as libc::c_int) as isize))
                    != 0
            {
                fgetcwd = 1 as libc::c_int;
                break;
            }
        }
        i += 1;
        i;
    }
    usysdep_signal(2 as libc::c_int);
    usysdep_signal(1 as libc::c_int);
    usysdep_signal(3 as libc::c_int);
    usysdep_signal(15 as libc::c_int);
    usysdep_signal(13 as libc::c_int);
    usysdep_initialize(
        puuconf,
        0o4 as libc::c_int
            | (if fgetcwd != 0 { 0o1 as libc::c_int } else { 0 as libc::c_int }),
    );
    zuser = zsysdep_login_name();
    iuuconf = uuconf_localname(puuconf, &mut zlocalname);
    if iuuconf == 1 as libc::c_int {
        zlocalname = zsysdep_localname();
        if zlocalname.is_null() {
            exit(78 as libc::c_int);
        }
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_system_info(puuconf, zlocalname, &mut slocalsys);
    if iuuconf != 0 as libc::c_int {
        if iuuconf != 1 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        iuuconf = uuconf_system_local(puuconf, &mut slocalsys);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        slocalsys.uuconf_zname = zlocalname as *mut libc::c_char;
    }
    zcmd = zremove_local_sys(&mut slocalsys, zcmd);
    zexclam = strchr(zcmd, '!' as i32);
    if zexclam.is_null() {
        zsys = zlocalname;
        fXxqtlocal = 1 as libc::c_int;
        zforward = 0 as *mut libc::c_char;
    } else {
        *zexclam = '\0' as i32 as libc::c_char;
        zsys = zcmd;
        zcmd = zexclam.offset(1 as libc::c_int as isize);
        fXxqtlocal = 0 as libc::c_int;
        zexclam = strrchr(zcmd, '!' as i32);
        if zexclam.is_null() {
            zforward = 0 as *mut libc::c_char;
        } else {
            clen = zexclam.offset_from(zcmd) as libc::c_long as size_t;
            zforward = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
            memcpy(zforward as *mut libc::c_void, zcmd as *const libc::c_void, clen);
            *zforward.offset(clen as isize) = '\0' as i32 as libc::c_char;
            zcmd = zexclam.offset(1 as libc::c_int as isize);
        }
    }
    if fXxqtlocal != 0 {
        sXxqtsys = slocalsys;
    } else {
        iuuconf = uuconf_system_info(puuconf, zsys, &mut sXxqtsys);
        if iuuconf != 0 as libc::c_int {
            if iuuconf != 1 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            if funknown_system(puuconf, zsys, &mut sXxqtsys) == 0 {
                ulog(
                    LOG_FATAL,
                    b"%s: System not found\0" as *const u8 as *const libc::c_char,
                    zsys,
                );
            }
        }
    }
    zXxqtloc = sXxqtsys.uuconf_zlocalname;
    if zXxqtloc.is_null() {
        zXxqtloc = zlocalname;
    }
    zinput_from = 0 as *const libc::c_char;
    zinput_to = 0 as *const libc::c_char;
    zinput_temp = 0 as *const libc::c_char;
    finputcopied = 0 as libc::c_int;
    zcall_system = 0 as *mut libc::c_char;
    fcall_any = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < cargs {
        let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
        let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zforw: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut finput: boolean = 0;
        let mut foutput: boolean = 0;
        let mut flocal: boolean = 0;
        let mut fonxqt: boolean = 0;
        if *(*pzargs.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '(' as i32
        {
            clen = strlen(*pzargs.offset(i as isize));
            if *(*pzargs.offset(i as isize))
                .offset(clen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int != ')' as i32
            {
                ulog(
                    LOG_ERROR,
                    b"Mismatched parentheses\0" as *const u8 as *const libc::c_char,
                );
            } else {
                *(*pzargs.offset(i as isize))
                    .offset(
                        clen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            let ref mut fresh3 = *pzargs.offset(i as isize);
            *fresh3 = (*fresh3).offset(1);
            *fresh3;
        } else {
            finput = 0 as libc::c_int;
            foutput = 0 as libc::c_int;
            if (i + 1 as libc::c_int) < cargs {
                if *(*pzargs.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '<' as i32
                {
                    finput = 1 as libc::c_int;
                } else if *(*pzargs.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '>' as i32
                {
                    foutput = 1 as libc::c_int;
                }
                if finput != 0 || foutput != 0 {
                    let ref mut fresh4 = *pzargs.offset(i as isize);
                    *fresh4 = 0 as *mut libc::c_char;
                    i += 1;
                    i;
                }
            }
            zexclam = strchr(*pzargs.offset(i as isize), '!' as i32);
            if !(zexclam.is_null() && finput == 0 && foutput == 0) {
                if !zexclam.is_null() {
                    let ref mut fresh5 = *pzargs.offset(i as isize);
                    *fresh5 = zremove_local_sys(
                        &mut slocalsys,
                        *pzargs.offset(i as isize),
                    );
                    zexclam = strchr(*pzargs.offset(i as isize), '!' as i32);
                }
                if zexclam.is_null() {
                    zsystem = zlocalname;
                    zfile = *pzargs.offset(i as isize);
                    flocal = 1 as libc::c_int;
                    zforw = 0 as *mut libc::c_char;
                } else {
                    *zexclam = '\0' as i32 as libc::c_char;
                    zsystem = *pzargs.offset(i as isize);
                    zfile = zexclam.offset(1 as libc::c_int as isize);
                    flocal = 0 as libc::c_int;
                    zexclam = strrchr(zfile, '!' as i32);
                    if zexclam.is_null() {
                        zforw = 0 as *mut libc::c_char;
                    } else {
                        *zexclam = '\0' as i32 as libc::c_char;
                        zforw = zfile;
                        zfile = zexclam.offset(1 as libc::c_int as isize);
                    }
                }
                if flocal != 0 {
                    fonxqt = fXxqtlocal;
                } else if fXxqtlocal != 0 {
                    fonxqt = 0 as libc::c_int;
                } else if if zforward.is_null() {
                    (zforw != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                } else {
                    (zforw == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                } != 0
                {
                    fonxqt = 0 as libc::c_int;
                } else if !zforward.is_null() && !zforw.is_null()
                    && strcmp(zforward, zforw) != 0 as libc::c_int
                {
                    fonxqt = 0 as libc::c_int;
                } else if strcmp(zsystem, sXxqtsys.uuconf_zname) == 0 as libc::c_int {
                    fonxqt = 1 as libc::c_int;
                } else if (sXxqtsys.uuconf_pzalias).is_null() {
                    fonxqt = 0 as libc::c_int;
                } else {
                    let mut pzal: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                    fonxqt = 0 as libc::c_int;
                    pzal = sXxqtsys.uuconf_pzalias;
                    while !(*pzal).is_null() {
                        if strcmp(zsystem, *pzal) == 0 as libc::c_int {
                            fonxqt = 1 as libc::c_int;
                            break;
                        } else {
                            pzal = pzal.offset(1);
                            pzal;
                        }
                    }
                }
                if flocal != 0 {
                    zfile = zsysdep_local_file_cwd(
                        zfile,
                        sXxqtsys.uuconf_zpubdir,
                        0 as *mut libc::c_void as *mut boolean,
                    );
                } else if fexpand != 0 {
                    zfile = zsysdep_add_cwd(zfile);
                }
                if zfile.is_null() {
                    uxabort(71 as libc::c_int);
                }
                if foutput != 0 {
                    if flocal != 0 {
                        if fin_directory_list(
                            zfile,
                            sXxqtsys.uuconf_pzremote_receive,
                            sXxqtsys.uuconf_zpubdir,
                            1 as libc::c_int,
                            0 as libc::c_int,
                            0 as *mut libc::c_void as *const libc::c_char,
                        ) == 0
                        {
                            ulog(
                                LOG_FATAL,
                                b"Not permitted to create %s\0" as *const u8
                                    as *const libc::c_char,
                                zfile,
                            );
                        }
                    }
                    if !zforward.is_null() || !zforw.is_null() {
                        ulog(
                            LOG_FATAL,
                            b"May not forward standard output\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if fonxqt != 0 {
                        uxadd_xqt_line(
                            'O' as i32,
                            zfile,
                            0 as *mut libc::c_void as *const libc::c_char,
                        );
                    } else if flocal != 0 {
                        uxadd_xqt_line('O' as i32, zfile, zXxqtloc);
                    } else {
                        uxadd_xqt_line('O' as i32, zfile, zsystem);
                    }
                    let ref mut fresh6 = *pzargs.offset(i as isize);
                    *fresh6 = 0 as *mut libc::c_char;
                } else {
                    if finput != 0 {
                        if fread_stdin != 0 {
                            ulog(
                                LOG_FATAL,
                                b"Standard input specified twice\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        let ref mut fresh7 = *pzargs.offset(i as isize);
                        *fresh7 = 0 as *mut libc::c_char;
                    }
                    if flocal != 0 {
                        let mut zuse: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut zdata: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut abtname: [libc::c_char; 15] = [0; 15];
                        let mut abdname: [libc::c_char; 15] = [0; 15];
                        if fsysdep_access(zfile) == 0 {
                            uxabort(66 as libc::c_int);
                        }
                        zdata = zsysdep_data_file_name(
                            &mut sXxqtsys,
                            zXxqtloc,
                            bXgrade as libc::c_int,
                            0 as libc::c_int,
                            abtname.as_mut_ptr(),
                            abdname.as_mut_ptr(),
                            0 as *mut libc::c_void as *mut libc::c_char,
                        );
                        if zdata.is_null() {
                            uxabort(71 as libc::c_int);
                        }
                        if fcopy != 0 || flink != 0 || fXxqtlocal != 0 {
                            let mut fdid: boolean = 0;
                            uxrecord_file(zdata);
                            fdid = 0 as libc::c_int;
                            if flink != 0 {
                                let mut fworked: boolean = 0;
                                if fsysdep_link(zfile, zdata, &mut fworked) == 0 {
                                    uxabort(71 as libc::c_int);
                                }
                                if fworked != 0 {
                                    fdid = 1 as libc::c_int;
                                } else if fdontcopy != 0 {
                                    ulog(
                                        LOG_FATAL,
                                        b"%s: Can't link to spool directory\0" as *const u8
                                            as *const libc::c_char,
                                        zfile,
                                    );
                                }
                            }
                            if fdid == 0 {
                                let mut efile: openfile_t = 0 as *mut FILE;
                                efile = esysdep_user_fopen(
                                    zfile,
                                    1 as libc::c_int,
                                    1 as libc::c_int,
                                );
                                if efile.is_null() {
                                    uxabort(66 as libc::c_int);
                                }
                                if fcopy_open_file(
                                    efile,
                                    zdata,
                                    0 as libc::c_int,
                                    1 as libc::c_int,
                                    1 as libc::c_int,
                                ) == 0
                                {
                                    uxabort(73 as libc::c_int);
                                }
                                fclose(efile);
                            }
                            zuse = abtname.as_mut_ptr();
                        } else {
                            ubuffree(zdata);
                            if fsysdep_daemon_access(zfile) == 0 {
                                uxabort(66 as libc::c_int);
                            }
                            if fin_directory_list(
                                zfile,
                                sXxqtsys.uuconf_pzlocal_send,
                                sXxqtsys.uuconf_zpubdir,
                                1 as libc::c_int,
                                1 as libc::c_int,
                                zuser,
                            ) == 0
                            {
                                ulog(
                                    LOG_FATAL,
                                    b"Not permitted to send from %s\0" as *const u8
                                        as *const libc::c_char,
                                    zfile,
                                );
                            }
                            zuse = zfile;
                        }
                        if fXxqtlocal != 0 {
                            if finput != 0 {
                                uxadd_xqt_line(
                                    'I' as i32,
                                    zuse,
                                    0 as *mut libc::c_void as *mut libc::c_char,
                                );
                            } else {
                                let ref mut fresh8 = *pzargs.offset(i as isize);
                                *fresh8 = zuse;
                            }
                        } else {
                            finputcopied = (fcopy != 0 || flink != 0) as libc::c_int;
                            if finput != 0 {
                                zinput_from = zuse;
                                zinput_to = zbufcpy(abdname.as_mut_ptr());
                                zinput_temp = zbufcpy(abtname.as_mut_ptr());
                            } else {
                                let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
                                uxadd_send_file(
                                    zuse,
                                    abdname.as_mut_ptr(),
                                    if finputcopied != 0 {
                                        b"C\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"c\0" as *const u8 as *const libc::c_char
                                    },
                                    abtname.as_mut_ptr(),
                                    zforward,
                                );
                                zbase = zsysdep_base_name(zfile);
                                if zbase.is_null() {
                                    uxabort(71 as libc::c_int);
                                }
                                uxadd_xqt_line('F' as i32, abdname.as_mut_ptr(), zbase);
                                let ref mut fresh9 = *pzargs.offset(i as isize);
                                *fresh9 = zbase;
                            }
                        }
                    } else if fonxqt != 0 {
                        if finput != 0 {
                            uxadd_xqt_line(
                                'I' as i32,
                                zfile,
                                0 as *mut libc::c_void as *const libc::c_char,
                            );
                        } else {
                            let ref mut fresh10 = *pzargs.offset(i as isize);
                            *fresh10 = zfile;
                        }
                    } else {
                        let mut sfromsys: uuconf_system = uuconf_system {
                            uuconf_zname: 0 as *const libc::c_char as *mut libc::c_char,
                            uuconf_pzalias: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_qalternate: 0 as *const uuconf_system
                                as *mut uuconf_system,
                            uuconf_zalternate: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_fcall: 0,
                            uuconf_fcalled: 0,
                            uuconf_qtimegrade: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_qcalltimegrade: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_qcalledtimegrade: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_cmax_retries: 0,
                            uuconf_csuccess_wait: 0,
                            uuconf_qcall_local_size: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_qcall_remote_size: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_qcalled_local_size: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_qcalled_remote_size: 0 as *const uuconf_timespan
                                as *mut uuconf_timespan,
                            uuconf_ibaud: 0,
                            uuconf_ihighbaud: 0,
                            uuconf_zport: 0 as *const libc::c_char as *mut libc::c_char,
                            uuconf_qport: 0 as *const uuconf_port as *mut uuconf_port,
                            uuconf_zphone: 0 as *const libc::c_char as *mut libc::c_char,
                            uuconf_schat: uuconf_chat {
                                uuconf_pzchat: 0 as *const *mut libc::c_char
                                    as *mut *mut libc::c_char,
                                uuconf_pzprogram: 0 as *const *mut libc::c_char
                                    as *mut *mut libc::c_char,
                                uuconf_ctimeout: 0,
                                uuconf_pzfail: 0 as *const *mut libc::c_char
                                    as *mut *mut libc::c_char,
                                uuconf_fstrip: 0,
                            },
                            uuconf_zcall_login: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_zcall_password: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_zcalled_login: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_fcallback: 0,
                            uuconf_fsequence: 0,
                            uuconf_zprotocols: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_qproto_params: 0 as *const uuconf_proto_param
                                as *mut uuconf_proto_param,
                            uuconf_scalled_chat: uuconf_chat {
                                uuconf_pzchat: 0 as *const *mut libc::c_char
                                    as *mut *mut libc::c_char,
                                uuconf_pzprogram: 0 as *const *mut libc::c_char
                                    as *mut *mut libc::c_char,
                                uuconf_ctimeout: 0,
                                uuconf_pzfail: 0 as *const *mut libc::c_char
                                    as *mut *mut libc::c_char,
                                uuconf_fstrip: 0,
                            },
                            uuconf_zdebug: 0 as *const libc::c_char as *mut libc::c_char,
                            uuconf_zmax_remote_debug: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_fsend_request: 0,
                            uuconf_frec_request: 0,
                            uuconf_fcall_transfer: 0,
                            uuconf_fcalled_transfer: 0,
                            uuconf_pzlocal_send: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_pzremote_send: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_pzlocal_receive: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_pzremote_receive: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_pzpath: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_pzcmds: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_cfree_space: 0,
                            uuconf_pzforward_from: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_pzforward_to: 0 as *const *mut libc::c_char
                                as *mut *mut libc::c_char,
                            uuconf_zpubdir: 0 as *const libc::c_char,
                            uuconf_zlocalname: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            uuconf_cmax_file_time: 0,
                            uuconf_palloc: 0 as *const libc::c_void as *mut libc::c_void,
                        };
                        let mut abtname_0: [libc::c_char; 15] = [0; 15];
                        let mut s: scmd = scmd {
                            bcmd: 0,
                            bgrade: 0,
                            pseq: 0 as *mut libc::c_void,
                            zfrom: 0 as *const libc::c_char,
                            zto: 0 as *const libc::c_char,
                            zuser: 0 as *const libc::c_char,
                            zoptions: 0 as *const libc::c_char,
                            ztemp: 0 as *const libc::c_char,
                            imode: 0,
                            znotify: 0 as *const libc::c_char,
                            cbytes: 0,
                            zcmd: 0 as *const libc::c_char,
                            ipos: 0,
                        };
                        let mut zjobid: *mut libc::c_char = 0 as *mut libc::c_char;
                        iuuconf = uuconf_system_info(puuconf, zsystem, &mut sfromsys);
                        if iuuconf != 0 as libc::c_int {
                            if iuuconf != 1 as libc::c_int {
                                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
                            }
                            if funknown_system(puuconf, zsystem, &mut sfromsys) == 0 {
                                ulog(
                                    LOG_FATAL,
                                    b"%s: System not found\0" as *const u8
                                        as *const libc::c_char,
                                    zsystem,
                                );
                            }
                        }
                        if fonxqt != 0 {
                            if finput != 0 {
                                uxadd_xqt_line(
                                    'I' as i32,
                                    zfile,
                                    0 as *mut libc::c_void as *const libc::c_char,
                                );
                            } else {
                                let ref mut fresh11 = *pzargs.offset(i as isize);
                                *fresh11 = zfile;
                            }
                        } else {
                            let mut zdata_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut ftemp: boolean = 0;
                            if sfromsys.uuconf_fcall_transfer == 0
                                && sfromsys.uuconf_fcalled_transfer == 0
                            {
                                ulog(
                                    LOG_FATAL,
                                    b"Not permitted to transfer files to or from %s\0"
                                        as *const u8 as *const libc::c_char,
                                    sfromsys.uuconf_zname,
                                );
                            }
                            if !zforw.is_null() {
                                ulog(
                                    LOG_FATAL,
                                    b"File forwarding not supported\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            zdata_0 = zsysdep_data_file_name(
                                &mut slocalsys,
                                zXxqtloc,
                                bXgrade as libc::c_int,
                                0 as libc::c_int,
                                abtname_0.as_mut_ptr(),
                                0 as *mut libc::c_void as *mut libc::c_char,
                                0 as *mut libc::c_void as *mut libc::c_char,
                            );
                            if zdata_0.is_null() {
                                uxabort(71 as libc::c_int);
                            }
                            ubuffree(zdata_0);
                            s.bcmd = 'R' as i32 as libc::c_char;
                            s.bgrade = bXgrade;
                            s.pseq = 0 as *mut libc::c_void;
                            s.zfrom = zfile;
                            s.zto = zbufcpy(abtname_0.as_mut_ptr());
                            s.zuser = zuser;
                            s.zoptions = b"9\0" as *const u8 as *const libc::c_char;
                            s.ztemp = b"\0" as *const u8 as *const libc::c_char;
                            s.imode = 0o600 as libc::c_int as libc::c_uint;
                            s.znotify = b"\0" as *const u8 as *const libc::c_char;
                            s.cbytes = -(1 as libc::c_int) as libc::c_long;
                            s.zcmd = 0 as *const libc::c_char;
                            s.ipos = 0 as libc::c_int as libc::c_long;
                            zjobid = zsysdep_spool_commands(
                                &mut sfromsys,
                                bXgrade as libc::c_int,
                                1 as libc::c_int,
                                &mut s,
                                &mut ftemp,
                            );
                            if zjobid.is_null() {
                                uxabort(
                                    if ftemp != 0 {
                                        75 as libc::c_int
                                    } else {
                                        65 as libc::c_int
                                    },
                                );
                            }
                            if fjobid != 0 {
                                printf(
                                    b"%s\n\0" as *const u8 as *const libc::c_char,
                                    zjobid,
                                );
                            }
                            ubuffree(zjobid);
                            if fcall_any != 0 {
                                ubuffree(zcall_system);
                                zcall_system = 0 as *mut libc::c_char;
                            } else {
                                fcall_any = 1 as libc::c_int;
                                zcall_system = zbufcpy(sfromsys.uuconf_zname);
                            }
                            if fXxqtlocal != 0 {
                                if finput != 0 {
                                    uxadd_xqt_line(
                                        'F' as i32,
                                        abtname_0.as_mut_ptr(),
                                        0 as *mut libc::c_void as *mut libc::c_char,
                                    );
                                    uxadd_xqt_line(
                                        'I' as i32,
                                        abtname_0.as_mut_ptr(),
                                        0 as *mut libc::c_void as *mut libc::c_char,
                                    );
                                } else {
                                    let mut zbase_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                    zbase_0 = zsysdep_base_name(zfile);
                                    if zbase_0.is_null() {
                                        uxabort(71 as libc::c_int);
                                    }
                                    uxadd_xqt_line('F' as i32, abtname_0.as_mut_ptr(), zbase_0);
                                    let ref mut fresh12 = *pzargs.offset(i as isize);
                                    *fresh12 = zbase_0;
                                }
                            } else {
                                let mut abxtname: [libc::c_char; 15] = [0; 15];
                                let mut zbase_1: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut zxqt: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut e: *mut FILE = 0 as *mut FILE;
                                zdata_0 = zsysdep_data_file_name(
                                    &mut sXxqtsys,
                                    zXxqtloc,
                                    bXgrade as libc::c_int,
                                    1 as libc::c_int,
                                    abxtname.as_mut_ptr(),
                                    0 as *mut libc::c_void as *mut libc::c_char,
                                    0 as *mut libc::c_void as *mut libc::c_char,
                                );
                                if zdata_0.is_null() {
                                    uxabort(71 as libc::c_int);
                                }
                                ubuffree(zdata_0);
                                zbase_1 = zsysdep_base_name(zfile);
                                if zbase_1.is_null() {
                                    uxabort(71 as libc::c_int);
                                }
                                zxqt = zsysdep_xqt_file_name();
                                if zxqt.is_null() {
                                    uxabort(71 as libc::c_int);
                                }
                                e = esysdep_fopen(
                                    zxqt,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    1 as libc::c_int,
                                );
                                if e.is_null() {
                                    uxabort(71 as libc::c_int);
                                }
                                uxrecord_file(zxqt);
                                fprintf(
                                    e,
                                    b"U %s %s\n\0" as *const u8 as *const libc::c_char,
                                    zsysdep_login_name(),
                                    zlocalname,
                                );
                                fprintf(
                                    e,
                                    b"F %s %s\n\0" as *const u8 as *const libc::c_char,
                                    abtname_0.as_mut_ptr(),
                                    zbase_1,
                                );
                                fprintf(
                                    e,
                                    b"C uucp -C -W -d -g %c %s %s!\0" as *const u8
                                        as *const libc::c_char,
                                    bXgrade as libc::c_int,
                                    zbase_1,
                                    sXxqtsys.uuconf_zname,
                                );
                                if !zforward.is_null() {
                                    fprintf(
                                        e,
                                        b"%s!\0" as *const u8 as *const libc::c_char,
                                        zforward,
                                    );
                                }
                                fprintf(
                                    e,
                                    b"%s\n\0" as *const u8 as *const libc::c_char,
                                    abxtname.as_mut_ptr(),
                                );
                                if fsysdep_sync(e, zxqt) == 0 {
                                    ulog(
                                        LOG_FATAL,
                                        b"fsync failed\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                if fclose(e) != 0 as libc::c_int {
                                    ulog(
                                        LOG_FATAL,
                                        b"fclose: %s\0" as *const u8 as *const libc::c_char,
                                        strerror(*__errno_location()),
                                    );
                                }
                                if finput != 0 {
                                    uxadd_xqt_line(
                                        'F' as i32,
                                        abxtname.as_mut_ptr(),
                                        0 as *mut libc::c_void as *mut libc::c_char,
                                    );
                                    uxadd_xqt_line(
                                        'I' as i32,
                                        abxtname.as_mut_ptr(),
                                        0 as *mut libc::c_void as *mut libc::c_char,
                                    );
                                    ubuffree(zbase_1);
                                } else {
                                    uxadd_xqt_line('F' as i32, abxtname.as_mut_ptr(), zbase_1);
                                    let ref mut fresh13 = *pzargs.offset(i as isize);
                                    *fresh13 = zbase_1;
                                }
                            }
                        }
                        uuconf_free_block(sfromsys.uuconf_palloc);
                    }
                }
            }
        }
        i += 1;
        i;
    }
    if fread_stdin != 0 {
        let mut zdata_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut abtname_1: [libc::c_char; 15] = [0; 15];
        let mut abdname_0: [libc::c_char; 15] = [0; 15];
        let mut e_0: *mut FILE = 0 as *mut FILE;
        zdata_1 = zsysdep_data_file_name(
            &mut sXxqtsys,
            zXxqtloc,
            bXgrade as libc::c_int,
            0 as libc::c_int,
            abtname_1.as_mut_ptr(),
            abdname_0.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        if zdata_1.is_null() {
            uxabort(71 as libc::c_int);
        }
        e_0 = esysdep_fopen(
            zdata_1,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if e_0.is_null() {
            uxabort(71 as libc::c_int);
        }
        eXclose = e_0;
        uxrecord_file(zdata_1);
        uxcopy_stdin(e_0);
        if fsysdep_sync(e_0, zdata_1) == 0 {
            ulog(LOG_FATAL, b"fsync failed\0" as *const u8 as *const libc::c_char);
        }
        eXclose = 0 as *mut FILE;
        if fclose(e_0) != 0 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"fclose: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if fXxqtlocal != 0 {
            uxadd_xqt_line(
                'I' as i32,
                abtname_1.as_mut_ptr(),
                0 as *mut libc::c_void as *const libc::c_char,
            );
        } else {
            zinput_from = zbufcpy(abtname_1.as_mut_ptr());
            zinput_to = zbufcpy(abdname_0.as_mut_ptr());
            zinput_temp = zinput_from;
            finputcopied = 1 as libc::c_int;
        }
    }
    if fretstdin != 0 {
        uxadd_xqt_line(
            'B' as i32,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_char,
        );
    }
    if !zstatus_file.is_null() {
        uxadd_xqt_line(
            'M' as i32,
            zstatus_file,
            0 as *mut libc::c_void as *const libc::c_char,
        );
    }
    fneedshell = 0 as libc::c_int;
    if *zcmd
        .offset(
            strcspn(zcmd, b"\"'`*?[;&()|<>\\$\0" as *const u8 as *const libc::c_char)
                as isize,
        ) as libc::c_int != '\0' as i32
    {
        fneedshell = 1 as libc::c_int;
    }
    clen = (strlen(zcmd)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < cargs {
        if !(*pzargs.offset(i as isize)).is_null() {
            clen = (clen as libc::c_ulong)
                .wrapping_add(
                    (strlen(*pzargs.offset(i as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            if *(*pzargs.offset(i as isize))
                .offset(
                    strcspn(
                        *pzargs.offset(i as isize),
                        b"\"'`*?[;&()|<>\\$\0" as *const u8 as *const libc::c_char,
                    ) as isize,
                ) as libc::c_int != '\0' as i32
            {
                fneedshell = 1 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    zfullcmd = zbufalc(clen);
    strcpy(zfullcmd, zcmd);
    i = 0 as libc::c_int;
    while i < cargs {
        if !(*pzargs.offset(i as isize)).is_null() {
            strcat(zfullcmd, b" \0" as *const u8 as *const libc::c_char);
            strcat(zfullcmd, *pzargs.offset(i as isize));
        }
        i += 1;
        i;
    }
    fpoll = 0 as libc::c_int;
    if eXxqt_file.is_null() && !zinput_from.is_null() && zforward.is_null() {
        let mut s_0: scmd = scmd {
            bcmd: 0,
            bgrade: 0,
            pseq: 0 as *mut libc::c_void,
            zfrom: 0 as *const libc::c_char,
            zto: 0 as *const libc::c_char,
            zuser: 0 as *const libc::c_char,
            zoptions: 0 as *const libc::c_char,
            ztemp: 0 as *const libc::c_char,
            imode: 0,
            znotify: 0 as *const libc::c_char,
            cbytes: 0,
            zcmd: 0 as *const libc::c_char,
            ipos: 0,
        };
        let mut zoptions: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0.bcmd = 'E' as i32 as libc::c_char;
        s_0.bgrade = bXgrade;
        s_0.pseq = 0 as *mut libc::c_void;
        s_0.zuser = zuser;
        s_0.zfrom = zinput_from;
        s_0.zto = zinput_to;
        s_0.zoptions = aboptions.as_mut_ptr();
        zoptions = aboptions.as_mut_ptr();
        let fresh14 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh14 = (if finputcopied != 0 { 'C' as i32 } else { 'c' as i32 })
            as libc::c_char;
        if fno_ack != 0 {
            let fresh15 = zoptions;
            zoptions = zoptions.offset(1);
            *fresh15 = 'N' as i32 as libc::c_char;
        }
        if ferror_ack != 0 {
            let fresh16 = zoptions;
            zoptions = zoptions.offset(1);
            *fresh16 = 'Z' as i32 as libc::c_char;
        }
        if !zrequestor.is_null() {
            let fresh17 = zoptions;
            zoptions = zoptions.offset(1);
            *fresh17 = 'R' as i32 as libc::c_char;
        }
        if fneedshell != 0 {
            let fresh18 = zoptions;
            zoptions = zoptions.offset(1);
            *fresh18 = 'e' as i32 as libc::c_char;
        }
        *zoptions = '\0' as i32 as libc::c_char;
        s_0.ztemp = zinput_temp;
        s_0.imode = 0o666 as libc::c_int as libc::c_uint;
        if zrequestor.is_null() {
            zrequestor = b"\"\"\0" as *const u8 as *const libc::c_char;
        }
        s_0.znotify = zrequestor;
        s_0.cbytes = -(1 as libc::c_int) as libc::c_long;
        s_0.zcmd = zfullcmd;
        s_0.ipos = 0 as libc::c_int as libc::c_long;
        cXcmds += 1;
        cXcmds;
        pasXcmds = xrealloc(
            pasXcmds as pointer,
            (cXcmds as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<scmd>() as libc::c_ulong),
        ) as *mut scmd;
        *pasXcmds.offset((cXcmds - 1 as libc::c_int) as isize) = s_0;
        uxadd_name(zinput_from);
    } else if *zfullcmd as libc::c_int == '\0' as i32 && eXxqt_file.is_null()
        && zinput_from.is_null() && cXcmds == 0 as libc::c_int
    {
        fpoll = 1 as libc::c_int;
    } else {
        uxadd_xqt_line('U' as i32, zuser, zXxqtloc);
        if !zinput_from.is_null() {
            uxadd_xqt_line(
                'F' as i32,
                zinput_to,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            uxadd_xqt_line(
                'I' as i32,
                zinput_to,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            uxadd_send_file(
                zinput_from,
                zinput_to,
                if finputcopied != 0 {
                    b"C\0" as *const u8 as *const libc::c_char
                } else {
                    b"c\0" as *const u8 as *const libc::c_char
                },
                zinput_temp,
                zforward,
            );
        }
        if fno_ack != 0 {
            uxadd_xqt_line(
                'N' as i32,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        if ferror_ack != 0 {
            uxadd_xqt_line(
                'Z' as i32,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        if !zrequestor.is_null() {
            uxadd_xqt_line(
                'R' as i32,
                zrequestor,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        if fneedshell != 0 {
            uxadd_xqt_line(
                'e' as i32,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        uxadd_xqt_line(
            'C' as i32,
            zfullcmd,
            0 as *mut libc::c_void as *const libc::c_char,
        );
        if fsysdep_sync(
            eXxqt_file,
            b"execution file\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ulog(LOG_FATAL, b"fsync failed\0" as *const u8 as *const libc::c_char);
        }
        if fclose(eXxqt_file) != 0 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"fclose: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        eXxqt_file = 0 as *mut FILE;
        if fXxqtlocal == 0 {
            uxadd_send_file(
                abXxqt_tname.as_mut_ptr(),
                abXxqt_xname.as_mut_ptr(),
                b"C\0" as *const u8 as *const libc::c_char,
                abXxqt_tname.as_mut_ptr(),
                zforward,
            );
        }
    }
    if afSignal[0 as libc::c_int as usize] != 0
        || afSignal[1 as libc::c_int as usize] != 0
        || afSignal[2 as libc::c_int as usize] != 0
        || afSignal[3 as libc::c_int as usize] != 0
        || afSignal[4 as libc::c_int as usize] != 0
    {
        uxabort(71 as libc::c_int);
    }
    if cXcmds > 0 as libc::c_int || fpoll != 0 {
        let mut zjobid_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ftemp_0: boolean = 0;
        if fpoll == 0 && sXxqtsys.uuconf_fcall_transfer == 0
            && sXxqtsys.uuconf_fcalled_transfer == 0
        {
            ulog(
                LOG_FATAL,
                b"Not permitted to transfer files to or from %s\0" as *const u8
                    as *const libc::c_char,
                sXxqtsys.uuconf_zname,
            );
        }
        zjobid_0 = zsysdep_spool_commands(
            &mut sXxqtsys,
            bXgrade as libc::c_int,
            cXcmds,
            pasXcmds,
            &mut ftemp_0,
        );
        if zjobid_0.is_null() {
            ulog_close();
            exit(if ftemp_0 != 0 { 75 as libc::c_int } else { 65 as libc::c_int });
        }
        if fjobid != 0 {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, zjobid_0);
        }
        ubuffree(zjobid_0);
        if fcall_any != 0 {
            ubuffree(zcall_system);
            zcall_system = 0 as *mut libc::c_char;
        } else {
            fcall_any = 1 as libc::c_int;
            zcall_system = zbufcpy(sXxqtsys.uuconf_zname);
        }
    }
    if fpoll == 0 {
        ulog_to_file(puuconf, 1 as libc::c_int);
        ulog_system(sXxqtsys.uuconf_zname);
        ulog_user(zuser);
        if zXnames.is_null() {
            ulog(
                LOG_NORMAL,
                b"Queuing %s\0" as *const u8 as *const libc::c_char,
                zfullcmd,
            );
        } else {
            ulog(
                LOG_NORMAL,
                b"Queuing %s (%s)\0" as *const u8 as *const libc::c_char,
                zfullcmd,
                zXnames,
            );
        }
        ulog_close();
    }
    if fuucico == 0 || zcall_system.is_null() && fcall_any == 0 {
        if fXxqtlocal != 0 && fuucico != 0 {
            let mut zconfigarg: *mut libc::c_char = 0 as *mut libc::c_char;
            if zconfig.is_null() {
                zconfigarg = 0 as *mut libc::c_char;
            } else {
                zconfigarg = zbufalc(
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_add(strlen(zconfig)),
                );
                sprintf(
                    zconfigarg,
                    b"-I%s\0" as *const u8 as *const libc::c_char,
                    zconfig,
                );
            }
            fsysdep_run(
                0 as libc::c_int,
                b"uuxqt\0" as *const u8 as *const libc::c_char,
                zconfigarg,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
    } else {
        let mut zcicoarg: *const libc::c_char = 0 as *const libc::c_char;
        let mut zconfigarg_0: *mut libc::c_char = 0 as *mut libc::c_char;
        if zcall_system.is_null() {
            zcicoarg = b"-r1\0" as *const u8 as *const libc::c_char;
        } else {
            let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
            z = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_add(strlen(zcall_system)),
            );
            sprintf(z, b"-Cs%s\0" as *const u8 as *const libc::c_char, zcall_system);
            zcicoarg = z;
        }
        if zconfig.is_null() {
            zconfigarg_0 = 0 as *mut libc::c_char;
        } else {
            zconfigarg_0 = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_add(strlen(zconfig)),
            );
            sprintf(
                zconfigarg_0,
                b"-I%s\0" as *const u8 as *const libc::c_char,
                zconfig,
            );
        }
        fsysdep_run(
            0 as libc::c_int,
            b"uucico\0" as *const u8 as *const libc::c_char,
            zcicoarg,
            zconfigarg_0,
        );
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn uxhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Usage: %s [options] [-] command\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    printf(
        b" -,-p,--stdin: Read standard input for standard input of command\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -c,--nocopy: Do not copy local files to spool directory (default)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -C,--copy: Copy local files to spool directory\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -l,--link: link local files to spool directory\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -g,--grade grade: Set job grade (must be alphabetic)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -n,--notification=no: Do not report completion status\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -z,--notification=error: Report completion status only on error\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -r,--nouucico: Do not start uucico daemon\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -a,--requestor address: Address to mail status report to\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -b,--return-stdin: Return standard input with status report\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -s,--status file: Report completion status to file\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b" -j,--jobid: Report job id\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -x,--debug debug: Set debugging level\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -I,--config file: Set configuration file to use\n\0" as *const u8
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
unsafe extern "C" fn uxusage() {
    fprintf(
        stderr,
        b"Usage: %s [options] [-] command\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    fprintf(
        stderr,
        b"Use %s --help for help\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    exit(64 as libc::c_int);
}
unsafe extern "C" fn uxadd_xqt_line(
    mut bchar: libc::c_int,
    mut z1: *const libc::c_char,
    mut z2: *const libc::c_char,
) {
    let mut z1q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z2q: *mut libc::c_char = 0 as *mut libc::c_char;
    if eXxqt_file.is_null() {
        let mut zxqt_name: *const libc::c_char = 0 as *const libc::c_char;
        if fXxqtlocal != 0 {
            zxqt_name = zsysdep_xqt_file_name();
        } else {
            zxqt_name = zsysdep_data_file_name(
                &mut sXxqtsys,
                zXxqtloc,
                bXgrade as libc::c_int,
                1 as libc::c_int,
                abXxqt_tname.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_char,
                abXxqt_xname.as_mut_ptr(),
            );
        }
        if zxqt_name.is_null() {
            uxabort(71 as libc::c_int);
        }
        uxrecord_file(zxqt_name);
        eXxqt_file = esysdep_fopen(
            zxqt_name,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if eXxqt_file.is_null() {
            uxabort(71 as libc::c_int);
        }
    }
    z1q = 0 as *mut libc::c_char;
    z2q = 0 as *mut libc::c_char;
    if fXquote != 0 {
        if fXquote_output == 0 {
            fprintf(eXxqt_file, b"Q\n\0" as *const u8 as *const libc::c_char);
            fXquote_output = 1 as libc::c_int;
        }
        if !z1.is_null() {
            z1q = zquote_cmd_string(z1, 0 as libc::c_int);
            z1 = z1q;
        }
        if !z2.is_null() {
            z2q = zquote_cmd_string(z2, 0 as libc::c_int);
            z2 = z2q;
        }
    }
    if z1.is_null() {
        fprintf(eXxqt_file, b"%c\n\0" as *const u8 as *const libc::c_char, bchar);
    } else if z2.is_null() {
        fprintf(eXxqt_file, b"%c %s\n\0" as *const u8 as *const libc::c_char, bchar, z1);
    } else {
        fprintf(
            eXxqt_file,
            b"%c %s %s\n\0" as *const u8 as *const libc::c_char,
            bchar,
            z1,
            z2,
        );
    }
    if !z1q.is_null() {
        ubuffree(z1q);
    }
    if !z2q.is_null() {
        ubuffree(z2q);
    }
}
unsafe extern "C" fn uxadd_send_file(
    mut zfrom: *const libc::c_char,
    mut zto: *const libc::c_char,
    mut zoptions: *const libc::c_char,
    mut ztemp: *const libc::c_char,
    mut zforward: *const libc::c_char,
) {
    let mut s: scmd = scmd {
        bcmd: 0,
        bgrade: 0,
        pseq: 0 as *mut libc::c_void,
        zfrom: 0 as *const libc::c_char,
        zto: 0 as *const libc::c_char,
        zuser: 0 as *const libc::c_char,
        zoptions: 0 as *const libc::c_char,
        ztemp: 0 as *const libc::c_char,
        imode: 0,
        znotify: 0 as *const libc::c_char,
        cbytes: 0,
        zcmd: 0 as *const libc::c_char,
        ipos: 0,
    };
    if !zforward.is_null() {
        let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zxqt: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut abtname: [libc::c_char; 15] = [0; 15];
        let mut abdname: [libc::c_char; 15] = [0; 15];
        let mut abxname: [libc::c_char; 15] = [0; 15];
        let mut e: *mut FILE = 0 as *mut FILE;
        zbase = zsysdep_base_name(zfrom);
        if zbase.is_null() {
            uxabort(71 as libc::c_int);
        }
        zxqt = zsysdep_data_file_name(
            &mut sXxqtsys,
            zXxqtloc,
            bXgrade as libc::c_int,
            1 as libc::c_int,
            abtname.as_mut_ptr(),
            abdname.as_mut_ptr(),
            abxname.as_mut_ptr(),
        );
        if zxqt.is_null() {
            uxabort(71 as libc::c_int);
        }
        e = esysdep_fopen(zxqt, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
        if e.is_null() {
            uxabort(71 as libc::c_int);
        }
        uxrecord_file(zxqt);
        fprintf(
            e,
            b"U %s %s\n\0" as *const u8 as *const libc::c_char,
            zsysdep_login_name(),
            zXxqtloc,
        );
        fprintf(
            e,
            b"F %s %s\n\0" as *const u8 as *const libc::c_char,
            abdname.as_mut_ptr(),
            zbase,
        );
        fprintf(
            e,
            b"C uucp -C -W -d -g %c %s %s!%s\n\0" as *const u8 as *const libc::c_char,
            bXgrade as libc::c_int,
            zbase,
            zforward,
            zto,
        );
        ubuffree(zbase);
        if fsysdep_sync(e, zxqt) == 0 {
            ulog(LOG_FATAL, b"fsync failed\0" as *const u8 as *const libc::c_char);
        }
        if fclose(e) != 0 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"fclose: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        s.bcmd = 'S' as i32 as libc::c_char;
        s.bgrade = bXgrade;
        s.pseq = 0 as *mut libc::c_void;
        s.zfrom = zbufcpy(abtname.as_mut_ptr());
        s.zto = zbufcpy(abxname.as_mut_ptr());
        s.zuser = zsysdep_login_name();
        s.zoptions = b"C\0" as *const u8 as *const libc::c_char;
        s.ztemp = s.zfrom;
        s.imode = 0o666 as libc::c_int as libc::c_uint;
        s.znotify = 0 as *const libc::c_char;
        s.cbytes = -(1 as libc::c_int) as libc::c_long;
        s.zcmd = 0 as *const libc::c_char;
        s.ipos = 0 as libc::c_int as libc::c_long;
        cXcmds += 1;
        cXcmds;
        pasXcmds = xrealloc(
            pasXcmds as pointer,
            (cXcmds as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<scmd>() as libc::c_ulong),
        ) as *mut scmd;
        *pasXcmds.offset((cXcmds - 1 as libc::c_int) as isize) = s;
        uxadd_name(abtname.as_mut_ptr());
        zto = abdname.as_mut_ptr();
    }
    s.bcmd = 'S' as i32 as libc::c_char;
    s.bgrade = bXgrade;
    s.pseq = 0 as *mut libc::c_void;
    s.zfrom = zbufcpy(zfrom);
    s.zto = zbufcpy(zto);
    s.zuser = zsysdep_login_name();
    s.zoptions = zbufcpy(zoptions);
    s.ztemp = zbufcpy(ztemp);
    s.imode = 0o666 as libc::c_int as libc::c_uint;
    s.znotify = b"\0" as *const u8 as *const libc::c_char;
    s.cbytes = -(1 as libc::c_int) as libc::c_long;
    s.zcmd = 0 as *const libc::c_char;
    s.ipos = 0 as libc::c_int as libc::c_long;
    cXcmds += 1;
    cXcmds;
    pasXcmds = xrealloc(
        pasXcmds as pointer,
        (cXcmds as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<scmd>() as libc::c_ulong),
    ) as *mut scmd;
    *pasXcmds.offset((cXcmds - 1 as libc::c_int) as isize) = s;
    uxadd_name(zfrom);
}
unsafe extern "C" fn uxcopy_stdin(mut e: *mut FILE) {
    let mut cread: size_t = 0;
    let mut ab: [libc::c_char; 1024] = [0; 1024];
    loop {
        let mut cwrite: size_t = 0;
        cread = 0 as libc::c_int as size_t;
        while cread < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
            let mut b: libc::c_int = 0;
            if afSignal[0 as libc::c_int as usize] != 0
                || afSignal[1 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0
            {
                uxabort(71 as libc::c_int);
            }
            b = getchar();
            if b == -(1 as libc::c_int) {
                break;
            }
            ab[cread as usize] = b as libc::c_char;
            cread = cread.wrapping_add(1);
            cread;
        }
        if afSignal[0 as libc::c_int as usize] != 0
            || afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0
        {
            uxabort(71 as libc::c_int);
        }
        if cread > 0 as libc::c_int as libc::c_ulong {
            cwrite = fwrite(
                ab.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                cread,
                e,
            );
            if cwrite != cread {
                ulog(
                    LOG_FATAL,
                    b"fwrite: Wrote %d when attempted %d\0" as *const u8
                        as *const libc::c_char,
                    cwrite as libc::c_int,
                    cread as libc::c_int,
                );
            }
        }
        if !(cread == ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong) {
            break;
        }
    };
}
static mut cXfiles: libc::c_int = 0;
static mut pXaz: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
unsafe extern "C" fn uxrecord_file(mut zfile: *const libc::c_char) {
    pXaz = xrealloc(
        pXaz as pointer,
        ((cXfiles + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    let ref mut fresh19 = *pXaz.offset(cXfiles as isize);
    *fresh19 = zfile;
    cXfiles += 1;
    cXfiles;
}
unsafe extern "C" fn uxfatal() {
    uxabort(69 as libc::c_int);
}
unsafe extern "C" fn uxabort(mut istat: libc::c_int) {
    let mut i: libc::c_int = 0;
    if !eXxqt_file.is_null() {
        fclose(eXxqt_file);
    }
    if !eXclose.is_null() {
        fclose(eXclose);
    }
    i = 0 as libc::c_int;
    while i < cXfiles {
        remove(*pXaz.offset(i as isize));
        i += 1;
        i;
    }
    ulog_close();
    exit(istat);
}
unsafe extern "C" fn uxadd_name(mut z: *const libc::c_char) {
    if zXnames.is_null() {
        zXnames = zbufcpy(z);
    } else {
        let mut cold: size_t = 0;
        let mut cadd: size_t = 0;
        let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
        cold = strlen(zXnames);
        cadd = strlen(z);
        znew = zbufalc(
            cold.wrapping_add(2 as libc::c_int as libc::c_ulong).wrapping_add(cadd),
        );
        memcpy(znew as *mut libc::c_void, zXnames as *const libc::c_void, cold);
        *znew.offset(cold as isize) = ' ' as i32 as libc::c_char;
        memcpy(
            znew.offset(cold as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            z as *const libc::c_void,
            cadd.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        ubuffree(zXnames);
        zXnames = znew;
    };
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

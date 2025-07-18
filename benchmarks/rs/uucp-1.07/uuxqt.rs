use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    static mut gnu_optarg: *mut libc::c_char;
    static mut gnu_optind: libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut iDebug: libc::c_int;
    fn funknown_system(
        puuconf: pointer,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> boolean;
    fn fspool_file(zfile: *const libc::c_char) -> boolean;
    fn fin_directory_list(
        zfile: *const libc::c_char,
        pzdirs: *mut *mut libc::c_char,
        zpubdir: *const libc::c_char,
        fcheck: boolean,
        freadable: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_fatal_fn(pfn: Option::<unsafe extern "C" fn() -> ()>);
    fn ulog_to_file(puuconf: pointer, ffile: boolean);
    fn ulog_system(zsystem: *const libc::c_char);
    fn ulog_user(zuser: *const libc::c_char);
    fn ulog_close();
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn cescape(zbuf: *mut libc::c_char) -> size_t;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    fn xfree(_: pointer);
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
    fn uuconf_maxuuxqts(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pcmaxuuxqt: *mut libc::c_int,
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
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn zsysdep_localname() -> *const libc::c_char;
    fn usysdep_signal(isig: libc::c_int);
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_file_exists(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_mail(
        zto: *const libc::c_char,
        zsubject: *const libc::c_char,
        cstrs: libc::c_int,
        paz: *mut *const libc::c_char,
    ) -> boolean;
    fn zsysdep_save_corrupt_file(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn zsysdep_save_failed_file(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn zsysdep_spool_file_name(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        pseq: pointer,
    ) -> *mut libc::c_char;
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
    fn fsysdep_get_xqt_init(zsystem: *const libc::c_char) -> boolean;
    fn zsysdep_get_xqt(
        zsystem: *const libc::c_char,
        pzsystem: *mut *mut libc::c_char,
        pferr: *mut boolean,
    ) -> *mut libc::c_char;
    fn usysdep_get_xqt_free(zsystem: *const libc::c_char);
    fn zsysdep_find_command(
        zcmd: *const libc::c_char,
        pzcmds: *mut *mut libc::c_char,
        pzpath: *mut *mut libc::c_char,
        pferr: *mut boolean,
    ) -> *mut libc::c_char;
    fn zsysdep_xqt_local_file(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_execute(
        qsys: *const uuconf_system,
        zuser: *const libc::c_char,
        pazargs: *mut *const libc::c_char,
        zfullcmd: *const libc::c_char,
        zinput: *const libc::c_char,
        zoutput: *const libc::c_char,
        fshell: boolean,
        ilock: libc::c_int,
        pzerror: *mut *mut libc::c_char,
        pftemp: *mut boolean,
    ) -> boolean;
    fn ixsysdep_lock_uuxqt(
        zcmd: *const libc::c_char,
        cmaxuuxqts: libc::c_int,
    ) -> libc::c_int;
    fn fsysdep_unlock_uuxqt(
        iseq: libc::c_int,
        zcmd: *const libc::c_char,
        cmaxuuxqts: libc::c_int,
    ) -> boolean;
    fn fsysdep_uuxqt_locked(zcmd: *const libc::c_char) -> boolean;
    fn fsysdep_lock_uuxqt_file(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_unlock_uuxqt_file(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_lock_uuxqt_dir(ilock: libc::c_int) -> boolean;
    fn fsysdep_unlock_uuxqt_dir(ilock: libc::c_int) -> boolean;
    fn fsysdep_copy_uuxqt_files(
        cfiles: libc::c_int,
        pzfrom: *const *const libc::c_char,
        pzto: *const *const libc::c_char,
        ilock: libc::c_int,
        pzinput: *mut *mut libc::c_char,
    ) -> boolean;
    fn zsysdep_base_name(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn csysdep_bytes_free(zfile: *const libc::c_char) -> libc::c_long;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub static mut uuxqt_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: uuxqt.c,v 1.92 2002/03/05 19:10:42 ian Rel $\0")
};
static mut iQlock_seq: libc::c_int = -(1 as libc::c_int);
static mut zQunlock_cmd: *const libc::c_char = 0 as *const libc::c_char;
static mut zQunlock_file: *const libc::c_char = 0 as *const libc::c_char;
static mut fQunlock_directory: boolean = 0;
pub static mut cQmaxuuxqts: libc::c_int = 0;
static mut zQoutput: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zQmail: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut asQlongopts: [option; 7] = [
    {
        let mut init = option {
            name: b"command\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"system\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
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
    let mut zcmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut zdosys: *const libc::c_char = 0 as *const libc::c_char;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut zlocalname: *const libc::c_char = 0 as *const libc::c_char;
    let mut fany: boolean = 0;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zgetsys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ferr: boolean = 0;
    let mut fsys: boolean = 0;
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
    zProgram = *argv.offset(0 as libc::c_int as isize);
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"c:I:s:vx:\0" as *const u8 as *const libc::c_char,
            asQlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            99 => {
                zcmd = gnu_optarg;
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            115 => {
                zdosys = gnu_optarg;
            }
            120 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            118 => {
                printf(
                    b"uuxqt (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
                uqhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                uqusage();
            }
        }
    }
    if gnu_optind != argc {
        uqusage();
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
    iuuconf = uuconf_maxuuxqts(puuconf, &mut cQmaxuuxqts);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    usysdep_signal(2 as libc::c_int);
    usysdep_signal(1 as libc::c_int);
    usysdep_signal(3 as libc::c_int);
    usysdep_signal(15 as libc::c_int);
    usysdep_signal(13 as libc::c_int);
    usysdep_initialize(puuconf, 0o4 as libc::c_int);
    ulog_to_file(puuconf, 1 as libc::c_int);
    ulog_fatal_fn(Some(uqabort as unsafe extern "C" fn() -> ()));
    iuuconf = uuconf_localname(puuconf, &mut zlocalname);
    if iuuconf == 1 as libc::c_int {
        zlocalname = zsysdep_localname();
        if zlocalname.is_null() {
            exit(1 as libc::c_int);
        }
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    fsys = 0 as libc::c_int;
    if !zdosys.is_null() {
        iuuconf = uuconf_system_info(puuconf, zdosys, &mut ssys);
        if iuuconf != 0 as libc::c_int {
            if iuuconf != 1 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            if strcmp(zdosys, zlocalname) == 0 as libc::c_int {
                iuuconf = uuconf_system_local(puuconf, &mut ssys);
                if iuuconf != 0 as libc::c_int {
                    ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
                }
                ssys.uuconf_zname = zlocalname as *mut libc::c_char;
            } else if funknown_system(puuconf, zdosys, &mut ssys) == 0 {
                ulog(
                    LOG_FATAL,
                    b"%s: system not found\0" as *const u8 as *const libc::c_char,
                    zdosys,
                );
            }
        }
        zdosys = zbufcpy(ssys.uuconf_zname);
        fsys = 1 as libc::c_int;
    }
    iQlock_seq = ixsysdep_lock_uuxqt(zcmd, cQmaxuuxqts);
    if iQlock_seq < 0 as libc::c_int {
        ulog_close();
        usysdep_exit(1 as libc::c_int);
    }
    zQunlock_cmd = zcmd;
    loop {
        fany = 0 as libc::c_int;
        if fsysdep_get_xqt_init(zdosys) == 0 {
            ulog_close();
            usysdep_exit(0 as libc::c_int);
        }
        loop {
            z = zsysdep_get_xqt(zdosys, &mut zgetsys, &mut ferr);
            if z.is_null() {
                break;
            }
            let mut zloc: *const libc::c_char = 0 as *const libc::c_char;
            let mut fprocessed: boolean = 0;
            let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
            if fsys == 0 || strcmp(ssys.uuconf_zname, zgetsys) != 0 as libc::c_int {
                if fsys != 0 {
                    uuconf_free_block(ssys.uuconf_palloc);
                }
                iuuconf = uuconf_system_info(puuconf, zgetsys, &mut ssys);
                if iuuconf != 0 as libc::c_int {
                    if iuuconf != 1 as libc::c_int {
                        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                        ubuffree(z);
                        ubuffree(zgetsys);
                        continue;
                    } else if strcmp(zgetsys, zlocalname) == 0 as libc::c_int {
                        iuuconf = uuconf_system_local(puuconf, &mut ssys);
                        if iuuconf != 0 as libc::c_int {
                            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                            ubuffree(z);
                            ubuffree(zgetsys);
                            continue;
                        } else {
                            ssys.uuconf_zname = zlocalname as *mut libc::c_char;
                        }
                    } else if funknown_system(puuconf, zgetsys, &mut ssys) == 0 {
                        ulog(
                            LOG_ERROR,
                            b"%s: Execute file for unknown system %s\0" as *const u8
                                as *const libc::c_char,
                            z,
                            zgetsys,
                        );
                        remove(z);
                        ubuffree(z);
                        ubuffree(zgetsys);
                        continue;
                    }
                }
                fsys = 1 as libc::c_int;
            }
            if afSignal[0 as libc::c_int as usize] != 0
                || afSignal[1 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0
            {
                ubuffree(z);
                ubuffree(zgetsys);
                break;
            } else if !zdosys.is_null()
                && strcmp(zdosys, ssys.uuconf_zname) != 0 as libc::c_int
            {
                ubuffree(z);
                ubuffree(zgetsys);
            } else {
                zloc = ssys.uuconf_zlocalname;
                if zloc.is_null() {
                    zloc = zlocalname;
                }
                ulog_system(ssys.uuconf_zname);
                zbase = zsysdep_base_name(z);
                uqdo_xqt_file(puuconf, z, zbase, &mut ssys, zloc, zcmd, &mut fprocessed);
                ubuffree(zbase);
                ulog_system(0 as *mut libc::c_void as *const libc::c_char);
                ulog_user(0 as *mut libc::c_void as *const libc::c_char);
                if fprocessed != 0 {
                    fany = 1 as libc::c_int;
                }
                ubuffree(z);
                ubuffree(zgetsys);
            }
        }
        usysdep_get_xqt_free(zdosys);
        if !(fany != 0
            && !(afSignal[0 as libc::c_int as usize] != 0
                || afSignal[1 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0))
        {
            break;
        }
    }
    fsysdep_unlock_uuxqt(iQlock_seq, zcmd, cQmaxuuxqts);
    iQlock_seq = -(1 as libc::c_int);
    ulog_close();
    if afSignal[0 as libc::c_int as usize] != 0
        || afSignal[1 as libc::c_int as usize] != 0
        || afSignal[2 as libc::c_int as usize] != 0
        || afSignal[3 as libc::c_int as usize] != 0
        || afSignal[4 as libc::c_int as usize] != 0
    {
        ferr = 1 as libc::c_int;
    }
    usysdep_exit((ferr == 0) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn uqhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Usage: %s [-c,--command cmd] [-s,--system system]\n\0" as *const u8
            as *const libc::c_char,
        zProgram,
    );
    printf(
        b" -c,--command cmd: Set type of command to execute\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -s,--system system: Execute commands only for named system\n\0" as *const u8
            as *const libc::c_char,
    );
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
unsafe extern "C" fn uqusage() {
    fprintf(
        stderr,
        b"Usage: %s [-c,--command cmd] [-s,--system system]\n\0" as *const u8
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
unsafe extern "C" fn uqabort() {
    ulog_system(0 as *mut libc::c_void as *const libc::c_char);
    ulog_user(0 as *mut libc::c_void as *const libc::c_char);
    if fQunlock_directory != 0 {
        fsysdep_unlock_uuxqt_dir(iQlock_seq);
    }
    if !zQunlock_file.is_null() {
        fsysdep_unlock_uuxqt_file(zQunlock_file);
    }
    if iQlock_seq >= 0 as libc::c_int {
        fsysdep_unlock_uuxqt(iQlock_seq, zQunlock_cmd, cQmaxuuxqts);
    }
    ulog_close();
    usysdep_exit(0 as libc::c_int);
}
static mut azQargs: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut zQcmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zQinput: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zQoutfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zQoutsys: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut cQfiles: libc::c_int = 0;
static mut azQfiles: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut azQfiles_to: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut zQrequestor: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut zQuser: *const libc::c_char = 0 as *const libc::c_char;
static mut zQsystem: *const libc::c_char = 0 as *const libc::c_char;
static mut fQno_ack: boolean = 0;
static mut fQsuccess_ack: boolean = 0;
static mut fQsend_input: boolean = 0;
static mut fQuse_exec: boolean = 0;
static mut zQstatus_file: *const libc::c_char = 0 as *const libc::c_char;
static mut fQquoted: boolean = 0;
static mut asQcmds: [uuconf_cmdtab; 13] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"C\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    iqcmd
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"I\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zQinput as *const *mut libc::c_char
                    as *mut *mut libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"O\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    iqout
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"F\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    iqfile
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"R\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    iqrequestor
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"U\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    iquser
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"N\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &fQno_ack as *const boolean as *mut boolean as pointer,
                uuconf_pifn: Some(
                    iqset
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"n\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &fQsuccess_ack as *const boolean as *mut boolean as pointer,
                uuconf_pifn: Some(
                    iqset
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"B\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &fQsend_input as *const boolean as *mut boolean as pointer,
                uuconf_pifn: Some(
                    iqset
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"E\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &fQuse_exec as *const boolean as *mut boolean as pointer,
                uuconf_pifn: Some(
                    iqset
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"M\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zQstatus_file as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"Q\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &fQquoted as *const boolean as *mut boolean as pointer,
                uuconf_pifn: Some(
                    iqset
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
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: 0 as *const libc::c_char,
                uuconf_itype: 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: None,
            };
            init
        },
    ]
};
unsafe extern "C" fn iqcmd(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut clen: size_t = 0;
    if argc <= 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    azQargs = xmalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    clen = 0 as libc::c_int as size_t;
    i = 1 as libc::c_int;
    while i < argc {
        let ref mut fresh0 = *azQargs.offset((i - 1 as libc::c_int) as isize);
        *fresh0 = zbufcpy(*argv.offset(i as isize));
        clen = (clen as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        i += 1;
        i;
    }
    let ref mut fresh1 = *azQargs.offset((i - 1 as libc::c_int) as isize);
    *fresh1 = 0 as *mut libc::c_char;
    zQcmd = xmalloc(clen) as *mut libc::c_char;
    *zQcmd.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    i = 1 as libc::c_int;
    while i < argc - 1 as libc::c_int {
        strcat(zQcmd, *argv.offset(i as isize));
        strcat(zQcmd, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    strcat(zQcmd, *argv.offset(i as isize));
    return 0 as libc::c_int;
}
unsafe extern "C" fn iqout(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    if argc > 1 as libc::c_int {
        zQoutfile = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    }
    if argc > 2 as libc::c_int {
        zQoutsys = zbufcpy(*argv.offset(2 as libc::c_int as isize));
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iqfile(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if fspool_file(*argv.offset(1 as libc::c_int as isize)) == 0 {
        return 0 as libc::c_int;
    }
    cQfiles += 1;
    cQfiles;
    azQfiles = xrealloc(
        azQfiles as pointer,
        (cQfiles as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    azQfiles_to = xrealloc(
        azQfiles_to as pointer,
        (cQfiles as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh2 = *azQfiles.offset((cQfiles - 1 as libc::c_int) as isize);
    *fresh2 = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    if argc > 2 as libc::c_int {
        let ref mut fresh3 = *azQfiles_to.offset((cQfiles - 1 as libc::c_int) as isize);
        *fresh3 = zbufcpy(*argv.offset(2 as libc::c_int as isize));
    } else {
        let ref mut fresh4 = *azQfiles_to.offset((cQfiles - 1 as libc::c_int) as isize);
        *fresh4 = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iqrequestor(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    if argc == 2 as libc::c_int {
        zQrequestor = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    } else if argc > 2 as libc::c_int {
        zQrequestor = zbufalc(
            (strlen(*argv.offset(1 as libc::c_int as isize)))
                .wrapping_add(strlen(*argv.offset(2 as libc::c_int as isize)))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                ),
        );
        sprintf(
            zQrequestor,
            b"%s!%s\0" as *const u8 as *const libc::c_char,
            *argv.offset(2 as libc::c_int as isize),
            *argv.offset(1 as libc::c_int as isize),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iquser(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    if argc > 1 as libc::c_int {
        zQuser = *argv.offset(1 as libc::c_int as isize);
    }
    if argc > 2 as libc::c_int {
        zQsystem = *argv.offset(2 as libc::c_int as isize);
    }
    return 0x800 as libc::c_int;
}
unsafe extern "C" fn iqset(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut pf: *mut boolean = pvar as *mut boolean;
    *pf = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn uqdo_xqt_file(
    mut puuconf: pointer,
    mut zfile: *const libc::c_char,
    mut zbase: *const libc::c_char,
    mut qsys: *const uuconf_system,
    mut zlocalname: *const libc::c_char,
    mut zcmd: *const libc::c_char,
    mut pfprocessed: *mut boolean,
) {
    let mut zabsolute: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ferr: boolean = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut iuuconf: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iclean: libc::c_int = 0;
    let mut zmail: *const libc::c_char = 0 as *const libc::c_char;
    let mut zoutput: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zinput: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fbadname: boolean = 0;
    let mut abtemp: [libc::c_char; 15] = [0; 15];
    let mut abdata: [libc::c_char; 15] = [0; 15];
    let mut zerror: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut soutsys: uuconf_system = uuconf_system {
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
    let mut qoutsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut fshell: boolean = 0;
    let mut clen: size_t = 0;
    let mut zfullcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ftemp: boolean = 0;
    *pfprocessed = 0 as libc::c_int;
    e = fopen(zfile, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        return;
    }
    azQargs = 0 as *mut *mut libc::c_char;
    zQcmd = 0 as *mut libc::c_char;
    zQinput = 0 as *mut libc::c_char;
    zQoutfile = 0 as *mut libc::c_char;
    zQoutsys = 0 as *mut libc::c_char;
    cQfiles = 0 as libc::c_int;
    azQfiles = 0 as *mut *mut libc::c_char;
    azQfiles_to = 0 as *mut *mut libc::c_char;
    zQrequestor = 0 as *mut libc::c_char;
    zQuser = 0 as *const libc::c_char;
    zQsystem = 0 as *const libc::c_char;
    fQno_ack = 0 as libc::c_int;
    fQsuccess_ack = 0 as libc::c_int;
    fQsend_input = 0 as libc::c_int;
    fQuse_exec = 0 as libc::c_int;
    zQstatus_file = 0 as *const libc::c_char;
    fQquoted = 0 as libc::c_int;
    iuuconf = uuconf_cmd_file(
        puuconf,
        e,
        asQcmds.as_ptr(),
        zbase as pointer,
        ::std::mem::transmute::<
            *mut libc::c_void,
            uuconf_cmdtabfn,
        >(0 as *mut libc::c_void),
        0x1 as libc::c_int | 0x4 as libc::c_int,
        0 as *mut libc::c_void,
    );
    fclose(e);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        if iuuconf & 0xff as libc::c_int == 5 as libc::c_int
            || iuuconf & 0xff as libc::c_int == 6 as libc::c_int
        {
            let mut az: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
            let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
            i = 0 as libc::c_int;
            let fresh5 = i;
            i = i + 1;
            az[fresh5
                as usize] = b"The execution file\n\t\0" as *const u8
                as *const libc::c_char;
            let fresh6 = i;
            i = i + 1;
            az[fresh6 as usize] = zfile;
            let fresh7 = i;
            i = i + 1;
            az[fresh7
                as usize] = b"\nfor system\n\t\0" as *const u8 as *const libc::c_char;
            let fresh8 = i;
            i = i + 1;
            az[fresh8 as usize] = (*qsys).uuconf_zname;
            let fresh9 = i;
            i = i + 1;
            az[fresh9
                as usize] = b"\nwas corrupt.  \0" as *const u8 as *const libc::c_char;
            znew = zsysdep_save_corrupt_file(zfile);
            if znew.is_null() {
                let fresh10 = i;
                i = i + 1;
                az[fresh10
                    as usize] = b"The file could not be preserved.\n\0" as *const u8
                    as *const libc::c_char;
                remove(zfile);
            } else {
                let fresh11 = i;
                i = i + 1;
                az[fresh11
                    as usize] = b"It has been moved to\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh12 = i;
                i = i + 1;
                az[fresh12 as usize] = znew;
                let fresh13 = i;
                i = i + 1;
                az[fresh13 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
            }
            fsysdep_mail(
                b"uucp\0" as *const u8 as *const libc::c_char,
                b"Corrupt execution file\0" as *const u8 as *const libc::c_char,
                i,
                az.as_mut_ptr(),
            );
            ubuffree(znew);
        }
        return;
    }
    if fQquoted != 0 {
        if !azQargs.is_null() {
            i = 0 as libc::c_int;
            while !(*azQargs.offset(i as isize)).is_null() {
                cescape(*azQargs.offset(i as isize));
                i += 1;
                i;
            }
        }
        if !zQcmd.is_null() {
            cescape(zQcmd);
        }
        if !zQinput.is_null() {
            cescape(zQinput);
        }
        if !zQoutfile.is_null() {
            cescape(zQoutfile);
        }
        if !zQoutsys.is_null() {
            cescape(zQoutsys);
        }
        i = 0 as libc::c_int;
        while i < cQfiles {
            cescape(*azQfiles.offset(i as isize));
            if !(*azQfiles_to.offset(i as isize)).is_null() {
                cescape(*azQfiles_to.offset(i as isize));
            }
            i += 1;
            i;
        }
        if !zQrequestor.is_null() {
            cescape(zQrequestor);
        }
        if !zQuser.is_null() {
            cescape(zQuser as *mut libc::c_char);
        }
        if !zQsystem.is_null() {
            cescape(zQsystem as *mut libc::c_char);
        }
        if !zQstatus_file.is_null() {
            cescape(zQstatus_file as *mut libc::c_char);
        }
    }
    iclean = 0 as libc::c_int;
    if azQargs.is_null() {
        ulog(
            LOG_ERROR,
            b"%s: No command given\0" as *const u8 as *const libc::c_char,
            zbase,
        );
        uqcleanup(zfile, iclean | 0o1 as libc::c_int);
        return;
    }
    if !zcmd.is_null() {
        if strcmp(zcmd, *azQargs.offset(0 as libc::c_int as isize)) != 0 as libc::c_int {
            uqcleanup(zfile, iclean);
            return;
        }
    } else if fsysdep_uuxqt_locked(*azQargs.offset(0 as libc::c_int as isize)) != 0 {
        uqcleanup(zfile, iclean);
        return;
    }
    if fsysdep_lock_uuxqt_file(zfile) == 0 {
        uqcleanup(zfile, iclean);
        return;
    }
    zQunlock_file = zfile;
    if fsysdep_file_exists(zfile) == 0 {
        uqcleanup(zfile, iclean);
        return;
    }
    if !zQuser.is_null() {
        ulog_user(zQuser);
    } else if !zQrequestor.is_null() {
        ulog_user(zQrequestor);
    } else {
        ulog_user(b"unknown\0" as *const u8 as *const libc::c_char);
    }
    if zQsystem.is_null()
        || strncmp(zQsystem, (*qsys).uuconf_zname, strlen((*qsys).uuconf_zname))
            != 0 as libc::c_int
    {
        zQsystem = (*qsys).uuconf_zname;
    }
    i = 0 as libc::c_int;
    while i < cQfiles {
        let mut zreal: *mut libc::c_char = 0 as *mut libc::c_char;
        zreal = zsysdep_spool_file_name(
            qsys,
            *azQfiles.offset(i as isize),
            0 as *mut libc::c_void,
        );
        if zreal.is_null() {
            uqcleanup(zfile, iclean);
            return;
        }
        if fsysdep_file_exists(zreal) == 0 {
            uqcleanup(zfile, iclean);
            return;
        }
        ubuffree(*azQfiles.offset(i as isize));
        let ref mut fresh14 = *azQfiles.offset(i as isize);
        *fresh14 = zbufcpy(zreal);
        ubuffree(zreal);
        i += 1;
        i;
    }
    if fsysdep_lock_uuxqt_dir(iQlock_seq) == 0 {
        ulog(
            LOG_ERROR,
            b"Could not lock execute directory\0" as *const u8 as *const libc::c_char,
        );
        uqcleanup(zfile, iclean);
        return;
    }
    fQunlock_directory = 1 as libc::c_int;
    iclean |= 0o1 as libc::c_int | 0o2 as libc::c_int;
    *pfprocessed = 1 as libc::c_int;
    zmail = 0 as *const libc::c_char;
    if !zQrequestor.is_null() {
        zmail = zQrequestor;
    } else if !zQuser.is_null() {
        zmail = zQuser;
    }
    if !zmail.is_null() && (strchr(zmail, '@' as i32)).is_null()
        && strcmp(zQsystem, zlocalname) != 0 as libc::c_int
    {
        let mut zset: *mut libc::c_char = 0 as *mut libc::c_char;
        zset = zbufalc(
            (strlen(zQsystem))
                .wrapping_add(strlen(zmail))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        sprintf(zset, b"%s!%s\0" as *const u8 as *const libc::c_char, zQsystem, zmail);
        zmail = zset;
        zQmail = zset;
        iclean |= 0o40 as libc::c_int;
    }
    if strcmp(
        *azQargs.offset(0 as libc::c_int as isize),
        b"uucp\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fmany: boolean = 0;
        let mut finoptions: boolean = 0;
        let mut azargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut zuser: *const libc::c_char = 0 as *const libc::c_char;
        zfrom = 0 as *mut libc::c_char;
        zto = 0 as *mut libc::c_char;
        fmany = 0 as libc::c_int;
        finoptions = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while !(*azQargs.offset(i as isize)).is_null() {
            if *(*azQargs.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32 && finoptions != 0
            {
                if *(*azQargs.offset(i as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                {
                    if *(*azQargs.offset(i as isize)).offset(2 as libc::c_int as isize)
                        as libc::c_int == '\0' as i32
                    {
                        finoptions = 0 as libc::c_int;
                    } else if strncmp(
                        (*azQargs.offset(i as isize)).offset(2 as libc::c_int as isize),
                        b"g\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                        || strncmp(
                            (*azQargs.offset(i as isize))
                                .offset(2 as libc::c_int as isize),
                            b"not\0" as *const u8 as *const libc::c_char,
                            3 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        || strncmp(
                            (*azQargs.offset(i as isize))
                                .offset(2 as libc::c_int as isize),
                            b"s\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                    {
                        if (strchr(
                            (*azQargs.offset(i as isize))
                                .offset(2 as libc::c_int as isize),
                            '=' as i32,
                        ))
                            .is_null()
                        {
                            i += 1;
                            i;
                        }
                    } else if strncmp(
                        (*azQargs.offset(i as isize)).offset(2 as libc::c_int as isize),
                        b"con\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                        || strncmp(
                            (*azQargs.offset(i as isize))
                                .offset(2 as libc::c_int as isize),
                            b"us\0" as *const u8 as *const libc::c_char,
                            2 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        || strncmp(
                            (*azQargs.offset(i as isize))
                                .offset(2 as libc::c_int as isize),
                            b"de\0" as *const u8 as *const libc::c_char,
                            2 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                    {
                        *(*azQargs.offset(i as isize))
                            .offset(
                                1 as libc::c_int as isize,
                            ) = 'r' as i32 as libc::c_char;
                        *(*azQargs.offset(i as isize))
                            .offset(
                                2 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        if (strchr(
                            (*azQargs.offset(i as isize))
                                .offset(3 as libc::c_int as isize),
                            '=' as i32,
                        ))
                            .is_null()
                        {
                            i += 1;
                            i;
                            let ref mut fresh15 = *azQargs.offset(i as isize);
                            *fresh15 = zbufcpy(
                                b"-r\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                } else {
                    let mut zopts: *mut libc::c_char = 0 as *mut libc::c_char;
                    zopts = (*azQargs.offset(i as isize))
                        .offset(1 as libc::c_int as isize);
                    while *zopts as libc::c_int != '\0' as i32 {
                        if *zopts as libc::c_int == 'g' as i32
                            || *zopts as libc::c_int == 'n' as i32
                            || *zopts as libc::c_int == 's' as i32
                        {
                            if *zopts.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32
                            {
                                i += 1;
                                i;
                            }
                            break;
                        } else if *zopts as libc::c_int == 'I' as i32
                            || *zopts as libc::c_int == 'u' as i32
                            || *zopts as libc::c_int == 'x' as i32
                        {
                            *zopts = 'r' as i32 as libc::c_char;
                            if *zopts.offset(1 as libc::c_int as isize) as libc::c_int
                                != '\0' as i32
                            {
                                *zopts
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = '\0' as i32 as libc::c_char;
                            } else {
                                i += 1;
                                i;
                                let ref mut fresh16 = *azQargs.offset(i as isize);
                                *fresh16 = zbufcpy(
                                    b"-r\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            break;
                        } else {
                            zopts = zopts.offset(1);
                            zopts;
                        }
                    }
                }
            } else if zfrom.is_null() {
                zfrom = *azQargs.offset(i as isize);
            } else if zto.is_null() {
                zto = *azQargs.offset(i as isize);
            } else {
                fmany = 1 as libc::c_int;
                break;
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while !(*azQargs.offset(i as isize)).is_null() {
            i += 1;
            i;
        }
        azargs = xmalloc(
            ((i + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh17 = *azargs.offset(0 as libc::c_int as isize);
        *fresh17 = *azQargs.offset(0 as libc::c_int as isize);
        zuser = zQuser;
        if zuser.is_null() {
            zuser = b"uucp\0" as *const u8 as *const libc::c_char;
        }
        let ref mut fresh18 = *azargs.offset(1 as libc::c_int as isize);
        *fresh18 = zbufalc(
            (strlen(zQsystem))
                .wrapping_add(strlen(zuser))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
                ),
        );
        sprintf(
            *azargs.offset(1 as libc::c_int as isize),
            b"-u%s!%s\0" as *const u8 as *const libc::c_char,
            zQsystem,
            zuser,
        );
        memcpy(
            azargs.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            azQargs.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (i as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
        xfree(azQargs as pointer);
        azQargs = azargs;
        zabsolute = zsysdep_find_command(
            b"uucp\0" as *const u8 as *const libc::c_char,
            (*qsys).uuconf_pzcmds,
            (*qsys).uuconf_pzpath,
            &mut ferr,
        );
        if zabsolute.is_null() && ferr == 0 {
            let mut azcmds: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
            if fqforward(
                zfrom,
                (*qsys).uuconf_pzforward_from,
                b"from\0" as *const u8 as *const libc::c_char,
                zmail,
            ) == 0
                || fqforward(
                    zto,
                    (*qsys).uuconf_pzforward_to,
                    b"to\0" as *const u8 as *const libc::c_char,
                    zmail,
                ) == 0
            {
                uqcleanup(zfile, iclean);
                return;
            }
            if fmany != 0 || zfrom.is_null() || zto.is_null() {
                ulog(
                    LOG_ERROR,
                    b"Bad uucp request %s\0" as *const u8 as *const libc::c_char,
                    zQcmd,
                );
                if !zmail.is_null() && fQno_ack == 0 {
                    let mut az_0: [*const libc::c_char; 20] = [0
                        as *const libc::c_char; 20];
                    i = 0 as libc::c_int;
                    let fresh19 = i;
                    i = i + 1;
                    az_0[fresh19
                        as usize] = b"Your execution request failed because it was an\0"
                        as *const u8 as *const libc::c_char;
                    let fresh20 = i;
                    i = i + 1;
                    az_0[fresh20
                        as usize] = b" unsupported uucp request.\n\0" as *const u8
                        as *const libc::c_char;
                    let fresh21 = i;
                    i = i + 1;
                    az_0[fresh21
                        as usize] = b"Execution requested was:\n\t\0" as *const u8
                        as *const libc::c_char;
                    let fresh22 = i;
                    i = i + 1;
                    az_0[fresh22 as usize] = zQcmd;
                    let fresh23 = i;
                    i = i + 1;
                    az_0[fresh23 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                    fsysdep_mail(
                        zmail,
                        b"Execution failed\0" as *const u8 as *const libc::c_char,
                        i,
                        az_0.as_mut_ptr(),
                    );
                }
                uqcleanup(zfile, iclean);
                return;
            }
            azcmds[0 as libc::c_int
                as usize] = b"uucp\0" as *const u8 as *const libc::c_char;
            azcmds[1 as libc::c_int as usize] = 0 as *const libc::c_char;
            zabsolute = zsysdep_find_command(
                b"uucp\0" as *const u8 as *const libc::c_char,
                azcmds.as_mut_ptr() as *mut *mut libc::c_char,
                (*qsys).uuconf_pzpath,
                &mut ferr,
            );
        }
        if zabsolute.is_null() {
            if ferr == 0 {
                ulog(
                    LOG_ERROR,
                    b"Can't find uucp executable\0" as *const u8 as *const libc::c_char,
                );
            }
            uqcleanup(zfile, iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int));
            *pfprocessed = 0 as libc::c_int;
            return;
        }
    } else {
        zabsolute = zsysdep_find_command(
            *azQargs.offset(0 as libc::c_int as isize),
            (*qsys).uuconf_pzcmds,
            (*qsys).uuconf_pzpath,
            &mut ferr,
        );
        if zabsolute.is_null() {
            if ferr != 0 {
                uqcleanup(zfile, iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int));
                *pfprocessed = 0 as libc::c_int;
                return;
            }
            ulog(
                LOG_ERROR,
                b"Not permitted to execute %s\0" as *const u8 as *const libc::c_char,
                *azQargs.offset(0 as libc::c_int as isize),
            );
            if !zmail.is_null() && fQno_ack == 0 {
                let mut az_1: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
                i = 0 as libc::c_int;
                let fresh24 = i;
                i = i + 1;
                az_1[fresh24
                    as usize] = b"Your execution request failed because you are not\0"
                    as *const u8 as *const libc::c_char;
                let fresh25 = i;
                i = i + 1;
                az_1[fresh25
                    as usize] = b" permitted to execute\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh26 = i;
                i = i + 1;
                az_1[fresh26 as usize] = *azQargs.offset(0 as libc::c_int as isize);
                let fresh27 = i;
                i = i + 1;
                az_1[fresh27
                    as usize] = b"\non this system.\n\0" as *const u8
                    as *const libc::c_char;
                let fresh28 = i;
                i = i + 1;
                az_1[fresh28
                    as usize] = b"Execution requested was:\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh29 = i;
                i = i + 1;
                az_1[fresh29 as usize] = zQcmd;
                let fresh30 = i;
                i = i + 1;
                az_1[fresh30 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                fsysdep_mail(
                    zmail,
                    b"Execution failed\0" as *const u8 as *const libc::c_char,
                    i,
                    az_1.as_mut_ptr(),
                );
            }
            iclean = isave_files(qsys, zmail, zfile, iclean);
            uqcleanup(zfile, iclean);
            return;
        }
    }
    ubuffree(*azQargs.offset(0 as libc::c_int as isize));
    let ref mut fresh31 = *azQargs.offset(0 as libc::c_int as isize);
    *fresh31 = zabsolute;
    i = 1 as libc::c_int;
    while !(*azQargs.offset(i as isize)).is_null() {
        let mut zlocal: *mut libc::c_char = 0 as *mut libc::c_char;
        zlocal = zsysdep_xqt_local_file(qsys, *azQargs.offset(i as isize));
        if !zlocal.is_null() {
            ubuffree(*azQargs.offset(i as isize));
            let ref mut fresh32 = *azQargs.offset(i as isize);
            *fresh32 = zlocal;
        }
        i += 1;
        i;
    }
    ulog(
        LOG_NORMAL,
        b"Executing %s (%s)\0" as *const u8 as *const libc::c_char,
        zbase,
        zQcmd,
    );
    if !zQinput.is_null() {
        let mut fspool: boolean = 0;
        let mut zreal_0: *mut libc::c_char = 0 as *mut libc::c_char;
        fspool = fspool_file(zQinput);
        if fspool == 0 {
            zreal_0 = zsysdep_local_file(zQinput, (*qsys).uuconf_zpubdir, &mut fbadname);
        } else {
            zreal_0 = zsysdep_spool_file_name(qsys, zQinput, 0 as *mut libc::c_void);
            fbadname = 0 as libc::c_int;
        }
        if zreal_0.is_null() && fbadname == 0 {
            uqcleanup(zfile, iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int));
            *pfprocessed = 0 as libc::c_int;
            return;
        }
        if !zreal_0.is_null() {
            zQinput = zreal_0;
            iclean |= 0o4 as libc::c_int;
            if fspool != 0 {
                iclean |= 0o10 as libc::c_int;
            }
        }
        if zreal_0.is_null()
            || fspool == 0
                && fin_directory_list(
                    zQinput,
                    (*qsys).uuconf_pzremote_send,
                    (*qsys).uuconf_zpubdir,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    0 as *mut libc::c_void as *const libc::c_char,
                ) == 0
        {
            ulog(
                LOG_ERROR,
                b"Not permitted to read %s\0" as *const u8 as *const libc::c_char,
                zQinput,
            );
            if !zmail.is_null() && fQno_ack == 0 {
                let mut az_2: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
                i = 0 as libc::c_int;
                let fresh33 = i;
                i = i + 1;
                az_2[fresh33
                    as usize] = b"Your execution request failed because you are\0"
                    as *const u8 as *const libc::c_char;
                let fresh34 = i;
                i = i + 1;
                az_2[fresh34
                    as usize] = b" not permitted to read\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh35 = i;
                i = i + 1;
                az_2[fresh35 as usize] = zQinput;
                let fresh36 = i;
                i = i + 1;
                az_2[fresh36
                    as usize] = b"\non this system.\n\0" as *const u8
                    as *const libc::c_char;
                let fresh37 = i;
                i = i + 1;
                az_2[fresh37
                    as usize] = b"Execution requested was:\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh38 = i;
                i = i + 1;
                az_2[fresh38 as usize] = zQcmd;
                let fresh39 = i;
                i = i + 1;
                az_2[fresh39 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                fsysdep_mail(
                    zmail,
                    b"Execution failed\0" as *const u8 as *const libc::c_char,
                    i,
                    az_2.as_mut_ptr(),
                );
            }
            uqcleanup(zfile, iclean);
            return;
        }
    }
    zoutput = 0 as *mut libc::c_char;
    if zQoutfile.is_null() {
        qoutsys = 0 as *const uuconf_system;
    } else if !zQoutsys.is_null() && strcmp(zQoutsys, zlocalname) != 0 as libc::c_int {
        let mut zdata: *mut libc::c_char = 0 as *mut libc::c_char;
        if strcmp(zQoutsys, (*qsys).uuconf_zname) == 0 as libc::c_int {
            qoutsys = qsys;
        } else {
            iuuconf = uuconf_system_info(puuconf, zQoutsys, &mut soutsys);
            if iuuconf != 0 as libc::c_int {
                if iuuconf != 1 as libc::c_int {
                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                    uqcleanup(
                        zfile,
                        iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int),
                    );
                    *pfprocessed = 0 as libc::c_int;
                    return;
                }
                if funknown_system(puuconf, zQoutsys, &mut soutsys) == 0 {
                    ulog(
                        LOG_ERROR,
                        b"Can't send standard output to unknown system %s\0" as *const u8
                            as *const libc::c_char,
                        zQoutsys,
                    );
                    uqcleanup(zfile, iclean);
                    return;
                }
            }
            qoutsys = &mut soutsys;
        }
        zdata = zsysdep_data_file_name(
            qoutsys,
            zlocalname,
            'N' as i32,
            0 as libc::c_int,
            abtemp.as_mut_ptr(),
            abdata.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        if zdata.is_null() {
            uqcleanup(zfile, iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int));
            *pfprocessed = 0 as libc::c_int;
            return;
        }
        zoutput = zdata;
        zQoutput = zoutput;
        iclean |= 0o20 as libc::c_int;
    } else {
        let mut fok: boolean = 0;
        qoutsys = 0 as *const uuconf_system;
        if fspool_file(zQoutfile) != 0 {
            fok = 0 as libc::c_int;
        } else {
            zoutput = zsysdep_local_file(
                zQoutfile,
                (*qsys).uuconf_zpubdir,
                &mut fbadname,
            );
            if zoutput.is_null() {
                if fbadname == 0 {
                    uqcleanup(
                        zfile,
                        iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int),
                    );
                    *pfprocessed = 0 as libc::c_int;
                    return;
                }
                fok = 0 as libc::c_int;
            } else {
                ubuffree(zQoutfile);
                zQoutfile = zoutput;
                fok = fin_directory_list(
                    zQoutfile,
                    (*qsys).uuconf_pzremote_receive,
                    (*qsys).uuconf_zpubdir,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *const libc::c_char,
                );
            }
        }
        if fok == 0 {
            ulog(
                LOG_ERROR,
                b"Not permitted to write to %s\0" as *const u8 as *const libc::c_char,
                zQoutfile,
            );
            if !zmail.is_null() && fQno_ack == 0 {
                let mut az_3: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
                i = 0 as libc::c_int;
                let fresh40 = i;
                i = i + 1;
                az_3[fresh40
                    as usize] = b"Your execution request failed because you are\0"
                    as *const u8 as *const libc::c_char;
                let fresh41 = i;
                i = i + 1;
                az_3[fresh41
                    as usize] = b" not permitted to write to\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh42 = i;
                i = i + 1;
                az_3[fresh42 as usize] = zQoutfile;
                let fresh43 = i;
                i = i + 1;
                az_3[fresh43
                    as usize] = b"\non this system.\n\0" as *const u8
                    as *const libc::c_char;
                let fresh44 = i;
                i = i + 1;
                az_3[fresh44
                    as usize] = b"Execution requested was:\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh45 = i;
                i = i + 1;
                az_3[fresh45 as usize] = zQcmd;
                let fresh46 = i;
                i = i + 1;
                az_3[fresh46 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                fsysdep_mail(
                    zmail,
                    b"Execution failed\0" as *const u8 as *const libc::c_char,
                    i,
                    az_3.as_mut_ptr(),
                );
            }
            uqcleanup(zfile, iclean);
            return;
        }
    }
    zinput = zQinput;
    if fsysdep_copy_uuxqt_files(
        cQfiles,
        azQfiles as *mut *const libc::c_char,
        azQfiles_to as *mut *const libc::c_char,
        iQlock_seq,
        &mut zinput,
    ) == 0
    {
        uqcleanup(zfile, iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int));
        *pfprocessed = 0 as libc::c_int;
        return;
    }
    if !zQinput.is_null() && strcmp(zQinput, zinput) != 0 as libc::c_int {
        if iclean & 0o4 as libc::c_int != 0 as libc::c_int {
            ubuffree(zQinput);
        }
        zQinput = zinput;
        iclean |= 0o4 as libc::c_int;
    }
    fshell = 0 as libc::c_int;
    clen = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while !(*azQargs.offset(i as isize)).is_null() {
        clen = (clen as libc::c_ulong)
            .wrapping_add(
                (strlen(*azQargs.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        i += 1;
        i;
    }
    zfullcmd = zbufalc(clen);
    strcpy(zfullcmd, *azQargs.offset(0 as libc::c_int as isize));
    i = 1 as libc::c_int;
    while !(*azQargs.offset(i as isize)).is_null() {
        strcat(zfullcmd, b" \0" as *const u8 as *const libc::c_char);
        strcat(zfullcmd, *azQargs.offset(i as isize));
        i += 1;
        i;
    }
    if fsysdep_execute(
        qsys,
        if zQuser.is_null() {
            b"uucp\0" as *const u8 as *const libc::c_char
        } else {
            zQuser
        },
        azQargs as *mut *const libc::c_char,
        zfullcmd,
        zQinput,
        zoutput,
        fshell,
        iQlock_seq,
        &mut zerror,
        &mut ftemp,
    ) == 0
    {
        ubuffree(zfullcmd);
        if ftemp != 0 {
            ulog(
                LOG_NORMAL,
                b"Will retry later (%s)\0" as *const u8 as *const libc::c_char,
                zbase,
            );
            if !zoutput.is_null() {
                remove(zoutput);
            }
            if !zerror.is_null() {
                remove(zerror);
                ubuffree(zerror);
            }
            uqcleanup(zfile, iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int));
            *pfprocessed = 0 as libc::c_int;
            return;
        }
        ulog(
            LOG_NORMAL,
            b"Execution failed (%s)\0" as *const u8 as *const libc::c_char,
            zbase,
        );
        if !zmail.is_null() && fQno_ack == 0 {
            let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
            let mut cgot: libc::c_int = 0;
            let mut eerr: *mut FILE = 0 as *mut FILE;
            let mut istart: libc::c_int = 0;
            cgot = 20 as libc::c_int;
            pz = xmalloc(
                (cgot as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
            i = 0 as libc::c_int;
            let fresh47 = i;
            i = i + 1;
            let ref mut fresh48 = *pz.offset(fresh47 as isize);
            *fresh48 = b"Execution request failed:\n\t\0" as *const u8
                as *const libc::c_char;
            let fresh49 = i;
            i = i + 1;
            let ref mut fresh50 = *pz.offset(fresh49 as isize);
            *fresh50 = zQcmd;
            let fresh51 = i;
            i = i + 1;
            let ref mut fresh52 = *pz.offset(fresh51 as isize);
            *fresh52 = b"\n\0" as *const u8 as *const libc::c_char;
            if zerror.is_null() {
                eerr = 0 as *mut FILE;
            } else {
                eerr = fopen(zerror, b"r\0" as *const u8 as *const libc::c_char);
            }
            if eerr.is_null() {
                let fresh53 = i;
                i = i + 1;
                let ref mut fresh54 = *pz.offset(fresh53 as isize);
                *fresh54 = b"There was no output on standard error\n\0" as *const u8
                    as *const libc::c_char;
                istart = i;
            } else {
                let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cline: size_t = 0;
                let fresh55 = i;
                i = i + 1;
                let ref mut fresh56 = *pz.offset(fresh55 as isize);
                *fresh56 = b"Standard error output was:\n\0" as *const u8
                    as *const libc::c_char;
                istart = i;
                zline = 0 as *mut libc::c_char;
                cline = 0 as libc::c_int as size_t;
                while getline(&mut zline, &mut cline, eerr)
                    > 0 as libc::c_int as libc::c_long
                {
                    if i >= cgot {
                        cgot += 20 as libc::c_int;
                        pz = xrealloc(
                            pz as pointer,
                            (cgot as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*const libc::c_char>()
                                        as libc::c_ulong,
                                ),
                        ) as *mut *const libc::c_char;
                    }
                    let fresh57 = i;
                    i = i + 1;
                    let ref mut fresh58 = *pz.offset(fresh57 as isize);
                    *fresh58 = zbufcpy(zline);
                }
                fclose(eerr);
                xfree(zline as pointer);
            }
            fsysdep_mail(
                zmail,
                b"Execution failed\0" as *const u8 as *const libc::c_char,
                i,
                pz,
            );
            while istart < i {
                ubuffree(*pz.offset(istart as isize) as *mut libc::c_char);
                istart += 1;
                istart;
            }
            xfree(pz as pointer);
        }
        if !qoutsys.is_null() {
            remove(zoutput);
        }
        iclean = isave_files(qsys, zmail, zfile, iclean);
    } else {
        ubuffree(zfullcmd);
        if !zmail.is_null() && fQsuccess_ack != 0 {
            let mut az_4: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
            i = 0 as libc::c_int;
            let fresh59 = i;
            i = i + 1;
            az_4[fresh59
                as usize] = b"\nExecution request succeeded:\n\t\0" as *const u8
                as *const libc::c_char;
            let fresh60 = i;
            i = i + 1;
            az_4[fresh60 as usize] = zQcmd;
            let fresh61 = i;
            i = i + 1;
            az_4[fresh61 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
            fsysdep_mail(
                zmail,
                b"Execution succeded\0" as *const u8 as *const libc::c_char,
                i,
                az_4.as_mut_ptr(),
            );
        }
        if !qoutsys.is_null() {
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
            s.bcmd = 'S' as i32 as libc::c_char;
            s.bgrade = 'N' as i32 as libc::c_char;
            s.pseq = 0 as *mut libc::c_void;
            s.zfrom = abtemp.as_mut_ptr();
            s.zto = zQoutfile;
            if !zQuser.is_null() {
                s.zuser = zQuser;
            } else {
                s.zuser = b"uucp\0" as *const u8 as *const libc::c_char;
            }
            if !zmail.is_null() && fQsuccess_ack != 0 {
                s.zoptions = b"Cn\0" as *const u8 as *const libc::c_char;
            } else {
                s.zoptions = b"C\0" as *const u8 as *const libc::c_char;
            }
            s.ztemp = abtemp.as_mut_ptr();
            s.imode = 0o666 as libc::c_int as libc::c_uint;
            if !zmail.is_null() && fQsuccess_ack != 0 {
                s.znotify = zmail;
            } else {
                s.znotify = b"\0" as *const u8 as *const libc::c_char;
            }
            s.cbytes = -(1 as libc::c_int) as libc::c_long;
            s.zcmd = 0 as *const libc::c_char;
            s.ipos = 0 as libc::c_int as libc::c_long;
            ubuffree(
                zsysdep_spool_commands(
                    qoutsys,
                    'N' as i32,
                    1 as libc::c_int,
                    &mut s,
                    0 as *mut libc::c_void as *mut boolean,
                ),
            );
        }
    }
    if !zerror.is_null() {
        remove(zerror);
        ubuffree(zerror);
    }
    uqcleanup(zfile, iclean);
}
unsafe extern "C" fn isave_files(
    mut qsys: *const uuconf_system,
    mut zmail: *const libc::c_char,
    mut zfile: *const libc::c_char,
    mut iclean: libc::c_int,
) -> libc::c_int {
    let mut cspace: libc::c_long = 0;
    let mut zsavecmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pzsave: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut ifile: libc::c_int = 0;
    let mut zsaveinput: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: libc::c_int = 0;
    cspace = csysdep_bytes_free(zfile);
    if cspace == -(1 as libc::c_int) as libc::c_long {
        cspace = 10240 as libc::c_int as libc::c_long;
    }
    cspace
        -= (*qsys).uuconf_cfree_space
            + (*qsys).uuconf_cfree_space / 2 as libc::c_int as libc::c_long;
    if cspace < 0 as libc::c_int as libc::c_long {
        return iclean;
    }
    zsavecmd = zsysdep_save_failed_file(zfile);
    if zsavecmd.is_null() {
        return iclean;
    }
    c = 1 as libc::c_int;
    pzsave = xmalloc(
        (cQfiles as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    ifile = 0 as libc::c_int;
    while ifile < cQfiles {
        if !(*azQfiles.offset(ifile as isize)).is_null() {
            c += 1;
            c;
            let ref mut fresh62 = *pzsave.offset(ifile as isize);
            *fresh62 = zsysdep_save_failed_file(*azQfiles.offset(ifile as isize));
            if (*pzsave.offset(ifile as isize)).is_null() {
                ubuffree(zsavecmd);
                i = 0 as libc::c_int;
                while i < ifile {
                    if !(*azQfiles.offset(i as isize)).is_null() {
                        ubuffree(*pzsave.offset(i as isize));
                    }
                    i += 1;
                    i;
                }
                xfree(pzsave as pointer);
                return iclean;
            }
        }
        ifile += 1;
        ifile;
    }
    zsaveinput = 0 as *mut libc::c_char;
    if iclean & 0o10 as libc::c_int != 0 as libc::c_int
        && fsysdep_file_exists(zQinput) != 0
    {
        zsaveinput = zsysdep_save_failed_file(zQinput);
        if zsaveinput.is_null() {
            ubuffree(zsavecmd);
            i = 0 as libc::c_int;
            while i < cQfiles {
                if !(*azQfiles.offset(i as isize)).is_null() {
                    ubuffree(*pzsave.offset(i as isize));
                }
                i += 1;
                i;
            }
            xfree(pzsave as pointer);
            return iclean;
        }
    }
    pz = xmalloc(
        ((20 as libc::c_int + 2 as libc::c_int * cQfiles) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    i = 0 as libc::c_int;
    let fresh63 = i;
    i = i + 1;
    let ref mut fresh64 = *pz.offset(fresh63 as isize);
    *fresh64 = b"A UUCP execution request failed:\n\t\0" as *const u8
        as *const libc::c_char;
    let fresh65 = i;
    i = i + 1;
    let ref mut fresh66 = *pz.offset(fresh65 as isize);
    *fresh66 = zQcmd;
    if !zmail.is_null() {
        let fresh67 = i;
        i = i + 1;
        let ref mut fresh68 = *pz.offset(fresh67 as isize);
        *fresh68 = b"\nThe request was made by\n\t\0" as *const u8
            as *const libc::c_char;
        let fresh69 = i;
        i = i + 1;
        let ref mut fresh70 = *pz.offset(fresh69 as isize);
        *fresh70 = zmail;
    } else {
        let fresh71 = i;
        i = i + 1;
        let ref mut fresh72 = *pz.offset(fresh71 as isize);
        *fresh72 = b"\nThe request came from system\n\t\0" as *const u8
            as *const libc::c_char;
        let fresh73 = i;
        i = i + 1;
        let ref mut fresh74 = *pz.offset(fresh73 as isize);
        *fresh74 = (*qsys).uuconf_zname;
    }
    if c == 1 as libc::c_int && zsaveinput.is_null() {
        let fresh75 = i;
        i = i + 1;
        let ref mut fresh76 = *pz.offset(fresh75 as isize);
        *fresh76 = b"\nThe following file has been saved:\n\t\0" as *const u8
            as *const libc::c_char;
    } else {
        let fresh77 = i;
        i = i + 1;
        let ref mut fresh78 = *pz.offset(fresh77 as isize);
        *fresh78 = b"\nThe following files have been saved:\n\t\0" as *const u8
            as *const libc::c_char;
    }
    let fresh79 = i;
    i = i + 1;
    let ref mut fresh80 = *pz.offset(fresh79 as isize);
    *fresh80 = zsavecmd;
    ifile = 0 as libc::c_int;
    while ifile < cQfiles {
        if !(*azQfiles.offset(ifile as isize)).is_null() {
            let fresh81 = i;
            i = i + 1;
            let ref mut fresh82 = *pz.offset(fresh81 as isize);
            *fresh82 = b"\n\t\0" as *const u8 as *const libc::c_char;
            let fresh83 = i;
            i = i + 1;
            let ref mut fresh84 = *pz.offset(fresh83 as isize);
            *fresh84 = *pzsave.offset(ifile as isize);
        }
        ifile += 1;
        ifile;
    }
    if !zsaveinput.is_null() {
        let fresh85 = i;
        i = i + 1;
        let ref mut fresh86 = *pz.offset(fresh85 as isize);
        *fresh86 = b"\n\t\0" as *const u8 as *const libc::c_char;
        let fresh87 = i;
        i = i + 1;
        let ref mut fresh88 = *pz.offset(fresh87 as isize);
        *fresh88 = zsaveinput;
    }
    let fresh89 = i;
    i = i + 1;
    let ref mut fresh90 = *pz.offset(fresh89 as isize);
    *fresh90 = b"\n\0" as *const u8 as *const libc::c_char;
    fsysdep_mail(
        b"uucp\0" as *const u8 as *const libc::c_char,
        b"UUCP execution files saved after failure\0" as *const u8
            as *const libc::c_char,
        i,
        pz,
    );
    xfree(pz as pointer);
    ubuffree(zsavecmd);
    ifile = 0 as libc::c_int;
    while ifile < cQfiles {
        if !(*azQfiles.offset(ifile as isize)).is_null() {
            ubuffree(*pzsave.offset(ifile as isize));
        }
        ifile += 1;
        ifile;
    }
    xfree(pzsave as pointer);
    ubuffree(zsaveinput);
    return iclean & !(0o1 as libc::c_int | 0o2 as libc::c_int);
}
unsafe extern "C" fn uqcleanup(mut zfile: *const libc::c_char, mut iflags: libc::c_int) {
    let mut i: libc::c_int = 0;
    if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"uqcleanup: %s, %d\0" as *const u8 as *const libc::c_char,
            zfile,
            iflags,
        );
    }
    if iflags & 0o1 as libc::c_int != 0 as libc::c_int {
        remove(zfile);
    }
    if iflags & 0o2 as libc::c_int != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < cQfiles {
            if !(*azQfiles.offset(i as isize)).is_null() {
                remove(*azQfiles.offset(i as isize));
            }
            i += 1;
            i;
        }
        if iflags & 0o10 as libc::c_int != 0 as libc::c_int {
            remove(zQinput);
        }
    }
    if !zQunlock_file.is_null() {
        fsysdep_unlock_uuxqt_file(zQunlock_file);
        zQunlock_file = 0 as *const libc::c_char;
    }
    if iflags & 0o4 as libc::c_int != 0 as libc::c_int {
        ubuffree(zQinput);
    }
    if iflags & 0o20 as libc::c_int != 0 as libc::c_int {
        ubuffree(zQoutput);
    }
    if iflags & 0o40 as libc::c_int != 0 as libc::c_int {
        ubuffree(zQmail);
    }
    if fQunlock_directory != 0 {
        fsysdep_unlock_uuxqt_dir(iQlock_seq);
        fQunlock_directory = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < cQfiles {
        ubuffree(*azQfiles.offset(i as isize));
        ubuffree(*azQfiles_to.offset(i as isize));
        i += 1;
        i;
    }
    ubuffree(zQoutfile);
    ubuffree(zQoutsys);
    ubuffree(zQrequestor);
    if !azQargs.is_null() {
        i = 0 as libc::c_int;
        while !(*azQargs.offset(i as isize)).is_null() {
            ubuffree(*azQargs.offset(i as isize));
            i += 1;
            i;
        }
        xfree(azQargs as pointer);
        azQargs = 0 as *mut *mut libc::c_char;
    }
    xfree(zQcmd as pointer);
    zQcmd = 0 as *mut libc::c_char;
    xfree(azQfiles as pointer);
    azQfiles = 0 as *mut *mut libc::c_char;
    xfree(azQfiles_to as pointer);
    azQfiles_to = 0 as *mut *mut libc::c_char;
}
unsafe extern "C" fn fqforward(
    mut zfile: *const libc::c_char,
    mut pzallowed: *mut *mut libc::c_char,
    mut zlog: *const libc::c_char,
    mut zmail: *const libc::c_char,
) -> boolean {
    let mut zexclam: *const libc::c_char = 0 as *const libc::c_char;
    if zfile.is_null() {
        return 1 as libc::c_int;
    }
    zexclam = strchr(zfile, '!' as i32);
    if !zexclam.is_null() {
        let mut clen: size_t = 0;
        let mut zsys: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fret: boolean = 0;
        clen = zexclam.offset_from(zfile) as libc::c_long as size_t;
        zsys = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
        memcpy(zsys as *mut libc::c_void, zfile as *const libc::c_void, clen);
        *zsys.offset(clen as isize) = '\0' as i32 as libc::c_char;
        fret = 0 as libc::c_int;
        if !pzallowed.is_null() {
            let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            pz = pzallowed;
            while !(*pz).is_null() {
                if strcmp(*pz, b"ANY\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int || strcmp(*pz, zsys) == 0 as libc::c_int
                {
                    fret = 1 as libc::c_int;
                    break;
                } else {
                    pz = pz.offset(1);
                    pz;
                }
            }
        }
        if fret == 0 {
            ulog(
                LOG_ERROR,
                b"Not permitted to forward %s %s (%s)\0" as *const u8
                    as *const libc::c_char,
                zlog,
                zsys,
                zQcmd,
            );
            if !zmail.is_null() && fQno_ack == 0 {
                let mut i: libc::c_int = 0;
                let mut az: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
                i = 0 as libc::c_int;
                let fresh91 = i;
                i = i + 1;
                az[fresh91
                    as usize] = b"Your execution request failed because you are\0"
                    as *const u8 as *const libc::c_char;
                let fresh92 = i;
                i = i + 1;
                az[fresh92
                    as usize] = b" not permitted to forward files\n\0" as *const u8
                    as *const libc::c_char;
                let fresh93 = i;
                i = i + 1;
                az[fresh93 as usize] = zlog;
                let fresh94 = i;
                i = i + 1;
                az[fresh94
                    as usize] = b" the system\n\t\0" as *const u8 as *const libc::c_char;
                let fresh95 = i;
                i = i + 1;
                az[fresh95 as usize] = zsys;
                let fresh96 = i;
                i = i + 1;
                az[fresh96 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                let fresh97 = i;
                i = i + 1;
                az[fresh97
                    as usize] = b"Execution requested was:\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh98 = i;
                i = i + 1;
                az[fresh98 as usize] = zQcmd;
                let fresh99 = i;
                i = i + 1;
                az[fresh99 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                fsysdep_mail(
                    zmail,
                    b"Execution failed\0" as *const u8 as *const libc::c_char,
                    i,
                    az.as_mut_ptr(),
                );
            }
        }
        ubuffree(zsys);
        return fret;
    }
    return 1 as libc::c_int;
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

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
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
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn zsysdep_localname() -> *const libc::c_char;
    fn zsysdep_login_name() -> *const libc::c_char;
    fn usysdep_signal(isig: libc::c_int);
    fn fsysdep_run(
        ffork: boolean,
        zprogram: *const libc::c_char,
        zarg1: *const libc::c_char,
        zarg2: *const libc::c_char,
    ) -> boolean;
    fn zsysdep_add_base(
        zfile: *const libc::c_char,
        zname: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn fsysdep_change_mode(zfile: *const libc::c_char, imode: libc::c_uint) -> boolean;
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
    fn zsysdep_local_file_cwd(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn zsysdep_add_cwd(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn fsysdep_needs_cwd(zfile: *const libc::c_char) -> boolean;
    fn zsysdep_base_name(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn ixsysdep_user_file_mode(zfile: *const libc::c_char) -> libc::c_uint;
    fn fsysdep_access(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_daemon_access(zfile: *const libc::c_char) -> boolean;
    fn zsysdep_uuto(
        zdest: *const libc::c_char,
        zlocalname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
    fn usysdep_walk_tree(
        zdir: *const libc::c_char,
        pufn: Option::<
            unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, pointer) -> (),
        >,
        pinfo: pointer,
    ) -> boolean;
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
pub struct sjob {
    pub qnext: *mut sjob,
    pub qsys: *const uuconf_system,
    pub ccmds: libc::c_int,
    pub pascmds: *mut scmd,
    pub pazlogs: *mut *const libc::c_char,
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
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
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
pub type _argtype = libc::c_uint;
pub const optional_argument: _argtype = 2;
pub static mut uucp_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: uucp.c,v 1.76 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asClongopts: [option; 19] = [
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
            name: b"nocopy\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"directories\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nodirectories\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
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
            name: b"mail\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"notify\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
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
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
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
            name: b"uuto\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
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
static mut pCuuconf: pointer = 0 as *const libc::c_void as *mut libc::c_void;
static mut fCcopy: boolean = 1 as libc::c_int;
static mut bCgrade: libc::c_char = 'N' as i32 as libc::c_char;
static mut fCmail: boolean = 0 as libc::c_int;
static mut zCnotify: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
static mut fCexpand: boolean = 1 as libc::c_int;
static mut fCmkdirs: boolean = 1 as libc::c_int;
static mut zClocalname: *const libc::c_char = 0 as *const libc::c_char;
static mut zCuser: *const libc::c_char = 0 as *const libc::c_char;
static mut fCremote: boolean = 0 as libc::c_int;
static mut fClocaldest: boolean = 0;
static mut sCdestsys: uuconf_system = uuconf_system {
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
static mut zCforward: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut abCsend_options: [libc::c_char; 20] = [0; 20];
static mut abCrec_options: [libc::c_char; 20] = [0; 20];
static mut fCneeds_cwd: boolean = 0;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut fjobid: boolean = 0 as libc::c_int;
    let mut fuucico: boolean = 1 as libc::c_int;
    let mut frecursive: boolean = 0 as libc::c_int;
    let mut zstatus_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut fuuto: boolean = 0 as libc::c_int;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut fgetcwd: boolean = 0;
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
    let mut zexclam: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zdestfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zdestsys: *const libc::c_char = 0 as *const libc::c_char;
    let mut zoptions: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fexit: boolean = 0;
    zProgram = *argv.offset(0 as libc::c_int as isize);
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"cCdfg:I:jmn:prRs:tu:Wvx:\0" as *const u8 as *const libc::c_char,
            asClongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            99 => {
                fCcopy = 0 as libc::c_int;
            }
            112 | 67 => {
                fCcopy = 1 as libc::c_int;
            }
            100 => {
                fCmkdirs = 1 as libc::c_int;
            }
            102 => {
                fCmkdirs = 0 as libc::c_int;
            }
            103 => {
                bCgrade = *gnu_optarg.offset(0 as libc::c_int as isize);
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            106 => {
                fjobid = 1 as libc::c_int;
            }
            109 => {
                fCmail = 1 as libc::c_int;
            }
            110 => {
                zCnotify = gnu_optarg;
            }
            114 => {
                fuucico = 0 as libc::c_int;
            }
            82 => {
                frecursive = 1 as libc::c_int;
            }
            115 => {
                zstatus_file = gnu_optarg;
            }
            116 => {
                fuuto = 1 as libc::c_int;
            }
            117 => {
                zCuser = gnu_optarg;
            }
            87 => {
                fCexpand = 0 as libc::c_int;
            }
            120 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            118 => {
                printf(
                    b"uucp (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
                uchelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                ucusage();
            }
        }
    }
    if *(*__ctype_b_loc()).offset(bCgrade as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
        || ((bCgrade as libc::c_int) < '0' as i32 || bCgrade as libc::c_int > '9' as i32)
            && ((bCgrade as libc::c_int) < 'a' as i32
                || bCgrade as libc::c_int > 'z' as i32)
            && ((bCgrade as libc::c_int) < 'A' as i32
                || bCgrade as libc::c_int > 'Z' as i32)
    {
        ulog(LOG_ERROR, b"Ignoring illegal grade\0" as *const u8 as *const libc::c_char);
        bCgrade = 'N' as i32 as libc::c_char;
    }
    if !zCuser.is_null() {
        if !(strchr(zCuser, '!' as i32)).is_null() {
            fCremote = 1 as libc::c_int;
        } else {
            ulog(
                LOG_ERROR,
                b"Ignoring local user name\0" as *const u8 as *const libc::c_char,
            );
            zCuser = 0 as *const libc::c_char;
        }
    }
    if argc - gnu_optind < 2 as libc::c_int {
        ucusage();
    }
    iuuconf = uuconf_init(
        &mut puuconf,
        0 as *mut libc::c_void as *const libc::c_char,
        zconfig,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    pCuuconf = puuconf;
    let mut zdebug: *const libc::c_char = 0 as *const libc::c_char;
    iuuconf = uuconf_debuglevel(puuconf, &mut zdebug);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    if !zdebug.is_null() {
        iDebug |= idebug_parse(zdebug);
    }
    fgetcwd = 0 as libc::c_int;
    i = gnu_optind;
    while i < argc {
        zexclam = strrchr(*argv.offset(i as isize), '!' as i32);
        if zexclam.is_null() {
            zexclam = *argv.offset(i as isize);
        } else {
            zexclam = zexclam.offset(1);
            zexclam;
        }
        if fsysdep_needs_cwd(zexclam) != 0 {
            fgetcwd = 1 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
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
    ulog_fatal_fn(Some(ucabort as unsafe extern "C" fn() -> ()));
    if zCuser.is_null() {
        zCuser = zsysdep_login_name();
    }
    iuuconf = uuconf_localname(puuconf, &mut zClocalname);
    if iuuconf == 1 as libc::c_int {
        zClocalname = zsysdep_localname();
        if zClocalname.is_null() {
            exit(1 as libc::c_int);
        }
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_system_info(puuconf, zClocalname, &mut slocalsys);
    if iuuconf != 0 as libc::c_int {
        if iuuconf != 1 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        iuuconf = uuconf_system_local(puuconf, &mut slocalsys);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        slocalsys.uuconf_zname = zClocalname as *mut libc::c_char;
    }
    if fuuto != 0 {
        if *zCnotify as libc::c_int == '\0' as i32 {
            zexclam = strrchr(
                *argv.offset((argc - 1 as libc::c_int) as isize),
                '!' as i32,
            );
            if zexclam.is_null() {
                ucusage();
            }
            zCnotify = zexclam.offset(1 as libc::c_int as isize);
        }
        let ref mut fresh0 = *argv.offset((argc - 1 as libc::c_int) as isize);
        *fresh0 = zsysdep_uuto(
            *argv.offset((argc - 1 as libc::c_int) as isize),
            zClocalname,
        );
        if (*argv.offset((argc - 1 as libc::c_int) as isize)).is_null() {
            ucusage();
        }
    }
    zoptions = abCsend_options.as_mut_ptr();
    if fCcopy != 0 {
        let fresh1 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh1 = 'C' as i32 as libc::c_char;
    } else {
        let fresh2 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh2 = 'c' as i32 as libc::c_char;
    }
    if fCmkdirs != 0 {
        let fresh3 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh3 = 'd' as i32 as libc::c_char;
    } else {
        let fresh4 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh4 = 'f' as i32 as libc::c_char;
    }
    if fCmail != 0 {
        let fresh5 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh5 = 'm' as i32 as libc::c_char;
    }
    if *zCnotify as libc::c_int != '\0' as i32 {
        let fresh6 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh6 = 'n' as i32 as libc::c_char;
    }
    *zoptions = '\0' as i32 as libc::c_char;
    zoptions = abCrec_options.as_mut_ptr();
    if fCmkdirs != 0 {
        let fresh7 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh7 = 'd' as i32 as libc::c_char;
    } else {
        let fresh8 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh8 = 'f' as i32 as libc::c_char;
    }
    if fCmail != 0 {
        let fresh9 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh9 = 'm' as i32 as libc::c_char;
    }
    *zoptions = '\0' as i32 as libc::c_char;
    let ref mut fresh10 = *argv.offset((argc - 1 as libc::c_int) as isize);
    *fresh10 = zremove_local_sys(
        &mut slocalsys,
        *argv.offset((argc - 1 as libc::c_int) as isize),
    );
    zexclam = strchr(*argv.offset((argc - 1 as libc::c_int) as isize), '!' as i32);
    if zexclam.is_null() {
        zdestsys = zClocalname;
        zdestfile = *argv.offset((argc - 1 as libc::c_int) as isize);
        fClocaldest = 1 as libc::c_int;
    } else {
        let mut clen: size_t = 0;
        let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
        clen = zexclam.offset_from(*argv.offset((argc - 1 as libc::c_int) as isize))
            as libc::c_long as size_t;
        zcopy = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
        memcpy(
            zcopy as *mut libc::c_void,
            *argv.offset((argc - 1 as libc::c_int) as isize) as *const libc::c_void,
            clen,
        );
        *zcopy.offset(clen as isize) = '\0' as i32 as libc::c_char;
        zdestsys = zcopy;
        zdestfile = zexclam.offset(1 as libc::c_int as isize);
        fClocaldest = 0 as libc::c_int;
    }
    iuuconf = uuconf_system_info(puuconf, zdestsys, &mut sCdestsys);
    if iuuconf != 0 as libc::c_int {
        if iuuconf != 1 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        if fClocaldest != 0 {
            iuuconf = uuconf_system_local(puuconf, &mut sCdestsys);
            if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            sCdestsys.uuconf_zname = zClocalname as *mut libc::c_char;
        } else if funknown_system(puuconf, zdestsys, &mut sCdestsys) == 0 {
            ulog(
                LOG_FATAL,
                b"%s: System not found\0" as *const u8 as *const libc::c_char,
                zdestsys,
            );
        }
    }
    zexclam = strrchr(zdestfile, '!' as i32);
    if zexclam.is_null() {
        zCforward = 0 as *mut libc::c_char;
    } else {
        let mut clen_0: size_t = 0;
        if fClocaldest != 0 {
            ulog(LOG_FATAL, b"Can't happen\0" as *const u8 as *const libc::c_char);
        }
        clen_0 = zexclam.offset_from(zdestfile) as libc::c_long as size_t;
        zCforward = zbufalc(clen_0.wrapping_add(1 as libc::c_int as libc::c_ulong));
        memcpy(zCforward as *mut libc::c_void, zdestfile as *const libc::c_void, clen_0);
        *zCforward.offset(clen_0 as isize) = '\0' as i32 as libc::c_char;
        zdestfile = zexclam.offset(1 as libc::c_int as isize);
    }
    if fClocaldest != 0 {
        zdestfile = zsysdep_local_file_cwd(
            zdestfile,
            sCdestsys.uuconf_zpubdir,
            0 as *mut libc::c_void as *mut boolean,
        );
    } else if fCexpand != 0 {
        zdestfile = zsysdep_add_cwd(zdestfile);
    }
    if zdestfile.is_null() {
        ulog_close();
        usysdep_exit(0 as libc::c_int);
    }
    i = gnu_optind;
    while i < argc - 1 as libc::c_int
        && !(afSignal[0 as libc::c_int as usize] != 0
            || afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0)
    {
        let mut flocal: boolean = 0;
        let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
        fCneeds_cwd = 0 as libc::c_int;
        let ref mut fresh11 = *argv.offset(i as isize);
        *fresh11 = zremove_local_sys(&mut slocalsys, *argv.offset(i as isize));
        if !(strchr(*argv.offset(i as isize), '!' as i32)).is_null() {
            flocal = 0 as libc::c_int;
            zfrom = zbufcpy(*argv.offset(i as isize));
        } else {
            flocal = 1 as libc::c_int;
            if fsysdep_needs_cwd(*argv.offset(i as isize)) != 0 {
                fCneeds_cwd = 1 as libc::c_int;
            }
            zfrom = zsysdep_local_file_cwd(
                *argv.offset(i as isize),
                sCdestsys.uuconf_zpubdir,
                0 as *mut libc::c_void as *mut boolean,
            );
            if zfrom.is_null() {
                ucabort();
            }
        }
        if flocal == 0 || fsysdep_directory(zfrom) == 0 {
            uccopy(zfrom, zdestfile, 0 as libc::c_int);
        } else {
            let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zindir: *mut libc::c_char = 0 as *mut libc::c_char;
            if frecursive == 0 {
                ulog(
                    LOG_FATAL,
                    b"%s: directory without -R\0" as *const u8 as *const libc::c_char,
                    zfrom,
                );
            }
            zbase = zsysdep_base_name(zfrom);
            if zbase.is_null() {
                ucabort();
            }
            zindir = zsysdep_in_dir(zdestfile, zbase);
            ubuffree(zbase);
            if zindir.is_null() {
                ucabort();
            }
            usysdep_walk_tree(
                zfrom,
                Some(
                    ucdirfile
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            pointer,
                        ) -> (),
                ),
                zindir as pointer,
            );
            ubuffree(zindir);
        }
        ubuffree(zfrom);
        i += 1;
        i;
    }
    if afSignal[0 as libc::c_int as usize] != 0
        || afSignal[1 as libc::c_int as usize] != 0
        || afSignal[2 as libc::c_int as usize] != 0
        || afSignal[3 as libc::c_int as usize] != 0
        || afSignal[4 as libc::c_int as usize] != 0
    {
        ucabort();
    }
    ulog_to_file(puuconf, 1 as libc::c_int);
    ulog_user(zCuser);
    ucspool_cmds(fjobid);
    ulog_close();
    if fuucico == 0 {
        fexit = 1 as libc::c_int;
    } else {
        let mut zsys: *const libc::c_char = 0 as *const libc::c_char;
        let mut fany: boolean = 0;
        zsys = zcone_system(&mut fany);
        if zsys.is_null() && fany == 0 {
            fexit = 1 as libc::c_int;
        } else {
            let mut zarg: *const libc::c_char = 0 as *const libc::c_char;
            let mut zconfigarg: *mut libc::c_char = 0 as *mut libc::c_char;
            if zsys.is_null() {
                zarg = b"-r1\0" as *const u8 as *const libc::c_char;
            } else {
                let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                z = zbufalc(
                    (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                        .wrapping_add(strlen(zsys)),
                );
                sprintf(z, b"-Cs%s\0" as *const u8 as *const libc::c_char, zsys);
                zarg = z;
            }
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
            fexit = fsysdep_run(
                0 as libc::c_int,
                b"uucico\0" as *const u8 as *const libc::c_char,
                zarg,
                zconfigarg,
            );
        }
    }
    usysdep_exit(fexit);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ucusage() {
    fprintf(
        stderr,
        b"Usage: %s [options] file1 [file2 ...] dest\n\0" as *const u8
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
unsafe extern "C" fn uchelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Usage: %s [options] file1 [file2 ...] dest\n\0" as *const u8
            as *const libc::c_char,
        zProgram,
    );
    printf(
        b" -c,--nocopy: Do not copy local files to spool directory\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -C,-p,--copy: Copy local files to spool directory (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -d,--directories: Create necessary directories (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -f,--nodirectories: Do not create directories (fail if they do not exist)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -g,--grade grade: Set job grade (must be alphabetic)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -m,--mail: Report status of copy by mail\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -n,--notify user: Report status of copy by mail to remote user\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -R,--recursive: Copy directories recursively\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -r,--nouucico: Do not start uucico daemon\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -s,--status file: Report completion status to file\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b" -j,--jobid: Report job id\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -W,--noexpand: Do not add current directory to remote filenames\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b" -t,--uuto: Emulate uuto\n\0" as *const u8 as *const libc::c_char);
    printf(b" -u,--user name: Set user name\n\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn ucdirfile(
    mut zfull: *const libc::c_char,
    mut zrelative: *const libc::c_char,
    mut pinfo: pointer,
) {
    let mut zdestfile: *const libc::c_char = pinfo as *const libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    zto = zsysdep_in_dir(zdestfile, zrelative);
    if zto.is_null() {
        ucabort();
    }
    uccopy(zfull, zto, 1 as libc::c_int);
    ubuffree(zto);
}
unsafe extern "C" fn uccopy(
    mut zfile: *const libc::c_char,
    mut zdest: *const libc::c_char,
    mut fforcelocal: boolean,
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
    let mut zexclam: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    zexclam = strchr(zfile, '!' as i32);
    if zexclam.is_null() || fforcelocal != 0 {
        let mut efrom: openfile_t = 0 as *mut FILE;
        if fsysdep_access(zfile) == 0 {
            ucabort();
        }
        if fCremote != 0 && fCneeds_cwd == 0
            && fin_directory_list(
                zfile,
                sCdestsys.uuconf_pzremote_send,
                sCdestsys.uuconf_zpubdir,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as *mut libc::c_void as *const libc::c_char,
            ) == 0
        {
            ulog(
                LOG_FATAL,
                b"Not permitted to send %s\0" as *const u8 as *const libc::c_char,
                zfile,
            );
        }
        if fClocaldest != 0 {
            let mut fok: boolean = 0;
            let mut imode: libc::c_uint = 0;
            if fCremote != 0 {
                fok = fin_directory_list(
                    zdest,
                    sCdestsys.uuconf_pzremote_receive,
                    sCdestsys.uuconf_zpubdir,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *const libc::c_char,
                );
            } else {
                fok = fin_directory_list(
                    zdest,
                    sCdestsys.uuconf_pzlocal_receive,
                    sCdestsys.uuconf_zpubdir,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    zCuser,
                );
            }
            if fok == 0 {
                ulog(
                    LOG_FATAL,
                    b"Not permitted to receive to %s\0" as *const u8
                        as *const libc::c_char,
                    zdest,
                );
            }
            zto = zsysdep_add_base(zdest, zfile);
            if zto.is_null() {
                ucabort();
            }
            efrom = esysdep_user_fopen(zfile, 1 as libc::c_int, 1 as libc::c_int);
            if efrom.is_null() {
                ucabort();
            }
            if fcopy_open_file(efrom, zto, 0 as libc::c_int, fCmkdirs, 1 as libc::c_int)
                == 0
            {
                ucabort();
            }
            fclose(efrom);
            ubuffree(zto);
            imode = ixsysdep_user_file_mode(zfile);
            if imode != 0 as libc::c_int as libc::c_uint {
                fsysdep_change_mode(zto, imode);
            }
        } else {
            let mut zloc: *const libc::c_char = 0 as *const libc::c_char;
            let mut abtname: [libc::c_char; 15] = [0; 15];
            let mut imode_0: libc::c_uint = 0;
            let mut ztemp: *mut libc::c_char = 0 as *mut libc::c_char;
            imode_0 = ixsysdep_user_file_mode(zfile);
            if imode_0 == 0 as libc::c_int as libc::c_uint {
                ucabort();
            }
            zloc = sCdestsys.uuconf_zlocalname;
            if zloc.is_null() {
                zloc = zClocalname;
            }
            ztemp = zsysdep_data_file_name(
                &mut sCdestsys,
                zloc,
                bCgrade as libc::c_int,
                0 as libc::c_int,
                abtname.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            if ztemp.is_null() {
                ucabort();
            }
            if fCcopy == 0 {
                ubuffree(ztemp);
                if fsysdep_daemon_access(zfile) == 0 {
                    ucabort();
                }
                if fin_directory_list(
                    zfile,
                    sCdestsys.uuconf_pzlocal_send,
                    sCdestsys.uuconf_zpubdir,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    if fCremote != 0 {
                        0 as *mut libc::c_void as *const libc::c_char
                    } else {
                        zCuser
                    },
                ) == 0
                {
                    ulog(
                        LOG_FATAL,
                        b"Daemon not permitted to send %s (suggest --copy)\0"
                            as *const u8 as *const libc::c_char,
                        zfile,
                    );
                }
            } else {
                efrom = esysdep_user_fopen(zfile, 1 as libc::c_int, 1 as libc::c_int);
                if efrom.is_null() {
                    ucabort();
                }
                ucrecord_file(ztemp);
                if fcopy_open_file(
                    efrom,
                    ztemp,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                ) == 0
                {
                    ucabort();
                }
                fclose(efrom);
            }
            if zCforward.is_null() {
                s.bcmd = 'S' as i32 as libc::c_char;
                s.bgrade = bCgrade;
                s.pseq = 0 as *mut libc::c_void;
                s.zfrom = zbufcpy(zfile);
                s.zto = zbufcpy(zdest);
                s.zuser = zCuser;
                s.zoptions = abCsend_options.as_mut_ptr();
                s.ztemp = zbufcpy(abtname.as_mut_ptr());
                s.imode = imode_0;
                s.znotify = zCnotify;
                s.cbytes = -(1 as libc::c_int) as libc::c_long;
                s.zcmd = 0 as *const libc::c_char;
                s.ipos = 0 as libc::c_int as libc::c_long;
                ucadd_cmd(
                    &mut sCdestsys,
                    &mut s,
                    0 as *mut libc::c_void as *const libc::c_char,
                );
            } else {
                let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut zxqt: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut abxtname: [libc::c_char; 15] = [0; 15];
                let mut abdname: [libc::c_char; 15] = [0; 15];
                let mut abxname: [libc::c_char; 15] = [0; 15];
                let mut e: *mut FILE = 0 as *mut FILE;
                let mut zlog: *mut libc::c_char = 0 as *mut libc::c_char;
                zbase = zsysdep_base_name(zfile);
                if zbase.is_null() {
                    ucabort();
                }
                zxqt = zsysdep_data_file_name(
                    &mut sCdestsys,
                    zloc,
                    bCgrade as libc::c_int,
                    1 as libc::c_int,
                    abxtname.as_mut_ptr(),
                    abdname.as_mut_ptr(),
                    abxname.as_mut_ptr(),
                );
                if zxqt.is_null() {
                    ucabort();
                }
                e = esysdep_fopen(
                    zxqt,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                if e.is_null() {
                    ucabort();
                }
                ucrecord_file(zxqt);
                fprintf(
                    e,
                    b"U %s %s\n\0" as *const u8 as *const libc::c_char,
                    zCuser,
                    zloc,
                );
                fprintf(
                    e,
                    b"F %s %s\n\0" as *const u8 as *const libc::c_char,
                    abdname.as_mut_ptr(),
                    zbase,
                );
                fprintf(e, b"C uucp -C\0" as *const u8 as *const libc::c_char);
                if fCmkdirs != 0 {
                    fprintf(e, b" -d\0" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(e, b" -f\0" as *const u8 as *const libc::c_char);
                }
                fprintf(
                    e,
                    b" -g %c\0" as *const u8 as *const libc::c_char,
                    bCgrade as libc::c_int,
                );
                if fCmail != 0 {
                    fprintf(e, b" -m\0" as *const u8 as *const libc::c_char);
                }
                if *zCnotify as libc::c_int != '\0' as i32 {
                    fprintf(
                        e,
                        b" -n %s\0" as *const u8 as *const libc::c_char,
                        zCnotify,
                    );
                }
                if fCexpand == 0 {
                    fprintf(e, b" -W\0" as *const u8 as *const libc::c_char);
                }
                fprintf(
                    e,
                    b" %s %s!%s\n\0" as *const u8 as *const libc::c_char,
                    zbase,
                    zCforward,
                    zdest,
                );
                ubuffree(zbase);
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
                s.bcmd = 'S' as i32 as libc::c_char;
                s.bgrade = bCgrade;
                s.pseq = 0 as *mut libc::c_void;
                s.zfrom = zbufcpy(abxtname.as_mut_ptr());
                s.zto = zbufcpy(abxname.as_mut_ptr());
                s.zuser = zCuser;
                s.zoptions = b"C\0" as *const u8 as *const libc::c_char;
                s.ztemp = s.zfrom;
                s.imode = 0o666 as libc::c_int as libc::c_uint;
                s.znotify = 0 as *const libc::c_char;
                s.cbytes = -(1 as libc::c_int) as libc::c_long;
                s.zcmd = 0 as *const libc::c_char;
                s.ipos = 0 as libc::c_int as libc::c_long;
                zlog = zbufalc(
                    (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                        .wrapping_add(strlen(zfile))
                        .wrapping_add(strlen(zCforward))
                        .wrapping_add(strlen(zdest)),
                );
                sprintf(
                    zlog,
                    b"Queuing uucp %s %s!%s\0" as *const u8 as *const libc::c_char,
                    zfile,
                    zCforward,
                    zdest,
                );
                ucadd_cmd(&mut sCdestsys, &mut s, zlog);
                s.bcmd = 'S' as i32 as libc::c_char;
                s.bgrade = bCgrade;
                s.pseq = 0 as *mut libc::c_void;
                s.zfrom = zbufcpy(zfile);
                s.zto = zbufcpy(abdname.as_mut_ptr());
                s.zuser = zCuser;
                s
                    .zoptions = if fCcopy != 0 {
                    b"C\0" as *const u8 as *const libc::c_char
                } else {
                    b"c\0" as *const u8 as *const libc::c_char
                };
                s.ztemp = zbufcpy(abtname.as_mut_ptr());
                s.imode = 0o666 as libc::c_int as libc::c_uint;
                s.znotify = 0 as *const libc::c_char;
                s.cbytes = -(1 as libc::c_int) as libc::c_long;
                s.zcmd = 0 as *const libc::c_char;
                s.ipos = 0 as libc::c_int as libc::c_long;
                ucadd_cmd(
                    &mut sCdestsys,
                    &mut s,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    } else {
        let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zforward: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut clen: size_t = 0;
        let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut qfromsys: *mut uuconf_system = 0 as *mut uuconf_system;
        let mut iuuconf: libc::c_int = 0;
        let mut zloc_0: *const libc::c_char = 0 as *const libc::c_char;
        zfrom = strrchr(zfile, '!' as i32);
        if zfrom == zexclam {
            zforward = 0 as *mut libc::c_char;
        } else {
            clen = (zfrom.offset_from(zexclam) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t;
            zforward = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
            memcpy(
                zforward as *mut libc::c_void,
                zexclam.offset(1 as libc::c_int as isize) as *const libc::c_void,
                clen,
            );
            *zforward.offset(clen as isize) = '\0' as i32 as libc::c_char;
        }
        zfrom = zfrom.offset(1);
        zfrom;
        if fCexpand != 0 {
            zfrom = zsysdep_add_cwd(zfrom);
            if zfrom.is_null() {
                ucabort();
            }
        }
        clen = zexclam.offset_from(zfile) as libc::c_long as size_t;
        zcopy = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
        memcpy(zcopy as *mut libc::c_void, zfile as *const libc::c_void, clen);
        *zcopy.offset(clen as isize) = '\0' as i32 as libc::c_char;
        qfromsys = xmalloc(::std::mem::size_of::<uuconf_system>() as libc::c_ulong)
            as *mut uuconf_system;
        iuuconf = uuconf_system_info(pCuuconf, zcopy, qfromsys);
        if iuuconf == 1 as libc::c_int {
            if funknown_system(pCuuconf, zcopy, qfromsys) == 0 {
                ulog(
                    LOG_FATAL,
                    b"%s: System not found\0" as *const u8 as *const libc::c_char,
                    zcopy,
                );
            }
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_FATAL, pCuuconf, iuuconf);
        }
        ubuffree(zcopy);
        zloc_0 = (*qfromsys).uuconf_zlocalname;
        if zloc_0.is_null() {
            zloc_0 = zClocalname;
        }
        if zforward.is_null() && fClocaldest != 0 {
            let mut fok_0: boolean = 0;
            if fCremote != 0 {
                fok_0 = fin_directory_list(
                    zdest,
                    (*qfromsys).uuconf_pzremote_receive,
                    (*qfromsys).uuconf_zpubdir,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *const libc::c_char,
                );
            } else {
                fok_0 = fin_directory_list(
                    zdest,
                    (*qfromsys).uuconf_pzlocal_receive,
                    (*qfromsys).uuconf_zpubdir,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    zCuser,
                );
            }
            if fok_0 == 0 {
                ulog(
                    LOG_FATAL,
                    b"Not permitted to receive to %s\0" as *const u8
                        as *const libc::c_char,
                    zdest,
                );
            }
            if *zfrom
                .offset(
                    strcspn(zfrom, b"*?[\0" as *const u8 as *const libc::c_char) as isize,
                ) as libc::c_int != '\0' as i32
            {
                s.bcmd = 'X' as i32 as libc::c_char;
                zto = zbufalc(
                    (strlen(zloc_0))
                        .wrapping_add(strlen(zdest))
                        .wrapping_add(
                            ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                        ),
                );
                sprintf(
                    zto,
                    b"%s!%s\0" as *const u8 as *const libc::c_char,
                    zloc_0,
                    zdest,
                );
            } else {
                s.bcmd = 'R' as i32 as libc::c_char;
                zto = zbufcpy(zdest);
            }
            s.bgrade = bCgrade;
            s.pseq = 0 as *mut libc::c_void;
            s.zfrom = zfrom;
            s.zto = zto;
            s.zuser = zCuser;
            s.zoptions = abCrec_options.as_mut_ptr();
            s.ztemp = b"\0" as *const u8 as *const libc::c_char;
            s.imode = 0 as libc::c_int as libc::c_uint;
            s.znotify = b"\0" as *const u8 as *const libc::c_char;
            s.cbytes = -(1 as libc::c_int) as libc::c_long;
            s.zcmd = 0 as *const libc::c_char;
            s.ipos = 0 as libc::c_int as libc::c_long;
            ucadd_cmd(qfromsys, &mut s, 0 as *mut libc::c_void as *const libc::c_char);
        } else {
            let mut zxqt_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut abtname_0: [libc::c_char; 15] = [0; 15];
            let mut abxname_0: [libc::c_char; 15] = [0; 15];
            let mut e_0: *mut FILE = 0 as *mut FILE;
            let mut zcmd: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zlog_0: *mut libc::c_char = 0 as *mut libc::c_char;
            zxqt_0 = zsysdep_data_file_name(
                qfromsys,
                zloc_0,
                bCgrade as libc::c_int,
                1 as libc::c_int,
                abtname_0.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_char,
                abxname_0.as_mut_ptr(),
            );
            if zxqt_0.is_null() {
                ucabort();
            }
            e_0 = esysdep_fopen(
                zxqt_0,
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            if e_0.is_null() {
                ucabort();
            }
            ucrecord_file(zxqt_0);
            fprintf(
                e_0,
                b"U %s %s\n\0" as *const u8 as *const libc::c_char,
                zCuser,
                zloc_0,
            );
            fprintf(e_0, b"C uucp -C\0" as *const u8 as *const libc::c_char);
            if fCmkdirs != 0 {
                fprintf(e_0, b" -d\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(e_0, b" -f\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                e_0,
                b" -g %c\0" as *const u8 as *const libc::c_char,
                bCgrade as libc::c_int,
            );
            if fCmail != 0 {
                fprintf(e_0, b" -m\0" as *const u8 as *const libc::c_char);
            }
            if *zCnotify as libc::c_int != '\0' as i32 {
                fprintf(e_0, b" -n %s\0" as *const u8 as *const libc::c_char, zCnotify);
            }
            if fCexpand == 0 {
                fprintf(e_0, b" -W\0" as *const u8 as *const libc::c_char);
            }
            clen = (strlen(zfrom))
                .wrapping_add(strlen(zloc_0))
                .wrapping_add(strlen(sCdestsys.uuconf_zname))
                .wrapping_add(strlen(zdest));
            if !zforward.is_null() {
                clen = (clen as libc::c_ulong).wrapping_add(strlen(zforward)) as size_t
                    as size_t;
            }
            if !zCforward.is_null() {
                clen = (clen as libc::c_ulong).wrapping_add(strlen(zCforward)) as size_t
                    as size_t;
            }
            zcmd = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_add(clen),
            );
            *zcmd = '\0' as i32 as libc::c_char;
            if !zforward.is_null() {
                sprintf(
                    zcmd.offset(strlen(zcmd) as isize),
                    b"%s!\0" as *const u8 as *const libc::c_char,
                    zforward,
                );
            }
            sprintf(
                zcmd.offset(strlen(zcmd) as isize),
                b"%s %s!\0" as *const u8 as *const libc::c_char,
                zfrom,
                zloc_0,
            );
            if fClocaldest == 0 {
                sprintf(
                    zcmd.offset(strlen(zcmd) as isize),
                    b"%s!\0" as *const u8 as *const libc::c_char,
                    sCdestsys.uuconf_zname,
                );
            }
            if !zCforward.is_null() {
                sprintf(
                    zcmd.offset(strlen(zcmd) as isize),
                    b"%s!\0" as *const u8 as *const libc::c_char,
                    zCforward,
                );
            }
            sprintf(
                zcmd.offset(strlen(zcmd) as isize),
                b"%s\0" as *const u8 as *const libc::c_char,
                zdest,
            );
            fprintf(e_0, b" %s\n\0" as *const u8 as *const libc::c_char, zcmd);
            if fsysdep_sync(e_0, zxqt_0) == 0 {
                ulog(LOG_FATAL, b"fsync failed\0" as *const u8 as *const libc::c_char);
            }
            if fclose(e_0) != 0 as libc::c_int {
                ulog(
                    LOG_FATAL,
                    b"fclose: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            s.bcmd = 'S' as i32 as libc::c_char;
            s.bgrade = bCgrade;
            s.pseq = 0 as *mut libc::c_void;
            s.zfrom = zbufcpy(abtname_0.as_mut_ptr());
            s.zto = zbufcpy(abxname_0.as_mut_ptr());
            s.zuser = zCuser;
            s.zoptions = b"C\0" as *const u8 as *const libc::c_char;
            s.ztemp = s.zfrom;
            s.imode = 0o666 as libc::c_int as libc::c_uint;
            s.znotify = 0 as *const libc::c_char;
            s.cbytes = -(1 as libc::c_int) as libc::c_long;
            s.zcmd = 0 as *const libc::c_char;
            s.ipos = 0 as libc::c_int as libc::c_long;
            zlog_0 = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_add(strlen(zcmd)),
            );
            sprintf(
                zlog_0,
                b"Queueing uucp %s\0" as *const u8 as *const libc::c_char,
                zcmd,
            );
            ucadd_cmd(qfromsys, &mut s, zlog_0);
            ubuffree(zcmd);
            ubuffree(zforward);
        }
    };
}
static mut qCjobs: *mut sjob = 0 as *const sjob as *mut sjob;
unsafe extern "C" fn ucadd_cmd(
    mut qsys: *const uuconf_system,
    mut qcmd: *const scmd,
    mut zlog: *const libc::c_char,
) {
    let mut qjob: *mut sjob = 0 as *mut sjob;
    if (*qsys).uuconf_fcall_transfer == 0 && (*qsys).uuconf_fcalled_transfer == 0 {
        ulog(
            LOG_FATAL,
            b"Not permitted to transfer files to or from %s\0" as *const u8
                as *const libc::c_char,
            (*qsys).uuconf_zname,
        );
    }
    qjob = qCjobs;
    while !qjob.is_null() {
        if strcmp((*(*qjob).qsys).uuconf_zname, (*qsys).uuconf_zname) == 0 as libc::c_int
        {
            break;
        }
        qjob = (*qjob).qnext;
    }
    if qjob.is_null() {
        qjob = xmalloc(::std::mem::size_of::<sjob>() as libc::c_ulong) as *mut sjob;
        (*qjob).qnext = qCjobs;
        (*qjob).qsys = qsys;
        (*qjob).ccmds = 0 as libc::c_int;
        (*qjob).pascmds = 0 as *mut scmd;
        (*qjob).pazlogs = 0 as *mut *const libc::c_char;
        qCjobs = qjob;
    }
    (*qjob)
        .pascmds = xrealloc(
        (*qjob).pascmds as pointer,
        (((*qjob).ccmds + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<scmd>() as libc::c_ulong),
    ) as *mut scmd;
    *((*qjob).pascmds).offset((*qjob).ccmds as isize) = *qcmd;
    (*qjob)
        .pazlogs = xrealloc(
        (*qjob).pazlogs as pointer,
        (((*qjob).ccmds + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    let ref mut fresh12 = *((*qjob).pazlogs).offset((*qjob).ccmds as isize);
    *fresh12 = zlog;
    (*qjob).ccmds += 1;
    (*qjob).ccmds;
}
unsafe extern "C" fn ucspool_cmds(mut fjobid: boolean) {
    let mut qjob: *mut sjob = 0 as *mut sjob;
    let mut zjobid: *mut libc::c_char = 0 as *mut libc::c_char;
    qjob = qCjobs;
    while !qjob.is_null() {
        ulog_system((*(*qjob).qsys).uuconf_zname);
        zjobid = zsysdep_spool_commands(
            (*qjob).qsys,
            bCgrade as libc::c_int,
            (*qjob).ccmds,
            (*qjob).pascmds,
            0 as *mut libc::c_void as *mut boolean,
        );
        if !zjobid.is_null() {
            let mut i: libc::c_int = 0;
            let mut qcmd: *mut scmd = 0 as *mut scmd;
            let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
            i = 0 as libc::c_int;
            qcmd = (*qjob).pascmds;
            pz = (*qjob).pazlogs;
            while i < (*qjob).ccmds {
                if !(*pz).is_null() {
                    if **pz as libc::c_int != '\0' as i32 {
                        ulog(
                            LOG_NORMAL,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            *pz,
                        );
                    }
                } else if (*qcmd).bcmd as libc::c_int == 'S' as i32 {
                    ulog(
                        LOG_NORMAL,
                        b"Queuing send of %s to %s\0" as *const u8
                            as *const libc::c_char,
                        (*qcmd).zfrom,
                        (*qcmd).zto,
                    );
                } else if (*qcmd).bcmd as libc::c_int == 'R' as i32 {
                    ulog(
                        LOG_NORMAL,
                        b"Queuing request of %s to %s\0" as *const u8
                            as *const libc::c_char,
                        (*qcmd).zfrom,
                        (*qcmd).zto,
                    );
                } else {
                    let mut zto: *const libc::c_char = 0 as *const libc::c_char;
                    zto = strrchr((*qcmd).zto, '!' as i32);
                    if !zto.is_null() {
                        zto = zto.offset(1);
                        zto;
                    } else {
                        zto = (*qcmd).zto;
                    }
                    ulog(
                        LOG_NORMAL,
                        b"Queuing request of %s to %s\0" as *const u8
                            as *const libc::c_char,
                        (*qcmd).zfrom,
                        zto,
                    );
                }
                i += 1;
                i;
                qcmd = qcmd.offset(1);
                qcmd;
                pz = pz.offset(1);
                pz;
            }
            if fjobid != 0 {
                printf(b"%s\n\0" as *const u8 as *const libc::c_char, zjobid);
            }
            ubuffree(zjobid);
        }
        qjob = (*qjob).qnext;
    }
}
unsafe extern "C" fn zcone_system(mut pfany: *mut boolean) -> *const libc::c_char {
    if qCjobs.is_null() {
        *pfany = 0 as libc::c_int;
        return 0 as *const libc::c_char;
    }
    *pfany = 1 as libc::c_int;
    if ((*qCjobs).qnext).is_null() {
        return (*(*qCjobs).qsys).uuconf_zname
    } else {
        return 0 as *const libc::c_char
    };
}
static mut cCfiles: libc::c_int = 0;
static mut pCaz: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
unsafe extern "C" fn ucrecord_file(mut zfile: *const libc::c_char) {
    pCaz = xrealloc(
        pCaz as pointer,
        ((cCfiles + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    let ref mut fresh13 = *pCaz.offset(cCfiles as isize);
    *fresh13 = zfile;
    cCfiles += 1;
    cCfiles;
}
unsafe extern "C" fn ucabort() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < cCfiles {
        remove(*pCaz.offset(i as isize));
        i += 1;
        i;
    }
    ulog_close();
    usysdep_exit(0 as libc::c_int);
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

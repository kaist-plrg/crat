use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
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
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_close();
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    static mut zProgram: *const libc::c_char;
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
    fn uuconf_logfile(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzlog: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_statsfile(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzstats: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debugfile(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debuglevel(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn usysdep_sleep(cseconds: libc::c_int);
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
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type UUCONF_POINTER = *mut libc::c_void;
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const no_argument: _argtype = 0;
pub const required_argument: _argtype = 1;
pub const optional_argument: _argtype = 2;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type _argtype = libc::c_uint;
pub static mut uulog_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: uulog.c,v 1.29 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asLlongopts: [option; 12] = [
    {
        let mut init = option {
            name: b"debuglog\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"follow\0" as *const u8 as *const libc::c_char,
            has_arg: optional_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
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
            name: b"statslog\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
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
            name: b"uuxqtlog\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
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
            val: 'X' as i32,
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
    let mut fdebug: boolean = 0 as libc::c_int;
    let mut fforever: boolean = 0 as libc::c_int;
    let mut cshow: libc::c_int = 0 as libc::c_int;
    let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut fstats: boolean = 0 as libc::c_int;
    let mut zuser: *const libc::c_char = 0 as *const libc::c_char;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut fuuxqt: boolean = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut zlogfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut zstatsfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut zdebugfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut zfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut pzshow: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ishow: libc::c_int = 0 as libc::c_int;
    let mut csystem: size_t = 0 as libc::c_int as size_t;
    let mut cuser: size_t = 0 as libc::c_int as size_t;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: size_t = 0;
    zProgram = *argv.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut clen: size_t = 0;
            let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
            clen = strlen(*argv.offset(i as isize));
            znew = zbufalc(clen.wrapping_add(2 as libc::c_int as libc::c_ulong));
            *znew.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
            *znew.offset(1 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
            memcpy(
                znew.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                (*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                clen,
            );
            let ref mut fresh0 = *argv.offset(i as isize);
            *fresh0 = znew;
        }
        i += 1;
        i;
    }
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"Df:FI:n:s:Su:vxX:\0" as *const u8 as *const libc::c_char,
            asLlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            68 => {
                fdebug = 1 as libc::c_int;
            }
            102 => {
                fforever = 1 as libc::c_int;
                zsystem = gnu_optarg;
                if cshow == 0 as libc::c_int {
                    cshow = 10 as libc::c_int;
                }
            }
            70 => {
                fforever = 1 as libc::c_int;
                if cshow == 0 as libc::c_int {
                    cshow = 10 as libc::c_int;
                }
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            110 => {
                cshow = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
            }
            115 => {
                zsystem = gnu_optarg;
            }
            83 => {
                fstats = 1 as libc::c_int;
            }
            117 => {
                zuser = gnu_optarg;
            }
            120 => {
                fuuxqt = 1 as libc::c_int;
            }
            88 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            118 => {
                printf(
                    b"uulog (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
            2 => {
                fforever = 1 as libc::c_int;
                if cshow == 0 as libc::c_int {
                    cshow = 10 as libc::c_int;
                }
                if !gnu_optarg.is_null() {
                    zsystem = gnu_optarg;
                }
            }
            1 => {
                ulhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                ulusage();
            }
        }
    }
    if gnu_optind != argc || fstats != 0 && fdebug != 0 {
        ulusage();
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
    iuuconf = uuconf_logfile(puuconf, &mut zlogfile);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_statsfile(puuconf, &mut zstatsfile);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_debugfile(puuconf, &mut zdebugfile);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    usysdep_initialize(puuconf, 0o2 as libc::c_int);
    if !zsystem.is_null() {
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
        iuuconf = uuconf_system_info(puuconf, zsystem, &mut ssys);
        if iuuconf == 0 as libc::c_int {
            zsystem = zbufcpy(ssys.uuconf_zname);
            uuconf_free_block(ssys.uuconf_palloc);
        }
    }
    if fstats != 0 {
        zfile = zstatsfile;
    } else if fdebug != 0 {
        zfile = zdebugfile;
    } else {
        zfile = zlogfile;
    }
    e = fopen(zfile, b"r\0" as *const u8 as *const libc::c_char);
    if e.is_null() {
        ulog(
            LOG_ERROR,
            b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
            zfile,
            strerror(*__errno_location()),
        );
        usysdep_exit(0 as libc::c_int);
    }
    if cshow > 0 as libc::c_int {
        pzshow = xmalloc(
            (cshow as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        ishow = 0 as libc::c_int;
        while ishow < cshow {
            let ref mut fresh1 = *pzshow.offset(ishow as isize);
            *fresh1 = 0 as *mut libc::c_char;
            ishow += 1;
            ishow;
        }
        ishow = 0 as libc::c_int;
    }
    if !zsystem.is_null() {
        csystem = strlen(zsystem);
    }
    if !zuser.is_null() {
        cuser = strlen(zuser);
    }
    zline = 0 as *mut libc::c_char;
    cline = 0 as libc::c_int as size_t;
    loop {
        while getline(&mut zline, &mut cline, e) > 0 as libc::c_int as libc::c_long {
            let mut zluser: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zlsys: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut znext: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cluser: size_t = 0;
            let mut clsys: size_t = 0;
            znext = zline
                .offset(
                    strspn(zline, b" \t\0" as *const u8 as *const libc::c_char) as isize,
                );
            if fstats == 0 {
                znext = znext
                    .offset(
                        strcspn(znext, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
                znext = znext
                    .offset(
                        strspn(znext, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
                zlsys = znext;
                clsys = strcspn(znext, b" \t\0" as *const u8 as *const libc::c_char);
                znext = znext.offset(clsys as isize);
                znext = znext
                    .offset(
                        strspn(znext, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
                zluser = znext;
                cluser = strcspn(znext, b" \t\0" as *const u8 as *const libc::c_char);
            } else {
                zluser = znext;
                cluser = strcspn(znext, b" \t\0" as *const u8 as *const libc::c_char);
                znext = znext.offset(cluser as isize);
                znext = znext
                    .offset(
                        strspn(znext, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
                zlsys = znext;
                clsys = strcspn(znext, b" \t\0" as *const u8 as *const libc::c_char);
            }
            if !zsystem.is_null()
                && (csystem != clsys
                    || strncmp(zsystem, zlsys, clsys) != 0 as libc::c_int)
            {
                continue;
            }
            if !zuser.is_null()
                && (cuser != cluser
                    || strncmp(zuser, zluser, cluser) != 0 as libc::c_int)
            {
                continue;
            }
            if cshow <= 0 as libc::c_int {
                printf(b"%s\0" as *const u8 as *const libc::c_char, zline);
            } else {
                ubuffree(*pzshow.offset(ishow as isize) as pointer as *mut libc::c_char);
                let ref mut fresh2 = *pzshow.offset(ishow as isize);
                *fresh2 = zbufcpy(zline);
                ishow = (ishow + 1 as libc::c_int) % cshow;
            }
        }
        if cshow > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < cshow {
                if !(*pzshow.offset(ishow as isize)).is_null() {
                    printf(
                        b"%s\0" as *const u8 as *const libc::c_char,
                        *pzshow.offset(ishow as isize),
                    );
                }
                ishow = (ishow + 1 as libc::c_int) % cshow;
                i += 1;
                i;
            }
        }
        if fforever == 0 || ferror(e) != 0 {
            break;
        }
        clearerr(e);
        cshow = 0 as libc::c_int;
        usysdep_sleep(1 as libc::c_int);
    }
    fclose(e);
    ulog_close();
    usysdep_exit(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ulusage() {
    fprintf(
        stderr,
        b"Usage: %s [-n #] [-sf system] [-u user] [-xDSF] [-I file] [-X debug]\n\0"
            as *const u8 as *const libc::c_char,
        zProgram,
    );
    fprintf(
        stderr,
        b"Use %s --help for help\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn ulhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Usage: %s [-n #] [-sf system] [-u user] [-DSF] [-I file] [-X debug]\n\0"
            as *const u8 as *const libc::c_char,
        zProgram,
    );
    printf(
        b" -n,--lines: show given number of lines from end of log\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -s,--system: print entries for named system\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -f system,--follow=system: follow entries for named system\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -u,--user user: print entries for named user\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -F,--follow: follow entries for any system\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -S,--statslog: show statistics file\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -D,--debuglog: show debugging file\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -X,--debug debug: Set debugging level\n\0" as *const u8 as *const libc::c_char,
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

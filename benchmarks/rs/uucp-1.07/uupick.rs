use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
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
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
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
    fn uuconf_pubdir(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzpub: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn fsysdep_move_file(
        zorig: *const libc::c_char,
        zto: *const libc::c_char,
        fmkdirs: boolean,
        fpublic: boolean,
        fcheck: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
    fn usysdep_walk_tree(
        zdir: *const libc::c_char,
        pufn: Option::<
            unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, pointer) -> (),
        >,
        pinfo: pointer,
    ) -> boolean;
    fn fsysdep_uupick_init(
        zsystem: *const libc::c_char,
        zpubdir: *const libc::c_char,
    ) -> boolean;
    fn zsysdep_uupick(
        zsystem: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pzfrom: *mut *mut libc::c_char,
        pzfull: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_uupick_free(
        zsystem: *const libc::c_char,
        zpubdir: *const libc::c_char,
    ) -> boolean;
    fn zsysdep_uupick_local_file(
        zfile: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_rmdir(zdir: *const libc::c_char) -> boolean;
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
pub type _argtype = libc::c_uint;
pub const optional_argument: _argtype = 2;
pub static mut uupick_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: uupick.c,v 1.19 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asPlongopts: [option; 6] = [
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
    let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut zpubdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfull: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zallsys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ab: [libc::c_char; 1000] = [0; 1000];
    let mut fquit: boolean = 0;
    zProgram = b"uupick\0" as *const u8 as *const libc::c_char;
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"I:s:vx:\0" as *const u8 as *const libc::c_char,
            asPlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            115 => {
                zsystem = gnu_optarg;
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            120 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            118 => {
                printf(
                    b"uupick (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
                uphelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                upusage();
            }
        }
    }
    if argc != gnu_optind {
        upusage();
    }
    iuuconf = uuconf_init(
        &mut puuconf,
        0 as *mut libc::c_void as *const libc::c_char,
        zconfig,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    usysdep_initialize(puuconf, 0o1 as libc::c_int | 0o2 as libc::c_int);
    zpubdir = 0 as *const libc::c_char;
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
            zpubdir = zbufcpy(ssys.uuconf_zpubdir);
            uuconf_free_block(ssys.uuconf_palloc);
        }
    }
    if zpubdir.is_null() {
        iuuconf = uuconf_pubdir(puuconf, &mut zpubdir);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
    }
    if fsysdep_uupick_init(zsystem, zpubdir) == 0 {
        usysdep_exit(0 as libc::c_int);
    }
    zallsys = 0 as *mut libc::c_char;
    fquit = 0 as libc::c_int;
    while fquit == 0
        && {
            zfile = zsysdep_uupick(zsystem, zpubdir, &mut zfrom, &mut zfull);
            !zfile.is_null()
        }
    {
        let mut fdir: boolean = 0;
        let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zlocal: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut e: *mut FILE = 0 as *mut FILE;
        let mut fcontinue: boolean = 0;
        fdir = fsysdep_directory(zfull);
        loop {
            let mut fbadname: boolean = 0;
            fcontinue = 0 as libc::c_int;
            if zallsys.is_null() || strcmp(zallsys, zfrom) != 0 as libc::c_int {
                if !zallsys.is_null() {
                    ubuffree(zallsys);
                    zallsys = 0 as *mut libc::c_char;
                }
                printf(
                    b"from %s: %s %s ?\n\0" as *const u8 as *const libc::c_char,
                    zfrom,
                    if fdir != 0 {
                        b"dir\0" as *const u8 as *const libc::c_char
                    } else {
                        b"file\0" as *const u8 as *const libc::c_char
                    },
                    zfile,
                );
                if (fgets(
                    ab.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong
                        as libc::c_int,
                    stdin,
                ))
                    .is_null()
                {
                    break;
                }
            }
            if ab[0 as libc::c_int as usize] as libc::c_int == 'q' as i32 {
                fquit = 1 as libc::c_int;
                break;
            } else {
                match ab[0 as libc::c_int as usize] as libc::c_int {
                    10 => {}
                    100 => {
                        if fdir != 0 {
                            fsysdep_rmdir(zfull);
                        } else if remove(zfull) != 0 as libc::c_int {
                            ulog(
                                LOG_ERROR,
                                b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
                                zfull,
                                strerror(*__errno_location()),
                            );
                        }
                    }
                    109 | 97 => {
                        zto = ab
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize)
                            .offset(
                                strspn(
                                    ab.as_mut_ptr().offset(1 as libc::c_int as isize),
                                    b" \t\0" as *const u8 as *const libc::c_char,
                                ) as isize,
                            );
                        *zto
                            .offset(
                                strcspn(zto, b" \t\n\0" as *const u8 as *const libc::c_char)
                                    as isize,
                            ) = '\0' as i32 as libc::c_char;
                        zlocal = zsysdep_uupick_local_file(zto, &mut fbadname);
                        if zlocal.is_null() {
                            if fbadname == 0 {
                                usysdep_exit(0 as libc::c_int);
                            }
                            ulog(
                                LOG_ERROR,
                                b"%s: bad local file name\0" as *const u8
                                    as *const libc::c_char,
                                zto,
                            );
                            fcontinue = 1 as libc::c_int;
                        } else {
                            zto = zsysdep_in_dir(zlocal, zfile);
                            ubuffree(zlocal);
                            if zto.is_null() {
                                usysdep_exit(0 as libc::c_int);
                            }
                            if fdir == 0 {
                                upmove(zfull, zto);
                            } else {
                                usysdep_walk_tree(
                                    zfull,
                                    Some(
                                        upmovedir
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                *const libc::c_char,
                                                pointer,
                                            ) -> (),
                                    ),
                                    zto as pointer,
                                );
                                fsysdep_rmdir(zfull);
                            }
                            ubuffree(zto);
                            if ab[0 as libc::c_int as usize] as libc::c_int == 'a' as i32
                            {
                                zallsys = zbufcpy(zfrom);
                                ab[0 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
                            }
                        }
                    }
                    112 => {
                        if fdir != 0 {
                            ulog(
                                LOG_ERROR,
                                b"Can't print directory\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            e = fopen(zfull, b"r\0" as *const u8 as *const libc::c_char);
                            if e.is_null() {
                                ulog(
                                    LOG_ERROR,
                                    b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
                                    zfull,
                                    strerror(*__errno_location()),
                                );
                            } else {
                                while !(fgets(
                                    ab.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 1000]>()
                                        as libc::c_ulong as libc::c_int,
                                    e,
                                ))
                                    .is_null()
                                {
                                    fputs(ab.as_mut_ptr(), stdout);
                                }
                                fclose(e);
                            }
                        }
                        fcontinue = 1 as libc::c_int;
                    }
                    33 => {
                        system(ab.as_mut_ptr().offset(1 as libc::c_int as isize));
                        fcontinue = 1 as libc::c_int;
                    }
                    _ => {
                        printf(
                            b"uupick commands:\n\0" as *const u8 as *const libc::c_char,
                        );
                        printf(b"q: quit\n\0" as *const u8 as *const libc::c_char);
                        printf(
                            b"<return>: skip file\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        printf(
                            b"m [dir]: move file to directory\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        printf(
                            b"a [dir]: move all files from this system to directory\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        printf(
                            b"p: list file to stdout\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        printf(
                            b"d: delete file\n\0" as *const u8 as *const libc::c_char,
                        );
                        printf(
                            b"! command: shell escape\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        fcontinue = 1 as libc::c_int;
                    }
                }
                if !(fcontinue != 0) {
                    break;
                }
            }
        }
        ubuffree(zfull);
        ubuffree(zfrom);
        ubuffree(zfile);
    }
    fsysdep_uupick_free(zsystem, zpubdir);
    usysdep_exit(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn upusage() {
    fprintf(
        stderr,
        b"Usage: %s [-s system] [-I config] [-x debug]\n\0" as *const u8
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
unsafe extern "C" fn uphelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -s,--system system: Only consider files from named system\n\0" as *const u8
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
unsafe extern "C" fn upmovedir(
    mut zfull: *const libc::c_char,
    mut zrelative: *const libc::c_char,
    mut pinfo: pointer,
) {
    let mut ztodir: *const libc::c_char = pinfo as *const libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    zto = zsysdep_in_dir(ztodir, zrelative);
    if zto.is_null() {
        usysdep_exit(0 as libc::c_int);
    }
    upmove(zfull, zto);
    ubuffree(zto);
}
unsafe extern "C" fn upmove(
    mut zfrom: *const libc::c_char,
    mut zto: *const libc::c_char,
) {
    fsysdep_move_file(
        zfrom,
        zto,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
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

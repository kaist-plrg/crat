use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
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
    fn uuconf_init(
        uuconf_ppglobal: *mut *mut libc::c_void,
        uuconf_zprogram: *const libc::c_char,
        uuconf_zname: *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_system_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzsystems: *mut *mut *mut libc::c_char,
        uuconf_falias: libc::c_int,
    ) -> libc::c_int;
    fn uuconf_system_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zsystem: *const libc::c_char,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_find_port(
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
    fn uuconf_dialer_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdialer: *const libc::c_char,
        uuconf_qdialer: *mut uuconf_dialer,
    ) -> libc::c_int;
    fn uuconf_config_files(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_names: *mut uuconf_config_file_names,
    ) -> libc::c_int;
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_spooldir(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzspool: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_pubdir(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzpub: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_lockdir(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzlock: *mut *const libc::c_char,
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
    fn uuconf_strip(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pistrip: *mut libc::c_int,
    ) -> libc::c_int;
    fn uuconf_maxuuxqts(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pcmaxuuxqt: *mut libc::c_int,
    ) -> libc::c_int;
    fn uuconf_runuuxqt(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pirunuuxqt: *mut libc::c_int,
    ) -> libc::c_int;
    fn uuconf_callout(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *const uuconf_system,
        uuconf_pzlog: *mut *mut libc::c_char,
        uuconf_pzpass: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn uuconf_error_string(
        uuconf_pglobal: *mut libc::c_void,
        ierror: libc::c_int,
        zbuf: *mut libc::c_char,
        cbuf: UUCONF_SIZE_T,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
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
pub type UUCONF_SIZE_T = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sinfo {
    pub puuconf: pointer,
    pub qsys: *const uuconf_system,
    pub fgot: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_config_file_names {
    pub uuconf_ztaylor_config: *const libc::c_char,
    pub uuconf_pztaylor_sys: *const *const libc::c_char,
    pub uuconf_pztaylor_port: *const *const libc::c_char,
    pub uuconf_pztaylor_dial: *const *const libc::c_char,
    pub uuconf_pzdialcode: *const *const libc::c_char,
    pub uuconf_pztaylor_pwd: *const *const libc::c_char,
    pub uuconf_pztaylor_call: *const *const libc::c_char,
    pub uuconf_zv2_systems: *const libc::c_char,
    pub uuconf_zv2_device: *const libc::c_char,
    pub uuconf_zv2_userfile: *const libc::c_char,
    pub uuconf_zv2_cmds: *const libc::c_char,
    pub uuconf_pzhdb_systems: *const *const libc::c_char,
    pub uuconf_pzhdb_devices: *const *const libc::c_char,
    pub uuconf_pzhdb_dialers: *const *const libc::c_char,
    pub uuconf_zhdb_permissions: *const libc::c_char,
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
pub static mut uuchk_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: uuchk.c,v 1.71 2002/03/05 19:10:42 ian Rel $\0")
};
static mut zKprogram: *const libc::c_char = 0 as *const libc::c_char;
static mut asKlongopts: [option; 6] = [
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
    let mut iopt: libc::c_int = 0;
    let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut iret: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    zKprogram = *argv.offset(0 as libc::c_int as isize);
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"I:s:vx:\0" as *const u8 as *const libc::c_char,
            asKlongopts.as_ptr(),
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
                zconfig = gnu_optarg;
            }
            120 => {}
            118 => {
                printf(
                    b"uuchk (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
                ukhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                ukusage();
            }
        }
    }
    if gnu_optind != argc {
        fprintf(
            stderr,
            b"%s: too many arguments\0" as *const u8 as *const libc::c_char,
            zKprogram,
        );
        ukusage();
    }
    iret = uuconf_init(
        &mut puuconf,
        0 as *mut libc::c_void as *const libc::c_char,
        zconfig,
    );
    if iret != 0 as libc::c_int {
        ukuuconf_error(puuconf, iret);
    }
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
        iret = uuconf_system_info(puuconf, zsystem, &mut ssys);
        if iret == 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s: system not found\n\0" as *const u8 as *const libc::c_char,
                zsystem,
            );
            exit(1 as libc::c_int);
        } else if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        ukshow(&mut ssys, puuconf);
        uuconf_free_block(ssys.uuconf_palloc);
    } else {
        let mut snames: uuconf_config_file_names = uuconf_config_file_names {
            uuconf_ztaylor_config: 0 as *const libc::c_char,
            uuconf_pztaylor_sys: 0 as *const *const libc::c_char,
            uuconf_pztaylor_port: 0 as *const *const libc::c_char,
            uuconf_pztaylor_dial: 0 as *const *const libc::c_char,
            uuconf_pzdialcode: 0 as *const *const libc::c_char,
            uuconf_pztaylor_pwd: 0 as *const *const libc::c_char,
            uuconf_pztaylor_call: 0 as *const *const libc::c_char,
            uuconf_zv2_systems: 0 as *const libc::c_char,
            uuconf_zv2_device: 0 as *const libc::c_char,
            uuconf_zv2_userfile: 0 as *const libc::c_char,
            uuconf_zv2_cmds: 0 as *const libc::c_char,
            uuconf_pzhdb_systems: 0 as *const *const libc::c_char,
            uuconf_pzhdb_devices: 0 as *const *const libc::c_char,
            uuconf_pzhdb_dialers: 0 as *const *const libc::c_char,
            uuconf_zhdb_permissions: 0 as *const libc::c_char,
        };
        let mut zstr: *const libc::c_char = 0 as *const libc::c_char;
        let mut iint: libc::c_int = 0;
        let mut pzsystems: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        iret = uuconf_config_files(puuconf, &mut snames);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        if !(snames.uuconf_ztaylor_config).is_null() {
            printf(
                b"config file: %s\n\0" as *const u8 as *const libc::c_char,
                snames.uuconf_ztaylor_config,
            );
        }
        ukshow_names(
            b"sys file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pztaylor_sys,
        );
        ukshow_names(
            b"port file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pztaylor_port,
        );
        ukshow_names(
            b"dial file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pztaylor_dial,
        );
        ukshow_names(
            b"dialcode file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pzdialcode,
        );
        ukshow_names(
            b"passwd file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pztaylor_pwd,
        );
        ukshow_names(
            b"call file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pztaylor_call,
        );
        if !(snames.uuconf_zv2_systems).is_null() {
            printf(
                b"V2 L.sys file: %s\n\0" as *const u8 as *const libc::c_char,
                snames.uuconf_zv2_systems,
            );
        }
        if !(snames.uuconf_zv2_device).is_null() {
            printf(
                b"V2 L-devices file: %s\n\0" as *const u8 as *const libc::c_char,
                snames.uuconf_zv2_device,
            );
        }
        if !(snames.uuconf_zv2_userfile).is_null() {
            printf(
                b"V2 USERFILE file: %s\n\0" as *const u8 as *const libc::c_char,
                snames.uuconf_zv2_userfile,
            );
        }
        if !(snames.uuconf_zv2_cmds).is_null() {
            printf(
                b"V2 L.cmds file: %s\n\0" as *const u8 as *const libc::c_char,
                snames.uuconf_zv2_cmds,
            );
        }
        ukshow_names(
            b"HDB Systems file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pzhdb_systems,
        );
        ukshow_names(
            b"HDB Devices file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pzhdb_devices,
        );
        ukshow_names(
            b"HDB Dialers file\0" as *const u8 as *const libc::c_char,
            snames.uuconf_pzhdb_dialers,
        );
        if !(snames.uuconf_zhdb_permissions).is_null() {
            printf(
                b"HDB Permissions file: %s\n\0" as *const u8 as *const libc::c_char,
                snames.uuconf_zhdb_permissions,
            );
        }
        iret = uuconf_localname(puuconf, &mut zstr);
        if iret == 0 as libc::c_int {
            printf(b"Local node name %s\n\0" as *const u8 as *const libc::c_char, zstr);
        } else if iret != 1 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        iret = uuconf_spooldir(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(b"Spool directory %s\n\0" as *const u8 as *const libc::c_char, zstr);
        iret = uuconf_pubdir(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(b"Public directory %s\n\0" as *const u8 as *const libc::c_char, zstr);
        iret = uuconf_lockdir(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(b"Lock directory %s\n\0" as *const u8 as *const libc::c_char, zstr);
        iret = uuconf_logfile(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(b"Log file %s\n\0" as *const u8 as *const libc::c_char, zstr);
        iret = uuconf_statsfile(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(b"Statistics file %s\n\0" as *const u8 as *const libc::c_char, zstr);
        iret = uuconf_debugfile(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(b"Debug file %s\n\0" as *const u8 as *const libc::c_char, zstr);
        iret = uuconf_debuglevel(puuconf, &mut zstr);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        if !zstr.is_null() {
            printf(
                b"Global debugging level %s\n\0" as *const u8 as *const libc::c_char,
                zstr,
            );
        }
        iret = uuconf_strip(puuconf, &mut iint);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        printf(
            b"uucico -l will %sstrip login names and passwords\n\0" as *const u8
                as *const libc::c_char,
            if iint & 0o1 as libc::c_int != 0 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"not \0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"uucico will %sstrip UUCP protocol commands\n\0" as *const u8
                as *const libc::c_char,
            if iint & 0o2 as libc::c_int != 0 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"not \0" as *const u8 as *const libc::c_char
            },
        );
        iret = uuconf_maxuuxqts(puuconf, &mut iint);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        if iint != 0 as libc::c_int {
            printf(
                b"Maximum number of uuxqt processes permitted %d\n\0" as *const u8
                    as *const libc::c_char,
                iint,
            );
        }
        iret = uuconf_runuuxqt(puuconf, &mut iint);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        if iint > 0 as libc::c_int {
            printf(
                b"Start uuxqt every %d jobs\n\0" as *const u8 as *const libc::c_char,
                iint,
            );
        } else {
            match iint {
                0 => {
                    printf(b"Never start uuxqt\n\0" as *const u8 as *const libc::c_char);
                }
                -1 => {
                    printf(
                        b"Start uuxqt once per uucico invocation\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                -2 => {
                    printf(
                        b"Start uuxqt once per call\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Illegal value from uuconf_runuuxqt\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
        iret = uuconf_system_names(puuconf, &mut pzsystems, 0 as libc::c_int);
        if iret != 0 as libc::c_int {
            ukuuconf_error(puuconf, iret);
        }
        if (*pzsystems).is_null() {
            fprintf(
                stderr,
                b"%s: no systems found\n\0" as *const u8 as *const libc::c_char,
                zKprogram,
            );
            exit(1 as libc::c_int);
        }
        while !(*pzsystems).is_null() {
            let mut ssys_0: uuconf_system = uuconf_system {
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
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            iret = uuconf_system_info(puuconf, *pzsystems, &mut ssys_0);
            if iret != 0 as libc::c_int {
                ukuuconf_error(puuconf, iret);
            } else {
                ukshow(&mut ssys_0, puuconf);
            }
            uuconf_free_block(ssys_0.uuconf_palloc);
            pzsystems = pzsystems.offset(1);
            pzsystems;
        }
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn ukusage() {
    fprintf(
        stderr,
        b"Usage: %s [-s system] [-I file]\n\0" as *const u8 as *const libc::c_char,
        zKprogram,
    );
    fprintf(
        stderr,
        b"Use %s --help for help\n\0" as *const u8 as *const libc::c_char,
        zKprogram,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn ukhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Usage: %s [-s system] [-I file] [-v]\n\0" as *const u8 as *const libc::c_char,
        zKprogram,
    );
    printf(
        b" -s,--system system: Only print configuration for named system\n\0"
            as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn ukshow_names(
    mut zheader: *const libc::c_char,
    mut pznames: *const *const libc::c_char,
) {
    if pznames.is_null() {
        return;
    }
    if (*pznames.offset(1 as libc::c_int as isize)).is_null() {
        printf(
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            zheader,
            *pznames.offset(0 as libc::c_int as isize),
        );
    } else {
        let mut pz: *const *const libc::c_char = 0 as *const *const libc::c_char;
        printf(b"%s:\n\0" as *const u8 as *const libc::c_char, zheader);
        pz = pznames;
        while !(*pz).is_null() {
            printf(b"  %s\n\0" as *const u8 as *const libc::c_char, *pz);
            pz = pz.offset(1);
            pz;
        }
    };
}
unsafe extern "C" fn ukshow(mut qsys: *const uuconf_system, mut puuconf: pointer) {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut iret: libc::c_int = 0;
    let mut fanycall: boolean = 0;
    printf(b"System: %s\0" as *const u8 as *const libc::c_char, (*qsys).uuconf_zname);
    if !((*qsys).uuconf_pzalias).is_null() {
        printf(b" (\0" as *const u8 as *const libc::c_char);
        pz = (*qsys).uuconf_pzalias;
        while !(*pz).is_null() {
            printf(b"%s\0" as *const u8 as *const libc::c_char, *pz);
            if !(*pz.offset(1 as libc::c_int as isize)).is_null() {
                printf(b" \0" as *const u8 as *const libc::c_char);
            }
            pz = pz.offset(1);
            pz;
        }
        printf(b")\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    fanycall = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !qsys.is_null() {
        let mut fcall: boolean = 0;
        let mut fcalled: boolean = 0;
        let mut qtime: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
        let mut qspan: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
        if i != 0 as libc::c_int || !((*qsys).uuconf_qalternate).is_null() {
            printf(b"Alternate %d\0" as *const u8 as *const libc::c_char, i);
            if !((*qsys).uuconf_zalternate).is_null() {
                printf(
                    b" (%s)\0" as *const u8 as *const libc::c_char,
                    (*qsys).uuconf_zalternate,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        fcall = (*qsys).uuconf_fcall;
        if ((*qsys).uuconf_qtimegrade).is_null() {
            fcall = 0 as libc::c_int;
        }
        fcalled = (*qsys).uuconf_fcalled;
        if fcall == 0 && fcalled == 0 {
            printf(
                b" This alternate is never used\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if fcall != 0 {
                fanycall = 1 as libc::c_int;
            }
            if fcalled != 0 {
                if !((*qsys).uuconf_zcalled_login).is_null()
                    && strcmp(
                        (*qsys).uuconf_zcalled_login,
                        b"ANY\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    if i == 0 as libc::c_int && ((*qsys).uuconf_qalternate).is_null() {
                        printf(
                            b" Caller must log in as %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*qsys).uuconf_zcalled_login,
                        );
                    } else {
                        printf(
                            b" When called using login name %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*qsys).uuconf_zcalled_login,
                        );
                    }
                } else {
                    printf(
                        b" When called using any login name\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !((*qsys).uuconf_zlocalname).is_null() {
                    printf(
                        b" Will use %s as name of local system\n\0" as *const u8
                            as *const libc::c_char,
                        (*qsys).uuconf_zlocalname,
                    );
                }
            }
            if fcalled != 0 && (*qsys).uuconf_fcallback != 0 {
                printf(
                    b" If called, will call back\n\0" as *const u8 as *const libc::c_char,
                );
                fcalled = 0 as libc::c_int;
            }
            if fcall != 0 {
                let mut si: sinfo = sinfo {
                    puuconf: 0 as *mut libc::c_void,
                    qsys: 0 as *const uuconf_system,
                    fgot: 0,
                };
                if i == 0 as libc::c_int && ((*qsys).uuconf_qalternate).is_null() {
                    printf(b" Call out\0" as *const u8 as *const libc::c_char);
                } else {
                    printf(
                        b" This alternate applies when calling\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !((*qsys).uuconf_zport).is_null() || !((*qsys).uuconf_qport).is_null()
                {
                    printf(b" using \0" as *const u8 as *const libc::c_char);
                    if !((*qsys).uuconf_zport).is_null() {
                        printf(
                            b"port %s\0" as *const u8 as *const libc::c_char,
                            (*qsys).uuconf_zport,
                        );
                    } else {
                        printf(
                            b"a specially defined port\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if (*qsys).uuconf_ibaud != 0 as libc::c_int as libc::c_long {
                        printf(
                            b" at speed %ld\0" as *const u8 as *const libc::c_char,
                            (*qsys).uuconf_ibaud,
                        );
                        if (*qsys).uuconf_ihighbaud != 0 as libc::c_int as libc::c_long {
                            printf(
                                b" to %ld\0" as *const u8 as *const libc::c_char,
                                (*qsys).uuconf_ihighbaud,
                            );
                        }
                    }
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                } else if (*qsys).uuconf_ibaud != 0 as libc::c_int as libc::c_long {
                    printf(
                        b" at speed %ld\0" as *const u8 as *const libc::c_char,
                        (*qsys).uuconf_ibaud,
                    );
                    if (*qsys).uuconf_ihighbaud != 0 as libc::c_int as libc::c_long {
                        printf(
                            b" to %ld\0" as *const u8 as *const libc::c_char,
                            (*qsys).uuconf_ihighbaud,
                        );
                    }
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                } else {
                    printf(b" using any port\n\0" as *const u8 as *const libc::c_char);
                }
                si.puuconf = puuconf;
                si.qsys = qsys;
                si.fgot = 0 as libc::c_int;
                if !((*qsys).uuconf_qport).is_null() {
                    printf(
                        b" The port is defined as:\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ikshow_port((*qsys).uuconf_qport, &mut si as *mut sinfo as pointer);
                } else {
                    let mut sdummy: uuconf_port = uuconf_port {
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
                    printf(
                        b" The possible ports are:\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    iret = uuconf_find_port(
                        puuconf,
                        (*qsys).uuconf_zport,
                        (*qsys).uuconf_ibaud,
                        (*qsys).uuconf_ihighbaud,
                        Some(
                            ikshow_port
                                as unsafe extern "C" fn(
                                    *mut uuconf_port,
                                    pointer,
                                ) -> libc::c_int,
                        ),
                        &mut si as *mut sinfo as pointer,
                        &mut sdummy,
                    );
                    if iret != 1 as libc::c_int {
                        ukuuconf_error(puuconf, iret);
                    }
                    if si.fgot == 0 {
                        printf(
                            b" *** There are no matching ports\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                if !((*qsys).uuconf_zphone).is_null() {
                    if !((*qsys).uuconf_zport).is_null()
                        && strcmp(
                            (*qsys).uuconf_zport,
                            b"TCP\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || !((*qsys).uuconf_qport).is_null()
                            && ((*(*qsys).uuconf_qport).uuconf_ttype as libc::c_uint
                                == UUCONF_PORTTYPE_TCP as libc::c_int as libc::c_uint
                                || (*(*qsys).uuconf_qport).uuconf_ttype as libc::c_uint
                                    == UUCONF_PORTTYPE_TLI as libc::c_int as libc::c_uint)
                    {
                        printf(
                            b" Remote address %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*qsys).uuconf_zphone,
                        );
                    } else {
                        printf(
                            b" Phone number %s\n\0" as *const u8 as *const libc::c_char,
                            (*qsys).uuconf_zphone,
                        );
                    }
                }
                ukshow_chat(
                    &(*qsys).uuconf_schat,
                    b" Chat\0" as *const u8 as *const libc::c_char,
                );
                if !((*qsys).uuconf_zcall_login).is_null()
                    || !((*qsys).uuconf_zcall_password).is_null()
                {
                    let mut zlogin: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut zpass: *mut libc::c_char = 0 as *mut libc::c_char;
                    iret = uuconf_callout(puuconf, qsys, &mut zlogin, &mut zpass);
                    if iret == 1 as libc::c_int {
                        printf(
                            b" Can not determine login name or password\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if iret & 0xff as libc::c_int == 2 as libc::c_int {
                        printf(
                            b" Can not read call out file\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if iret != 0 as libc::c_int {
                        ukuuconf_error(puuconf, iret);
                    } else {
                        if !zlogin.is_null() {
                            printf(
                                b" Login name %s\n\0" as *const u8 as *const libc::c_char,
                                zlogin,
                            );
                            free(zlogin as pointer);
                        }
                        if !zpass.is_null() {
                            printf(
                                b" Password %s\n\0" as *const u8 as *const libc::c_char,
                                zpass,
                            );
                            free(zpass as pointer);
                        }
                    }
                }
                qtime = qcompress_span((*qsys).uuconf_qtimegrade);
                qspan = qtime;
                while !qspan.is_null() {
                    printf(b" \0" as *const u8 as *const libc::c_char);
                    ukshow_time(qspan);
                    printf(b" may call if \0" as *const u8 as *const libc::c_char);
                    if (*qspan).uuconf_ival as libc::c_char as libc::c_int == 'z' as i32
                    {
                        printf(b"any work\0" as *const u8 as *const libc::c_char);
                    } else {
                        printf(
                            b"work grade %c or higher\0" as *const u8
                                as *const libc::c_char,
                            (*qspan).uuconf_ival as libc::c_char as libc::c_int,
                        );
                    }
                    if (*qspan).uuconf_cretry != 0 as libc::c_int {
                        printf(
                            b" (retry %d)\0" as *const u8 as *const libc::c_char,
                            (*qspan).uuconf_cretry,
                        );
                    }
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                    qspan = (*qspan).uuconf_qnext;
                }
                if (*qsys).uuconf_cmax_retries > 0 as libc::c_int {
                    printf(
                        b" May retry the call up to %d times\n\0" as *const u8
                            as *const libc::c_char,
                        (*qsys).uuconf_cmax_retries,
                    );
                }
                if !((*qsys).uuconf_qcalltimegrade).is_null() {
                    let mut fprint: boolean = 0;
                    let mut fother: boolean = 0;
                    qtime = qcompress_span((*qsys).uuconf_qcalltimegrade);
                    fprint = 0 as libc::c_int;
                    fother = 0 as libc::c_int;
                    if (*qtime).uuconf_istart != 0 as libc::c_int {
                        fother = 1 as libc::c_int;
                    }
                    qspan = qtime;
                    while !qspan.is_null() {
                        if (*qspan).uuconf_ival as libc::c_char as libc::c_int
                            == 'z' as i32
                        {
                            fother = 1 as libc::c_int;
                        } else {
                            fprint = 1 as libc::c_int;
                            printf(b" \0" as *const u8 as *const libc::c_char);
                            ukshow_time(qspan);
                            printf(
                                b" may accept work grade %c or higher\n\0" as *const u8
                                    as *const libc::c_char,
                                (*qspan).uuconf_ival as libc::c_char as libc::c_int,
                            );
                            if ((*qspan).uuconf_qnext).is_null() {
                                if (*qspan).uuconf_iend
                                    != 7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                                {
                                    fother = 1 as libc::c_int;
                                }
                            } else if (*qspan).uuconf_iend
                                != (*(*qspan).uuconf_qnext).uuconf_istart
                            {
                                fother = 1 as libc::c_int;
                            }
                        }
                        qspan = (*qspan).uuconf_qnext;
                    }
                    if fprint != 0 && fother != 0 {
                        printf(
                            b" (At other times may accept any work)\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
            if fcalled != 0 {
                if !((*qsys).uuconf_qcalledtimegrade).is_null() {
                    let mut fprint_0: boolean = 0;
                    let mut fother_0: boolean = 0;
                    qtime = qcompress_span((*qsys).uuconf_qcalledtimegrade);
                    fprint_0 = 0 as libc::c_int;
                    fother_0 = 0 as libc::c_int;
                    if (*qtime).uuconf_istart != 0 as libc::c_int {
                        fother_0 = 1 as libc::c_int;
                    }
                    qspan = qtime;
                    while !qspan.is_null() {
                        if (*qspan).uuconf_ival as libc::c_char as libc::c_int
                            == 'z' as i32
                        {
                            fother_0 = 1 as libc::c_int;
                        } else {
                            fprint_0 = 1 as libc::c_int;
                            printf(b" \0" as *const u8 as *const libc::c_char);
                            ukshow_time(qspan);
                            printf(
                                b" will send work grade %c or higher\n\0" as *const u8
                                    as *const libc::c_char,
                                (*qspan).uuconf_ival as libc::c_char as libc::c_int,
                            );
                            if ((*qspan).uuconf_qnext).is_null() {
                                if (*qspan).uuconf_iend
                                    != 7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                                {
                                    fother_0 = 1 as libc::c_int;
                                }
                            } else if (*qspan).uuconf_iend
                                != (*(*qspan).uuconf_qnext).uuconf_istart
                            {
                                fother_0 = 1 as libc::c_int;
                            }
                        }
                        qspan = (*qspan).uuconf_qnext;
                    }
                    if fprint_0 != 0 && fother_0 != 0 {
                        printf(
                            b" (At other times will send any work)\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
            if fcall != 0 && (*qsys).uuconf_csuccess_wait != 0 as libc::c_int {
                printf(
                    b" Will wait %d seconds after a successful call\n\0" as *const u8
                        as *const libc::c_char,
                    (*qsys).uuconf_csuccess_wait,
                );
            }
            if (*qsys).uuconf_fsequence != 0 {
                printf(
                    b" Sequence numbers are used\n\0" as *const u8 as *const libc::c_char,
                );
            }
            if fcalled != 0 {
                ukshow_chat(
                    &(*qsys).uuconf_scalled_chat,
                    b" When called, chat\0" as *const u8 as *const libc::c_char,
                );
            }
            if !((*qsys).uuconf_zdebug).is_null() {
                printf(
                    b" Debugging level %s\n\0" as *const u8 as *const libc::c_char,
                    (*qsys).uuconf_zdebug,
                );
            }
            if !((*qsys).uuconf_zmax_remote_debug).is_null() {
                printf(
                    b" Max remote debugging level %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*qsys).uuconf_zmax_remote_debug,
                );
            }
            if fcall != 0 {
                ukshow_size(
                    (*qsys).uuconf_qcall_local_size,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                ukshow_size(
                    (*qsys).uuconf_qcall_remote_size,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            if fcalled != 0 {
                ukshow_size(
                    (*qsys).uuconf_qcalled_local_size,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                ukshow_size(
                    (*qsys).uuconf_qcalled_remote_size,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            if fcall != 0 {
                printf(
                    b" May %smake local requests when calling\n\0" as *const u8
                        as *const libc::c_char,
                    if (*qsys).uuconf_fcall_transfer != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if fcalled != 0 {
                printf(
                    b" May %smake local requests when called\n\0" as *const u8
                        as *const libc::c_char,
                    if (*qsys).uuconf_fcalled_transfer != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if (*qsys).uuconf_fcall_transfer != 0 || (*qsys).uuconf_fcalled_transfer != 0
            {
                printf(
                    b" May send by local request:\0" as *const u8 as *const libc::c_char,
                );
                pz = (*qsys).uuconf_pzlocal_send;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (*qsys).uuconf_fsend_request == 0 {
                printf(
                    b" May not send files by remote request\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                printf(
                    b" May send by remote request:\0" as *const u8 as *const libc::c_char,
                );
                pz = (*qsys).uuconf_pzremote_send;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (*qsys).uuconf_fcall_transfer != 0 || (*qsys).uuconf_fcalled_transfer != 0
            {
                printf(
                    b" May accept by local request:\0" as *const u8
                        as *const libc::c_char,
                );
                pz = (*qsys).uuconf_pzlocal_receive;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (*qsys).uuconf_frec_request == 0 {
                printf(
                    b" May not receive files by remote request\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                printf(
                    b" May receive by remote request:\0" as *const u8
                        as *const libc::c_char,
                );
                pz = (*qsys).uuconf_pzremote_receive;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            printf(b" May execute\0" as *const u8 as *const libc::c_char);
            pz = (*qsys).uuconf_pzcmds;
            while !(*pz).is_null() {
                printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                pz = pz.offset(1);
                pz;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b" Execution path\0" as *const u8 as *const libc::c_char);
            pz = (*qsys).uuconf_pzpath;
            while !(*pz).is_null() {
                printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                pz = pz.offset(1);
                pz;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            if (*qsys).uuconf_cfree_space != 0 as libc::c_int as libc::c_long {
                printf(
                    b" Will leave %ld bytes available\n\0" as *const u8
                        as *const libc::c_char,
                    (*qsys).uuconf_cfree_space,
                );
            }
            if !((*qsys).uuconf_zpubdir).is_null() {
                printf(
                    b" Public directory is %s\n\0" as *const u8 as *const libc::c_char,
                    (*qsys).uuconf_zpubdir,
                );
            }
            if !((*qsys).uuconf_pzforward_from).is_null() {
                printf(b" May forward from\0" as *const u8 as *const libc::c_char);
                pz = (*qsys).uuconf_pzforward_from;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if !((*qsys).uuconf_pzforward_to).is_null() {
                printf(b" May forward to\0" as *const u8 as *const libc::c_char);
                pz = (*qsys).uuconf_pzforward_to;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (*qsys).uuconf_cmax_file_time > 0 as libc::c_int as libc::c_long {
                printf(
                    b" Maximum file send time: %ld\n\0" as *const u8
                        as *const libc::c_char,
                    (*qsys).uuconf_cmax_file_time,
                );
            }
            if !((*qsys).uuconf_zprotocols).is_null() {
                printf(
                    b" Will use protocols %s\n\0" as *const u8 as *const libc::c_char,
                    (*qsys).uuconf_zprotocols,
                );
            } else {
                printf(
                    b" Will use any known protocol\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !((*qsys).uuconf_qproto_params).is_null() {
                ukshow_proto_params((*qsys).uuconf_qproto_params, 1 as libc::c_int);
            }
        }
        qsys = (*qsys).uuconf_qalternate;
        i += 1;
        i;
    }
    if fanycall == 0 {
        printf(
            b" Calls will never be placed to this system\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn ikshow_port(
    mut qport: *mut uuconf_port,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qi: *mut sinfo = pinfo as *mut sinfo;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut qmodem: *mut uuconf_modem_port = 0 as *mut uuconf_modem_port;
    let mut qtcp: *mut uuconf_tcp_port = 0 as *mut uuconf_tcp_port;
    let mut qtli: *mut uuconf_tli_port = 0 as *mut uuconf_tli_port;
    let mut qpipe: *mut uuconf_pipe_port = 0 as *mut uuconf_pipe_port;
    (*qi).fgot = 1 as libc::c_int;
    printf(
        b"  Port name %s\n\0" as *const u8 as *const libc::c_char,
        (*qport).uuconf_zname,
    );
    match (*qport).uuconf_ttype as libc::c_uint {
        1 => {
            printf(b"   Port type stdin\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"   Port type direct\n\0" as *const u8 as *const libc::c_char);
            if !((*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice).is_null() {
                printf(
                    b"   Device %s\n\0" as *const u8 as *const libc::c_char,
                    (*qport).uuconf_u.uuconf_sdirect.uuconf_zdevice,
                );
            } else {
                printf(
                    b"   Using port name as device name\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            printf(
                b"   Speed %ld\n\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_u.uuconf_sdirect.uuconf_ibaud,
            );
            printf(
                b"   Carrier %savailable\n\0" as *const u8 as *const libc::c_char,
                if (*qport).uuconf_u.uuconf_sdirect.uuconf_fcarrier != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"not \0" as *const u8 as *const libc::c_char
                },
            );
            printf(
                b"   Hardware flow control %savailable\n\0" as *const u8
                    as *const libc::c_char,
                if (*qport).uuconf_u.uuconf_sdirect.uuconf_fhardflow != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"not \0" as *const u8 as *const libc::c_char
                },
            );
        }
        2 => {
            qmodem = &mut (*qport).uuconf_u.uuconf_smodem;
            printf(b"   Port type modem\n\0" as *const u8 as *const libc::c_char);
            if !((*qmodem).uuconf_zdevice).is_null() {
                printf(
                    b"   Device %s\n\0" as *const u8 as *const libc::c_char,
                    (*qmodem).uuconf_zdevice,
                );
            } else {
                printf(
                    b"   Using port name as device name\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !((*qmodem).uuconf_zdial_device).is_null() {
                printf(
                    b"   Dial device %s\n\0" as *const u8 as *const libc::c_char,
                    (*qmodem).uuconf_zdial_device,
                );
            }
            printf(
                b"   Speed %ld\n\0" as *const u8 as *const libc::c_char,
                (*qmodem).uuconf_ibaud,
            );
            if (*qmodem).uuconf_ilowbaud != (*qmodem).uuconf_ihighbaud {
                printf(
                    b"   Speed range %ld to %ld\n\0" as *const u8 as *const libc::c_char,
                    (*qmodem).uuconf_ilowbaud,
                    (*qmodem).uuconf_ihighbaud,
                );
            }
            printf(
                b"   Carrier %savailable\n\0" as *const u8 as *const libc::c_char,
                if (*qmodem).uuconf_fcarrier != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"not \0" as *const u8 as *const libc::c_char
                },
            );
            printf(
                b"   Hardware flow control %savailable\n\0" as *const u8
                    as *const libc::c_char,
                if (*qmodem).uuconf_fhardflow != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"not \0" as *const u8 as *const libc::c_char
                },
            );
            if !((*qmodem).uuconf_qdialer).is_null() {
                printf(
                    b"   Specially defined dialer\n\0" as *const u8
                        as *const libc::c_char,
                );
                ukshow_dialer((*qmodem).uuconf_qdialer);
            } else if !((*qmodem).uuconf_pzdialer).is_null()
                && !(*((*qmodem).uuconf_pzdialer).offset(0 as libc::c_int as isize))
                    .is_null()
            {
                let mut sdial: uuconf_dialer = uuconf_dialer {
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
                let mut iret: libc::c_int = 0;
                if (*((*qmodem).uuconf_pzdialer).offset(1 as libc::c_int as isize))
                    .is_null()
                    || (*((*qmodem).uuconf_pzdialer).offset(2 as libc::c_int as isize))
                        .is_null()
                {
                    iret = uuconf_dialer_info(
                        (*qi).puuconf,
                        *((*qmodem).uuconf_pzdialer).offset(0 as libc::c_int as isize),
                        &mut sdial,
                    );
                    if iret == 1 as libc::c_int {
                        printf(
                            b"   *** No dialer %s\n\0" as *const u8
                                as *const libc::c_char,
                            *((*qmodem).uuconf_pzdialer)
                                .offset(0 as libc::c_int as isize),
                        );
                    } else if iret != 0 as libc::c_int {
                        ukuuconf_error((*qi).puuconf, iret);
                    } else {
                        printf(
                            b"   Dialer %s\n\0" as *const u8 as *const libc::c_char,
                            *((*qmodem).uuconf_pzdialer)
                                .offset(0 as libc::c_int as isize),
                        );
                        ukshow_dialer(&mut sdial);
                        if !(*((*qmodem).uuconf_pzdialer)
                            .offset(1 as libc::c_int as isize))
                            .is_null()
                        {
                            printf(
                                b"   Token %s\n\0" as *const u8 as *const libc::c_char,
                                *((*qmodem).uuconf_pzdialer)
                                    .offset(1 as libc::c_int as isize),
                            );
                        }
                    }
                } else {
                    pz = (*qmodem).uuconf_pzdialer;
                    while !(*pz).is_null() {
                        iret = uuconf_dialer_info((*qi).puuconf, *pz, &mut sdial);
                        if iret == 1 as libc::c_int {
                            printf(
                                b"   *** No dialer %s\n\0" as *const u8
                                    as *const libc::c_char,
                                *pz,
                            );
                        } else if iret != 0 as libc::c_int {
                            ukuuconf_error((*qi).puuconf, iret);
                        } else {
                            printf(
                                b"   Dialer %s\n\0" as *const u8 as *const libc::c_char,
                                *pz,
                            );
                            ukshow_dialer(&mut sdial);
                        }
                        pz = pz.offset(1);
                        pz;
                        if !(*pz).is_null() {
                            printf(
                                b"   Token %s\n\0" as *const u8 as *const libc::c_char,
                                *pz,
                            );
                            pz = pz.offset(1);
                            pz;
                        }
                    }
                }
            } else {
                printf(
                    b"   *** No dialer information\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        4 => {
            qtcp = &mut (*qport).uuconf_u.uuconf_stcp;
            printf(b"   Port type tcp\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"   TCP service %s\n\0" as *const u8 as *const libc::c_char,
                (*qtcp).uuconf_zport,
            );
            if (*qtcp).uuconf_iversion != 0 as libc::c_int {
                printf(
                    b"   IP version %d\n\0" as *const u8 as *const libc::c_char,
                    (*qtcp).uuconf_iversion,
                );
            }
            if !((*qtcp).uuconf_pzdialer).is_null()
                && !(*((*qtcp).uuconf_pzdialer).offset(0 as libc::c_int as isize))
                    .is_null()
            {
                printf(b"   Dialer sequence\0" as *const u8 as *const libc::c_char);
                pz = (*qtcp).uuconf_pzdialer;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        5 => {
            qtli = &mut (*qport).uuconf_u.uuconf_stli;
            printf(
                b"   Port type TLI%s\n\0" as *const u8 as *const libc::c_char,
                if (*qtli).uuconf_fstream != 0 {
                    b"S\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if !((*qtli).uuconf_zdevice).is_null() {
                printf(
                    b"   Device %s\n\0" as *const u8 as *const libc::c_char,
                    (*qtli).uuconf_zdevice,
                );
            } else {
                printf(
                    b"   Using port name as device name\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !((*qtli).uuconf_pzpush).is_null() {
                printf(b"   Push\0" as *const u8 as *const libc::c_char);
                pz = (*qtli).uuconf_pzpush;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if !((*qtli).uuconf_pzdialer).is_null()
                && !(*((*qtli).uuconf_pzdialer).offset(0 as libc::c_int as isize))
                    .is_null()
            {
                printf(b"   Dialer sequence\0" as *const u8 as *const libc::c_char);
                pz = (*qtli).uuconf_pzdialer;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if !((*qtli).uuconf_zservaddr).is_null() {
                printf(
                    b"   Server address %s\n\0" as *const u8 as *const libc::c_char,
                    (*qtli).uuconf_zservaddr,
                );
            }
        }
        6 => {
            qpipe = &mut (*qport).uuconf_u.uuconf_spipe;
            printf(b"   Port type pipe\n\0" as *const u8 as *const libc::c_char);
            if !((*qpipe).uuconf_pzcmd).is_null() {
                printf(b"   Command\0" as *const u8 as *const libc::c_char);
                pz = (*qpipe).uuconf_pzcmd;
                while !(*pz).is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                    pz = pz.offset(1);
                    pz;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        _ => {
            fprintf(stderr, b"   CAN'T HAPPEN\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if !((*qport).uuconf_zprotocols).is_null() {
        printf(
            b"   Will use protocols %s\n\0" as *const u8 as *const libc::c_char,
            (*qport).uuconf_zprotocols,
        );
    }
    if !((*qport).uuconf_zlockname).is_null() {
        printf(
            b"   Will use lockname %s\n\0" as *const u8 as *const libc::c_char,
            (*qport).uuconf_zlockname,
        );
    }
    if (*qport).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int {
        ukshow_reliable(
            (*qport).uuconf_ireliable,
            b"   \0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*qport).uuconf_qproto_params).is_null() {
        ukshow_proto_params((*qport).uuconf_qproto_params, 3 as libc::c_int);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ukshow_dialer(mut q: *mut uuconf_dialer) {
    ukshow_chat(
        &mut (*q).uuconf_schat,
        b"    Chat\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"    Wait for dialtone %s\n\0" as *const u8 as *const libc::c_char,
        (*q).uuconf_zdialtone,
    );
    printf(
        b"    Pause while dialing %s\n\0" as *const u8 as *const libc::c_char,
        (*q).uuconf_zpause,
    );
    printf(
        b"    Carrier %savailable\n\0" as *const u8 as *const libc::c_char,
        if (*q).uuconf_fcarrier != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"not \0" as *const u8 as *const libc::c_char
        },
    );
    if (*q).uuconf_fcarrier != 0 {
        printf(
            b"    Wait %d seconds for carrier\n\0" as *const u8 as *const libc::c_char,
            (*q).uuconf_ccarrier_wait,
        );
    }
    if (*q).uuconf_fdtr_toggle != 0 {
        printf(b"    Toggle DTR\0" as *const u8 as *const libc::c_char);
        if (*q).uuconf_fdtr_toggle_wait != 0 {
            printf(b" and wait\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    ukshow_chat(
        &mut (*q).uuconf_scomplete,
        b"    When complete chat\0" as *const u8 as *const libc::c_char,
    );
    ukshow_chat(
        &mut (*q).uuconf_sabort,
        b"    When aborting chat\0" as *const u8 as *const libc::c_char,
    );
    if (*q).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int {
        ukshow_reliable(
            (*q).uuconf_ireliable,
            b"   \0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*q).uuconf_qproto_params).is_null() {
        ukshow_proto_params((*q).uuconf_qproto_params, 4 as libc::c_int);
    }
}
unsafe extern "C" fn ukshow_chat(
    mut qchat: *const uuconf_chat,
    mut zhdr: *const libc::c_char,
) {
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !((*qchat).uuconf_pzprogram).is_null() {
        printf(b"%s program\0" as *const u8 as *const libc::c_char, zhdr);
        pz = (*qchat).uuconf_pzprogram;
        while !(*pz).is_null() {
            printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
            pz = pz.offset(1);
            pz;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !((*qchat).uuconf_pzchat).is_null() {
        printf(b"%s script\0" as *const u8 as *const libc::c_char, zhdr);
        pz = (*qchat).uuconf_pzchat;
        while !(*pz).is_null() {
            if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                || pz == (*qchat).uuconf_pzchat
            {
                printf(b" \0" as *const u8 as *const libc::c_char);
            }
            printf(b"%s\0" as *const u8 as *const libc::c_char, *pz);
            pz = pz.offset(1);
            pz;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"%s script timeout %d\n\0" as *const u8 as *const libc::c_char,
            zhdr,
            (*qchat).uuconf_ctimeout,
        );
        if !((*qchat).uuconf_pzfail).is_null() {
            printf(b"%s failure strings\0" as *const u8 as *const libc::c_char, zhdr);
            pz = (*qchat).uuconf_pzfail;
            while !(*pz).is_null() {
                printf(b" %s\0" as *const u8 as *const libc::c_char, *pz);
                pz = pz.offset(1);
                pz;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if (*qchat).uuconf_fstrip != 0 {
            printf(
                b"%s script incoming bytes stripped to seven bits\n\0" as *const u8
                    as *const libc::c_char,
                zhdr,
            );
        }
    }
}
unsafe extern "C" fn ukshow_size(
    mut qspan: *mut uuconf_timespan,
    mut fcall: boolean,
    mut flocal: boolean,
) {
    let mut q: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
    let mut fother: boolean = 0;
    qspan = qcompress_span(qspan);
    if qspan.is_null() {
        return;
    }
    printf(
        b" If call%s the following applies to a %s request:\n\0" as *const u8
            as *const libc::c_char,
        if fcall != 0 {
            b"ing\0" as *const u8 as *const libc::c_char
        } else {
            b"ed\0" as *const u8 as *const libc::c_char
        },
        if flocal != 0 {
            b"local\0" as *const u8 as *const libc::c_char
        } else {
            b"remote\0" as *const u8 as *const libc::c_char
        },
    );
    fother = 0 as libc::c_int;
    if (*qspan).uuconf_istart >= 60 as libc::c_int {
        fother = 1 as libc::c_int;
    }
    q = qspan;
    while !q.is_null() {
        printf(b"  \0" as *const u8 as *const libc::c_char);
        ukshow_time(q);
        printf(
            b" may transfer files %ld bytes or smaller\n\0" as *const u8
                as *const libc::c_char,
            (*q).uuconf_ival,
        );
        if ((*q).uuconf_qnext).is_null() {
            if (*q).uuconf_iend
                <= 6 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                    + 23 as libc::c_int * 60 as libc::c_int
            {
                fother = 1 as libc::c_int;
            }
        } else if (*q).uuconf_iend + 60 as libc::c_int
            <= (*(*q).uuconf_qnext).uuconf_istart
        {
            fother = 1 as libc::c_int;
        }
        q = (*q).uuconf_qnext;
    }
    if fother != 0 {
        printf(
            b"  (At other times may send files of any size)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn ukshow_reliable(mut i: libc::c_int, mut zhdr: *const libc::c_char) {
    printf(b"%sCharacteristics:\0" as *const u8 as *const libc::c_char, zhdr);
    if i & 0o2 as libc::c_int != 0 as libc::c_int {
        printf(b" eight-bit-clean\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b" not-eight-bit-clean\0" as *const u8 as *const libc::c_char);
    }
    if i & 0o4 as libc::c_int != 0 as libc::c_int {
        printf(b" reliable\0" as *const u8 as *const libc::c_char);
    }
    if i & 0o10 as libc::c_int != 0 as libc::c_int {
        printf(b" end-to-end\0" as *const u8 as *const libc::c_char);
    }
    if i & 0o20 as libc::c_int != 0 as libc::c_int {
        printf(b" fullduplex\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b" halfduplex\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ukshow_proto_params(
    mut pas: *mut uuconf_proto_param,
    mut cindent: libc::c_int,
) {
    let mut q: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
    q = pas;
    while (*q).uuconf_bproto != '\0' as i32 {
        let mut i: libc::c_int = 0;
        let mut qe: *mut uuconf_proto_param_entry = 0 as *mut uuconf_proto_param_entry;
        i = 0 as libc::c_int;
        while i < cindent {
            printf(b" \0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
        printf(
            b"For protocol %c will use the following parameters\n\0" as *const u8
                as *const libc::c_char,
            (*q).uuconf_bproto,
        );
        qe = (*q).uuconf_qentries;
        while (*qe).uuconf_cargs > 0 as libc::c_int {
            let mut ia: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < cindent {
                printf(b" \0" as *const u8 as *const libc::c_char);
                i += 1;
                i;
            }
            ia = 0 as libc::c_int;
            while ia < (*qe).uuconf_cargs {
                printf(
                    b" %s\0" as *const u8 as *const libc::c_char,
                    *((*qe).uuconf_pzargs).offset(ia as isize),
                );
                ia += 1;
                ia;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            qe = qe.offset(1);
            qe;
        }
        q = q.offset(1);
        q;
    }
}
unsafe extern "C" fn ukshow_time(mut q: *const uuconf_timespan) {
    let mut idaystart: libc::c_int = 0;
    let mut idayend: libc::c_int = 0;
    let mut ihourstart: libc::c_int = 0;
    let mut ihourend: libc::c_int = 0;
    let mut iminutestart: libc::c_int = 0;
    let mut iminuteend: libc::c_int = 0;
    let zdays: *const libc::c_char = b"Sun\0Mon\0Tue\0Wed\0Thu\0Fri\0Sat\0Sun\0"
        as *const u8 as *const libc::c_char;
    if (*q).uuconf_istart == 0 as libc::c_int
        && (*q).uuconf_iend == 7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
    {
        printf(b"At any time\0" as *const u8 as *const libc::c_char);
        return;
    }
    idaystart = (*q).uuconf_istart / (24 as libc::c_int * 60 as libc::c_int);
    ihourstart = (*q).uuconf_istart % (24 as libc::c_int * 60 as libc::c_int)
        / 60 as libc::c_int;
    iminutestart = (*q).uuconf_istart % 60 as libc::c_int;
    idayend = (*q).uuconf_iend / (24 as libc::c_int * 60 as libc::c_int);
    ihourend = (*q).uuconf_iend % (24 as libc::c_int * 60 as libc::c_int)
        / 60 as libc::c_int;
    iminuteend = (*q).uuconf_iend % 60 as libc::c_int;
    if idaystart == idayend {
        printf(
            b"%s from %02d:%02d to %02d:%02d\0" as *const u8 as *const libc::c_char,
            zdays.offset((idaystart * 4 as libc::c_int) as isize),
            ihourstart,
            iminutestart,
            ihourend,
            iminuteend,
        );
    } else {
        printf(
            b"From %s %02d:%02d to %s %02d:%02d\0" as *const u8 as *const libc::c_char,
            zdays.offset((idaystart * 4 as libc::c_int) as isize),
            ihourstart,
            iminutestart,
            zdays.offset((idayend * 4 as libc::c_int) as isize),
            ihourend,
            iminuteend,
        );
    };
}
unsafe extern "C" fn qcompress_span(
    mut qlist: *mut uuconf_timespan,
) -> *mut uuconf_timespan {
    let mut pq: *mut *mut uuconf_timespan = 0 as *mut *mut uuconf_timespan;
    pq = &mut qlist;
    while !(*pq).is_null() {
        if !((**pq).uuconf_qnext).is_null()
            && (**pq).uuconf_iend == (*(**pq).uuconf_qnext).uuconf_istart
            && (**pq).uuconf_ival == (*(**pq).uuconf_qnext).uuconf_ival
        {
            let mut qnext: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
            qnext = (**pq).uuconf_qnext;
            (**pq).uuconf_qnext = (*qnext).uuconf_qnext;
            (**pq).uuconf_iend = (*qnext).uuconf_iend;
        } else {
            pq = &mut (**pq).uuconf_qnext;
        }
    }
    return qlist;
}
unsafe extern "C" fn ukuuconf_error(mut puuconf: pointer, mut iret: libc::c_int) {
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
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            zKprogram,
            ab.as_mut_ptr(),
        );
    } else {
        fprintf(
            stderr,
            b"%s:%s\n\0" as *const u8 as *const libc::c_char,
            zKprogram,
            ab.as_mut_ptr(),
        );
    }
    exit(1 as libc::c_int);
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

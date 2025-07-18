use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn _getopt_long_r(
        ___argc: libc::c_int,
        ___argv: *mut *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const rpl_option,
        __longind: *mut libc::c_int,
        __data: *mut _getopt_data,
    ) -> libc::c_int;
    fn _getopt_long_only_r(
        ___argc: libc::c_int,
        ___argv: *mut *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const rpl_option,
        __longind: *mut libc::c_int,
        __data: *mut _getopt_data,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut program_invocation_name: *mut libc::c_char;
    static mut program_invocation_short_name: *mut libc::c_char;
    fn argp_state_help(
        __state: *const argp_state,
        __stream: *mut FILE,
        __flags: libc::c_uint,
    );
    fn argp_error(__state: *const argp_state, __fmt: *const libc::c_char, _: ...);
    static mut argp_program_version_hook: Option::<
        unsafe extern "C" fn(*mut FILE, *mut argp_state) -> (),
    >;
    static mut argp_program_version: *const libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __ord = libc::c_uint;
pub const RETURN_IN_ORDER: __ord = 2;
pub const PERMUTE: __ord = 1;
pub const REQUIRE_ORDER: __ord = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub rpl_optind: libc::c_int,
    pub rpl_opterr: libc::c_int,
    pub rpl_optopt: libc::c_int,
    pub rpl_optarg: *mut libc::c_char,
    pub __initialized: libc::c_int,
    pub __nextchar: *mut libc::c_char,
    pub __ordering: __ord,
    pub __first_nonopt: libc::c_int,
    pub __last_nonopt: libc::c_int,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type error_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const libc::c_char,
    pub doc: *const libc::c_char,
    pub children: *const argp_child,
    pub help_filter: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> *mut libc::c_char,
    >,
    pub argp_domain: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: libc::c_int,
    pub header: *const libc::c_char,
    pub group: libc::c_int,
}
pub type argp_parser_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub next: libc::c_int,
    pub flags: libc::c_uint,
    pub arg_num: libc::c_uint,
    pub quoted: libc::c_int,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser {
    pub argp: *const argp,
    pub short_opts: *mut libc::c_char,
    pub long_opts: *mut rpl_option,
    pub opt_data: _getopt_data,
    pub groups: *mut group,
    pub egroup: *mut group,
    pub child_inputs: *mut *mut libc::c_void,
    pub try_getopt: libc::c_int,
    pub state: argp_state,
    pub storage: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub parser: argp_parser_t,
    pub argp: *const argp,
    pub short_end: *mut libc::c_char,
    pub args_processed: libc::c_uint,
    pub parent: *mut group,
    pub parent_index: libc::c_uint,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_convert_state {
    pub parser: *mut parser,
    pub short_end: *mut libc::c_char,
    pub long_end: *mut rpl_option,
    pub child_inputs_end: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_sizes {
    pub short_len: size_t,
    pub long_len: size_t,
    pub num_groups: size_t,
    pub num_child_inputs: size_t,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _option_is_short(mut __opt: *const argp_option) -> libc::c_int {
    if (*__opt).flags & 0x8 as libc::c_int != 0 {
        return 0 as libc::c_int
    } else {
        let mut __key: libc::c_int = (*__opt).key;
        return (__key > 0 as libc::c_int
            && __key <= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            && *(*__ctype_b_loc()).offset(__key as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0)
            as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn _option_is_end(mut __opt: *const argp_option) -> libc::c_int {
    return ((*__opt).key == 0 && ((*__opt).name).is_null() && ((*__opt).doc).is_null()
        && (*__opt).group == 0) as libc::c_int;
}
static mut _argp_hang: libc::c_int = 0;
static mut argp_default_options: [argp_option; 5] = [
    {
        let mut init = argp_option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            key: '?' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"give this help list\0" as *const u8 as *const libc::c_char,
            group: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"usage\0" as *const u8 as *const libc::c_char,
            key: -(3 as libc::c_int),
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"give a short usage message\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"program-name\0" as *const u8 as *const libc::c_char,
            key: -(2 as libc::c_int),
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"set the program name\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"HANG\0" as *const u8 as *const libc::c_char,
            key: -(4 as libc::c_int),
            arg: b"SECS\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
            doc: b"hang for SECS seconds (default 3600)\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn argp_default_parser(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        63 => {
            argp_state_help(
                state,
                (*state).out_stream,
                (0x2 as libc::c_int | 0x8 as libc::c_int | 0x200 as libc::c_int
                    | (0x10 as libc::c_int | 0x20 as libc::c_int) | 0x40 as libc::c_int)
                    as libc::c_uint,
            );
        }
        -3 => {
            argp_state_help(
                state,
                (*state).out_stream,
                (0x1 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint,
            );
        }
        -2 => {
            program_invocation_name = arg;
            (*state).name = last_component(arg);
            program_invocation_short_name = (*state).name;
            if (*state).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint
                == 0x1 as libc::c_int as libc::c_uint
            {
                let ref mut fresh0 = *((*state).argv).offset(0 as libc::c_int as isize);
                *fresh0 = arg;
            }
        }
        -4 => {
            ::std::ptr::write_volatile(
                &mut _argp_hang as *mut libc::c_int,
                atoi(
                    if !arg.is_null() {
                        arg as *const libc::c_char
                    } else {
                        b"3600\0" as *const u8 as *const libc::c_char
                    },
                ),
            );
            loop {
                let fresh1 = ::std::ptr::read_volatile::<
                    libc::c_int,
                >(&_argp_hang as *const libc::c_int);
                ::std::ptr::write_volatile(
                    &mut _argp_hang as *mut libc::c_int,
                    ::std::ptr::read_volatile::<
                        libc::c_int,
                    >(&_argp_hang as *const libc::c_int) - 1,
                );
                if !(fresh1 > 0 as libc::c_int) {
                    break;
                }
                sleep(1 as libc::c_int as libc::c_uint);
            }
        }
        _ => return 7 as libc::c_int,
    }
    return 0 as libc::c_int;
}
static mut argp_default_argp: argp = unsafe {
    {
        let mut init = argp {
            options: argp_default_options.as_ptr(),
            parser: Some(
                argp_default_parser
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: 0 as *const libc::c_char,
            doc: 0 as *const libc::c_char,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: b"libc\0" as *const u8 as *const libc::c_char,
        };
        init
    }
};
static mut argp_version_options: [argp_option; 2] = [
    {
        let mut init = argp_option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            key: 'V' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"print program version\0" as *const u8 as *const libc::c_char,
            group: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn argp_version_parser(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        86 => {
            if argp_program_version_hook.is_some() {
                (Some(argp_program_version_hook.unwrap()))
                    .unwrap()((*state).out_stream, state);
            } else if !argp_program_version.is_null() {
                fprintf(
                    (*state).out_stream,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    argp_program_version,
                );
            } else {
                argp_error(
                    state,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        (*(*state).root_argp).argp_domain,
                        b"(PROGRAM ERROR) No version known!?\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if (*state).flags & 0x20 as libc::c_int as libc::c_uint == 0 {
                exit(0 as libc::c_int);
            }
        }
        _ => return 7 as libc::c_int,
    }
    return 0 as libc::c_int;
}
static mut argp_version_argp: argp = unsafe {
    {
        let mut init = argp {
            options: argp_version_options.as_ptr(),
            parser: Some(
                argp_version_parser
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: 0 as *const libc::c_char,
            doc: 0 as *const libc::c_char,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: b"libc\0" as *const u8 as *const libc::c_char,
        };
        init
    }
};
unsafe extern "C" fn find_long_option(
    mut long_options: *mut rpl_option,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut l: *mut rpl_option = long_options;
    while !((*l).name).is_null() {
        if !name.is_null() && strcmp((*l).name, name) == 0 as libc::c_int {
            return l.offset_from(long_options) as libc::c_long as libc::c_int
        } else {
            l = l.offset(1);
            l;
        }
    }
    if name.is_null() {
        return l.offset_from(long_options) as libc::c_long as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn group_parse(
    mut group: *mut group,
    mut state: *mut argp_state,
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
) -> error_t {
    if ((*group).parser).is_some() {
        let mut err: error_t = 0;
        (*state).hook = (*group).hook;
        (*state).input = (*group).input;
        (*state).child_inputs = (*group).child_inputs;
        (*state).arg_num = (*group).args_processed;
        err = (Some(((*group).parser).unwrap())).unwrap()(key, arg, state);
        (*group).hook = (*state).hook;
        return err;
    } else {
        return 7 as libc::c_int
    };
}
unsafe extern "C" fn convert_options(
    mut argp: *const argp,
    mut parent: *mut group,
    mut parent_index: libc::c_uint,
    mut group: *mut group,
    mut cvt: *mut parser_convert_state,
) -> *mut group {
    let mut real: *const argp_option = (*argp).options;
    let mut children: *const argp_child = (*argp).children;
    if !real.is_null() || ((*argp).parser).is_some() {
        let mut opt: *const argp_option = 0 as *const argp_option;
        if !real.is_null() {
            opt = real;
            while _option_is_end(opt) == 0 {
                if (*opt).flags & 0x4 as libc::c_int == 0 {
                    real = opt;
                }
                if (*real).flags & 0x8 as libc::c_int == 0 {
                    if _option_is_short(opt) != 0 {
                        let fresh2 = (*cvt).short_end;
                        (*cvt).short_end = ((*cvt).short_end).offset(1);
                        *fresh2 = (*opt).key as libc::c_char;
                        if !((*real).arg).is_null() {
                            let fresh3 = (*cvt).short_end;
                            (*cvt).short_end = ((*cvt).short_end).offset(1);
                            *fresh3 = ':' as i32 as libc::c_char;
                            if (*real).flags & 0x1 as libc::c_int != 0 {
                                let fresh4 = (*cvt).short_end;
                                (*cvt).short_end = ((*cvt).short_end).offset(1);
                                *fresh4 = ':' as i32 as libc::c_char;
                            }
                        }
                        *(*cvt).short_end = '\0' as i32 as libc::c_char;
                    }
                    if !((*opt).name).is_null()
                        && find_long_option((*(*cvt).parser).long_opts, (*opt).name)
                            < 0 as libc::c_int
                    {
                        (*(*cvt).long_end).name = (*opt).name;
                        (*(*cvt).long_end)
                            .has_arg = if !((*real).arg).is_null() {
                            if (*real).flags & 0x1 as libc::c_int != 0 {
                                2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            }
                        } else {
                            0 as libc::c_int
                        };
                        (*(*cvt).long_end).flag = 0 as *mut libc::c_int;
                        (*(*cvt).long_end)
                            .val = (((if (*opt).key != 0 {
                            (*opt).key
                        } else {
                            (*real).key
                        })
                            & ((1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(8 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) as libc::c_long
                            + ((group.offset_from((*(*cvt).parser).groups)
                                as libc::c_long + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)))
                            as libc::c_int;
                        (*cvt).long_end = ((*cvt).long_end).offset(1);
                        (*(*cvt).long_end).name = 0 as *const libc::c_char;
                    }
                }
                opt = opt.offset(1);
                opt;
            }
        }
        (*group).parser = (*argp).parser;
        (*group).argp = argp;
        (*group).short_end = (*cvt).short_end;
        (*group).args_processed = 0 as libc::c_int as libc::c_uint;
        (*group).parent = parent;
        (*group).parent_index = parent_index;
        (*group).input = 0 as *mut libc::c_void;
        (*group).hook = 0 as *mut libc::c_void;
        (*group).child_inputs = 0 as *mut *mut libc::c_void;
        if !children.is_null() {
            let mut num_children: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while !((*children.offset(num_children as isize)).argp).is_null() {
                num_children = num_children.wrapping_add(1);
                num_children;
            }
            (*group).child_inputs = (*cvt).child_inputs_end;
            (*cvt)
                .child_inputs_end = ((*cvt).child_inputs_end)
                .offset(num_children as isize);
        }
        let fresh5 = group;
        group = group.offset(1);
        parent = fresh5;
    } else {
        parent = 0 as *mut group;
    }
    if !children.is_null() {
        let mut index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while !((*children).argp).is_null() {
            let fresh6 = children;
            children = children.offset(1);
            let fresh7 = index;
            index = index.wrapping_add(1);
            group = convert_options((*fresh6).argp, parent, fresh7, group, cvt);
        }
    }
    return group;
}
unsafe extern "C" fn parser_convert(
    mut parser: *mut parser,
    mut argp: *const argp,
    mut flags: libc::c_int,
) {
    let mut cvt: parser_convert_state = parser_convert_state {
        parser: 0 as *mut parser,
        short_end: 0 as *mut libc::c_char,
        long_end: 0 as *mut rpl_option,
        child_inputs_end: 0 as *mut *mut libc::c_void,
    };
    cvt.parser = parser;
    cvt.short_end = (*parser).short_opts;
    cvt.long_end = (*parser).long_opts;
    cvt.child_inputs_end = (*parser).child_inputs;
    if flags & 0x8 as libc::c_int != 0 {
        let fresh8 = cvt.short_end;
        cvt.short_end = (cvt.short_end).offset(1);
        *fresh8 = '-' as i32 as libc::c_char;
    } else if flags & 0x4 as libc::c_int != 0 {
        let fresh9 = cvt.short_end;
        cvt.short_end = (cvt.short_end).offset(1);
        *fresh9 = '+' as i32 as libc::c_char;
    }
    *cvt.short_end = '\0' as i32 as libc::c_char;
    (*cvt.long_end).name = 0 as *const libc::c_char;
    (*parser).argp = argp;
    if !argp.is_null() {
        (*parser)
            .egroup = convert_options(
            argp,
            0 as *mut group,
            0 as libc::c_int as libc::c_uint,
            (*parser).groups,
            &mut cvt,
        );
    } else {
        (*parser).egroup = (*parser).groups;
    };
}
unsafe extern "C" fn calc_sizes(mut argp: *const argp, mut szs: *mut parser_sizes) {
    let mut child: *const argp_child = (*argp).children;
    let mut opt: *const argp_option = (*argp).options;
    if !opt.is_null() || ((*argp).parser).is_some() {
        (*szs).num_groups = ((*szs).num_groups).wrapping_add(1);
        (*szs).num_groups;
        if !opt.is_null() {
            let mut num_opts: libc::c_int = 0 as libc::c_int;
            loop {
                let fresh10 = opt;
                opt = opt.offset(1);
                if !(_option_is_end(fresh10) == 0) {
                    break;
                }
                num_opts += 1;
                num_opts;
            }
            (*szs)
                .short_len = ((*szs).short_len as libc::c_ulong)
                .wrapping_add((num_opts * 3 as libc::c_int) as libc::c_ulong) as size_t
                as size_t;
            (*szs)
                .long_len = ((*szs).long_len as libc::c_ulong)
                .wrapping_add(num_opts as libc::c_ulong) as size_t as size_t;
        }
    }
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let fresh11 = child;
            child = child.offset(1);
            calc_sizes((*fresh11).argp, szs);
            (*szs).num_child_inputs = ((*szs).num_child_inputs).wrapping_add(1);
            (*szs).num_child_inputs;
        }
    }
}
unsafe extern "C" fn parser_init(
    mut parser: *mut parser,
    mut argp: *const argp,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut flags: libc::c_int,
    mut input: *mut libc::c_void,
) -> error_t {
    let mut err: error_t = 0 as libc::c_int;
    let mut group: *mut group = 0 as *mut group;
    let mut szs: parser_sizes = parser_sizes {
        short_len: 0,
        long_len: 0,
        num_groups: 0,
        num_child_inputs: 0,
    };
    let mut opt_data: _getopt_data = {
        let mut init = _getopt_data {
            rpl_optind: 1 as libc::c_int,
            rpl_opterr: 1 as libc::c_int,
            rpl_optopt: 0,
            rpl_optarg: 0 as *mut libc::c_char,
            __initialized: 0,
            __nextchar: 0 as *mut libc::c_char,
            __ordering: REQUIRE_ORDER,
            __first_nonopt: 0,
            __last_nonopt: 0,
        };
        init
    };
    let mut storage: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut glen: size_t = 0;
    let mut gsum: size_t = 0;
    let mut clen: size_t = 0;
    let mut csum: size_t = 0;
    let mut llen: size_t = 0;
    let mut lsum: size_t = 0;
    let mut slen: size_t = 0;
    let mut ssum: size_t = 0;
    szs
        .short_len = (if flags & 0x4 as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as size_t;
    szs.long_len = 0 as libc::c_int as size_t;
    szs.num_groups = 0 as libc::c_int as size_t;
    szs.num_child_inputs = 0 as libc::c_int as size_t;
    if !argp.is_null() {
        calc_sizes(argp, &mut szs);
    }
    glen = (szs.num_groups)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<group>() as libc::c_ulong);
    clen = (szs.num_child_inputs)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
    llen = (szs.long_len)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<rpl_option>() as libc::c_ulong);
    slen = (szs.short_len).wrapping_add(1 as libc::c_int as libc::c_ulong);
    gsum = glen;
    csum = gsum
        .wrapping_add(clen)
        .wrapping_add(::std::mem::align_of::<rpl_option>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::align_of::<rpl_option>() as libc::c_ulong)
        .wrapping_mul(::std::mem::align_of::<rpl_option>() as libc::c_ulong);
    lsum = csum.wrapping_add(llen);
    ssum = lsum.wrapping_add(slen);
    (*parser).storage = malloc(ssum);
    if ((*parser).storage).is_null() {
        return 12 as libc::c_int;
    }
    storage = (*parser).storage as *mut libc::c_char;
    (*parser).groups = (*parser).storage as *mut group;
    (*parser).child_inputs = storage.offset(gsum as isize) as *mut *mut libc::c_void;
    (*parser).long_opts = storage.offset(csum as isize) as *mut rpl_option;
    (*parser).short_opts = storage.offset(lsum as isize);
    (*parser).opt_data = opt_data;
    memset((*parser).child_inputs as *mut libc::c_void, 0 as libc::c_int, clen);
    parser_convert(parser, argp, flags);
    memset(
        &mut (*parser).state as *mut argp_state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<argp_state>() as libc::c_ulong,
    );
    (*parser).state.root_argp = (*parser).argp;
    (*parser).state.argc = argc;
    (*parser).state.argv = argv;
    (*parser).state.flags = flags as libc::c_uint;
    (*parser).state.err_stream = stderr;
    (*parser).state.out_stream = stdout;
    (*parser).state.next = 0 as libc::c_int;
    (*parser).state.pstate = parser as *mut libc::c_void;
    (*parser).try_getopt = 1 as libc::c_int;
    if (*parser).groups < (*parser).egroup {
        (*(*parser).groups).input = input;
    }
    group = (*parser).groups;
    while group < (*parser).egroup && (err == 0 || err == 7 as libc::c_int) {
        if !((*group).parent).is_null() {
            (*group)
                .input = *((*(*group).parent).child_inputs)
                .offset((*group).parent_index as isize);
        }
        if ((*group).parser).is_none() && !((*(*group).argp).children).is_null()
            && !((*(*(*group).argp).children).argp).is_null()
        {
            let ref mut fresh12 = *((*group).child_inputs)
                .offset(0 as libc::c_int as isize);
            *fresh12 = (*group).input;
        }
        err = group_parse(
            group,
            &mut (*parser).state,
            0x1000003 as libc::c_int,
            0 as *mut libc::c_char,
        );
        group = group.offset(1);
        group;
    }
    if err == 7 as libc::c_int {
        err = 0 as libc::c_int;
    }
    if err != 0 {
        return err;
    }
    if (*parser).state.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        (*parser).opt_data.rpl_opterr = 0 as libc::c_int;
        if (*parser).state.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            (*parser).state.argv = ((*parser).state.argv).offset(-1);
            (*parser).state.argv;
            (*parser).state.argc += 1;
            (*parser).state.argc;
        }
    } else {
        (*parser).opt_data.rpl_opterr = 1 as libc::c_int;
    }
    if (*parser).state.argv == argv
        && !(*argv.offset(0 as libc::c_int as isize)).is_null()
    {
        (*parser).state.name = last_component(*argv.offset(0 as libc::c_int as isize));
    } else {
        (*parser).state.name = program_invocation_short_name;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parser_finalize(
    mut parser: *mut parser,
    mut err: error_t,
    mut arg_ebadkey: libc::c_int,
    mut end_index: *mut libc::c_int,
) -> error_t {
    let mut group: *mut group = 0 as *mut group;
    if err == 7 as libc::c_int && arg_ebadkey != 0 {
        err = 0 as libc::c_int;
    }
    if err == 0 {
        if (*parser).state.next == (*parser).state.argc {
            group = (*parser).groups;
            while group < (*parser).egroup && (err == 0 || err == 7 as libc::c_int) {
                if (*group).args_processed == 0 as libc::c_int as libc::c_uint {
                    err = group_parse(
                        group,
                        &mut (*parser).state,
                        0x1000002 as libc::c_int,
                        0 as *mut libc::c_char,
                    );
                }
                group = group.offset(1);
                group;
            }
            group = ((*parser).egroup).offset(-(1 as libc::c_int as isize));
            while group >= (*parser).groups && (err == 0 || err == 7 as libc::c_int) {
                err = group_parse(
                    group,
                    &mut (*parser).state,
                    0x1000001 as libc::c_int,
                    0 as *mut libc::c_char,
                );
                group = group.offset(-1);
                group;
            }
            if err == 7 as libc::c_int {
                err = 0 as libc::c_int;
            }
            if !end_index.is_null() {
                *end_index = (*parser).state.next;
            }
        } else if !end_index.is_null() {
            *end_index = (*parser).state.next;
        } else {
            if (*parser).state.flags & 0x2 as libc::c_int as libc::c_uint == 0
                && !((*parser).state.err_stream).is_null()
            {
                fprintf(
                    (*parser).state.err_stream,
                    dcgettext(
                        (*(*parser).argp).argp_domain,
                        b"%s: Too many arguments\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*parser).state.name,
                );
            }
            err = 7 as libc::c_int;
        }
    }
    if err != 0 {
        if err == 7 as libc::c_int {
            argp_state_help(
                &mut (*parser).state,
                (*parser).state.err_stream,
                (0x4 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint,
            );
        }
        group = (*parser).groups;
        while group < (*parser).egroup {
            group_parse(
                group,
                &mut (*parser).state,
                0x1000005 as libc::c_int,
                0 as *mut libc::c_char,
            );
            group = group.offset(1);
            group;
        }
    } else {
        group = ((*parser).egroup).offset(-(1 as libc::c_int as isize));
        while group >= (*parser).groups && (err == 0 || err == 7 as libc::c_int) {
            err = group_parse(
                group,
                &mut (*parser).state,
                0x1000004 as libc::c_int,
                0 as *mut libc::c_char,
            );
            group = group.offset(-1);
            group;
        }
        if err == 7 as libc::c_int {
            err = 0 as libc::c_int;
        }
    }
    group = ((*parser).egroup).offset(-(1 as libc::c_int as isize));
    while group >= (*parser).groups {
        group_parse(
            group,
            &mut (*parser).state,
            0x1000007 as libc::c_int,
            0 as *mut libc::c_char,
        );
        group = group.offset(-1);
        group;
    }
    if err == 7 as libc::c_int {
        err = 22 as libc::c_int;
    }
    rpl_free((*parser).storage);
    return err;
}
unsafe extern "C" fn parser_parse_arg(
    mut parser: *mut parser,
    mut val: *mut libc::c_char,
) -> error_t {
    (*parser).state.next -= 1;
    let mut index: libc::c_int = (*parser).state.next;
    let mut err: error_t = 7 as libc::c_int;
    let mut group: *mut group = 0 as *mut group;
    let mut key: libc::c_int = 0 as libc::c_int;
    group = (*parser).groups;
    while group < (*parser).egroup && err == 7 as libc::c_int {
        (*parser).state.next += 1;
        (*parser).state.next;
        key = 0 as libc::c_int;
        err = group_parse(group, &mut (*parser).state, key, val);
        if err == 7 as libc::c_int {
            (*parser).state.next -= 1;
            (*parser).state.next;
            key = 0x1000006 as libc::c_int;
            err = group_parse(group, &mut (*parser).state, key, 0 as *mut libc::c_char);
        }
        group = group.offset(1);
        group;
    }
    if err == 0 {
        if key == 0x1000006 as libc::c_int {
            (*parser).state.next = (*parser).state.argc;
        }
        if (*parser).state.next > index {
            group = group.offset(-1);
            (*group)
                .args_processed = ((*group).args_processed)
                .wrapping_add(((*parser).state.next - index) as libc::c_uint);
        } else {
            (*parser).try_getopt = 1 as libc::c_int;
        }
    }
    return err;
}
unsafe extern "C" fn parser_parse_opt(
    mut parser: *mut parser,
    mut opt: libc::c_int,
    mut val: *mut libc::c_char,
) -> error_t {
    let mut group_key: libc::c_int = opt
        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong);
    let mut err: error_t = 7 as libc::c_int;
    if group_key == 0 as libc::c_int {
        let mut group: *mut group = 0 as *mut group;
        let mut short_index: *mut libc::c_char = strchr((*parser).short_opts, opt);
        if !short_index.is_null() {
            group = (*parser).groups;
            while group < (*parser).egroup {
                if (*group).short_end > short_index {
                    err = group_parse(
                        group,
                        &mut (*parser).state,
                        opt,
                        (*parser).opt_data.rpl_optarg,
                    );
                    break;
                } else {
                    group = group.offset(1);
                    group;
                }
            }
        }
    } else {
        let mut user_key: libc::c_int = (if opt
            & (1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
        {
            !(((1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
        } else {
            0 as libc::c_int
        })
            | opt
                & ((1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int;
        err = group_parse(
            &mut *((*parser).groups).offset((group_key - 1 as libc::c_int) as isize),
            &mut (*parser).state,
            user_key,
            (*parser).opt_data.rpl_optarg,
        );
    }
    if err == 7 as libc::c_int {
        static mut bad_key_err: [libc::c_char; 53] = unsafe {
            *::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"(PROGRAM ERROR) Option should have been recognized!?\0")
        };
        if group_key == 0 as libc::c_int {
            argp_error(
                &mut (*parser).state as *mut argp_state,
                b"-%c: %s\0" as *const u8 as *const libc::c_char,
                opt,
                dcgettext(
                    (*(*parser).argp).argp_domain,
                    bad_key_err.as_ptr(),
                    5 as libc::c_int,
                ),
            );
        } else {
            let mut long_opt: *mut rpl_option = (*parser).long_opts;
            while (*long_opt).val != opt && !((*long_opt).name).is_null() {
                long_opt = long_opt.offset(1);
                long_opt;
            }
            argp_error(
                &mut (*parser).state as *mut argp_state,
                b"--%s: %s\0" as *const u8 as *const libc::c_char,
                if !((*long_opt).name).is_null() {
                    (*long_opt).name
                } else {
                    b"???\0" as *const u8 as *const libc::c_char
                },
                dcgettext(
                    (*(*parser).argp).argp_domain,
                    bad_key_err.as_ptr(),
                    5 as libc::c_int,
                ),
            );
        }
    }
    return err;
}
unsafe extern "C" fn parser_parse_next(
    mut parser: *mut parser,
    mut arg_ebadkey: *mut libc::c_int,
) -> error_t {
    let mut opt: libc::c_int = 0;
    let mut err: error_t = 0 as libc::c_int;
    if (*parser).state.quoted != 0 && (*parser).state.next < (*parser).state.quoted {
        (*parser).state.quoted = 0 as libc::c_int;
    }
    if (*parser).try_getopt != 0 && (*parser).state.quoted == 0 {
        (*parser).opt_data.rpl_optind = (*parser).state.next;
        (*parser).opt_data.rpl_optopt = -(1 as libc::c_int);
        if (*parser).state.flags & 0x40 as libc::c_int as libc::c_uint != 0 {
            opt = _getopt_long_only_r(
                (*parser).state.argc,
                (*parser).state.argv,
                (*parser).short_opts,
                (*parser).long_opts,
                0 as *mut libc::c_int,
                &mut (*parser).opt_data,
            );
        } else {
            opt = _getopt_long_r(
                (*parser).state.argc,
                (*parser).state.argv,
                (*parser).short_opts,
                (*parser).long_opts,
                0 as *mut libc::c_int,
                &mut (*parser).opt_data,
            );
        }
        (*parser).state.next = (*parser).opt_data.rpl_optind;
        if opt == -(1 as libc::c_int) {
            (*parser).try_getopt = 0 as libc::c_int;
            if (*parser).state.next > 1 as libc::c_int
                && strcmp(
                    *((*parser).state.argv)
                        .offset(((*parser).state.next - 1 as libc::c_int) as isize),
                    b"--\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                (*parser).state.quoted = (*parser).state.next;
            }
        } else if opt == '?' as i32
            && (*parser).opt_data.rpl_optopt != -(1 as libc::c_int)
        {
            *arg_ebadkey = 0 as libc::c_int;
            return 7 as libc::c_int;
        }
    } else {
        opt = -(1 as libc::c_int);
    }
    if opt == -(1 as libc::c_int) {
        if (*parser).state.next >= (*parser).state.argc
            || (*parser).state.flags & 0x4 as libc::c_int as libc::c_uint != 0
        {
            *arg_ebadkey = 1 as libc::c_int;
            return 7 as libc::c_int;
        } else {
            opt = 1 as libc::c_int;
            let fresh13 = (*parser).state.next;
            (*parser).state.next = (*parser).state.next + 1;
            (*parser)
                .opt_data
                .rpl_optarg = *((*parser).state.argv).offset(fresh13 as isize);
        }
    }
    if opt == 1 as libc::c_int {
        err = parser_parse_arg(parser, (*parser).opt_data.rpl_optarg);
    } else {
        err = parser_parse_opt(parser, opt, (*parser).opt_data.rpl_optarg);
    }
    if err == 7 as libc::c_int {
        *arg_ebadkey = (opt == -(1 as libc::c_int) || opt == 1 as libc::c_int)
            as libc::c_int;
    }
    return err;
}
pub unsafe extern "C" fn argp_parse(
    mut argp: *const argp,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut flags: libc::c_uint,
    mut end_index: *mut libc::c_int,
    mut input: *mut libc::c_void,
) -> error_t {
    let mut err: error_t = 0;
    let mut parser: parser = parser {
        argp: 0 as *const argp,
        short_opts: 0 as *mut libc::c_char,
        long_opts: 0 as *mut rpl_option,
        opt_data: _getopt_data {
            rpl_optind: 0,
            rpl_opterr: 0,
            rpl_optopt: 0,
            rpl_optarg: 0 as *mut libc::c_char,
            __initialized: 0,
            __nextchar: 0 as *mut libc::c_char,
            __ordering: REQUIRE_ORDER,
            __first_nonopt: 0,
            __last_nonopt: 0,
        },
        groups: 0 as *mut group,
        egroup: 0 as *mut group,
        child_inputs: 0 as *mut *mut libc::c_void,
        try_getopt: 0,
        state: argp_state {
            root_argp: 0 as *const argp,
            argc: 0,
            argv: 0 as *mut *mut libc::c_char,
            next: 0,
            flags: 0,
            arg_num: 0,
            quoted: 0,
            input: 0 as *mut libc::c_void,
            child_inputs: 0 as *mut *mut libc::c_void,
            hook: 0 as *mut libc::c_void,
            name: 0 as *mut libc::c_char,
            err_stream: 0 as *mut FILE,
            out_stream: 0 as *mut FILE,
            pstate: 0 as *mut libc::c_void,
        },
        storage: 0 as *mut libc::c_void,
    };
    let mut arg_ebadkey: libc::c_int = 0 as libc::c_int;
    if flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        if program_invocation_name.is_null() {
            program_invocation_name = *argv.offset(0 as libc::c_int as isize);
        }
        if program_invocation_short_name.is_null() {
            program_invocation_short_name = last_component(
                *argv.offset(0 as libc::c_int as isize),
            );
        }
    }
    if flags & 0x10 as libc::c_int as libc::c_uint == 0 {
        let mut fresh14 = ::std::vec::from_elem(
            0,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<argp_child>() as libc::c_ulong)
                as usize,
        );
        let mut child: *mut argp_child = fresh14.leak().as_mut_ptr() as *mut argp_child;
        let mut fresh15 = ::std::vec::from_elem(
            0,
            ::std::mem::size_of::<argp>() as libc::c_ulong as usize,
        );
        let mut top_argp: *mut argp = fresh15.leak().as_mut_ptr() as *mut argp;
        memset(
            top_argp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<argp>() as libc::c_ulong,
        );
        (*top_argp).children = child;
        memset(
            child as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<argp_child>() as libc::c_ulong),
        );
        if !argp.is_null() {
            let fresh16 = child;
            child = child.offset(1);
            (*fresh16).argp = argp;
        }
        let fresh17 = child;
        child = child.offset(1);
        (*fresh17).argp = &argp_default_argp;
        if !argp_program_version.is_null() || argp_program_version_hook.is_some() {
            let fresh18 = child;
            child = child.offset(1);
            (*fresh18).argp = &argp_version_argp;
        }
        (*child).argp = 0 as *const argp;
        argp = top_argp;
    }
    err = parser_init(&mut parser, argp, argc, argv, flags as libc::c_int, input);
    if err == 0 {
        while err == 0 {
            err = parser_parse_next(&mut parser, &mut arg_ebadkey);
        }
        err = parser_finalize(&mut parser, err, arg_ebadkey, end_index);
    }
    return err;
}
pub unsafe extern "C" fn _argp_input(
    mut argp: *const argp,
    mut state: *const argp_state,
) -> *mut libc::c_void {
    if !state.is_null() {
        let mut group: *mut group = 0 as *mut group;
        let mut parser: *mut parser = (*state).pstate as *mut parser;
        group = (*parser).groups;
        while group < (*parser).egroup {
            if (*group).argp == argp {
                return (*group).input;
            }
            group = group.offset(1);
            group;
        }
    }
    return 0 as *mut libc::c_void;
}

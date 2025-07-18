use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_float;
    fn exit(_: libc::c_int) -> !;
    fn pkNewConfiguration() -> PkConfiguration;
    fn pkNewVM(config: *mut PkConfiguration) -> *mut PKVM;
    fn pkFreeVM(vm: *mut PKVM);
    fn pkRunString(vm: *mut PKVM, source: *const libc::c_char) -> PkResult;
    fn pkRunFile(vm: *mut PKVM, path: *const libc::c_char) -> PkResult;
    fn pkRunREPL(vm: *mut PKVM) -> PkResult;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type intptr_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub const PK_RESULT_RUNTIME_ERROR: PkResult = 3;
pub const PK_RESULT_COMPILE_ERROR: PkResult = 2;
pub const PK_RESULT_UNEXPECTED_EOF: PkResult = 1;
pub const PK_RESULT_SUCCESS: PkResult = 0;
pub type PkResult = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PkConfiguration {
    pub realloc_fn: pkReallocFn,
    pub stderr_write: pkWriteFn,
    pub stdout_write: pkWriteFn,
    pub stdin_read: pkReadFn,
    pub resolve_path_fn: pkResolvePathFn,
    pub load_script_fn: pkLoadScriptFn,
    pub load_dl_fn: pkLoadDL,
    pub import_dl_fn: pkImportDL,
    pub unload_dl_fn: pkUnloadDL,
    pub use_ansi_escape: bool,
    pub user_data: *mut libc::c_void,
}
pub type pkUnloadDL = Option::<unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> ()>;
pub type pkImportDL = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> *mut PkHandle,
>;
pub type pkLoadDL = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut libc::c_void,
>;
pub type pkLoadScriptFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut libc::c_char,
>;
pub type pkResolvePathFn = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        *const libc::c_char,
        *const libc::c_char,
    ) -> *mut libc::c_char,
>;
pub type pkReadFn = Option::<unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_char>;
pub type pkWriteFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
>;
pub type pkReallocFn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argparse {
    pub options: *const argparse_option,
    pub usages: *const *const libc::c_char,
    pub flags: libc::c_int,
    pub description: *const libc::c_char,
    pub epilog: *const libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *const libc::c_char,
    pub out: *mut *const libc::c_char,
    pub cpidx: libc::c_int,
    pub optvalue: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argparse_option {
    pub type_0: argparse_option_type,
    pub short_name: libc::c_char,
    pub long_name: *const libc::c_char,
    pub value: *mut libc::c_void,
    pub help: *const libc::c_char,
    pub callback: Option::<argparse_callback>,
    pub data: intptr_t,
    pub flags: libc::c_int,
}
pub type argparse_callback = unsafe extern "C" fn(
    *mut argparse,
    *const argparse_option,
) -> libc::c_int;
pub type argparse_option_type = libc::c_uint;
pub const ARGPARSE_OPT_STRING: argparse_option_type = 6;
pub const ARGPARSE_OPT_FLOAT: argparse_option_type = 5;
pub const ARGPARSE_OPT_INTEGER: argparse_option_type = 4;
pub const ARGPARSE_OPT_BIT: argparse_option_type = 3;
pub const ARGPARSE_OPT_BOOLEAN: argparse_option_type = 2;
pub const ARGPARSE_OPT_GROUP: argparse_option_type = 1;
pub const ARGPARSE_OPT_END: argparse_option_type = 0;
pub type argparse_flag = libc::c_uint;
pub const ARGPARSE_STOP_AT_NON_OPTION: argparse_flag = 1;
pub type argparse_option_flags = libc::c_uint;
pub const OPT_NONEG: argparse_option_flags = 1;
unsafe extern "C" fn argparse_options_check(mut options: *const argparse_option) {
    while (*options).type_0 as libc::c_uint
        != ARGPARSE_OPT_END as libc::c_int as libc::c_uint
    {
        match (*options).type_0 as libc::c_uint {
            0 | 2 | 3 | 4 | 5 | 6 | 1 => {}
            _ => {
                fprintf(
                    stderr,
                    b"wrong option type: %d\0" as *const u8 as *const libc::c_char,
                    (*options).type_0 as libc::c_uint,
                );
            }
        }
        options = options.offset(1);
        options;
    }
}
unsafe extern "C" fn argparse_short_opt(
    mut self_0: *mut argparse,
    mut options: *const argparse_option,
) -> libc::c_int {
    while (*options).type_0 as libc::c_uint
        != ARGPARSE_OPT_END as libc::c_int as libc::c_uint
    {
        if (*options).short_name as libc::c_int == *(*self_0).optvalue as libc::c_int {
            (*self_0)
                .optvalue = if *((*self_0).optvalue).offset(1 as libc::c_int as isize)
                as libc::c_int != 0
            {
                ((*self_0).optvalue).offset(1 as libc::c_int as isize)
            } else {
                0 as *const libc::c_char
            };
            return argparse_getvalue(self_0, options, 0 as libc::c_int);
        }
        options = options.offset(1);
        options;
    }
    return -(2 as libc::c_int);
}
unsafe extern "C" fn prefix_cmp(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    loop {
        if *prefix == 0 {
            return 0 as libc::c_int
        } else if *str as libc::c_int != *prefix as libc::c_int {
            return *prefix as libc::c_uchar as libc::c_int
                - *str as libc::c_uchar as libc::c_int
        }
        str = str.offset(1);
        str;
        prefix = prefix.offset(1);
        prefix;
    };
}
unsafe extern "C" fn prefix_skip(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut len: size_t = strlen(prefix);
    return if strncmp(str, prefix, len) != 0 {
        0 as *const libc::c_char
    } else {
        str.offset(len as isize)
    };
}
unsafe extern "C" fn argparse_error(
    mut self_0: *mut argparse,
    mut opt: *const argparse_option,
    mut reason: *const libc::c_char,
    mut flags: libc::c_int,
) {
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"error: option `--%s` %s\n\0" as *const u8 as *const libc::c_char,
            (*opt).long_name,
            reason,
        );
    } else {
        fprintf(
            stderr,
            b"error: option `-%c` %s\n\0" as *const u8 as *const libc::c_char,
            (*opt).short_name as libc::c_int,
            reason,
        );
    }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn argparse_getvalue(
    mut self_0: *mut argparse,
    mut opt: *const argparse_option,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if !((*opt).value).is_null() {
        match (*opt).type_0 as libc::c_uint {
            2 => {
                if flags & 1 as libc::c_int != 0 {
                    *((*opt).value
                        as *mut libc::c_int) = *((*opt).value as *mut libc::c_int)
                        - 1 as libc::c_int;
                } else {
                    *((*opt).value
                        as *mut libc::c_int) = *((*opt).value as *mut libc::c_int)
                        + 1 as libc::c_int;
                }
                if *((*opt).value as *mut libc::c_int) < 0 as libc::c_int {
                    *((*opt).value as *mut libc::c_int) = 0 as libc::c_int;
                }
            }
            3 => {
                if flags & 1 as libc::c_int != 0 {
                    let ref mut fresh0 = *((*opt).value as *mut libc::c_int);
                    *fresh0 = (*fresh0 as libc::c_long & !(*opt).data) as libc::c_int;
                } else {
                    let ref mut fresh1 = *((*opt).value as *mut libc::c_int);
                    *fresh1 = (*fresh1 as libc::c_long | (*opt).data) as libc::c_int;
                }
            }
            6 => {
                if !((*self_0).optvalue).is_null() {
                    let ref mut fresh2 = *((*opt).value as *mut *const libc::c_char);
                    *fresh2 = (*self_0).optvalue;
                    (*self_0).optvalue = 0 as *const libc::c_char;
                } else if (*self_0).argc > 1 as libc::c_int {
                    (*self_0).argc -= 1;
                    (*self_0).argc;
                    (*self_0).argv = ((*self_0).argv).offset(1);
                    let ref mut fresh3 = *((*opt).value as *mut *const libc::c_char);
                    *fresh3 = *(*self_0).argv;
                } else {
                    argparse_error(
                        self_0,
                        opt,
                        b"requires a value\0" as *const u8 as *const libc::c_char,
                        flags,
                    );
                }
            }
            4 => {
                *__errno_location() = 0 as libc::c_int;
                if !((*self_0).optvalue).is_null() {
                    *((*opt).value
                        as *mut libc::c_int) = strtol(
                        (*self_0).optvalue,
                        &mut s as *mut *const libc::c_char as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    ) as libc::c_int;
                    (*self_0).optvalue = 0 as *const libc::c_char;
                } else if (*self_0).argc > 1 as libc::c_int {
                    (*self_0).argc -= 1;
                    (*self_0).argc;
                    (*self_0).argv = ((*self_0).argv).offset(1);
                    *((*opt).value
                        as *mut libc::c_int) = strtol(
                        *(*self_0).argv,
                        &mut s as *mut *const libc::c_char as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    ) as libc::c_int;
                } else {
                    argparse_error(
                        self_0,
                        opt,
                        b"requires a value\0" as *const u8 as *const libc::c_char,
                        flags,
                    );
                }
                if *__errno_location() == 34 as libc::c_int {
                    argparse_error(
                        self_0,
                        opt,
                        b"numerical result out of range\0" as *const u8
                            as *const libc::c_char,
                        flags,
                    );
                }
                if *s.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                    argparse_error(
                        self_0,
                        opt,
                        b"expects an integer value\0" as *const u8
                            as *const libc::c_char,
                        flags,
                    );
                }
            }
            5 => {
                *__errno_location() = 0 as libc::c_int;
                if !((*self_0).optvalue).is_null() {
                    *((*opt).value
                        as *mut libc::c_float) = strtof(
                        (*self_0).optvalue,
                        &mut s as *mut *const libc::c_char as *mut *mut libc::c_char,
                    );
                    (*self_0).optvalue = 0 as *const libc::c_char;
                } else if (*self_0).argc > 1 as libc::c_int {
                    (*self_0).argc -= 1;
                    (*self_0).argc;
                    (*self_0).argv = ((*self_0).argv).offset(1);
                    *((*opt).value
                        as *mut libc::c_float) = strtof(
                        *(*self_0).argv,
                        &mut s as *mut *const libc::c_char as *mut *mut libc::c_char,
                    );
                } else {
                    argparse_error(
                        self_0,
                        opt,
                        b"requires a value\0" as *const u8 as *const libc::c_char,
                        flags,
                    );
                }
                if *__errno_location() == 34 as libc::c_int {
                    argparse_error(
                        self_0,
                        opt,
                        b"numerical result out of range\0" as *const u8
                            as *const libc::c_char,
                        flags,
                    );
                }
                if *s.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                    argparse_error(
                        self_0,
                        opt,
                        b"expects a numerical value\0" as *const u8
                            as *const libc::c_char,
                        flags,
                    );
                }
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"cli/argparse.h\0" as *const u8 as *const libc::c_char,
                    285 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"int argparse_getvalue(struct argparse *, const struct argparse_option *, int)\0",
                    ))
                        .as_ptr(),
                );
                'c_3288: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"cli/argparse.h\0" as *const u8 as *const libc::c_char,
                        285 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 78],
                            &[libc::c_char; 78],
                        >(
                            b"int argparse_getvalue(struct argparse *, const struct argparse_option *, int)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
    }
    if ((*opt).callback).is_some() {
        return ((*opt).callback).unwrap()(self_0, opt);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn argparse_long_opt(
    mut self_0: *mut argparse,
    mut options: *const argparse_option,
) -> libc::c_int {
    let mut current_block_4: u64;
    while (*options).type_0 as libc::c_uint
        != ARGPARSE_OPT_END as libc::c_int as libc::c_uint
    {
        let mut rest: *const libc::c_char = 0 as *const libc::c_char;
        let mut opt_flags: libc::c_int = 0 as libc::c_int;
        if !((*options).long_name).is_null() {
            rest = prefix_skip(
                (*((*self_0).argv).offset(0 as libc::c_int as isize))
                    .offset(2 as libc::c_int as isize),
                (*options).long_name,
            );
            if rest.is_null() {
                if (*options).flags & OPT_NONEG as libc::c_int != 0 {
                    current_block_4 = 792017965103506125;
                } else if (*options).type_0 as libc::c_uint
                    != ARGPARSE_OPT_BOOLEAN as libc::c_int as libc::c_uint
                    && (*options).type_0 as libc::c_uint
                        != ARGPARSE_OPT_BIT as libc::c_int as libc::c_uint
                {
                    current_block_4 = 792017965103506125;
                } else if prefix_cmp(
                    (*((*self_0).argv).offset(0 as libc::c_int as isize))
                        .offset(2 as libc::c_int as isize),
                    b"no-\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    current_block_4 = 792017965103506125;
                } else {
                    rest = prefix_skip(
                        (*((*self_0).argv).offset(0 as libc::c_int as isize))
                            .offset(2 as libc::c_int as isize)
                            .offset(3 as libc::c_int as isize),
                        (*options).long_name,
                    );
                    if rest.is_null() {
                        current_block_4 = 792017965103506125;
                    } else {
                        opt_flags |= 1 as libc::c_int;
                        current_block_4 = 17965632435239708295;
                    }
                }
            } else {
                current_block_4 = 17965632435239708295;
            }
            match current_block_4 {
                792017965103506125 => {}
                _ => {
                    if *rest != 0 {
                        if *rest as libc::c_int != '=' as i32 {
                            current_block_4 = 792017965103506125;
                        } else {
                            (*self_0).optvalue = rest.offset(1 as libc::c_int as isize);
                            current_block_4 = 12599329904712511516;
                        }
                    } else {
                        current_block_4 = 12599329904712511516;
                    }
                    match current_block_4 {
                        792017965103506125 => {}
                        _ => {
                            return argparse_getvalue(
                                self_0,
                                options,
                                opt_flags | (1 as libc::c_int) << 1 as libc::c_int,
                            );
                        }
                    }
                }
            }
        }
        options = options.offset(1);
        options;
    }
    return -(2 as libc::c_int);
}
pub unsafe extern "C" fn argparse_parse(
    mut self_0: *mut argparse,
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    (*self_0).argc = argc - 1 as libc::c_int;
    (*self_0).argv = argv.offset(1 as libc::c_int as isize);
    (*self_0).out = argv;
    argparse_options_check((*self_0).options);
    while (*self_0).argc != 0 {
        let mut arg: *const libc::c_char = *((*self_0).argv)
            .offset(0 as libc::c_int as isize);
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
            || *arg.offset(1 as libc::c_int as isize) == 0
        {
            if (*self_0).flags & ARGPARSE_STOP_AT_NON_OPTION as libc::c_int != 0 {
                break;
            }
            let fresh4 = (*self_0).cpidx;
            (*self_0).cpidx = (*self_0).cpidx + 1;
            let ref mut fresh5 = *((*self_0).out).offset(fresh4 as isize);
            *fresh5 = *((*self_0).argv).offset(0 as libc::c_int as isize);
        } else {
            if *arg.offset(1 as libc::c_int as isize) as libc::c_int != '-' as i32 {
                (*self_0).optvalue = arg.offset(1 as libc::c_int as isize);
                match argparse_short_opt(self_0, (*self_0).options) {
                    -2 => {
                        current_block = 4043643181702140832;
                    }
                    -1 | _ => {
                        loop {
                            if ((*self_0).optvalue).is_null() {
                                current_block = 12517898123489920830;
                                break;
                            }
                            match argparse_short_opt(self_0, (*self_0).options) {
                                -2 => {
                                    current_block = 4043643181702140832;
                                    break;
                                }
                                -1 | _ => {}
                            }
                        }
                    }
                }
            } else if *arg.offset(2 as libc::c_int as isize) == 0 {
                (*self_0).argc -= 1;
                (*self_0).argc;
                (*self_0).argv = ((*self_0).argv).offset(1);
                (*self_0).argv;
                break;
            } else {
                match argparse_long_opt(self_0, (*self_0).options) {
                    -2 => {
                        current_block = 4043643181702140832;
                    }
                    -1 | _ => {
                        current_block = 12517898123489920830;
                    }
                }
            }
            match current_block {
                12517898123489920830 => {}
                _ => {
                    fprintf(
                        stderr,
                        b"error: unknown option `%s`\n\0" as *const u8
                            as *const libc::c_char,
                        *((*self_0).argv).offset(0 as libc::c_int as isize),
                    );
                    argparse_usage(self_0);
                    exit(1 as libc::c_int);
                }
            }
        }
        (*self_0).argc -= 1;
        (*self_0).argc;
        (*self_0).argv = ((*self_0).argv).offset(1);
        (*self_0).argv;
    }
    memmove(
        ((*self_0).out).offset((*self_0).cpidx as isize) as *mut libc::c_void,
        (*self_0).argv as *mut libc::c_void,
        ((*self_0).argc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    );
    let ref mut fresh6 = *((*self_0).out)
        .offset(((*self_0).cpidx + (*self_0).argc) as isize);
    *fresh6 = 0 as *const libc::c_char;
    return (*self_0).cpidx + (*self_0).argc;
}
pub unsafe extern "C" fn argparse_describe(
    mut self_0: *mut argparse,
    mut description: *const libc::c_char,
    mut epilog: *const libc::c_char,
) {
    (*self_0).description = description;
    (*self_0).epilog = epilog;
}
pub unsafe extern "C" fn argparse_init(
    mut self_0: *mut argparse,
    mut options: *mut argparse_option,
    mut usages: *const *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    memset(
        self_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<argparse>() as libc::c_ulong,
    );
    (*self_0).options = options;
    (*self_0).usages = usages;
    (*self_0).flags = flags;
    (*self_0).description = 0 as *const libc::c_char;
    (*self_0).epilog = 0 as *const libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn argparse_usage(mut self_0: *mut argparse) {
    if !((*self_0).usages).is_null() {
        let fresh7 = (*self_0).usages;
        (*self_0).usages = ((*self_0).usages).offset(1);
        fprintf(stdout, b"Usage: %s\n\0" as *const u8 as *const libc::c_char, *fresh7);
        while !(*(*self_0).usages).is_null() && **(*self_0).usages as libc::c_int != 0 {
            let fresh8 = (*self_0).usages;
            (*self_0).usages = ((*self_0).usages).offset(1);
            fprintf(
                stdout,
                b"   or: %s\n\0" as *const u8 as *const libc::c_char,
                *fresh8,
            );
        }
    } else {
        fprintf(stdout, b"Usage:\n\0" as *const u8 as *const libc::c_char);
    }
    if !((*self_0).description).is_null() {
        fprintf(
            stdout,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (*self_0).description,
        );
    }
    let mut options: *const argparse_option = 0 as *const argparse_option;
    let mut usage_opts_width: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0;
    options = (*self_0).options;
    while (*options).type_0 as libc::c_uint
        != ARGPARSE_OPT_END as libc::c_int as libc::c_uint
    {
        len = 0 as libc::c_int as size_t;
        if (*options).short_name != 0 {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        if (*options).short_name as libc::c_int != 0 && !((*options).long_name).is_null()
        {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        if !((*options).long_name).is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen((*options).long_name))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        }
        if (*options).type_0 as libc::c_uint
            == ARGPARSE_OPT_INTEGER as libc::c_int as libc::c_uint
        {
            len = (len as libc::c_ulong)
                .wrapping_add(strlen(b"=<int>\0" as *const u8 as *const libc::c_char))
                as size_t as size_t;
        }
        if (*options).type_0 as libc::c_uint
            == ARGPARSE_OPT_FLOAT as libc::c_int as libc::c_uint
        {
            len = (len as libc::c_ulong)
                .wrapping_add(strlen(b"=<flt>\0" as *const u8 as *const libc::c_char))
                as size_t as size_t;
        } else if (*options).type_0 as libc::c_uint
            == ARGPARSE_OPT_STRING as libc::c_int as libc::c_uint
        {
            len = (len as libc::c_ulong)
                .wrapping_add(strlen(b"=<str>\0" as *const u8 as *const libc::c_char))
                as size_t as size_t;
        }
        len = len
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                len.wrapping_add(3 as libc::c_int as libc::c_ulong)
                    & 3 as libc::c_int as libc::c_ulong,
            );
        if usage_opts_width < len {
            usage_opts_width = len;
        }
        options = options.offset(1);
        options;
    }
    usage_opts_width = (usage_opts_width as libc::c_ulong)
        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
    options = (*self_0).options;
    while (*options).type_0 as libc::c_uint
        != ARGPARSE_OPT_END as libc::c_int as libc::c_uint
    {
        let mut pos: size_t = 0 as libc::c_int as size_t;
        let mut pad: size_t = 0 as libc::c_int as size_t;
        if (*options).type_0 as libc::c_uint
            == ARGPARSE_OPT_GROUP as libc::c_int as libc::c_uint
        {
            fputc('\n' as i32, stdout);
            fprintf(
                stdout,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*options).help,
            );
            fputc('\n' as i32, stdout);
        } else {
            pos = fprintf(stdout, b"    \0" as *const u8 as *const libc::c_char)
                as size_t;
            if (*options).short_name != 0 {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        fprintf(
                            stdout,
                            b"-%c\0" as *const u8 as *const libc::c_char,
                            (*options).short_name as libc::c_int,
                        ) as libc::c_ulong,
                    ) as size_t as size_t;
            }
            if !((*options).long_name).is_null()
                && (*options).short_name as libc::c_int != 0
            {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        fprintf(stdout, b", \0" as *const u8 as *const libc::c_char)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
            if !((*options).long_name).is_null() {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        fprintf(
                            stdout,
                            b"--%s\0" as *const u8 as *const libc::c_char,
                            (*options).long_name,
                        ) as libc::c_ulong,
                    ) as size_t as size_t;
            }
            if (*options).type_0 as libc::c_uint
                == ARGPARSE_OPT_INTEGER as libc::c_int as libc::c_uint
            {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        fprintf(stdout, b"=<int>\0" as *const u8 as *const libc::c_char)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            } else if (*options).type_0 as libc::c_uint
                == ARGPARSE_OPT_FLOAT as libc::c_int as libc::c_uint
            {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        fprintf(stdout, b"=<flt>\0" as *const u8 as *const libc::c_char)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            } else if (*options).type_0 as libc::c_uint
                == ARGPARSE_OPT_STRING as libc::c_int as libc::c_uint
            {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        fprintf(stdout, b"=<str>\0" as *const u8 as *const libc::c_char)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
            if pos <= usage_opts_width {
                pad = usage_opts_width.wrapping_sub(pos);
            } else {
                fputc('\n' as i32, stdout);
                pad = usage_opts_width;
            }
            fprintf(
                stdout,
                b"%*s%s\n\0" as *const u8 as *const libc::c_char,
                pad as libc::c_int + 2 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
                (*options).help,
            );
        }
        options = options.offset(1);
        options;
    }
    if !((*self_0).epilog).is_null() {
        fprintf(stdout, b"%s\n\0" as *const u8 as *const libc::c_char, (*self_0).epilog);
    }
}
pub unsafe extern "C" fn argparse_help_cb(
    mut self_0: *mut argparse,
    mut option: *const argparse_option,
) -> libc::c_int {
    argparse_usage(self_0);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn intializePocketVM() -> *mut PKVM {
    let mut config: PkConfiguration = pkNewConfiguration();
    if isatty(fileno(stderr)) != 0 {
        config.use_ansi_escape = 1 as libc::c_int != 0;
    }
    let mut vm: *mut PKVM = pkNewVM(&mut config);
    return vm;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut usage: [*const libc::c_char; 2] = [
        b"pocket ... [-c cmd | file] ...\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut debug: libc::c_int = 0 as libc::c_int;
    let mut help: libc::c_int = 0 as libc::c_int;
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut version: libc::c_int = 0 as libc::c_int;
    let mut cli_opts: [argparse_option; 6] = [
        {
            let mut init = argparse_option {
                type_0: ARGPARSE_OPT_STRING,
                short_name: 'c' as i32 as libc::c_char,
                long_name: b"cmd\0" as *const u8 as *const libc::c_char,
                value: &mut cmd as *mut *const libc::c_char as *mut libc::c_void,
                help: b"Evaluate and run the passed string.\0" as *const u8
                    as *const libc::c_char,
                callback: None,
                data: 0 as libc::c_int as intptr_t,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = argparse_option {
                type_0: ARGPARSE_OPT_BOOLEAN,
                short_name: 'd' as i32 as libc::c_char,
                long_name: b"debug\0" as *const u8 as *const libc::c_char,
                value: &mut debug as *mut libc::c_int as *mut libc::c_void,
                help: b"Compile and run the debug version.\0" as *const u8
                    as *const libc::c_char,
                callback: None,
                data: 0 as libc::c_int as intptr_t,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = argparse_option {
                type_0: ARGPARSE_OPT_BOOLEAN,
                short_name: 'h' as i32 as libc::c_char,
                long_name: b"help\0" as *const u8 as *const libc::c_char,
                value: &mut help as *mut libc::c_int as *mut libc::c_void,
                help: b"Prints this help message and exit.\0" as *const u8
                    as *const libc::c_char,
                callback: None,
                data: 0 as libc::c_int as intptr_t,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = argparse_option {
                type_0: ARGPARSE_OPT_BOOLEAN,
                short_name: 'q' as i32 as libc::c_char,
                long_name: b"quiet\0" as *const u8 as *const libc::c_char,
                value: &mut quiet as *mut libc::c_int as *mut libc::c_void,
                help: b"Don't print version and copyright statement on REPL startup.\0"
                    as *const u8 as *const libc::c_char,
                callback: None,
                data: 0 as libc::c_int as intptr_t,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = argparse_option {
                type_0: ARGPARSE_OPT_BOOLEAN,
                short_name: 'v' as i32 as libc::c_char,
                long_name: b"version\0" as *const u8 as *const libc::c_char,
                value: &mut version as *mut libc::c_int as *mut libc::c_void,
                help: b"Prints the pocketlang version and exit.\0" as *const u8
                    as *const libc::c_char,
                callback: None,
                data: 0 as libc::c_int as intptr_t,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = argparse_option {
                type_0: ARGPARSE_OPT_END,
                short_name: 0 as libc::c_int as libc::c_char,
                long_name: 0 as *const libc::c_char,
                value: 0 as *mut libc::c_void,
                help: 0 as *const libc::c_char,
                callback: None,
                data: 0 as libc::c_int as intptr_t,
                flags: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut argparse: argparse = argparse {
        options: 0 as *const argparse_option,
        usages: 0 as *const *const libc::c_char,
        flags: 0,
        description: 0 as *const libc::c_char,
        epilog: 0 as *const libc::c_char,
        argc: 0,
        argv: 0 as *mut *const libc::c_char,
        out: 0 as *mut *const libc::c_char,
        cpidx: 0,
        optvalue: 0 as *const libc::c_char,
    };
    argparse_init(
        &mut argparse,
        cli_opts.as_mut_ptr(),
        usage.as_mut_ptr(),
        0 as libc::c_int,
    );
    argc = argparse_parse(&mut argparse, argc, argv);
    if help != 0 {
        argparse_usage(&mut argparse);
        return 0 as libc::c_int;
    }
    if version != 0 {
        fprintf(
            stdout,
            b"pocketlang %s\n\0" as *const u8 as *const libc::c_char,
            b"0.1.0\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    let mut exitcode: libc::c_int = 0 as libc::c_int;
    let mut vm: *mut PKVM = intializePocketVM();
    if !cmd.is_null() {
        let mut result: PkResult = pkRunString(vm, cmd);
        exitcode = result as libc::c_int;
    } else if argc == 0 as libc::c_int {
        if quiet == 0 {
            printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                b"PocketLang 0.1.0 (https://github.com/ThakeeNathees/pocketlang/)\nCopyright (c) 2020-2021 ThakeeNathees\nCopyright (c) 2021-2022 Pocketlang Contributors\nFree and open source software under the terms of the MIT license.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exitcode = pkRunREPL(vm) as libc::c_int;
    } else {
        let mut file_path: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
        let mut result_0: PkResult = pkRunFile(vm, file_path);
        exitcode = result_0 as libc::c_int;
    }
    pkFreeVM(vm);
    return exitcode;
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
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}

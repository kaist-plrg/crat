use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    static nettle_md5: nettle_hash;
    static nettle_sha1: nettle_hash;
    static nettle_sha256: nettle_hash;
    static nettle_base64: nettle_armor;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn die(format: *const libc::c_char, _: ...) -> !;
    fn werror(format: *const libc::c_char, _: ...);
    fn xalloc(size: size_t) -> *mut libc::c_void;
    fn sexp_input_init(input: *mut sexp_input, f: *mut FILE);
    fn sexp_get_char(input: *mut sexp_input);
    fn sexp_output_init(
        output: *mut sexp_output,
        f: *mut FILE,
        width: libc::c_uint,
        prefer_hex: libc::c_int,
    );
    fn sexp_output_hash_init(
        output: *mut sexp_output,
        hash: *const nettle_hash,
        ctx: *mut libc::c_void,
    );
    fn sexp_put_newline(output: *mut sexp_output, indent: libc::c_uint);
    fn sexp_put_soft_newline(output: *mut sexp_output, indent: libc::c_uint);
    fn sexp_put_char(output: *mut sexp_output, c: uint8_t);
    fn sexp_put_data(
        output: *mut sexp_output,
        length: libc::c_uint,
        data: *const uint8_t,
    );
    fn sexp_put_code_start(output: *mut sexp_output, coding: *const nettle_armor);
    fn sexp_put_code_end(output: *mut sexp_output);
    fn sexp_put_string(
        output: *mut sexp_output,
        mode: sexp_mode,
        string: *mut nettle_buffer,
    );
    fn sexp_put_digest(output: *mut sexp_output);
    fn sexp_compound_token_init(token: *mut sexp_compound_token);
    fn sexp_compound_token_clear(token: *mut sexp_compound_token);
    fn sexp_parse_init(
        parser: *mut sexp_parser,
        input: *mut sexp_input,
        mode: sexp_mode,
    );
    fn sexp_parse(parser: *mut sexp_parser, token: *mut sexp_compound_token);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const libc::c_char,
) -> libc::c_int;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const libc::c_char,
    pub encode_context_size: libc::c_uint,
    pub decode_context_size: libc::c_uint,
    pub encode_final_length: libc::c_uint,
    pub encode_init: Option::<nettle_armor_init_func>,
    pub encode_length: Option::<nettle_armor_length_func>,
    pub encode_update: Option::<nettle_armor_encode_update_func>,
    pub encode_final: Option::<nettle_armor_encode_final_func>,
    pub decode_init: Option::<nettle_armor_init_func>,
    pub decode_length: Option::<nettle_armor_length_func>,
    pub decode_update: Option::<nettle_armor_decode_update_func>,
    pub decode_final: Option::<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type sexp_mode = libc::c_uint;
pub const SEXP_TRANSPORT: sexp_mode = 2;
pub const SEXP_ADVANCED: sexp_mode = 1;
pub const SEXP_CANONICAL: sexp_mode = 0;
pub type sexp_token = libc::c_uint;
pub const SEXP_CODING_END: sexp_token = 9;
pub const SEXP_TRANSPORT_START: sexp_token = 8;
pub const SEXP_DISPLAY_END: sexp_token = 7;
pub const SEXP_DISPLAY_START: sexp_token = 6;
pub const SEXP_EOF: sexp_token = 5;
pub const SEXP_LIST_END: sexp_token = 4;
pub const SEXP_LIST_START: sexp_token = 3;
pub const SEXP_COMMENT: sexp_token = 2;
pub const SEXP_DISPLAY: sexp_token = 1;
pub const SEXP_STRING: sexp_token = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: libc::c_uchar,
    pub bits: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
pub type sexp_char_type = libc::c_uint;
pub const SEXP_END_CHAR: sexp_char_type = 2;
pub const SEXP_EOF_CHAR: sexp_char_type = 1;
pub const SEXP_NORMAL_CHAR: sexp_char_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_input {
    pub f: *mut FILE,
    pub ctype: sexp_char_type,
    pub c: uint8_t,
    pub coding: *const nettle_armor,
    pub state: C2RustUnnamed,
    pub terminator: uint8_t,
    pub token: sexp_token,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub base64: base64_decode_ctx,
    pub hex: base16_decode_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_output {
    pub f: *mut FILE,
    pub line_width: libc::c_uint,
    pub coding: *const nettle_armor,
    pub coding_indent: libc::c_uint,
    pub prefer_hex: libc::c_int,
    pub hash: *const nettle_hash,
    pub ctx: *mut libc::c_void,
    pub base64: base64_decode_ctx,
    pub pos: libc::c_uint,
    pub soft_newline: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_compound_token {
    pub type_0: sexp_token,
    pub display: nettle_buffer,
    pub string: nettle_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_parser {
    pub input: *mut sexp_input,
    pub mode: sexp_mode,
    pub level: libc::c_uint,
    pub transport: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conv_options {
    pub mode: sexp_mode,
    pub prefer_hex: libc::c_int,
    pub once: libc::c_int,
    pub lock: libc::c_int,
    pub width: libc::c_uint,
    pub hash: *const nettle_hash,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_HELP: C2RustUnnamed_0 = 303;
pub const OPT_LOCK: C2RustUnnamed_0 = 302;
pub const OPT_HASH: C2RustUnnamed_0 = 301;
pub const OPT_ONCE: C2RustUnnamed_0 = 300;
unsafe extern "C" fn sexp_convert_item(
    mut parser: *mut sexp_parser,
    mut token: *mut sexp_compound_token,
    mut output: *mut sexp_output,
    mut mode_out: sexp_mode,
    mut indent: libc::c_uint,
) {
    if mode_out as libc::c_uint == SEXP_TRANSPORT as libc::c_int as libc::c_uint {
        sexp_put_char(output, '{' as i32 as uint8_t);
        sexp_put_code_start(output, &nettle_base64);
        sexp_convert_item(
            parser,
            token,
            output,
            SEXP_CANONICAL,
            0 as libc::c_int as libc::c_uint,
        );
        sexp_put_code_end(output);
        sexp_put_char(output, '}' as i32 as uint8_t);
    } else {
        match (*token).type_0 as libc::c_uint {
            4 => {
                die(b"Unmatched end of list.\n\0" as *const u8 as *const libc::c_char);
            }
            5 => {
                die(b"Unexpected end of file.\n\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                die(
                    b"Unexpected end of coding.\n\0" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                let mut item: libc::c_uint = 0;
                sexp_put_char(output, '(' as i32 as uint8_t);
                item = 0 as libc::c_int as libc::c_uint;
                loop {
                    sexp_parse(parser, token);
                    if !((*token).type_0 as libc::c_uint
                        != SEXP_LIST_END as libc::c_int as libc::c_uint)
                    {
                        break;
                    }
                    if mode_out as libc::c_uint
                        == SEXP_ADVANCED as libc::c_int as libc::c_uint
                    {
                        match item {
                            0 => {
                                if (*token).type_0 as libc::c_uint
                                    == SEXP_COMMENT as libc::c_int as libc::c_uint
                                {
                                    indent = (*output).pos;
                                    item = item.wrapping_add(1);
                                    item;
                                }
                            }
                            1 => {
                                sexp_put_char(output, ' ' as i32 as uint8_t);
                                indent = (*output).pos;
                            }
                            _ => {
                                sexp_put_newline(output, indent);
                            }
                        }
                    }
                    sexp_convert_item(parser, token, output, mode_out, indent);
                    item = item.wrapping_add(1);
                    item;
                }
                sexp_put_char(output, ')' as i32 as uint8_t);
            }
            0 => {
                sexp_put_string(output, mode_out, &mut (*token).string);
            }
            1 => {
                sexp_put_char(output, '[' as i32 as uint8_t);
                sexp_put_string(output, mode_out, &mut (*token).display);
                sexp_put_char(output, ']' as i32 as uint8_t);
                sexp_put_string(output, mode_out, &mut (*token).string);
            }
            2 => {
                if mode_out as libc::c_uint
                    == SEXP_ADVANCED as libc::c_int as libc::c_uint
                {
                    sexp_put_data(
                        output,
                        (*token).string.size as libc::c_uint,
                        (*token).string.contents,
                    );
                    sexp_put_soft_newline(output, indent);
                }
            }
            _ => {
                abort();
            }
        }
    };
}
unsafe extern "C" fn match_argument(
    mut given: *const libc::c_char,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if !given.is_null() && !name.is_null() {} else {
        __assert_fail(
            b"given != NULL && name != NULL\0" as *const u8 as *const libc::c_char,
            b"sexp-conv.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int match_argument(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3852: {
        if !given.is_null() && !name.is_null() {} else {
            __assert_fail(
                b"given != NULL && name != NULL\0" as *const u8 as *const libc::c_char,
                b"sexp-conv.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int match_argument(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (strcmp(given, name) == 0) as libc::c_int;
}
unsafe extern "C" fn parse_options(
    mut o: *mut conv_options,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    (*o).mode = SEXP_ADVANCED;
    (*o).prefer_hex = 0 as libc::c_int;
    (*o).once = 0 as libc::c_int;
    (*o).lock = 0 as libc::c_int;
    (*o).hash = 0 as *const nettle_hash;
    (*o).width = 72 as libc::c_int as libc::c_uint;
    loop {
        static mut hashes: [*const nettle_hash; 4] = unsafe {
            [
                &nettle_md5 as *const nettle_hash,
                &nettle_sha1 as *const nettle_hash,
                &nettle_sha256 as *const nettle_hash,
                0 as *const nettle_hash,
            ]
        };
        static mut options: [option; 9] = [
            {
                let mut init = option {
                    name: b"help\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_HELP as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"version\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'V' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"once\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_ONCE as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"syntax\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 's' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"hash\0" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_HASH as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"raw-hash\0" as *const u8 as *const libc::c_char,
                    has_arg: 2 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_HASH as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"width\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'w' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"lock\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_LOCK as libc::c_int,
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
        let mut c: libc::c_int = 0;
        let mut option_index: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_uint = 0;
        c = getopt_long(
            argc,
            argv,
            b"Vs:w:\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            &mut option_index,
        );
        match c {
            -1 => {
                if optind != argc {
                    die(
                        b"sexp-conv: Command line takes no arguments, only options.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                return;
            }
            63 => {
                exit(1 as libc::c_int);
            }
            119 => {
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut width: libc::c_int = 0;
                if !optarg.is_null() {} else {
                    __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"sexp-conv.c\0" as *const u8 as *const libc::c_char,
                        284 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 56],
                            &[libc::c_char; 56],
                        >(b"void parse_options(struct conv_options *, int, char **)\0"))
                            .as_ptr(),
                    );
                }
                'c_4290: {
                    if !optarg.is_null() {} else {
                        __assert_fail(
                            b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                            b"sexp-conv.c\0" as *const u8 as *const libc::c_char,
                            284 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 56],
                                &[libc::c_char; 56],
                            >(
                                b"void parse_options(struct conv_options *, int, char **)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                width = strtol(optarg, &mut end, 0 as libc::c_int) as libc::c_int;
                if *optarg == 0 || *end as libc::c_int != 0 || width < 0 as libc::c_int {
                    die(
                        b"sexp-conv: Invalid width `%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                }
                (*o).width = width as libc::c_uint;
            }
            115 => {
                if !((*o).hash).is_null() {
                    werror(
                        b"sexp-conv: Combining --hash and -s usually makes no sense.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                if match_argument(
                    optarg,
                    b"advanced\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    (*o).mode = SEXP_ADVANCED;
                } else if match_argument(
                    optarg,
                    b"transport\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    (*o).mode = SEXP_TRANSPORT;
                } else if match_argument(
                    optarg,
                    b"canonical\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    (*o).mode = SEXP_CANONICAL;
                } else if match_argument(
                    optarg,
                    b"hex\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    (*o).mode = SEXP_ADVANCED;
                    (*o).prefer_hex = 1 as libc::c_int;
                } else {
                    die(
                        b"Available syntax variants: advanced, transport, canonical\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            300 => {
                (*o).once = 1 as libc::c_int;
            }
            301 => {
                (*o).mode = SEXP_CANONICAL;
                if optarg.is_null() {
                    (*o).hash = &nettle_sha1;
                } else {
                    i = 0 as libc::c_int as libc::c_uint;
                    loop {
                        if (hashes[i as usize]).is_null() {
                            die(
                                b"sexp_conv: Unknown hash algorithm `%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                optarg,
                            );
                        }
                        if match_argument(optarg, (*hashes[i as usize]).name) != 0 {
                            (*o).hash = hashes[i as usize];
                            break;
                        } else {
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                }
            }
            302 => {
                (*o).lock = 1 as libc::c_int;
            }
            303 => {
                printf(
                    b"Usage: sexp-conv [OPTION...]\n  Conversion:     sexp-conv [OPTION...] <INPUT-SEXP\n  Fingerprinting: sexp-conv --hash=HASH <INPUT-SEXP\n\nReads an s-expression on stdin, and outputs the same\nsexp on stdout, possibly with a different syntax.\n\n       --hash[=ALGORITHM]   Outputs only the hash of the expression.\n                            Available hash algorithms:\n                            \0"
                        as *const u8 as *const libc::c_char,
                );
                i = 0 as libc::c_int as libc::c_uint;
                while !(hashes[i as usize]).is_null() {
                    if i != 0 {
                        printf(b", \0" as *const u8 as *const libc::c_char);
                    }
                    printf(
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*hashes[i as usize]).name,
                    );
                    i = i.wrapping_add(1);
                    i;
                }
                printf(
                    b" (default is sha1).\n   -s, --syntax=SYNTAX      The syntax used for the output. Available\n                            variants: advanced, hex, transport, canonical\n       --once               Process only the first s-expression.\n   -w, --width=WIDTH        Linewidth for base64 encoded data.\n                            Zero means no limit.\n       --lock               Lock output file.\n       --raw-hash           Alias for --hash, for compatibility\n                            with lsh-1.x.\n\nReport bugs to nettle-bugs@lists.lysator.liu.se.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            86 => {
                printf(
                    b"sexp-conv (nettle 3.9)\n\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                abort();
            }
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut options: conv_options = conv_options {
        mode: SEXP_CANONICAL,
        prefer_hex: 0,
        once: 0,
        lock: 0,
        width: 0,
        hash: 0 as *const nettle_hash,
    };
    let mut input: sexp_input = sexp_input {
        f: 0 as *mut FILE,
        ctype: SEXP_NORMAL_CHAR,
        c: 0,
        coding: 0 as *const nettle_armor,
        state: C2RustUnnamed {
            base64: base64_decode_ctx {
                table: 0 as *const libc::c_schar,
                word: 0,
                bits: 0,
                padding: 0,
            },
        },
        terminator: 0,
        token: SEXP_STRING,
    };
    let mut parser: sexp_parser = sexp_parser {
        input: 0 as *mut sexp_input,
        mode: SEXP_CANONICAL,
        level: 0,
        transport: 0,
    };
    let mut token: sexp_compound_token = sexp_compound_token {
        type_0: SEXP_STRING,
        display: nettle_buffer {
            contents: 0 as *mut uint8_t,
            alloc: 0,
            realloc_ctx: 0 as *mut libc::c_void,
            realloc: None,
            size: 0,
        },
        string: nettle_buffer {
            contents: 0 as *mut uint8_t,
            alloc: 0,
            realloc_ctx: 0 as *mut libc::c_void,
            realloc: None,
            size: 0,
        },
    };
    let mut output: sexp_output = sexp_output {
        f: 0 as *mut FILE,
        line_width: 0,
        coding: 0 as *const nettle_armor,
        coding_indent: 0,
        prefer_hex: 0,
        hash: 0 as *const nettle_hash,
        ctx: 0 as *mut libc::c_void,
        base64: base64_decode_ctx {
            table: 0 as *const libc::c_schar,
            word: 0,
            bits: 0,
            padding: 0,
        },
        pos: 0,
        soft_newline: 0,
    };
    parse_options(&mut options, argc, argv);
    sexp_input_init(&mut input, stdin);
    sexp_parse_init(&mut parser, &mut input, SEXP_ADVANCED);
    sexp_compound_token_init(&mut token);
    sexp_output_init(&mut output, stdout, options.width, options.prefer_hex);
    if options.lock != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        memset(
            &mut fl as *mut flock as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<flock>() as libc::c_ulong,
        );
        fl.l_type = 1 as libc::c_int as libc::c_short;
        fl.l_whence = 0 as libc::c_int as libc::c_short;
        fl.l_start = 0 as libc::c_int as __off_t;
        fl.l_len = 0 as libc::c_int as __off_t;
        if fcntl(1 as libc::c_int, 7 as libc::c_int, &mut fl as *mut flock)
            == -(1 as libc::c_int)
        {
            die(
                b"Locking output file failed: %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    }
    if !(options.hash).is_null() {
        let mut ctx: *mut libc::c_void = xalloc((*options.hash).context_size as size_t);
        sexp_output_hash_init(&mut output, options.hash, ctx);
    }
    sexp_get_char(&mut input);
    sexp_parse(&mut parser, &mut token);
    if token.type_0 as libc::c_uint == SEXP_EOF as libc::c_int as libc::c_uint {
        if options.once != 0 {
            die(
                b"sexp-conv: No input expression.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    loop {
        sexp_convert_item(
            &mut parser,
            &mut token,
            &mut output,
            options.mode,
            0 as libc::c_int as libc::c_uint,
        );
        if !(options.hash).is_null() {
            sexp_put_digest(&mut output);
            sexp_put_newline(&mut output, 0 as libc::c_int as libc::c_uint);
        } else if options.mode as libc::c_uint
            != SEXP_CANONICAL as libc::c_int as libc::c_uint
        {
            sexp_put_newline(&mut output, 0 as libc::c_int as libc::c_uint);
        }
        sexp_parse(&mut parser, &mut token);
        if !(options.once == 0
            && token.type_0 as libc::c_uint != SEXP_EOF as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    sexp_compound_token_clear(&mut token);
    if fflush(output.f) < 0 as libc::c_int {
        die(
            b"Final fflush failed: %s.\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return 0 as libc::c_int;
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

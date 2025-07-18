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
    fn abort() -> !;
    fn die(format: *const libc::c_char, _: ...) -> !;
    fn nettle_buffer_init(buffer: *mut nettle_buffer);
    fn nettle_buffer_clear(buffer: *mut nettle_buffer);
    fn sexp_get_token(
        input: *mut sexp_input,
        mode: sexp_mode,
        string: *mut nettle_buffer,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
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
pub type sexp_char_type = libc::c_uint;
pub const SEXP_END_CHAR: sexp_char_type = 2;
pub const SEXP_EOF_CHAR: sexp_char_type = 1;
pub const SEXP_NORMAL_CHAR: sexp_char_type = 0;
pub type FILE = _IO_FILE;
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
pub unsafe extern "C" fn sexp_compound_token_init(mut token: *mut sexp_compound_token) {
    (*token).type_0 = SEXP_STRING;
    nettle_buffer_init(&mut (*token).display);
    nettle_buffer_init(&mut (*token).string);
}
pub unsafe extern "C" fn sexp_compound_token_clear(mut token: *mut sexp_compound_token) {
    nettle_buffer_clear(&mut (*token).display);
    nettle_buffer_clear(&mut (*token).string);
}
pub unsafe extern "C" fn sexp_parse_init(
    mut parser: *mut sexp_parser,
    mut input: *mut sexp_input,
    mut mode: sexp_mode,
) {
    (*parser).input = input;
    (*parser).mode = mode;
    (*parser).level = 1 as libc::c_int as libc::c_uint;
    (*parser).transport = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn sexp_check_token(
    mut parser: *mut sexp_parser,
    mut token: sexp_token,
    mut string: *mut nettle_buffer,
) {
    sexp_get_token(
        (*parser).input,
        (if (*parser).transport != 0 {
            SEXP_CANONICAL as libc::c_int as libc::c_uint
        } else {
            (*parser).mode as libc::c_uint
        }) as sexp_mode,
        string,
    );
    if (*(*parser).input).token as libc::c_uint != token as libc::c_uint {
        die(b"Syntax error.\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn sexp_parse(
    mut parser: *mut sexp_parser,
    mut token: *mut sexp_compound_token,
) {
    loop {
        sexp_get_token(
            (*parser).input,
            (if (*parser).transport != 0 {
                SEXP_CANONICAL as libc::c_int as libc::c_uint
            } else {
                (*parser).mode as libc::c_uint
            }) as sexp_mode,
            &mut (*token).string,
        );
        match (*(*parser).input).token as libc::c_uint {
            4 => {
                if (*parser).level == (*parser).transport {
                    die(
                        b"Unmatched end of list in transport encoded data.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                (*parser).level = ((*parser).level).wrapping_sub(1);
                (*parser).level;
                if (*parser).level == 0 {
                    die(
                        b"Unmatched end of list.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*token).type_0 = SEXP_LIST_END;
            }
            5 => {
                if (*parser).level > 1 as libc::c_int as libc::c_uint {
                    die(
                        b"Unexpected end of file.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*token).type_0 = SEXP_EOF;
                return;
            }
            3 => {
                (*parser).level = ((*parser).level).wrapping_add(1);
                (*parser).level;
                (*token).type_0 = SEXP_LIST_START;
                return;
            }
            6 => {
                sexp_check_token(parser, SEXP_STRING, &mut (*token).display);
                sexp_check_token(parser, SEXP_DISPLAY_END, &mut (*token).display);
                sexp_check_token(parser, SEXP_STRING, &mut (*token).string);
                (*token).type_0 = SEXP_DISPLAY;
            }
            0 => {
                (*token).type_0 = SEXP_STRING;
            }
            2 => {
                (*token).type_0 = SEXP_COMMENT;
                return;
            }
            8 => {
                if (*parser).mode as libc::c_uint
                    == SEXP_CANONICAL as libc::c_int as libc::c_uint
                {
                    die(
                        b"Base64 not allowed in canonical mode.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*parser).level = ((*parser).level).wrapping_add(1);
                (*parser).level;
                (*parser).transport = (*parser).level;
                continue;
            }
            9 => {
                die(
                    b"Unexpected end of transport encoding.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            7 => {
                die(
                    b"Unexpected end of display tag.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            1 => {
                abort();
            }
            _ => {
                continue;
            }
        }
        if (*parser).level == (*parser).transport {
            sexp_check_token(parser, SEXP_CODING_END, &mut (*token).string);
            if (*parser).transport != 0 {} else {
                __assert_fail(
                    b"parser->transport\0" as *const u8 as *const libc::c_char,
                    b"parse.c\0" as *const u8 as *const libc::c_char,
                    121 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 68],
                        &[libc::c_char; 68],
                    >(
                        b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1769: {
                if (*parser).transport != 0 {} else {
                    __assert_fail(
                        b"parser->transport\0" as *const u8 as *const libc::c_char,
                        b"parse.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 68],
                            &[libc::c_char; 68],
                        >(
                            b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*parser).level == (*parser).transport {} else {
                __assert_fail(
                    b"parser->level == parser->transport\0" as *const u8
                        as *const libc::c_char,
                    b"parse.c\0" as *const u8 as *const libc::c_char,
                    122 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 68],
                        &[libc::c_char; 68],
                    >(
                        b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1718: {
                if (*parser).level == (*parser).transport {} else {
                    __assert_fail(
                        b"parser->level == parser->transport\0" as *const u8
                            as *const libc::c_char,
                        b"parse.c\0" as *const u8 as *const libc::c_char,
                        122 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 68],
                            &[libc::c_char; 68],
                        >(
                            b"void sexp_parse(struct sexp_parser *, struct sexp_compound_token *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*parser).level = ((*parser).level).wrapping_sub(1);
            (*parser).level;
            (*parser).transport = 0 as libc::c_int as libc::c_uint;
        }
        return;
    };
}

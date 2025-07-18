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
    fn abort() -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static nettle_base64: nettle_armor;
    static nettle_base16: nettle_armor;
    fn nettle_buffer_reset(buffer: *mut nettle_buffer);
    fn nettle_buffer_grow(buffer: *mut nettle_buffer, length: size_t) -> libc::c_int;
    static sexp_token_chars: [libc::c_char; 128];
    fn die(format: *const libc::c_char, _: ...) -> !;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
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
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
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
pub unsafe extern "C" fn sexp_input_init(mut input: *mut sexp_input, mut f: *mut FILE) {
    (*input).f = f;
    (*input).coding = 0 as *const nettle_armor;
}
unsafe extern "C" fn sexp_get_raw_char(mut input: *mut sexp_input) {
    let mut c: libc::c_int = getc((*input).f);
    if c < 0 as libc::c_int {
        if ferror((*input).f) != 0 {
            die(
                b"Read error: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        (*input).ctype = SEXP_EOF_CHAR;
    } else {
        (*input).ctype = SEXP_NORMAL_CHAR;
        (*input).c = c as uint8_t;
    };
}
pub unsafe extern "C" fn sexp_get_char(mut input: *mut sexp_input) {
    if !((*input).coding).is_null() {
        loop {
            let mut done: size_t = 0;
            sexp_get_raw_char(input);
            if (*input).ctype as libc::c_uint
                == SEXP_EOF_CHAR as libc::c_int as libc::c_uint
            {
                die(
                    b"Unexpected end of file in coded data.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*input).c as libc::c_int == (*input).terminator as libc::c_int {
                (*input).ctype = SEXP_END_CHAR;
                return;
            }
            done = 1 as libc::c_int as size_t;
            if ((*(*input).coding).decode_update)
                .unwrap()(
                &mut (*input).state as *mut C2RustUnnamed as *mut libc::c_void,
                &mut done,
                &mut (*input).c,
                1 as libc::c_int as size_t,
                &mut (*input).c as *mut uint8_t as *const libc::c_char,
            ) == 0
            {
                die(b"Invalid coded data.\n\0" as *const u8 as *const libc::c_char);
            }
            if done != 0 {
                return;
            }
        }
    } else {
        sexp_get_raw_char(input);
    };
}
unsafe extern "C" fn sexp_next_char(mut input: *mut sexp_input) -> uint8_t {
    sexp_get_char(input);
    if (*input).ctype as libc::c_uint != SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
    {
        die(b"Unexpected end of file.\n\0" as *const u8 as *const libc::c_char);
    }
    return (*input).c;
}
unsafe extern "C" fn sexp_push_char(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    if (*input).ctype as libc::c_uint == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"input->ctype == SEXP_NORMAL_CHAR\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void sexp_push_char(struct sexp_input *, struct nettle_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_3101: {
        if (*input).ctype as libc::c_uint
            == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"input->ctype == SEXP_NORMAL_CHAR\0" as *const u8
                    as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void sexp_push_char(struct sexp_input *, struct nettle_buffer *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(((*string).size < (*string).alloc
        || nettle_buffer_grow(string, 1 as libc::c_int as size_t) != 0)
        && {
            let fresh0 = (*string).size;
            (*string).size = ((*string).size).wrapping_add(1);
            *((*string).contents).offset(fresh0 as isize) = (*input).c;
            1 as libc::c_int != 0
        })
    {
        die(b"Virtual memory exhasuted.\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn sexp_input_start_coding(
    mut input: *mut sexp_input,
    mut coding: *const nettle_armor,
    mut terminator: uint8_t,
) {
    if ((*input).coding).is_null() {} else {
        __assert_fail(
            b"!input->coding\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"void sexp_input_start_coding(struct sexp_input *, const struct nettle_armor *, uint8_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3430: {
        if ((*input).coding).is_null() {} else {
            __assert_fail(
                b"!input->coding\0" as *const u8 as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                128 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"void sexp_input_start_coding(struct sexp_input *, const struct nettle_armor *, uint8_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*input).coding = coding;
    ((*(*input).coding).decode_init)
        .unwrap()(&mut (*input).state as *mut C2RustUnnamed as *mut libc::c_void);
    (*input).terminator = terminator;
}
unsafe extern "C" fn sexp_input_end_coding(mut input: *mut sexp_input) {
    if !((*input).coding).is_null() {} else {
        __assert_fail(
            b"input->coding\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void sexp_input_end_coding(struct sexp_input *)\0"))
                .as_ptr(),
        );
    }
    'c_3314: {
        if !((*input).coding).is_null() {} else {
            __assert_fail(
                b"input->coding\0" as *const u8 as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void sexp_input_end_coding(struct sexp_input *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*(*input).coding).decode_final)
        .unwrap()(&mut (*input).state as *mut C2RustUnnamed as *mut libc::c_void) == 0
    {
        die(b"Invalid coded data.\n\0" as *const u8 as *const libc::c_char);
    }
    (*input).coding = 0 as *const nettle_armor;
}
unsafe extern "C" fn sexp_get_quoted_char(mut input: *mut sexp_input) -> libc::c_int {
    sexp_next_char(input);
    match (*input).c as libc::c_int {
        34 => return 0 as libc::c_int,
        92 => {
            sexp_next_char(input);
            match (*input).c as libc::c_int {
                98 => {
                    (*input).c = '\u{8}' as i32 as uint8_t;
                    return 1 as libc::c_int;
                }
                116 => {
                    (*input).c = '\t' as i32 as uint8_t;
                    return 1 as libc::c_int;
                }
                110 => {
                    (*input).c = '\n' as i32 as uint8_t;
                    return 1 as libc::c_int;
                }
                102 => {
                    (*input).c = '\u{c}' as i32 as uint8_t;
                    return 1 as libc::c_int;
                }
                114 => {
                    (*input).c = '\r' as i32 as uint8_t;
                    return 1 as libc::c_int;
                }
                92 => {
                    (*input).c = '\\' as i32 as uint8_t;
                    return 1 as libc::c_int;
                }
                111 | 120 => {
                    abort();
                }
                10 => {
                    if sexp_next_char(input) as libc::c_int == '\r' as i32 {
                        sexp_next_char(input);
                    }
                }
                13 => {
                    if sexp_next_char(input) as libc::c_int == '\n' as i32 {
                        sexp_next_char(input);
                    }
                }
                _ => {}
            }
            return 1 as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn sexp_get_token_string(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    if ((*input).coding).is_null() {} else {
        __assert_fail(
            b"!input->coding\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3218: {
        if ((*input).coding).is_null() {} else {
            __assert_fail(
                b"!input->coding\0" as *const u8 as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*input).ctype as libc::c_uint == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"input->ctype == SEXP_NORMAL_CHAR\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3174: {
        if (*input).ctype as libc::c_uint
            == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"input->ctype == SEXP_NORMAL_CHAR\0" as *const u8
                    as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                194 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(((*input).c as libc::c_int) < 0x80 as libc::c_int
        && sexp_token_chars[(*input).c as usize] as libc::c_int != 0)
    {
        die(b"Invalid token.\n\0" as *const u8 as *const libc::c_char);
    }
    loop {
        sexp_push_char(input, string);
        sexp_get_char(input);
        if !((*input).ctype as libc::c_uint
            == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
            && (((*input).c as libc::c_int) < 0x80 as libc::c_int
                && sexp_token_chars[(*input).c as usize] as libc::c_int != 0))
        {
            break;
        }
    }
    if (*string).size != 0 {} else {
        __assert_fail(
            b"string->size\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2955: {
        if (*string).size != 0 {} else {
            __assert_fail(
                b"string->size\0" as *const u8 as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"void sexp_get_token_string(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn sexp_get_string(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    nettle_buffer_reset(string);
    (*input).token = SEXP_STRING;
    let mut current_block_16: u64;
    match (*input).c as libc::c_int {
        34 => {
            while sexp_get_quoted_char(input) != 0 {
                sexp_push_char(input, string);
            }
            sexp_get_char(input);
            current_block_16 = 6009453772311597924;
        }
        35 => {
            sexp_input_start_coding(input, &nettle_base16, '#' as i32 as uint8_t);
            current_block_16 = 318935293075366127;
        }
        124 => {
            sexp_input_start_coding(input, &nettle_base64, '|' as i32 as uint8_t);
            current_block_16 = 318935293075366127;
        }
        _ => {
            sexp_get_token_string(input, string);
            current_block_16 = 6009453772311597924;
        }
    }
    match current_block_16 {
        318935293075366127 => {
            loop {
                sexp_get_char(input);
                match (*input).ctype as libc::c_uint {
                    0 => {
                        sexp_push_char(input, string);
                    }
                    1 => {
                        die(
                            b"Unexpected end of file in coded string.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    2 => {
                        sexp_input_end_coding(input);
                        sexp_get_char(input);
                        return;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn sexp_get_string_length(
    mut input: *mut sexp_input,
    mut mode: sexp_mode,
    mut string: *mut nettle_buffer,
) {
    let mut length: libc::c_uint = 0;
    nettle_buffer_reset(string);
    (*input).token = SEXP_STRING;
    length = ((*input).c as libc::c_int - '0' as i32) as libc::c_uint;
    if length == 0 {
        sexp_next_char(input);
    } else {
        if length < 10 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"length < 10\0" as *const u8 as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void sexp_get_string_length(struct sexp_input *, enum sexp_mode, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4280: {
            if length < 10 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"length < 10\0" as *const u8 as *const libc::c_char,
                    b"input.c\0" as *const u8 as *const libc::c_char,
                    275 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 89],
                        &[libc::c_char; 89],
                    >(
                        b"void sexp_get_string_length(struct sexp_input *, enum sexp_mode, struct nettle_buffer *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        loop {
            sexp_next_char(input);
            if ((*input).c as libc::c_int) < '0' as i32
                || (*input).c as libc::c_int > '9' as i32
            {
                break;
            }
            length = length
                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                .wrapping_add((*input).c as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint);
        }
    }
    if (*input).c as libc::c_int == ':' as i32 {
        while length != 0 {
            sexp_next_char(input);
            sexp_push_char(input, string);
            length = length.wrapping_sub(1);
            length;
        }
    } else if mode as libc::c_uint != SEXP_ADVANCED as libc::c_int as libc::c_uint {
        die(
            b"Encountered advanced string in canonical mode.\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        let mut current_block_31: u64;
        match (*input).c as libc::c_int {
            34 => {
                while length != 0 {
                    if sexp_get_quoted_char(input) != 0 {
                        sexp_push_char(input, string);
                    } else {
                        die(
                            b"Unexpected end of string.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    length = length.wrapping_sub(1);
                    length;
                }
                if sexp_get_quoted_char(input) != 0 {
                    die(
                        b"Quoted string longer than expected.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_31 = 5494826135382683477;
            }
            35 => {
                sexp_input_start_coding(input, &nettle_base16, '#' as i32 as uint8_t);
                current_block_31 = 9530210482322611345;
            }
            124 => {
                sexp_input_start_coding(input, &nettle_base64, '|' as i32 as uint8_t);
                current_block_31 = 9530210482322611345;
            }
            _ => {
                die(b"Invalid string.\n\0" as *const u8 as *const libc::c_char);
            }
        }
        match current_block_31 {
            9530210482322611345 => {
                while length != 0 {
                    sexp_next_char(input);
                    sexp_push_char(input, string);
                    length = length.wrapping_sub(1);
                    length;
                }
                sexp_get_char(input);
                if (*input).ctype as libc::c_uint
                    != SEXP_END_CHAR as libc::c_int as libc::c_uint
                {
                    die(
                        b"Coded string too long.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                sexp_input_end_coding(input);
            }
            _ => {}
        }
    }
    sexp_get_char(input);
}
unsafe extern "C" fn sexp_get_comment(
    mut input: *mut sexp_input,
    mut string: *mut nettle_buffer,
) {
    nettle_buffer_reset(string);
    if (*input).ctype as libc::c_uint == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"input->ctype == SEXP_NORMAL_CHAR\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_3825: {
        if (*input).ctype as libc::c_uint
            == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"input->ctype == SEXP_NORMAL_CHAR\0" as *const u8
                    as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                349 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*input).c as libc::c_int == ';' as i32 {} else {
        __assert_fail(
            b"input->c == ';'\0" as *const u8 as *const libc::c_char,
            b"input.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_3782: {
        if (*input).c as libc::c_int == ';' as i32 {} else {
            __assert_fail(
                b"input->c == ';'\0" as *const u8 as *const libc::c_char,
                b"input.c\0" as *const u8 as *const libc::c_char,
                350 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"void sexp_get_comment(struct sexp_input *, struct nettle_buffer *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    loop {
        sexp_push_char(input, string);
        sexp_get_raw_char(input);
        if !((*input).ctype as libc::c_uint
            == SEXP_NORMAL_CHAR as libc::c_int as libc::c_uint
            && (*input).c as libc::c_int != '\n' as i32)
        {
            break;
        }
    }
    (*input).token = SEXP_COMMENT;
}
pub unsafe extern "C" fn sexp_get_token(
    mut input: *mut sexp_input,
    mut mode: sexp_mode,
    mut string: *mut nettle_buffer,
) {
    loop {
        match (*input).ctype as libc::c_uint {
            1 => {
                (*input).token = SEXP_EOF;
                return;
            }
            2 => {
                (*input).token = SEXP_CODING_END;
                sexp_input_end_coding(input);
                sexp_get_char(input);
                return;
            }
            0 => {
                match (*input).c as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        sexp_get_string_length(input, mode, string);
                        return;
                    }
                    40 => {
                        (*input).token = SEXP_LIST_START;
                        sexp_get_char(input);
                        return;
                    }
                    41 => {
                        (*input).token = SEXP_LIST_END;
                        sexp_get_char(input);
                        return;
                    }
                    91 => {
                        (*input).token = SEXP_DISPLAY_START;
                        sexp_get_char(input);
                        return;
                    }
                    93 => {
                        (*input).token = SEXP_DISPLAY_END;
                        sexp_get_char(input);
                        return;
                    }
                    123 => {
                        if mode as libc::c_uint
                            == SEXP_CANONICAL as libc::c_int as libc::c_uint
                        {
                            die(
                                b"Unexpected transport data in canonical mode.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        sexp_input_start_coding(
                            input,
                            &nettle_base64,
                            '}' as i32 as uint8_t,
                        );
                        sexp_get_char(input);
                        (*input).token = SEXP_TRANSPORT_START;
                        return;
                    }
                    32 | 9 | 10 | 13 => {
                        if mode as libc::c_uint
                            == SEXP_CANONICAL as libc::c_int as libc::c_uint
                        {
                            die(
                                b"Whitespace encountered in canonical mode.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        sexp_get_char(input);
                    }
                    59 => {
                        if mode as libc::c_uint
                            == SEXP_CANONICAL as libc::c_int as libc::c_uint
                        {
                            die(
                                b"Comment encountered in canonical mode.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        sexp_get_comment(input, string);
                        return;
                    }
                    _ => {
                        if mode as libc::c_uint
                            != SEXP_ADVANCED as libc::c_int as libc::c_uint
                        {
                            die(
                                b"Encountered advanced string in canonical mode.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        sexp_get_string(input, string);
                        return;
                    }
                }
            }
            _ => {}
        }
    };
}

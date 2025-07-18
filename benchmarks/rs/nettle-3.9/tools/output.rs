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
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static sexp_token_chars: [libc::c_char; 128];
    static nettle_base64: nettle_armor;
    static nettle_base16: nettle_armor;
    fn die(format: *const libc::c_char, _: ...) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type sexp_mode = libc::c_uint;
pub const SEXP_TRANSPORT: sexp_mode = 2;
pub const SEXP_ADVANCED: sexp_mode = 1;
pub const SEXP_CANONICAL: sexp_mode = 0;
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
pub unsafe extern "C" fn sexp_output_init(
    mut output: *mut sexp_output,
    mut f: *mut FILE,
    mut width: libc::c_uint,
    mut prefer_hex: libc::c_int,
) {
    (*output).f = f;
    (*output).line_width = width;
    (*output).coding = 0 as *const nettle_armor;
    (*output).prefer_hex = prefer_hex;
    (*output).hash = 0 as *const nettle_hash;
    (*output).ctx = 0 as *mut libc::c_void;
    (*output).pos = 0 as libc::c_int as libc::c_uint;
    (*output).soft_newline = 0 as libc::c_int;
}
pub unsafe extern "C" fn sexp_output_hash_init(
    mut output: *mut sexp_output,
    mut hash: *const nettle_hash,
    mut ctx: *mut libc::c_void,
) {
    (*output).hash = hash;
    (*output).ctx = ctx;
    ((*hash).init).unwrap()(ctx);
}
unsafe extern "C" fn sexp_put_raw_char(mut output: *mut sexp_output, mut c: uint8_t) {
    if putc(c as libc::c_int, (*output).f) < 0 as libc::c_int {
        die(
            b"Write failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    (*output).pos = ((*output).pos).wrapping_add(1);
    (*output).pos;
    (*output).soft_newline = 0 as libc::c_int;
}
pub unsafe extern "C" fn sexp_put_newline(
    mut output: *mut sexp_output,
    mut indent: libc::c_uint,
) {
    if (*output).soft_newline != 0 {
        (*output).soft_newline = 0 as libc::c_int;
    } else {
        let mut i: libc::c_uint = 0;
        sexp_put_raw_char(output, '\n' as i32 as uint8_t);
        (*output).pos = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int as libc::c_uint;
        while i < indent {
            sexp_put_raw_char(output, ' ' as i32 as uint8_t);
            i = i.wrapping_add(1);
            i;
        }
        (*output).pos = indent;
    };
}
pub unsafe extern "C" fn sexp_put_soft_newline(
    mut output: *mut sexp_output,
    mut indent: libc::c_uint,
) {
    sexp_put_newline(output, indent);
    (*output).soft_newline = 1 as libc::c_int;
}
pub unsafe extern "C" fn sexp_put_char(mut output: *mut sexp_output, mut c: uint8_t) {
    if !((*output).coding).is_null() {
        let mut encoded: [libc::c_char; 2] = [0; 2];
        let mut done: libc::c_uint = 0;
        let mut i: libc::c_uint = 0;
        done = ((*(*output).coding).encode_update)
            .unwrap()(
            &mut (*output).base64 as *mut base64_decode_ctx as *mut libc::c_void,
            encoded.as_mut_ptr(),
            1 as libc::c_int as size_t,
            &mut c,
        ) as libc::c_uint;
        if done as libc::c_ulong
            <= ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"done <= sizeof(encoded)\0" as *const u8 as *const libc::c_char,
                b"output.c\0" as *const u8 as *const libc::c_char,
                124 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void sexp_put_char(struct sexp_output *, uint8_t)\0"))
                    .as_ptr(),
            );
        }
        'c_2958: {
            if done as libc::c_ulong
                <= ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
            {} else {
                __assert_fail(
                    b"done <= sizeof(encoded)\0" as *const u8 as *const libc::c_char,
                    b"output.c\0" as *const u8 as *const libc::c_char,
                    124 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"void sexp_put_char(struct sexp_output *, uint8_t)\0"))
                        .as_ptr(),
                );
            }
        };
        i = 0 as libc::c_int as libc::c_uint;
        while i < done {
            if (*output).line_width != 0 && (*output).pos >= (*output).line_width
                && (*output).pos
                    >= ((*output).coding_indent)
                        .wrapping_add(10 as libc::c_int as libc::c_uint)
            {
                sexp_put_newline(output, (*output).coding_indent);
            }
            sexp_put_raw_char(output, encoded[i as usize] as uint8_t);
            i = i.wrapping_add(1);
            i;
        }
    } else if !((*output).hash).is_null() {
        ((*(*output).hash).update)
            .unwrap()((*output).ctx, 1 as libc::c_int as size_t, &mut c);
    } else {
        sexp_put_raw_char(output, c);
    };
}
pub unsafe extern "C" fn sexp_put_data(
    mut output: *mut sexp_output,
    mut length: libc::c_uint,
    mut data: *const uint8_t,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        sexp_put_char(output, *data.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn sexp_put_length(
    mut output: *mut sexp_output,
    mut length: libc::c_uint,
) {
    let mut digit: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    loop {
        let mut next: libc::c_uint = digit
            .wrapping_mul(10 as libc::c_int as libc::c_uint);
        if next > length {
            break;
        }
        digit = next;
    }
    while digit != 0 {
        sexp_put_char(
            output,
            ('0' as i32 as libc::c_uint).wrapping_add(length.wrapping_div(digit))
                as uint8_t,
        );
        length = length.wrapping_rem(digit);
        digit = digit.wrapping_div(10 as libc::c_int as libc::c_uint);
    }
}
pub unsafe extern "C" fn sexp_put_code_start(
    mut output: *mut sexp_output,
    mut coding: *const nettle_armor,
) {
    if ((*output).coding).is_null() {} else {
        __assert_fail(
            b"!output->coding\0" as *const u8 as *const libc::c_char,
            b"output.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void sexp_put_code_start(struct sexp_output *, const struct nettle_armor *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3098: {
        if ((*output).coding).is_null() {} else {
            __assert_fail(
                b"!output->coding\0" as *const u8 as *const libc::c_char,
                b"output.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void sexp_put_code_start(struct sexp_output *, const struct nettle_armor *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*output).coding_indent = (*output).pos;
    (*output).coding = coding;
    ((*(*output).coding).encode_init)
        .unwrap()(&mut (*output).base64 as *mut base64_decode_ctx as *mut libc::c_void);
}
pub unsafe extern "C" fn sexp_put_code_end(mut output: *mut sexp_output) {
    let mut encoded: [libc::c_char; 3] = [0; 3];
    let mut done: libc::c_uint = 0;
    if !((*output).coding).is_null() {} else {
        __assert_fail(
            b"output->coding\0" as *const u8 as *const libc::c_char,
            b"output.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                .as_ptr(),
        );
    }
    'c_3231: {
        if !((*output).coding).is_null() {} else {
            __assert_fail(
                b"output->coding\0" as *const u8 as *const libc::c_char,
                b"output.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                    .as_ptr(),
            );
        }
    };
    done = ((*(*output).coding).encode_final)
        .unwrap()(
        &mut (*output).base64 as *mut base64_decode_ctx as *mut libc::c_void,
        encoded.as_mut_ptr(),
    ) as libc::c_uint;
    if done as libc::c_ulong
        <= ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"done <= sizeof(encoded)\0" as *const u8 as *const libc::c_char,
            b"output.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                .as_ptr(),
        );
    }
    'c_3171: {
        if done as libc::c_ulong
            <= ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"done <= sizeof(encoded)\0" as *const u8 as *const libc::c_char,
                b"output.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void sexp_put_code_end(struct sexp_output *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*output).coding = 0 as *const nettle_armor;
    sexp_put_data(output, done, encoded.as_mut_ptr() as *const uint8_t);
}
pub unsafe extern "C" fn sexp_put_string(
    mut output: *mut sexp_output,
    mut mode: sexp_mode,
    mut string: *mut nettle_buffer,
) {
    if (*string).size == 0 {
        sexp_put_data(
            output,
            2 as libc::c_int as libc::c_uint,
            (if mode as libc::c_uint == SEXP_ADVANCED as libc::c_int as libc::c_uint {
                b"\"\"\0" as *const u8 as *const libc::c_char
            } else {
                b"0:\0" as *const u8 as *const libc::c_char
            }) as *const uint8_t,
        );
    } else if mode as libc::c_uint == SEXP_ADVANCED as libc::c_int as libc::c_uint {
        let mut i: libc::c_uint = 0;
        let mut token: libc::c_int = ((*((*string).contents)
            .offset(0 as libc::c_int as isize) as libc::c_int) < '0' as i32
            || *((*string).contents).offset(0 as libc::c_int as isize) as libc::c_int
                > '9' as i32) as libc::c_int;
        let mut quote_friendly: libc::c_int = 1 as libc::c_int;
        static mut escape_names: [libc::c_char; 32] = [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            'b' as i32 as libc::c_char,
            't' as i32 as libc::c_char,
            'n' as i32 as libc::c_char,
            0 as libc::c_int as libc::c_char,
            'f' as i32 as libc::c_char,
            'r' as i32 as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ];
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*string).size {
            let mut c: uint8_t = *((*string).contents).offset(i as isize);
            if token
                & !((c as libc::c_int) < 0x80 as libc::c_int
                    && sexp_token_chars[c as usize] as libc::c_int != 0) as libc::c_int
                != 0
            {
                token = 0 as libc::c_int;
            }
            if quote_friendly != 0 {
                if c as libc::c_int >= 0x7f as libc::c_int {
                    quote_friendly = 0 as libc::c_int;
                } else if (c as libc::c_int) < 0x20 as libc::c_int
                    && escape_names[c as usize] == 0
                {
                    quote_friendly = 0 as libc::c_int;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if token != 0 {
            sexp_put_data(output, (*string).size as libc::c_uint, (*string).contents);
        } else if quote_friendly != 0 {
            sexp_put_char(output, '"' as i32 as uint8_t);
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) < (*string).size {
                let mut escape: libc::c_int = 0 as libc::c_int;
                let mut c_0: uint8_t = *((*string).contents).offset(i as isize);
                if (c_0 as libc::c_int) < 0x7f as libc::c_int {} else {
                    __assert_fail(
                        b"c < 0x7f\0" as *const u8 as *const libc::c_char,
                        b"output.c\0" as *const u8 as *const libc::c_char,
                        248 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 83],
                            &[libc::c_char; 83],
                        >(
                            b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3655: {
                    if (c_0 as libc::c_int) < 0x7f as libc::c_int {} else {
                        __assert_fail(
                            b"c < 0x7f\0" as *const u8 as *const libc::c_char,
                            b"output.c\0" as *const u8 as *const libc::c_char,
                            248 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 83],
                                &[libc::c_char; 83],
                            >(
                                b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if c_0 as libc::c_int == '\\' as i32 || c_0 as libc::c_int == '"' as i32
                {
                    escape = 1 as libc::c_int;
                } else if (c_0 as libc::c_int) < 0x20 as libc::c_int {
                    escape = 1 as libc::c_int;
                    c_0 = escape_names[c_0 as usize] as uint8_t;
                    if c_0 != 0 {} else {
                        __assert_fail(
                            b"c\0" as *const u8 as *const libc::c_char,
                            b"output.c\0" as *const u8 as *const libc::c_char,
                            256 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 83],
                                &[libc::c_char; 83],
                            >(
                                b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_3522: {
                        if c_0 != 0 {} else {
                            __assert_fail(
                                b"c\0" as *const u8 as *const libc::c_char,
                                b"output.c\0" as *const u8 as *const libc::c_char,
                                256 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 83],
                                    &[libc::c_char; 83],
                                >(
                                    b"void sexp_put_string(struct sexp_output *, enum sexp_mode, struct nettle_buffer *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                if escape != 0 {
                    sexp_put_char(output, '\\' as i32 as uint8_t);
                }
                sexp_put_char(output, c_0);
                i = i.wrapping_add(1);
                i;
            }
            sexp_put_char(output, '"' as i32 as uint8_t);
        } else {
            let mut delimiter: uint8_t = 0;
            let mut coding: *const nettle_armor = 0 as *const nettle_armor;
            if (*output).prefer_hex != 0 {
                delimiter = '#' as i32 as uint8_t;
                coding = &nettle_base16;
            } else {
                delimiter = '|' as i32 as uint8_t;
                coding = &nettle_base64;
            }
            sexp_put_char(output, delimiter);
            sexp_put_code_start(output, coding);
            sexp_put_data(output, (*string).size as libc::c_uint, (*string).contents);
            sexp_put_code_end(output);
            sexp_put_char(output, delimiter);
        }
    } else {
        sexp_put_length(output, (*string).size as libc::c_uint);
        sexp_put_char(output, ':' as i32 as uint8_t);
        sexp_put_data(output, (*string).size as libc::c_uint, (*string).contents);
    };
}
pub unsafe extern "C" fn sexp_put_digest(mut output: *mut sexp_output) {
    let mut digest: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul((*(*output).hash).digest_size as libc::c_ulong) as usize,
    );
    digest = fresh0.leak().as_mut_ptr() as *mut uint8_t;
    if !((*output).hash).is_null() {} else {
        __assert_fail(
            b"output->hash\0" as *const u8 as *const libc::c_char,
            b"output.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void sexp_put_digest(struct sexp_output *)\0"))
                .as_ptr(),
        );
    }
    'c_3926: {
        if !((*output).hash).is_null() {} else {
            __assert_fail(
                b"output->hash\0" as *const u8 as *const libc::c_char,
                b"output.c\0" as *const u8 as *const libc::c_char,
                304 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void sexp_put_digest(struct sexp_output *)\0"))
                    .as_ptr(),
            );
        }
    };
    ((*(*output).hash).digest)
        .unwrap()((*output).ctx, (*(*output).hash).digest_size as size_t, digest);
    sexp_put_code_start(output, &nettle_base16);
    sexp_put_data(output, (*(*output).hash).digest_size, digest);
    sexp_put_code_end(output);
}

use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn nettle_buffer_grow(buffer: *mut nettle_buffer, length: size_t) -> libc::c_int;
    fn nettle_buffer_write(
        buffer: *mut nettle_buffer,
        length: size_t,
        data: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_buffer_space(buffer: *mut nettle_buffer, length: size_t) -> *mut uint8_t;
    fn nettle_mpz_sizeinbase_256_s(x: *const __mpz_struct) -> size_t;
    fn nettle_mpz_get_str_256(length: size_t, s: *mut uint8_t, x: *const __mpz_struct);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
pub type mpz_srcptr = *const __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mp_limb_t = libc::c_ulong;
unsafe extern "C" fn format_prefix(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
) -> libc::c_uint {
    let mut digit: size_t = 1 as libc::c_int as size_t;
    let mut prefix_length: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    loop {
        let mut next: size_t = digit.wrapping_mul(10 as libc::c_int as libc::c_ulong);
        if next > length {
            break;
        }
        prefix_length = prefix_length.wrapping_add(1);
        prefix_length;
        digit = next;
    }
    if !buffer.is_null() {
        while digit != 0 {
            if !(((*buffer).size < (*buffer).alloc
                || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
                && {
                    let fresh0 = (*buffer).size;
                    (*buffer).size = ((*buffer).size).wrapping_add(1);
                    *((*buffer).contents)
                        .offset(
                            fresh0 as isize,
                        ) = ('0' as i32 as libc::c_ulong)
                        .wrapping_add(length.wrapping_div(digit)) as uint8_t;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int as libc::c_uint;
            }
            length = (length as libc::c_ulong).wrapping_rem(digit) as size_t as size_t;
            digit = (digit as libc::c_ulong)
                .wrapping_div(10 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh1 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh1 as isize) = ':' as i32 as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    return prefix_length.wrapping_add(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn format_string(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut s: *const uint8_t,
) -> size_t {
    let mut prefix_length: libc::c_uint = format_prefix(buffer, length);
    if prefix_length == 0 {
        return 0 as libc::c_int as size_t;
    }
    if !buffer.is_null() && nettle_buffer_write(buffer, length, s) == 0 {
        return 0 as libc::c_int as size_t;
    }
    return (prefix_length as libc::c_ulong).wrapping_add(length);
}
#[inline]
unsafe extern "C" fn strlen_u8(mut s: *const uint8_t) -> size_t {
    return strlen(s as *const libc::c_char);
}
pub unsafe extern "C" fn nettle_sexp_vformat(
    mut buffer: *mut nettle_buffer,
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> size_t {
    let mut nesting: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut done: size_t = 0 as libc::c_int as size_t;
    loop {
        let fresh2 = format;
        format = format.offset(1);
        match *fresh2 as libc::c_int {
            32 | 9 => {}
            0 => {
                if nesting == 0 {} else {
                    __assert_fail(
                        b"!nesting\0" as *const u8 as *const libc::c_char,
                        b"sexp-format.c\0" as *const u8 as *const libc::c_char,
                        126 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 89],
                            &[libc::c_char; 89],
                        >(
                            b"size_t nettle_sexp_vformat(struct nettle_buffer *, const char *, struct __va_list_tag *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3797: {
                    if nesting == 0 {} else {
                        __assert_fail(
                            b"!nesting\0" as *const u8 as *const libc::c_char,
                            b"sexp-format.c\0" as *const u8 as *const libc::c_char,
                            126 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 89],
                                &[libc::c_char; 89],
                            >(
                                b"size_t nettle_sexp_vformat(struct nettle_buffer *, const char *, struct __va_list_tag *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                return done;
            }
            40 => {
                if !buffer.is_null()
                    && !(((*buffer).size < (*buffer).alloc
                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
                        && {
                            let fresh3 = (*buffer).size;
                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                            *((*buffer).contents)
                                .offset(fresh3 as isize) = '(' as i32 as uint8_t;
                            1 as libc::c_int != 0
                        })
                {
                    return 0 as libc::c_int as size_t;
                }
                done = done.wrapping_add(1);
                done;
                nesting = nesting.wrapping_add(1);
                nesting;
            }
            41 => {
                if nesting != 0 {} else {
                    __assert_fail(
                        b"nesting\0" as *const u8 as *const libc::c_char,
                        b"sexp-format.c\0" as *const u8 as *const libc::c_char,
                        139 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 89],
                            &[libc::c_char; 89],
                        >(
                            b"size_t nettle_sexp_vformat(struct nettle_buffer *, const char *, struct __va_list_tag *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3699: {
                    if nesting != 0 {} else {
                        __assert_fail(
                            b"nesting\0" as *const u8 as *const libc::c_char,
                            b"sexp-format.c\0" as *const u8 as *const libc::c_char,
                            139 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 89],
                                &[libc::c_char; 89],
                            >(
                                b"size_t nettle_sexp_vformat(struct nettle_buffer *, const char *, struct __va_list_tag *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if !buffer.is_null()
                    && !(((*buffer).size < (*buffer).alloc
                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
                        && {
                            let fresh4 = (*buffer).size;
                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                            *((*buffer).contents)
                                .offset(fresh4 as isize) = ')' as i32 as uint8_t;
                            1 as libc::c_int != 0
                        })
                {
                    return 0 as libc::c_int as size_t;
                }
                done = done.wrapping_add(1);
                done;
                nesting = nesting.wrapping_sub(1);
                nesting;
            }
            37 => {
                let mut nul_flag: libc::c_int = 0 as libc::c_int;
                if *format as libc::c_int == '0' as i32 {
                    format = format.offset(1);
                    format;
                    nul_flag = 1 as libc::c_int;
                }
                let mut current_block_93: u64;
                let fresh5 = format;
                format = format.offset(1);
                match *fresh5 as libc::c_int {
                    40 | 41 => {
                        if !buffer.is_null()
                            && !(((*buffer).size < (*buffer).alloc
                                || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                    != 0)
                                && {
                                    let fresh6 = (*buffer).size;
                                    (*buffer).size = ((*buffer).size).wrapping_add(1);
                                    *((*buffer).contents)
                                        .offset(
                                            fresh6 as isize,
                                        ) = *format.offset(-(1 as libc::c_int) as isize) as uint8_t;
                                    1 as libc::c_int != 0
                                })
                        {
                            return 0 as libc::c_int as size_t;
                        }
                        done = done.wrapping_add(1);
                        done;
                    }
                    115 => {
                        let mut s: *const uint8_t = 0 as *const uint8_t;
                        let mut length_0: size_t = 0;
                        let mut output_length_0: size_t = 0;
                        if nul_flag != 0 {
                            s = args.arg::<*const uint8_t>();
                            length_0 = strlen_u8(s);
                        } else {
                            length_0 = args.arg::<size_t>();
                            s = args.arg::<*const uint8_t>();
                        }
                        output_length_0 = format_string(buffer, length_0, s);
                        if output_length_0 == 0 {
                            return 0 as libc::c_int as size_t;
                        }
                        done = (done as libc::c_ulong).wrapping_add(output_length_0)
                            as size_t as size_t;
                    }
                    116 => {
                        let mut s_0: *const uint8_t = 0 as *const uint8_t;
                        let mut length_1: size_t = 0;
                        let mut output_length_1: size_t = 0;
                        if nul_flag != 0 {
                            s_0 = args.arg::<*const uint8_t>();
                            if s_0.is_null() {
                                current_block_93 = 5023038348526654800;
                            } else {
                                length_1 = strlen_u8(s_0);
                                current_block_93 = 2480299350034459858;
                            }
                        } else {
                            length_1 = args.arg::<size_t>();
                            s_0 = args.arg::<*const uint8_t>();
                            if s_0.is_null() {
                                current_block_93 = 5023038348526654800;
                            } else {
                                current_block_93 = 2480299350034459858;
                            }
                        }
                        match current_block_93 {
                            5023038348526654800 => {}
                            _ => {
                                if !buffer.is_null()
                                    && !(((*buffer).size < (*buffer).alloc
                                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                            != 0)
                                        && {
                                            let fresh7 = (*buffer).size;
                                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                                            *((*buffer).contents)
                                                .offset(fresh7 as isize) = '[' as i32 as uint8_t;
                                            1 as libc::c_int != 0
                                        })
                                {
                                    return 0 as libc::c_int as size_t;
                                }
                                done = done.wrapping_add(1);
                                done;
                                output_length_1 = format_string(buffer, length_1, s_0);
                                if output_length_1 == 0 {
                                    return 0 as libc::c_int as size_t;
                                }
                                done = (done as libc::c_ulong).wrapping_add(output_length_1)
                                    as size_t as size_t;
                                if !buffer.is_null()
                                    && !(((*buffer).size < (*buffer).alloc
                                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                            != 0)
                                        && {
                                            let fresh8 = (*buffer).size;
                                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                                            *((*buffer).contents)
                                                .offset(fresh8 as isize) = ']' as i32 as uint8_t;
                                            1 as libc::c_int != 0
                                        })
                                {
                                    return 0 as libc::c_int as size_t;
                                }
                                done = done.wrapping_add(1);
                                done;
                            }
                        }
                    }
                    108 => {
                        let mut s_1: *const uint8_t = 0 as *const uint8_t;
                        let mut length_2: size_t = 0;
                        if nul_flag != 0 {
                            s_1 = args.arg::<*const uint8_t>();
                            length_2 = strlen_u8(s_1);
                        } else {
                            length_2 = args.arg::<size_t>();
                            s_1 = args.arg::<*const uint8_t>();
                        }
                        if !buffer.is_null()
                            && nettle_buffer_write(buffer, length_2, s_1) == 0
                        {
                            return 0 as libc::c_int as size_t;
                        }
                        done = (done as libc::c_ulong).wrapping_add(length_2) as size_t
                            as size_t;
                    }
                    105 => {
                        let mut x: uint32_t = args.arg::<uint32_t>();
                        let mut length_3: libc::c_uint = 0;
                        if x < 0x80 as libc::c_int as libc::c_uint {
                            length_3 = 1 as libc::c_int as libc::c_uint;
                        } else if (x as libc::c_long) < 0x8000 as libc::c_long {
                            length_3 = 2 as libc::c_int as libc::c_uint;
                        } else if (x as libc::c_long) < 0x800000 as libc::c_long {
                            length_3 = 3 as libc::c_int as libc::c_uint;
                        } else if (x as libc::c_long) < 0x80000000 as libc::c_long {
                            length_3 = 4 as libc::c_int as libc::c_uint;
                        } else {
                            length_3 = 5 as libc::c_int as libc::c_uint;
                        }
                        if !buffer.is_null()
                            && !(((*buffer).size < (*buffer).alloc
                                || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                    != 0)
                                && {
                                    let fresh9 = (*buffer).size;
                                    (*buffer).size = ((*buffer).size).wrapping_add(1);
                                    *((*buffer).contents)
                                        .offset(
                                            fresh9 as isize,
                                        ) = ('0' as i32 as libc::c_uint).wrapping_add(length_3)
                                        as uint8_t;
                                    1 as libc::c_int != 0
                                }
                                && (((*buffer).size < (*buffer).alloc
                                    || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                        != 0)
                                    && {
                                        let fresh10 = (*buffer).size;
                                        (*buffer).size = ((*buffer).size).wrapping_add(1);
                                        *((*buffer).contents)
                                            .offset(fresh10 as isize) = ':' as i32 as uint8_t;
                                        1 as libc::c_int != 0
                                    }))
                        {
                            return 0 as libc::c_int as size_t;
                        }
                        done = (done as libc::c_ulong)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_uint).wrapping_add(length_3)
                                    as libc::c_ulong,
                            ) as size_t as size_t;
                        if !buffer.is_null() {
                            let mut current_block_80: u64;
                            match length_3 {
                                5 => {
                                    if !(((*buffer).size < (*buffer).alloc
                                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                            != 0)
                                        && {
                                            let fresh11 = (*buffer).size;
                                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                                            *((*buffer).contents)
                                                .offset(fresh11 as isize) = 0 as libc::c_int as uint8_t;
                                            1 as libc::c_int != 0
                                        })
                                    {
                                        return 0 as libc::c_int as size_t;
                                    }
                                    current_block_80 = 15439061234641355781;
                                }
                                4 => {
                                    current_block_80 = 15439061234641355781;
                                }
                                3 => {
                                    current_block_80 = 4859325652183472587;
                                }
                                2 => {
                                    current_block_80 = 18077455698464958581;
                                }
                                1 => {
                                    current_block_80 = 8528973633699019552;
                                }
                                _ => {
                                    abort();
                                }
                            }
                            match current_block_80 {
                                15439061234641355781 => {
                                    if !(((*buffer).size < (*buffer).alloc
                                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                            != 0)
                                        && {
                                            let fresh12 = (*buffer).size;
                                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                                            *((*buffer).contents)
                                                .offset(
                                                    fresh12 as isize,
                                                ) = (x >> 24 as libc::c_int) as uint8_t;
                                            1 as libc::c_int != 0
                                        })
                                    {
                                        return 0 as libc::c_int as size_t;
                                    }
                                    current_block_80 = 4859325652183472587;
                                }
                                _ => {}
                            }
                            match current_block_80 {
                                4859325652183472587 => {
                                    if !(((*buffer).size < (*buffer).alloc
                                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                            != 0)
                                        && {
                                            let fresh13 = (*buffer).size;
                                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                                            *((*buffer).contents)
                                                .offset(
                                                    fresh13 as isize,
                                                ) = (x >> 16 as libc::c_int
                                                & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                                            1 as libc::c_int != 0
                                        })
                                    {
                                        return 0 as libc::c_int as size_t;
                                    }
                                    current_block_80 = 18077455698464958581;
                                }
                                _ => {}
                            }
                            match current_block_80 {
                                18077455698464958581 => {
                                    if !(((*buffer).size < (*buffer).alloc
                                        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                            != 0)
                                        && {
                                            let fresh14 = (*buffer).size;
                                            (*buffer).size = ((*buffer).size).wrapping_add(1);
                                            *((*buffer).contents)
                                                .offset(
                                                    fresh14 as isize,
                                                ) = (x >> 8 as libc::c_int
                                                & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                                            1 as libc::c_int != 0
                                        })
                                    {
                                        return 0 as libc::c_int as size_t;
                                    }
                                }
                                _ => {}
                            }
                            if !(((*buffer).size < (*buffer).alloc
                                || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t)
                                    != 0)
                                && {
                                    let fresh15 = (*buffer).size;
                                    (*buffer).size = ((*buffer).size).wrapping_add(1);
                                    *((*buffer).contents)
                                        .offset(
                                            fresh15 as isize,
                                        ) = (x & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                                    1 as libc::c_int != 0
                                })
                            {
                                return 0 as libc::c_int as size_t;
                            }
                        }
                    }
                    98 => {
                        let mut n: mpz_srcptr = args.arg::<mpz_srcptr>();
                        let mut length_4: size_t = 0;
                        let mut prefix_length: libc::c_uint = 0;
                        length_4 = nettle_mpz_sizeinbase_256_s(n);
                        prefix_length = format_prefix(buffer, length_4);
                        if prefix_length == 0 {
                            return 0 as libc::c_int as size_t;
                        }
                        done = (done as libc::c_ulong)
                            .wrapping_add(prefix_length as libc::c_ulong) as size_t
                            as size_t;
                        if !buffer.is_null() {
                            let mut space: *mut uint8_t = nettle_buffer_space(
                                buffer,
                                length_4,
                            );
                            if space.is_null() {
                                return 0 as libc::c_int as size_t;
                            }
                            nettle_mpz_get_str_256(length_4, space, n);
                        }
                        done = (done as libc::c_ulong).wrapping_add(length_4) as size_t
                            as size_t;
                    }
                    _ => {
                        abort();
                    }
                }
            }
            _ => {
                let mut start: *const libc::c_char = format
                    .offset(-(1 as libc::c_int as isize));
                let mut length: size_t = (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        strcspn(format, b"()% \t\0" as *const u8 as *const libc::c_char),
                    );
                let mut output_length: size_t = format_string(
                    buffer,
                    length,
                    start as *const uint8_t,
                );
                if output_length == 0 {
                    return 0 as libc::c_int as size_t;
                }
                done = (done as libc::c_ulong).wrapping_add(output_length) as size_t
                    as size_t;
                format = start.offset(length as isize);
            }
        }
    };
}
pub unsafe extern "C" fn nettle_sexp_format(
    mut buffer: *mut nettle_buffer,
    mut format: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut done: size_t = 0;
    args_0 = args.clone();
    done = nettle_sexp_vformat(buffer, format, args_0.as_va_list());
    return done;
}

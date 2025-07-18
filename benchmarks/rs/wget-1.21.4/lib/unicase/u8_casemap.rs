use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type unicode_normalization_form;
    fn u8_normalize(
        nf: uninorm_t,
        s: *const uint8_t,
        n: size_t,
        resultbuf: *mut uint8_t,
        lengthp: *mut size_t,
    ) -> *mut uint8_t;
    fn __errno_location() -> *mut libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn u8_mbtouc_unsafe_aux(
        puc: *mut ucs4_t,
        s: *const uint8_t,
        n: size_t,
    ) -> libc::c_int;
    fn u8_uctomb_aux(s: *mut uint8_t, uc: ucs4_t, n: ptrdiff_t) -> libc::c_int;
    fn u8_cpy(dest: *mut uint8_t, src: *const uint8_t, n: size_t) -> *mut uint8_t;
    fn uc_combining_class(uc: ucs4_t) -> libc::c_int;
    fn uc_is_property_soft_dotted(uc: ucs4_t) -> bool;
    fn uc_is_cased(uc: ucs4_t) -> bool;
    fn uc_is_case_ignorable(uc: ucs4_t) -> bool;
    fn gl_unicase_special_lookup(
        str: *const libc::c_char,
        len: size_t,
    ) -> *const special_casing_rule;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type uninorm_t = *const unicode_normalization_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_prefix_context {
    pub last_char_except_ignorable: uint32_t,
    pub last_char_normal_or_above: uint32_t,
}
pub type casing_prefix_context_t = casing_prefix_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_suffix_context {
    pub first_char_except_ignorable: uint32_t,
    pub bits: uint32_t,
}
pub type casing_suffix_context_t = casing_suffix_context;
pub const UC_CCC_NR: C2RustUnnamed = 0;
pub const UC_CCC_A: C2RustUnnamed = 230;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct special_casing_rule {
    pub code: [libc::c_char; 3],
    #[bitfield(name = "has_next", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "context", ty = "libc::c_int", bits = "1..=7")]
    pub has_next_context: [u8; 1],
    pub language: [libc::c_char; 2],
    pub upper: [libc::c_ushort; 3],
    pub lower: [libc::c_ushort; 3],
    pub title: [libc::c_ushort; 3],
    pub casefold: [libc::c_ushort; 3],
}
pub const SCC_AFTER_I: C2RustUnnamed_0 = 5;
pub const SCC_BEFORE_DOT: C2RustUnnamed_0 = 4;
pub const SCC_MORE_ABOVE: C2RustUnnamed_0 = 3;
pub const SCC_AFTER_SOFT_DOTTED: C2RustUnnamed_0 = 2;
pub const SCC_FINAL_SIGMA: C2RustUnnamed_0 = 1;
pub const SCC_ALWAYS: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const UC_CCC_IS: C2RustUnnamed = 240;
pub const UC_CCC_DA: C2RustUnnamed = 234;
pub const UC_CCC_DB: C2RustUnnamed = 233;
pub const UC_CCC_AR: C2RustUnnamed = 232;
pub const UC_CCC_AL: C2RustUnnamed = 228;
pub const UC_CCC_R: C2RustUnnamed = 226;
pub const UC_CCC_L: C2RustUnnamed = 224;
pub const UC_CCC_BR: C2RustUnnamed = 222;
pub const UC_CCC_B: C2RustUnnamed = 220;
pub const UC_CCC_BL: C2RustUnnamed = 218;
pub const UC_CCC_ATAR: C2RustUnnamed = 216;
pub const UC_CCC_ATA: C2RustUnnamed = 214;
pub const UC_CCC_ATB: C2RustUnnamed = 202;
pub const UC_CCC_ATBL: C2RustUnnamed = 200;
pub const UC_CCC_VR: C2RustUnnamed = 9;
pub const UC_CCC_KV: C2RustUnnamed = 8;
pub const UC_CCC_NK: C2RustUnnamed = 7;
pub const UC_CCC_OV: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn u8_mbtouc_unsafe(
    mut puc: *mut ucs4_t,
    mut s: *const uint8_t,
    mut n: size_t,
) -> libc::c_int {
    let mut c: uint8_t = *s;
    if (c as libc::c_int) < 0x80 as libc::c_int {
        *puc = c as ucs4_t;
        return 1 as libc::c_int;
    } else {
        return u8_mbtouc_unsafe_aux(puc, s, n)
    };
}
#[inline]
unsafe extern "C" fn u8_uctomb(
    mut s: *mut uint8_t,
    mut uc: ucs4_t,
    mut n: ptrdiff_t,
) -> libc::c_int {
    if uc < 0x80 as libc::c_int as libc::c_uint && n > 0 as libc::c_int as libc::c_long {
        *s.offset(0 as libc::c_int as isize) = uc as uint8_t;
        return 1 as libc::c_int;
    } else {
        return u8_uctomb_aux(s, uc, n)
    };
}
pub unsafe extern "C" fn u8_casemap(
    mut s: *const uint8_t,
    mut n: size_t,
    mut prefix_context: casing_prefix_context_t,
    mut suffix_context: casing_suffix_context_t,
    mut iso639_language: *const libc::c_char,
    mut single_character_map: Option::<unsafe extern "C" fn(ucs4_t) -> ucs4_t>,
    mut offset_in_rule: size_t,
    mut nf: uninorm_t,
    mut resultbuf: *mut uint8_t,
    mut lengthp: *mut size_t,
) -> *mut uint8_t {
    let mut current_block: u64;
    let mut result: *mut uint8_t = 0 as *mut uint8_t;
    let mut length: size_t = 0;
    let mut allocated: size_t = 0;
    if !nf.is_null() || resultbuf.is_null() {
        result = 0 as *mut uint8_t;
        allocated = 0 as libc::c_int as size_t;
    } else {
        result = resultbuf;
        allocated = *lengthp;
    }
    length = 0 as libc::c_int as size_t;
    let mut s_end: *const uint8_t = s.offset(n as isize);
    let mut last_char_except_ignorable: ucs4_t = prefix_context
        .last_char_except_ignorable;
    let mut last_char_normal_or_above: ucs4_t = prefix_context.last_char_normal_or_above;
    's_43: loop {
        if !(s < s_end) {
            current_block = 17917672080766325409;
            break;
        }
        let mut uc: ucs4_t = 0;
        let mut count: libc::c_int = u8_mbtouc_unsafe(
            &mut uc,
            s,
            s_end.offset_from(s) as libc::c_long as size_t,
        );
        let mut mapped_uc: [ucs4_t; 3] = [0; 3];
        let mut mapped_count: libc::c_uint = 0;
        if uc < 0x10000 as libc::c_int as libc::c_uint {
            let mut code: [libc::c_char; 3] = [0; 3];
            code[0 as libc::c_int
                as usize] = (uc >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as libc::c_char;
            code[1 as libc::c_int
                as usize] = (uc & 0xff as libc::c_int as libc::c_uint) as libc::c_char;
            code[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            loop {
                let mut rule: *const special_casing_rule = gl_unicase_special_lookup(
                    code.as_mut_ptr(),
                    3 as libc::c_int as size_t,
                );
                if rule.is_null() {
                    current_block = 16108440464692313034;
                    break;
                }
                if (*rule).language[0 as libc::c_int as usize] as libc::c_int
                    == '\0' as i32
                    || !iso639_language.is_null()
                        && *iso639_language.offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == (*rule).language[0 as libc::c_int as usize] as libc::c_int
                        && *iso639_language.offset(1 as libc::c_int as isize)
                            as libc::c_int
                            == (*rule).language[1 as libc::c_int as usize] as libc::c_int
                {
                    let mut context: libc::c_int = (*rule).context();
                    let mut applies: bool = false;
                    if context < 0 as libc::c_int {
                        context = -context;
                    }
                    match context {
                        0 => {
                            applies = 1 as libc::c_int != 0;
                        }
                        1 => {
                            applies = uc_is_cased(last_char_except_ignorable);
                            if applies {
                                let mut s2: *const uint8_t = s.offset(count as isize);
                                loop {
                                    if s2 < s_end {
                                        let mut uc2: ucs4_t = 0;
                                        let mut count2: libc::c_int = u8_mbtouc_unsafe(
                                            &mut uc2,
                                            s2,
                                            s_end.offset_from(s2) as libc::c_long as size_t,
                                        );
                                        if !uc_is_case_ignorable(uc2) {
                                            applies = !uc_is_cased(uc2);
                                            break;
                                        } else {
                                            s2 = s2.offset(count2 as isize);
                                        }
                                    } else {
                                        applies = !uc_is_cased(
                                            suffix_context.first_char_except_ignorable,
                                        );
                                        break;
                                    }
                                }
                            }
                        }
                        2 => {
                            applies = uc_is_property_soft_dotted(
                                last_char_normal_or_above,
                            );
                        }
                        3 => {
                            let mut s2_0: *const uint8_t = s.offset(count as isize);
                            applies = 0 as libc::c_int != 0;
                            loop {
                                if s2_0 < s_end {
                                    let mut uc2_0: ucs4_t = 0;
                                    let mut count2_0: libc::c_int = u8_mbtouc_unsafe(
                                        &mut uc2_0,
                                        s2_0,
                                        s_end.offset_from(s2_0) as libc::c_long as size_t,
                                    );
                                    let mut ccc: libc::c_int = uc_combining_class(uc2_0);
                                    if ccc == UC_CCC_A as libc::c_int {
                                        applies = 1 as libc::c_int != 0;
                                        break;
                                    } else {
                                        if ccc == UC_CCC_NR as libc::c_int {
                                            break;
                                        }
                                        s2_0 = s2_0.offset(count2_0 as isize);
                                    }
                                } else {
                                    applies = suffix_context.bits
                                        & 1 as libc::c_int as libc::c_uint
                                        != 0 as libc::c_int as libc::c_uint;
                                    break;
                                }
                            }
                        }
                        4 => {
                            let mut s2_1: *const uint8_t = s.offset(count as isize);
                            applies = 0 as libc::c_int != 0;
                            loop {
                                if s2_1 < s_end {
                                    let mut uc2_1: ucs4_t = 0;
                                    let mut count2_1: libc::c_int = u8_mbtouc_unsafe(
                                        &mut uc2_1,
                                        s2_1,
                                        s_end.offset_from(s2_1) as libc::c_long as size_t,
                                    );
                                    if uc2_1 == 0x307 as libc::c_int as libc::c_uint {
                                        applies = 1 as libc::c_int != 0;
                                        break;
                                    } else {
                                        let mut ccc_0: libc::c_int = uc_combining_class(uc2_1);
                                        if ccc_0 == UC_CCC_A as libc::c_int
                                            || ccc_0 == UC_CCC_NR as libc::c_int
                                        {
                                            break;
                                        }
                                        s2_1 = s2_1.offset(count2_1 as isize);
                                    }
                                } else {
                                    applies = suffix_context.bits
                                        & 2 as libc::c_int as libc::c_uint
                                        != 0 as libc::c_int as libc::c_uint;
                                    break;
                                }
                            }
                        }
                        5 => {
                            applies = last_char_normal_or_above
                                == 'I' as i32 as libc::c_uint;
                        }
                        _ => {
                            abort();
                        }
                    }
                    if (*rule).context() < 0 as libc::c_int {
                        applies = !applies;
                    }
                    if applies {
                        let mut mapped_in_rule: *const libc::c_ushort = (rule
                            as *const libc::c_char)
                            .offset(offset_in_rule as isize) as *const libc::c_ushort;
                        if *mapped_in_rule.offset(0 as libc::c_int as isize)
                            as libc::c_int == 0 as libc::c_int
                        {
                            mapped_count = 0 as libc::c_int as libc::c_uint;
                        } else {
                            mapped_uc[0 as libc::c_int
                                as usize] = *mapped_in_rule
                                .offset(0 as libc::c_int as isize) as ucs4_t;
                            if *mapped_in_rule.offset(1 as libc::c_int as isize)
                                as libc::c_int == 0 as libc::c_int
                            {
                                mapped_count = 1 as libc::c_int as libc::c_uint;
                            } else {
                                mapped_uc[1 as libc::c_int
                                    as usize] = *mapped_in_rule
                                    .offset(1 as libc::c_int as isize) as ucs4_t;
                                if *mapped_in_rule.offset(2 as libc::c_int as isize)
                                    as libc::c_int == 0 as libc::c_int
                                {
                                    mapped_count = 2 as libc::c_int as libc::c_uint;
                                } else {
                                    mapped_uc[2 as libc::c_int
                                        as usize] = *mapped_in_rule
                                        .offset(2 as libc::c_int as isize) as ucs4_t;
                                    mapped_count = 3 as libc::c_int as libc::c_uint;
                                }
                            }
                        }
                        current_block = 8464383504555462953;
                        break;
                    }
                }
                if (*rule).has_next() == 0 {
                    current_block = 16108440464692313034;
                    break;
                }
                code[2 as libc::c_int as usize] += 1;
                code[2 as libc::c_int as usize];
            }
        } else {
            current_block = 16108440464692313034;
        }
        match current_block {
            16108440464692313034 => {
                mapped_uc[0 as libc::c_int as usize] = single_character_map.unwrap()(uc);
                mapped_count = 1 as libc::c_int as libc::c_uint;
            }
            _ => {}
        }
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < mapped_count {
            let mut muc: ucs4_t = mapped_uc[i as usize];
            if length < allocated {
                let mut ret: libc::c_int = u8_uctomb(
                    result.offset(length as isize),
                    muc,
                    allocated.wrapping_sub(length) as ptrdiff_t,
                );
                if ret == -(1 as libc::c_int) {
                    *__errno_location() = 22 as libc::c_int;
                    current_block = 11264440884544272135;
                    break 's_43;
                } else if ret >= 0 as libc::c_int {
                    length = (length as libc::c_ulong).wrapping_add(ret as libc::c_ulong)
                        as size_t as size_t;
                    current_block = 17167606947040001567;
                } else {
                    current_block = 10769842751669019566;
                }
            } else {
                current_block = 10769842751669019566;
            }
            match current_block {
                10769842751669019566 => {
                    let mut old_allocated: size_t = allocated;
                    let mut new_allocated: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(old_allocated);
                    if new_allocated < 64 as libc::c_int as libc::c_ulong {
                        new_allocated = 64 as libc::c_int as size_t;
                    }
                    if new_allocated < old_allocated {
                        abort();
                    }
                    let mut larger_result: *mut uint8_t = 0 as *mut uint8_t;
                    if result.is_null() {
                        larger_result = malloc(
                            new_allocated
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint8_t;
                        if larger_result.is_null() {
                            *__errno_location() = 12 as libc::c_int;
                            current_block = 11264440884544272135;
                            break 's_43;
                        }
                    } else if result == resultbuf {
                        larger_result = malloc(
                            new_allocated
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint8_t;
                        if larger_result.is_null() {
                            *__errno_location() = 12 as libc::c_int;
                            current_block = 11264440884544272135;
                            break 's_43;
                        } else {
                            u8_cpy(larger_result, resultbuf, length);
                        }
                    } else {
                        larger_result = realloc(
                            result as *mut libc::c_void,
                            new_allocated
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint8_t;
                        if larger_result.is_null() {
                            *__errno_location() = 12 as libc::c_int;
                            current_block = 11264440884544272135;
                            break 's_43;
                        }
                    }
                    result = larger_result;
                    allocated = new_allocated;
                    let mut ret_0: libc::c_int = u8_uctomb(
                        result.offset(length as isize),
                        muc,
                        allocated.wrapping_sub(length) as ptrdiff_t,
                    );
                    if ret_0 == -(1 as libc::c_int) {
                        *__errno_location() = 22 as libc::c_int;
                        current_block = 11264440884544272135;
                        break 's_43;
                    } else {
                        if ret_0 < 0 as libc::c_int {
                            abort();
                        }
                        length = (length as libc::c_ulong)
                            .wrapping_add(ret_0 as libc::c_ulong) as size_t as size_t;
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        if !uc_is_case_ignorable(uc) {
            last_char_except_ignorable = uc;
        }
        let mut ccc_1: libc::c_int = uc_combining_class(uc);
        if ccc_1 == UC_CCC_A as libc::c_int || ccc_1 == UC_CCC_NR as libc::c_int {
            last_char_normal_or_above = uc;
        }
        s = s.offset(count as isize);
    }
    match current_block {
        17917672080766325409 => {
            if !nf.is_null() {
                let mut normalized_result: *mut uint8_t = 0 as *mut uint8_t;
                normalized_result = u8_normalize(nf, result, length, resultbuf, lengthp);
                if !normalized_result.is_null() {
                    rpl_free(result as *mut libc::c_void);
                    return normalized_result;
                }
            } else {
                if length == 0 as libc::c_int as libc::c_ulong {
                    if result.is_null() {
                        result = malloc(1 as libc::c_int as libc::c_ulong)
                            as *mut uint8_t;
                        if result.is_null() {
                            *__errno_location() = 12 as libc::c_int;
                            current_block = 11264440884544272135;
                        } else {
                            current_block = 11235674318412060590;
                        }
                    } else {
                        current_block = 11235674318412060590;
                    }
                } else {
                    if result != resultbuf && length < allocated {
                        let mut memory: *mut uint8_t = 0 as *mut uint8_t;
                        memory = realloc(
                            result as *mut libc::c_void,
                            length
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint8_t;
                        if !memory.is_null() {
                            result = memory;
                        }
                    }
                    current_block = 11235674318412060590;
                }
                match current_block {
                    11264440884544272135 => {}
                    _ => {
                        *lengthp = length;
                        return result;
                    }
                }
            }
        }
        _ => {}
    }
    if result != resultbuf {
        let mut saved_errno: libc::c_int = *__errno_location();
        rpl_free(result as *mut libc::c_void);
        *__errno_location() = saved_errno;
    }
    return 0 as *mut uint8_t;
}

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn __errno_location() -> *mut libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub align: libc::c_uint,
    pub buf: [libc::c_char; 4096],
}
pub unsafe extern "C" fn mem_cd_iconv(
    mut src: *const libc::c_char,
    mut srclen: size_t,
    mut cd: iconv_t,
    mut resultp: *mut *mut libc::c_char,
    mut lengthp: *mut size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut length: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    iconv(
        cd,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
    );
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut tmp: C2RustUnnamed = C2RustUnnamed { align: 0 };
    let mut inptr: *const libc::c_char = src;
    let mut insize: size_t = srclen;
    while insize > 0 as libc::c_int as libc::c_ulong {
        let mut outptr: *mut libc::c_char = (tmp.buf).as_mut_ptr();
        let mut outsize: size_t = 4096 as libc::c_int as size_t;
        let mut res: size_t = iconv(
            cd,
            &mut inptr as *mut *const libc::c_char as *mut *mut libc::c_char,
            &mut insize,
            &mut outptr,
            &mut outsize,
        );
        if res == -(1 as libc::c_int) as size_t {
            if !(*__errno_location() == 7 as libc::c_int) {
                if *__errno_location() == 22 as libc::c_int {
                    break;
                }
                return -(1 as libc::c_int);
            }
        }
        count = (count as libc::c_ulong)
            .wrapping_add(
                outptr.offset_from((tmp.buf).as_mut_ptr()) as libc::c_long
                    as libc::c_ulong,
            ) as size_t as size_t;
    }
    let mut outptr_0: *mut libc::c_char = (tmp.buf).as_mut_ptr();
    let mut outsize_0: size_t = 4096 as libc::c_int as size_t;
    let mut res_0: size_t = iconv(
        cd,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
        &mut outptr_0,
        &mut outsize_0,
    );
    if res_0 == -(1 as libc::c_int) as size_t {
        return -(1 as libc::c_int);
    }
    count = (count as libc::c_ulong)
        .wrapping_add(
            outptr_0.offset_from((tmp.buf).as_mut_ptr()) as libc::c_long as libc::c_ulong,
        ) as size_t as size_t;
    length = count;
    if length == 0 as libc::c_int as libc::c_ulong {
        *lengthp = 0 as libc::c_int as size_t;
        return 0 as libc::c_int;
    }
    if !(*resultp).is_null() && *lengthp >= length {
        result = *resultp;
    } else {
        result = malloc(length) as *mut libc::c_char;
        if result.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    iconv(
        cd,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
    );
    let mut inptr_0: *const libc::c_char = src;
    let mut insize_0: size_t = srclen;
    let mut outptr_1: *mut libc::c_char = result;
    let mut outsize_1: size_t = length;
    loop {
        if !(insize_0 > 0 as libc::c_int as libc::c_ulong) {
            current_block = 652864300344834934;
            break;
        }
        let mut res_1: size_t = iconv(
            cd,
            &mut inptr_0 as *mut *const libc::c_char as *mut *mut libc::c_char,
            &mut insize_0,
            &mut outptr_1,
            &mut outsize_1,
        );
        if !(res_1 == -(1 as libc::c_int) as size_t) {
            continue;
        }
        if *__errno_location() == 22 as libc::c_int {
            current_block = 652864300344834934;
            break;
        } else {
            current_block = 9118788152453554342;
            break;
        }
    }
    match current_block {
        652864300344834934 => {
            let mut res_2: size_t = iconv(
                cd,
                0 as *mut *mut libc::c_char,
                0 as *mut size_t,
                &mut outptr_1,
                &mut outsize_1,
            );
            if !(res_2 == -(1 as libc::c_int) as size_t) {
                if outsize_1 != 0 as libc::c_int as libc::c_ulong {
                    abort();
                }
                *resultp = result;
                *lengthp = length;
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    if result != *resultp {
        rpl_free(result as *mut libc::c_void);
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn str_cd_iconv(
    mut src: *const libc::c_char,
    mut cd: iconv_t,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_size: size_t = 0;
    let mut length: size_t = 0;
    let mut inptr: *const libc::c_char = src;
    let mut inbytes_remaining: size_t = strlen(src);
    result_size = inbytes_remaining;
    let mut approx_sqrt_SIZE_MAX: size_t = -(1 as libc::c_int) as size_t
        >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
    if result_size
        <= approx_sqrt_SIZE_MAX.wrapping_div(16 as libc::c_int as libc::c_ulong)
    {
        result_size = (result_size as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    result_size = (result_size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    result = malloc(result_size) as *mut libc::c_char;
    if result.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    iconv(
        cd,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
    );
    let mut outptr: *mut libc::c_char = result;
    let mut outbytes_remaining: size_t = result_size
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let mut res: size_t = iconv(
            cd,
            &mut inptr as *mut *const libc::c_char as *mut *mut libc::c_char,
            &mut inbytes_remaining,
            &mut outptr,
            &mut outbytes_remaining,
        );
        if !(res == -(1 as libc::c_int) as size_t) {
            current_block = 6669252993407410313;
            break;
        }
        if *__errno_location() == 22 as libc::c_int {
            current_block = 6669252993407410313;
            break;
        }
        if !(*__errno_location() == 7 as libc::c_int) {
            current_block = 8876238068154707511;
            break;
        }
        let mut used: size_t = outptr.offset_from(result) as libc::c_long as size_t;
        let mut newsize: size_t = result_size
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let mut newresult: *mut libc::c_char = 0 as *mut libc::c_char;
        if !(newsize > result_size) {
            *__errno_location() = 12 as libc::c_int;
            current_block = 8876238068154707511;
            break;
        } else {
            newresult = realloc(result as *mut libc::c_void, newsize)
                as *mut libc::c_char;
            if newresult.is_null() {
                *__errno_location() = 12 as libc::c_int;
                current_block = 8876238068154707511;
                break;
            } else {
                result = newresult;
                result_size = newsize;
                outptr = result.offset(used as isize);
                outbytes_remaining = result_size
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(used);
            }
        }
    }
    match current_block {
        6669252993407410313 => {
            loop {
                let mut res_0: size_t = iconv(
                    cd,
                    0 as *mut *mut libc::c_char,
                    0 as *mut size_t,
                    &mut outptr,
                    &mut outbytes_remaining,
                );
                if !(res_0 == -(1 as libc::c_int) as size_t) {
                    current_block = 6450597802325118133;
                    break;
                }
                if !(*__errno_location() == 7 as libc::c_int) {
                    current_block = 8876238068154707511;
                    break;
                }
                let mut used_0: size_t = outptr.offset_from(result) as libc::c_long
                    as size_t;
                let mut newsize_0: size_t = result_size
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                let mut newresult_0: *mut libc::c_char = 0 as *mut libc::c_char;
                if !(newsize_0 > result_size) {
                    *__errno_location() = 12 as libc::c_int;
                    current_block = 8876238068154707511;
                    break;
                } else {
                    newresult_0 = realloc(result as *mut libc::c_void, newsize_0)
                        as *mut libc::c_char;
                    if newresult_0.is_null() {
                        *__errno_location() = 12 as libc::c_int;
                        current_block = 8876238068154707511;
                        break;
                    } else {
                        result = newresult_0;
                        result_size = newsize_0;
                        outptr = result.offset(used_0 as isize);
                        outbytes_remaining = result_size
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(used_0);
                    }
                }
            }
            match current_block {
                8876238068154707511 => {}
                _ => {
                    let fresh0 = outptr;
                    outptr = outptr.offset(1);
                    *fresh0 = '\0' as i32 as libc::c_char;
                    length = outptr.offset_from(result) as libc::c_long as size_t;
                    if length < result_size {
                        let mut smaller_result: *mut libc::c_char = realloc(
                            result as *mut libc::c_void,
                            length,
                        ) as *mut libc::c_char;
                        if !smaller_result.is_null() {
                            result = smaller_result;
                        }
                    }
                    return result;
                }
            }
        }
        _ => {}
    }
    rpl_free(result as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn str_iconv(
    mut src: *const libc::c_char,
    mut from_codeset: *const libc::c_char,
    mut to_codeset: *const libc::c_char,
) -> *mut libc::c_char {
    if *src as libc::c_int == '\0' as i32
        || c_strcasecmp(from_codeset, to_codeset) == 0 as libc::c_int
    {
        let mut result: *mut libc::c_char = strdup(src);
        if result.is_null() {
            *__errno_location() = 12 as libc::c_int;
        }
        return result;
    } else {
        let mut cd: iconv_t = 0 as *mut libc::c_void;
        let mut result_0: *mut libc::c_char = 0 as *mut libc::c_char;
        cd = iconv_open(to_codeset, from_codeset);
        if cd == -(1 as libc::c_int) as iconv_t {
            return 0 as *mut libc::c_char;
        }
        result_0 = str_cd_iconv(src, cd);
        if result_0.is_null() {
            let mut saved_errno: libc::c_int = *__errno_location();
            iconv_close(cd);
            *__errno_location() = saved_errno;
        } else if iconv_close(cd) < 0 as libc::c_int {
            rpl_free(result_0 as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        return result_0;
    };
}

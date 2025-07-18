use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn log(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn ZopfliLengthLimitedCodeLengths(
        frequencies: *const size_t,
        n: libc::c_int,
        maxbits: libc::c_int,
        bitlengths: *mut libc::c_uint,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn ZopfliLengthsToSymbols(
    mut lengths: *const libc::c_uint,
    mut n: size_t,
    mut maxbits: libc::c_uint,
    mut symbols: *mut libc::c_uint,
) {
    let mut bl_count: *mut size_t = malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(
                maxbits.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ),
    ) as *mut size_t;
    let mut next_code: *mut size_t = malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(
                maxbits.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ),
    ) as *mut size_t;
    let mut bits: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        *symbols.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    bits = 0 as libc::c_int as libc::c_uint;
    while bits <= maxbits {
        *bl_count.offset(bits as isize) = 0 as libc::c_int as size_t;
        bits = bits.wrapping_add(1);
        bits;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        if *lengths.offset(i as isize) <= maxbits {} else {
            __assert_fail(
                b"lengths[i] <= maxbits\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/tree.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"void ZopfliLengthsToSymbols(const unsigned int *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_571: {
            if *lengths.offset(i as isize) <= maxbits {} else {
                __assert_fail(
                    b"lengths[i] <= maxbits\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/tree.c\0" as *const u8 as *const libc::c_char,
                    47 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 88],
                        &[libc::c_char; 88],
                    >(
                        b"void ZopfliLengthsToSymbols(const unsigned int *, size_t, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let ref mut fresh0 = *bl_count.offset(*lengths.offset(i as isize) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        i = i.wrapping_add(1);
        i;
    }
    code = 0 as libc::c_int as libc::c_uint;
    *bl_count.offset(0 as libc::c_int as isize) = 0 as libc::c_int as size_t;
    bits = 1 as libc::c_int as libc::c_uint;
    while bits <= maxbits {
        code = ((code as libc::c_ulong)
            .wrapping_add(
                *bl_count
                    .offset(bits.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize),
            ) << 1 as libc::c_int) as libc::c_uint;
        *next_code.offset(bits as isize) = code as size_t;
        bits = bits.wrapping_add(1);
        bits;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        let mut len: libc::c_uint = *lengths.offset(i as isize);
        if len != 0 as libc::c_int as libc::c_uint {
            *symbols
                .offset(i as isize) = *next_code.offset(len as isize) as libc::c_uint;
            let ref mut fresh1 = *next_code.offset(len as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
            *fresh1;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(bl_count as *mut libc::c_void);
    free(next_code as *mut libc::c_void);
}
pub unsafe extern "C" fn ZopfliCalculateEntropy(
    mut count: *const size_t,
    mut n: size_t,
    mut bitlengths: *mut libc::c_double,
) {
    static mut kInvLog2: libc::c_double = 1.4426950408889f64;
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut log2sum: libc::c_double = 0.;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        sum = (sum as libc::c_ulong).wrapping_add(*count.offset(i as isize))
            as libc::c_uint as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    log2sum = (if sum == 0 as libc::c_int as libc::c_uint {
        log(n as libc::c_double)
    } else {
        log(sum as libc::c_double)
    }) * kInvLog2;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        if *count.offset(i as isize) == 0 as libc::c_int as libc::c_ulong {
            *bitlengths.offset(i as isize) = log2sum;
        } else {
            *bitlengths
                .offset(
                    i as isize,
                ) = log2sum
                - log(*count.offset(i as isize) as libc::c_double) * kInvLog2;
        }
        if *bitlengths.offset(i as isize) < 0 as libc::c_int as libc::c_double
            && *bitlengths.offset(i as isize) > -1e-5f64
        {
            *bitlengths.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        }
        if *bitlengths.offset(i as isize) >= 0 as libc::c_int as libc::c_double {} else {
            __assert_fail(
                b"bitlengths[i] >= 0\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/tree.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"void ZopfliCalculateEntropy(const size_t *, size_t, double *)\0"))
                    .as_ptr(),
            );
        }
        'c_696: {
            if *bitlengths.offset(i as isize) >= 0 as libc::c_int as libc::c_double
            {} else {
                __assert_fail(
                    b"bitlengths[i] >= 0\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/tree.c\0" as *const u8 as *const libc::c_char,
                    92 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"void ZopfliCalculateEntropy(const size_t *, size_t, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn ZopfliCalculateBitLengths(
    mut count: *const size_t,
    mut n: size_t,
    mut maxbits: libc::c_int,
    mut bitlengths: *mut libc::c_uint,
) {
    let mut error: libc::c_int = ZopfliLengthLimitedCodeLengths(
        count,
        n as libc::c_int,
        maxbits,
        bitlengths,
    );
    if error == 0 {} else {
        __assert_fail(
            b"!error\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/tree.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void ZopfliCalculateBitLengths(const size_t *, size_t, int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_310: {
        if error == 0 {} else {
            __assert_fail(
                b"!error\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/tree.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void ZopfliCalculateBitLengths(const size_t *, size_t, int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}

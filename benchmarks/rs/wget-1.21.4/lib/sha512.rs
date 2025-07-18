use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type u64_0 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [u64_0; 8],
    pub total: [u64_0; 2],
    pub buflen: size_t,
    pub buffer: [u64_0; 32],
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong)
        >> 56 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
            >> 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
            >> 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
            >> 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
            << 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
            << 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56 as libc::c_int)
        as __uint64_t;
}
static mut fillbuf: [libc::c_uchar; 128] = [
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn sha512_init_ctx(mut ctx: *mut sha512_ctx) {
    (*ctx)
        .state[0 as libc::c_int
        as usize] = ((0x6a09e667 as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xf3bcc908 as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[1 as libc::c_int
        as usize] = ((0xbb67ae85 as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x84caa73b as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[2 as libc::c_int
        as usize] = ((0x3c6ef372 as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xfe94f82b as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[3 as libc::c_int
        as usize] = ((0xa54ff53a as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x5f1d36f1 as libc::c_int as libc::c_ulong);
    (*ctx)
        .state[4 as libc::c_int
        as usize] = ((0x510e527f as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xade682d1 as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[5 as libc::c_int
        as usize] = ((0x9b05688c as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x2b3e6c1f as libc::c_int as libc::c_ulong);
    (*ctx)
        .state[6 as libc::c_int
        as usize] = ((0x1f83d9ab as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xfb41bd6b as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[7 as libc::c_int
        as usize] = ((0x5be0cd19 as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x137e2179 as libc::c_int as libc::c_ulong);
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as u64_0;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn sha384_init_ctx(mut ctx: *mut sha512_ctx) {
    (*ctx)
        .state[0 as libc::c_int
        as usize] = ((0xcbbb9d5d as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xc1059ed8 as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[1 as libc::c_int
        as usize] = ((0x629a292a as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x367cd507 as libc::c_int as libc::c_ulong);
    (*ctx)
        .state[2 as libc::c_int
        as usize] = ((0x9159015a as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x3070dd17 as libc::c_int as libc::c_ulong);
    (*ctx)
        .state[3 as libc::c_int
        as usize] = ((0x152fecd8 as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xf70e5939 as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[4 as libc::c_int
        as usize] = ((0x67332667 as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xffc00b31 as libc::c_uint as libc::c_ulong);
    (*ctx)
        .state[5 as libc::c_int
        as usize] = ((0x8eb44a87 as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x68581511 as libc::c_int as libc::c_ulong);
    (*ctx)
        .state[6 as libc::c_int
        as usize] = ((0xdb0c2e0d as libc::c_uint as u64_0) << 32 as libc::c_int)
        .wrapping_add(0x64f98fa7 as libc::c_int as libc::c_ulong);
    (*ctx)
        .state[7 as libc::c_int
        as usize] = ((0x47b5481d as libc::c_int as u64_0) << 32 as libc::c_int)
        .wrapping_add(0xbefa4fa4 as libc::c_uint as libc::c_ulong);
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as u64_0;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn set_uint64(mut cp: *mut libc::c_char, mut v: u64_0) {
    memcpy(
        cp as *mut libc::c_void,
        &mut v as *mut u64_0 as *const libc::c_void,
        ::std::mem::size_of::<u64_0>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sha512_read_ctx(
    mut ctx: *const sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_char = resbuf as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        set_uint64(
            r
                .offset(
                    (i as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<u64_0>() as libc::c_ulong)
                        as isize,
                ),
            __bswap_64((*ctx).state[i as usize]),
        );
        i += 1;
        i;
    }
    return resbuf;
}
pub unsafe extern "C" fn sha384_read_ctx(
    mut ctx: *const sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_char = resbuf as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        set_uint64(
            r
                .offset(
                    (i as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<u64_0>() as libc::c_ulong)
                        as isize,
                ),
            __bswap_64((*ctx).state[i as usize]),
        );
        i += 1;
        i;
    }
    return resbuf;
}
unsafe extern "C" fn sha512_conclude_ctx(mut ctx: *mut sha512_ctx) {
    let mut bytes: size_t = (*ctx).buflen;
    let mut size: size_t = (if bytes < 112 as libc::c_int as libc::c_ulong {
        128 as libc::c_int / 8 as libc::c_int
    } else {
        128 as libc::c_int * 2 as libc::c_int / 8 as libc::c_int
    }) as size_t;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize]).wrapping_add(bytes);
    if (*ctx).total[0 as libc::c_int as usize] < bytes {
        (*ctx)
            .total[1 as libc::c_int
            as usize] = ((*ctx).total[1 as libc::c_int as usize])
            .wrapping_add(1 as libc::c_int as u64_0);
    }
    set_uint64(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as *mut u64_0 as *mut libc::c_char,
        __bswap_64(
            (*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
                | (*ctx).total[0 as libc::c_int as usize] >> 61 as libc::c_int,
        ),
    );
    set_uint64(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as *mut u64_0 as *mut libc::c_char,
        __bswap_64((*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int),
    );
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char).offset(bytes as isize)
            as *mut libc::c_char as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(bytes),
    );
    sha512_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ctx,
    );
}
pub unsafe extern "C" fn sha512_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sha512_conclude_ctx(ctx);
    return sha512_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn sha384_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sha512_conclude_ctx(ctx);
    return sha384_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn sha512_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha512_init_ctx(&mut ctx);
    sha512_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha512_finish_ctx(&mut ctx, resblock);
}
pub unsafe extern "C" fn sha384_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha384_init_ctx(&mut ctx);
    sha512_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha384_finish_ctx(&mut ctx, resblock);
}
pub unsafe extern "C" fn sha512_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    if (*ctx).buflen != 0 as libc::c_int as libc::c_ulong {
        let mut left_over: size_t = (*ctx).buflen;
        let mut add: size_t = if (256 as libc::c_int as libc::c_ulong)
            .wrapping_sub(left_over) > len
        {
            len
        } else {
            (256 as libc::c_int as libc::c_ulong).wrapping_sub(left_over)
        };
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            add,
        );
        (*ctx)
            .buflen = ((*ctx).buflen as libc::c_ulong).wrapping_add(add) as size_t
            as size_t;
        if (*ctx).buflen > 128 as libc::c_int as libc::c_ulong {
            sha512_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                (*ctx).buflen & !(127 as libc::c_int) as libc::c_ulong,
                ctx,
            );
            (*ctx).buflen &= 127 as libc::c_int as libc::c_ulong;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                    .offset(
                        (left_over.wrapping_add(add)
                            & !(127 as libc::c_int) as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (*ctx).buflen,
            );
        }
        buffer = (buffer as *const libc::c_char).offset(add as isize)
            as *const libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(add) as size_t as size_t;
    }
    if len >= 128 as libc::c_int as libc::c_ulong {
        if (buffer as uintptr_t)
            .wrapping_rem(::std::mem::align_of::<u64_0>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            while len > 128 as libc::c_int as libc::c_ulong {
                sha512_process_block(
                    memcpy(
                        ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                        buffer,
                        128 as libc::c_int as libc::c_ulong,
                    ),
                    128 as libc::c_int as size_t,
                    ctx,
                );
                buffer = (buffer as *const libc::c_char)
                    .offset(128 as libc::c_int as isize) as *const libc::c_void;
                len = (len as libc::c_ulong)
                    .wrapping_sub(128 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
        } else {
            sha512_process_block(
                buffer,
                len & !(127 as libc::c_int) as libc::c_ulong,
                ctx,
            );
            buffer = (buffer as *const libc::c_char)
                .offset((len & !(127 as libc::c_int) as libc::c_ulong) as isize)
                as *const libc::c_void;
            len &= 127 as libc::c_int as libc::c_ulong;
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        let mut left_over_0: size_t = (*ctx).buflen;
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over_0 as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            len,
        );
        left_over_0 = (left_over_0 as libc::c_ulong).wrapping_add(len) as size_t
            as size_t;
        if left_over_0 >= 128 as libc::c_int as libc::c_ulong {
            sha512_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                128 as libc::c_int as size_t,
                ctx,
            );
            left_over_0 = (left_over_0 as libc::c_ulong)
                .wrapping_sub(128 as libc::c_int as libc::c_ulong) as size_t as size_t;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer).as_mut_ptr().offset(16 as libc::c_int as isize)
                    as *mut u64_0 as *const libc::c_void,
                left_over_0,
            );
        }
        (*ctx).buflen = left_over_0;
    }
}
static mut sha512_round_constants: [u64_0; 80] = [0; 80];
pub unsafe extern "C" fn sha512_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    let mut words: *const u64_0 = buffer as *const u64_0;
    let mut endp: *const u64_0 = words
        .offset(
            len.wrapping_div(::std::mem::size_of::<u64_0>() as libc::c_ulong) as isize,
        );
    let mut x: [u64_0; 16] = [0; 16];
    let mut a: u64_0 = (*ctx).state[0 as libc::c_int as usize];
    let mut b: u64_0 = (*ctx).state[1 as libc::c_int as usize];
    let mut c: u64_0 = (*ctx).state[2 as libc::c_int as usize];
    let mut d: u64_0 = (*ctx).state[3 as libc::c_int as usize];
    let mut e: u64_0 = (*ctx).state[4 as libc::c_int as usize];
    let mut f: u64_0 = (*ctx).state[5 as libc::c_int as usize];
    let mut g: u64_0 = (*ctx).state[6 as libc::c_int as usize];
    let mut h: u64_0 = (*ctx).state[7 as libc::c_int as usize];
    let mut lolen: u64_0 = len;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize]).wrapping_add(lolen);
    (*ctx)
        .total[1 as libc::c_int
        as usize] = ((*ctx).total[1 as libc::c_int as usize])
        .wrapping_add(
            (len >> 31 as libc::c_int >> 31 as libc::c_int >> 2 as libc::c_int)
                .wrapping_add(
                    ((*ctx).total[0 as libc::c_int as usize] < lolen) as libc::c_int
                        as u64_0,
                ),
        );
    while words < endp {
        let mut t: libc::c_int = 0;
        t = 0 as libc::c_int;
        while t < 16 as libc::c_int {
            x[t as usize] = __bswap_64(*words);
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        let mut t0: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        let mut t1: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[0 as libc::c_int as usize])
                                    .wrapping_add(x[0 as libc::c_int as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1);
        h = t0.wrapping_add(t1);
        let mut t0_0: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        let mut t1_0: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[1 as libc::c_int as usize])
                                    .wrapping_add(x[1 as libc::c_int as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_0);
        g = t0_0.wrapping_add(t1_0);
        let mut t0_1: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        let mut t1_1: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[2 as libc::c_int as usize])
                                    .wrapping_add(x[2 as libc::c_int as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_1);
        f = t0_1.wrapping_add(t1_1);
        let mut t0_2: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        let mut t1_2: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[3 as libc::c_int as usize])
                                    .wrapping_add(x[3 as libc::c_int as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_2);
        e = t0_2.wrapping_add(t1_2);
        let mut t0_3: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        let mut t1_3: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[4 as libc::c_int as usize])
                                    .wrapping_add(x[4 as libc::c_int as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_3);
        d = t0_3.wrapping_add(t1_3);
        let mut t0_4: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        let mut t1_4: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[5 as libc::c_int as usize])
                                    .wrapping_add(x[5 as libc::c_int as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_4);
        c = t0_4.wrapping_add(t1_4);
        let mut t0_5: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        let mut t1_5: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[6 as libc::c_int as usize])
                                    .wrapping_add(x[6 as libc::c_int as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_5);
        b = t0_5.wrapping_add(t1_5);
        let mut t0_6: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        let mut t1_6: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[7 as libc::c_int as usize])
                                    .wrapping_add(x[7 as libc::c_int as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_6);
        a = t0_6.wrapping_add(t1_6);
        let mut t0_7: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        let mut t1_7: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[8 as libc::c_int as usize])
                                    .wrapping_add(x[8 as libc::c_int as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_7);
        h = t0_7.wrapping_add(t1_7);
        let mut t0_8: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        let mut t1_8: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[9 as libc::c_int as usize])
                                    .wrapping_add(x[9 as libc::c_int as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_8);
        g = t0_8.wrapping_add(t1_8);
        let mut t0_9: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        let mut t1_9: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[10 as libc::c_int as usize])
                                    .wrapping_add(x[10 as libc::c_int as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_9);
        f = t0_9.wrapping_add(t1_9);
        let mut t0_10: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        let mut t1_10: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[11 as libc::c_int as usize])
                                    .wrapping_add(x[11 as libc::c_int as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_10);
        e = t0_10.wrapping_add(t1_10);
        let mut t0_11: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        let mut t1_11: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[12 as libc::c_int as usize])
                                    .wrapping_add(x[12 as libc::c_int as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_11);
        d = t0_11.wrapping_add(t1_11);
        let mut t0_12: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        let mut t1_12: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[13 as libc::c_int as usize])
                                    .wrapping_add(x[13 as libc::c_int as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_12);
        c = t0_12.wrapping_add(t1_12);
        let mut t0_13: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        let mut t1_13: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[14 as libc::c_int as usize])
                                    .wrapping_add(x[14 as libc::c_int as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_13);
        b = t0_13.wrapping_add(t1_13);
        let mut t0_14: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        let mut t1_14: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[15 as libc::c_int as usize])
                                    .wrapping_add(x[15 as libc::c_int as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_14);
        a = t0_14.wrapping_add(t1_14);
        let mut t0_15: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(16 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(16 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(16 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(16 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(16 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(16 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(16 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(16 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(16 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(16 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(16 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(16 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(16 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_15: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[16 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(16 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_15);
        h = t0_15.wrapping_add(t1_15);
        let mut t0_16: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(17 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(17 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(17 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(17 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(17 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(17 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(17 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(17 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(17 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(17 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(17 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(17 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(17 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_16: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[17 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(17 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_16);
        g = t0_16.wrapping_add(t1_16);
        let mut t0_17: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(18 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(18 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(18 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(18 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(18 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(18 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(18 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(18 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(18 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(18 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(18 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(18 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(18 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_17: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[18 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(18 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_17);
        f = t0_17.wrapping_add(t1_17);
        let mut t0_18: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(19 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(19 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(19 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(19 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(19 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(19 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(19 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(19 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(19 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(19 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(19 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(19 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(19 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_18: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[19 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(19 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_18);
        e = t0_18.wrapping_add(t1_18);
        let mut t0_19: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(20 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(20 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(20 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(20 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(20 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(20 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(20 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(20 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(20 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(20 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(20 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(20 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(20 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_19: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[20 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(20 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_19);
        d = t0_19.wrapping_add(t1_19);
        let mut t0_20: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(21 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(21 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(21 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(21 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(21 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(21 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(21 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(21 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(21 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(21 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(21 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(21 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(21 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_20: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[21 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(21 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_20);
        c = t0_20.wrapping_add(t1_20);
        let mut t0_21: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(22 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(22 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(22 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(22 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(22 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(22 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(22 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(22 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(22 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(22 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(22 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(22 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(22 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_21: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[22 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(22 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_21);
        b = t0_21.wrapping_add(t1_21);
        let mut t0_22: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(23 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(23 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(23 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(23 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(23 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(23 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(23 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(23 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(23 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(23 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(23 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(23 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(23 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_22: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[23 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(23 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_22);
        a = t0_22.wrapping_add(t1_22);
        let mut t0_23: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(24 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(24 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(24 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(24 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(24 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(24 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(24 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(24 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(24 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(24 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(24 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(24 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(24 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_23: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[24 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(24 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_23);
        h = t0_23.wrapping_add(t1_23);
        let mut t0_24: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(25 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(25 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(25 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(25 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(25 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(25 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(25 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(25 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(25 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(25 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(25 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(25 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(25 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_24: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[25 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(25 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_24);
        g = t0_24.wrapping_add(t1_24);
        let mut t0_25: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(26 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(26 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(26 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(26 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(26 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(26 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(26 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(26 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(26 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(26 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(26 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(26 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(26 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_25: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[26 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(26 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_25);
        f = t0_25.wrapping_add(t1_25);
        let mut t0_26: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(27 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(27 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(27 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(27 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(27 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(27 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(27 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(27 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(27 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(27 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(27 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(27 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(27 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_26: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[27 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(27 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_26);
        e = t0_26.wrapping_add(t1_26);
        let mut t0_27: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(28 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(28 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(28 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(28 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(28 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(28 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(28 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(28 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(28 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(28 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(28 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(28 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(28 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_27: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[28 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(28 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_27);
        d = t0_27.wrapping_add(t1_27);
        let mut t0_28: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(29 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(29 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(29 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(29 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(29 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(29 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(29 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(29 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(29 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(29 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(29 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(29 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(29 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_28: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[29 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(29 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_28);
        c = t0_28.wrapping_add(t1_28);
        let mut t0_29: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(30 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(30 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(30 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(30 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(30 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(30 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(30 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(30 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(30 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(30 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(30 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(30 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(30 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_29: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[30 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(30 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_29);
        b = t0_29.wrapping_add(t1_29);
        let mut t0_30: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(31 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(31 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(31 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(31 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(31 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(31 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(31 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(31 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(31 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(31 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(31 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(31 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(31 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_30: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[31 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(31 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_30);
        a = t0_30.wrapping_add(t1_30);
        let mut t0_31: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(32 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(32 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(32 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(32 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(32 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(32 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(32 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(32 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(32 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(32 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(32 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(32 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(32 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_31: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[32 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(32 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_31);
        h = t0_31.wrapping_add(t1_31);
        let mut t0_32: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(33 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(33 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(33 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(33 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(33 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(33 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(33 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(33 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(33 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(33 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(33 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(33 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(33 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_32: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[33 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(33 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_32);
        g = t0_32.wrapping_add(t1_32);
        let mut t0_33: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(34 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(34 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(34 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(34 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(34 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(34 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(34 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(34 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(34 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(34 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(34 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(34 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(34 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_33: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[34 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(34 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_33);
        f = t0_33.wrapping_add(t1_33);
        let mut t0_34: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(35 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(35 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(35 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(35 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(35 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(35 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(35 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(35 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(35 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(35 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(35 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(35 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(35 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_34: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[35 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(35 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_34);
        e = t0_34.wrapping_add(t1_34);
        let mut t0_35: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(36 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(36 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(36 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(36 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(36 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(36 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(36 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(36 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(36 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(36 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(36 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(36 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(36 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_35: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[36 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(36 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_35);
        d = t0_35.wrapping_add(t1_35);
        let mut t0_36: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(37 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(37 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(37 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(37 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(37 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(37 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(37 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(37 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(37 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(37 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(37 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(37 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(37 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_36: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[37 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(37 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_36);
        c = t0_36.wrapping_add(t1_36);
        let mut t0_37: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(38 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(38 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(38 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(38 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(38 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(38 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(38 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(38 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(38 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(38 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(38 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(38 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(38 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_37: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[38 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(38 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_37);
        b = t0_37.wrapping_add(t1_37);
        let mut t0_38: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(39 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(39 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(39 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(39 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(39 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(39 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(39 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(39 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(39 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(39 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(39 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(39 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(39 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_38: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[39 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(39 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_38);
        a = t0_38.wrapping_add(t1_38);
        let mut t0_39: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(40 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(40 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(40 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(40 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(40 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(40 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(40 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(40 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(40 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(40 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(40 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(40 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(40 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_39: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[40 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(40 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_39);
        h = t0_39.wrapping_add(t1_39);
        let mut t0_40: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(41 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(41 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(41 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(41 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(41 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(41 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(41 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(41 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(41 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(41 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(41 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(41 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(41 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_40: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[41 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(41 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_40);
        g = t0_40.wrapping_add(t1_40);
        let mut t0_41: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(42 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(42 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(42 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(42 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(42 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(42 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(42 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(42 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(42 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(42 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(42 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(42 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(42 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_41: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[42 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(42 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_41);
        f = t0_41.wrapping_add(t1_41);
        let mut t0_42: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(43 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(43 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(43 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(43 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(43 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(43 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(43 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(43 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(43 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(43 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(43 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(43 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(43 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_42: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[43 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(43 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_42);
        e = t0_42.wrapping_add(t1_42);
        let mut t0_43: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(44 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(44 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(44 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(44 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(44 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(44 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(44 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(44 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(44 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(44 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(44 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(44 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(44 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_43: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[44 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(44 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_43);
        d = t0_43.wrapping_add(t1_43);
        let mut t0_44: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(45 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(45 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(45 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(45 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(45 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(45 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(45 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(45 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(45 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(45 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(45 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(45 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(45 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_44: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[45 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(45 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_44);
        c = t0_44.wrapping_add(t1_44);
        let mut t0_45: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(46 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(46 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(46 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(46 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(46 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(46 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(46 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(46 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(46 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(46 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(46 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(46 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(46 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_45: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[46 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(46 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_45);
        b = t0_45.wrapping_add(t1_45);
        let mut t0_46: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(47 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(47 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(47 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(47 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(47 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(47 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(47 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(47 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(47 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(47 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(47 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(47 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(47 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_46: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[47 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(47 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_46);
        a = t0_46.wrapping_add(t1_46);
        let mut t0_47: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(48 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(48 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(48 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(48 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(48 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(48 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(48 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(48 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(48 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(48 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(48 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(48 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(48 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_47: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[48 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(48 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_47);
        h = t0_47.wrapping_add(t1_47);
        let mut t0_48: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(49 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(49 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(49 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(49 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(49 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(49 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(49 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(49 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(49 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(49 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(49 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(49 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(49 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_48: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[49 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(49 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_48);
        g = t0_48.wrapping_add(t1_48);
        let mut t0_49: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(50 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(50 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(50 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(50 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(50 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(50 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(50 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(50 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(50 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(50 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(50 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(50 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(50 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_49: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[50 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(50 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_49);
        f = t0_49.wrapping_add(t1_49);
        let mut t0_50: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(51 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(51 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(51 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(51 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(51 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(51 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(51 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(51 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(51 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(51 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(51 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(51 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(51 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_50: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[51 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(51 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_50);
        e = t0_50.wrapping_add(t1_50);
        let mut t0_51: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(52 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(52 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(52 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(52 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(52 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(52 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(52 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(52 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(52 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(52 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(52 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(52 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(52 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_51: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[52 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(52 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_51);
        d = t0_51.wrapping_add(t1_51);
        let mut t0_52: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(53 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(53 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(53 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(53 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(53 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(53 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(53 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(53 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(53 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(53 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(53 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(53 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(53 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_52: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[53 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(53 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_52);
        c = t0_52.wrapping_add(t1_52);
        let mut t0_53: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(54 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(54 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(54 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(54 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(54 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(54 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(54 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(54 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(54 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(54 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(54 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(54 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(54 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_53: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[54 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(54 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_53);
        b = t0_53.wrapping_add(t1_53);
        let mut t0_54: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(55 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(55 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(55 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(55 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(55 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(55 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(55 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(55 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(55 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(55 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(55 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(55 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(55 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_54: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[55 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(55 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_54);
        a = t0_54.wrapping_add(t1_54);
        let mut t0_55: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(56 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(56 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(56 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(56 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(56 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(56 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(56 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(56 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(56 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(56 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(56 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(56 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(56 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_55: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[56 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(56 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_55);
        h = t0_55.wrapping_add(t1_55);
        let mut t0_56: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(57 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(57 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(57 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(57 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(57 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(57 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(57 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(57 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(57 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(57 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(57 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(57 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(57 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_56: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[57 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(57 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_56);
        g = t0_56.wrapping_add(t1_56);
        let mut t0_57: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(58 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(58 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(58 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(58 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(58 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(58 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(58 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(58 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(58 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(58 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(58 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(58 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(58 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_57: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[58 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(58 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_57);
        f = t0_57.wrapping_add(t1_57);
        let mut t0_58: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(59 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(59 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(59 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(59 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(59 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(59 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(59 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(59 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(59 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(59 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(59 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(59 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(59 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_58: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[59 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(59 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_58);
        e = t0_58.wrapping_add(t1_58);
        let mut t0_59: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(60 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(60 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(60 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(60 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(60 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(60 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(60 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(60 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(60 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(60 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(60 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(60 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(60 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_59: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[60 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(60 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_59);
        d = t0_59.wrapping_add(t1_59);
        let mut t0_60: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(61 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(61 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(61 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(61 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(61 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(61 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(61 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(61 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(61 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(61 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(61 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(61 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(61 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_60: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[61 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(61 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_60);
        c = t0_60.wrapping_add(t1_60);
        let mut t0_61: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(62 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(62 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(62 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(62 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(62 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(62 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(62 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(62 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(62 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(62 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(62 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(62 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(62 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_61: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[62 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(62 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_61);
        b = t0_61.wrapping_add(t1_61);
        let mut t0_62: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(63 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(63 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(63 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(63 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(63 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(63 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(63 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(63 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(63 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(63 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(63 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(63 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(63 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_62: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[63 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(63 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_62);
        a = t0_62.wrapping_add(t1_62);
        let mut t0_63: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(64 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(64 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(64 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(64 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(64 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(64 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(64 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(64 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(64 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(64 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(64 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(64 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(64 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_63: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[64 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(64 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_63);
        h = t0_63.wrapping_add(t1_63);
        let mut t0_64: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(65 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(65 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(65 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(65 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(65 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(65 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(65 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(65 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(65 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(65 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(65 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(65 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(65 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_64: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[65 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(65 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_64);
        g = t0_64.wrapping_add(t1_64);
        let mut t0_65: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(66 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(66 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(66 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(66 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(66 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(66 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(66 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(66 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(66 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(66 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(66 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(66 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(66 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_65: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[66 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(66 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_65);
        f = t0_65.wrapping_add(t1_65);
        let mut t0_66: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(67 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(67 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(67 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(67 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(67 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(67 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(67 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(67 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(67 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(67 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(67 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(67 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(67 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_66: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[67 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(67 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_66);
        e = t0_66.wrapping_add(t1_66);
        let mut t0_67: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(68 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(68 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(68 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(68 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(68 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(68 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(68 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(68 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(68 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(68 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(68 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(68 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(68 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_67: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[68 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(68 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_67);
        d = t0_67.wrapping_add(t1_67);
        let mut t0_68: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(69 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(69 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(69 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(69 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(69 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(69 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(69 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(69 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(69 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(69 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(69 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(69 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(69 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_68: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[69 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(69 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_68);
        c = t0_68.wrapping_add(t1_68);
        let mut t0_69: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(70 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(70 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(70 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(70 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(70 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(70 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(70 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(70 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(70 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(70 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(70 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(70 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(70 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_69: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[70 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(70 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_69);
        b = t0_69.wrapping_add(t1_69);
        let mut t0_70: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(71 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(71 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(71 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(71 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(71 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(71 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(71 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(71 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(71 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(71 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(71 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(71 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(71 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_70: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[71 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(71 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_70);
        a = t0_70.wrapping_add(t1_70);
        let mut t0_71: u64_0 = ((a << 36 as libc::c_int
            | a >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((a << 30 as libc::c_int | a >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (a << 25 as libc::c_int | a >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(a & b | c & (a | b));
        x[(72 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(72 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(72 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(72 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(72 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(72 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(72 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(72 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(72 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(72 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(72 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(72 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(72 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_71: u64_0 = h
            .wrapping_add(
                ((e << 50 as libc::c_int | e >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((e << 46 as libc::c_int
                        | e >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (e << 23 as libc::c_int
                            | e >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[72 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(72 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_71);
        h = t0_71.wrapping_add(t1_71);
        let mut t0_72: u64_0 = ((h << 36 as libc::c_int
            | h >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((h << 30 as libc::c_int | h >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (h << 25 as libc::c_int | h >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(h & a | b & (h | a));
        x[(73 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(73 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(73 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(73 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(73 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(73 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(73 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(73 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(73 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(73 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(73 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(73 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(73 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_72: u64_0 = g
            .wrapping_add(
                ((d << 50 as libc::c_int | d >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((d << 46 as libc::c_int
                        | d >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (d << 23 as libc::c_int
                            | d >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[73 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(73 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_72);
        g = t0_72.wrapping_add(t1_72);
        let mut t0_73: u64_0 = ((g << 36 as libc::c_int
            | g >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((g << 30 as libc::c_int | g >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (g << 25 as libc::c_int | g >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(g & h | a & (g | h));
        x[(74 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(74 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(74 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(74 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(74 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(74 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(74 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(74 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(74 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(74 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(74 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(74 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(74 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_73: u64_0 = f
            .wrapping_add(
                ((c << 50 as libc::c_int | c >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((c << 46 as libc::c_int
                        | c >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (c << 23 as libc::c_int
                            | c >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[74 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(74 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_73);
        f = t0_73.wrapping_add(t1_73);
        let mut t0_74: u64_0 = ((f << 36 as libc::c_int
            | f >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((f << 30 as libc::c_int | f >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (f << 25 as libc::c_int | f >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(f & g | h & (f | g));
        x[(75 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(75 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(75 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(75 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(75 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(75 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(75 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(75 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(75 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(75 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(75 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(75 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(75 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_74: u64_0 = e
            .wrapping_add(
                ((b << 50 as libc::c_int | b >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((b << 46 as libc::c_int
                        | b >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (b << 23 as libc::c_int
                            | b >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[75 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(75 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_74);
        e = t0_74.wrapping_add(t1_74);
        let mut t0_75: u64_0 = ((e << 36 as libc::c_int
            | e >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((e << 30 as libc::c_int | e >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (e << 25 as libc::c_int | e >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(e & f | g & (e | f));
        x[(76 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(76 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(76 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(76 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(76 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(76 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(76 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(76 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(76 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(76 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(76 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(76 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(76 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_75: u64_0 = d
            .wrapping_add(
                ((a << 50 as libc::c_int | a >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((a << 46 as libc::c_int
                        | a >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (a << 23 as libc::c_int
                            | a >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[76 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(76 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_75);
        d = t0_75.wrapping_add(t1_75);
        let mut t0_76: u64_0 = ((d << 36 as libc::c_int
            | d >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((d << 30 as libc::c_int | d >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (d << 25 as libc::c_int | d >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(d & e | f & (d | e));
        x[(77 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(77 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(77 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(77 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(77 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(77 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(77 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(77 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(77 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(77 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(77 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(77 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(77 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_76: u64_0 = c
            .wrapping_add(
                ((h << 50 as libc::c_int | h >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((h << 46 as libc::c_int
                        | h >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (h << 23 as libc::c_int
                            | h >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[77 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(77 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_76);
        c = t0_76.wrapping_add(t1_76);
        let mut t0_77: u64_0 = ((c << 36 as libc::c_int
            | c >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((c << 30 as libc::c_int | c >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (c << 25 as libc::c_int | c >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(c & d | e & (c | d));
        x[(78 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(78 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(78 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(78 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(78 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(78 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(78 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(78 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(78 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(78 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(78 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(78 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(78 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_77: u64_0 = b
            .wrapping_add(
                ((g << 50 as libc::c_int | g >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((g << 46 as libc::c_int
                        | g >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (g << 23 as libc::c_int
                            | g >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[78 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(78 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_77);
        b = t0_77.wrapping_add(t1_77);
        let mut t0_78: u64_0 = ((b << 36 as libc::c_int
            | b >> 64 as libc::c_int - 36 as libc::c_int)
            ^ ((b << 30 as libc::c_int | b >> 64 as libc::c_int - 30 as libc::c_int)
                ^ (b << 25 as libc::c_int | b >> 64 as libc::c_int - 25 as libc::c_int)))
            .wrapping_add(b & c | d & (b | c));
        x[(79 as libc::c_int & 15 as libc::c_int)
            as usize] = (x[(79 as libc::c_int & 15 as libc::c_int) as usize])
            .wrapping_add(
                ((x[(79 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int) as usize]
                    << 45 as libc::c_int
                    | x[(79 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] >> 64 as libc::c_int - 45 as libc::c_int)
                    ^ ((x[(79 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                        as usize] << 3 as libc::c_int
                        | x[(79 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 64 as libc::c_int - 3 as libc::c_int)
                        ^ x[(79 as libc::c_int - 2 as libc::c_int & 15 as libc::c_int)
                            as usize] >> 6 as libc::c_int))
                    .wrapping_add(
                        (x[(79 as libc::c_int - 7 as libc::c_int & 15 as libc::c_int)
                            as usize])
                            .wrapping_add(
                                (x[(79 as libc::c_int - 15 as libc::c_int
                                    & 15 as libc::c_int) as usize] << 63 as libc::c_int
                                    | x[(79 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize]
                                        >> 64 as libc::c_int - 63 as libc::c_int)
                                    ^ ((x[(79 as libc::c_int - 15 as libc::c_int
                                        & 15 as libc::c_int) as usize] << 56 as libc::c_int
                                        | x[(79 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize]
                                            >> 64 as libc::c_int - 56 as libc::c_int)
                                        ^ x[(79 as libc::c_int - 15 as libc::c_int
                                            & 15 as libc::c_int) as usize] >> 7 as libc::c_int),
                            ),
                    ),
            );
        let mut t1_78: u64_0 = a
            .wrapping_add(
                ((f << 50 as libc::c_int | f >> 64 as libc::c_int - 50 as libc::c_int)
                    ^ ((f << 46 as libc::c_int
                        | f >> 64 as libc::c_int - 46 as libc::c_int)
                        ^ (f << 23 as libc::c_int
                            | f >> 64 as libc::c_int - 23 as libc::c_int)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[79 as libc::c_int as usize])
                                    .wrapping_add(
                                        x[(79 as libc::c_int & 15 as libc::c_int) as usize],
                                    ),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_78);
        a = t0_78.wrapping_add(t1_78);
        (*ctx)
            .state[0 as libc::c_int
            as usize] = ((*ctx).state[0 as libc::c_int as usize]).wrapping_add(a);
        a = (*ctx).state[0 as libc::c_int as usize];
        (*ctx)
            .state[1 as libc::c_int
            as usize] = ((*ctx).state[1 as libc::c_int as usize]).wrapping_add(b);
        b = (*ctx).state[1 as libc::c_int as usize];
        (*ctx)
            .state[2 as libc::c_int
            as usize] = ((*ctx).state[2 as libc::c_int as usize]).wrapping_add(c);
        c = (*ctx).state[2 as libc::c_int as usize];
        (*ctx)
            .state[3 as libc::c_int
            as usize] = ((*ctx).state[3 as libc::c_int as usize]).wrapping_add(d);
        d = (*ctx).state[3 as libc::c_int as usize];
        (*ctx)
            .state[4 as libc::c_int
            as usize] = ((*ctx).state[4 as libc::c_int as usize]).wrapping_add(e);
        e = (*ctx).state[4 as libc::c_int as usize];
        (*ctx)
            .state[5 as libc::c_int
            as usize] = ((*ctx).state[5 as libc::c_int as usize]).wrapping_add(f);
        f = (*ctx).state[5 as libc::c_int as usize];
        (*ctx)
            .state[6 as libc::c_int
            as usize] = ((*ctx).state[6 as libc::c_int as usize]).wrapping_add(g);
        g = (*ctx).state[6 as libc::c_int as usize];
        (*ctx)
            .state[7 as libc::c_int
            as usize] = ((*ctx).state[7 as libc::c_int as usize]).wrapping_add(h);
        h = (*ctx).state[7 as libc::c_int as usize];
    }
}
unsafe extern "C" fn run_static_initializers() {
    sha512_round_constants = [
        ((0x428a2f98 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xd728ae22 as libc::c_uint as libc::c_ulong),
        ((0x71374491 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x23ef65cd as libc::c_int as libc::c_ulong),
        ((0xb5c0fbcf as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xec4d3b2f as libc::c_uint as libc::c_ulong),
        ((0xe9b5dba5 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x8189dbbc as libc::c_uint as libc::c_ulong),
        ((0x3956c25b as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xf348b538 as libc::c_uint as libc::c_ulong),
        ((0x59f111f1 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xb605d019 as libc::c_uint as libc::c_ulong),
        ((0x923f82a4 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xaf194f9b as libc::c_uint as libc::c_ulong),
        ((0xab1c5ed5 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xda6d8118 as libc::c_uint as libc::c_ulong),
        ((0xd807aa98 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xa3030242 as libc::c_uint as libc::c_ulong),
        ((0x12835b01 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x45706fbe as libc::c_int as libc::c_ulong),
        ((0x243185be as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x4ee4b28c as libc::c_int as libc::c_ulong),
        ((0x550c7dc3 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xd5ffb4e2 as libc::c_uint as libc::c_ulong),
        ((0x72be5d74 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xf27b896f as libc::c_uint as libc::c_ulong),
        ((0x80deb1fe as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x3b1696b1 as libc::c_int as libc::c_ulong),
        ((0x9bdc06a7 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x25c71235 as libc::c_int as libc::c_ulong),
        ((0xc19bf174 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xcf692694 as libc::c_uint as libc::c_ulong),
        ((0xe49b69c1 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x9ef14ad2 as libc::c_uint as libc::c_ulong),
        ((0xefbe4786 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x384f25e3 as libc::c_int as libc::c_ulong),
        ((0xfc19dc6 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x8b8cd5b5 as libc::c_uint as libc::c_ulong),
        ((0x240ca1cc as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x77ac9c65 as libc::c_int as libc::c_ulong),
        ((0x2de92c6f as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x592b0275 as libc::c_int as libc::c_ulong),
        ((0x4a7484aa as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x6ea6e483 as libc::c_int as libc::c_ulong),
        ((0x5cb0a9dc as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xbd41fbd4 as libc::c_uint as libc::c_ulong),
        ((0x76f988da as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x831153b5 as libc::c_uint as libc::c_ulong),
        ((0x983e5152 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xee66dfab as libc::c_uint as libc::c_ulong),
        ((0xa831c66d as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x2db43210 as libc::c_int as libc::c_ulong),
        ((0xb00327c8 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x98fb213f as libc::c_uint as libc::c_ulong),
        ((0xbf597fc7 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xbeef0ee4 as libc::c_uint as libc::c_ulong),
        ((0xc6e00bf3 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x3da88fc2 as libc::c_int as libc::c_ulong),
        ((0xd5a79147 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x930aa725 as libc::c_uint as libc::c_ulong),
        ((0x6ca6351 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xe003826f as libc::c_uint as libc::c_ulong),
        ((0x14292967 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xa0e6e70 as libc::c_int as libc::c_ulong),
        ((0x27b70a85 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x46d22ffc as libc::c_int as libc::c_ulong),
        ((0x2e1b2138 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x5c26c926 as libc::c_int as libc::c_ulong),
        ((0x4d2c6dfc as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x5ac42aed as libc::c_int as libc::c_ulong),
        ((0x53380d13 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x9d95b3df as libc::c_uint as libc::c_ulong),
        ((0x650a7354 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x8baf63de as libc::c_uint as libc::c_ulong),
        ((0x766a0abb as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x3c77b2a8 as libc::c_int as libc::c_ulong),
        ((0x81c2c92e as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x47edaee6 as libc::c_int as libc::c_ulong),
        ((0x92722c85 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x1482353b as libc::c_int as libc::c_ulong),
        ((0xa2bfe8a1 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x4cf10364 as libc::c_int as libc::c_ulong),
        ((0xa81a664b as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xbc423001 as libc::c_uint as libc::c_ulong),
        ((0xc24b8b70 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xd0f89791 as libc::c_uint as libc::c_ulong),
        ((0xc76c51a3 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x654be30 as libc::c_int as libc::c_ulong),
        ((0xd192e819 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xd6ef5218 as libc::c_uint as libc::c_ulong),
        ((0xd6990624 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x5565a910 as libc::c_int as libc::c_ulong),
        ((0xf40e3585 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x5771202a as libc::c_int as libc::c_ulong),
        ((0x106aa070 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x32bbd1b8 as libc::c_int as libc::c_ulong),
        ((0x19a4c116 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xb8d2d0c8 as libc::c_uint as libc::c_ulong),
        ((0x1e376c08 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x5141ab53 as libc::c_int as libc::c_ulong),
        ((0x2748774c as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xdf8eeb99 as libc::c_uint as libc::c_ulong),
        ((0x34b0bcb5 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xe19b48a8 as libc::c_uint as libc::c_ulong),
        ((0x391c0cb3 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xc5c95a63 as libc::c_uint as libc::c_ulong),
        ((0x4ed8aa4a as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xe3418acb as libc::c_uint as libc::c_ulong),
        ((0x5b9cca4f as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x7763e373 as libc::c_int as libc::c_ulong),
        ((0x682e6ff3 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xd6b2b8a3 as libc::c_uint as libc::c_ulong),
        ((0x748f82ee as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x5defb2fc as libc::c_int as libc::c_ulong),
        ((0x78a5636f as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x43172f60 as libc::c_int as libc::c_ulong),
        ((0x84c87814 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xa1f0ab72 as libc::c_uint as libc::c_ulong),
        ((0x8cc70208 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x1a6439ec as libc::c_int as libc::c_ulong),
        ((0x90befffa as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x23631e28 as libc::c_int as libc::c_ulong),
        ((0xa4506ceb as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xde82bde9 as libc::c_uint as libc::c_ulong),
        ((0xbef9a3f7 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xb2c67915 as libc::c_uint as libc::c_ulong),
        ((0xc67178f2 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xe372532b as libc::c_uint as libc::c_ulong),
        ((0xca273ece as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xea26619c as libc::c_uint as libc::c_ulong),
        ((0xd186b8c7 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x21c0c207 as libc::c_int as libc::c_ulong),
        ((0xeada7dd6 as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xcde0eb1e as libc::c_uint as libc::c_ulong),
        ((0xf57d4f7f as libc::c_uint as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xee6ed178 as libc::c_uint as libc::c_ulong),
        ((0x6f067aa as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x72176fba as libc::c_int as libc::c_ulong),
        ((0xa637dc5 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xa2c898a6 as libc::c_uint as libc::c_ulong),
        ((0x113f9804 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xbef90dae as libc::c_uint as libc::c_ulong),
        ((0x1b710b35 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x131c471b as libc::c_int as libc::c_ulong),
        ((0x28db77f5 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x23047d84 as libc::c_int as libc::c_ulong),
        ((0x32caab7b as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x40c72493 as libc::c_int as libc::c_ulong),
        ((0x3c9ebe0a as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x15c9bebc as libc::c_int as libc::c_ulong),
        ((0x431d67c4 as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x9c100d4c as libc::c_uint as libc::c_ulong),
        ((0x4cc5d4be as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xcb3e42b6 as libc::c_uint as libc::c_ulong),
        ((0x597f299c as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0xfc657e2a as libc::c_uint as libc::c_ulong),
        ((0x5fcb6fab as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x3ad6faec as libc::c_int as libc::c_ulong),
        ((0x6c44198c as libc::c_int as u64_0) << 32 as libc::c_int)
            .wrapping_add(0x4a475817 as libc::c_int as libc::c_ulong),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

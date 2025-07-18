use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub total: [uint32_t; 2],
    pub buflen: size_t,
    pub buffer: [uint32_t; 32],
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut fillbuf: [libc::c_uchar; 64] = [
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
];
pub unsafe extern "C" fn sha256_init_ctx(mut ctx: *mut sha256_ctx) {
    (*ctx).state[0 as libc::c_int as usize] = 0x6a09e667 as libc::c_ulong as uint32_t;
    (*ctx).state[1 as libc::c_int as usize] = 0xbb67ae85 as libc::c_ulong as uint32_t;
    (*ctx).state[2 as libc::c_int as usize] = 0x3c6ef372 as libc::c_ulong as uint32_t;
    (*ctx).state[3 as libc::c_int as usize] = 0xa54ff53a as libc::c_ulong as uint32_t;
    (*ctx).state[4 as libc::c_int as usize] = 0x510e527f as libc::c_ulong as uint32_t;
    (*ctx).state[5 as libc::c_int as usize] = 0x9b05688c as libc::c_ulong as uint32_t;
    (*ctx).state[6 as libc::c_int as usize] = 0x1f83d9ab as libc::c_ulong as uint32_t;
    (*ctx).state[7 as libc::c_int as usize] = 0x5be0cd19 as libc::c_ulong as uint32_t;
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn sha224_init_ctx(mut ctx: *mut sha256_ctx) {
    (*ctx).state[0 as libc::c_int as usize] = 0xc1059ed8 as libc::c_ulong as uint32_t;
    (*ctx).state[1 as libc::c_int as usize] = 0x367cd507 as libc::c_ulong as uint32_t;
    (*ctx).state[2 as libc::c_int as usize] = 0x3070dd17 as libc::c_ulong as uint32_t;
    (*ctx).state[3 as libc::c_int as usize] = 0xf70e5939 as libc::c_ulong as uint32_t;
    (*ctx).state[4 as libc::c_int as usize] = 0xffc00b31 as libc::c_ulong as uint32_t;
    (*ctx).state[5 as libc::c_int as usize] = 0x68581511 as libc::c_ulong as uint32_t;
    (*ctx).state[6 as libc::c_int as usize] = 0x64f98fa7 as libc::c_ulong as uint32_t;
    (*ctx).state[7 as libc::c_int as usize] = 0xbefa4fa4 as libc::c_ulong as uint32_t;
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn set_uint32(mut cp: *mut libc::c_char, mut v: uint32_t) {
    memcpy(
        cp as *mut libc::c_void,
        &mut v as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn sha256_read_ctx(
    mut ctx: *const sha256_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_char = resbuf as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        set_uint32(
            r
                .offset(
                    (i as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                        as isize,
                ),
            __bswap_32((*ctx).state[i as usize]),
        );
        i += 1;
        i;
    }
    return resbuf;
}
pub unsafe extern "C" fn sha224_read_ctx(
    mut ctx: *const sha256_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_char = resbuf as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        set_uint32(
            r
                .offset(
                    (i as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                        as isize,
                ),
            __bswap_32((*ctx).state[i as usize]),
        );
        i += 1;
        i;
    }
    return resbuf;
}
unsafe extern "C" fn sha256_conclude_ctx(mut ctx: *mut sha256_ctx) {
    let mut bytes: size_t = (*ctx).buflen;
    let mut size: size_t = (if bytes < 56 as libc::c_int as libc::c_ulong {
        64 as libc::c_int / 4 as libc::c_int
    } else {
        64 as libc::c_int * 2 as libc::c_int / 4 as libc::c_int
    }) as size_t;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(bytes) as uint32_t as uint32_t;
    if ((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong) < bytes {
        (*ctx)
            .total[1 as libc::c_int
            as usize] = ((*ctx).total[1 as libc::c_int as usize]).wrapping_add(1);
        (*ctx).total[1 as libc::c_int as usize];
    }
    set_uint32(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as *mut uint32_t as *mut libc::c_char,
        __bswap_32(
            (*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
                | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int,
        ),
    );
    set_uint32(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as *mut uint32_t as *mut libc::c_char,
        __bswap_32((*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int),
    );
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char).offset(bytes as isize)
            as *mut libc::c_char as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_sub(bytes),
    );
    sha256_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ctx,
    );
}
pub unsafe extern "C" fn sha256_finish_ctx(
    mut ctx: *mut sha256_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sha256_conclude_ctx(ctx);
    return sha256_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn sha224_finish_ctx(
    mut ctx: *mut sha256_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sha256_conclude_ctx(ctx);
    return sha224_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn sha256_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha256_ctx = sha256_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha256_init_ctx(&mut ctx);
    sha256_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha256_finish_ctx(&mut ctx, resblock);
}
pub unsafe extern "C" fn sha224_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha256_ctx = sha256_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha224_init_ctx(&mut ctx);
    sha256_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha224_finish_ctx(&mut ctx, resblock);
}
pub unsafe extern "C" fn sha256_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha256_ctx,
) {
    if (*ctx).buflen != 0 as libc::c_int as libc::c_ulong {
        let mut left_over: size_t = (*ctx).buflen;
        let mut add: size_t = if (128 as libc::c_int as libc::c_ulong)
            .wrapping_sub(left_over) > len
        {
            len
        } else {
            (128 as libc::c_int as libc::c_ulong).wrapping_sub(left_over)
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
        if (*ctx).buflen > 64 as libc::c_int as libc::c_ulong {
            sha256_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                (*ctx).buflen & !(63 as libc::c_int) as libc::c_ulong,
                ctx,
            );
            (*ctx).buflen &= 63 as libc::c_int as libc::c_ulong;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                    .offset(
                        (left_over.wrapping_add(add)
                            & !(63 as libc::c_int) as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (*ctx).buflen,
            );
        }
        buffer = (buffer as *const libc::c_char).offset(add as isize)
            as *const libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(add) as size_t as size_t;
    }
    if len >= 64 as libc::c_int as libc::c_ulong {
        if (buffer as uintptr_t)
            .wrapping_rem(::std::mem::align_of::<uint32_t>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            while len > 64 as libc::c_int as libc::c_ulong {
                sha256_process_block(
                    memcpy(
                        ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                        buffer,
                        64 as libc::c_int as libc::c_ulong,
                    ),
                    64 as libc::c_int as size_t,
                    ctx,
                );
                buffer = (buffer as *const libc::c_char)
                    .offset(64 as libc::c_int as isize) as *const libc::c_void;
                len = (len as libc::c_ulong)
                    .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
        } else {
            sha256_process_block(
                buffer,
                len & !(63 as libc::c_int) as libc::c_ulong,
                ctx,
            );
            buffer = (buffer as *const libc::c_char)
                .offset((len & !(63 as libc::c_int) as libc::c_ulong) as isize)
                as *const libc::c_void;
            len &= 63 as libc::c_int as libc::c_ulong;
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
        if left_over_0 >= 64 as libc::c_int as libc::c_ulong {
            sha256_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                64 as libc::c_int as size_t,
                ctx,
            );
            left_over_0 = (left_over_0 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer).as_mut_ptr().offset(16 as libc::c_int as isize)
                    as *mut uint32_t as *const libc::c_void,
                left_over_0,
            );
        }
        (*ctx).buflen = left_over_0;
    }
}
static mut sha256_round_constants: [uint32_t; 64] = [
    0x428a2f98 as libc::c_ulong as uint32_t,
    0x71374491 as libc::c_ulong as uint32_t,
    0xb5c0fbcf as libc::c_ulong as uint32_t,
    0xe9b5dba5 as libc::c_ulong as uint32_t,
    0x3956c25b as libc::c_ulong as uint32_t,
    0x59f111f1 as libc::c_ulong as uint32_t,
    0x923f82a4 as libc::c_ulong as uint32_t,
    0xab1c5ed5 as libc::c_ulong as uint32_t,
    0xd807aa98 as libc::c_ulong as uint32_t,
    0x12835b01 as libc::c_ulong as uint32_t,
    0x243185be as libc::c_ulong as uint32_t,
    0x550c7dc3 as libc::c_ulong as uint32_t,
    0x72be5d74 as libc::c_ulong as uint32_t,
    0x80deb1fe as libc::c_ulong as uint32_t,
    0x9bdc06a7 as libc::c_ulong as uint32_t,
    0xc19bf174 as libc::c_ulong as uint32_t,
    0xe49b69c1 as libc::c_ulong as uint32_t,
    0xefbe4786 as libc::c_ulong as uint32_t,
    0xfc19dc6 as libc::c_ulong as uint32_t,
    0x240ca1cc as libc::c_ulong as uint32_t,
    0x2de92c6f as libc::c_ulong as uint32_t,
    0x4a7484aa as libc::c_ulong as uint32_t,
    0x5cb0a9dc as libc::c_ulong as uint32_t,
    0x76f988da as libc::c_ulong as uint32_t,
    0x983e5152 as libc::c_ulong as uint32_t,
    0xa831c66d as libc::c_ulong as uint32_t,
    0xb00327c8 as libc::c_ulong as uint32_t,
    0xbf597fc7 as libc::c_ulong as uint32_t,
    0xc6e00bf3 as libc::c_ulong as uint32_t,
    0xd5a79147 as libc::c_ulong as uint32_t,
    0x6ca6351 as libc::c_ulong as uint32_t,
    0x14292967 as libc::c_ulong as uint32_t,
    0x27b70a85 as libc::c_ulong as uint32_t,
    0x2e1b2138 as libc::c_ulong as uint32_t,
    0x4d2c6dfc as libc::c_ulong as uint32_t,
    0x53380d13 as libc::c_ulong as uint32_t,
    0x650a7354 as libc::c_ulong as uint32_t,
    0x766a0abb as libc::c_ulong as uint32_t,
    0x81c2c92e as libc::c_ulong as uint32_t,
    0x92722c85 as libc::c_ulong as uint32_t,
    0xa2bfe8a1 as libc::c_ulong as uint32_t,
    0xa81a664b as libc::c_ulong as uint32_t,
    0xc24b8b70 as libc::c_ulong as uint32_t,
    0xc76c51a3 as libc::c_ulong as uint32_t,
    0xd192e819 as libc::c_ulong as uint32_t,
    0xd6990624 as libc::c_ulong as uint32_t,
    0xf40e3585 as libc::c_ulong as uint32_t,
    0x106aa070 as libc::c_ulong as uint32_t,
    0x19a4c116 as libc::c_ulong as uint32_t,
    0x1e376c08 as libc::c_ulong as uint32_t,
    0x2748774c as libc::c_ulong as uint32_t,
    0x34b0bcb5 as libc::c_ulong as uint32_t,
    0x391c0cb3 as libc::c_ulong as uint32_t,
    0x4ed8aa4a as libc::c_ulong as uint32_t,
    0x5b9cca4f as libc::c_ulong as uint32_t,
    0x682e6ff3 as libc::c_ulong as uint32_t,
    0x748f82ee as libc::c_ulong as uint32_t,
    0x78a5636f as libc::c_ulong as uint32_t,
    0x84c87814 as libc::c_ulong as uint32_t,
    0x8cc70208 as libc::c_ulong as uint32_t,
    0x90befffa as libc::c_ulong as uint32_t,
    0xa4506ceb as libc::c_ulong as uint32_t,
    0xbef9a3f7 as libc::c_ulong as uint32_t,
    0xc67178f2 as libc::c_ulong as uint32_t,
];
pub unsafe extern "C" fn sha256_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha256_ctx,
) {
    let mut words: *const uint32_t = buffer as *const uint32_t;
    let mut nwords: size_t = len
        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    let mut endp: *const uint32_t = words.offset(nwords as isize);
    let mut x: [uint32_t; 16] = [0; 16];
    let mut a: uint32_t = (*ctx).state[0 as libc::c_int as usize];
    let mut b: uint32_t = (*ctx).state[1 as libc::c_int as usize];
    let mut c: uint32_t = (*ctx).state[2 as libc::c_int as usize];
    let mut d: uint32_t = (*ctx).state[3 as libc::c_int as usize];
    let mut e: uint32_t = (*ctx).state[4 as libc::c_int as usize];
    let mut f: uint32_t = (*ctx).state[5 as libc::c_int as usize];
    let mut g: uint32_t = (*ctx).state[6 as libc::c_int as usize];
    let mut h: uint32_t = (*ctx).state[7 as libc::c_int as usize];
    let mut lolen: uint32_t = len as uint32_t;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(lolen) as uint32_t as uint32_t;
    (*ctx)
        .total[1 as libc::c_int
        as usize] = ((*ctx).total[1 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            (len >> 31 as libc::c_int >> 1 as libc::c_int)
                .wrapping_add(
                    ((*ctx).total[0 as libc::c_int as usize] < lolen) as libc::c_int
                        as libc::c_ulong,
                ),
        ) as uint32_t as uint32_t;
    while words < endp {
        let mut tm: uint32_t = 0;
        let mut t0: uint32_t = 0;
        let mut t1: uint32_t = 0;
        let mut t: libc::c_int = 0;
        t = 0 as libc::c_int;
        while t < 16 as libc::c_int {
            x[t as usize] = __bswap_32(*words);
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[0 as libc::c_int as usize])
            .wrapping_add(x[0 as libc::c_int as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[1 as libc::c_int as usize])
            .wrapping_add(x[1 as libc::c_int as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[2 as libc::c_int as usize])
            .wrapping_add(x[2 as libc::c_int as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[3 as libc::c_int as usize])
            .wrapping_add(x[3 as libc::c_int as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[4 as libc::c_int as usize])
            .wrapping_add(x[4 as libc::c_int as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[5 as libc::c_int as usize])
            .wrapping_add(x[5 as libc::c_int as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[6 as libc::c_int as usize])
            .wrapping_add(x[6 as libc::c_int as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[7 as libc::c_int as usize])
            .wrapping_add(x[7 as libc::c_int as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[8 as libc::c_int as usize])
            .wrapping_add(x[8 as libc::c_int as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[9 as libc::c_int as usize])
            .wrapping_add(x[9 as libc::c_int as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[10 as libc::c_int as usize])
            .wrapping_add(x[10 as libc::c_int as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[11 as libc::c_int as usize])
            .wrapping_add(x[11 as libc::c_int as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[12 as libc::c_int as usize])
            .wrapping_add(x[12 as libc::c_int as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[13 as libc::c_int as usize])
            .wrapping_add(x[13 as libc::c_int as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[14 as libc::c_int as usize])
            .wrapping_add(x[14 as libc::c_int as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[15 as libc::c_int as usize])
            .wrapping_add(x[15 as libc::c_int as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        tm = ((x[(16 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(16 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(16 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(16 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(16 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(16 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(16 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(16 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(16 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(16 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(16 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(16 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(16 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[16 as libc::c_int as usize])
            .wrapping_add(x[(16 as libc::c_int & 0xf as libc::c_int) as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        tm = ((x[(17 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(17 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(17 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(17 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(17 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(17 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(17 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(17 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(17 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(17 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(17 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(17 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(17 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[17 as libc::c_int as usize])
            .wrapping_add(x[(17 as libc::c_int & 0xf as libc::c_int) as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        tm = ((x[(18 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(18 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(18 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(18 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(18 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(18 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(18 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(18 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(18 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(18 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(18 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(18 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(18 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[18 as libc::c_int as usize])
            .wrapping_add(x[(18 as libc::c_int & 0xf as libc::c_int) as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        tm = ((x[(19 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(19 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(19 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(19 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(19 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(19 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(19 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(19 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(19 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(19 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(19 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(19 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(19 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[19 as libc::c_int as usize])
            .wrapping_add(x[(19 as libc::c_int & 0xf as libc::c_int) as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        tm = ((x[(20 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(20 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(20 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(20 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(20 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(20 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(20 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(20 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(20 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(20 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(20 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(20 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(20 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[20 as libc::c_int as usize])
            .wrapping_add(x[(20 as libc::c_int & 0xf as libc::c_int) as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        tm = ((x[(21 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(21 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(21 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(21 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(21 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(21 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(21 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(21 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(21 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(21 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(21 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(21 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(21 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[21 as libc::c_int as usize])
            .wrapping_add(x[(21 as libc::c_int & 0xf as libc::c_int) as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        tm = ((x[(22 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(22 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(22 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(22 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(22 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(22 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(22 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(22 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(22 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(22 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(22 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(22 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(22 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[22 as libc::c_int as usize])
            .wrapping_add(x[(22 as libc::c_int & 0xf as libc::c_int) as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        tm = ((x[(23 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(23 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(23 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(23 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(23 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(23 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(23 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(23 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(23 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(23 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(23 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(23 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(23 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[23 as libc::c_int as usize])
            .wrapping_add(x[(23 as libc::c_int & 0xf as libc::c_int) as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        tm = ((x[(24 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(24 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(24 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(24 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(24 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(24 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(24 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(24 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(24 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(24 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(24 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(24 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(24 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[24 as libc::c_int as usize])
            .wrapping_add(x[(24 as libc::c_int & 0xf as libc::c_int) as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        tm = ((x[(25 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(25 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(25 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(25 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(25 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(25 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(25 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(25 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(25 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(25 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(25 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(25 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(25 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[25 as libc::c_int as usize])
            .wrapping_add(x[(25 as libc::c_int & 0xf as libc::c_int) as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        tm = ((x[(26 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(26 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(26 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(26 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(26 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(26 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(26 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(26 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(26 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(26 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(26 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(26 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(26 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[26 as libc::c_int as usize])
            .wrapping_add(x[(26 as libc::c_int & 0xf as libc::c_int) as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        tm = ((x[(27 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(27 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(27 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(27 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(27 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(27 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(27 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(27 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(27 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(27 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(27 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(27 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(27 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[27 as libc::c_int as usize])
            .wrapping_add(x[(27 as libc::c_int & 0xf as libc::c_int) as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        tm = ((x[(28 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(28 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(28 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(28 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(28 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(28 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(28 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(28 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(28 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(28 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(28 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(28 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(28 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[28 as libc::c_int as usize])
            .wrapping_add(x[(28 as libc::c_int & 0xf as libc::c_int) as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        tm = ((x[(29 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(29 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(29 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(29 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(29 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(29 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(29 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(29 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(29 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(29 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(29 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(29 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(29 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[29 as libc::c_int as usize])
            .wrapping_add(x[(29 as libc::c_int & 0xf as libc::c_int) as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        tm = ((x[(30 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(30 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(30 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(30 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(30 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(30 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(30 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(30 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(30 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(30 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(30 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(30 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(30 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[30 as libc::c_int as usize])
            .wrapping_add(x[(30 as libc::c_int & 0xf as libc::c_int) as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        tm = ((x[(31 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(31 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(31 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(31 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(31 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(31 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(31 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(31 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(31 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(31 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(31 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(31 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(31 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[31 as libc::c_int as usize])
            .wrapping_add(x[(31 as libc::c_int & 0xf as libc::c_int) as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        tm = ((x[(32 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(32 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(32 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(32 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(32 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(32 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(32 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(32 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(32 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(32 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(32 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(32 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(32 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[32 as libc::c_int as usize])
            .wrapping_add(x[(32 as libc::c_int & 0xf as libc::c_int) as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        tm = ((x[(33 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(33 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(33 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(33 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(33 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(33 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(33 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(33 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(33 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(33 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(33 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(33 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(33 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[33 as libc::c_int as usize])
            .wrapping_add(x[(33 as libc::c_int & 0xf as libc::c_int) as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        tm = ((x[(34 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(34 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(34 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(34 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(34 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(34 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(34 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(34 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(34 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(34 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(34 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(34 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(34 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[34 as libc::c_int as usize])
            .wrapping_add(x[(34 as libc::c_int & 0xf as libc::c_int) as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        tm = ((x[(35 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(35 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(35 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(35 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(35 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(35 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(35 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(35 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(35 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(35 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(35 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(35 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(35 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[35 as libc::c_int as usize])
            .wrapping_add(x[(35 as libc::c_int & 0xf as libc::c_int) as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        tm = ((x[(36 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(36 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(36 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(36 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(36 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(36 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(36 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(36 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(36 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(36 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(36 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(36 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(36 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[36 as libc::c_int as usize])
            .wrapping_add(x[(36 as libc::c_int & 0xf as libc::c_int) as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        tm = ((x[(37 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(37 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(37 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(37 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(37 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(37 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(37 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(37 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(37 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(37 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(37 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(37 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(37 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[37 as libc::c_int as usize])
            .wrapping_add(x[(37 as libc::c_int & 0xf as libc::c_int) as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        tm = ((x[(38 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(38 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(38 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(38 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(38 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(38 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(38 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(38 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(38 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(38 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(38 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(38 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(38 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[38 as libc::c_int as usize])
            .wrapping_add(x[(38 as libc::c_int & 0xf as libc::c_int) as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        tm = ((x[(39 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(39 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(39 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(39 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(39 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(39 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(39 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(39 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(39 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(39 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(39 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(39 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(39 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[39 as libc::c_int as usize])
            .wrapping_add(x[(39 as libc::c_int & 0xf as libc::c_int) as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        tm = ((x[(40 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(40 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(40 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(40 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(40 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(40 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(40 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(40 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(40 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(40 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(40 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(40 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(40 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[40 as libc::c_int as usize])
            .wrapping_add(x[(40 as libc::c_int & 0xf as libc::c_int) as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        tm = ((x[(41 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(41 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(41 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(41 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(41 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(41 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(41 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(41 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(41 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(41 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(41 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(41 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(41 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[41 as libc::c_int as usize])
            .wrapping_add(x[(41 as libc::c_int & 0xf as libc::c_int) as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        tm = ((x[(42 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(42 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(42 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(42 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(42 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(42 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(42 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(42 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(42 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(42 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(42 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(42 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(42 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[42 as libc::c_int as usize])
            .wrapping_add(x[(42 as libc::c_int & 0xf as libc::c_int) as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        tm = ((x[(43 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(43 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(43 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(43 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(43 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(43 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(43 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(43 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(43 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(43 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(43 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(43 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(43 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[43 as libc::c_int as usize])
            .wrapping_add(x[(43 as libc::c_int & 0xf as libc::c_int) as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        tm = ((x[(44 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(44 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(44 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(44 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(44 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(44 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(44 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(44 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(44 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(44 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(44 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(44 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(44 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[44 as libc::c_int as usize])
            .wrapping_add(x[(44 as libc::c_int & 0xf as libc::c_int) as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        tm = ((x[(45 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(45 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(45 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(45 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(45 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(45 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(45 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(45 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(45 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(45 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(45 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(45 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(45 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[45 as libc::c_int as usize])
            .wrapping_add(x[(45 as libc::c_int & 0xf as libc::c_int) as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        tm = ((x[(46 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(46 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(46 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(46 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(46 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(46 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(46 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(46 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(46 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(46 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(46 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(46 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(46 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[46 as libc::c_int as usize])
            .wrapping_add(x[(46 as libc::c_int & 0xf as libc::c_int) as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        tm = ((x[(47 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(47 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(47 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(47 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(47 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(47 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(47 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(47 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(47 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(47 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(47 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(47 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(47 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[47 as libc::c_int as usize])
            .wrapping_add(x[(47 as libc::c_int & 0xf as libc::c_int) as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        tm = ((x[(48 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(48 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(48 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(48 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(48 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(48 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(48 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(48 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(48 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(48 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(48 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(48 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(48 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[48 as libc::c_int as usize])
            .wrapping_add(x[(48 as libc::c_int & 0xf as libc::c_int) as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        tm = ((x[(49 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(49 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(49 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(49 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(49 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(49 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(49 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(49 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(49 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(49 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(49 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(49 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(49 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[49 as libc::c_int as usize])
            .wrapping_add(x[(49 as libc::c_int & 0xf as libc::c_int) as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        tm = ((x[(50 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(50 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(50 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(50 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(50 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(50 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(50 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(50 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(50 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(50 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(50 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(50 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(50 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[50 as libc::c_int as usize])
            .wrapping_add(x[(50 as libc::c_int & 0xf as libc::c_int) as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        tm = ((x[(51 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(51 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(51 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(51 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(51 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(51 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(51 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(51 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(51 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(51 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(51 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(51 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(51 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[51 as libc::c_int as usize])
            .wrapping_add(x[(51 as libc::c_int & 0xf as libc::c_int) as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        tm = ((x[(52 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(52 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(52 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(52 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(52 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(52 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(52 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(52 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(52 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(52 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(52 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(52 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(52 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[52 as libc::c_int as usize])
            .wrapping_add(x[(52 as libc::c_int & 0xf as libc::c_int) as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        tm = ((x[(53 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(53 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(53 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(53 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(53 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(53 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(53 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(53 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(53 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(53 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(53 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(53 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(53 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[53 as libc::c_int as usize])
            .wrapping_add(x[(53 as libc::c_int & 0xf as libc::c_int) as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        tm = ((x[(54 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(54 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(54 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(54 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(54 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(54 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(54 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(54 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(54 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(54 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(54 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(54 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(54 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[54 as libc::c_int as usize])
            .wrapping_add(x[(54 as libc::c_int & 0xf as libc::c_int) as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        tm = ((x[(55 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(55 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(55 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(55 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(55 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(55 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(55 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(55 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(55 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(55 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(55 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(55 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(55 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[55 as libc::c_int as usize])
            .wrapping_add(x[(55 as libc::c_int & 0xf as libc::c_int) as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        t0 = ((a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (a << 10 as libc::c_int | a >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        tm = ((x[(56 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(56 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(56 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(56 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(56 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(56 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(56 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(56 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(56 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(56 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(56 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(56 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(56 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = h
            .wrapping_add(
                (e << 26 as libc::c_int | e >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (e << 21 as libc::c_int
                        | e >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (e << 7 as libc::c_int | e >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(sha256_round_constants[56 as libc::c_int as usize])
            .wrapping_add(x[(56 as libc::c_int & 0xf as libc::c_int) as usize]);
        d = (d as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        h = t0.wrapping_add(t1);
        t0 = ((h << 30 as libc::c_int | h >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (h << 19 as libc::c_int | h >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (h << 10 as libc::c_int | h >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(h & a | b & (h | a));
        tm = ((x[(57 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(57 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(57 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(57 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(57 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(57 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(57 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(57 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(57 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(57 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(57 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(57 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(57 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = g
            .wrapping_add(
                (d << 26 as libc::c_int | d >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (d << 21 as libc::c_int
                        | d >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(sha256_round_constants[57 as libc::c_int as usize])
            .wrapping_add(x[(57 as libc::c_int & 0xf as libc::c_int) as usize]);
        c = (c as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        g = t0.wrapping_add(t1);
        t0 = ((g << 30 as libc::c_int | g >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (g << 19 as libc::c_int | g >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (g << 10 as libc::c_int | g >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(g & h | a & (g | h));
        tm = ((x[(58 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(58 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(58 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(58 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(58 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(58 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(58 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(58 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(58 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(58 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(58 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(58 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(58 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = f
            .wrapping_add(
                (c << 26 as libc::c_int | c >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (c << 21 as libc::c_int
                        | c >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (c << 7 as libc::c_int | c >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(sha256_round_constants[58 as libc::c_int as usize])
            .wrapping_add(x[(58 as libc::c_int & 0xf as libc::c_int) as usize]);
        b = (b as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        f = t0.wrapping_add(t1);
        t0 = ((f << 30 as libc::c_int | f >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (f << 19 as libc::c_int | f >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (f << 10 as libc::c_int | f >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(f & g | h & (f | g));
        tm = ((x[(59 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(59 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(59 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(59 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(59 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(59 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(59 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(59 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(59 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(59 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(59 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(59 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(59 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = e
            .wrapping_add(
                (b << 26 as libc::c_int | b >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (b << 21 as libc::c_int
                        | b >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (b << 7 as libc::c_int | b >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(sha256_round_constants[59 as libc::c_int as usize])
            .wrapping_add(x[(59 as libc::c_int & 0xf as libc::c_int) as usize]);
        a = (a as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        e = t0.wrapping_add(t1);
        t0 = ((e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (e << 19 as libc::c_int | e >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (e << 10 as libc::c_int | e >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(e & f | g & (e | f));
        tm = ((x[(60 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(60 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(60 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(60 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(60 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(60 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(60 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(60 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(60 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(60 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(60 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(60 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(60 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = d
            .wrapping_add(
                (a << 26 as libc::c_int | a >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (a << 21 as libc::c_int
                        | a >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(sha256_round_constants[60 as libc::c_int as usize])
            .wrapping_add(x[(60 as libc::c_int & 0xf as libc::c_int) as usize]);
        h = (h as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        d = t0.wrapping_add(t1);
        t0 = ((d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (d << 19 as libc::c_int | d >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(d & e | f & (d | e));
        tm = ((x[(61 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(61 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(61 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(61 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(61 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(61 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(61 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(61 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(61 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(61 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(61 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(61 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(61 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = c
            .wrapping_add(
                (h << 26 as libc::c_int | h >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (h << 21 as libc::c_int
                        | h >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (h << 7 as libc::c_int | h >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(sha256_round_constants[61 as libc::c_int as usize])
            .wrapping_add(x[(61 as libc::c_int & 0xf as libc::c_int) as usize]);
        g = (g as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        c = t0.wrapping_add(t1);
        t0 = ((c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (c << 19 as libc::c_int | c >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (c << 10 as libc::c_int | c >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(c & d | e & (c | d));
        tm = ((x[(62 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(62 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(62 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(62 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(62 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(62 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(62 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(62 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(62 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(62 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(62 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(62 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(62 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = b
            .wrapping_add(
                (g << 26 as libc::c_int | g >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (g << 21 as libc::c_int
                        | g >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (g << 7 as libc::c_int | g >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(sha256_round_constants[62 as libc::c_int as usize])
            .wrapping_add(x[(62 as libc::c_int & 0xf as libc::c_int) as usize]);
        f = (f as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        b = t0.wrapping_add(t1);
        t0 = ((b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int)
            ^ (b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int)
            ^ (b << 10 as libc::c_int | b >> 32 as libc::c_int - 10 as libc::c_int))
            .wrapping_add(b & c | d & (b | c));
        tm = ((x[(63 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | x[(63 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 32 as libc::c_int - 15 as libc::c_int)
            ^ (x[(63 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                << 13 as libc::c_int
                | x[(63 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> 32 as libc::c_int - 13 as libc::c_int)
            ^ x[(63 as libc::c_int - 2 as libc::c_int & 0xf as libc::c_int) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(
                x[(63 as libc::c_int - 7 as libc::c_int & 0xf as libc::c_int) as usize],
            )
            .wrapping_add(
                (x[(63 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int) as usize]
                    << 25 as libc::c_int
                    | x[(63 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 32 as libc::c_int - 25 as libc::c_int)
                    ^ (x[(63 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] << 14 as libc::c_int
                        | x[(63 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                            as usize] >> 32 as libc::c_int - 14 as libc::c_int)
                    ^ x[(63 as libc::c_int - 15 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(x[(63 as libc::c_int & 0xf as libc::c_int) as usize]);
        x[(63 as libc::c_int & 0xf as libc::c_int) as usize] = tm;
        t1 = a
            .wrapping_add(
                (f << 26 as libc::c_int | f >> 32 as libc::c_int - 26 as libc::c_int)
                    ^ (f << 21 as libc::c_int
                        | f >> 32 as libc::c_int - 21 as libc::c_int)
                    ^ (f << 7 as libc::c_int | f >> 32 as libc::c_int - 7 as libc::c_int),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(sha256_round_constants[63 as libc::c_int as usize])
            .wrapping_add(x[(63 as libc::c_int & 0xf as libc::c_int) as usize]);
        e = (e as libc::c_uint).wrapping_add(t1) as uint32_t as uint32_t;
        a = t0.wrapping_add(t1);
        (*ctx)
            .state[0 as libc::c_int
            as usize] = ((*ctx).state[0 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(a) as uint32_t as uint32_t;
        a = (*ctx).state[0 as libc::c_int as usize];
        (*ctx)
            .state[1 as libc::c_int
            as usize] = ((*ctx).state[1 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(b) as uint32_t as uint32_t;
        b = (*ctx).state[1 as libc::c_int as usize];
        (*ctx)
            .state[2 as libc::c_int
            as usize] = ((*ctx).state[2 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(c) as uint32_t as uint32_t;
        c = (*ctx).state[2 as libc::c_int as usize];
        (*ctx)
            .state[3 as libc::c_int
            as usize] = ((*ctx).state[3 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(d) as uint32_t as uint32_t;
        d = (*ctx).state[3 as libc::c_int as usize];
        (*ctx)
            .state[4 as libc::c_int
            as usize] = ((*ctx).state[4 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(e) as uint32_t as uint32_t;
        e = (*ctx).state[4 as libc::c_int as usize];
        (*ctx)
            .state[5 as libc::c_int
            as usize] = ((*ctx).state[5 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(f) as uint32_t as uint32_t;
        f = (*ctx).state[5 as libc::c_int as usize];
        (*ctx)
            .state[6 as libc::c_int
            as usize] = ((*ctx).state[6 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(g) as uint32_t as uint32_t;
        g = (*ctx).state[6 as libc::c_int as usize];
        (*ctx)
            .state[7 as libc::c_int
            as usize] = ((*ctx).state[7 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(h) as uint32_t as uint32_t;
        h = (*ctx).state[7 as libc::c_int as usize];
    }
}

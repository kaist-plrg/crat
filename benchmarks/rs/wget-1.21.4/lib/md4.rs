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
pub struct md4_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
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
pub unsafe extern "C" fn md4_init_ctx(mut ctx: *mut md4_ctx) {
    (*ctx).A = 0x67452301 as libc::c_int as uint32_t;
    (*ctx).B = 0xefcdab89 as libc::c_uint;
    (*ctx).C = 0x98badcfe as libc::c_uint;
    (*ctx).D = 0x10325476 as libc::c_int as uint32_t;
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as uint32_t;
}
unsafe extern "C" fn set_uint32(mut cp: *mut libc::c_char, mut v: uint32_t) {
    memcpy(
        cp as *mut libc::c_void,
        &mut v as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn md4_read_ctx(
    mut ctx: *const md4_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_char = resbuf as *mut libc::c_char;
    set_uint32(
        r
            .offset(
                (0 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as isize,
            ),
        (*ctx).A,
    );
    set_uint32(
        r
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as isize,
            ),
        (*ctx).B,
    );
    set_uint32(
        r
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as isize,
            ),
        (*ctx).C,
    );
    set_uint32(
        r
            .offset(
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as isize,
            ),
        (*ctx).D,
    );
    return resbuf;
}
pub unsafe extern "C" fn md4_finish_ctx(
    mut ctx: *mut md4_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut bytes: uint32_t = (*ctx).buflen;
    let mut pad: size_t = 0;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(bytes) as uint32_t as uint32_t;
    if (*ctx).total[0 as libc::c_int as usize] < bytes {
        (*ctx)
            .total[1 as libc::c_int
            as usize] = ((*ctx).total[1 as libc::c_int as usize]).wrapping_add(1);
        (*ctx).total[1 as libc::c_int as usize];
    }
    pad = (if bytes >= 56 as libc::c_int as libc::c_uint {
        ((64 as libc::c_int + 56 as libc::c_int) as libc::c_uint).wrapping_sub(bytes)
    } else {
        (56 as libc::c_int as libc::c_uint).wrapping_sub(bytes)
    }) as size_t;
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char).offset(bytes as isize)
            as *mut libc::c_char as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        pad,
    );
    (*ctx)
        .buffer[(bytes as libc::c_ulong)
        .wrapping_add(pad)
        .wrapping_div(4 as libc::c_int as libc::c_ulong)
        as usize] = (*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int;
    (*ctx)
        .buffer[(bytes as libc::c_ulong)
        .wrapping_add(pad)
        .wrapping_div(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        as usize] = (*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
        | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int;
    md4_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        (bytes as libc::c_ulong)
            .wrapping_add(pad)
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
        ctx,
    );
    return md4_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn md4_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: md4_ctx = md4_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    md4_init_ctx(&mut ctx);
    md4_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return md4_finish_ctx(&mut ctx, resblock);
}
pub unsafe extern "C" fn md4_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md4_ctx,
) {
    if (*ctx).buflen != 0 as libc::c_int as libc::c_uint {
        let mut left_over: size_t = (*ctx).buflen as size_t;
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
            .buflen = ((*ctx).buflen as libc::c_ulong).wrapping_add(add) as uint32_t
            as uint32_t;
        if (*ctx).buflen > 64 as libc::c_int as libc::c_uint {
            md4_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                ((*ctx).buflen & !(63 as libc::c_int) as libc::c_uint) as size_t,
                ctx,
            );
            (*ctx).buflen &= 63 as libc::c_int as libc::c_uint;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                    .offset(
                        (left_over.wrapping_add(add)
                            & !(63 as libc::c_int) as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (*ctx).buflen as libc::c_ulong,
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
                md4_process_block(
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
            md4_process_block(buffer, len & !(63 as libc::c_int) as libc::c_ulong, ctx);
            buffer = (buffer as *const libc::c_char)
                .offset((len & !(63 as libc::c_int) as libc::c_ulong) as isize)
                as *const libc::c_void;
            len &= 63 as libc::c_int as libc::c_ulong;
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        let mut left_over_0: size_t = (*ctx).buflen as size_t;
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over_0 as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            len,
        );
        left_over_0 = (left_over_0 as libc::c_ulong).wrapping_add(len) as size_t
            as size_t;
        if left_over_0 >= 64 as libc::c_int as libc::c_ulong {
            md4_process_block(
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
        (*ctx).buflen = left_over_0 as uint32_t;
    }
}
pub unsafe extern "C" fn md4_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md4_ctx,
) {
    let mut words: *const uint32_t = buffer as *const uint32_t;
    let mut nwords: size_t = len
        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    let mut endp: *const uint32_t = words.offset(nwords as isize);
    let mut x: [uint32_t; 16] = [0; 16];
    let mut A: uint32_t = (*ctx).A;
    let mut B: uint32_t = (*ctx).B;
    let mut C: uint32_t = (*ctx).C;
    let mut D: uint32_t = (*ctx).D;
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
        let mut t: libc::c_int = 0;
        t = 0 as libc::c_int;
        while t < 16 as libc::c_int {
            x[t as usize] = *words;
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[0 as libc::c_int as usize])
            << 3 as libc::c_int
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[0 as libc::c_int as usize])
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[1 as libc::c_int as usize])
            << 7 as libc::c_int
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[1 as libc::c_int as usize])
                >> 32 as libc::c_int - 7 as libc::c_int;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[2 as libc::c_int as usize])
            << 11 as libc::c_int
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[2 as libc::c_int as usize])
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[3 as libc::c_int as usize])
            << 19 as libc::c_int
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[3 as libc::c_int as usize])
                >> 32 as libc::c_int - 19 as libc::c_int;
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[4 as libc::c_int as usize])
            << 3 as libc::c_int
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[4 as libc::c_int as usize])
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[5 as libc::c_int as usize])
            << 7 as libc::c_int
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[5 as libc::c_int as usize])
                >> 32 as libc::c_int - 7 as libc::c_int;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[6 as libc::c_int as usize])
            << 11 as libc::c_int
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[6 as libc::c_int as usize])
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[7 as libc::c_int as usize])
            << 19 as libc::c_int
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[7 as libc::c_int as usize])
                >> 32 as libc::c_int - 19 as libc::c_int;
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[8 as libc::c_int as usize])
            << 3 as libc::c_int
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[8 as libc::c_int as usize])
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[9 as libc::c_int as usize])
            << 7 as libc::c_int
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[9 as libc::c_int as usize])
                >> 32 as libc::c_int - 7 as libc::c_int;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[10 as libc::c_int as usize])
            << 11 as libc::c_int
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[10 as libc::c_int as usize])
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[11 as libc::c_int as usize])
            << 19 as libc::c_int
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[11 as libc::c_int as usize])
                >> 32 as libc::c_int - 19 as libc::c_int;
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[12 as libc::c_int as usize])
            << 3 as libc::c_int
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[12 as libc::c_int as usize])
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[13 as libc::c_int as usize])
            << 7 as libc::c_int
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[13 as libc::c_int as usize])
                >> 32 as libc::c_int - 7 as libc::c_int;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[14 as libc::c_int as usize])
            << 11 as libc::c_int
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[14 as libc::c_int as usize])
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[15 as libc::c_int as usize])
            << 19 as libc::c_int
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[15 as libc::c_int as usize])
                >> 32 as libc::c_int - 19 as libc::c_int;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[0 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[4 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 5 as libc::c_int
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 5 as libc::c_int;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[8 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[12 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 13 as libc::c_int
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 13 as libc::c_int;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[1 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[5 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 5 as libc::c_int
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 5 as libc::c_int;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[9 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[13 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 13 as libc::c_int
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 13 as libc::c_int;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[2 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[6 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 5 as libc::c_int
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 5 as libc::c_int;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[10 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[14 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 13 as libc::c_int
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 13 as libc::c_int;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[3 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[7 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 5 as libc::c_int
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 5 as libc::c_int;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[11 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[15 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint) << 13 as libc::c_int
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 13 as libc::c_int;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[0 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[8 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[4 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 11 as libc::c_int
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[12 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 15 as libc::c_int
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 15 as libc::c_int;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[2 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[10 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[6 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 11 as libc::c_int
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[14 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 15 as libc::c_int
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 15 as libc::c_int;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[1 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[9 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[5 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 11 as libc::c_int
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[13 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 15 as libc::c_int
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 15 as libc::c_int;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[3 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 3 as libc::c_int
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 3 as libc::c_int;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[11 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 9 as libc::c_int
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 9 as libc::c_int;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[7 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 11 as libc::c_int
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 11 as libc::c_int;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[15 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint) << 15 as libc::c_int
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                >> 32 as libc::c_int - 15 as libc::c_int;
        (*ctx).A = ((*ctx).A as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        A = (*ctx).A;
        (*ctx).B = ((*ctx).B as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        B = (*ctx).B;
        (*ctx).C = ((*ctx).C as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        C = (*ctx).C;
        (*ctx).D = ((*ctx).D as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        D = (*ctx).D;
    }
}

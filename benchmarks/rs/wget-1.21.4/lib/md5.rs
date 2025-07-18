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
pub struct md5_ctx {
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
pub unsafe extern "C" fn md5_init_ctx(mut ctx: *mut md5_ctx) {
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
pub unsafe extern "C" fn md5_read_ctx(
    mut ctx: *const md5_ctx,
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
pub unsafe extern "C" fn md5_finish_ctx(
    mut ctx: *mut md5_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut bytes: uint32_t = (*ctx).buflen;
    let mut size: size_t = (if bytes < 56 as libc::c_int as libc::c_uint {
        64 as libc::c_int / 4 as libc::c_int
    } else {
        64 as libc::c_int * 2 as libc::c_int / 4 as libc::c_int
    }) as size_t;
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
    (*ctx)
        .buffer[size.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as usize] = (*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int;
    (*ctx)
        .buffer[size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = (*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
        | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int;
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char).offset(bytes as isize)
            as *mut libc::c_char as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_sub(bytes as libc::c_ulong),
    );
    md5_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ctx,
    );
    return md5_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn md5_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: md5_ctx = md5_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    md5_init_ctx(&mut ctx);
    md5_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return md5_finish_ctx(&mut ctx, resblock);
}
pub unsafe extern "C" fn md5_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md5_ctx,
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
            md5_process_block(
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
                md5_process_block(
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
            md5_process_block(buffer, len & !(63 as libc::c_int) as libc::c_ulong, ctx);
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
            md5_process_block(
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
pub unsafe extern "C" fn md5_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md5_ctx,
) {
    let mut correct_words: [uint32_t; 16] = [0; 16];
    let mut words: *const uint32_t = buffer as *const uint32_t;
    let mut nwords: size_t = len
        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    let mut endp: *const uint32_t = words.offset(nwords as isize);
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
        let mut cwp: *mut uint32_t = correct_words.as_mut_ptr();
        let mut A_save: uint32_t = A;
        let mut B_save: uint32_t = B;
        let mut C_save: uint32_t = C;
        let mut D_save: uint32_t = D;
        let fresh0 = cwp;
        cwp = cwp.offset(1);
        *fresh0 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh0)
                    .wrapping_add(0xd76aa478 as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        let fresh1 = cwp;
        cwp = cwp.offset(1);
        *fresh1 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh1)
                    .wrapping_add(0xe8c7b756 as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        let fresh2 = cwp;
        cwp = cwp.offset(1);
        *fresh2 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh2)
                    .wrapping_add(0x242070db as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        let fresh3 = cwp;
        cwp = cwp.offset(1);
        *fresh3 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh3)
                    .wrapping_add(0xc1bdceee as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        let fresh4 = cwp;
        cwp = cwp.offset(1);
        *fresh4 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh4)
                    .wrapping_add(0xf57c0faf as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        let fresh5 = cwp;
        cwp = cwp.offset(1);
        *fresh5 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh5)
                    .wrapping_add(0x4787c62a as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        let fresh6 = cwp;
        cwp = cwp.offset(1);
        *fresh6 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh6)
                    .wrapping_add(0xa8304613 as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        let fresh7 = cwp;
        cwp = cwp.offset(1);
        *fresh7 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh7)
                    .wrapping_add(0xfd469501 as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        let fresh8 = cwp;
        cwp = cwp.offset(1);
        *fresh8 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh8)
                    .wrapping_add(0x698098d8 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        let fresh9 = cwp;
        cwp = cwp.offset(1);
        *fresh9 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh9)
                    .wrapping_add(0x8b44f7af as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        let fresh10 = cwp;
        cwp = cwp.offset(1);
        *fresh10 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh10)
                    .wrapping_add(0xffff5bb1 as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        let fresh11 = cwp;
        cwp = cwp.offset(1);
        *fresh11 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh11)
                    .wrapping_add(0x895cd7be as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        let fresh12 = cwp;
        cwp = cwp.offset(1);
        *fresh12 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh12)
                    .wrapping_add(0x6b901122 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        let fresh13 = cwp;
        cwp = cwp.offset(1);
        *fresh13 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh13)
                    .wrapping_add(0xfd987193 as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        let fresh14 = cwp;
        cwp = cwp.offset(1);
        *fresh14 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh14)
                    .wrapping_add(0xa679438e as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        let fresh15 = cwp;
        cwp = cwp.offset(1);
        *fresh15 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh15)
                    .wrapping_add(0x49b40821 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[1 as libc::c_int as usize])
                    .wrapping_add(0xf61e2562 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[6 as libc::c_int as usize])
                    .wrapping_add(0xc040b340 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[11 as libc::c_int as usize])
                    .wrapping_add(0x265e5a51 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[0 as libc::c_int as usize])
                    .wrapping_add(0xe9b6c7aa as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[5 as libc::c_int as usize])
                    .wrapping_add(0xd62f105d as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[10 as libc::c_int as usize])
                    .wrapping_add(0x2441453 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[15 as libc::c_int as usize])
                    .wrapping_add(0xd8a1e681 as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[4 as libc::c_int as usize])
                    .wrapping_add(0xe7d3fbc8 as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[9 as libc::c_int as usize])
                    .wrapping_add(0x21e1cde6 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[14 as libc::c_int as usize])
                    .wrapping_add(0xc33707d6 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[3 as libc::c_int as usize])
                    .wrapping_add(0xf4d50d87 as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[8 as libc::c_int as usize])
                    .wrapping_add(0x455a14ed as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[13 as libc::c_int as usize])
                    .wrapping_add(0xa9e3e905 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[2 as libc::c_int as usize])
                    .wrapping_add(0xfcefa3f8 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[7 as libc::c_int as usize])
                    .wrapping_add(0x676f02d9 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[12 as libc::c_int as usize])
                    .wrapping_add(0x8d2a4c8a as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[5 as libc::c_int as usize])
                    .wrapping_add(0xfffa3942 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[8 as libc::c_int as usize])
                    .wrapping_add(0x8771f681 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[11 as libc::c_int as usize])
                    .wrapping_add(0x6d9d6122 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[14 as libc::c_int as usize])
                    .wrapping_add(0xfde5380c as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[1 as libc::c_int as usize])
                    .wrapping_add(0xa4beea44 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[4 as libc::c_int as usize])
                    .wrapping_add(0x4bdecfa9 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[7 as libc::c_int as usize])
                    .wrapping_add(0xf6bb4b60 as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[10 as libc::c_int as usize])
                    .wrapping_add(0xbebfbc70 as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[13 as libc::c_int as usize])
                    .wrapping_add(0x289b7ec6 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[0 as libc::c_int as usize])
                    .wrapping_add(0xeaa127fa as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[3 as libc::c_int as usize])
                    .wrapping_add(0xd4ef3085 as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[6 as libc::c_int as usize])
                    .wrapping_add(0x4881d05 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[9 as libc::c_int as usize])
                    .wrapping_add(0xd9d4d039 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[12 as libc::c_int as usize])
                    .wrapping_add(0xe6db99e5 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[15 as libc::c_int as usize])
                    .wrapping_add(0x1fa27cf8 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[2 as libc::c_int as usize])
                    .wrapping_add(0xc4ac5665 as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[0 as libc::c_int as usize])
                    .wrapping_add(0xf4292244 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[7 as libc::c_int as usize])
                    .wrapping_add(0x432aff97 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[14 as libc::c_int as usize])
                    .wrapping_add(0xab9423a7 as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[5 as libc::c_int as usize])
                    .wrapping_add(0xfc93a039 as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[12 as libc::c_int as usize])
                    .wrapping_add(0x655b59c3 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[3 as libc::c_int as usize])
                    .wrapping_add(0x8f0ccc92 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[10 as libc::c_int as usize])
                    .wrapping_add(0xffeff47d as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[1 as libc::c_int as usize])
                    .wrapping_add(0x85845dd1 as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[8 as libc::c_int as usize])
                    .wrapping_add(0x6fa87e4f as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[15 as libc::c_int as usize])
                    .wrapping_add(0xfe2ce6e0 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[6 as libc::c_int as usize])
                    .wrapping_add(0xa3014314 as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[13 as libc::c_int as usize])
                    .wrapping_add(0x4e0811a1 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[4 as libc::c_int as usize])
                    .wrapping_add(0xf7537e82 as libc::c_uint),
            ) as uint32_t as uint32_t;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[11 as libc::c_int as usize])
                    .wrapping_add(0xbd3af235 as libc::c_uint),
            ) as uint32_t as uint32_t;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[2 as libc::c_int as usize])
                    .wrapping_add(0x2ad7d2bb as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[9 as libc::c_int as usize])
                    .wrapping_add(0xeb86d391 as libc::c_uint),
            ) as uint32_t as uint32_t;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
        A = (A as libc::c_uint).wrapping_add(A_save) as uint32_t as uint32_t;
        B = (B as libc::c_uint).wrapping_add(B_save) as uint32_t as uint32_t;
        C = (C as libc::c_uint).wrapping_add(C_save) as uint32_t as uint32_t;
        D = (D as libc::c_uint).wrapping_add(D_save) as uint32_t as uint32_t;
    }
    (*ctx).A = A;
    (*ctx).B = B;
    (*ctx).C = C;
    (*ctx).D = D;
}

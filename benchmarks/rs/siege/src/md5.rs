use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type md5_uint32 = uint32_t;
pub type md5_uintptr = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub A: md5_uint32,
    pub B: md5_uint32,
    pub C: md5_uint32,
    pub D: md5_uint32,
    pub total: [md5_uint32; 2],
    pub buflen: md5_uint32,
    pub buffer: [libc::c_char; 128],
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
    (*ctx).A = 0x67452301 as libc::c_int as md5_uint32;
    (*ctx).B = 0xefcdab89 as libc::c_uint;
    (*ctx).C = 0x98badcfe as libc::c_uint;
    (*ctx).D = 0x10325476 as libc::c_int as md5_uint32;
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as md5_uint32;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as md5_uint32;
}
pub unsafe extern "C" fn md5_read_ctx(
    mut ctx: *const md5_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut buffer: [md5_uint32; 4] = [0; 4];
    buffer[0 as libc::c_int as usize] = (*ctx).A;
    buffer[1 as libc::c_int as usize] = (*ctx).B;
    buffer[2 as libc::c_int as usize] = (*ctx).C;
    buffer[3 as libc::c_int as usize] = (*ctx).D;
    memcpy(
        resbuf,
        buffer.as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    return resbuf;
}
pub unsafe extern "C" fn md5_finish_ctx(
    mut ctx: *mut md5_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut bytes: md5_uint32 = (*ctx).buflen;
    let mut swap_bytes: md5_uint32 = 0;
    let mut pad: size_t = 0;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(bytes) as md5_uint32 as md5_uint32;
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
        &mut *((*ctx).buffer).as_mut_ptr().offset(bytes as isize) as *mut libc::c_char
            as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        pad,
    );
    swap_bytes = (*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int;
    memcpy(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset((bytes as libc::c_ulong).wrapping_add(pad) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        &mut swap_bytes as *mut md5_uint32 as *const libc::c_void,
        ::std::mem::size_of::<md5_uint32>() as libc::c_ulong,
    );
    swap_bytes = (*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
        | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int;
    memcpy(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(
                (bytes as libc::c_ulong)
                    .wrapping_add(pad)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut libc::c_char as *mut libc::c_void,
        &mut swap_bytes as *mut md5_uint32 as *const libc::c_void,
        ::std::mem::size_of::<md5_uint32>() as libc::c_ulong,
    );
    md5_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        (bytes as libc::c_ulong)
            .wrapping_add(pad)
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
        ctx,
    );
    return md5_read_ctx(ctx, resbuf);
}
pub unsafe extern "C" fn md5_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    let mut ctx: md5_ctx = md5_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 128],
    };
    let mut buffer: [libc::c_char; 4168] = [0; 4168];
    let mut sum: size_t = 0;
    md5_init_ctx(&mut ctx);
    loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
            n = fread(
                buffer.as_mut_ptr().offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                (4096 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if !(sum < 4096 as libc::c_int as libc::c_ulong
                && n != 0 as libc::c_int as libc::c_ulong)
            {
                break;
            }
        }
        if n == 0 as libc::c_int as libc::c_ulong && ferror(stream) != 0 {
            return 1 as libc::c_int;
        }
        if n == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        md5_process_block(
            buffer.as_mut_ptr() as *const libc::c_void,
            4096 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        md5_process_bytes(buffer.as_mut_ptr() as *const libc::c_void, sum, &mut ctx);
    }
    md5_finish_ctx(&mut ctx, resblock);
    return 0 as libc::c_int;
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
        buffer: [0; 128],
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
            &mut *((*ctx).buffer).as_mut_ptr().offset(left_over as isize)
                as *mut libc::c_char as *mut libc::c_void,
            buffer,
            add,
        );
        (*ctx)
            .buflen = ((*ctx).buflen as libc::c_ulong).wrapping_add(add) as md5_uint32
            as md5_uint32;
        if left_over.wrapping_add(add) > 64 as libc::c_int as libc::c_ulong {
            md5_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                left_over.wrapping_add(add) & !(63 as libc::c_int) as libc::c_ulong,
                ctx,
            );
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer)
                    .as_mut_ptr()
                    .offset(
                        (left_over.wrapping_add(add)
                            & !(63 as libc::c_int) as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                left_over.wrapping_add(add) & 63 as libc::c_int as libc::c_ulong,
            );
            (*ctx)
                .buflen = (left_over.wrapping_add(add)
                & 63 as libc::c_int as libc::c_ulong) as md5_uint32;
        }
        buffer = (buffer as *const libc::c_char).offset(add as isize)
            as *const libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(add) as size_t as size_t;
    }
    if len > 64 as libc::c_int as libc::c_ulong {
        if (buffer as md5_uintptr)
            .wrapping_rem(::std::mem::align_of::<md5_uint32>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            while len > 64 as libc::c_int as libc::c_ulong {
                memcpy(
                    ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                    buffer,
                    64 as libc::c_int as libc::c_ulong,
                );
                md5_process_block(
                    ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
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
        memcpy(((*ctx).buffer).as_mut_ptr() as *mut libc::c_void, buffer, len);
        (*ctx).buflen = len as md5_uint32;
    }
}
pub unsafe extern "C" fn md5_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md5_ctx,
) {
    let mut correct_words: [md5_uint32; 16] = [0; 16];
    let mut words: *const md5_uint32 = buffer as *const md5_uint32;
    let mut nwords: size_t = len
        .wrapping_div(::std::mem::size_of::<md5_uint32>() as libc::c_ulong);
    let mut endp: *const md5_uint32 = words.offset(nwords as isize);
    let mut A: md5_uint32 = (*ctx).A;
    let mut B: md5_uint32 = (*ctx).B;
    let mut C: md5_uint32 = (*ctx).C;
    let mut D: md5_uint32 = (*ctx).D;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(len) as md5_uint32 as md5_uint32;
    (*ctx)
        .total[1 as libc::c_int
        as usize] = ((*ctx).total[1 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            (len >> 31 as libc::c_int >> 1 as libc::c_int)
                .wrapping_add(
                    (((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong) < len)
                        as libc::c_int as libc::c_ulong,
                ),
        ) as md5_uint32 as md5_uint32;
    while words < endp {
        let mut cwp: *mut md5_uint32 = correct_words.as_mut_ptr();
        let mut A_save: md5_uint32 = A;
        let mut B_save: md5_uint32 = B;
        let mut C_save: md5_uint32 = C;
        let mut D_save: md5_uint32 = D;
        let fresh0 = cwp;
        cwp = cwp.offset(1);
        *fresh0 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh0)
                    .wrapping_add(0xd76aa478 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        let fresh1 = cwp;
        cwp = cwp.offset(1);
        *fresh1 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh1)
                    .wrapping_add(0xe8c7b756 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        let fresh2 = cwp;
        cwp = cwp.offset(1);
        *fresh2 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh2)
                    .wrapping_add(0x242070db as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        let fresh3 = cwp;
        cwp = cwp.offset(1);
        *fresh3 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh3)
                    .wrapping_add(0xc1bdceee as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        let fresh4 = cwp;
        cwp = cwp.offset(1);
        *fresh4 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh4)
                    .wrapping_add(0xf57c0faf as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        let fresh5 = cwp;
        cwp = cwp.offset(1);
        *fresh5 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh5)
                    .wrapping_add(0x4787c62a as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        let fresh6 = cwp;
        cwp = cwp.offset(1);
        *fresh6 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh6)
                    .wrapping_add(0xa8304613 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        let fresh7 = cwp;
        cwp = cwp.offset(1);
        *fresh7 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh7)
                    .wrapping_add(0xfd469501 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        let fresh8 = cwp;
        cwp = cwp.offset(1);
        *fresh8 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh8)
                    .wrapping_add(0x698098d8 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        let fresh9 = cwp;
        cwp = cwp.offset(1);
        *fresh9 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh9)
                    .wrapping_add(0x8b44f7af as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        let fresh10 = cwp;
        cwp = cwp.offset(1);
        *fresh10 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh10)
                    .wrapping_add(0xffff5bb1 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        let fresh11 = cwp;
        cwp = cwp.offset(1);
        *fresh11 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh11)
                    .wrapping_add(0x895cd7be as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        let fresh12 = cwp;
        cwp = cwp.offset(1);
        *fresh12 = *words;
        A = (A as libc::c_uint)
            .wrapping_add(
                (D ^ B & (C ^ D))
                    .wrapping_add(*fresh12)
                    .wrapping_add(0x6b901122 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        A = A << 7 as libc::c_int | A >> 32 as libc::c_int - 7 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        let fresh13 = cwp;
        cwp = cwp.offset(1);
        *fresh13 = *words;
        D = (D as libc::c_uint)
            .wrapping_add(
                (C ^ A & (B ^ C))
                    .wrapping_add(*fresh13)
                    .wrapping_add(0xfd987193 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        D = D << 12 as libc::c_int | D >> 32 as libc::c_int - 12 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        let fresh14 = cwp;
        cwp = cwp.offset(1);
        *fresh14 = *words;
        C = (C as libc::c_uint)
            .wrapping_add(
                (B ^ D & (A ^ B))
                    .wrapping_add(*fresh14)
                    .wrapping_add(0xa679438e as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        C = C << 17 as libc::c_int | C >> 32 as libc::c_int - 17 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        let fresh15 = cwp;
        cwp = cwp.offset(1);
        *fresh15 = *words;
        B = (B as libc::c_uint)
            .wrapping_add(
                (A ^ C & (D ^ A))
                    .wrapping_add(*fresh15)
                    .wrapping_add(0x49b40821 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        words = words.offset(1);
        words;
        B = B << 22 as libc::c_int | B >> 32 as libc::c_int - 22 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[1 as libc::c_int as usize])
                    .wrapping_add(0xf61e2562 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[6 as libc::c_int as usize])
                    .wrapping_add(0xc040b340 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[11 as libc::c_int as usize])
                    .wrapping_add(0x265e5a51 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[0 as libc::c_int as usize])
                    .wrapping_add(0xe9b6c7aa as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[5 as libc::c_int as usize])
                    .wrapping_add(0xd62f105d as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[10 as libc::c_int as usize])
                    .wrapping_add(0x2441453 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[15 as libc::c_int as usize])
                    .wrapping_add(0xd8a1e681 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[4 as libc::c_int as usize])
                    .wrapping_add(0xe7d3fbc8 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[9 as libc::c_int as usize])
                    .wrapping_add(0x21e1cde6 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[14 as libc::c_int as usize])
                    .wrapping_add(0xc33707d6 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[3 as libc::c_int as usize])
                    .wrapping_add(0xf4d50d87 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[8 as libc::c_int as usize])
                    .wrapping_add(0x455a14ed as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ D & (B ^ C))
                    .wrapping_add(correct_words[13 as libc::c_int as usize])
                    .wrapping_add(0xa9e3e905 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 5 as libc::c_int | A >> 32 as libc::c_int - 5 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ C & (A ^ B))
                    .wrapping_add(correct_words[2 as libc::c_int as usize])
                    .wrapping_add(0xfcefa3f8 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 9 as libc::c_int | D >> 32 as libc::c_int - 9 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ B & (D ^ A))
                    .wrapping_add(correct_words[7 as libc::c_int as usize])
                    .wrapping_add(0x676f02d9 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        C = C << 14 as libc::c_int | C >> 32 as libc::c_int - 14 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ A & (C ^ D))
                    .wrapping_add(correct_words[12 as libc::c_int as usize])
                    .wrapping_add(0x8d2a4c8a as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 20 as libc::c_int | B >> 32 as libc::c_int - 20 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[5 as libc::c_int as usize])
                    .wrapping_add(0xfffa3942 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[8 as libc::c_int as usize])
                    .wrapping_add(0x8771f681 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[11 as libc::c_int as usize])
                    .wrapping_add(0x6d9d6122 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[14 as libc::c_int as usize])
                    .wrapping_add(0xfde5380c as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[1 as libc::c_int as usize])
                    .wrapping_add(0xa4beea44 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[4 as libc::c_int as usize])
                    .wrapping_add(0x4bdecfa9 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[7 as libc::c_int as usize])
                    .wrapping_add(0xf6bb4b60 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[10 as libc::c_int as usize])
                    .wrapping_add(0xbebfbc70 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[13 as libc::c_int as usize])
                    .wrapping_add(0x289b7ec6 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[0 as libc::c_int as usize])
                    .wrapping_add(0xeaa127fa as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[3 as libc::c_int as usize])
                    .wrapping_add(0xd4ef3085 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[6 as libc::c_int as usize])
                    .wrapping_add(0x4881d05 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (B ^ C ^ D)
                    .wrapping_add(correct_words[9 as libc::c_int as usize])
                    .wrapping_add(0xd9d4d039 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 4 as libc::c_int | A >> 32 as libc::c_int - 4 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (A ^ B ^ C)
                    .wrapping_add(correct_words[12 as libc::c_int as usize])
                    .wrapping_add(0xe6db99e5 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 11 as libc::c_int | D >> 32 as libc::c_int - 11 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (D ^ A ^ B)
                    .wrapping_add(correct_words[15 as libc::c_int as usize])
                    .wrapping_add(0x1fa27cf8 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        C = C << 16 as libc::c_int | C >> 32 as libc::c_int - 16 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (C ^ D ^ A)
                    .wrapping_add(correct_words[2 as libc::c_int as usize])
                    .wrapping_add(0xc4ac5665 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 23 as libc::c_int | B >> 32 as libc::c_int - 23 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[0 as libc::c_int as usize])
                    .wrapping_add(0xf4292244 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[7 as libc::c_int as usize])
                    .wrapping_add(0x432aff97 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[14 as libc::c_int as usize])
                    .wrapping_add(0xab9423a7 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[5 as libc::c_int as usize])
                    .wrapping_add(0xfc93a039 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[12 as libc::c_int as usize])
                    .wrapping_add(0x655b59c3 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[3 as libc::c_int as usize])
                    .wrapping_add(0x8f0ccc92 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[10 as libc::c_int as usize])
                    .wrapping_add(0xffeff47d as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[1 as libc::c_int as usize])
                    .wrapping_add(0x85845dd1 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[8 as libc::c_int as usize])
                    .wrapping_add(0x6fa87e4f as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[15 as libc::c_int as usize])
                    .wrapping_add(0xfe2ce6e0 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[6 as libc::c_int as usize])
                    .wrapping_add(0xa3014314 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[13 as libc::c_int as usize])
                    .wrapping_add(0x4e0811a1 as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint)
            .wrapping_add(
                (C ^ (B | !D))
                    .wrapping_add(correct_words[4 as libc::c_int as usize])
                    .wrapping_add(0xf7537e82 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        A = A << 6 as libc::c_int | A >> 32 as libc::c_int - 6 as libc::c_int;
        A = (A as libc::c_uint).wrapping_add(B) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint)
            .wrapping_add(
                (B ^ (A | !C))
                    .wrapping_add(correct_words[11 as libc::c_int as usize])
                    .wrapping_add(0xbd3af235 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        D = D << 10 as libc::c_int | D >> 32 as libc::c_int - 10 as libc::c_int;
        D = (D as libc::c_uint).wrapping_add(A) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint)
            .wrapping_add(
                (A ^ (D | !B))
                    .wrapping_add(correct_words[2 as libc::c_int as usize])
                    .wrapping_add(0x2ad7d2bb as libc::c_int as md5_uint32),
            ) as md5_uint32 as md5_uint32;
        C = C << 15 as libc::c_int | C >> 32 as libc::c_int - 15 as libc::c_int;
        C = (C as libc::c_uint).wrapping_add(D) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint)
            .wrapping_add(
                (D ^ (C | !A))
                    .wrapping_add(correct_words[9 as libc::c_int as usize])
                    .wrapping_add(0xeb86d391 as libc::c_uint),
            ) as md5_uint32 as md5_uint32;
        B = B << 21 as libc::c_int | B >> 32 as libc::c_int - 21 as libc::c_int;
        B = (B as libc::c_uint).wrapping_add(C) as md5_uint32 as md5_uint32;
        A = (A as libc::c_uint).wrapping_add(A_save) as md5_uint32 as md5_uint32;
        B = (B as libc::c_uint).wrapping_add(B_save) as md5_uint32 as md5_uint32;
        C = (C as libc::c_uint).wrapping_add(C_save) as md5_uint32 as md5_uint32;
        D = (D as libc::c_uint).wrapping_add(D_save) as md5_uint32 as md5_uint32;
    }
    (*ctx).A = A;
    (*ctx).B = B;
    (*ctx).C = C;
    (*ctx).D = D;
}

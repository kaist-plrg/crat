use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_t {
    pub x: uint64_t,
    pub y: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_v {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm128_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tiny_queue_t {
    pub front: libc::c_int,
    pub count: libc::c_int,
    pub a: [libc::c_int; 32],
}
pub static mut seq_nt4_table: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern "C" fn hash64(mut key: uint64_t, mut mask: uint64_t) -> uint64_t {
    key = (!key).wrapping_add(key << 21 as libc::c_int) & mask;
    key = key ^ key >> 24 as libc::c_int;
    key = key.wrapping_add(key << 3 as libc::c_int).wrapping_add(key << 8 as libc::c_int)
        & mask;
    key = key ^ key >> 14 as libc::c_int;
    key = key.wrapping_add(key << 2 as libc::c_int).wrapping_add(key << 4 as libc::c_int)
        & mask;
    key = key ^ key >> 28 as libc::c_int;
    key = key.wrapping_add(key << 31 as libc::c_int) & mask;
    return key;
}
#[inline]
unsafe extern "C" fn tq_push(mut q: *mut tiny_queue_t, mut x: libc::c_int) {
    let fresh0 = (*q).count;
    (*q).count = (*q).count + 1;
    (*q).a[(fresh0 + (*q).front & 0x1f as libc::c_int) as usize] = x;
}
#[inline]
unsafe extern "C" fn tq_shift(mut q: *mut tiny_queue_t) -> libc::c_int {
    let mut x: libc::c_int = 0;
    if (*q).count == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let fresh1 = (*q).front;
    (*q).front = (*q).front + 1;
    x = (*q).a[fresh1 as usize];
    (*q).front &= 0x1f as libc::c_int;
    (*q).count -= 1;
    (*q).count;
    return x;
}
pub unsafe extern "C" fn mm_sketch(
    mut km: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut rid: uint32_t,
    mut is_hpc: libc::c_int,
    mut p: *mut mm128_v,
) {
    let mut shift1: uint64_t = (2 as libc::c_int * (k - 1 as libc::c_int)) as uint64_t;
    let mut mask: uint64_t = ((1 as libc::c_ulonglong) << 2 as libc::c_int * k)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut kmer: [uint64_t; 2] = [
        0 as libc::c_int as uint64_t,
        0 as libc::c_int as uint64_t,
    ];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut buf_pos: libc::c_int = 0;
    let mut min_pos: libc::c_int = 0;
    let mut kmer_span: libc::c_int = 0 as libc::c_int;
    let mut buf: [mm128_t; 256] = [mm128_t { x: 0, y: 0 }; 256];
    let mut min: mm128_t = {
        let mut init = mm128_t {
            x: 18446744073709551615 as libc::c_ulong,
            y: 18446744073709551615 as libc::c_ulong,
        };
        init
    };
    let mut tq: tiny_queue_t = tiny_queue_t {
        front: 0,
        count: 0,
        a: [0; 32],
    };
    if len > 0 as libc::c_int && (w > 0 as libc::c_int && w < 256 as libc::c_int)
        && (k > 0 as libc::c_int && k <= 28 as libc::c_int)
    {} else {
        __assert_fail(
            b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0" as *const u8
                as *const libc::c_char,
            b"sketch.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void mm_sketch(void *, const char *, int, int, int, uint32_t, int, mm128_v *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5533: {
        if len > 0 as libc::c_int && (w > 0 as libc::c_int && w < 256 as libc::c_int)
            && (k > 0 as libc::c_int && k <= 28 as libc::c_int)
        {} else {
            __assert_fail(
                b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0" as *const u8
                    as *const libc::c_char,
                b"sketch.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void mm_sketch(void *, const char *, int, int, int, uint32_t, int, mm128_v *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        (w * 16 as libc::c_int) as libc::c_ulong,
    );
    memset(
        &mut tq as *mut tiny_queue_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tiny_queue_t>() as libc::c_ulong,
    );
    if (*p).m < ((*p).n).wrapping_add((len / w) as libc::c_ulong) {
        (*p).m = ((*p).n).wrapping_add((len / w) as libc::c_ulong);
        (*p).m = ((*p).m).wrapping_sub(1);
        (*p).m;
        (*p).m |= (*p).m >> 1 as libc::c_int;
        (*p).m |= (*p).m >> 2 as libc::c_int;
        (*p).m |= (*p).m >> 4 as libc::c_int;
        (*p).m |= (*p).m >> 8 as libc::c_int;
        (*p).m |= (*p).m >> 16 as libc::c_int;
        (*p).m = ((*p).m).wrapping_add(1);
        (*p).m;
        (*p)
            .a = krealloc(
            km,
            (*p).a as *mut libc::c_void,
            (::std::mem::size_of::<mm128_t>() as libc::c_ulong).wrapping_mul((*p).m),
        ) as *mut mm128_t;
    }
    let mut current_block_107: u64;
    min_pos = 0 as libc::c_int;
    buf_pos = min_pos;
    l = buf_pos;
    i = l;
    while i < len {
        let mut c: libc::c_int = seq_nt4_table[*str.offset(i as isize) as uint8_t
            as usize] as libc::c_int;
        let mut info: mm128_t = {
            let mut init = mm128_t {
                x: 18446744073709551615 as libc::c_ulong,
                y: 18446744073709551615 as libc::c_ulong,
            };
            init
        };
        if c < 4 as libc::c_int {
            let mut z: libc::c_int = 0;
            if is_hpc != 0 {
                let mut skip_len: libc::c_int = 1 as libc::c_int;
                if (i + 1 as libc::c_int) < len
                    && seq_nt4_table[*str.offset((i + 1 as libc::c_int) as isize)
                        as uint8_t as usize] as libc::c_int == c
                {
                    skip_len = 2 as libc::c_int;
                    while i + skip_len < len {
                        if seq_nt4_table[*str.offset((i + skip_len) as isize) as uint8_t
                            as usize] as libc::c_int != c
                        {
                            break;
                        }
                        skip_len += 1;
                        skip_len;
                    }
                    i += skip_len - 1 as libc::c_int;
                }
                tq_push(&mut tq, skip_len);
                kmer_span += skip_len;
                if tq.count > k {
                    kmer_span -= tq_shift(&mut tq);
                }
            } else {
                kmer_span = if (l + 1 as libc::c_int) < k {
                    l + 1 as libc::c_int
                } else {
                    k
                };
            }
            kmer[0 as libc::c_int
                as usize] = (kmer[0 as libc::c_int as usize] << 2 as libc::c_int
                | c as libc::c_ulong) & mask;
            kmer[1 as libc::c_int
                as usize] = ((kmer[1 as libc::c_int as usize] >> 2 as libc::c_int)
                as libc::c_ulonglong
                | (3 as libc::c_ulonglong ^ c as libc::c_ulonglong) << shift1)
                as uint64_t;
            if kmer[0 as libc::c_int as usize] == kmer[1 as libc::c_int as usize] {
                current_block_107 = 1394248824506584008;
            } else {
                z = if kmer[0 as libc::c_int as usize] < kmer[1 as libc::c_int as usize]
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                l += 1;
                l;
                if l >= k && kmer_span < 256 as libc::c_int {
                    info
                        .x = hash64(kmer[z as usize], mask) << 8 as libc::c_int
                        | kmer_span as libc::c_ulong;
                    info
                        .y = (rid as uint64_t) << 32 as libc::c_int
                        | ((i as uint32_t) << 1 as libc::c_int) as libc::c_ulong
                        | z as libc::c_ulong;
                }
                current_block_107 = 5494826135382683477;
            }
        } else {
            l = 0 as libc::c_int;
            tq.front = 0 as libc::c_int;
            tq.count = tq.front;
            kmer_span = 0 as libc::c_int;
            current_block_107 = 5494826135382683477;
        }
        match current_block_107 {
            5494826135382683477 => {
                buf[buf_pos as usize] = info;
                if l == w + k - 1 as libc::c_int
                    && min.x != 18446744073709551615 as libc::c_ulong
                {
                    j = buf_pos + 1 as libc::c_int;
                    while j < w {
                        if min.x == buf[j as usize].x && buf[j as usize].y != min.y {
                            if (*p).n == (*p).m {
                                (*p)
                                    .m = if (*p).m != 0 {
                                    (*p).m << 1 as libc::c_int
                                } else {
                                    2 as libc::c_int as libc::c_ulong
                                };
                                (*p)
                                    .a = krealloc(
                                    km,
                                    (*p).a as *mut libc::c_void,
                                    (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                        .wrapping_mul((*p).m),
                                ) as *mut mm128_t;
                            }
                            let fresh2 = (*p).n;
                            (*p).n = ((*p).n).wrapping_add(1);
                            *((*p).a).offset(fresh2 as isize) = buf[j as usize];
                        }
                        j += 1;
                        j;
                    }
                    j = 0 as libc::c_int;
                    while j < buf_pos {
                        if min.x == buf[j as usize].x && buf[j as usize].y != min.y {
                            if (*p).n == (*p).m {
                                (*p)
                                    .m = if (*p).m != 0 {
                                    (*p).m << 1 as libc::c_int
                                } else {
                                    2 as libc::c_int as libc::c_ulong
                                };
                                (*p)
                                    .a = krealloc(
                                    km,
                                    (*p).a as *mut libc::c_void,
                                    (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                        .wrapping_mul((*p).m),
                                ) as *mut mm128_t;
                            }
                            let fresh3 = (*p).n;
                            (*p).n = ((*p).n).wrapping_add(1);
                            *((*p).a).offset(fresh3 as isize) = buf[j as usize];
                        }
                        j += 1;
                        j;
                    }
                }
                if info.x <= min.x {
                    if l >= w + k && min.x != 18446744073709551615 as libc::c_ulong {
                        if (*p).n == (*p).m {
                            (*p)
                                .m = if (*p).m != 0 {
                                (*p).m << 1 as libc::c_int
                            } else {
                                2 as libc::c_int as libc::c_ulong
                            };
                            (*p)
                                .a = krealloc(
                                km,
                                (*p).a as *mut libc::c_void,
                                (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                    .wrapping_mul((*p).m),
                            ) as *mut mm128_t;
                        }
                        let fresh4 = (*p).n;
                        (*p).n = ((*p).n).wrapping_add(1);
                        *((*p).a).offset(fresh4 as isize) = min;
                    }
                    min = info;
                    min_pos = buf_pos;
                } else if buf_pos == min_pos {
                    if l >= w + k - 1 as libc::c_int
                        && min.x != 18446744073709551615 as libc::c_ulong
                    {
                        if (*p).n == (*p).m {
                            (*p)
                                .m = if (*p).m != 0 {
                                (*p).m << 1 as libc::c_int
                            } else {
                                2 as libc::c_int as libc::c_ulong
                            };
                            (*p)
                                .a = krealloc(
                                km,
                                (*p).a as *mut libc::c_void,
                                (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                    .wrapping_mul((*p).m),
                            ) as *mut mm128_t;
                        }
                        let fresh5 = (*p).n;
                        (*p).n = ((*p).n).wrapping_add(1);
                        *((*p).a).offset(fresh5 as isize) = min;
                    }
                    j = buf_pos + 1 as libc::c_int;
                    min.x = 18446744073709551615 as libc::c_ulong;
                    while j < w {
                        if min.x >= buf[j as usize].x {
                            min = buf[j as usize];
                            min_pos = j;
                        }
                        j += 1;
                        j;
                    }
                    j = 0 as libc::c_int;
                    while j <= buf_pos {
                        if min.x >= buf[j as usize].x {
                            min = buf[j as usize];
                            min_pos = j;
                        }
                        j += 1;
                        j;
                    }
                    if l >= w + k - 1 as libc::c_int
                        && min.x != 18446744073709551615 as libc::c_ulong
                    {
                        j = buf_pos + 1 as libc::c_int;
                        while j < w {
                            if min.x == buf[j as usize].x && min.y != buf[j as usize].y {
                                if (*p).n == (*p).m {
                                    (*p)
                                        .m = if (*p).m != 0 {
                                        (*p).m << 1 as libc::c_int
                                    } else {
                                        2 as libc::c_int as libc::c_ulong
                                    };
                                    (*p)
                                        .a = krealloc(
                                        km,
                                        (*p).a as *mut libc::c_void,
                                        (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                            .wrapping_mul((*p).m),
                                    ) as *mut mm128_t;
                                }
                                let fresh6 = (*p).n;
                                (*p).n = ((*p).n).wrapping_add(1);
                                *((*p).a).offset(fresh6 as isize) = buf[j as usize];
                            }
                            j += 1;
                            j;
                        }
                        j = 0 as libc::c_int;
                        while j <= buf_pos {
                            if min.x == buf[j as usize].x && min.y != buf[j as usize].y {
                                if (*p).n == (*p).m {
                                    (*p)
                                        .m = if (*p).m != 0 {
                                        (*p).m << 1 as libc::c_int
                                    } else {
                                        2 as libc::c_int as libc::c_ulong
                                    };
                                    (*p)
                                        .a = krealloc(
                                        km,
                                        (*p).a as *mut libc::c_void,
                                        (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                            .wrapping_mul((*p).m),
                                    ) as *mut mm128_t;
                                }
                                let fresh7 = (*p).n;
                                (*p).n = ((*p).n).wrapping_add(1);
                                *((*p).a).offset(fresh7 as isize) = buf[j as usize];
                            }
                            j += 1;
                            j;
                        }
                    }
                }
                buf_pos += 1;
                if buf_pos == w {
                    buf_pos = 0 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if min.x != 18446744073709551615 as libc::c_ulong {
        if (*p).n == (*p).m {
            (*p)
                .m = if (*p).m != 0 {
                (*p).m << 1 as libc::c_int
            } else {
                2 as libc::c_int as libc::c_ulong
            };
            (*p)
                .a = krealloc(
                km,
                (*p).a as *mut libc::c_void,
                (::std::mem::size_of::<mm128_t>() as libc::c_ulong).wrapping_mul((*p).m),
            ) as *mut mm128_t;
        }
        let fresh8 = (*p).n;
        (*p).n = ((*p).n).wrapping_add(1);
        *((*p).a).offset(fresh8 as isize) = min;
    }
}

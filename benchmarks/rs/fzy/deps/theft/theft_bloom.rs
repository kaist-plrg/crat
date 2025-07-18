use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn theft_hash_onepass(data: *mut uint8_t, bytes: size_t) -> theft_hash;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type theft_hash = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_bloom {
    pub bit_count: uint8_t,
    pub size: size_t,
    pub bits: [uint8_t; 0],
}
pub unsafe extern "C" fn theft_bloom_init(mut bit_size2: uint8_t) -> *mut theft_bloom {
    let mut sz: size_t = ((1 as libc::c_int)
        << bit_size2 as libc::c_int - 3 as libc::c_int) as size_t;
    let mut b: *mut theft_bloom = malloc(
        (::std::mem::size_of::<theft_bloom>() as libc::c_ulong).wrapping_add(sz),
    ) as *mut theft_bloom;
    if !b.is_null() {
        (*b).size = sz;
        (*b).bit_count = bit_size2;
        memset(((*b).bits).as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, sz);
    }
    return b;
}
pub unsafe extern "C" fn theft_bloom_mark(
    mut b: *mut theft_bloom,
    mut data: *mut uint8_t,
    mut data_size: size_t,
) {
    let mut hash: uint64_t = theft_hash_onepass(data, data_size);
    let mut bc: uint8_t = (*b).bit_count;
    let mut mask: uint64_t = (((1 as libc::c_int) << bc as libc::c_int)
        - 1 as libc::c_int) as uint64_t;
    let mut bit_inc: libc::c_int = (64 as libc::c_int - bc as libc::c_int)
        / 4 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int - bc as libc::c_int {
        let mut v: uint64_t = (hash & mask << i) >> i;
        let mut offset: size_t = v.wrapping_div(8 as libc::c_int as libc::c_ulong);
        let mut bit: uint8_t = ((1 as libc::c_int)
            << (v & 0x7 as libc::c_int as libc::c_ulong)) as uint8_t;
        let ref mut fresh0 = *((*b).bits).as_mut_ptr().offset(offset as isize);
        *fresh0 = (*fresh0 as libc::c_int | bit as libc::c_int) as uint8_t;
        i += bit_inc;
    }
}
pub unsafe extern "C" fn theft_bloom_check(
    mut b: *mut theft_bloom,
    mut data: *mut uint8_t,
    mut data_size: size_t,
) -> bool {
    let mut hash: uint64_t = theft_hash_onepass(data, data_size);
    let mut bc: uint8_t = (*b).bit_count;
    let mut mask: uint64_t = (((1 as libc::c_int) << bc as libc::c_int)
        - 1 as libc::c_int) as uint64_t;
    let mut bit_inc: libc::c_int = (64 as libc::c_int - bc as libc::c_int)
        / 4 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int - bc as libc::c_int {
        let mut v: uint64_t = (hash & mask << i) >> i;
        let mut offset: size_t = v.wrapping_div(8 as libc::c_int as libc::c_ulong);
        let mut bit: uint8_t = ((1 as libc::c_int)
            << (v & 0x7 as libc::c_int as libc::c_ulong)) as uint8_t;
        if 0 as libc::c_int
            == *((*b).bits).as_mut_ptr().offset(offset as isize) as libc::c_int
                & bit as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        i += bit_inc;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn theft_bloom_free(mut b: *mut theft_bloom) {
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn theft_bloom_dump(mut b: *mut theft_bloom) {
    let mut counts: [uint8_t; 256] = [0; 256];
    memset(
        counts.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 256]>() as libc::c_ulong,
    );
    let mut total: size_t = 0 as libc::c_int as size_t;
    let mut row_total: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*b).size {
        let mut count: uint8_t = get_bits_set_count(
            counts.as_mut_ptr(),
            *((*b).bits).as_mut_ptr().offset(i as isize),
        );
        total = (total as libc::c_ulong).wrapping_add(count as libc::c_ulong) as size_t
            as size_t;
        row_total = (row_total as libc::c_int + count as libc::c_int) as uint16_t;
        i = i.wrapping_add(1);
        i;
    }
    if total > (*b).size {
        fprintf(
            stderr,
            b"\nWARNING: bloom filter is %zd%% full, larger bloom_bits value recommended.\n\0"
                as *const u8 as *const libc::c_char,
            (100 as libc::c_int as libc::c_ulong)
                .wrapping_mul(total)
                .wrapping_div(
                    (8 as libc::c_int as libc::c_ulong).wrapping_mul((*b).size),
                ),
        );
    }
}
pub unsafe extern "C" fn theft_bloom_recommendation(mut trials: libc::c_int) -> uint8_t {
    let mut res: uint8_t = 17 as libc::c_int as uint8_t;
    let min: uint8_t = (13 as libc::c_int - 3 as libc::c_int) as uint8_t;
    let max: uint8_t = (33 as libc::c_int - 3 as libc::c_int) as uint8_t;
    let mut i: uint8_t = min;
    while (i as libc::c_int) < max as libc::c_int {
        let mut v: int32_t = (1 as libc::c_int) << i as libc::c_int;
        if v > 14 as libc::c_int * trials {
            res = (i as libc::c_int + 3 as libc::c_int) as uint8_t;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    return res;
}
unsafe extern "C" fn get_bits_set_count(
    mut counts: *mut uint8_t,
    mut byte: uint8_t,
) -> uint8_t {
    let mut v: uint8_t = *counts.offset(byte as isize);
    if v as libc::c_int != 0xff as libc::c_int {
        return v;
    }
    let mut t: uint8_t = 0 as libc::c_int as uint8_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < 8 as libc::c_int {
        if byte as libc::c_int & (1 as libc::c_int) << i as libc::c_int != 0 {
            t = t.wrapping_add(1);
            t;
        }
        i = i.wrapping_add(1);
        i;
    }
    *counts.offset(byte as isize) = t;
    return t;
}

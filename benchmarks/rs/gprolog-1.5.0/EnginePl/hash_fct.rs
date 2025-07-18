use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashIncrInfo {
    pub len: libc::c_int,
    pub hash: uint32_t,
}
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
#[inline(always)]
unsafe extern "C" fn ROTL32(mut x: uint32_t, mut r: int8_t) -> uint32_t {
    return x << r as libc::c_int | x >> 32 as libc::c_int - r as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn Hash_Initialize() -> uint32_t {
    return 1688943522 as libc::c_int as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn Hash_Block(mut k1: uint32_t, mut h1: uint32_t) -> uint32_t {
    let c1: uint32_t = 0xcc9e2d51 as libc::c_uint;
    let c2: uint32_t = 0x1b873593 as libc::c_int as uint32_t;
    k1 = (k1 as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
    k1 = ROTL32(k1, 15 as libc::c_int as int8_t);
    k1 = (k1 as libc::c_uint).wrapping_mul(c2) as uint32_t as uint32_t;
    h1 ^= k1;
    h1 = ROTL32(h1, 13 as libc::c_int as int8_t);
    h1 = h1
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    return h1;
}
#[inline(always)]
unsafe extern "C" fn Hash_Finalize(mut h: uint32_t, mut len: libc::c_int) -> uint32_t {
    h ^= len as libc::c_uint;
    h ^= h >> 16 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(0x85ebca6b as libc::c_uint) as uint32_t
        as uint32_t;
    h ^= h >> 13 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(0xc2b2ae35 as libc::c_uint) as uint32_t
        as uint32_t;
    h ^= h >> 16 as libc::c_int;
    return h;
}
unsafe extern "C" fn Hash_Buffer_Aligned(
    mut key: *const libc::c_void,
    mut len: libc::c_int,
    mut seed: uint32_t,
) -> uint32_t {
    let mut data: *mut uint8_t = key as *mut uint8_t;
    let mut limit_block: *const uint8_t = data
        .offset(len as isize)
        .offset(-(4 as libc::c_int as isize));
    let c1: uint32_t = 0xcc9e2d51 as libc::c_uint;
    let c2: uint32_t = 0x1b873593 as libc::c_int as uint32_t;
    let mut h1: uint32_t = seed;
    let mut k1: uint32_t = 0;
    while data <= limit_block as *mut uint8_t {
        k1 = *(data as *mut uint32_t);
        data = data.offset(4 as libc::c_int as isize);
        h1 = Hash_Block(k1, h1);
    }
    k1 = 0 as libc::c_int as uint32_t;
    let mut current_block_12: u64;
    match len & 3 as libc::c_int {
        3 => {
            k1
                ^= ((*data.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint;
            current_block_12 = 7387201502321997221;
        }
        2 => {
            current_block_12 = 7387201502321997221;
        }
        1 => {
            current_block_12 = 111797835255413507;
        }
        _ => {
            current_block_12 = 13586036798005543211;
        }
    }
    match current_block_12 {
        7387201502321997221 => {
            k1
                ^= ((*data.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint;
            current_block_12 = 111797835255413507;
        }
        _ => {}
    }
    match current_block_12 {
        111797835255413507 => {
            k1 ^= *data.offset(0 as libc::c_int as isize) as libc::c_uint;
            k1 = (k1 as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
            k1 = ROTL32(k1, 15 as libc::c_int as int8_t);
            k1 = (k1 as libc::c_uint).wrapping_mul(c2) as uint32_t as uint32_t;
            h1 ^= k1;
        }
        _ => {}
    }
    return h1;
}
unsafe extern "C" fn Hash_Buffer_Unaligned(
    mut key: *const libc::c_void,
    mut len: libc::c_int,
    mut seed: uint32_t,
) -> uint32_t {
    let mut data: *mut uint8_t = key as *mut uint8_t;
    let mut limit_block: *const uint8_t = data
        .offset(len as isize)
        .offset(-(4 as libc::c_int as isize));
    let c1: uint32_t = 0xcc9e2d51 as libc::c_uint;
    let c2: uint32_t = 0x1b873593 as libc::c_int as uint32_t;
    let mut h1: uint32_t = seed;
    let mut k1: uint32_t = 0;
    while data <= limit_block as *mut uint8_t {
        memcpy(
            &mut k1 as *mut uint32_t as *mut libc::c_void,
            data as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        data = data.offset(4 as libc::c_int as isize);
        h1 = Hash_Block(k1, h1);
    }
    k1 = 0 as libc::c_int as uint32_t;
    let mut current_block_12: u64;
    match len & 3 as libc::c_int {
        3 => {
            k1
                ^= ((*data.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint;
            current_block_12 = 16788951267886524043;
        }
        2 => {
            current_block_12 = 16788951267886524043;
        }
        1 => {
            current_block_12 = 7851151787501427912;
        }
        _ => {
            current_block_12 = 13586036798005543211;
        }
    }
    match current_block_12 {
        16788951267886524043 => {
            k1
                ^= ((*data.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint;
            current_block_12 = 7851151787501427912;
        }
        _ => {}
    }
    match current_block_12 {
        7851151787501427912 => {
            k1 ^= *data.offset(0 as libc::c_int as isize) as libc::c_uint;
            k1 = (k1 as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
            k1 = ROTL32(k1, 15 as libc::c_int as int8_t);
            k1 = (k1 as libc::c_uint).wrapping_mul(c2) as uint32_t as uint32_t;
            h1 ^= k1;
        }
        _ => {}
    }
    return h1;
}
pub unsafe extern "C" fn Hash_Buffer(
    mut key: *const libc::c_void,
    mut len: libc::c_int,
    mut seed: uint32_t,
) -> uint32_t {
    if key as uintptr_t & 3 as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        return Hash_Buffer_Aligned(key, len, seed);
    }
    return Hash_Buffer_Unaligned(key, len, seed);
}
pub unsafe extern "C" fn Pl_Hash_Buffer(
    mut data: *const libc::c_void,
    mut len: libc::c_int,
) -> uint32_t {
    let mut hash: uint32_t = Hash_Initialize();
    hash = Hash_Buffer(data, len, hash);
    hash = Hash_Finalize(hash, len);
    return hash;
}
pub unsafe extern "C" fn Pl_Hash_Incr_Init(mut hi: *mut HashIncrInfo) {
    (*hi).len = 0 as libc::c_int;
    (*hi).hash = Hash_Initialize();
}
pub unsafe extern "C" fn Pl_Hash_Incr_Buffer(
    mut hi: *mut HashIncrInfo,
    mut data: *const libc::c_void,
    mut len: libc::c_int,
) {
    (*hi).len += len;
    (*hi).hash = Hash_Buffer(data, len, (*hi).hash);
}
pub unsafe extern "C" fn Pl_Hash_Incr_Int32(mut hi: *mut HashIncrInfo, mut x: uint32_t) {
    (*hi).len += 4 as libc::c_int;
    (*hi).hash = Hash_Block(__uint32_identity(x), (*hi).hash);
}
pub unsafe extern "C" fn Pl_Hash_Incr_Int64(mut hi: *mut HashIncrInfo, mut x: uint64_t) {
    Pl_Hash_Incr_Int32(hi, x as uint32_t);
    Pl_Hash_Incr_Int32(hi, (x >> 32 as libc::c_int) as uint32_t);
}
pub unsafe extern "C" fn Pl_Hash_Incr_Double(
    mut hi: *mut HashIncrInfo,
    mut x: libc::c_double,
) {
    let mut exp: libc::c_int = 0;
    let mut man64: uint64_t = 0;
    let mut rest: uint64_t = 0;
    x = frexp(x, &mut exp);
    if x < 0.0f64 {
        x = -(x + 0.5f64);
    }
    x *= ((1 as libc::c_int as uint64_t) << 63 as libc::c_int) as libc::c_double;
    man64 = x as int64_t as uint64_t;
    rest = 0 as libc::c_int as uint64_t;
    man64 = man64
        .wrapping_add(rest)
        .wrapping_add(
            ((1 as libc::c_int as uint64_t) << 63 as libc::c_int)
                .wrapping_div(1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(exp as libc::c_ulong),
        );
    Pl_Hash_Incr_Int64(hi, man64);
}
pub unsafe extern "C" fn Pl_Hash_Incr_Term(mut hi: *mut HashIncrInfo) -> uint32_t {
    (*hi).hash = Hash_Finalize((*hi).hash, (*hi).len);
    return (*hi).hash;
}

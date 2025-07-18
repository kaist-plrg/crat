use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_mt {
    pub mt: [uint64_t; 312],
    pub mti: int16_t,
}
pub unsafe extern "C" fn theft_mt_init(mut seed: uint64_t) -> *mut theft_mt {
    let mut mt: *mut theft_mt = malloc(
        ::std::mem::size_of::<theft_mt>() as libc::c_ulong,
    ) as *mut theft_mt;
    if mt.is_null() {
        return 0 as *mut theft_mt;
    }
    theft_mt_reset(mt, seed);
    return mt;
}
pub unsafe extern "C" fn theft_mt_free(mut mt: *mut theft_mt) {
    free(mt as *mut libc::c_void);
}
pub unsafe extern "C" fn theft_mt_reset(mut mt: *mut theft_mt, mut seed: uint64_t) {
    (*mt).mt[0 as libc::c_int as usize] = seed;
    let mut mti: uint16_t = 0 as libc::c_int as uint16_t;
    mti = 1 as libc::c_int as uint16_t;
    while (mti as libc::c_int) < 312 as libc::c_int {
        (*mt)
            .mt[mti
            as usize] = (6364136223846793005 as libc::c_ulonglong)
            .wrapping_mul(
                ((*mt).mt[(mti as libc::c_int - 1 as libc::c_int) as usize]
                    ^ (*mt).mt[(mti as libc::c_int - 1 as libc::c_int) as usize]
                        >> 62 as libc::c_int) as libc::c_ulonglong,
            )
            .wrapping_add(mti as libc::c_ulonglong) as uint64_t;
        mti = mti.wrapping_add(1);
        mti;
    }
    (*mt).mti = mti as int16_t;
}
pub unsafe extern "C" fn theft_mt_random(mut mt: *mut theft_mt) -> uint64_t {
    return genrand64_int64(mt);
}
pub unsafe extern "C" fn theft_mt_random_double(
    mut mt: *mut theft_mt,
) -> libc::c_double {
    return (genrand64_int64(mt) >> 11 as libc::c_int) as libc::c_double
        * (1.0f64 / 9007199254740991.0f64);
}
unsafe extern "C" fn genrand64_int64(mut r: *mut theft_mt) -> uint64_t {
    let mut i: libc::c_int = 0;
    let mut x: uint64_t = 0;
    static mut mag01: [uint64_t; 2] = [
        0 as libc::c_ulonglong as uint64_t,
        0xb5026f5aa96619e9 as libc::c_ulonglong as uint64_t,
    ];
    if (*r).mti as libc::c_int >= 312 as libc::c_int {
        if (*r).mti as libc::c_int == 312 as libc::c_int + 1 as libc::c_int {
            theft_mt_reset(r, 5489 as libc::c_ulonglong as uint64_t);
        }
        i = 0 as libc::c_int;
        while i < 312 as libc::c_int - 156 as libc::c_int {
            x = ((*r).mt[i as usize] as libc::c_ulonglong
                & 0xffffffff80000000 as libc::c_ulonglong
                | (*r).mt[(i + 1 as libc::c_int) as usize] as libc::c_ulonglong
                    & 0x7fffffff as libc::c_ulonglong) as uint64_t;
            (*r)
                .mt[i
                as usize] = (*r).mt[(i + 156 as libc::c_int) as usize]
                ^ x >> 1 as libc::c_int
                ^ mag01[(x as libc::c_ulonglong & 1 as libc::c_ulonglong) as libc::c_int
                    as usize];
            i += 1;
            i;
        }
        while i < 312 as libc::c_int - 1 as libc::c_int {
            x = ((*r).mt[i as usize] as libc::c_ulonglong
                & 0xffffffff80000000 as libc::c_ulonglong
                | (*r).mt[(i + 1 as libc::c_int) as usize] as libc::c_ulonglong
                    & 0x7fffffff as libc::c_ulonglong) as uint64_t;
            (*r)
                .mt[i
                as usize] = (*r)
                .mt[(i + (156 as libc::c_int - 312 as libc::c_int)) as usize]
                ^ x >> 1 as libc::c_int
                ^ mag01[(x as libc::c_ulonglong & 1 as libc::c_ulonglong) as libc::c_int
                    as usize];
            i += 1;
            i;
        }
        x = ((*r).mt[(312 as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_ulonglong & 0xffffffff80000000 as libc::c_ulonglong
            | (*r).mt[0 as libc::c_int as usize] as libc::c_ulonglong
                & 0x7fffffff as libc::c_ulonglong) as uint64_t;
        (*r)
            .mt[(312 as libc::c_int - 1 as libc::c_int)
            as usize] = (*r).mt[(156 as libc::c_int - 1 as libc::c_int) as usize]
            ^ x >> 1 as libc::c_int
            ^ mag01[(x as libc::c_ulonglong & 1 as libc::c_ulonglong) as libc::c_int
                as usize];
        (*r).mti = 0 as libc::c_int as int16_t;
    }
    let fresh0 = (*r).mti;
    (*r).mti = (*r).mti + 1;
    x = (*r).mt[fresh0 as usize];
    x = (x as libc::c_ulonglong
        ^ (x >> 29 as libc::c_int) as libc::c_ulonglong
            & 0x5555555555555555 as libc::c_ulonglong) as uint64_t;
    x = (x as libc::c_ulonglong
        ^ (x << 17 as libc::c_int) as libc::c_ulonglong
            & 0x71d67fffeda60000 as libc::c_ulonglong) as uint64_t;
    x = (x as libc::c_ulonglong
        ^ (x << 37 as libc::c_int) as libc::c_ulonglong
            & 0xfff7eee000000000 as libc::c_ulonglong) as uint64_t;
    x ^= x >> 43 as libc::c_int;
    return x;
}

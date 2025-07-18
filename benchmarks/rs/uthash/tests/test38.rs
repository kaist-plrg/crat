use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *const libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
    pub bloom_sig: uint32_t,
    pub bloom_bv: *mut uint8_t,
    pub bloom_nbits: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_t {
    pub a: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut tests: *mut test_t = 0 as *mut test_t;
    let mut test: *mut test_t = 0 as *mut test_t;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    while b < 3 as libc::c_int {
        a = 0 as libc::c_int;
        while a < 10 as libc::c_int {
            test = 0 as *mut test_t;
            test = 0 as *mut test_t;
            if !tests.is_null() {
                let mut _hf_hashv: libc::c_uint = 0;
                let mut _hj_i: libc::c_uint = 0;
                let mut _hj_j: libc::c_uint = 0;
                let mut _hj_k: libc::c_uint = 0;
                let mut _hj_key: *const libc::c_uchar = &mut a as *mut libc::c_int
                    as *const libc::c_uchar;
                _hf_hashv = 0xfeedbeef as libc::c_uint;
                _hj_j = 0x9e3779b9 as libc::c_uint;
                _hj_i = _hj_j;
                _hj_k = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    as libc::c_uint;
                while _hj_k >= 12 as libc::c_uint {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                        << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                        << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                        << 24 as libc::c_int,
                                ),
                        );
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                        << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                        << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                        << 24 as libc::c_int,
                                ),
                        );
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                        << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key.offset(10 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key.offset(11 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 13 as libc::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 8 as libc::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 13 as libc::c_int;
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 12 as libc::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 16 as libc::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 5 as libc::c_int;
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 3 as libc::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 10 as libc::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 15 as libc::c_int;
                    _hj_key = _hj_key.offset(12 as libc::c_int as isize);
                    _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
                }
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_uint,
                    );
                let mut current_block_55: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv
                            .wrapping_add(
                                (*_hj_key.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_55 = 16496499871998433688;
                    }
                    10 => {
                        current_block_55 = 16496499871998433688;
                    }
                    9 => {
                        current_block_55 = 17181025359314527317;
                    }
                    8 => {
                        current_block_55 = 12972549306614970870;
                    }
                    7 => {
                        current_block_55 = 11837076978836381723;
                    }
                    6 => {
                        current_block_55 = 13205855207806107180;
                    }
                    5 => {
                        current_block_55 = 2396836420025011240;
                    }
                    4 => {
                        current_block_55 = 7255765040852174736;
                    }
                    3 => {
                        current_block_55 = 13962048922322332216;
                    }
                    2 => {
                        current_block_55 = 18112195297577013772;
                    }
                    1 => {
                        current_block_55 = 14554460264371529240;
                    }
                    _ => {
                        current_block_55 = 2989495919056355252;
                    }
                }
                match current_block_55 {
                    16496499871998433688 => {
                        _hf_hashv = _hf_hashv
                            .wrapping_add(
                                (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            );
                        current_block_55 = 17181025359314527317;
                    }
                    _ => {}
                }
                match current_block_55 {
                    17181025359314527317 => {
                        _hf_hashv = _hf_hashv
                            .wrapping_add(
                                (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            );
                        current_block_55 = 12972549306614970870;
                    }
                    _ => {}
                }
                match current_block_55 {
                    12972549306614970870 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            );
                        current_block_55 = 11837076978836381723;
                    }
                    _ => {}
                }
                match current_block_55 {
                    11837076978836381723 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            );
                        current_block_55 = 13205855207806107180;
                    }
                    _ => {}
                }
                match current_block_55 {
                    13205855207806107180 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            );
                        current_block_55 = 2396836420025011240;
                    }
                    _ => {}
                }
                match current_block_55 {
                    2396836420025011240 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                            );
                        current_block_55 = 7255765040852174736;
                    }
                    _ => {}
                }
                match current_block_55 {
                    7255765040852174736 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            );
                        current_block_55 = 13962048922322332216;
                    }
                    _ => {}
                }
                match current_block_55 {
                    13962048922322332216 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            );
                        current_block_55 = 18112195297577013772;
                    }
                    _ => {}
                }
                match current_block_55 {
                    18112195297577013772 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            );
                        current_block_55 = 14554460264371529240;
                    }
                    _ => {}
                }
                match current_block_55 {
                    14554460264371529240 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                            );
                    }
                    _ => {}
                }
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as libc::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as libc::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as libc::c_int;
                test = 0 as *mut test_t;
                if !tests.is_null() {
                    let mut _hf_bkt: libc::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & ((*(*tests).hh.tbl).num_buckets)
                            .wrapping_sub(1 as libc::c_uint);
                    if 1 as libc::c_int != 0 as libc::c_int {
                        if !((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt as isize))
                            .hh_head)
                            .is_null()
                        {
                            test = ((*((*(*tests).hh.tbl).buckets)
                                .offset(_hf_bkt as isize))
                                .hh_head as *mut libc::c_char)
                                .offset(-((*(*tests).hh.tbl).hho as isize))
                                as *mut libc::c_void as *mut test_t;
                        } else {
                            test = 0 as *mut test_t;
                        }
                        while !test.is_null() {
                            if (*test).hh.hashv == _hf_hashv
                                && (*test).hh.keylen as libc::c_ulong
                                    == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            {
                                if memcmp(
                                    (*test).hh.key,
                                    &mut a as *mut libc::c_int as *const libc::c_void,
                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                ) == 0 as libc::c_int
                                {
                                    break;
                                }
                            }
                            if !((*test).hh.hh_next).is_null() {
                                test = ((*test).hh.hh_next as *mut libc::c_char)
                                    .offset(-((*(*tests).hh.tbl).hho as isize))
                                    as *mut libc::c_void as *mut test_t;
                            } else {
                                test = 0 as *mut test_t;
                            }
                        }
                    }
                }
            }
            if test.is_null() {
                test = malloc(::std::mem::size_of::<test_t>() as libc::c_ulong)
                    as *mut test_t;
                if test.is_null() {
                    exit(-(1 as libc::c_int));
                }
                memset(
                    test as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<test_t>() as libc::c_ulong,
                );
                (*test).a = a;
                let mut _ha_hashv: libc::c_uint = 0;
                let mut _hj_i_0: libc::c_uint = 0;
                let mut _hj_j_0: libc::c_uint = 0;
                let mut _hj_k_0: libc::c_uint = 0;
                let mut _hj_key_0: *const libc::c_uchar = &mut (*test).a
                    as *mut libc::c_int as *const libc::c_uchar;
                _ha_hashv = 0xfeedbeef as libc::c_uint;
                _hj_j_0 = 0x9e3779b9 as libc::c_uint;
                _hj_i_0 = _hj_j_0;
                _hj_k_0 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    as libc::c_uint;
                while _hj_k_0 >= 12 as libc::c_uint {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(0 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_0.offset(1 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(2 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(3 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(4 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_0.offset(5 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(6 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(7 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(8 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_0.offset(9 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(10 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(11 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                    _hj_i_0 ^= _ha_hashv >> 13 as libc::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                    _ha_hashv ^= _hj_j_0 >> 13 as libc::c_int;
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                    _hj_i_0 ^= _ha_hashv >> 12 as libc::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                    _ha_hashv ^= _hj_j_0 >> 5 as libc::c_int;
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                    _hj_i_0 ^= _ha_hashv >> 3 as libc::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                    _ha_hashv ^= _hj_j_0 >> 15 as libc::c_int;
                    _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
                    _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
                }
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_uint,
                    );
                let mut current_block_178: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv
                            .wrapping_add(
                                (*_hj_key_0.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_178 = 3137761655869617204;
                    }
                    10 => {
                        current_block_178 = 3137761655869617204;
                    }
                    9 => {
                        current_block_178 = 11002582195031630784;
                    }
                    8 => {
                        current_block_178 = 3565449706180540421;
                    }
                    7 => {
                        current_block_178 = 13006631542393395751;
                    }
                    6 => {
                        current_block_178 = 8617283666136554728;
                    }
                    5 => {
                        current_block_178 = 1951311294146316411;
                    }
                    4 => {
                        current_block_178 = 10469605675374413955;
                    }
                    3 => {
                        current_block_178 = 9434444550647791986;
                    }
                    2 => {
                        current_block_178 = 16390215964003829843;
                    }
                    1 => {
                        current_block_178 = 418406577547333731;
                    }
                    _ => {
                        current_block_178 = 12099607619007264150;
                    }
                }
                match current_block_178 {
                    3137761655869617204 => {
                        _ha_hashv = _ha_hashv
                            .wrapping_add(
                                (*_hj_key_0.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_178 = 11002582195031630784;
                    }
                    _ => {}
                }
                match current_block_178 {
                    11002582195031630784 => {
                        _ha_hashv = _ha_hashv
                            .wrapping_add(
                                (*_hj_key_0.offset(8 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_178 = 3565449706180540421;
                    }
                    _ => {}
                }
                match current_block_178 {
                    3565449706180540421 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(
                                (*_hj_key_0.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_178 = 13006631542393395751;
                    }
                    _ => {}
                }
                match current_block_178 {
                    13006631542393395751 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(
                                (*_hj_key_0.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_178 = 8617283666136554728;
                    }
                    _ => {}
                }
                match current_block_178 {
                    8617283666136554728 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(
                                (*_hj_key_0.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_178 = 1951311294146316411;
                    }
                    _ => {}
                }
                match current_block_178 {
                    1951311294146316411 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(
                                *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                            );
                        current_block_178 = 10469605675374413955;
                    }
                    _ => {}
                }
                match current_block_178 {
                    10469605675374413955 => {
                        _hj_i_0 = _hj_i_0
                            .wrapping_add(
                                (*_hj_key_0.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_178 = 9434444550647791986;
                    }
                    _ => {}
                }
                match current_block_178 {
                    9434444550647791986 => {
                        _hj_i_0 = _hj_i_0
                            .wrapping_add(
                                (*_hj_key_0.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_178 = 16390215964003829843;
                    }
                    _ => {}
                }
                match current_block_178 {
                    16390215964003829843 => {
                        _hj_i_0 = _hj_i_0
                            .wrapping_add(
                                (*_hj_key_0.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_178 = 418406577547333731;
                    }
                    _ => {}
                }
                match current_block_178 {
                    418406577547333731 => {
                        _hj_i_0 = _hj_i_0
                            .wrapping_add(
                                *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                            );
                    }
                    _ => {}
                }
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 13 as libc::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 13 as libc::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 12 as libc::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 5 as libc::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 3 as libc::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 15 as libc::c_int;
                (*test).hh.hashv = _ha_hashv;
                (*test)
                    .hh
                    .key = &mut (*test).a as *mut libc::c_int as *const libc::c_void;
                (*test)
                    .hh
                    .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    as libc::c_uint;
                if tests.is_null() {
                    (*test).hh.next = 0 as *mut libc::c_void;
                    (*test).hh.prev = 0 as *mut libc::c_void;
                    (*test)
                        .hh
                        .tbl = malloc(
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    ) as *mut UT_hash_table;
                    if ((*test).hh.tbl).is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            (*test).hh.tbl as *mut libc::c_void,
                            '\0' as i32,
                            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                        );
                        (*(*test).hh.tbl).tail = &mut (*test).hh;
                        (*(*test).hh.tbl).num_buckets = 32 as libc::c_uint;
                        (*(*test).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                        (*(*test).hh.tbl)
                            .hho = (&mut (*test).hh as *mut UT_hash_handle
                            as *mut libc::c_char)
                            .offset_from(test as *mut libc::c_char) as libc::c_long;
                        (*(*test).hh.tbl)
                            .buckets = malloc(
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        ) as *mut UT_hash_bucket;
                        (*(*test).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                        if ((*(*test).hh.tbl).buckets).is_null() {
                            exit(-(1 as libc::c_int));
                        } else {
                            memset(
                                (*(*test).hh.tbl).buckets as *mut libc::c_void,
                                '\0' as i32,
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            );
                        }
                    }
                    tests = test;
                } else {
                    (*test).hh.tbl = (*tests).hh.tbl;
                    (*test).hh.next = 0 as *mut libc::c_void;
                    (*test)
                        .hh
                        .prev = ((*(*tests).hh.tbl).tail as *mut libc::c_char)
                        .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void;
                    (*(*(*tests).hh.tbl).tail).next = test as *mut libc::c_void;
                    (*(*tests).hh.tbl).tail = &mut (*test).hh;
                }
                let mut _ha_bkt: libc::c_uint = 0;
                (*(*tests).hh.tbl)
                    .num_items = ((*(*tests).hh.tbl).num_items).wrapping_add(1);
                (*(*tests).hh.tbl).num_items;
                _ha_bkt = _ha_hashv
                    & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*tests).hh.tbl)
                    .buckets)
                    .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
                (*_ha_head).count;
                (*test).hh.hh_next = (*_ha_head).hh_head;
                (*test).hh.hh_prev = 0 as *mut UT_hash_handle;
                if !((*_ha_head).hh_head).is_null() {
                    (*(*_ha_head).hh_head).hh_prev = &mut (*test).hh;
                }
                (*_ha_head).hh_head = &mut (*test).hh;
                if (*_ha_head).count
                    >= ((*_ha_head).expand_mult)
                        .wrapping_add(1 as libc::c_uint)
                        .wrapping_mul(10 as libc::c_uint)
                    && (*(*test).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: libc::c_uint = 0;
                    let mut _he_bkt_i: libc::c_uint = 0;
                    let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_new_buckets: *mut UT_hash_bucket = 0
                        as *mut UT_hash_bucket;
                    let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                    _he_new_buckets = malloc(
                        (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                            .wrapping_mul((*(*test).hh.tbl).num_buckets as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut libc::c_void,
                            '\0' as i32,
                            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                .wrapping_mul(
                                    (*(*test).hh.tbl).num_buckets as libc::c_ulong,
                                )
                                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                        );
                        (*(*test).hh.tbl)
                            .ideal_chain_maxlen = ((*(*test).hh.tbl).num_items
                            >> ((*(*test).hh.tbl).log2_num_buckets)
                                .wrapping_add(1 as libc::c_uint))
                            .wrapping_add(
                                (if (*(*test).hh.tbl).num_items
                                    & ((*(*test).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                                {
                                    1 as libc::c_uint
                                } else {
                                    0 as libc::c_uint
                                }),
                            );
                        (*(*test).hh.tbl)
                            .nonideal_items = 0 as libc::c_int as libc::c_uint;
                        _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                        while _he_bkt_i < (*(*test).hh.tbl).num_buckets {
                            _he_thh = (*((*(*test).hh.tbl).buckets)
                                .offset(_he_bkt_i as isize))
                                .hh_head;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & ((*(*test).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint);
                                _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                                    as *mut UT_hash_bucket;
                                (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                                if (*_he_newbkt).count
                                    > (*(*test).hh.tbl).ideal_chain_maxlen
                                {
                                    (*(*test).hh.tbl)
                                        .nonideal_items = ((*(*test).hh.tbl).nonideal_items)
                                        .wrapping_add(1);
                                    (*(*test).hh.tbl).nonideal_items;
                                    if (*_he_newbkt).count
                                        > ((*_he_newbkt).expand_mult)
                                            .wrapping_mul((*(*test).hh.tbl).ideal_chain_maxlen)
                                    {
                                        (*_he_newbkt)
                                            .expand_mult = ((*_he_newbkt).expand_mult).wrapping_add(1);
                                        (*_he_newbkt).expand_mult;
                                    }
                                }
                                (*_he_thh).hh_prev = 0 as *mut UT_hash_handle;
                                (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                                if !((*_he_newbkt).hh_head).is_null() {
                                    (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                                }
                                (*_he_newbkt).hh_head = _he_thh;
                                _he_thh = _he_hh_nxt;
                            }
                            _he_bkt_i = _he_bkt_i.wrapping_add(1);
                            _he_bkt_i;
                        }
                        free((*(*test).hh.tbl).buckets as *mut libc::c_void);
                        (*(*test).hh.tbl)
                            .num_buckets = ((*(*test).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint);
                        (*(*test).hh.tbl)
                            .log2_num_buckets = ((*(*test).hh.tbl).log2_num_buckets)
                            .wrapping_add(1);
                        (*(*test).hh.tbl).log2_num_buckets;
                        (*(*test).hh.tbl).buckets = _he_new_buckets;
                        (*(*test).hh.tbl)
                            .ineff_expands = if (*(*test).hh.tbl).nonideal_items
                            > (*(*test).hh.tbl).num_items >> 1 as libc::c_int
                        {
                            ((*(*test).hh.tbl).ineff_expands)
                                .wrapping_add(1 as libc::c_uint)
                        } else {
                            0 as libc::c_uint
                        };
                        if (*(*test).hh.tbl).ineff_expands > 1 as libc::c_uint {
                            (*(*test).hh.tbl)
                                .noexpand = 1 as libc::c_int as libc::c_uint;
                        }
                    }
                }
            }
            a += 1;
            a;
        }
        b += 1;
        b;
    }
    printf(
        b"hash count %u\n\0" as *const u8 as *const libc::c_char,
        if !tests.is_null() { (*(*tests).hh.tbl).num_items } else { 0 as libc::c_uint },
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

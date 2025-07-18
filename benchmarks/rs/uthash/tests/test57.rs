use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub struct el_t {
    pub key: *mut libc::c_void,
    pub i: libc::c_int,
    pub hh: UT_hash_handle,
}
pub unsafe extern "C" fn findit(
    mut hash: *mut el_t,
    mut keytofind: *mut libc::c_void,
) -> *mut el_t {
    let mut found: *mut el_t = 0 as *mut el_t;
    found = 0 as *mut el_t;
    if !hash.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut keytofind as *mut *mut libc::c_void
            as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
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
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
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
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    as libc::c_uint,
            );
        let mut current_block_52: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 6518094531753430203;
            }
            10 => {
                current_block_52 = 6518094531753430203;
            }
            9 => {
                current_block_52 = 6558758903416202777;
            }
            8 => {
                current_block_52 = 1034985688366755324;
            }
            7 => {
                current_block_52 = 1708148710342859856;
            }
            6 => {
                current_block_52 = 9047376510614419450;
            }
            5 => {
                current_block_52 = 17295929920418635907;
            }
            4 => {
                current_block_52 = 9113621750889162830;
            }
            3 => {
                current_block_52 = 14710078419898575087;
            }
            2 => {
                current_block_52 = 16220771416666260789;
            }
            1 => {
                current_block_52 = 15154740142284061113;
            }
            _ => {
                current_block_52 = 721385680381463314;
            }
        }
        match current_block_52 {
            6518094531753430203 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 6558758903416202777;
            }
            _ => {}
        }
        match current_block_52 {
            6558758903416202777 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 1034985688366755324;
            }
            _ => {}
        }
        match current_block_52 {
            1034985688366755324 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 1708148710342859856;
            }
            _ => {}
        }
        match current_block_52 {
            1708148710342859856 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 9047376510614419450;
            }
            _ => {}
        }
        match current_block_52 {
            9047376510614419450 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 17295929920418635907;
            }
            _ => {}
        }
        match current_block_52 {
            17295929920418635907 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_52 = 9113621750889162830;
            }
            _ => {}
        }
        match current_block_52 {
            9113621750889162830 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 14710078419898575087;
            }
            _ => {}
        }
        match current_block_52 {
            14710078419898575087 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 16220771416666260789;
            }
            _ => {}
        }
        match current_block_52 {
            16220771416666260789 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 15154740142284061113;
            }
            _ => {}
        }
        match current_block_52 {
            15154740142284061113 => {
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
        found = 0 as *mut el_t;
        if !hash.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*hash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*hash).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    found = ((*((*(*hash).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*hash).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut el_t;
                } else {
                    found = 0 as *mut el_t;
                }
                while !found.is_null() {
                    if (*found).hh.hashv == _hf_hashv
                        && (*found).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong
                    {
                        if memcmp(
                            (*found).hh.key,
                            &mut keytofind as *mut *mut libc::c_void
                                as *const libc::c_void,
                            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*found).hh.hh_next).is_null() {
                        found = ((*found).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*hash).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut el_t;
                    } else {
                        found = 0 as *mut el_t;
                    }
                }
            }
        }
    }
    return found;
}
unsafe fn main_0() -> libc::c_int {
    let mut hash: *mut el_t = 0 as *mut el_t;
    let mut e1: el_t = el_t {
        key: 0 as *mut libc::c_void,
        i: 0,
        hh: UT_hash_handle {
            tbl: 0 as *mut UT_hash_table,
            prev: 0 as *mut libc::c_void,
            next: 0 as *mut libc::c_void,
            hh_prev: 0 as *mut UT_hash_handle,
            hh_next: 0 as *mut UT_hash_handle,
            key: 0 as *const libc::c_void,
            keylen: 0,
            hashv: 0,
        },
    };
    let mut e2: el_t = el_t {
        key: 0 as *mut libc::c_void,
        i: 0,
        hh: UT_hash_handle {
            tbl: 0 as *mut UT_hash_table,
            prev: 0 as *mut libc::c_void,
            next: 0 as *mut libc::c_void,
            hh_prev: 0 as *mut UT_hash_handle,
            hh_next: 0 as *mut UT_hash_handle,
            key: 0 as *const libc::c_void,
            keylen: 0,
            hashv: 0,
        },
    };
    e1.key = 0 as *mut libc::c_void;
    e1.i = 1 as libc::c_int;
    e2.key = &mut e2 as *mut el_t as *mut libc::c_void;
    e2.i = 2 as libc::c_int;
    if (findit(hash, 0 as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, NULL) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_6769: {
        if (findit(hash, 0 as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, NULL) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_6715: {
        if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (findit(hash, &mut e2 as *mut el_t as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, &e2) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_6661: {
        if (findit(hash, &mut e2 as *mut el_t as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, &e2) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut e1.key as *mut *mut libc::c_void
        as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_uint;
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
        _ha_hashv = _ha_hashv
            .wrapping_add(
                (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as libc::c_int;
        _hj_key = _hj_key.offset(12 as libc::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
    }
    _ha_hashv = _ha_hashv
        .wrapping_add(
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_58: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 4277766904657099408;
        }
        10 => {
            current_block_58 = 4277766904657099408;
        }
        9 => {
            current_block_58 = 12844153872181735999;
        }
        8 => {
            current_block_58 = 10933093562742521242;
        }
        7 => {
            current_block_58 = 15287460968811925808;
        }
        6 => {
            current_block_58 = 13917059506305561758;
        }
        5 => {
            current_block_58 = 15836351015799117231;
        }
        4 => {
            current_block_58 = 628097227490471054;
        }
        3 => {
            current_block_58 = 7649554592564646292;
        }
        2 => {
            current_block_58 = 17020850572916640716;
        }
        1 => {
            current_block_58 = 8052684671881261617;
        }
        _ => {
            current_block_58 = 10150597327160359210;
        }
    }
    match current_block_58 {
        4277766904657099408 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 12844153872181735999;
        }
        _ => {}
    }
    match current_block_58 {
        12844153872181735999 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 10933093562742521242;
        }
        _ => {}
    }
    match current_block_58 {
        10933093562742521242 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 15287460968811925808;
        }
        _ => {}
    }
    match current_block_58 {
        15287460968811925808 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 13917059506305561758;
        }
        _ => {}
    }
    match current_block_58 {
        13917059506305561758 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 15836351015799117231;
        }
        _ => {}
    }
    match current_block_58 {
        15836351015799117231 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_58 = 628097227490471054;
        }
        _ => {}
    }
    match current_block_58 {
        628097227490471054 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 7649554592564646292;
        }
        _ => {}
    }
    match current_block_58 {
        7649554592564646292 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 17020850572916640716;
        }
        _ => {}
    }
    match current_block_58 {
        17020850572916640716 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 8052684671881261617;
        }
        _ => {}
    }
    match current_block_58 {
        8052684671881261617 => {
            _hj_i = _hj_i
                .wrapping_add(
                    *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_ha_hashv);
    _hj_i ^= _ha_hashv >> 13 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_ha_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as libc::c_int;
    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
    _ha_hashv ^= _hj_j >> 13 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_ha_hashv);
    _hj_i ^= _ha_hashv >> 12 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_ha_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as libc::c_int;
    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
    _ha_hashv ^= _hj_j >> 5 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_ha_hashv);
    _hj_i ^= _ha_hashv >> 3 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_ha_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as libc::c_int;
    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
    _ha_hashv ^= _hj_j >> 15 as libc::c_int;
    e1.hh.hashv = _ha_hashv;
    e1.hh.key = &mut e1.key as *mut *mut libc::c_void as *const libc::c_void;
    e1
        .hh
        .keylen = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        as libc::c_uint;
    if hash.is_null() {
        e1.hh.next = 0 as *mut libc::c_void;
        e1.hh.prev = 0 as *mut libc::c_void;
        e1
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (e1.hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                e1.hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*e1.hh.tbl).tail = &mut e1.hh;
            (*e1.hh.tbl).num_buckets = 32 as libc::c_uint;
            (*e1.hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*e1.hh.tbl)
                .hho = (&mut e1.hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(&mut e1 as *mut el_t as *mut libc::c_char) as libc::c_long;
            (*e1.hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*e1.hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*e1.hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*e1.hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hash = &mut e1;
    } else {
        e1.hh.tbl = (*hash).hh.tbl;
        e1.hh.next = 0 as *mut libc::c_void;
        e1
            .hh
            .prev = ((*(*hash).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*hash).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*hash).hh.tbl).tail).next = &mut e1 as *mut el_t as *mut libc::c_void;
        (*(*hash).hh.tbl).tail = &mut e1.hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*hash).hh.tbl).num_items = ((*(*hash).hh.tbl).num_items).wrapping_add(1);
    (*(*hash).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*hash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*hash).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    e1.hh.hh_next = (*_ha_head).hh_head;
    e1.hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut e1.hh;
    }
    (*_ha_head).hh_head = &mut e1.hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*e1.hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*e1.hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*e1.hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*e1.hh.tbl)
                .ideal_chain_maxlen = ((*e1.hh.tbl).num_items
                >> ((*e1.hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*e1.hh.tbl).num_items
                        & ((*e1.hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*e1.hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*e1.hh.tbl).num_buckets {
                _he_thh = (*((*e1.hh.tbl).buckets).offset(_he_bkt_i as isize)).hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*e1.hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*e1.hh.tbl).ideal_chain_maxlen {
                        (*e1.hh.tbl)
                            .nonideal_items = ((*e1.hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*e1.hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*e1.hh.tbl).ideal_chain_maxlen)
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
            free((*e1.hh.tbl).buckets as *mut libc::c_void);
            (*e1.hh.tbl)
                .num_buckets = ((*e1.hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*e1.hh.tbl)
                .log2_num_buckets = ((*e1.hh.tbl).log2_num_buckets).wrapping_add(1);
            (*e1.hh.tbl).log2_num_buckets;
            (*e1.hh.tbl).buckets = _he_new_buckets;
            (*e1.hh.tbl)
                .ineff_expands = if (*e1.hh.tbl).nonideal_items
                > (*e1.hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*e1.hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*e1.hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*e1.hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if findit(hash, 0 as *mut libc::c_void) == &mut e1 as *mut el_t {} else {
        __assert_fail(
            b"findit(hash, NULL) == &e1\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4923: {
        if findit(hash, 0 as *mut libc::c_void) == &mut e1 as *mut el_t {} else {
            __assert_fail(
                b"findit(hash, NULL) == &e1\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                35 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4869: {
        if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                36 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (findit(hash, &mut e2 as *mut el_t as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, &e2) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4815: {
        if (findit(hash, &mut e2 as *mut el_t as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, &e2) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    let mut _ha_hashv_0: libc::c_uint = 0;
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *const libc::c_uchar = &mut e2.key as *mut *mut libc::c_void
        as *const libc::c_uchar;
    _ha_hashv_0 = 0xfeedbeef as libc::c_uint;
    _hj_j_0 = 0x9e3779b9 as libc::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        as libc::c_uint;
    while _hj_k_0 >= 12 as libc::c_uint {
        _hj_i_0 = _hj_i_0
            .wrapping_add(
                (*_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j_0 = _hj_j_0
            .wrapping_add(
                (*_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _ha_hashv_0 = _ha_hashv_0
            .wrapping_add(
                (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv_0);
        _hj_i_0 ^= _ha_hashv_0 >> 13 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_0);
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_0);
        _ha_hashv_0 ^= _hj_j_0 >> 13 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv_0);
        _hj_i_0 ^= _ha_hashv_0 >> 12 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_0);
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_0);
        _ha_hashv_0 ^= _hj_j_0 >> 5 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv_0);
        _hj_i_0 ^= _ha_hashv_0 >> 3 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_0);
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_0);
        _ha_hashv_0 ^= _hj_j_0 >> 15 as libc::c_int;
        _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
        _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
    }
    _ha_hashv_0 = _ha_hashv_0
        .wrapping_add(
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_252: u64;
    match _hj_k_0 {
        11 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_252 = 4686027536167621817;
        }
        10 => {
            current_block_252 = 4686027536167621817;
        }
        9 => {
            current_block_252 = 7248293206456826136;
        }
        8 => {
            current_block_252 = 3087231908759333057;
        }
        7 => {
            current_block_252 = 8177577260923278842;
        }
        6 => {
            current_block_252 = 8840419618977828003;
        }
        5 => {
            current_block_252 = 11128930359057533396;
        }
        4 => {
            current_block_252 = 12399680141611551320;
        }
        3 => {
            current_block_252 = 3067226839132823687;
        }
        2 => {
            current_block_252 = 6932005650290936717;
        }
        1 => {
            current_block_252 = 1525014864485157485;
        }
        _ => {
            current_block_252 = 11620251134136044114;
        }
    }
    match current_block_252 {
        4686027536167621817 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_252 = 7248293206456826136;
        }
        _ => {}
    }
    match current_block_252 {
        7248293206456826136 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_252 = 3087231908759333057;
        }
        _ => {}
    }
    match current_block_252 {
        3087231908759333057 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_252 = 8177577260923278842;
        }
        _ => {}
    }
    match current_block_252 {
        8177577260923278842 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_252 = 8840419618977828003;
        }
        _ => {}
    }
    match current_block_252 {
        8840419618977828003 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_252 = 11128930359057533396;
        }
        _ => {}
    }
    match current_block_252 {
        11128930359057533396 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_252 = 12399680141611551320;
        }
        _ => {}
    }
    match current_block_252 {
        12399680141611551320 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_252 = 3067226839132823687;
        }
        _ => {}
    }
    match current_block_252 {
        3067226839132823687 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_252 = 6932005650290936717;
        }
        _ => {}
    }
    match current_block_252 {
        6932005650290936717 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_252 = 1525014864485157485;
        }
        _ => {}
    }
    match current_block_252 {
        1525014864485157485 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv_0);
    _hj_i_0 ^= _ha_hashv_0 >> 13 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv_0);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_0);
    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_0);
    _ha_hashv_0 ^= _hj_j_0 >> 13 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv_0);
    _hj_i_0 ^= _ha_hashv_0 >> 12 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv_0);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_0);
    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_0);
    _ha_hashv_0 ^= _hj_j_0 >> 5 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv_0);
    _hj_i_0 ^= _ha_hashv_0 >> 3 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv_0);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_0);
    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_0);
    _ha_hashv_0 ^= _hj_j_0 >> 15 as libc::c_int;
    e2.hh.hashv = _ha_hashv_0;
    e2.hh.key = &mut e2.key as *mut *mut libc::c_void as *const libc::c_void;
    e2
        .hh
        .keylen = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        as libc::c_uint;
    if hash.is_null() {
        e2.hh.next = 0 as *mut libc::c_void;
        e2.hh.prev = 0 as *mut libc::c_void;
        e2
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (e2.hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                e2.hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*e2.hh.tbl).tail = &mut e2.hh;
            (*e2.hh.tbl).num_buckets = 32 as libc::c_uint;
            (*e2.hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*e2.hh.tbl)
                .hho = (&mut e2.hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(&mut e2 as *mut el_t as *mut libc::c_char) as libc::c_long;
            (*e2.hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*e2.hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*e2.hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*e2.hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hash = &mut e2;
    } else {
        e2.hh.tbl = (*hash).hh.tbl;
        e2.hh.next = 0 as *mut libc::c_void;
        e2
            .hh
            .prev = ((*(*hash).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*hash).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*hash).hh.tbl).tail).next = &mut e2 as *mut el_t as *mut libc::c_void;
        (*(*hash).hh.tbl).tail = &mut e2.hh;
    }
    let mut _ha_bkt_0: libc::c_uint = 0;
    (*(*hash).hh.tbl).num_items = ((*(*hash).hh.tbl).num_items).wrapping_add(1);
    (*(*hash).hh.tbl).num_items;
    _ha_bkt_0 = _ha_hashv_0
        & ((*(*hash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*hash).hh.tbl).buckets)
        .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
    (*_ha_head_0).count;
    e2.hh.hh_next = (*_ha_head_0).hh_head;
    e2.hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_0).hh_head).is_null() {
        (*(*_ha_head_0).hh_head).hh_prev = &mut e2.hh;
    }
    (*_ha_head_0).hh_head = &mut e2.hh;
    if (*_ha_head_0).count
        >= ((*_ha_head_0).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*e2.hh.tbl).noexpand == 0
    {
        let mut _he_bkt_0: libc::c_uint = 0;
        let mut _he_bkt_i_0: libc::c_uint = 0;
        let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_0 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*e2.hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_0.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_0 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*e2.hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*e2.hh.tbl)
                .ideal_chain_maxlen = ((*e2.hh.tbl).num_items
                >> ((*e2.hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*e2.hh.tbl).num_items
                        & ((*e2.hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*e2.hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_0 < (*e2.hh.tbl).num_buckets {
                _he_thh_0 = (*((*e2.hh.tbl).buckets).offset(_he_bkt_i_0 as isize))
                    .hh_head;
                while !_he_thh_0.is_null() {
                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                    _he_bkt_0 = (*_he_thh_0).hashv
                        & ((*e2.hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                    if (*_he_newbkt_0).count > (*e2.hh.tbl).ideal_chain_maxlen {
                        (*e2.hh.tbl)
                            .nonideal_items = ((*e2.hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*e2.hh.tbl).nonideal_items;
                        if (*_he_newbkt_0).count
                            > ((*_he_newbkt_0).expand_mult)
                                .wrapping_mul((*e2.hh.tbl).ideal_chain_maxlen)
                        {
                            (*_he_newbkt_0)
                                .expand_mult = ((*_he_newbkt_0).expand_mult)
                                .wrapping_add(1);
                            (*_he_newbkt_0).expand_mult;
                        }
                    }
                    (*_he_thh_0).hh_prev = 0 as *mut UT_hash_handle;
                    (*_he_thh_0).hh_next = (*_he_newbkt_0).hh_head;
                    if !((*_he_newbkt_0).hh_head).is_null() {
                        (*(*_he_newbkt_0).hh_head).hh_prev = _he_thh_0;
                    }
                    (*_he_newbkt_0).hh_head = _he_thh_0;
                    _he_thh_0 = _he_hh_nxt_0;
                }
                _he_bkt_i_0 = _he_bkt_i_0.wrapping_add(1);
                _he_bkt_i_0;
            }
            free((*e2.hh.tbl).buckets as *mut libc::c_void);
            (*e2.hh.tbl)
                .num_buckets = ((*e2.hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*e2.hh.tbl)
                .log2_num_buckets = ((*e2.hh.tbl).log2_num_buckets).wrapping_add(1);
            (*e2.hh.tbl).log2_num_buckets;
            (*e2.hh.tbl).buckets = _he_new_buckets_0;
            (*e2.hh.tbl)
                .ineff_expands = if (*e2.hh.tbl).nonideal_items
                > (*e2.hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*e2.hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*e2.hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*e2.hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if findit(hash, 0 as *mut libc::c_void) == &mut e1 as *mut el_t {} else {
        __assert_fail(
            b"findit(hash, NULL) == &e1\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3065: {
        if findit(hash, 0 as *mut libc::c_void) == &mut e1 as *mut el_t {} else {
            __assert_fail(
                b"findit(hash, NULL) == &e1\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3011: {
        if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if findit(hash, &mut e2 as *mut el_t as *mut libc::c_void) == &mut e2 as *mut el_t
    {} else {
        __assert_fail(
            b"findit(hash, &e2) == &e2\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2961: {
        if findit(hash, &mut e2 as *mut el_t as *mut libc::c_void)
            == &mut e2 as *mut el_t
        {} else {
            __assert_fail(
                b"findit(hash, &e2) == &e2\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                42 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    let mut _hd_hh_del: *mut UT_hash_handle = &mut e1.hh;
    if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
        free((*(*hash).hh.tbl).buckets as *mut libc::c_void);
        free((*hash).hh.tbl as *mut libc::c_void);
        hash = 0 as *mut el_t;
    } else {
        let mut _hd_bkt: libc::c_uint = 0;
        if _hd_hh_del == (*(*hash).hh.tbl).tail {
            (*(*hash).hh.tbl)
                .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                .offset((*(*hash).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle;
        }
        if !((*_hd_hh_del).prev).is_null() {
            let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                .offset((*(*hash).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            *fresh0 = (*_hd_hh_del).next;
        } else {
            hash = (*_hd_hh_del).next as *mut el_t;
        }
        if !((*_hd_hh_del).next).is_null() {
            let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                .offset((*(*hash).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh1 = (*_hd_hh_del).prev;
        }
        _hd_bkt = (*_hd_hh_del).hashv
            & ((*(*hash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*hash).hh.tbl).buckets)
            .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
        (*_hd_head).count = ((*_hd_head).count).wrapping_sub(1);
        (*_hd_head).count;
        if (*_hd_head).hh_head == _hd_hh_del {
            (*_hd_head).hh_head = (*_hd_hh_del).hh_next;
        }
        if !((*_hd_hh_del).hh_prev).is_null() {
            (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
        }
        if !((*_hd_hh_del).hh_next).is_null() {
            (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
        }
        (*(*hash).hh.tbl).num_items = ((*(*hash).hh.tbl).num_items).wrapping_sub(1);
        (*(*hash).hh.tbl).num_items;
    }
    if (findit(hash, 0 as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, NULL) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2582: {
        if (findit(hash, 0 as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, NULL) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2526: {
        if (findit(hash, &mut e1 as *mut el_t as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"findit(hash, &e1) == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if findit(hash, &mut e2 as *mut el_t as *mut libc::c_void) == &mut e2 as *mut el_t
    {} else {
        __assert_fail(
            b"findit(hash, &e2) == &e2\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2473: {
        if findit(hash, &mut e2 as *mut el_t as *mut libc::c_void)
            == &mut e2 as *mut el_t
        {} else {
            __assert_fail(
                b"findit(hash, &e2) == &e2\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !hash.is_null() {
        free((*(*hash).hh.tbl).buckets as *mut libc::c_void);
        free((*hash).hh.tbl as *mut libc::c_void);
        hash = 0 as *mut el_t;
    }
    if hash.is_null() {} else {
        __assert_fail(
            b"hash == NULL\0" as *const u8 as *const libc::c_char,
            b"test57.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2375: {
        if hash.is_null() {} else {
            __assert_fail(
                b"hash == NULL\0" as *const u8 as *const libc::c_char,
                b"test57.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type ptrdiff_t = libc::c_long;
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
pub struct record_key_t {
    pub a: libc::c_char,
    pub b: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record_t {
    pub key: record_key_t,
    pub hh: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut l: record_t = record_t {
        key: record_key_t { a: 0, b: 0 },
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
    let mut p: *mut record_t = 0 as *mut record_t;
    let mut r: *mut record_t = 0 as *mut record_t;
    let mut tmp: *mut record_t = 0 as *mut record_t;
    let mut records: *mut record_t = 0 as *mut record_t;
    r = malloc(::std::mem::size_of::<record_t>() as libc::c_ulong) as *mut record_t;
    if r.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        r as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<record_t>() as libc::c_ulong,
    );
    (*r).key.a = 'a' as i32 as libc::c_char;
    (*r).key.b = 1 as libc::c_int;
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut (*r).key as *mut record_key_t
        as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::std::mem::size_of::<record_key_t>() as libc::c_ulong as libc::c_uint;
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
            ::std::mem::size_of::<record_key_t>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_58: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 10703124919230875173;
        }
        10 => {
            current_block_58 = 10703124919230875173;
        }
        9 => {
            current_block_58 = 5736421875614469061;
        }
        8 => {
            current_block_58 = 6246763041535604849;
        }
        7 => {
            current_block_58 = 696810026292259735;
        }
        6 => {
            current_block_58 = 18195282705153904920;
        }
        5 => {
            current_block_58 = 16051936835507880021;
        }
        4 => {
            current_block_58 = 14928940103145537714;
        }
        3 => {
            current_block_58 = 12999328127362432425;
        }
        2 => {
            current_block_58 = 2430319949834955361;
        }
        1 => {
            current_block_58 = 14180127615403059768;
        }
        _ => {
            current_block_58 = 6174974146017752131;
        }
    }
    match current_block_58 {
        10703124919230875173 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 5736421875614469061;
        }
        _ => {}
    }
    match current_block_58 {
        5736421875614469061 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 6246763041535604849;
        }
        _ => {}
    }
    match current_block_58 {
        6246763041535604849 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 696810026292259735;
        }
        _ => {}
    }
    match current_block_58 {
        696810026292259735 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 18195282705153904920;
        }
        _ => {}
    }
    match current_block_58 {
        18195282705153904920 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 16051936835507880021;
        }
        _ => {}
    }
    match current_block_58 {
        16051936835507880021 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_58 = 14928940103145537714;
        }
        _ => {}
    }
    match current_block_58 {
        14928940103145537714 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 12999328127362432425;
        }
        _ => {}
    }
    match current_block_58 {
        12999328127362432425 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 2430319949834955361;
        }
        _ => {}
    }
    match current_block_58 {
        2430319949834955361 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 14180127615403059768;
        }
        _ => {}
    }
    match current_block_58 {
        14180127615403059768 => {
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
    (*r).hh.hashv = _ha_hashv;
    (*r).hh.key = &mut (*r).key as *mut record_key_t as *const libc::c_void;
    (*r)
        .hh
        .keylen = ::std::mem::size_of::<record_key_t>() as libc::c_ulong as libc::c_uint;
    if records.is_null() {
        (*r).hh.next = 0 as *mut libc::c_void;
        (*r).hh.prev = 0 as *mut libc::c_void;
        (*r)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*r).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*r).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*r).hh.tbl).tail = &mut (*r).hh;
            (*(*r).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*r).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*r).hh.tbl)
                .hho = (&mut (*r).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(r as *mut libc::c_char) as libc::c_long;
            (*(*r).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*r).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*r).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*r).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        records = r;
    } else {
        (*r).hh.tbl = (*records).hh.tbl;
        (*r).hh.next = 0 as *mut libc::c_void;
        (*r)
            .hh
            .prev = ((*(*records).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*records).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*records).hh.tbl).tail).next = r as *mut libc::c_void;
        (*(*records).hh.tbl).tail = &mut (*r).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*records).hh.tbl).num_items = ((*(*records).hh.tbl).num_items).wrapping_add(1);
    (*(*records).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*records).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*records).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*r).hh.hh_next = (*_ha_head).hh_head;
    (*r).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*r).hh;
    }
    (*_ha_head).hh_head = &mut (*r).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*r).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*r).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*r).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*r).hh.tbl)
                .ideal_chain_maxlen = ((*(*r).hh.tbl).num_items
                >> ((*(*r).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*r).hh.tbl).num_items
                        & ((*(*r).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*r).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*r).hh.tbl).num_buckets {
                _he_thh = (*((*(*r).hh.tbl).buckets).offset(_he_bkt_i as isize)).hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*r).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*r).hh.tbl).ideal_chain_maxlen {
                        (*(*r).hh.tbl)
                            .nonideal_items = ((*(*r).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*r).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*r).hh.tbl).ideal_chain_maxlen)
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
            free((*(*r).hh.tbl).buckets as *mut libc::c_void);
            (*(*r).hh.tbl)
                .num_buckets = ((*(*r).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*r).hh.tbl)
                .log2_num_buckets = ((*(*r).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*r).hh.tbl).log2_num_buckets;
            (*(*r).hh.tbl).buckets = _he_new_buckets;
            (*(*r).hh.tbl)
                .ineff_expands = if (*(*r).hh.tbl).nonideal_items
                > (*(*r).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*r).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*r).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*r).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    memset(
        &mut l as *mut record_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<record_t>() as libc::c_ulong,
    );
    l.key.a = 'a' as i32 as libc::c_char;
    l.key.b = 1 as libc::c_int;
    p = 0 as *mut record_t;
    if !records.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_0: libc::c_uint = 0;
        let mut _hj_j_0: libc::c_uint = 0;
        let mut _hj_k_0: libc::c_uint = 0;
        let mut _hj_key_0: *const libc::c_uchar = &mut l.key as *mut record_key_t
            as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j_0 = 0x9e3779b9 as libc::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::std::mem::size_of::<record_key_t>() as libc::c_ulong as libc::c_uint;
        while _hj_k_0 >= 12 as libc::c_uint {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint)
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
                    (*_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint)
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
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
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
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv);
            _hj_i_0 ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_0);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_0);
            _hf_hashv ^= _hj_j_0 >> 13 as libc::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv);
            _hj_i_0 ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_0);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_0);
            _hf_hashv ^= _hj_j_0 >> 5 as libc::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv);
            _hj_i_0 ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_0);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_0);
            _hf_hashv ^= _hj_j_0 >> 15 as libc::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(
                ::std::mem::size_of::<record_key_t>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_253: u64;
        match _hj_k_0 {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_253 = 18233339199029667;
            }
            10 => {
                current_block_253 = 18233339199029667;
            }
            9 => {
                current_block_253 = 15876325417469710291;
            }
            8 => {
                current_block_253 = 16621153270273717494;
            }
            7 => {
                current_block_253 = 16445404902389957641;
            }
            6 => {
                current_block_253 = 13563171475922671077;
            }
            5 => {
                current_block_253 = 13159533426611663449;
            }
            4 => {
                current_block_253 = 14300543941082899683;
            }
            3 => {
                current_block_253 = 8777519408277806717;
            }
            2 => {
                current_block_253 = 5378243231166472823;
            }
            1 => {
                current_block_253 = 15347768847062404082;
            }
            _ => {
                current_block_253 = 8966873693763611635;
            }
        }
        match current_block_253 {
            18233339199029667 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_253 = 15876325417469710291;
            }
            _ => {}
        }
        match current_block_253 {
            15876325417469710291 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_253 = 16621153270273717494;
            }
            _ => {}
        }
        match current_block_253 {
            16621153270273717494 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_253 = 16445404902389957641;
            }
            _ => {}
        }
        match current_block_253 {
            16445404902389957641 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_253 = 13563171475922671077;
            }
            _ => {}
        }
        match current_block_253 {
            13563171475922671077 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_253 = 13159533426611663449;
            }
            _ => {}
        }
        match current_block_253 {
            13159533426611663449 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_253 = 14300543941082899683;
            }
            _ => {}
        }
        match current_block_253 {
            14300543941082899683 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_253 = 8777519408277806717;
            }
            _ => {}
        }
        match current_block_253 {
            8777519408277806717 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_253 = 5378243231166472823;
            }
            _ => {}
        }
        match current_block_253 {
            5378243231166472823 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_253 = 15347768847062404082;
            }
            _ => {}
        }
        match current_block_253 {
            15347768847062404082 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv);
        _hj_i_0 ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_0);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_0);
        _hf_hashv ^= _hj_j_0 >> 13 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv);
        _hj_i_0 ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_0);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_0);
        _hf_hashv ^= _hj_j_0 >> 5 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv);
        _hj_i_0 ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_0);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_0);
        _hf_hashv ^= _hj_j_0 >> 15 as libc::c_int;
        p = 0 as *mut record_t;
        if !records.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*records).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*records).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    p = ((*((*(*records).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*records).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut record_t;
                } else {
                    p = 0 as *mut record_t;
                }
                while !p.is_null() {
                    if (*p).hh.hashv == _hf_hashv
                        && (*p).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<record_key_t>() as libc::c_ulong
                    {
                        if memcmp(
                            (*p).hh.key,
                            &mut l.key as *mut record_key_t as *const libc::c_void,
                            ::std::mem::size_of::<record_key_t>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*p).hh.hh_next).is_null() {
                        p = ((*p).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*records).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut record_t;
                    } else {
                        p = 0 as *mut record_t;
                    }
                }
            }
        }
    }
    if !p.is_null() {
        printf(
            b"found %c %d\n\0" as *const u8 as *const libc::c_char,
            (*p).key.a as libc::c_int,
            (*p).key.b,
        );
    }
    p = records;
    tmp = (if !records.is_null() { (*records).hh.next } else { 0 as *mut libc::c_void })
        as *mut record_t;
    while !p.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*p).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*records).hh.tbl).buckets as *mut libc::c_void);
            free((*records).hh.tbl as *mut libc::c_void);
            records = 0 as *mut record_t;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*records).hh.tbl).tail {
                (*(*records).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*records).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*records).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                records = (*_hd_hh_del).next as *mut record_t;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*records).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*records).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*records).hh.tbl).buckets)
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
            (*(*records).hh.tbl)
                .num_items = ((*(*records).hh.tbl).num_items).wrapping_sub(1);
            (*(*records).hh.tbl).num_items;
        }
        free(p as *mut libc::c_void);
        p = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { 0 as *mut libc::c_void })
            as *mut record_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

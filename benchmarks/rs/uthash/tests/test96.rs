use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct clockface {
    pub time: libc::c_int,
    pub hh: UT_hash_handle,
}
pub unsafe extern "C" fn clockface_hash(mut time: libc::c_int) -> libc::c_int {
    return time % 4 as libc::c_int;
}
pub unsafe extern "C" fn clockface_neq(
    mut t1: libc::c_int,
    mut t2: libc::c_int,
) -> libc::c_int {
    return (t1 % 12 as libc::c_int != t2 % 12 as libc::c_int) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut random_data: [libc::c_int; 40] = [
        56 as libc::c_int,
        7 as libc::c_int,
        10 as libc::c_int,
        39 as libc::c_int,
        82 as libc::c_int,
        15 as libc::c_int,
        31 as libc::c_int,
        26 as libc::c_int,
        51 as libc::c_int,
        83 as libc::c_int,
        46 as libc::c_int,
        92 as libc::c_int,
        49 as libc::c_int,
        25 as libc::c_int,
        80 as libc::c_int,
        54 as libc::c_int,
        97 as libc::c_int,
        9 as libc::c_int,
        34 as libc::c_int,
        86 as libc::c_int,
        87 as libc::c_int,
        28 as libc::c_int,
        13 as libc::c_int,
        91 as libc::c_int,
        95 as libc::c_int,
        63 as libc::c_int,
        71 as libc::c_int,
        100 as libc::c_int,
        44 as libc::c_int,
        42 as libc::c_int,
        16 as libc::c_int,
        32 as libc::c_int,
        6 as libc::c_int,
        85 as libc::c_int,
        40 as libc::c_int,
        20 as libc::c_int,
        18 as libc::c_int,
        99 as libc::c_int,
        22 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut times: *mut clockface = 0 as *mut clockface;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        let mut elt: *mut clockface = malloc(
            ::std::mem::size_of::<clockface>() as libc::c_ulong,
        ) as *mut clockface;
        let mut found: *mut clockface = 0 as *mut clockface;
        (*elt).time = random_data[i as usize];
        found = 0 as *mut clockface;
        if !times.is_null() {
            let mut _hf_hashv: libc::c_uint = 0;
            _hf_hashv = clockface_hash(
                *(&mut (*elt).time as *mut libc::c_int as *const libc::c_int),
            ) as libc::c_uint;
            found = 0 as *mut clockface;
            if !times.is_null() {
                let mut _hf_bkt: libc::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & ((*(*times).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                if 1 as libc::c_int != 0 as libc::c_int {
                    if !((*((*(*times).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head)
                        .is_null()
                    {
                        found = ((*((*(*times).hh.tbl).buckets).offset(_hf_bkt as isize))
                            .hh_head as *mut libc::c_char)
                            .offset(-((*(*times).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut clockface;
                    } else {
                        found = 0 as *mut clockface;
                    }
                    while !found.is_null() {
                        if (*found).hh.hashv == _hf_hashv
                            && (*found).hh.keylen as libc::c_ulong
                                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            if clockface_neq(
                                *((*found).hh.key as *const libc::c_int),
                                *(&mut (*elt).time as *mut libc::c_int
                                    as *const libc::c_int),
                            ) == 0 as libc::c_int
                            {
                                break;
                            }
                        }
                        if !((*found).hh.hh_next).is_null() {
                            found = ((*found).hh.hh_next as *mut libc::c_char)
                                .offset(-((*(*times).hh.tbl).hho as isize))
                                as *mut libc::c_void as *mut clockface;
                        } else {
                            found = 0 as *mut clockface;
                        }
                    }
                }
            }
        }
        if !found.is_null() {
            printf(
                b"time %d found with value %d\n\0" as *const u8 as *const libc::c_char,
                (*elt).time,
                (*found).time,
            );
        } else {
            printf(
                b"time %d not found, inserting it\n\0" as *const u8
                    as *const libc::c_char,
                (*elt).time,
            );
            let mut _ha_hashv: libc::c_uint = 0;
            _ha_hashv = clockface_hash(
                *(&mut (*elt).time as *mut libc::c_int as *const libc::c_int),
            ) as libc::c_uint;
            (*elt).hh.hashv = _ha_hashv;
            (*elt).hh.key = &mut (*elt).time as *mut libc::c_int as *const libc::c_void;
            (*elt)
                .hh
                .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            if times.is_null() {
                (*elt).hh.next = 0 as *mut libc::c_void;
                (*elt).hh.prev = 0 as *mut libc::c_void;
                (*elt)
                    .hh
                    .tbl = malloc(
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                ) as *mut UT_hash_table;
                if ((*elt).hh.tbl).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*elt).hh.tbl as *mut libc::c_void,
                        '\0' as i32,
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    );
                    (*(*elt).hh.tbl).tail = &mut (*elt).hh;
                    (*(*elt).hh.tbl).num_buckets = 32 as libc::c_uint;
                    (*(*elt).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                    (*(*elt).hh.tbl)
                        .hho = (&mut (*elt).hh as *mut UT_hash_handle
                        as *mut libc::c_char)
                        .offset_from(elt as *mut libc::c_char) as libc::c_long;
                    (*(*elt).hh.tbl)
                        .buckets = malloc(
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    ) as *mut UT_hash_bucket;
                    (*(*elt).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                    if ((*(*elt).hh.tbl).buckets).is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            (*(*elt).hh.tbl).buckets as *mut libc::c_void,
                            '\0' as i32,
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        );
                    }
                }
                times = elt;
            } else {
                (*elt).hh.tbl = (*times).hh.tbl;
                (*elt).hh.next = 0 as *mut libc::c_void;
                (*elt)
                    .hh
                    .prev = ((*(*times).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*times).hh.tbl).hho as isize)) as *mut libc::c_void;
                (*(*(*times).hh.tbl).tail).next = elt as *mut libc::c_void;
                (*(*times).hh.tbl).tail = &mut (*elt).hh;
            }
            let mut _ha_bkt: libc::c_uint = 0;
            (*(*times).hh.tbl)
                .num_items = ((*(*times).hh.tbl).num_items).wrapping_add(1);
            (*(*times).hh.tbl).num_items;
            _ha_bkt = _ha_hashv
                & ((*(*times).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*times).hh.tbl).buckets)
                .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
            (*_ha_head).count;
            (*elt).hh.hh_next = (*_ha_head).hh_head;
            (*elt).hh.hh_prev = 0 as *mut UT_hash_handle;
            if !((*_ha_head).hh_head).is_null() {
                (*(*_ha_head).hh_head).hh_prev = &mut (*elt).hh;
            }
            (*_ha_head).hh_head = &mut (*elt).hh;
            if (*_ha_head).count
                >= ((*_ha_head).expand_mult)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_mul(10 as libc::c_uint) && (*(*elt).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: libc::c_uint = 0;
                let mut _he_bkt_i: libc::c_uint = 0;
                let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                _he_new_buckets = malloc(
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*elt).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut libc::c_void,
                        '\0' as i32,
                        (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                            .wrapping_mul((*(*elt).hh.tbl).num_buckets as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                    );
                    (*(*elt).hh.tbl)
                        .ideal_chain_maxlen = ((*(*elt).hh.tbl).num_items
                        >> ((*(*elt).hh.tbl).log2_num_buckets)
                            .wrapping_add(1 as libc::c_uint))
                        .wrapping_add(
                            (if (*(*elt).hh.tbl).num_items
                                & ((*(*elt).hh.tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                            {
                                1 as libc::c_uint
                            } else {
                                0 as libc::c_uint
                            }),
                        );
                    (*(*elt).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                    _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                    while _he_bkt_i < (*(*elt).hh.tbl).num_buckets {
                        _he_thh = (*((*(*elt).hh.tbl).buckets)
                            .offset(_he_bkt_i as isize))
                            .hh_head;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & ((*(*elt).hh.tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_uint);
                            _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                                as *mut UT_hash_bucket;
                            (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                            if (*_he_newbkt).count > (*(*elt).hh.tbl).ideal_chain_maxlen
                            {
                                (*(*elt).hh.tbl)
                                    .nonideal_items = ((*(*elt).hh.tbl).nonideal_items)
                                    .wrapping_add(1);
                                (*(*elt).hh.tbl).nonideal_items;
                                if (*_he_newbkt).count
                                    > ((*_he_newbkt).expand_mult)
                                        .wrapping_mul((*(*elt).hh.tbl).ideal_chain_maxlen)
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
                    free((*(*elt).hh.tbl).buckets as *mut libc::c_void);
                    (*(*elt).hh.tbl)
                        .num_buckets = ((*(*elt).hh.tbl).num_buckets)
                        .wrapping_mul(2 as libc::c_uint);
                    (*(*elt).hh.tbl)
                        .log2_num_buckets = ((*(*elt).hh.tbl).log2_num_buckets)
                        .wrapping_add(1);
                    (*(*elt).hh.tbl).log2_num_buckets;
                    (*(*elt).hh.tbl).buckets = _he_new_buckets;
                    (*(*elt).hh.tbl)
                        .ineff_expands = if (*(*elt).hh.tbl).nonideal_items
                        > (*(*elt).hh.tbl).num_items >> 1 as libc::c_int
                    {
                        ((*(*elt).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                    } else {
                        0 as libc::c_uint
                    };
                    if (*(*elt).hh.tbl).ineff_expands > 1 as libc::c_uint {
                        (*(*elt).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

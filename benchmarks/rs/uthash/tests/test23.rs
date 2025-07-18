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
pub struct item {
    pub key: libc::c_int,
    pub data: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut i: *mut item = 0 as *mut item;
    let mut j: *mut item = 0 as *mut item;
    let mut items: *mut item = 0 as *mut item;
    let mut k: libc::c_int = 0;
    k = 12345 as libc::c_int;
    i = malloc(::std::mem::size_of::<item>() as libc::c_ulong) as *mut item;
    if i.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*i).key = k;
    (*i).data = 0 as libc::c_int;
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut (*i).key as *mut libc::c_int
        as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
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
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_58: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 10186842371145338957;
        }
        10 => {
            current_block_58 = 10186842371145338957;
        }
        9 => {
            current_block_58 = 8261084354856074439;
        }
        8 => {
            current_block_58 = 12514172292819301680;
        }
        7 => {
            current_block_58 = 14730262267959436050;
        }
        6 => {
            current_block_58 = 4238101913630127839;
        }
        5 => {
            current_block_58 = 13347018074205139155;
        }
        4 => {
            current_block_58 = 5330031725609985395;
        }
        3 => {
            current_block_58 = 8551572785456621305;
        }
        2 => {
            current_block_58 = 7071498457278444726;
        }
        1 => {
            current_block_58 = 11510708383247420660;
        }
        _ => {
            current_block_58 = 14775119014532381840;
        }
    }
    match current_block_58 {
        10186842371145338957 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 8261084354856074439;
        }
        _ => {}
    }
    match current_block_58 {
        8261084354856074439 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 12514172292819301680;
        }
        _ => {}
    }
    match current_block_58 {
        12514172292819301680 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 14730262267959436050;
        }
        _ => {}
    }
    match current_block_58 {
        14730262267959436050 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 4238101913630127839;
        }
        _ => {}
    }
    match current_block_58 {
        4238101913630127839 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 13347018074205139155;
        }
        _ => {}
    }
    match current_block_58 {
        13347018074205139155 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_58 = 5330031725609985395;
        }
        _ => {}
    }
    match current_block_58 {
        5330031725609985395 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_58 = 8551572785456621305;
        }
        _ => {}
    }
    match current_block_58 {
        8551572785456621305 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_58 = 7071498457278444726;
        }
        _ => {}
    }
    match current_block_58 {
        7071498457278444726 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_58 = 11510708383247420660;
        }
        _ => {}
    }
    match current_block_58 {
        11510708383247420660 => {
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
    (*i).hh.hashv = _ha_hashv;
    (*i).hh.key = &mut (*i).key as *mut libc::c_int as *const libc::c_void;
    (*i)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if items.is_null() {
        (*i).hh.next = 0 as *mut libc::c_void;
        (*i).hh.prev = 0 as *mut libc::c_void;
        (*i)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*i).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*i).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*i).hh.tbl).tail = &mut (*i).hh;
            (*(*i).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*i).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*i).hh.tbl)
                .hho = (&mut (*i).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(i as *mut libc::c_char) as libc::c_long;
            (*(*i).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*i).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*i).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*i).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        items = i;
    } else {
        (*i).hh.tbl = (*items).hh.tbl;
        (*i).hh.next = 0 as *mut libc::c_void;
        (*i)
            .hh
            .prev = ((*(*items).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*items).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*items).hh.tbl).tail).next = i as *mut libc::c_void;
        (*(*items).hh.tbl).tail = &mut (*i).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*items).hh.tbl).num_items = ((*(*items).hh.tbl).num_items).wrapping_add(1);
    (*(*items).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*items).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*i).hh.hh_next = (*_ha_head).hh_head;
    (*i).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*i).hh;
    }
    (*_ha_head).hh_head = &mut (*i).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*i).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*i).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*i).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*i).hh.tbl)
                .ideal_chain_maxlen = ((*(*i).hh.tbl).num_items
                >> ((*(*i).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*i).hh.tbl).num_items
                        & ((*(*i).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*i).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*i).hh.tbl).num_buckets {
                _he_thh = (*((*(*i).hh.tbl).buckets).offset(_he_bkt_i as isize)).hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*i).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*i).hh.tbl).ideal_chain_maxlen {
                        (*(*i).hh.tbl)
                            .nonideal_items = ((*(*i).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*i).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*i).hh.tbl).ideal_chain_maxlen)
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
            free((*(*i).hh.tbl).buckets as *mut libc::c_void);
            (*(*i).hh.tbl)
                .num_buckets = ((*(*i).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*i).hh.tbl)
                .log2_num_buckets = ((*(*i).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*i).hh.tbl).log2_num_buckets;
            (*(*i).hh.tbl).buckets = _he_new_buckets;
            (*(*i).hh.tbl)
                .ineff_expands = if (*(*i).hh.tbl).nonideal_items
                > (*(*i).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*i).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*i).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*i).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    k = 6789 as libc::c_int;
    i = malloc(::std::mem::size_of::<item>() as libc::c_ulong) as *mut item;
    if i.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*i).key = k;
    (*i).data = 0 as libc::c_int;
    let mut _ha_hashv_0: libc::c_uint = 0;
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *const libc::c_uchar = &mut (*i).key as *mut libc::c_int
        as *const libc::c_uchar;
    _ha_hashv_0 = 0xfeedbeef as libc::c_uint;
    _hj_j_0 = 0x9e3779b9 as libc::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
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
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_256: u64;
    match _hj_k_0 {
        11 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_256 = 6372208342811818303;
        }
        10 => {
            current_block_256 = 6372208342811818303;
        }
        9 => {
            current_block_256 = 17756757930076294871;
        }
        8 => {
            current_block_256 = 12692937682296807826;
        }
        7 => {
            current_block_256 = 6225258385525537431;
        }
        6 => {
            current_block_256 = 2018927983898164238;
        }
        5 => {
            current_block_256 = 4202242720334207286;
        }
        4 => {
            current_block_256 = 6323618852566759355;
        }
        3 => {
            current_block_256 = 7853761040057074878;
        }
        2 => {
            current_block_256 = 6142051208924225422;
        }
        1 => {
            current_block_256 = 12735356383178692136;
        }
        _ => {
            current_block_256 = 9467764101860050311;
        }
    }
    match current_block_256 {
        6372208342811818303 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_256 = 17756757930076294871;
        }
        _ => {}
    }
    match current_block_256 {
        17756757930076294871 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_256 = 12692937682296807826;
        }
        _ => {}
    }
    match current_block_256 {
        12692937682296807826 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_256 = 6225258385525537431;
        }
        _ => {}
    }
    match current_block_256 {
        6225258385525537431 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_256 = 2018927983898164238;
        }
        _ => {}
    }
    match current_block_256 {
        2018927983898164238 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_256 = 4202242720334207286;
        }
        _ => {}
    }
    match current_block_256 {
        4202242720334207286 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_256 = 6323618852566759355;
        }
        _ => {}
    }
    match current_block_256 {
        6323618852566759355 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_256 = 7853761040057074878;
        }
        _ => {}
    }
    match current_block_256 {
        7853761040057074878 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_256 = 6142051208924225422;
        }
        _ => {}
    }
    match current_block_256 {
        6142051208924225422 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_256 = 12735356383178692136;
        }
        _ => {}
    }
    match current_block_256 {
        12735356383178692136 => {
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
    (*i).hh.hashv = _ha_hashv_0;
    (*i).hh.key = &mut (*i).key as *mut libc::c_int as *const libc::c_void;
    (*i)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if items.is_null() {
        (*i).hh.next = 0 as *mut libc::c_void;
        (*i).hh.prev = 0 as *mut libc::c_void;
        (*i)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*i).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*i).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*i).hh.tbl).tail = &mut (*i).hh;
            (*(*i).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*i).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*i).hh.tbl)
                .hho = (&mut (*i).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(i as *mut libc::c_char) as libc::c_long;
            (*(*i).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*i).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*i).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*i).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        items = i;
    } else {
        (*i).hh.tbl = (*items).hh.tbl;
        (*i).hh.next = 0 as *mut libc::c_void;
        (*i)
            .hh
            .prev = ((*(*items).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*items).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*items).hh.tbl).tail).next = i as *mut libc::c_void;
        (*(*items).hh.tbl).tail = &mut (*i).hh;
    }
    let mut _ha_bkt_0: libc::c_uint = 0;
    (*(*items).hh.tbl).num_items = ((*(*items).hh.tbl).num_items).wrapping_add(1);
    (*(*items).hh.tbl).num_items;
    _ha_bkt_0 = _ha_hashv_0
        & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*items).hh.tbl).buckets)
        .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
    (*_ha_head_0).count;
    (*i).hh.hh_next = (*_ha_head_0).hh_head;
    (*i).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_0).hh_head).is_null() {
        (*(*_ha_head_0).hh_head).hh_prev = &mut (*i).hh;
    }
    (*_ha_head_0).hh_head = &mut (*i).hh;
    if (*_ha_head_0).count
        >= ((*_ha_head_0).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*i).hh.tbl).noexpand == 0
    {
        let mut _he_bkt_0: libc::c_uint = 0;
        let mut _he_bkt_i_0: libc::c_uint = 0;
        let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_0 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*i).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_0.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_0 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*i).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*i).hh.tbl)
                .ideal_chain_maxlen = ((*(*i).hh.tbl).num_items
                >> ((*(*i).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*i).hh.tbl).num_items
                        & ((*(*i).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*i).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_0 < (*(*i).hh.tbl).num_buckets {
                _he_thh_0 = (*((*(*i).hh.tbl).buckets).offset(_he_bkt_i_0 as isize))
                    .hh_head;
                while !_he_thh_0.is_null() {
                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                    _he_bkt_0 = (*_he_thh_0).hashv
                        & ((*(*i).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                    if (*_he_newbkt_0).count > (*(*i).hh.tbl).ideal_chain_maxlen {
                        (*(*i).hh.tbl)
                            .nonideal_items = ((*(*i).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*i).hh.tbl).nonideal_items;
                        if (*_he_newbkt_0).count
                            > ((*_he_newbkt_0).expand_mult)
                                .wrapping_mul((*(*i).hh.tbl).ideal_chain_maxlen)
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
            free((*(*i).hh.tbl).buckets as *mut libc::c_void);
            (*(*i).hh.tbl)
                .num_buckets = ((*(*i).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*i).hh.tbl)
                .log2_num_buckets = ((*(*i).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*i).hh.tbl).log2_num_buckets;
            (*(*i).hh.tbl).buckets = _he_new_buckets_0;
            (*(*i).hh.tbl)
                .ineff_expands = if (*(*i).hh.tbl).nonideal_items
                > (*(*i).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*i).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*i).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*i).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    k = 98765 as libc::c_int;
    i = malloc(::std::mem::size_of::<item>() as libc::c_ulong) as *mut item;
    if i.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*i).key = k;
    (*i).data = 0 as libc::c_int;
    let mut _ha_hashv_1: libc::c_uint = 0;
    let mut _hj_i_1: libc::c_uint = 0;
    let mut _hj_j_1: libc::c_uint = 0;
    let mut _hj_k_1: libc::c_uint = 0;
    let mut _hj_key_1: *const libc::c_uchar = &mut (*i).key as *mut libc::c_int
        as *const libc::c_uchar;
    _ha_hashv_1 = 0xfeedbeef as libc::c_uint;
    _hj_j_1 = 0x9e3779b9 as libc::c_uint;
    _hj_i_1 = _hj_j_1;
    _hj_k_1 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    while _hj_k_1 >= 12 as libc::c_uint {
        _hj_i_1 = _hj_i_1
            .wrapping_add(
                (*_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_1.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j_1 = _hj_j_1
            .wrapping_add(
                (*_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_1.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _ha_hashv_1 = _ha_hashv_1
            .wrapping_add(
                (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_1.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_1);
        _hj_i_1 ^= _ha_hashv_1 >> 13 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
        _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_i_1);
        _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_j_1);
        _ha_hashv_1 ^= _hj_j_1 >> 13 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_1);
        _hj_i_1 ^= _ha_hashv_1 >> 12 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
        _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_i_1);
        _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_j_1);
        _ha_hashv_1 ^= _hj_j_1 >> 5 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_1);
        _hj_i_1 ^= _ha_hashv_1 >> 3 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
        _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_i_1);
        _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_j_1);
        _ha_hashv_1 ^= _hj_j_1 >> 15 as libc::c_int;
        _hj_key_1 = _hj_key_1.offset(12 as libc::c_int as isize);
        _hj_k_1 = _hj_k_1.wrapping_sub(12 as libc::c_uint);
    }
    _ha_hashv_1 = _ha_hashv_1
        .wrapping_add(
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_454: u64;
    match _hj_k_1 {
        11 => {
            _ha_hashv_1 = _ha_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_454 = 5659211898862597072;
        }
        10 => {
            current_block_454 = 5659211898862597072;
        }
        9 => {
            current_block_454 = 10052327690419870037;
        }
        8 => {
            current_block_454 = 17700659406875415849;
        }
        7 => {
            current_block_454 = 9505829889477478441;
        }
        6 => {
            current_block_454 = 4867305559695474996;
        }
        5 => {
            current_block_454 = 1027629641173791562;
        }
        4 => {
            current_block_454 = 16651643841640744462;
        }
        3 => {
            current_block_454 = 8412500553708127522;
        }
        2 => {
            current_block_454 = 8086359396125652250;
        }
        1 => {
            current_block_454 = 16761401835556897358;
        }
        _ => {
            current_block_454 = 1137393634913614354;
        }
    }
    match current_block_454 {
        5659211898862597072 => {
            _ha_hashv_1 = _ha_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_454 = 10052327690419870037;
        }
        _ => {}
    }
    match current_block_454 {
        10052327690419870037 => {
            _ha_hashv_1 = _ha_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_454 = 17700659406875415849;
        }
        _ => {}
    }
    match current_block_454 {
        17700659406875415849 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_454 = 9505829889477478441;
        }
        _ => {}
    }
    match current_block_454 {
        9505829889477478441 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_454 = 4867305559695474996;
        }
        _ => {}
    }
    match current_block_454 {
        4867305559695474996 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_454 = 1027629641173791562;
        }
        _ => {}
    }
    match current_block_454 {
        1027629641173791562 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    *_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_454 = 16651643841640744462;
        }
        _ => {}
    }
    match current_block_454 {
        16651643841640744462 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_454 = 8412500553708127522;
        }
        _ => {}
    }
    match current_block_454 {
        8412500553708127522 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_454 = 8086359396125652250;
        }
        _ => {}
    }
    match current_block_454 {
        8086359396125652250 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_454 = 16761401835556897358;
        }
        _ => {}
    }
    match current_block_454 {
        16761401835556897358 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    *_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_1);
    _hj_i_1 ^= _ha_hashv_1 >> 13 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_1);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
    _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_i_1);
    _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_j_1);
    _ha_hashv_1 ^= _hj_j_1 >> 13 as libc::c_int;
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_1);
    _hj_i_1 ^= _ha_hashv_1 >> 12 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_1);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
    _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_i_1);
    _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_j_1);
    _ha_hashv_1 ^= _hj_j_1 >> 5 as libc::c_int;
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_1);
    _hj_i_1 ^= _ha_hashv_1 >> 3 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_1);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
    _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_i_1);
    _ha_hashv_1 = _ha_hashv_1.wrapping_sub(_hj_j_1);
    _ha_hashv_1 ^= _hj_j_1 >> 15 as libc::c_int;
    (*i).hh.hashv = _ha_hashv_1;
    (*i).hh.key = &mut (*i).key as *mut libc::c_int as *const libc::c_void;
    (*i)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if items.is_null() {
        (*i).hh.next = 0 as *mut libc::c_void;
        (*i).hh.prev = 0 as *mut libc::c_void;
        (*i)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*i).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*i).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*i).hh.tbl).tail = &mut (*i).hh;
            (*(*i).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*i).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*i).hh.tbl)
                .hho = (&mut (*i).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(i as *mut libc::c_char) as libc::c_long;
            (*(*i).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*i).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*i).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*i).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        items = i;
    } else {
        (*i).hh.tbl = (*items).hh.tbl;
        (*i).hh.next = 0 as *mut libc::c_void;
        (*i)
            .hh
            .prev = ((*(*items).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*items).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*items).hh.tbl).tail).next = i as *mut libc::c_void;
        (*(*items).hh.tbl).tail = &mut (*i).hh;
    }
    let mut _ha_bkt_1: libc::c_uint = 0;
    (*(*items).hh.tbl).num_items = ((*(*items).hh.tbl).num_items).wrapping_add(1);
    (*(*items).hh.tbl).num_items;
    _ha_bkt_1 = _ha_hashv_1
        & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_1: *mut UT_hash_bucket = &mut *((*(*items).hh.tbl).buckets)
        .offset(_ha_bkt_1 as isize) as *mut UT_hash_bucket;
    (*_ha_head_1).count = ((*_ha_head_1).count).wrapping_add(1);
    (*_ha_head_1).count;
    (*i).hh.hh_next = (*_ha_head_1).hh_head;
    (*i).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_1).hh_head).is_null() {
        (*(*_ha_head_1).hh_head).hh_prev = &mut (*i).hh;
    }
    (*_ha_head_1).hh_head = &mut (*i).hh;
    if (*_ha_head_1).count
        >= ((*_ha_head_1).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*i).hh.tbl).noexpand == 0
    {
        let mut _he_bkt_1: libc::c_uint = 0;
        let mut _he_bkt_i_1: libc::c_uint = 0;
        let mut _he_thh_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_1 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*i).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_1.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_1 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*i).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*i).hh.tbl)
                .ideal_chain_maxlen = ((*(*i).hh.tbl).num_items
                >> ((*(*i).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*i).hh.tbl).num_items
                        & ((*(*i).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*i).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_1 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_1 < (*(*i).hh.tbl).num_buckets {
                _he_thh_1 = (*((*(*i).hh.tbl).buckets).offset(_he_bkt_i_1 as isize))
                    .hh_head;
                while !_he_thh_1.is_null() {
                    _he_hh_nxt_1 = (*_he_thh_1).hh_next;
                    _he_bkt_1 = (*_he_thh_1).hashv
                        & ((*(*i).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_1 = &mut *_he_new_buckets_1.offset(_he_bkt_1 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_1).count = ((*_he_newbkt_1).count).wrapping_add(1);
                    if (*_he_newbkt_1).count > (*(*i).hh.tbl).ideal_chain_maxlen {
                        (*(*i).hh.tbl)
                            .nonideal_items = ((*(*i).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*i).hh.tbl).nonideal_items;
                        if (*_he_newbkt_1).count
                            > ((*_he_newbkt_1).expand_mult)
                                .wrapping_mul((*(*i).hh.tbl).ideal_chain_maxlen)
                        {
                            (*_he_newbkt_1)
                                .expand_mult = ((*_he_newbkt_1).expand_mult)
                                .wrapping_add(1);
                            (*_he_newbkt_1).expand_mult;
                        }
                    }
                    (*_he_thh_1).hh_prev = 0 as *mut UT_hash_handle;
                    (*_he_thh_1).hh_next = (*_he_newbkt_1).hh_head;
                    if !((*_he_newbkt_1).hh_head).is_null() {
                        (*(*_he_newbkt_1).hh_head).hh_prev = _he_thh_1;
                    }
                    (*_he_newbkt_1).hh_head = _he_thh_1;
                    _he_thh_1 = _he_hh_nxt_1;
                }
                _he_bkt_i_1 = _he_bkt_i_1.wrapping_add(1);
                _he_bkt_i_1;
            }
            free((*(*i).hh.tbl).buckets as *mut libc::c_void);
            (*(*i).hh.tbl)
                .num_buckets = ((*(*i).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*i).hh.tbl)
                .log2_num_buckets = ((*(*i).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*i).hh.tbl).log2_num_buckets;
            (*(*i).hh.tbl).buckets = _he_new_buckets_1;
            (*(*i).hh.tbl)
                .ineff_expands = if (*(*i).hh.tbl).nonideal_items
                > (*(*i).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*i).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*i).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*i).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    k = 12345 as libc::c_int;
    j = 0 as *mut item;
    if !items.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_2: libc::c_uint = 0;
        let mut _hj_j_2: libc::c_uint = 0;
        let mut _hj_k_2: libc::c_uint = 0;
        let mut _hj_key_2: *const libc::c_uchar = &mut k as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j_2 = 0x9e3779b9 as libc::c_uint;
        _hj_i_2 = _hj_j_2;
        _hj_k_2 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k_2 >= 12 as libc::c_uint {
            _hj_i_2 = _hj_i_2
                .wrapping_add(
                    (*_hj_key_2.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_2.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_j_2 = _hj_j_2
                .wrapping_add(
                    (*_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_2.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key_2.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_2.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(11 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv);
            _hj_i_2 ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_2);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_2);
            _hf_hashv ^= _hj_j_2 >> 13 as libc::c_int;
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv);
            _hj_i_2 ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_2);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_2);
            _hf_hashv ^= _hj_j_2 >> 5 as libc::c_int;
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv);
            _hj_i_2 ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_2);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_2);
            _hf_hashv ^= _hj_j_2 >> 15 as libc::c_int;
            _hj_key_2 = _hj_key_2.offset(12 as libc::c_int as isize);
            _hj_k_2 = _hj_k_2.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_647: u64;
        match _hj_k_2 {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_2.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_647 = 11429293225007286200;
            }
            10 => {
                current_block_647 = 11429293225007286200;
            }
            9 => {
                current_block_647 = 13265596162993994991;
            }
            8 => {
                current_block_647 = 17434292904315486127;
            }
            7 => {
                current_block_647 = 2280485322995220430;
            }
            6 => {
                current_block_647 = 12207942584542961383;
            }
            5 => {
                current_block_647 = 10213527007967979201;
            }
            4 => {
                current_block_647 = 7877609712483693419;
            }
            3 => {
                current_block_647 = 11026693166592709250;
            }
            2 => {
                current_block_647 = 8663876389390588228;
            }
            1 => {
                current_block_647 = 2721638304048114319;
            }
            _ => {
                current_block_647 = 3755746800816054816;
            }
        }
        match current_block_647 {
            11429293225007286200 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_2.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_647 = 13265596162993994991;
            }
            _ => {}
        }
        match current_block_647 {
            13265596162993994991 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_2.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_647 = 17434292904315486127;
            }
            _ => {}
        }
        match current_block_647 {
            17434292904315486127 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_647 = 2280485322995220430;
            }
            _ => {}
        }
        match current_block_647 {
            2280485322995220430 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_647 = 12207942584542961383;
            }
            _ => {}
        }
        match current_block_647 {
            12207942584542961383 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_647 = 10213527007967979201;
            }
            _ => {}
        }
        match current_block_647 {
            10213527007967979201 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        *_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_647 = 7877609712483693419;
            }
            _ => {}
        }
        match current_block_647 {
            7877609712483693419 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_647 = 11026693166592709250;
            }
            _ => {}
        }
        match current_block_647 {
            11026693166592709250 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_647 = 8663876389390588228;
            }
            _ => {}
        }
        match current_block_647 {
            8663876389390588228 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_647 = 2721638304048114319;
            }
            _ => {}
        }
        match current_block_647 {
            2721638304048114319 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        *_hj_key_2.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv);
        _hj_i_2 ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_2);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_2);
        _hf_hashv ^= _hj_j_2 >> 13 as libc::c_int;
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv);
        _hj_i_2 ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_2);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_2);
        _hf_hashv ^= _hj_j_2 >> 5 as libc::c_int;
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv);
        _hj_i_2 ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_2);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_2);
        _hf_hashv ^= _hj_j_2 >> 15 as libc::c_int;
        j = 0 as *mut item;
        if !items.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*items).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    j = ((*((*(*items).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                        as *mut libc::c_char)
                        .offset(-((*(*items).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut item;
                } else {
                    j = 0 as *mut item;
                }
                while !j.is_null() {
                    if (*j).hh.hashv == _hf_hashv
                        && (*j).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*j).hh.key,
                            &mut k as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*j).hh.hh_next).is_null() {
                        j = ((*j).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*items).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut item;
                    } else {
                        j = 0 as *mut item;
                    }
                }
            }
        }
    }
    if !j.is_null() {
        printf(b"found %d\n\0" as *const u8 as *const libc::c_char, k);
    }
    k = 6789 as libc::c_int;
    j = 0 as *mut item;
    if !items.is_null() {
        let mut _hf_hashv_0: libc::c_uint = 0;
        let mut _hj_i_3: libc::c_uint = 0;
        let mut _hj_j_3: libc::c_uint = 0;
        let mut _hj_k_3: libc::c_uint = 0;
        let mut _hj_key_3: *const libc::c_uchar = &mut k as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv_0 = 0xfeedbeef as libc::c_uint;
        _hj_j_3 = 0x9e3779b9 as libc::c_uint;
        _hj_i_3 = _hj_j_3;
        _hj_k_3 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k_3 >= 12 as libc::c_uint {
            _hj_i_3 = _hj_i_3
                .wrapping_add(
                    (*_hj_key_3.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_3.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_3.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_3.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_j_3 = _hj_j_3
                .wrapping_add(
                    (*_hj_key_3.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_3.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_3.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_3.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hf_hashv_0 = _hf_hashv_0
                .wrapping_add(
                    (*_hj_key_3.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_3.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_3.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_3.offset(11 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_0);
            _hj_i_3 ^= _hf_hashv_0 >> 13 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_0);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_3);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_3);
            _hf_hashv_0 ^= _hj_j_3 >> 13 as libc::c_int;
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_0);
            _hj_i_3 ^= _hf_hashv_0 >> 12 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_0);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_3);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_3);
            _hf_hashv_0 ^= _hj_j_3 >> 5 as libc::c_int;
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_0);
            _hj_i_3 ^= _hf_hashv_0 >> 3 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_0);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_3);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_3);
            _hf_hashv_0 ^= _hj_j_3 >> 15 as libc::c_int;
            _hj_key_3 = _hj_key_3.offset(12 as libc::c_int as isize);
            _hj_k_3 = _hj_k_3.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv_0 = _hf_hashv_0
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_769: u64;
        match _hj_k_3 {
            11 => {
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_3.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_769 = 1857025224403959886;
            }
            10 => {
                current_block_769 = 1857025224403959886;
            }
            9 => {
                current_block_769 = 14099984419294177675;
            }
            8 => {
                current_block_769 = 10876502116601478655;
            }
            7 => {
                current_block_769 = 1993567080563942064;
            }
            6 => {
                current_block_769 = 9635081749408746063;
            }
            5 => {
                current_block_769 = 4048417546045435912;
            }
            4 => {
                current_block_769 = 139255840664946815;
            }
            3 => {
                current_block_769 = 17954593875197965021;
            }
            2 => {
                current_block_769 = 13534938107135860449;
            }
            1 => {
                current_block_769 = 14514781131754033399;
            }
            _ => {
                current_block_769 = 13552990829748394779;
            }
        }
        match current_block_769 {
            1857025224403959886 => {
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_3.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_769 = 14099984419294177675;
            }
            _ => {}
        }
        match current_block_769 {
            14099984419294177675 => {
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_3.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_769 = 10876502116601478655;
            }
            _ => {}
        }
        match current_block_769 {
            10876502116601478655 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        (*_hj_key_3.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_769 = 1993567080563942064;
            }
            _ => {}
        }
        match current_block_769 {
            1993567080563942064 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        (*_hj_key_3.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_769 = 9635081749408746063;
            }
            _ => {}
        }
        match current_block_769 {
            9635081749408746063 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        (*_hj_key_3.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_769 = 4048417546045435912;
            }
            _ => {}
        }
        match current_block_769 {
            4048417546045435912 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        *_hj_key_3.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_769 = 139255840664946815;
            }
            _ => {}
        }
        match current_block_769 {
            139255840664946815 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        (*_hj_key_3.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_769 = 17954593875197965021;
            }
            _ => {}
        }
        match current_block_769 {
            17954593875197965021 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        (*_hj_key_3.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_769 = 13534938107135860449;
            }
            _ => {}
        }
        match current_block_769 {
            13534938107135860449 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        (*_hj_key_3.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_769 = 14514781131754033399;
            }
            _ => {}
        }
        match current_block_769 {
            14514781131754033399 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        *_hj_key_3.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_0);
        _hj_i_3 ^= _hf_hashv_0 >> 13 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_0);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_3);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_3);
        _hf_hashv_0 ^= _hj_j_3 >> 13 as libc::c_int;
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_0);
        _hj_i_3 ^= _hf_hashv_0 >> 12 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_0);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_3);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_3);
        _hf_hashv_0 ^= _hj_j_3 >> 5 as libc::c_int;
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_0);
        _hj_i_3 ^= _hf_hashv_0 >> 3 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_0);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_3);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_3);
        _hf_hashv_0 ^= _hj_j_3 >> 15 as libc::c_int;
        j = 0 as *mut item;
        if !items.is_null() {
            let mut _hf_bkt_0: libc::c_uint = 0;
            _hf_bkt_0 = _hf_hashv_0
                & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*items).hh.tbl).buckets).offset(_hf_bkt_0 as isize)).hh_head)
                    .is_null()
                {
                    j = ((*((*(*items).hh.tbl).buckets).offset(_hf_bkt_0 as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*items).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut item;
                } else {
                    j = 0 as *mut item;
                }
                while !j.is_null() {
                    if (*j).hh.hashv == _hf_hashv_0
                        && (*j).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*j).hh.key,
                            &mut k as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*j).hh.hh_next).is_null() {
                        j = ((*j).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*items).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut item;
                    } else {
                        j = 0 as *mut item;
                    }
                }
            }
        }
    }
    if !j.is_null() {
        printf(b"found %d\n\0" as *const u8 as *const libc::c_char, k);
    }
    k = 98765 as libc::c_int;
    j = 0 as *mut item;
    if !items.is_null() {
        let mut _hf_hashv_1: libc::c_uint = 0;
        let mut _hj_i_4: libc::c_uint = 0;
        let mut _hj_j_4: libc::c_uint = 0;
        let mut _hj_k_4: libc::c_uint = 0;
        let mut _hj_key_4: *const libc::c_uchar = &mut k as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv_1 = 0xfeedbeef as libc::c_uint;
        _hj_j_4 = 0x9e3779b9 as libc::c_uint;
        _hj_i_4 = _hj_j_4;
        _hj_k_4 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k_4 >= 12 as libc::c_uint {
            _hj_i_4 = _hj_i_4
                .wrapping_add(
                    (*_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_4.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_j_4 = _hj_j_4
                .wrapping_add(
                    (*_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_4.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hf_hashv_1 = _hf_hashv_1
                .wrapping_add(
                    (*_hj_key_4.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_4.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(11 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_1);
            _hj_i_4 ^= _hf_hashv_1 >> 13 as libc::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_1);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_4);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_4);
            _hf_hashv_1 ^= _hj_j_4 >> 13 as libc::c_int;
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_1);
            _hj_i_4 ^= _hf_hashv_1 >> 12 as libc::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_1);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_4);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_4);
            _hf_hashv_1 ^= _hj_j_4 >> 5 as libc::c_int;
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_1);
            _hj_i_4 ^= _hf_hashv_1 >> 3 as libc::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_1);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_4);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_4);
            _hf_hashv_1 ^= _hj_j_4 >> 15 as libc::c_int;
            _hj_key_4 = _hj_key_4.offset(12 as libc::c_int as isize);
            _hj_k_4 = _hj_k_4.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv_1 = _hf_hashv_1
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_891: u64;
        match _hj_k_4 {
            11 => {
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_4.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_891 = 3076503085047436559;
            }
            10 => {
                current_block_891 = 3076503085047436559;
            }
            9 => {
                current_block_891 = 9872773783459987952;
            }
            8 => {
                current_block_891 = 7466475382891248240;
            }
            7 => {
                current_block_891 = 4878894749840974462;
            }
            6 => {
                current_block_891 = 11138754234040523364;
            }
            5 => {
                current_block_891 = 9955782189060567682;
            }
            4 => {
                current_block_891 = 6261178120443034396;
            }
            3 => {
                current_block_891 = 9688527801505696160;
            }
            2 => {
                current_block_891 = 13582656083946270270;
            }
            1 => {
                current_block_891 = 10971651906497889248;
            }
            _ => {
                current_block_891 = 10458606196841421391;
            }
        }
        match current_block_891 {
            3076503085047436559 => {
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_4.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_891 = 9872773783459987952;
            }
            _ => {}
        }
        match current_block_891 {
            9872773783459987952 => {
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_4.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_891 = 7466475382891248240;
            }
            _ => {}
        }
        match current_block_891 {
            7466475382891248240 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        (*_hj_key_4.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_891 = 4878894749840974462;
            }
            _ => {}
        }
        match current_block_891 {
            4878894749840974462 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        (*_hj_key_4.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_891 = 11138754234040523364;
            }
            _ => {}
        }
        match current_block_891 {
            11138754234040523364 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        (*_hj_key_4.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_891 = 9955782189060567682;
            }
            _ => {}
        }
        match current_block_891 {
            9955782189060567682 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        *_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_891 = 6261178120443034396;
            }
            _ => {}
        }
        match current_block_891 {
            6261178120443034396 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        (*_hj_key_4.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_891 = 9688527801505696160;
            }
            _ => {}
        }
        match current_block_891 {
            9688527801505696160 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        (*_hj_key_4.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_891 = 13582656083946270270;
            }
            _ => {}
        }
        match current_block_891 {
            13582656083946270270 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        (*_hj_key_4.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_891 = 10971651906497889248;
            }
            _ => {}
        }
        match current_block_891 {
            10971651906497889248 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        *_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_1);
        _hj_i_4 ^= _hf_hashv_1 >> 13 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_1);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_4);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_4);
        _hf_hashv_1 ^= _hj_j_4 >> 13 as libc::c_int;
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_1);
        _hj_i_4 ^= _hf_hashv_1 >> 12 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_1);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_4);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_4);
        _hf_hashv_1 ^= _hj_j_4 >> 5 as libc::c_int;
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_1);
        _hj_i_4 ^= _hf_hashv_1 >> 3 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_1);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_4);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_4);
        _hf_hashv_1 ^= _hj_j_4 >> 15 as libc::c_int;
        j = 0 as *mut item;
        if !items.is_null() {
            let mut _hf_bkt_1: libc::c_uint = 0;
            _hf_bkt_1 = _hf_hashv_1
                & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*items).hh.tbl).buckets).offset(_hf_bkt_1 as isize)).hh_head)
                    .is_null()
                {
                    j = ((*((*(*items).hh.tbl).buckets).offset(_hf_bkt_1 as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*items).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut item;
                } else {
                    j = 0 as *mut item;
                }
                while !j.is_null() {
                    if (*j).hh.hashv == _hf_hashv_1
                        && (*j).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*j).hh.key,
                            &mut k as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*j).hh.hh_next).is_null() {
                        j = ((*j).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*items).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut item;
                    } else {
                        j = 0 as *mut item;
                    }
                }
            }
        }
    }
    if !j.is_null() {
        printf(b"found %d\n\0" as *const u8 as *const libc::c_char, k);
    }
    j = items;
    while !j.is_null() {
        printf(b"deleting %d\n\0" as *const u8 as *const libc::c_char, (*j).key);
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*j).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*items).hh.tbl).buckets as *mut libc::c_void);
            free((*items).hh.tbl as *mut libc::c_void);
            items = 0 as *mut item;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*items).hh.tbl).tail {
                (*(*items).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*items).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*items).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                items = (*_hd_hh_del).next as *mut item;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*items).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*items).hh.tbl).buckets)
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
            (*(*items).hh.tbl)
                .num_items = ((*(*items).hh.tbl).num_items).wrapping_sub(1);
            (*(*items).hh.tbl).num_items;
        }
        j = (*j).hh.next as *mut item;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

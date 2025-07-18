use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    pub name: [libc::c_char; 10],
    pub sub: *mut item,
    pub val: libc::c_int,
    pub hh: UT_hash_handle,
}
pub type item_t = item;
unsafe fn main_0() -> libc::c_int {
    let mut item1: *mut item_t = 0 as *mut item_t;
    let mut item2: *mut item_t = 0 as *mut item_t;
    let mut tmp1: *mut item_t = 0 as *mut item_t;
    let mut tmp2: *mut item_t = 0 as *mut item_t;
    let mut items: *mut item_t = 0 as *mut item_t;
    let mut i: *mut item_t = malloc(::std::mem::size_of::<item_t>() as libc::c_ulong)
        as *mut item_t;
    if i.is_null() {
        exit(-(1 as libc::c_int));
    }
    strcpy(((*i).name).as_mut_ptr(), b"bob\0" as *const u8 as *const libc::c_char);
    (*i).sub = 0 as *mut item;
    (*i).val = 0 as libc::c_int;
    let mut _uthash_hastr_keylen: libc::c_uint = strlen(((*i).name).as_mut_ptr())
        as libc::c_uint;
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut *((*i).name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = _uthash_hastr_keylen;
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
    _ha_hashv = _ha_hashv.wrapping_add(_uthash_hastr_keylen);
    let mut current_block_57: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 4417942540951631311;
        }
        10 => {
            current_block_57 = 4417942540951631311;
        }
        9 => {
            current_block_57 = 4180356275209017683;
        }
        8 => {
            current_block_57 = 12820147105077459416;
        }
        7 => {
            current_block_57 = 1321270241106104922;
        }
        6 => {
            current_block_57 = 13605335638665870143;
        }
        5 => {
            current_block_57 = 10178099981017662987;
        }
        4 => {
            current_block_57 = 782278476146786725;
        }
        3 => {
            current_block_57 = 16778187160894002461;
        }
        2 => {
            current_block_57 = 13115695204126317217;
        }
        1 => {
            current_block_57 = 10417353277134363042;
        }
        _ => {
            current_block_57 = 2989495919056355252;
        }
    }
    match current_block_57 {
        4417942540951631311 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 4180356275209017683;
        }
        _ => {}
    }
    match current_block_57 {
        4180356275209017683 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 12820147105077459416;
        }
        _ => {}
    }
    match current_block_57 {
        12820147105077459416 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 1321270241106104922;
        }
        _ => {}
    }
    match current_block_57 {
        1321270241106104922 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 13605335638665870143;
        }
        _ => {}
    }
    match current_block_57 {
        13605335638665870143 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 10178099981017662987;
        }
        _ => {}
    }
    match current_block_57 {
        10178099981017662987 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_57 = 782278476146786725;
        }
        _ => {}
    }
    match current_block_57 {
        782278476146786725 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 16778187160894002461;
        }
        _ => {}
    }
    match current_block_57 {
        16778187160894002461 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 13115695204126317217;
        }
        _ => {}
    }
    match current_block_57 {
        13115695204126317217 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 10417353277134363042;
        }
        _ => {}
    }
    match current_block_57 {
        10417353277134363042 => {
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
    (*i)
        .hh
        .key = &mut *((*i).name).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char as *const libc::c_void;
    (*i).hh.keylen = _uthash_hastr_keylen;
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
    let mut s: *mut item_t = malloc(::std::mem::size_of::<item_t>() as libc::c_ulong)
        as *mut item_t;
    if s.is_null() {
        exit(-(1 as libc::c_int));
    }
    strcpy(((*s).name).as_mut_ptr(), b"age\0" as *const u8 as *const libc::c_char);
    (*s).sub = 0 as *mut item;
    (*s).val = 37 as libc::c_int;
    let mut _uthash_hastr_keylen_0: libc::c_uint = strlen(((*s).name).as_mut_ptr())
        as libc::c_uint;
    let mut _ha_hashv_0: libc::c_uint = 0;
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *const libc::c_uchar = &mut *((*s).name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_uchar;
    _ha_hashv_0 = 0xfeedbeef as libc::c_uint;
    _hj_j_0 = 0x9e3779b9 as libc::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = _uthash_hastr_keylen_0;
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
    _ha_hashv_0 = _ha_hashv_0.wrapping_add(_uthash_hastr_keylen_0);
    let mut current_block_256: u64;
    match _hj_k_0 {
        11 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_256 = 2282256267574538215;
        }
        10 => {
            current_block_256 = 2282256267574538215;
        }
        9 => {
            current_block_256 = 6010056518000876263;
        }
        8 => {
            current_block_256 = 1752605773408826991;
        }
        7 => {
            current_block_256 = 1283995450065962895;
        }
        6 => {
            current_block_256 = 488614886186827655;
        }
        5 => {
            current_block_256 = 9547857900946778638;
        }
        4 => {
            current_block_256 = 9100868043760823246;
        }
        3 => {
            current_block_256 = 13811281175036752192;
        }
        2 => {
            current_block_256 = 16115749157586614201;
        }
        1 => {
            current_block_256 = 7846608097676908463;
        }
        _ => {
            current_block_256 = 18340277188286182087;
        }
    }
    match current_block_256 {
        2282256267574538215 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_256 = 6010056518000876263;
        }
        _ => {}
    }
    match current_block_256 {
        6010056518000876263 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_256 = 1752605773408826991;
        }
        _ => {}
    }
    match current_block_256 {
        1752605773408826991 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_256 = 1283995450065962895;
        }
        _ => {}
    }
    match current_block_256 {
        1283995450065962895 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_256 = 488614886186827655;
        }
        _ => {}
    }
    match current_block_256 {
        488614886186827655 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_256 = 9547857900946778638;
        }
        _ => {}
    }
    match current_block_256 {
        9547857900946778638 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_256 = 9100868043760823246;
        }
        _ => {}
    }
    match current_block_256 {
        9100868043760823246 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_256 = 13811281175036752192;
        }
        _ => {}
    }
    match current_block_256 {
        13811281175036752192 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_256 = 16115749157586614201;
        }
        _ => {}
    }
    match current_block_256 {
        16115749157586614201 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_256 = 7846608097676908463;
        }
        _ => {}
    }
    match current_block_256 {
        7846608097676908463 => {
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
    (*s).hh.hashv = _ha_hashv_0;
    (*s)
        .hh
        .key = &mut *((*s).name).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char as *const libc::c_void;
    (*s).hh.keylen = _uthash_hastr_keylen_0;
    if ((*i).sub).is_null() {
        (*s).hh.next = 0 as *mut libc::c_void;
        (*s).hh.prev = 0 as *mut libc::c_void;
        (*s)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*s).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*s).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*s).hh.tbl).tail = &mut (*s).hh;
            (*(*s).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*s).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*s).hh.tbl)
                .hho = (&mut (*s).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(s as *mut libc::c_char) as libc::c_long;
            (*(*s).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*s).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*s).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*s).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        (*i).sub = s;
    } else {
        (*s).hh.tbl = (*(*i).sub).hh.tbl;
        (*s).hh.next = 0 as *mut libc::c_void;
        (*s)
            .hh
            .prev = ((*(*(*i).sub).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*(*i).sub).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*(*i).sub).hh.tbl).tail).next = s as *mut libc::c_void;
        (*(*(*i).sub).hh.tbl).tail = &mut (*s).hh;
    }
    let mut _ha_bkt_0: libc::c_uint = 0;
    (*(*(*i).sub).hh.tbl).num_items = ((*(*(*i).sub).hh.tbl).num_items).wrapping_add(1);
    (*(*(*i).sub).hh.tbl).num_items;
    _ha_bkt_0 = _ha_hashv_0
        & ((*(*(*i).sub).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*(*i).sub).hh.tbl).buckets)
        .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
    (*_ha_head_0).count;
    (*s).hh.hh_next = (*_ha_head_0).hh_head;
    (*s).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_0).hh_head).is_null() {
        (*(*_ha_head_0).hh_head).hh_prev = &mut (*s).hh;
    }
    (*_ha_head_0).hh_head = &mut (*s).hh;
    if (*_ha_head_0).count
        >= ((*_ha_head_0).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*s).hh.tbl).noexpand == 0
    {
        let mut _he_bkt_0: libc::c_uint = 0;
        let mut _he_bkt_i_0: libc::c_uint = 0;
        let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_0 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*s).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_0.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_0 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*s).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*s).hh.tbl)
                .ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                >> ((*(*s).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*s).hh.tbl).num_items
                        & ((*(*s).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*s).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_0 < (*(*s).hh.tbl).num_buckets {
                _he_thh_0 = (*((*(*s).hh.tbl).buckets).offset(_he_bkt_i_0 as isize))
                    .hh_head;
                while !_he_thh_0.is_null() {
                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                    _he_bkt_0 = (*_he_thh_0).hashv
                        & ((*(*s).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                    if (*_he_newbkt_0).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                        (*(*s).hh.tbl)
                            .nonideal_items = ((*(*s).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*s).hh.tbl).nonideal_items;
                        if (*_he_newbkt_0).count
                            > ((*_he_newbkt_0).expand_mult)
                                .wrapping_mul((*(*s).hh.tbl).ideal_chain_maxlen)
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
            free((*(*s).hh.tbl).buckets as *mut libc::c_void);
            (*(*s).hh.tbl)
                .num_buckets = ((*(*s).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*s).hh.tbl)
                .log2_num_buckets = ((*(*s).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*s).hh.tbl).log2_num_buckets;
            (*(*s).hh.tbl).buckets = _he_new_buckets_0;
            (*(*s).hh.tbl)
                .ineff_expands = if (*(*s).hh.tbl).nonideal_items
                > (*(*s).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*s).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*s).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*s).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    item1 = items;
    tmp1 = (if !items.is_null() { (*items).hh.next } else { 0 as *mut libc::c_void })
        as *mut item_t;
    while !item1.is_null() {
        item2 = (*item1).sub;
        tmp2 = (if !((*item1).sub).is_null() {
            (*(*item1).sub).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut item_t;
        while !item2.is_null() {
            printf(
                b"$items{%s}{%s} = %d\n\0" as *const u8 as *const libc::c_char,
                ((*item1).name).as_mut_ptr(),
                ((*item2).name).as_mut_ptr(),
                (*item2).val,
            );
            item2 = tmp2;
            tmp2 = (if !tmp2.is_null() {
                (*tmp2).hh.next
            } else {
                0 as *mut libc::c_void
            }) as *mut item_t;
        }
        item1 = tmp1;
        tmp1 = (if !tmp1.is_null() { (*tmp1).hh.next } else { 0 as *mut libc::c_void })
            as *mut item_t;
    }
    item1 = items;
    tmp1 = (if !items.is_null() { (*items).hh.next } else { 0 as *mut libc::c_void })
        as *mut item_t;
    while !item1.is_null() {
        item2 = (*item1).sub;
        tmp2 = (if !((*item1).sub).is_null() {
            (*(*item1).sub).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut item_t;
        while !item2.is_null() {
            let mut _hd_hh_del: *mut UT_hash_handle = &mut (*item2).hh;
            if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
                free((*(*(*item1).sub).hh.tbl).buckets as *mut libc::c_void);
                free((*(*item1).sub).hh.tbl as *mut libc::c_void);
                (*item1).sub = 0 as *mut item;
            } else {
                let mut _hd_bkt: libc::c_uint = 0;
                if _hd_hh_del == (*(*(*item1).sub).hh.tbl).tail {
                    (*(*(*item1).sub).hh.tbl)
                        .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*(*item1).sub).hh.tbl).hho as isize)
                        as *mut libc::c_void as *mut UT_hash_handle;
                }
                if !((*_hd_hh_del).prev).is_null() {
                    let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*(*item1).sub).hh.tbl).hho as isize)
                        as *mut libc::c_void as *mut UT_hash_handle))
                        .next;
                    *fresh0 = (*_hd_hh_del).next;
                } else {
                    (*item1).sub = (*_hd_hh_del).next as *mut item;
                }
                if !((*_hd_hh_del).next).is_null() {
                    let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                        .offset((*(*(*item1).sub).hh.tbl).hho as isize)
                        as *mut libc::c_void as *mut UT_hash_handle))
                        .prev;
                    *fresh1 = (*_hd_hh_del).prev;
                }
                _hd_bkt = (*_hd_hh_del).hashv
                    & ((*(*(*item1).sub).hh.tbl).num_buckets)
                        .wrapping_sub(1 as libc::c_uint);
                let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*(*item1).sub).hh.tbl)
                    .buckets)
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
                (*(*(*item1).sub).hh.tbl)
                    .num_items = ((*(*(*item1).sub).hh.tbl).num_items).wrapping_sub(1);
                (*(*(*item1).sub).hh.tbl).num_items;
            }
            free(item2 as *mut libc::c_void);
            item2 = tmp2;
            tmp2 = (if !tmp2.is_null() {
                (*tmp2).hh.next
            } else {
                0 as *mut libc::c_void
            }) as *mut item_t;
        }
        let mut _hd_hh_del_0: *mut UT_hash_handle = &mut (*item1).hh;
        if ((*_hd_hh_del_0).prev).is_null() && ((*_hd_hh_del_0).next).is_null() {
            free((*(*items).hh.tbl).buckets as *mut libc::c_void);
            free((*items).hh.tbl as *mut libc::c_void);
            items = 0 as *mut item_t;
        } else {
            let mut _hd_bkt_0: libc::c_uint = 0;
            if _hd_hh_del_0 == (*(*items).hh.tbl).tail {
                (*(*items).hh.tbl)
                    .tail = ((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*items).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del_0).prev).is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*items).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del_0).next;
            } else {
                items = (*_hd_hh_del_0).next as *mut item_t;
            }
            if !((*_hd_hh_del_0).next).is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del_0).next as *mut libc::c_char)
                    .offset((*(*items).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & ((*(*items).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head_0: *mut UT_hash_bucket = &mut *((*(*items).hh.tbl).buckets)
                .offset(_hd_bkt_0 as isize) as *mut UT_hash_bucket;
            (*_hd_head_0).count = ((*_hd_head_0).count).wrapping_sub(1);
            (*_hd_head_0).count;
            if (*_hd_head_0).hh_head == _hd_hh_del_0 {
                (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next;
            }
            if !((*_hd_hh_del_0).hh_prev).is_null() {
                (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
            }
            if !((*_hd_hh_del_0).hh_next).is_null() {
                (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
            }
            (*(*items).hh.tbl)
                .num_items = ((*(*items).hh.tbl).num_items).wrapping_sub(1);
            (*(*items).hh.tbl).num_items;
        }
        free(item1 as *mut libc::c_void);
        item1 = tmp1;
        tmp1 = (if !tmp1.is_null() { (*tmp1).hh.next } else { 0 as *mut libc::c_void })
            as *mut item_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

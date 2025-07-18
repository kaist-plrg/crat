use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
pub struct my_struct {
    pub bkey: [libc::c_char; 5],
    pub data: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut s: *mut my_struct = 0 as *mut my_struct;
    let mut t: *mut my_struct = 0 as *mut my_struct;
    let mut bins: *mut my_struct = 0 as *mut my_struct;
    let mut binary: [libc::c_char; 5] = [
        '\u{3}' as i32 as libc::c_char,
        '\u{1}' as i32 as libc::c_char,
        '\u{4}' as i32 as libc::c_char,
        '\u{1}' as i32 as libc::c_char,
        '\u{6}' as i32 as libc::c_char,
    ];
    s = calloc(1 as libc::c_ulong, ::std::mem::size_of::<my_struct>() as libc::c_ulong)
        as *mut my_struct;
    if s.is_null() {
        exit(-(1 as libc::c_int));
    }
    memcpy(
        ((*s).bkey).as_mut_ptr() as *mut libc::c_void,
        binary.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
    );
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut (*s).bkey as *mut [libc::c_char; 5]
        as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as libc::c_uint;
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
            ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_56: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_56 = 12138864134669321211;
        }
        10 => {
            current_block_56 = 12138864134669321211;
        }
        9 => {
            current_block_56 = 406160567019899801;
        }
        8 => {
            current_block_56 = 2439367911297498807;
        }
        7 => {
            current_block_56 = 10750045426498873682;
        }
        6 => {
            current_block_56 = 4001380918227989591;
        }
        5 => {
            current_block_56 = 11642370719904150331;
        }
        4 => {
            current_block_56 = 17900618501041725375;
        }
        3 => {
            current_block_56 = 7601961152928474769;
        }
        2 => {
            current_block_56 = 13895280960623862409;
        }
        1 => {
            current_block_56 = 7696108116658018464;
        }
        _ => {
            current_block_56 = 2122094917359643297;
        }
    }
    match current_block_56 {
        12138864134669321211 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_56 = 406160567019899801;
        }
        _ => {}
    }
    match current_block_56 {
        406160567019899801 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_56 = 2439367911297498807;
        }
        _ => {}
    }
    match current_block_56 {
        2439367911297498807 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_56 = 10750045426498873682;
        }
        _ => {}
    }
    match current_block_56 {
        10750045426498873682 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_56 = 4001380918227989591;
        }
        _ => {}
    }
    match current_block_56 {
        4001380918227989591 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_56 = 11642370719904150331;
        }
        _ => {}
    }
    match current_block_56 {
        11642370719904150331 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_56 = 17900618501041725375;
        }
        _ => {}
    }
    match current_block_56 {
        17900618501041725375 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_56 = 7601961152928474769;
        }
        _ => {}
    }
    match current_block_56 {
        7601961152928474769 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_56 = 13895280960623862409;
        }
        _ => {}
    }
    match current_block_56 {
        13895280960623862409 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_56 = 7696108116658018464;
        }
        _ => {}
    }
    match current_block_56 {
        7696108116658018464 => {
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
    (*s).hh.hashv = _ha_hashv;
    (*s).hh.key = &mut (*s).bkey as *mut [libc::c_char; 5] as *const libc::c_void;
    (*s)
        .hh
        .keylen = ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
        as libc::c_uint;
    if bins.is_null() {
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
        bins = s;
    } else {
        (*s).hh.tbl = (*bins).hh.tbl;
        (*s).hh.next = 0 as *mut libc::c_void;
        (*s)
            .hh
            .prev = ((*(*bins).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*bins).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*bins).hh.tbl).tail).next = s as *mut libc::c_void;
        (*(*bins).hh.tbl).tail = &mut (*s).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*bins).hh.tbl).num_items = ((*(*bins).hh.tbl).num_items).wrapping_add(1);
    (*(*bins).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*bins).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*bins).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*s).hh.hh_next = (*_ha_head).hh_head;
    (*s).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*s).hh;
    }
    (*_ha_head).hh_head = &mut (*s).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*s).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*s).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
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
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                _he_thh = (*((*(*s).hh.tbl).buckets).offset(_he_bkt_i as isize)).hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*s).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                        (*(*s).hh.tbl)
                            .nonideal_items = ((*(*s).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*s).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*s).hh.tbl).ideal_chain_maxlen)
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
            free((*(*s).hh.tbl).buckets as *mut libc::c_void);
            (*(*s).hh.tbl)
                .num_buckets = ((*(*s).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*s).hh.tbl)
                .log2_num_buckets = ((*(*s).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*s).hh.tbl).log2_num_buckets;
            (*(*s).hh.tbl).buckets = _he_new_buckets;
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
    t = 0 as *mut my_struct;
    if !bins.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_0: libc::c_uint = 0;
        let mut _hj_j_0: libc::c_uint = 0;
        let mut _hj_k_0: libc::c_uint = 0;
        let mut _hj_key_0: *const libc::c_uchar = binary.as_mut_ptr()
            as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j_0 = 0x9e3779b9 as libc::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
            as libc::c_uint;
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
                ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as libc::c_uint,
            );
        let mut current_block_248: u64;
        match _hj_k_0 {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_248 = 11228796017659943257;
            }
            10 => {
                current_block_248 = 11228796017659943257;
            }
            9 => {
                current_block_248 = 3644872370092143722;
            }
            8 => {
                current_block_248 = 14810327539386637663;
            }
            7 => {
                current_block_248 = 16260099921281466515;
            }
            6 => {
                current_block_248 = 7534632980397467121;
            }
            5 => {
                current_block_248 = 5943209734910057546;
            }
            4 => {
                current_block_248 = 6757666663575783116;
            }
            3 => {
                current_block_248 = 14423724542182203168;
            }
            2 => {
                current_block_248 = 9779264176850003141;
            }
            1 => {
                current_block_248 = 6442902164396703799;
            }
            _ => {
                current_block_248 = 14557636130817708122;
            }
        }
        match current_block_248 {
            11228796017659943257 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_248 = 3644872370092143722;
            }
            _ => {}
        }
        match current_block_248 {
            3644872370092143722 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_248 = 14810327539386637663;
            }
            _ => {}
        }
        match current_block_248 {
            14810327539386637663 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_248 = 16260099921281466515;
            }
            _ => {}
        }
        match current_block_248 {
            16260099921281466515 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_248 = 7534632980397467121;
            }
            _ => {}
        }
        match current_block_248 {
            7534632980397467121 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_248 = 5943209734910057546;
            }
            _ => {}
        }
        match current_block_248 {
            5943209734910057546 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_248 = 6757666663575783116;
            }
            _ => {}
        }
        match current_block_248 {
            6757666663575783116 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_248 = 14423724542182203168;
            }
            _ => {}
        }
        match current_block_248 {
            14423724542182203168 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_248 = 9779264176850003141;
            }
            _ => {}
        }
        match current_block_248 {
            9779264176850003141 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_248 = 6442902164396703799;
            }
            _ => {}
        }
        match current_block_248 {
            6442902164396703799 => {
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
        t = 0 as *mut my_struct;
        if !bins.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*bins).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*bins).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    t = ((*((*(*bins).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                        as *mut libc::c_char)
                        .offset(-((*(*bins).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut my_struct;
                } else {
                    t = 0 as *mut my_struct;
                }
                while !t.is_null() {
                    if (*t).hh.hashv == _hf_hashv
                        && (*t).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<[libc::c_char; 5]>()
                                as libc::c_ulong
                    {
                        if memcmp(
                            (*t).hh.key,
                            binary.as_mut_ptr() as *const libc::c_void,
                            ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*t).hh.hh_next).is_null() {
                        t = ((*t).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*bins).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut my_struct;
                    } else {
                        t = 0 as *mut my_struct;
                    }
                }
            }
        }
    }
    if !t.is_null() {
        printf(b"found\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

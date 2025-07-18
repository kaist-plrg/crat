use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
}
pub type size_t = libc::c_ulong;
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
pub struct msg_t {
    pub hh: UT_hash_handle,
    pub len: size_t,
    pub encoding: libc::c_char,
    pub text: [libc::c_int; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookup_key_t {
    pub encoding: libc::c_char,
    pub text: [libc::c_int; 0],
}
unsafe fn main_0() -> libc::c_int {
    let mut keylen: libc::c_uint = 0;
    let mut msg: *mut msg_t = 0 as *mut msg_t;
    let mut tmp: *mut msg_t = 0 as *mut msg_t;
    let mut msgs: *mut msg_t = 0 as *mut msg_t;
    let mut lookup_key: *mut lookup_key_t = 0 as *mut lookup_key_t;
    let mut beijing: [libc::c_int; 2] = [0x5317 as libc::c_int, 0x4eac as libc::c_int];
    msg = malloc(
        (::std::mem::size_of::<msg_t>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong),
    ) as *mut msg_t;
    if msg.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        msg as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<msg_t>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong),
    );
    (*msg).len = ::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong;
    (*msg).encoding = '\u{1}' as i32 as libc::c_char;
    memcpy(
        ((*msg).text).as_mut_ptr() as *mut libc::c_void,
        beijing.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
    );
    keylen = (68 as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
        .wrapping_sub(64 as libc::c_ulong) as libc::c_uint;
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut (*msg).encoding as *mut libc::c_char
        as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = keylen;
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
    _ha_hashv = _ha_hashv.wrapping_add(keylen);
    let mut current_block_60: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_60 = 4463504010575437024;
        }
        10 => {
            current_block_60 = 4463504010575437024;
        }
        9 => {
            current_block_60 = 365507064735404242;
        }
        8 => {
            current_block_60 = 920473454035407359;
        }
        7 => {
            current_block_60 = 18253173546996597019;
        }
        6 => {
            current_block_60 = 4061546693912444135;
        }
        5 => {
            current_block_60 = 10136303058756343858;
        }
        4 => {
            current_block_60 = 12413672676664956391;
        }
        3 => {
            current_block_60 = 12732681208002987555;
        }
        2 => {
            current_block_60 = 12148659518933446462;
        }
        1 => {
            current_block_60 = 11316695763593360198;
        }
        _ => {
            current_block_60 = 14220266465818359136;
        }
    }
    match current_block_60 {
        4463504010575437024 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_60 = 365507064735404242;
        }
        _ => {}
    }
    match current_block_60 {
        365507064735404242 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_60 = 920473454035407359;
        }
        _ => {}
    }
    match current_block_60 {
        920473454035407359 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_60 = 18253173546996597019;
        }
        _ => {}
    }
    match current_block_60 {
        18253173546996597019 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_60 = 4061546693912444135;
        }
        _ => {}
    }
    match current_block_60 {
        4061546693912444135 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_60 = 10136303058756343858;
        }
        _ => {}
    }
    match current_block_60 {
        10136303058756343858 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_60 = 12413672676664956391;
        }
        _ => {}
    }
    match current_block_60 {
        12413672676664956391 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_60 = 12732681208002987555;
        }
        _ => {}
    }
    match current_block_60 {
        12732681208002987555 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_60 = 12148659518933446462;
        }
        _ => {}
    }
    match current_block_60 {
        12148659518933446462 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_60 = 11316695763593360198;
        }
        _ => {}
    }
    match current_block_60 {
        11316695763593360198 => {
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
    (*msg).hh.hashv = _ha_hashv;
    (*msg).hh.key = &mut (*msg).encoding as *mut libc::c_char as *const libc::c_void;
    (*msg).hh.keylen = keylen;
    if msgs.is_null() {
        (*msg).hh.next = 0 as *mut libc::c_void;
        (*msg).hh.prev = 0 as *mut libc::c_void;
        (*msg)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*msg).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*msg).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*msg).hh.tbl).tail = &mut (*msg).hh;
            (*(*msg).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*msg).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*msg).hh.tbl)
                .hho = (&mut (*msg).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(msg as *mut libc::c_char) as libc::c_long;
            (*(*msg).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*msg).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*msg).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*msg).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        msgs = msg;
    } else {
        (*msg).hh.tbl = (*msgs).hh.tbl;
        (*msg).hh.next = 0 as *mut libc::c_void;
        (*msg)
            .hh
            .prev = ((*(*msgs).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*msgs).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*msgs).hh.tbl).tail).next = msg as *mut libc::c_void;
        (*(*msgs).hh.tbl).tail = &mut (*msg).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*msgs).hh.tbl).num_items = ((*(*msgs).hh.tbl).num_items).wrapping_add(1);
    (*(*msgs).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*msgs).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*msgs).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*msg).hh.hh_next = (*_ha_head).hh_head;
    (*msg).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*msg).hh;
    }
    (*_ha_head).hh_head = &mut (*msg).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*msg).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*msg).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*msg).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*msg).hh.tbl)
                .ideal_chain_maxlen = ((*(*msg).hh.tbl).num_items
                >> ((*(*msg).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*msg).hh.tbl).num_items
                        & ((*(*msg).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*msg).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*msg).hh.tbl).num_buckets {
                _he_thh = (*((*(*msg).hh.tbl).buckets).offset(_he_bkt_i as isize))
                    .hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*msg).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*msg).hh.tbl).ideal_chain_maxlen {
                        (*(*msg).hh.tbl)
                            .nonideal_items = ((*(*msg).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*msg).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*msg).hh.tbl).ideal_chain_maxlen)
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
            free((*(*msg).hh.tbl).buckets as *mut libc::c_void);
            (*(*msg).hh.tbl)
                .num_buckets = ((*(*msg).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*msg).hh.tbl)
                .log2_num_buckets = ((*(*msg).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*msg).hh.tbl).log2_num_buckets;
            (*(*msg).hh.tbl).buckets = _he_new_buckets;
            (*(*msg).hh.tbl)
                .ineff_expands = if (*(*msg).hh.tbl).nonideal_items
                > (*(*msg).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*msg).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*msg).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*msg).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    msg = 0 as *mut msg_t;
    lookup_key = malloc(
        (::std::mem::size_of::<lookup_key_t>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong),
    ) as *mut lookup_key_t;
    if lookup_key.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        lookup_key as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<lookup_key_t>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong),
    );
    (*lookup_key).encoding = '\u{1}' as i32 as libc::c_char;
    memcpy(
        ((*lookup_key).text).as_mut_ptr() as *mut libc::c_void,
        beijing.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
    );
    msg = 0 as *mut msg_t;
    if !msgs.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_0: libc::c_uint = 0;
        let mut _hj_j_0: libc::c_uint = 0;
        let mut _hj_k_0: libc::c_uint = 0;
        let mut _hj_key_0: *const libc::c_uchar = &mut (*lookup_key).encoding
            as *mut libc::c_char as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j_0 = 0x9e3779b9 as libc::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = keylen;
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
        _hf_hashv = _hf_hashv.wrapping_add(keylen);
        let mut current_block_260: u64;
        match _hj_k_0 {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_260 = 2768525010212484757;
            }
            10 => {
                current_block_260 = 2768525010212484757;
            }
            9 => {
                current_block_260 = 14507743300660289476;
            }
            8 => {
                current_block_260 = 9596750539301179110;
            }
            7 => {
                current_block_260 = 8641749324128961562;
            }
            6 => {
                current_block_260 = 6204983741691654886;
            }
            5 => {
                current_block_260 = 14371489722196013220;
            }
            4 => {
                current_block_260 = 8514979938056295568;
            }
            3 => {
                current_block_260 = 15955263502462794145;
            }
            2 => {
                current_block_260 = 9617609482120873840;
            }
            1 => {
                current_block_260 = 1725138493261267529;
            }
            _ => {
                current_block_260 = 2956103269722328116;
            }
        }
        match current_block_260 {
            2768525010212484757 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_260 = 14507743300660289476;
            }
            _ => {}
        }
        match current_block_260 {
            14507743300660289476 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_260 = 9596750539301179110;
            }
            _ => {}
        }
        match current_block_260 {
            9596750539301179110 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_260 = 8641749324128961562;
            }
            _ => {}
        }
        match current_block_260 {
            8641749324128961562 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_260 = 6204983741691654886;
            }
            _ => {}
        }
        match current_block_260 {
            6204983741691654886 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_260 = 14371489722196013220;
            }
            _ => {}
        }
        match current_block_260 {
            14371489722196013220 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_260 = 8514979938056295568;
            }
            _ => {}
        }
        match current_block_260 {
            8514979938056295568 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_260 = 15955263502462794145;
            }
            _ => {}
        }
        match current_block_260 {
            15955263502462794145 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_260 = 9617609482120873840;
            }
            _ => {}
        }
        match current_block_260 {
            9617609482120873840 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_260 = 1725138493261267529;
            }
            _ => {}
        }
        match current_block_260 {
            1725138493261267529 => {
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
        msg = 0 as *mut msg_t;
        if !msgs.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*msgs).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*msgs).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    msg = ((*((*(*msgs).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*msgs).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut msg_t;
                } else {
                    msg = 0 as *mut msg_t;
                }
                while !msg.is_null() {
                    if (*msg).hh.hashv == _hf_hashv && (*msg).hh.keylen == keylen {
                        if memcmp(
                            (*msg).hh.key,
                            &mut (*lookup_key).encoding as *mut libc::c_char
                                as *const libc::c_void,
                            keylen as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*msg).hh.hh_next).is_null() {
                        msg = ((*msg).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*msgs).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut msg_t;
                    } else {
                        msg = 0 as *mut msg_t;
                    }
                }
            }
        }
    }
    if !msg.is_null() {
        printf(b"found \n\0" as *const u8 as *const libc::c_char);
    }
    free(lookup_key as *mut libc::c_void);
    msg = msgs;
    tmp = (if !msgs.is_null() { (*msgs).hh.next } else { 0 as *mut libc::c_void })
        as *mut msg_t;
    while !msg.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*msg).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*msgs).hh.tbl).buckets as *mut libc::c_void);
            free((*msgs).hh.tbl as *mut libc::c_void);
            msgs = 0 as *mut msg_t;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*msgs).hh.tbl).tail {
                (*(*msgs).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*msgs).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*msgs).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                msgs = (*_hd_hh_del).next as *mut msg_t;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*msgs).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*msgs).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*msgs).hh.tbl).buckets)
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
            (*(*msgs).hh.tbl).num_items = ((*(*msgs).hh.tbl).num_items).wrapping_sub(1);
            (*(*msgs).hh.tbl).num_items;
        }
        free(msg as *mut libc::c_void);
        msg = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { 0 as *mut libc::c_void })
            as *mut msg_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

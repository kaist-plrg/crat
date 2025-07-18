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
pub struct inner {
    pub a: libc::c_int,
    pub b: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_event {
    pub is: inner,
    pub event_code: libc::c_char,
    pub user_id: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut e: *mut my_event = 0 as *mut my_event;
    let mut ev: my_event = my_event {
        is: inner { a: 0, b: 0 },
        event_code: 0,
        user_id: 0,
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
    let mut events: *mut my_event = 0 as *mut my_event;
    let mut keylen: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    keylen = (8 as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        .wrapping_sub(0 as libc::c_ulong) as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        e = malloc(::std::mem::size_of::<my_event>() as libc::c_ulong) as *mut my_event;
        if e.is_null() {
            exit(-(1 as libc::c_int));
        }
        memset(
            e as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<my_event>() as libc::c_ulong,
        );
        (*e)
            .is
            .a = i
            * (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int
                * 365 as libc::c_int);
        (*e).is.b = 0 as libc::c_int;
        (*e).event_code = ('a' as i32 + i % 2 as libc::c_int) as libc::c_char;
        (*e).user_id = i;
        let mut _ha_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut (*e).is as *mut inner
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
        let mut current_block_62: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_62 = 4169525235919384658;
            }
            10 => {
                current_block_62 = 4169525235919384658;
            }
            9 => {
                current_block_62 = 15579503665015831555;
            }
            8 => {
                current_block_62 = 15123861389650177679;
            }
            7 => {
                current_block_62 = 16496499871998433688;
            }
            6 => {
                current_block_62 = 17181025359314527317;
            }
            5 => {
                current_block_62 = 12972549306614970870;
            }
            4 => {
                current_block_62 = 11837076978836381723;
            }
            3 => {
                current_block_62 = 13205855207806107180;
            }
            2 => {
                current_block_62 = 2396836420025011240;
            }
            1 => {
                current_block_62 = 7255765040852174736;
            }
            _ => {
                current_block_62 = 7990025728955927862;
            }
        }
        match current_block_62 {
            4169525235919384658 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_62 = 15579503665015831555;
            }
            _ => {}
        }
        match current_block_62 {
            15579503665015831555 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_62 = 15123861389650177679;
            }
            _ => {}
        }
        match current_block_62 {
            15123861389650177679 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_62 = 16496499871998433688;
            }
            _ => {}
        }
        match current_block_62 {
            16496499871998433688 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_62 = 17181025359314527317;
            }
            _ => {}
        }
        match current_block_62 {
            17181025359314527317 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_62 = 12972549306614970870;
            }
            _ => {}
        }
        match current_block_62 {
            12972549306614970870 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_62 = 11837076978836381723;
            }
            _ => {}
        }
        match current_block_62 {
            11837076978836381723 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_62 = 13205855207806107180;
            }
            _ => {}
        }
        match current_block_62 {
            13205855207806107180 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_62 = 2396836420025011240;
            }
            _ => {}
        }
        match current_block_62 {
            2396836420025011240 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_62 = 7255765040852174736;
            }
            _ => {}
        }
        match current_block_62 {
            7255765040852174736 => {
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
        (*e).hh.hashv = _ha_hashv;
        (*e).hh.key = &mut (*e).is as *mut inner as *const libc::c_void;
        (*e).hh.keylen = keylen;
        if events.is_null() {
            (*e).hh.next = 0 as *mut libc::c_void;
            (*e).hh.prev = 0 as *mut libc::c_void;
            (*e)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*e).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*e).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*e).hh.tbl).tail = &mut (*e).hh;
                (*(*e).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*e).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*e).hh.tbl)
                    .hho = (&mut (*e).hh as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(e as *mut libc::c_char) as libc::c_long;
                (*(*e).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*e).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*e).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*e).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            events = e;
        } else {
            (*e).hh.tbl = (*events).hh.tbl;
            (*e).hh.next = 0 as *mut libc::c_void;
            (*e)
                .hh
                .prev = ((*(*events).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*events).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*events).hh.tbl).tail).next = e as *mut libc::c_void;
            (*(*events).hh.tbl).tail = &mut (*e).hh;
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*events).hh.tbl).num_items = ((*(*events).hh.tbl).num_items).wrapping_add(1);
        (*(*events).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*events).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*events).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*e).hh.hh_next = (*_ha_head).hh_head;
        (*e).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*e).hh;
        }
        (*_ha_head).hh_head = &mut (*e).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*e).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*e).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*e).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*e).hh.tbl)
                    .ideal_chain_maxlen = ((*(*e).hh.tbl).num_items
                    >> ((*(*e).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*e).hh.tbl).num_items
                            & ((*(*e).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*e).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*e).hh.tbl).num_buckets {
                    _he_thh = (*((*(*e).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*e).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*e).hh.tbl).ideal_chain_maxlen {
                            (*(*e).hh.tbl)
                                .nonideal_items = ((*(*e).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*e).hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul((*(*e).hh.tbl).ideal_chain_maxlen)
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
                free((*(*e).hh.tbl).buckets as *mut libc::c_void);
                (*(*e).hh.tbl)
                    .num_buckets = ((*(*e).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*e).hh.tbl)
                    .log2_num_buckets = ((*(*e).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*e).hh.tbl).log2_num_buckets;
                (*(*e).hh.tbl).buckets = _he_new_buckets;
                (*(*e).hh.tbl)
                    .ineff_expands = if (*(*e).hh.tbl).nonideal_items
                    > (*(*e).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*e).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*e).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*e).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        i += 1;
        i;
    }
    memset(
        &mut ev as *mut my_event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<my_event>() as libc::c_ulong,
    );
    ev
        .is
        .a = 5 as libc::c_int
        * (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int
            * 365 as libc::c_int);
    ev.is.b = 0 as libc::c_int;
    ev.event_code = 'b' as i32 as libc::c_char;
    e = 0 as *mut my_event;
    if !events.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_0: libc::c_uint = 0;
        let mut _hj_j_0: libc::c_uint = 0;
        let mut _hj_k_0: libc::c_uint = 0;
        let mut _hj_key_0: *const libc::c_uchar = &mut ev.is as *mut inner
            as *const libc::c_uchar;
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
                current_block_260 = 6757666663575783116;
            }
            10 => {
                current_block_260 = 6757666663575783116;
            }
            9 => {
                current_block_260 = 14423724542182203168;
            }
            8 => {
                current_block_260 = 9779264176850003141;
            }
            7 => {
                current_block_260 = 6442902164396703799;
            }
            6 => {
                current_block_260 = 13206828058566973673;
            }
            5 => {
                current_block_260 = 8981535396670572599;
            }
            4 => {
                current_block_260 = 12220727204062025905;
            }
            3 => {
                current_block_260 = 68925855748695021;
            }
            2 => {
                current_block_260 = 9193047302863557867;
            }
            1 => {
                current_block_260 = 11388719723789057958;
            }
            _ => {
                current_block_260 = 8147588656546899898;
            }
        }
        match current_block_260 {
            6757666663575783116 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_260 = 14423724542182203168;
            }
            _ => {}
        }
        match current_block_260 {
            14423724542182203168 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_260 = 9779264176850003141;
            }
            _ => {}
        }
        match current_block_260 {
            9779264176850003141 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_260 = 6442902164396703799;
            }
            _ => {}
        }
        match current_block_260 {
            6442902164396703799 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_260 = 13206828058566973673;
            }
            _ => {}
        }
        match current_block_260 {
            13206828058566973673 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_260 = 8981535396670572599;
            }
            _ => {}
        }
        match current_block_260 {
            8981535396670572599 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_260 = 12220727204062025905;
            }
            _ => {}
        }
        match current_block_260 {
            12220727204062025905 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_260 = 68925855748695021;
            }
            _ => {}
        }
        match current_block_260 {
            68925855748695021 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_260 = 9193047302863557867;
            }
            _ => {}
        }
        match current_block_260 {
            9193047302863557867 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_260 = 11388719723789057958;
            }
            _ => {}
        }
        match current_block_260 {
            11388719723789057958 => {
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
        e = 0 as *mut my_event;
        if !events.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*events).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*events).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    e = ((*((*(*events).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*events).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut my_event;
                } else {
                    e = 0 as *mut my_event;
                }
                while !e.is_null() {
                    if (*e).hh.hashv == _hf_hashv && (*e).hh.keylen == keylen {
                        if memcmp(
                            (*e).hh.key,
                            &mut ev.is as *mut inner as *const libc::c_void,
                            keylen as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*e).hh.hh_next).is_null() {
                        e = ((*e).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*events).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut my_event;
                    } else {
                        e = 0 as *mut my_event;
                    }
                }
            }
        }
    }
    if !e.is_null() {
        printf(
            b"found: user %d, unix time %d\n\0" as *const u8 as *const libc::c_char,
            (*e).user_id,
            (*e).is.a,
        );
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

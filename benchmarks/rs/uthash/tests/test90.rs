use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub struct item {
    pub sort_field: *mut libc::c_uchar,
    pub sort_field_len: size_t,
    pub some_user_data: libc::c_int,
    pub hh: UT_hash_handle,
}
pub unsafe extern "C" fn sort_func(
    mut a: *const item,
    mut b: *const item,
) -> libc::c_int {
    let mut va: libc::c_int = *((*a).sort_field as *mut libc::c_void
        as *mut libc::c_int);
    let mut vb: libc::c_int = *((*b).sort_field as *mut libc::c_void
        as *mut libc::c_int);
    return if va < vb { -(1 as libc::c_int) } else { (va > vb) as libc::c_int };
}
unsafe fn main_0() -> libc::c_int {
    let mut i: size_t = 0;
    let mut p: *mut item = 0 as *mut item;
    let mut tmp: *mut item = 0 as *mut item;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut list: *mut item = 0 as *mut item;
    let mut counter: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 100 as libc::c_int as libc::c_ulong {
        p = malloc(::std::mem::size_of::<item>() as libc::c_ulong) as *mut item;
        (*p).sort_field_len = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        (*p).sort_field = malloc((*p).sort_field_len) as *mut libc::c_uchar;
        let fresh0 = counter;
        counter = counter + 1;
        *((*p).sort_field as *mut libc::c_void as *mut libc::c_int) = fresh0;
        let mut _hs_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = (*p).sort_field as *const libc::c_uchar;
        _hs_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = (*p).sort_field_len as libc::c_uint;
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
            _hs_hashv = _hs_hashv
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
            _hj_i = _hj_i.wrapping_sub(_hs_hashv);
            _hj_i ^= _hs_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hs_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hs_hashv = _hs_hashv.wrapping_sub(_hj_i);
            _hs_hashv = _hs_hashv.wrapping_sub(_hj_j);
            _hs_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hs_hashv);
            _hj_i ^= _hs_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hs_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hs_hashv = _hs_hashv.wrapping_sub(_hj_i);
            _hs_hashv = _hs_hashv.wrapping_sub(_hj_j);
            _hs_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hs_hashv);
            _hj_i ^= _hs_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hs_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hs_hashv = _hs_hashv.wrapping_sub(_hj_i);
            _hs_hashv = _hs_hashv.wrapping_sub(_hj_j);
            _hs_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
        }
        _hs_hashv = _hs_hashv.wrapping_add((*p).sort_field_len as libc::c_uint);
        let mut current_block_56: u64;
        match _hj_k {
            11 => {
                _hs_hashv = _hs_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_56 = 13746107101806401364;
            }
            10 => {
                current_block_56 = 13746107101806401364;
            }
            9 => {
                current_block_56 = 8175499866402809331;
            }
            8 => {
                current_block_56 = 14256654709387854067;
            }
            7 => {
                current_block_56 = 8674483627294258383;
            }
            6 => {
                current_block_56 = 16481076385570880913;
            }
            5 => {
                current_block_56 = 5042068165056489667;
            }
            4 => {
                current_block_56 = 12208436626450457004;
            }
            3 => {
                current_block_56 = 17216480190185804420;
            }
            2 => {
                current_block_56 = 7466072693530372590;
            }
            1 => {
                current_block_56 = 10615577408538939548;
            }
            _ => {
                current_block_56 = 2989495919056355252;
            }
        }
        match current_block_56 {
            13746107101806401364 => {
                _hs_hashv = _hs_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_56 = 8175499866402809331;
            }
            _ => {}
        }
        match current_block_56 {
            8175499866402809331 => {
                _hs_hashv = _hs_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_56 = 14256654709387854067;
            }
            _ => {}
        }
        match current_block_56 {
            14256654709387854067 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_56 = 8674483627294258383;
            }
            _ => {}
        }
        match current_block_56 {
            8674483627294258383 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_56 = 16481076385570880913;
            }
            _ => {}
        }
        match current_block_56 {
            16481076385570880913 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_56 = 5042068165056489667;
            }
            _ => {}
        }
        match current_block_56 {
            5042068165056489667 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_56 = 12208436626450457004;
            }
            _ => {}
        }
        match current_block_56 {
            12208436626450457004 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_56 = 17216480190185804420;
            }
            _ => {}
        }
        match current_block_56 {
            17216480190185804420 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_56 = 7466072693530372590;
            }
            _ => {}
        }
        match current_block_56 {
            7466072693530372590 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_56 = 10615577408538939548;
            }
            _ => {}
        }
        match current_block_56 {
            10615577408538939548 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hs_hashv);
        _hj_i ^= _hs_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hs_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hs_hashv = _hs_hashv.wrapping_sub(_hj_i);
        _hs_hashv = _hs_hashv.wrapping_sub(_hj_j);
        _hs_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hs_hashv);
        _hj_i ^= _hs_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hs_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hs_hashv = _hs_hashv.wrapping_sub(_hj_i);
        _hs_hashv = _hs_hashv.wrapping_sub(_hj_j);
        _hs_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hs_hashv);
        _hj_i ^= _hs_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hs_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hs_hashv = _hs_hashv.wrapping_sub(_hj_i);
        _hs_hashv = _hs_hashv.wrapping_sub(_hj_j);
        _hs_hashv ^= _hj_j >> 15 as libc::c_int;
        (*p).hh.hashv = _hs_hashv;
        (*p).hh.key = (*p).sort_field as *mut libc::c_char as *const libc::c_void;
        (*p).hh.keylen = (*p).sort_field_len as libc::c_uint;
        if list.is_null() {
            (*p).hh.next = 0 as *mut libc::c_void;
            (*p).hh.prev = 0 as *mut libc::c_void;
            (*p)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*p).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*p).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*p).hh.tbl).tail = &mut (*p).hh;
                (*(*p).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*p).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*p).hh.tbl)
                    .hho = (&mut (*p).hh as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(p as *mut libc::c_char) as libc::c_long;
                (*(*p).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*p).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*p).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*p).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            list = p;
        } else {
            let mut _hs_iter: *mut libc::c_void = list as *mut libc::c_void;
            (*p).hh.tbl = (*list).hh.tbl;
            while !(sort_func(_hs_iter as *mut item as *const item, p)
                > 0 as libc::c_int)
            {
                _hs_iter = (*((_hs_iter as *mut libc::c_char)
                    .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                if _hs_iter.is_null() {
                    break;
                }
            }
            if !_hs_iter.is_null() {
                (*p).hh.next = _hs_iter;
                (*p)
                    .hh
                    .prev = (*((_hs_iter as *mut libc::c_char)
                    .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                if !((*p).hh.prev).is_null() {
                    let ref mut fresh1 = (*(((*p).hh.prev as *mut libc::c_char)
                        .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh1 = p as *mut libc::c_void;
                } else {
                    list = p;
                }
                let ref mut fresh2 = (*((_hs_iter as *mut libc::c_char)
                    .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh2 = p as *mut libc::c_void;
            } else {
                (*p).hh.next = 0 as *mut libc::c_void;
                (*p)
                    .hh
                    .prev = ((*(*list).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*list).hh.tbl).hho as isize)) as *mut libc::c_void;
                (*(*(*list).hh.tbl).tail).next = p as *mut libc::c_void;
                (*(*list).hh.tbl).tail = &mut (*p).hh;
            }
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*list).hh.tbl).num_items = ((*(*list).hh.tbl).num_items).wrapping_add(1);
        (*(*list).hh.tbl).num_items;
        _ha_bkt = _hs_hashv
            & ((*(*list).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*list).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*p).hh.hh_next = (*_ha_head).hh_head;
        (*p).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*p).hh;
        }
        (*_ha_head).hh_head = &mut (*p).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*p).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*p).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*p).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*p).hh.tbl)
                    .ideal_chain_maxlen = ((*(*p).hh.tbl).num_items
                    >> ((*(*p).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*p).hh.tbl).num_items
                            & ((*(*p).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*p).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*p).hh.tbl).num_buckets {
                    _he_thh = (*((*(*p).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*p).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*p).hh.tbl).ideal_chain_maxlen {
                            (*(*p).hh.tbl)
                                .nonideal_items = ((*(*p).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*p).hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul((*(*p).hh.tbl).ideal_chain_maxlen)
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
                free((*(*p).hh.tbl).buckets as *mut libc::c_void);
                (*(*p).hh.tbl)
                    .num_buckets = ((*(*p).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*p).hh.tbl)
                    .log2_num_buckets = ((*(*p).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*p).hh.tbl).log2_num_buckets;
                (*(*p).hh.tbl).buckets = _he_new_buckets;
                (*(*p).hh.tbl)
                    .ineff_expands = if (*(*p).hh.tbl).nonideal_items
                    > (*(*p).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*p).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*p).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*p).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    printf(b"filling in is ok\n\0" as *const u8 as *const libc::c_char);
    p = list;
    tmp = (if !list.is_null() { (*list).hh.next } else { 0 as *mut libc::c_void })
        as *mut item;
    while !p.is_null() {
        total += *((*p).sort_field as *mut libc::c_void as *mut libc::c_int);
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*p).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*list).hh.tbl).buckets as *mut libc::c_void);
            free((*list).hh.tbl as *mut libc::c_void);
            list = 0 as *mut item;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*list).hh.tbl).tail {
                (*(*list).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh3 = (*_hd_hh_del).next;
            } else {
                list = (*_hd_hh_del).next as *mut item;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*list).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh4 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*list).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*list).hh.tbl).buckets)
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
            (*(*list).hh.tbl).num_items = ((*(*list).hh.tbl).num_items).wrapping_sub(1);
            (*(*list).hh.tbl).num_items;
        }
        free((*p).sort_field as *mut libc::c_void);
        free(p as *mut libc::c_void);
        p = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { 0 as *mut libc::c_void })
            as *mut item;
    }
    if total == 4950 as libc::c_int {} else {
        __assert_fail(
            b"total == 4950\0" as *const u8 as *const libc::c_char,
            b"test90.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1882: {
        if total == 4950 as libc::c_int {} else {
            __assert_fail(
                b"total == 4950\0" as *const u8 as *const libc::c_char,
                b"test90.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    printf(b"cleanup is ok\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

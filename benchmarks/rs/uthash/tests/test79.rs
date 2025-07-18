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
pub struct hs_t {
    pub id: libc::c_int,
    pub tag: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe extern "C" fn pr(mut hdpp: *mut *mut hs_t) {
    let mut el: *mut hs_t = 0 as *mut hs_t;
    let mut tmp: *mut hs_t = 0 as *mut hs_t;
    let mut hdp: *mut hs_t = *hdpp;
    el = hdp;
    tmp = (if !hdp.is_null() { (*hdp).hh.next } else { 0 as *mut libc::c_void })
        as *mut hs_t;
    while !el.is_null() {
        printf(
            b"id %d, tag %d\n\0" as *const u8 as *const libc::c_char,
            (*el).id,
            (*el).tag,
        );
        el = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { 0 as *mut libc::c_void })
            as *mut hs_t;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut hs_head: *mut hs_t = 0 as *mut hs_t;
    let mut tmp: *mut hs_t = 0 as *mut hs_t;
    let mut replaced: *mut hs_t = 0 as *mut hs_t;
    tmp = malloc(::std::mem::size_of::<hs_t>() as libc::c_ulong) as *mut hs_t;
    if tmp.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*tmp).id = 10 as libc::c_int;
    (*tmp).tag = 100 as libc::c_int;
    let mut _hr_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut (*tmp).id as *mut libc::c_int
        as *const libc::c_uchar;
    _hr_hashv = 0xfeedbeef as libc::c_uint;
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
        _hr_hashv = _hr_hashv
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
        _hj_i = _hj_i.wrapping_sub(_hr_hashv);
        _hj_i ^= _hr_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hr_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_i);
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_j);
        _hr_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hr_hashv);
        _hj_i ^= _hr_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hr_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_i);
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_j);
        _hr_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hr_hashv);
        _hj_i ^= _hr_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hr_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_i);
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_j);
        _hr_hashv ^= _hj_j >> 15 as libc::c_int;
        _hj_key = _hj_key.offset(12 as libc::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
    }
    _hr_hashv = _hr_hashv
        .wrapping_add(
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_57: u64;
    match _hj_k {
        11 => {
            _hr_hashv = _hr_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 16152950153032035779;
        }
        10 => {
            current_block_57 = 16152950153032035779;
        }
        9 => {
            current_block_57 = 11248830434552844007;
        }
        8 => {
            current_block_57 = 10259572716349292157;
        }
        7 => {
            current_block_57 = 14262213187466411498;
        }
        6 => {
            current_block_57 = 13580471147157397510;
        }
        5 => {
            current_block_57 = 13829076254344524468;
        }
        4 => {
            current_block_57 = 16165420641460984288;
        }
        3 => {
            current_block_57 = 8433652531900264989;
        }
        2 => {
            current_block_57 = 14394250340306253603;
        }
        1 => {
            current_block_57 = 11520128967991814264;
        }
        _ => {
            current_block_57 = 1434579379687443766;
        }
    }
    match current_block_57 {
        16152950153032035779 => {
            _hr_hashv = _hr_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 11248830434552844007;
        }
        _ => {}
    }
    match current_block_57 {
        11248830434552844007 => {
            _hr_hashv = _hr_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 10259572716349292157;
        }
        _ => {}
    }
    match current_block_57 {
        10259572716349292157 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 14262213187466411498;
        }
        _ => {}
    }
    match current_block_57 {
        14262213187466411498 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 13580471147157397510;
        }
        _ => {}
    }
    match current_block_57 {
        13580471147157397510 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 13829076254344524468;
        }
        _ => {}
    }
    match current_block_57 {
        13829076254344524468 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_57 = 16165420641460984288;
        }
        _ => {}
    }
    match current_block_57 {
        16165420641460984288 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 8433652531900264989;
        }
        _ => {}
    }
    match current_block_57 {
        8433652531900264989 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 14394250340306253603;
        }
        _ => {}
    }
    match current_block_57 {
        14394250340306253603 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 11520128967991814264;
        }
        _ => {}
    }
    match current_block_57 {
        11520128967991814264 => {
            _hj_i = _hj_i
                .wrapping_add(
                    *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hr_hashv);
    _hj_i ^= _hr_hashv >> 13 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hr_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as libc::c_int;
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_i);
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_j);
    _hr_hashv ^= _hj_j >> 13 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hr_hashv);
    _hj_i ^= _hr_hashv >> 12 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hr_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as libc::c_int;
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_i);
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_j);
    _hr_hashv ^= _hj_j >> 5 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hr_hashv);
    _hj_i ^= _hr_hashv >> 3 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hr_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as libc::c_int;
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_i);
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_j);
    _hr_hashv ^= _hj_j >> 15 as libc::c_int;
    replaced = 0 as *mut hs_t;
    replaced = 0 as *mut hs_t;
    if !hs_head.is_null() {
        let mut _hf_bkt: libc::c_uint = 0;
        _hf_bkt = _hr_hashv
            & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*hs_head).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                .is_null()
            {
                replaced = ((*((*(*hs_head).hh.tbl).buckets).offset(_hf_bkt as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*hs_head).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut hs_t;
            } else {
                replaced = 0 as *mut hs_t;
            }
            while !replaced.is_null() {
                if (*replaced).hh.hashv == _hr_hashv
                    && (*replaced).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*replaced).hh.key,
                        &mut (*tmp).id as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*replaced).hh.hh_next).is_null() {
                    replaced = ((*replaced).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*hs_head).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut hs_t;
                } else {
                    replaced = 0 as *mut hs_t;
                }
            }
        }
    }
    if !replaced.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*replaced).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*hs_head).hh.tbl).buckets as *mut libc::c_void);
            free((*hs_head).hh.tbl as *mut libc::c_void);
            hs_head = 0 as *mut hs_t;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*hs_head).hh.tbl).tail {
                (*(*hs_head).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                hs_head = (*_hd_hh_del).next as *mut hs_t;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*hs_head).hh.tbl).buckets)
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
            (*(*hs_head).hh.tbl)
                .num_items = ((*(*hs_head).hh.tbl).num_items).wrapping_sub(1);
            (*(*hs_head).hh.tbl).num_items;
        }
    }
    (*tmp).hh.hashv = _hr_hashv;
    (*tmp).hh.key = &mut (*tmp).id as *mut libc::c_int as *const libc::c_void;
    (*tmp)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if hs_head.is_null() {
        (*tmp).hh.next = 0 as *mut libc::c_void;
        (*tmp).hh.prev = 0 as *mut libc::c_void;
        (*tmp)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*tmp).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*tmp).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*tmp).hh.tbl).tail = &mut (*tmp).hh;
            (*(*tmp).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*tmp).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*tmp).hh.tbl)
                .hho = (&mut (*tmp).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(tmp as *mut libc::c_char) as libc::c_long;
            (*(*tmp).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*tmp).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*tmp).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*tmp).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hs_head = tmp;
    } else {
        (*tmp).hh.tbl = (*hs_head).hh.tbl;
        (*tmp).hh.next = 0 as *mut libc::c_void;
        (*tmp)
            .hh
            .prev = ((*(*hs_head).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*hs_head).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*hs_head).hh.tbl).tail).next = tmp as *mut libc::c_void;
        (*(*hs_head).hh.tbl).tail = &mut (*tmp).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*hs_head).hh.tbl).num_items = ((*(*hs_head).hh.tbl).num_items).wrapping_add(1);
    (*(*hs_head).hh.tbl).num_items;
    _ha_bkt = _hr_hashv
        & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*hs_head).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*tmp).hh.hh_next = (*_ha_head).hh_head;
    (*tmp).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*tmp).hh;
    }
    (*_ha_head).hh_head = &mut (*tmp).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*tmp).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*tmp).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*tmp).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*tmp).hh.tbl)
                .ideal_chain_maxlen = ((*(*tmp).hh.tbl).num_items
                >> ((*(*tmp).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*tmp).hh.tbl).num_items
                        & ((*(*tmp).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*tmp).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*tmp).hh.tbl).num_buckets {
                _he_thh = (*((*(*tmp).hh.tbl).buckets).offset(_he_bkt_i as isize))
                    .hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*tmp).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*tmp).hh.tbl).ideal_chain_maxlen {
                        (*(*tmp).hh.tbl)
                            .nonideal_items = ((*(*tmp).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*tmp).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*tmp).hh.tbl).ideal_chain_maxlen)
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
            free((*(*tmp).hh.tbl).buckets as *mut libc::c_void);
            (*(*tmp).hh.tbl)
                .num_buckets = ((*(*tmp).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*tmp).hh.tbl)
                .log2_num_buckets = ((*(*tmp).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*tmp).hh.tbl).log2_num_buckets;
            (*(*tmp).hh.tbl).buckets = _he_new_buckets;
            (*(*tmp).hh.tbl)
                .ineff_expands = if (*(*tmp).hh.tbl).nonideal_items
                > (*(*tmp).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*tmp).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*tmp).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*tmp).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if replaced.is_null() {
        printf(
            b"added %d %d\n\0" as *const u8 as *const libc::c_char,
            (*tmp).id,
            (*tmp).tag,
        );
    } else {
        printf(
            b"ERROR, ended up replacing a value, replaced: %p\n\0" as *const u8
                as *const libc::c_char,
            replaced as *mut libc::c_void,
        );
    }
    pr(&mut hs_head);
    tmp = malloc(::std::mem::size_of::<hs_t>() as libc::c_ulong) as *mut hs_t;
    if tmp.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*tmp).id = 11 as libc::c_int;
    (*tmp).tag = 101 as libc::c_int;
    let mut _hr_hashv_0: libc::c_uint = 0;
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *const libc::c_uchar = &mut (*tmp).id as *mut libc::c_int
        as *const libc::c_uchar;
    _hr_hashv_0 = 0xfeedbeef as libc::c_uint;
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
        _hr_hashv_0 = _hr_hashv_0
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
        _hj_i_0 = _hj_i_0.wrapping_sub(_hr_hashv_0);
        _hj_i_0 ^= _hr_hashv_0 >> 13 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hr_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_0);
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_0);
        _hr_hashv_0 ^= _hj_j_0 >> 13 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hr_hashv_0);
        _hj_i_0 ^= _hr_hashv_0 >> 12 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hr_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_0);
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_0);
        _hr_hashv_0 ^= _hj_j_0 >> 5 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hr_hashv_0);
        _hj_i_0 ^= _hr_hashv_0 >> 3 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hr_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_0);
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_0);
        _hr_hashv_0 ^= _hj_j_0 >> 15 as libc::c_int;
        _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
        _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
    }
    _hr_hashv_0 = _hr_hashv_0
        .wrapping_add(
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_331: u64;
    match _hj_k_0 {
        11 => {
            _hr_hashv_0 = _hr_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_331 = 11687009940029432263;
        }
        10 => {
            current_block_331 = 11687009940029432263;
        }
        9 => {
            current_block_331 = 10636319182549236349;
        }
        8 => {
            current_block_331 = 7443509283371717434;
        }
        7 => {
            current_block_331 = 11222106743190825696;
        }
        6 => {
            current_block_331 = 17936095999271763517;
        }
        5 => {
            current_block_331 = 10507642832066914428;
        }
        4 => {
            current_block_331 = 17897994839440848612;
        }
        3 => {
            current_block_331 = 10284990178511843868;
        }
        2 => {
            current_block_331 = 5392639490290372941;
        }
        1 => {
            current_block_331 = 15227643218958109545;
        }
        _ => {
            current_block_331 = 9567781456482637466;
        }
    }
    match current_block_331 {
        11687009940029432263 => {
            _hr_hashv_0 = _hr_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_331 = 10636319182549236349;
        }
        _ => {}
    }
    match current_block_331 {
        10636319182549236349 => {
            _hr_hashv_0 = _hr_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_331 = 7443509283371717434;
        }
        _ => {}
    }
    match current_block_331 {
        7443509283371717434 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_331 = 11222106743190825696;
        }
        _ => {}
    }
    match current_block_331 {
        11222106743190825696 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_331 = 17936095999271763517;
        }
        _ => {}
    }
    match current_block_331 {
        17936095999271763517 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_331 = 10507642832066914428;
        }
        _ => {}
    }
    match current_block_331 {
        10507642832066914428 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_331 = 17897994839440848612;
        }
        _ => {}
    }
    match current_block_331 {
        17897994839440848612 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_331 = 10284990178511843868;
        }
        _ => {}
    }
    match current_block_331 {
        10284990178511843868 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_331 = 5392639490290372941;
        }
        _ => {}
    }
    match current_block_331 {
        5392639490290372941 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_331 = 15227643218958109545;
        }
        _ => {}
    }
    match current_block_331 {
        15227643218958109545 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(_hr_hashv_0);
    _hj_i_0 ^= _hr_hashv_0 >> 13 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(_hr_hashv_0);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_0);
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_0);
    _hr_hashv_0 ^= _hj_j_0 >> 13 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(_hr_hashv_0);
    _hj_i_0 ^= _hr_hashv_0 >> 12 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(_hr_hashv_0);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_0);
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_0);
    _hr_hashv_0 ^= _hj_j_0 >> 5 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(_hr_hashv_0);
    _hj_i_0 ^= _hr_hashv_0 >> 3 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(_hr_hashv_0);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_0);
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_0);
    _hr_hashv_0 ^= _hj_j_0 >> 15 as libc::c_int;
    replaced = 0 as *mut hs_t;
    replaced = 0 as *mut hs_t;
    if !hs_head.is_null() {
        let mut _hf_bkt_0: libc::c_uint = 0;
        _hf_bkt_0 = _hr_hashv_0
            & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*hs_head).hh.tbl).buckets).offset(_hf_bkt_0 as isize)).hh_head)
                .is_null()
            {
                replaced = ((*((*(*hs_head).hh.tbl).buckets).offset(_hf_bkt_0 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*hs_head).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut hs_t;
            } else {
                replaced = 0 as *mut hs_t;
            }
            while !replaced.is_null() {
                if (*replaced).hh.hashv == _hr_hashv_0
                    && (*replaced).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*replaced).hh.key,
                        &mut (*tmp).id as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*replaced).hh.hh_next).is_null() {
                    replaced = ((*replaced).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*hs_head).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut hs_t;
                } else {
                    replaced = 0 as *mut hs_t;
                }
            }
        }
    }
    if !replaced.is_null() {
        let mut _hd_hh_del_0: *mut UT_hash_handle = &mut (*replaced).hh;
        if ((*_hd_hh_del_0).prev).is_null() && ((*_hd_hh_del_0).next).is_null() {
            free((*(*hs_head).hh.tbl).buckets as *mut libc::c_void);
            free((*hs_head).hh.tbl as *mut libc::c_void);
            hs_head = 0 as *mut hs_t;
        } else {
            let mut _hd_bkt_0: libc::c_uint = 0;
            if _hd_hh_del_0 == (*(*hs_head).hh.tbl).tail {
                (*(*hs_head).hh.tbl)
                    .tail = ((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del_0).prev).is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del_0).next;
            } else {
                hs_head = (*_hd_hh_del_0).next as *mut hs_t;
            }
            if !((*_hd_hh_del_0).next).is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del_0).next as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head_0: *mut UT_hash_bucket = &mut *((*(*hs_head).hh.tbl)
                .buckets)
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
            (*(*hs_head).hh.tbl)
                .num_items = ((*(*hs_head).hh.tbl).num_items).wrapping_sub(1);
            (*(*hs_head).hh.tbl).num_items;
        }
    }
    (*tmp).hh.hashv = _hr_hashv_0;
    (*tmp).hh.key = &mut (*tmp).id as *mut libc::c_int as *const libc::c_void;
    (*tmp)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if hs_head.is_null() {
        (*tmp).hh.next = 0 as *mut libc::c_void;
        (*tmp).hh.prev = 0 as *mut libc::c_void;
        (*tmp)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*tmp).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*tmp).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*tmp).hh.tbl).tail = &mut (*tmp).hh;
            (*(*tmp).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*tmp).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*tmp).hh.tbl)
                .hho = (&mut (*tmp).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(tmp as *mut libc::c_char) as libc::c_long;
            (*(*tmp).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*tmp).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*tmp).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*tmp).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hs_head = tmp;
    } else {
        (*tmp).hh.tbl = (*hs_head).hh.tbl;
        (*tmp).hh.next = 0 as *mut libc::c_void;
        (*tmp)
            .hh
            .prev = ((*(*hs_head).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*hs_head).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*hs_head).hh.tbl).tail).next = tmp as *mut libc::c_void;
        (*(*hs_head).hh.tbl).tail = &mut (*tmp).hh;
    }
    let mut _ha_bkt_0: libc::c_uint = 0;
    (*(*hs_head).hh.tbl).num_items = ((*(*hs_head).hh.tbl).num_items).wrapping_add(1);
    (*(*hs_head).hh.tbl).num_items;
    _ha_bkt_0 = _hr_hashv_0
        & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*hs_head).hh.tbl).buckets)
        .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
    (*_ha_head_0).count;
    (*tmp).hh.hh_next = (*_ha_head_0).hh_head;
    (*tmp).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_0).hh_head).is_null() {
        (*(*_ha_head_0).hh_head).hh_prev = &mut (*tmp).hh;
    }
    (*_ha_head_0).hh_head = &mut (*tmp).hh;
    if (*_ha_head_0).count
        >= ((*_ha_head_0).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*tmp).hh.tbl).noexpand == 0
    {
        let mut _he_bkt_0: libc::c_uint = 0;
        let mut _he_bkt_i_0: libc::c_uint = 0;
        let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_0 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*tmp).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_0.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_0 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*tmp).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*tmp).hh.tbl)
                .ideal_chain_maxlen = ((*(*tmp).hh.tbl).num_items
                >> ((*(*tmp).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*tmp).hh.tbl).num_items
                        & ((*(*tmp).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*tmp).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_0 < (*(*tmp).hh.tbl).num_buckets {
                _he_thh_0 = (*((*(*tmp).hh.tbl).buckets).offset(_he_bkt_i_0 as isize))
                    .hh_head;
                while !_he_thh_0.is_null() {
                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                    _he_bkt_0 = (*_he_thh_0).hashv
                        & ((*(*tmp).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                    if (*_he_newbkt_0).count > (*(*tmp).hh.tbl).ideal_chain_maxlen {
                        (*(*tmp).hh.tbl)
                            .nonideal_items = ((*(*tmp).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*tmp).hh.tbl).nonideal_items;
                        if (*_he_newbkt_0).count
                            > ((*_he_newbkt_0).expand_mult)
                                .wrapping_mul((*(*tmp).hh.tbl).ideal_chain_maxlen)
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
            free((*(*tmp).hh.tbl).buckets as *mut libc::c_void);
            (*(*tmp).hh.tbl)
                .num_buckets = ((*(*tmp).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*tmp).hh.tbl)
                .log2_num_buckets = ((*(*tmp).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*tmp).hh.tbl).log2_num_buckets;
            (*(*tmp).hh.tbl).buckets = _he_new_buckets_0;
            (*(*tmp).hh.tbl)
                .ineff_expands = if (*(*tmp).hh.tbl).nonideal_items
                > (*(*tmp).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*tmp).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*tmp).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*tmp).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if replaced.is_null() {
        printf(
            b"added %d %d\n\0" as *const u8 as *const libc::c_char,
            (*tmp).id,
            (*tmp).tag,
        );
    } else {
        printf(
            b"ERROR, ended up replacing a value, replaced: %p\n\0" as *const u8
                as *const libc::c_char,
            replaced as *mut libc::c_void,
        );
    }
    pr(&mut hs_head);
    tmp = malloc(::std::mem::size_of::<hs_t>() as libc::c_ulong) as *mut hs_t;
    if tmp.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*tmp).id = 11 as libc::c_int;
    (*tmp).tag = 102 as libc::c_int;
    let mut _hr_hashv_1: libc::c_uint = 0;
    let mut _hj_i_1: libc::c_uint = 0;
    let mut _hj_j_1: libc::c_uint = 0;
    let mut _hj_k_1: libc::c_uint = 0;
    let mut _hj_key_1: *const libc::c_uchar = &mut (*tmp).id as *mut libc::c_int
        as *const libc::c_uchar;
    _hr_hashv_1 = 0xfeedbeef as libc::c_uint;
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
        _hr_hashv_1 = _hr_hashv_1
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
        _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv_1);
        _hj_i_1 ^= _hr_hashv_1 >> 13 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
        _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_i_1);
        _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_j_1);
        _hr_hashv_1 ^= _hj_j_1 >> 13 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv_1);
        _hj_i_1 ^= _hr_hashv_1 >> 12 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
        _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_i_1);
        _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_j_1);
        _hr_hashv_1 ^= _hj_j_1 >> 5 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv_1);
        _hj_i_1 ^= _hr_hashv_1 >> 3 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
        _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_i_1);
        _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_j_1);
        _hr_hashv_1 ^= _hj_j_1 >> 15 as libc::c_int;
        _hj_key_1 = _hj_key_1.offset(12 as libc::c_int as isize);
        _hj_k_1 = _hj_k_1.wrapping_sub(12 as libc::c_uint);
    }
    _hr_hashv_1 = _hr_hashv_1
        .wrapping_add(
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_605: u64;
    match _hj_k_1 {
        11 => {
            _hr_hashv_1 = _hr_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_605 = 17306716369277798168;
        }
        10 => {
            current_block_605 = 17306716369277798168;
        }
        9 => {
            current_block_605 = 5420455305703726507;
        }
        8 => {
            current_block_605 = 7571425676338483961;
        }
        7 => {
            current_block_605 = 10911681226309595049;
        }
        6 => {
            current_block_605 = 13597032505220455247;
        }
        5 => {
            current_block_605 = 11322302728796175764;
        }
        4 => {
            current_block_605 = 7808664388140442342;
        }
        3 => {
            current_block_605 = 6698305622715213732;
        }
        2 => {
            current_block_605 = 8942117886667888374;
        }
        1 => {
            current_block_605 = 8745216392357474161;
        }
        _ => {
            current_block_605 = 6721946588916655032;
        }
    }
    match current_block_605 {
        17306716369277798168 => {
            _hr_hashv_1 = _hr_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_605 = 5420455305703726507;
        }
        _ => {}
    }
    match current_block_605 {
        5420455305703726507 => {
            _hr_hashv_1 = _hr_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_605 = 7571425676338483961;
        }
        _ => {}
    }
    match current_block_605 {
        7571425676338483961 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_605 = 10911681226309595049;
        }
        _ => {}
    }
    match current_block_605 {
        10911681226309595049 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_605 = 13597032505220455247;
        }
        _ => {}
    }
    match current_block_605 {
        13597032505220455247 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_605 = 11322302728796175764;
        }
        _ => {}
    }
    match current_block_605 {
        11322302728796175764 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    *_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_605 = 7808664388140442342;
        }
        _ => {}
    }
    match current_block_605 {
        7808664388140442342 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_605 = 6698305622715213732;
        }
        _ => {}
    }
    match current_block_605 {
        6698305622715213732 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_605 = 8942117886667888374;
        }
        _ => {}
    }
    match current_block_605 {
        8942117886667888374 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_605 = 8745216392357474161;
        }
        _ => {}
    }
    match current_block_605 {
        8745216392357474161 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    *_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv_1);
    _hj_i_1 ^= _hr_hashv_1 >> 13 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv_1);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
    _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_i_1);
    _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_j_1);
    _hr_hashv_1 ^= _hj_j_1 >> 13 as libc::c_int;
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv_1);
    _hj_i_1 ^= _hr_hashv_1 >> 12 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv_1);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
    _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_i_1);
    _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_j_1);
    _hr_hashv_1 ^= _hj_j_1 >> 5 as libc::c_int;
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv_1);
    _hj_i_1 ^= _hr_hashv_1 >> 3 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv_1);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
    _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_i_1);
    _hr_hashv_1 = _hr_hashv_1.wrapping_sub(_hj_j_1);
    _hr_hashv_1 ^= _hj_j_1 >> 15 as libc::c_int;
    replaced = 0 as *mut hs_t;
    replaced = 0 as *mut hs_t;
    if !hs_head.is_null() {
        let mut _hf_bkt_1: libc::c_uint = 0;
        _hf_bkt_1 = _hr_hashv_1
            & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*hs_head).hh.tbl).buckets).offset(_hf_bkt_1 as isize)).hh_head)
                .is_null()
            {
                replaced = ((*((*(*hs_head).hh.tbl).buckets).offset(_hf_bkt_1 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*hs_head).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut hs_t;
            } else {
                replaced = 0 as *mut hs_t;
            }
            while !replaced.is_null() {
                if (*replaced).hh.hashv == _hr_hashv_1
                    && (*replaced).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*replaced).hh.key,
                        &mut (*tmp).id as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*replaced).hh.hh_next).is_null() {
                    replaced = ((*replaced).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*hs_head).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut hs_t;
                } else {
                    replaced = 0 as *mut hs_t;
                }
            }
        }
    }
    if !replaced.is_null() {
        let mut _hd_hh_del_1: *mut UT_hash_handle = &mut (*replaced).hh;
        if ((*_hd_hh_del_1).prev).is_null() && ((*_hd_hh_del_1).next).is_null() {
            free((*(*hs_head).hh.tbl).buckets as *mut libc::c_void);
            free((*hs_head).hh.tbl as *mut libc::c_void);
            hs_head = 0 as *mut hs_t;
        } else {
            let mut _hd_bkt_1: libc::c_uint = 0;
            if _hd_hh_del_1 == (*(*hs_head).hh.tbl).tail {
                (*(*hs_head).hh.tbl)
                    .tail = ((*_hd_hh_del_1).prev as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del_1).prev).is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del_1).prev as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh4 = (*_hd_hh_del_1).next;
            } else {
                hs_head = (*_hd_hh_del_1).next as *mut hs_t;
            }
            if !((*_hd_hh_del_1).next).is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del_1).next as *mut libc::c_char)
                    .offset((*(*hs_head).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh5 = (*_hd_hh_del_1).prev;
            }
            _hd_bkt_1 = (*_hd_hh_del_1).hashv
                & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head_1: *mut UT_hash_bucket = &mut *((*(*hs_head).hh.tbl)
                .buckets)
                .offset(_hd_bkt_1 as isize) as *mut UT_hash_bucket;
            (*_hd_head_1).count = ((*_hd_head_1).count).wrapping_sub(1);
            (*_hd_head_1).count;
            if (*_hd_head_1).hh_head == _hd_hh_del_1 {
                (*_hd_head_1).hh_head = (*_hd_hh_del_1).hh_next;
            }
            if !((*_hd_hh_del_1).hh_prev).is_null() {
                (*(*_hd_hh_del_1).hh_prev).hh_next = (*_hd_hh_del_1).hh_next;
            }
            if !((*_hd_hh_del_1).hh_next).is_null() {
                (*(*_hd_hh_del_1).hh_next).hh_prev = (*_hd_hh_del_1).hh_prev;
            }
            (*(*hs_head).hh.tbl)
                .num_items = ((*(*hs_head).hh.tbl).num_items).wrapping_sub(1);
            (*(*hs_head).hh.tbl).num_items;
        }
    }
    (*tmp).hh.hashv = _hr_hashv_1;
    (*tmp).hh.key = &mut (*tmp).id as *mut libc::c_int as *const libc::c_void;
    (*tmp)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if hs_head.is_null() {
        (*tmp).hh.next = 0 as *mut libc::c_void;
        (*tmp).hh.prev = 0 as *mut libc::c_void;
        (*tmp)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*tmp).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*tmp).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*tmp).hh.tbl).tail = &mut (*tmp).hh;
            (*(*tmp).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*tmp).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*tmp).hh.tbl)
                .hho = (&mut (*tmp).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(tmp as *mut libc::c_char) as libc::c_long;
            (*(*tmp).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*tmp).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*tmp).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*tmp).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hs_head = tmp;
    } else {
        (*tmp).hh.tbl = (*hs_head).hh.tbl;
        (*tmp).hh.next = 0 as *mut libc::c_void;
        (*tmp)
            .hh
            .prev = ((*(*hs_head).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*hs_head).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*hs_head).hh.tbl).tail).next = tmp as *mut libc::c_void;
        (*(*hs_head).hh.tbl).tail = &mut (*tmp).hh;
    }
    let mut _ha_bkt_1: libc::c_uint = 0;
    (*(*hs_head).hh.tbl).num_items = ((*(*hs_head).hh.tbl).num_items).wrapping_add(1);
    (*(*hs_head).hh.tbl).num_items;
    _ha_bkt_1 = _hr_hashv_1
        & ((*(*hs_head).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_1: *mut UT_hash_bucket = &mut *((*(*hs_head).hh.tbl).buckets)
        .offset(_ha_bkt_1 as isize) as *mut UT_hash_bucket;
    (*_ha_head_1).count = ((*_ha_head_1).count).wrapping_add(1);
    (*_ha_head_1).count;
    (*tmp).hh.hh_next = (*_ha_head_1).hh_head;
    (*tmp).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_1).hh_head).is_null() {
        (*(*_ha_head_1).hh_head).hh_prev = &mut (*tmp).hh;
    }
    (*_ha_head_1).hh_head = &mut (*tmp).hh;
    if (*_ha_head_1).count
        >= ((*_ha_head_1).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*tmp).hh.tbl).noexpand == 0
    {
        let mut _he_bkt_1: libc::c_uint = 0;
        let mut _he_bkt_i_1: libc::c_uint = 0;
        let mut _he_thh_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_1 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*tmp).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_1.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_1 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*tmp).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*tmp).hh.tbl)
                .ideal_chain_maxlen = ((*(*tmp).hh.tbl).num_items
                >> ((*(*tmp).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*tmp).hh.tbl).num_items
                        & ((*(*tmp).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*tmp).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_1 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_1 < (*(*tmp).hh.tbl).num_buckets {
                _he_thh_1 = (*((*(*tmp).hh.tbl).buckets).offset(_he_bkt_i_1 as isize))
                    .hh_head;
                while !_he_thh_1.is_null() {
                    _he_hh_nxt_1 = (*_he_thh_1).hh_next;
                    _he_bkt_1 = (*_he_thh_1).hashv
                        & ((*(*tmp).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_1 = &mut *_he_new_buckets_1.offset(_he_bkt_1 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_1).count = ((*_he_newbkt_1).count).wrapping_add(1);
                    if (*_he_newbkt_1).count > (*(*tmp).hh.tbl).ideal_chain_maxlen {
                        (*(*tmp).hh.tbl)
                            .nonideal_items = ((*(*tmp).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*tmp).hh.tbl).nonideal_items;
                        if (*_he_newbkt_1).count
                            > ((*_he_newbkt_1).expand_mult)
                                .wrapping_mul((*(*tmp).hh.tbl).ideal_chain_maxlen)
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
            free((*(*tmp).hh.tbl).buckets as *mut libc::c_void);
            (*(*tmp).hh.tbl)
                .num_buckets = ((*(*tmp).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*tmp).hh.tbl)
                .log2_num_buckets = ((*(*tmp).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*tmp).hh.tbl).log2_num_buckets;
            (*(*tmp).hh.tbl).buckets = _he_new_buckets_1;
            (*(*tmp).hh.tbl)
                .ineff_expands = if (*(*tmp).hh.tbl).nonideal_items
                > (*(*tmp).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*tmp).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*tmp).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*tmp).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if replaced.is_null() {
        printf(
            b"ERROR, exected to replace a value with key: %d\n\0" as *const u8
                as *const libc::c_char,
            (*tmp).id,
        );
    } else {
        printf(
            b"replaced %d that had tag %d with tag %d\n\0" as *const u8
                as *const libc::c_char,
            (*tmp).id,
            (*replaced).tag,
            (*tmp).tag,
        );
    }
    pr(&mut hs_head);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

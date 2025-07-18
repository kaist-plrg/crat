use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub struct hstruct_t {
    pub name: [libc::c_char; 32],
    pub weight: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe extern "C" fn cmpfunc(
    mut s1: *const hstruct_t,
    mut s2: *const hstruct_t,
) -> libc::c_int {
    return if (*s1).weight < (*s2).weight {
        -(1 as libc::c_int)
    } else {
        ((*s1).weight > (*s2).weight) as libc::c_int
    };
}
pub unsafe extern "C" fn printtable(mut hTable: *const hstruct_t) {
    let mut search: *const hstruct_t = 0 as *const hstruct_t;
    let mut tmp: *const hstruct_t = 0 as *const hstruct_t;
    search = hTable;
    tmp = (if !hTable.is_null() { (*hTable).hh.next } else { 0 as *mut libc::c_void })
        as *const hstruct_t;
    while !search.is_null() {
        printf(
            b"%d: %s\n\0" as *const u8 as *const libc::c_char,
            (*search).weight,
            ((*search).name).as_ptr(),
        );
        search = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { 0 as *mut libc::c_void })
            as *const hstruct_t;
    }
    printf(b"###\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn delitem(
    mut hTable: *mut *mut hstruct_t,
    mut name: *const libc::c_char,
) {
    let mut item: *mut hstruct_t = 0 as *mut hstruct_t;
    let mut _uthash_hfstr_keylen: libc::c_uint = strlen(name) as libc::c_uint;
    item = 0 as *mut hstruct_t;
    if !(*hTable).is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = name as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = _uthash_hfstr_keylen;
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
        _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
        let mut current_block_52: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 2994683124351785234;
            }
            10 => {
                current_block_52 = 2994683124351785234;
            }
            9 => {
                current_block_52 = 11227813453055322446;
            }
            8 => {
                current_block_52 = 2862295478527784457;
            }
            7 => {
                current_block_52 = 18147978640797309873;
            }
            6 => {
                current_block_52 = 10389134853097574123;
            }
            5 => {
                current_block_52 = 15370744082028861854;
            }
            4 => {
                current_block_52 = 3645930888634205132;
            }
            3 => {
                current_block_52 = 17009877711962317848;
            }
            2 => {
                current_block_52 = 16841485667497224234;
            }
            1 => {
                current_block_52 = 9201261121548257604;
            }
            _ => {
                current_block_52 = 9441801433784995173;
            }
        }
        match current_block_52 {
            2994683124351785234 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 11227813453055322446;
            }
            _ => {}
        }
        match current_block_52 {
            11227813453055322446 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 2862295478527784457;
            }
            _ => {}
        }
        match current_block_52 {
            2862295478527784457 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 18147978640797309873;
            }
            _ => {}
        }
        match current_block_52 {
            18147978640797309873 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 10389134853097574123;
            }
            _ => {}
        }
        match current_block_52 {
            10389134853097574123 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 15370744082028861854;
            }
            _ => {}
        }
        match current_block_52 {
            15370744082028861854 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_52 = 3645930888634205132;
            }
            _ => {}
        }
        match current_block_52 {
            3645930888634205132 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 17009877711962317848;
            }
            _ => {}
        }
        match current_block_52 {
            17009877711962317848 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 16841485667497224234;
            }
            _ => {}
        }
        match current_block_52 {
            16841485667497224234 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 9201261121548257604;
            }
            _ => {}
        }
        match current_block_52 {
            9201261121548257604 => {
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
        item = 0 as *mut hstruct_t;
        if !(*hTable).is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(**hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(**hTable).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    item = ((*((*(**hTable).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(**hTable).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut hstruct_t;
                } else {
                    item = 0 as *mut hstruct_t;
                }
                while !item.is_null() {
                    if (*item).hh.hashv == _hf_hashv
                        && (*item).hh.keylen == _uthash_hfstr_keylen
                    {
                        if memcmp(
                            (*item).hh.key,
                            name as *const libc::c_void,
                            _uthash_hfstr_keylen as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*item).hh.hh_next).is_null() {
                        item = ((*item).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(**hTable).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut hstruct_t;
                    } else {
                        item = 0 as *mut hstruct_t;
                    }
                }
            }
        }
    }
    let mut _hd_hh_del: *mut UT_hash_handle = &mut (*item).hh;
    if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
        free((*(**hTable).hh.tbl).buckets as *mut libc::c_void);
        free((**hTable).hh.tbl as *mut libc::c_void);
        *hTable = 0 as *mut hstruct_t;
    } else {
        let mut _hd_bkt: libc::c_uint = 0;
        if _hd_hh_del == (*(**hTable).hh.tbl).tail {
            (*(**hTable).hh.tbl)
                .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                .offset((*(**hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle;
        }
        if !((*_hd_hh_del).prev).is_null() {
            let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                .offset((*(**hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            *fresh0 = (*_hd_hh_del).next;
        } else {
            *hTable = (*_hd_hh_del).next as *mut hstruct_t;
        }
        if !((*_hd_hh_del).next).is_null() {
            let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                .offset((*(**hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh1 = (*_hd_hh_del).prev;
        }
        _hd_bkt = (*_hd_hh_del).hashv
            & ((*(**hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _hd_head: *mut UT_hash_bucket = &mut *((*(**hTable).hh.tbl).buckets)
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
        (*(**hTable).hh.tbl)
            .num_items = ((*(**hTable).hh.tbl).num_items).wrapping_sub(1);
        (*(**hTable).hh.tbl).num_items;
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut hTable: *mut hstruct_t = 0 as *mut hstruct_t;
    let mut replaced: *mut hstruct_t = 0 as *mut hstruct_t;
    let mut hashvalue: libc::c_uint = 0;
    let mut tst: [hstruct_t; 13] = [
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 2 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 8 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 1 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 8 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 3 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 5 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 6 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 15 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 6 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 9 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 10 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 43 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = hstruct_t {
                name: *::std::mem::transmute::<
                    &[u8; 32],
                    &mut [libc::c_char; 32],
                >(b"muh12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                weight: 7 as libc::c_int,
                hh: {
                    let mut init = UT_hash_handle {
                        tbl: 0 as *mut UT_hash_table,
                        prev: 0 as *mut libc::c_void,
                        next: 0 as *mut libc::c_void,
                        hh_prev: 0 as *mut UT_hash_handle,
                        hh_next: 0 as *mut UT_hash_handle,
                        key: 0 as *const libc::c_void,
                        keylen: 0,
                        hashv: 0,
                    };
                    init
                },
            };
            init
        },
    ];
    let mut index: libc::c_int = 0;
    index = 0 as libc::c_int;
    while index < 11 as libc::c_int {
        let mut _hs_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut *((*tst
            .as_mut_ptr()
            .offset(index as isize))
            .name)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_uchar;
        _hs_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = strlen((tst[index as usize].name).as_mut_ptr()) as libc::c_uint;
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
        _hs_hashv = _hs_hashv
            .wrapping_add(
                strlen((tst[index as usize].name).as_mut_ptr()) as libc::c_uint,
            );
        let mut current_block_52: u64;
        match _hj_k {
            11 => {
                _hs_hashv = _hs_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 1194356806173913549;
            }
            10 => {
                current_block_52 = 1194356806173913549;
            }
            9 => {
                current_block_52 = 12660334309370614189;
            }
            8 => {
                current_block_52 = 8854263334847882453;
            }
            7 => {
                current_block_52 = 4017677640696164912;
            }
            6 => {
                current_block_52 = 7572128004406765047;
            }
            5 => {
                current_block_52 = 3800893665139888790;
            }
            4 => {
                current_block_52 = 6972138495692529429;
            }
            3 => {
                current_block_52 = 8628691928863547280;
            }
            2 => {
                current_block_52 = 6405334113228567422;
            }
            1 => {
                current_block_52 = 7537626883245681487;
            }
            _ => {
                current_block_52 = 12381812505308290051;
            }
        }
        match current_block_52 {
            1194356806173913549 => {
                _hs_hashv = _hs_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 12660334309370614189;
            }
            _ => {}
        }
        match current_block_52 {
            12660334309370614189 => {
                _hs_hashv = _hs_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 8854263334847882453;
            }
            _ => {}
        }
        match current_block_52 {
            8854263334847882453 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 4017677640696164912;
            }
            _ => {}
        }
        match current_block_52 {
            4017677640696164912 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 7572128004406765047;
            }
            _ => {}
        }
        match current_block_52 {
            7572128004406765047 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 3800893665139888790;
            }
            _ => {}
        }
        match current_block_52 {
            3800893665139888790 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_52 = 6972138495692529429;
            }
            _ => {}
        }
        match current_block_52 {
            6972138495692529429 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 8628691928863547280;
            }
            _ => {}
        }
        match current_block_52 {
            8628691928863547280 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 6405334113228567422;
            }
            _ => {}
        }
        match current_block_52 {
            6405334113228567422 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 7537626883245681487;
            }
            _ => {}
        }
        match current_block_52 {
            7537626883245681487 => {
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
        tst[index as usize].hh.hashv = _hs_hashv;
        tst[index as usize]
            .hh
            .key = &mut *((*tst.as_mut_ptr().offset(index as isize)).name)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_void;
        tst[index as usize]
            .hh
            .keylen = strlen((tst[index as usize].name).as_mut_ptr()) as libc::c_uint;
        if hTable.is_null() {
            tst[index as usize].hh.next = 0 as *mut libc::c_void;
            tst[index as usize].hh.prev = 0 as *mut libc::c_void;
            tst[index as usize]
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if (tst[index as usize].hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    tst[index as usize].hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*tst[index as usize].hh.tbl)
                    .tail = &mut (*tst.as_mut_ptr().offset(index as isize)).hh;
                (*tst[index as usize].hh.tbl).num_buckets = 32 as libc::c_uint;
                (*tst[index as usize].hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*tst[index as usize].hh.tbl)
                    .hho = (&mut (*tst.as_mut_ptr().offset(index as isize)).hh
                    as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(
                        &mut *tst.as_mut_ptr().offset(index as isize) as *mut hstruct_t
                            as *mut libc::c_char,
                    ) as libc::c_long;
                (*tst[index as usize].hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*tst[index as usize].hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*tst[index as usize].hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*tst[index as usize].hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            hTable = &mut *tst.as_mut_ptr().offset(index as isize) as *mut hstruct_t;
        } else {
            let mut _hs_iter: *mut libc::c_void = hTable as *mut libc::c_void;
            tst[index as usize].hh.tbl = (*hTable).hh.tbl;
            while !(cmpfunc(
                _hs_iter as *mut hstruct_t as *const hstruct_t,
                &mut *tst.as_mut_ptr().offset(index as isize),
            ) > 0 as libc::c_int)
            {
                _hs_iter = (*((_hs_iter as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                if _hs_iter.is_null() {
                    break;
                }
            }
            if !_hs_iter.is_null() {
                tst[index as usize].hh.next = _hs_iter;
                tst[index as usize]
                    .hh
                    .prev = (*((_hs_iter as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                if !(tst[index as usize].hh.prev).is_null() {
                    let ref mut fresh2 = (*((tst[index as usize].hh.prev
                        as *mut libc::c_char)
                        .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh2 = &mut *tst.as_mut_ptr().offset(index as isize)
                        as *mut hstruct_t as *mut libc::c_void;
                } else {
                    hTable = &mut *tst.as_mut_ptr().offset(index as isize)
                        as *mut hstruct_t;
                }
                let ref mut fresh3 = (*((_hs_iter as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = &mut *tst.as_mut_ptr().offset(index as isize) as *mut hstruct_t
                    as *mut libc::c_void;
            } else {
                tst[index as usize].hh.next = 0 as *mut libc::c_void;
                tst[index as usize]
                    .hh
                    .prev = ((*(*hTable).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void;
                (*(*(*hTable).hh.tbl).tail)
                    .next = &mut *tst.as_mut_ptr().offset(index as isize)
                    as *mut hstruct_t as *mut libc::c_void;
                (*(*hTable).hh.tbl)
                    .tail = &mut (*tst.as_mut_ptr().offset(index as isize)).hh;
            }
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_add(1);
        (*(*hTable).hh.tbl).num_items;
        _ha_bkt = _hs_hashv
            & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        tst[index as usize].hh.hh_next = (*_ha_head).hh_head;
        tst[index as usize].hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head)
                .hh_prev = &mut (*tst.as_mut_ptr().offset(index as isize)).hh;
        }
        (*_ha_head).hh_head = &mut (*tst.as_mut_ptr().offset(index as isize)).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint)
            && (*tst[index as usize].hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul(
                        (*tst[index as usize].hh.tbl).num_buckets as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul(
                            (*tst[index as usize].hh.tbl).num_buckets as libc::c_ulong,
                        )
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*tst[index as usize].hh.tbl)
                    .ideal_chain_maxlen = ((*tst[index as usize].hh.tbl).num_items
                    >> ((*tst[index as usize].hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*tst[index as usize].hh.tbl).num_items
                            & ((*tst[index as usize].hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*tst[index as usize].hh.tbl)
                    .nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*tst[index as usize].hh.tbl).num_buckets {
                    _he_thh = (*((*tst[index as usize].hh.tbl).buckets)
                        .offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*tst[index as usize].hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count
                            > (*tst[index as usize].hh.tbl).ideal_chain_maxlen
                        {
                            (*tst[index as usize].hh.tbl)
                                .nonideal_items = ((*tst[index as usize].hh.tbl)
                                .nonideal_items)
                                .wrapping_add(1);
                            (*tst[index as usize].hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul(
                                        (*tst[index as usize].hh.tbl).ideal_chain_maxlen,
                                    )
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
                free((*tst[index as usize].hh.tbl).buckets as *mut libc::c_void);
                (*tst[index as usize].hh.tbl)
                    .num_buckets = ((*tst[index as usize].hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*tst[index as usize].hh.tbl)
                    .log2_num_buckets = ((*tst[index as usize].hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*tst[index as usize].hh.tbl).log2_num_buckets;
                (*tst[index as usize].hh.tbl).buckets = _he_new_buckets;
                (*tst[index as usize].hh.tbl)
                    .ineff_expands = if (*tst[index as usize].hh.tbl).nonideal_items
                    > (*tst[index as usize].hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*tst[index as usize].hh.tbl).ineff_expands)
                        .wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*tst[index as usize].hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*tst[index as usize].hh.tbl)
                        .noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        index += 1;
        index;
    }
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *const libc::c_uchar = (tst[11 as libc::c_int as usize].name)
        .as_mut_ptr() as *const libc::c_uchar;
    hashvalue = 0xfeedbeef as libc::c_uint;
    _hj_j_0 = 0x9e3779b9 as libc::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr())
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
        hashvalue = hashvalue
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
        _hj_i_0 = _hj_i_0.wrapping_sub(hashvalue);
        _hj_i_0 ^= hashvalue >> 13 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(hashvalue);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_0);
        hashvalue = hashvalue.wrapping_sub(_hj_j_0);
        hashvalue ^= _hj_j_0 >> 13 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(hashvalue);
        _hj_i_0 ^= hashvalue >> 12 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(hashvalue);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_0);
        hashvalue = hashvalue.wrapping_sub(_hj_j_0);
        hashvalue ^= _hj_j_0 >> 5 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(hashvalue);
        _hj_i_0 ^= hashvalue >> 3 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(hashvalue);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_0);
        hashvalue = hashvalue.wrapping_sub(_hj_j_0);
        hashvalue ^= _hj_j_0 >> 15 as libc::c_int;
        _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
        _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
    }
    hashvalue = hashvalue
        .wrapping_add(
            strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint,
        );
    let mut current_block_258: u64;
    match _hj_k_0 {
        11 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_258 = 2387731292535907928;
        }
        10 => {
            current_block_258 = 2387731292535907928;
        }
        9 => {
            current_block_258 = 843610826356838339;
        }
        8 => {
            current_block_258 = 12884625819277968603;
        }
        7 => {
            current_block_258 = 7741439392210097897;
        }
        6 => {
            current_block_258 = 6782506343435084693;
        }
        5 => {
            current_block_258 = 5748404149956308062;
        }
        4 => {
            current_block_258 = 15277436037701823412;
        }
        3 => {
            current_block_258 = 13999892719267820590;
        }
        2 => {
            current_block_258 = 17579905605962965662;
        }
        1 => {
            current_block_258 = 607878295432143320;
        }
        _ => {
            current_block_258 = 13526015532137226550;
        }
    }
    match current_block_258 {
        2387731292535907928 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_258 = 843610826356838339;
        }
        _ => {}
    }
    match current_block_258 {
        843610826356838339 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_258 = 12884625819277968603;
        }
        _ => {}
    }
    match current_block_258 {
        12884625819277968603 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_258 = 7741439392210097897;
        }
        _ => {}
    }
    match current_block_258 {
        7741439392210097897 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_258 = 6782506343435084693;
        }
        _ => {}
    }
    match current_block_258 {
        6782506343435084693 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_258 = 5748404149956308062;
        }
        _ => {}
    }
    match current_block_258 {
        5748404149956308062 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_258 = 15277436037701823412;
        }
        _ => {}
    }
    match current_block_258 {
        15277436037701823412 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_258 = 13999892719267820590;
        }
        _ => {}
    }
    match current_block_258 {
        13999892719267820590 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_258 = 17579905605962965662;
        }
        _ => {}
    }
    match current_block_258 {
        17579905605962965662 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_258 = 607878295432143320;
        }
        _ => {}
    }
    match current_block_258 {
        607878295432143320 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(hashvalue);
    _hj_i_0 ^= hashvalue >> 13 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(hashvalue);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_0);
    hashvalue = hashvalue.wrapping_sub(_hj_j_0);
    hashvalue ^= _hj_j_0 >> 13 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(hashvalue);
    _hj_i_0 ^= hashvalue >> 12 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(hashvalue);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_0);
    hashvalue = hashvalue.wrapping_sub(_hj_j_0);
    hashvalue ^= _hj_j_0 >> 5 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub(hashvalue);
    _hj_i_0 ^= hashvalue >> 3 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub(hashvalue);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_0);
    hashvalue = hashvalue.wrapping_sub(_hj_j_0);
    hashvalue ^= _hj_j_0 >> 15 as libc::c_int;
    tst[11 as libc::c_int as usize].hh.hashv = hashvalue;
    tst[11 as libc::c_int as usize]
        .hh
        .key = &mut *((*tst.as_mut_ptr().offset(11 as libc::c_int as isize)).name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void;
    tst[11 as libc::c_int as usize]
        .hh
        .keylen = strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr())
        as libc::c_uint;
    if hTable.is_null() {
        tst[11 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
        tst[11 as libc::c_int as usize].hh.prev = 0 as *mut libc::c_void;
        tst[11 as libc::c_int as usize]
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (tst[11 as libc::c_int as usize].hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                tst[11 as libc::c_int as usize].hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(11 as libc::c_int as isize)).hh;
            (*tst[11 as libc::c_int as usize].hh.tbl).num_buckets = 32 as libc::c_uint;
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = 5 as libc::c_uint;
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .hho = (&mut (*tst.as_mut_ptr().offset(11 as libc::c_int as isize)).hh
                as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(
                    &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
                        as *mut hstruct_t as *mut libc::c_char,
                ) as libc::c_long;
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .signature = 0xa0111fe1 as libc::c_uint;
            if ((*tst[11 as libc::c_int as usize].hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*tst[11 as libc::c_int as usize].hh.tbl).buckets
                        as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hTable = &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
            as *mut hstruct_t;
    } else {
        let mut _hs_iter_0: *mut libc::c_void = hTable as *mut libc::c_void;
        tst[11 as libc::c_int as usize].hh.tbl = (*hTable).hh.tbl;
        while !(cmpfunc(
            _hs_iter_0 as *mut hstruct_t as *const hstruct_t,
            &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize),
        ) > 0 as libc::c_int)
        {
            _hs_iter_0 = (*((_hs_iter_0 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            if _hs_iter_0.is_null() {
                break;
            }
        }
        if !_hs_iter_0.is_null() {
            tst[11 as libc::c_int as usize].hh.next = _hs_iter_0;
            tst[11 as libc::c_int as usize]
                .hh
                .prev = (*((_hs_iter_0 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            if !(tst[11 as libc::c_int as usize].hh.prev).is_null() {
                let ref mut fresh4 = (*((tst[11 as libc::c_int as usize].hh.prev
                    as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh4 = &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
                    as *mut hstruct_t as *mut libc::c_void;
            } else {
                hTable = &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
                    as *mut hstruct_t;
            }
            let ref mut fresh5 = (*((_hs_iter_0 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh5 = &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
        } else {
            tst[11 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
            tst[11 as libc::c_int as usize]
                .hh
                .prev = ((*(*hTable).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*hTable).hh.tbl).tail)
                .next = &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
            (*(*hTable).hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(11 as libc::c_int as isize)).hh;
        }
    }
    let mut _ha_bkt_0: libc::c_uint = 0;
    (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_add(1);
    (*(*hTable).hh.tbl).num_items;
    _ha_bkt_0 = hashvalue
        & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
        .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
    (*_ha_head_0).count;
    tst[11 as libc::c_int as usize].hh.hh_next = (*_ha_head_0).hh_head;
    tst[11 as libc::c_int as usize].hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_0).hh_head).is_null() {
        (*(*_ha_head_0).hh_head)
            .hh_prev = &mut (*tst.as_mut_ptr().offset(11 as libc::c_int as isize)).hh;
    }
    (*_ha_head_0)
        .hh_head = &mut (*tst.as_mut_ptr().offset(11 as libc::c_int as isize)).hh;
    if (*_ha_head_0).count
        >= ((*_ha_head_0).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint)
        && (*tst[11 as libc::c_int as usize].hh.tbl).noexpand == 0
    {
        let mut _he_bkt_0: libc::c_uint = 0;
        let mut _he_bkt_i_0: libc::c_uint = 0;
        let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_0 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul(
                    (*tst[11 as libc::c_int as usize].hh.tbl).num_buckets
                        as libc::c_ulong,
                )
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_0.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_0 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul(
                        (*tst[11 as libc::c_int as usize].hh.tbl).num_buckets
                            as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .ideal_chain_maxlen = ((*tst[11 as libc::c_int as usize].hh.tbl)
                .num_items
                >> ((*tst[11 as libc::c_int as usize].hh.tbl).log2_num_buckets)
                    .wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*tst[11 as libc::c_int as usize].hh.tbl).num_items
                        & ((*tst[11 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_0 < (*tst[11 as libc::c_int as usize].hh.tbl).num_buckets {
                _he_thh_0 = (*((*tst[11 as libc::c_int as usize].hh.tbl).buckets)
                    .offset(_he_bkt_i_0 as isize))
                    .hh_head;
                while !_he_thh_0.is_null() {
                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                    _he_bkt_0 = (*_he_thh_0).hashv
                        & ((*tst[11 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                    if (*_he_newbkt_0).count
                        > (*tst[11 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen
                    {
                        (*tst[11 as libc::c_int as usize].hh.tbl)
                            .nonideal_items = ((*tst[11 as libc::c_int as usize].hh.tbl)
                            .nonideal_items)
                            .wrapping_add(1);
                        (*tst[11 as libc::c_int as usize].hh.tbl).nonideal_items;
                        if (*_he_newbkt_0).count
                            > ((*_he_newbkt_0).expand_mult)
                                .wrapping_mul(
                                    (*tst[11 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen,
                                )
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
            free((*tst[11 as libc::c_int as usize].hh.tbl).buckets as *mut libc::c_void);
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .num_buckets = ((*tst[11 as libc::c_int as usize].hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = ((*tst[11 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets)
                .wrapping_add(1);
            (*tst[11 as libc::c_int as usize].hh.tbl).log2_num_buckets;
            (*tst[11 as libc::c_int as usize].hh.tbl).buckets = _he_new_buckets_0;
            (*tst[11 as libc::c_int as usize].hh.tbl)
                .ineff_expands = if (*tst[11 as libc::c_int as usize].hh.tbl)
                .nonideal_items
                > (*tst[11 as libc::c_int as usize].hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*tst[11 as libc::c_int as usize].hh.tbl).ineff_expands)
                    .wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*tst[11 as libc::c_int as usize].hh.tbl).ineff_expands
                > 1 as libc::c_uint
            {
                (*tst[11 as libc::c_int as usize].hh.tbl)
                    .noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    printtable(hTable);
    let mut _hr_hashv: libc::c_uint = 0;
    let mut _hj_i_1: libc::c_uint = 0;
    let mut _hj_j_1: libc::c_uint = 0;
    let mut _hj_k_1: libc::c_uint = 0;
    let mut _hj_key_1: *const libc::c_uchar = &mut *((*tst
        .as_mut_ptr()
        .offset(12 as libc::c_int as isize))
        .name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_uchar;
    _hr_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j_1 = 0x9e3779b9 as libc::c_uint;
    _hj_i_1 = _hj_j_1;
    _hj_k_1 = strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr())
        as libc::c_uint;
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
        _hr_hashv = _hr_hashv
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
        _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv);
        _hj_i_1 ^= _hr_hashv >> 13 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_i_1);
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_j_1);
        _hr_hashv ^= _hj_j_1 >> 13 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv);
        _hj_i_1 ^= _hr_hashv >> 12 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_i_1);
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_j_1);
        _hr_hashv ^= _hj_j_1 >> 5 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv);
        _hj_i_1 ^= _hr_hashv >> 3 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_i_1);
        _hr_hashv = _hr_hashv.wrapping_sub(_hj_j_1);
        _hr_hashv ^= _hj_j_1 >> 15 as libc::c_int;
        _hj_key_1 = _hj_key_1.offset(12 as libc::c_int as isize);
        _hj_k_1 = _hj_k_1.wrapping_sub(12 as libc::c_uint);
    }
    _hr_hashv = _hr_hashv
        .wrapping_add(
            strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint,
        );
    let mut current_block_461: u64;
    match _hj_k_1 {
        11 => {
            _hr_hashv = _hr_hashv
                .wrapping_add(
                    (*_hj_key_1.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_461 = 9697412716423290366;
        }
        10 => {
            current_block_461 = 9697412716423290366;
        }
        9 => {
            current_block_461 = 15513286801263913470;
        }
        8 => {
            current_block_461 = 14549140913924859045;
        }
        7 => {
            current_block_461 = 18240736655628938287;
        }
        6 => {
            current_block_461 = 3312927314369267527;
        }
        5 => {
            current_block_461 = 1904836663932494064;
        }
        4 => {
            current_block_461 = 15220931962766639339;
        }
        3 => {
            current_block_461 = 3954829711460148713;
        }
        2 => {
            current_block_461 = 15873613702180875992;
        }
        1 => {
            current_block_461 = 7988855212177271024;
        }
        _ => {
            current_block_461 = 16992166533277814053;
        }
    }
    match current_block_461 {
        9697412716423290366 => {
            _hr_hashv = _hr_hashv
                .wrapping_add(
                    (*_hj_key_1.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_461 = 15513286801263913470;
        }
        _ => {}
    }
    match current_block_461 {
        15513286801263913470 => {
            _hr_hashv = _hr_hashv
                .wrapping_add(
                    (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_461 = 14549140913924859045;
        }
        _ => {}
    }
    match current_block_461 {
        14549140913924859045 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_461 = 18240736655628938287;
        }
        _ => {}
    }
    match current_block_461 {
        18240736655628938287 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_461 = 3312927314369267527;
        }
        _ => {}
    }
    match current_block_461 {
        3312927314369267527 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_461 = 1904836663932494064;
        }
        _ => {}
    }
    match current_block_461 {
        1904836663932494064 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    *_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_461 = 15220931962766639339;
        }
        _ => {}
    }
    match current_block_461 {
        15220931962766639339 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_461 = 3954829711460148713;
        }
        _ => {}
    }
    match current_block_461 {
        3954829711460148713 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_461 = 15873613702180875992;
        }
        _ => {}
    }
    match current_block_461 {
        15873613702180875992 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_461 = 7988855212177271024;
        }
        _ => {}
    }
    match current_block_461 {
        7988855212177271024 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    *_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv);
    _hj_i_1 ^= _hr_hashv >> 13 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_i_1);
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_j_1);
    _hr_hashv ^= _hj_j_1 >> 13 as libc::c_int;
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv);
    _hj_i_1 ^= _hr_hashv >> 12 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_i_1);
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_j_1);
    _hr_hashv ^= _hj_j_1 >> 5 as libc::c_int;
    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
    _hj_i_1 = _hj_i_1.wrapping_sub(_hr_hashv);
    _hj_i_1 ^= _hr_hashv >> 3 as libc::c_int;
    _hj_j_1 = _hj_j_1.wrapping_sub(_hr_hashv);
    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
    _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_i_1);
    _hr_hashv = _hr_hashv.wrapping_sub(_hj_j_1);
    _hr_hashv ^= _hj_j_1 >> 15 as libc::c_int;
    replaced = 0 as *mut hstruct_t;
    replaced = 0 as *mut hstruct_t;
    if !hTable.is_null() {
        let mut _hf_bkt: libc::c_uint = 0;
        _hf_bkt = _hr_hashv
            & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*hTable).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                .is_null()
            {
                replaced = ((*((*(*hTable).hh.tbl).buckets).offset(_hf_bkt as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut hstruct_t;
            } else {
                replaced = 0 as *mut hstruct_t;
            }
            while !replaced.is_null() {
                if (*replaced).hh.hashv == _hr_hashv
                    && (*replaced).hh.keylen as libc::c_ulong
                        == strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr())
                {
                    if memcmp(
                        (*replaced).hh.key,
                        &mut *((*tst.as_mut_ptr().offset(12 as libc::c_int as isize))
                            .name)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut libc::c_char
                            as *const libc::c_void,
                        strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr()),
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*replaced).hh.hh_next).is_null() {
                    replaced = ((*replaced).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut hstruct_t;
                } else {
                    replaced = 0 as *mut hstruct_t;
                }
            }
        }
    }
    if !replaced.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*replaced).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*hTable).hh.tbl).buckets as *mut libc::c_void);
            free((*hTable).hh.tbl as *mut libc::c_void);
            hTable = 0 as *mut hstruct_t;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*hTable).hh.tbl).tail {
                (*(*hTable).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh6 = (*_hd_hh_del).next;
            } else {
                hTable = (*_hd_hh_del).next as *mut hstruct_t;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh7 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh7 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
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
            (*(*hTable).hh.tbl)
                .num_items = ((*(*hTable).hh.tbl).num_items).wrapping_sub(1);
            (*(*hTable).hh.tbl).num_items;
        }
    }
    tst[12 as libc::c_int as usize].hh.hashv = _hr_hashv;
    tst[12 as libc::c_int as usize]
        .hh
        .key = &mut *((*tst.as_mut_ptr().offset(12 as libc::c_int as isize)).name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void;
    tst[12 as libc::c_int as usize]
        .hh
        .keylen = strlen((tst[11 as libc::c_int as usize].name).as_mut_ptr())
        as libc::c_uint;
    if hTable.is_null() {
        tst[12 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
        tst[12 as libc::c_int as usize].hh.prev = 0 as *mut libc::c_void;
        tst[12 as libc::c_int as usize]
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (tst[12 as libc::c_int as usize].hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                tst[12 as libc::c_int as usize].hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(12 as libc::c_int as isize)).hh;
            (*tst[12 as libc::c_int as usize].hh.tbl).num_buckets = 32 as libc::c_uint;
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = 5 as libc::c_uint;
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .hho = (&mut (*tst.as_mut_ptr().offset(12 as libc::c_int as isize)).hh
                as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(
                    &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize)
                        as *mut hstruct_t as *mut libc::c_char,
                ) as libc::c_long;
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .signature = 0xa0111fe1 as libc::c_uint;
            if ((*tst[12 as libc::c_int as usize].hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*tst[12 as libc::c_int as usize].hh.tbl).buckets
                        as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hTable = &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize)
            as *mut hstruct_t;
    } else {
        let mut _hs_iter_1: *mut libc::c_void = hTable as *mut libc::c_void;
        tst[12 as libc::c_int as usize].hh.tbl = (*hTable).hh.tbl;
        while !(cmpfunc(
            _hs_iter_1 as *mut hstruct_t as *const hstruct_t,
            &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize),
        ) > 0 as libc::c_int)
        {
            _hs_iter_1 = (*((_hs_iter_1 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            if _hs_iter_1.is_null() {
                break;
            }
        }
        if !_hs_iter_1.is_null() {
            tst[12 as libc::c_int as usize].hh.next = _hs_iter_1;
            tst[12 as libc::c_int as usize]
                .hh
                .prev = (*((_hs_iter_1 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            if !(tst[12 as libc::c_int as usize].hh.prev).is_null() {
                let ref mut fresh8 = (*((tst[12 as libc::c_int as usize].hh.prev
                    as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh8 = &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize)
                    as *mut hstruct_t as *mut libc::c_void;
            } else {
                hTable = &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize)
                    as *mut hstruct_t;
            }
            let ref mut fresh9 = (*((_hs_iter_1 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh9 = &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
        } else {
            tst[12 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
            tst[12 as libc::c_int as usize]
                .hh
                .prev = ((*(*hTable).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*hTable).hh.tbl).tail)
                .next = &mut *tst.as_mut_ptr().offset(12 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
            (*(*hTable).hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(12 as libc::c_int as isize)).hh;
        }
    }
    let mut _ha_bkt_1: libc::c_uint = 0;
    (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_add(1);
    (*(*hTable).hh.tbl).num_items;
    _ha_bkt_1 = _hr_hashv
        & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_1: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
        .offset(_ha_bkt_1 as isize) as *mut UT_hash_bucket;
    (*_ha_head_1).count = ((*_ha_head_1).count).wrapping_add(1);
    (*_ha_head_1).count;
    tst[12 as libc::c_int as usize].hh.hh_next = (*_ha_head_1).hh_head;
    tst[12 as libc::c_int as usize].hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_1).hh_head).is_null() {
        (*(*_ha_head_1).hh_head)
            .hh_prev = &mut (*tst.as_mut_ptr().offset(12 as libc::c_int as isize)).hh;
    }
    (*_ha_head_1)
        .hh_head = &mut (*tst.as_mut_ptr().offset(12 as libc::c_int as isize)).hh;
    if (*_ha_head_1).count
        >= ((*_ha_head_1).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint)
        && (*tst[12 as libc::c_int as usize].hh.tbl).noexpand == 0
    {
        let mut _he_bkt_1: libc::c_uint = 0;
        let mut _he_bkt_i_1: libc::c_uint = 0;
        let mut _he_thh_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_1 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul(
                    (*tst[12 as libc::c_int as usize].hh.tbl).num_buckets
                        as libc::c_ulong,
                )
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_1.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_1 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul(
                        (*tst[12 as libc::c_int as usize].hh.tbl).num_buckets
                            as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .ideal_chain_maxlen = ((*tst[12 as libc::c_int as usize].hh.tbl)
                .num_items
                >> ((*tst[12 as libc::c_int as usize].hh.tbl).log2_num_buckets)
                    .wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*tst[12 as libc::c_int as usize].hh.tbl).num_items
                        & ((*tst[12 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_1 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_1 < (*tst[12 as libc::c_int as usize].hh.tbl).num_buckets {
                _he_thh_1 = (*((*tst[12 as libc::c_int as usize].hh.tbl).buckets)
                    .offset(_he_bkt_i_1 as isize))
                    .hh_head;
                while !_he_thh_1.is_null() {
                    _he_hh_nxt_1 = (*_he_thh_1).hh_next;
                    _he_bkt_1 = (*_he_thh_1).hashv
                        & ((*tst[12 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_1 = &mut *_he_new_buckets_1.offset(_he_bkt_1 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_1).count = ((*_he_newbkt_1).count).wrapping_add(1);
                    if (*_he_newbkt_1).count
                        > (*tst[12 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen
                    {
                        (*tst[12 as libc::c_int as usize].hh.tbl)
                            .nonideal_items = ((*tst[12 as libc::c_int as usize].hh.tbl)
                            .nonideal_items)
                            .wrapping_add(1);
                        (*tst[12 as libc::c_int as usize].hh.tbl).nonideal_items;
                        if (*_he_newbkt_1).count
                            > ((*_he_newbkt_1).expand_mult)
                                .wrapping_mul(
                                    (*tst[12 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen,
                                )
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
            free((*tst[12 as libc::c_int as usize].hh.tbl).buckets as *mut libc::c_void);
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .num_buckets = ((*tst[12 as libc::c_int as usize].hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = ((*tst[12 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets)
                .wrapping_add(1);
            (*tst[12 as libc::c_int as usize].hh.tbl).log2_num_buckets;
            (*tst[12 as libc::c_int as usize].hh.tbl).buckets = _he_new_buckets_1;
            (*tst[12 as libc::c_int as usize].hh.tbl)
                .ineff_expands = if (*tst[12 as libc::c_int as usize].hh.tbl)
                .nonideal_items
                > (*tst[12 as libc::c_int as usize].hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*tst[12 as libc::c_int as usize].hh.tbl).ineff_expands)
                    .wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*tst[12 as libc::c_int as usize].hh.tbl).ineff_expands
                > 1 as libc::c_uint
            {
                (*tst[12 as libc::c_int as usize].hh.tbl)
                    .noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if replaced
        == &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize) as *mut hstruct_t
    {} else {
        __assert_fail(
            b"replaced == &tst[11]\0" as *const u8 as *const libc::c_char,
            b"test87.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_11485: {
        if replaced
            == &mut *tst.as_mut_ptr().offset(11 as libc::c_int as isize)
                as *mut hstruct_t
        {} else {
            __assert_fail(
                b"replaced == &tst[11]\0" as *const u8 as *const libc::c_char,
                b"test87.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    printtable(hTable);
    tst[2 as libc::c_int as usize].weight = 9 as libc::c_int;
    let mut _hr_hashv_0: libc::c_uint = 0;
    let mut _hj_i_2: libc::c_uint = 0;
    let mut _hj_j_2: libc::c_uint = 0;
    let mut _hj_k_2: libc::c_uint = 0;
    let mut _hj_key_2: *const libc::c_uchar = &mut *((*tst
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize))
        .name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_uchar;
    _hr_hashv_0 = 0xfeedbeef as libc::c_uint;
    _hj_j_2 = 0x9e3779b9 as libc::c_uint;
    _hj_i_2 = _hj_j_2;
    _hj_k_2 = strlen((tst[2 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint;
    while _hj_k_2 >= 12 as libc::c_uint {
        _hj_i_2 = _hj_i_2
            .wrapping_add(
                (*_hj_key_2.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_2.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_2.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_2.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j_2 = _hj_j_2
            .wrapping_add(
                (*_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_2.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_2.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_2.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hr_hashv_0 = _hr_hashv_0
            .wrapping_add(
                (*_hj_key_2.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_2.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_2.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_2.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hr_hashv_0);
        _hj_i_2 ^= _hr_hashv_0 >> 13 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hr_hashv_0);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_2);
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_2);
        _hr_hashv_0 ^= _hj_j_2 >> 13 as libc::c_int;
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hr_hashv_0);
        _hj_i_2 ^= _hr_hashv_0 >> 12 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hr_hashv_0);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_2);
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_2);
        _hr_hashv_0 ^= _hj_j_2 >> 5 as libc::c_int;
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hr_hashv_0);
        _hj_i_2 ^= _hr_hashv_0 >> 3 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hr_hashv_0);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_2);
        _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_2);
        _hr_hashv_0 ^= _hj_j_2 >> 15 as libc::c_int;
        _hj_key_2 = _hj_key_2.offset(12 as libc::c_int as isize);
        _hj_k_2 = _hj_k_2.wrapping_sub(12 as libc::c_uint);
    }
    _hr_hashv_0 = _hr_hashv_0
        .wrapping_add(
            strlen((tst[2 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint,
        );
    let mut current_block_739: u64;
    match _hj_k_2 {
        11 => {
            _hr_hashv_0 = _hr_hashv_0
                .wrapping_add(
                    (*_hj_key_2.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_739 = 9950405900916038993;
        }
        10 => {
            current_block_739 = 9950405900916038993;
        }
        9 => {
            current_block_739 = 15634005063732231070;
        }
        8 => {
            current_block_739 = 17949475799717808259;
        }
        7 => {
            current_block_739 = 12164000533135916401;
        }
        6 => {
            current_block_739 = 17731510417861920602;
        }
        5 => {
            current_block_739 = 16089748462499744479;
        }
        4 => {
            current_block_739 = 4659183887397131710;
        }
        3 => {
            current_block_739 = 3126671451676797781;
        }
        2 => {
            current_block_739 = 8849365848522132641;
        }
        1 => {
            current_block_739 = 17352501618758964873;
        }
        _ => {
            current_block_739 = 17658124290648448443;
        }
    }
    match current_block_739 {
        9950405900916038993 => {
            _hr_hashv_0 = _hr_hashv_0
                .wrapping_add(
                    (*_hj_key_2.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_739 = 15634005063732231070;
        }
        _ => {}
    }
    match current_block_739 {
        15634005063732231070 => {
            _hr_hashv_0 = _hr_hashv_0
                .wrapping_add(
                    (*_hj_key_2.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_739 = 17949475799717808259;
        }
        _ => {}
    }
    match current_block_739 {
        17949475799717808259 => {
            _hj_j_2 = _hj_j_2
                .wrapping_add(
                    (*_hj_key_2.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_739 = 12164000533135916401;
        }
        _ => {}
    }
    match current_block_739 {
        12164000533135916401 => {
            _hj_j_2 = _hj_j_2
                .wrapping_add(
                    (*_hj_key_2.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_739 = 17731510417861920602;
        }
        _ => {}
    }
    match current_block_739 {
        17731510417861920602 => {
            _hj_j_2 = _hj_j_2
                .wrapping_add(
                    (*_hj_key_2.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_739 = 16089748462499744479;
        }
        _ => {}
    }
    match current_block_739 {
        16089748462499744479 => {
            _hj_j_2 = _hj_j_2
                .wrapping_add(
                    *_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_739 = 4659183887397131710;
        }
        _ => {}
    }
    match current_block_739 {
        4659183887397131710 => {
            _hj_i_2 = _hj_i_2
                .wrapping_add(
                    (*_hj_key_2.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_739 = 3126671451676797781;
        }
        _ => {}
    }
    match current_block_739 {
        3126671451676797781 => {
            _hj_i_2 = _hj_i_2
                .wrapping_add(
                    (*_hj_key_2.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_739 = 8849365848522132641;
        }
        _ => {}
    }
    match current_block_739 {
        8849365848522132641 => {
            _hj_i_2 = _hj_i_2
                .wrapping_add(
                    (*_hj_key_2.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_739 = 17352501618758964873;
        }
        _ => {}
    }
    match current_block_739 {
        17352501618758964873 => {
            _hj_i_2 = _hj_i_2
                .wrapping_add(
                    *_hj_key_2.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
    _hj_i_2 = _hj_i_2.wrapping_sub(_hr_hashv_0);
    _hj_i_2 ^= _hr_hashv_0 >> 13 as libc::c_int;
    _hj_j_2 = _hj_j_2.wrapping_sub(_hr_hashv_0);
    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
    _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_2);
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_2);
    _hr_hashv_0 ^= _hj_j_2 >> 13 as libc::c_int;
    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
    _hj_i_2 = _hj_i_2.wrapping_sub(_hr_hashv_0);
    _hj_i_2 ^= _hr_hashv_0 >> 12 as libc::c_int;
    _hj_j_2 = _hj_j_2.wrapping_sub(_hr_hashv_0);
    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
    _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_2);
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_2);
    _hr_hashv_0 ^= _hj_j_2 >> 5 as libc::c_int;
    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
    _hj_i_2 = _hj_i_2.wrapping_sub(_hr_hashv_0);
    _hj_i_2 ^= _hr_hashv_0 >> 3 as libc::c_int;
    _hj_j_2 = _hj_j_2.wrapping_sub(_hr_hashv_0);
    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
    _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_i_2);
    _hr_hashv_0 = _hr_hashv_0.wrapping_sub(_hj_j_2);
    _hr_hashv_0 ^= _hj_j_2 >> 15 as libc::c_int;
    replaced = 0 as *mut hstruct_t;
    replaced = 0 as *mut hstruct_t;
    if !hTable.is_null() {
        let mut _hf_bkt_0: libc::c_uint = 0;
        _hf_bkt_0 = _hr_hashv_0
            & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*hTable).hh.tbl).buckets).offset(_hf_bkt_0 as isize)).hh_head)
                .is_null()
            {
                replaced = ((*((*(*hTable).hh.tbl).buckets).offset(_hf_bkt_0 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut hstruct_t;
            } else {
                replaced = 0 as *mut hstruct_t;
            }
            while !replaced.is_null() {
                if (*replaced).hh.hashv == _hr_hashv_0
                    && (*replaced).hh.keylen as libc::c_ulong
                        == strlen((tst[2 as libc::c_int as usize].name).as_mut_ptr())
                {
                    if memcmp(
                        (*replaced).hh.key,
                        &mut *((*tst.as_mut_ptr().offset(2 as libc::c_int as isize))
                            .name)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut libc::c_char
                            as *const libc::c_void,
                        strlen((tst[2 as libc::c_int as usize].name).as_mut_ptr()),
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*replaced).hh.hh_next).is_null() {
                    replaced = ((*replaced).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut hstruct_t;
                } else {
                    replaced = 0 as *mut hstruct_t;
                }
            }
        }
    }
    if !replaced.is_null() {
        let mut _hd_hh_del_0: *mut UT_hash_handle = &mut (*replaced).hh;
        if ((*_hd_hh_del_0).prev).is_null() && ((*_hd_hh_del_0).next).is_null() {
            free((*(*hTable).hh.tbl).buckets as *mut libc::c_void);
            free((*hTable).hh.tbl as *mut libc::c_void);
            hTable = 0 as *mut hstruct_t;
        } else {
            let mut _hd_bkt_0: libc::c_uint = 0;
            if _hd_hh_del_0 == (*(*hTable).hh.tbl).tail {
                (*(*hTable).hh.tbl)
                    .tail = ((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del_0).prev).is_null() {
                let ref mut fresh10 = (*(((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh10 = (*_hd_hh_del_0).next;
            } else {
                hTable = (*_hd_hh_del_0).next as *mut hstruct_t;
            }
            if !((*_hd_hh_del_0).next).is_null() {
                let ref mut fresh11 = (*(((*_hd_hh_del_0).next as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh11 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head_0: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
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
            (*(*hTable).hh.tbl)
                .num_items = ((*(*hTable).hh.tbl).num_items).wrapping_sub(1);
            (*(*hTable).hh.tbl).num_items;
        }
    }
    tst[2 as libc::c_int as usize].hh.hashv = _hr_hashv_0;
    tst[2 as libc::c_int as usize]
        .hh
        .key = &mut *((*tst.as_mut_ptr().offset(2 as libc::c_int as isize)).name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void;
    tst[2 as libc::c_int as usize]
        .hh
        .keylen = strlen((tst[2 as libc::c_int as usize].name).as_mut_ptr())
        as libc::c_uint;
    if hTable.is_null() {
        tst[2 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
        tst[2 as libc::c_int as usize].hh.prev = 0 as *mut libc::c_void;
        tst[2 as libc::c_int as usize]
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (tst[2 as libc::c_int as usize].hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                tst[2 as libc::c_int as usize].hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(2 as libc::c_int as isize)).hh;
            (*tst[2 as libc::c_int as usize].hh.tbl).num_buckets = 32 as libc::c_uint;
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = 5 as libc::c_uint;
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .hho = (&mut (*tst.as_mut_ptr().offset(2 as libc::c_int as isize)).hh
                as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(
                    &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize)
                        as *mut hstruct_t as *mut libc::c_char,
                ) as libc::c_long;
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .signature = 0xa0111fe1 as libc::c_uint;
            if ((*tst[2 as libc::c_int as usize].hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*tst[2 as libc::c_int as usize].hh.tbl).buckets
                        as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hTable = &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut hstruct_t;
    } else {
        let mut _hs_iter_2: *mut libc::c_void = hTable as *mut libc::c_void;
        tst[2 as libc::c_int as usize].hh.tbl = (*hTable).hh.tbl;
        while !(cmpfunc(
            _hs_iter_2 as *mut hstruct_t as *const hstruct_t,
            &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int)
        {
            _hs_iter_2 = (*((_hs_iter_2 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            if _hs_iter_2.is_null() {
                break;
            }
        }
        if !_hs_iter_2.is_null() {
            tst[2 as libc::c_int as usize].hh.next = _hs_iter_2;
            tst[2 as libc::c_int as usize]
                .hh
                .prev = (*((_hs_iter_2 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            if !(tst[2 as libc::c_int as usize].hh.prev).is_null() {
                let ref mut fresh12 = (*((tst[2 as libc::c_int as usize].hh.prev
                    as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh12 = &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut hstruct_t as *mut libc::c_void;
            } else {
                hTable = &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut hstruct_t;
            }
            let ref mut fresh13 = (*((_hs_iter_2 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh13 = &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
        } else {
            tst[2 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
            tst[2 as libc::c_int as usize]
                .hh
                .prev = ((*(*hTable).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*hTable).hh.tbl).tail)
                .next = &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
            (*(*hTable).hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(2 as libc::c_int as isize)).hh;
        }
    }
    let mut _ha_bkt_2: libc::c_uint = 0;
    (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_add(1);
    (*(*hTable).hh.tbl).num_items;
    _ha_bkt_2 = _hr_hashv_0
        & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_2: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
        .offset(_ha_bkt_2 as isize) as *mut UT_hash_bucket;
    (*_ha_head_2).count = ((*_ha_head_2).count).wrapping_add(1);
    (*_ha_head_2).count;
    tst[2 as libc::c_int as usize].hh.hh_next = (*_ha_head_2).hh_head;
    tst[2 as libc::c_int as usize].hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_2).hh_head).is_null() {
        (*(*_ha_head_2).hh_head)
            .hh_prev = &mut (*tst.as_mut_ptr().offset(2 as libc::c_int as isize)).hh;
    }
    (*_ha_head_2)
        .hh_head = &mut (*tst.as_mut_ptr().offset(2 as libc::c_int as isize)).hh;
    if (*_ha_head_2).count
        >= ((*_ha_head_2).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint)
        && (*tst[2 as libc::c_int as usize].hh.tbl).noexpand == 0
    {
        let mut _he_bkt_2: libc::c_uint = 0;
        let mut _he_bkt_i_2: libc::c_uint = 0;
        let mut _he_thh_2: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_2: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_2: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_2: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_2 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul(
                    (*tst[2 as libc::c_int as usize].hh.tbl).num_buckets as libc::c_ulong,
                )
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_2.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_2 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul(
                        (*tst[2 as libc::c_int as usize].hh.tbl).num_buckets
                            as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .ideal_chain_maxlen = ((*tst[2 as libc::c_int as usize].hh.tbl).num_items
                >> ((*tst[2 as libc::c_int as usize].hh.tbl).log2_num_buckets)
                    .wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*tst[2 as libc::c_int as usize].hh.tbl).num_items
                        & ((*tst[2 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_2 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_2 < (*tst[2 as libc::c_int as usize].hh.tbl).num_buckets {
                _he_thh_2 = (*((*tst[2 as libc::c_int as usize].hh.tbl).buckets)
                    .offset(_he_bkt_i_2 as isize))
                    .hh_head;
                while !_he_thh_2.is_null() {
                    _he_hh_nxt_2 = (*_he_thh_2).hh_next;
                    _he_bkt_2 = (*_he_thh_2).hashv
                        & ((*tst[2 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_2 = &mut *_he_new_buckets_2.offset(_he_bkt_2 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_2).count = ((*_he_newbkt_2).count).wrapping_add(1);
                    if (*_he_newbkt_2).count
                        > (*tst[2 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen
                    {
                        (*tst[2 as libc::c_int as usize].hh.tbl)
                            .nonideal_items = ((*tst[2 as libc::c_int as usize].hh.tbl)
                            .nonideal_items)
                            .wrapping_add(1);
                        (*tst[2 as libc::c_int as usize].hh.tbl).nonideal_items;
                        if (*_he_newbkt_2).count
                            > ((*_he_newbkt_2).expand_mult)
                                .wrapping_mul(
                                    (*tst[2 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen,
                                )
                        {
                            (*_he_newbkt_2)
                                .expand_mult = ((*_he_newbkt_2).expand_mult)
                                .wrapping_add(1);
                            (*_he_newbkt_2).expand_mult;
                        }
                    }
                    (*_he_thh_2).hh_prev = 0 as *mut UT_hash_handle;
                    (*_he_thh_2).hh_next = (*_he_newbkt_2).hh_head;
                    if !((*_he_newbkt_2).hh_head).is_null() {
                        (*(*_he_newbkt_2).hh_head).hh_prev = _he_thh_2;
                    }
                    (*_he_newbkt_2).hh_head = _he_thh_2;
                    _he_thh_2 = _he_hh_nxt_2;
                }
                _he_bkt_i_2 = _he_bkt_i_2.wrapping_add(1);
                _he_bkt_i_2;
            }
            free((*tst[2 as libc::c_int as usize].hh.tbl).buckets as *mut libc::c_void);
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .num_buckets = ((*tst[2 as libc::c_int as usize].hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = ((*tst[2 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets)
                .wrapping_add(1);
            (*tst[2 as libc::c_int as usize].hh.tbl).log2_num_buckets;
            (*tst[2 as libc::c_int as usize].hh.tbl).buckets = _he_new_buckets_2;
            (*tst[2 as libc::c_int as usize].hh.tbl)
                .ineff_expands = if (*tst[2 as libc::c_int as usize].hh.tbl)
                .nonideal_items
                > (*tst[2 as libc::c_int as usize].hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*tst[2 as libc::c_int as usize].hh.tbl).ineff_expands)
                    .wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*tst[2 as libc::c_int as usize].hh.tbl).ineff_expands > 1 as libc::c_uint
            {
                (*tst[2 as libc::c_int as usize].hh.tbl)
                    .noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if replaced
        == &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut hstruct_t
    {} else {
        __assert_fail(
            b"replaced == &tst[2]\0" as *const u8 as *const libc::c_char,
            b"test87.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_8758: {
        if replaced
            == &mut *tst.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut hstruct_t
        {} else {
            __assert_fail(
                b"replaced == &tst[2]\0" as *const u8 as *const libc::c_char,
                b"test87.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    printtable(hTable);
    tst[6 as libc::c_int as usize].weight = 16 as libc::c_int;
    let mut _hj_i_3: libc::c_uint = 0;
    let mut _hj_j_3: libc::c_uint = 0;
    let mut _hj_k_3: libc::c_uint = 0;
    let mut _hj_key_3: *const libc::c_uchar = &mut *((*tst
        .as_mut_ptr()
        .offset(6 as libc::c_int as isize))
        .name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_uchar;
    hashvalue = 0xfeedbeef as libc::c_uint;
    _hj_j_3 = 0x9e3779b9 as libc::c_uint;
    _hj_i_3 = _hj_j_3;
    _hj_k_3 = strlen((tst[6 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint;
    while _hj_k_3 >= 12 as libc::c_uint {
        _hj_i_3 = _hj_i_3
            .wrapping_add(
                (*_hj_key_3.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_3.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_3.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_3.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j_3 = _hj_j_3
            .wrapping_add(
                (*_hj_key_3.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_3.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_3.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_3.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        hashvalue = hashvalue
            .wrapping_add(
                (*_hj_key_3.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_3.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_3.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_3.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(hashvalue);
        _hj_i_3 ^= hashvalue >> 13 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(hashvalue);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_3);
        hashvalue = hashvalue.wrapping_sub(_hj_j_3);
        hashvalue ^= _hj_j_3 >> 13 as libc::c_int;
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(hashvalue);
        _hj_i_3 ^= hashvalue >> 12 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(hashvalue);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_3);
        hashvalue = hashvalue.wrapping_sub(_hj_j_3);
        hashvalue ^= _hj_j_3 >> 5 as libc::c_int;
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(hashvalue);
        _hj_i_3 ^= hashvalue >> 3 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(hashvalue);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_3);
        hashvalue = hashvalue.wrapping_sub(_hj_j_3);
        hashvalue ^= _hj_j_3 >> 15 as libc::c_int;
        _hj_key_3 = _hj_key_3.offset(12 as libc::c_int as isize);
        _hj_k_3 = _hj_k_3.wrapping_sub(12 as libc::c_uint);
    }
    hashvalue = hashvalue
        .wrapping_add(
            strlen((tst[6 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint,
        );
    let mut current_block_1017: u64;
    match _hj_k_3 {
        11 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_3.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_1017 = 9487288224710417896;
        }
        10 => {
            current_block_1017 = 9487288224710417896;
        }
        9 => {
            current_block_1017 = 6567346657724079290;
        }
        8 => {
            current_block_1017 = 13423106481271102964;
        }
        7 => {
            current_block_1017 = 9234767613549682605;
        }
        6 => {
            current_block_1017 = 13953806965229014186;
        }
        5 => {
            current_block_1017 = 1913088862481173662;
        }
        4 => {
            current_block_1017 = 14073747602822267527;
        }
        3 => {
            current_block_1017 = 15623170874954075441;
        }
        2 => {
            current_block_1017 = 1929081184716539073;
        }
        1 => {
            current_block_1017 = 16530812127519842298;
        }
        _ => {
            current_block_1017 = 3577490590249162810;
        }
    }
    match current_block_1017 {
        9487288224710417896 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_3.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_1017 = 6567346657724079290;
        }
        _ => {}
    }
    match current_block_1017 {
        6567346657724079290 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_3.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_1017 = 13423106481271102964;
        }
        _ => {}
    }
    match current_block_1017 {
        13423106481271102964 => {
            _hj_j_3 = _hj_j_3
                .wrapping_add(
                    (*_hj_key_3.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_1017 = 9234767613549682605;
        }
        _ => {}
    }
    match current_block_1017 {
        9234767613549682605 => {
            _hj_j_3 = _hj_j_3
                .wrapping_add(
                    (*_hj_key_3.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_1017 = 13953806965229014186;
        }
        _ => {}
    }
    match current_block_1017 {
        13953806965229014186 => {
            _hj_j_3 = _hj_j_3
                .wrapping_add(
                    (*_hj_key_3.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_1017 = 1913088862481173662;
        }
        _ => {}
    }
    match current_block_1017 {
        1913088862481173662 => {
            _hj_j_3 = _hj_j_3
                .wrapping_add(
                    *_hj_key_3.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_1017 = 14073747602822267527;
        }
        _ => {}
    }
    match current_block_1017 {
        14073747602822267527 => {
            _hj_i_3 = _hj_i_3
                .wrapping_add(
                    (*_hj_key_3.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_1017 = 15623170874954075441;
        }
        _ => {}
    }
    match current_block_1017 {
        15623170874954075441 => {
            _hj_i_3 = _hj_i_3
                .wrapping_add(
                    (*_hj_key_3.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_1017 = 1929081184716539073;
        }
        _ => {}
    }
    match current_block_1017 {
        1929081184716539073 => {
            _hj_i_3 = _hj_i_3
                .wrapping_add(
                    (*_hj_key_3.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_1017 = 16530812127519842298;
        }
        _ => {}
    }
    match current_block_1017 {
        16530812127519842298 => {
            _hj_i_3 = _hj_i_3
                .wrapping_add(
                    *_hj_key_3.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
    _hj_i_3 = _hj_i_3.wrapping_sub(hashvalue);
    _hj_i_3 ^= hashvalue >> 13 as libc::c_int;
    _hj_j_3 = _hj_j_3.wrapping_sub(hashvalue);
    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
    _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_3);
    hashvalue = hashvalue.wrapping_sub(_hj_j_3);
    hashvalue ^= _hj_j_3 >> 13 as libc::c_int;
    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
    _hj_i_3 = _hj_i_3.wrapping_sub(hashvalue);
    _hj_i_3 ^= hashvalue >> 12 as libc::c_int;
    _hj_j_3 = _hj_j_3.wrapping_sub(hashvalue);
    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
    _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_3);
    hashvalue = hashvalue.wrapping_sub(_hj_j_3);
    hashvalue ^= _hj_j_3 >> 5 as libc::c_int;
    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
    _hj_i_3 = _hj_i_3.wrapping_sub(hashvalue);
    _hj_i_3 ^= hashvalue >> 3 as libc::c_int;
    _hj_j_3 = _hj_j_3.wrapping_sub(hashvalue);
    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
    _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_3);
    hashvalue = hashvalue.wrapping_sub(_hj_j_3);
    hashvalue ^= _hj_j_3 >> 15 as libc::c_int;
    replaced = 0 as *mut hstruct_t;
    replaced = 0 as *mut hstruct_t;
    if !hTable.is_null() {
        let mut _hf_bkt_1: libc::c_uint = 0;
        _hf_bkt_1 = hashvalue
            & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*hTable).hh.tbl).buckets).offset(_hf_bkt_1 as isize)).hh_head)
                .is_null()
            {
                replaced = ((*((*(*hTable).hh.tbl).buckets).offset(_hf_bkt_1 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut hstruct_t;
            } else {
                replaced = 0 as *mut hstruct_t;
            }
            while !replaced.is_null() {
                if (*replaced).hh.hashv == hashvalue
                    && (*replaced).hh.keylen as libc::c_ulong
                        == strlen((tst[6 as libc::c_int as usize].name).as_mut_ptr())
                {
                    if memcmp(
                        (*replaced).hh.key,
                        &mut *((*tst.as_mut_ptr().offset(6 as libc::c_int as isize))
                            .name)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut libc::c_char
                            as *const libc::c_void,
                        strlen((tst[6 as libc::c_int as usize].name).as_mut_ptr()),
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*replaced).hh.hh_next).is_null() {
                    replaced = ((*replaced).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut hstruct_t;
                } else {
                    replaced = 0 as *mut hstruct_t;
                }
            }
        }
    }
    if !replaced.is_null() {
        let mut _hd_hh_del_1: *mut UT_hash_handle = &mut (*replaced).hh;
        if ((*_hd_hh_del_1).prev).is_null() && ((*_hd_hh_del_1).next).is_null() {
            free((*(*hTable).hh.tbl).buckets as *mut libc::c_void);
            free((*hTable).hh.tbl as *mut libc::c_void);
            hTable = 0 as *mut hstruct_t;
        } else {
            let mut _hd_bkt_1: libc::c_uint = 0;
            if _hd_hh_del_1 == (*(*hTable).hh.tbl).tail {
                (*(*hTable).hh.tbl)
                    .tail = ((*_hd_hh_del_1).prev as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del_1).prev).is_null() {
                let ref mut fresh14 = (*(((*_hd_hh_del_1).prev as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh14 = (*_hd_hh_del_1).next;
            } else {
                hTable = (*_hd_hh_del_1).next as *mut hstruct_t;
            }
            if !((*_hd_hh_del_1).next).is_null() {
                let ref mut fresh15 = (*(((*_hd_hh_del_1).next as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh15 = (*_hd_hh_del_1).prev;
            }
            _hd_bkt_1 = (*_hd_hh_del_1).hashv
                & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _hd_head_1: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
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
            (*(*hTable).hh.tbl)
                .num_items = ((*(*hTable).hh.tbl).num_items).wrapping_sub(1);
            (*(*hTable).hh.tbl).num_items;
        }
    }
    tst[6 as libc::c_int as usize].hh.hashv = hashvalue;
    tst[6 as libc::c_int as usize]
        .hh
        .key = &mut *((*tst.as_mut_ptr().offset(6 as libc::c_int as isize)).name)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void;
    tst[6 as libc::c_int as usize]
        .hh
        .keylen = strlen((tst[6 as libc::c_int as usize].name).as_mut_ptr())
        as libc::c_uint;
    if hTable.is_null() {
        tst[6 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
        tst[6 as libc::c_int as usize].hh.prev = 0 as *mut libc::c_void;
        tst[6 as libc::c_int as usize]
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (tst[6 as libc::c_int as usize].hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                tst[6 as libc::c_int as usize].hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(6 as libc::c_int as isize)).hh;
            (*tst[6 as libc::c_int as usize].hh.tbl).num_buckets = 32 as libc::c_uint;
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = 5 as libc::c_uint;
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .hho = (&mut (*tst.as_mut_ptr().offset(6 as libc::c_int as isize)).hh
                as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(
                    &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize)
                        as *mut hstruct_t as *mut libc::c_char,
                ) as libc::c_long;
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .signature = 0xa0111fe1 as libc::c_uint;
            if ((*tst[6 as libc::c_int as usize].hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*tst[6 as libc::c_int as usize].hh.tbl).buckets
                        as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hTable = &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize)
            as *mut hstruct_t;
    } else {
        let mut _hs_iter_3: *mut libc::c_void = hTable as *mut libc::c_void;
        tst[6 as libc::c_int as usize].hh.tbl = (*hTable).hh.tbl;
        while !(cmpfunc(
            _hs_iter_3 as *mut hstruct_t as *const hstruct_t,
            &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize),
        ) > 0 as libc::c_int)
        {
            _hs_iter_3 = (*((_hs_iter_3 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            if _hs_iter_3.is_null() {
                break;
            }
        }
        if !_hs_iter_3.is_null() {
            tst[6 as libc::c_int as usize].hh.next = _hs_iter_3;
            tst[6 as libc::c_int as usize]
                .hh
                .prev = (*((_hs_iter_3 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            if !(tst[6 as libc::c_int as usize].hh.prev).is_null() {
                let ref mut fresh16 = (*((tst[6 as libc::c_int as usize].hh.prev
                    as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh16 = &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize)
                    as *mut hstruct_t as *mut libc::c_void;
            } else {
                hTable = &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize)
                    as *mut hstruct_t;
            }
            let ref mut fresh17 = (*((_hs_iter_3 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh17 = &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
        } else {
            tst[6 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
            tst[6 as libc::c_int as usize]
                .hh
                .prev = ((*(*hTable).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*hTable).hh.tbl).tail)
                .next = &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
            (*(*hTable).hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(6 as libc::c_int as isize)).hh;
        }
    }
    let mut _ha_bkt_3: libc::c_uint = 0;
    (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_add(1);
    (*(*hTable).hh.tbl).num_items;
    _ha_bkt_3 = hashvalue
        & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_3: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
        .offset(_ha_bkt_3 as isize) as *mut UT_hash_bucket;
    (*_ha_head_3).count = ((*_ha_head_3).count).wrapping_add(1);
    (*_ha_head_3).count;
    tst[6 as libc::c_int as usize].hh.hh_next = (*_ha_head_3).hh_head;
    tst[6 as libc::c_int as usize].hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_3).hh_head).is_null() {
        (*(*_ha_head_3).hh_head)
            .hh_prev = &mut (*tst.as_mut_ptr().offset(6 as libc::c_int as isize)).hh;
    }
    (*_ha_head_3)
        .hh_head = &mut (*tst.as_mut_ptr().offset(6 as libc::c_int as isize)).hh;
    if (*_ha_head_3).count
        >= ((*_ha_head_3).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint)
        && (*tst[6 as libc::c_int as usize].hh.tbl).noexpand == 0
    {
        let mut _he_bkt_3: libc::c_uint = 0;
        let mut _he_bkt_i_3: libc::c_uint = 0;
        let mut _he_thh_3: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_3: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_3: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_3: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_3 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul(
                    (*tst[6 as libc::c_int as usize].hh.tbl).num_buckets as libc::c_ulong,
                )
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_3.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_3 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul(
                        (*tst[6 as libc::c_int as usize].hh.tbl).num_buckets
                            as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .ideal_chain_maxlen = ((*tst[6 as libc::c_int as usize].hh.tbl).num_items
                >> ((*tst[6 as libc::c_int as usize].hh.tbl).log2_num_buckets)
                    .wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*tst[6 as libc::c_int as usize].hh.tbl).num_items
                        & ((*tst[6 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_3 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_3 < (*tst[6 as libc::c_int as usize].hh.tbl).num_buckets {
                _he_thh_3 = (*((*tst[6 as libc::c_int as usize].hh.tbl).buckets)
                    .offset(_he_bkt_i_3 as isize))
                    .hh_head;
                while !_he_thh_3.is_null() {
                    _he_hh_nxt_3 = (*_he_thh_3).hh_next;
                    _he_bkt_3 = (*_he_thh_3).hashv
                        & ((*tst[6 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_3 = &mut *_he_new_buckets_3.offset(_he_bkt_3 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_3).count = ((*_he_newbkt_3).count).wrapping_add(1);
                    if (*_he_newbkt_3).count
                        > (*tst[6 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen
                    {
                        (*tst[6 as libc::c_int as usize].hh.tbl)
                            .nonideal_items = ((*tst[6 as libc::c_int as usize].hh.tbl)
                            .nonideal_items)
                            .wrapping_add(1);
                        (*tst[6 as libc::c_int as usize].hh.tbl).nonideal_items;
                        if (*_he_newbkt_3).count
                            > ((*_he_newbkt_3).expand_mult)
                                .wrapping_mul(
                                    (*tst[6 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen,
                                )
                        {
                            (*_he_newbkt_3)
                                .expand_mult = ((*_he_newbkt_3).expand_mult)
                                .wrapping_add(1);
                            (*_he_newbkt_3).expand_mult;
                        }
                    }
                    (*_he_thh_3).hh_prev = 0 as *mut UT_hash_handle;
                    (*_he_thh_3).hh_next = (*_he_newbkt_3).hh_head;
                    if !((*_he_newbkt_3).hh_head).is_null() {
                        (*(*_he_newbkt_3).hh_head).hh_prev = _he_thh_3;
                    }
                    (*_he_newbkt_3).hh_head = _he_thh_3;
                    _he_thh_3 = _he_hh_nxt_3;
                }
                _he_bkt_i_3 = _he_bkt_i_3.wrapping_add(1);
                _he_bkt_i_3;
            }
            free((*tst[6 as libc::c_int as usize].hh.tbl).buckets as *mut libc::c_void);
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .num_buckets = ((*tst[6 as libc::c_int as usize].hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = ((*tst[6 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets)
                .wrapping_add(1);
            (*tst[6 as libc::c_int as usize].hh.tbl).log2_num_buckets;
            (*tst[6 as libc::c_int as usize].hh.tbl).buckets = _he_new_buckets_3;
            (*tst[6 as libc::c_int as usize].hh.tbl)
                .ineff_expands = if (*tst[6 as libc::c_int as usize].hh.tbl)
                .nonideal_items
                > (*tst[6 as libc::c_int as usize].hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*tst[6 as libc::c_int as usize].hh.tbl).ineff_expands)
                    .wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*tst[6 as libc::c_int as usize].hh.tbl).ineff_expands > 1 as libc::c_uint
            {
                (*tst[6 as libc::c_int as usize].hh.tbl)
                    .noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if replaced
        == &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut hstruct_t
    {} else {
        __assert_fail(
            b"replaced == &tst[6]\0" as *const u8 as *const libc::c_char,
            b"test87.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_6030: {
        if replaced
            == &mut *tst.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut hstruct_t
        {} else {
            __assert_fail(
                b"replaced == &tst[6]\0" as *const u8 as *const libc::c_char,
                b"test87.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    printtable(hTable);
    let mut _hd_hh_del_2: *mut UT_hash_handle = &mut (*tst
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
        .hh;
    if ((*_hd_hh_del_2).prev).is_null() && ((*_hd_hh_del_2).next).is_null() {
        free((*(*hTable).hh.tbl).buckets as *mut libc::c_void);
        free((*hTable).hh.tbl as *mut libc::c_void);
        hTable = 0 as *mut hstruct_t;
    } else {
        let mut _hd_bkt_2: libc::c_uint = 0;
        if _hd_hh_del_2 == (*(*hTable).hh.tbl).tail {
            (*(*hTable).hh.tbl)
                .tail = ((*_hd_hh_del_2).prev as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle;
        }
        if !((*_hd_hh_del_2).prev).is_null() {
            let ref mut fresh18 = (*(((*_hd_hh_del_2).prev as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            *fresh18 = (*_hd_hh_del_2).next;
        } else {
            hTable = (*_hd_hh_del_2).next as *mut hstruct_t;
        }
        if !((*_hd_hh_del_2).next).is_null() {
            let ref mut fresh19 = (*(((*_hd_hh_del_2).next as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh19 = (*_hd_hh_del_2).prev;
        }
        _hd_bkt_2 = (*_hd_hh_del_2).hashv
            & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _hd_head_2: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
            .offset(_hd_bkt_2 as isize) as *mut UT_hash_bucket;
        (*_hd_head_2).count = ((*_hd_head_2).count).wrapping_sub(1);
        (*_hd_head_2).count;
        if (*_hd_head_2).hh_head == _hd_hh_del_2 {
            (*_hd_head_2).hh_head = (*_hd_hh_del_2).hh_next;
        }
        if !((*_hd_hh_del_2).hh_prev).is_null() {
            (*(*_hd_hh_del_2).hh_prev).hh_next = (*_hd_hh_del_2).hh_next;
        }
        if !((*_hd_hh_del_2).hh_next).is_null() {
            (*(*_hd_hh_del_2).hh_next).hh_prev = (*_hd_hh_del_2).hh_prev;
        }
        (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_sub(1);
        (*(*hTable).hh.tbl).num_items;
    }
    printtable(hTable);
    let mut _hj_i_4: libc::c_uint = 0;
    let mut _hj_j_4: libc::c_uint = 0;
    let mut _hj_k_4: libc::c_uint = 0;
    let mut _hj_key_4: *const libc::c_uchar = (tst[1 as libc::c_int as usize].name)
        .as_mut_ptr() as *const libc::c_uchar;
    hashvalue = 0xfeedbeef as libc::c_uint;
    _hj_j_4 = 0x9e3779b9 as libc::c_uint;
    _hj_i_4 = _hj_j_4;
    _hj_k_4 = strlen((tst[1 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint;
    while _hj_k_4 >= 12 as libc::c_uint {
        _hj_i_4 = _hj_i_4
            .wrapping_add(
                (*_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_4.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_4.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_4.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j_4 = _hj_j_4
            .wrapping_add(
                (*_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_4.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_4.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_4.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        hashvalue = hashvalue
            .wrapping_add(
                (*_hj_key_4.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_4.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_4.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_4.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(hashvalue);
        _hj_i_4 ^= hashvalue >> 13 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(hashvalue);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_4);
        hashvalue = hashvalue.wrapping_sub(_hj_j_4);
        hashvalue ^= _hj_j_4 >> 13 as libc::c_int;
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(hashvalue);
        _hj_i_4 ^= hashvalue >> 12 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(hashvalue);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_4);
        hashvalue = hashvalue.wrapping_sub(_hj_j_4);
        hashvalue ^= _hj_j_4 >> 5 as libc::c_int;
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(hashvalue);
        _hj_i_4 ^= hashvalue >> 3 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(hashvalue);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
        hashvalue = hashvalue.wrapping_sub(_hj_i_4);
        hashvalue = hashvalue.wrapping_sub(_hj_j_4);
        hashvalue ^= _hj_j_4 >> 15 as libc::c_int;
        _hj_key_4 = _hj_key_4.offset(12 as libc::c_int as isize);
        _hj_k_4 = _hj_k_4.wrapping_sub(12 as libc::c_uint);
    }
    hashvalue = hashvalue
        .wrapping_add(
            strlen((tst[1 as libc::c_int as usize].name).as_mut_ptr()) as libc::c_uint,
        );
    let mut current_block_1332: u64;
    match _hj_k_4 {
        11 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_4.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_1332 = 12984810581895796367;
        }
        10 => {
            current_block_1332 = 12984810581895796367;
        }
        9 => {
            current_block_1332 = 13435336783582280770;
        }
        8 => {
            current_block_1332 = 6466194886500373342;
        }
        7 => {
            current_block_1332 = 1097971609518322498;
        }
        6 => {
            current_block_1332 = 8030768351463208789;
        }
        5 => {
            current_block_1332 = 9787172760135764549;
        }
        4 => {
            current_block_1332 = 9426422318600939105;
        }
        3 => {
            current_block_1332 = 4517842913570656226;
        }
        2 => {
            current_block_1332 = 7896051486410544307;
        }
        1 => {
            current_block_1332 = 17409469539518602963;
        }
        _ => {
            current_block_1332 = 17957773900984220658;
        }
    }
    match current_block_1332 {
        12984810581895796367 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_4.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_1332 = 13435336783582280770;
        }
        _ => {}
    }
    match current_block_1332 {
        13435336783582280770 => {
            hashvalue = hashvalue
                .wrapping_add(
                    (*_hj_key_4.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_1332 = 6466194886500373342;
        }
        _ => {}
    }
    match current_block_1332 {
        6466194886500373342 => {
            _hj_j_4 = _hj_j_4
                .wrapping_add(
                    (*_hj_key_4.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_1332 = 1097971609518322498;
        }
        _ => {}
    }
    match current_block_1332 {
        1097971609518322498 => {
            _hj_j_4 = _hj_j_4
                .wrapping_add(
                    (*_hj_key_4.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_1332 = 8030768351463208789;
        }
        _ => {}
    }
    match current_block_1332 {
        8030768351463208789 => {
            _hj_j_4 = _hj_j_4
                .wrapping_add(
                    (*_hj_key_4.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_1332 = 9787172760135764549;
        }
        _ => {}
    }
    match current_block_1332 {
        9787172760135764549 => {
            _hj_j_4 = _hj_j_4
                .wrapping_add(
                    *_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_1332 = 9426422318600939105;
        }
        _ => {}
    }
    match current_block_1332 {
        9426422318600939105 => {
            _hj_i_4 = _hj_i_4
                .wrapping_add(
                    (*_hj_key_4.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_1332 = 4517842913570656226;
        }
        _ => {}
    }
    match current_block_1332 {
        4517842913570656226 => {
            _hj_i_4 = _hj_i_4
                .wrapping_add(
                    (*_hj_key_4.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_1332 = 7896051486410544307;
        }
        _ => {}
    }
    match current_block_1332 {
        7896051486410544307 => {
            _hj_i_4 = _hj_i_4
                .wrapping_add(
                    (*_hj_key_4.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_1332 = 17409469539518602963;
        }
        _ => {}
    }
    match current_block_1332 {
        17409469539518602963 => {
            _hj_i_4 = _hj_i_4
                .wrapping_add(
                    *_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
    _hj_i_4 = _hj_i_4.wrapping_sub(hashvalue);
    _hj_i_4 ^= hashvalue >> 13 as libc::c_int;
    _hj_j_4 = _hj_j_4.wrapping_sub(hashvalue);
    _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
    _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_4);
    hashvalue = hashvalue.wrapping_sub(_hj_j_4);
    hashvalue ^= _hj_j_4 >> 13 as libc::c_int;
    _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
    _hj_i_4 = _hj_i_4.wrapping_sub(hashvalue);
    _hj_i_4 ^= hashvalue >> 12 as libc::c_int;
    _hj_j_4 = _hj_j_4.wrapping_sub(hashvalue);
    _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
    _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_4);
    hashvalue = hashvalue.wrapping_sub(_hj_j_4);
    hashvalue ^= _hj_j_4 >> 5 as libc::c_int;
    _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
    _hj_i_4 = _hj_i_4.wrapping_sub(hashvalue);
    _hj_i_4 ^= hashvalue >> 3 as libc::c_int;
    _hj_j_4 = _hj_j_4.wrapping_sub(hashvalue);
    _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
    _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
    hashvalue = hashvalue.wrapping_sub(_hj_i_4);
    hashvalue = hashvalue.wrapping_sub(_hj_j_4);
    hashvalue ^= _hj_j_4 >> 15 as libc::c_int;
    tst[1 as libc::c_int as usize].hh.hashv = hashvalue;
    tst[1 as libc::c_int as usize]
        .hh
        .key = (tst[1 as libc::c_int as usize].name).as_mut_ptr() as *const libc::c_void;
    tst[1 as libc::c_int as usize]
        .hh
        .keylen = strlen((tst[1 as libc::c_int as usize].name).as_mut_ptr())
        as libc::c_uint;
    if hTable.is_null() {
        tst[1 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
        tst[1 as libc::c_int as usize].hh.prev = 0 as *mut libc::c_void;
        tst[1 as libc::c_int as usize]
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if (tst[1 as libc::c_int as usize].hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                tst[1 as libc::c_int as usize].hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(1 as libc::c_int as isize)).hh;
            (*tst[1 as libc::c_int as usize].hh.tbl).num_buckets = 32 as libc::c_uint;
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = 5 as libc::c_uint;
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .hho = (&mut (*tst.as_mut_ptr().offset(1 as libc::c_int as isize)).hh
                as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(
                    &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut hstruct_t as *mut libc::c_char,
                ) as libc::c_long;
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .signature = 0xa0111fe1 as libc::c_uint;
            if ((*tst[1 as libc::c_int as usize].hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*tst[1 as libc::c_int as usize].hh.tbl).buckets
                        as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        hTable = &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut hstruct_t;
    } else {
        let mut _hs_iter_4: *mut libc::c_void = hTable as *mut libc::c_void;
        tst[1 as libc::c_int as usize].hh.tbl = (*hTable).hh.tbl;
        while !(cmpfunc(
            _hs_iter_4 as *mut hstruct_t as *const hstruct_t,
            &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int)
        {
            _hs_iter_4 = (*((_hs_iter_4 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .next;
            if _hs_iter_4.is_null() {
                break;
            }
        }
        if !_hs_iter_4.is_null() {
            tst[1 as libc::c_int as usize].hh.next = _hs_iter_4;
            tst[1 as libc::c_int as usize]
                .hh
                .prev = (*((_hs_iter_4 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            if !(tst[1 as libc::c_int as usize].hh.prev).is_null() {
                let ref mut fresh20 = (*((tst[1 as libc::c_int as usize].hh.prev
                    as *mut libc::c_char)
                    .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh20 = &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut hstruct_t as *mut libc::c_void;
            } else {
                hTable = &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut hstruct_t;
            }
            let ref mut fresh21 = (*((_hs_iter_4 as *mut libc::c_char)
                .offset((*(*hTable).hh.tbl).hho as isize) as *mut libc::c_void
                as *mut UT_hash_handle))
                .prev;
            *fresh21 = &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
        } else {
            tst[1 as libc::c_int as usize].hh.next = 0 as *mut libc::c_void;
            tst[1 as libc::c_int as usize]
                .hh
                .prev = ((*(*hTable).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*hTable).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*hTable).hh.tbl).tail)
                .next = &mut *tst.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut hstruct_t as *mut libc::c_void;
            (*(*hTable).hh.tbl)
                .tail = &mut (*tst.as_mut_ptr().offset(1 as libc::c_int as isize)).hh;
        }
    }
    let mut _ha_bkt_4: libc::c_uint = 0;
    (*(*hTable).hh.tbl).num_items = ((*(*hTable).hh.tbl).num_items).wrapping_add(1);
    (*(*hTable).hh.tbl).num_items;
    _ha_bkt_4 = hashvalue
        & ((*(*hTable).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_4: *mut UT_hash_bucket = &mut *((*(*hTable).hh.tbl).buckets)
        .offset(_ha_bkt_4 as isize) as *mut UT_hash_bucket;
    (*_ha_head_4).count = ((*_ha_head_4).count).wrapping_add(1);
    (*_ha_head_4).count;
    tst[1 as libc::c_int as usize].hh.hh_next = (*_ha_head_4).hh_head;
    tst[1 as libc::c_int as usize].hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_4).hh_head).is_null() {
        (*(*_ha_head_4).hh_head)
            .hh_prev = &mut (*tst.as_mut_ptr().offset(1 as libc::c_int as isize)).hh;
    }
    (*_ha_head_4)
        .hh_head = &mut (*tst.as_mut_ptr().offset(1 as libc::c_int as isize)).hh;
    if (*_ha_head_4).count
        >= ((*_ha_head_4).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint)
        && (*tst[1 as libc::c_int as usize].hh.tbl).noexpand == 0
    {
        let mut _he_bkt_4: libc::c_uint = 0;
        let mut _he_bkt_i_4: libc::c_uint = 0;
        let mut _he_thh_4: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_4: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_4: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_4: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_4 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul(
                    (*tst[1 as libc::c_int as usize].hh.tbl).num_buckets as libc::c_ulong,
                )
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_4.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_4 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul(
                        (*tst[1 as libc::c_int as usize].hh.tbl).num_buckets
                            as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .ideal_chain_maxlen = ((*tst[1 as libc::c_int as usize].hh.tbl).num_items
                >> ((*tst[1 as libc::c_int as usize].hh.tbl).log2_num_buckets)
                    .wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*tst[1 as libc::c_int as usize].hh.tbl).num_items
                        & ((*tst[1 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_4 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_4 < (*tst[1 as libc::c_int as usize].hh.tbl).num_buckets {
                _he_thh_4 = (*((*tst[1 as libc::c_int as usize].hh.tbl).buckets)
                    .offset(_he_bkt_i_4 as isize))
                    .hh_head;
                while !_he_thh_4.is_null() {
                    _he_hh_nxt_4 = (*_he_thh_4).hh_next;
                    _he_bkt_4 = (*_he_thh_4).hashv
                        & ((*tst[1 as libc::c_int as usize].hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_4 = &mut *_he_new_buckets_4.offset(_he_bkt_4 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_4).count = ((*_he_newbkt_4).count).wrapping_add(1);
                    if (*_he_newbkt_4).count
                        > (*tst[1 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen
                    {
                        (*tst[1 as libc::c_int as usize].hh.tbl)
                            .nonideal_items = ((*tst[1 as libc::c_int as usize].hh.tbl)
                            .nonideal_items)
                            .wrapping_add(1);
                        (*tst[1 as libc::c_int as usize].hh.tbl).nonideal_items;
                        if (*_he_newbkt_4).count
                            > ((*_he_newbkt_4).expand_mult)
                                .wrapping_mul(
                                    (*tst[1 as libc::c_int as usize].hh.tbl).ideal_chain_maxlen,
                                )
                        {
                            (*_he_newbkt_4)
                                .expand_mult = ((*_he_newbkt_4).expand_mult)
                                .wrapping_add(1);
                            (*_he_newbkt_4).expand_mult;
                        }
                    }
                    (*_he_thh_4).hh_prev = 0 as *mut UT_hash_handle;
                    (*_he_thh_4).hh_next = (*_he_newbkt_4).hh_head;
                    if !((*_he_newbkt_4).hh_head).is_null() {
                        (*(*_he_newbkt_4).hh_head).hh_prev = _he_thh_4;
                    }
                    (*_he_newbkt_4).hh_head = _he_thh_4;
                    _he_thh_4 = _he_hh_nxt_4;
                }
                _he_bkt_i_4 = _he_bkt_i_4.wrapping_add(1);
                _he_bkt_i_4;
            }
            free((*tst[1 as libc::c_int as usize].hh.tbl).buckets as *mut libc::c_void);
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .num_buckets = ((*tst[1 as libc::c_int as usize].hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets = ((*tst[1 as libc::c_int as usize].hh.tbl)
                .log2_num_buckets)
                .wrapping_add(1);
            (*tst[1 as libc::c_int as usize].hh.tbl).log2_num_buckets;
            (*tst[1 as libc::c_int as usize].hh.tbl).buckets = _he_new_buckets_4;
            (*tst[1 as libc::c_int as usize].hh.tbl)
                .ineff_expands = if (*tst[1 as libc::c_int as usize].hh.tbl)
                .nonideal_items
                > (*tst[1 as libc::c_int as usize].hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*tst[1 as libc::c_int as usize].hh.tbl).ineff_expands)
                    .wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*tst[1 as libc::c_int as usize].hh.tbl).ineff_expands > 1 as libc::c_uint
            {
                (*tst[1 as libc::c_int as usize].hh.tbl)
                    .noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    printtable(hTable);
    delitem(&mut hTable, b"muh1\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh7\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh3\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh9\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh2\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh11\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh4\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh6\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh5\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh8\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh10\0" as *const u8 as *const libc::c_char);
    delitem(&mut hTable, b"muh12\0" as *const u8 as *const libc::c_char);
    printtable(hTable);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

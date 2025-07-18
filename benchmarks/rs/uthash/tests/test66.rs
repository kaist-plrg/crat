use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
pub struct person_t {
    pub first_name: [libc::c_char; 10],
    pub id: libc::c_int,
    pub hh: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut people: *mut person_t = 0 as *mut person_t;
    let mut person: *mut person_t = 0 as *mut person_t;
    let mut name: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut names: [*const libc::c_char; 11] = [
        b"bob\0" as *const u8 as *const libc::c_char,
        b"jack\0" as *const u8 as *const libc::c_char,
        b"gary\0" as *const u8 as *const libc::c_char,
        b"ty\0" as *const u8 as *const libc::c_char,
        b"bo\0" as *const u8 as *const libc::c_char,
        b"phil\0" as *const u8 as *const libc::c_char,
        b"art\0" as *const u8 as *const libc::c_char,
        b"gil\0" as *const u8 as *const libc::c_char,
        b"buck\0" as *const u8 as *const libc::c_char,
        b"ted\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut id: libc::c_int = 0 as libc::c_int;
    name = names.as_mut_ptr();
    while !(*name).is_null() {
        person = malloc(::std::mem::size_of::<person_t>() as libc::c_ulong)
            as *mut person_t;
        if person.is_null() {
            exit(-(1 as libc::c_int));
        }
        strcpy(((*person).first_name).as_mut_ptr(), *name);
        let fresh0 = id;
        id = id + 1;
        (*person).id = fresh0;
        let mut _uthash_hastr_keylen: libc::c_uint = strlen(
            ((*person).first_name).as_mut_ptr(),
        ) as libc::c_uint;
        let mut _ha_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut *((*person).first_name)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_uchar;
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
        let mut current_block_58: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 8520138978958347577;
            }
            10 => {
                current_block_58 = 8520138978958347577;
            }
            9 => {
                current_block_58 = 3023463617567964315;
            }
            8 => {
                current_block_58 = 2694013637280298776;
            }
            7 => {
                current_block_58 = 12200064328057794513;
            }
            6 => {
                current_block_58 = 2266290538919981374;
            }
            5 => {
                current_block_58 = 10255622956629795657;
            }
            4 => {
                current_block_58 = 14141891332124833771;
            }
            3 => {
                current_block_58 = 6068896490444857321;
            }
            2 => {
                current_block_58 = 5349868339541838124;
            }
            1 => {
                current_block_58 = 14033308474174107022;
            }
            _ => {
                current_block_58 = 5891011138178424807;
            }
        }
        match current_block_58 {
            8520138978958347577 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 3023463617567964315;
            }
            _ => {}
        }
        match current_block_58 {
            3023463617567964315 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 2694013637280298776;
            }
            _ => {}
        }
        match current_block_58 {
            2694013637280298776 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 12200064328057794513;
            }
            _ => {}
        }
        match current_block_58 {
            12200064328057794513 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 2266290538919981374;
            }
            _ => {}
        }
        match current_block_58 {
            2266290538919981374 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 10255622956629795657;
            }
            _ => {}
        }
        match current_block_58 {
            10255622956629795657 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_58 = 14141891332124833771;
            }
            _ => {}
        }
        match current_block_58 {
            14141891332124833771 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 6068896490444857321;
            }
            _ => {}
        }
        match current_block_58 {
            6068896490444857321 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 5349868339541838124;
            }
            _ => {}
        }
        match current_block_58 {
            5349868339541838124 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 14033308474174107022;
            }
            _ => {}
        }
        match current_block_58 {
            14033308474174107022 => {
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
        (*person).hh.hashv = _ha_hashv;
        (*person)
            .hh
            .key = &mut *((*person).first_name)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_void;
        (*person).hh.keylen = _uthash_hastr_keylen;
        if people.is_null() {
            (*person).hh.next = 0 as *mut libc::c_void;
            (*person).hh.prev = 0 as *mut libc::c_void;
            (*person)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*person).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*person).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*person).hh.tbl).tail = &mut (*person).hh;
                (*(*person).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*person).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*person).hh.tbl)
                    .hho = (&mut (*person).hh as *mut UT_hash_handle
                    as *mut libc::c_char)
                    .offset_from(person as *mut libc::c_char) as libc::c_long;
                (*(*person).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*person).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*person).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*person).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            people = person;
        } else {
            (*person).hh.tbl = (*people).hh.tbl;
            (*person).hh.next = 0 as *mut libc::c_void;
            (*person)
                .hh
                .prev = ((*(*people).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*people).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*people).hh.tbl).tail).next = person as *mut libc::c_void;
            (*(*people).hh.tbl).tail = &mut (*person).hh;
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*people).hh.tbl).num_items = ((*(*people).hh.tbl).num_items).wrapping_add(1);
        (*(*people).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*people).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*people).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*person).hh.hh_next = (*_ha_head).hh_head;
        (*person).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*person).hh;
        }
        (*_ha_head).hh_head = &mut (*person).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*person).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*person).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*person).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*person).hh.tbl)
                    .ideal_chain_maxlen = ((*(*person).hh.tbl).num_items
                    >> ((*(*person).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*person).hh.tbl).num_items
                            & ((*(*person).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*person).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*person).hh.tbl).num_buckets {
                    _he_thh = (*((*(*person).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*person).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*person).hh.tbl).ideal_chain_maxlen {
                            (*(*person).hh.tbl)
                                .nonideal_items = ((*(*person).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*person).hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul((*(*person).hh.tbl).ideal_chain_maxlen)
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
                free((*(*person).hh.tbl).buckets as *mut libc::c_void);
                (*(*person).hh.tbl)
                    .num_buckets = ((*(*person).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*person).hh.tbl)
                    .log2_num_buckets = ((*(*person).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*person).hh.tbl).log2_num_buckets;
                (*(*person).hh.tbl).buckets = _he_new_buckets;
                (*(*person).hh.tbl)
                    .ineff_expands = if (*(*person).hh.tbl).nonideal_items
                    > (*(*person).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*person).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*person).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*person).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        printf(
            b"added %s (id %d)\n\0" as *const u8 as *const libc::c_char,
            ((*person).first_name).as_mut_ptr(),
            (*person).id,
        );
        name = name.offset(1);
        name;
    }
    person = 0 as *mut person_t;
    let mut p: *mut *mut person_t = &mut person;
    name = names.as_mut_ptr();
    while !(*name).is_null() {
        let mut _uthash_hfstr_keylen: libc::c_uint = strlen(*name) as libc::c_uint;
        *p = 0 as *mut person_t;
        if !people.is_null() {
            let mut _hf_hashv: libc::c_uint = 0;
            let mut _hj_i_0: libc::c_uint = 0;
            let mut _hj_j_0: libc::c_uint = 0;
            let mut _hj_k_0: libc::c_uint = 0;
            let mut _hj_key_0: *const libc::c_uchar = *name as *const libc::c_uchar;
            _hf_hashv = 0xfeedbeef as libc::c_uint;
            _hj_j_0 = 0x9e3779b9 as libc::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = _uthash_hfstr_keylen;
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
            _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
            let mut current_block_257: u64;
            match _hj_k_0 {
                11 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_257 = 6849681728645387701;
                }
                10 => {
                    current_block_257 = 6849681728645387701;
                }
                9 => {
                    current_block_257 = 7054481666415124968;
                }
                8 => {
                    current_block_257 = 6728217270221394124;
                }
                7 => {
                    current_block_257 = 15457071197130330223;
                }
                6 => {
                    current_block_257 = 18037386337399018045;
                }
                5 => {
                    current_block_257 = 4634307283396172174;
                }
                4 => {
                    current_block_257 = 14008594173364196201;
                }
                3 => {
                    current_block_257 = 7549413860336125482;
                }
                2 => {
                    current_block_257 = 10495276606450942267;
                }
                1 => {
                    current_block_257 = 15065918127120777014;
                }
                _ => {
                    current_block_257 = 11099343707781121639;
                }
            }
            match current_block_257 {
                6849681728645387701 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_257 = 7054481666415124968;
                }
                _ => {}
            }
            match current_block_257 {
                7054481666415124968 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_257 = 6728217270221394124;
                }
                _ => {}
            }
            match current_block_257 {
                6728217270221394124 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_257 = 15457071197130330223;
                }
                _ => {}
            }
            match current_block_257 {
                15457071197130330223 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_257 = 18037386337399018045;
                }
                _ => {}
            }
            match current_block_257 {
                18037386337399018045 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_257 = 4634307283396172174;
                }
                _ => {}
            }
            match current_block_257 {
                4634307283396172174 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_257 = 14008594173364196201;
                }
                _ => {}
            }
            match current_block_257 {
                14008594173364196201 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_257 = 7549413860336125482;
                }
                _ => {}
            }
            match current_block_257 {
                7549413860336125482 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_257 = 10495276606450942267;
                }
                _ => {}
            }
            match current_block_257 {
                10495276606450942267 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_257 = 15065918127120777014;
                }
                _ => {}
            }
            match current_block_257 {
                15065918127120777014 => {
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
            *p = 0 as *mut person_t;
            if !people.is_null() {
                let mut _hf_bkt: libc::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & ((*(*people).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                if 1 as libc::c_int != 0 as libc::c_int {
                    if !((*((*(*people).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head)
                        .is_null()
                    {
                        *p = ((*((*(*people).hh.tbl).buckets).offset(_hf_bkt as isize))
                            .hh_head as *mut libc::c_char)
                            .offset(-((*(*people).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut person_t;
                    } else {
                        *p = 0 as *mut person_t;
                    }
                    while !(*p).is_null() {
                        if (**p).hh.hashv == _hf_hashv
                            && (**p).hh.keylen == _uthash_hfstr_keylen
                        {
                            if memcmp(
                                (**p).hh.key,
                                *name as *const libc::c_void,
                                _uthash_hfstr_keylen as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                break;
                            }
                        }
                        if !((**p).hh.hh_next).is_null() {
                            *p = ((**p).hh.hh_next as *mut libc::c_char)
                                .offset(-((*(*people).hh.tbl).hho as isize))
                                as *mut libc::c_void as *mut person_t;
                        } else {
                            *p = 0 as *mut person_t;
                        }
                    }
                }
            }
        }
        if !person.is_null() {
            printf(
                b"found %s (id %d)\n\0" as *const u8 as *const libc::c_char,
                ((*person).first_name).as_mut_ptr(),
                (*person).id,
            );
        } else {
            printf(b"failed to find %s\n\0" as *const u8 as *const libc::c_char, *name);
        }
        name = name.offset(1);
        name;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

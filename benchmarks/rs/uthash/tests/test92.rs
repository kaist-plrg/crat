use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct example_user_t {
    pub id: libc::c_int,
    pub cookie: libc::c_int,
    pub hh: UT_hash_handle,
    pub hh2: UT_hash_handle,
    pub mem_failed: libc::c_int,
}
static mut malloc_cnt: libc::c_int = 0 as libc::c_int;
static mut malloc_failed: libc::c_int = 0 as libc::c_int;
static mut free_cnt: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn alt_malloc(mut sz: size_t) -> *mut libc::c_void {
    malloc_cnt -= 1;
    if malloc_cnt <= 0 as libc::c_int {
        malloc_failed = 1 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    malloc_failed = 0 as libc::c_int;
    return malloc(sz);
}
unsafe extern "C" fn alt_free(mut ptr: *mut libc::c_void) {
    free_cnt += 1;
    free_cnt;
    free(ptr);
}
unsafe extern "C" fn complain(
    mut index: libc::c_int,
    mut users: *mut example_user_t,
    mut user: *mut example_user_t,
) {
    let mut expected_frees: libc::c_int = 3 as libc::c_int - index;
    if !users.is_null() {
        printf(
            b"%d: users hash must be empty\n\0" as *const u8 as *const libc::c_char,
            index,
        );
    }
    if !((*user).hh.tbl).is_null() {
        printf(
            b"%d hash table must be empty\n\0" as *const u8 as *const libc::c_char,
            index,
        );
    }
    if free_cnt != expected_frees {
        printf(
            b"%d Expected %d frees, only had %d\n\0" as *const u8 as *const libc::c_char,
            index,
            expected_frees,
            free_cnt,
        );
    }
    if (*user).mem_failed != 1 as libc::c_int {
        printf(
            b"%d Expected user->mem_failed(%d) to be 1\n\0" as *const u8
                as *const libc::c_char,
            index,
            (*user).mem_failed,
        );
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut users: *mut example_user_t = 0 as *mut example_user_t;
    let mut user: *mut example_user_t = malloc(
        ::std::mem::size_of::<example_user_t>() as libc::c_ulong,
    ) as *mut example_user_t;
    let mut test: *mut example_user_t = 0 as *mut example_user_t;
    let mut users2: *mut example_user_t = 0 as *mut example_user_t;
    let mut id: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut saved_cnt: libc::c_int = 0;
    (*user).id = id;
    malloc_cnt = 2 as libc::c_int;
    (*user).mem_failed = 0 as libc::c_int;
    free_cnt = 0 as libc::c_int;
    (*user).hh.tbl = 1 as libc::c_int as *mut UT_hash_table;
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
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
    let mut current_block_56: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_56 = 10595313236571774494;
        }
        10 => {
            current_block_56 = 10595313236571774494;
        }
        9 => {
            current_block_56 = 17180366934518335839;
        }
        8 => {
            current_block_56 = 6119782984148224838;
        }
        7 => {
            current_block_56 = 1307902132907409461;
        }
        6 => {
            current_block_56 = 11926768043810305289;
        }
        5 => {
            current_block_56 = 7513194316507744915;
        }
        4 => {
            current_block_56 = 3263196846889284050;
        }
        3 => {
            current_block_56 = 1452672780278964339;
        }
        2 => {
            current_block_56 = 11372034502583561295;
        }
        1 => {
            current_block_56 = 10974190239340099840;
        }
        _ => {
            current_block_56 = 12497913735442871383;
        }
    }
    match current_block_56 {
        10595313236571774494 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_56 = 17180366934518335839;
        }
        _ => {}
    }
    match current_block_56 {
        17180366934518335839 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_56 = 6119782984148224838;
        }
        _ => {}
    }
    match current_block_56 {
        6119782984148224838 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_56 = 1307902132907409461;
        }
        _ => {}
    }
    match current_block_56 {
        1307902132907409461 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_56 = 11926768043810305289;
        }
        _ => {}
    }
    match current_block_56 {
        11926768043810305289 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_56 = 7513194316507744915;
        }
        _ => {}
    }
    match current_block_56 {
        7513194316507744915 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_56 = 3263196846889284050;
        }
        _ => {}
    }
    match current_block_56 {
        3263196846889284050 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_56 = 1452672780278964339;
        }
        _ => {}
    }
    match current_block_56 {
        1452672780278964339 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_56 = 11372034502583561295;
        }
        _ => {}
    }
    match current_block_56 {
        11372034502583561295 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_56 = 10974190239340099840;
        }
        _ => {}
    }
    match current_block_56 {
        10974190239340099840 => {
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
    let mut _ha_oomed: libc::c_int = 0 as libc::c_int;
    (*user).hh.hashv = _ha_hashv;
    (*user).hh.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
    (*user)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if users.is_null() {
        (*user).hh.next = 0 as *mut libc::c_void;
        (*user).hh.prev = 0 as *mut libc::c_void;
        (*user)
            .hh
            .tbl = alt_malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*user).hh.tbl).is_null() {
            _ha_oomed = 1 as libc::c_int;
        } else {
            memset(
                (*user).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*user).hh.tbl).tail = &mut (*user).hh;
            (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*user).hh.tbl)
                .hho = (&mut (*user).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(user as *mut libc::c_char) as libc::c_long;
            (*(*user).hh.tbl)
                .buckets = alt_malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*user).hh.tbl).buckets).is_null() {
                _ha_oomed = 1 as libc::c_int;
                alt_free((*user).hh.tbl as *mut libc::c_void);
            } else {
                memset(
                    (*(*user).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
                if _ha_oomed != 0 {
                    alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                    alt_free((*user).hh.tbl as *mut libc::c_void);
                }
            }
        }
        if _ha_oomed == 0 {
            users = user;
        }
    } else {
        (*user).hh.tbl = (*users).hh.tbl;
        (*user).hh.next = 0 as *mut libc::c_void;
        (*user)
            .hh
            .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
        (*(*users).hh.tbl).tail = &mut (*user).hh;
    }
    if _ha_oomed == 0 {
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*users).hh.tbl).num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
        (*(*users).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*user).hh.hh_next = (*_ha_head).hh_head;
        (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*user).hh;
        }
        (*_ha_head).hh_head = &mut (*user).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*user).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = alt_malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                _ha_oomed = 1 as libc::c_int;
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*user).hh.tbl)
                    .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                    >> ((*(*user).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*user).hh.tbl).num_items
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*user).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*user).hh.tbl).num_buckets {
                    _he_thh = (*((*(*user).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*user).hh.tbl).ideal_chain_maxlen {
                            (*(*user).hh.tbl)
                                .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*user).hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
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
                alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                (*(*user).hh.tbl)
                    .num_buckets = ((*(*user).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*user).hh.tbl)
                    .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*user).hh.tbl).log2_num_buckets;
                (*(*user).hh.tbl).buckets = _he_new_buckets;
                (*(*user).hh.tbl)
                    .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                    > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*user).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*user).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_hd_head).count = ((*_hd_head).count).wrapping_sub(1);
                (*_hd_head).count;
                if (*_hd_head).hh_head == &mut (*user).hh as *mut UT_hash_handle {
                    (*_hd_head).hh_head = (*user).hh.hh_next;
                }
                if !((*user).hh.hh_prev).is_null() {
                    (*(*user).hh.hh_prev).hh_next = (*user).hh.hh_next;
                }
                if !((*user).hh.hh_next).is_null() {
                    (*(*user).hh.hh_next).hh_prev = (*user).hh.hh_prev;
                }
            }
        }
        if _ha_oomed != 0 {
            let mut _hd_hh_item: *mut UT_hash_handle = &mut (*user).hh;
            let mut _hd_bkt: libc::c_uint = 0;
            _hd_bkt = (*_hd_hh_item).hashv
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let ref mut fresh0 = (*((*(*users).hh.tbl).buckets).offset(_hd_bkt as isize))
                .count;
            *fresh0 = (*fresh0).wrapping_add(1);
            *fresh0;
            (*_hd_hh_item).hh_next = 0 as *mut UT_hash_handle;
            (*_hd_hh_item).hh_prev = 0 as *mut UT_hash_handle;
            let mut _hd_hh_del: *mut UT_hash_handle = &mut (*user).hh;
            if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
                alt_free((*(*users).hh.tbl).buckets as *mut libc::c_void);
                alt_free((*users).hh.tbl as *mut libc::c_void);
                users = 0 as *mut example_user_t;
            } else {
                let mut _hd_bkt_0: libc::c_uint = 0;
                if _hd_hh_del == (*(*users).hh.tbl).tail {
                    (*(*users).hh.tbl)
                        .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle;
                }
                if !((*_hd_hh_del).prev).is_null() {
                    let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh1 = (*_hd_hh_del).next;
                } else {
                    users = (*_hd_hh_del).next as *mut example_user_t;
                }
                if !((*_hd_hh_del).next).is_null() {
                    let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh2 = (*_hd_hh_del).prev;
                }
                _hd_bkt_0 = (*_hd_hh_del).hashv
                    & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let mut _hd_head_0: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_hd_bkt_0 as isize) as *mut UT_hash_bucket;
                (*_hd_head_0).count = ((*_hd_head_0).count).wrapping_sub(1);
                (*_hd_head_0).count;
                if (*_hd_head_0).hh_head == _hd_hh_del {
                    (*_hd_head_0).hh_head = (*_hd_hh_del).hh_next;
                }
                if !((*_hd_hh_del).hh_prev).is_null() {
                    (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                }
                if !((*_hd_hh_del).hh_next).is_null() {
                    (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                }
                (*(*users).hh.tbl)
                    .num_items = ((*(*users).hh.tbl).num_items).wrapping_sub(1);
                (*(*users).hh.tbl).num_items;
            }
            (*user).hh.tbl = 0 as *mut UT_hash_table;
            (*user).mem_failed = 1 as libc::c_int;
        }
    } else {
        (*user).hh.tbl = 0 as *mut UT_hash_table;
        (*user).mem_failed = 1 as libc::c_int;
    }
    complain(2 as libc::c_int, users, user);
    malloc_cnt = 1 as libc::c_int;
    (*user).mem_failed = 0 as libc::c_int;
    free_cnt = 0 as libc::c_int;
    (*user).hh.tbl = 1 as libc::c_int as *mut UT_hash_table;
    let mut _ha_hashv_0: libc::c_uint = 0;
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
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
    let mut current_block_339: u64;
    match _hj_k_0 {
        11 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_339 = 6128560643697369861;
        }
        10 => {
            current_block_339 = 6128560643697369861;
        }
        9 => {
            current_block_339 = 9350638009254788289;
        }
        8 => {
            current_block_339 = 2412100623285860292;
        }
        7 => {
            current_block_339 = 182651403394654243;
        }
        6 => {
            current_block_339 = 16210616986695975448;
        }
        5 => {
            current_block_339 = 10734812651405308313;
        }
        4 => {
            current_block_339 = 9156700406342754034;
        }
        3 => {
            current_block_339 = 13769564399173561133;
        }
        2 => {
            current_block_339 = 174955779737117733;
        }
        1 => {
            current_block_339 = 5533732057557150228;
        }
        _ => {
            current_block_339 = 16749893710890171700;
        }
    }
    match current_block_339 {
        6128560643697369861 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_339 = 9350638009254788289;
        }
        _ => {}
    }
    match current_block_339 {
        9350638009254788289 => {
            _ha_hashv_0 = _ha_hashv_0
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_339 = 2412100623285860292;
        }
        _ => {}
    }
    match current_block_339 {
        2412100623285860292 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_339 = 182651403394654243;
        }
        _ => {}
    }
    match current_block_339 {
        182651403394654243 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_339 = 16210616986695975448;
        }
        _ => {}
    }
    match current_block_339 {
        16210616986695975448 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_339 = 10734812651405308313;
        }
        _ => {}
    }
    match current_block_339 {
        10734812651405308313 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_339 = 9156700406342754034;
        }
        _ => {}
    }
    match current_block_339 {
        9156700406342754034 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_339 = 13769564399173561133;
        }
        _ => {}
    }
    match current_block_339 {
        13769564399173561133 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_339 = 174955779737117733;
        }
        _ => {}
    }
    match current_block_339 {
        174955779737117733 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_339 = 5533732057557150228;
        }
        _ => {}
    }
    match current_block_339 {
        5533732057557150228 => {
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
    let mut _ha_oomed_0: libc::c_int = 0 as libc::c_int;
    (*user).hh.hashv = _ha_hashv_0;
    (*user).hh.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
    (*user)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if users.is_null() {
        (*user).hh.next = 0 as *mut libc::c_void;
        (*user).hh.prev = 0 as *mut libc::c_void;
        (*user)
            .hh
            .tbl = alt_malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*user).hh.tbl).is_null() {
            _ha_oomed_0 = 1 as libc::c_int;
        } else {
            memset(
                (*user).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*user).hh.tbl).tail = &mut (*user).hh;
            (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*user).hh.tbl)
                .hho = (&mut (*user).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(user as *mut libc::c_char) as libc::c_long;
            (*(*user).hh.tbl)
                .buckets = alt_malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*user).hh.tbl).buckets).is_null() {
                _ha_oomed_0 = 1 as libc::c_int;
                alt_free((*user).hh.tbl as *mut libc::c_void);
            } else {
                memset(
                    (*(*user).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
                if _ha_oomed_0 != 0 {
                    alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                    alt_free((*user).hh.tbl as *mut libc::c_void);
                }
            }
        }
        if _ha_oomed_0 == 0 {
            users = user;
        }
    } else {
        (*user).hh.tbl = (*users).hh.tbl;
        (*user).hh.next = 0 as *mut libc::c_void;
        (*user)
            .hh
            .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
        (*(*users).hh.tbl).tail = &mut (*user).hh;
    }
    if _ha_oomed_0 == 0 {
        let mut _ha_bkt_0: libc::c_uint = 0;
        (*(*users).hh.tbl).num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
        (*(*users).hh.tbl).num_items;
        _ha_bkt_0 = _ha_hashv_0
            & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl).buckets)
            .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
        (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
        (*_ha_head_0).count;
        (*user).hh.hh_next = (*_ha_head_0).hh_head;
        (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head_0).hh_head).is_null() {
            (*(*_ha_head_0).hh_head).hh_prev = &mut (*user).hh;
        }
        (*_ha_head_0).hh_head = &mut (*user).hh;
        if (*_ha_head_0).count
            >= ((*_ha_head_0).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*user).hh.tbl).noexpand == 0
        {
            let mut _he_bkt_0: libc::c_uint = 0;
            let mut _he_bkt_i_0: libc::c_uint = 0;
            let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets_0 = alt_malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets_0.is_null() {
                _ha_oomed_0 = 1 as libc::c_int;
            } else {
                memset(
                    _he_new_buckets_0 as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*user).hh.tbl)
                    .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                    >> ((*(*user).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*user).hh.tbl).num_items
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*user).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i_0 < (*(*user).hh.tbl).num_buckets {
                    _he_thh_0 = (*((*(*user).hh.tbl).buckets)
                        .offset(_he_bkt_i_0 as isize))
                        .hh_head;
                    while !_he_thh_0.is_null() {
                        _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                        _he_bkt_0 = (*_he_thh_0).hashv
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                        if (*_he_newbkt_0).count > (*(*user).hh.tbl).ideal_chain_maxlen {
                            (*(*user).hh.tbl)
                                .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*user).hh.tbl).nonideal_items;
                            if (*_he_newbkt_0).count
                                > ((*_he_newbkt_0).expand_mult)
                                    .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
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
                alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                (*(*user).hh.tbl)
                    .num_buckets = ((*(*user).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*user).hh.tbl)
                    .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*user).hh.tbl).log2_num_buckets;
                (*(*user).hh.tbl).buckets = _he_new_buckets_0;
                (*(*user).hh.tbl)
                    .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                    > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*user).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*user).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
            if _ha_oomed_0 != 0 {
                let mut _hd_head_1: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
                (*_hd_head_1).count = ((*_hd_head_1).count).wrapping_sub(1);
                (*_hd_head_1).count;
                if (*_hd_head_1).hh_head == &mut (*user).hh as *mut UT_hash_handle {
                    (*_hd_head_1).hh_head = (*user).hh.hh_next;
                }
                if !((*user).hh.hh_prev).is_null() {
                    (*(*user).hh.hh_prev).hh_next = (*user).hh.hh_next;
                }
                if !((*user).hh.hh_next).is_null() {
                    (*(*user).hh.hh_next).hh_prev = (*user).hh.hh_prev;
                }
            }
        }
        if _ha_oomed_0 != 0 {
            let mut _hd_hh_item_0: *mut UT_hash_handle = &mut (*user).hh;
            let mut _hd_bkt_1: libc::c_uint = 0;
            _hd_bkt_1 = (*_hd_hh_item_0).hashv
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let ref mut fresh3 = (*((*(*users).hh.tbl).buckets)
                .offset(_hd_bkt_1 as isize))
                .count;
            *fresh3 = (*fresh3).wrapping_add(1);
            *fresh3;
            (*_hd_hh_item_0).hh_next = 0 as *mut UT_hash_handle;
            (*_hd_hh_item_0).hh_prev = 0 as *mut UT_hash_handle;
            let mut _hd_hh_del_0: *mut UT_hash_handle = &mut (*user).hh;
            if ((*_hd_hh_del_0).prev).is_null() && ((*_hd_hh_del_0).next).is_null() {
                alt_free((*(*users).hh.tbl).buckets as *mut libc::c_void);
                alt_free((*users).hh.tbl as *mut libc::c_void);
                users = 0 as *mut example_user_t;
            } else {
                let mut _hd_bkt_2: libc::c_uint = 0;
                if _hd_hh_del_0 == (*(*users).hh.tbl).tail {
                    (*(*users).hh.tbl)
                        .tail = ((*_hd_hh_del_0).prev as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle;
                }
                if !((*_hd_hh_del_0).prev).is_null() {
                    let ref mut fresh4 = (*(((*_hd_hh_del_0).prev as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh4 = (*_hd_hh_del_0).next;
                } else {
                    users = (*_hd_hh_del_0).next as *mut example_user_t;
                }
                if !((*_hd_hh_del_0).next).is_null() {
                    let ref mut fresh5 = (*(((*_hd_hh_del_0).next as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh5 = (*_hd_hh_del_0).prev;
                }
                _hd_bkt_2 = (*_hd_hh_del_0).hashv
                    & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let mut _hd_head_2: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_hd_bkt_2 as isize) as *mut UT_hash_bucket;
                (*_hd_head_2).count = ((*_hd_head_2).count).wrapping_sub(1);
                (*_hd_head_2).count;
                if (*_hd_head_2).hh_head == _hd_hh_del_0 {
                    (*_hd_head_2).hh_head = (*_hd_hh_del_0).hh_next;
                }
                if !((*_hd_hh_del_0).hh_prev).is_null() {
                    (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
                }
                if !((*_hd_hh_del_0).hh_next).is_null() {
                    (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
                }
                (*(*users).hh.tbl)
                    .num_items = ((*(*users).hh.tbl).num_items).wrapping_sub(1);
                (*(*users).hh.tbl).num_items;
            }
            (*user).hh.tbl = 0 as *mut UT_hash_table;
            (*user).mem_failed = 1 as libc::c_int;
        }
    } else {
        (*user).hh.tbl = 0 as *mut UT_hash_table;
        (*user).mem_failed = 1 as libc::c_int;
    }
    complain(3 as libc::c_int, users, user);
    malloc_cnt = 4 as libc::c_int;
    (*user).mem_failed = 0 as libc::c_int;
    let mut _ha_hashv_1: libc::c_uint = 0;
    let mut _hj_i_1: libc::c_uint = 0;
    let mut _hj_j_1: libc::c_uint = 0;
    let mut _hj_k_1: libc::c_uint = 0;
    let mut _hj_key_1: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
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
    let mut current_block_620: u64;
    match _hj_k_1 {
        11 => {
            _ha_hashv_1 = _ha_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_620 = 8609698614768814962;
        }
        10 => {
            current_block_620 = 8609698614768814962;
        }
        9 => {
            current_block_620 = 1398023371624501419;
        }
        8 => {
            current_block_620 = 588349670397479588;
        }
        7 => {
            current_block_620 = 3064206605735001219;
        }
        6 => {
            current_block_620 = 9129788203149128072;
        }
        5 => {
            current_block_620 = 7453932734704531162;
        }
        4 => {
            current_block_620 = 5688418571186215685;
        }
        3 => {
            current_block_620 = 17875332839355784350;
        }
        2 => {
            current_block_620 = 17307707134264379049;
        }
        1 => {
            current_block_620 = 16373974457796776492;
        }
        _ => {
            current_block_620 = 6198430992841073810;
        }
    }
    match current_block_620 {
        8609698614768814962 => {
            _ha_hashv_1 = _ha_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_620 = 1398023371624501419;
        }
        _ => {}
    }
    match current_block_620 {
        1398023371624501419 => {
            _ha_hashv_1 = _ha_hashv_1
                .wrapping_add(
                    (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_620 = 588349670397479588;
        }
        _ => {}
    }
    match current_block_620 {
        588349670397479588 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_620 = 3064206605735001219;
        }
        _ => {}
    }
    match current_block_620 {
        3064206605735001219 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_620 = 9129788203149128072;
        }
        _ => {}
    }
    match current_block_620 {
        9129788203149128072 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_620 = 7453932734704531162;
        }
        _ => {}
    }
    match current_block_620 {
        7453932734704531162 => {
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    *_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_620 = 5688418571186215685;
        }
        _ => {}
    }
    match current_block_620 {
        5688418571186215685 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_620 = 17875332839355784350;
        }
        _ => {}
    }
    match current_block_620 {
        17875332839355784350 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_620 = 17307707134264379049;
        }
        _ => {}
    }
    match current_block_620 {
        17307707134264379049 => {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_620 = 16373974457796776492;
        }
        _ => {}
    }
    match current_block_620 {
        16373974457796776492 => {
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
    let mut _ha_oomed_1: libc::c_int = 0 as libc::c_int;
    (*user).hh.hashv = _ha_hashv_1;
    (*user).hh.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
    (*user)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if users.is_null() {
        (*user).hh.next = 0 as *mut libc::c_void;
        (*user).hh.prev = 0 as *mut libc::c_void;
        (*user)
            .hh
            .tbl = alt_malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*user).hh.tbl).is_null() {
            _ha_oomed_1 = 1 as libc::c_int;
        } else {
            memset(
                (*user).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*user).hh.tbl).tail = &mut (*user).hh;
            (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*user).hh.tbl)
                .hho = (&mut (*user).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(user as *mut libc::c_char) as libc::c_long;
            (*(*user).hh.tbl)
                .buckets = alt_malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*user).hh.tbl).buckets).is_null() {
                _ha_oomed_1 = 1 as libc::c_int;
                alt_free((*user).hh.tbl as *mut libc::c_void);
            } else {
                memset(
                    (*(*user).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
                if _ha_oomed_1 != 0 {
                    alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                    alt_free((*user).hh.tbl as *mut libc::c_void);
                }
            }
        }
        if _ha_oomed_1 == 0 {
            users = user;
        }
    } else {
        (*user).hh.tbl = (*users).hh.tbl;
        (*user).hh.next = 0 as *mut libc::c_void;
        (*user)
            .hh
            .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
        (*(*users).hh.tbl).tail = &mut (*user).hh;
    }
    if _ha_oomed_1 == 0 {
        let mut _ha_bkt_1: libc::c_uint = 0;
        (*(*users).hh.tbl).num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
        (*(*users).hh.tbl).num_items;
        _ha_bkt_1 = _ha_hashv_1
            & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head_1: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl).buckets)
            .offset(_ha_bkt_1 as isize) as *mut UT_hash_bucket;
        (*_ha_head_1).count = ((*_ha_head_1).count).wrapping_add(1);
        (*_ha_head_1).count;
        (*user).hh.hh_next = (*_ha_head_1).hh_head;
        (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head_1).hh_head).is_null() {
            (*(*_ha_head_1).hh_head).hh_prev = &mut (*user).hh;
        }
        (*_ha_head_1).hh_head = &mut (*user).hh;
        if (*_ha_head_1).count
            >= ((*_ha_head_1).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*user).hh.tbl).noexpand == 0
        {
            let mut _he_bkt_1: libc::c_uint = 0;
            let mut _he_bkt_i_1: libc::c_uint = 0;
            let mut _he_thh_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt_1: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt_1: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets_1 = alt_malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets_1.is_null() {
                _ha_oomed_1 = 1 as libc::c_int;
            } else {
                memset(
                    _he_new_buckets_1 as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*user).hh.tbl)
                    .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                    >> ((*(*user).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*user).hh.tbl).num_items
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*user).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i_1 = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i_1 < (*(*user).hh.tbl).num_buckets {
                    _he_thh_1 = (*((*(*user).hh.tbl).buckets)
                        .offset(_he_bkt_i_1 as isize))
                        .hh_head;
                    while !_he_thh_1.is_null() {
                        _he_hh_nxt_1 = (*_he_thh_1).hh_next;
                        _he_bkt_1 = (*_he_thh_1).hashv
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt_1 = &mut *_he_new_buckets_1.offset(_he_bkt_1 as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt_1).count = ((*_he_newbkt_1).count).wrapping_add(1);
                        if (*_he_newbkt_1).count > (*(*user).hh.tbl).ideal_chain_maxlen {
                            (*(*user).hh.tbl)
                                .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*user).hh.tbl).nonideal_items;
                            if (*_he_newbkt_1).count
                                > ((*_he_newbkt_1).expand_mult)
                                    .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
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
                alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                (*(*user).hh.tbl)
                    .num_buckets = ((*(*user).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*user).hh.tbl)
                    .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*user).hh.tbl).log2_num_buckets;
                (*(*user).hh.tbl).buckets = _he_new_buckets_1;
                (*(*user).hh.tbl)
                    .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                    > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*user).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*user).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
            if _ha_oomed_1 != 0 {
                let mut _hd_head_3: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_ha_bkt_1 as isize) as *mut UT_hash_bucket;
                (*_hd_head_3).count = ((*_hd_head_3).count).wrapping_sub(1);
                (*_hd_head_3).count;
                if (*_hd_head_3).hh_head == &mut (*user).hh as *mut UT_hash_handle {
                    (*_hd_head_3).hh_head = (*user).hh.hh_next;
                }
                if !((*user).hh.hh_prev).is_null() {
                    (*(*user).hh.hh_prev).hh_next = (*user).hh.hh_next;
                }
                if !((*user).hh.hh_next).is_null() {
                    (*(*user).hh.hh_next).hh_prev = (*user).hh.hh_prev;
                }
            }
        }
        if _ha_oomed_1 != 0 {
            let mut _hd_hh_item_1: *mut UT_hash_handle = &mut (*user).hh;
            let mut _hd_bkt_3: libc::c_uint = 0;
            _hd_bkt_3 = (*_hd_hh_item_1).hashv
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let ref mut fresh6 = (*((*(*users).hh.tbl).buckets)
                .offset(_hd_bkt_3 as isize))
                .count;
            *fresh6 = (*fresh6).wrapping_add(1);
            *fresh6;
            (*_hd_hh_item_1).hh_next = 0 as *mut UT_hash_handle;
            (*_hd_hh_item_1).hh_prev = 0 as *mut UT_hash_handle;
            let mut _hd_hh_del_1: *mut UT_hash_handle = &mut (*user).hh;
            if ((*_hd_hh_del_1).prev).is_null() && ((*_hd_hh_del_1).next).is_null() {
                alt_free((*(*users).hh.tbl).buckets as *mut libc::c_void);
                alt_free((*users).hh.tbl as *mut libc::c_void);
                users = 0 as *mut example_user_t;
            } else {
                let mut _hd_bkt_4: libc::c_uint = 0;
                if _hd_hh_del_1 == (*(*users).hh.tbl).tail {
                    (*(*users).hh.tbl)
                        .tail = ((*_hd_hh_del_1).prev as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle;
                }
                if !((*_hd_hh_del_1).prev).is_null() {
                    let ref mut fresh7 = (*(((*_hd_hh_del_1).prev as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh7 = (*_hd_hh_del_1).next;
                } else {
                    users = (*_hd_hh_del_1).next as *mut example_user_t;
                }
                if !((*_hd_hh_del_1).next).is_null() {
                    let ref mut fresh8 = (*(((*_hd_hh_del_1).next as *mut libc::c_char)
                        .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh8 = (*_hd_hh_del_1).prev;
                }
                _hd_bkt_4 = (*_hd_hh_del_1).hashv
                    & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let mut _hd_head_4: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_hd_bkt_4 as isize) as *mut UT_hash_bucket;
                (*_hd_head_4).count = ((*_hd_head_4).count).wrapping_sub(1);
                (*_hd_head_4).count;
                if (*_hd_head_4).hh_head == _hd_hh_del_1 {
                    (*_hd_head_4).hh_head = (*_hd_hh_del_1).hh_next;
                }
                if !((*_hd_hh_del_1).hh_prev).is_null() {
                    (*(*_hd_hh_del_1).hh_prev).hh_next = (*_hd_hh_del_1).hh_next;
                }
                if !((*_hd_hh_del_1).hh_next).is_null() {
                    (*(*_hd_hh_del_1).hh_next).hh_prev = (*_hd_hh_del_1).hh_prev;
                }
                (*(*users).hh.tbl)
                    .num_items = ((*(*users).hh.tbl).num_items).wrapping_sub(1);
                (*(*users).hh.tbl).num_items;
            }
            (*user).hh.tbl = 0 as *mut UT_hash_table;
            (*user).mem_failed = 1 as libc::c_int;
        }
    } else {
        (*user).hh.tbl = 0 as *mut UT_hash_table;
        (*user).mem_failed = 1 as libc::c_int;
    }
    if (*user).mem_failed != 0 {
        printf(
            b"mem_failed must be 0, not %d\n\0" as *const u8 as *const libc::c_char,
            (*user).mem_failed,
        );
    }
    test = 0 as *mut example_user_t;
    if !users.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_2: libc::c_uint = 0;
        let mut _hj_j_2: libc::c_uint = 0;
        let mut _hj_k_2: libc::c_uint = 0;
        let mut _hj_key_2: *const libc::c_uchar = &mut id as *mut libc::c_int
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
        let mut current_block_902: u64;
        match _hj_k_2 {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_2.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_902 = 5033478145031579882;
            }
            10 => {
                current_block_902 = 5033478145031579882;
            }
            9 => {
                current_block_902 = 13040918234989416905;
            }
            8 => {
                current_block_902 = 13043828181439542487;
            }
            7 => {
                current_block_902 = 14467630112290157245;
            }
            6 => {
                current_block_902 = 4147470538878086186;
            }
            5 => {
                current_block_902 = 3493394124312272479;
            }
            4 => {
                current_block_902 = 3296024903401526033;
            }
            3 => {
                current_block_902 = 4822005742776393872;
            }
            2 => {
                current_block_902 = 4360140461302388439;
            }
            1 => {
                current_block_902 = 18131238365012881386;
            }
            _ => {
                current_block_902 = 9267933025378065668;
            }
        }
        match current_block_902 {
            5033478145031579882 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_2.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_902 = 13040918234989416905;
            }
            _ => {}
        }
        match current_block_902 {
            13040918234989416905 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_2.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_902 = 13043828181439542487;
            }
            _ => {}
        }
        match current_block_902 {
            13043828181439542487 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_902 = 14467630112290157245;
            }
            _ => {}
        }
        match current_block_902 {
            14467630112290157245 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_902 = 4147470538878086186;
            }
            _ => {}
        }
        match current_block_902 {
            4147470538878086186 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_902 = 3493394124312272479;
            }
            _ => {}
        }
        match current_block_902 {
            3493394124312272479 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        *_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_902 = 3296024903401526033;
            }
            _ => {}
        }
        match current_block_902 {
            3296024903401526033 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_902 = 4822005742776393872;
            }
            _ => {}
        }
        match current_block_902 {
            4822005742776393872 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_902 = 4360140461302388439;
            }
            _ => {}
        }
        match current_block_902 {
            4360140461302388439 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_902 = 18131238365012881386;
            }
            _ => {}
        }
        match current_block_902 {
            18131238365012881386 => {
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
        test = 0 as *mut example_user_t;
        if !users.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*users).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    test = ((*((*(*users).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut example_user_t;
                } else {
                    test = 0 as *mut example_user_t;
                }
                while !test.is_null() {
                    if (*test).hh.hashv == _hf_hashv
                        && (*test).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*test).hh.key,
                            &mut id as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*test).hh.hh_next).is_null() {
                        test = ((*test).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        test = 0 as *mut example_user_t;
                    }
                }
            }
        }
    }
    if test.is_null() {
        printf(b"test user ID %d not found\n\0" as *const u8 as *const libc::c_char, id);
    }
    if (if !users.is_null() { (*(*users).hh.tbl).num_items } else { 0 as libc::c_uint })
        != 1 as libc::c_int as libc::c_uint
    {
        printf(
            b"Got HASH_COUNT(users)=%d, should be 1\n\0" as *const u8
                as *const libc::c_char,
            if !users.is_null() {
                (*(*users).hh.tbl).num_items
            } else {
                0 as libc::c_uint
            },
        );
    }
    malloc_failed = 0 as libc::c_int;
    free_cnt = 0 as libc::c_int;
    malloc_cnt = 1 as libc::c_int;
    id = 1 as libc::c_int;
    loop {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        (*user).id = id;
        if id >= 1000 as libc::c_int {
            puts(
                b"too many allocs before memory request\0" as *const u8
                    as *const libc::c_char,
            );
            break;
        } else {
            (*user).hh.tbl = 1 as libc::c_int as *mut UT_hash_table;
            let mut _ha_hashv_2: libc::c_uint = 0;
            let mut _hj_i_3: libc::c_uint = 0;
            let mut _hj_j_3: libc::c_uint = 0;
            let mut _hj_k_3: libc::c_uint = 0;
            let mut _hj_key_3: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
                as *const libc::c_uchar;
            _ha_hashv_2 = 0xfeedbeef as libc::c_uint;
            _hj_j_3 = 0x9e3779b9 as libc::c_uint;
            _hj_i_3 = _hj_j_3;
            _hj_k_3 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
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
                _ha_hashv_2 = _ha_hashv_2
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
                _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_2);
                _hj_i_3 ^= _ha_hashv_2 >> 13 as libc::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_2);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
                _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_i_3);
                _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_j_3);
                _ha_hashv_2 ^= _hj_j_3 >> 13 as libc::c_int;
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_2);
                _hj_i_3 ^= _ha_hashv_2 >> 12 as libc::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_2);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
                _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_i_3);
                _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_j_3);
                _ha_hashv_2 ^= _hj_j_3 >> 5 as libc::c_int;
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_2);
                _hj_i_3 ^= _ha_hashv_2 >> 3 as libc::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_2);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
                _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_i_3);
                _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_j_3);
                _ha_hashv_2 ^= _hj_j_3 >> 15 as libc::c_int;
                _hj_key_3 = _hj_key_3.offset(12 as libc::c_int as isize);
                _hj_k_3 = _hj_k_3.wrapping_sub(12 as libc::c_uint);
            }
            _ha_hashv_2 = _ha_hashv_2
                .wrapping_add(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
                );
            let mut current_block_1033: u64;
            match _hj_k_3 {
                11 => {
                    _ha_hashv_2 = _ha_hashv_2
                        .wrapping_add(
                            (*_hj_key_3.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_1033 = 12141488940674688119;
                }
                10 => {
                    current_block_1033 = 12141488940674688119;
                }
                9 => {
                    current_block_1033 = 11650662791505227050;
                }
                8 => {
                    current_block_1033 = 3864413838222808248;
                }
                7 => {
                    current_block_1033 = 12686155273142351515;
                }
                6 => {
                    current_block_1033 = 701802178761638199;
                }
                5 => {
                    current_block_1033 = 9112997927631832806;
                }
                4 => {
                    current_block_1033 = 14680065516422604636;
                }
                3 => {
                    current_block_1033 = 16743029750287244012;
                }
                2 => {
                    current_block_1033 = 11503143143197623604;
                }
                1 => {
                    current_block_1033 = 16223918737641857188;
                }
                _ => {
                    current_block_1033 = 6717489231776426038;
                }
            }
            match current_block_1033 {
                12141488940674688119 => {
                    _ha_hashv_2 = _ha_hashv_2
                        .wrapping_add(
                            (*_hj_key_3.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_1033 = 11650662791505227050;
                }
                _ => {}
            }
            match current_block_1033 {
                11650662791505227050 => {
                    _ha_hashv_2 = _ha_hashv_2
                        .wrapping_add(
                            (*_hj_key_3.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_1033 = 3864413838222808248;
                }
                _ => {}
            }
            match current_block_1033 {
                3864413838222808248 => {
                    _hj_j_3 = _hj_j_3
                        .wrapping_add(
                            (*_hj_key_3.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_1033 = 12686155273142351515;
                }
                _ => {}
            }
            match current_block_1033 {
                12686155273142351515 => {
                    _hj_j_3 = _hj_j_3
                        .wrapping_add(
                            (*_hj_key_3.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_1033 = 701802178761638199;
                }
                _ => {}
            }
            match current_block_1033 {
                701802178761638199 => {
                    _hj_j_3 = _hj_j_3
                        .wrapping_add(
                            (*_hj_key_3.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_1033 = 9112997927631832806;
                }
                _ => {}
            }
            match current_block_1033 {
                9112997927631832806 => {
                    _hj_j_3 = _hj_j_3
                        .wrapping_add(
                            *_hj_key_3.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_1033 = 14680065516422604636;
                }
                _ => {}
            }
            match current_block_1033 {
                14680065516422604636 => {
                    _hj_i_3 = _hj_i_3
                        .wrapping_add(
                            (*_hj_key_3.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_1033 = 16743029750287244012;
                }
                _ => {}
            }
            match current_block_1033 {
                16743029750287244012 => {
                    _hj_i_3 = _hj_i_3
                        .wrapping_add(
                            (*_hj_key_3.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_1033 = 11503143143197623604;
                }
                _ => {}
            }
            match current_block_1033 {
                11503143143197623604 => {
                    _hj_i_3 = _hj_i_3
                        .wrapping_add(
                            (*_hj_key_3.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_1033 = 16223918737641857188;
                }
                _ => {}
            }
            match current_block_1033 {
                16223918737641857188 => {
                    _hj_i_3 = _hj_i_3
                        .wrapping_add(
                            *_hj_key_3.offset(0 as libc::c_int as isize) as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_2);
            _hj_i_3 ^= _ha_hashv_2 >> 13 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_2);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
            _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_i_3);
            _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_j_3);
            _ha_hashv_2 ^= _hj_j_3 >> 13 as libc::c_int;
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_2);
            _hj_i_3 ^= _ha_hashv_2 >> 12 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_2);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
            _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_i_3);
            _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_j_3);
            _ha_hashv_2 ^= _hj_j_3 >> 5 as libc::c_int;
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_2);
            _hj_i_3 ^= _ha_hashv_2 >> 3 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_2);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
            _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_i_3);
            _ha_hashv_2 = _ha_hashv_2.wrapping_sub(_hj_j_3);
            _ha_hashv_2 ^= _hj_j_3 >> 15 as libc::c_int;
            let mut _ha_oomed_2: libc::c_int = 0 as libc::c_int;
            (*user).hh.hashv = _ha_hashv_2;
            (*user).hh.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
            (*user)
                .hh
                .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            if users.is_null() {
                (*user).hh.next = 0 as *mut libc::c_void;
                (*user).hh.prev = 0 as *mut libc::c_void;
                (*user)
                    .hh
                    .tbl = alt_malloc(
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                ) as *mut UT_hash_table;
                if ((*user).hh.tbl).is_null() {
                    _ha_oomed_2 = 1 as libc::c_int;
                } else {
                    memset(
                        (*user).hh.tbl as *mut libc::c_void,
                        '\0' as i32,
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    );
                    (*(*user).hh.tbl).tail = &mut (*user).hh;
                    (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
                    (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                    (*(*user).hh.tbl)
                        .hho = (&mut (*user).hh as *mut UT_hash_handle
                        as *mut libc::c_char)
                        .offset_from(user as *mut libc::c_char) as libc::c_long;
                    (*(*user).hh.tbl)
                        .buckets = alt_malloc(
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    ) as *mut UT_hash_bucket;
                    (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                    if ((*(*user).hh.tbl).buckets).is_null() {
                        _ha_oomed_2 = 1 as libc::c_int;
                        alt_free((*user).hh.tbl as *mut libc::c_void);
                    } else {
                        memset(
                            (*(*user).hh.tbl).buckets as *mut libc::c_void,
                            '\0' as i32,
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        );
                        if _ha_oomed_2 != 0 {
                            alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                            alt_free((*user).hh.tbl as *mut libc::c_void);
                        }
                    }
                }
                if _ha_oomed_2 == 0 {
                    users = user;
                }
            } else {
                (*user).hh.tbl = (*users).hh.tbl;
                (*user).hh.next = 0 as *mut libc::c_void;
                (*user)
                    .hh
                    .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
                (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
                (*(*users).hh.tbl).tail = &mut (*user).hh;
            }
            if _ha_oomed_2 == 0 {
                let mut _ha_bkt_2: libc::c_uint = 0;
                (*(*users).hh.tbl)
                    .num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
                (*(*users).hh.tbl).num_items;
                _ha_bkt_2 = _ha_hashv_2
                    & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let mut _ha_head_2: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
                    .offset(_ha_bkt_2 as isize) as *mut UT_hash_bucket;
                (*_ha_head_2).count = ((*_ha_head_2).count).wrapping_add(1);
                (*_ha_head_2).count;
                (*user).hh.hh_next = (*_ha_head_2).hh_head;
                (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
                if !((*_ha_head_2).hh_head).is_null() {
                    (*(*_ha_head_2).hh_head).hh_prev = &mut (*user).hh;
                }
                (*_ha_head_2).hh_head = &mut (*user).hh;
                if (*_ha_head_2).count
                    >= ((*_ha_head_2).expand_mult)
                        .wrapping_add(1 as libc::c_uint)
                        .wrapping_mul(10 as libc::c_uint)
                    && (*(*user).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt_2: libc::c_uint = 0;
                    let mut _he_bkt_i_2: libc::c_uint = 0;
                    let mut _he_thh_2: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_hh_nxt_2: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_new_buckets_2: *mut UT_hash_bucket = 0
                        as *mut UT_hash_bucket;
                    let mut _he_newbkt_2: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                    _he_new_buckets_2 = alt_malloc(
                        (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                            .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets_2.is_null() {
                        _ha_oomed_2 = 1 as libc::c_int;
                    } else {
                        memset(
                            _he_new_buckets_2 as *mut libc::c_void,
                            '\0' as i32,
                            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                .wrapping_mul(
                                    (*(*user).hh.tbl).num_buckets as libc::c_ulong,
                                )
                                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                        );
                        (*(*user).hh.tbl)
                            .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                            >> ((*(*user).hh.tbl).log2_num_buckets)
                                .wrapping_add(1 as libc::c_uint))
                            .wrapping_add(
                                (if (*(*user).hh.tbl).num_items
                                    & ((*(*user).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                                {
                                    1 as libc::c_uint
                                } else {
                                    0 as libc::c_uint
                                }),
                            );
                        (*(*user).hh.tbl)
                            .nonideal_items = 0 as libc::c_int as libc::c_uint;
                        _he_bkt_i_2 = 0 as libc::c_int as libc::c_uint;
                        while _he_bkt_i_2 < (*(*user).hh.tbl).num_buckets {
                            _he_thh_2 = (*((*(*user).hh.tbl).buckets)
                                .offset(_he_bkt_i_2 as isize))
                                .hh_head;
                            while !_he_thh_2.is_null() {
                                _he_hh_nxt_2 = (*_he_thh_2).hh_next;
                                _he_bkt_2 = (*_he_thh_2).hashv
                                    & ((*(*user).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint);
                                _he_newbkt_2 = &mut *_he_new_buckets_2
                                    .offset(_he_bkt_2 as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt_2)
                                    .count = ((*_he_newbkt_2).count).wrapping_add(1);
                                if (*_he_newbkt_2).count
                                    > (*(*user).hh.tbl).ideal_chain_maxlen
                                {
                                    (*(*user).hh.tbl)
                                        .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                        .wrapping_add(1);
                                    (*(*user).hh.tbl).nonideal_items;
                                    if (*_he_newbkt_2).count
                                        > ((*_he_newbkt_2).expand_mult)
                                            .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
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
                        alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                        (*(*user).hh.tbl)
                            .num_buckets = ((*(*user).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint);
                        (*(*user).hh.tbl)
                            .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                            .wrapping_add(1);
                        (*(*user).hh.tbl).log2_num_buckets;
                        (*(*user).hh.tbl).buckets = _he_new_buckets_2;
                        (*(*user).hh.tbl)
                            .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                            > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                        {
                            ((*(*user).hh.tbl).ineff_expands)
                                .wrapping_add(1 as libc::c_uint)
                        } else {
                            0 as libc::c_uint
                        };
                        if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                            (*(*user).hh.tbl)
                                .noexpand = 1 as libc::c_int as libc::c_uint;
                        }
                    }
                    if _ha_oomed_2 != 0 {
                        let mut _hd_head_5: *mut UT_hash_bucket = &mut *((*(*users)
                            .hh
                            .tbl)
                            .buckets)
                            .offset(_ha_bkt_2 as isize) as *mut UT_hash_bucket;
                        (*_hd_head_5).count = ((*_hd_head_5).count).wrapping_sub(1);
                        (*_hd_head_5).count;
                        if (*_hd_head_5).hh_head
                            == &mut (*user).hh as *mut UT_hash_handle
                        {
                            (*_hd_head_5).hh_head = (*user).hh.hh_next;
                        }
                        if !((*user).hh.hh_prev).is_null() {
                            (*(*user).hh.hh_prev).hh_next = (*user).hh.hh_next;
                        }
                        if !((*user).hh.hh_next).is_null() {
                            (*(*user).hh.hh_next).hh_prev = (*user).hh.hh_prev;
                        }
                    }
                }
                if _ha_oomed_2 != 0 {
                    let mut _hd_hh_item_2: *mut UT_hash_handle = &mut (*user).hh;
                    let mut _hd_bkt_5: libc::c_uint = 0;
                    _hd_bkt_5 = (*_hd_hh_item_2).hashv
                        & ((*(*users).hh.tbl).num_buckets)
                            .wrapping_sub(1 as libc::c_uint);
                    let ref mut fresh9 = (*((*(*users).hh.tbl).buckets)
                        .offset(_hd_bkt_5 as isize))
                        .count;
                    *fresh9 = (*fresh9).wrapping_add(1);
                    *fresh9;
                    (*_hd_hh_item_2).hh_next = 0 as *mut UT_hash_handle;
                    (*_hd_hh_item_2).hh_prev = 0 as *mut UT_hash_handle;
                    let mut _hd_hh_del_2: *mut UT_hash_handle = &mut (*user).hh;
                    if ((*_hd_hh_del_2).prev).is_null()
                        && ((*_hd_hh_del_2).next).is_null()
                    {
                        alt_free((*(*users).hh.tbl).buckets as *mut libc::c_void);
                        alt_free((*users).hh.tbl as *mut libc::c_void);
                        users = 0 as *mut example_user_t;
                    } else {
                        let mut _hd_bkt_6: libc::c_uint = 0;
                        if _hd_hh_del_2 == (*(*users).hh.tbl).tail {
                            (*(*users).hh.tbl)
                                .tail = ((*_hd_hh_del_2).prev as *mut libc::c_char)
                                .offset((*(*users).hh.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle;
                        }
                        if !((*_hd_hh_del_2).prev).is_null() {
                            let ref mut fresh10 = (*(((*_hd_hh_del_2).prev
                                as *mut libc::c_char)
                                .offset((*(*users).hh.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle))
                                .next;
                            *fresh10 = (*_hd_hh_del_2).next;
                        } else {
                            users = (*_hd_hh_del_2).next as *mut example_user_t;
                        }
                        if !((*_hd_hh_del_2).next).is_null() {
                            let ref mut fresh11 = (*(((*_hd_hh_del_2).next
                                as *mut libc::c_char)
                                .offset((*(*users).hh.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle))
                                .prev;
                            *fresh11 = (*_hd_hh_del_2).prev;
                        }
                        _hd_bkt_6 = (*_hd_hh_del_2).hashv
                            & ((*(*users).hh.tbl).num_buckets)
                                .wrapping_sub(1 as libc::c_uint);
                        let mut _hd_head_6: *mut UT_hash_bucket = &mut *((*(*users)
                            .hh
                            .tbl)
                            .buckets)
                            .offset(_hd_bkt_6 as isize) as *mut UT_hash_bucket;
                        (*_hd_head_6).count = ((*_hd_head_6).count).wrapping_sub(1);
                        (*_hd_head_6).count;
                        if (*_hd_head_6).hh_head == _hd_hh_del_2 {
                            (*_hd_head_6).hh_head = (*_hd_hh_del_2).hh_next;
                        }
                        if !((*_hd_hh_del_2).hh_prev).is_null() {
                            (*(*_hd_hh_del_2).hh_prev).hh_next = (*_hd_hh_del_2).hh_next;
                        }
                        if !((*_hd_hh_del_2).hh_next).is_null() {
                            (*(*_hd_hh_del_2).hh_next).hh_prev = (*_hd_hh_del_2).hh_prev;
                        }
                        (*(*users).hh.tbl)
                            .num_items = ((*(*users).hh.tbl).num_items).wrapping_sub(1);
                        (*(*users).hh.tbl).num_items;
                    }
                    (*user).hh.tbl = 0 as *mut UT_hash_table;
                    (*user).mem_failed = 1 as libc::c_int;
                }
            } else {
                (*user).hh.tbl = 0 as *mut UT_hash_table;
                (*user).mem_failed = 1 as libc::c_int;
            }
            if malloc_failed != 0 {
                if id < 10 as libc::c_int {
                    puts(
                        b"there is no way your bucket size is <= 10\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !((*user).hh.tbl).is_null() {
                    puts(
                        b"user->hh.tbl should be NULL after failure\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if (*user).mem_failed != 1 as libc::c_int {
                    printf(
                        b"mem_failed should be 1 after failure, not %d\n\0" as *const u8
                            as *const libc::c_char,
                        (*user).mem_failed,
                    );
                }
                if free_cnt != 0 as libc::c_int {
                    printf(
                        b"Expected 0 frees, had %d\n\0" as *const u8
                            as *const libc::c_char,
                        free_cnt,
                    );
                }
                i = 0 as libc::c_int;
                while i < id {
                    test = 0 as *mut example_user_t;
                    if !users.is_null() {
                        let mut _hf_hashv_0: libc::c_uint = 0;
                        let mut _hj_i_4: libc::c_uint = 0;
                        let mut _hj_j_4: libc::c_uint = 0;
                        let mut _hj_k_4: libc::c_uint = 0;
                        let mut _hj_key_4: *const libc::c_uchar = &mut i
                            as *mut libc::c_int as *const libc::c_uchar;
                        _hf_hashv_0 = 0xfeedbeef as libc::c_uint;
                        _hj_j_4 = 0x9e3779b9 as libc::c_uint;
                        _hj_i_4 = _hj_j_4;
                        _hj_k_4 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_uint;
                        while _hj_k_4 >= 12 as libc::c_uint {
                            _hj_i_4 = _hj_i_4
                                .wrapping_add(
                                    (*_hj_key_4.offset(0 as libc::c_int as isize)
                                        as libc::c_uint)
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
                                    (*_hj_key_4.offset(4 as libc::c_int as isize)
                                        as libc::c_uint)
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
                            _hf_hashv_0 = _hf_hashv_0
                                .wrapping_add(
                                    (*_hj_key_4.offset(8 as libc::c_int as isize)
                                        as libc::c_uint)
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
                            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_0);
                            _hj_i_4 ^= _hf_hashv_0 >> 13 as libc::c_int;
                            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_0);
                            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                            _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_4);
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_4);
                            _hf_hashv_0 ^= _hj_j_4 >> 13 as libc::c_int;
                            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_0);
                            _hj_i_4 ^= _hf_hashv_0 >> 12 as libc::c_int;
                            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_0);
                            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                            _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_4);
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_4);
                            _hf_hashv_0 ^= _hj_j_4 >> 5 as libc::c_int;
                            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_0);
                            _hj_i_4 ^= _hf_hashv_0 >> 3 as libc::c_int;
                            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_0);
                            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                            _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_4);
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_4);
                            _hf_hashv_0 ^= _hj_j_4 >> 15 as libc::c_int;
                            _hj_key_4 = _hj_key_4.offset(12 as libc::c_int as isize);
                            _hj_k_4 = _hj_k_4.wrapping_sub(12 as libc::c_uint);
                        }
                        _hf_hashv_0 = _hf_hashv_0
                            .wrapping_add(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_uint,
                            );
                        let mut current_block_1325: u64;
                        match _hj_k_4 {
                            11 => {
                                _hf_hashv_0 = _hf_hashv_0
                                    .wrapping_add(
                                        (*_hj_key_4.offset(10 as libc::c_int as isize)
                                            as libc::c_uint) << 24 as libc::c_int,
                                    );
                                current_block_1325 = 11815220093294077134;
                            }
                            10 => {
                                current_block_1325 = 11815220093294077134;
                            }
                            9 => {
                                current_block_1325 = 4301956666638608522;
                            }
                            8 => {
                                current_block_1325 = 2881585595495500279;
                            }
                            7 => {
                                current_block_1325 = 3937414690860382654;
                            }
                            6 => {
                                current_block_1325 = 15133783685141389824;
                            }
                            5 => {
                                current_block_1325 = 5912668457996645581;
                            }
                            4 => {
                                current_block_1325 = 1437842837371977635;
                            }
                            3 => {
                                current_block_1325 = 9415257114864717776;
                            }
                            2 => {
                                current_block_1325 = 12342418202971607573;
                            }
                            1 => {
                                current_block_1325 = 1979616052774745799;
                            }
                            _ => {
                                current_block_1325 = 17068772966245984851;
                            }
                        }
                        match current_block_1325 {
                            11815220093294077134 => {
                                _hf_hashv_0 = _hf_hashv_0
                                    .wrapping_add(
                                        (*_hj_key_4.offset(9 as libc::c_int as isize)
                                            as libc::c_uint) << 16 as libc::c_int,
                                    );
                                current_block_1325 = 4301956666638608522;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            4301956666638608522 => {
                                _hf_hashv_0 = _hf_hashv_0
                                    .wrapping_add(
                                        (*_hj_key_4.offset(8 as libc::c_int as isize)
                                            as libc::c_uint) << 8 as libc::c_int,
                                    );
                                current_block_1325 = 2881585595495500279;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            2881585595495500279 => {
                                _hj_j_4 = _hj_j_4
                                    .wrapping_add(
                                        (*_hj_key_4.offset(7 as libc::c_int as isize)
                                            as libc::c_uint) << 24 as libc::c_int,
                                    );
                                current_block_1325 = 3937414690860382654;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            3937414690860382654 => {
                                _hj_j_4 = _hj_j_4
                                    .wrapping_add(
                                        (*_hj_key_4.offset(6 as libc::c_int as isize)
                                            as libc::c_uint) << 16 as libc::c_int,
                                    );
                                current_block_1325 = 15133783685141389824;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            15133783685141389824 => {
                                _hj_j_4 = _hj_j_4
                                    .wrapping_add(
                                        (*_hj_key_4.offset(5 as libc::c_int as isize)
                                            as libc::c_uint) << 8 as libc::c_int,
                                    );
                                current_block_1325 = 5912668457996645581;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            5912668457996645581 => {
                                _hj_j_4 = _hj_j_4
                                    .wrapping_add(
                                        *_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint,
                                    );
                                current_block_1325 = 1437842837371977635;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            1437842837371977635 => {
                                _hj_i_4 = _hj_i_4
                                    .wrapping_add(
                                        (*_hj_key_4.offset(3 as libc::c_int as isize)
                                            as libc::c_uint) << 24 as libc::c_int,
                                    );
                                current_block_1325 = 9415257114864717776;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            9415257114864717776 => {
                                _hj_i_4 = _hj_i_4
                                    .wrapping_add(
                                        (*_hj_key_4.offset(2 as libc::c_int as isize)
                                            as libc::c_uint) << 16 as libc::c_int,
                                    );
                                current_block_1325 = 12342418202971607573;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            12342418202971607573 => {
                                _hj_i_4 = _hj_i_4
                                    .wrapping_add(
                                        (*_hj_key_4.offset(1 as libc::c_int as isize)
                                            as libc::c_uint) << 8 as libc::c_int,
                                    );
                                current_block_1325 = 1979616052774745799;
                            }
                            _ => {}
                        }
                        match current_block_1325 {
                            1979616052774745799 => {
                                _hj_i_4 = _hj_i_4
                                    .wrapping_add(
                                        *_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint,
                                    );
                            }
                            _ => {}
                        }
                        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_0);
                        _hj_i_4 ^= _hf_hashv_0 >> 13 as libc::c_int;
                        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_0);
                        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                        _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_4);
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_4);
                        _hf_hashv_0 ^= _hj_j_4 >> 13 as libc::c_int;
                        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_0);
                        _hj_i_4 ^= _hf_hashv_0 >> 12 as libc::c_int;
                        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_0);
                        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                        _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_4);
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_4);
                        _hf_hashv_0 ^= _hj_j_4 >> 5 as libc::c_int;
                        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_0);
                        _hj_i_4 ^= _hf_hashv_0 >> 3 as libc::c_int;
                        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_0);
                        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                        _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_4);
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_4);
                        _hf_hashv_0 ^= _hj_j_4 >> 15 as libc::c_int;
                        test = 0 as *mut example_user_t;
                        if !users.is_null() {
                            let mut _hf_bkt_0: libc::c_uint = 0;
                            _hf_bkt_0 = _hf_hashv_0
                                & ((*(*users).hh.tbl).num_buckets)
                                    .wrapping_sub(1 as libc::c_uint);
                            if 1 as libc::c_int != 0 as libc::c_int {
                                if !((*((*(*users).hh.tbl).buckets)
                                    .offset(_hf_bkt_0 as isize))
                                    .hh_head)
                                    .is_null()
                                {
                                    test = ((*((*(*users).hh.tbl).buckets)
                                        .offset(_hf_bkt_0 as isize))
                                        .hh_head as *mut libc::c_char)
                                        .offset(-((*(*users).hh.tbl).hho as isize))
                                        as *mut libc::c_void as *mut example_user_t;
                                } else {
                                    test = 0 as *mut example_user_t;
                                }
                                while !test.is_null() {
                                    if (*test).hh.hashv == _hf_hashv_0
                                        && (*test).hh.keylen as libc::c_ulong
                                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    {
                                        if memcmp(
                                            (*test).hh.key,
                                            &mut i as *mut libc::c_int as *const libc::c_void,
                                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                        ) == 0 as libc::c_int
                                        {
                                            break;
                                        }
                                    }
                                    if !((*test).hh.hh_next).is_null() {
                                        test = ((*test).hh.hh_next as *mut libc::c_char)
                                            .offset(-((*(*users).hh.tbl).hho as isize))
                                            as *mut libc::c_void as *mut example_user_t;
                                    } else {
                                        test = 0 as *mut example_user_t;
                                    }
                                }
                            }
                        }
                    }
                    if test.is_null() {
                        printf(
                            b"test user ID %d not found\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                        );
                    }
                    i += 1;
                    i;
                }
                (*user).hh.tbl = 0 as *mut UT_hash_table;
                (*user).mem_failed = 0 as libc::c_int;
                malloc_failed = 0 as libc::c_int;
                let mut _ha_hashv_3: libc::c_uint = 0;
                let mut _hj_i_5: libc::c_uint = 0;
                let mut _hj_j_5: libc::c_uint = 0;
                let mut _hj_k_5: libc::c_uint = 0;
                let mut _hj_key_5: *const libc::c_uchar = &mut (*user).id
                    as *mut libc::c_int as *const libc::c_uchar;
                _ha_hashv_3 = 0xfeedbeef as libc::c_uint;
                _hj_j_5 = 0x9e3779b9 as libc::c_uint;
                _hj_i_5 = _hj_j_5;
                _hj_k_5 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    as libc::c_uint;
                while _hj_k_5 >= 12 as libc::c_uint {
                    _hj_i_5 = _hj_i_5
                        .wrapping_add(
                            (*_hj_key_5.offset(0 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_5.offset(1 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_5.offset(2 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_5.offset(3 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_j_5 = _hj_j_5
                        .wrapping_add(
                            (*_hj_key_5.offset(4 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_5.offset(5 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_5.offset(6 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_5.offset(7 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _ha_hashv_3 = _ha_hashv_3
                        .wrapping_add(
                            (*_hj_key_5.offset(8 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_5.offset(9 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_5.offset(10 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_5.offset(11 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                    _hj_i_5 = _hj_i_5.wrapping_sub(_ha_hashv_3);
                    _hj_i_5 ^= _ha_hashv_3 >> 13 as libc::c_int;
                    _hj_j_5 = _hj_j_5.wrapping_sub(_ha_hashv_3);
                    _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                    _hj_j_5 ^= _hj_i_5 << 8 as libc::c_int;
                    _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_i_5);
                    _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_j_5);
                    _ha_hashv_3 ^= _hj_j_5 >> 13 as libc::c_int;
                    _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                    _hj_i_5 = _hj_i_5.wrapping_sub(_ha_hashv_3);
                    _hj_i_5 ^= _ha_hashv_3 >> 12 as libc::c_int;
                    _hj_j_5 = _hj_j_5.wrapping_sub(_ha_hashv_3);
                    _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                    _hj_j_5 ^= _hj_i_5 << 16 as libc::c_int;
                    _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_i_5);
                    _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_j_5);
                    _ha_hashv_3 ^= _hj_j_5 >> 5 as libc::c_int;
                    _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                    _hj_i_5 = _hj_i_5.wrapping_sub(_ha_hashv_3);
                    _hj_i_5 ^= _ha_hashv_3 >> 3 as libc::c_int;
                    _hj_j_5 = _hj_j_5.wrapping_sub(_ha_hashv_3);
                    _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                    _hj_j_5 ^= _hj_i_5 << 10 as libc::c_int;
                    _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_i_5);
                    _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_j_5);
                    _ha_hashv_3 ^= _hj_j_5 >> 15 as libc::c_int;
                    _hj_key_5 = _hj_key_5.offset(12 as libc::c_int as isize);
                    _hj_k_5 = _hj_k_5.wrapping_sub(12 as libc::c_uint);
                }
                _ha_hashv_3 = _ha_hashv_3
                    .wrapping_add(
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_uint,
                    );
                let mut current_block_1450: u64;
                match _hj_k_5 {
                    11 => {
                        _ha_hashv_3 = _ha_hashv_3
                            .wrapping_add(
                                (*_hj_key_5.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_1450 = 18143820646854549318;
                    }
                    10 => {
                        current_block_1450 = 18143820646854549318;
                    }
                    9 => {
                        current_block_1450 = 1052835847673256808;
                    }
                    8 => {
                        current_block_1450 = 10968976688043669178;
                    }
                    7 => {
                        current_block_1450 = 13143640764037514010;
                    }
                    6 => {
                        current_block_1450 = 140619065014995601;
                    }
                    5 => {
                        current_block_1450 = 14064143672928710029;
                    }
                    4 => {
                        current_block_1450 = 12987009165854333515;
                    }
                    3 => {
                        current_block_1450 = 3013628610491913122;
                    }
                    2 => {
                        current_block_1450 = 4727416921854594903;
                    }
                    1 => {
                        current_block_1450 = 18077247625661940693;
                    }
                    _ => {
                        current_block_1450 = 16388801870556751896;
                    }
                }
                match current_block_1450 {
                    18143820646854549318 => {
                        _ha_hashv_3 = _ha_hashv_3
                            .wrapping_add(
                                (*_hj_key_5.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_1450 = 1052835847673256808;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    1052835847673256808 => {
                        _ha_hashv_3 = _ha_hashv_3
                            .wrapping_add(
                                (*_hj_key_5.offset(8 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_1450 = 10968976688043669178;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    10968976688043669178 => {
                        _hj_j_5 = _hj_j_5
                            .wrapping_add(
                                (*_hj_key_5.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_1450 = 13143640764037514010;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    13143640764037514010 => {
                        _hj_j_5 = _hj_j_5
                            .wrapping_add(
                                (*_hj_key_5.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_1450 = 140619065014995601;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    140619065014995601 => {
                        _hj_j_5 = _hj_j_5
                            .wrapping_add(
                                (*_hj_key_5.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_1450 = 14064143672928710029;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    14064143672928710029 => {
                        _hj_j_5 = _hj_j_5
                            .wrapping_add(
                                *_hj_key_5.offset(4 as libc::c_int as isize) as libc::c_uint,
                            );
                        current_block_1450 = 12987009165854333515;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    12987009165854333515 => {
                        _hj_i_5 = _hj_i_5
                            .wrapping_add(
                                (*_hj_key_5.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_1450 = 3013628610491913122;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    3013628610491913122 => {
                        _hj_i_5 = _hj_i_5
                            .wrapping_add(
                                (*_hj_key_5.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_1450 = 4727416921854594903;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    4727416921854594903 => {
                        _hj_i_5 = _hj_i_5
                            .wrapping_add(
                                (*_hj_key_5.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_1450 = 18077247625661940693;
                    }
                    _ => {}
                }
                match current_block_1450 {
                    18077247625661940693 => {
                        _hj_i_5 = _hj_i_5
                            .wrapping_add(
                                *_hj_key_5.offset(0 as libc::c_int as isize) as libc::c_uint,
                            );
                    }
                    _ => {}
                }
                _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                _hj_i_5 = _hj_i_5.wrapping_sub(_ha_hashv_3);
                _hj_i_5 ^= _ha_hashv_3 >> 13 as libc::c_int;
                _hj_j_5 = _hj_j_5.wrapping_sub(_ha_hashv_3);
                _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                _hj_j_5 ^= _hj_i_5 << 8 as libc::c_int;
                _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_i_5);
                _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_j_5);
                _ha_hashv_3 ^= _hj_j_5 >> 13 as libc::c_int;
                _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                _hj_i_5 = _hj_i_5.wrapping_sub(_ha_hashv_3);
                _hj_i_5 ^= _ha_hashv_3 >> 12 as libc::c_int;
                _hj_j_5 = _hj_j_5.wrapping_sub(_ha_hashv_3);
                _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                _hj_j_5 ^= _hj_i_5 << 16 as libc::c_int;
                _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_i_5);
                _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_j_5);
                _ha_hashv_3 ^= _hj_j_5 >> 5 as libc::c_int;
                _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                _hj_i_5 = _hj_i_5.wrapping_sub(_ha_hashv_3);
                _hj_i_5 ^= _ha_hashv_3 >> 3 as libc::c_int;
                _hj_j_5 = _hj_j_5.wrapping_sub(_ha_hashv_3);
                _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                _hj_j_5 ^= _hj_i_5 << 10 as libc::c_int;
                _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_i_5);
                _ha_hashv_3 = _ha_hashv_3.wrapping_sub(_hj_j_5);
                _ha_hashv_3 ^= _hj_j_5 >> 15 as libc::c_int;
                let mut _ha_oomed_3: libc::c_int = 0 as libc::c_int;
                (*user).hh.hashv = _ha_hashv_3;
                (*user)
                    .hh
                    .key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
                (*user)
                    .hh
                    .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    as libc::c_uint;
                if users.is_null() {
                    (*user).hh.next = 0 as *mut libc::c_void;
                    (*user).hh.prev = 0 as *mut libc::c_void;
                    (*user)
                        .hh
                        .tbl = alt_malloc(
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    ) as *mut UT_hash_table;
                    if ((*user).hh.tbl).is_null() {
                        _ha_oomed_3 = 1 as libc::c_int;
                    } else {
                        memset(
                            (*user).hh.tbl as *mut libc::c_void,
                            '\0' as i32,
                            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                        );
                        (*(*user).hh.tbl).tail = &mut (*user).hh;
                        (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
                        (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                        (*(*user).hh.tbl)
                            .hho = (&mut (*user).hh as *mut UT_hash_handle
                            as *mut libc::c_char)
                            .offset_from(user as *mut libc::c_char) as libc::c_long;
                        (*(*user).hh.tbl)
                            .buckets = alt_malloc(
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        ) as *mut UT_hash_bucket;
                        (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                        if ((*(*user).hh.tbl).buckets).is_null() {
                            _ha_oomed_3 = 1 as libc::c_int;
                            alt_free((*user).hh.tbl as *mut libc::c_void);
                        } else {
                            memset(
                                (*(*user).hh.tbl).buckets as *mut libc::c_void,
                                '\0' as i32,
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            );
                            if _ha_oomed_3 != 0 {
                                alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                                alt_free((*user).hh.tbl as *mut libc::c_void);
                            }
                        }
                    }
                    if _ha_oomed_3 == 0 {
                        users = user;
                    }
                } else {
                    (*user).hh.tbl = (*users).hh.tbl;
                    (*user).hh.next = 0 as *mut libc::c_void;
                    (*user)
                        .hh
                        .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
                        .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
                    (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
                    (*(*users).hh.tbl).tail = &mut (*user).hh;
                }
                if _ha_oomed_3 == 0 {
                    let mut _ha_bkt_3: libc::c_uint = 0;
                    (*(*users).hh.tbl)
                        .num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
                    (*(*users).hh.tbl).num_items;
                    _ha_bkt_3 = _ha_hashv_3
                        & ((*(*users).hh.tbl).num_buckets)
                            .wrapping_sub(1 as libc::c_uint);
                    let mut _ha_head_3: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                        .buckets)
                        .offset(_ha_bkt_3 as isize) as *mut UT_hash_bucket;
                    (*_ha_head_3).count = ((*_ha_head_3).count).wrapping_add(1);
                    (*_ha_head_3).count;
                    (*user).hh.hh_next = (*_ha_head_3).hh_head;
                    (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
                    if !((*_ha_head_3).hh_head).is_null() {
                        (*(*_ha_head_3).hh_head).hh_prev = &mut (*user).hh;
                    }
                    (*_ha_head_3).hh_head = &mut (*user).hh;
                    if (*_ha_head_3).count
                        >= ((*_ha_head_3).expand_mult)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_mul(10 as libc::c_uint)
                        && (*(*user).hh.tbl).noexpand == 0
                    {
                        let mut _he_bkt_3: libc::c_uint = 0;
                        let mut _he_bkt_i_3: libc::c_uint = 0;
                        let mut _he_thh_3: *mut UT_hash_handle = 0
                            as *mut UT_hash_handle;
                        let mut _he_hh_nxt_3: *mut UT_hash_handle = 0
                            as *mut UT_hash_handle;
                        let mut _he_new_buckets_3: *mut UT_hash_bucket = 0
                            as *mut UT_hash_bucket;
                        let mut _he_newbkt_3: *mut UT_hash_bucket = 0
                            as *mut UT_hash_bucket;
                        _he_new_buckets_3 = alt_malloc(
                            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                .wrapping_mul(
                                    (*(*user).hh.tbl).num_buckets as libc::c_ulong,
                                )
                                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                        ) as *mut UT_hash_bucket;
                        if _he_new_buckets_3.is_null() {
                            _ha_oomed_3 = 1 as libc::c_int;
                        } else {
                            memset(
                                _he_new_buckets_3 as *mut libc::c_void,
                                '\0' as i32,
                                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                    .wrapping_mul(
                                        (*(*user).hh.tbl).num_buckets as libc::c_ulong,
                                    )
                                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                            );
                            (*(*user).hh.tbl)
                                .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                                >> ((*(*user).hh.tbl).log2_num_buckets)
                                    .wrapping_add(1 as libc::c_uint))
                                .wrapping_add(
                                    (if (*(*user).hh.tbl).num_items
                                        & ((*(*user).hh.tbl).num_buckets)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                                    {
                                        1 as libc::c_uint
                                    } else {
                                        0 as libc::c_uint
                                    }),
                                );
                            (*(*user).hh.tbl)
                                .nonideal_items = 0 as libc::c_int as libc::c_uint;
                            _he_bkt_i_3 = 0 as libc::c_int as libc::c_uint;
                            while _he_bkt_i_3 < (*(*user).hh.tbl).num_buckets {
                                _he_thh_3 = (*((*(*user).hh.tbl).buckets)
                                    .offset(_he_bkt_i_3 as isize))
                                    .hh_head;
                                while !_he_thh_3.is_null() {
                                    _he_hh_nxt_3 = (*_he_thh_3).hh_next;
                                    _he_bkt_3 = (*_he_thh_3).hashv
                                        & ((*(*user).hh.tbl).num_buckets)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_uint);
                                    _he_newbkt_3 = &mut *_he_new_buckets_3
                                        .offset(_he_bkt_3 as isize) as *mut UT_hash_bucket;
                                    (*_he_newbkt_3)
                                        .count = ((*_he_newbkt_3).count).wrapping_add(1);
                                    if (*_he_newbkt_3).count
                                        > (*(*user).hh.tbl).ideal_chain_maxlen
                                    {
                                        (*(*user).hh.tbl)
                                            .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                            .wrapping_add(1);
                                        (*(*user).hh.tbl).nonideal_items;
                                        if (*_he_newbkt_3).count
                                            > ((*_he_newbkt_3).expand_mult)
                                                .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
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
                            alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                            (*(*user).hh.tbl)
                                .num_buckets = ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint);
                            (*(*user).hh.tbl)
                                .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                                .wrapping_add(1);
                            (*(*user).hh.tbl).log2_num_buckets;
                            (*(*user).hh.tbl).buckets = _he_new_buckets_3;
                            (*(*user).hh.tbl)
                                .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                                > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                            {
                                ((*(*user).hh.tbl).ineff_expands)
                                    .wrapping_add(1 as libc::c_uint)
                            } else {
                                0 as libc::c_uint
                            };
                            if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                                (*(*user).hh.tbl)
                                    .noexpand = 1 as libc::c_int as libc::c_uint;
                            }
                        }
                        if _ha_oomed_3 != 0 {
                            let mut _hd_head_7: *mut UT_hash_bucket = &mut *((*(*users)
                                .hh
                                .tbl)
                                .buckets)
                                .offset(_ha_bkt_3 as isize) as *mut UT_hash_bucket;
                            (*_hd_head_7).count = ((*_hd_head_7).count).wrapping_sub(1);
                            (*_hd_head_7).count;
                            if (*_hd_head_7).hh_head
                                == &mut (*user).hh as *mut UT_hash_handle
                            {
                                (*_hd_head_7).hh_head = (*user).hh.hh_next;
                            }
                            if !((*user).hh.hh_prev).is_null() {
                                (*(*user).hh.hh_prev).hh_next = (*user).hh.hh_next;
                            }
                            if !((*user).hh.hh_next).is_null() {
                                (*(*user).hh.hh_next).hh_prev = (*user).hh.hh_prev;
                            }
                        }
                    }
                    if _ha_oomed_3 != 0 {
                        let mut _hd_hh_item_3: *mut UT_hash_handle = &mut (*user).hh;
                        let mut _hd_bkt_7: libc::c_uint = 0;
                        _hd_bkt_7 = (*_hd_hh_item_3).hashv
                            & ((*(*users).hh.tbl).num_buckets)
                                .wrapping_sub(1 as libc::c_uint);
                        let ref mut fresh12 = (*((*(*users).hh.tbl).buckets)
                            .offset(_hd_bkt_7 as isize))
                            .count;
                        *fresh12 = (*fresh12).wrapping_add(1);
                        *fresh12;
                        (*_hd_hh_item_3).hh_next = 0 as *mut UT_hash_handle;
                        (*_hd_hh_item_3).hh_prev = 0 as *mut UT_hash_handle;
                        let mut _hd_hh_del_3: *mut UT_hash_handle = &mut (*user).hh;
                        if ((*_hd_hh_del_3).prev).is_null()
                            && ((*_hd_hh_del_3).next).is_null()
                        {
                            alt_free((*(*users).hh.tbl).buckets as *mut libc::c_void);
                            alt_free((*users).hh.tbl as *mut libc::c_void);
                            users = 0 as *mut example_user_t;
                        } else {
                            let mut _hd_bkt_8: libc::c_uint = 0;
                            if _hd_hh_del_3 == (*(*users).hh.tbl).tail {
                                (*(*users).hh.tbl)
                                    .tail = ((*_hd_hh_del_3).prev as *mut libc::c_char)
                                    .offset((*(*users).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle;
                            }
                            if !((*_hd_hh_del_3).prev).is_null() {
                                let ref mut fresh13 = (*(((*_hd_hh_del_3).prev
                                    as *mut libc::c_char)
                                    .offset((*(*users).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle))
                                    .next;
                                *fresh13 = (*_hd_hh_del_3).next;
                            } else {
                                users = (*_hd_hh_del_3).next as *mut example_user_t;
                            }
                            if !((*_hd_hh_del_3).next).is_null() {
                                let ref mut fresh14 = (*(((*_hd_hh_del_3).next
                                    as *mut libc::c_char)
                                    .offset((*(*users).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle))
                                    .prev;
                                *fresh14 = (*_hd_hh_del_3).prev;
                            }
                            _hd_bkt_8 = (*_hd_hh_del_3).hashv
                                & ((*(*users).hh.tbl).num_buckets)
                                    .wrapping_sub(1 as libc::c_uint);
                            let mut _hd_head_8: *mut UT_hash_bucket = &mut *((*(*users)
                                .hh
                                .tbl)
                                .buckets)
                                .offset(_hd_bkt_8 as isize) as *mut UT_hash_bucket;
                            (*_hd_head_8).count = ((*_hd_head_8).count).wrapping_sub(1);
                            (*_hd_head_8).count;
                            if (*_hd_head_8).hh_head == _hd_hh_del_3 {
                                (*_hd_head_8).hh_head = (*_hd_hh_del_3).hh_next;
                            }
                            if !((*_hd_hh_del_3).hh_prev).is_null() {
                                (*(*_hd_hh_del_3).hh_prev)
                                    .hh_next = (*_hd_hh_del_3).hh_next;
                            }
                            if !((*_hd_hh_del_3).hh_next).is_null() {
                                (*(*_hd_hh_del_3).hh_next)
                                    .hh_prev = (*_hd_hh_del_3).hh_prev;
                            }
                            (*(*users).hh.tbl)
                                .num_items = ((*(*users).hh.tbl).num_items).wrapping_sub(1);
                            (*(*users).hh.tbl).num_items;
                        }
                        (*user).hh.tbl = 0 as *mut UT_hash_table;
                        (*user).mem_failed = 1 as libc::c_int;
                    }
                } else {
                    (*user).hh.tbl = 0 as *mut UT_hash_table;
                    (*user).mem_failed = 1 as libc::c_int;
                }
                if malloc_failed == 0 {
                    puts(
                        b"malloc should have been attempted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !((*user).hh.tbl).is_null() {
                    puts(
                        b"user->hh.tbl should be NULL after second failure\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*user).mem_failed != 1 as libc::c_int {
                    printf(
                        b"mem_failed should be 1 after second failure, not %d\n\0"
                            as *const u8 as *const libc::c_char,
                        (*user).mem_failed,
                    );
                }
                break;
            } else {
                id += 1;
                id;
            }
        }
    }
    saved_cnt = id;
    if (if !users.is_null() { (*(*users).hh.tbl).num_items } else { 0 as libc::c_uint })
        != saved_cnt as libc::c_uint
    {
        printf(
            b"Got HASH_COUNT(users)=%d, should be %d\n\0" as *const u8
                as *const libc::c_char,
            if !users.is_null() {
                (*(*users).hh.tbl).num_items
            } else {
                0 as libc::c_uint
            },
            saved_cnt,
        );
    }
    i = 0 as libc::c_int;
    while i < saved_cnt {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        id += 1;
        (*user).id = id;
        malloc_cnt = 20 as libc::c_int;
        let mut _ha_hashv_4: libc::c_uint = 0;
        let mut _hj_i_6: libc::c_uint = 0;
        let mut _hj_j_6: libc::c_uint = 0;
        let mut _hj_k_6: libc::c_uint = 0;
        let mut _hj_key_6: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
            as *const libc::c_uchar;
        _ha_hashv_4 = 0xfeedbeef as libc::c_uint;
        _hj_j_6 = 0x9e3779b9 as libc::c_uint;
        _hj_i_6 = _hj_j_6;
        _hj_k_6 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k_6 >= 12 as libc::c_uint {
            _hj_i_6 = _hj_i_6
                .wrapping_add(
                    (*_hj_key_6.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_6.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_6.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_6.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_j_6 = _hj_j_6
                .wrapping_add(
                    (*_hj_key_6.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_6.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_6.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_6.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _ha_hashv_4 = _ha_hashv_4
                .wrapping_add(
                    (*_hj_key_6.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_6.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_6.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_6.offset(11 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
            _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_4);
            _hj_i_6 ^= _ha_hashv_4 >> 13 as libc::c_int;
            _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_4);
            _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
            _hj_j_6 ^= _hj_i_6 << 8 as libc::c_int;
            _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_i_6);
            _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_j_6);
            _ha_hashv_4 ^= _hj_j_6 >> 13 as libc::c_int;
            _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
            _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_4);
            _hj_i_6 ^= _ha_hashv_4 >> 12 as libc::c_int;
            _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_4);
            _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
            _hj_j_6 ^= _hj_i_6 << 16 as libc::c_int;
            _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_i_6);
            _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_j_6);
            _ha_hashv_4 ^= _hj_j_6 >> 5 as libc::c_int;
            _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
            _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_4);
            _hj_i_6 ^= _ha_hashv_4 >> 3 as libc::c_int;
            _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_4);
            _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
            _hj_j_6 ^= _hj_i_6 << 10 as libc::c_int;
            _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_i_6);
            _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_j_6);
            _ha_hashv_4 ^= _hj_j_6 >> 15 as libc::c_int;
            _hj_key_6 = _hj_key_6.offset(12 as libc::c_int as isize);
            _hj_k_6 = _hj_k_6.wrapping_sub(12 as libc::c_uint);
        }
        _ha_hashv_4 = _ha_hashv_4
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_1746: u64;
        match _hj_k_6 {
            11 => {
                _ha_hashv_4 = _ha_hashv_4
                    .wrapping_add(
                        (*_hj_key_6.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_1746 = 17279024411669645389;
            }
            10 => {
                current_block_1746 = 17279024411669645389;
            }
            9 => {
                current_block_1746 = 842883584864673994;
            }
            8 => {
                current_block_1746 = 18431053683317803030;
            }
            7 => {
                current_block_1746 = 13681275273360481710;
            }
            6 => {
                current_block_1746 = 11042831158399790661;
            }
            5 => {
                current_block_1746 = 18415398407217357927;
            }
            4 => {
                current_block_1746 = 14868746346460426310;
            }
            3 => {
                current_block_1746 = 5161946086944071447;
            }
            2 => {
                current_block_1746 = 17697770112021979980;
            }
            1 => {
                current_block_1746 = 15100205504772456296;
            }
            _ => {
                current_block_1746 = 14506394747943023865;
            }
        }
        match current_block_1746 {
            17279024411669645389 => {
                _ha_hashv_4 = _ha_hashv_4
                    .wrapping_add(
                        (*_hj_key_6.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_1746 = 842883584864673994;
            }
            _ => {}
        }
        match current_block_1746 {
            842883584864673994 => {
                _ha_hashv_4 = _ha_hashv_4
                    .wrapping_add(
                        (*_hj_key_6.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_1746 = 18431053683317803030;
            }
            _ => {}
        }
        match current_block_1746 {
            18431053683317803030 => {
                _hj_j_6 = _hj_j_6
                    .wrapping_add(
                        (*_hj_key_6.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_1746 = 13681275273360481710;
            }
            _ => {}
        }
        match current_block_1746 {
            13681275273360481710 => {
                _hj_j_6 = _hj_j_6
                    .wrapping_add(
                        (*_hj_key_6.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_1746 = 11042831158399790661;
            }
            _ => {}
        }
        match current_block_1746 {
            11042831158399790661 => {
                _hj_j_6 = _hj_j_6
                    .wrapping_add(
                        (*_hj_key_6.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_1746 = 18415398407217357927;
            }
            _ => {}
        }
        match current_block_1746 {
            18415398407217357927 => {
                _hj_j_6 = _hj_j_6
                    .wrapping_add(
                        *_hj_key_6.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_1746 = 14868746346460426310;
            }
            _ => {}
        }
        match current_block_1746 {
            14868746346460426310 => {
                _hj_i_6 = _hj_i_6
                    .wrapping_add(
                        (*_hj_key_6.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_1746 = 5161946086944071447;
            }
            _ => {}
        }
        match current_block_1746 {
            5161946086944071447 => {
                _hj_i_6 = _hj_i_6
                    .wrapping_add(
                        (*_hj_key_6.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_1746 = 17697770112021979980;
            }
            _ => {}
        }
        match current_block_1746 {
            17697770112021979980 => {
                _hj_i_6 = _hj_i_6
                    .wrapping_add(
                        (*_hj_key_6.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_1746 = 15100205504772456296;
            }
            _ => {}
        }
        match current_block_1746 {
            15100205504772456296 => {
                _hj_i_6 = _hj_i_6
                    .wrapping_add(
                        *_hj_key_6.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
        _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_4);
        _hj_i_6 ^= _ha_hashv_4 >> 13 as libc::c_int;
        _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_4);
        _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
        _hj_j_6 ^= _hj_i_6 << 8 as libc::c_int;
        _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_i_6);
        _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_j_6);
        _ha_hashv_4 ^= _hj_j_6 >> 13 as libc::c_int;
        _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
        _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_4);
        _hj_i_6 ^= _ha_hashv_4 >> 12 as libc::c_int;
        _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_4);
        _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
        _hj_j_6 ^= _hj_i_6 << 16 as libc::c_int;
        _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_i_6);
        _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_j_6);
        _ha_hashv_4 ^= _hj_j_6 >> 5 as libc::c_int;
        _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
        _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_4);
        _hj_i_6 ^= _ha_hashv_4 >> 3 as libc::c_int;
        _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_4);
        _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
        _hj_j_6 ^= _hj_i_6 << 10 as libc::c_int;
        _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_i_6);
        _ha_hashv_4 = _ha_hashv_4.wrapping_sub(_hj_j_6);
        _ha_hashv_4 ^= _hj_j_6 >> 15 as libc::c_int;
        let mut _ha_oomed_4: libc::c_int = 0 as libc::c_int;
        (*user).hh.hashv = _ha_hashv_4;
        (*user).hh.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
        (*user)
            .hh
            .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            as libc::c_uint;
        if users.is_null() {
            (*user).hh.next = 0 as *mut libc::c_void;
            (*user).hh.prev = 0 as *mut libc::c_void;
            (*user)
                .hh
                .tbl = alt_malloc(
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            ) as *mut UT_hash_table;
            if ((*user).hh.tbl).is_null() {
                _ha_oomed_4 = 1 as libc::c_int;
            } else {
                memset(
                    (*user).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*user).hh.tbl).tail = &mut (*user).hh;
                (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*user).hh.tbl)
                    .hho = (&mut (*user).hh as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(user as *mut libc::c_char) as libc::c_long;
                (*(*user).hh.tbl)
                    .buckets = alt_malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*user).hh.tbl).buckets).is_null() {
                    _ha_oomed_4 = 1 as libc::c_int;
                    alt_free((*user).hh.tbl as *mut libc::c_void);
                } else {
                    memset(
                        (*(*user).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                    if _ha_oomed_4 != 0 {
                        alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                        alt_free((*user).hh.tbl as *mut libc::c_void);
                    }
                }
            }
            if _ha_oomed_4 == 0 {
                users = user;
            }
        } else {
            (*user).hh.tbl = (*users).hh.tbl;
            (*user).hh.next = 0 as *mut libc::c_void;
            (*user)
                .hh
                .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
            (*(*users).hh.tbl).tail = &mut (*user).hh;
        }
        if _ha_oomed_4 == 0 {
            let mut _ha_bkt_4: libc::c_uint = 0;
            (*(*users).hh.tbl)
                .num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
            (*(*users).hh.tbl).num_items;
            _ha_bkt_4 = _ha_hashv_4
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let mut _ha_head_4: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl).buckets)
                .offset(_ha_bkt_4 as isize) as *mut UT_hash_bucket;
            (*_ha_head_4).count = ((*_ha_head_4).count).wrapping_add(1);
            (*_ha_head_4).count;
            (*user).hh.hh_next = (*_ha_head_4).hh_head;
            (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
            if !((*_ha_head_4).hh_head).is_null() {
                (*(*_ha_head_4).hh_head).hh_prev = &mut (*user).hh;
            }
            (*_ha_head_4).hh_head = &mut (*user).hh;
            if (*_ha_head_4).count
                >= ((*_ha_head_4).expand_mult)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_mul(10 as libc::c_uint) && (*(*user).hh.tbl).noexpand == 0
            {
                let mut _he_bkt_4: libc::c_uint = 0;
                let mut _he_bkt_i_4: libc::c_uint = 0;
                let mut _he_thh_4: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                let mut _he_hh_nxt_4: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                let mut _he_new_buckets_4: *mut UT_hash_bucket = 0
                    as *mut UT_hash_bucket;
                let mut _he_newbkt_4: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                _he_new_buckets_4 = alt_malloc(
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets_4.is_null() {
                    _ha_oomed_4 = 1 as libc::c_int;
                } else {
                    memset(
                        _he_new_buckets_4 as *mut libc::c_void,
                        '\0' as i32,
                        (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                            .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                    );
                    (*(*user).hh.tbl)
                        .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                        >> ((*(*user).hh.tbl).log2_num_buckets)
                            .wrapping_add(1 as libc::c_uint))
                        .wrapping_add(
                            (if (*(*user).hh.tbl).num_items
                                & ((*(*user).hh.tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                            {
                                1 as libc::c_uint
                            } else {
                                0 as libc::c_uint
                            }),
                        );
                    (*(*user).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                    _he_bkt_i_4 = 0 as libc::c_int as libc::c_uint;
                    while _he_bkt_i_4 < (*(*user).hh.tbl).num_buckets {
                        _he_thh_4 = (*((*(*user).hh.tbl).buckets)
                            .offset(_he_bkt_i_4 as isize))
                            .hh_head;
                        while !_he_thh_4.is_null() {
                            _he_hh_nxt_4 = (*_he_thh_4).hh_next;
                            _he_bkt_4 = (*_he_thh_4).hashv
                                & ((*(*user).hh.tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_uint);
                            _he_newbkt_4 = &mut *_he_new_buckets_4
                                .offset(_he_bkt_4 as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt_4)
                                .count = ((*_he_newbkt_4).count).wrapping_add(1);
                            if (*_he_newbkt_4).count
                                > (*(*user).hh.tbl).ideal_chain_maxlen
                            {
                                (*(*user).hh.tbl)
                                    .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                    .wrapping_add(1);
                                (*(*user).hh.tbl).nonideal_items;
                                if (*_he_newbkt_4).count
                                    > ((*_he_newbkt_4).expand_mult)
                                        .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
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
                    alt_free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                    (*(*user).hh.tbl)
                        .num_buckets = ((*(*user).hh.tbl).num_buckets)
                        .wrapping_mul(2 as libc::c_uint);
                    (*(*user).hh.tbl)
                        .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                        .wrapping_add(1);
                    (*(*user).hh.tbl).log2_num_buckets;
                    (*(*user).hh.tbl).buckets = _he_new_buckets_4;
                    (*(*user).hh.tbl)
                        .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                        > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                    {
                        ((*(*user).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                    } else {
                        0 as libc::c_uint
                    };
                    if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                        (*(*user).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                    }
                }
                if _ha_oomed_4 != 0 {
                    let mut _hd_head_9: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                        .buckets)
                        .offset(_ha_bkt_4 as isize) as *mut UT_hash_bucket;
                    (*_hd_head_9).count = ((*_hd_head_9).count).wrapping_sub(1);
                    (*_hd_head_9).count;
                    if (*_hd_head_9).hh_head == &mut (*user).hh as *mut UT_hash_handle {
                        (*_hd_head_9).hh_head = (*user).hh.hh_next;
                    }
                    if !((*user).hh.hh_prev).is_null() {
                        (*(*user).hh.hh_prev).hh_next = (*user).hh.hh_next;
                    }
                    if !((*user).hh.hh_next).is_null() {
                        (*(*user).hh.hh_next).hh_prev = (*user).hh.hh_prev;
                    }
                }
            }
            if _ha_oomed_4 != 0 {
                let mut _hd_hh_item_4: *mut UT_hash_handle = &mut (*user).hh;
                let mut _hd_bkt_9: libc::c_uint = 0;
                _hd_bkt_9 = (*_hd_hh_item_4).hashv
                    & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let ref mut fresh15 = (*((*(*users).hh.tbl).buckets)
                    .offset(_hd_bkt_9 as isize))
                    .count;
                *fresh15 = (*fresh15).wrapping_add(1);
                *fresh15;
                (*_hd_hh_item_4).hh_next = 0 as *mut UT_hash_handle;
                (*_hd_hh_item_4).hh_prev = 0 as *mut UT_hash_handle;
                let mut _hd_hh_del_4: *mut UT_hash_handle = &mut (*user).hh;
                if ((*_hd_hh_del_4).prev).is_null() && ((*_hd_hh_del_4).next).is_null() {
                    alt_free((*(*users).hh.tbl).buckets as *mut libc::c_void);
                    alt_free((*users).hh.tbl as *mut libc::c_void);
                    users = 0 as *mut example_user_t;
                } else {
                    let mut _hd_bkt_10: libc::c_uint = 0;
                    if _hd_hh_del_4 == (*(*users).hh.tbl).tail {
                        (*(*users).hh.tbl)
                            .tail = ((*_hd_hh_del_4).prev as *mut libc::c_char)
                            .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                            as *mut UT_hash_handle;
                    }
                    if !((*_hd_hh_del_4).prev).is_null() {
                        let ref mut fresh16 = (*(((*_hd_hh_del_4).prev
                            as *mut libc::c_char)
                            .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                            as *mut UT_hash_handle))
                            .next;
                        *fresh16 = (*_hd_hh_del_4).next;
                    } else {
                        users = (*_hd_hh_del_4).next as *mut example_user_t;
                    }
                    if !((*_hd_hh_del_4).next).is_null() {
                        let ref mut fresh17 = (*(((*_hd_hh_del_4).next
                            as *mut libc::c_char)
                            .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh17 = (*_hd_hh_del_4).prev;
                    }
                    _hd_bkt_10 = (*_hd_hh_del_4).hashv
                        & ((*(*users).hh.tbl).num_buckets)
                            .wrapping_sub(1 as libc::c_uint);
                    let mut _hd_head_10: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                        .buckets)
                        .offset(_hd_bkt_10 as isize) as *mut UT_hash_bucket;
                    (*_hd_head_10).count = ((*_hd_head_10).count).wrapping_sub(1);
                    (*_hd_head_10).count;
                    if (*_hd_head_10).hh_head == _hd_hh_del_4 {
                        (*_hd_head_10).hh_head = (*_hd_hh_del_4).hh_next;
                    }
                    if !((*_hd_hh_del_4).hh_prev).is_null() {
                        (*(*_hd_hh_del_4).hh_prev).hh_next = (*_hd_hh_del_4).hh_next;
                    }
                    if !((*_hd_hh_del_4).hh_next).is_null() {
                        (*(*_hd_hh_del_4).hh_next).hh_prev = (*_hd_hh_del_4).hh_prev;
                    }
                    (*(*users).hh.tbl)
                        .num_items = ((*(*users).hh.tbl).num_items).wrapping_sub(1);
                    (*(*users).hh.tbl).num_items;
                }
                (*user).hh.tbl = 0 as *mut UT_hash_table;
                (*user).mem_failed = 1 as libc::c_int;
            }
        } else {
            (*user).hh.tbl = 0 as *mut UT_hash_table;
            (*user).mem_failed = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    user = users;
    test = (if !users.is_null() { (*users).hh.next } else { 0 as *mut libc::c_void })
        as *mut example_user_t;
    while !user.is_null() {
        (*user).mem_failed = 0 as libc::c_int;
        user = test;
        test = (if !test.is_null() { (*test).hh.next } else { 0 as *mut libc::c_void })
            as *mut example_user_t;
    }
    malloc_cnt = 0 as libc::c_int;
    free_cnt = 0 as libc::c_int;
    let mut _src_bkt: libc::c_uint = 0;
    let mut _dst_bkt: libc::c_uint = 0;
    let mut _last_elt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _elt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _src_hh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _dst_hh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _last_elt_hh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _dst_hho: ptrdiff_t = (&mut (*users2).hh2 as *mut UT_hash_handle
        as *mut libc::c_char)
        .offset_from(users2 as *mut libc::c_char) as libc::c_long;
    if !users.is_null() {
        _src_bkt = 0 as libc::c_int as libc::c_uint;
        while _src_bkt < (*(*users).hh.tbl).num_buckets {
            let mut current_block_2159: u64;
            _src_hh = (*((*(*users).hh.tbl).buckets).offset(_src_bkt as isize)).hh_head;
            while !_src_hh.is_null() {
                _elt = (_src_hh as *mut libc::c_char)
                    .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
                let mut _hs_oomed: libc::c_int = 0 as libc::c_int;
                _dst_hh = (_elt as *mut libc::c_char).offset(_dst_hho as isize)
                    as *mut libc::c_void as *mut UT_hash_handle;
                (*_dst_hh).key = (*_src_hh).key;
                (*_dst_hh).keylen = (*_src_hh).keylen;
                (*_dst_hh).hashv = (*_src_hh).hashv;
                (*_dst_hh).prev = _last_elt;
                (*_dst_hh).next = 0 as *mut libc::c_void;
                if !_last_elt_hh.is_null() {
                    (*_last_elt_hh).next = _elt;
                }
                if users2.is_null() {
                    users2 = _elt as *mut example_user_t;
                    (*users2)
                        .hh2
                        .tbl = alt_malloc(
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    ) as *mut UT_hash_table;
                    if ((*users2).hh2.tbl).is_null() {
                        _hs_oomed = 1 as libc::c_int;
                    } else {
                        memset(
                            (*users2).hh2.tbl as *mut libc::c_void,
                            '\0' as i32,
                            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                        );
                        (*(*users2).hh2.tbl).tail = &mut (*users2).hh2;
                        (*(*users2).hh2.tbl).num_buckets = 32 as libc::c_uint;
                        (*(*users2).hh2.tbl).log2_num_buckets = 5 as libc::c_uint;
                        (*(*users2).hh2.tbl)
                            .hho = (&mut (*users2).hh2 as *mut UT_hash_handle
                            as *mut libc::c_char)
                            .offset_from(users2 as *mut libc::c_char) as libc::c_long;
                        (*(*users2).hh2.tbl)
                            .buckets = alt_malloc(
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        ) as *mut UT_hash_bucket;
                        (*(*users2).hh2.tbl).signature = 0xa0111fe1 as libc::c_uint;
                        if ((*(*users2).hh2.tbl).buckets).is_null() {
                            _hs_oomed = 1 as libc::c_int;
                            alt_free((*users2).hh2.tbl as *mut libc::c_void);
                        } else {
                            memset(
                                (*(*users2).hh2.tbl).buckets as *mut libc::c_void,
                                '\0' as i32,
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            );
                            if _hs_oomed != 0 {
                                alt_free((*(*users2).hh2.tbl).buckets as *mut libc::c_void);
                                alt_free((*users2).hh2.tbl as *mut libc::c_void);
                            }
                        }
                    }
                    if _hs_oomed != 0 {
                        (*(_elt as *mut example_user_t)).mem_failed = 1 as libc::c_int;
                        users2 = 0 as *mut example_user_t;
                        current_block_2159 = 14375782100664666947;
                    } else {
                        current_block_2159 = 17317333352697215463;
                    }
                } else {
                    (*_dst_hh).tbl = (*users2).hh2.tbl;
                    current_block_2159 = 17317333352697215463;
                }
                match current_block_2159 {
                    17317333352697215463 => {
                        _dst_bkt = (*_dst_hh).hashv
                            & ((*(*_dst_hh).tbl).num_buckets)
                                .wrapping_sub(1 as libc::c_uint);
                        let mut _ha_head_5: *mut UT_hash_bucket = &mut *((*(*_dst_hh)
                            .tbl)
                            .buckets)
                            .offset(_dst_bkt as isize) as *mut UT_hash_bucket;
                        (*_ha_head_5).count = ((*_ha_head_5).count).wrapping_add(1);
                        (*_ha_head_5).count;
                        (*_dst_hh).hh_next = (*_ha_head_5).hh_head;
                        (*_dst_hh).hh_prev = 0 as *mut UT_hash_handle;
                        if !((*_ha_head_5).hh_head).is_null() {
                            (*(*_ha_head_5).hh_head).hh_prev = _dst_hh;
                        }
                        (*_ha_head_5).hh_head = _dst_hh;
                        if (*_ha_head_5).count
                            >= ((*_ha_head_5).expand_mult)
                                .wrapping_add(1 as libc::c_uint)
                                .wrapping_mul(10 as libc::c_uint)
                            && (*(*_dst_hh).tbl).noexpand == 0
                        {
                            let mut _he_bkt_5: libc::c_uint = 0;
                            let mut _he_bkt_i_5: libc::c_uint = 0;
                            let mut _he_thh_5: *mut UT_hash_handle = 0
                                as *mut UT_hash_handle;
                            let mut _he_hh_nxt_5: *mut UT_hash_handle = 0
                                as *mut UT_hash_handle;
                            let mut _he_new_buckets_5: *mut UT_hash_bucket = 0
                                as *mut UT_hash_bucket;
                            let mut _he_newbkt_5: *mut UT_hash_bucket = 0
                                as *mut UT_hash_bucket;
                            _he_new_buckets_5 = alt_malloc(
                                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                    .wrapping_mul(
                                        (*(*_dst_hh).tbl).num_buckets as libc::c_ulong,
                                    )
                                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                            ) as *mut UT_hash_bucket;
                            if _he_new_buckets_5.is_null() {
                                _hs_oomed = 1 as libc::c_int;
                            } else {
                                memset(
                                    _he_new_buckets_5 as *mut libc::c_void,
                                    '\0' as i32,
                                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                        .wrapping_mul(
                                            (*(*_dst_hh).tbl).num_buckets as libc::c_ulong,
                                        )
                                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                                );
                                (*(*_dst_hh).tbl)
                                    .ideal_chain_maxlen = ((*(*_dst_hh).tbl).num_items
                                    >> ((*(*_dst_hh).tbl).log2_num_buckets)
                                        .wrapping_add(1 as libc::c_uint))
                                    .wrapping_add(
                                        (if (*(*_dst_hh).tbl).num_items
                                            & ((*(*_dst_hh).tbl).num_buckets)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                                        {
                                            1 as libc::c_uint
                                        } else {
                                            0 as libc::c_uint
                                        }),
                                    );
                                (*(*_dst_hh).tbl)
                                    .nonideal_items = 0 as libc::c_int as libc::c_uint;
                                _he_bkt_i_5 = 0 as libc::c_int as libc::c_uint;
                                while _he_bkt_i_5 < (*(*_dst_hh).tbl).num_buckets {
                                    _he_thh_5 = (*((*(*_dst_hh).tbl).buckets)
                                        .offset(_he_bkt_i_5 as isize))
                                        .hh_head;
                                    while !_he_thh_5.is_null() {
                                        _he_hh_nxt_5 = (*_he_thh_5).hh_next;
                                        _he_bkt_5 = (*_he_thh_5).hashv
                                            & ((*(*_dst_hh).tbl).num_buckets)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_sub(1 as libc::c_uint);
                                        _he_newbkt_5 = &mut *_he_new_buckets_5
                                            .offset(_he_bkt_5 as isize) as *mut UT_hash_bucket;
                                        (*_he_newbkt_5)
                                            .count = ((*_he_newbkt_5).count).wrapping_add(1);
                                        if (*_he_newbkt_5).count
                                            > (*(*_dst_hh).tbl).ideal_chain_maxlen
                                        {
                                            (*(*_dst_hh).tbl)
                                                .nonideal_items = ((*(*_dst_hh).tbl).nonideal_items)
                                                .wrapping_add(1);
                                            (*(*_dst_hh).tbl).nonideal_items;
                                            if (*_he_newbkt_5).count
                                                > ((*_he_newbkt_5).expand_mult)
                                                    .wrapping_mul((*(*_dst_hh).tbl).ideal_chain_maxlen)
                                            {
                                                (*_he_newbkt_5)
                                                    .expand_mult = ((*_he_newbkt_5).expand_mult)
                                                    .wrapping_add(1);
                                                (*_he_newbkt_5).expand_mult;
                                            }
                                        }
                                        (*_he_thh_5).hh_prev = 0 as *mut UT_hash_handle;
                                        (*_he_thh_5).hh_next = (*_he_newbkt_5).hh_head;
                                        if !((*_he_newbkt_5).hh_head).is_null() {
                                            (*(*_he_newbkt_5).hh_head).hh_prev = _he_thh_5;
                                        }
                                        (*_he_newbkt_5).hh_head = _he_thh_5;
                                        _he_thh_5 = _he_hh_nxt_5;
                                    }
                                    _he_bkt_i_5 = _he_bkt_i_5.wrapping_add(1);
                                    _he_bkt_i_5;
                                }
                                alt_free((*(*_dst_hh).tbl).buckets as *mut libc::c_void);
                                (*(*_dst_hh).tbl)
                                    .num_buckets = ((*(*_dst_hh).tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint);
                                (*(*_dst_hh).tbl)
                                    .log2_num_buckets = ((*(*_dst_hh).tbl).log2_num_buckets)
                                    .wrapping_add(1);
                                (*(*_dst_hh).tbl).log2_num_buckets;
                                (*(*_dst_hh).tbl).buckets = _he_new_buckets_5;
                                (*(*_dst_hh).tbl)
                                    .ineff_expands = if (*(*_dst_hh).tbl).nonideal_items
                                    > (*(*_dst_hh).tbl).num_items >> 1 as libc::c_int
                                {
                                    ((*(*_dst_hh).tbl).ineff_expands)
                                        .wrapping_add(1 as libc::c_uint)
                                } else {
                                    0 as libc::c_uint
                                };
                                if (*(*_dst_hh).tbl).ineff_expands > 1 as libc::c_uint {
                                    (*(*_dst_hh).tbl)
                                        .noexpand = 1 as libc::c_int as libc::c_uint;
                                }
                            }
                            if _hs_oomed != 0 {
                                let mut _hd_head_11: *mut UT_hash_bucket = &mut *((*(*_dst_hh)
                                    .tbl)
                                    .buckets)
                                    .offset(_dst_bkt as isize) as *mut UT_hash_bucket;
                                (*_hd_head_11)
                                    .count = ((*_hd_head_11).count).wrapping_sub(1);
                                (*_hd_head_11).count;
                                if (*_hd_head_11).hh_head == _dst_hh {
                                    (*_hd_head_11).hh_head = (*_dst_hh).hh_next;
                                }
                                if !((*_dst_hh).hh_prev).is_null() {
                                    (*(*_dst_hh).hh_prev).hh_next = (*_dst_hh).hh_next;
                                }
                                if !((*_dst_hh).hh_next).is_null() {
                                    (*(*_dst_hh).hh_next).hh_prev = (*_dst_hh).hh_prev;
                                }
                            }
                        }
                        (*(*users2).hh2.tbl)
                            .num_items = ((*(*users2).hh2.tbl).num_items)
                            .wrapping_add(1);
                        (*(*users2).hh2.tbl).num_items;
                        if _hs_oomed != 0 {
                            let mut _hd_hh_item_5: *mut UT_hash_handle = _dst_hh;
                            let mut _hd_bkt_11: libc::c_uint = 0;
                            _hd_bkt_11 = (*_hd_hh_item_5).hashv
                                & ((*(*users2).hh2.tbl).num_buckets)
                                    .wrapping_sub(1 as libc::c_uint);
                            let ref mut fresh18 = (*((*(*users2).hh2.tbl).buckets)
                                .offset(_hd_bkt_11 as isize))
                                .count;
                            *fresh18 = (*fresh18).wrapping_add(1);
                            *fresh18;
                            (*_hd_hh_item_5).hh_next = 0 as *mut UT_hash_handle;
                            (*_hd_hh_item_5).hh_prev = 0 as *mut UT_hash_handle;
                            let mut _hd_hh_del_5: *mut UT_hash_handle = _dst_hh;
                            if ((*_hd_hh_del_5).prev).is_null()
                                && ((*_hd_hh_del_5).next).is_null()
                            {
                                alt_free((*(*users2).hh2.tbl).buckets as *mut libc::c_void);
                                alt_free((*users2).hh2.tbl as *mut libc::c_void);
                                users2 = 0 as *mut example_user_t;
                            } else {
                                let mut _hd_bkt_12: libc::c_uint = 0;
                                if _hd_hh_del_5 == (*(*users2).hh2.tbl).tail {
                                    (*(*users2).hh2.tbl)
                                        .tail = ((*_hd_hh_del_5).prev as *mut libc::c_char)
                                        .offset((*(*users2).hh2.tbl).hho as isize)
                                        as *mut libc::c_void as *mut UT_hash_handle;
                                }
                                if !((*_hd_hh_del_5).prev).is_null() {
                                    let ref mut fresh19 = (*(((*_hd_hh_del_5).prev
                                        as *mut libc::c_char)
                                        .offset((*(*users2).hh2.tbl).hho as isize)
                                        as *mut libc::c_void as *mut UT_hash_handle))
                                        .next;
                                    *fresh19 = (*_hd_hh_del_5).next;
                                } else {
                                    users2 = (*_hd_hh_del_5).next as *mut example_user_t;
                                }
                                if !((*_hd_hh_del_5).next).is_null() {
                                    let ref mut fresh20 = (*(((*_hd_hh_del_5).next
                                        as *mut libc::c_char)
                                        .offset((*(*users2).hh2.tbl).hho as isize)
                                        as *mut libc::c_void as *mut UT_hash_handle))
                                        .prev;
                                    *fresh20 = (*_hd_hh_del_5).prev;
                                }
                                _hd_bkt_12 = (*_hd_hh_del_5).hashv
                                    & ((*(*users2).hh2.tbl).num_buckets)
                                        .wrapping_sub(1 as libc::c_uint);
                                let mut _hd_head_12: *mut UT_hash_bucket = &mut *((*(*users2)
                                    .hh2
                                    .tbl)
                                    .buckets)
                                    .offset(_hd_bkt_12 as isize) as *mut UT_hash_bucket;
                                (*_hd_head_12)
                                    .count = ((*_hd_head_12).count).wrapping_sub(1);
                                (*_hd_head_12).count;
                                if (*_hd_head_12).hh_head == _hd_hh_del_5 {
                                    (*_hd_head_12).hh_head = (*_hd_hh_del_5).hh_next;
                                }
                                if !((*_hd_hh_del_5).hh_prev).is_null() {
                                    (*(*_hd_hh_del_5).hh_prev)
                                        .hh_next = (*_hd_hh_del_5).hh_next;
                                }
                                if !((*_hd_hh_del_5).hh_next).is_null() {
                                    (*(*_hd_hh_del_5).hh_next)
                                        .hh_prev = (*_hd_hh_del_5).hh_prev;
                                }
                                (*(*users2).hh2.tbl)
                                    .num_items = ((*(*users2).hh2.tbl).num_items)
                                    .wrapping_sub(1);
                                (*(*users2).hh2.tbl).num_items;
                            }
                            (*_dst_hh).tbl = 0 as *mut UT_hash_table;
                            (*(_elt as *mut example_user_t))
                                .mem_failed = 1 as libc::c_int;
                        } else {
                            _last_elt = _elt;
                            _last_elt_hh = _dst_hh;
                        }
                    }
                    _ => {}
                }
                _src_hh = (*_src_hh).hh_next;
            }
            _src_bkt = _src_bkt.wrapping_add(1);
            _src_bkt;
        }
    }
    if !users2.is_null() {
        puts(
            b"Nothing should have been copied into users2\0" as *const u8
                as *const libc::c_char,
        );
    }
    user = users;
    test = (if !users.is_null() { (*users).hh.next } else { 0 as *mut libc::c_void })
        as *mut example_user_t;
    while !user.is_null() {
        if !((*user).hh2.tbl).is_null() {
            printf(
                b"User ID %d has tbl at %p\n\0" as *const u8 as *const libc::c_char,
                (*user).id,
                (*user).hh2.tbl as *mut libc::c_void,
            );
        }
        if (*user).mem_failed != 1 as libc::c_int {
            printf(
                b"User ID %d has mem_failed(%d), should be 1\n\0" as *const u8
                    as *const libc::c_char,
                (*user).id,
                (*user).mem_failed,
            );
        }
        (*user).mem_failed = 0 as libc::c_int;
        user = test;
        test = (if !test.is_null() { (*test).hh.next } else { 0 as *mut libc::c_void })
            as *mut example_user_t;
    }
    malloc_cnt = 4 as libc::c_int;
    let mut _src_bkt_0: libc::c_uint = 0;
    let mut _dst_bkt_0: libc::c_uint = 0;
    let mut _last_elt_0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _elt_0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _src_hh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _dst_hh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _last_elt_hh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _dst_hho_0: ptrdiff_t = (&mut (*users2).hh2 as *mut UT_hash_handle
        as *mut libc::c_char)
        .offset_from(users2 as *mut libc::c_char) as libc::c_long;
    if !users.is_null() {
        _src_bkt_0 = 0 as libc::c_int as libc::c_uint;
        while _src_bkt_0 < (*(*users).hh.tbl).num_buckets {
            let mut current_block_2360: u64;
            _src_hh_0 = (*((*(*users).hh.tbl).buckets).offset(_src_bkt_0 as isize))
                .hh_head;
            while !_src_hh_0.is_null() {
                _elt_0 = (_src_hh_0 as *mut libc::c_char)
                    .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
                let mut _hs_oomed_0: libc::c_int = 0 as libc::c_int;
                _dst_hh_0 = (_elt_0 as *mut libc::c_char).offset(_dst_hho_0 as isize)
                    as *mut libc::c_void as *mut UT_hash_handle;
                (*_dst_hh_0).key = (*_src_hh_0).key;
                (*_dst_hh_0).keylen = (*_src_hh_0).keylen;
                (*_dst_hh_0).hashv = (*_src_hh_0).hashv;
                (*_dst_hh_0).prev = _last_elt_0;
                (*_dst_hh_0).next = 0 as *mut libc::c_void;
                if !_last_elt_hh_0.is_null() {
                    (*_last_elt_hh_0).next = _elt_0;
                }
                if users2.is_null() {
                    users2 = _elt_0 as *mut example_user_t;
                    (*users2)
                        .hh2
                        .tbl = alt_malloc(
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    ) as *mut UT_hash_table;
                    if ((*users2).hh2.tbl).is_null() {
                        _hs_oomed_0 = 1 as libc::c_int;
                    } else {
                        memset(
                            (*users2).hh2.tbl as *mut libc::c_void,
                            '\0' as i32,
                            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                        );
                        (*(*users2).hh2.tbl).tail = &mut (*users2).hh2;
                        (*(*users2).hh2.tbl).num_buckets = 32 as libc::c_uint;
                        (*(*users2).hh2.tbl).log2_num_buckets = 5 as libc::c_uint;
                        (*(*users2).hh2.tbl)
                            .hho = (&mut (*users2).hh2 as *mut UT_hash_handle
                            as *mut libc::c_char)
                            .offset_from(users2 as *mut libc::c_char) as libc::c_long;
                        (*(*users2).hh2.tbl)
                            .buckets = alt_malloc(
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        ) as *mut UT_hash_bucket;
                        (*(*users2).hh2.tbl).signature = 0xa0111fe1 as libc::c_uint;
                        if ((*(*users2).hh2.tbl).buckets).is_null() {
                            _hs_oomed_0 = 1 as libc::c_int;
                            alt_free((*users2).hh2.tbl as *mut libc::c_void);
                        } else {
                            memset(
                                (*(*users2).hh2.tbl).buckets as *mut libc::c_void,
                                '\0' as i32,
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            );
                            if _hs_oomed_0 != 0 {
                                alt_free((*(*users2).hh2.tbl).buckets as *mut libc::c_void);
                                alt_free((*users2).hh2.tbl as *mut libc::c_void);
                            }
                        }
                    }
                    if _hs_oomed_0 != 0 {
                        (*(_elt_0 as *mut example_user_t)).mem_failed = 1 as libc::c_int;
                        users2 = 0 as *mut example_user_t;
                        current_block_2360 = 10699270605246059003;
                    } else {
                        current_block_2360 = 6299786575171263901;
                    }
                } else {
                    (*_dst_hh_0).tbl = (*users2).hh2.tbl;
                    current_block_2360 = 6299786575171263901;
                }
                match current_block_2360 {
                    6299786575171263901 => {
                        _dst_bkt_0 = (*_dst_hh_0).hashv
                            & ((*(*_dst_hh_0).tbl).num_buckets)
                                .wrapping_sub(1 as libc::c_uint);
                        let mut _ha_head_6: *mut UT_hash_bucket = &mut *((*(*_dst_hh_0)
                            .tbl)
                            .buckets)
                            .offset(_dst_bkt_0 as isize) as *mut UT_hash_bucket;
                        (*_ha_head_6).count = ((*_ha_head_6).count).wrapping_add(1);
                        (*_ha_head_6).count;
                        (*_dst_hh_0).hh_next = (*_ha_head_6).hh_head;
                        (*_dst_hh_0).hh_prev = 0 as *mut UT_hash_handle;
                        if !((*_ha_head_6).hh_head).is_null() {
                            (*(*_ha_head_6).hh_head).hh_prev = _dst_hh_0;
                        }
                        (*_ha_head_6).hh_head = _dst_hh_0;
                        if (*_ha_head_6).count
                            >= ((*_ha_head_6).expand_mult)
                                .wrapping_add(1 as libc::c_uint)
                                .wrapping_mul(10 as libc::c_uint)
                            && (*(*_dst_hh_0).tbl).noexpand == 0
                        {
                            let mut _he_bkt_6: libc::c_uint = 0;
                            let mut _he_bkt_i_6: libc::c_uint = 0;
                            let mut _he_thh_6: *mut UT_hash_handle = 0
                                as *mut UT_hash_handle;
                            let mut _he_hh_nxt_6: *mut UT_hash_handle = 0
                                as *mut UT_hash_handle;
                            let mut _he_new_buckets_6: *mut UT_hash_bucket = 0
                                as *mut UT_hash_bucket;
                            let mut _he_newbkt_6: *mut UT_hash_bucket = 0
                                as *mut UT_hash_bucket;
                            _he_new_buckets_6 = alt_malloc(
                                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                    .wrapping_mul(
                                        (*(*_dst_hh_0).tbl).num_buckets as libc::c_ulong,
                                    )
                                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                            ) as *mut UT_hash_bucket;
                            if _he_new_buckets_6.is_null() {
                                _hs_oomed_0 = 1 as libc::c_int;
                            } else {
                                memset(
                                    _he_new_buckets_6 as *mut libc::c_void,
                                    '\0' as i32,
                                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                        .wrapping_mul(
                                            (*(*_dst_hh_0).tbl).num_buckets as libc::c_ulong,
                                        )
                                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                                );
                                (*(*_dst_hh_0).tbl)
                                    .ideal_chain_maxlen = ((*(*_dst_hh_0).tbl).num_items
                                    >> ((*(*_dst_hh_0).tbl).log2_num_buckets)
                                        .wrapping_add(1 as libc::c_uint))
                                    .wrapping_add(
                                        (if (*(*_dst_hh_0).tbl).num_items
                                            & ((*(*_dst_hh_0).tbl).num_buckets)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                                        {
                                            1 as libc::c_uint
                                        } else {
                                            0 as libc::c_uint
                                        }),
                                    );
                                (*(*_dst_hh_0).tbl)
                                    .nonideal_items = 0 as libc::c_int as libc::c_uint;
                                _he_bkt_i_6 = 0 as libc::c_int as libc::c_uint;
                                while _he_bkt_i_6 < (*(*_dst_hh_0).tbl).num_buckets {
                                    _he_thh_6 = (*((*(*_dst_hh_0).tbl).buckets)
                                        .offset(_he_bkt_i_6 as isize))
                                        .hh_head;
                                    while !_he_thh_6.is_null() {
                                        _he_hh_nxt_6 = (*_he_thh_6).hh_next;
                                        _he_bkt_6 = (*_he_thh_6).hashv
                                            & ((*(*_dst_hh_0).tbl).num_buckets)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_sub(1 as libc::c_uint);
                                        _he_newbkt_6 = &mut *_he_new_buckets_6
                                            .offset(_he_bkt_6 as isize) as *mut UT_hash_bucket;
                                        (*_he_newbkt_6)
                                            .count = ((*_he_newbkt_6).count).wrapping_add(1);
                                        if (*_he_newbkt_6).count
                                            > (*(*_dst_hh_0).tbl).ideal_chain_maxlen
                                        {
                                            (*(*_dst_hh_0).tbl)
                                                .nonideal_items = ((*(*_dst_hh_0).tbl).nonideal_items)
                                                .wrapping_add(1);
                                            (*(*_dst_hh_0).tbl).nonideal_items;
                                            if (*_he_newbkt_6).count
                                                > ((*_he_newbkt_6).expand_mult)
                                                    .wrapping_mul((*(*_dst_hh_0).tbl).ideal_chain_maxlen)
                                            {
                                                (*_he_newbkt_6)
                                                    .expand_mult = ((*_he_newbkt_6).expand_mult)
                                                    .wrapping_add(1);
                                                (*_he_newbkt_6).expand_mult;
                                            }
                                        }
                                        (*_he_thh_6).hh_prev = 0 as *mut UT_hash_handle;
                                        (*_he_thh_6).hh_next = (*_he_newbkt_6).hh_head;
                                        if !((*_he_newbkt_6).hh_head).is_null() {
                                            (*(*_he_newbkt_6).hh_head).hh_prev = _he_thh_6;
                                        }
                                        (*_he_newbkt_6).hh_head = _he_thh_6;
                                        _he_thh_6 = _he_hh_nxt_6;
                                    }
                                    _he_bkt_i_6 = _he_bkt_i_6.wrapping_add(1);
                                    _he_bkt_i_6;
                                }
                                alt_free((*(*_dst_hh_0).tbl).buckets as *mut libc::c_void);
                                (*(*_dst_hh_0).tbl)
                                    .num_buckets = ((*(*_dst_hh_0).tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint);
                                (*(*_dst_hh_0).tbl)
                                    .log2_num_buckets = ((*(*_dst_hh_0).tbl).log2_num_buckets)
                                    .wrapping_add(1);
                                (*(*_dst_hh_0).tbl).log2_num_buckets;
                                (*(*_dst_hh_0).tbl).buckets = _he_new_buckets_6;
                                (*(*_dst_hh_0).tbl)
                                    .ineff_expands = if (*(*_dst_hh_0).tbl).nonideal_items
                                    > (*(*_dst_hh_0).tbl).num_items >> 1 as libc::c_int
                                {
                                    ((*(*_dst_hh_0).tbl).ineff_expands)
                                        .wrapping_add(1 as libc::c_uint)
                                } else {
                                    0 as libc::c_uint
                                };
                                if (*(*_dst_hh_0).tbl).ineff_expands > 1 as libc::c_uint {
                                    (*(*_dst_hh_0).tbl)
                                        .noexpand = 1 as libc::c_int as libc::c_uint;
                                }
                            }
                            if _hs_oomed_0 != 0 {
                                let mut _hd_head_13: *mut UT_hash_bucket = &mut *((*(*_dst_hh_0)
                                    .tbl)
                                    .buckets)
                                    .offset(_dst_bkt_0 as isize) as *mut UT_hash_bucket;
                                (*_hd_head_13)
                                    .count = ((*_hd_head_13).count).wrapping_sub(1);
                                (*_hd_head_13).count;
                                if (*_hd_head_13).hh_head == _dst_hh_0 {
                                    (*_hd_head_13).hh_head = (*_dst_hh_0).hh_next;
                                }
                                if !((*_dst_hh_0).hh_prev).is_null() {
                                    (*(*_dst_hh_0).hh_prev).hh_next = (*_dst_hh_0).hh_next;
                                }
                                if !((*_dst_hh_0).hh_next).is_null() {
                                    (*(*_dst_hh_0).hh_next).hh_prev = (*_dst_hh_0).hh_prev;
                                }
                            }
                        }
                        (*(*users2).hh2.tbl)
                            .num_items = ((*(*users2).hh2.tbl).num_items)
                            .wrapping_add(1);
                        (*(*users2).hh2.tbl).num_items;
                        if _hs_oomed_0 != 0 {
                            let mut _hd_hh_item_6: *mut UT_hash_handle = _dst_hh_0;
                            let mut _hd_bkt_13: libc::c_uint = 0;
                            _hd_bkt_13 = (*_hd_hh_item_6).hashv
                                & ((*(*users2).hh2.tbl).num_buckets)
                                    .wrapping_sub(1 as libc::c_uint);
                            let ref mut fresh21 = (*((*(*users2).hh2.tbl).buckets)
                                .offset(_hd_bkt_13 as isize))
                                .count;
                            *fresh21 = (*fresh21).wrapping_add(1);
                            *fresh21;
                            (*_hd_hh_item_6).hh_next = 0 as *mut UT_hash_handle;
                            (*_hd_hh_item_6).hh_prev = 0 as *mut UT_hash_handle;
                            let mut _hd_hh_del_6: *mut UT_hash_handle = _dst_hh_0;
                            if ((*_hd_hh_del_6).prev).is_null()
                                && ((*_hd_hh_del_6).next).is_null()
                            {
                                alt_free((*(*users2).hh2.tbl).buckets as *mut libc::c_void);
                                alt_free((*users2).hh2.tbl as *mut libc::c_void);
                                users2 = 0 as *mut example_user_t;
                            } else {
                                let mut _hd_bkt_14: libc::c_uint = 0;
                                if _hd_hh_del_6 == (*(*users2).hh2.tbl).tail {
                                    (*(*users2).hh2.tbl)
                                        .tail = ((*_hd_hh_del_6).prev as *mut libc::c_char)
                                        .offset((*(*users2).hh2.tbl).hho as isize)
                                        as *mut libc::c_void as *mut UT_hash_handle;
                                }
                                if !((*_hd_hh_del_6).prev).is_null() {
                                    let ref mut fresh22 = (*(((*_hd_hh_del_6).prev
                                        as *mut libc::c_char)
                                        .offset((*(*users2).hh2.tbl).hho as isize)
                                        as *mut libc::c_void as *mut UT_hash_handle))
                                        .next;
                                    *fresh22 = (*_hd_hh_del_6).next;
                                } else {
                                    users2 = (*_hd_hh_del_6).next as *mut example_user_t;
                                }
                                if !((*_hd_hh_del_6).next).is_null() {
                                    let ref mut fresh23 = (*(((*_hd_hh_del_6).next
                                        as *mut libc::c_char)
                                        .offset((*(*users2).hh2.tbl).hho as isize)
                                        as *mut libc::c_void as *mut UT_hash_handle))
                                        .prev;
                                    *fresh23 = (*_hd_hh_del_6).prev;
                                }
                                _hd_bkt_14 = (*_hd_hh_del_6).hashv
                                    & ((*(*users2).hh2.tbl).num_buckets)
                                        .wrapping_sub(1 as libc::c_uint);
                                let mut _hd_head_14: *mut UT_hash_bucket = &mut *((*(*users2)
                                    .hh2
                                    .tbl)
                                    .buckets)
                                    .offset(_hd_bkt_14 as isize) as *mut UT_hash_bucket;
                                (*_hd_head_14)
                                    .count = ((*_hd_head_14).count).wrapping_sub(1);
                                (*_hd_head_14).count;
                                if (*_hd_head_14).hh_head == _hd_hh_del_6 {
                                    (*_hd_head_14).hh_head = (*_hd_hh_del_6).hh_next;
                                }
                                if !((*_hd_hh_del_6).hh_prev).is_null() {
                                    (*(*_hd_hh_del_6).hh_prev)
                                        .hh_next = (*_hd_hh_del_6).hh_next;
                                }
                                if !((*_hd_hh_del_6).hh_next).is_null() {
                                    (*(*_hd_hh_del_6).hh_next)
                                        .hh_prev = (*_hd_hh_del_6).hh_prev;
                                }
                                (*(*users2).hh2.tbl)
                                    .num_items = ((*(*users2).hh2.tbl).num_items)
                                    .wrapping_sub(1);
                                (*(*users2).hh2.tbl).num_items;
                            }
                            (*_dst_hh_0).tbl = 0 as *mut UT_hash_table;
                            (*(_elt_0 as *mut example_user_t))
                                .mem_failed = 1 as libc::c_int;
                        } else {
                            _last_elt_0 = _elt_0;
                            _last_elt_hh_0 = _dst_hh_0;
                        }
                    }
                    _ => {}
                }
                _src_hh_0 = (*_src_hh_0).hh_next;
            }
            _src_bkt_0 = _src_bkt_0.wrapping_add(1);
            _src_bkt_0;
        }
    }
    saved_cnt = 0 as libc::c_int;
    user = users;
    test = (if !users.is_null() { (*users).hh.next } else { 0 as *mut libc::c_void })
        as *mut example_user_t;
    while !user.is_null() {
        let mut user2: *mut example_user_t = 0 as *mut example_user_t;
        user2 = 0 as *mut example_user_t;
        if !users2.is_null() {
            let mut _hf_hashv_1: libc::c_uint = 0;
            let mut _hj_i_7: libc::c_uint = 0;
            let mut _hj_j_7: libc::c_uint = 0;
            let mut _hj_k_7: libc::c_uint = 0;
            let mut _hj_key_7: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
                as *const libc::c_uchar;
            _hf_hashv_1 = 0xfeedbeef as libc::c_uint;
            _hj_j_7 = 0x9e3779b9 as libc::c_uint;
            _hj_i_7 = _hj_j_7;
            _hj_k_7 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            while _hj_k_7 >= 12 as libc::c_uint {
                _hj_i_7 = _hj_i_7
                    .wrapping_add(
                        (*_hj_key_7.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_7.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_7.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_7.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_j_7 = _hj_j_7
                    .wrapping_add(
                        (*_hj_key_7.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_7.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_7.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_7.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_7.offset(8 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_7.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_7.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_7.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_i_7 = _hj_i_7.wrapping_sub(_hj_j_7);
                _hj_i_7 = _hj_i_7.wrapping_sub(_hf_hashv_1);
                _hj_i_7 ^= _hf_hashv_1 >> 13 as libc::c_int;
                _hj_j_7 = _hj_j_7.wrapping_sub(_hf_hashv_1);
                _hj_j_7 = _hj_j_7.wrapping_sub(_hj_i_7);
                _hj_j_7 ^= _hj_i_7 << 8 as libc::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_7);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_7);
                _hf_hashv_1 ^= _hj_j_7 >> 13 as libc::c_int;
                _hj_i_7 = _hj_i_7.wrapping_sub(_hj_j_7);
                _hj_i_7 = _hj_i_7.wrapping_sub(_hf_hashv_1);
                _hj_i_7 ^= _hf_hashv_1 >> 12 as libc::c_int;
                _hj_j_7 = _hj_j_7.wrapping_sub(_hf_hashv_1);
                _hj_j_7 = _hj_j_7.wrapping_sub(_hj_i_7);
                _hj_j_7 ^= _hj_i_7 << 16 as libc::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_7);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_7);
                _hf_hashv_1 ^= _hj_j_7 >> 5 as libc::c_int;
                _hj_i_7 = _hj_i_7.wrapping_sub(_hj_j_7);
                _hj_i_7 = _hj_i_7.wrapping_sub(_hf_hashv_1);
                _hj_i_7 ^= _hf_hashv_1 >> 3 as libc::c_int;
                _hj_j_7 = _hj_j_7.wrapping_sub(_hf_hashv_1);
                _hj_j_7 = _hj_j_7.wrapping_sub(_hj_i_7);
                _hj_j_7 ^= _hj_i_7 << 10 as libc::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_7);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_7);
                _hf_hashv_1 ^= _hj_j_7 >> 15 as libc::c_int;
                _hj_key_7 = _hj_key_7.offset(12 as libc::c_int as isize);
                _hj_k_7 = _hj_k_7.wrapping_sub(12 as libc::c_uint);
            }
            _hf_hashv_1 = _hf_hashv_1
                .wrapping_add(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
                );
            let mut current_block_2423: u64;
            match _hj_k_7 {
                11 => {
                    _hf_hashv_1 = _hf_hashv_1
                        .wrapping_add(
                            (*_hj_key_7.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_2423 = 14152827358935435035;
                }
                10 => {
                    current_block_2423 = 14152827358935435035;
                }
                9 => {
                    current_block_2423 = 12205017671613270583;
                }
                8 => {
                    current_block_2423 = 6770129144439005619;
                }
                7 => {
                    current_block_2423 = 1133543008428547523;
                }
                6 => {
                    current_block_2423 = 8462258133321559388;
                }
                5 => {
                    current_block_2423 = 3846739063131686548;
                }
                4 => {
                    current_block_2423 = 5475236905426780108;
                }
                3 => {
                    current_block_2423 = 5932193673735597124;
                }
                2 => {
                    current_block_2423 = 10451395258337508939;
                }
                1 => {
                    current_block_2423 = 3076503085047436559;
                }
                _ => {
                    current_block_2423 = 7710573646835651295;
                }
            }
            match current_block_2423 {
                14152827358935435035 => {
                    _hf_hashv_1 = _hf_hashv_1
                        .wrapping_add(
                            (*_hj_key_7.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_2423 = 12205017671613270583;
                }
                _ => {}
            }
            match current_block_2423 {
                12205017671613270583 => {
                    _hf_hashv_1 = _hf_hashv_1
                        .wrapping_add(
                            (*_hj_key_7.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_2423 = 6770129144439005619;
                }
                _ => {}
            }
            match current_block_2423 {
                6770129144439005619 => {
                    _hj_j_7 = _hj_j_7
                        .wrapping_add(
                            (*_hj_key_7.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_2423 = 1133543008428547523;
                }
                _ => {}
            }
            match current_block_2423 {
                1133543008428547523 => {
                    _hj_j_7 = _hj_j_7
                        .wrapping_add(
                            (*_hj_key_7.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_2423 = 8462258133321559388;
                }
                _ => {}
            }
            match current_block_2423 {
                8462258133321559388 => {
                    _hj_j_7 = _hj_j_7
                        .wrapping_add(
                            (*_hj_key_7.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_2423 = 3846739063131686548;
                }
                _ => {}
            }
            match current_block_2423 {
                3846739063131686548 => {
                    _hj_j_7 = _hj_j_7
                        .wrapping_add(
                            *_hj_key_7.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_2423 = 5475236905426780108;
                }
                _ => {}
            }
            match current_block_2423 {
                5475236905426780108 => {
                    _hj_i_7 = _hj_i_7
                        .wrapping_add(
                            (*_hj_key_7.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_2423 = 5932193673735597124;
                }
                _ => {}
            }
            match current_block_2423 {
                5932193673735597124 => {
                    _hj_i_7 = _hj_i_7
                        .wrapping_add(
                            (*_hj_key_7.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_2423 = 10451395258337508939;
                }
                _ => {}
            }
            match current_block_2423 {
                10451395258337508939 => {
                    _hj_i_7 = _hj_i_7
                        .wrapping_add(
                            (*_hj_key_7.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_2423 = 3076503085047436559;
                }
                _ => {}
            }
            match current_block_2423 {
                3076503085047436559 => {
                    _hj_i_7 = _hj_i_7
                        .wrapping_add(
                            *_hj_key_7.offset(0 as libc::c_int as isize) as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i_7 = _hj_i_7.wrapping_sub(_hj_j_7);
            _hj_i_7 = _hj_i_7.wrapping_sub(_hf_hashv_1);
            _hj_i_7 ^= _hf_hashv_1 >> 13 as libc::c_int;
            _hj_j_7 = _hj_j_7.wrapping_sub(_hf_hashv_1);
            _hj_j_7 = _hj_j_7.wrapping_sub(_hj_i_7);
            _hj_j_7 ^= _hj_i_7 << 8 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_7);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_7);
            _hf_hashv_1 ^= _hj_j_7 >> 13 as libc::c_int;
            _hj_i_7 = _hj_i_7.wrapping_sub(_hj_j_7);
            _hj_i_7 = _hj_i_7.wrapping_sub(_hf_hashv_1);
            _hj_i_7 ^= _hf_hashv_1 >> 12 as libc::c_int;
            _hj_j_7 = _hj_j_7.wrapping_sub(_hf_hashv_1);
            _hj_j_7 = _hj_j_7.wrapping_sub(_hj_i_7);
            _hj_j_7 ^= _hj_i_7 << 16 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_7);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_7);
            _hf_hashv_1 ^= _hj_j_7 >> 5 as libc::c_int;
            _hj_i_7 = _hj_i_7.wrapping_sub(_hj_j_7);
            _hj_i_7 = _hj_i_7.wrapping_sub(_hf_hashv_1);
            _hj_i_7 ^= _hf_hashv_1 >> 3 as libc::c_int;
            _hj_j_7 = _hj_j_7.wrapping_sub(_hf_hashv_1);
            _hj_j_7 = _hj_j_7.wrapping_sub(_hj_i_7);
            _hj_j_7 ^= _hj_i_7 << 10 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_7);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_7);
            _hf_hashv_1 ^= _hj_j_7 >> 15 as libc::c_int;
            user2 = 0 as *mut example_user_t;
            if !users2.is_null() {
                let mut _hf_bkt_1: libc::c_uint = 0;
                _hf_bkt_1 = _hf_hashv_1
                    & ((*(*users2).hh2.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                if 1 as libc::c_int != 0 as libc::c_int {
                    if !((*((*(*users2).hh2.tbl).buckets).offset(_hf_bkt_1 as isize))
                        .hh_head)
                        .is_null()
                    {
                        user2 = ((*((*(*users2).hh2.tbl).buckets)
                            .offset(_hf_bkt_1 as isize))
                            .hh_head as *mut libc::c_char)
                            .offset(-((*(*users2).hh2.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        user2 = 0 as *mut example_user_t;
                    }
                    while !user2.is_null() {
                        if (*user2).hh2.hashv == _hf_hashv_1
                            && (*user2).hh2.keylen as libc::c_ulong
                                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            if memcmp(
                                (*user2).hh2.key,
                                &mut (*user).id as *mut libc::c_int as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                break;
                            }
                        }
                        if !((*user2).hh2.hh_next).is_null() {
                            user2 = ((*user2).hh2.hh_next as *mut libc::c_char)
                                .offset(-((*(*users2).hh2.tbl).hho as isize))
                                as *mut libc::c_void as *mut example_user_t;
                        } else {
                            user2 = 0 as *mut example_user_t;
                        }
                    }
                }
            }
        }
        if !user2.is_null() {
            if ((*user).hh2.tbl).is_null() {
                printf(
                    b"User ID %d has tbl==NULL\n\0" as *const u8 as *const libc::c_char,
                    (*user).id,
                );
            }
            if (*user).mem_failed != 0 as libc::c_int {
                printf(
                    b"User ID %d has mem_failed(%d), expected 0\n\0" as *const u8
                        as *const libc::c_char,
                    (*user).id,
                    (*user).mem_failed,
                );
            }
        } else {
            saved_cnt += 1;
            saved_cnt;
            if !((*user).hh2.tbl).is_null() {
                printf(
                    b"User ID %d has tbl at %p, expected 0\n\0" as *const u8
                        as *const libc::c_char,
                    (*user).id,
                    (*user).hh2.tbl as *mut libc::c_void,
                );
            }
            if (*user).mem_failed != 1 as libc::c_int {
                printf(
                    b"User ID %d has mem_failed(%d), expected is 1\n\0" as *const u8
                        as *const libc::c_char,
                    (*user).id,
                    (*user).mem_failed,
                );
            }
        }
        user = test;
        test = (if !test.is_null() { (*test).hh.next } else { 0 as *mut libc::c_void })
            as *mut example_user_t;
    }
    if (saved_cnt as libc::c_uint)
        .wrapping_add(
            (if !users2.is_null() {
                (*(*users2).hh2.tbl).num_items
            } else {
                0 as libc::c_uint
            }),
        )
        != (if !users.is_null() {
            (*(*users).hh.tbl).num_items
        } else {
            0 as libc::c_uint
        })
    {
        printf(
            b"Selected elements : %d + %d != %d\n\0" as *const u8 as *const libc::c_char,
            saved_cnt,
            if !users2.is_null() {
                (*(*users2).hh2.tbl).num_items
            } else {
                0 as libc::c_uint
            },
            if !users.is_null() {
                (*(*users).hh.tbl).num_items
            } else {
                0 as libc::c_uint
            },
        );
    }
    puts(b"End\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

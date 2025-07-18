use ::libc;
extern "C" {
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
pub struct example_user_t {
    pub id: libc::c_int,
    pub hh: UT_hash_handle,
    pub ah: UT_hash_handle,
}
unsafe extern "C" fn evens(mut userv: *mut libc::c_void) -> libc::c_int {
    let mut user: *mut example_user_t = userv as *mut example_user_t;
    return if (*user).id % 2 as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn idcmp(
    mut _a: *mut libc::c_void,
    mut _b: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut example_user_t = _a as *mut example_user_t;
    let mut b: *mut example_user_t = _b as *mut example_user_t;
    return (*a).id - (*b).id;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut user: *mut example_user_t = 0 as *mut example_user_t;
    let mut users: *mut example_user_t = 0 as *mut example_user_t;
    let mut ausers: *mut example_user_t = 0 as *mut example_user_t;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        if user.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*user).id = i;
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
        let mut current_block_57: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_57 = 13337910766845200452;
            }
            10 => {
                current_block_57 = 13337910766845200452;
            }
            9 => {
                current_block_57 = 8638925518304655853;
            }
            8 => {
                current_block_57 = 9135797479030497140;
            }
            7 => {
                current_block_57 = 13283050878216410150;
            }
            6 => {
                current_block_57 = 521867529032381887;
            }
            5 => {
                current_block_57 = 1707055309309379042;
            }
            4 => {
                current_block_57 = 10980792033740183381;
            }
            3 => {
                current_block_57 = 7149172411463922887;
            }
            2 => {
                current_block_57 = 12631348047353356;
            }
            1 => {
                current_block_57 = 8739749648707081367;
            }
            _ => {
                current_block_57 = 12497913735442871383;
            }
        }
        match current_block_57 {
            13337910766845200452 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_57 = 8638925518304655853;
            }
            _ => {}
        }
        match current_block_57 {
            8638925518304655853 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_57 = 9135797479030497140;
            }
            _ => {}
        }
        match current_block_57 {
            9135797479030497140 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_57 = 13283050878216410150;
            }
            _ => {}
        }
        match current_block_57 {
            13283050878216410150 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_57 = 521867529032381887;
            }
            _ => {}
        }
        match current_block_57 {
            521867529032381887 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_57 = 1707055309309379042;
            }
            _ => {}
        }
        match current_block_57 {
            1707055309309379042 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_57 = 10980792033740183381;
            }
            _ => {}
        }
        match current_block_57 {
            10980792033740183381 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_57 = 7149172411463922887;
            }
            _ => {}
        }
        match current_block_57 {
            7149172411463922887 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_57 = 12631348047353356;
            }
            _ => {}
        }
        match current_block_57 {
            12631348047353356 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_57 = 8739749648707081367;
            }
            _ => {}
        }
        match current_block_57 {
            8739749648707081367 => {
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
        (*user).hh.hashv = _ha_hashv;
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
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*user).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
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
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*user).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*user).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            users = user;
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
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
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
                free((*(*user).hh.tbl).buckets as *mut libc::c_void);
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
        }
        i += 1;
        i;
    }
    user = users;
    while !user.is_null() {
        printf(b"user %d\n\0" as *const u8 as *const libc::c_char, (*user).id);
        user = (*user).hh.next as *mut example_user_t;
    }
    let mut _src_bkt: libc::c_uint = 0;
    let mut _dst_bkt: libc::c_uint = 0;
    let mut _last_elt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _elt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _src_hh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _dst_hh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _last_elt_hh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _dst_hho: ptrdiff_t = (&mut (*ausers).ah as *mut UT_hash_handle
        as *mut libc::c_char)
        .offset_from(ausers as *mut libc::c_char) as libc::c_long;
    if !users.is_null() {
        _src_bkt = 0 as libc::c_int as libc::c_uint;
        while _src_bkt < (*(*users).hh.tbl).num_buckets {
            _src_hh = (*((*(*users).hh.tbl).buckets).offset(_src_bkt as isize)).hh_head;
            while !_src_hh.is_null() {
                _elt = (_src_hh as *mut libc::c_char)
                    .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
                if evens(_elt) != 0 {
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
                    if ausers.is_null() {
                        ausers = _elt as *mut example_user_t;
                        (*ausers)
                            .ah
                            .tbl = malloc(
                            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                        ) as *mut UT_hash_table;
                        if ((*ausers).ah.tbl).is_null() {
                            exit(-(1 as libc::c_int));
                        } else {
                            memset(
                                (*ausers).ah.tbl as *mut libc::c_void,
                                '\0' as i32,
                                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                            );
                            (*(*ausers).ah.tbl).tail = &mut (*ausers).ah;
                            (*(*ausers).ah.tbl).num_buckets = 32 as libc::c_uint;
                            (*(*ausers).ah.tbl).log2_num_buckets = 5 as libc::c_uint;
                            (*(*ausers).ah.tbl)
                                .hho = (&mut (*ausers).ah as *mut UT_hash_handle
                                as *mut libc::c_char)
                                .offset_from(ausers as *mut libc::c_char) as libc::c_long;
                            (*(*ausers).ah.tbl)
                                .buckets = malloc(
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            ) as *mut UT_hash_bucket;
                            (*(*ausers).ah.tbl).signature = 0xa0111fe1 as libc::c_uint;
                            if ((*(*ausers).ah.tbl).buckets).is_null() {
                                exit(-(1 as libc::c_int));
                            } else {
                                memset(
                                    (*(*ausers).ah.tbl).buckets as *mut libc::c_void,
                                    '\0' as i32,
                                    (32 as libc::c_uint as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                        ),
                                );
                            }
                        }
                    } else {
                        (*_dst_hh).tbl = (*ausers).ah.tbl;
                    }
                    _dst_bkt = (*_dst_hh).hashv
                        & ((*(*_dst_hh).tbl).num_buckets)
                            .wrapping_sub(1 as libc::c_uint);
                    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*_dst_hh).tbl)
                        .buckets)
                        .offset(_dst_bkt as isize) as *mut UT_hash_bucket;
                    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
                    (*_ha_head_0).count;
                    (*_dst_hh).hh_next = (*_ha_head_0).hh_head;
                    (*_dst_hh).hh_prev = 0 as *mut UT_hash_handle;
                    if !((*_ha_head_0).hh_head).is_null() {
                        (*(*_ha_head_0).hh_head).hh_prev = _dst_hh;
                    }
                    (*_ha_head_0).hh_head = _dst_hh;
                    if (*_ha_head_0).count
                        >= ((*_ha_head_0).expand_mult)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_mul(10 as libc::c_uint)
                        && (*(*_dst_hh).tbl).noexpand == 0
                    {
                        let mut _he_bkt_0: libc::c_uint = 0;
                        let mut _he_bkt_i_0: libc::c_uint = 0;
                        let mut _he_thh_0: *mut UT_hash_handle = 0
                            as *mut UT_hash_handle;
                        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0
                            as *mut UT_hash_handle;
                        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0
                            as *mut UT_hash_bucket;
                        let mut _he_newbkt_0: *mut UT_hash_bucket = 0
                            as *mut UT_hash_bucket;
                        _he_new_buckets_0 = malloc(
                            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                .wrapping_mul(
                                    (*(*_dst_hh).tbl).num_buckets as libc::c_ulong,
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
                            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
                            while _he_bkt_i_0 < (*(*_dst_hh).tbl).num_buckets {
                                _he_thh_0 = (*((*(*_dst_hh).tbl).buckets)
                                    .offset(_he_bkt_i_0 as isize))
                                    .hh_head;
                                while !_he_thh_0.is_null() {
                                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                                    _he_bkt_0 = (*_he_thh_0).hashv
                                        & ((*(*_dst_hh).tbl).num_buckets)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_uint);
                                    _he_newbkt_0 = &mut *_he_new_buckets_0
                                        .offset(_he_bkt_0 as isize) as *mut UT_hash_bucket;
                                    (*_he_newbkt_0)
                                        .count = ((*_he_newbkt_0).count).wrapping_add(1);
                                    if (*_he_newbkt_0).count
                                        > (*(*_dst_hh).tbl).ideal_chain_maxlen
                                    {
                                        (*(*_dst_hh).tbl)
                                            .nonideal_items = ((*(*_dst_hh).tbl).nonideal_items)
                                            .wrapping_add(1);
                                        (*(*_dst_hh).tbl).nonideal_items;
                                        if (*_he_newbkt_0).count
                                            > ((*_he_newbkt_0).expand_mult)
                                                .wrapping_mul((*(*_dst_hh).tbl).ideal_chain_maxlen)
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
                            free((*(*_dst_hh).tbl).buckets as *mut libc::c_void);
                            (*(*_dst_hh).tbl)
                                .num_buckets = ((*(*_dst_hh).tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint);
                            (*(*_dst_hh).tbl)
                                .log2_num_buckets = ((*(*_dst_hh).tbl).log2_num_buckets)
                                .wrapping_add(1);
                            (*(*_dst_hh).tbl).log2_num_buckets;
                            (*(*_dst_hh).tbl).buckets = _he_new_buckets_0;
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
                    }
                    (*(*ausers).ah.tbl)
                        .num_items = ((*(*ausers).ah.tbl).num_items).wrapping_add(1);
                    (*(*ausers).ah.tbl).num_items;
                    _last_elt = _elt;
                    _last_elt_hh = _dst_hh;
                }
                _src_hh = (*_src_hh).hh_next;
            }
            _src_bkt = _src_bkt.wrapping_add(1);
            _src_bkt;
        }
    }
    let mut _hs_i: libc::c_uint = 0;
    let mut _hs_looping: libc::c_uint = 0;
    let mut _hs_nmerges: libc::c_uint = 0;
    let mut _hs_insize: libc::c_uint = 0;
    let mut _hs_psize: libc::c_uint = 0;
    let mut _hs_qsize: libc::c_uint = 0;
    let mut _hs_p: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_q: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_e: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_list: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_tail: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    if !ausers.is_null() {
        _hs_insize = 1 as libc::c_int as libc::c_uint;
        _hs_looping = 1 as libc::c_int as libc::c_uint;
        _hs_list = &mut (*ausers).ah;
        while _hs_looping != 0 as libc::c_uint {
            _hs_p = _hs_list;
            _hs_list = 0 as *mut UT_hash_handle;
            _hs_tail = 0 as *mut UT_hash_handle;
            _hs_nmerges = 0 as libc::c_int as libc::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_nmerges;
                _hs_q = _hs_p;
                _hs_psize = 0 as libc::c_int as libc::c_uint;
                _hs_i = 0 as libc::c_int as libc::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_psize;
                    _hs_q = if !((*_hs_q).next).is_null() {
                        ((*_hs_q).next as *mut libc::c_char)
                            .offset((*(*ausers).ah.tbl).hho as isize)
                            as *mut libc::c_void as *mut UT_hash_handle
                    } else {
                        0 as *mut UT_hash_handle
                    };
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                    _hs_i;
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as libc::c_uint
                    || _hs_qsize != 0 as libc::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as libc::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = if !((*_hs_q).next).is_null() {
                            ((*_hs_q).next as *mut libc::c_char)
                                .offset((*(*ausers).ah.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                        _hs_qsize;
                    } else if _hs_qsize == 0 as libc::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = if !((*_hs_p).next).is_null() {
                                ((*_hs_p).next as *mut libc::c_char)
                                    .offset((*(*ausers).ah.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                        _hs_psize;
                    } else if idcmp(
                        (_hs_p as *mut libc::c_char)
                            .offset(-((*(*ausers).ah.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t
                            as *mut libc::c_void,
                        (_hs_q as *mut libc::c_char)
                            .offset(-((*(*ausers).ah.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t
                            as *mut libc::c_void,
                    ) <= 0 as libc::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = if !((*_hs_p).next).is_null() {
                                ((*_hs_p).next as *mut libc::c_char)
                                    .offset((*(*ausers).ah.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                        _hs_psize;
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = if !((*_hs_q).next).is_null() {
                            ((*_hs_q).next as *mut libc::c_char)
                                .offset((*(*ausers).ah.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                        _hs_qsize;
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail)
                            .next = if !_hs_e.is_null() {
                            (_hs_e as *mut libc::c_char)
                                .offset(-((*(*ausers).ah.tbl).hho as isize))
                                as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e)
                            .prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut libc::c_char)
                                .offset(-((*(*ausers).ah.tbl).hho as isize))
                                as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = 0 as *mut libc::c_void;
            }
            if _hs_nmerges <= 1 as libc::c_uint {
                _hs_looping = 0 as libc::c_int as libc::c_uint;
                (*(*ausers).ah.tbl).tail = _hs_tail;
                ausers = (_hs_list as *mut libc::c_char)
                    .offset(-((*(*ausers).ah.tbl).hho as isize)) as *mut libc::c_void
                    as *mut example_user_t;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as libc::c_uint);
        }
    }
    user = users;
    while !user.is_null() {
        let mut found: *mut example_user_t = 0 as *mut example_user_t;
        let mut should_find: libc::c_int = (evens(user as *mut libc::c_void) != 0)
            as libc::c_int;
        found = 0 as *mut example_user_t;
        if !ausers.is_null() {
            let mut _hf_hashv: libc::c_uint = 0;
            let mut _hj_i_0: libc::c_uint = 0;
            let mut _hj_j_0: libc::c_uint = 0;
            let mut _hj_k_0: libc::c_uint = 0;
            let mut _hj_key_0: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
                as *const libc::c_uchar;
            _hf_hashv = 0xfeedbeef as libc::c_uint;
            _hj_j_0 = 0x9e3779b9 as libc::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
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
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
                );
            let mut current_block_439: u64;
            match _hj_k_0 {
                11 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_439 = 2767518686136159550;
                }
                10 => {
                    current_block_439 = 2767518686136159550;
                }
                9 => {
                    current_block_439 = 8400996634817571718;
                }
                8 => {
                    current_block_439 = 10057186977503183797;
                }
                7 => {
                    current_block_439 = 11928884684822391877;
                }
                6 => {
                    current_block_439 = 562533387689367468;
                }
                5 => {
                    current_block_439 = 12340929100992884138;
                }
                4 => {
                    current_block_439 = 5635111002786020586;
                }
                3 => {
                    current_block_439 = 5820000367945469958;
                }
                2 => {
                    current_block_439 = 18393791430963133800;
                }
                1 => {
                    current_block_439 = 16670538605579187138;
                }
                _ => {
                    current_block_439 = 17581461676304123093;
                }
            }
            match current_block_439 {
                2767518686136159550 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_439 = 8400996634817571718;
                }
                _ => {}
            }
            match current_block_439 {
                8400996634817571718 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_439 = 10057186977503183797;
                }
                _ => {}
            }
            match current_block_439 {
                10057186977503183797 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_439 = 11928884684822391877;
                }
                _ => {}
            }
            match current_block_439 {
                11928884684822391877 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_439 = 562533387689367468;
                }
                _ => {}
            }
            match current_block_439 {
                562533387689367468 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_439 = 12340929100992884138;
                }
                _ => {}
            }
            match current_block_439 {
                12340929100992884138 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_439 = 5635111002786020586;
                }
                _ => {}
            }
            match current_block_439 {
                5635111002786020586 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_439 = 5820000367945469958;
                }
                _ => {}
            }
            match current_block_439 {
                5820000367945469958 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_439 = 18393791430963133800;
                }
                _ => {}
            }
            match current_block_439 {
                18393791430963133800 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_439 = 16670538605579187138;
                }
                _ => {}
            }
            match current_block_439 {
                16670538605579187138 => {
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
            found = 0 as *mut example_user_t;
            if !ausers.is_null() {
                let mut _hf_bkt: libc::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & ((*(*ausers).ah.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                if 1 as libc::c_int != 0 as libc::c_int {
                    if !((*((*(*ausers).ah.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head)
                        .is_null()
                    {
                        found = ((*((*(*ausers).ah.tbl).buckets)
                            .offset(_hf_bkt as isize))
                            .hh_head as *mut libc::c_char)
                            .offset(-((*(*ausers).ah.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        found = 0 as *mut example_user_t;
                    }
                    while !found.is_null() {
                        if (*found).ah.hashv == _hf_hashv
                            && (*found).ah.keylen as libc::c_ulong
                                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            if memcmp(
                                (*found).ah.key,
                                &mut (*user).id as *mut libc::c_int as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                break;
                            }
                        }
                        if !((*found).ah.hh_next).is_null() {
                            found = ((*found).ah.hh_next as *mut libc::c_char)
                                .offset(-((*(*ausers).ah.tbl).hho as isize))
                                as *mut libc::c_void as *mut example_user_t;
                        } else {
                            found = 0 as *mut example_user_t;
                        }
                    }
                }
            }
        }
        printf(
            b"user %d, should_find=%d, found=%d\n\0" as *const u8 as *const libc::c_char,
            (*user).id,
            should_find,
            !found.is_null() as libc::c_int,
        );
        user = (*user).hh.next as *mut example_user_t;
    }
    user = ausers;
    while !user.is_null() {
        printf(b"auser %d\n\0" as *const u8 as *const libc::c_char, (*user).id);
        user = (*user).ah.next as *mut example_user_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

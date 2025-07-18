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
    pub cookie: libc::c_int,
    pub hh: UT_hash_handle,
    pub alth: UT_hash_handle,
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut user: *mut example_user_t = 0 as *mut example_user_t;
    let mut tmp: *mut example_user_t = 0 as *mut example_user_t;
    let mut users: *mut example_user_t = 0 as *mut example_user_t;
    let mut altusers: *mut example_user_t = 0 as *mut example_user_t;
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        if user.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*user).id = i;
        (*user).cookie = i * i;
        if i < 10 as libc::c_int {
            let mut _ha_hashv: libc::c_uint = 0;
            let mut _hj_i: libc::c_uint = 0;
            let mut _hj_j: libc::c_uint = 0;
            let mut _hj_k: libc::c_uint = 0;
            let mut _hj_key: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
                as *const libc::c_uchar;
            _ha_hashv = 0xfeedbeef as libc::c_uint;
            _hj_j = 0x9e3779b9 as libc::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
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
                                (*_hj_key.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
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
            let mut current_block_58: u64;
            match _hj_k {
                11 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_58 = 3201577399599230911;
                }
                10 => {
                    current_block_58 = 3201577399599230911;
                }
                9 => {
                    current_block_58 = 16533270327114151251;
                }
                8 => {
                    current_block_58 = 15793692102704341778;
                }
                7 => {
                    current_block_58 = 3813018556091210616;
                }
                6 => {
                    current_block_58 = 14776416267805999636;
                }
                5 => {
                    current_block_58 = 10094668786874726522;
                }
                4 => {
                    current_block_58 = 2901522158981431600;
                }
                3 => {
                    current_block_58 = 9418962588538182498;
                }
                2 => {
                    current_block_58 = 13663800821592822100;
                }
                1 => {
                    current_block_58 = 9896969144909681259;
                }
                _ => {
                    current_block_58 = 13460095289871124136;
                }
            }
            match current_block_58 {
                3201577399599230911 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_58 = 16533270327114151251;
                }
                _ => {}
            }
            match current_block_58 {
                16533270327114151251 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_58 = 15793692102704341778;
                }
                _ => {}
            }
            match current_block_58 {
                15793692102704341778 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_58 = 3813018556091210616;
                }
                _ => {}
            }
            match current_block_58 {
                3813018556091210616 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_58 = 14776416267805999636;
                }
                _ => {}
            }
            match current_block_58 {
                14776416267805999636 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_58 = 10094668786874726522;
                }
                _ => {}
            }
            match current_block_58 {
                10094668786874726522 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_58 = 2901522158981431600;
                }
                _ => {}
            }
            match current_block_58 {
                2901522158981431600 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_58 = 9418962588538182498;
                }
                _ => {}
            }
            match current_block_58 {
                9418962588538182498 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_58 = 13663800821592822100;
                }
                _ => {}
            }
            match current_block_58 {
                13663800821592822100 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_58 = 9896969144909681259;
                }
                _ => {}
            }
            match current_block_58 {
                9896969144909681259 => {
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
                    .tbl = malloc(
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                ) as *mut UT_hash_table;
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
                        .hho = (&mut (*user).hh as *mut UT_hash_handle
                        as *mut libc::c_char)
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
            (*(*users).hh.tbl)
                .num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
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
                        _he_thh = (*((*(*user).hh.tbl).buckets)
                            .offset(_he_bkt_i as isize))
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
                            if (*_he_newbkt).count > (*(*user).hh.tbl).ideal_chain_maxlen
                            {
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
        }
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
            _ha_hashv_0 = _ha_hashv_0
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
        let mut current_block_251: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv_0 = _ha_hashv_0
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_251 = 2854140694134532279;
            }
            10 => {
                current_block_251 = 2854140694134532279;
            }
            9 => {
                current_block_251 = 1712622082914920987;
            }
            8 => {
                current_block_251 = 4453115554739748;
            }
            7 => {
                current_block_251 = 3610867440718534498;
            }
            6 => {
                current_block_251 = 8412371970001094869;
            }
            5 => {
                current_block_251 = 434378063245968008;
            }
            4 => {
                current_block_251 = 2971336542024300429;
            }
            3 => {
                current_block_251 = 13339597974530717324;
            }
            2 => {
                current_block_251 = 10228338024067843677;
            }
            1 => {
                current_block_251 = 10062444578504115004;
            }
            _ => {
                current_block_251 = 16813369756331276724;
            }
        }
        match current_block_251 {
            2854140694134532279 => {
                _ha_hashv_0 = _ha_hashv_0
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_251 = 1712622082914920987;
            }
            _ => {}
        }
        match current_block_251 {
            1712622082914920987 => {
                _ha_hashv_0 = _ha_hashv_0
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_251 = 4453115554739748;
            }
            _ => {}
        }
        match current_block_251 {
            4453115554739748 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_251 = 3610867440718534498;
            }
            _ => {}
        }
        match current_block_251 {
            3610867440718534498 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_251 = 8412371970001094869;
            }
            _ => {}
        }
        match current_block_251 {
            8412371970001094869 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_251 = 434378063245968008;
            }
            _ => {}
        }
        match current_block_251 {
            434378063245968008 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_251 = 2971336542024300429;
            }
            _ => {}
        }
        match current_block_251 {
            2971336542024300429 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_251 = 13339597974530717324;
            }
            _ => {}
        }
        match current_block_251 {
            13339597974530717324 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_251 = 10228338024067843677;
            }
            _ => {}
        }
        match current_block_251 {
            10228338024067843677 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_251 = 10062444578504115004;
            }
            _ => {}
        }
        match current_block_251 {
            10062444578504115004 => {
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
        (*user).alth.hashv = _ha_hashv_0;
        (*user).alth.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
        (*user)
            .alth
            .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            as libc::c_uint;
        if altusers.is_null() {
            (*user).alth.next = 0 as *mut libc::c_void;
            (*user).alth.prev = 0 as *mut libc::c_void;
            (*user)
                .alth
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*user).alth.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*user).alth.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*user).alth.tbl).tail = &mut (*user).alth;
                (*(*user).alth.tbl).num_buckets = 32 as libc::c_uint;
                (*(*user).alth.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*user).alth.tbl)
                    .hho = (&mut (*user).alth as *mut UT_hash_handle
                    as *mut libc::c_char)
                    .offset_from(user as *mut libc::c_char) as libc::c_long;
                (*(*user).alth.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*user).alth.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*user).alth.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*user).alth.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            altusers = user;
        } else {
            (*user).alth.tbl = (*altusers).alth.tbl;
            (*user).alth.next = 0 as *mut libc::c_void;
            (*user)
                .alth
                .prev = ((*(*altusers).alth.tbl).tail as *mut libc::c_char)
                .offset(-((*(*altusers).alth.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*altusers).alth.tbl).tail).next = user as *mut libc::c_void;
            (*(*altusers).alth.tbl).tail = &mut (*user).alth;
        }
        let mut _ha_bkt_0: libc::c_uint = 0;
        (*(*altusers).alth.tbl)
            .num_items = ((*(*altusers).alth.tbl).num_items).wrapping_add(1);
        (*(*altusers).alth.tbl).num_items;
        _ha_bkt_0 = _ha_hashv_0
            & ((*(*altusers).alth.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*altusers).alth.tbl).buckets)
            .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
        (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
        (*_ha_head_0).count;
        (*user).alth.hh_next = (*_ha_head_0).hh_head;
        (*user).alth.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head_0).hh_head).is_null() {
            (*(*_ha_head_0).hh_head).hh_prev = &mut (*user).alth;
        }
        (*_ha_head_0).hh_head = &mut (*user).alth;
        if (*_ha_head_0).count
            >= ((*_ha_head_0).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*user).alth.tbl).noexpand == 0
        {
            let mut _he_bkt_0: libc::c_uint = 0;
            let mut _he_bkt_i_0: libc::c_uint = 0;
            let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets_0 = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).alth.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets_0.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets_0 as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*user).alth.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*user).alth.tbl)
                    .ideal_chain_maxlen = ((*(*user).alth.tbl).num_items
                    >> ((*(*user).alth.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*user).alth.tbl).num_items
                            & ((*(*user).alth.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*user).alth.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i_0 < (*(*user).alth.tbl).num_buckets {
                    _he_thh_0 = (*((*(*user).alth.tbl).buckets)
                        .offset(_he_bkt_i_0 as isize))
                        .hh_head;
                    while !_he_thh_0.is_null() {
                        _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                        _he_bkt_0 = (*_he_thh_0).hashv
                            & ((*(*user).alth.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                        if (*_he_newbkt_0).count > (*(*user).alth.tbl).ideal_chain_maxlen
                        {
                            (*(*user).alth.tbl)
                                .nonideal_items = ((*(*user).alth.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*user).alth.tbl).nonideal_items;
                            if (*_he_newbkt_0).count
                                > ((*_he_newbkt_0).expand_mult)
                                    .wrapping_mul((*(*user).alth.tbl).ideal_chain_maxlen)
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
                free((*(*user).alth.tbl).buckets as *mut libc::c_void);
                (*(*user).alth.tbl)
                    .num_buckets = ((*(*user).alth.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*user).alth.tbl)
                    .log2_num_buckets = ((*(*user).alth.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*user).alth.tbl).log2_num_buckets;
                (*(*user).alth.tbl).buckets = _he_new_buckets_0;
                (*(*user).alth.tbl)
                    .ineff_expands = if (*(*user).alth.tbl).nonideal_items
                    > (*(*user).alth.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*user).alth.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*user).alth.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*user).alth.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        i += 1;
        i;
    }
    i = 9 as libc::c_int;
    tmp = 0 as *mut example_user_t;
    if !users.is_null() {
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i_1: libc::c_uint = 0;
        let mut _hj_j_1: libc::c_uint = 0;
        let mut _hj_k_1: libc::c_uint = 0;
        let mut _hj_key_1: *const libc::c_uchar = &mut i as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j_1 = 0x9e3779b9 as libc::c_uint;
        _hj_i_1 = _hj_j_1;
        _hj_k_1 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k_1 >= 12 as libc::c_uint {
            _hj_i_1 = _hj_i_1
                .wrapping_add(
                    (*_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_j_1 = _hj_j_1
                .wrapping_add(
                    (*_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(11 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        ),
                );
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv);
            _hj_i_1 ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_1);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_1);
            _hf_hashv ^= _hj_j_1 >> 13 as libc::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv);
            _hj_i_1 ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_1);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_1);
            _hf_hashv ^= _hj_j_1 >> 5 as libc::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv);
            _hj_i_1 ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_1);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_1);
            _hf_hashv ^= _hj_j_1 >> 15 as libc::c_int;
            _hj_key_1 = _hj_key_1.offset(12 as libc::c_int as isize);
            _hj_k_1 = _hj_k_1.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_446: u64;
        match _hj_k_1 {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_1.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_446 = 5684663562884776962;
            }
            10 => {
                current_block_446 = 5684663562884776962;
            }
            9 => {
                current_block_446 = 15932501721594529961;
            }
            8 => {
                current_block_446 = 10494966441929050541;
            }
            7 => {
                current_block_446 = 16625017989975992022;
            }
            6 => {
                current_block_446 = 7599094102524275098;
            }
            5 => {
                current_block_446 = 11046353700707405348;
            }
            4 => {
                current_block_446 = 10889058540383362684;
            }
            3 => {
                current_block_446 = 8048250273032649887;
            }
            2 => {
                current_block_446 = 9441948780229972447;
            }
            1 => {
                current_block_446 = 15543780299440839850;
            }
            _ => {
                current_block_446 = 3583881587047060489;
            }
        }
        match current_block_446 {
            5684663562884776962 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_1.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_446 = 15932501721594529961;
            }
            _ => {}
        }
        match current_block_446 {
            15932501721594529961 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_446 = 10494966441929050541;
            }
            _ => {}
        }
        match current_block_446 {
            10494966441929050541 => {
                _hj_j_1 = _hj_j_1
                    .wrapping_add(
                        (*_hj_key_1.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_446 = 16625017989975992022;
            }
            _ => {}
        }
        match current_block_446 {
            16625017989975992022 => {
                _hj_j_1 = _hj_j_1
                    .wrapping_add(
                        (*_hj_key_1.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_446 = 7599094102524275098;
            }
            _ => {}
        }
        match current_block_446 {
            7599094102524275098 => {
                _hj_j_1 = _hj_j_1
                    .wrapping_add(
                        (*_hj_key_1.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_446 = 11046353700707405348;
            }
            _ => {}
        }
        match current_block_446 {
            11046353700707405348 => {
                _hj_j_1 = _hj_j_1
                    .wrapping_add(
                        *_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_446 = 10889058540383362684;
            }
            _ => {}
        }
        match current_block_446 {
            10889058540383362684 => {
                _hj_i_1 = _hj_i_1
                    .wrapping_add(
                        (*_hj_key_1.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_446 = 8048250273032649887;
            }
            _ => {}
        }
        match current_block_446 {
            8048250273032649887 => {
                _hj_i_1 = _hj_i_1
                    .wrapping_add(
                        (*_hj_key_1.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_446 = 9441948780229972447;
            }
            _ => {}
        }
        match current_block_446 {
            9441948780229972447 => {
                _hj_i_1 = _hj_i_1
                    .wrapping_add(
                        (*_hj_key_1.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_446 = 15543780299440839850;
            }
            _ => {}
        }
        match current_block_446 {
            15543780299440839850 => {
                _hj_i_1 = _hj_i_1
                    .wrapping_add(
                        *_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv);
        _hj_i_1 ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_1);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_1);
        _hf_hashv ^= _hj_j_1 >> 13 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv);
        _hj_i_1 ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_1);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_1);
        _hf_hashv ^= _hj_j_1 >> 5 as libc::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv);
        _hj_i_1 ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i_1);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j_1);
        _hf_hashv ^= _hj_j_1 >> 15 as libc::c_int;
        tmp = 0 as *mut example_user_t;
        if !users.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*users).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                    .is_null()
                {
                    tmp = ((*((*(*users).hh.tbl).buckets).offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut example_user_t;
                } else {
                    tmp = 0 as *mut example_user_t;
                }
                while !tmp.is_null() {
                    if (*tmp).hh.hashv == _hf_hashv
                        && (*tmp).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*tmp).hh.key,
                            &mut i as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*tmp).hh.hh_next).is_null() {
                        tmp = ((*tmp).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        tmp = 0 as *mut example_user_t;
                    }
                }
            }
        }
    }
    printf(
        b"%d %s in hh\n\0" as *const u8 as *const libc::c_char,
        i,
        if !tmp.is_null() {
            b"found\0" as *const u8 as *const libc::c_char
        } else {
            b"not found\0" as *const u8 as *const libc::c_char
        },
    );
    tmp = 0 as *mut example_user_t;
    if !altusers.is_null() {
        let mut _hf_hashv_0: libc::c_uint = 0;
        let mut _hj_i_2: libc::c_uint = 0;
        let mut _hj_j_2: libc::c_uint = 0;
        let mut _hj_k_2: libc::c_uint = 0;
        let mut _hj_key_2: *const libc::c_uchar = &mut i as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv_0 = 0xfeedbeef as libc::c_uint;
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
            _hf_hashv_0 = _hf_hashv_0
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
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_0);
            _hj_i_2 ^= _hf_hashv_0 >> 13 as libc::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_0);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_2);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_2);
            _hf_hashv_0 ^= _hj_j_2 >> 13 as libc::c_int;
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_0);
            _hj_i_2 ^= _hf_hashv_0 >> 12 as libc::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_0);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_2);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_2);
            _hf_hashv_0 ^= _hj_j_2 >> 5 as libc::c_int;
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_0);
            _hj_i_2 ^= _hf_hashv_0 >> 3 as libc::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_0);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_2);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_2);
            _hf_hashv_0 ^= _hj_j_2 >> 15 as libc::c_int;
            _hj_key_2 = _hj_key_2.offset(12 as libc::c_int as isize);
            _hj_k_2 = _hj_k_2.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv_0 = _hf_hashv_0
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_565: u64;
        match _hj_k_2 {
            11 => {
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_2.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_565 = 17897743260619974852;
            }
            10 => {
                current_block_565 = 17897743260619974852;
            }
            9 => {
                current_block_565 = 11568943310408519519;
            }
            8 => {
                current_block_565 = 4903928233898345168;
            }
            7 => {
                current_block_565 = 1901201689292909658;
            }
            6 => {
                current_block_565 = 8000897061338684977;
            }
            5 => {
                current_block_565 = 8372684985505791519;
            }
            4 => {
                current_block_565 = 2674107679507246681;
            }
            3 => {
                current_block_565 = 10302601536639739651;
            }
            2 => {
                current_block_565 = 4144057507804714715;
            }
            1 => {
                current_block_565 = 10955101428458776178;
            }
            _ => {
                current_block_565 = 5807655230524383057;
            }
        }
        match current_block_565 {
            17897743260619974852 => {
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_2.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_565 = 11568943310408519519;
            }
            _ => {}
        }
        match current_block_565 {
            11568943310408519519 => {
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_2.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_565 = 4903928233898345168;
            }
            _ => {}
        }
        match current_block_565 {
            4903928233898345168 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_565 = 1901201689292909658;
            }
            _ => {}
        }
        match current_block_565 {
            1901201689292909658 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_565 = 8000897061338684977;
            }
            _ => {}
        }
        match current_block_565 {
            8000897061338684977 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        (*_hj_key_2.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_565 = 8372684985505791519;
            }
            _ => {}
        }
        match current_block_565 {
            8372684985505791519 => {
                _hj_j_2 = _hj_j_2
                    .wrapping_add(
                        *_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_565 = 2674107679507246681;
            }
            _ => {}
        }
        match current_block_565 {
            2674107679507246681 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_565 = 10302601536639739651;
            }
            _ => {}
        }
        match current_block_565 {
            10302601536639739651 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_565 = 4144057507804714715;
            }
            _ => {}
        }
        match current_block_565 {
            4144057507804714715 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        (*_hj_key_2.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_565 = 10955101428458776178;
            }
            _ => {}
        }
        match current_block_565 {
            10955101428458776178 => {
                _hj_i_2 = _hj_i_2
                    .wrapping_add(
                        *_hj_key_2.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_0);
        _hj_i_2 ^= _hf_hashv_0 >> 13 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_0);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_2);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_2);
        _hf_hashv_0 ^= _hj_j_2 >> 13 as libc::c_int;
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_0);
        _hj_i_2 ^= _hf_hashv_0 >> 12 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_0);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_2);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_2);
        _hf_hashv_0 ^= _hj_j_2 >> 5 as libc::c_int;
        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
        _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_0);
        _hj_i_2 ^= _hf_hashv_0 >> 3 as libc::c_int;
        _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_0);
        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
        _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_2);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_2);
        _hf_hashv_0 ^= _hj_j_2 >> 15 as libc::c_int;
        tmp = 0 as *mut example_user_t;
        if !altusers.is_null() {
            let mut _hf_bkt_0: libc::c_uint = 0;
            _hf_bkt_0 = _hf_hashv_0
                & ((*(*altusers).alth.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*altusers).alth.tbl).buckets).offset(_hf_bkt_0 as isize))
                    .hh_head)
                    .is_null()
                {
                    tmp = ((*((*(*altusers).alth.tbl).buckets)
                        .offset(_hf_bkt_0 as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*altusers).alth.tbl).hho as isize))
                        as *mut libc::c_void as *mut example_user_t;
                } else {
                    tmp = 0 as *mut example_user_t;
                }
                while !tmp.is_null() {
                    if (*tmp).alth.hashv == _hf_hashv_0
                        && (*tmp).alth.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*tmp).alth.key,
                            &mut i as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*tmp).alth.hh_next).is_null() {
                        tmp = ((*tmp).alth.hh_next as *mut libc::c_char)
                            .offset(-((*(*altusers).alth.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        tmp = 0 as *mut example_user_t;
                    }
                }
            }
        }
    }
    printf(
        b"%d %s in alth\n\0" as *const u8 as *const libc::c_char,
        i,
        if !tmp.is_null() {
            b"found\0" as *const u8 as *const libc::c_char
        } else {
            b"not found\0" as *const u8 as *const libc::c_char
        },
    );
    i = 10 as libc::c_int;
    tmp = 0 as *mut example_user_t;
    if !users.is_null() {
        let mut _hf_hashv_1: libc::c_uint = 0;
        let mut _hj_i_3: libc::c_uint = 0;
        let mut _hj_j_3: libc::c_uint = 0;
        let mut _hj_k_3: libc::c_uint = 0;
        let mut _hj_key_3: *const libc::c_uchar = &mut i as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv_1 = 0xfeedbeef as libc::c_uint;
        _hj_j_3 = 0x9e3779b9 as libc::c_uint;
        _hj_i_3 = _hj_j_3;
        _hj_k_3 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
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
            _hf_hashv_1 = _hf_hashv_1
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
            _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_1);
            _hj_i_3 ^= _hf_hashv_1 >> 13 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_1);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_3);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_3);
            _hf_hashv_1 ^= _hj_j_3 >> 13 as libc::c_int;
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_1);
            _hj_i_3 ^= _hf_hashv_1 >> 12 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_1);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_3);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_3);
            _hf_hashv_1 ^= _hj_j_3 >> 5 as libc::c_int;
            _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
            _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_1);
            _hj_i_3 ^= _hf_hashv_1 >> 3 as libc::c_int;
            _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_1);
            _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
            _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_3);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_3);
            _hf_hashv_1 ^= _hj_j_3 >> 15 as libc::c_int;
            _hj_key_3 = _hj_key_3.offset(12 as libc::c_int as isize);
            _hj_k_3 = _hj_k_3.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv_1 = _hf_hashv_1
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_685: u64;
        match _hj_k_3 {
            11 => {
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_3.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_685 = 14825232298057625833;
            }
            10 => {
                current_block_685 = 14825232298057625833;
            }
            9 => {
                current_block_685 = 5918751339751240251;
            }
            8 => {
                current_block_685 = 2976740345871137345;
            }
            7 => {
                current_block_685 = 8104518946956323795;
            }
            6 => {
                current_block_685 = 14137955793484410301;
            }
            5 => {
                current_block_685 = 3505445475423316423;
            }
            4 => {
                current_block_685 = 1647352901621120930;
            }
            3 => {
                current_block_685 = 9386991487288681626;
            }
            2 => {
                current_block_685 = 12637559302324684038;
            }
            1 => {
                current_block_685 = 4888706221226997782;
            }
            _ => {
                current_block_685 = 17193116482801528934;
            }
        }
        match current_block_685 {
            14825232298057625833 => {
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_3.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_685 = 5918751339751240251;
            }
            _ => {}
        }
        match current_block_685 {
            5918751339751240251 => {
                _hf_hashv_1 = _hf_hashv_1
                    .wrapping_add(
                        (*_hj_key_3.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_685 = 2976740345871137345;
            }
            _ => {}
        }
        match current_block_685 {
            2976740345871137345 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        (*_hj_key_3.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_685 = 8104518946956323795;
            }
            _ => {}
        }
        match current_block_685 {
            8104518946956323795 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        (*_hj_key_3.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_685 = 14137955793484410301;
            }
            _ => {}
        }
        match current_block_685 {
            14137955793484410301 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        (*_hj_key_3.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_685 = 3505445475423316423;
            }
            _ => {}
        }
        match current_block_685 {
            3505445475423316423 => {
                _hj_j_3 = _hj_j_3
                    .wrapping_add(
                        *_hj_key_3.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_685 = 1647352901621120930;
            }
            _ => {}
        }
        match current_block_685 {
            1647352901621120930 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        (*_hj_key_3.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_685 = 9386991487288681626;
            }
            _ => {}
        }
        match current_block_685 {
            9386991487288681626 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        (*_hj_key_3.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_685 = 12637559302324684038;
            }
            _ => {}
        }
        match current_block_685 {
            12637559302324684038 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        (*_hj_key_3.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_685 = 4888706221226997782;
            }
            _ => {}
        }
        match current_block_685 {
            4888706221226997782 => {
                _hj_i_3 = _hj_i_3
                    .wrapping_add(
                        *_hj_key_3.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_1);
        _hj_i_3 ^= _hf_hashv_1 >> 13 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_1);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 8 as libc::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_3);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_3);
        _hf_hashv_1 ^= _hj_j_3 >> 13 as libc::c_int;
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_1);
        _hj_i_3 ^= _hf_hashv_1 >> 12 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_1);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 16 as libc::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_3);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_3);
        _hf_hashv_1 ^= _hj_j_3 >> 5 as libc::c_int;
        _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
        _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_1);
        _hj_i_3 ^= _hf_hashv_1 >> 3 as libc::c_int;
        _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_1);
        _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
        _hj_j_3 ^= _hj_i_3 << 10 as libc::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_3);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_3);
        _hf_hashv_1 ^= _hj_j_3 >> 15 as libc::c_int;
        tmp = 0 as *mut example_user_t;
        if !users.is_null() {
            let mut _hf_bkt_1: libc::c_uint = 0;
            _hf_bkt_1 = _hf_hashv_1
                & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*users).hh.tbl).buckets).offset(_hf_bkt_1 as isize)).hh_head)
                    .is_null()
                {
                    tmp = ((*((*(*users).hh.tbl).buckets).offset(_hf_bkt_1 as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut example_user_t;
                } else {
                    tmp = 0 as *mut example_user_t;
                }
                while !tmp.is_null() {
                    if (*tmp).hh.hashv == _hf_hashv_1
                        && (*tmp).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*tmp).hh.key,
                            &mut i as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*tmp).hh.hh_next).is_null() {
                        tmp = ((*tmp).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        tmp = 0 as *mut example_user_t;
                    }
                }
            }
        }
    }
    printf(
        b"%d %s in hh\n\0" as *const u8 as *const libc::c_char,
        i,
        if !tmp.is_null() {
            b"found\0" as *const u8 as *const libc::c_char
        } else {
            b"not found\0" as *const u8 as *const libc::c_char
        },
    );
    tmp = 0 as *mut example_user_t;
    if !altusers.is_null() {
        let mut _hf_hashv_2: libc::c_uint = 0;
        let mut _hj_i_4: libc::c_uint = 0;
        let mut _hj_j_4: libc::c_uint = 0;
        let mut _hj_k_4: libc::c_uint = 0;
        let mut _hj_key_4: *const libc::c_uchar = &mut i as *mut libc::c_int
            as *const libc::c_uchar;
        _hf_hashv_2 = 0xfeedbeef as libc::c_uint;
        _hj_j_4 = 0x9e3779b9 as libc::c_uint;
        _hj_i_4 = _hj_j_4;
        _hj_k_4 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k_4 >= 12 as libc::c_uint {
            _hj_i_4 = _hj_i_4
                .wrapping_add(
                    (*_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint)
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
                    (*_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint)
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
            _hf_hashv_2 = _hf_hashv_2
                .wrapping_add(
                    (*_hj_key_4.offset(8 as libc::c_int as isize) as libc::c_uint)
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
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_2);
            _hj_i_4 ^= _hf_hashv_2 >> 13 as libc::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_2);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
            _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_4);
            _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_4);
            _hf_hashv_2 ^= _hj_j_4 >> 13 as libc::c_int;
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_2);
            _hj_i_4 ^= _hf_hashv_2 >> 12 as libc::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_2);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
            _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_4);
            _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_4);
            _hf_hashv_2 ^= _hj_j_4 >> 5 as libc::c_int;
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_2);
            _hj_i_4 ^= _hf_hashv_2 >> 3 as libc::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_2);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
            _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_4);
            _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_4);
            _hf_hashv_2 ^= _hj_j_4 >> 15 as libc::c_int;
            _hj_key_4 = _hj_key_4.offset(12 as libc::c_int as isize);
            _hj_k_4 = _hj_k_4.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv_2 = _hf_hashv_2
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_804: u64;
        match _hj_k_4 {
            11 => {
                _hf_hashv_2 = _hf_hashv_2
                    .wrapping_add(
                        (*_hj_key_4.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_804 = 5743135109845226318;
            }
            10 => {
                current_block_804 = 5743135109845226318;
            }
            9 => {
                current_block_804 = 14433852283058032344;
            }
            8 => {
                current_block_804 = 13178896713188023455;
            }
            7 => {
                current_block_804 = 2767518686136159550;
            }
            6 => {
                current_block_804 = 8400996634817571718;
            }
            5 => {
                current_block_804 = 10057186977503183797;
            }
            4 => {
                current_block_804 = 11928884684822391877;
            }
            3 => {
                current_block_804 = 562533387689367468;
            }
            2 => {
                current_block_804 = 12340929100992884138;
            }
            1 => {
                current_block_804 = 5635111002786020586;
            }
            _ => {
                current_block_804 = 5570469760270364154;
            }
        }
        match current_block_804 {
            5743135109845226318 => {
                _hf_hashv_2 = _hf_hashv_2
                    .wrapping_add(
                        (*_hj_key_4.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_804 = 14433852283058032344;
            }
            _ => {}
        }
        match current_block_804 {
            14433852283058032344 => {
                _hf_hashv_2 = _hf_hashv_2
                    .wrapping_add(
                        (*_hj_key_4.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_804 = 13178896713188023455;
            }
            _ => {}
        }
        match current_block_804 {
            13178896713188023455 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        (*_hj_key_4.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_804 = 2767518686136159550;
            }
            _ => {}
        }
        match current_block_804 {
            2767518686136159550 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        (*_hj_key_4.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_804 = 8400996634817571718;
            }
            _ => {}
        }
        match current_block_804 {
            8400996634817571718 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        (*_hj_key_4.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_804 = 10057186977503183797;
            }
            _ => {}
        }
        match current_block_804 {
            10057186977503183797 => {
                _hj_j_4 = _hj_j_4
                    .wrapping_add(
                        *_hj_key_4.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_804 = 11928884684822391877;
            }
            _ => {}
        }
        match current_block_804 {
            11928884684822391877 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        (*_hj_key_4.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_804 = 562533387689367468;
            }
            _ => {}
        }
        match current_block_804 {
            562533387689367468 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        (*_hj_key_4.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_804 = 12340929100992884138;
            }
            _ => {}
        }
        match current_block_804 {
            12340929100992884138 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        (*_hj_key_4.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_804 = 5635111002786020586;
            }
            _ => {}
        }
        match current_block_804 {
            5635111002786020586 => {
                _hj_i_4 = _hj_i_4
                    .wrapping_add(
                        *_hj_key_4.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_2);
        _hj_i_4 ^= _hf_hashv_2 >> 13 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_2);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 8 as libc::c_int;
        _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_4);
        _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_4);
        _hf_hashv_2 ^= _hj_j_4 >> 13 as libc::c_int;
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_2);
        _hj_i_4 ^= _hf_hashv_2 >> 12 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_2);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 16 as libc::c_int;
        _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_4);
        _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_4);
        _hf_hashv_2 ^= _hj_j_4 >> 5 as libc::c_int;
        _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
        _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_2);
        _hj_i_4 ^= _hf_hashv_2 >> 3 as libc::c_int;
        _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_2);
        _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
        _hj_j_4 ^= _hj_i_4 << 10 as libc::c_int;
        _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_4);
        _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_4);
        _hf_hashv_2 ^= _hj_j_4 >> 15 as libc::c_int;
        tmp = 0 as *mut example_user_t;
        if !altusers.is_null() {
            let mut _hf_bkt_2: libc::c_uint = 0;
            _hf_bkt_2 = _hf_hashv_2
                & ((*(*altusers).alth.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*altusers).alth.tbl).buckets).offset(_hf_bkt_2 as isize))
                    .hh_head)
                    .is_null()
                {
                    tmp = ((*((*(*altusers).alth.tbl).buckets)
                        .offset(_hf_bkt_2 as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*altusers).alth.tbl).hho as isize))
                        as *mut libc::c_void as *mut example_user_t;
                } else {
                    tmp = 0 as *mut example_user_t;
                }
                while !tmp.is_null() {
                    if (*tmp).alth.hashv == _hf_hashv_2
                        && (*tmp).alth.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if memcmp(
                            (*tmp).alth.key,
                            &mut i as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*tmp).alth.hh_next).is_null() {
                        tmp = ((*tmp).alth.hh_next as *mut libc::c_char)
                            .offset(-((*(*altusers).alth.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t;
                    } else {
                        tmp = 0 as *mut example_user_t;
                    }
                }
            }
        }
    }
    printf(
        b"%d %s in alth\n\0" as *const u8 as *const libc::c_char,
        i,
        if !tmp.is_null() {
            b"found\0" as *const u8 as *const libc::c_char
        } else {
            b"not found\0" as *const u8 as *const libc::c_char
        },
    );
    if !users.is_null() {
        free((*(*users).hh.tbl).buckets as *mut libc::c_void);
        free((*users).hh.tbl as *mut libc::c_void);
        users = 0 as *mut example_user_t;
    }
    if !altusers.is_null() {
        free((*(*altusers).alth.tbl).buckets as *mut libc::c_void);
        free((*altusers).alth.tbl as *mut libc::c_void);
        altusers = 0 as *mut example_user_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

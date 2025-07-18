use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
}
static mut malloc_cnt: libc::c_int = 0 as libc::c_int;
static mut malloc_failed: libc::c_int = 0;
static mut is_fatal: libc::c_int = 0;
static mut j_buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut users: *mut example_user_t = 0 as *const example_user_t
    as *mut example_user_t;
static mut user_id: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn alt_malloc(mut sz: size_t) -> *mut libc::c_void {
    malloc_cnt -= 1;
    if malloc_cnt <= 0 as libc::c_int {
        malloc_failed = 1 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    malloc_failed = 0 as libc::c_int;
    return malloc(sz);
}
unsafe extern "C" fn alt_fatal(mut s: *const libc::c_char) {
    is_fatal = 1 as libc::c_int;
    longjmp(j_buf.as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn init_users(mut need_malloc_cnt: libc::c_int) {
    users = 0 as *mut example_user_t;
    let mut user: *mut example_user_t = malloc(
        ::std::mem::size_of::<example_user_t>() as libc::c_ulong,
    ) as *mut example_user_t;
    (*user).id = user_id;
    is_fatal = 0 as libc::c_int;
    malloc_cnt = need_malloc_cnt;
    if _setjmp(j_buf.as_mut_ptr()) == 0 {
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
        let mut current_block_55: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_55 = 5918751339751240251;
            }
            10 => {
                current_block_55 = 5918751339751240251;
            }
            9 => {
                current_block_55 = 2976740345871137345;
            }
            8 => {
                current_block_55 = 8104518946956323795;
            }
            7 => {
                current_block_55 = 14137955793484410301;
            }
            6 => {
                current_block_55 = 3505445475423316423;
            }
            5 => {
                current_block_55 = 1647352901621120930;
            }
            4 => {
                current_block_55 = 9386991487288681626;
            }
            3 => {
                current_block_55 = 12637559302324684038;
            }
            2 => {
                current_block_55 = 4888706221226997782;
            }
            1 => {
                current_block_55 = 8293607905036306531;
            }
            _ => {
                current_block_55 = 11626999923138678822;
            }
        }
        match current_block_55 {
            5918751339751240251 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_55 = 2976740345871137345;
            }
            _ => {}
        }
        match current_block_55 {
            2976740345871137345 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_55 = 8104518946956323795;
            }
            _ => {}
        }
        match current_block_55 {
            8104518946956323795 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_55 = 14137955793484410301;
            }
            _ => {}
        }
        match current_block_55 {
            14137955793484410301 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_55 = 3505445475423316423;
            }
            _ => {}
        }
        match current_block_55 {
            3505445475423316423 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_55 = 1647352901621120930;
            }
            _ => {}
        }
        match current_block_55 {
            1647352901621120930 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_55 = 9386991487288681626;
            }
            _ => {}
        }
        match current_block_55 {
            9386991487288681626 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_55 = 12637559302324684038;
            }
            _ => {}
        }
        match current_block_55 {
            12637559302324684038 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_55 = 4888706221226997782;
            }
            _ => {}
        }
        match current_block_55 {
            4888706221226997782 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_55 = 8293607905036306531;
            }
            _ => {}
        }
        match current_block_55 {
            8293607905036306531 => {
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
                .tbl = alt_malloc(
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            ) as *mut UT_hash_table;
            if ((*user).hh.tbl).is_null() {
                alt_fatal(b"out of memory\0" as *const u8 as *const libc::c_char);
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
                    alt_fatal(b"out of memory\0" as *const u8 as *const libc::c_char);
                    free((*user).hh.tbl as *mut libc::c_void);
                } else {
                    memset(
                        (*(*user).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                    (*(*user).hh.tbl).bloom_nbits = 16 as libc::c_int as uint8_t;
                    (*(*user).hh.tbl)
                        .bloom_bv = alt_malloc(
                        ((1 as libc::c_ulong) << 16 as libc::c_int)
                            .wrapping_div(8 as libc::c_ulong)
                            .wrapping_add(
                                (if ((1 as libc::c_ulong) << 16 as libc::c_int)
                                    .wrapping_rem(8 as libc::c_ulong) != 0 as libc::c_ulong
                                {
                                    1 as libc::c_ulong
                                } else {
                                    0 as libc::c_ulong
                                }),
                            ),
                    ) as *mut uint8_t;
                    if ((*(*user).hh.tbl).bloom_bv).is_null() {
                        alt_fatal(
                            b"out of memory\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        memset(
                            (*(*user).hh.tbl).bloom_bv as *mut libc::c_void,
                            '\0' as i32,
                            ((1 as libc::c_ulong) << 16 as libc::c_int)
                                .wrapping_div(8 as libc::c_ulong)
                                .wrapping_add(
                                    (if ((1 as libc::c_ulong) << 16 as libc::c_int)
                                        .wrapping_rem(8 as libc::c_ulong) != 0 as libc::c_ulong
                                    {
                                        1 as libc::c_ulong
                                    } else {
                                        0 as libc::c_ulong
                                    }),
                                ),
                        );
                        (*(*user).hh.tbl).bloom_sig = 0xb12220f2 as libc::c_uint;
                    }
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
            _he_new_buckets = alt_malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                alt_fatal(b"out of memory\0" as *const u8 as *const libc::c_char);
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
        let ref mut fresh0 = *((*(*users).hh.tbl).bloom_bv)
            .offset(
                (_ha_hashv
                    & ((1 as libc::c_ulong)
                        << (*(*users).hh.tbl).bloom_nbits as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as uint32_t)
                    .wrapping_div(8 as libc::c_uint) as isize,
            );
        *fresh0 = (*fresh0 as libc::c_uint
            | (1 as libc::c_uint)
                << (_ha_hashv
                    & ((1 as libc::c_ulong)
                        << (*(*users).hh.tbl).bloom_nbits as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as uint32_t)
                    .wrapping_rem(8 as libc::c_uint)) as uint8_t;
    } else {
        free(user as *mut libc::c_void);
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut user: *mut example_user_t = 0 as *mut example_user_t;
    init_users(3 as libc::c_int);
    if is_fatal == 0 {
        printf(
            b"fatal not called after bloom failure\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    init_users(2 as libc::c_int);
    if is_fatal == 0 {
        printf(
            b"fatal not called after bucket creation failure\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    init_users(1 as libc::c_int);
    if is_fatal == 0 {
        printf(
            b"fatal not called after table creation failure\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    init_users(4 as libc::c_int);
    if is_fatal != 0 {
        printf(
            b"fatal error when creating hash normally\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    users = 0 as *mut example_user_t;
    malloc_cnt = 4 as libc::c_int;
    loop {
        let fresh1 = user_id;
        user_id = user_id + 1;
        if fresh1 == 1000 as libc::c_int {
            printf(
                b"there is no way 1000 iterations didn't require realloc\n\0"
                    as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
                as *mut example_user_t;
            (*user).id = user_id;
            if _setjmp(j_buf.as_mut_ptr()) == 0 {
                let mut _ha_hashv: libc::c_uint = 0;
                let mut _hj_i: libc::c_uint = 0;
                let mut _hj_j: libc::c_uint = 0;
                let mut _hj_k: libc::c_uint = 0;
                let mut _hj_key: *const libc::c_uchar = &mut (*user).id
                    as *mut libc::c_int as *const libc::c_uchar;
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
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_uint,
                    );
                let mut current_block_73: u64;
                match _hj_k {
                    11 => {
                        _ha_hashv = _ha_hashv
                            .wrapping_add(
                                (*_hj_key.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_73 = 2336934363235394480;
                    }
                    10 => {
                        current_block_73 = 2336934363235394480;
                    }
                    9 => {
                        current_block_73 = 356153273329192892;
                    }
                    8 => {
                        current_block_73 = 3113249944262274402;
                    }
                    7 => {
                        current_block_73 = 7967198120077568877;
                    }
                    6 => {
                        current_block_73 = 8059081802635738315;
                    }
                    5 => {
                        current_block_73 = 12511408670735699126;
                    }
                    4 => {
                        current_block_73 = 16272498523267842364;
                    }
                    3 => {
                        current_block_73 = 3651303023350803605;
                    }
                    2 => {
                        current_block_73 = 5266824145032302674;
                    }
                    1 => {
                        current_block_73 = 5210693777381988995;
                    }
                    _ => {
                        current_block_73 = 8835654301469918283;
                    }
                }
                match current_block_73 {
                    2336934363235394480 => {
                        _ha_hashv = _ha_hashv
                            .wrapping_add(
                                (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            );
                        current_block_73 = 356153273329192892;
                    }
                    _ => {}
                }
                match current_block_73 {
                    356153273329192892 => {
                        _ha_hashv = _ha_hashv
                            .wrapping_add(
                                (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            );
                        current_block_73 = 3113249944262274402;
                    }
                    _ => {}
                }
                match current_block_73 {
                    3113249944262274402 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            );
                        current_block_73 = 7967198120077568877;
                    }
                    _ => {}
                }
                match current_block_73 {
                    7967198120077568877 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            );
                        current_block_73 = 8059081802635738315;
                    }
                    _ => {}
                }
                match current_block_73 {
                    8059081802635738315 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            );
                        current_block_73 = 12511408670735699126;
                    }
                    _ => {}
                }
                match current_block_73 {
                    12511408670735699126 => {
                        _hj_j = _hj_j
                            .wrapping_add(
                                *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                            );
                        current_block_73 = 16272498523267842364;
                    }
                    _ => {}
                }
                match current_block_73 {
                    16272498523267842364 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            );
                        current_block_73 = 3651303023350803605;
                    }
                    _ => {}
                }
                match current_block_73 {
                    3651303023350803605 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            );
                        current_block_73 = 5266824145032302674;
                    }
                    _ => {}
                }
                match current_block_73 {
                    5266824145032302674 => {
                        _hj_i = _hj_i
                            .wrapping_add(
                                (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            );
                        current_block_73 = 5210693777381988995;
                    }
                    _ => {}
                }
                match current_block_73 {
                    5210693777381988995 => {
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
                        alt_fatal(
                            b"out of memory\0" as *const u8 as *const libc::c_char,
                        );
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
                            alt_fatal(
                                b"out of memory\0" as *const u8 as *const libc::c_char,
                            );
                            free((*user).hh.tbl as *mut libc::c_void);
                        } else {
                            memset(
                                (*(*user).hh.tbl).buckets as *mut libc::c_void,
                                '\0' as i32,
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            );
                            (*(*user).hh.tbl).bloom_nbits = 16 as libc::c_int as uint8_t;
                            (*(*user).hh.tbl)
                                .bloom_bv = alt_malloc(
                                ((1 as libc::c_ulong) << 16 as libc::c_int)
                                    .wrapping_div(8 as libc::c_ulong)
                                    .wrapping_add(
                                        (if ((1 as libc::c_ulong) << 16 as libc::c_int)
                                            .wrapping_rem(8 as libc::c_ulong) != 0 as libc::c_ulong
                                        {
                                            1 as libc::c_ulong
                                        } else {
                                            0 as libc::c_ulong
                                        }),
                                    ),
                            ) as *mut uint8_t;
                            if ((*(*user).hh.tbl).bloom_bv).is_null() {
                                alt_fatal(
                                    b"out of memory\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                memset(
                                    (*(*user).hh.tbl).bloom_bv as *mut libc::c_void,
                                    '\0' as i32,
                                    ((1 as libc::c_ulong) << 16 as libc::c_int)
                                        .wrapping_div(8 as libc::c_ulong)
                                        .wrapping_add(
                                            (if ((1 as libc::c_ulong) << 16 as libc::c_int)
                                                .wrapping_rem(8 as libc::c_ulong) != 0 as libc::c_ulong
                                            {
                                                1 as libc::c_ulong
                                            } else {
                                                0 as libc::c_ulong
                                            }),
                                        ),
                                );
                                (*(*user).hh.tbl).bloom_sig = 0xb12220f2 as libc::c_uint;
                            }
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
                let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl)
                    .buckets)
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
                        .wrapping_mul(10 as libc::c_uint)
                    && (*(*user).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: libc::c_uint = 0;
                    let mut _he_bkt_i: libc::c_uint = 0;
                    let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_new_buckets: *mut UT_hash_bucket = 0
                        as *mut UT_hash_bucket;
                    let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                    _he_new_buckets = alt_malloc(
                        (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                            .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        alt_fatal(
                            b"out of memory\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        memset(
                            _he_new_buckets as *mut libc::c_void,
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
                                if (*_he_newbkt).count
                                    > (*(*user).hh.tbl).ideal_chain_maxlen
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
                }
                let ref mut fresh2 = *((*(*users).hh.tbl).bloom_bv)
                    .offset(
                        (_ha_hashv
                            & ((1 as libc::c_ulong)
                                << (*(*users).hh.tbl).bloom_nbits as libc::c_int)
                                .wrapping_sub(1 as libc::c_uint as libc::c_ulong)
                                as uint32_t)
                            .wrapping_div(8 as libc::c_uint) as isize,
                    );
                *fresh2 = (*fresh2 as libc::c_uint
                    | (1 as libc::c_uint)
                        << (_ha_hashv
                            & ((1 as libc::c_ulong)
                                << (*(*users).hh.tbl).bloom_nbits as libc::c_int)
                                .wrapping_sub(1 as libc::c_uint as libc::c_ulong)
                                as uint32_t)
                            .wrapping_rem(8 as libc::c_uint)) as uint8_t;
            } else {
                free(user as *mut libc::c_void);
            }
            if malloc_failed != 0 {
                if is_fatal == 0 {
                    printf(
                        b"fatal not called after bucket not extended\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if user_id < 10 as libc::c_int {
                    printf(
                        b"there is no way your bucket size is 10\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                break;
            } else {
                malloc_cnt = 0 as libc::c_int;
            }
        }
    }
    if !users.is_null() {
        free((*(*users).hh.tbl).bloom_bv as *mut libc::c_void);
        free((*(*users).hh.tbl).buckets as *mut libc::c_void);
        free((*users).hh.tbl as *mut libc::c_void);
        users = 0 as *mut example_user_t;
    }
    printf(b"End\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

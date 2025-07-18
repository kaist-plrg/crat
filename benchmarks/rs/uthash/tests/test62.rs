use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
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
pub struct test_t {
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub hh: UT_hash_handle,
}
pub unsafe extern "C" fn TrivialHash(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> libc::c_uint {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        h = h.wrapping_add(*s.offset(i as isize) as libc::c_uchar as libc::c_uint);
        i = i.wrapping_add(1);
        i;
    }
    return h;
}
pub unsafe extern "C" fn make_test(mut value: libc::c_int) -> *mut test_t {
    let mut test: *mut test_t = malloc(::std::mem::size_of::<test_t>() as libc::c_ulong)
        as *mut test_t;
    if !test.is_null() {} else {
        __assert_fail(
            b"test != NULL\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"struct test_t *make_test(int)\0"))
                .as_ptr(),
        );
    }
    'c_1473: {
        if !test.is_null() {} else {
            __assert_fail(
                b"test != NULL\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                27 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"struct test_t *make_test(int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*test).a = value;
    return test;
}
unsafe fn main_0() -> libc::c_int {
    let mut tests: *mut test_t = 0 as *mut test_t;
    let mut test: *mut test_t = 0 as *mut test_t;
    let mut x: libc::c_int = 0;
    let mut h: libc::c_uint = 0;
    x = 0x42 as libc::c_int;
    h = TrivialHash(
        &mut x as *mut libc::c_int as *const libc::c_char,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    if h == 0x42 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h == 0x42\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_5224: {
        if h == 0x42 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"h == 0x42\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0x4002 as libc::c_int;
    h = TrivialHash(
        &mut x as *mut libc::c_int as *const libc::c_char,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    if h == 0x42 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"h == 0x42\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_5167: {
        if h == 0x42 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"h == 0x42\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    test = make_test(0x42 as libc::c_int);
    let mut _ha_hashv: libc::c_uint = 0;
    _ha_hashv = TrivialHash(
        &mut (*test).a as *mut libc::c_int as *const libc::c_char,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    (*test).hh.hashv = _ha_hashv;
    (*test).hh.key = &mut (*test).a as *mut libc::c_int as *const libc::c_void;
    (*test)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if tests.is_null() {
        (*test).hh.next = 0 as *mut libc::c_void;
        (*test).hh.prev = 0 as *mut libc::c_void;
        (*test)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*test).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*test).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*test).hh.tbl).tail = &mut (*test).hh;
            (*(*test).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*test).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*test).hh.tbl)
                .hho = (&mut (*test).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(test as *mut libc::c_char) as libc::c_long;
            (*(*test).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*test).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*test).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*test).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        tests = test;
    } else {
        (*test).hh.tbl = (*tests).hh.tbl;
        (*test).hh.next = 0 as *mut libc::c_void;
        (*test)
            .hh
            .prev = ((*(*tests).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*tests).hh.tbl).tail).next = test as *mut libc::c_void;
        (*(*tests).hh.tbl).tail = &mut (*test).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*tests).hh.tbl).num_items = ((*(*tests).hh.tbl).num_items).wrapping_add(1);
    (*(*tests).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*tests).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*test).hh.hh_next = (*_ha_head).hh_head;
    (*test).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*test).hh;
    }
    (*_ha_head).hh_head = &mut (*test).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*test).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*test).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*test).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*test).hh.tbl)
                .ideal_chain_maxlen = ((*(*test).hh.tbl).num_items
                >> ((*(*test).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*test).hh.tbl).num_items
                        & ((*(*test).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*test).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*test).hh.tbl).num_buckets {
                _he_thh = (*((*(*test).hh.tbl).buckets).offset(_he_bkt_i as isize))
                    .hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*test).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*test).hh.tbl).ideal_chain_maxlen {
                        (*(*test).hh.tbl)
                            .nonideal_items = ((*(*test).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*test).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*test).hh.tbl).ideal_chain_maxlen)
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
            free((*(*test).hh.tbl).buckets as *mut libc::c_void);
            (*(*test).hh.tbl)
                .num_buckets = ((*(*test).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*test).hh.tbl)
                .log2_num_buckets = ((*(*test).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*test).hh.tbl).log2_num_buckets;
            (*(*test).hh.tbl).buckets = _he_new_buckets;
            (*(*test).hh.tbl)
                .ineff_expands = if (*(*test).hh.tbl).nonideal_items
                > (*(*test).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*test).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*test).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*test).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    test = make_test(0x4002 as libc::c_int);
    let mut _ha_hashv_0: libc::c_uint = 0;
    _ha_hashv_0 = TrivialHash(
        &mut (*test).a as *mut libc::c_int as *const libc::c_char,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    (*test).hh.hashv = _ha_hashv_0;
    (*test).hh.key = &mut (*test).a as *mut libc::c_int as *const libc::c_void;
    (*test)
        .hh
        .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
    if tests.is_null() {
        (*test).hh.next = 0 as *mut libc::c_void;
        (*test).hh.prev = 0 as *mut libc::c_void;
        (*test)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*test).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*test).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*test).hh.tbl).tail = &mut (*test).hh;
            (*(*test).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*test).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*test).hh.tbl)
                .hho = (&mut (*test).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(test as *mut libc::c_char) as libc::c_long;
            (*(*test).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*test).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*test).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*test).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        tests = test;
    } else {
        (*test).hh.tbl = (*tests).hh.tbl;
        (*test).hh.next = 0 as *mut libc::c_void;
        (*test)
            .hh
            .prev = ((*(*tests).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*tests).hh.tbl).tail).next = test as *mut libc::c_void;
        (*(*tests).hh.tbl).tail = &mut (*test).hh;
    }
    let mut _ha_bkt_0: libc::c_uint = 0;
    (*(*tests).hh.tbl).num_items = ((*(*tests).hh.tbl).num_items).wrapping_add(1);
    (*(*tests).hh.tbl).num_items;
    _ha_bkt_0 = _ha_hashv_0
        & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*tests).hh.tbl).buckets)
        .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
    (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
    (*_ha_head_0).count;
    (*test).hh.hh_next = (*_ha_head_0).hh_head;
    (*test).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head_0).hh_head).is_null() {
        (*(*_ha_head_0).hh_head).hh_prev = &mut (*test).hh;
    }
    (*_ha_head_0).hh_head = &mut (*test).hh;
    if (*_ha_head_0).count
        >= ((*_ha_head_0).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*test).hh.tbl).noexpand == 0
    {
        let mut _he_bkt_0: libc::c_uint = 0;
        let mut _he_bkt_i_0: libc::c_uint = 0;
        let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets_0 = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*test).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets_0.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets_0 as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*test).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*test).hh.tbl)
                .ideal_chain_maxlen = ((*(*test).hh.tbl).num_items
                >> ((*(*test).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*test).hh.tbl).num_items
                        & ((*(*test).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*test).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i_0 < (*(*test).hh.tbl).num_buckets {
                _he_thh_0 = (*((*(*test).hh.tbl).buckets).offset(_he_bkt_i_0 as isize))
                    .hh_head;
                while !_he_thh_0.is_null() {
                    _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                    _he_bkt_0 = (*_he_thh_0).hashv
                        & ((*(*test).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt_0 = &mut *_he_new_buckets_0.offset(_he_bkt_0 as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt_0).count = ((*_he_newbkt_0).count).wrapping_add(1);
                    if (*_he_newbkt_0).count > (*(*test).hh.tbl).ideal_chain_maxlen {
                        (*(*test).hh.tbl)
                            .nonideal_items = ((*(*test).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*test).hh.tbl).nonideal_items;
                        if (*_he_newbkt_0).count
                            > ((*_he_newbkt_0).expand_mult)
                                .wrapping_mul((*(*test).hh.tbl).ideal_chain_maxlen)
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
            free((*(*test).hh.tbl).buckets as *mut libc::c_void);
            (*(*test).hh.tbl)
                .num_buckets = ((*(*test).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*test).hh.tbl)
                .log2_num_buckets = ((*(*test).hh.tbl).log2_num_buckets).wrapping_add(1);
            (*(*test).hh.tbl).log2_num_buckets;
            (*(*test).hh.tbl).buckets = _he_new_buckets_0;
            (*(*test).hh.tbl)
                .ineff_expands = if (*(*test).hh.tbl).nonideal_items
                > (*(*test).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*test).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*test).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*test).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    x = 0x4002 as libc::c_int;
    test = 0 as *mut test_t;
    test = 0 as *mut test_t;
    if !tests.is_null() {
        let mut _hf_bkt: libc::c_uint = 0;
        _hf_bkt = 0x42 as libc::c_int as libc::c_uint
            & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
                .is_null()
            {
                test = ((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                    as *mut libc::c_char)
                    .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut test_t;
            } else {
                test = 0 as *mut test_t;
            }
            while !test.is_null() {
                if (*test).hh.hashv == 0x42 as libc::c_int as libc::c_uint
                    && (*test).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*test).hh.key,
                        &mut x as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*test).hh.hh_next).is_null() {
                    test = ((*test).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut test_t;
                } else {
                    test = 0 as *mut test_t;
                }
            }
        }
    }
    if !test.is_null() {} else {
        __assert_fail(
            b"test != NULL\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2815: {
        if !test.is_null() {} else {
            __assert_fail(
                b"test != NULL\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*test).a == 0x4002 as libc::c_int {} else {
        __assert_fail(
            b"test->a == 0x4002\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2775: {
        if (*test).a == 0x4002 as libc::c_int {} else {
            __assert_fail(
                b"test->a == 0x4002\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0x42 as libc::c_int;
    test = 0 as *mut test_t;
    test = 0 as *mut test_t;
    if !tests.is_null() {
        let mut _hf_bkt_0: libc::c_uint = 0;
        _hf_bkt_0 = 0x42 as libc::c_int as libc::c_uint
            & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_0 as isize)).hh_head)
                .is_null()
            {
                test = ((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_0 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut test_t;
            } else {
                test = 0 as *mut test_t;
            }
            while !test.is_null() {
                if (*test).hh.hashv == 0x42 as libc::c_int as libc::c_uint
                    && (*test).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*test).hh.key,
                        &mut x as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*test).hh.hh_next).is_null() {
                    test = ((*test).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut test_t;
                } else {
                    test = 0 as *mut test_t;
                }
            }
        }
    }
    if !test.is_null() {} else {
        __assert_fail(
            b"test != NULL\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2497: {
        if !test.is_null() {} else {
            __assert_fail(
                b"test != NULL\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*test).a == 0x42 as libc::c_int {} else {
        __assert_fail(
            b"test->a == 0x0042\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2456: {
        if (*test).a == 0x42 as libc::c_int {} else {
            __assert_fail(
                b"test->a == 0x0042\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0x4002 as libc::c_int;
    test = 0 as *mut test_t;
    test = 0 as *mut test_t;
    if !tests.is_null() {
        let mut _hf_bkt_1: libc::c_uint = 0;
        _hf_bkt_1 = 0x43 as libc::c_int as libc::c_uint
            & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_1 as isize)).hh_head)
                .is_null()
            {
                test = ((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_1 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut test_t;
            } else {
                test = 0 as *mut test_t;
            }
            while !test.is_null() {
                if (*test).hh.hashv == 0x43 as libc::c_int as libc::c_uint
                    && (*test).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*test).hh.key,
                        &mut x as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*test).hh.hh_next).is_null() {
                    test = ((*test).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut test_t;
                } else {
                    test = 0 as *mut test_t;
                }
            }
        }
    }
    if test.is_null() {} else {
        __assert_fail(
            b"test == NULL\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2178: {
        if test.is_null() {} else {
            __assert_fail(
                b"test == NULL\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                67 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0x42 as libc::c_int;
    test = 0 as *mut test_t;
    test = 0 as *mut test_t;
    if !tests.is_null() {
        let mut _hf_bkt_2: libc::c_uint = 0;
        _hf_bkt_2 = 0x43 as libc::c_int as libc::c_uint
            & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_2 as isize)).hh_head)
                .is_null()
            {
                test = ((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_2 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut test_t;
            } else {
                test = 0 as *mut test_t;
            }
            while !test.is_null() {
                if (*test).hh.hashv == 0x43 as libc::c_int as libc::c_uint
                    && (*test).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*test).hh.key,
                        &mut x as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*test).hh.hh_next).is_null() {
                    test = ((*test).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut test_t;
                } else {
                    test = 0 as *mut test_t;
                }
            }
        }
    }
    if test.is_null() {} else {
        __assert_fail(
            b"test == NULL\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1900: {
        if test.is_null() {} else {
            __assert_fail(
                b"test == NULL\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0x4003 as libc::c_int;
    test = 0 as *mut test_t;
    test = 0 as *mut test_t;
    if !tests.is_null() {
        let mut _hf_bkt_3: libc::c_uint = 0;
        _hf_bkt_3 = 0x42 as libc::c_int as libc::c_uint
            & ((*(*tests).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_3 as isize)).hh_head)
                .is_null()
            {
                test = ((*((*(*tests).hh.tbl).buckets).offset(_hf_bkt_3 as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut test_t;
            } else {
                test = 0 as *mut test_t;
            }
            while !test.is_null() {
                if (*test).hh.hashv == 0x42 as libc::c_int as libc::c_uint
                    && (*test).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if memcmp(
                        (*test).hh.key,
                        &mut x as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*test).hh.hh_next).is_null() {
                    test = ((*test).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*tests).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut test_t;
                } else {
                    test = 0 as *mut test_t;
                }
            }
        }
    }
    if test.is_null() {} else {
        __assert_fail(
            b"test == NULL\0" as *const u8 as *const libc::c_char,
            b"test62.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1612: {
        if test.is_null() {} else {
            __assert_fail(
                b"test == NULL\0" as *const u8 as *const libc::c_char,
                b"test62.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !tests.is_null() {
        free((*(*tests).hh.tbl).buckets as *mut libc::c_void);
        free((*tests).hh.tbl as *mut libc::c_void);
        tests = 0 as *mut test_t;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

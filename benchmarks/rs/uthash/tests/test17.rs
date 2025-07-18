use ::libc;
extern "C" {
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
}
unsafe extern "C" fn rev(
    mut _a: *mut libc::c_void,
    mut _b: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut example_user_t = _a as *mut example_user_t;
    let mut b: *mut example_user_t = _b as *mut example_user_t;
    printf(
        b"called for a:%d, b:%d\n\0" as *const u8 as *const libc::c_char,
        (*a).id,
        (*b).id,
    );
    return (*a).id - (*b).id;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut user: *mut example_user_t = 0 as *mut example_user_t;
    let mut users: *mut example_user_t = 0 as *mut example_user_t;
    i = 9 as libc::c_int;
    while i >= 0 as libc::c_int {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        if user.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*user).id = i;
        (*user).cookie = i * i;
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
        let mut current_block_58: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 6490414228117698753;
            }
            10 => {
                current_block_58 = 6490414228117698753;
            }
            9 => {
                current_block_58 = 4098161721221443587;
            }
            8 => {
                current_block_58 = 10896113030001404424;
            }
            7 => {
                current_block_58 = 4690278604938096435;
            }
            6 => {
                current_block_58 = 1677079002017831167;
            }
            5 => {
                current_block_58 = 16807228163047660531;
            }
            4 => {
                current_block_58 = 16364167804578183552;
            }
            3 => {
                current_block_58 = 12395523488977389649;
            }
            2 => {
                current_block_58 = 13123768761735600930;
            }
            1 => {
                current_block_58 = 13909856113418196919;
            }
            _ => {
                current_block_58 = 1924505913685386279;
            }
        }
        match current_block_58 {
            6490414228117698753 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 4098161721221443587;
            }
            _ => {}
        }
        match current_block_58 {
            4098161721221443587 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 10896113030001404424;
            }
            _ => {}
        }
        match current_block_58 {
            10896113030001404424 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 4690278604938096435;
            }
            _ => {}
        }
        match current_block_58 {
            4690278604938096435 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 1677079002017831167;
            }
            _ => {}
        }
        match current_block_58 {
            1677079002017831167 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 16807228163047660531;
            }
            _ => {}
        }
        match current_block_58 {
            16807228163047660531 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_58 = 16364167804578183552;
            }
            _ => {}
        }
        match current_block_58 {
            16364167804578183552 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 12395523488977389649;
            }
            _ => {}
        }
        match current_block_58 {
            12395523488977389649 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 13123768761735600930;
            }
            _ => {}
        }
        match current_block_58 {
            13123768761735600930 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 13909856113418196919;
            }
            _ => {}
        }
        match current_block_58 {
            13909856113418196919 => {
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
        i -= 1;
        i;
    }
    user = users;
    while !user.is_null() {
        printf(
            b"user %d, cookie %d\n\0" as *const u8 as *const libc::c_char,
            (*user).id,
            (*user).cookie,
        );
        user = (*user).hh.next as *mut example_user_t;
    }
    printf(b"sorting\n\0" as *const u8 as *const libc::c_char);
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
    if !users.is_null() {
        _hs_insize = 1 as libc::c_int as libc::c_uint;
        _hs_looping = 1 as libc::c_int as libc::c_uint;
        _hs_list = &mut (*users).hh;
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
                            .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                            as *mut UT_hash_handle
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
                                .offset((*(*users).hh.tbl).hho as isize)
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
                                    .offset((*(*users).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                        _hs_psize;
                    } else if rev(
                        (_hs_p as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t
                            as *mut libc::c_void,
                        (_hs_q as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t
                            as *mut libc::c_void,
                    ) <= 0 as libc::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = if !((*_hs_p).next).is_null() {
                                ((*_hs_p).next as *mut libc::c_char)
                                    .offset((*(*users).hh.tbl).hho as isize)
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
                                .offset((*(*users).hh.tbl).hho as isize)
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
                                .offset(-((*(*users).hh.tbl).hho as isize))
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
                                .offset(-((*(*users).hh.tbl).hho as isize))
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
                (*(*users).hh.tbl).tail = _hs_tail;
                users = (_hs_list as *mut libc::c_char)
                    .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut example_user_t;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as libc::c_uint);
        }
    }
    user = users;
    while !user.is_null() {
        printf(
            b"user %d, cookie %d\n\0" as *const u8 as *const libc::c_char,
            (*user).id,
            (*user).cookie,
        );
        user = (*user).hh.next as *mut example_user_t;
    }
    printf(b"adding 10-20\n\0" as *const u8 as *const libc::c_char);
    i = 20 as libc::c_int;
    while i >= 10 as libc::c_int {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        if user.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*user).id = i;
        (*user).cookie = i * i;
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
        let mut current_block_338: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv_0 = _ha_hashv_0
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_338 = 13649806185683973568;
            }
            10 => {
                current_block_338 = 13649806185683973568;
            }
            9 => {
                current_block_338 = 177781814286114453;
            }
            8 => {
                current_block_338 = 6782191668262527820;
            }
            7 => {
                current_block_338 = 12956768993232374699;
            }
            6 => {
                current_block_338 = 1195002048942084387;
            }
            5 => {
                current_block_338 = 18179026256847650081;
            }
            4 => {
                current_block_338 = 977741606773428974;
            }
            3 => {
                current_block_338 = 8205257777967751236;
            }
            2 => {
                current_block_338 = 8308901264692237116;
            }
            1 => {
                current_block_338 = 8124030124241512660;
            }
            _ => {
                current_block_338 = 6660696667549920226;
            }
        }
        match current_block_338 {
            13649806185683973568 => {
                _ha_hashv_0 = _ha_hashv_0
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_338 = 177781814286114453;
            }
            _ => {}
        }
        match current_block_338 {
            177781814286114453 => {
                _ha_hashv_0 = _ha_hashv_0
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_338 = 6782191668262527820;
            }
            _ => {}
        }
        match current_block_338 {
            6782191668262527820 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_338 = 12956768993232374699;
            }
            _ => {}
        }
        match current_block_338 {
            12956768993232374699 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_338 = 1195002048942084387;
            }
            _ => {}
        }
        match current_block_338 {
            1195002048942084387 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_338 = 18179026256847650081;
            }
            _ => {}
        }
        match current_block_338 {
            18179026256847650081 => {
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_338 = 977741606773428974;
            }
            _ => {}
        }
        match current_block_338 {
            977741606773428974 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_338 = 8205257777967751236;
            }
            _ => {}
        }
        match current_block_338 {
            8205257777967751236 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_338 = 8308901264692237116;
            }
            _ => {}
        }
        match current_block_338 {
            8308901264692237116 => {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_338 = 8124030124241512660;
            }
            _ => {}
        }
        match current_block_338 {
            8124030124241512660 => {
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
        (*user).hh.hashv = _ha_hashv_0;
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
            _he_new_buckets_0 = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets_0.is_null() {
                exit(-(1 as libc::c_int));
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
                free((*(*user).hh.tbl).buckets as *mut libc::c_void);
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
        }
        i -= 1;
        i;
    }
    user = users;
    while !user.is_null() {
        printf(
            b"user %d, cookie %d\n\0" as *const u8 as *const libc::c_char,
            (*user).id,
            (*user).cookie,
        );
        user = (*user).hh.next as *mut example_user_t;
    }
    printf(b"sorting\n\0" as *const u8 as *const libc::c_char);
    let mut _hs_i_0: libc::c_uint = 0;
    let mut _hs_looping_0: libc::c_uint = 0;
    let mut _hs_nmerges_0: libc::c_uint = 0;
    let mut _hs_insize_0: libc::c_uint = 0;
    let mut _hs_psize_0: libc::c_uint = 0;
    let mut _hs_qsize_0: libc::c_uint = 0;
    let mut _hs_p_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_q_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_e_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_list_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_tail_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    if !users.is_null() {
        _hs_insize_0 = 1 as libc::c_int as libc::c_uint;
        _hs_looping_0 = 1 as libc::c_int as libc::c_uint;
        _hs_list_0 = &mut (*users).hh;
        while _hs_looping_0 != 0 as libc::c_uint {
            _hs_p_0 = _hs_list_0;
            _hs_list_0 = 0 as *mut UT_hash_handle;
            _hs_tail_0 = 0 as *mut UT_hash_handle;
            _hs_nmerges_0 = 0 as libc::c_int as libc::c_uint;
            while !_hs_p_0.is_null() {
                _hs_nmerges_0 = _hs_nmerges_0.wrapping_add(1);
                _hs_nmerges_0;
                _hs_q_0 = _hs_p_0;
                _hs_psize_0 = 0 as libc::c_int as libc::c_uint;
                _hs_i_0 = 0 as libc::c_int as libc::c_uint;
                while _hs_i_0 < _hs_insize_0 {
                    _hs_psize_0 = _hs_psize_0.wrapping_add(1);
                    _hs_psize_0;
                    _hs_q_0 = if !((*_hs_q_0).next).is_null() {
                        ((*_hs_q_0).next as *mut libc::c_char)
                            .offset((*(*users).hh.tbl).hho as isize) as *mut libc::c_void
                            as *mut UT_hash_handle
                    } else {
                        0 as *mut UT_hash_handle
                    };
                    if _hs_q_0.is_null() {
                        break;
                    }
                    _hs_i_0 = _hs_i_0.wrapping_add(1);
                    _hs_i_0;
                }
                _hs_qsize_0 = _hs_insize_0;
                while _hs_psize_0 != 0 as libc::c_uint
                    || _hs_qsize_0 != 0 as libc::c_uint && !_hs_q_0.is_null()
                {
                    if _hs_psize_0 == 0 as libc::c_uint {
                        _hs_e_0 = _hs_q_0;
                        _hs_q_0 = if !((*_hs_q_0).next).is_null() {
                            ((*_hs_q_0).next as *mut libc::c_char)
                                .offset((*(*users).hh.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                        _hs_qsize_0;
                    } else if _hs_qsize_0 == 0 as libc::c_uint || _hs_q_0.is_null() {
                        _hs_e_0 = _hs_p_0;
                        if !_hs_p_0.is_null() {
                            _hs_p_0 = if !((*_hs_p_0).next).is_null() {
                                ((*_hs_p_0).next as *mut libc::c_char)
                                    .offset((*(*users).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                        _hs_psize_0;
                    } else if rev(
                        (_hs_p_0 as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t
                            as *mut libc::c_void,
                        (_hs_q_0 as *mut libc::c_char)
                            .offset(-((*(*users).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut example_user_t
                            as *mut libc::c_void,
                    ) <= 0 as libc::c_int
                    {
                        _hs_e_0 = _hs_p_0;
                        if !_hs_p_0.is_null() {
                            _hs_p_0 = if !((*_hs_p_0).next).is_null() {
                                ((*_hs_p_0).next as *mut libc::c_char)
                                    .offset((*(*users).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                        _hs_psize_0;
                    } else {
                        _hs_e_0 = _hs_q_0;
                        _hs_q_0 = if !((*_hs_q_0).next).is_null() {
                            ((*_hs_q_0).next as *mut libc::c_char)
                                .offset((*(*users).hh.tbl).hho as isize)
                                as *mut libc::c_void as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                        _hs_qsize_0;
                    }
                    if !_hs_tail_0.is_null() {
                        (*_hs_tail_0)
                            .next = if !_hs_e_0.is_null() {
                            (_hs_e_0 as *mut libc::c_char)
                                .offset(-((*(*users).hh.tbl).hho as isize))
                                as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        };
                    } else {
                        _hs_list_0 = _hs_e_0;
                    }
                    if !_hs_e_0.is_null() {
                        (*_hs_e_0)
                            .prev = if !_hs_tail_0.is_null() {
                            (_hs_tail_0 as *mut libc::c_char)
                                .offset(-((*(*users).hh.tbl).hho as isize))
                                as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        };
                    }
                    _hs_tail_0 = _hs_e_0;
                }
                _hs_p_0 = _hs_q_0;
            }
            if !_hs_tail_0.is_null() {
                (*_hs_tail_0).next = 0 as *mut libc::c_void;
            }
            if _hs_nmerges_0 <= 1 as libc::c_uint {
                _hs_looping_0 = 0 as libc::c_int as libc::c_uint;
                (*(*users).hh.tbl).tail = _hs_tail_0;
                users = (_hs_list_0 as *mut libc::c_char)
                    .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut example_user_t;
            }
            _hs_insize_0 = _hs_insize_0.wrapping_mul(2 as libc::c_uint);
        }
    }
    user = users;
    while !user.is_null() {
        printf(
            b"user %d, cookie %d\n\0" as *const u8 as *const libc::c_char,
            (*user).id,
            (*user).cookie,
        );
        user = (*user).hh.next as *mut example_user_t;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

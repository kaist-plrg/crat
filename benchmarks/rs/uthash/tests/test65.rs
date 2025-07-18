use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct CacheEntry {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub hh: UT_hash_handle,
}
pub static mut cache: *mut CacheEntry = 0 as *const CacheEntry as *mut CacheEntry;
unsafe extern "C" fn add_to_cache(
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut entry: *mut CacheEntry = 0 as *mut CacheEntry;
    let mut tmp_entry: *mut CacheEntry = 0 as *mut CacheEntry;
    entry = malloc(::std::mem::size_of::<CacheEntry>() as libc::c_ulong)
        as *mut CacheEntry;
    if entry.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*entry).key = strdup(key);
    (*entry).value = strdup(value);
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = (*entry).key as *const libc::c_uchar;
    _ha_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = strlen((*entry).key) as libc::c_uint;
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
    _ha_hashv = _ha_hashv.wrapping_add(strlen((*entry).key) as libc::c_uint);
    let mut current_block_57: u64;
    match _hj_k {
        11 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 17582427730828511370;
        }
        10 => {
            current_block_57 = 17582427730828511370;
        }
        9 => {
            current_block_57 = 541337446757090908;
        }
        8 => {
            current_block_57 = 8627143394729521319;
        }
        7 => {
            current_block_57 = 17150516651049684703;
        }
        6 => {
            current_block_57 = 7768484920969134428;
        }
        5 => {
            current_block_57 = 629556595112414427;
        }
        4 => {
            current_block_57 = 3253329295677155743;
        }
        3 => {
            current_block_57 = 3996865988069592899;
        }
        2 => {
            current_block_57 = 17946290042664438067;
        }
        1 => {
            current_block_57 = 2609838824379316571;
        }
        _ => {
            current_block_57 = 1434579379687443766;
        }
    }
    match current_block_57 {
        17582427730828511370 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 541337446757090908;
        }
        _ => {}
    }
    match current_block_57 {
        541337446757090908 => {
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 8627143394729521319;
        }
        _ => {}
    }
    match current_block_57 {
        8627143394729521319 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 17150516651049684703;
        }
        _ => {}
    }
    match current_block_57 {
        17150516651049684703 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 7768484920969134428;
        }
        _ => {}
    }
    match current_block_57 {
        7768484920969134428 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 629556595112414427;
        }
        _ => {}
    }
    match current_block_57 {
        629556595112414427 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_57 = 3253329295677155743;
        }
        _ => {}
    }
    match current_block_57 {
        3253329295677155743 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_57 = 3996865988069592899;
        }
        _ => {}
    }
    match current_block_57 {
        3996865988069592899 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_57 = 17946290042664438067;
        }
        _ => {}
    }
    match current_block_57 {
        17946290042664438067 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_57 = 2609838824379316571;
        }
        _ => {}
    }
    match current_block_57 {
        2609838824379316571 => {
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
    (*entry).hh.hashv = _ha_hashv;
    (*entry).hh.key = (*entry).key as *const libc::c_void;
    (*entry).hh.keylen = strlen((*entry).key) as libc::c_uint;
    if cache.is_null() {
        (*entry).hh.next = 0 as *mut libc::c_void;
        (*entry).hh.prev = 0 as *mut libc::c_void;
        (*entry)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*entry).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                (*entry).hh.tbl as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
            );
            (*(*entry).hh.tbl).tail = &mut (*entry).hh;
            (*(*entry).hh.tbl).num_buckets = 32 as libc::c_uint;
            (*(*entry).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
            (*(*entry).hh.tbl)
                .hho = (&mut (*entry).hh as *mut UT_hash_handle as *mut libc::c_char)
                .offset_from(entry as *mut libc::c_char) as libc::c_long;
            (*(*entry).hh.tbl)
                .buckets = malloc(
                (32 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            (*(*entry).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
            if ((*(*entry).hh.tbl).buckets).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*(*entry).hh.tbl).buckets as *mut libc::c_void,
                    '\0' as i32,
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
            }
        }
        cache = entry;
    } else {
        (*entry).hh.tbl = (*cache).hh.tbl;
        (*entry).hh.next = 0 as *mut libc::c_void;
        (*entry)
            .hh
            .prev = ((*(*cache).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*cache).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*(*cache).hh.tbl).tail).next = entry as *mut libc::c_void;
        (*(*cache).hh.tbl).tail = &mut (*entry).hh;
    }
    let mut _ha_bkt: libc::c_uint = 0;
    (*(*cache).hh.tbl).num_items = ((*(*cache).hh.tbl).num_items).wrapping_add(1);
    (*(*cache).hh.tbl).num_items;
    _ha_bkt = _ha_hashv
        & ((*(*cache).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*cache).hh.tbl).buckets)
        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
    (*_ha_head).count;
    (*entry).hh.hh_next = (*_ha_head).hh_head;
    (*entry).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*_ha_head).hh_head).is_null() {
        (*(*_ha_head).hh_head).hh_prev = &mut (*entry).hh;
    }
    (*_ha_head).hh_head = &mut (*entry).hh;
    if (*_ha_head).count
        >= ((*_ha_head).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint) && (*(*entry).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                .wrapping_mul((*(*entry).hh.tbl).num_buckets as libc::c_ulong)
                .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        } else {
            memset(
                _he_new_buckets as *mut libc::c_void,
                '\0' as i32,
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*entry).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            );
            (*(*entry).hh.tbl)
                .ideal_chain_maxlen = ((*(*entry).hh.tbl).num_items
                >> ((*(*entry).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                .wrapping_add(
                    (if (*(*entry).hh.tbl).num_items
                        & ((*(*entry).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                    {
                        1 as libc::c_uint
                    } else {
                        0 as libc::c_uint
                    }),
                );
            (*(*entry).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
            _he_bkt_i = 0 as libc::c_int as libc::c_uint;
            while _he_bkt_i < (*(*entry).hh.tbl).num_buckets {
                _he_thh = (*((*(*entry).hh.tbl).buckets).offset(_he_bkt_i as isize))
                    .hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*entry).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                        as *mut UT_hash_bucket;
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*entry).hh.tbl).ideal_chain_maxlen {
                        (*(*entry).hh.tbl)
                            .nonideal_items = ((*(*entry).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*(*entry).hh.tbl).nonideal_items;
                        if (*_he_newbkt).count
                            > ((*_he_newbkt).expand_mult)
                                .wrapping_mul((*(*entry).hh.tbl).ideal_chain_maxlen)
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
            free((*(*entry).hh.tbl).buckets as *mut libc::c_void);
            (*(*entry).hh.tbl)
                .num_buckets = ((*(*entry).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*entry).hh.tbl)
                .log2_num_buckets = ((*(*entry).hh.tbl).log2_num_buckets)
                .wrapping_add(1);
            (*(*entry).hh.tbl).log2_num_buckets;
            (*(*entry).hh.tbl).buckets = _he_new_buckets;
            (*(*entry).hh.tbl)
                .ineff_expands = if (*(*entry).hh.tbl).nonideal_items
                > (*(*entry).hh.tbl).num_items >> 1 as libc::c_int
            {
                ((*(*entry).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
            } else {
                0 as libc::c_uint
            };
            if (*(*entry).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*entry).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
            }
        }
    }
    if (if !cache.is_null() { (*(*cache).hh.tbl).num_items } else { 0 as libc::c_uint })
        >= 50 as libc::c_uint
    {
        entry = cache;
        tmp_entry = (if !cache.is_null() {
            (*cache).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut CacheEntry;
        if !entry.is_null() {
            printf(
                b"LRU deleting %s %s\n\0" as *const u8 as *const libc::c_char,
                (*entry).key,
                (*entry).value,
            );
            let mut _hd_hh_del: *mut UT_hash_handle = &mut (*entry).hh;
            if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
                free((*(*cache).hh.tbl).buckets as *mut libc::c_void);
                free((*cache).hh.tbl as *mut libc::c_void);
                cache = 0 as *mut CacheEntry;
            } else {
                let mut _hd_bkt: libc::c_uint = 0;
                if _hd_hh_del == (*(*cache).hh.tbl).tail {
                    (*(*cache).hh.tbl)
                        .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*cache).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle;
                }
                if !((*_hd_hh_del).prev).is_null() {
                    let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*cache).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh0 = (*_hd_hh_del).next;
                } else {
                    cache = (*_hd_hh_del).next as *mut CacheEntry;
                }
                if !((*_hd_hh_del).next).is_null() {
                    let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                        .offset((*(*cache).hh.tbl).hho as isize) as *mut libc::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh1 = (*_hd_hh_del).prev;
                }
                _hd_bkt = (*_hd_hh_del).hashv
                    & ((*(*cache).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*cache).hh.tbl)
                    .buckets)
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
                (*(*cache).hh.tbl)
                    .num_items = ((*(*cache).hh.tbl).num_items).wrapping_sub(1);
                (*(*cache).hh.tbl).num_items;
            }
            free((*entry).key as *mut libc::c_void);
            free((*entry).value as *mut libc::c_void);
            free(entry as *mut libc::c_void);
        }
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut linebuf: [libc::c_char; 100] = [0; 100];
    let mut nbuf: [libc::c_char; 11] = [0; 11];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    file = fopen(
        b"test65.dat\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        perror(b"can't open: \0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    while !(fgets(
        linebuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
        file,
    ))
        .is_null()
    {
        let fresh2 = i;
        i = i.wrapping_add(1);
        snprintf(
            nbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            fresh2,
        );
        add_to_cache(linebuf.as_mut_ptr(), nbuf.as_mut_ptr());
    }
    fclose(file);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

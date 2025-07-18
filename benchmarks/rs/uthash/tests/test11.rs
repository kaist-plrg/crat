use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_rec {
    pub boy_name: [libc::c_char; 20],
    pub hh: UT_hash_handle,
}
unsafe extern "C" fn namecmp(
    mut _a: *mut libc::c_void,
    mut _b: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut name_rec = _a as *mut name_rec;
    let mut b: *mut name_rec = _b as *mut name_rec;
    return strcmp(((*a).boy_name).as_mut_ptr(), ((*b).boy_name).as_mut_ptr());
}
unsafe fn main_0() -> libc::c_int {
    let mut name: *mut name_rec = 0 as *mut name_rec;
    let mut names: *mut name_rec = 0 as *mut name_rec;
    let mut linebuf: [libc::c_char; 20] = [0; 20];
    let mut file: *mut FILE = 0 as *mut FILE;
    file = fopen(
        b"test11.dat\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        perror(b"can't open: \0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    while !(fgets(linebuf.as_mut_ptr(), 20 as libc::c_int, file)).is_null() {
        name = malloc(::std::mem::size_of::<name_rec>() as libc::c_ulong)
            as *mut name_rec;
        if name.is_null() {
            exit(-(1 as libc::c_int));
        }
        strcpy(((*name).boy_name).as_mut_ptr(), linebuf.as_mut_ptr());
        let mut _uthash_hastr_keylen: libc::c_uint = strlen(
            ((*name).boy_name).as_mut_ptr(),
        ) as libc::c_uint;
        let mut _ha_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut *((*name).boy_name)
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
        let mut current_block_61: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_61 = 9509448830601559158;
            }
            10 => {
                current_block_61 = 9509448830601559158;
            }
            9 => {
                current_block_61 = 7181068393185102741;
            }
            8 => {
                current_block_61 = 5567015853417394809;
            }
            7 => {
                current_block_61 = 14762652318727335126;
            }
            6 => {
                current_block_61 = 17504133385821656526;
            }
            5 => {
                current_block_61 = 12388873576323557050;
            }
            4 => {
                current_block_61 = 3423167110703549460;
            }
            3 => {
                current_block_61 = 669308683109053207;
            }
            2 => {
                current_block_61 = 11482884981470717677;
            }
            1 => {
                current_block_61 = 876562227259399540;
            }
            _ => {
                current_block_61 = 11777552016271000781;
            }
        }
        match current_block_61 {
            9509448830601559158 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_61 = 7181068393185102741;
            }
            _ => {}
        }
        match current_block_61 {
            7181068393185102741 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_61 = 5567015853417394809;
            }
            _ => {}
        }
        match current_block_61 {
            5567015853417394809 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_61 = 14762652318727335126;
            }
            _ => {}
        }
        match current_block_61 {
            14762652318727335126 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_61 = 17504133385821656526;
            }
            _ => {}
        }
        match current_block_61 {
            17504133385821656526 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_61 = 12388873576323557050;
            }
            _ => {}
        }
        match current_block_61 {
            12388873576323557050 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_61 = 3423167110703549460;
            }
            _ => {}
        }
        match current_block_61 {
            3423167110703549460 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_61 = 669308683109053207;
            }
            _ => {}
        }
        match current_block_61 {
            669308683109053207 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_61 = 11482884981470717677;
            }
            _ => {}
        }
        match current_block_61 {
            11482884981470717677 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_61 = 876562227259399540;
            }
            _ => {}
        }
        match current_block_61 {
            876562227259399540 => {
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
        (*name).hh.hashv = _ha_hashv;
        (*name)
            .hh
            .key = &mut *((*name).boy_name)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_void;
        (*name).hh.keylen = _uthash_hastr_keylen;
        if names.is_null() {
            (*name).hh.next = 0 as *mut libc::c_void;
            (*name).hh.prev = 0 as *mut libc::c_void;
            (*name)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*name).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*name).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*name).hh.tbl).tail = &mut (*name).hh;
                (*(*name).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*name).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*name).hh.tbl)
                    .hho = (&mut (*name).hh as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(name as *mut libc::c_char) as libc::c_long;
                (*(*name).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*name).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*name).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*name).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            names = name;
        } else {
            (*name).hh.tbl = (*names).hh.tbl;
            (*name).hh.next = 0 as *mut libc::c_void;
            (*name)
                .hh
                .prev = ((*(*names).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*names).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*names).hh.tbl).tail).next = name as *mut libc::c_void;
            (*(*names).hh.tbl).tail = &mut (*name).hh;
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*names).hh.tbl).num_items = ((*(*names).hh.tbl).num_items).wrapping_add(1);
        (*(*names).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*names).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*names).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*name).hh.hh_next = (*_ha_head).hh_head;
        (*name).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*name).hh;
        }
        (*_ha_head).hh_head = &mut (*name).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*name).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*name).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*name).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*name).hh.tbl)
                    .ideal_chain_maxlen = ((*(*name).hh.tbl).num_items
                    >> ((*(*name).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*name).hh.tbl).num_items
                            & ((*(*name).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*name).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*name).hh.tbl).num_buckets {
                    _he_thh = (*((*(*name).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*name).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*name).hh.tbl).ideal_chain_maxlen {
                            (*(*name).hh.tbl)
                                .nonideal_items = ((*(*name).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*name).hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul((*(*name).hh.tbl).ideal_chain_maxlen)
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
                free((*(*name).hh.tbl).buckets as *mut libc::c_void);
                (*(*name).hh.tbl)
                    .num_buckets = ((*(*name).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*name).hh.tbl)
                    .log2_num_buckets = ((*(*name).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*name).hh.tbl).log2_num_buckets;
                (*(*name).hh.tbl).buckets = _he_new_buckets;
                (*(*name).hh.tbl)
                    .ineff_expands = if (*(*name).hh.tbl).nonideal_items
                    > (*(*name).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*name).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*name).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*name).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
    }
    fclose(file);
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
    if !names.is_null() {
        _hs_insize = 1 as libc::c_int as libc::c_uint;
        _hs_looping = 1 as libc::c_int as libc::c_uint;
        _hs_list = &mut (*names).hh;
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
                            .offset((*(*names).hh.tbl).hho as isize) as *mut libc::c_void
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
                                .offset((*(*names).hh.tbl).hho as isize)
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
                                    .offset((*(*names).hh.tbl).hho as isize)
                                    as *mut libc::c_void as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                        _hs_psize;
                    } else if namecmp(
                        (_hs_p as *mut libc::c_char)
                            .offset(-((*(*names).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut name_rec as *mut libc::c_void,
                        (_hs_q as *mut libc::c_char)
                            .offset(-((*(*names).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut name_rec as *mut libc::c_void,
                    ) <= 0 as libc::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = if !((*_hs_p).next).is_null() {
                                ((*_hs_p).next as *mut libc::c_char)
                                    .offset((*(*names).hh.tbl).hho as isize)
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
                                .offset((*(*names).hh.tbl).hho as isize)
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
                                .offset(-((*(*names).hh.tbl).hho as isize))
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
                                .offset(-((*(*names).hh.tbl).hho as isize))
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
                (*(*names).hh.tbl).tail = _hs_tail;
                names = (_hs_list as *mut libc::c_char)
                    .offset(-((*(*names).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut name_rec;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as libc::c_uint);
        }
    }
    name = names;
    while !name.is_null() {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            ((*name).boy_name).as_mut_ptr(),
        );
        name = (*name).hh.next as *mut name_rec;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

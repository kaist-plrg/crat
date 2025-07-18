use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn hash_init(
        ht: *mut hash_table,
        size: libc::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn jhash_string(key: *const libc::c_uchar) -> libc::c_uint;
    static mut hash_deleted_item: *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: libc::c_ulong,
    pub ht_capacity: libc::c_ulong,
    pub ht_fill: libc::c_ulong,
    pub ht_empty_slots: libc::c_ulong,
    pub ht_collisions: libc::c_ulong,
    pub ht_lookups: libc::c_ulong,
    pub ht_rehashes: libc::c_uint,
}
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
pub type sc_buflen_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strcache {
    pub next: *mut strcache,
    pub end: sc_buflen_t,
    pub bytesfree: sc_buflen_t,
    pub count: sc_buflen_t,
    pub buffer: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hugestring {
    pub next: *mut hugestring,
    pub buffer: [libc::c_char; 1],
}
static mut strcache: *mut strcache = 0 as *const strcache as *mut strcache;
static mut fullcache: *mut strcache = 0 as *const strcache as *mut strcache;
static mut total_buffers: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut total_strings: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut total_size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
unsafe extern "C" fn new_cache(
    mut head: *mut *mut strcache,
    mut buflen: sc_buflen_t,
) -> *mut strcache {
    let mut new: *mut strcache = xmalloc(
        (buflen as libc::c_ulong).wrapping_add(14 as libc::c_ulong),
    ) as *mut strcache;
    (*new).end = 0 as libc::c_int as sc_buflen_t;
    (*new).count = 0 as libc::c_int as sc_buflen_t;
    (*new).bytesfree = buflen;
    (*new).next = *head;
    *head = new;
    total_buffers = total_buffers.wrapping_add(1);
    total_buffers;
    return new;
}
unsafe extern "C" fn copy_string(
    mut sp: *mut strcache,
    mut str: *const libc::c_char,
    mut len: sc_buflen_t,
) -> *const libc::c_char {
    let mut res: *mut libc::c_char = &mut *((*sp).buffer)
        .as_mut_ptr()
        .offset((*sp).end as isize) as *mut libc::c_char;
    memmove(res as *mut libc::c_void, str as *const libc::c_void, len as libc::c_ulong);
    let fresh0 = len;
    len = len.wrapping_add(1);
    *res.offset(fresh0 as isize) = '\0' as i32 as libc::c_char;
    (*sp).end = ((*sp).end as libc::c_int + len as libc::c_int) as sc_buflen_t;
    (*sp)
        .bytesfree = ((*sp).bytesfree as libc::c_int - len as libc::c_int)
        as sc_buflen_t;
    (*sp).count = ((*sp).count).wrapping_add(1);
    (*sp).count;
    return res;
}
unsafe extern "C" fn add_string(
    mut str: *const libc::c_char,
    mut len: sc_buflen_t,
) -> *const libc::c_char {
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    let mut sp: *mut strcache = 0 as *mut strcache;
    let mut spp: *mut *mut strcache = &mut strcache;
    let mut sz: sc_buflen_t = (len as libc::c_int + 1 as libc::c_int) as sc_buflen_t;
    total_strings = total_strings.wrapping_add(1);
    total_strings;
    total_size = total_size.wrapping_add(sz as libc::c_ulong);
    if sz as libc::c_ulong
        > (8192 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            )
            .wrapping_sub(14 as libc::c_ulong)
    {
        sp = new_cache(&mut fullcache, sz);
        return copy_string(sp, str, len);
    }
    while !(*spp).is_null() {
        if (**spp).bytesfree as libc::c_int > sz as libc::c_int {
            break;
        }
        spp = &mut (**spp).next;
    }
    sp = *spp;
    if sp.is_null() {
        sp = new_cache(
            &mut strcache,
            (8192 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
                )
                .wrapping_sub(14 as libc::c_ulong) as sc_buflen_t,
        );
        spp = &mut strcache;
    }
    res = copy_string(sp, str, len);
    if total_strings > 20 as libc::c_int as libc::c_ulong
        && ((*sp).bytesfree as libc::c_ulong)
            < total_size
                .wrapping_div(total_strings)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        *spp = (*sp).next;
        (*sp).next = fullcache;
        fullcache = sp;
    }
    return res;
}
static mut hugestrings: *mut hugestring = 0 as *const hugestring as *mut hugestring;
unsafe extern "C" fn add_hugestring(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut new: *mut hugestring = xmalloc(
        (::std::mem::size_of::<hugestring>() as libc::c_ulong).wrapping_add(len),
    ) as *mut hugestring;
    memcpy(
        ((*new).buffer).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    *((*new).buffer).as_mut_ptr().offset(len as isize) = '\0' as i32 as libc::c_char;
    (*new).next = hugestrings;
    hugestrings = new;
    return ((*new).buffer).as_mut_ptr();
}
unsafe extern "C" fn str_hash_1(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = key as *const libc::c_char
        as *const libc::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn str_hash_2(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn str_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    return if x as *const libc::c_char == y as *const libc::c_char {
        0 as libc::c_int
    } else {
        strcmp(x as *const libc::c_char, y as *const libc::c_char)
    };
}
static mut strings: hash_table = hash_table {
    ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
};
static mut total_adds: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
unsafe extern "C" fn add_hash(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut slot: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    if len
        > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
            as libc::c_ulong
    {
        return add_hugestring(str, len);
    }
    slot = hash_find_slot(&mut strings, str as *const libc::c_void)
        as *const *mut libc::c_char;
    key = *slot;
    total_adds = total_adds.wrapping_add(1);
    total_adds;
    if !(key.is_null() || key as *mut libc::c_void == hash_deleted_item) {
        return key;
    }
    key = add_string(str, len as sc_buflen_t);
    hash_insert_at(
        &mut strings,
        key as *const libc::c_void,
        slot as *const libc::c_void,
    );
    return key;
}
pub unsafe extern "C" fn strcache_iscached(mut str: *const libc::c_char) -> libc::c_int {
    let mut sp: *mut strcache = 0 as *mut strcache;
    sp = strcache;
    while !sp.is_null() {
        if str >= ((*sp).buffer).as_mut_ptr() as *const libc::c_char
            && str
                < ((*sp).buffer).as_mut_ptr().offset((*sp).end as libc::c_int as isize)
                    as *const libc::c_char
        {
            return 1 as libc::c_int;
        }
        sp = (*sp).next;
    }
    sp = fullcache;
    while !sp.is_null() {
        if str >= ((*sp).buffer).as_mut_ptr() as *const libc::c_char
            && str
                < ((*sp).buffer).as_mut_ptr().offset((*sp).end as libc::c_int as isize)
                    as *const libc::c_char
        {
            return 1 as libc::c_int;
        }
        sp = (*sp).next;
    }
    let mut hp: *mut hugestring = 0 as *mut hugestring;
    hp = hugestrings;
    while !hp.is_null() {
        if str == ((*hp).buffer).as_mut_ptr() as *const libc::c_char {
            return 1 as libc::c_int;
        }
        hp = (*hp).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strcache_add(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    return add_hash(str, strlen(str));
}
pub unsafe extern "C" fn strcache_add_len(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    if *str.offset(len as isize) as libc::c_int != '\0' as i32 {
        let mut fresh1 = ::std::vec::from_elem(
            0,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut key: *mut libc::c_char = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(key as *mut libc::c_void, str as *const libc::c_void, len);
        *key.offset(len as isize) = '\0' as i32 as libc::c_char;
        str = key;
    }
    return add_hash(str, len);
}
pub unsafe extern "C" fn strcache_init() {
    hash_init(
        &mut strings,
        8000 as libc::c_int as libc::c_ulong,
        Some(str_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(str_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(
            str_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn strcache_print_stats(mut prefix: *const libc::c_char) {
    let mut sp: *const strcache = 0 as *const strcache;
    let mut numbuffs: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut fullbuffs: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut totfree: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut maxfree: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut minfree: libc::c_ulong = (8192 as libc::c_int as libc::c_ulong)
        .wrapping_sub(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        )
        .wrapping_sub(14 as libc::c_ulong);
    if strcache.is_null() {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\n%s No strcache buffers\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            prefix,
        );
        return;
    }
    sp = (*strcache).next;
    while !sp.is_null() {
        let mut bf: sc_buflen_t = (*sp).bytesfree;
        totfree = totfree.wrapping_add(bf as libc::c_ulong);
        maxfree = if bf as libc::c_ulong > maxfree {
            bf as libc::c_ulong
        } else {
            maxfree
        };
        minfree = if (bf as libc::c_ulong) < minfree {
            bf as libc::c_ulong
        } else {
            minfree
        };
        numbuffs = numbuffs.wrapping_add(1);
        numbuffs;
        sp = (*sp).next;
    }
    sp = fullcache;
    while !sp.is_null() {
        let mut bf_0: sc_buflen_t = (*sp).bytesfree;
        totfree = totfree.wrapping_add(bf_0 as libc::c_ulong);
        maxfree = if bf_0 as libc::c_ulong > maxfree {
            bf_0 as libc::c_ulong
        } else {
            maxfree
        };
        minfree = if (bf_0 as libc::c_ulong) < minfree {
            bf_0 as libc::c_ulong
        } else {
            minfree
        };
        numbuffs = numbuffs.wrapping_add(1);
        numbuffs;
        fullbuffs = fullbuffs.wrapping_add(1);
        fullbuffs;
        sp = (*sp).next;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\n%s strcache buffers: %lu (%lu) / strings = %lu / storage = %lu B / avg = %lu B\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        prefix,
        numbuffs.wrapping_add(1 as libc::c_int as libc::c_ulong),
        fullbuffs,
        total_strings,
        total_size,
        total_size.wrapping_div(total_strings),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"%s current buf: size = %hu B / used = %hu B / count = %hu / avg = %u B\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        prefix,
        (8192 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            )
            .wrapping_sub(14 as libc::c_ulong) as sc_buflen_t as libc::c_int,
        (*strcache).end as libc::c_int,
        (*strcache).count as libc::c_int,
        ((*strcache).end as libc::c_int / (*strcache).count as libc::c_int)
            as libc::c_uint,
    );
    if numbuffs != 0 {
        let mut sz: libc::c_ulong = total_size
            .wrapping_sub((*strcache).end as libc::c_ulong);
        let mut cnt: libc::c_ulong = total_strings
            .wrapping_sub((*strcache).count as libc::c_ulong);
        let mut avgfree: sc_buflen_t = totfree.wrapping_div(numbuffs) as sc_buflen_t;
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s other used: total = %lu B / count = %lu / avg = %lu B\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            prefix,
            sz,
            cnt,
            sz.wrapping_div(cnt),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s other free: total = %lu B / max = %lu B / min = %lu B / avg = %hu B\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            prefix,
            totfree,
            maxfree,
            minfree,
            avgfree as libc::c_int,
        );
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\n%s strcache performance: lookups = %lu / hit rate = %lu%%\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        prefix,
        total_adds,
        (100.0f64 * total_adds.wrapping_sub(total_strings) as libc::c_double
            / total_adds as libc::c_double) as libc::c_ulong,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"# hash-table stats:\n# \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    hash_print_stats(&mut strings, stdout);
}

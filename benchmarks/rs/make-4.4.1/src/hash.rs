use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_map_func_t = Option::<unsafe extern "C" fn(*const libc::c_void) -> ()>;
pub type hash_map_arg_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
>;
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
pub type qsort_cmp_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub static mut hash_deleted_item: *mut libc::c_void = unsafe {
    &hash_deleted_item as *const *mut libc::c_void as *mut *mut libc::c_void
        as *mut libc::c_void
};
pub unsafe extern "C" fn hash_init(
    mut ht: *mut hash_table,
    mut size: libc::c_ulong,
    mut hash_1: hash_func_t,
    mut hash_2: hash_func_t,
    mut hash_cmp: hash_cmp_func_t,
) {
    (*ht).ht_size = round_up_2(size);
    (*ht).ht_empty_slots = (*ht).ht_size;
    (*ht)
        .ht_vec = xcalloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul((*ht).ht_size),
    ) as *mut *mut libc::c_void;
    if ((*ht).ht_vec).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"can't allocate %lu bytes for hash table: memory exhausted\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ((*ht).ht_size)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
        exit(1 as libc::c_int);
    }
    (*ht)
        .ht_capacity = ((*ht).ht_size)
        .wrapping_sub(((*ht).ht_size).wrapping_div(16 as libc::c_int as libc::c_ulong));
    (*ht).ht_fill = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_collisions = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_lookups = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_rehashes = 0 as libc::c_int as libc::c_uint;
    (*ht).ht_hash_1 = hash_1;
    (*ht).ht_hash_2 = hash_2;
    (*ht).ht_compare = hash_cmp;
}
pub unsafe extern "C" fn hash_load(
    mut ht: *mut hash_table,
    mut item_table: *mut libc::c_void,
    mut cardinality: libc::c_ulong,
    mut size: libc::c_ulong,
) {
    let mut items: *mut libc::c_char = item_table as *mut libc::c_char;
    loop {
        let fresh0 = cardinality;
        cardinality = cardinality.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        hash_insert(ht, items as *const libc::c_void);
        items = items.offset(size as isize);
    };
}
pub unsafe extern "C" fn hash_find_slot(
    mut ht: *mut hash_table,
    mut key: *const libc::c_void,
) -> *mut *mut libc::c_void {
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut deleted_slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut hash_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hash_1: libc::c_uint = (Some(((*ht).ht_hash_1).unwrap())).unwrap()(key)
        as libc::c_uint;
    (*ht).ht_lookups = ((*ht).ht_lookups).wrapping_add(1);
    (*ht).ht_lookups;
    loop {
        hash_1 = (hash_1 as libc::c_ulong
            & ((*ht).ht_size).wrapping_sub(1 as libc::c_int as libc::c_ulong))
            as libc::c_uint;
        slot = &mut *((*ht).ht_vec).offset(hash_1 as isize) as *mut *mut libc::c_void;
        if (*slot).is_null() {
            return if !deleted_slot.is_null() { deleted_slot } else { slot };
        }
        if *slot == hash_deleted_item {
            if deleted_slot.is_null() {
                deleted_slot = slot;
            }
        } else {
            if key == *slot as *const libc::c_void {
                return slot;
            }
            if (Some(((*ht).ht_compare).unwrap())).unwrap()(key, *slot)
                == 0 as libc::c_int
            {
                return slot;
            }
            (*ht).ht_collisions = ((*ht).ht_collisions).wrapping_add(1);
            (*ht).ht_collisions;
        }
        if hash_2 == 0 {
            hash_2 = ((Some(((*ht).ht_hash_2).unwrap())).unwrap()(key)
                | 1 as libc::c_int as libc::c_ulong) as libc::c_uint;
        }
        hash_1 = hash_1.wrapping_add(hash_2);
    };
}
pub unsafe extern "C" fn hash_find_item(
    mut ht: *mut hash_table,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    let mut slot: *mut *mut libc::c_void = hash_find_slot(ht, key);
    return if (*slot).is_null() || *slot == hash_deleted_item {
        0 as *mut libc::c_void
    } else {
        *slot
    };
}
pub unsafe extern "C" fn hash_insert(
    mut ht: *mut hash_table,
    mut item: *const libc::c_void,
) -> *mut libc::c_void {
    let mut slot: *mut *mut libc::c_void = hash_find_slot(ht, item);
    let mut old_item: *const libc::c_void = *slot;
    hash_insert_at(ht, item, slot as *const libc::c_void);
    return (if old_item.is_null() || old_item as *mut libc::c_void == hash_deleted_item {
        0 as *const libc::c_void
    } else {
        old_item
    }) as *mut libc::c_void;
}
pub unsafe extern "C" fn hash_insert_at(
    mut ht: *mut hash_table,
    mut item: *const libc::c_void,
    mut slot: *const libc::c_void,
) -> *mut libc::c_void {
    let mut old_item: *const libc::c_void = *(slot as *mut *mut libc::c_void);
    if old_item.is_null() || old_item as *mut libc::c_void == hash_deleted_item {
        (*ht).ht_fill = ((*ht).ht_fill).wrapping_add(1);
        (*ht).ht_fill;
        if old_item.is_null() {
            (*ht).ht_empty_slots = ((*ht).ht_empty_slots).wrapping_sub(1);
            (*ht).ht_empty_slots;
        }
        old_item = item;
    }
    let ref mut fresh1 = *(slot as *mut *const libc::c_void);
    *fresh1 = item;
    if (*ht).ht_empty_slots < ((*ht).ht_size).wrapping_sub((*ht).ht_capacity) {
        hash_rehash(ht);
        return hash_find_slot(ht, item) as *mut libc::c_void;
    } else {
        return slot as *mut libc::c_void
    };
}
pub unsafe extern "C" fn hash_delete(
    mut ht: *mut hash_table,
    mut item: *const libc::c_void,
) -> *mut libc::c_void {
    let mut slot: *mut *mut libc::c_void = hash_find_slot(ht, item);
    return hash_delete_at(ht, slot as *const libc::c_void);
}
pub unsafe extern "C" fn hash_delete_at(
    mut ht: *mut hash_table,
    mut slot: *const libc::c_void,
) -> *mut libc::c_void {
    let mut item: *mut libc::c_void = *(slot as *mut *mut libc::c_void);
    if !(item.is_null() || item == hash_deleted_item) {
        let ref mut fresh2 = *(slot as *mut *const libc::c_void);
        *fresh2 = hash_deleted_item;
        (*ht).ht_fill = ((*ht).ht_fill).wrapping_sub(1);
        (*ht).ht_fill;
        return item;
    } else {
        return 0 as *mut libc::c_void
    };
}
pub unsafe extern "C" fn hash_free_items(mut ht: *mut hash_table) {
    let mut vec: *mut *mut libc::c_void = (*ht).ht_vec;
    let mut end: *mut *mut libc::c_void = &mut *vec.offset((*ht).ht_size as isize)
        as *mut *mut libc::c_void;
    while vec < end {
        let mut item: *mut libc::c_void = *vec;
        if !(item.is_null() || item == hash_deleted_item) {
            free(item);
        }
        *vec = 0 as *mut libc::c_void;
        vec = vec.offset(1);
        vec;
    }
    (*ht).ht_fill = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_empty_slots = (*ht).ht_size;
}
pub unsafe extern "C" fn hash_delete_items(mut ht: *mut hash_table) {
    let mut vec: *mut *mut libc::c_void = (*ht).ht_vec;
    let mut end: *mut *mut libc::c_void = &mut *vec.offset((*ht).ht_size as isize)
        as *mut *mut libc::c_void;
    while vec < end {
        *vec = 0 as *mut libc::c_void;
        vec = vec.offset(1);
        vec;
    }
    (*ht).ht_fill = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_collisions = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_lookups = 0 as libc::c_int as libc::c_ulong;
    (*ht).ht_rehashes = 0 as libc::c_int as libc::c_uint;
    (*ht).ht_empty_slots = (*ht).ht_size;
}
pub unsafe extern "C" fn hash_free(
    mut ht: *mut hash_table,
    mut free_items: libc::c_int,
) {
    if free_items != 0 {
        hash_free_items(ht);
    } else {
        (*ht).ht_fill = 0 as libc::c_int as libc::c_ulong;
        (*ht).ht_empty_slots = (*ht).ht_size;
    }
    free((*ht).ht_vec as *mut libc::c_void);
    (*ht).ht_vec = 0 as *mut *mut libc::c_void;
    (*ht).ht_capacity = 0 as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn hash_map(mut ht: *mut hash_table, mut map: hash_map_func_t) {
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut end: *mut *mut libc::c_void = &mut *((*ht).ht_vec)
        .offset((*ht).ht_size as isize) as *mut *mut libc::c_void;
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item) {
            (Some(map.unwrap())).unwrap()(*slot);
        }
        slot = slot.offset(1);
        slot;
    }
}
pub unsafe extern "C" fn hash_map_arg(
    mut ht: *mut hash_table,
    mut map: hash_map_arg_func_t,
    mut arg: *mut libc::c_void,
) {
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut end: *mut *mut libc::c_void = &mut *((*ht).ht_vec)
        .offset((*ht).ht_size as isize) as *mut *mut libc::c_void;
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item) {
            (Some(map.unwrap())).unwrap()(*slot, arg);
        }
        slot = slot.offset(1);
        slot;
    }
}
unsafe extern "C" fn hash_rehash(mut ht: *mut hash_table) {
    let mut old_ht_size: libc::c_ulong = (*ht).ht_size;
    let mut old_vec: *mut *mut libc::c_void = (*ht).ht_vec;
    let mut ovp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    if (*ht).ht_fill >= (*ht).ht_capacity {
        (*ht).ht_size = ((*ht).ht_size).wrapping_mul(2 as libc::c_int as libc::c_ulong);
        (*ht)
            .ht_capacity = ((*ht).ht_size)
            .wrapping_sub((*ht).ht_size >> 4 as libc::c_int);
    }
    (*ht).ht_rehashes = ((*ht).ht_rehashes).wrapping_add(1);
    (*ht).ht_rehashes;
    (*ht)
        .ht_vec = xcalloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul((*ht).ht_size),
    ) as *mut *mut libc::c_void;
    ovp = old_vec;
    while ovp < &mut *old_vec.offset(old_ht_size as isize) as *mut *mut libc::c_void {
        if !((*ovp).is_null() || *ovp == hash_deleted_item) {
            let mut slot: *mut *mut libc::c_void = hash_find_slot(ht, *ovp);
            *slot = *ovp;
        }
        ovp = ovp.offset(1);
        ovp;
    }
    (*ht).ht_empty_slots = ((*ht).ht_size).wrapping_sub((*ht).ht_fill);
    free(old_vec as *mut libc::c_void);
}
pub unsafe extern "C" fn hash_print_stats(
    mut ht: *mut hash_table,
    mut out_FILE: *mut FILE,
) {
    fprintf(
        out_FILE,
        dcgettext(
            0 as *const libc::c_char,
            b"Load=%lu/%lu=%.0f%%, \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*ht).ht_fill,
        (*ht).ht_size,
        100.0f64 * (*ht).ht_fill as libc::c_double / (*ht).ht_size as libc::c_double,
    );
    fprintf(
        out_FILE,
        dcgettext(
            0 as *const libc::c_char,
            b"Rehash=%u, \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*ht).ht_rehashes,
    );
    fprintf(
        out_FILE,
        dcgettext(
            0 as *const libc::c_char,
            b"Collisions=%lu/%lu=%.0f%%\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*ht).ht_collisions,
        (*ht).ht_lookups,
        if (*ht).ht_lookups != 0 {
            100.0f64 * (*ht).ht_collisions as libc::c_double
                / (*ht).ht_lookups as libc::c_double
        } else {
            0 as libc::c_int as libc::c_double
        },
    );
}
pub unsafe extern "C" fn hash_dump(
    mut ht: *mut hash_table,
    mut vector_0: *mut *mut libc::c_void,
    mut compare: qsort_cmp_t,
) -> *mut *mut libc::c_void {
    let mut vector: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut end: *mut *mut libc::c_void = &mut *((*ht).ht_vec)
        .offset((*ht).ht_size as isize) as *mut *mut libc::c_void;
    if vector_0.is_null() {
        vector_0 = xmalloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(
                    ((*ht).ht_fill).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut *mut libc::c_void;
    }
    vector = vector_0;
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item) {
            let fresh3 = vector;
            vector = vector.offset(1);
            *fresh3 = *slot;
        }
        slot = slot.offset(1);
        slot;
    }
    *vector = 0 as *mut libc::c_void;
    if compare.is_some() {
        qsort(
            vector_0 as *mut libc::c_void,
            (*ht).ht_fill,
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
            compare,
        );
    }
    return vector_0;
}
unsafe extern "C" fn round_up_2(mut n: libc::c_ulong) -> libc::c_ulong {
    n |= n >> 1 as libc::c_int;
    n |= n >> 2 as libc::c_int;
    n |= n >> 4 as libc::c_int;
    n |= n >> 8 as libc::c_int;
    n |= n >> 16 as libc::c_int;
    n |= n >> 32 as libc::c_int;
    return n.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn jhash(
    mut k: *const libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_uint {
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    c = (0xdeadbeef as libc::c_uint).wrapping_add(length as libc::c_uint);
    b = c;
    a = b;
    while length > 12 as libc::c_int {
        let mut val: libc::c_uint = 0;
        memcpy(
            &mut val as *mut libc::c_uint as *mut libc::c_void,
            k as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        a = a.wrapping_add(val);
        let mut val_0: libc::c_uint = 0;
        memcpy(
            &mut val_0 as *mut libc::c_uint as *mut libc::c_void,
            k.offset(4 as libc::c_int as isize) as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        b = b.wrapping_add(val_0);
        let mut val_1: libc::c_uint = 0;
        memcpy(
            &mut val_1 as *mut libc::c_uint as *mut libc::c_void,
            k.offset(8 as libc::c_int as isize) as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        c = c.wrapping_add(val_1);
        a = a.wrapping_sub(c);
        a ^= c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 8 as libc::c_int | b >> 32 as libc::c_int - 8 as libc::c_int;
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 4 as libc::c_int | b >> 32 as libc::c_int - 4 as libc::c_int;
        b = b.wrapping_add(a);
        length -= 12 as libc::c_int;
        k = k.offset(12 as libc::c_int as isize);
    }
    if length == 0 {
        return c;
    }
    if length > 8 as libc::c_int {
        let mut val_2: libc::c_uint = 0;
        memcpy(
            &mut val_2 as *mut libc::c_uint as *mut libc::c_void,
            k as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        a = a.wrapping_add(val_2);
        length -= 4 as libc::c_int;
        k = k.offset(4 as libc::c_int as isize);
    }
    if length > 4 as libc::c_int {
        let mut val_3: libc::c_uint = 0;
        memcpy(
            &mut val_3 as *mut libc::c_uint as *mut libc::c_void,
            k as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        b = b.wrapping_add(val_3);
        length -= 4 as libc::c_int;
        k = k.offset(4 as libc::c_int as isize);
    }
    if length == 4 as libc::c_int {
        c = c
            .wrapping_add(
                (*k.offset(3 as libc::c_int as isize) as libc::c_uint)
                    << 24 as libc::c_int,
            );
    }
    if length >= 3 as libc::c_int {
        c = c
            .wrapping_add(
                (*k.offset(2 as libc::c_int as isize) as libc::c_uint)
                    << 16 as libc::c_int,
            );
    }
    if length >= 2 as libc::c_int {
        c = c
            .wrapping_add(
                (*k.offset(1 as libc::c_int as isize) as libc::c_uint)
                    << 8 as libc::c_int,
            );
    }
    c = c.wrapping_add(*k.offset(0 as libc::c_int as isize) as libc::c_uint);
    c ^= b;
    c = c
        .wrapping_sub(
            b << 14 as libc::c_int | b >> 32 as libc::c_int - 14 as libc::c_int,
        );
    a ^= c;
    a = a
        .wrapping_sub(
            c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int,
        );
    b ^= a;
    b = b
        .wrapping_sub(
            a << 25 as libc::c_int | a >> 32 as libc::c_int - 25 as libc::c_int,
        );
    c ^= b;
    c = c
        .wrapping_sub(
            b << 16 as libc::c_int | b >> 32 as libc::c_int - 16 as libc::c_int,
        );
    a ^= c;
    a = a
        .wrapping_sub(c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int);
    b ^= a;
    b = b
        .wrapping_sub(
            a << 14 as libc::c_int | a >> 32 as libc::c_int - 14 as libc::c_int,
        );
    c ^= b;
    c = c
        .wrapping_sub(
            b << 24 as libc::c_int | b >> 32 as libc::c_int - 24 as libc::c_int,
        );
    return c;
}
pub unsafe extern "C" fn jhash_string(mut k: *const libc::c_uchar) -> libc::c_uint {
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut have_nul: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut start: *const libc::c_uchar = k;
    let mut klen: size_t = strlen(k as *const libc::c_char);
    c = 0xdeadbeef as libc::c_uint;
    b = c;
    a = b;
    loop {
        let mut val: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut pn: size_t = klen;
        if pn >= ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong {
            memcpy(
                &mut val as *mut libc::c_uint as *mut libc::c_void,
                k as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
            );
        } else {
            memcpy(
                &mut val as *mut libc::c_uint as *mut libc::c_void,
                k as *const libc::c_void,
                pn,
            );
        }
        have_nul = val.wrapping_sub(0x1010101 as libc::c_int as libc::c_uint) & !val
            & 0x80808080 as libc::c_uint;
        if have_nul == 0 {
            a = a.wrapping_add(val);
        } else if val & 0xff as libc::c_int as libc::c_uint != 0 {
            if val & 0xff00 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                a = a.wrapping_add(val & 0xff as libc::c_int as libc::c_uint);
            } else if val & 0xff0000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                a = a.wrapping_add(val & 0xffff as libc::c_int as libc::c_uint);
            } else {
                a = a.wrapping_add(val);
            }
        }
        if have_nul != 0 {
            break;
        }
        k = k.offset(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as isize);
        klen = (klen as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as size_t as size_t;
        let mut val_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut pn_0: size_t = klen;
        if pn_0 >= ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong {
            memcpy(
                &mut val_0 as *mut libc::c_uint as *mut libc::c_void,
                k as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
            );
        } else {
            memcpy(
                &mut val_0 as *mut libc::c_uint as *mut libc::c_void,
                k as *const libc::c_void,
                pn_0,
            );
        }
        have_nul = val_0.wrapping_sub(0x1010101 as libc::c_int as libc::c_uint) & !val_0
            & 0x80808080 as libc::c_uint;
        if have_nul == 0 {
            b = b.wrapping_add(val_0);
        } else if val_0 & 0xff as libc::c_int as libc::c_uint != 0 {
            if val_0 & 0xff00 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                b = b.wrapping_add(val_0 & 0xff as libc::c_int as libc::c_uint);
            } else if val_0 & 0xff0000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                b = b.wrapping_add(val_0 & 0xffff as libc::c_int as libc::c_uint);
            } else {
                b = b.wrapping_add(val_0);
            }
        }
        if have_nul != 0 {
            break;
        }
        k = k.offset(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as isize);
        klen = (klen as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as size_t as size_t;
        let mut val_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut pn_1: size_t = klen;
        if pn_1 >= ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong {
            memcpy(
                &mut val_1 as *mut libc::c_uint as *mut libc::c_void,
                k as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
            );
        } else {
            memcpy(
                &mut val_1 as *mut libc::c_uint as *mut libc::c_void,
                k as *const libc::c_void,
                pn_1,
            );
        }
        have_nul = val_1.wrapping_sub(0x1010101 as libc::c_int as libc::c_uint) & !val_1
            & 0x80808080 as libc::c_uint;
        if have_nul == 0 {
            c = c.wrapping_add(val_1);
        } else if val_1 & 0xff as libc::c_int as libc::c_uint != 0 {
            if val_1 & 0xff00 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                c = c.wrapping_add(val_1 & 0xff as libc::c_int as libc::c_uint);
            } else if val_1 & 0xff0000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                c = c.wrapping_add(val_1 & 0xffff as libc::c_int as libc::c_uint);
            } else {
                c = c.wrapping_add(val_1);
            }
        }
        if have_nul != 0 {
            break;
        }
        k = k.offset(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as isize);
        klen = (klen as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as size_t as size_t;
        a = a.wrapping_sub(c);
        a ^= c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 8 as libc::c_int | b >> 32 as libc::c_int - 8 as libc::c_int;
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 4 as libc::c_int | b >> 32 as libc::c_int - 4 as libc::c_int;
        b = b.wrapping_add(a);
    }
    c ^= b;
    c = c
        .wrapping_sub(
            b << 14 as libc::c_int | b >> 32 as libc::c_int - 14 as libc::c_int,
        );
    a ^= c;
    a = a
        .wrapping_sub(
            c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int,
        );
    b ^= a;
    b = b
        .wrapping_sub(
            a << 25 as libc::c_int | a >> 32 as libc::c_int - 25 as libc::c_int,
        );
    c ^= b;
    c = c
        .wrapping_sub(
            b << 16 as libc::c_int | b >> 32 as libc::c_int - 16 as libc::c_int,
        );
    a ^= c;
    a = a
        .wrapping_sub(c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int);
    b ^= a;
    b = b
        .wrapping_sub(
            a << 14 as libc::c_int | a >> 32 as libc::c_int - 14 as libc::c_int,
        );
    c ^= b;
    c = c
        .wrapping_sub(
            b << 24 as libc::c_int | b >> 32 as libc::c_int - 24 as libc::c_int,
        );
    return c.wrapping_add(k.offset_from(start) as libc::c_long as libc::c_uint);
}

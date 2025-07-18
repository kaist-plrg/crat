use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ck_ht_map;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn gh_log_die() -> !;
    fn ck_ht_next(
        _: *mut ck_ht_t,
        _: *mut ck_ht_iterator_t,
        entry: *mut *mut ck_ht_entry_t,
    ) -> bool;
    fn ck_ht_hash(
        _: *mut ck_ht_hash_t,
        _: *mut ck_ht_t,
        _: *const libc::c_void,
        _: uint16_t,
    );
    fn ck_ht_get_spmc(_: *mut ck_ht_t, _: ck_ht_hash_t, _: *mut ck_ht_entry_t) -> bool;
    fn ck_ht_init(
        _: *mut ck_ht_t,
        _: libc::c_uint,
        _: Option::<ck_ht_hash_cb_t>,
        _: *mut ck_malloc,
        _: uint64_t,
        _: uint64_t,
    ) -> bool;
    fn ck_ht_count(_: *mut ck_ht_t) -> uint64_t;
    fn ck_ht_put_spmc(_: *mut ck_ht_t, _: ck_ht_hash_t, _: *mut ck_ht_entry_t) -> bool;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_spinlock_t = libc::c_int;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type value_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_metric {
    pub next: *mut brubeck_metric,
    pub tags: *const brubeck_tag_set,
    pub lock: pthread_spinlock_t,
    pub key_len: uint16_t,
    pub type_0: uint8_t,
    pub private_state: uint8_t,
    pub as_0: C2RustUnnamed,
    pub key: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub gauge: C2RustUnnamed_1,
    pub meter: C2RustUnnamed_1,
    pub counter: C2RustUnnamed_0,
    pub histogram: brubeck_histo,
    pub other: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo {
    pub values: *mut value_t,
    pub count: uint32_t,
    pub alloc: uint16_t,
    pub size: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub value: value_t,
    pub previous: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub value: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag_set {
    pub index: uint32_t,
    pub tag_len: uint16_t,
    pub num_tags: uint16_t,
    pub tags: [brubeck_tag; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_hashtable_t {
    pub table: ck_ht_t,
    pub write_mutex: pthread_mutex_t,
}
pub type ck_ht_t = ck_ht;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht {
    pub m: *mut ck_malloc,
    pub map: *mut ck_ht_map,
    pub mode: libc::c_uint,
    pub seed: uint64_t,
    pub h: Option::<ck_ht_hash_cb_t>,
}
pub type ck_ht_hash_cb_t = unsafe extern "C" fn(
    *mut ck_ht_hash_t,
    *const libc::c_void,
    size_t,
    uint64_t,
) -> ();
pub type ck_ht_hash_t = ck_ht_hash;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht_hash {
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_malloc {
    pub malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub realloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            size_t,
            bool,
        ) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t, bool) -> ()>,
}
pub type uintptr_t = libc::c_ulong;
pub type ck_ht_entry_t = ck_ht_entry;
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct ck_ht_entry { pub inner: ck_ht_entry_Inner }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht_entry_Inner {
    pub key: uintptr_t,
    pub value: uintptr_t,
    pub key_length: uint64_t,
    pub hash: uint64_t,
}
#[allow(dead_code, non_upper_case_globals)]
const ck_ht_entry_PADDING: usize = ::std::mem::size_of::<ck_ht_entry>()
    - ::std::mem::size_of::<ck_ht_entry_Inner>();
pub type ck_ht_iterator_t = ck_ht_iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht_iterator {
    pub current: *mut ck_ht_entry,
    pub offset: uint64_t,
}
#[inline]
unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(size);
    if (ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn ck_ht_entry_set(
    mut entry: *mut ck_ht_entry,
    mut h: ck_ht_hash_t,
    mut key: *const libc::c_void,
    mut key_length: uint16_t,
    mut value: *const libc::c_void,
) {
    (*entry).inner.key = key as uintptr_t;
    (*entry).inner.value = value as uintptr_t;
    (*entry).inner.key_length = key_length as uint64_t;
    (*entry).inner.hash = h.value;
}
#[inline]
unsafe extern "C" fn ck_ht_entry_key_set(
    mut entry: *mut ck_ht_entry_t,
    mut key: *const libc::c_void,
    mut key_length: uint16_t,
) {
    (*entry).inner.key = key as uintptr_t;
    (*entry).inner.key_length = key_length as uint64_t;
}
#[inline]
unsafe extern "C" fn ck_ht_entry_value(
    mut entry: *mut ck_ht_entry_t,
) -> *mut libc::c_void {
    return (*entry).inner.value as *mut libc::c_void;
}
unsafe extern "C" fn ht_malloc(mut r: size_t) -> *mut libc::c_void {
    return xmalloc(r);
}
unsafe extern "C" fn ht_free(mut p: *mut libc::c_void, mut b: size_t, mut r: bool) {
    free(p);
}
static mut ALLOCATOR: ck_malloc = unsafe {
    {
        let mut init = ck_malloc {
            malloc: Some(ht_malloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            realloc: None,
            free: Some(
                ht_free as unsafe extern "C" fn(*mut libc::c_void, size_t, bool) -> (),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn brubeck_hashtable_new(
    size: uint64_t,
) -> *mut brubeck_hashtable_t {
    let mut ht: *mut brubeck_hashtable_t = xmalloc(
        ::std::mem::size_of::<brubeck_hashtable_t>() as libc::c_ulong,
    ) as *mut brubeck_hashtable_t;
    pthread_mutex_init(&mut (*ht).write_mutex, 0 as *const pthread_mutexattr_t);
    if !ck_ht_init(
        &mut (*ht).table,
        2 as libc::c_uint,
        None,
        &mut ALLOCATOR,
        size,
        0xdeadbeef as libc::c_uint as uint64_t,
    ) {
        free(ht as *mut libc::c_void);
        return 0 as *mut brubeck_hashtable_t;
    }
    return ht;
}
pub unsafe extern "C" fn brubeck_hashtable_free(mut ht: *mut brubeck_hashtable_t) {}
pub unsafe extern "C" fn brubeck_hashtable_find(
    mut ht: *mut brubeck_hashtable_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
) -> *mut brubeck_metric {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry { inner: ck_ht_entry_Inner {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    }};
    ck_ht_hash(&mut h, &mut (*ht).table, key as *const libc::c_void, key_len);
    ck_ht_entry_key_set(&mut entry, key as *const libc::c_void, key_len);
    if ck_ht_get_spmc(&mut (*ht).table, h, &mut entry) {
        return ck_ht_entry_value(&mut entry) as *mut brubeck_metric;
    }
    return 0 as *mut brubeck_metric;
}
pub unsafe extern "C" fn brubeck_hashtable_insert(
    mut ht: *mut brubeck_hashtable_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
    mut val: *mut brubeck_metric,
) -> bool {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry { inner: ck_ht_entry_Inner {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    }};
    let mut result: bool = false;
    ck_ht_hash(&mut h, &mut (*ht).table, key as *const libc::c_void, key_len);
    ck_ht_entry_set(
        &mut entry,
        h,
        key as *const libc::c_void,
        key_len,
        val as *const libc::c_void,
    );
    pthread_mutex_lock(&mut (*ht).write_mutex);
    result = ck_ht_put_spmc(&mut (*ht).table, h, &mut entry);
    pthread_mutex_unlock(&mut (*ht).write_mutex);
    return result;
}
pub unsafe extern "C" fn brubeck_hashtable_size(
    mut ht: *mut brubeck_hashtable_t,
) -> size_t {
    let mut len: size_t = 0;
    pthread_mutex_lock(&mut (*ht).write_mutex);
    len = ck_ht_count(&mut (*ht).table);
    pthread_mutex_unlock(&mut (*ht).write_mutex);
    return len;
}
pub unsafe extern "C" fn brubeck_hashtable_foreach(
    mut ht: *mut brubeck_hashtable_t,
    mut callback: Option::<
        unsafe extern "C" fn(*mut brubeck_metric, *mut libc::c_void) -> (),
    >,
    mut payload: *mut libc::c_void,
) {
    let mut iterator: ck_ht_iterator_t = {
        let mut init = ck_ht_iterator {
            current: 0 as *mut ck_ht_entry,
            offset: 0 as libc::c_int as uint64_t,
        };
        init
    };
    let mut entry: *mut ck_ht_entry_t = 0 as *mut ck_ht_entry_t;
    pthread_mutex_lock(&mut (*ht).write_mutex);
    while ck_ht_next(&mut (*ht).table, &mut iterator, &mut entry) {
        callback.unwrap()(ck_ht_entry_value(entry) as *mut brubeck_metric, payload);
    }
    pthread_mutex_unlock(&mut (*ht).write_mutex);
}
pub unsafe extern "C" fn brubeck_hashtable_to_a(
    mut ht: *mut brubeck_hashtable_t,
    mut length: *mut size_t,
) -> *mut *mut brubeck_metric {
    let mut iterator: ck_ht_iterator_t = {
        let mut init = ck_ht_iterator {
            current: 0 as *mut ck_ht_entry,
            offset: 0 as libc::c_int as uint64_t,
        };
        init
    };
    let mut entry: *mut ck_ht_entry_t = 0 as *mut ck_ht_entry_t;
    let mut array: *mut *mut brubeck_metric = 0 as *mut *mut brubeck_metric;
    let mut i: size_t = 0 as libc::c_int as size_t;
    pthread_mutex_lock(&mut (*ht).write_mutex);
    *length = ck_ht_count(&mut (*ht).table);
    array = xmalloc(
        (*length)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut brubeck_metric;
    while ck_ht_next(&mut (*ht).table, &mut iterator, &mut entry) {
        let fresh0 = i;
        i = i.wrapping_add(1);
        let ref mut fresh1 = *array.offset(fresh0 as isize);
        *fresh1 = ck_ht_entry_value(entry) as *mut brubeck_metric;
    }
    pthread_mutex_unlock(&mut (*ht).write_mutex);
    return array;
}

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ck_ht_map;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn gh_log_die() -> !;
    fn ck_ht_hash(
        _: *mut ck_ht_hash_t,
        _: *mut ck_ht_t,
        _: *const libc::c_void,
        _: uint16_t,
    );
    fn ck_ht_put_spmc(_: *mut ck_ht_t, _: ck_ht_hash_t, _: *mut ck_ht_entry_t) -> bool;
    fn ck_ht_get_spmc(_: *mut ck_ht_t, _: ck_ht_hash_t, _: *mut ck_ht_entry_t) -> bool;
    fn ck_ht_init(
        _: *mut ck_ht_t,
        _: libc::c_uint,
        _: Option::<ck_ht_hash_cb_t>,
        _: *mut ck_malloc,
        _: uint64_t,
        _: uint64_t,
    ) -> bool;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
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
pub struct brubeck_tags_t {
    pub num_tag_sets: uint32_t,
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
static mut char_assoc: libc::c_char = '=' as i32 as libc::c_char;
static mut str_assoc: *const libc::c_char = b"=\0" as *const u8 as *const libc::c_char;
static mut str_delim: *const libc::c_char = b",\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn tags_malloc(mut r: size_t) -> *mut libc::c_void {
    return xmalloc(r);
}
unsafe extern "C" fn tags_free(mut p: *mut libc::c_void, mut b: size_t, mut r: bool) {
    free(p);
}
static mut ALLOCATOR: ck_malloc = unsafe {
    {
        let mut init = ck_malloc {
            malloc: Some(
                tags_malloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
            ),
            realloc: None,
            free: Some(
                tags_free as unsafe extern "C" fn(*mut libc::c_void, size_t, bool) -> (),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn brubeck_tags_create(size: uint64_t) -> *mut brubeck_tags_t {
    let mut tags: *mut brubeck_tags_t = xmalloc(
        ::std::mem::size_of::<brubeck_tags_t>() as libc::c_ulong,
    ) as *mut brubeck_tags_t;
    memset(
        tags as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_tags_t>() as libc::c_ulong,
    );
    pthread_mutex_init(&mut (*tags).write_mutex, 0 as *const pthread_mutexattr_t);
    if !ck_ht_init(
        &mut (*tags).table,
        2 as libc::c_uint,
        None,
        &mut ALLOCATOR,
        size,
        0xbadf00d as libc::c_int as uint64_t,
    ) {
        free(tags as *mut libc::c_void);
        return 0 as *mut brubeck_tags_t;
    }
    return tags;
}
pub unsafe extern "C" fn brubeck_tags_insert(
    mut tags: *mut brubeck_tags_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
    mut val: *mut brubeck_tag_set,
) -> bool {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry { inner: ck_ht_entry_Inner {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    }};
    let mut result: bool = false;
    ck_ht_hash(&mut h, &mut (*tags).table, key as *const libc::c_void, key_len);
    ck_ht_entry_set(
        &mut entry,
        h,
        key as *const libc::c_void,
        key_len,
        val as *const libc::c_void,
    );
    pthread_mutex_lock(&mut (*tags).write_mutex);
    result = ck_ht_put_spmc(&mut (*tags).table, h, &mut entry);
    if result {
        (*tags).num_tag_sets = ((*tags).num_tag_sets).wrapping_add(1);
        (*val).index = (*tags).num_tag_sets;
    }
    pthread_mutex_unlock(&mut (*tags).write_mutex);
    return result;
}
pub unsafe extern "C" fn brubeck_tags_find(
    mut tags: *mut brubeck_tags_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
) -> *mut brubeck_tag_set {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry { inner: ck_ht_entry_Inner {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    }};
    ck_ht_hash(&mut h, &mut (*tags).table, key as *const libc::c_void, key_len);
    ck_ht_entry_key_set(&mut entry, key as *const libc::c_void, key_len);
    if ck_ht_get_spmc(&mut (*tags).table, h, &mut entry) {
        return ck_ht_entry_value(&mut entry) as *mut brubeck_tag_set;
    }
    return 0 as *mut brubeck_tag_set;
}
pub unsafe extern "C" fn brubeck_tag_offset(mut str: *const libc::c_char) -> uint16_t {
    let mut offset: uint16_t = 0 as libc::c_int as uint16_t;
    offset = 0 as libc::c_int as uint16_t;
    while *str as libc::c_int != 0 && *str as libc::c_int != ',' as i32
        && *str as libc::c_int != '#' as i32
    {
        str = str.offset(1);
        str;
        offset = offset.wrapping_add(1);
        offset;
    }
    return offset;
}
pub unsafe extern "C" fn count_char_in_str(
    mut str: *const libc::c_char,
    mut c: libc::c_char,
) -> uint16_t {
    let mut count: uint16_t = 0;
    count = 0 as libc::c_int as uint16_t;
    while *str.offset(count as isize) != 0 {
        if *str.offset(count as isize) as libc::c_int == c as libc::c_int {
            count = count.wrapping_add(1);
            count;
        } else {
            c += 1;
            c;
        };
    }
    return count;
}
pub unsafe extern "C" fn parse_tag(
    mut kv_str: *mut libc::c_char,
    mut tag: *mut brubeck_tag,
) -> bool {
    let mut state: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    key = strtok_r(kv_str, str_assoc, &mut state);
    if key.is_null() {
        return 0 as libc::c_int != 0;
    }
    value = strtok_r(0 as *mut libc::c_char, str_assoc, &mut state);
    if value.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*tag).key = key;
    (*tag).value = value;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn brubeck_parse_tags(
    mut tag_str: *mut libc::c_char,
    mut tag_str_len: uint16_t,
) -> *mut brubeck_tag_set {
    let mut state: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_possible_tags: uint16_t = (if !tag_str.is_null() {
        count_char_in_str(tag_str, char_assoc) as libc::c_int
    } else {
        0 as libc::c_int
    }) as uint16_t;
    let mut alloc_size: size_t = (::std::mem::size_of::<brubeck_tag_set>()
        as libc::c_ulong)
        .wrapping_add(
            (num_possible_tags as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut brubeck_tag>() as libc::c_ulong),
        );
    let mut tag_set: *mut brubeck_tag_set = malloc(alloc_size) as *mut brubeck_tag_set;
    memset(tag_set as *mut libc::c_void, 0 as libc::c_int, alloc_size);
    if !tag_str.is_null() {
        (*tag_set).tag_len = tag_str_len;
        tag_str = strtok_r(tag_str, str_delim, &mut state);
        while !tag_str.is_null() {
            if parse_tag(
                tag_str,
                &mut *((*tag_set).tags).as_mut_ptr().offset((*tag_set).num_tags as isize),
            ) {
                (*tag_set).num_tags = ((*tag_set).num_tags).wrapping_add(1);
                (*tag_set).num_tags;
            }
            tag_str = strtok_r(0 as *mut libc::c_char, str_delim, &mut state);
        }
    }
    return tag_set;
}
pub unsafe extern "C" fn brubeck_get_tag_set_of_tag_str(
    mut tags: *mut brubeck_tags_t,
    mut tag_str: *const libc::c_char,
    mut tag_str_len: uint16_t,
) -> *const brubeck_tag_set {
    let mut tag_set: *mut brubeck_tag_set = 0 as *mut brubeck_tag_set;
    tag_set = brubeck_tags_find(tags, tag_str, tag_str_len);
    if tag_set.is_null() {
        let mut tag_str_for_parse: *mut libc::c_char = strdup(tag_str);
        let mut tag_str_for_key: *mut libc::c_char = strdup(tag_str);
        tag_set = brubeck_parse_tags(tag_str_for_parse, tag_str_len);
        if !brubeck_tags_insert(tags, tag_str_for_key, tag_str_len, tag_set) {
            free(tag_set as *mut libc::c_void);
            free(tag_str_for_parse as *mut libc::c_void);
            free(tag_str_for_key as *mut libc::c_void);
            tag_set = brubeck_tags_find(tags, tag_str, tag_str_len);
        }
    }
    return tag_set;
}
pub unsafe extern "C" fn brubeck_get_tag_set(
    mut tags: *mut brubeck_tags_t,
    mut key_str: *const libc::c_char,
    mut key_len: uint16_t,
) -> *const brubeck_tag_set {
    let mut tag_offset: uint16_t = 0;
    tag_offset = brubeck_tag_offset(key_str);
    return brubeck_get_tag_set_of_tag_str(
        tags,
        key_str.offset(tag_offset as libc::c_int as isize),
        (key_len as libc::c_int - tag_offset as libc::c_int) as uint16_t,
    );
}

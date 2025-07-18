use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type u_int32_t = __uint32_t;
pub type method = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASH_T {
    pub size: libc::c_int,
    pub entries: libc::c_int,
    pub index: libc::c_int,
    pub table: *mut *mut NODE,
    pub free: method,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NODE {
    pub key: *mut libc::c_char,
    pub val: *mut libc::c_void,
    pub next: *mut NODE,
}
pub type HASH = *mut HASH_T;
pub static mut HASHSIZE: size_t = ::std::mem::size_of::<HASH_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_hash() -> HASH {
    let mut this: HASH = 0 as *mut HASH_T;
    let mut size: libc::c_int = 10240 as libc::c_int;
    this = calloc(HASHSIZE, 1 as libc::c_int as libc::c_ulong) as HASH;
    (*this).size = size;
    (*this).entries = 0 as libc::c_int;
    (*this).index = 0 as libc::c_int;
    while (*this).size < size {
        (*this).size <<= 1 as libc::c_int;
    }
    (*this)
        .table = calloc(
        ((*this).size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut *mut NODE;
    (*this).free = None;
    return this;
}
pub unsafe extern "C" fn hash_size(mut this: HASH) -> libc::c_int {
    return (*this).entries;
}
pub unsafe extern "C" fn hash_reset(mut this: HASH, mut size: ssize_t) {
    (*this).size = 2 as libc::c_int;
    (*this).entries = 0 as libc::c_int;
    while ((*this).size as libc::c_long) < size {
        (*this).size <<= 1 as libc::c_int;
    }
    (*this)
        .table = calloc(
        ((*this).size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut *mut NODE;
}
pub unsafe extern "C" fn hash_add(
    mut this: HASH,
    mut key: *mut libc::c_char,
    mut val: *mut libc::c_void,
) {
    let mut len: size_t = 0 as libc::c_int as size_t;
    if __lookup(this, key) as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
    {
        return;
    }
    len = strlen(val as *const libc::c_char);
    hash_nadd(this, key, val, len);
}
pub unsafe extern "C" fn hash_nadd(
    mut this: HASH,
    mut key: *mut libc::c_char,
    mut val: *mut libc::c_void,
    mut len: size_t,
) {
    let mut x: libc::c_int = 0;
    let mut node: *mut NODE = 0 as *mut NODE;
    if __lookup(this, key) as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
    {
        return;
    }
    if (*this).entries >= (*this).size / 4 as libc::c_int {
        __resize(this);
    }
    x = __genkey((*this).size, key) as libc::c_int;
    node = xmalloc(::std::mem::size_of::<NODE>() as libc::c_ulong) as *mut NODE;
    (*node).key = strdup(key);
    (*node).val = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    memset(
        (*node).val,
        '\0' as i32,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy((*node).val, val, len);
    (*node).next = *((*this).table).offset(x as isize);
    let ref mut fresh0 = *((*this).table).offset(x as isize);
    *fresh0 = node;
    (*this).entries += 1;
    (*this).entries;
}
pub unsafe extern "C" fn hash_get(
    mut this: HASH,
    mut key: *mut libc::c_char,
) -> *mut libc::c_void {
    let mut x: libc::c_int = 0;
    let mut node: *mut NODE = 0 as *mut NODE;
    x = __genkey((*this).size, key) as libc::c_int;
    node = *((*this).table).offset(x as isize);
    while !node.is_null() {
        if strcmp((*node).key, key) == 0 {
            return (*node).val;
        }
        node = (*node).next;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn hash_remove(mut this: HASH, mut key: *mut libc::c_char) {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut n1: *mut NODE = 0 as *mut NODE;
    let mut n2: *mut NODE = 0 as *mut NODE;
    if __lookup(this, key) as libc::c_uint
        == boolean_false as libc::c_int as libc::c_uint
    {
        return;
    }
    if ((*this).free).is_none() {
        (*this).free = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    }
    x = __genkey((*this).size, key) as libc::c_int;
    n1 = *((*this).table).offset(x as isize);
    while !n1.is_null() {
        n2 = (*n1).next;
        if !((*n1).key).is_null() {
            xfree((*n1).key as *mut libc::c_void);
            (*this).entries -= 1;
            (*this).entries;
        }
        if !((*n1).val).is_null() {
            ((*this).free).unwrap()((*n1).val);
        }
        xfree(n1 as *mut libc::c_void);
        n1 = n2;
    }
    let ref mut fresh1 = *((*this).table).offset(x as isize);
    *fresh1 = 0 as *mut NODE;
}
pub unsafe extern "C" fn hash_remover(
    mut this: HASH,
    mut key: *mut libc::c_char,
    mut m: method,
) {
    (*this).free = m;
    hash_remove(this, key);
}
pub unsafe extern "C" fn hash_set_destroyer(mut this: HASH, mut m: method) {
    if this.is_null() {
        return;
    }
    (*this).free = m;
}
pub unsafe extern "C" fn hash_contains(
    mut this: HASH,
    mut key: *mut libc::c_char,
) -> BOOLEAN {
    return __lookup(this, key);
}
pub unsafe extern "C" fn hash_get_keys(mut this: HASH) -> *mut *mut libc::c_char {
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut node: *mut NODE = 0 as *mut NODE;
    let mut keys: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if this.is_null() || (*this).entries == 0 as libc::c_int {
        return 0 as *mut *mut libc::c_char;
    }
    keys = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((*this).entries as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    x = 0 as libc::c_int;
    while x < (*this).size {
        node = *((*this).table).offset(x as isize);
        while !node.is_null() {
            let mut len: size_t = (strlen((*node).key))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let ref mut fresh2 = *keys.offset(i as isize);
            *fresh2 = malloc(len) as *mut libc::c_char;
            memset(*keys.offset(i as isize) as *mut libc::c_void, '\0' as i32, len);
            memcpy(
                *keys.offset(i as isize) as *mut libc::c_void,
                (*node).key as *const libc::c_void,
                strlen((*node).key),
            );
            i += 1;
            i;
            node = (*node).next;
        }
        x += 1;
        x;
    }
    return keys;
}
pub unsafe extern "C" fn hash_free_keys(
    mut this: HASH,
    mut keys: *mut *mut libc::c_char,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < (*this).entries {
        if !(*keys.offset(x as isize)).is_null() {
            let mut tmp: *mut libc::c_char = *keys.offset(x as isize);
            xfree(tmp as *mut libc::c_void);
        }
        x += 1;
        x;
    }
    xfree(keys as *mut libc::c_void);
}
pub unsafe extern "C" fn hash_destroy(mut this: HASH) -> HASH {
    let mut x: libc::c_int = 0;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    if this.is_null() {
        return this;
    }
    if ((*this).free).is_none() {
        (*this).free = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    }
    x = 0 as libc::c_int;
    while x < (*this).size {
        t1 = *((*this).table).offset(x as isize);
        while !t1.is_null() {
            t2 = (*t1).next;
            if !((*t1).key).is_null() {
                xfree((*t1).key as *mut libc::c_void);
            }
            if !((*t1).val).is_null() {
                ((*this).free).unwrap()((*t1).val);
            }
            xfree(t1 as *mut libc::c_void);
            t1 = t2;
        }
        let ref mut fresh3 = *((*this).table).offset(x as isize);
        *fresh3 = 0 as *mut NODE;
        x += 1;
        x;
    }
    if !((*this).table).is_null() {
        xfree((*this).table as *mut libc::c_void);
        memset(
            this as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<HASH_T>() as libc::c_ulong,
        );
    }
    xfree(this as *mut libc::c_void);
    return 0 as HASH;
}
pub unsafe extern "C" fn hash_destroyer(mut this: HASH, mut m: method) -> HASH {
    (*this).free = m;
    return hash_destroy(this);
}
pub unsafe extern "C" fn hash_get_entries(mut this: HASH) -> libc::c_int {
    return (*this).entries;
}
unsafe extern "C" fn __resize(mut this: HASH) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut last_node: *mut NODE = 0 as *mut NODE;
    let mut last_table: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut x: libc::c_int = 0;
    let mut hash: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    size = (*this).size;
    last_table = (*this).table;
    hash_reset(this, (size * 2 as libc::c_int) as ssize_t);
    x = 0 as libc::c_int;
    while x < size {
        last_node = *last_table.offset(x as isize);
        while !last_node.is_null() {
            tmp = last_node;
            last_node = (*last_node).next;
            hash = __genkey((*this).size, (*tmp).key) as libc::c_int;
            (*tmp).next = *((*this).table).offset(hash as isize);
            let ref mut fresh4 = *((*this).table).offset(hash as isize);
            *fresh4 = tmp;
            (*this).entries += 1;
            (*this).entries;
        }
        x += 1;
        x;
    }
}
unsafe extern "C" fn fnv_32_buf(
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut hval: libc::c_uint,
) -> u_int32_t {
    let mut bp: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    let mut be: *mut libc::c_uchar = bp.offset(len as isize);
    while bp < be {
        let fresh5 = bp;
        bp = bp.offset(1);
        hval ^= *fresh5 as u_int32_t;
        hval = hval
            .wrapping_add(
                (hval << 1 as libc::c_int)
                    .wrapping_add(hval << 4 as libc::c_int)
                    .wrapping_add(hval << 7 as libc::c_int)
                    .wrapping_add(hval << 8 as libc::c_int)
                    .wrapping_add(hval << 24 as libc::c_int),
            );
    }
    return hval;
}
unsafe extern "C" fn __genkey(
    mut size: libc::c_int,
    mut str: *mut libc::c_char,
) -> libc::c_uint {
    let mut hash: libc::c_uint = 0;
    let mut data: *mut libc::c_void = str as *mut libc::c_void;
    hash = fnv_32_buf(data, strlen(str), 2166136261 as libc::c_longlong as libc::c_uint);
    hash = hash.wrapping_rem(size as libc::c_uint);
    return hash;
}
unsafe extern "C" fn __lookup(mut this: HASH, mut key: *mut libc::c_char) -> BOOLEAN {
    let mut x: libc::c_int = 0;
    let mut node: *mut NODE = 0 as *mut NODE;
    if key.is_null() {
        return boolean_false;
    }
    x = __genkey((*this).size, key) as libc::c_int;
    node = *((*this).table).offset(x as isize);
    while !node.is_null() {
        if strcmp((*node).key, key) == 0 {
            return boolean_true;
        }
        node = (*node).next;
    }
    return boolean_false;
}

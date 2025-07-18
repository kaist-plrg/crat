use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringhash_st {
    pub hash_table: *mut HashTable,
    pub next_idx: libc::c_uint,
    pub next_item: *mut HashList,
}
pub type HashList = hash_list_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_list_st {
    pub next: *mut hash_list_st,
    pub key: *mut libc::c_char,
    pub keylen: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type HashTable = *mut HashList;
pub unsafe extern "C" fn strhash_init() -> StringHashPtr {
    let mut tmp: StringHashPtr = 0 as *mut stringhash_st;
    tmp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<stringhash_st>() as libc::c_ulong,
    ) as StringHashPtr;
    if tmp.is_null() {
        return 0 as StringHashPtr;
    }
    (*tmp)
        .hash_table = calloc(
        8192 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<HashTable>() as libc::c_ulong,
    ) as *mut HashTable;
    if ((*tmp).hash_table).is_null() {
        free(tmp as *mut libc::c_void);
        return 0 as StringHashPtr;
    }
    return tmp;
}
pub unsafe extern "C" fn strhash_free(mut hash: StringHashPtr) {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut list_next: *mut HashList = 0 as *mut HashList;
    let mut i: libc::c_int = 0;
    if hash.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < 8192 as libc::c_int {
        list = *((*hash).hash_table).offset(i as isize);
        while !list.is_null() {
            list_next = (*list).next;
            free((*list).key as *mut libc::c_void);
            free(list as *mut libc::c_void);
            list = list_next;
        }
        i += 1;
        i;
    }
    free((*hash).hash_table as *mut libc::c_void);
    free(hash as *mut libc::c_void);
}
pub unsafe extern "C" fn strhash_put(
    mut hash: StringHashPtr,
    mut key: *mut libc::c_char,
    mut keylen: libc::c_int,
    mut data: *mut libc::c_void,
    mut old_data: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut prev: *mut HashList = 0 as *mut HashList;
    let mut pos: libc::c_int = 0;
    let mut cmp_val: libc::c_int = 0;
    if hash.is_null() || key.is_null() || keylen <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !old_data.is_null() {
        *old_data = 0 as *mut libc::c_void;
    }
    pos = count_hash(key, keylen);
    list = *((*hash).hash_table).offset(pos as isize);
    while !list.is_null() {
        if (*list).keylen == keylen {
            cmp_val = memcmp(
                key as *const libc::c_void,
                (*list).key as *const libc::c_void,
                keylen as libc::c_ulong,
            );
            if cmp_val == 0 as libc::c_int {
                if !old_data.is_null() {
                    *old_data = (*list).data;
                }
                (*list).data = data;
                return 1 as libc::c_int;
            } else if cmp_val < 0 as libc::c_int {
                break;
            }
        } else if (*list).keylen > keylen {
            break;
        }
        prev = list;
        list = (*list).next;
    }
    list = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<HashList>() as libc::c_ulong,
    ) as *mut HashList;
    if list.is_null() {
        return 0 as libc::c_int;
    }
    (*list).key = malloc(keylen as libc::c_ulong) as *mut libc::c_char;
    if ((*list).key).is_null() {
        free(list as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    memcpy(
        (*list).key as *mut libc::c_void,
        key as *const libc::c_void,
        keylen as libc::c_ulong,
    );
    (*list).keylen = keylen;
    (*list).data = data;
    if prev.is_null() {
        (*list).next = *((*hash).hash_table).offset(pos as isize);
        let ref mut fresh0 = *((*hash).hash_table).offset(pos as isize);
        *fresh0 = list;
    } else {
        (*list).next = (*prev).next;
        (*prev).next = list;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn strhash_get(
    mut hash: StringHashPtr,
    mut key: *const libc::c_char,
    mut keylen: libc::c_int,
    mut data: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut pos: libc::c_int = 0;
    let mut cmp_val: libc::c_int = 0;
    if hash.is_null() || key.is_null() || keylen <= 0 as libc::c_int || data.is_null() {
        return 0 as libc::c_int;
    }
    *data = 0 as *mut libc::c_void;
    pos = count_hash(key, keylen);
    list = *((*hash).hash_table).offset(pos as isize);
    while !list.is_null() {
        if (*list).keylen == keylen {
            cmp_val = memcmp(
                key as *const libc::c_void,
                (*list).key as *const libc::c_void,
                keylen as libc::c_ulong,
            );
            if cmp_val == 0 as libc::c_int {
                *data = (*list).data;
                return 1 as libc::c_int;
            } else if cmp_val < 0 as libc::c_int {
                break;
            }
        } else if (*list).keylen > keylen {
            break;
        }
        list = (*list).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strhash_delete(
    mut hash: StringHashPtr,
    mut key: *const libc::c_char,
    mut keylen: libc::c_int,
    mut data: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut list: *mut HashList = 0 as *mut HashList;
    let mut prev: *mut HashList = 0 as *mut HashList;
    let mut pos: libc::c_int = 0;
    let mut cmp_val: libc::c_int = 0;
    if hash.is_null() || key.is_null() || keylen <= 0 as libc::c_int || data.is_null() {
        return 0 as libc::c_int;
    }
    *data = 0 as *mut libc::c_void;
    pos = count_hash(key, keylen);
    list = *((*hash).hash_table).offset(pos as isize);
    while !list.is_null() {
        if (*list).keylen == keylen {
            cmp_val = memcmp(
                key as *const libc::c_void,
                (*list).key as *const libc::c_void,
                keylen as libc::c_ulong,
            );
            if cmp_val == 0 as libc::c_int {
                if prev.is_null() {
                    let ref mut fresh1 = *((*hash).hash_table).offset(pos as isize);
                    *fresh1 = (*list).next;
                } else {
                    (*prev).next = (*list).next;
                }
                *data = (*list).data;
                free((*list).key as *mut libc::c_void);
                free(list as *mut libc::c_void);
                (*hash).next_idx = 0 as libc::c_int as libc::c_uint;
                (*hash).next_item = 0 as *mut HashList;
                return 1 as libc::c_int;
            } else if cmp_val < 0 as libc::c_int {
                break;
            }
        } else if (*list).keylen > keylen {
            break;
        }
        prev = list;
        list = (*list).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strhash_get_first(
    mut hash: StringHashPtr,
    mut key_return: *mut *mut libc::c_char,
    mut keylen_return: *mut libc::c_int,
    mut data_return: *mut *mut libc::c_void,
) -> libc::c_int {
    if hash.is_null() || key_return.is_null() || keylen_return.is_null()
        || data_return.is_null()
    {
        return 0 as libc::c_int;
    }
    (*hash).next_idx = 0 as libc::c_int as libc::c_uint;
    while (*hash).next_idx < 8192 as libc::c_int as libc::c_uint {
        (*hash).next_item = *((*hash).hash_table).offset((*hash).next_idx as isize);
        if !((*hash).next_item).is_null() {
            *key_return = (*(*hash).next_item).key;
            *keylen_return = (*(*hash).next_item).keylen;
            *data_return = (*(*hash).next_item).data;
            return 1 as libc::c_int;
        }
        (*hash).next_idx = ((*hash).next_idx).wrapping_add(1);
        (*hash).next_idx;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strhash_get_next(
    mut hash: StringHashPtr,
    mut key_return: *mut *mut libc::c_char,
    mut keylen_return: *mut libc::c_int,
    mut data_return: *mut *mut libc::c_void,
) -> libc::c_int {
    if hash.is_null() || key_return.is_null() || keylen_return.is_null()
        || data_return.is_null()
    {
        return 0 as libc::c_int;
    }
    while (*hash).next_idx < 8192 as libc::c_int as libc::c_uint {
        if ((*hash).next_item).is_null() {
            (*hash).next_item = *((*hash).hash_table).offset((*hash).next_idx as isize);
        } else {
            (*hash).next_item = (*(*hash).next_item).next;
        }
        if !((*hash).next_item).is_null() {
            *key_return = (*(*hash).next_item).key;
            *keylen_return = (*(*hash).next_item).keylen;
            *data_return = (*(*hash).next_item).data;
            return 1 as libc::c_int;
        }
        (*hash).next_idx = ((*hash).next_idx).wrapping_add(1);
        (*hash).next_idx;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn count_hash(
    mut key: *const libc::c_char,
    mut keylen: libc::c_int,
) -> libc::c_int {
    let mut val: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < keylen {
        val = val << 5 as libc::c_int
            ^ *key.offset(i as isize) as libc::c_uchar as libc::c_uint
            ^ val >> 16 as libc::c_int ^ val >> 7 as libc::c_int;
        i += 1;
        i;
    }
    return val.wrapping_rem(8192 as libc::c_int as libc::c_uint) as libc::c_int;
}

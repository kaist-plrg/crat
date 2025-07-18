use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type key_hash_fn = Option::<unsafe extern "C" fn(*const libc::c_void) -> size_t>;
pub type key_cmp_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type free_fn = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_list {
    pub prev: *mut hashtable_list,
    pub next: *mut hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_pair {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub hash: size_t,
    pub list: hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_bucket {
    pub first: *mut hashtable_list,
    pub last: *mut hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable {
    pub size: size_t,
    pub buckets: *mut hashtable_bucket,
    pub num_buckets: size_t,
    pub list: hashtable_list,
    pub hash_key: key_hash_fn,
    pub cmp_keys: key_cmp_fn,
    pub free_key: free_fn,
    pub free_value: free_fn,
}
pub type hashtable_t = hashtable;
pub type list_t = hashtable_list;
pub type bucket_t = hashtable_bucket;
pub type pair_t = hashtable_pair;
#[inline]
unsafe extern "C" fn list_init(mut list: *mut list_t) {
    (*list).next = list;
    (*list).prev = list;
}
#[inline]
unsafe extern "C" fn list_insert(mut list: *mut list_t, mut node: *mut list_t) {
    (*node).next = list;
    (*node).prev = (*list).prev;
    (*(*list).prev).next = node;
    (*list).prev = node;
}
#[inline]
unsafe extern "C" fn list_remove(mut list: *mut list_t) {
    (*(*list).prev).next = (*list).next;
    (*(*list).next).prev = (*list).prev;
}
#[inline]
unsafe extern "C" fn bucket_is_empty(
    mut hashtable: *mut hashtable_t,
    mut bucket: *mut bucket_t,
) -> libc::c_int {
    return ((*bucket).first == &mut (*hashtable).list as *mut hashtable_list
        && (*bucket).first == (*bucket).last) as libc::c_int;
}
unsafe extern "C" fn insert_to_bucket(
    mut hashtable: *mut hashtable_t,
    mut bucket: *mut bucket_t,
    mut list: *mut list_t,
) {
    if bucket_is_empty(hashtable, bucket) != 0 {
        list_insert(&mut (*hashtable).list, list);
        (*bucket).last = list;
        (*bucket).first = (*bucket).last;
    } else {
        list_insert((*bucket).first, list);
        (*bucket).first = list;
    };
}
static mut primes: [size_t; 29] = [
    5 as libc::c_int as size_t,
    13 as libc::c_int as size_t,
    23 as libc::c_int as size_t,
    53 as libc::c_int as size_t,
    97 as libc::c_int as size_t,
    193 as libc::c_int as size_t,
    389 as libc::c_int as size_t,
    769 as libc::c_int as size_t,
    1543 as libc::c_int as size_t,
    3079 as libc::c_int as size_t,
    6151 as libc::c_int as size_t,
    12289 as libc::c_int as size_t,
    24593 as libc::c_int as size_t,
    49157 as libc::c_int as size_t,
    98317 as libc::c_int as size_t,
    196613 as libc::c_int as size_t,
    393241 as libc::c_int as size_t,
    786433 as libc::c_int as size_t,
    1572869 as libc::c_int as size_t,
    3145739 as libc::c_int as size_t,
    6291469 as libc::c_int as size_t,
    12582917 as libc::c_int as size_t,
    25165843 as libc::c_int as size_t,
    50331653 as libc::c_int as size_t,
    100663319 as libc::c_int as size_t,
    201326611 as libc::c_int as size_t,
    402653189 as libc::c_int as size_t,
    805306457 as libc::c_int as size_t,
    1610612741 as libc::c_int as size_t,
];
#[inline]
unsafe extern "C" fn num_buckets(mut hashtable: *mut hashtable_t) -> size_t {
    return primes[(*hashtable).num_buckets as usize];
}
unsafe extern "C" fn hashtable_find_pair(
    mut hashtable: *mut hashtable_t,
    mut bucket: *mut bucket_t,
    mut key: *const libc::c_void,
    mut hash: size_t,
) -> *mut pair_t {
    let mut list: *mut list_t = 0 as *mut list_t;
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    if bucket_is_empty(hashtable, bucket) != 0 {
        return 0 as *mut pair_t;
    }
    list = (*bucket).first;
    loop {
        pair = (list as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
            as *mut pair_t;
        if (*pair).hash == hash
            && ((*hashtable).cmp_keys).unwrap()((*pair).key, key) != 0
        {
            return pair;
        }
        if list == (*bucket).last {
            break;
        }
        list = (*list).next;
    }
    return 0 as *mut pair_t;
}
unsafe extern "C" fn hashtable_do_del(
    mut hashtable: *mut hashtable_t,
    mut key: *const libc::c_void,
    mut hash: size_t,
) -> libc::c_int {
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    let mut bucket: *mut bucket_t = 0 as *mut bucket_t;
    let mut index: size_t = 0;
    index = hash.wrapping_rem(num_buckets(hashtable));
    bucket = &mut *((*hashtable).buckets).offset(index as isize)
        as *mut hashtable_bucket;
    pair = hashtable_find_pair(hashtable, bucket, key, hash);
    if pair.is_null() {
        return -(1 as libc::c_int);
    }
    if &mut (*pair).list as *mut hashtable_list == (*bucket).first
        && &mut (*pair).list as *mut hashtable_list == (*bucket).last
    {
        (*bucket).last = &mut (*hashtable).list;
        (*bucket).first = (*bucket).last;
    } else if &mut (*pair).list as *mut hashtable_list == (*bucket).first {
        (*bucket).first = (*pair).list.next;
    } else if &mut (*pair).list as *mut hashtable_list == (*bucket).last {
        (*bucket).last = (*pair).list.prev;
    }
    list_remove(&mut (*pair).list);
    if ((*hashtable).free_key).is_some() {
        ((*hashtable).free_key).unwrap()((*pair).key);
    }
    if ((*hashtable).free_value).is_some() {
        ((*hashtable).free_value).unwrap()((*pair).value);
    }
    free(pair as *mut libc::c_void);
    (*hashtable).size = ((*hashtable).size).wrapping_sub(1);
    (*hashtable).size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn hashtable_do_clear(mut hashtable: *mut hashtable_t) {
    let mut list: *mut list_t = 0 as *mut list_t;
    let mut next: *mut list_t = 0 as *mut list_t;
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    list = (*hashtable).list.next;
    while list != &mut (*hashtable).list as *mut hashtable_list {
        next = (*list).next;
        pair = (list as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
            as *mut pair_t;
        if ((*hashtable).free_key).is_some() {
            ((*hashtable).free_key).unwrap()((*pair).key);
        }
        if ((*hashtable).free_value).is_some() {
            ((*hashtable).free_value).unwrap()((*pair).value);
        }
        free(pair as *mut libc::c_void);
        list = next;
    }
}
unsafe extern "C" fn hashtable_do_rehash(
    mut hashtable: *mut hashtable_t,
) -> libc::c_int {
    let mut list: *mut list_t = 0 as *mut list_t;
    let mut next: *mut list_t = 0 as *mut list_t;
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    let mut i: size_t = 0;
    let mut index: size_t = 0;
    let mut new_size: size_t = 0;
    free((*hashtable).buckets as *mut libc::c_void);
    (*hashtable).num_buckets = ((*hashtable).num_buckets).wrapping_add(1);
    (*hashtable).num_buckets;
    new_size = num_buckets(hashtable);
    (*hashtable)
        .buckets = malloc(
        new_size.wrapping_mul(::std::mem::size_of::<bucket_t>() as libc::c_ulong),
    ) as *mut hashtable_bucket;
    if ((*hashtable).buckets).is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_buckets(hashtable) {
        let ref mut fresh0 = (*((*hashtable).buckets).offset(i as isize)).last;
        *fresh0 = &mut (*hashtable).list;
        let ref mut fresh1 = (*((*hashtable).buckets).offset(i as isize)).first;
        *fresh1 = *fresh0;
        i = i.wrapping_add(1);
        i;
    }
    list = (*hashtable).list.next;
    list_init(&mut (*hashtable).list);
    while list != &mut (*hashtable).list as *mut hashtable_list {
        next = (*list).next;
        pair = (list as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
            as *mut pair_t;
        index = ((*pair).hash).wrapping_rem(new_size);
        insert_to_bucket(
            hashtable,
            &mut *((*hashtable).buckets).offset(index as isize),
            &mut (*pair).list,
        );
        list = next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hashtable_create(
    mut hash_key: key_hash_fn,
    mut cmp_keys: key_cmp_fn,
    mut free_key: free_fn,
    mut free_value: free_fn,
) -> *mut hashtable_t {
    let mut hashtable: *mut hashtable_t = malloc(
        ::std::mem::size_of::<hashtable_t>() as libc::c_ulong,
    ) as *mut hashtable_t;
    if hashtable.is_null() {
        return 0 as *mut hashtable_t;
    }
    if hashtable_init(hashtable, hash_key, cmp_keys, free_key, free_value) != 0 {
        free(hashtable as *mut libc::c_void);
        return 0 as *mut hashtable_t;
    }
    return hashtable;
}
pub unsafe extern "C" fn hashtable_destroy(mut hashtable: *mut hashtable_t) {
    hashtable_close(hashtable);
    free(hashtable as *mut libc::c_void);
}
pub unsafe extern "C" fn hashtable_init(
    mut hashtable: *mut hashtable_t,
    mut hash_key: key_hash_fn,
    mut cmp_keys: key_cmp_fn,
    mut free_key: free_fn,
    mut free_value: free_fn,
) -> libc::c_int {
    let mut i: size_t = 0;
    (*hashtable).size = 0 as libc::c_int as size_t;
    (*hashtable).num_buckets = 0 as libc::c_int as size_t;
    (*hashtable)
        .buckets = malloc(
        (num_buckets(hashtable))
            .wrapping_mul(::std::mem::size_of::<bucket_t>() as libc::c_ulong),
    ) as *mut hashtable_bucket;
    if ((*hashtable).buckets).is_null() {
        return -(1 as libc::c_int);
    }
    list_init(&mut (*hashtable).list);
    (*hashtable).hash_key = hash_key;
    (*hashtable).cmp_keys = cmp_keys;
    (*hashtable).free_key = free_key;
    (*hashtable).free_value = free_value;
    i = 0 as libc::c_int as size_t;
    while i < num_buckets(hashtable) {
        let ref mut fresh2 = (*((*hashtable).buckets).offset(i as isize)).last;
        *fresh2 = &mut (*hashtable).list;
        let ref mut fresh3 = (*((*hashtable).buckets).offset(i as isize)).first;
        *fresh3 = *fresh2;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hashtable_close(mut hashtable: *mut hashtable_t) {
    hashtable_do_clear(hashtable);
    free((*hashtable).buckets as *mut libc::c_void);
}
pub unsafe extern "C" fn hashtable_set(
    mut hashtable: *mut hashtable_t,
    mut key: *mut libc::c_void,
    mut value: *mut libc::c_void,
) -> libc::c_int {
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    let mut bucket: *mut bucket_t = 0 as *mut bucket_t;
    let mut hash: size_t = 0;
    let mut index: size_t = 0;
    if (*hashtable).size >= num_buckets(hashtable) {
        if hashtable_do_rehash(hashtable) != 0 {
            return -(1 as libc::c_int);
        }
    }
    hash = ((*hashtable).hash_key).unwrap()(key);
    index = hash.wrapping_rem(num_buckets(hashtable));
    bucket = &mut *((*hashtable).buckets).offset(index as isize)
        as *mut hashtable_bucket;
    pair = hashtable_find_pair(hashtable, bucket, key, hash);
    if !pair.is_null() {
        if ((*hashtable).free_key).is_some() {
            ((*hashtable).free_key).unwrap()(key);
        }
        if ((*hashtable).free_value).is_some() {
            ((*hashtable).free_value).unwrap()((*pair).value);
        }
        (*pair).value = value;
    } else {
        pair = malloc(::std::mem::size_of::<pair_t>() as libc::c_ulong) as *mut pair_t;
        if pair.is_null() {
            return -(1 as libc::c_int);
        }
        (*pair).key = key;
        (*pair).value = value;
        (*pair).hash = hash;
        list_init(&mut (*pair).list);
        insert_to_bucket(hashtable, bucket, &mut (*pair).list);
        (*hashtable).size = ((*hashtable).size).wrapping_add(1);
        (*hashtable).size;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hashtable_get(
    mut hashtable: *mut hashtable_t,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    let mut hash: size_t = 0;
    let mut bucket: *mut bucket_t = 0 as *mut bucket_t;
    hash = ((*hashtable).hash_key).unwrap()(key);
    bucket = &mut *((*hashtable).buckets)
        .offset(
            hash
                .wrapping_rem(
                    (num_buckets
                        as unsafe extern "C" fn(*mut hashtable_t) -> size_t)(hashtable),
                ) as isize,
        ) as *mut hashtable_bucket;
    pair = hashtable_find_pair(hashtable, bucket, key, hash);
    if pair.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*pair).value;
}
pub unsafe extern "C" fn hashtable_del(
    mut hashtable: *mut hashtable_t,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut hash: size_t = ((*hashtable).hash_key).unwrap()(key);
    return hashtable_do_del(hashtable, key, hash);
}
pub unsafe extern "C" fn hashtable_clear(mut hashtable: *mut hashtable_t) {
    let mut i: size_t = 0;
    hashtable_do_clear(hashtable);
    i = 0 as libc::c_int as size_t;
    while i < num_buckets(hashtable) {
        let ref mut fresh4 = (*((*hashtable).buckets).offset(i as isize)).last;
        *fresh4 = &mut (*hashtable).list;
        let ref mut fresh5 = (*((*hashtable).buckets).offset(i as isize)).first;
        *fresh5 = *fresh4;
        i = i.wrapping_add(1);
        i;
    }
    list_init(&mut (*hashtable).list);
    (*hashtable).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn hashtable_iter(
    mut hashtable: *mut hashtable_t,
) -> *mut libc::c_void {
    return hashtable_iter_next(
        hashtable,
        &mut (*hashtable).list as *mut hashtable_list as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn hashtable_iter_at(
    mut hashtable: *mut hashtable_t,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    let mut pair: *mut pair_t = 0 as *mut pair_t;
    let mut hash: size_t = 0;
    let mut bucket: *mut bucket_t = 0 as *mut bucket_t;
    hash = ((*hashtable).hash_key).unwrap()(key);
    bucket = &mut *((*hashtable).buckets)
        .offset(
            hash
                .wrapping_rem(
                    (num_buckets
                        as unsafe extern "C" fn(*mut hashtable_t) -> size_t)(hashtable),
                ) as isize,
        ) as *mut hashtable_bucket;
    pair = hashtable_find_pair(hashtable, bucket, key, hash);
    if pair.is_null() {
        return 0 as *mut libc::c_void;
    }
    return &mut (*pair).list as *mut hashtable_list as *mut libc::c_void;
}
pub unsafe extern "C" fn hashtable_iter_next(
    mut hashtable: *mut hashtable_t,
    mut iter: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut list: *mut list_t = iter as *mut list_t;
    if (*list).next == &mut (*hashtable).list as *mut hashtable_list {
        return 0 as *mut libc::c_void;
    }
    return (*list).next as *mut libc::c_void;
}
pub unsafe extern "C" fn hashtable_iter_key(
    mut iter: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut pair: *mut pair_t = (iter as *mut list_t as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut pair_t;
    return (*pair).key;
}
pub unsafe extern "C" fn hashtable_iter_value(
    mut iter: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut pair: *mut pair_t = (iter as *mut list_t as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut pair_t;
    return (*pair).value;
}
pub unsafe extern "C" fn hashtable_iter_set(
    mut hashtable: *mut hashtable_t,
    mut iter: *mut libc::c_void,
    mut value: *mut libc::c_void,
) {
    let mut pair: *mut pair_t = (iter as *mut list_t as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut pair_t;
    if ((*hashtable).free_value).is_some() {
        ((*hashtable).free_value).unwrap()((*pair).value);
    }
    (*pair).value = value;
}

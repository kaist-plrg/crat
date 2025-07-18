use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn hashtable_init(
        hashtable: *mut hashtable_t,
        hash_key_0: key_hash_fn,
        cmp_keys: key_cmp_fn,
        free_key: free_fn,
        free_value: free_fn,
    ) -> libc::c_int;
    fn hashtable_close(hashtable: *mut hashtable_t);
    fn hashtable_set(
        hashtable: *mut hashtable_t,
        key: *mut libc::c_void,
        value: *mut libc::c_void,
    ) -> libc::c_int;
    fn hashtable_get(
        hashtable: *mut hashtable_t,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hashtable_del(
        hashtable: *mut hashtable_t,
        key: *const libc::c_void,
    ) -> libc::c_int;
    fn hashtable_clear(hashtable: *mut hashtable_t);
    fn hashtable_iter(hashtable: *mut hashtable_t) -> *mut libc::c_void;
    fn hashtable_iter_at(
        hashtable: *mut hashtable_t,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hashtable_iter_next(
        hashtable: *mut hashtable_t,
        iter: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn hashtable_iter_key(iter: *mut libc::c_void) -> *mut libc::c_void;
    fn hashtable_iter_value(iter: *mut libc::c_void) -> *mut libc::c_void;
    fn hashtable_iter_set(
        hashtable: *mut hashtable_t,
        iter: *mut libc::c_void,
        value: *mut libc::c_void,
    );
    fn utf8_check_string(
        string: *const libc::c_char,
        length: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type json_type = libc::c_uint;
pub const JSON_NULL: json_type = 7;
pub const JSON_FALSE: json_type = 6;
pub const JSON_TRUE: json_type = 5;
pub const JSON_REAL: json_type = 4;
pub const JSON_INTEGER: json_type = 3;
pub const JSON_STRING: json_type = 2;
pub const JSON_ARRAY: json_type = 1;
pub const JSON_OBJECT: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_int_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_t {
    pub json: json_t,
    pub hashtable: hashtable_t,
    pub serial: size_t,
    pub visited: libc::c_int,
}
pub type hashtable_t = hashtable;
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
pub type free_fn = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type key_cmp_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type key_hash_fn = Option::<unsafe extern "C" fn(*const libc::c_void) -> size_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_list {
    pub prev: *mut hashtable_list,
    pub next: *mut hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_bucket {
    pub first: *mut hashtable_list,
    pub last: *mut hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_real_t {
    pub json: json_t,
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_integer_t {
    pub json: json_t,
    pub value: json_int_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_t {
    pub json: json_t,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_t {
    pub json: json_t,
    pub size: size_t,
    pub entries: size_t,
    pub table: *mut *mut json_t,
    pub visited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_key_t {
    pub serial: size_t,
    pub key: [libc::c_char; 1],
}
#[inline]
unsafe extern "C" fn json_array_append(
    mut array: *mut json_t,
    mut value: *mut json_t,
) -> libc::c_int {
    return json_array_append_new(array, json_incref(value));
}
#[inline]
unsafe extern "C" fn json_object_set_nocheck(
    mut object: *mut json_t,
    mut key: *const libc::c_char,
    mut value: *mut json_t,
) -> libc::c_int {
    return json_object_set_new_nocheck(object, key, json_incref(value));
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t
        && {
            (*json).refcount = ((*json).refcount).wrapping_sub(1);
            (*json).refcount == 0 as libc::c_int as libc::c_ulong
        }
    {
        json_delete(json);
    }
}
#[inline]
unsafe extern "C" fn json_incref(mut json: *mut json_t) -> *mut json_t {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t {
        (*json).refcount = ((*json).refcount).wrapping_add(1);
        (*json).refcount;
    }
    return json;
}
#[inline]
unsafe extern "C" fn json_init(mut json: *mut json_t, mut type_0: json_type) {
    (*json).type_0 = type_0;
    (*json).refcount = 1 as libc::c_int as size_t;
}
unsafe extern "C" fn hash_key(mut ptr: *const libc::c_void) -> size_t {
    let mut str: *const libc::c_char = ((*(ptr as *const object_key_t)).key).as_ptr();
    let mut hash: size_t = 5381 as libc::c_int as size_t;
    let mut c: size_t = 0;
    loop {
        c = *str as size_t;
        if !(c != 0) {
            break;
        }
        hash = (hash << 5 as libc::c_int).wrapping_add(hash).wrapping_add(c);
        str = str.offset(1);
        str;
    }
    return hash;
}
unsafe extern "C" fn key_equal(
    mut ptr1: *const libc::c_void,
    mut ptr2: *const libc::c_void,
) -> libc::c_int {
    return (strcmp(
        ((*(ptr1 as *const object_key_t)).key).as_ptr(),
        ((*(ptr2 as *const object_key_t)).key).as_ptr(),
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn value_decref(mut value: *mut libc::c_void) {
    json_decref(value as *mut json_t);
}
pub unsafe extern "C" fn json_object() -> *mut json_t {
    let mut object: *mut json_object_t = malloc(
        ::std::mem::size_of::<json_object_t>() as libc::c_ulong,
    ) as *mut json_object_t;
    if object.is_null() {
        return 0 as *mut json_t;
    }
    json_init(&mut (*object).json, JSON_OBJECT);
    if hashtable_init(
        &mut (*object).hashtable,
        Some(hash_key as unsafe extern "C" fn(*const libc::c_void) -> size_t),
        Some(
            key_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(value_decref as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    ) != 0
    {
        free(object as *mut libc::c_void);
        return 0 as *mut json_t;
    }
    (*object).serial = 0 as libc::c_int as size_t;
    (*object).visited = 0 as libc::c_int;
    return &mut (*object).json;
}
unsafe extern "C" fn json_delete_object(mut object: *mut json_object_t) {
    hashtable_close(&mut (*object).hashtable);
    free(object as *mut libc::c_void);
}
pub unsafe extern "C" fn json_object_size(mut json: *const json_t) -> size_t {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int) as size_t;
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    return (*object).hashtable.size;
}
pub unsafe extern "C" fn json_object_get(
    mut json: *const json_t,
    mut key: *const libc::c_char,
) -> *mut json_t {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return 0 as *mut json_t;
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    return hashtable_get(
        &mut (*object).hashtable,
        (key as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
            as *mut object_key_t as *const libc::c_void,
    ) as *mut json_t;
}
pub unsafe extern "C" fn json_object_set_new_nocheck(
    mut json: *mut json_t,
    mut key: *const libc::c_char,
    mut value: *mut json_t,
) -> libc::c_int {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    let mut k: *mut object_key_t = 0 as *mut object_key_t;
    if key.is_null() || value.is_null() {
        return -(1 as libc::c_int);
    }
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
        || json == value
    {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    k = malloc(
        (8 as libc::c_ulong)
            .wrapping_add(strlen(key))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut object_key_t;
    if k.is_null() {
        return -(1 as libc::c_int);
    }
    let fresh0 = (*object).serial;
    (*object).serial = ((*object).serial).wrapping_add(1);
    (*k).serial = fresh0;
    strcpy(((*k).key).as_mut_ptr(), key);
    if hashtable_set(
        &mut (*object).hashtable,
        k as *mut libc::c_void,
        value as *mut libc::c_void,
    ) != 0
    {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_object_set_new(
    mut json: *mut json_t,
    mut key: *const libc::c_char,
    mut value: *mut json_t,
) -> libc::c_int {
    if key.is_null() || utf8_check_string(key, -(1 as libc::c_int)) == 0 {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    return json_object_set_new_nocheck(json, key, value);
}
pub unsafe extern "C" fn json_object_del(
    mut json: *mut json_t,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    return hashtable_del(
        &mut (*object).hashtable,
        (key as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
            as *mut object_key_t as *const libc::c_void,
    );
}
pub unsafe extern "C" fn json_object_clear(mut json: *mut json_t) -> libc::c_int {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    hashtable_clear(&mut (*object).hashtable);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_object_update(
    mut object: *mut json_t,
    mut other: *mut json_t,
) -> libc::c_int {
    let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(!object.is_null()
        && (*object).type_0 as libc::c_uint
            == JSON_OBJECT as libc::c_int as libc::c_uint)
        || !(!other.is_null()
            && (*other).type_0 as libc::c_uint
                == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    iter = json_object_iter(other);
    while !iter.is_null() {
        let mut key: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *mut json_t = 0 as *mut json_t;
        key = json_object_iter_key(iter);
        value = json_object_iter_value(iter);
        if json_object_set_nocheck(object, key, value) != 0 {
            return -(1 as libc::c_int);
        }
        iter = json_object_iter_next(other, iter);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_object_iter(mut json: *mut json_t) -> *mut libc::c_void {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return 0 as *mut libc::c_void;
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    return hashtable_iter(&mut (*object).hashtable);
}
pub unsafe extern "C" fn json_object_iter_at(
    mut json: *mut json_t,
    mut key: *const libc::c_char,
) -> *mut libc::c_void {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if key.is_null()
        || !(!json.is_null()
            && (*json).type_0 as libc::c_uint
                == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return 0 as *mut libc::c_void;
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    return hashtable_iter_at(
        &mut (*object).hashtable,
        (key as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
            as *mut object_key_t as *const libc::c_void,
    );
}
pub unsafe extern "C" fn json_object_iter_next(
    mut json: *mut json_t,
    mut iter: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
        || iter.is_null()
    {
        return 0 as *mut libc::c_void;
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    return hashtable_iter_next(&mut (*object).hashtable, iter);
}
pub unsafe extern "C" fn jsonp_object_iter_fullkey(
    mut iter: *mut libc::c_void,
) -> *const object_key_t {
    if iter.is_null() {
        return 0 as *const object_key_t;
    }
    return hashtable_iter_key(iter) as *const object_key_t;
}
pub unsafe extern "C" fn json_object_iter_key(
    mut iter: *mut libc::c_void,
) -> *const libc::c_char {
    if iter.is_null() {
        return 0 as *const libc::c_char;
    }
    return ((*jsonp_object_iter_fullkey(iter)).key).as_ptr();
}
pub unsafe extern "C" fn json_object_iter_value(
    mut iter: *mut libc::c_void,
) -> *mut json_t {
    if iter.is_null() {
        return 0 as *mut json_t;
    }
    return hashtable_iter_value(iter) as *mut json_t;
}
pub unsafe extern "C" fn json_object_iter_set_new(
    mut json: *mut json_t,
    mut iter: *mut libc::c_void,
    mut value: *mut json_t,
) -> libc::c_int {
    let mut object: *mut json_object_t = 0 as *mut json_object_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint)
        || iter.is_null() || value.is_null()
    {
        return -(1 as libc::c_int);
    }
    object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_object_t;
    hashtable_iter_set(&mut (*object).hashtable, iter, value as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn json_object_equal(
    mut object1: *mut json_t,
    mut object2: *mut json_t,
) -> libc::c_int {
    let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
    if json_object_size(object1) != json_object_size(object2) {
        return 0 as libc::c_int;
    }
    iter = json_object_iter(object1);
    while !iter.is_null() {
        let mut key: *const libc::c_char = 0 as *const libc::c_char;
        let mut value1: *mut json_t = 0 as *mut json_t;
        let mut value2: *mut json_t = 0 as *mut json_t;
        key = json_object_iter_key(iter);
        value1 = json_object_iter_value(iter);
        value2 = json_object_get(object2, key);
        if json_equal(value1, value2) == 0 {
            return 0 as libc::c_int;
        }
        iter = json_object_iter_next(object1, iter);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn json_object_copy(mut object: *mut json_t) -> *mut json_t {
    let mut result: *mut json_t = 0 as *mut json_t;
    let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
    result = json_object();
    if result.is_null() {
        return 0 as *mut json_t;
    }
    iter = json_object_iter(object);
    while !iter.is_null() {
        let mut key: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *mut json_t = 0 as *mut json_t;
        key = json_object_iter_key(iter);
        value = json_object_iter_value(iter);
        json_object_set_nocheck(result, key, value);
        iter = json_object_iter_next(object, iter);
    }
    return result;
}
unsafe extern "C" fn json_object_deep_copy(mut object: *mut json_t) -> *mut json_t {
    let mut result: *mut json_t = 0 as *mut json_t;
    let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
    result = json_object();
    if result.is_null() {
        return 0 as *mut json_t;
    }
    iter = json_object_iter(object);
    while !iter.is_null() {
        let mut key: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *mut json_t = 0 as *mut json_t;
        key = json_object_iter_key(iter);
        value = json_object_iter_value(iter);
        json_object_set_new_nocheck(result, key, json_deep_copy(value));
        iter = json_object_iter_next(object, iter);
    }
    return result;
}
pub unsafe extern "C" fn json_array() -> *mut json_t {
    let mut array: *mut json_array_t = malloc(
        ::std::mem::size_of::<json_array_t>() as libc::c_ulong,
    ) as *mut json_array_t;
    if array.is_null() {
        return 0 as *mut json_t;
    }
    json_init(&mut (*array).json, JSON_ARRAY);
    (*array).entries = 0 as libc::c_int as size_t;
    (*array).size = 8 as libc::c_int as size_t;
    (*array)
        .table = malloc(
        ((*array).size)
            .wrapping_mul(::std::mem::size_of::<*mut json_t>() as libc::c_ulong),
    ) as *mut *mut json_t;
    if ((*array).table).is_null() {
        free(array as *mut libc::c_void);
        return 0 as *mut json_t;
    }
    (*array).visited = 0 as libc::c_int;
    return &mut (*array).json;
}
unsafe extern "C" fn json_delete_array(mut array: *mut json_array_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*array).entries {
        json_decref(*((*array).table).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free((*array).table as *mut libc::c_void);
    free(array as *mut libc::c_void);
}
pub unsafe extern "C" fn json_array_size(mut json: *const json_t) -> size_t {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int as size_t;
    }
    return (*((json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t))
        .entries;
}
pub unsafe extern "C" fn json_array_get(
    mut json: *const json_t,
    mut index: size_t,
) -> *mut json_t {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        return 0 as *mut json_t;
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    if index >= (*array).entries {
        return 0 as *mut json_t;
    }
    return *((*array).table).offset(index as isize);
}
pub unsafe extern "C" fn json_array_set_new(
    mut json: *mut json_t,
    mut index: size_t,
    mut value: *mut json_t,
) -> libc::c_int {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    if value.is_null() {
        return -(1 as libc::c_int);
    }
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
        || json == value
    {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    if index >= (*array).entries {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    json_decref(*((*array).table).offset(index as isize));
    let ref mut fresh1 = *((*array).table).offset(index as isize);
    *fresh1 = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn array_move(
    mut array: *mut json_array_t,
    mut dest: size_t,
    mut src: size_t,
    mut count: size_t,
) {
    memmove(
        &mut *((*array).table).offset(dest as isize) as *mut *mut json_t
            as *mut libc::c_void,
        &mut *((*array).table).offset(src as isize) as *mut *mut json_t
            as *const libc::c_void,
        count.wrapping_mul(::std::mem::size_of::<*mut json_t>() as libc::c_ulong),
    );
}
unsafe extern "C" fn array_copy(
    mut dest: *mut *mut json_t,
    mut dpos: size_t,
    mut src: *mut *mut json_t,
    mut spos: size_t,
    mut count: size_t,
) {
    memcpy(
        &mut *dest.offset(dpos as isize) as *mut *mut json_t as *mut libc::c_void,
        &mut *src.offset(spos as isize) as *mut *mut json_t as *const libc::c_void,
        count.wrapping_mul(::std::mem::size_of::<*mut json_t>() as libc::c_ulong),
    );
}
unsafe extern "C" fn json_array_grow(
    mut array: *mut json_array_t,
    mut amount: size_t,
    mut copy: libc::c_int,
) -> *mut *mut json_t {
    let mut new_size: size_t = 0;
    let mut old_table: *mut *mut json_t = 0 as *mut *mut json_t;
    let mut new_table: *mut *mut json_t = 0 as *mut *mut json_t;
    if ((*array).entries).wrapping_add(amount) <= (*array).size {
        return (*array).table;
    }
    old_table = (*array).table;
    new_size = if ((*array).size).wrapping_add(amount)
        > ((*array).size).wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        ((*array).size).wrapping_add(amount)
    } else {
        ((*array).size).wrapping_mul(2 as libc::c_int as libc::c_ulong)
    };
    new_table = malloc(
        new_size.wrapping_mul(::std::mem::size_of::<*mut json_t>() as libc::c_ulong),
    ) as *mut *mut json_t;
    if new_table.is_null() {
        return 0 as *mut *mut json_t;
    }
    (*array).size = new_size;
    (*array).table = new_table;
    if copy != 0 {
        array_copy(
            (*array).table,
            0 as libc::c_int as size_t,
            old_table,
            0 as libc::c_int as size_t,
            (*array).entries,
        );
        free(old_table as *mut libc::c_void);
        return (*array).table;
    }
    return old_table;
}
pub unsafe extern "C" fn json_array_append_new(
    mut json: *mut json_t,
    mut value: *mut json_t,
) -> libc::c_int {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    if value.is_null() {
        return -(1 as libc::c_int);
    }
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
        || json == value
    {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    if (json_array_grow(array, 1 as libc::c_int as size_t, 1 as libc::c_int)).is_null() {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    let ref mut fresh2 = *((*array).table).offset((*array).entries as isize);
    *fresh2 = value;
    (*array).entries = ((*array).entries).wrapping_add(1);
    (*array).entries;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_array_insert_new(
    mut json: *mut json_t,
    mut index: size_t,
    mut value: *mut json_t,
) -> libc::c_int {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    let mut old_table: *mut *mut json_t = 0 as *mut *mut json_t;
    if value.is_null() {
        return -(1 as libc::c_int);
    }
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
        || json == value
    {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    if index > (*array).entries {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    old_table = json_array_grow(array, 1 as libc::c_int as size_t, 0 as libc::c_int);
    if old_table.is_null() {
        json_decref(value);
        return -(1 as libc::c_int);
    }
    if old_table != (*array).table {
        array_copy(
            (*array).table,
            0 as libc::c_int as size_t,
            old_table,
            0 as libc::c_int as size_t,
            index,
        );
        array_copy(
            (*array).table,
            index.wrapping_add(1 as libc::c_int as libc::c_ulong),
            old_table,
            index,
            ((*array).entries).wrapping_sub(index),
        );
        free(old_table as *mut libc::c_void);
    } else {
        array_move(
            array,
            index.wrapping_add(1 as libc::c_int as libc::c_ulong),
            index,
            ((*array).entries).wrapping_sub(index),
        );
    }
    let ref mut fresh3 = *((*array).table).offset(index as isize);
    *fresh3 = value;
    (*array).entries = ((*array).entries).wrapping_add(1);
    (*array).entries;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_array_remove(
    mut json: *mut json_t,
    mut index: size_t,
) -> libc::c_int {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    if index >= (*array).entries {
        return -(1 as libc::c_int);
    }
    json_decref(*((*array).table).offset(index as isize));
    array_move(
        array,
        index,
        index.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ((*array).entries).wrapping_sub(index),
    );
    (*array).entries = ((*array).entries).wrapping_sub(1);
    (*array).entries;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_array_clear(mut json: *mut json_t) -> libc::c_int {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    let mut i: size_t = 0;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    i = 0 as libc::c_int as size_t;
    while i < (*array).entries {
        json_decref(*((*array).table).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    (*array).entries = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_array_extend(
    mut json: *mut json_t,
    mut other_json: *mut json_t,
) -> libc::c_int {
    let mut array: *mut json_array_t = 0 as *mut json_array_t;
    let mut other: *mut json_array_t = 0 as *mut json_array_t;
    let mut i: size_t = 0;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
        || !(!other_json.is_null()
            && (*other_json).type_0 as libc::c_uint
                == JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    other = (other_json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_array_t;
    if (json_array_grow(array, (*other).entries, 1 as libc::c_int)).is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < (*other).entries {
        json_incref(*((*other).table).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    array_copy(
        (*array).table,
        (*array).entries,
        (*other).table,
        0 as libc::c_int as size_t,
        (*other).entries,
    );
    (*array)
        .entries = ((*array).entries as libc::c_ulong).wrapping_add((*other).entries)
        as size_t as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn json_array_equal(
    mut array1: *mut json_t,
    mut array2: *mut json_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut size: size_t = 0;
    size = json_array_size(array1);
    if size != json_array_size(array2) {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        let mut value1: *mut json_t = 0 as *mut json_t;
        let mut value2: *mut json_t = 0 as *mut json_t;
        value1 = json_array_get(array1, i);
        value2 = json_array_get(array2, i);
        if json_equal(value1, value2) == 0 {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn json_array_copy(mut array: *mut json_t) -> *mut json_t {
    let mut result: *mut json_t = 0 as *mut json_t;
    let mut i: size_t = 0;
    result = json_array();
    if result.is_null() {
        return 0 as *mut json_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < json_array_size(array) {
        json_array_append(result, json_array_get(array, i));
        i = i.wrapping_add(1);
        i;
    }
    return result;
}
unsafe extern "C" fn json_array_deep_copy(mut array: *mut json_t) -> *mut json_t {
    let mut result: *mut json_t = 0 as *mut json_t;
    let mut i: size_t = 0;
    result = json_array();
    if result.is_null() {
        return 0 as *mut json_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < json_array_size(array) {
        json_array_append_new(result, json_deep_copy(json_array_get(array, i)));
        i = i.wrapping_add(1);
        i;
    }
    return result;
}
pub unsafe extern "C" fn json_string_nocheck(
    mut value: *const libc::c_char,
) -> *mut json_t {
    let mut string: *mut json_string_t = 0 as *mut json_string_t;
    if value.is_null() {
        return 0 as *mut json_t;
    }
    string = malloc(::std::mem::size_of::<json_string_t>() as libc::c_ulong)
        as *mut json_string_t;
    if string.is_null() {
        return 0 as *mut json_t;
    }
    json_init(&mut (*string).json, JSON_STRING);
    (*string).value = strdup(value);
    if ((*string).value).is_null() {
        free(string as *mut libc::c_void);
        return 0 as *mut json_t;
    }
    return &mut (*string).json;
}
pub unsafe extern "C" fn json_string(mut value: *const libc::c_char) -> *mut json_t {
    if value.is_null() || utf8_check_string(value, -(1 as libc::c_int)) == 0 {
        return 0 as *mut json_t;
    }
    return json_string_nocheck(value);
}
pub unsafe extern "C" fn json_string_value(
    mut json: *const json_t,
) -> *const libc::c_char {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint)
    {
        return 0 as *const libc::c_char;
    }
    return (*((json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_string_t))
        .value;
}
pub unsafe extern "C" fn json_string_set_nocheck(
    mut json: *mut json_t,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut dup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut json_string_t = 0 as *mut json_string_t;
    dup = strdup(value);
    if dup.is_null() {
        return -(1 as libc::c_int);
    }
    string = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_string_t;
    free((*string).value as *mut libc::c_void);
    (*string).value = dup;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_string_set(
    mut json: *mut json_t,
    mut value: *const libc::c_char,
) -> libc::c_int {
    if value.is_null() || utf8_check_string(value, -(1 as libc::c_int)) == 0 {
        return -(1 as libc::c_int);
    }
    return json_string_set_nocheck(json, value);
}
unsafe extern "C" fn json_delete_string(mut string: *mut json_string_t) {
    free((*string).value as *mut libc::c_void);
    free(string as *mut libc::c_void);
}
unsafe extern "C" fn json_string_equal(
    mut string1: *mut json_t,
    mut string2: *mut json_t,
) -> libc::c_int {
    return (strcmp(json_string_value(string1), json_string_value(string2))
        == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn json_string_copy(mut string: *mut json_t) -> *mut json_t {
    return json_string_nocheck(json_string_value(string));
}
pub unsafe extern "C" fn json_integer(mut value: json_int_t) -> *mut json_t {
    let mut integer: *mut json_integer_t = malloc(
        ::std::mem::size_of::<json_integer_t>() as libc::c_ulong,
    ) as *mut json_integer_t;
    if integer.is_null() {
        return 0 as *mut json_t;
    }
    json_init(&mut (*integer).json, JSON_INTEGER);
    (*integer).value = value;
    return &mut (*integer).json;
}
pub unsafe extern "C" fn json_integer_value(mut json: *const json_t) -> json_int_t {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int as json_int_t;
    }
    return (*((json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_integer_t))
        .value;
}
pub unsafe extern "C" fn json_integer_set(
    mut json: *mut json_t,
    mut value: json_int_t,
) -> libc::c_int {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    (*((json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_integer_t))
        .value = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn json_delete_integer(mut integer: *mut json_integer_t) {
    free(integer as *mut libc::c_void);
}
unsafe extern "C" fn json_integer_equal(
    mut integer1: *mut json_t,
    mut integer2: *mut json_t,
) -> libc::c_int {
    return (json_integer_value(integer1) == json_integer_value(integer2)) as libc::c_int;
}
unsafe extern "C" fn json_integer_copy(mut integer: *mut json_t) -> *mut json_t {
    return json_integer(json_integer_value(integer));
}
pub unsafe extern "C" fn json_real(mut value: libc::c_double) -> *mut json_t {
    let mut real: *mut json_real_t = malloc(
        ::std::mem::size_of::<json_real_t>() as libc::c_ulong,
    ) as *mut json_real_t;
    if real.is_null() {
        return 0 as *mut json_t;
    }
    json_init(&mut (*real).json, JSON_REAL);
    (*real).value = value;
    return &mut (*real).json;
}
pub unsafe extern "C" fn json_real_value(mut json: *const json_t) -> libc::c_double {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int as libc::c_double;
    }
    return (*((json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_real_t))
        .value;
}
pub unsafe extern "C" fn json_real_set(
    mut json: *mut json_t,
    mut value: libc::c_double,
) -> libc::c_int {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    (*((json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut json_real_t))
        .value = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn json_delete_real(mut real: *mut json_real_t) {
    free(real as *mut libc::c_void);
}
unsafe extern "C" fn json_real_equal(
    mut real1: *mut json_t,
    mut real2: *mut json_t,
) -> libc::c_int {
    return (json_real_value(real1) == json_real_value(real2)) as libc::c_int;
}
unsafe extern "C" fn json_real_copy(mut real: *mut json_t) -> *mut json_t {
    return json_real(json_real_value(real));
}
pub unsafe extern "C" fn json_number_value(mut json: *const json_t) -> libc::c_double {
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint
    {
        return json_integer_value(json) as libc::c_double
    } else if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint
    {
        return json_real_value(json)
    } else {
        return 0.0f64
    };
}
pub unsafe extern "C" fn json_true() -> *mut json_t {
    static mut the_true: json_t = {
        let mut init = json_t {
            type_0: JSON_TRUE,
            refcount: -(1 as libc::c_int) as size_t,
        };
        init
    };
    return &mut the_true;
}
pub unsafe extern "C" fn json_false() -> *mut json_t {
    static mut the_false: json_t = {
        let mut init = json_t {
            type_0: JSON_FALSE,
            refcount: -(1 as libc::c_int) as size_t,
        };
        init
    };
    return &mut the_false;
}
pub unsafe extern "C" fn json_null() -> *mut json_t {
    static mut the_null: json_t = {
        let mut init = json_t {
            type_0: JSON_NULL,
            refcount: -(1 as libc::c_int) as size_t,
        };
        init
    };
    return &mut the_null;
}
pub unsafe extern "C" fn json_delete(mut json: *mut json_t) {
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint
    {
        json_delete_object(
            (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_object_t,
        );
    } else if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint
    {
        json_delete_array(
            (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_array_t,
        );
    } else if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
    {
        json_delete_string(
            (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_string_t,
        );
    } else if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint
    {
        json_delete_integer(
            (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_integer_t,
        );
    } else if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint
    {
        json_delete_real(
            (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_real_t,
        );
    }
}
pub unsafe extern "C" fn json_equal(
    mut json1: *mut json_t,
    mut json2: *mut json_t,
) -> libc::c_int {
    if json1.is_null() || json2.is_null() {
        return 0 as libc::c_int;
    }
    if (*json1).type_0 as libc::c_uint != (*json2).type_0 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if json1 == json2 {
        return 1 as libc::c_int;
    }
    if !json1.is_null()
        && (*json1).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint
    {
        return json_object_equal(json1, json2);
    }
    if !json1.is_null()
        && (*json1).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint
    {
        return json_array_equal(json1, json2);
    }
    if !json1.is_null()
        && (*json1).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
    {
        return json_string_equal(json1, json2);
    }
    if !json1.is_null()
        && (*json1).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint
    {
        return json_integer_equal(json1, json2);
    }
    if !json1.is_null()
        && (*json1).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint
    {
        return json_real_equal(json1, json2);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_copy(mut json: *mut json_t) -> *mut json_t {
    if json.is_null() {
        return 0 as *mut json_t;
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint
    {
        return json_object_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint
    {
        return json_array_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
    {
        return json_string_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint
    {
        return json_integer_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint
    {
        return json_real_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_TRUE as libc::c_int as libc::c_uint
        || !json.is_null()
            && (*json).type_0 as libc::c_uint
                == JSON_FALSE as libc::c_int as libc::c_uint
        || !json.is_null()
            && (*json).type_0 as libc::c_uint == JSON_NULL as libc::c_int as libc::c_uint
    {
        return json;
    }
    return 0 as *mut json_t;
}
pub unsafe extern "C" fn json_deep_copy(mut json: *mut json_t) -> *mut json_t {
    if json.is_null() {
        return 0 as *mut json_t;
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_OBJECT as libc::c_int as libc::c_uint
    {
        return json_object_deep_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint
    {
        return json_array_deep_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
    {
        return json_string_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_INTEGER as libc::c_int as libc::c_uint
    {
        return json_integer_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_REAL as libc::c_int as libc::c_uint
    {
        return json_real_copy(json);
    }
    if !json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_TRUE as libc::c_int as libc::c_uint
        || !json.is_null()
            && (*json).type_0 as libc::c_uint
                == JSON_FALSE as libc::c_int as libc::c_uint
        || !json.is_null()
            && (*json).type_0 as libc::c_uint == JSON_NULL as libc::c_int as libc::c_uint
    {
        return json;
    }
    return 0 as *mut json_t;
}

use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type method = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY_T {
    pub index: libc::c_int,
    pub length: libc::c_int,
    pub data: *mut array,
    pub free: method,
}
pub type array = *mut libc::c_void;
pub type ARRAY = *mut ARRAY_T;
pub static mut ARRAYSIZE: size_t = ::std::mem::size_of::<ARRAY_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_array() -> ARRAY {
    let mut this: ARRAY = 0 as *mut ARRAY_T;
    this = xcalloc(
        ::std::mem::size_of::<ARRAY_T>() as libc::c_ulong,
        1 as libc::c_int as size_t,
    ) as ARRAY;
    (*this).index = -(1 as libc::c_int);
    (*this).length = 0 as libc::c_int;
    (*this).free = None;
    return this;
}
pub unsafe extern "C" fn array_destroy(mut this: ARRAY) -> ARRAY {
    let mut i: libc::c_int = 0;
    if this.is_null() {
        return 0 as ARRAY;
    }
    if ((*this).free).is_none() {
        (*this).free = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    }
    i = 0 as libc::c_int;
    while i < (*this).length {
        ((*this).free).unwrap()(*((*this).data).offset(i as isize));
        i += 1;
        i;
    }
    xfree((*this).data as *mut libc::c_void);
    xfree(this as *mut libc::c_void);
    this = 0 as ARRAY;
    return this;
}
pub unsafe extern "C" fn array_destroyer(mut this: ARRAY, mut m: method) -> ARRAY {
    (*this).free = m;
    return array_destroy(this);
}
pub unsafe extern "C" fn array_set_destroyer(mut this: ARRAY, mut m: method) {
    (*this).free = m;
}
pub unsafe extern "C" fn array_push(mut this: ARRAY, mut thing: *mut libc::c_void) {
    let mut len: libc::c_int = 0 as libc::c_int;
    if thing.is_null() {
        return;
    }
    len = (strlen(thing as *const libc::c_char))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    array_npush(this, thing, len as size_t);
}
pub unsafe extern "C" fn array_npush(
    mut this: ARRAY,
    mut thing: *mut libc::c_void,
    mut len: size_t,
) {
    let mut arr: array = 0 as *mut libc::c_void;
    if thing.is_null() {
        return;
    }
    if ((*this).data).is_null() && (*this).length == 0 as libc::c_int {
        (*this)
            .data = xmalloc(::std::mem::size_of::<array>() as libc::c_ulong)
            as *mut array;
    } else {
        (*this)
            .data = realloc(
            (*this).data as *mut libc::c_void,
            (((*this).length + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<array>() as libc::c_ulong),
        ) as *mut array;
    }
    arr = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    memset(arr, '\0' as i32, len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    memcpy(arr, thing, len);
    let ref mut fresh0 = *((*this).data).offset((*this).length as isize);
    *fresh0 = arr;
    (*this).length += 1 as libc::c_int;
}
pub unsafe extern "C" fn array_get(
    mut this: ARRAY,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    if index > (*this).length {
        return 0 as *mut libc::c_void;
    }
    return *((*this).data).offset(index as isize);
}
pub unsafe extern "C" fn array_remove(
    mut this: ARRAY,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut arr: array = 0 as *mut libc::c_void;
    if index > (*this).length {
        return 0 as *mut libc::c_void;
    }
    arr = *((*this).data).offset(index as isize);
    (*this).length -= 1;
    length = (*this).length;
    while index < length {
        let ref mut fresh1 = *((*this).data).offset(index as isize);
        *fresh1 = *((*this).data).offset((index + 1 as libc::c_int) as isize);
        index += 1;
        index;
    }
    return arr;
}
pub unsafe extern "C" fn array_pop(mut this: ARRAY) -> *mut libc::c_void {
    if this.is_null() {
        return 0 as *mut libc::c_void;
    }
    return if (*this).length != 0 {
        (*this).length -= 1;
        *((*this).data).offset((*this).length as isize)
    } else {
        0 as *mut libc::c_void
    };
}
pub unsafe extern "C" fn array_next(mut this: ARRAY) -> *mut libc::c_void {
    (*this).index += 1;
    (*this).index;
    return *((*this).data).offset(((*this).index % (*this).length) as isize);
}
pub unsafe extern "C" fn array_prev(mut this: ARRAY) -> *mut libc::c_void {
    (*this).index -= 1;
    (*this).index;
    return *((*this).data)
        .offset(
            (((*this).index + ((*this).length - 1 as libc::c_int)) % (*this).length)
                as isize,
        );
}
pub unsafe extern "C" fn array_length(mut this: ARRAY) -> size_t {
    return (*this).length as size_t;
}
pub unsafe extern "C" fn array_to_string(mut this: ARRAY) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*this).length == 0 as libc::c_int {
        return b"NULL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    i = 0 as libc::c_int as size_t;
    while i < array_length(this) {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (strlen(array_get(this, i as libc::c_int) as *const libc::c_char))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    str = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    memset(
        str as *mut libc::c_void,
        '\0' as i32,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    while i < array_length(this) {
        strcat(str, b"[\0" as *const u8 as *const libc::c_char);
        strcat(str, array_get(this, i as libc::c_int) as *const libc::c_char);
        if i == (array_length(this)).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            strcat(str, b"]\0" as *const u8 as *const libc::c_char);
        } else {
            strcat(str, b"],\0" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        i;
    }
    return str;
}
pub unsafe extern "C" fn array_print(mut this: ARRAY) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, array_to_string(this));
}

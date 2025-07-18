use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strbuffer_t {
    pub value: *mut libc::c_char,
    pub length: libc::c_int,
    pub size: libc::c_int,
}
pub unsafe extern "C" fn strbuffer_init(mut strbuff: *mut strbuffer_t) -> libc::c_int {
    (*strbuff).size = 16 as libc::c_int;
    (*strbuff).length = 0 as libc::c_int;
    (*strbuff).value = malloc((*strbuff).size as libc::c_ulong) as *mut libc::c_char;
    if ((*strbuff).value).is_null() {
        return -(1 as libc::c_int);
    }
    *((*strbuff).value).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strbuffer_close(mut strbuff: *mut strbuffer_t) {
    free((*strbuff).value as *mut libc::c_void);
    (*strbuff).size = 0 as libc::c_int;
    (*strbuff).length = 0 as libc::c_int;
    (*strbuff).value = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn strbuffer_clear(mut strbuff: *mut strbuffer_t) {
    (*strbuff).length = 0 as libc::c_int;
    *((*strbuff).value).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn strbuffer_value(
    mut strbuff: *const strbuffer_t,
) -> *const libc::c_char {
    return (*strbuff).value;
}
pub unsafe extern "C" fn strbuffer_steal_value(
    mut strbuff: *mut strbuffer_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = (*strbuff).value;
    strbuffer_init(strbuff);
    return result;
}
pub unsafe extern "C" fn strbuffer_append(
    mut strbuff: *mut strbuffer_t,
    mut string: *const libc::c_char,
) -> libc::c_int {
    return strbuffer_append_bytes(strbuff, string, strlen(string) as libc::c_int);
}
pub unsafe extern "C" fn strbuffer_append_byte(
    mut strbuff: *mut strbuffer_t,
    mut byte: libc::c_char,
) -> libc::c_int {
    return strbuffer_append_bytes(strbuff, &mut byte, 1 as libc::c_int);
}
pub unsafe extern "C" fn strbuffer_append_bytes(
    mut strbuff: *mut strbuffer_t,
    mut data: *const libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    if (*strbuff).length + size >= (*strbuff).size {
        (*strbuff)
            .size = if (*strbuff).size * 2 as libc::c_int
            > (*strbuff).length + size + 1 as libc::c_int
        {
            (*strbuff).size * 2 as libc::c_int
        } else {
            (*strbuff).length + size + 1 as libc::c_int
        };
        (*strbuff)
            .value = realloc(
            (*strbuff).value as *mut libc::c_void,
            (*strbuff).size as libc::c_ulong,
        ) as *mut libc::c_char;
        if ((*strbuff).value).is_null() {
            return -(1 as libc::c_int);
        }
    }
    memcpy(
        ((*strbuff).value).offset((*strbuff).length as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        size as libc::c_ulong,
    );
    (*strbuff).length += size;
    *((*strbuff).value).offset((*strbuff).length as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strbuffer_pop(mut strbuff: *mut strbuffer_t) -> libc::c_char {
    if (*strbuff).length > 0 as libc::c_int {
        (*strbuff).length -= 1;
        let mut c: libc::c_char = *((*strbuff).value).offset((*strbuff).length as isize);
        *((*strbuff).value)
            .offset((*strbuff).length as isize) = '\0' as i32 as libc::c_char;
        return c;
    } else {
        return '\0' as i32 as libc::c_char
    };
}

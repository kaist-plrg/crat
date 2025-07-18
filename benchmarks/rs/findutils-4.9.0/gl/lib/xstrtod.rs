use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub unsafe extern "C" fn xstrtod(
    mut str: *const libc::c_char,
    mut ptr: *mut *const libc::c_char,
    mut result: *mut libc::c_double,
    mut convert: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut *mut libc::c_char,
        ) -> libc::c_double,
    >,
) -> bool {
    let mut val: libc::c_double = 0.;
    let mut terminator: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: bool = 1 as libc::c_int != 0;
    *__errno_location() = 0 as libc::c_int;
    val = convert.unwrap()(str, &mut terminator);
    if terminator == str as *mut libc::c_char
        || ptr.is_null() && *terminator as libc::c_int != '\0' as i32
    {
        ok = 0 as libc::c_int != 0;
    } else if val != 0 as libc::c_int as libc::c_double
        && *__errno_location() == 34 as libc::c_int
    {
        ok = 0 as libc::c_int != 0;
    }
    if !ptr.is_null() {
        *ptr = terminator;
    }
    *result = val;
    return ok;
}

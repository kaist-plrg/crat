use ::libc;
extern "C" {
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
pub unsafe extern "C" fn _glp_strtrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = (strrchr(str, '\0' as i32)).offset(-(1 as libc::c_int as isize));
    while t >= str {
        if *t as libc::c_int != ' ' as i32 {
            break;
        }
        *t = '\0' as i32 as libc::c_char;
        t = t.offset(-1);
        t;
    }
    return str;
}

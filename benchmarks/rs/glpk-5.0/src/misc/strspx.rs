use ::libc;
pub unsafe extern "C" fn _glp_strspx(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = str;
    s = t;
    while *s != 0 {
        if *s as libc::c_int != ' ' as i32 {
            let fresh0 = t;
            t = t.offset(1);
            *fresh0 = *s;
        }
        s = s.offset(1);
        s;
    }
    *t = '\0' as i32 as libc::c_char;
    return str;
}

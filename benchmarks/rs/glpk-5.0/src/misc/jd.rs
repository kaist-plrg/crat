use ::libc;
pub unsafe extern "C" fn _glp_jday(
    mut d: libc::c_int,
    mut m: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ya: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    if !(1 as libc::c_int <= d && d <= 31 as libc::c_int && 1 as libc::c_int <= m
        && m <= 12 as libc::c_int && 1 as libc::c_int <= y && y <= 4000 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if m >= 3 as libc::c_int {
        m -= 3 as libc::c_int;
    } else {
        m += 9 as libc::c_int;
        y -= 1;
        y;
    }
    c = y / 100 as libc::c_int;
    ya = y - 100 as libc::c_int * c;
    j = 146097 as libc::c_int * c / 4 as libc::c_int
        + 1461 as libc::c_int * ya / 4 as libc::c_int
        + (153 as libc::c_int * m + 2 as libc::c_int) / 5 as libc::c_int + d
        + 1721119 as libc::c_int;
    _glp_jdate(j, &mut dd, 0 as *mut libc::c_int, 0 as *mut libc::c_int);
    if d != dd {
        return -(1 as libc::c_int);
    }
    return j;
}
pub unsafe extern "C" fn _glp_jdate(
    mut j: libc::c_int,
    mut d_: *mut libc::c_int,
    mut m_: *mut libc::c_int,
    mut y_: *mut libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if !(1721426 as libc::c_int <= j && j <= 3182395 as libc::c_int) {
        return 1 as libc::c_int;
    }
    j -= 1721119 as libc::c_int;
    y = (4 as libc::c_int * j - 1 as libc::c_int) / 146097 as libc::c_int;
    j = (4 as libc::c_int * j - 1 as libc::c_int) % 146097 as libc::c_int;
    d = j / 4 as libc::c_int;
    j = (4 as libc::c_int * d + 3 as libc::c_int) / 1461 as libc::c_int;
    d = (4 as libc::c_int * d + 3 as libc::c_int) % 1461 as libc::c_int;
    d = (d + 4 as libc::c_int) / 4 as libc::c_int;
    m = (5 as libc::c_int * d - 3 as libc::c_int) / 153 as libc::c_int;
    d = (5 as libc::c_int * d - 3 as libc::c_int) % 153 as libc::c_int;
    d = (d + 5 as libc::c_int) / 5 as libc::c_int;
    y = 100 as libc::c_int * y + j;
    if m <= 9 as libc::c_int {
        m += 3 as libc::c_int;
    } else {
        m -= 9 as libc::c_int;
        y += 1;
        y;
    }
    if !d_.is_null() {
        *d_ = d;
    }
    if !m_.is_null() {
        *m_ = m;
    }
    if !y_.is_null() {
        *y_ = y;
    }
    return 0 as libc::c_int;
}

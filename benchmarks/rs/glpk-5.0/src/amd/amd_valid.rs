use ::libc;
pub unsafe extern "C" fn _glp_amd_valid(
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
    mut Ap: *const libc::c_int,
    mut Ai: *const libc::c_int,
) -> libc::c_int {
    let mut nz: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut ilast: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut result: libc::c_int = 0 as libc::c_int;
    if n_row < 0 as libc::c_int || n_col < 0 as libc::c_int || Ap.is_null()
        || Ai.is_null()
    {
        return -(2 as libc::c_int);
    }
    nz = *Ap.offset(n_col as isize);
    if *Ap.offset(0 as libc::c_int as isize) != 0 as libc::c_int || nz < 0 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    j = 0 as libc::c_int;
    while j < n_col {
        p1 = *Ap.offset(j as isize);
        p2 = *Ap.offset((j + 1 as libc::c_int) as isize);
        if p1 > p2 {
            return -(2 as libc::c_int);
        }
        ilast = -(1 as libc::c_int);
        p = p1;
        while p < p2 {
            i = *Ai.offset(p as isize);
            if i < 0 as libc::c_int || i >= n_row {
                return -(2 as libc::c_int);
            }
            if i <= ilast {
                result = 1 as libc::c_int;
            }
            ilast = i;
            p += 1;
            p;
        }
        j += 1;
        j;
    }
    return result;
}

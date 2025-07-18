use ::libc;
pub unsafe extern "C" fn _glp_amd_preprocess(
    mut n: libc::c_int,
    mut Ap: *const libc::c_int,
    mut Ai: *const libc::c_int,
    mut Rp: *mut libc::c_int,
    mut Ri: *mut libc::c_int,
    mut W: *mut libc::c_int,
    mut Flag: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *W.offset(i as isize) = 0 as libc::c_int;
        *Flag.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < n {
        p2 = *Ap.offset((j + 1 as libc::c_int) as isize);
        p = *Ap.offset(j as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            if *Flag.offset(i as isize) != j {
                let ref mut fresh0 = *W.offset(i as isize);
                *fresh0 += 1;
                *fresh0;
                *Flag.offset(i as isize) = j;
            }
            p += 1;
            p;
        }
        j += 1;
        j;
    }
    *Rp.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *Rp
            .offset(
                (i + 1 as libc::c_int) as isize,
            ) = *Rp.offset(i as isize) + *W.offset(i as isize);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        *W.offset(i as isize) = *Rp.offset(i as isize);
        *Flag.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < n {
        p2 = *Ap.offset((j + 1 as libc::c_int) as isize);
        p = *Ap.offset(j as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            if *Flag.offset(i as isize) != j {
                let ref mut fresh1 = *W.offset(i as isize);
                let fresh2 = *fresh1;
                *fresh1 = *fresh1 + 1;
                *Ri.offset(fresh2 as isize) = j;
                *Flag.offset(i as isize) = j;
            }
            p += 1;
            p;
        }
        j += 1;
        j;
    }
}

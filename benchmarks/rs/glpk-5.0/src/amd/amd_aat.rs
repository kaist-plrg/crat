use ::libc;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn _glp_amd_aat(
    mut n: libc::c_int,
    mut Ap: *const libc::c_int,
    mut Ai: *const libc::c_int,
    mut Len: *mut libc::c_int,
    mut Tp: *mut libc::c_int,
    mut Info: *mut libc::c_double,
) -> size_t {
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pj: libc::c_int = 0;
    let mut pj2: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nzdiag: libc::c_int = 0;
    let mut nzboth: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut sym: libc::c_double = 0.;
    let mut nzaat: size_t = 0;
    if !Info.is_null() {
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            *Info.offset(i as isize) = -(1 as libc::c_int) as libc::c_double;
            i += 1;
            i;
        }
        *Info.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    }
    k = 0 as libc::c_int;
    while k < n {
        *Len.offset(k as isize) = 0 as libc::c_int;
        k += 1;
        k;
    }
    nzdiag = 0 as libc::c_int;
    nzboth = 0 as libc::c_int;
    nz = *Ap.offset(n as isize);
    k = 0 as libc::c_int;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as libc::c_int) as isize);
        p = p1;
        while p < p2 {
            j = *Ai.offset(p as isize);
            if j < k {
                let ref mut fresh0 = *Len.offset(j as isize);
                *fresh0 += 1;
                *fresh0;
                let ref mut fresh1 = *Len.offset(k as isize);
                *fresh1 += 1;
                *fresh1;
                p += 1;
                p;
                pj2 = *Ap.offset((j + 1 as libc::c_int) as isize);
                pj = *Tp.offset(j as isize);
                while pj < pj2 {
                    i = *Ai.offset(pj as isize);
                    if i < k {
                        let ref mut fresh2 = *Len.offset(i as isize);
                        *fresh2 += 1;
                        *fresh2;
                        let ref mut fresh3 = *Len.offset(j as isize);
                        *fresh3 += 1;
                        *fresh3;
                        pj += 1;
                        pj;
                    } else if i == k {
                        pj += 1;
                        pj;
                        nzboth += 1;
                        nzboth;
                        break;
                    } else {
                        break;
                    }
                }
                *Tp.offset(j as isize) = pj;
            } else if j == k {
                p += 1;
                p;
                nzdiag += 1;
                nzdiag;
                break;
            } else {
                break;
            }
        }
        *Tp.offset(k as isize) = p;
        k += 1;
        k;
    }
    j = 0 as libc::c_int;
    while j < n {
        pj = *Tp.offset(j as isize);
        while pj < *Ap.offset((j + 1 as libc::c_int) as isize) {
            i = *Ai.offset(pj as isize);
            let ref mut fresh4 = *Len.offset(i as isize);
            *fresh4 += 1;
            *fresh4;
            let ref mut fresh5 = *Len.offset(j as isize);
            *fresh5 += 1;
            *fresh5;
            pj += 1;
            pj;
        }
        j += 1;
        j;
    }
    if nz == nzdiag {
        sym = 1 as libc::c_int as libc::c_double;
    } else {
        sym = 2 as libc::c_int as libc::c_double * nzboth as libc::c_double
            / (nz - nzdiag) as libc::c_double;
    }
    nzaat = 0 as libc::c_int as size_t;
    k = 0 as libc::c_int;
    while k < n {
        nzaat = (nzaat as libc::c_ulong)
            .wrapping_add(*Len.offset(k as isize) as libc::c_ulong) as size_t as size_t;
        k += 1;
        k;
    }
    if !Info.is_null() {
        *Info.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        *Info.offset(1 as libc::c_int as isize) = n as libc::c_double;
        *Info.offset(2 as libc::c_int as isize) = nz as libc::c_double;
        *Info.offset(3 as libc::c_int as isize) = sym;
        *Info.offset(4 as libc::c_int as isize) = nzdiag as libc::c_double;
        *Info.offset(5 as libc::c_int as isize) = nzaat as libc::c_double;
    }
    return nzaat;
}

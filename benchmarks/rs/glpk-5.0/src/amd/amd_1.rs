use ::libc;
extern "C" {
    fn _glp_amd_2(
        n: libc::c_int,
        Pe: *mut libc::c_int,
        Iw: *mut libc::c_int,
        Len: *mut libc::c_int,
        iwlen: libc::c_int,
        pfree: libc::c_int,
        Nv: *mut libc::c_int,
        Next: *mut libc::c_int,
        Last: *mut libc::c_int,
        Head: *mut libc::c_int,
        Elen: *mut libc::c_int,
        Degree: *mut libc::c_int,
        W: *mut libc::c_int,
        Control: *mut libc::c_double,
        Info: *mut libc::c_double,
    );
}
pub unsafe extern "C" fn _glp_amd_1(
    mut n: libc::c_int,
    mut Ap: *const libc::c_int,
    mut Ai: *const libc::c_int,
    mut P: *mut libc::c_int,
    mut Pinv: *mut libc::c_int,
    mut Len: *mut libc::c_int,
    mut slen: libc::c_int,
    mut S: *mut libc::c_int,
    mut Control: *mut libc::c_double,
    mut Info: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut pfree: libc::c_int = 0;
    let mut iwlen: libc::c_int = 0;
    let mut pj: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut pj2: libc::c_int = 0;
    let mut Iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Pe: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Nv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Head: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Elen: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Degree: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut W: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Sp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Tp: *mut libc::c_int = 0 as *mut libc::c_int;
    iwlen = slen - 6 as libc::c_int * n;
    s = S;
    Pe = s;
    s = s.offset(n as isize);
    Nv = s;
    s = s.offset(n as isize);
    Head = s;
    s = s.offset(n as isize);
    Elen = s;
    s = s.offset(n as isize);
    Degree = s;
    s = s.offset(n as isize);
    W = s;
    s = s.offset(n as isize);
    Iw = s;
    s = s.offset(iwlen as isize);
    Sp = Nv;
    Tp = W;
    pfree = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < n {
        *Pe.offset(j as isize) = pfree;
        *Sp.offset(j as isize) = pfree;
        pfree += *Len.offset(j as isize);
        j += 1;
        j;
    }
    k = 0 as libc::c_int;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as libc::c_int) as isize);
        p = p1;
        while p < p2 {
            j = *Ai.offset(p as isize);
            if j < k {
                let ref mut fresh0 = *Sp.offset(j as isize);
                let fresh1 = *fresh0;
                *fresh0 = *fresh0 + 1;
                *Iw.offset(fresh1 as isize) = k;
                let ref mut fresh2 = *Sp.offset(k as isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                *Iw.offset(fresh3 as isize) = j;
                p += 1;
                p;
                pj2 = *Ap.offset((j + 1 as libc::c_int) as isize);
                pj = *Tp.offset(j as isize);
                while pj < pj2 {
                    i = *Ai.offset(pj as isize);
                    if i < k {
                        let ref mut fresh4 = *Sp.offset(i as isize);
                        let fresh5 = *fresh4;
                        *fresh4 = *fresh4 + 1;
                        *Iw.offset(fresh5 as isize) = j;
                        let ref mut fresh6 = *Sp.offset(j as isize);
                        let fresh7 = *fresh6;
                        *fresh6 = *fresh6 + 1;
                        *Iw.offset(fresh7 as isize) = i;
                        pj += 1;
                        pj;
                    } else if i == k {
                        pj += 1;
                        pj;
                        break;
                    } else {
                        break;
                    }
                }
                *Tp.offset(j as isize) = pj;
            } else if j == k {
                p += 1;
                p;
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
            let ref mut fresh8 = *Sp.offset(i as isize);
            let fresh9 = *fresh8;
            *fresh8 = *fresh8 + 1;
            *Iw.offset(fresh9 as isize) = j;
            let ref mut fresh10 = *Sp.offset(j as isize);
            let fresh11 = *fresh10;
            *fresh10 = *fresh10 + 1;
            *Iw.offset(fresh11 as isize) = i;
            pj += 1;
            pj;
        }
        j += 1;
        j;
    }
    _glp_amd_2(
        n,
        Pe,
        Iw,
        Len,
        iwlen,
        pfree,
        Nv,
        Pinv,
        P,
        Head,
        Elen,
        Degree,
        W,
        Control,
        Info,
    );
}

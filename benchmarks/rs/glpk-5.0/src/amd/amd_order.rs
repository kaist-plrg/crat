use ::libc;
extern "C" {
    fn _glp_amd_valid(
        n_row: libc::c_int,
        n_col: libc::c_int,
        Ap: *const libc::c_int,
        Ai: *const libc::c_int,
    ) -> libc::c_int;
    fn _glp_amd_preprocess(
        n: libc::c_int,
        Ap: *const libc::c_int,
        Ai: *const libc::c_int,
        Rp: *mut libc::c_int,
        Ri: *mut libc::c_int,
        W: *mut libc::c_int,
        Flag: *mut libc::c_int,
    );
    fn _glp_amd_aat(
        n: libc::c_int,
        Ap: *const libc::c_int,
        Ai: *const libc::c_int,
        Len: *mut libc::c_int,
        Tp: *mut libc::c_int,
        Info: *mut libc::c_double,
    ) -> size_t;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_amd_1(
        n: libc::c_int,
        Ap: *const libc::c_int,
        Ai: *const libc::c_int,
        P: *mut libc::c_int,
        Pinv: *mut libc::c_int,
        Len: *mut libc::c_int,
        slen: libc::c_int,
        S: *mut libc::c_int,
        Control: *mut libc::c_double,
        Info: *mut libc::c_double,
    );
    fn glp_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn _glp_amd_order(
    mut n: libc::c_int,
    mut Ap: *const libc::c_int,
    mut Ai: *const libc::c_int,
    mut P: *mut libc::c_int,
    mut Control: *mut libc::c_double,
    mut Info: *mut libc::c_double,
) -> libc::c_int {
    let mut Len: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut S: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut Pinv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut info: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut Rp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Ri: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ok: libc::c_int = 0;
    let mut nzaat: size_t = 0;
    let mut slen: size_t = 0;
    let mut mem: libc::c_double = 0 as libc::c_int as libc::c_double;
    info = (Info != 0 as *mut libc::c_void as *mut libc::c_double) as libc::c_int;
    if info != 0 {
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            *Info.offset(i as isize) = -(1 as libc::c_int) as libc::c_double;
            i += 1;
            i;
        }
        *Info.offset(1 as libc::c_int as isize) = n as libc::c_double;
        *Info.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    }
    if Ai.is_null() || Ap.is_null() || P.is_null() || n < 0 as libc::c_int {
        if info != 0 {
            *Info
                .offset(
                    0 as libc::c_int as isize,
                ) = -(2 as libc::c_int) as libc::c_double;
        }
        return -(2 as libc::c_int);
    }
    if n == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    nz = *Ap.offset(n as isize);
    if info != 0 {
        *Info.offset(2 as libc::c_int as isize) = nz as libc::c_double;
    }
    if nz < 0 as libc::c_int {
        if info != 0 {
            *Info
                .offset(
                    0 as libc::c_int as isize,
                ) = -(2 as libc::c_int) as libc::c_double;
        }
        return -(2 as libc::c_int);
    }
    if n as size_t
        >= (!(0 as libc::c_int as size_t))
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        || nz as size_t
            >= (!(0 as libc::c_int as size_t))
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        if info != 0 {
            *Info
                .offset(
                    0 as libc::c_int as isize,
                ) = -(1 as libc::c_int) as libc::c_double;
        }
        return -(1 as libc::c_int);
    }
    status = _glp_amd_valid(n, n, Ap, Ai);
    if status == -(2 as libc::c_int) {
        if info != 0 {
            *Info
                .offset(
                    0 as libc::c_int as isize,
                ) = -(2 as libc::c_int) as libc::c_double;
        }
        return -(2 as libc::c_int);
    }
    Len = glp_alloc(
        1 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut libc::c_int;
    Pinv = glp_alloc(
        1 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut libc::c_int;
    mem += n as libc::c_double;
    mem += n as libc::c_double;
    if Len.is_null() || Pinv.is_null() {
        if !Len.is_null() {
            glp_free(Len as *mut libc::c_void);
        }
        if !Pinv.is_null() {
            glp_free(Pinv as *mut libc::c_void);
        }
        if info != 0 {
            *Info
                .offset(
                    0 as libc::c_int as isize,
                ) = -(1 as libc::c_int) as libc::c_double;
        }
        return -(1 as libc::c_int);
    }
    if status == 1 as libc::c_int {
        Rp = glp_alloc(
            1 as libc::c_int,
            ((n + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_int;
        Ri = glp_alloc(
            1 as libc::c_int,
            ((if nz > 1 as libc::c_int { nz } else { 1 as libc::c_int })
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_int;
        mem += (n + 1 as libc::c_int) as libc::c_double;
        mem
            += (if nz > 1 as libc::c_int { nz } else { 1 as libc::c_int })
                as libc::c_double;
        if Rp.is_null() || Ri.is_null() {
            if !Rp.is_null() {
                glp_free(Rp as *mut libc::c_void);
            }
            if !Ri.is_null() {
                glp_free(Ri as *mut libc::c_void);
            }
            if !Len.is_null() {
                glp_free(Len as *mut libc::c_void);
            }
            if !Pinv.is_null() {
                glp_free(Pinv as *mut libc::c_void);
            }
            if info != 0 {
                *Info
                    .offset(
                        0 as libc::c_int as isize,
                    ) = -(1 as libc::c_int) as libc::c_double;
            }
            return -(1 as libc::c_int);
        }
        _glp_amd_preprocess(n, Ap, Ai, Rp, Ri, Len, Pinv);
        Cp = Rp;
        Ci = Ri;
    } else {
        Rp = 0 as *mut libc::c_int;
        Ri = 0 as *mut libc::c_int;
        Cp = Ap as *mut libc::c_int;
        Ci = Ai as *mut libc::c_int;
    }
    nzaat = _glp_amd_aat(
        n,
        Cp as *const libc::c_int,
        Ci as *const libc::c_int,
        Len,
        P,
        Info,
    );
    S = 0 as *mut libc::c_int;
    slen = nzaat;
    ok = (slen.wrapping_add(nzaat.wrapping_div(5 as libc::c_int as libc::c_ulong))
        >= slen) as libc::c_int;
    slen = (slen as libc::c_ulong)
        .wrapping_add(nzaat.wrapping_div(5 as libc::c_int as libc::c_ulong)) as size_t
        as size_t;
    i = 0 as libc::c_int;
    while ok != 0 && i < 7 as libc::c_int {
        ok = (slen.wrapping_add(n as libc::c_ulong) > slen) as libc::c_int;
        slen = (slen as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
            as size_t;
        i += 1;
        i;
    }
    mem += slen as libc::c_double;
    ok = (ok != 0
        && slen
            < (!(0 as libc::c_int as size_t))
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        as libc::c_int;
    ok = (ok != 0 && slen < 2147483647 as libc::c_int as libc::c_ulong) as libc::c_int;
    if ok != 0 {
        S = glp_alloc(
            1 as libc::c_int,
            slen.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_int;
    }
    if S.is_null() {
        if !Rp.is_null() {
            glp_free(Rp as *mut libc::c_void);
        }
        if !Ri.is_null() {
            glp_free(Ri as *mut libc::c_void);
        }
        if !Len.is_null() {
            glp_free(Len as *mut libc::c_void);
        }
        if !Pinv.is_null() {
            glp_free(Pinv as *mut libc::c_void);
        }
        if info != 0 {
            *Info
                .offset(
                    0 as libc::c_int as isize,
                ) = -(1 as libc::c_int) as libc::c_double;
        }
        return -(1 as libc::c_int);
    }
    if info != 0 {
        *Info
            .offset(
                7 as libc::c_int as isize,
            ) = mem
            * ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_double;
    }
    _glp_amd_1(
        n,
        Cp as *const libc::c_int,
        Ci as *const libc::c_int,
        P,
        Pinv,
        Len,
        slen as libc::c_int,
        S,
        Control,
        Info,
    );
    if !Rp.is_null() {
        glp_free(Rp as *mut libc::c_void);
    }
    if !Ri.is_null() {
        glp_free(Ri as *mut libc::c_void);
    }
    if !Len.is_null() {
        glp_free(Len as *mut libc::c_void);
    }
    if !Pinv.is_null() {
        glp_free(Pinv as *mut libc::c_void);
    }
    if !S.is_null() {
        glp_free(S as *mut libc::c_void);
    }
    if info != 0 {
        *Info.offset(0 as libc::c_int as isize) = status as libc::c_double;
    }
    return status;
}

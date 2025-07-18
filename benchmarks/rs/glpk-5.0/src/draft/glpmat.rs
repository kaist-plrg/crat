use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _glp_genqmd(
        neqns: *mut libc::c_int,
        xadj: *mut libc::c_int,
        adjncy: *mut libc::c_int,
        perm: *mut libc::c_int,
        invp: *mut libc::c_int,
        deg: *mut libc::c_int,
        marker: *mut libc::c_int,
        rchset: *mut libc::c_int,
        nbrhd: *mut libc::c_int,
        qsize: *mut libc::c_int,
        qlink: *mut libc::c_int,
        nofsub: *mut libc::c_int,
    );
    fn _glp_amd_order(
        n: libc::c_int,
        Ap: *const libc::c_int,
        Ai: *const libc::c_int,
        P: *mut libc::c_int,
        Control: *mut libc::c_double,
        Info: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_amd_defaults(Control: *mut libc::c_double);
    fn _glp_symamd(
        n: libc::c_int,
        A: *mut libc::c_int,
        p: *mut libc::c_int,
        perm: *mut libc::c_int,
        knobs: *mut libc::c_double,
        stats: *mut libc::c_int,
        allocate_0: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
        release_0: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn _glp_mat_check_fvs(
    mut n: libc::c_int,
    mut nnz: libc::c_int,
    mut ind: *mut libc::c_int,
    mut vec: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut flag: *mut libc::c_int = 0 as *mut libc::c_int;
    if n < 0 as libc::c_int {
        ret = 1 as libc::c_int;
    } else if nnz < 0 as libc::c_int {
        ret = 2 as libc::c_int;
    } else {
        flag = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        i = 1 as libc::c_int;
        while i <= n {
            *flag.offset(i as isize) = 0 as libc::c_int;
            i += 1;
            i;
        }
        t = 1 as libc::c_int;
        loop {
            if !(t <= nnz) {
                current_block = 17407779659766490442;
                break;
            }
            i = *ind.offset(t as isize);
            if !(1 as libc::c_int <= i && i <= n) {
                ret = 3 as libc::c_int;
                current_block = 11358736504997421262;
                break;
            } else if *flag.offset(i as isize) != 0 {
                ret = 4 as libc::c_int;
                current_block = 11358736504997421262;
                break;
            } else {
                *flag.offset(i as isize) = 1 as libc::c_int;
                t += 1;
                t;
            }
        }
        match current_block {
            11358736504997421262 => {}
            _ => {
                i = 1 as libc::c_int;
                loop {
                    if !(i <= n) {
                        current_block = 13797916685926291137;
                        break;
                    }
                    if *flag.offset(i as isize) == 0 && *vec.offset(i as isize) != 0.0f64
                    {
                        ret = 5 as libc::c_int;
                        current_block = 11358736504997421262;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                match current_block {
                    11358736504997421262 => {}
                    _ => {
                        ret = 0 as libc::c_int;
                    }
                }
            }
        }
    }
    if !flag.is_null() {
        glp_free(flag as *mut libc::c_void);
    }
    return ret;
}
pub unsafe extern "C" fn _glp_mat_check_pattern(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut flag: *mut libc::c_int = 0 as *mut libc::c_int;
    if m < 0 as libc::c_int {
        ret = 1 as libc::c_int;
    } else if n < 0 as libc::c_int {
        ret = 2 as libc::c_int;
    } else if *A_ptr.offset(1 as libc::c_int as isize) != 1 as libc::c_int {
        ret = 3 as libc::c_int;
    } else {
        flag = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        j = 1 as libc::c_int;
        while j <= n {
            *flag.offset(j as isize) = 0 as libc::c_int;
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        's_51: loop {
            if !(i <= m) {
                current_block = 10043043949733653460;
                break;
            }
            ptr = *A_ptr.offset(i as isize);
            while ptr < *A_ptr.offset((i + 1 as libc::c_int) as isize) {
                j = *A_ind.offset(ptr as isize);
                if !(1 as libc::c_int <= j && j <= n) {
                    ret = 4 as libc::c_int;
                    current_block = 13619538307002873504;
                    break 's_51;
                } else if *flag.offset(j as isize) != 0 {
                    ret = 5 as libc::c_int;
                    current_block = 13619538307002873504;
                    break 's_51;
                } else {
                    *flag.offset(j as isize) = 1 as libc::c_int;
                    ptr += 1;
                    ptr;
                }
            }
            ptr = *A_ptr.offset(i as isize);
            while ptr < *A_ptr.offset((i + 1 as libc::c_int) as isize) {
                j = *A_ind.offset(ptr as isize);
                *flag.offset(j as isize) = 0 as libc::c_int;
                ptr += 1;
                ptr;
            }
            i += 1;
            i;
        }
        match current_block {
            13619538307002873504 => {}
            _ => {
                ret = 0 as libc::c_int;
            }
        }
    }
    if !flag.is_null() {
        glp_free(flag as *mut libc::c_void);
    }
    return ret;
}
pub unsafe extern "C" fn _glp_mat_transpose(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut A_val: *mut libc::c_double,
    mut AT_ptr: *mut libc::c_int,
    mut AT_ind: *mut libc::c_int,
    mut AT_val: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= n {
        *AT_ptr.offset(j as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= m {
        beg = *A_ptr.offset(i as isize);
        end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            let ref mut fresh0 = *AT_ptr.offset(*A_ind.offset(t as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            t += 1;
            t;
        }
        i += 1;
        i;
    }
    pos = 1 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        len = *AT_ptr.offset(j as isize);
        pos += len;
        *AT_ptr.offset(j as isize) = pos;
        j += 1;
        j;
    }
    *AT_ptr.offset((n + 1 as libc::c_int) as isize) = pos;
    i = m;
    while i >= 1 as libc::c_int {
        beg = *A_ptr.offset(i as isize);
        end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            let ref mut fresh1 = *AT_ptr.offset(*A_ind.offset(t as isize) as isize);
            *fresh1 -= 1;
            pos = *fresh1;
            *AT_ind.offset(pos as isize) = i;
            if !A_val.is_null() {
                *AT_val.offset(pos as isize) = *A_val.offset(t as isize);
            }
            t += 1;
            t;
        }
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_mat_adat_symbolic(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut P_per: *mut libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut S_ptr: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut tt: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut S_ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut AT_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut AT_ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut temp: *mut libc::c_int = 0 as *mut libc::c_int;
    AT_ptr = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    AT_ind = glp_alloc(
        *A_ptr.offset((m + 1 as libc::c_int) as isize),
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    _glp_mat_transpose(
        m,
        n,
        A_ptr,
        A_ind,
        0 as *mut libc::c_double,
        AT_ptr,
        AT_ind,
        0 as *mut libc::c_double,
    );
    size = *A_ptr.offset((m + 1 as libc::c_int) as isize) - 1 as libc::c_int;
    if size < m {
        size = m;
    }
    S_ind = glp_alloc(
        1 as libc::c_int + size,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    map = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    jj = 1 as libc::c_int;
    while jj <= m {
        *map.offset(jj as isize) = 0 as libc::c_int;
        jj += 1;
        jj;
    }
    *S_ptr.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    ii = 1 as libc::c_int;
    while ii <= m {
        len = 0 as libc::c_int;
        i = *P_per.offset(ii as isize);
        t = *A_ptr.offset(i as isize);
        while t < *A_ptr.offset((i + 1 as libc::c_int) as isize) {
            k = *A_ind.offset(t as isize);
            tt = *AT_ptr.offset(k as isize);
            while tt < *AT_ptr.offset((k + 1 as libc::c_int) as isize) {
                j = *AT_ind.offset(tt as isize);
                jj = *P_per.offset((m + j) as isize);
                if ii < jj && *map.offset(jj as isize) == 0 {
                    len += 1;
                    *ind.offset(len as isize) = jj;
                    *map.offset(jj as isize) = 1 as libc::c_int;
                }
                tt += 1;
                tt;
            }
            t += 1;
            t;
        }
        *S_ptr
            .offset((ii + 1 as libc::c_int) as isize) = *S_ptr.offset(ii as isize) + len;
        if *S_ptr.offset((ii + 1 as libc::c_int) as isize) - 1 as libc::c_int > size {
            temp = S_ind;
            size += size;
            S_ind = glp_alloc(
                1 as libc::c_int + size,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            memcpy(
                &mut *S_ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                &mut *temp.offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as *const libc::c_void,
                ((*S_ptr.offset(ii as isize) - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            glp_free(temp as *mut libc::c_void);
        }
        (*S_ptr.offset((ii + 1 as libc::c_int) as isize) - 1 as libc::c_int <= size
            || {
                glp_assert_(
                    b"S_ptr[ii+1] - 1 <= size\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    298 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        memcpy(
            &mut *S_ind.offset(*S_ptr.offset(ii as isize) as isize) as *mut libc::c_int
                as *mut libc::c_void,
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        t = 1 as libc::c_int;
        while t <= len {
            *map.offset(*ind.offset(t as isize) as isize) = 0 as libc::c_int;
            t += 1;
            t;
        }
        ii += 1;
        ii;
    }
    glp_free(AT_ptr as *mut libc::c_void);
    glp_free(AT_ind as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(map as *mut libc::c_void);
    temp = S_ind;
    size = *S_ptr.offset((m + 1 as libc::c_int) as isize) - 1 as libc::c_int;
    S_ind = glp_alloc(
        1 as libc::c_int + size,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memcpy(
        &mut *S_ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *temp.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_void,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    glp_free(temp as *mut libc::c_void);
    return S_ind;
}
pub unsafe extern "C" fn _glp_mat_adat_numeric(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut P_per: *mut libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut A_val: *mut libc::c_double,
    mut D_diag: *mut libc::c_double,
    mut S_ptr: *mut libc::c_int,
    mut S_ind: *mut libc::c_int,
    mut S_val: *mut libc::c_double,
    mut S_diag: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut tt: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut beg1: libc::c_int = 0;
    let mut end1: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    work = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    ii = 1 as libc::c_int;
    while ii <= m {
        i = *P_per.offset(ii as isize);
        beg = *A_ptr.offset(i as isize);
        end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            *work.offset(*A_ind.offset(t as isize) as isize) = *A_val.offset(t as isize);
            t += 1;
            t;
        }
        beg = *S_ptr.offset(ii as isize);
        end = *S_ptr.offset((ii + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            jj = *S_ind.offset(t as isize);
            j = *P_per.offset(jj as isize);
            sum = 0.0f64;
            beg1 = *A_ptr.offset(j as isize);
            end1 = *A_ptr.offset((j + 1 as libc::c_int) as isize);
            tt = beg1;
            while tt < end1 {
                k = *A_ind.offset(tt as isize);
                sum
                    += *work.offset(k as isize) * *D_diag.offset(k as isize)
                        * *A_val.offset(tt as isize);
                tt += 1;
                tt;
            }
            *S_val.offset(t as isize) = sum;
            t += 1;
            t;
        }
        sum = 0.0f64;
        beg = *A_ptr.offset(i as isize);
        end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            k = *A_ind.offset(t as isize);
            sum
                += *A_val.offset(t as isize) * *D_diag.offset(k as isize)
                    * *A_val.offset(t as isize);
            *work.offset(k as isize) = 0.0f64;
            t += 1;
            t;
        }
        *S_diag.offset(ii as isize) = sum;
        ii += 1;
        ii;
    }
    glp_free(work as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_mat_min_degree(
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut P_per: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ne: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut xadj: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut adjncy: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut deg: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut marker: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rchset: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nbrhd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut qsize: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut qlink: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nofsub: libc::c_int = 0;
    ne = *A_ptr.offset((n + 1 as libc::c_int) as isize) - 1 as libc::c_int;
    ne += ne;
    xadj = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    adjncy = glp_alloc(
        1 as libc::c_int + ne,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    deg = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    marker = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    rchset = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    nbrhd = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    qsize = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    qlink = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        *xadj.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= n {
        t = *A_ptr.offset(i as isize);
        while t < *A_ptr.offset((i + 1 as libc::c_int) as isize) {
            j = *A_ind.offset(t as isize);
            (i < j && j <= n
                || {
                    glp_assert_(
                        b"i < j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                        451 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            let ref mut fresh2 = *xadj.offset(i as isize);
            *fresh2 += 1;
            *fresh2;
            let ref mut fresh3 = *xadj.offset(j as isize);
            *fresh3 += 1;
            *fresh3;
            t += 1;
            t;
        }
        i += 1;
        i;
    }
    pos = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        len = *xadj.offset(i as isize);
        pos += len;
        *xadj.offset(i as isize) = pos;
        i += 1;
        i;
    }
    *xadj.offset((n + 1 as libc::c_int) as isize) = pos;
    (pos - 1 as libc::c_int == ne
        || {
            glp_assert_(
                b"pos - 1 == ne\0" as *const u8 as *const libc::c_char,
                b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                460 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        t = *A_ptr.offset(i as isize);
        while t < *A_ptr.offset((i + 1 as libc::c_int) as isize) {
            j = *A_ind.offset(t as isize);
            let ref mut fresh4 = *xadj.offset(i as isize);
            *fresh4 -= 1;
            *adjncy.offset(*fresh4 as isize) = j;
            let ref mut fresh5 = *xadj.offset(j as isize);
            *fresh5 -= 1;
            *adjncy.offset(*fresh5 as isize) = i;
            t += 1;
            t;
        }
        i += 1;
        i;
    }
    _glp_genqmd(
        &mut n,
        xadj,
        adjncy,
        P_per,
        P_per.offset(n as isize),
        deg,
        marker,
        rchset,
        nbrhd,
        qsize,
        qlink,
        &mut nofsub,
    );
    i = 1 as libc::c_int;
    while i <= n {
        j = *P_per.offset(i as isize);
        (1 as libc::c_int <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    474 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*P_per.offset((n + j) as isize) == i
            || {
                glp_assert_(
                    b"P_per[n+j] == i\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
    glp_free(xadj as *mut libc::c_void);
    glp_free(adjncy as *mut libc::c_void);
    glp_free(deg as *mut libc::c_void);
    glp_free(marker as *mut libc::c_void);
    glp_free(rchset as *mut libc::c_void);
    glp_free(nbrhd as *mut libc::c_void);
    glp_free(qsize as *mut libc::c_void);
    glp_free(qlink as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_mat_amd_order1(
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut P_per: *mut libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut Control: [libc::c_double; 5] = [0.; 5];
    let mut Info: [libc::c_double; 20] = [0.; 20];
    _glp_amd_defaults(Control.as_mut_ptr());
    k = 1 as libc::c_int;
    while k < *A_ptr.offset((n + 1 as libc::c_int) as isize) {
        let ref mut fresh6 = *A_ind.offset(k as isize);
        *fresh6 -= 1;
        *fresh6;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k <= n + 1 as libc::c_int {
        let ref mut fresh7 = *A_ptr.offset(k as isize);
        *fresh7 -= 1;
        *fresh7;
        k += 1;
        k;
    }
    ret = _glp_amd_order(
        n,
        &mut *A_ptr.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_int,
        &mut *A_ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_int,
        &mut *P_per.offset(1 as libc::c_int as isize),
        Control.as_mut_ptr(),
        Info.as_mut_ptr(),
    );
    (ret == 0 as libc::c_int || ret == 1 as libc::c_int
        || {
            glp_assert_(
                b"ret == AMD_OK || ret == AMD_OK_BUT_JUMBLED\0" as *const u8
                    as *const libc::c_char,
                b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                510 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n + 1 as libc::c_int {
        let ref mut fresh8 = *A_ptr.offset(k as isize);
        *fresh8 += 1;
        *fresh8;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k < *A_ptr.offset((n + 1 as libc::c_int) as isize) {
        let ref mut fresh9 = *A_ind.offset(k as isize);
        *fresh9 += 1;
        *fresh9;
        k += 1;
        k;
    }
    memset(
        &mut *P_per.offset((n + 1 as libc::c_int) as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    k = 1 as libc::c_int;
    while k <= n {
        let ref mut fresh10 = *P_per.offset(k as isize);
        *fresh10 += 1;
        *fresh10;
        (1 as libc::c_int <= *P_per.offset(k as isize) && *P_per.offset(k as isize) <= n
            || {
                glp_assert_(
                    b"1 <= P_per[k] && P_per[k] <= n\0" as *const u8
                        as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    518 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*P_per.offset((n + *P_per.offset(k as isize)) as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"P_per[n+P_per[k]] == 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    519 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *P_per.offset((n + *P_per.offset(k as isize)) as isize) = k;
        k += 1;
        k;
    }
}
unsafe extern "C" fn allocate(mut n: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = glp_alloc(n as libc::c_int, size as libc::c_int);
    memset(ptr, 0 as libc::c_int, n.wrapping_mul(size));
    return ptr;
}
unsafe extern "C" fn release(mut ptr: *mut libc::c_void) {
    glp_free(ptr);
}
pub unsafe extern "C" fn _glp_mat_symamd_ord(
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut P_per: *mut libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    let mut stats: [libc::c_int; 20] = [0; 20];
    k = 1 as libc::c_int;
    while k < *A_ptr.offset((n + 1 as libc::c_int) as isize) {
        let ref mut fresh11 = *A_ind.offset(k as isize);
        *fresh11 -= 1;
        *fresh11;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k <= n + 1 as libc::c_int {
        let ref mut fresh12 = *A_ptr.offset(k as isize);
        *fresh12 -= 1;
        *fresh12;
        k += 1;
        k;
    }
    ok = _glp_symamd(
        n,
        &mut *A_ind.offset(1 as libc::c_int as isize),
        &mut *A_ptr.offset(1 as libc::c_int as isize),
        &mut *P_per.offset(1 as libc::c_int as isize),
        0 as *mut libc::c_double,
        stats.as_mut_ptr(),
        Some(allocate as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(release as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    (ok != 0
        || {
            glp_assert_(
                b"ok\0" as *const u8 as *const libc::c_char,
                b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                552 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n + 1 as libc::c_int {
        let ref mut fresh13 = *A_ptr.offset(k as isize);
        *fresh13 += 1;
        *fresh13;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k < *A_ptr.offset((n + 1 as libc::c_int) as isize) {
        let ref mut fresh14 = *A_ind.offset(k as isize);
        *fresh14 += 1;
        *fresh14;
        k += 1;
        k;
    }
    memset(
        &mut *P_per.offset((n + 1 as libc::c_int) as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    k = 1 as libc::c_int;
    while k <= n {
        let ref mut fresh15 = *P_per.offset(k as isize);
        *fresh15 += 1;
        *fresh15;
        (1 as libc::c_int <= *P_per.offset(k as isize) && *P_per.offset(k as isize) <= n
            || {
                glp_assert_(
                    b"1 <= P_per[k] && P_per[k] <= n\0" as *const u8
                        as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    560 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*P_per.offset((n + *P_per.offset(k as isize)) as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"P_per[n+P_per[k]] == 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    561 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *P_per.offset((n + *P_per.offset(k as isize)) as isize) = k;
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_mat_chol_symbolic(
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut U_ptr: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut min_j: libc::c_int = 0;
    let mut U_ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut head: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut next: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut temp: *mut libc::c_int = 0 as *mut libc::c_int;
    size = *A_ptr.offset((n + 1 as libc::c_int) as isize) - 1 as libc::c_int;
    if size < n {
        size = n;
    }
    size += size;
    U_ind = glp_alloc(
        1 as libc::c_int + size,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    head = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        *head.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    next = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    map = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        *map.offset(j as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    *U_ptr.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n {
        len = *A_ptr.offset((k + 1 as libc::c_int) as isize) - *A_ptr.offset(k as isize);
        memcpy(
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *mut libc::c_void,
            &mut *A_ind.offset(*A_ptr.offset(k as isize) as isize) as *mut libc::c_int
                as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        t = 1 as libc::c_int;
        while t <= len {
            j = *ind.offset(t as isize);
            (k < j && j <= n
                || {
                    glp_assert_(
                        b"k < j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                        658 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *map.offset(j as isize) = 1 as libc::c_int;
            t += 1;
            t;
        }
        i = *head.offset(k as isize);
        while i != 0 as libc::c_int {
            beg = *U_ptr.offset(i as isize);
            end = *U_ptr.offset((i + 1 as libc::c_int) as isize);
            t = beg;
            while t < end {
                j = *U_ind.offset(t as isize);
                if j > k && *map.offset(j as isize) == 0 {
                    len += 1;
                    *ind.offset(len as isize) = j;
                    *map.offset(j as isize) = 1 as libc::c_int;
                }
                t += 1;
                t;
            }
            i = *next.offset(i as isize);
        }
        *U_ptr.offset((k + 1 as libc::c_int) as isize) = *U_ptr.offset(k as isize) + len;
        if *U_ptr.offset((k + 1 as libc::c_int) as isize) - 1 as libc::c_int > size {
            temp = U_ind;
            size += size;
            U_ind = glp_alloc(
                1 as libc::c_int + size,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            memcpy(
                &mut *U_ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                &mut *temp.offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as *const libc::c_void,
                ((*U_ptr.offset(k as isize) - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            glp_free(temp as *mut libc::c_void);
        }
        (*U_ptr.offset((k + 1 as libc::c_int) as isize) - 1 as libc::c_int <= size
            || {
                glp_assert_(
                    b"U_ptr[k+1] - 1 <= size\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    682 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        memcpy(
            &mut *U_ind.offset(*U_ptr.offset(k as isize) as isize) as *mut libc::c_int
                as *mut libc::c_void,
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        min_j = n + 1 as libc::c_int;
        t = 1 as libc::c_int;
        while t <= len {
            j = *ind.offset(t as isize);
            *map.offset(j as isize) = 0 as libc::c_int;
            if min_j > j {
                min_j = j;
            }
            t += 1;
            t;
        }
        if min_j <= n {
            *next.offset(k as isize) = *head.offset(min_j as isize);
            *head.offset(min_j as isize) = k;
        }
        k += 1;
        k;
    }
    glp_free(head as *mut libc::c_void);
    glp_free(next as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(map as *mut libc::c_void);
    temp = U_ind;
    size = *U_ptr.offset((n + 1 as libc::c_int) as isize) - 1 as libc::c_int;
    U_ind = glp_alloc(
        1 as libc::c_int + size,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memcpy(
        &mut *U_ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *temp.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_void,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    glp_free(temp as *mut libc::c_void);
    return U_ind;
}
pub unsafe extern "C" fn _glp_mat_chol_numeric(
    mut n: libc::c_int,
    mut A_ptr: *mut libc::c_int,
    mut A_ind: *mut libc::c_int,
    mut A_val: *mut libc::c_double,
    mut A_diag: *mut libc::c_double,
    mut U_ptr: *mut libc::c_int,
    mut U_ind: *mut libc::c_int,
    mut U_val: *mut libc::c_double,
    mut U_diag: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut beg1: libc::c_int = 0;
    let mut end1: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ukk: libc::c_double = 0.;
    let mut uki: libc::c_double = 0.;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    work = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= n {
        beg = *A_ptr.offset(i as isize);
        end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            j = *A_ind.offset(t as isize);
            *work.offset(j as isize) = *A_val.offset(t as isize);
            t += 1;
            t;
        }
        beg = *U_ptr.offset(i as isize);
        end = *U_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            j = *U_ind.offset(t as isize);
            *U_val.offset(t as isize) = *work.offset(j as isize);
            *work.offset(j as isize) = 0.0f64;
            t += 1;
            t;
        }
        *U_diag.offset(i as isize) = *A_diag.offset(i as isize);
        i += 1;
        i;
    }
    k = 1 as libc::c_int;
    while k <= n {
        ukk = *U_diag.offset(k as isize);
        if ukk > 0.0f64 {
            ukk = sqrt(ukk);
            *U_diag.offset(k as isize) = ukk;
        } else {
            ukk = 1.7976931348623157e+308f64;
            *U_diag.offset(k as isize) = ukk;
            count += 1;
            count;
        }
        beg = *U_ptr.offset(k as isize);
        end = *U_ptr.offset((k + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            let ref mut fresh16 = *U_val.offset(t as isize);
            *fresh16 /= ukk;
            *work.offset(*U_ind.offset(t as isize) as isize) = *fresh16;
            t += 1;
            t;
        }
        t = beg;
        while t < end {
            i = *U_ind.offset(t as isize);
            (i > k
                || {
                    glp_assert_(
                        b"i > k\0" as *const u8 as *const libc::c_char,
                        b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                        821 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            uki = *work.offset(i as isize);
            beg1 = *U_ptr.offset(i as isize);
            end1 = *U_ptr.offset((i + 1 as libc::c_int) as isize);
            t1 = beg1;
            while t1 < end1 {
                *U_val.offset(t1 as isize)
                    -= uki * *work.offset(*U_ind.offset(t1 as isize) as isize);
                t1 += 1;
                t1;
            }
            *U_diag.offset(i as isize) -= uki * uki;
            t += 1;
            t;
        }
        t = beg;
        while t < end {
            *work.offset(*U_ind.offset(t as isize) as isize) = 0.0f64;
            t += 1;
            t;
        }
        k += 1;
        k;
    }
    glp_free(work as *mut libc::c_void);
    return count;
}
pub unsafe extern "C" fn _glp_mat_u_solve(
    mut n: libc::c_int,
    mut U_ptr: *mut libc::c_int,
    mut U_ind: *mut libc::c_int,
    mut U_val: *mut libc::c_double,
    mut U_diag: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    i = n;
    while i >= 1 as libc::c_int {
        temp = *x.offset(i as isize);
        beg = *U_ptr.offset(i as isize);
        end = *U_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            temp
                -= *U_val.offset(t as isize)
                    * *x.offset(*U_ind.offset(t as isize) as isize);
            t += 1;
            t;
        }
        (*U_diag.offset(i as isize) != 0.0f64
            || {
                glp_assert_(
                    b"U_diag[i] != 0.0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    873 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *x.offset(i as isize) = temp / *U_diag.offset(i as isize);
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_mat_ut_solve(
    mut n: libc::c_int,
    mut U_ptr: *mut libc::c_int,
    mut U_ind: *mut libc::c_int,
    mut U_val: *mut libc::c_double,
    mut U_diag: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i <= n {
        (*U_diag.offset(i as isize) != 0.0f64
            || {
                glp_assert_(
                    b"U_diag[i] != 0.0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpmat.c\0" as *const u8 as *const libc::c_char,
                    911 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        let ref mut fresh17 = *x.offset(i as isize);
        *fresh17 /= *U_diag.offset(i as isize);
        temp = *fresh17;
        if !(temp == 0.0f64) {
            beg = *U_ptr.offset(i as isize);
            end = *U_ptr.offset((i + 1 as libc::c_int) as isize);
            t = beg;
            while t < end {
                *x.offset(*U_ind.offset(t as isize) as isize)
                    -= *U_val.offset(t as isize) * temp;
                t += 1;
                t;
            }
        }
        i += 1;
        i;
    }
}

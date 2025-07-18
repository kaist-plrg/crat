use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type Colamd_Row = Colamd_Row_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colamd_Row_struct {
    pub start: libc::c_int,
    pub length: libc::c_int,
    pub shared1: C2RustUnnamed_0,
    pub shared2: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub mark: libc::c_int,
    pub first_column: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub degree: libc::c_int,
    pub p: libc::c_int,
}
pub type Colamd_Col = Colamd_Col_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colamd_Col_struct {
    pub start: libc::c_int,
    pub length: libc::c_int,
    pub shared1: C2RustUnnamed_4,
    pub shared2: C2RustUnnamed_3,
    pub shared3: C2RustUnnamed_2,
    pub shared4: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub degree_next: libc::c_int,
    pub hash_next: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub headhash: libc::c_int,
    pub hash: libc::c_int,
    pub prev: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub score: libc::c_int,
    pub order: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub thickness: libc::c_int,
    pub parent: libc::c_int,
}
unsafe extern "C" fn t_add(
    mut a: size_t,
    mut b: size_t,
    mut ok: *mut libc::c_int,
) -> size_t {
    *ok = (*ok != 0 && a.wrapping_add(b) >= (if a > b { a } else { b })) as libc::c_int;
    return if *ok != 0 { a.wrapping_add(b) } else { 0 as libc::c_int as libc::c_ulong };
}
unsafe extern "C" fn t_mult(
    mut a: size_t,
    mut k: size_t,
    mut ok: *mut libc::c_int,
) -> size_t {
    let mut i: size_t = 0;
    let mut s: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < k {
        s = t_add(s, a, ok);
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
pub unsafe extern "C" fn _glp_colamd_recommended(
    mut nnz: libc::c_int,
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
) -> size_t {
    let mut s: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut ok: libc::c_int = 1 as libc::c_int;
    if nnz < 0 as libc::c_int || n_row < 0 as libc::c_int || n_col < 0 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    s = t_mult(nnz as size_t, 2 as libc::c_int as size_t, &mut ok);
    c = (t_mult(
        t_add(n_col as size_t, 1 as libc::c_int as size_t, &mut ok),
        ::std::mem::size_of::<Colamd_Col>() as libc::c_ulong,
        &mut ok,
    ))
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    r = (t_mult(
        t_add(n_row as size_t, 1 as libc::c_int as size_t, &mut ok),
        ::std::mem::size_of::<Colamd_Row>() as libc::c_ulong,
        &mut ok,
    ))
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    s = t_add(s, c, &mut ok);
    s = t_add(s, r, &mut ok);
    s = t_add(s, n_col as size_t, &mut ok);
    s = t_add(s, (nnz / 5 as libc::c_int) as size_t, &mut ok);
    ok = (ok != 0 && s < 2147483647 as libc::c_int as libc::c_ulong) as libc::c_int;
    return if ok != 0 { s } else { 0 as libc::c_int as libc::c_ulong };
}
pub unsafe extern "C" fn _glp_colamd_set_defaults(mut knobs: *mut libc::c_double) {
    let mut i: libc::c_int = 0;
    if knobs.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        *knobs.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    *knobs.offset(0 as libc::c_int as isize) = 10 as libc::c_int as libc::c_double;
    *knobs.offset(1 as libc::c_int as isize) = 10 as libc::c_int as libc::c_double;
    *knobs.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
}
pub unsafe extern "C" fn _glp_symamd(
    mut n: libc::c_int,
    mut A: *mut libc::c_int,
    mut p: *mut libc::c_int,
    mut perm: *mut libc::c_int,
    mut knobs: *mut libc::c_double,
    mut stats: *mut libc::c_int,
    mut allocate: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    mut release: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut count: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mark: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut M: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Mlen: size_t = 0;
    let mut n_row: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mnz: libc::c_int = 0;
    let mut pp: libc::c_int = 0;
    let mut last_row: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut cknobs: [libc::c_double; 20] = [0.; 20];
    let mut default_knobs: [libc::c_double; 20] = [0.; 20];
    if stats.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        *stats.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    *stats.offset(3 as libc::c_int as isize) = 0 as libc::c_int;
    *stats.offset(4 as libc::c_int as isize) = -(1 as libc::c_int);
    *stats.offset(5 as libc::c_int as isize) = -(1 as libc::c_int);
    if A.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(1 as libc::c_int);
        return 0 as libc::c_int;
    }
    if p.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(2 as libc::c_int);
        return 0 as libc::c_int;
    }
    if n < 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(4 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = n;
        return 0 as libc::c_int;
    }
    nnz = *p.offset(n as isize);
    if nnz < 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(5 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = nnz;
        return 0 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) != 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(6 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = *p.offset(0 as libc::c_int as isize);
        return 0 as libc::c_int;
    }
    if knobs.is_null() {
        _glp_colamd_set_defaults(default_knobs.as_mut_ptr());
        knobs = default_knobs.as_mut_ptr();
    }
    count = (Some(allocate.unwrap()))
        .unwrap()(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if count.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(10 as libc::c_int);
        return 0 as libc::c_int;
    }
    mark = (Some(allocate.unwrap()))
        .unwrap()(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if mark.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(10 as libc::c_int);
        (Some(release.unwrap())).unwrap()(count as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    *stats.offset(6 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *mark.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < n {
        last_row = -(1 as libc::c_int);
        length = *p.offset((j + 1 as libc::c_int) as isize) - *p.offset(j as isize);
        if length < 0 as libc::c_int {
            *stats.offset(3 as libc::c_int as isize) = -(8 as libc::c_int);
            *stats.offset(4 as libc::c_int as isize) = j;
            *stats.offset(5 as libc::c_int as isize) = length;
            (Some(release.unwrap())).unwrap()(count as *mut libc::c_void);
            (Some(release.unwrap())).unwrap()(mark as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        pp = *p.offset(j as isize);
        while pp < *p.offset((j + 1 as libc::c_int) as isize) {
            i = *A.offset(pp as isize);
            if i < 0 as libc::c_int || i >= n {
                *stats.offset(3 as libc::c_int as isize) = -(9 as libc::c_int);
                *stats.offset(4 as libc::c_int as isize) = j;
                *stats.offset(5 as libc::c_int as isize) = i;
                *stats.offset(6 as libc::c_int as isize) = n;
                (Some(release.unwrap())).unwrap()(count as *mut libc::c_void);
                (Some(release.unwrap())).unwrap()(mark as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            if i <= last_row || *mark.offset(i as isize) == j {
                *stats.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
                *stats.offset(4 as libc::c_int as isize) = j;
                *stats.offset(5 as libc::c_int as isize) = i;
                let ref mut fresh0 = *stats.offset(6 as libc::c_int as isize);
                *fresh0 += 1;
                *fresh0;
            }
            if i > j && *mark.offset(i as isize) != j {
                let ref mut fresh1 = *count.offset(i as isize);
                *fresh1 += 1;
                *fresh1;
                let ref mut fresh2 = *count.offset(j as isize);
                *fresh2 += 1;
                *fresh2;
            }
            *mark.offset(i as isize) = j;
            last_row = i;
            pp += 1;
            pp;
        }
        j += 1;
        j;
    }
    *perm.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        *perm
            .offset(
                j as isize,
            ) = *perm.offset((j - 1 as libc::c_int) as isize)
            + *count.offset((j - 1 as libc::c_int) as isize);
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < n {
        *count.offset(j as isize) = *perm.offset(j as isize);
        j += 1;
        j;
    }
    mnz = *perm.offset(n as isize);
    n_row = mnz / 2 as libc::c_int;
    Mlen = _glp_colamd_recommended(mnz, n_row, n);
    M = (Some(allocate.unwrap()))
        .unwrap()(Mlen, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    if M.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(10 as libc::c_int);
        (Some(release.unwrap())).unwrap()(count as *mut libc::c_void);
        (Some(release.unwrap())).unwrap()(mark as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    if *stats.offset(3 as libc::c_int as isize) == 0 as libc::c_int {
        j = 0 as libc::c_int;
        while j < n {
            pp = *p.offset(j as isize);
            while pp < *p.offset((j + 1 as libc::c_int) as isize) {
                i = *A.offset(pp as isize);
                if i > j {
                    let ref mut fresh3 = *count.offset(i as isize);
                    let fresh4 = *fresh3;
                    *fresh3 = *fresh3 + 1;
                    *M.offset(fresh4 as isize) = k;
                    let ref mut fresh5 = *count.offset(j as isize);
                    let fresh6 = *fresh5;
                    *fresh5 = *fresh5 + 1;
                    *M.offset(fresh6 as isize) = k;
                    k += 1;
                    k;
                }
                pp += 1;
                pp;
            }
            j += 1;
            j;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            *mark.offset(i as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        j = 0 as libc::c_int;
        while j < n {
            pp = *p.offset(j as isize);
            while pp < *p.offset((j + 1 as libc::c_int) as isize) {
                i = *A.offset(pp as isize);
                if i > j && *mark.offset(i as isize) != j {
                    let ref mut fresh7 = *count.offset(i as isize);
                    let fresh8 = *fresh7;
                    *fresh7 = *fresh7 + 1;
                    *M.offset(fresh8 as isize) = k;
                    let ref mut fresh9 = *count.offset(j as isize);
                    let fresh10 = *fresh9;
                    *fresh9 = *fresh9 + 1;
                    *M.offset(fresh10 as isize) = k;
                    k += 1;
                    k;
                    *mark.offset(i as isize) = j;
                }
                pp += 1;
                pp;
            }
            j += 1;
            j;
        }
    }
    (Some(release.unwrap())).unwrap()(count as *mut libc::c_void);
    (Some(release.unwrap())).unwrap()(mark as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        cknobs[i as usize] = *knobs.offset(i as isize);
        i += 1;
        i;
    }
    cknobs[0 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_double;
    cknobs[1 as libc::c_int as usize] = *knobs.offset(0 as libc::c_int as isize);
    _glp_colamd(n_row, n, Mlen as libc::c_int, M, perm, cknobs.as_mut_ptr(), stats);
    *stats.offset(0 as libc::c_int as isize) = *stats.offset(1 as libc::c_int as isize);
    (Some(release.unwrap())).unwrap()(M as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_colamd(
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
    mut Alen: libc::c_int,
    mut A: *mut libc::c_int,
    mut p: *mut libc::c_int,
    mut knobs: *mut libc::c_double,
    mut stats: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut Row_size: size_t = 0;
    let mut Col_size: size_t = 0;
    let mut need: size_t = 0;
    let mut Row: *mut Colamd_Row = 0 as *mut Colamd_Row;
    let mut Col: *mut Colamd_Col = 0 as *mut Colamd_Col;
    let mut n_col2: libc::c_int = 0;
    let mut n_row2: libc::c_int = 0;
    let mut ngarbage: libc::c_int = 0;
    let mut max_deg: libc::c_int = 0;
    let mut default_knobs: [libc::c_double; 20] = [0.; 20];
    let mut aggressive: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    if stats.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        *stats.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    *stats.offset(3 as libc::c_int as isize) = 0 as libc::c_int;
    *stats.offset(4 as libc::c_int as isize) = -(1 as libc::c_int);
    *stats.offset(5 as libc::c_int as isize) = -(1 as libc::c_int);
    if A.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(1 as libc::c_int);
        return 0 as libc::c_int;
    }
    if p.is_null() {
        *stats.offset(3 as libc::c_int as isize) = -(2 as libc::c_int);
        return 0 as libc::c_int;
    }
    if n_row < 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(3 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = n_row;
        return 0 as libc::c_int;
    }
    if n_col < 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(4 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = n_col;
        return 0 as libc::c_int;
    }
    nnz = *p.offset(n_col as isize);
    if nnz < 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(5 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = nnz;
        return 0 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) != 0 as libc::c_int {
        *stats.offset(3 as libc::c_int as isize) = -(6 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = *p.offset(0 as libc::c_int as isize);
        return 0 as libc::c_int;
    }
    if knobs.is_null() {
        _glp_colamd_set_defaults(default_knobs.as_mut_ptr());
        knobs = default_knobs.as_mut_ptr();
    }
    aggressive = (*knobs.offset(2 as libc::c_int as isize)
        != 0 as libc::c_int as libc::c_double) as libc::c_int;
    ok = 1 as libc::c_int;
    Col_size = (t_mult(
        t_add(n_col as size_t, 1 as libc::c_int as size_t, &mut ok),
        ::std::mem::size_of::<Colamd_Col>() as libc::c_ulong,
        &mut ok,
    ))
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    Row_size = (t_mult(
        t_add(n_row as size_t, 1 as libc::c_int as size_t, &mut ok),
        ::std::mem::size_of::<Colamd_Row>() as libc::c_ulong,
        &mut ok,
    ))
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    need = t_mult(nnz as size_t, 2 as libc::c_int as size_t, &mut ok);
    need = t_add(need, n_col as size_t, &mut ok);
    need = t_add(need, Col_size, &mut ok);
    need = t_add(need, Row_size, &mut ok);
    if ok == 0 || need > Alen as size_t
        || need > 2147483647 as libc::c_int as libc::c_ulong
    {
        *stats.offset(3 as libc::c_int as isize) = -(7 as libc::c_int);
        *stats.offset(4 as libc::c_int as isize) = need as libc::c_int;
        *stats.offset(5 as libc::c_int as isize) = Alen;
        return 0 as libc::c_int;
    }
    Alen = (Alen as libc::c_ulong).wrapping_sub(Col_size.wrapping_add(Row_size))
        as libc::c_int as libc::c_int;
    Col = &mut *A.offset(Alen as isize) as *mut libc::c_int as *mut Colamd_Col;
    Row = &mut *A.offset((Alen as libc::c_ulong).wrapping_add(Col_size) as isize)
        as *mut libc::c_int as *mut Colamd_Row;
    if init_rows_cols(n_row, n_col, Row, Col, A, p, stats) == 0 {
        return 0 as libc::c_int;
    }
    init_scoring(
        n_row,
        n_col,
        Row,
        Col,
        A,
        p,
        knobs,
        &mut n_row2,
        &mut n_col2,
        &mut max_deg,
    );
    ngarbage = find_ordering(
        n_row,
        n_col,
        Alen,
        Row,
        Col,
        A,
        p,
        n_col2,
        max_deg,
        2 as libc::c_int * nnz,
        aggressive,
    );
    order_children(n_col, Col, p);
    *stats.offset(0 as libc::c_int as isize) = n_row - n_row2;
    *stats.offset(1 as libc::c_int as isize) = n_col - n_col2;
    *stats.offset(2 as libc::c_int as isize) = ngarbage;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_colamd_report(mut stats: *mut libc::c_int) {
    print_report(
        b"colamd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        stats,
    );
}
pub unsafe extern "C" fn _glp_symamd_report(mut stats: *mut libc::c_int) {
    print_report(
        b"symamd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        stats,
    );
}
unsafe extern "C" fn init_rows_cols(
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut libc::c_int,
    mut p: *mut libc::c_int,
    mut stats: *mut libc::c_int,
) -> libc::c_int {
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut cp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cp_end: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rp_end: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut last_row: libc::c_int = 0;
    col = 0 as libc::c_int;
    while col < n_col {
        (*Col.offset(col as isize)).start = *p.offset(col as isize);
        (*Col.offset(col as isize))
            .length = *p.offset((col + 1 as libc::c_int) as isize)
            - *p.offset(col as isize);
        if (*Col.offset(col as isize)).length < 0 as libc::c_int {
            *stats.offset(3 as libc::c_int as isize) = -(8 as libc::c_int);
            *stats.offset(4 as libc::c_int as isize) = col;
            *stats
                .offset(5 as libc::c_int as isize) = (*Col.offset(col as isize)).length;
            return 0 as libc::c_int;
        }
        (*Col.offset(col as isize)).shared1.thickness = 1 as libc::c_int;
        (*Col.offset(col as isize)).shared2.score = 0 as libc::c_int;
        (*Col.offset(col as isize)).shared3.prev = -(1 as libc::c_int);
        (*Col.offset(col as isize)).shared4.degree_next = -(1 as libc::c_int);
        col += 1;
        col;
    }
    *stats.offset(6 as libc::c_int as isize) = 0 as libc::c_int;
    row = 0 as libc::c_int;
    while row < n_row {
        (*Row.offset(row as isize)).length = 0 as libc::c_int;
        (*Row.offset(row as isize)).shared2.mark = -(1 as libc::c_int);
        row += 1;
        row;
    }
    col = 0 as libc::c_int;
    while col < n_col {
        last_row = -(1 as libc::c_int);
        cp = &mut *A.offset(*p.offset(col as isize) as isize) as *mut libc::c_int;
        cp_end = &mut *A.offset(*p.offset((col + 1 as libc::c_int) as isize) as isize)
            as *mut libc::c_int;
        while cp < cp_end {
            let fresh11 = cp;
            cp = cp.offset(1);
            row = *fresh11;
            if row < 0 as libc::c_int || row >= n_row {
                *stats.offset(3 as libc::c_int as isize) = -(9 as libc::c_int);
                *stats.offset(4 as libc::c_int as isize) = col;
                *stats.offset(5 as libc::c_int as isize) = row;
                *stats.offset(6 as libc::c_int as isize) = n_row;
                return 0 as libc::c_int;
            }
            if row <= last_row || (*Row.offset(row as isize)).shared2.mark == col {
                *stats.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
                *stats.offset(4 as libc::c_int as isize) = col;
                *stats.offset(5 as libc::c_int as isize) = row;
                let ref mut fresh12 = *stats.offset(6 as libc::c_int as isize);
                *fresh12 += 1;
                *fresh12;
            }
            if (*Row.offset(row as isize)).shared2.mark != col {
                let ref mut fresh13 = (*Row.offset(row as isize)).length;
                *fresh13 += 1;
                *fresh13;
            } else {
                let ref mut fresh14 = (*Col.offset(col as isize)).length;
                *fresh14 -= 1;
                *fresh14;
            }
            (*Row.offset(row as isize)).shared2.mark = col;
            last_row = row;
        }
        col += 1;
        col;
    }
    (*Row.offset(0 as libc::c_int as isize)).start = *p.offset(n_col as isize);
    (*Row.offset(0 as libc::c_int as isize))
        .shared1
        .p = (*Row.offset(0 as libc::c_int as isize)).start;
    (*Row.offset(0 as libc::c_int as isize)).shared2.mark = -(1 as libc::c_int);
    row = 1 as libc::c_int;
    while row < n_row {
        (*Row.offset(row as isize))
            .start = (*Row.offset((row - 1 as libc::c_int) as isize)).start
            + (*Row.offset((row - 1 as libc::c_int) as isize)).length;
        (*Row.offset(row as isize)).shared1.p = (*Row.offset(row as isize)).start;
        (*Row.offset(row as isize)).shared2.mark = -(1 as libc::c_int);
        row += 1;
        row;
    }
    if *stats.offset(3 as libc::c_int as isize) == 1 as libc::c_int {
        col = 0 as libc::c_int;
        while col < n_col {
            cp = &mut *A.offset(*p.offset(col as isize) as isize) as *mut libc::c_int;
            cp_end = &mut *A
                .offset(*p.offset((col + 1 as libc::c_int) as isize) as isize)
                as *mut libc::c_int;
            while cp < cp_end {
                let fresh15 = cp;
                cp = cp.offset(1);
                row = *fresh15;
                if (*Row.offset(row as isize)).shared2.mark != col {
                    let ref mut fresh16 = (*Row.offset(row as isize)).shared1.p;
                    let fresh17 = *fresh16;
                    *fresh16 = *fresh16 + 1;
                    *A.offset(fresh17 as isize) = col;
                    (*Row.offset(row as isize)).shared2.mark = col;
                }
            }
            col += 1;
            col;
        }
    } else {
        col = 0 as libc::c_int;
        while col < n_col {
            cp = &mut *A.offset(*p.offset(col as isize) as isize) as *mut libc::c_int;
            cp_end = &mut *A
                .offset(*p.offset((col + 1 as libc::c_int) as isize) as isize)
                as *mut libc::c_int;
            while cp < cp_end {
                let fresh18 = cp;
                cp = cp.offset(1);
                let ref mut fresh19 = (*Row.offset(*fresh18 as isize)).shared1.p;
                let fresh20 = *fresh19;
                *fresh19 = *fresh19 + 1;
                *A.offset(fresh20 as isize) = col;
            }
            col += 1;
            col;
        }
    }
    row = 0 as libc::c_int;
    while row < n_row {
        (*Row.offset(row as isize)).shared2.mark = 0 as libc::c_int;
        (*Row.offset(row as isize)).shared1.degree = (*Row.offset(row as isize)).length;
        row += 1;
        row;
    }
    if *stats.offset(3 as libc::c_int as isize) == 1 as libc::c_int {
        (*Col.offset(0 as libc::c_int as isize)).start = 0 as libc::c_int;
        *p
            .offset(
                0 as libc::c_int as isize,
            ) = (*Col.offset(0 as libc::c_int as isize)).start;
        col = 1 as libc::c_int;
        while col < n_col {
            (*Col.offset(col as isize))
                .start = (*Col.offset((col - 1 as libc::c_int) as isize)).start
                + (*Col.offset((col - 1 as libc::c_int) as isize)).length;
            *p.offset(col as isize) = (*Col.offset(col as isize)).start;
            col += 1;
            col;
        }
        row = 0 as libc::c_int;
        while row < n_row {
            rp = &mut *A.offset((*Row.offset(row as isize)).start as isize)
                as *mut libc::c_int;
            rp_end = rp.offset((*Row.offset(row as isize)).length as isize);
            while rp < rp_end {
                let fresh21 = rp;
                rp = rp.offset(1);
                let ref mut fresh22 = *p.offset(*fresh21 as isize);
                let fresh23 = *fresh22;
                *fresh22 = *fresh22 + 1;
                *A.offset(fresh23 as isize) = row;
            }
            row += 1;
            row;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn init_scoring(
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut libc::c_int,
    mut head: *mut libc::c_int,
    mut knobs: *mut libc::c_double,
    mut p_n_row2: *mut libc::c_int,
    mut p_n_col2: *mut libc::c_int,
    mut p_max_deg: *mut libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut cp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut deg: libc::c_int = 0;
    let mut cp_end: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut new_cp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut col_length: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut n_col2: libc::c_int = 0;
    let mut n_row2: libc::c_int = 0;
    let mut dense_row_count: libc::c_int = 0;
    let mut dense_col_count: libc::c_int = 0;
    let mut min_score: libc::c_int = 0;
    let mut max_deg: libc::c_int = 0;
    let mut next_col: libc::c_int = 0;
    if *knobs.offset(0 as libc::c_int as isize) < 0 as libc::c_int as libc::c_double {
        dense_row_count = n_col - 1 as libc::c_int;
    } else {
        dense_row_count = (if 16.0f64
            > *knobs.offset(0 as libc::c_int as isize) * sqrt(n_col as libc::c_double)
        {
            16.0f64
        } else {
            *knobs.offset(0 as libc::c_int as isize) * sqrt(n_col as libc::c_double)
        }) as libc::c_int;
    }
    if *knobs.offset(1 as libc::c_int as isize) < 0 as libc::c_int as libc::c_double {
        dense_col_count = n_row - 1 as libc::c_int;
    } else {
        dense_col_count = (if 16.0f64
            > *knobs.offset(1 as libc::c_int as isize)
                * sqrt((if n_row < n_col { n_row } else { n_col }) as libc::c_double)
        {
            16.0f64
        } else {
            *knobs.offset(1 as libc::c_int as isize)
                * sqrt((if n_row < n_col { n_row } else { n_col }) as libc::c_double)
        }) as libc::c_int;
    }
    max_deg = 0 as libc::c_int;
    n_col2 = n_col;
    n_row2 = n_row;
    c = n_col - 1 as libc::c_int;
    while c >= 0 as libc::c_int {
        deg = (*Col.offset(c as isize)).length;
        if deg == 0 as libc::c_int {
            n_col2 -= 1;
            (*Col.offset(c as isize)).shared2.order = n_col2;
            (*Col.offset(c as isize)).start = -(1 as libc::c_int);
        }
        c -= 1;
        c;
    }
    c = n_col - 1 as libc::c_int;
    while c >= 0 as libc::c_int {
        if !((*Col.offset(c as isize)).start < 0 as libc::c_int) {
            deg = (*Col.offset(c as isize)).length;
            if deg > dense_col_count {
                n_col2 -= 1;
                (*Col.offset(c as isize)).shared2.order = n_col2;
                cp = &mut *A.offset((*Col.offset(c as isize)).start as isize)
                    as *mut libc::c_int;
                cp_end = cp.offset((*Col.offset(c as isize)).length as isize);
                while cp < cp_end {
                    let fresh24 = cp;
                    cp = cp.offset(1);
                    let ref mut fresh25 = (*Row.offset(*fresh24 as isize))
                        .shared1
                        .degree;
                    *fresh25 -= 1;
                    *fresh25;
                }
                (*Col.offset(c as isize)).start = -(1 as libc::c_int);
            }
        }
        c -= 1;
        c;
    }
    r = 0 as libc::c_int;
    while r < n_row {
        deg = (*Row.offset(r as isize)).shared1.degree;
        if deg > dense_row_count || deg == 0 as libc::c_int {
            (*Row.offset(r as isize)).shared2.mark = -(1 as libc::c_int);
            n_row2 -= 1;
            n_row2;
        } else {
            max_deg = if max_deg > deg { max_deg } else { deg };
        }
        r += 1;
        r;
    }
    c = n_col - 1 as libc::c_int;
    while c >= 0 as libc::c_int {
        if !((*Col.offset(c as isize)).start < 0 as libc::c_int) {
            score = 0 as libc::c_int;
            cp = &mut *A.offset((*Col.offset(c as isize)).start as isize)
                as *mut libc::c_int;
            new_cp = cp;
            cp_end = cp.offset((*Col.offset(c as isize)).length as isize);
            while cp < cp_end {
                let fresh26 = cp;
                cp = cp.offset(1);
                row = *fresh26;
                if (*Row.offset(row as isize)).shared2.mark < 0 as libc::c_int {
                    continue;
                }
                let fresh27 = new_cp;
                new_cp = new_cp.offset(1);
                *fresh27 = row;
                score += (*Row.offset(row as isize)).shared1.degree - 1 as libc::c_int;
                score = if score < n_col { score } else { n_col };
            }
            col_length = new_cp
                .offset_from(
                    &mut *A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
            if col_length == 0 as libc::c_int {
                n_col2 -= 1;
                (*Col.offset(c as isize)).shared2.order = n_col2;
                (*Col.offset(c as isize)).start = -(1 as libc::c_int);
            } else {
                (*Col.offset(c as isize)).length = col_length;
                (*Col.offset(c as isize)).shared2.score = score;
            }
        }
        c -= 1;
        c;
    }
    c = 0 as libc::c_int;
    while c <= n_col {
        *head.offset(c as isize) = -(1 as libc::c_int);
        c += 1;
        c;
    }
    min_score = n_col;
    c = n_col - 1 as libc::c_int;
    while c >= 0 as libc::c_int {
        if (*Col.offset(c as isize)).start >= 0 as libc::c_int {
            score = (*Col.offset(c as isize)).shared2.score;
            next_col = *head.offset(score as isize);
            (*Col.offset(c as isize)).shared3.prev = -(1 as libc::c_int);
            (*Col.offset(c as isize)).shared4.degree_next = next_col;
            if next_col != -(1 as libc::c_int) {
                (*Col.offset(next_col as isize)).shared3.prev = c;
            }
            *head.offset(score as isize) = c;
            min_score = if min_score < score { min_score } else { score };
        }
        c -= 1;
        c;
    }
    *p_n_col2 = n_col2;
    *p_n_row2 = n_row2;
    *p_max_deg = max_deg;
}
unsafe extern "C" fn find_ordering(
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
    mut Alen: libc::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut libc::c_int,
    mut head: *mut libc::c_int,
    mut n_col2: libc::c_int,
    mut max_deg: libc::c_int,
    mut pfree: libc::c_int,
    mut aggressive: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut pivot_col: libc::c_int = 0;
    let mut cp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pivot_row: libc::c_int = 0;
    let mut new_cp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut new_rp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pivot_row_start: libc::c_int = 0;
    let mut pivot_row_degree: libc::c_int = 0;
    let mut pivot_row_length: libc::c_int = 0;
    let mut pivot_col_score: libc::c_int = 0;
    let mut needed_memory: libc::c_int = 0;
    let mut cp_end: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rp_end: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut max_score: libc::c_int = 0;
    let mut cur_score: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut head_column: libc::c_int = 0;
    let mut first_col: libc::c_int = 0;
    let mut tag_mark: libc::c_int = 0;
    let mut row_mark: libc::c_int = 0;
    let mut set_difference: libc::c_int = 0;
    let mut min_score: libc::c_int = 0;
    let mut col_thickness: libc::c_int = 0;
    let mut max_mark: libc::c_int = 0;
    let mut pivot_col_thickness: libc::c_int = 0;
    let mut prev_col: libc::c_int = 0;
    let mut next_col: libc::c_int = 0;
    let mut ngarbage: libc::c_int = 0;
    max_mark = 2147483647 as libc::c_int - n_col;
    tag_mark = clear_mark(0 as libc::c_int, max_mark, n_row, Row);
    min_score = 0 as libc::c_int;
    ngarbage = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < n_col2 {
        while *head.offset(min_score as isize) == -(1 as libc::c_int)
            && min_score < n_col
        {
            min_score += 1;
            min_score;
        }
        pivot_col = *head.offset(min_score as isize);
        next_col = (*Col.offset(pivot_col as isize)).shared4.degree_next;
        *head.offset(min_score as isize) = next_col;
        if next_col != -(1 as libc::c_int) {
            (*Col.offset(next_col as isize)).shared3.prev = -(1 as libc::c_int);
        }
        pivot_col_score = (*Col.offset(pivot_col as isize)).shared2.score;
        (*Col.offset(pivot_col as isize)).shared2.order = k;
        pivot_col_thickness = (*Col.offset(pivot_col as isize)).shared1.thickness;
        k += pivot_col_thickness;
        needed_memory = if pivot_col_score < n_col - k {
            pivot_col_score
        } else {
            n_col - k
        };
        if pfree + needed_memory >= Alen {
            pfree = garbage_collection(
                n_row,
                n_col,
                Row,
                Col,
                A,
                &mut *A.offset(pfree as isize),
            );
            ngarbage += 1;
            ngarbage;
            tag_mark = clear_mark(0 as libc::c_int, max_mark, n_row, Row);
        }
        pivot_row_start = pfree;
        pivot_row_degree = 0 as libc::c_int;
        (*Col.offset(pivot_col as isize)).shared1.thickness = -pivot_col_thickness;
        cp = &mut *A.offset((*Col.offset(pivot_col as isize)).start as isize)
            as *mut libc::c_int;
        cp_end = cp.offset((*Col.offset(pivot_col as isize)).length as isize);
        while cp < cp_end {
            let fresh28 = cp;
            cp = cp.offset(1);
            row = *fresh28;
            if (*Row.offset(row as isize)).shared2.mark >= 0 as libc::c_int {
                rp = &mut *A.offset((*Row.offset(row as isize)).start as isize)
                    as *mut libc::c_int;
                rp_end = rp.offset((*Row.offset(row as isize)).length as isize);
                while rp < rp_end {
                    let fresh29 = rp;
                    rp = rp.offset(1);
                    col = *fresh29;
                    col_thickness = (*Col.offset(col as isize)).shared1.thickness;
                    if col_thickness > 0 as libc::c_int
                        && (*Col.offset(col as isize)).start >= 0 as libc::c_int
                    {
                        (*Col.offset(col as isize)).shared1.thickness = -col_thickness;
                        let fresh30 = pfree;
                        pfree = pfree + 1;
                        *A.offset(fresh30 as isize) = col;
                        pivot_row_degree += col_thickness;
                    }
                }
            }
        }
        (*Col.offset(pivot_col as isize)).shared1.thickness = pivot_col_thickness;
        max_deg = if max_deg > pivot_row_degree { max_deg } else { pivot_row_degree };
        cp = &mut *A.offset((*Col.offset(pivot_col as isize)).start as isize)
            as *mut libc::c_int;
        cp_end = cp.offset((*Col.offset(pivot_col as isize)).length as isize);
        while cp < cp_end {
            let fresh31 = cp;
            cp = cp.offset(1);
            row = *fresh31;
            (*Row.offset(row as isize)).shared2.mark = -(1 as libc::c_int);
        }
        pivot_row_length = pfree - pivot_row_start;
        if pivot_row_length > 0 as libc::c_int {
            pivot_row = *A.offset((*Col.offset(pivot_col as isize)).start as isize);
        } else {
            pivot_row = -(1 as libc::c_int);
        }
        rp = &mut *A.offset(pivot_row_start as isize) as *mut libc::c_int;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh32 = rp;
            rp = rp.offset(1);
            col = *fresh32;
            col_thickness = -(*Col.offset(col as isize)).shared1.thickness;
            (*Col.offset(col as isize)).shared1.thickness = col_thickness;
            cur_score = (*Col.offset(col as isize)).shared2.score;
            prev_col = (*Col.offset(col as isize)).shared3.prev;
            next_col = (*Col.offset(col as isize)).shared4.degree_next;
            if prev_col == -(1 as libc::c_int) {
                *head.offset(cur_score as isize) = next_col;
            } else {
                (*Col.offset(prev_col as isize)).shared4.degree_next = next_col;
            }
            if next_col != -(1 as libc::c_int) {
                (*Col.offset(next_col as isize)).shared3.prev = prev_col;
            }
            cp = &mut *A.offset((*Col.offset(col as isize)).start as isize)
                as *mut libc::c_int;
            cp_end = cp.offset((*Col.offset(col as isize)).length as isize);
            while cp < cp_end {
                let fresh33 = cp;
                cp = cp.offset(1);
                row = *fresh33;
                row_mark = (*Row.offset(row as isize)).shared2.mark;
                if row_mark < 0 as libc::c_int {
                    continue;
                }
                set_difference = row_mark - tag_mark;
                if set_difference < 0 as libc::c_int {
                    set_difference = (*Row.offset(row as isize)).shared1.degree;
                }
                set_difference -= col_thickness;
                if set_difference == 0 as libc::c_int && aggressive != 0 {
                    (*Row.offset(row as isize)).shared2.mark = -(1 as libc::c_int);
                } else {
                    (*Row.offset(row as isize)).shared2.mark = set_difference + tag_mark;
                }
            }
        }
        rp = &mut *A.offset(pivot_row_start as isize) as *mut libc::c_int;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh34 = rp;
            rp = rp.offset(1);
            col = *fresh34;
            hash = 0 as libc::c_int as libc::c_uint;
            cur_score = 0 as libc::c_int;
            cp = &mut *A.offset((*Col.offset(col as isize)).start as isize)
                as *mut libc::c_int;
            new_cp = cp;
            cp_end = cp.offset((*Col.offset(col as isize)).length as isize);
            while cp < cp_end {
                let fresh35 = cp;
                cp = cp.offset(1);
                row = *fresh35;
                row_mark = (*Row.offset(row as isize)).shared2.mark;
                if row_mark < 0 as libc::c_int {
                    continue;
                }
                let fresh36 = new_cp;
                new_cp = new_cp.offset(1);
                *fresh36 = row;
                hash = hash.wrapping_add(row as libc::c_uint);
                cur_score += row_mark - tag_mark;
                cur_score = if cur_score < n_col { cur_score } else { n_col };
            }
            (*Col.offset(col as isize))
                .length = new_cp
                .offset_from(
                    &mut *A.offset((*Col.offset(col as isize)).start as isize)
                        as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
            if (*Col.offset(col as isize)).length == 0 as libc::c_int {
                (*Col.offset(col as isize)).start = -(1 as libc::c_int);
                pivot_row_degree -= (*Col.offset(col as isize)).shared1.thickness;
                (*Col.offset(col as isize)).shared2.order = k;
                k += (*Col.offset(col as isize)).shared1.thickness;
            } else {
                (*Col.offset(col as isize)).shared2.score = cur_score;
                hash = hash.wrapping_rem((n_col + 1 as libc::c_int) as libc::c_uint);
                head_column = *head.offset(hash as isize);
                if head_column > -(1 as libc::c_int) {
                    first_col = (*Col.offset(head_column as isize)).shared3.headhash;
                    (*Col.offset(head_column as isize)).shared3.headhash = col;
                } else {
                    first_col = -(head_column + 2 as libc::c_int);
                    *head.offset(hash as isize) = -(col + 2 as libc::c_int);
                }
                (*Col.offset(col as isize)).shared4.hash_next = first_col;
                (*Col.offset(col as isize)).shared3.hash = hash as libc::c_int;
            }
        }
        detect_super_cols(Col, A, head, pivot_row_start, pivot_row_length);
        (*Col.offset(pivot_col as isize)).start = -(1 as libc::c_int);
        tag_mark = clear_mark(
            tag_mark + max_deg + 1 as libc::c_int,
            max_mark,
            n_row,
            Row,
        );
        rp = &mut *A.offset(pivot_row_start as isize) as *mut libc::c_int;
        new_rp = rp;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh37 = rp;
            rp = rp.offset(1);
            col = *fresh37;
            if (*Col.offset(col as isize)).start < 0 as libc::c_int {
                continue;
            }
            let fresh38 = new_rp;
            new_rp = new_rp.offset(1);
            *fresh38 = col;
            let ref mut fresh39 = (*Col.offset(col as isize)).length;
            let fresh40 = *fresh39;
            *fresh39 = *fresh39 + 1;
            *A
                .offset(
                    ((*Col.offset(col as isize)).start + fresh40) as isize,
                ) = pivot_row;
            cur_score = (*Col.offset(col as isize)).shared2.score + pivot_row_degree;
            max_score = n_col - k - (*Col.offset(col as isize)).shared1.thickness;
            cur_score -= (*Col.offset(col as isize)).shared1.thickness;
            cur_score = if cur_score < max_score { cur_score } else { max_score };
            (*Col.offset(col as isize)).shared2.score = cur_score;
            next_col = *head.offset(cur_score as isize);
            (*Col.offset(col as isize)).shared4.degree_next = next_col;
            (*Col.offset(col as isize)).shared3.prev = -(1 as libc::c_int);
            if next_col != -(1 as libc::c_int) {
                (*Col.offset(next_col as isize)).shared3.prev = col;
            }
            *head.offset(cur_score as isize) = col;
            min_score = if min_score < cur_score { min_score } else { cur_score };
        }
        if pivot_row_degree > 0 as libc::c_int {
            (*Row.offset(pivot_row as isize)).start = pivot_row_start;
            (*Row.offset(pivot_row as isize))
                .length = new_rp
                .offset_from(
                    &mut *A.offset(pivot_row_start as isize) as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
            (*Row.offset(pivot_row as isize)).shared1.degree = pivot_row_degree;
            (*Row.offset(pivot_row as isize)).shared2.mark = 0 as libc::c_int;
        }
    }
    return ngarbage;
}
unsafe extern "C" fn order_children(
    mut n_col: libc::c_int,
    mut Col: *mut Colamd_Col,
    mut p: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    let mut order: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_col {
        if !((*Col.offset(i as isize)).start == -(1 as libc::c_int))
            && (*Col.offset(i as isize)).shared2.order == -(1 as libc::c_int)
        {
            parent = i;
            loop {
                parent = (*Col.offset(parent as isize)).shared1.parent;
                if (*Col.offset(parent as isize)).start == -(1 as libc::c_int) {
                    break;
                }
            }
            c = i;
            order = (*Col.offset(parent as isize)).shared2.order;
            loop {
                let fresh41 = order;
                order = order + 1;
                (*Col.offset(c as isize)).shared2.order = fresh41;
                (*Col.offset(c as isize)).shared1.parent = parent;
                c = (*Col.offset(c as isize)).shared1.parent;
                if !((*Col.offset(c as isize)).shared2.order == -(1 as libc::c_int)) {
                    break;
                }
            }
            (*Col.offset(parent as isize)).shared2.order = order;
        }
        i += 1;
        i;
    }
    c = 0 as libc::c_int;
    while c < n_col {
        *p.offset((*Col.offset(c as isize)).shared2.order as isize) = c;
        c += 1;
        c;
    }
}
unsafe extern "C" fn detect_super_cols(
    mut Col: *mut Colamd_Col,
    mut A: *mut libc::c_int,
    mut head: *mut libc::c_int,
    mut row_start: libc::c_int,
    mut row_length: libc::c_int,
) {
    let mut hash: libc::c_int = 0;
    let mut rp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c: libc::c_int = 0;
    let mut super_c: libc::c_int = 0;
    let mut cp1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cp2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut length: libc::c_int = 0;
    let mut prev_c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rp_end: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut col: libc::c_int = 0;
    let mut head_column: libc::c_int = 0;
    let mut first_col: libc::c_int = 0;
    rp = &mut *A.offset(row_start as isize) as *mut libc::c_int;
    rp_end = rp.offset(row_length as isize);
    while rp < rp_end {
        let fresh42 = rp;
        rp = rp.offset(1);
        col = *fresh42;
        if (*Col.offset(col as isize)).start < 0 as libc::c_int {
            continue;
        }
        hash = (*Col.offset(col as isize)).shared3.hash;
        head_column = *head.offset(hash as isize);
        if head_column > -(1 as libc::c_int) {
            first_col = (*Col.offset(head_column as isize)).shared3.headhash;
        } else {
            first_col = -(head_column + 2 as libc::c_int);
        }
        super_c = first_col;
        while super_c != -(1 as libc::c_int) {
            length = (*Col.offset(super_c as isize)).length;
            prev_c = super_c;
            c = (*Col.offset(super_c as isize)).shared4.hash_next;
            while c != -(1 as libc::c_int) {
                if (*Col.offset(c as isize)).length != length
                    || (*Col.offset(c as isize)).shared2.score
                        != (*Col.offset(super_c as isize)).shared2.score
                {
                    prev_c = c;
                } else {
                    cp1 = &mut *A.offset((*Col.offset(super_c as isize)).start as isize)
                        as *mut libc::c_int;
                    cp2 = &mut *A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut libc::c_int;
                    i = 0 as libc::c_int;
                    while i < length {
                        let fresh43 = cp1;
                        cp1 = cp1.offset(1);
                        let fresh44 = cp2;
                        cp2 = cp2.offset(1);
                        if *fresh43 != *fresh44 {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if i != length {
                        prev_c = c;
                    } else {
                        (*Col.offset(super_c as isize)).shared1.thickness
                            += (*Col.offset(c as isize)).shared1.thickness;
                        (*Col.offset(c as isize)).shared1.parent = super_c;
                        (*Col.offset(c as isize)).start = -(2 as libc::c_int);
                        (*Col.offset(c as isize)).shared2.order = -(1 as libc::c_int);
                        (*Col.offset(prev_c as isize))
                            .shared4
                            .hash_next = (*Col.offset(c as isize)).shared4.hash_next;
                    }
                }
                c = (*Col.offset(c as isize)).shared4.hash_next;
            }
            super_c = (*Col.offset(super_c as isize)).shared4.hash_next;
        }
        if head_column > -(1 as libc::c_int) {
            (*Col.offset(head_column as isize)).shared3.headhash = -(1 as libc::c_int);
        } else {
            *head.offset(hash as isize) = -(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn garbage_collection(
    mut n_row: libc::c_int,
    mut n_col: libc::c_int,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut libc::c_int,
    mut pfree: *mut libc::c_int,
) -> libc::c_int {
    let mut psrc: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pdest: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    pdest = &mut *A.offset(0 as libc::c_int as isize) as *mut libc::c_int;
    c = 0 as libc::c_int;
    while c < n_col {
        if (*Col.offset(c as isize)).start >= 0 as libc::c_int {
            psrc = &mut *A.offset((*Col.offset(c as isize)).start as isize)
                as *mut libc::c_int;
            (*Col.offset(c as isize))
                .start = pdest
                .offset_from(
                    &mut *A.offset(0 as libc::c_int as isize) as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
            length = (*Col.offset(c as isize)).length;
            j = 0 as libc::c_int;
            while j < length {
                let fresh45 = psrc;
                psrc = psrc.offset(1);
                r = *fresh45;
                if (*Row.offset(r as isize)).shared2.mark >= 0 as libc::c_int {
                    let fresh46 = pdest;
                    pdest = pdest.offset(1);
                    *fresh46 = r;
                }
                j += 1;
                j;
            }
            (*Col.offset(c as isize))
                .length = pdest
                .offset_from(
                    &mut *A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
        }
        c += 1;
        c;
    }
    r = 0 as libc::c_int;
    while r < n_row {
        if (*Row.offset(r as isize)).shared2.mark < 0 as libc::c_int
            || (*Row.offset(r as isize)).length == 0 as libc::c_int
        {
            (*Row.offset(r as isize)).shared2.mark = -(1 as libc::c_int);
        } else {
            psrc = &mut *A.offset((*Row.offset(r as isize)).start as isize)
                as *mut libc::c_int;
            (*Row.offset(r as isize)).shared2.first_column = *psrc;
            *psrc = -r - 1 as libc::c_int;
        }
        r += 1;
        r;
    }
    psrc = pdest;
    while psrc < pfree {
        let fresh47 = psrc;
        psrc = psrc.offset(1);
        if *fresh47 < 0 as libc::c_int {
            psrc = psrc.offset(-1);
            psrc;
            r = -*psrc - 1 as libc::c_int;
            *psrc = (*Row.offset(r as isize)).shared2.first_column;
            (*Row.offset(r as isize))
                .start = pdest
                .offset_from(
                    &mut *A.offset(0 as libc::c_int as isize) as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
            length = (*Row.offset(r as isize)).length;
            j = 0 as libc::c_int;
            while j < length {
                let fresh48 = psrc;
                psrc = psrc.offset(1);
                c = *fresh48;
                if (*Col.offset(c as isize)).start >= 0 as libc::c_int {
                    let fresh49 = pdest;
                    pdest = pdest.offset(1);
                    *fresh49 = c;
                }
                j += 1;
                j;
            }
            (*Row.offset(r as isize))
                .length = pdest
                .offset_from(
                    &mut *A.offset((*Row.offset(r as isize)).start as isize)
                        as *mut libc::c_int,
                ) as libc::c_long as libc::c_int;
        }
    }
    return pdest
        .offset_from(&mut *A.offset(0 as libc::c_int as isize) as *mut libc::c_int)
        as libc::c_long as libc::c_int;
}
unsafe extern "C" fn clear_mark(
    mut tag_mark: libc::c_int,
    mut max_mark: libc::c_int,
    mut n_row: libc::c_int,
    mut Row: *mut Colamd_Row,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if tag_mark <= 0 as libc::c_int || tag_mark >= max_mark {
        r = 0 as libc::c_int;
        while r < n_row {
            if (*Row.offset(r as isize)).shared2.mark >= 0 as libc::c_int {
                (*Row.offset(r as isize)).shared2.mark = 0 as libc::c_int;
            }
            r += 1;
            r;
        }
        tag_mark = 1 as libc::c_int;
    }
    return tag_mark;
}
unsafe extern "C" fn print_report(
    mut method: *mut libc::c_char,
    mut stats: *mut libc::c_int,
) {
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut i3: libc::c_int = 0;
    if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
        .is_some()
    {
        glp_printf(
            b"\n%s version %d.%d, %s: \0" as *const u8 as *const libc::c_char,
            method,
            2 as libc::c_int,
            7 as libc::c_int,
            b"Nov 1, 2007\0" as *const u8 as *const libc::c_char,
        );
    }
    if stats.is_null() {
        if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
            .is_some()
        {
            glp_printf(
                b"No statistics available.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    i1 = *stats.offset(4 as libc::c_int as isize);
    i2 = *stats.offset(5 as libc::c_int as isize);
    i3 = *stats.offset(6 as libc::c_int as isize);
    if *stats.offset(3 as libc::c_int as isize) >= 0 as libc::c_int {
        if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
            .is_some()
        {
            glp_printf(b"OK.  \0" as *const u8 as *const libc::c_char);
        }
    } else if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
        .is_some()
    {
        glp_printf(b"ERROR.  \0" as *const u8 as *const libc::c_char);
    }
    let mut current_block_100: u64;
    match *stats.offset(3 as libc::c_int as isize) {
        1 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Matrix has unsorted or duplicate row indices.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"%s: number of duplicate or out-of-order row indices: %d\n\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    i3,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"%s: last seen duplicate or out-of-order row index:   %d\n\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    i2,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"%s: last seen in column:                             %d\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    i1,
                );
            }
            current_block_100 = 12684763486994553302;
        }
        0 => {
            current_block_100 = 12684763486994553302;
        }
        -1 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Array A (row indices of matrix) not present.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -2 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Array p (column pointers for matrix) not present.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -3 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Invalid number of rows (%d).\n\0" as *const u8
                        as *const libc::c_char,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -4 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Invalid number of columns (%d).\n\0" as *const u8
                        as *const libc::c_char,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -5 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Invalid number of nonzero entries (%d).\n\0" as *const u8
                        as *const libc::c_char,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -6 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Invalid column pointer, p [0] = %d, must be zero.\n\0" as *const u8
                        as *const libc::c_char,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -7 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Array A too small.\n\0" as *const u8 as *const libc::c_char,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"        Need Alen >= %d, but given only Alen = %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    i1,
                    i2,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -8 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Column %d has a negative number of nonzero entries (%d).\n\0"
                        as *const u8 as *const libc::c_char,
                    i1,
                    i2,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -9 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"Row index (row %d) out of bounds (%d to %d) in column %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    i2,
                    0 as libc::c_int,
                    i3 - 1 as libc::c_int,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -10 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(b"Out of memory.\n\0" as *const u8 as *const libc::c_char);
            }
            current_block_100 = 15237655884915618618;
        }
        _ => {
            current_block_100 = 15237655884915618618;
        }
    }
    match current_block_100 {
        12684763486994553302 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"%s: number of dense or empty rows ignored:           %d\n\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    *stats.offset(0 as libc::c_int as isize),
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"%s: number of dense or empty columns ignored:        %d\n\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    *stats.offset(1 as libc::c_int as isize),
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .is_some()
            {
                glp_printf(
                    b"%s: number of garbage collections performed:         %d\n\0"
                        as *const u8 as *const libc::c_char,
                    method,
                    *stats.offset(2 as libc::c_int as isize),
                );
            }
        }
        _ => {}
    };
}

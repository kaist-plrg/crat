use ::libc;
extern "C" {
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n_0: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub n: libc::c_int,
    pub wt: *const libc::c_int,
    pub a: *const libc::c_uchar,
    pub record: libc::c_int,
    pub rec_level: libc::c_int,
    pub rec: *mut libc::c_int,
    pub clique: *mut libc::c_int,
    pub set: *mut libc::c_int,
}
unsafe extern "C" fn sub(
    mut csa: *mut csa,
    mut ct: libc::c_int,
    mut table: *mut libc::c_int,
    mut level: libc::c_int,
    mut weight: libc::c_int,
    mut l_weight: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut curr_weight: libc::c_int = 0;
    let mut left_weight: libc::c_int = 0;
    let mut p1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut p2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut newtable: *mut libc::c_int = 0 as *mut libc::c_int;
    newtable = glp_alloc(
        (*csa).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    if ct <= 0 as libc::c_int {
        if ct == 0 as libc::c_int {
            let fresh0 = level;
            level = level + 1;
            *((*csa).set)
                .offset(fresh0 as isize) = *table.offset(0 as libc::c_int as isize);
            weight += l_weight;
        }
        if weight > (*csa).record {
            (*csa).record = weight;
            (*csa).rec_level = level;
            i = 0 as libc::c_int;
            while i < level {
                *((*csa).rec).offset(i as isize) = *((*csa).set).offset(i as isize);
                i += 1;
                i;
            }
        }
    } else {
        i = ct;
        while i >= 0 as libc::c_int {
            if level == 0 as libc::c_int && i < ct {
                break;
            }
            k = *table.offset(i as isize);
            if level > 0 as libc::c_int
                && *((*csa).clique).offset(k as isize) <= (*csa).record - weight
            {
                break;
            }
            *((*csa).set).offset(level as isize) = k;
            curr_weight = weight + *((*csa).wt).offset(k as isize);
            l_weight -= *((*csa).wt).offset(k as isize);
            if l_weight <= (*csa).record - curr_weight {
                break;
            }
            p1 = newtable;
            p2 = table;
            left_weight = 0 as libc::c_int;
            while p2 < table.offset(i as isize) {
                let fresh1 = p2;
                p2 = p2.offset(1);
                j = *fresh1;
                if if j == k {
                    0 as libc::c_int
                } else if j > k {
                    *((*csa).a)
                        .offset(
                            ((j * (j - 1 as libc::c_int) / 2 as libc::c_int + k)
                                / 8 as libc::c_int) as isize,
                        ) as libc::c_int
                        & ((1 as libc::c_int)
                            << 8 as libc::c_int - 1 as libc::c_int
                                - (j * (j - 1 as libc::c_int) / 2 as libc::c_int + k)
                                    % 8 as libc::c_int) as libc::c_uchar as libc::c_int
                } else {
                    *((*csa).a)
                        .offset(
                            ((k * (k - 1 as libc::c_int) / 2 as libc::c_int + j)
                                / 8 as libc::c_int) as isize,
                        ) as libc::c_int
                        & ((1 as libc::c_int)
                            << 8 as libc::c_int - 1 as libc::c_int
                                - (k * (k - 1 as libc::c_int) / 2 as libc::c_int + j)
                                    % 8 as libc::c_int) as libc::c_uchar as libc::c_int
                } != 0
                {
                    let fresh2 = p1;
                    p1 = p1.offset(1);
                    *fresh2 = j;
                    left_weight += *((*csa).wt).offset(j as isize);
                }
            }
            if !(left_weight <= (*csa).record - curr_weight) {
                sub(
                    csa,
                    (p1.offset_from(newtable) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int,
                    newtable,
                    level + 1 as libc::c_int,
                    curr_weight,
                    left_weight,
                );
            }
            i -= 1;
            i;
        }
    }
    glp_free(newtable as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_wclique(
    mut n_: libc::c_int,
    mut w: *const libc::c_int,
    mut a_: *const libc::c_uchar,
    mut ind: *mut libc::c_int,
) -> libc::c_int {
    let mut csa_: csa = csa {
        n: 0,
        wt: 0 as *const libc::c_int,
        a: 0 as *const libc::c_uchar,
        record: 0,
        rec_level: 0,
        rec: 0 as *mut libc::c_int,
        clique: 0 as *mut libc::c_int,
        set: 0 as *mut libc::c_int,
    };
    let mut csa: *mut csa = &mut csa_;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut max_wt: libc::c_int = 0;
    let mut max_nwt: libc::c_int = 0;
    let mut wth: libc::c_int = 0;
    let mut used: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nwt: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut timer: libc::c_double = 0.;
    (*csa).n = n_;
    ((*csa).n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"misc/wclique.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*csa).wt = &*w.offset(1 as libc::c_int as isize) as *const libc::c_int;
    (*csa).a = a_;
    (*csa).record = 0 as libc::c_int;
    (*csa).rec_level = 0 as libc::c_int;
    (*csa).rec = &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int;
    (*csa)
        .clique = glp_alloc(
        (*csa).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa)
        .set = glp_alloc(
        (*csa).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    used = glp_alloc(
        (*csa).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    nwt = glp_alloc(
        (*csa).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    pos = glp_alloc(
        (*csa).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    timer = glp_time();
    i = 0 as libc::c_int;
    while i < (*csa).n {
        *nwt.offset(i as isize) = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*csa).n {
            if if i == j {
                0 as libc::c_int
            } else if i > j {
                *((*csa).a)
                    .offset(
                        ((i * (i - 1 as libc::c_int) / 2 as libc::c_int + j)
                            / 8 as libc::c_int) as isize,
                    ) as libc::c_int
                    & ((1 as libc::c_int)
                        << 8 as libc::c_int - 1 as libc::c_int
                            - (i * (i - 1 as libc::c_int) / 2 as libc::c_int + j)
                                % 8 as libc::c_int) as libc::c_uchar as libc::c_int
            } else {
                *((*csa).a)
                    .offset(
                        ((j * (j - 1 as libc::c_int) / 2 as libc::c_int + i)
                            / 8 as libc::c_int) as isize,
                    ) as libc::c_int
                    & ((1 as libc::c_int)
                        << 8 as libc::c_int - 1 as libc::c_int
                            - (j * (j - 1 as libc::c_int) / 2 as libc::c_int + i)
                                % 8 as libc::c_int) as libc::c_uchar as libc::c_int
            } != 0
            {
                *nwt.offset(i as isize) += *((*csa).wt).offset(j as isize);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*csa).n {
        *used.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = (*csa).n - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        max_wt = -(1 as libc::c_int);
        max_nwt = -(1 as libc::c_int);
        j = 0 as libc::c_int;
        while j < (*csa).n {
            if *used.offset(j as isize) == 0
                && (*((*csa).wt).offset(j as isize) > max_wt
                    || *((*csa).wt).offset(j as isize) == max_wt
                        && *nwt.offset(j as isize) > max_nwt)
            {
                max_wt = *((*csa).wt).offset(j as isize);
                max_nwt = *nwt.offset(j as isize);
                p = j;
            }
            j += 1;
            j;
        }
        *pos.offset(i as isize) = p;
        *used.offset(p as isize) = 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*csa).n {
            if *used.offset(j as isize) == 0 && j != p
                && (if p == j {
                    0 as libc::c_int
                } else {
                    (if p > j {
                        *((*csa).a)
                            .offset(
                                ((p * (p - 1 as libc::c_int) / 2 as libc::c_int + j)
                                    / 8 as libc::c_int) as isize,
                            ) as libc::c_int
                            & ((1 as libc::c_int)
                                << 8 as libc::c_int - 1 as libc::c_int
                                    - (p * (p - 1 as libc::c_int) / 2 as libc::c_int + j)
                                        % 8 as libc::c_int) as libc::c_uchar as libc::c_int
                    } else {
                        *((*csa).a)
                            .offset(
                                ((j * (j - 1 as libc::c_int) / 2 as libc::c_int + p)
                                    / 8 as libc::c_int) as isize,
                            ) as libc::c_int
                            & ((1 as libc::c_int)
                                << 8 as libc::c_int - 1 as libc::c_int
                                    - (j * (j - 1 as libc::c_int) / 2 as libc::c_int + p)
                                        % 8 as libc::c_int) as libc::c_uchar as libc::c_int
                    })
                }) != 0
            {
                *nwt.offset(j as isize) -= *((*csa).wt).offset(p as isize);
            }
            j += 1;
            j;
        }
        i -= 1;
        i;
    }
    wth = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*csa).n {
        wth += *((*csa).wt).offset(*pos.offset(i as isize) as isize);
        sub(csa, i, pos, 0 as libc::c_int, 0 as libc::c_int, wth);
        *((*csa).clique).offset(*pos.offset(i as isize) as isize) = (*csa).record;
        if glp_difftime(glp_time(), timer) >= 5.0f64 - 0.001f64 {
            glp_printf(
                b"level = %d (%d); best = %d\n\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
                (*csa).n,
                (*csa).record,
            );
            timer = glp_time();
        }
        i += 1;
        i;
    }
    glp_free((*csa).clique as *mut libc::c_void);
    glp_free((*csa).set as *mut libc::c_void);
    glp_free(used as *mut libc::c_void);
    glp_free(nwt as *mut libc::c_void);
    glp_free(pos as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i <= (*csa).rec_level {
        let ref mut fresh3 = *ind.offset(i as isize);
        *fresh3 += 1;
        *fresh3;
        i += 1;
        i;
    }
    return (*csa).rec_level;
}

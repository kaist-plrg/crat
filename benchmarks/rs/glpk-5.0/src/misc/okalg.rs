use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
unsafe extern "C" fn overflow(mut u: libc::c_int, mut v: libc::c_int) -> libc::c_int {
    if u > 0 as libc::c_int && v > 0 as libc::c_int && u + v < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if u < 0 as libc::c_int && v < 0 as libc::c_int && u + v > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_okalg(
    mut nv: libc::c_int,
    mut na: libc::c_int,
    mut tail: *const libc::c_int,
    mut head: *const libc::c_int,
    mut low: *const libc::c_int,
    mut cap: *const libc::c_int,
    mut cost: *const libc::c_int,
    mut x: *mut libc::c_int,
    mut pi: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut a: libc::c_int = 0;
    let mut aok: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lambda: libc::c_int = 0;
    let mut pos1: libc::c_int = 0;
    let mut pos2: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut arc: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut link: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    (nv >= 0 as libc::c_int
        || {
            glp_assert_(
                b"nv >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (na >= 0 as libc::c_int
        || {
            glp_assert_(
                b"na >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    a = 1 as libc::c_int;
    while a <= na {
        i = *tail.offset(a as isize);
        j = *head.offset(a as isize);
        (1 as libc::c_int <= i && i <= nv
            || {
                glp_assert_(
                    b"1 <= i && i <= nv\0" as *const u8 as *const libc::c_char,
                    b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                    107 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= j && j <= nv
            || {
                glp_assert_(
                    b"1 <= j && j <= nv\0" as *const u8 as *const libc::c_char,
                    b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                    108 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (i != j
            || {
                glp_assert_(
                    b"i != j\0" as *const u8 as *const libc::c_char,
                    b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                    109 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (0 as libc::c_int <= *low.offset(a as isize)
            && *low.offset(a as isize) <= *cap.offset(a as isize)
            || {
                glp_assert_(
                    b"0 <= low[a] && low[a] <= cap[a]\0" as *const u8
                        as *const libc::c_char,
                    b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        a += 1;
        a;
    }
    ptr = glp_alloc(
        1 as libc::c_int + nv + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    arc = glp_alloc(
        1 as libc::c_int + na + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    link = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    list = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= nv {
        *ptr.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    a = 1 as libc::c_int;
    while a <= na {
        let ref mut fresh0 = *ptr.offset(*tail.offset(a as isize) as isize);
        *fresh0 += 1;
        *fresh0;
        let ref mut fresh1 = *ptr.offset(*head.offset(a as isize) as isize);
        *fresh1 += 1;
        *fresh1;
        a += 1;
        a;
    }
    let ref mut fresh2 = *ptr.offset(1 as libc::c_int as isize);
    *fresh2 += 1;
    *fresh2;
    i = 1 as libc::c_int;
    while i < nv {
        *ptr.offset((i + 1 as libc::c_int) as isize) += *ptr.offset(i as isize);
        i += 1;
        i;
    }
    *ptr.offset((nv + 1 as libc::c_int) as isize) = *ptr.offset(nv as isize);
    a = 1 as libc::c_int;
    while a <= na {
        let ref mut fresh3 = *ptr.offset(*tail.offset(a as isize) as isize);
        *fresh3 -= 1;
        *arc.offset(*fresh3 as isize) = a;
        let ref mut fresh4 = *ptr.offset(*head.offset(a as isize) as isize);
        *fresh4 -= 1;
        *arc.offset(*fresh4 as isize) = a;
        a += 1;
        a;
    }
    (*ptr.offset(1 as libc::c_int as isize) == 1 as libc::c_int
        || {
            glp_assert_(
                b"ptr[1] == 1\0" as *const u8 as *const libc::c_char,
                b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*ptr.offset((nv + 1 as libc::c_int) as isize) == na + na + 1 as libc::c_int
        || {
            glp_assert_(
                b"ptr[nv+1] == na+na+1\0" as *const u8 as *const libc::c_char,
                b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    a = 1 as libc::c_int;
    while a <= na {
        *x.offset(a as isize) = 0 as libc::c_int;
        a += 1;
        a;
    }
    i = 1 as libc::c_int;
    while i <= nv {
        *pi.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    '_loop: loop {
        aok = 0 as libc::c_int;
        a = 1 as libc::c_int;
        while a <= na {
            i = *tail.offset(a as isize);
            j = *head.offset(a as isize);
            if overflow(
                *cost.offset(a as isize),
                *pi.offset(i as isize) - *pi.offset(j as isize),
            ) != 0
            {
                ret = 2 as libc::c_int;
                current_block = 3397862563670105393;
                break '_loop;
            } else {
                lambda = *cost.offset(a as isize)
                    + (*pi.offset(i as isize) - *pi.offset(j as isize));
                if *x.offset(a as isize) < *low.offset(a as isize)
                    || lambda < 0 as libc::c_int
                        && *x.offset(a as isize) < *cap.offset(a as isize)
                {
                    aok = a;
                    s = j;
                    t = i;
                    break;
                } else if *x.offset(a as isize) > *cap.offset(a as isize)
                    || lambda > 0 as libc::c_int
                        && *x.offset(a as isize) > *low.offset(a as isize)
                {
                    aok = a;
                    s = i;
                    t = j;
                    break;
                } else {
                    a += 1;
                    a;
                }
            }
        }
        if aok == 0 as libc::c_int {
            a = 1 as libc::c_int;
            current_block = 790185930182612747;
            break;
        } else {
            i = 1 as libc::c_int;
            while i <= nv {
                *link.offset(i as isize) = 0 as libc::c_int;
                i += 1;
                i;
            }
            *link.offset(s as isize) = aok;
            *list.offset(1 as libc::c_int as isize) = s;
            pos2 = 1 as libc::c_int;
            pos1 = pos2;
            's_367: loop {
                if !(pos1 <= pos2) {
                    current_block = 7416055328783156979;
                    break;
                }
                let fresh5 = pos1;
                pos1 = pos1 + 1;
                i = *list.offset(fresh5 as isize);
                k = *ptr.offset(i as isize);
                while k < *ptr.offset((i + 1 as libc::c_int) as isize) {
                    a = *arc.offset(k as isize);
                    if *tail.offset(a as isize) == i {
                        j = *head.offset(a as isize);
                        if *link.offset(j as isize) != 0 as libc::c_int {
                            current_block = 2168227384378665163;
                        } else if *x.offset(a as isize) >= *cap.offset(a as isize) {
                            current_block = 2168227384378665163;
                        } else if overflow(
                            *cost.offset(a as isize),
                            *pi.offset(i as isize) - *pi.offset(j as isize),
                        ) != 0
                        {
                            ret = 2 as libc::c_int;
                            current_block = 3397862563670105393;
                            break '_loop;
                        } else {
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(i as isize) - *pi.offset(j as isize));
                            if lambda > 0 as libc::c_int
                                && *x.offset(a as isize) >= *low.offset(a as isize)
                            {
                                current_block = 2168227384378665163;
                            } else {
                                current_block = 11071260907632769126;
                            }
                        }
                    } else if *head.offset(a as isize) == i {
                        j = *tail.offset(a as isize);
                        if *link.offset(j as isize) != 0 as libc::c_int {
                            current_block = 2168227384378665163;
                        } else if *x.offset(a as isize) <= *low.offset(a as isize) {
                            current_block = 2168227384378665163;
                        } else if overflow(
                            *cost.offset(a as isize),
                            *pi.offset(j as isize) - *pi.offset(i as isize),
                        ) != 0
                        {
                            ret = 2 as libc::c_int;
                            current_block = 3397862563670105393;
                            break '_loop;
                        } else {
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(j as isize) - *pi.offset(i as isize));
                            if lambda < 0 as libc::c_int
                                && *x.offset(a as isize) <= *cap.offset(a as isize)
                            {
                                current_block = 2168227384378665163;
                            } else {
                                current_block = 11071260907632769126;
                            }
                        }
                    } else {
                        (a != a
                            || {
                                glp_assert_(
                                    b"a != a\0" as *const u8 as *const libc::c_char,
                                    b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                    256 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        current_block = 11071260907632769126;
                    }
                    match current_block {
                        11071260907632769126 => {
                            *link.offset(j as isize) = a;
                            pos2 += 1;
                            *list.offset(pos2 as isize) = j;
                            if j == t {
                                current_block = 15779171657471745057;
                                break 's_367;
                            }
                        }
                        _ => {}
                    }
                    k += 1;
                    k;
                }
            }
            match current_block {
                15779171657471745057 => {
                    delta = 0 as libc::c_int;
                    j = t;
                    loop {
                        a = *link.offset(j as isize);
                        if *head.offset(a as isize) == j {
                            i = *tail.offset(a as isize);
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(i as isize) - *pi.offset(j as isize));
                            if lambda > 0 as libc::c_int
                                && *x.offset(a as isize) < *low.offset(a as isize)
                            {
                                temp = *low.offset(a as isize) - *x.offset(a as isize);
                            } else if lambda <= 0 as libc::c_int
                                && *x.offset(a as isize) < *cap.offset(a as isize)
                            {
                                temp = *cap.offset(a as isize) - *x.offset(a as isize);
                            } else {
                                (a != a
                                    || {
                                        glp_assert_(
                                            b"a != a\0" as *const u8 as *const libc::c_char,
                                            b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                            326 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                        } else if *tail.offset(a as isize) == j {
                            i = *head.offset(a as isize);
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(j as isize) - *pi.offset(i as isize));
                            if lambda < 0 as libc::c_int
                                && *x.offset(a as isize) > *cap.offset(a as isize)
                            {
                                temp = *x.offset(a as isize) - *cap.offset(a as isize);
                            } else if lambda >= 0 as libc::c_int
                                && *x.offset(a as isize) > *low.offset(a as isize)
                            {
                                temp = *x.offset(a as isize) - *low.offset(a as isize);
                            } else {
                                (a != a
                                    || {
                                        glp_assert_(
                                            b"a != a\0" as *const u8 as *const libc::c_char,
                                            b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                            341 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                        } else {
                            (a != a
                                || {
                                    glp_assert_(
                                        b"a != a\0" as *const u8 as *const libc::c_char,
                                        b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                        344 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                        if delta == 0 as libc::c_int || delta > temp {
                            delta = temp;
                        }
                        if i == t {
                            break;
                        }
                        j = i;
                    }
                    (delta > 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"delta > 0\0" as *const u8 as *const libc::c_char,
                                b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                349 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    j = t;
                    loop {
                        a = *link.offset(j as isize);
                        if *head.offset(a as isize) == j {
                            i = *tail.offset(a as isize);
                            *x.offset(a as isize) += delta;
                        } else if *tail.offset(a as isize) == j {
                            i = *head.offset(a as isize);
                            *x.offset(a as isize) -= delta;
                        } else {
                            (a != a
                                || {
                                    glp_assert_(
                                        b"a != a\0" as *const u8 as *const libc::c_char,
                                        b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                        367 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                        if i == t {
                            break;
                        }
                        j = i;
                    }
                }
                _ => {
                    delta = 0 as libc::c_int;
                    a = 1 as libc::c_int;
                    while a <= na {
                        i = *tail.offset(a as isize);
                        j = *head.offset(a as isize);
                        if *link.offset(i as isize) != 0 as libc::c_int
                            && *link.offset(j as isize) == 0 as libc::c_int
                        {
                            if overflow(
                                *cost.offset(a as isize),
                                *pi.offset(i as isize) - *pi.offset(j as isize),
                            ) != 0
                            {
                                ret = 2 as libc::c_int;
                                current_block = 3397862563670105393;
                                break '_loop;
                            } else {
                                lambda = *cost.offset(a as isize)
                                    + (*pi.offset(i as isize) - *pi.offset(j as isize));
                                if *x.offset(a as isize) <= *cap.offset(a as isize)
                                    && lambda > 0 as libc::c_int
                                {
                                    if delta == 0 as libc::c_int || delta > lambda {
                                        delta = lambda;
                                    }
                                }
                            }
                        } else if *link.offset(i as isize) == 0 as libc::c_int
                            && *link.offset(j as isize) != 0 as libc::c_int
                        {
                            if overflow(
                                *cost.offset(a as isize),
                                *pi.offset(i as isize) - *pi.offset(j as isize),
                            ) != 0
                            {
                                ret = 2 as libc::c_int;
                                current_block = 3397862563670105393;
                                break '_loop;
                            } else {
                                lambda = *cost.offset(a as isize)
                                    + (*pi.offset(i as isize) - *pi.offset(j as isize));
                                if *x.offset(a as isize) >= *low.offset(a as isize)
                                    && lambda < 0 as libc::c_int
                                {
                                    if delta == 0 as libc::c_int || delta > -lambda {
                                        delta = -lambda;
                                    }
                                }
                            }
                        }
                        a += 1;
                        a;
                    }
                    if delta == 0 as libc::c_int {
                        ret = 1 as libc::c_int;
                        current_block = 3397862563670105393;
                        break;
                    } else {
                        i = 1 as libc::c_int;
                        while i <= nv {
                            if *link.offset(i as isize) == 0 as libc::c_int {
                                if overflow(*pi.offset(i as isize), delta) != 0 {
                                    ret = 2 as libc::c_int;
                                    current_block = 3397862563670105393;
                                    break '_loop;
                                } else {
                                    *pi.offset(i as isize) += delta;
                                }
                            }
                            i += 1;
                            i;
                        }
                    }
                }
            }
        }
    }
    's_219: loop {
        match current_block {
            3397862563670105393 => {
                glp_free(ptr as *mut libc::c_void);
                break;
            }
            _ => {
                if a <= na {
                    if !(*low.offset(a as isize) <= *x.offset(a as isize)
                        && *x.offset(a as isize) <= *cap.offset(a as isize))
                    {
                        ret = 3 as libc::c_int;
                        current_block = 3397862563670105393;
                    } else {
                        a += 1;
                        a;
                        current_block = 790185930182612747;
                    }
                } else {
                    i = 1 as libc::c_int;
                    while i <= nv {
                        temp = 0 as libc::c_int;
                        k = *ptr.offset(i as isize);
                        while k < *ptr.offset((i + 1 as libc::c_int) as isize) {
                            a = *arc.offset(k as isize);
                            if *tail.offset(a as isize) == i {
                                temp += *x.offset(a as isize);
                            } else if *head.offset(a as isize) == i {
                                temp -= *x.offset(a as isize);
                            } else {
                                (a != a
                                    || {
                                        glp_assert_(
                                            b"a != a\0" as *const u8 as *const libc::c_char,
                                            b"misc/okalg.c\0" as *const u8 as *const libc::c_char,
                                            188 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                            k += 1;
                            k;
                        }
                        if temp != 0 as libc::c_int {
                            ret = 3 as libc::c_int;
                            current_block = 3397862563670105393;
                            continue 's_219;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    a = 1 as libc::c_int;
                    while a <= na {
                        i = *tail.offset(a as isize);
                        j = *head.offset(a as isize);
                        lambda = *cost.offset(a as isize)
                            + (*pi.offset(i as isize) - *pi.offset(j as isize));
                        if lambda > 0 as libc::c_int
                            && *x.offset(a as isize) != *low.offset(a as isize)
                            || lambda < 0 as libc::c_int
                                && *x.offset(a as isize) != *cap.offset(a as isize)
                        {
                            ret = 3 as libc::c_int;
                            current_block = 3397862563670105393;
                            continue 's_219;
                        } else {
                            a += 1;
                            a;
                        }
                    }
                    ret = 0 as libc::c_int;
                    current_block = 3397862563670105393;
                }
            }
        }
    }
    glp_free(arc as *mut libc::c_void);
    glp_free(link as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
    return ret;
}

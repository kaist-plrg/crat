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
pub unsafe extern "C" fn _glp_ffalg(
    mut nv: libc::c_int,
    mut na: libc::c_int,
    mut tail: *const libc::c_int,
    mut head: *const libc::c_int,
    mut s: libc::c_int,
    mut t: libc::c_int,
    mut cap: *const libc::c_int,
    mut x: *mut libc::c_int,
    mut cut: *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut a: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos1: libc::c_int = 0;
    let mut pos2: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut arc: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut link: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    (nv >= 2 as libc::c_int
        || {
            glp_assert_(
                b"nv >= 2\0" as *const u8 as *const libc::c_char,
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (na >= 0 as libc::c_int
        || {
            glp_assert_(
                b"na >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= s && s <= nv
        || {
            glp_assert_(
                b"1 <= s && s <= nv\0" as *const u8 as *const libc::c_char,
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= t && t <= nv
        || {
            glp_assert_(
                b"1 <= t && t <= nv\0" as *const u8 as *const libc::c_char,
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (s != t
        || {
            glp_assert_(
                b"s != t\0" as *const u8 as *const libc::c_char,
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int,
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
                    b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                    86 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= j && j <= nv
            || {
                glp_assert_(
                    b"1 <= j && j <= nv\0" as *const u8 as *const libc::c_char,
                    b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                    87 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (i != j
            || {
                glp_assert_(
                    b"i != j\0" as *const u8 as *const libc::c_char,
                    b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                    88 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*cap.offset(a as isize) >= 0 as libc::c_int
            || {
                glp_assert_(
                    b"cap[a] >= 0\0" as *const u8 as *const libc::c_char,
                    b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int,
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
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*ptr.offset((nv + 1 as libc::c_int) as isize) == na + na + 1 as libc::c_int
        || {
            glp_assert_(
                b"ptr[nv+1] == na+na+1\0" as *const u8 as *const libc::c_char,
                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    a = 1 as libc::c_int;
    while a <= na {
        *x.offset(a as isize) = 0 as libc::c_int;
        a += 1;
        a;
    }
    '_loop: loop {
        i = 1 as libc::c_int;
        while i <= nv {
            *link.offset(i as isize) = 0 as libc::c_int;
            i += 1;
            i;
        }
        *link.offset(s as isize) = -(1 as libc::c_int);
        *list.offset(1 as libc::c_int as isize) = s;
        pos2 = 1 as libc::c_int;
        pos1 = pos2;
        's_175: loop {
            if !(pos1 <= pos2) {
                break '_loop;
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
                        current_block = 11048769245176032998;
                    } else if *x.offset(a as isize) == *cap.offset(a as isize) {
                        current_block = 11048769245176032998;
                    } else {
                        current_block = 14775119014532381840;
                    }
                } else if *head.offset(a as isize) == i {
                    j = *tail.offset(a as isize);
                    if *link.offset(j as isize) != 0 as libc::c_int {
                        current_block = 11048769245176032998;
                    } else if *x.offset(a as isize) == 0 as libc::c_int {
                        current_block = 11048769245176032998;
                    } else {
                        current_block = 14775119014532381840;
                    }
                } else {
                    (a != a
                        || {
                            glp_assert_(
                                b"a != a\0" as *const u8 as *const libc::c_char,
                                b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                                154 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    current_block = 14775119014532381840;
                }
                match current_block {
                    14775119014532381840 => {
                        *link.offset(j as isize) = a;
                        pos2 += 1;
                        *list.offset(pos2 as isize) = j;
                        if j == t {
                            break 's_175;
                        }
                    }
                    _ => {}
                }
                k += 1;
                k;
            }
        }
        delta = 0 as libc::c_int;
        j = t;
        while j != s {
            a = *link.offset(j as isize);
            if *head.offset(a as isize) == j {
                i = *tail.offset(a as isize);
                temp = *cap.offset(a as isize) - *x.offset(a as isize);
            } else if *tail.offset(a as isize) == j {
                i = *head.offset(a as isize);
                temp = *x.offset(a as isize);
            } else {
                (a != a
                    || {
                        glp_assert_(
                            b"a != a\0" as *const u8 as *const libc::c_char,
                            b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                            189 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if delta == 0 as libc::c_int || delta > temp {
                delta = temp;
            }
            j = i;
        }
        (delta > 0 as libc::c_int
            || {
                glp_assert_(
                    b"delta > 0\0" as *const u8 as *const libc::c_char,
                    b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                    192 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j = t;
        while j != s {
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
                            b"misc/ffalg.c\0" as *const u8 as *const libc::c_char,
                            208 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            j = i;
        }
    }
    if !cut.is_null() {
        i = 1 as libc::c_int;
        while i <= nv {
            *cut
                .offset(
                    i as isize,
                ) = (*link.offset(i as isize) != 0 as libc::c_int) as libc::c_int
                as libc::c_char;
            i += 1;
            i;
        }
    }
    glp_free(ptr as *mut libc::c_void);
    glp_free(arc as *mut libc::c_void);
    glp_free(link as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
}

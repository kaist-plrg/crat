use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_realloc(
        ptr: *mut libc::c_void,
        n: libc::c_int,
        size: libc::c_int,
    ) -> *mut libc::c_void;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVA {
    pub n_max: libc::c_int,
    pub n: libc::c_int,
    pub ptr: *mut libc::c_int,
    pub len: *mut libc::c_int,
    pub cap: *mut libc::c_int,
    pub size: libc::c_int,
    pub m_ptr: libc::c_int,
    pub r_ptr: libc::c_int,
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub prev: *mut libc::c_int,
    pub next: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub talky: libc::c_int,
}
pub unsafe extern "C" fn _glp_sva_create_area(
    mut n_max: libc::c_int,
    mut size: libc::c_int,
) -> *mut SVA {
    let mut sva: *mut SVA = 0 as *mut SVA;
    ((0 as libc::c_int) < n_max && n_max < 2147483647 as libc::c_int
        || {
            glp_assert_(
                b"0 < n_max && n_max < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((0 as libc::c_int) < size && size < 2147483647 as libc::c_int
        || {
            glp_assert_(
                b"0 < size && size < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                42 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    sva = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<SVA>() as libc::c_ulong as libc::c_int,
    ) as *mut SVA;
    (*sva).n_max = n_max;
    (*sva).n = 0 as libc::c_int;
    (*sva)
        .ptr = glp_alloc(
        1 as libc::c_int + n_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva)
        .len = glp_alloc(
        1 as libc::c_int + n_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva)
        .cap = glp_alloc(
        1 as libc::c_int + n_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva).size = size;
    (*sva).m_ptr = 1 as libc::c_int;
    (*sva).r_ptr = size + 1 as libc::c_int;
    (*sva).tail = 0 as libc::c_int;
    (*sva).head = (*sva).tail;
    (*sva)
        .prev = glp_alloc(
        1 as libc::c_int + n_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva)
        .next = glp_alloc(
        1 as libc::c_int + n_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva)
        .ind = glp_alloc(
        1 as libc::c_int + size,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva)
        .val = glp_alloc(
        1 as libc::c_int + size,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*sva).talky = 0 as libc::c_int;
    return sva;
}
pub unsafe extern "C" fn _glp_sva_alloc_vecs(
    mut sva: *mut SVA,
    mut nnn: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = (*sva).n;
    let mut n_max: libc::c_int = (*sva).n_max;
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut len: *mut libc::c_int = (*sva).len;
    let mut cap: *mut libc::c_int = (*sva).cap;
    let mut prev: *mut libc::c_int = (*sva).prev;
    let mut next: *mut libc::c_int = (*sva).next;
    let mut k: libc::c_int = 0;
    let mut new_n: libc::c_int = 0;
    if (*sva).talky != 0 {
        glp_printf(
            b"sva_alloc_vecs: nnn = %d\n\0" as *const u8 as *const libc::c_char,
            nnn,
        );
    }
    (nnn > 0 as libc::c_int
        || {
            glp_assert_(
                b"nnn > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    new_n = n + nnn;
    (new_n > n
        || {
            glp_assert_(
                b"new_n > n\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if n_max < new_n {
        while n_max < new_n {
            n_max += n_max;
            (n_max > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"n_max > 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        92 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        (*sva).n_max = n_max;
        ptr = glp_realloc(
            ptr as *mut libc::c_void,
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sva).ptr = ptr;
        len = glp_realloc(
            len as *mut libc::c_void,
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sva).len = len;
        cap = glp_realloc(
            cap as *mut libc::c_void,
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sva).cap = cap;
        prev = glp_realloc(
            prev as *mut libc::c_void,
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sva).prev = prev;
        next = glp_realloc(
            next as *mut libc::c_void,
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sva).next = next;
    }
    (*sva).n = new_n;
    k = n + 1 as libc::c_int;
    while k <= new_n {
        let ref mut fresh0 = *cap.offset(k as isize);
        *fresh0 = 0 as libc::c_int;
        let ref mut fresh1 = *len.offset(k as isize);
        *fresh1 = *fresh0;
        *ptr.offset(k as isize) = *fresh1;
        let ref mut fresh2 = *next.offset(k as isize);
        *fresh2 = -(1 as libc::c_int);
        *prev.offset(k as isize) = *fresh2;
        k += 1;
        k;
    }
    if (*sva).talky != 0 {
        glp_printf(
            b"now sva->n_max = %d, sva->n = %d\n\0" as *const u8 as *const libc::c_char,
            (*sva).n_max,
            (*sva).n,
        );
    }
    return n + 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_sva_resize_area(
    mut sva: *mut SVA,
    mut delta: libc::c_int,
) {
    let mut n: libc::c_int = (*sva).n;
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut size: libc::c_int = (*sva).size;
    let mut m_ptr: libc::c_int = (*sva).m_ptr;
    let mut r_ptr: libc::c_int = (*sva).r_ptr;
    let mut k: libc::c_int = 0;
    let mut r_size: libc::c_int = 0;
    if (*sva).talky != 0 {
        glp_printf(
            b"sva_resize_area: delta = %d\n\0" as *const u8 as *const libc::c_char,
            delta,
        );
    }
    (delta != 0 as libc::c_int
        || {
            glp_assert_(
                b"delta != 0\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    r_size = size - r_ptr + 1 as libc::c_int;
    if delta < 0 as libc::c_int {
        (delta >= m_ptr - r_ptr
            || {
                glp_assert_(
                    b"delta >= m_ptr - r_ptr\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    149 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*sva).r_ptr += delta;
        memmove(
            &mut *((*sva).ind).offset((*sva).r_ptr as isize) as *mut libc::c_int
                as *mut libc::c_void,
            &mut *((*sva).ind).offset(r_ptr as isize) as *mut libc::c_int
                as *const libc::c_void,
            (r_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memmove(
            &mut *((*sva).val).offset((*sva).r_ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *((*sva).val).offset(r_ptr as isize) as *mut libc::c_double
                as *const libc::c_void,
            (r_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    }
    (delta < 2147483647 as libc::c_int - (*sva).size
        || {
            glp_assert_(
                b"delta < INT_MAX - sva->size\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                157 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sva).size += delta;
    (*sva)
        .ind = glp_realloc(
        (*sva).ind as *mut libc::c_void,
        1 as libc::c_int + (*sva).size,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*sva)
        .val = glp_realloc(
        (*sva).val as *mut libc::c_void,
        1 as libc::c_int + (*sva).size,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    if delta > 0 as libc::c_int {
        (*sva).r_ptr += delta;
        memmove(
            &mut *((*sva).ind).offset((*sva).r_ptr as isize) as *mut libc::c_int
                as *mut libc::c_void,
            &mut *((*sva).ind).offset(r_ptr as isize) as *mut libc::c_int
                as *const libc::c_void,
            (r_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memmove(
            &mut *((*sva).val).offset((*sva).r_ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *((*sva).val).offset(r_ptr as isize) as *mut libc::c_double
                as *const libc::c_void,
            (r_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    }
    k = 1 as libc::c_int;
    while k <= n {
        if *ptr.offset(k as isize) >= r_ptr {
            *ptr.offset(k as isize) += delta;
        }
        k += 1;
        k;
    }
    if (*sva).talky != 0 {
        glp_printf(
            b"now sva->size = %d\n\0" as *const u8 as *const libc::c_char,
            (*sva).size,
        );
    }
}
pub unsafe extern "C" fn _glp_sva_defrag_area(mut sva: *mut SVA) {
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut len: *mut libc::c_int = (*sva).len;
    let mut cap: *mut libc::c_int = (*sva).cap;
    let mut prev: *mut libc::c_int = (*sva).prev;
    let mut next: *mut libc::c_int = (*sva).next;
    let mut ind: *mut libc::c_int = (*sva).ind;
    let mut val: *mut libc::c_double = (*sva).val;
    let mut k: libc::c_int = 0;
    let mut next_k: libc::c_int = 0;
    let mut ptr_k: libc::c_int = 0;
    let mut len_k: libc::c_int = 0;
    let mut m_ptr: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut tail: libc::c_int = 0;
    if (*sva).talky != 0 {
        glp_printf(b"sva_defrag_area:\n\0" as *const u8 as *const libc::c_char);
        glp_printf(
            b"before defragmenting = %d %d %d\n\0" as *const u8 as *const libc::c_char,
            (*sva).m_ptr - 1 as libc::c_int,
            (*sva).r_ptr - (*sva).m_ptr,
            (*sva).size + 1 as libc::c_int - (*sva).r_ptr,
        );
    }
    m_ptr = 1 as libc::c_int;
    tail = 0 as libc::c_int;
    head = tail;
    k = (*sva).head;
    while k != 0 as libc::c_int {
        next_k = *next.offset(k as isize);
        len_k = *len.offset(k as isize);
        if len_k == 0 as libc::c_int {
            let ref mut fresh3 = *cap.offset(k as isize);
            *fresh3 = 0 as libc::c_int;
            *ptr.offset(k as isize) = *fresh3;
            let ref mut fresh4 = *next.offset(k as isize);
            *fresh4 = -(1 as libc::c_int);
            *prev.offset(k as isize) = *fresh4;
        } else {
            ptr_k = *ptr.offset(k as isize);
            (m_ptr <= ptr_k
                || {
                    glp_assert_(
                        b"m_ptr <= ptr_k\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        223 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if m_ptr < ptr_k {
                memmove(
                    &mut *ind.offset(m_ptr as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut *ind.offset(ptr_k as isize) as *mut libc::c_int
                        as *const libc::c_void,
                    (len_k as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
                memmove(
                    &mut *val.offset(m_ptr as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                    &mut *val.offset(ptr_k as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    (len_k as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                );
                *ptr.offset(k as isize) = m_ptr;
            }
            *cap.offset(k as isize) = len_k;
            m_ptr += len_k;
            *prev.offset(k as isize) = tail;
            *next.offset(k as isize) = 0 as libc::c_int;
            if head == 0 as libc::c_int {
                head = k;
            } else {
                *next.offset(tail as isize) = k;
            }
            tail = k;
        }
        k = next_k;
    }
    (m_ptr <= (*sva).r_ptr
        || {
            glp_assert_(
                b"m_ptr <= sva->r_ptr\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sva).m_ptr = m_ptr;
    (*sva).head = head;
    (*sva).tail = tail;
    if (*sva).talky != 0 {
        glp_printf(
            b"after defragmenting = %d %d %d\n\0" as *const u8 as *const libc::c_char,
            (*sva).m_ptr - 1 as libc::c_int,
            (*sva).r_ptr - (*sva).m_ptr,
            (*sva).size + 1 as libc::c_int - (*sva).r_ptr,
        );
    }
}
pub unsafe extern "C" fn _glp_sva_more_space(
    mut sva: *mut SVA,
    mut m_size: libc::c_int,
) {
    let mut size: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    if (*sva).talky != 0 {
        glp_printf(
            b"sva_more_space: m_size = %d\n\0" as *const u8 as *const libc::c_char,
            m_size,
        );
    }
    (m_size > (*sva).r_ptr - (*sva).m_ptr
        || {
            glp_assert_(
                b"m_size > sva->r_ptr - sva->m_ptr\0" as *const u8
                    as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                281 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_sva_defrag_area(sva);
    if m_size < (*sva).m_ptr - 1 as libc::c_int {
        m_size = (*sva).m_ptr - 1 as libc::c_int;
    }
    if (*sva).r_ptr - (*sva).m_ptr < m_size {
        size = (*sva).size;
        loop {
            delta = size - (*sva).size;
            if (*sva).r_ptr - (*sva).m_ptr + delta >= m_size {
                break;
            }
            size += size;
            (size > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"size > 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        297 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        _glp_sva_resize_area(sva, delta);
        ((*sva).r_ptr - (*sva).m_ptr >= m_size
            || {
                glp_assert_(
                    b"sva->r_ptr - sva->m_ptr >= m_size\0" as *const u8
                        as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    300 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
}
pub unsafe extern "C" fn _glp_sva_enlarge_cap(
    mut sva: *mut SVA,
    mut k: libc::c_int,
    mut new_cap: libc::c_int,
    mut skip: libc::c_int,
) {
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut len: *mut libc::c_int = (*sva).len;
    let mut cap: *mut libc::c_int = (*sva).cap;
    let mut prev: *mut libc::c_int = (*sva).prev;
    let mut next: *mut libc::c_int = (*sva).next;
    let mut ind: *mut libc::c_int = (*sva).ind;
    let mut val: *mut libc::c_double = (*sva).val;
    (1 as libc::c_int <= k && k <= (*sva).n
        || {
            glp_assert_(
                b"1 <= k && k <= sva->n\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                334 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (new_cap > *cap.offset(k as isize)
        || {
            glp_assert_(
                b"new_cap > cap[k]\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                335 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*sva).r_ptr - (*sva).m_ptr >= new_cap
        || {
            glp_assert_(
                b"sva->r_ptr - sva->m_ptr >= new_cap\0" as *const u8
                    as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                337 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if *cap.offset(k as isize) == 0 as libc::c_int {
        (*ptr.offset(k as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"ptr[k] == 0\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    341 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*len.offset(k as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"len[k] == 0\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    342 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    } else {
        (*ptr.offset(k as isize) + *len.offset(k as isize) <= (*sva).m_ptr
            || {
                glp_assert_(
                    b"ptr[k] + len[k] <= sva->m_ptr\0" as *const u8
                        as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    346 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *len.offset(k as isize) > 0 as libc::c_int {
            memcpy(
                &mut *ind.offset((*sva).m_ptr as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                &mut *ind.offset(*ptr.offset(k as isize) as isize) as *mut libc::c_int
                    as *const libc::c_void,
                (*len.offset(k as isize) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            if skip == 0 {
                memcpy(
                    &mut *val.offset((*sva).m_ptr as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                    &mut *val.offset(*ptr.offset(k as isize) as isize)
                        as *mut libc::c_double as *const libc::c_void,
                    (*len.offset(k as isize) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                );
            }
        }
        if *prev.offset(k as isize) == 0 as libc::c_int {
            (*sva).head = *next.offset(k as isize);
        } else {
            *cap.offset(*prev.offset(k as isize) as isize) += *cap.offset(k as isize);
            *next.offset(*prev.offset(k as isize) as isize) = *next.offset(k as isize);
        }
        if *next.offset(k as isize) == 0 as libc::c_int {
            (*sva).tail = *prev.offset(k as isize);
        } else {
            *prev.offset(*next.offset(k as isize) as isize) = *prev.offset(k as isize);
        }
    }
    *ptr.offset(k as isize) = (*sva).m_ptr;
    *cap.offset(k as isize) = new_cap;
    *prev.offset(k as isize) = (*sva).tail;
    *next.offset(k as isize) = 0 as libc::c_int;
    if (*sva).head == 0 as libc::c_int {
        (*sva).head = k;
    } else {
        *next.offset((*sva).tail as isize) = k;
    }
    (*sva).tail = k;
    (*sva).m_ptr += new_cap;
    ((*sva).m_ptr <= (*sva).r_ptr
        || {
            glp_assert_(
                b"sva->m_ptr <= sva->r_ptr\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                382 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_sva_reserve_cap(
    mut sva: *mut SVA,
    mut k: libc::c_int,
    mut new_cap: libc::c_int,
) {
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut len: *mut libc::c_int = (*sva).len;
    let mut cap: *mut libc::c_int = (*sva).cap;
    (1 as libc::c_int <= k && k <= (*sva).n
        || {
            glp_assert_(
                b"1 <= k && k <= sva->n\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                405 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (new_cap > 0 as libc::c_int
        || {
            glp_assert_(
                b"new_cap > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*ptr.offset(k as isize) == 0 as libc::c_int
        && *len.offset(k as isize) == 0 as libc::c_int
        && *cap.offset(k as isize) == 0 as libc::c_int
        || {
            glp_assert_(
                b"ptr[k] == 0 && len[k] == 0 && cap[k] == 0\0" as *const u8
                    as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                407 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*sva).r_ptr - (*sva).m_ptr >= new_cap
        || {
            glp_assert_(
                b"sva->r_ptr - sva->m_ptr >= new_cap\0" as *const u8
                    as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *ptr.offset(k as isize) = (*sva).r_ptr - new_cap;
    *cap.offset(k as isize) = new_cap;
    (*sva).r_ptr -= new_cap;
}
pub unsafe extern "C" fn _glp_sva_make_static(mut sva: *mut SVA, mut k: libc::c_int) {
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut len: *mut libc::c_int = (*sva).len;
    let mut cap: *mut libc::c_int = (*sva).cap;
    let mut prev: *mut libc::c_int = (*sva).prev;
    let mut next: *mut libc::c_int = (*sva).next;
    let mut ind: *mut libc::c_int = (*sva).ind;
    let mut val: *mut libc::c_double = (*sva).val;
    let mut ptr_k: libc::c_int = 0;
    let mut len_k: libc::c_int = 0;
    (1 as libc::c_int <= k && k <= (*sva).n
        || {
            glp_assert_(
                b"1 <= k && k <= sva->n\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if *cap.offset(k as isize) == 0 as libc::c_int {
        (*ptr.offset(k as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"ptr[k] == 0\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    445 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*len.offset(k as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"len[k] == 0\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    446 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    } else {
        len_k = *len.offset(k as isize);
        ((*sva).r_ptr - (*sva).m_ptr >= len_k
            || {
                glp_assert_(
                    b"sva->r_ptr - sva->m_ptr >= len_k\0" as *const u8
                        as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    451 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *prev.offset(k as isize) == 0 as libc::c_int {
            (*sva).head = *next.offset(k as isize);
        } else {
            *cap.offset(*prev.offset(k as isize) as isize) += *cap.offset(k as isize);
            *next.offset(*prev.offset(k as isize) as isize) = *next.offset(k as isize);
        }
        if *next.offset(k as isize) == 0 as libc::c_int {
            (*sva).tail = *prev.offset(k as isize);
        } else {
            *prev.offset(*next.offset(k as isize) as isize) = *prev.offset(k as isize);
        }
        if len_k == 0 as libc::c_int {
            let ref mut fresh5 = *cap.offset(k as isize);
            *fresh5 = 0 as libc::c_int;
            *ptr.offset(k as isize) = *fresh5;
        } else {
            ptr_k = (*sva).r_ptr - len_k;
            memcpy(
                &mut *ind.offset(ptr_k as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                &mut *ind.offset(*ptr.offset(k as isize) as isize) as *mut libc::c_int
                    as *const libc::c_void,
                (len_k as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            memcpy(
                &mut *val.offset(ptr_k as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *val.offset(*ptr.offset(k as isize) as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (len_k as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            *ptr.offset(k as isize) = ptr_k;
            *cap.offset(k as isize) = len_k;
            (*sva).r_ptr -= len_k;
        }
    };
}
pub unsafe extern "C" fn _glp_sva_check_area(mut sva: *mut SVA) {
    let mut n_max: libc::c_int = (*sva).n_max;
    let mut n: libc::c_int = (*sva).n;
    let mut ptr: *mut libc::c_int = (*sva).ptr;
    let mut len: *mut libc::c_int = (*sva).len;
    let mut cap: *mut libc::c_int = (*sva).cap;
    let mut size: libc::c_int = (*sva).size;
    let mut m_ptr: libc::c_int = (*sva).m_ptr;
    let mut r_ptr: libc::c_int = (*sva).r_ptr;
    let mut head: libc::c_int = (*sva).head;
    let mut tail: libc::c_int = (*sva).tail;
    let mut prev: *mut libc::c_int = (*sva).prev;
    let mut next: *mut libc::c_int = (*sva).next;
    let mut k: libc::c_int = 0;
    (0 as libc::c_int <= n && n <= n_max
        || {
            glp_assert_(
                b"0 <= n && n <= n_max\0" as *const u8 as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                505 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= m_ptr && m_ptr <= r_ptr && r_ptr <= size + 1 as libc::c_int
        || {
            glp_assert_(
                b"1 <= m_ptr && m_ptr <= r_ptr && r_ptr <= size+1\0" as *const u8
                    as *const libc::c_char,
                b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                507 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = head;
    while k != 0 as libc::c_int {
        (1 as libc::c_int <= k && k <= n
            || {
                glp_assert_(
                    b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    511 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*cap.offset(k as isize) > 0 as libc::c_int
            || {
                glp_assert_(
                    b"cap[k] > 0\0" as *const u8 as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    512 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (0 as libc::c_int <= *len.offset(k as isize)
            && *len.offset(k as isize) <= *cap.offset(k as isize)
            || {
                glp_assert_(
                    b"0 <= len[k] && len[k] <= cap[k]\0" as *const u8
                        as *const libc::c_char,
                    b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                    513 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *prev.offset(k as isize) == 0 as libc::c_int {
            (k == head
                || {
                    glp_assert_(
                        b"k == head\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        515 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            (1 as libc::c_int <= *prev.offset(k as isize)
                && *prev.offset(k as isize) <= n
                || {
                    glp_assert_(
                        b"1 <= prev[k] && prev[k] <= n\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        517 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*next.offset(*prev.offset(k as isize) as isize) == k
                || {
                    glp_assert_(
                        b"next[prev[k]] == k\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        518 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if *next.offset(k as isize) == 0 as libc::c_int {
            (k == tail
                || {
                    glp_assert_(
                        b"k == tail\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        521 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*ptr.offset(k as isize) + *cap.offset(k as isize) <= m_ptr
                || {
                    glp_assert_(
                        b"ptr[k] + cap[k] <= m_ptr\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        522 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            (1 as libc::c_int <= *next.offset(k as isize)
                && *next.offset(k as isize) <= n
                || {
                    glp_assert_(
                        b"1 <= next[k] && next[k] <= n\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        525 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*prev.offset(*next.offset(k as isize) as isize) == k
                || {
                    glp_assert_(
                        b"prev[next[k]] == k\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        526 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*ptr.offset(k as isize) + *cap.offset(k as isize)
                <= *ptr.offset(*next.offset(k as isize) as isize)
                || {
                    glp_assert_(
                        b"ptr[k] + cap[k] <= ptr[next[k]]\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        527 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        *cap.offset(k as isize) = -*cap.offset(k as isize);
        k = *next.offset(k as isize);
    }
    k = 1 as libc::c_int;
    while k <= n {
        if *cap.offset(k as isize) < 0 as libc::c_int {
            *cap.offset(k as isize) = -*cap.offset(k as isize);
        } else if *cap.offset(k as isize) == 0 as libc::c_int {
            (*ptr.offset(k as isize) == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"ptr[k] == 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        540 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*len.offset(k as isize) == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"len[k] == 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        541 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            (0 as libc::c_int <= *len.offset(k as isize)
                && *len.offset(k as isize) <= *cap.offset(k as isize)
                || {
                    glp_assert_(
                        b"0 <= len[k] && len[k] <= cap[k]\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        545 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (r_ptr <= *ptr.offset(k as isize)
                && *ptr.offset(k as isize) + *cap.offset(k as isize)
                    <= size + 1 as libc::c_int
                || {
                    glp_assert_(
                        b"r_ptr <= ptr[k] && ptr[k] + cap[k] <= size+1\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/sva.c\0" as *const u8 as *const libc::c_char,
                        546 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_sva_delete_area(mut sva: *mut SVA) {
    glp_free((*sva).ptr as *mut libc::c_void);
    glp_free((*sva).len as *mut libc::c_void);
    glp_free((*sva).cap as *mut libc::c_void);
    glp_free((*sva).prev as *mut libc::c_void);
    glp_free((*sva).next as *mut libc::c_void);
    glp_free((*sva).ind as *mut libc::c_void);
    glp_free((*sva).val as *mut libc::c_void);
    glp_free(sva as *mut libc::c_void);
}

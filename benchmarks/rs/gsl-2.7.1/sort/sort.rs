use ::libc;
pub type size_t = libc::c_ulong;
pub type gsl_comparison_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn swap(
    mut base: *mut libc::c_void,
    mut size: size_t,
    mut i: size_t,
    mut j: size_t,
) {
    let mut a: *mut libc::c_char = (base as *mut libc::c_char)
        .offset(size.wrapping_mul(i) as isize);
    let mut b: *mut libc::c_char = (base as *mut libc::c_char)
        .offset(size.wrapping_mul(j) as isize);
    let mut s: size_t = size;
    if i == j {
        return;
    }
    loop {
        let mut tmp: libc::c_char = *a;
        let fresh0 = a;
        a = a.offset(1);
        *fresh0 = *b;
        let fresh1 = b;
        b = b.offset(1);
        *fresh1 = tmp;
        s = s.wrapping_sub(1);
        if !(s > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn downheap(
    mut data: *mut libc::c_void,
    size: size_t,
    N: size_t,
    mut k: size_t,
    mut compare: gsl_comparison_fn_t,
) {
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && compare
                .unwrap()(
                (data as *mut libc::c_char).offset(size.wrapping_mul(j) as isize)
                    as *const libc::c_void,
                (data as *mut libc::c_char)
                    .offset(
                        size
                            .wrapping_mul(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) as *const libc::c_void,
            ) < 0 as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(compare
            .unwrap()(
            (data as *mut libc::c_char).offset(size.wrapping_mul(k) as isize)
                as *const libc::c_void,
            (data as *mut libc::c_char).offset(size.wrapping_mul(j) as isize)
                as *const libc::c_void,
        ) < 0 as libc::c_int)
        {
            break;
        }
        swap(data, size, j, k);
        k = j;
    }
}
pub unsafe extern "C" fn gsl_heapsort(
    mut data: *mut libc::c_void,
    mut count: size_t,
    mut size: size_t,
    mut compare: gsl_comparison_fn_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if count == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = count.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        downheap(data, size, N, k, compare);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        swap(data, size, 0 as libc::c_int as size_t, N);
        N = N.wrapping_sub(1);
        N;
        downheap(data, size, N, 0 as libc::c_int as size_t, compare);
    }
}

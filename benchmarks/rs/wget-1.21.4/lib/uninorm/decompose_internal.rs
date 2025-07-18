use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucs4_with_ccc {
    pub code: ucs4_t,
    pub ccc: libc::c_int,
}
unsafe extern "C" fn merge(
    mut src1: *const ucs4_with_ccc,
    mut n1: size_t,
    mut src2: *const ucs4_with_ccc,
    mut n2: size_t,
    mut dst: *mut ucs4_with_ccc,
) {
    loop {
        if (*src1).ccc - (*src2).ccc <= 0 as libc::c_int {
            let fresh0 = src1;
            src1 = src1.offset(1);
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = *fresh0;
            n1 = n1.wrapping_sub(1);
            n1;
            if n1 == 0 as libc::c_int as libc::c_ulong {
                break;
            }
        } else {
            let fresh2 = src2;
            src2 = src2.offset(1);
            let fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = *fresh2;
            n2 = n2.wrapping_sub(1);
            n2;
            if n2 == 0 as libc::c_int as libc::c_ulong {
                break;
            }
        }
    }
    if n1 > 0 as libc::c_int as libc::c_ulong {
        if dst != src1 as *mut ucs4_with_ccc {
            loop {
                let fresh4 = src1;
                src1 = src1.offset(1);
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = *fresh4;
                n1 = n1.wrapping_sub(1);
                n1;
                if !(n1 > 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
    } else if dst != src2 as *mut ucs4_with_ccc {
        loop {
            let fresh6 = src2;
            src2 = src2.offset(1);
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = *fresh6;
            n2 = n2.wrapping_sub(1);
            n2;
            if !(n2 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
}
unsafe extern "C" fn gl_uninorm_decompose_merge_sort_fromto(
    mut src: *const ucs4_with_ccc,
    mut dst: *mut ucs4_with_ccc,
    mut n: size_t,
    mut tmp: *mut ucs4_with_ccc,
) {
    match n {
        0 => return,
        1 => {
            *dst
                .offset(
                    0 as libc::c_int as isize,
                ) = *src.offset(0 as libc::c_int as isize);
            return;
        }
        2 => {
            if (*src.offset(0 as libc::c_int as isize)).ccc
                - (*src.offset(1 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(0 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
            } else {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(0 as libc::c_int as isize);
            }
        }
        3 => {
            if (*src.offset(0 as libc::c_int as isize)).ccc
                - (*src.offset(1 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                if (*src.offset(1 as libc::c_int as isize)).ccc
                    - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int
                {
                    *dst
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dst
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    *dst
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                } else if (*src.offset(0 as libc::c_int as isize)).ccc
                    - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int
                {
                    *dst
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dst
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                    *dst
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                } else {
                    *dst
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                    *dst
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dst
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                }
            } else if (*src.offset(0 as libc::c_int as isize)).ccc
                - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(0 as libc::c_int as isize);
                *dst
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *src.offset(2 as libc::c_int as isize);
            } else if (*src.offset(1 as libc::c_int as isize)).ccc
                - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(2 as libc::c_int as isize);
                *dst
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *src.offset(0 as libc::c_int as isize);
            } else {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(2 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *dst
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *src.offset(0 as libc::c_int as isize);
            }
        }
        _ => {
            let mut n1: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
            let mut n2: size_t = n
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
            gl_uninorm_decompose_merge_sort_fromto(
                src.offset(n1 as isize),
                dst.offset(n1 as isize),
                n2,
                tmp,
            );
            gl_uninorm_decompose_merge_sort_fromto(src, tmp, n1, dst);
            merge(tmp, n1, dst.offset(n1 as isize), n2, dst);
        }
    };
}
pub unsafe extern "C" fn gl_uninorm_decompose_merge_sort_inplace(
    mut src: *mut ucs4_with_ccc,
    mut n: size_t,
    mut tmp: *mut ucs4_with_ccc,
) {
    match n {
        0 | 1 => return,
        2 => {
            if !((*src.offset(0 as libc::c_int as isize)).ccc
                - (*src.offset(1 as libc::c_int as isize)).ccc <= 0 as libc::c_int)
            {
                let mut t: ucs4_with_ccc = *src.offset(0 as libc::c_int as isize);
                *src
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *src.offset(1 as libc::c_int as isize) = t;
            }
        }
        3 => {
            if (*src.offset(0 as libc::c_int as isize)).ccc
                - (*src.offset(1 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                if !((*src.offset(1 as libc::c_int as isize)).ccc
                    - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int)
                {
                    if (*src.offset(0 as libc::c_int as isize)).ccc
                        - (*src.offset(2 as libc::c_int as isize)).ccc
                        <= 0 as libc::c_int
                    {
                        let mut t_0: ucs4_with_ccc = *src
                            .offset(1 as libc::c_int as isize);
                        *src
                            .offset(
                                1 as libc::c_int as isize,
                            ) = *src.offset(2 as libc::c_int as isize);
                        *src.offset(2 as libc::c_int as isize) = t_0;
                    } else {
                        let mut t_1: ucs4_with_ccc = *src
                            .offset(0 as libc::c_int as isize);
                        *src
                            .offset(
                                0 as libc::c_int as isize,
                            ) = *src.offset(2 as libc::c_int as isize);
                        *src
                            .offset(
                                2 as libc::c_int as isize,
                            ) = *src.offset(1 as libc::c_int as isize);
                        *src.offset(1 as libc::c_int as isize) = t_1;
                    }
                }
            } else if (*src.offset(0 as libc::c_int as isize)).ccc
                - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                let mut t_2: ucs4_with_ccc = *src.offset(0 as libc::c_int as isize);
                *src
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *src.offset(1 as libc::c_int as isize) = t_2;
            } else if (*src.offset(1 as libc::c_int as isize)).ccc
                - (*src.offset(2 as libc::c_int as isize)).ccc <= 0 as libc::c_int
            {
                let mut t_3: ucs4_with_ccc = *src.offset(0 as libc::c_int as isize);
                *src
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *src
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(2 as libc::c_int as isize);
                *src.offset(2 as libc::c_int as isize) = t_3;
            } else {
                let mut t_4: ucs4_with_ccc = *src.offset(0 as libc::c_int as isize);
                *src
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(2 as libc::c_int as isize);
                *src.offset(2 as libc::c_int as isize) = t_4;
            }
        }
        _ => {
            let mut n1: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
            let mut n2: size_t = n
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
            gl_uninorm_decompose_merge_sort_inplace(src.offset(n1 as isize), n2, tmp);
            gl_uninorm_decompose_merge_sort_fromto(
                src,
                tmp,
                n1,
                tmp.offset(n1 as isize),
            );
            merge(tmp, n1, src.offset(n1 as isize), n2, src);
        }
    };
}

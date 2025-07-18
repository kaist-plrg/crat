use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RS {
    pub mm: libc::c_int,
    pub nn: libc::c_int,
    pub alpha_to: *mut data_t,
    pub index_of: *mut data_t,
    pub genpoly: *mut data_t,
    pub nroots: libc::c_int,
    pub fcr: libc::c_int,
    pub prim: libc::c_int,
    pub iprim: libc::c_int,
    pub pad: libc::c_int,
    pub gfpoly: libc::c_int,
}
pub type data_t = libc::c_uchar;
pub type RS = _RS;
#[inline]
unsafe extern "C" fn modnn(mut rs: *mut RS, mut x: libc::c_int) -> libc::c_int {
    while x >= (*rs).nn {
        x -= (*rs).nn;
        x = (x >> (*rs).mm) + (x & (*rs).nn);
    }
    return x;
}
unsafe extern "C" fn init_rs_char(
    mut symsize: libc::c_int,
    mut gfpoly: libc::c_int,
    mut fcr: libc::c_int,
    mut prim: libc::c_int,
    mut nroots: libc::c_int,
    mut pad: libc::c_int,
) -> *mut RS {
    let mut rs: *mut RS = 0 as *mut RS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sr: libc::c_int = 0;
    let mut root: libc::c_int = 0;
    let mut iprim: libc::c_int = 0;
    rs = 0 as *mut RS;
    if !(symsize < 0 as libc::c_int
        || symsize
            > (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<data_t>() as libc::c_ulong)
                as libc::c_int)
    {
        if !(fcr < 0 as libc::c_int || fcr >= (1 as libc::c_int) << symsize) {
            if !(prim <= 0 as libc::c_int || prim >= (1 as libc::c_int) << symsize) {
                if !(nroots < 0 as libc::c_int
                    || nroots >= (1 as libc::c_int) << symsize)
                {
                    if !(pad < 0 as libc::c_int
                        || pad
                            >= ((1 as libc::c_int) << symsize) - 1 as libc::c_int
                                - nroots)
                    {
                        rs = calloc(
                            1 as libc::c_int as libc::c_ulong,
                            ::std::mem::size_of::<RS>() as libc::c_ulong,
                        ) as *mut RS;
                        if !rs.is_null() {
                            (*rs).mm = symsize;
                            (*rs)
                                .nn = ((1 as libc::c_int) << symsize) - 1 as libc::c_int;
                            (*rs).pad = pad;
                            (*rs)
                                .alpha_to = malloc(
                                (::std::mem::size_of::<data_t>() as libc::c_ulong)
                                    .wrapping_mul(
                                        ((*rs).nn + 1 as libc::c_int) as libc::c_ulong,
                                    ),
                            ) as *mut data_t;
                            if ((*rs).alpha_to).is_null() {
                                free(rs as *mut libc::c_void);
                                rs = 0 as *mut RS;
                            } else {
                                (*rs)
                                    .index_of = malloc(
                                    (::std::mem::size_of::<data_t>() as libc::c_ulong)
                                        .wrapping_mul(
                                            ((*rs).nn + 1 as libc::c_int) as libc::c_ulong,
                                        ),
                                ) as *mut data_t;
                                if ((*rs).index_of).is_null() {
                                    free((*rs).alpha_to as *mut libc::c_void);
                                    free(rs as *mut libc::c_void);
                                    rs = 0 as *mut RS;
                                } else {
                                    *((*rs).index_of)
                                        .offset(0 as libc::c_int as isize) = (*rs).nn as data_t;
                                    *((*rs).alpha_to)
                                        .offset((*rs).nn as isize) = 0 as libc::c_int as data_t;
                                    sr = 1 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < (*rs).nn {
                                        *((*rs).index_of).offset(sr as isize) = i as data_t;
                                        *((*rs).alpha_to).offset(i as isize) = sr as data_t;
                                        sr <<= 1 as libc::c_int;
                                        if sr & (1 as libc::c_int) << symsize != 0 {
                                            sr ^= gfpoly;
                                        }
                                        sr &= (*rs).nn;
                                        i += 1;
                                        i;
                                    }
                                    if sr != 1 as libc::c_int {
                                        free((*rs).alpha_to as *mut libc::c_void);
                                        free((*rs).index_of as *mut libc::c_void);
                                        free(rs as *mut libc::c_void);
                                        rs = 0 as *mut RS;
                                    } else {
                                        (*rs)
                                            .genpoly = malloc(
                                            (::std::mem::size_of::<data_t>() as libc::c_ulong)
                                                .wrapping_mul((nroots + 1 as libc::c_int) as libc::c_ulong),
                                        ) as *mut data_t;
                                        if ((*rs).genpoly).is_null() {
                                            free((*rs).alpha_to as *mut libc::c_void);
                                            free((*rs).index_of as *mut libc::c_void);
                                            free(rs as *mut libc::c_void);
                                            rs = 0 as *mut RS;
                                        } else {
                                            (*rs).fcr = fcr;
                                            (*rs).prim = prim;
                                            (*rs).nroots = nroots;
                                            (*rs).gfpoly = gfpoly;
                                            iprim = 1 as libc::c_int;
                                            while iprim % prim != 0 as libc::c_int {
                                                iprim += (*rs).nn;
                                            }
                                            (*rs).iprim = iprim / prim;
                                            *((*rs).genpoly)
                                                .offset(
                                                    0 as libc::c_int as isize,
                                                ) = 1 as libc::c_int as data_t;
                                            i = 0 as libc::c_int;
                                            root = fcr * prim;
                                            while i < nroots {
                                                *((*rs).genpoly)
                                                    .offset(
                                                        (i + 1 as libc::c_int) as isize,
                                                    ) = 1 as libc::c_int as data_t;
                                                j = i;
                                                while j > 0 as libc::c_int {
                                                    if *((*rs).genpoly).offset(j as isize) as libc::c_int
                                                        != 0 as libc::c_int
                                                    {
                                                        *((*rs).genpoly)
                                                            .offset(
                                                                j as isize,
                                                            ) = (*((*rs).genpoly)
                                                            .offset((j - 1 as libc::c_int) as isize) as libc::c_int
                                                            ^ *((*rs).alpha_to)
                                                                .offset(
                                                                    modnn(
                                                                        rs,
                                                                        *((*rs).index_of)
                                                                            .offset(*((*rs).genpoly).offset(j as isize) as isize)
                                                                            as libc::c_int + root,
                                                                    ) as isize,
                                                                ) as libc::c_int) as data_t;
                                                    } else {
                                                        *((*rs).genpoly)
                                                            .offset(
                                                                j as isize,
                                                            ) = *((*rs).genpoly)
                                                            .offset((j - 1 as libc::c_int) as isize);
                                                    }
                                                    j -= 1;
                                                    j;
                                                }
                                                *((*rs).genpoly)
                                                    .offset(
                                                        0 as libc::c_int as isize,
                                                    ) = *((*rs).alpha_to)
                                                    .offset(
                                                        modnn(
                                                            rs,
                                                            *((*rs).index_of)
                                                                .offset(
                                                                    *((*rs).genpoly).offset(0 as libc::c_int as isize) as isize,
                                                                ) as libc::c_int + root,
                                                        ) as isize,
                                                    );
                                                i += 1;
                                                i;
                                                root += prim;
                                            }
                                            i = 0 as libc::c_int;
                                            while i <= nroots {
                                                *((*rs).genpoly)
                                                    .offset(
                                                        i as isize,
                                                    ) = *((*rs).index_of)
                                                    .offset(*((*rs).genpoly).offset(i as isize) as isize);
                                                i += 1;
                                                i;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return rs;
}
pub unsafe extern "C" fn init_rs(
    mut symsize: libc::c_int,
    mut gfpoly: libc::c_int,
    mut fcr: libc::c_int,
    mut prim: libc::c_int,
    mut nroots: libc::c_int,
    mut pad: libc::c_int,
) -> *mut RS {
    return init_rs_char(symsize, gfpoly, fcr, prim, nroots, pad);
}
pub unsafe extern "C" fn free_rs_char(mut rs: *mut RS) {
    free((*rs).alpha_to as *mut libc::c_void);
    free((*rs).index_of as *mut libc::c_void);
    free((*rs).genpoly as *mut libc::c_void);
    free(rs as *mut libc::c_void);
}
pub unsafe extern "C" fn encode_rs_char(
    mut rs: *mut RS,
    mut data: *const data_t,
    mut parity: *mut data_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut feedback: data_t = 0;
    memset(
        parity as *mut libc::c_void,
        0 as libc::c_int,
        ((*rs).nroots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<data_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*rs).nn - (*rs).nroots - (*rs).pad {
        feedback = *((*rs).index_of)
            .offset(
                (*data.offset(i as isize) as libc::c_int
                    ^ *parity.offset(0 as libc::c_int as isize) as libc::c_int) as isize,
            );
        if feedback as libc::c_int != (*rs).nn {
            j = 1 as libc::c_int;
            while j < (*rs).nroots {
                let ref mut fresh0 = *parity.offset(j as isize);
                *fresh0 = (*fresh0 as libc::c_int
                    ^ *((*rs).alpha_to)
                        .offset(
                            modnn(
                                rs,
                                feedback as libc::c_int
                                    + *((*rs).genpoly).offset(((*rs).nroots - j) as isize)
                                        as libc::c_int,
                            ) as isize,
                        ) as libc::c_int) as data_t;
                j += 1;
                j;
            }
        }
        memmove(
            &mut *parity.offset(0 as libc::c_int as isize) as *mut data_t
                as *mut libc::c_void,
            &mut *parity.offset(1 as libc::c_int as isize) as *mut data_t
                as *const libc::c_void,
            (::std::mem::size_of::<data_t>() as libc::c_ulong)
                .wrapping_mul(((*rs).nroots - 1 as libc::c_int) as libc::c_ulong),
        );
        if feedback as libc::c_int != (*rs).nn {
            *parity
                .offset(
                    ((*rs).nroots - 1 as libc::c_int) as isize,
                ) = *((*rs).alpha_to)
                .offset(
                    modnn(
                        rs,
                        feedback as libc::c_int
                            + *((*rs).genpoly).offset(0 as libc::c_int as isize)
                                as libc::c_int,
                    ) as isize,
                );
        } else {
            *parity
                .offset(
                    ((*rs).nroots - 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as data_t;
        }
        i += 1;
        i;
    }
}

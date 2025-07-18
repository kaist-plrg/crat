use ::libc;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_accum {
    pub size: Option::<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option::<unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int>,
    pub insert: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub delete_oldest: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
}
pub type medacc_type_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medacc_state_t {
    pub n: libc::c_int,
    pub idx: libc::c_int,
    pub ct: libc::c_int,
    pub data: *mut medacc_type_t,
    pub pos: *mut libc::c_int,
    pub heap: *mut libc::c_int,
}
unsafe extern "C" fn medacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<medacc_state_t>() as libc::c_ulong) as size_t
        as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            n.wrapping_mul(::std::mem::size_of::<medacc_type_t>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(n)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as size_t as size_t;
    return size;
}
unsafe extern "C" fn medacc_init(
    n: size_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut medacc_state_t = vstate as *mut medacc_state_t;
    let mut k: libc::c_int = n as libc::c_int;
    (*state).n = n as libc::c_int;
    (*state).ct = 0 as libc::c_int;
    (*state).idx = 0 as libc::c_int;
    (*state)
        .data = (vstate as *mut libc::c_uchar)
        .offset(::std::mem::size_of::<medacc_state_t>() as libc::c_ulong as isize)
        as *mut medacc_type_t;
    (*state)
        .pos = ((*state).data as *mut libc::c_uchar)
        .offset(
            n.wrapping_mul(::std::mem::size_of::<medacc_type_t>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_int;
    (*state)
        .heap = ((*state).pos)
        .offset(n as isize)
        .offset(n.wrapping_div(2 as libc::c_int as libc::c_ulong) as isize);
    loop {
        let fresh0 = k;
        k = k - 1;
        if !(fresh0 != 0) {
            break;
        }
        *((*state).pos)
            .offset(
                k as isize,
            ) = (k + 1 as libc::c_int) / 2 as libc::c_int
            * (if k & 1 as libc::c_int != 0 {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            });
        *((*state).heap).offset(*((*state).pos).offset(k as isize) as isize) = k;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn medacc_insert(
    x: medacc_type_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut medacc_state_t = vstate as *mut medacc_state_t;
    let mut isNew: libc::c_int = ((*state).ct < (*state).n) as libc::c_int;
    let mut p: libc::c_int = *((*state).pos).offset((*state).idx as isize);
    let mut old: medacc_type_t = *((*state).data).offset((*state).idx as isize);
    *((*state).data).offset((*state).idx as isize) = x;
    (*state).idx = ((*state).idx + 1 as libc::c_int) % (*state).n;
    (*state).ct += isNew;
    if p > 0 as libc::c_int {
        if isNew == 0 && old < x {
            minSortDown(state, p * 2 as libc::c_int);
        } else if minSortUp(state, p) != 0 {
            maxSortDown(state, -(1 as libc::c_int));
        }
    } else if p < 0 as libc::c_int {
        if isNew == 0 && x < old {
            maxSortDown(state, p * 2 as libc::c_int);
        } else if maxSortUp(state, p) != 0 {
            minSortDown(state, 1 as libc::c_int);
        }
    } else {
        if (*state).ct / 2 as libc::c_int != 0 {
            maxSortDown(state, -(1 as libc::c_int));
        }
        if ((*state).ct - 1 as libc::c_int) / 2 as libc::c_int != 0 {
            minSortDown(state, 1 as libc::c_int);
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn medacc_get(
    mut params: *mut libc::c_void,
    mut result: *mut medacc_type_t,
    mut vstate: *const libc::c_void,
) -> libc::c_int {
    let mut state: *const medacc_state_t = vstate as *const medacc_state_t;
    let mut median: medacc_type_t = *((*state).data)
        .offset(*((*state).heap).offset(0 as libc::c_int as isize) as isize);
    if (*state).ct & 1 as libc::c_int == 0 as libc::c_int {
        median = (median
            + *((*state).data)
                .offset(*((*state).heap).offset(-(1 as libc::c_int) as isize) as isize))
            / 2 as libc::c_int as libc::c_double;
    }
    *result = median;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn mmless(
    mut state: *const medacc_state_t,
    i: libc::c_int,
    j: libc::c_int,
) -> libc::c_int {
    return (*((*state).data).offset(*((*state).heap).offset(i as isize) as isize)
        < *((*state).data).offset(*((*state).heap).offset(j as isize) as isize))
        as libc::c_int;
}
unsafe extern "C" fn mmexchange(
    mut state: *mut medacc_state_t,
    i: libc::c_int,
    j: libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = *((*state).heap).offset(i as isize);
    *((*state).heap).offset(i as isize) = *((*state).heap).offset(j as isize);
    *((*state).heap).offset(j as isize) = t;
    *((*state).pos).offset(*((*state).heap).offset(i as isize) as isize) = i;
    *((*state).pos).offset(*((*state).heap).offset(j as isize) as isize) = j;
    return 1 as libc::c_int;
}
unsafe extern "C" fn mmCmpExch(
    mut state: *mut medacc_state_t,
    i: libc::c_int,
    j: libc::c_int,
) -> libc::c_int {
    return (mmless(state, i, j) != 0 && mmexchange(state, i, j) != 0) as libc::c_int;
}
unsafe extern "C" fn minSortDown(mut state: *mut medacc_state_t, mut i: libc::c_int) {
    while i <= ((*state).ct - 1 as libc::c_int) / 2 as libc::c_int {
        if i > 1 as libc::c_int
            && i < ((*state).ct - 1 as libc::c_int) / 2 as libc::c_int
            && mmless(state, i + 1 as libc::c_int, i) != 0
        {
            i += 1;
            i;
        }
        if mmCmpExch(state, i, i / 2 as libc::c_int) == 0 {
            break;
        }
        i *= 2 as libc::c_int;
    }
}
unsafe extern "C" fn maxSortDown(mut state: *mut medacc_state_t, mut i: libc::c_int) {
    while i >= -((*state).ct / 2 as libc::c_int) {
        if i < -(1 as libc::c_int) && i > -((*state).ct / 2 as libc::c_int)
            && mmless(state, i, i - 1 as libc::c_int) != 0
        {
            i -= 1;
            i;
        }
        if mmCmpExch(state, i / 2 as libc::c_int, i) == 0 {
            break;
        }
        i *= 2 as libc::c_int;
    }
}
unsafe extern "C" fn minSortUp(
    mut state: *mut medacc_state_t,
    mut i: libc::c_int,
) -> libc::c_int {
    while i > 0 as libc::c_int && mmCmpExch(state, i, i / 2 as libc::c_int) != 0 {
        i /= 2 as libc::c_int;
    }
    return (i == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn maxSortUp(
    mut state: *mut medacc_state_t,
    mut i: libc::c_int,
) -> libc::c_int {
    while i < 0 as libc::c_int && mmCmpExch(state, i / 2 as libc::c_int, i) != 0 {
        i /= 2 as libc::c_int;
    }
    return (i == 0 as libc::c_int) as libc::c_int;
}
static mut median_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(medacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                medacc_init
                    as unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int,
            ),
            insert: Some(
                medacc_insert
                    as unsafe extern "C" fn(
                        medacc_type_t,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            delete_oldest: None,
            get: Some(
                medacc_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut medacc_type_t,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub static mut gsl_movstat_accum_median: *const gsl_movstat_accum = unsafe {
    &median_accum_type as *const gsl_movstat_accum
};

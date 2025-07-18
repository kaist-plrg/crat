use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
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
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ulong {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ushort {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_short {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uchar {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_char {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
#[inline]
unsafe extern "C" fn index_ulong_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_ulong,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_long_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_long,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_long_double_downheap(
    mut p: *mut size_t,
    mut data: *const f128::f128,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_float_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_uint_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_uint,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_int_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_int,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(*data.offset(pki.wrapping_mul(stride) as isize)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize))
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_ushort_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_ushort,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((*data.offset(pki.wrapping_mul(stride) as isize) as libc::c_int)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_short_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_short,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((*data.offset(pki.wrapping_mul(stride) as isize) as libc::c_int)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_uchar_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_uchar,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((*data.offset(pki.wrapping_mul(stride) as isize) as libc::c_int)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[inline]
unsafe extern "C" fn index_char_downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_char,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
                < *data
                    .offset(
                        (*p
                            .offset(
                                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((*data.offset(pki.wrapping_mul(stride) as isize) as libc::c_int)
            < *data.offset((*p.offset(j as isize)).wrapping_mul(stride) as isize)
                as libc::c_int)
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
pub unsafe extern "C" fn gsl_sort_long_index(
    mut p: *mut size_t,
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_long_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_long_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_ulong_index(
    mut p: *mut size_t,
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_ulong_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_ulong_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_float_index(
    mut p: *mut size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_float_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_float_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_char_index(
    mut p: *mut size_t,
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_char_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_char_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_uint_index(
    mut p: *mut size_t,
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_uint_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_uint_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_int_index(
    mut p: *mut size_t,
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_int_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_int_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_index(
    mut p: *mut size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_short_index(
    mut p: *mut size_t,
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_short_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_short_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_uchar_index(
    mut p: *mut size_t,
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_uchar_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_uchar_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_ushort_index(
    mut p: *mut size_t,
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_ushort_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_ushort_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_long_double_index(
    mut p: *mut size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        index_long_double_downheap(p, data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: size_t = *p.offset(0 as libc::c_int as isize);
        *p.offset(0 as libc::c_int as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        index_long_double_downheap(p, data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_vector_short_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_short_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_uint_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_uint_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_float_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_float_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_ulong_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_ulong_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_long_double_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_long_double_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_char_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_char_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_uchar_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_uchar_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_long_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_long_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_ushort_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_ushort_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_int_index(
    mut permutation: *mut gsl_permutation,
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    if (*permutation).size != (*v).size {
        gsl_error(
            b"permutation and vector lengths are not equal\0" as *const u8
                as *const libc::c_char,
            b"./sortvecind_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_sort_int_index((*permutation).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}

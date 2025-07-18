use ::libc;
pub type size_t = libc::c_ulong;
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
#[inline]
unsafe extern "C" fn my_char_downheap(
    mut data: *mut libc::c_char,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_char = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v as libc::c_int)
            < *data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
        {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_downheap(
    mut data: *mut libc::c_double,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_double = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_ulong_downheap(
    mut data: *mut libc::c_ulong,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_ulong = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_uint_downheap(
    mut data: *mut libc::c_uint,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_uint = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_short_downheap(
    mut data: *mut libc::c_short,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_short = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v as libc::c_int)
            < *data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
        {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_ushort_downheap(
    mut data: *mut libc::c_ushort,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_ushort = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v as libc::c_int)
            < *data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
        {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_int_downheap(
    mut data: *mut libc::c_int,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_int = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_long_double_downheap(
    mut data: *mut f128::f128,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: f128::f128 = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_long_downheap(
    mut data: *mut libc::c_long,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_long = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_uchar_downheap(
    mut data: *mut libc::c_uchar,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_uchar = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v as libc::c_int)
            < *data.offset(j.wrapping_mul(stride) as isize) as libc::c_int)
        {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_float_downheap(
    mut data: *mut libc::c_float,
    stride: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v: libc::c_float = *data.offset(k.wrapping_mul(stride) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data.offset(j.wrapping_mul(stride) as isize)
                < *data
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v < *data.offset(j.wrapping_mul(stride) as isize)) {
            break;
        }
        *data
            .offset(
                k.wrapping_mul(stride) as isize,
            ) = *data.offset(j.wrapping_mul(stride) as isize);
        k = j;
    }
    *data.offset(k.wrapping_mul(stride) as isize) = v;
}
#[inline]
unsafe extern "C" fn my_uchar_downheap2(
    mut data1: *mut libc::c_uchar,
    stride1: size_t,
    mut data2: *mut libc::c_uchar,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_uchar = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_uchar = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v1 as libc::c_int)
            < *data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
        {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_char_downheap2(
    mut data1: *mut libc::c_char,
    stride1: size_t,
    mut data2: *mut libc::c_char,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_char = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_char = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v1 as libc::c_int)
            < *data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
        {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_long_double_downheap2(
    mut data1: *mut f128::f128,
    stride1: size_t,
    mut data2: *mut f128::f128,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: f128::f128 = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: f128::f128 = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_downheap2(
    mut data1: *mut libc::c_double,
    stride1: size_t,
    mut data2: *mut libc::c_double,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_double = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_double = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_float_downheap2(
    mut data1: *mut libc::c_float,
    stride1: size_t,
    mut data2: *mut libc::c_float,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_float = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_float = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_ulong_downheap2(
    mut data1: *mut libc::c_ulong,
    stride1: size_t,
    mut data2: *mut libc::c_ulong,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_ulong = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_ulong = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_long_downheap2(
    mut data1: *mut libc::c_long,
    stride1: size_t,
    mut data2: *mut libc::c_long,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_long = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_long = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_uint_downheap2(
    mut data1: *mut libc::c_uint,
    stride1: size_t,
    mut data2: *mut libc::c_uint,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_uint = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_uint = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_int_downheap2(
    mut data1: *mut libc::c_int,
    stride1: size_t,
    mut data2: *mut libc::c_int,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_int = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_int = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && *data1.offset(j.wrapping_mul(stride1) as isize)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    )
        {
            j = j.wrapping_add(1);
            j;
        }
        if !(v1 < *data1.offset(j.wrapping_mul(stride1) as isize)) {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_ushort_downheap2(
    mut data1: *mut libc::c_ushort,
    stride1: size_t,
    mut data2: *mut libc::c_ushort,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_ushort = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_ushort = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v1 as libc::c_int)
            < *data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
        {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
#[inline]
unsafe extern "C" fn my_short_downheap2(
    mut data1: *mut libc::c_short,
    stride1: size_t,
    mut data2: *mut libc::c_short,
    stride2: size_t,
    N: size_t,
    mut k: size_t,
) {
    let mut v1: libc::c_short = *data1.offset(k.wrapping_mul(stride1) as isize);
    let mut v2: libc::c_short = *data2.offset(k.wrapping_mul(stride2) as isize);
    while k <= N.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k);
        if j < N
            && (*data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
                < *data1
                    .offset(
                        j
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride1) as isize,
                    ) as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        if !((v1 as libc::c_int)
            < *data1.offset(j.wrapping_mul(stride1) as isize) as libc::c_int)
        {
            break;
        }
        *data1
            .offset(
                k.wrapping_mul(stride1) as isize,
            ) = *data1.offset(j.wrapping_mul(stride1) as isize);
        *data2
            .offset(
                k.wrapping_mul(stride2) as isize,
            ) = *data2.offset(j.wrapping_mul(stride2) as isize);
        k = j;
    }
    *data1.offset(k.wrapping_mul(stride1) as isize) = v1;
    *data2.offset(k.wrapping_mul(stride2) as isize) = v2;
}
pub unsafe extern "C" fn gsl_sort_uint(
    mut data: *mut libc::c_uint,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_uint_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_uint = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_uint_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_uchar(
    mut data: *mut libc::c_uchar,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_uchar_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_uchar = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_uchar_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_double = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_float(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_float_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_float = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_float_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_ulong(
    mut data: *mut libc::c_ulong,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_ulong_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_ulong = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_ulong_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_long(
    mut data: *mut libc::c_long,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_long_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_long = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_long_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_long_double(
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_long_double_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: f128::f128 = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_long_double_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_int(
    mut data: *mut libc::c_int,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_int_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_int = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_int_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_ushort(
    mut data: *mut libc::c_ushort,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_ushort_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_ushort = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_ushort_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_char(
    mut data: *mut libc::c_char,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_char_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_char = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_char_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_short(
    mut data: *mut libc::c_short,
    stride: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_short_downheap(data, stride, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_short = *data
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
        *data
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize,
            ) = *data.offset(N.wrapping_mul(stride) as isize);
        *data.offset(N.wrapping_mul(stride) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_short_downheap(data, stride, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort_vector_long(mut v: *mut gsl_vector_long) {
    gsl_sort_long((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_char(mut v: *mut gsl_vector_char) {
    gsl_sort_char((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_int(mut v: *mut gsl_vector_int) {
    gsl_sort_int((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector(mut v: *mut gsl_vector) {
    gsl_sort((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_uchar(mut v: *mut gsl_vector_uchar) {
    gsl_sort_uchar((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_long_double(
    mut v: *mut gsl_vector_long_double,
) {
    gsl_sort_long_double((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_uint(mut v: *mut gsl_vector_uint) {
    gsl_sort_uint((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_ulong(mut v: *mut gsl_vector_ulong) {
    gsl_sort_ulong((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_ushort(mut v: *mut gsl_vector_ushort) {
    gsl_sort_ushort((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_float(mut v: *mut gsl_vector_float) {
    gsl_sort_float((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_short(mut v: *mut gsl_vector_short) {
    gsl_sort_short((*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort2_short(
    mut data1: *mut libc::c_short,
    stride1: size_t,
    mut data2: *mut libc::c_short,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_short_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_short = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_short_downheap2(
            data1,
            stride1,
            data2,
            stride2,
            N,
            0 as libc::c_int as size_t,
        );
    }
}
pub unsafe extern "C" fn gsl_sort2_ushort(
    mut data1: *mut libc::c_ushort,
    stride1: size_t,
    mut data2: *mut libc::c_ushort,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_ushort_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_ushort = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_ushort_downheap2(
            data1,
            stride1,
            data2,
            stride2,
            N,
            0 as libc::c_int as size_t,
        );
    }
}
pub unsafe extern "C" fn gsl_sort2_long_double(
    mut data1: *mut f128::f128,
    stride1: size_t,
    mut data2: *mut f128::f128,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_long_double_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: f128::f128 = f128::f128::ZERO;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_long_double_downheap2(
            data1,
            stride1,
            data2,
            stride2,
            N,
            0 as libc::c_int as size_t,
        );
    }
}
pub unsafe extern "C" fn gsl_sort2_int(
    mut data1: *mut libc::c_int,
    stride1: size_t,
    mut data2: *mut libc::c_int,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_int_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_int = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_int_downheap2(data1, stride1, data2, stride2, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort2(
    mut data1: *mut libc::c_double,
    stride1: size_t,
    mut data2: *mut libc::c_double,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_double = 0.;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_downheap2(data1, stride1, data2, stride2, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort2_float(
    mut data1: *mut libc::c_float,
    stride1: size_t,
    mut data2: *mut libc::c_float,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_float_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_float = 0.;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_float_downheap2(
            data1,
            stride1,
            data2,
            stride2,
            N,
            0 as libc::c_int as size_t,
        );
    }
}
pub unsafe extern "C" fn gsl_sort2_uint(
    mut data1: *mut libc::c_uint,
    stride1: size_t,
    mut data2: *mut libc::c_uint,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_uint_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_uint = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_uint_downheap2(data1, stride1, data2, stride2, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort2_char(
    mut data1: *mut libc::c_char,
    stride1: size_t,
    mut data2: *mut libc::c_char,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_char_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_char = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_char_downheap2(data1, stride1, data2, stride2, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort2_uchar(
    mut data1: *mut libc::c_uchar,
    stride1: size_t,
    mut data2: *mut libc::c_uchar,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_uchar_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_uchar = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_uchar_downheap2(
            data1,
            stride1,
            data2,
            stride2,
            N,
            0 as libc::c_int as size_t,
        );
    }
}
pub unsafe extern "C" fn gsl_sort2_long(
    mut data1: *mut libc::c_long,
    stride1: size_t,
    mut data2: *mut libc::c_long,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_long_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_long = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_long_downheap2(data1, stride1, data2, stride2, N, 0 as libc::c_int as size_t);
    }
}
pub unsafe extern "C" fn gsl_sort2_ulong(
    mut data1: *mut libc::c_ulong,
    stride1: size_t,
    mut data2: *mut libc::c_ulong,
    stride2: size_t,
    n: size_t,
) {
    let mut N: size_t = 0;
    let mut k: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    N = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    k = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        my_ulong_downheap2(data1, stride1, data2, stride2, N, k);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    while N > 0 as libc::c_int as libc::c_ulong {
        let mut tmp: libc::c_ulong = 0;
        tmp = *data1
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
        *data1
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize,
            ) = *data1.offset(N.wrapping_mul(stride1) as isize);
        *data1.offset(N.wrapping_mul(stride1) as isize) = tmp;
        tmp = *data2
            .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
        *data2
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize,
            ) = *data2.offset(N.wrapping_mul(stride2) as isize);
        *data2.offset(N.wrapping_mul(stride2) as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        my_ulong_downheap2(
            data1,
            stride1,
            data2,
            stride2,
            N,
            0 as libc::c_int as size_t,
        );
    }
}
pub unsafe extern "C" fn gsl_sort_vector2_uchar(
    mut v1: *mut gsl_vector_uchar,
    mut v2: *mut gsl_vector_uchar,
) {
    gsl_sort2_uchar((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_ulong(
    mut v1: *mut gsl_vector_ulong,
    mut v2: *mut gsl_vector_ulong,
) {
    gsl_sort2_ulong((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2(
    mut v1: *mut gsl_vector,
    mut v2: *mut gsl_vector,
) {
    gsl_sort2((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_int(
    mut v1: *mut gsl_vector_int,
    mut v2: *mut gsl_vector_int,
) {
    gsl_sort2_int((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_uint(
    mut v1: *mut gsl_vector_uint,
    mut v2: *mut gsl_vector_uint,
) {
    gsl_sort2_uint((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_short(
    mut v1: *mut gsl_vector_short,
    mut v2: *mut gsl_vector_short,
) {
    gsl_sort2_short((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_long_double(
    mut v1: *mut gsl_vector_long_double,
    mut v2: *mut gsl_vector_long_double,
) {
    gsl_sort2_long_double(
        (*v1).data,
        (*v1).stride,
        (*v2).data,
        (*v2).stride,
        (*v1).size,
    );
}
pub unsafe extern "C" fn gsl_sort_vector2_long(
    mut v1: *mut gsl_vector_long,
    mut v2: *mut gsl_vector_long,
) {
    gsl_sort2_long((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_char(
    mut v1: *mut gsl_vector_char,
    mut v2: *mut gsl_vector_char,
) {
    gsl_sort2_char((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_ushort(
    mut v1: *mut gsl_vector_ushort,
    mut v2: *mut gsl_vector_ushort,
) {
    gsl_sort2_ushort((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}
pub unsafe extern "C" fn gsl_sort_vector2_float(
    mut v1: *mut gsl_vector_float,
    mut v2: *mut gsl_vector_float,
) {
    gsl_sort2_float((*v1).data, (*v1).stride, (*v2).data, (*v2).stride, (*v1).size);
}

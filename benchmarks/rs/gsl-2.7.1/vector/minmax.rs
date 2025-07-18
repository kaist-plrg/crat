use ::libc;
use num_traits::Float;
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
pub unsafe extern "C" fn gsl_vector_float_max(
    mut v: *const gsl_vector_float,
) -> libc::c_float {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_float = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        if x.is_nan() as i32 != 0 {
            return x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_ushort_max(
    mut v: *const gsl_vector_ushort,
) -> libc::c_ushort {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ushort = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_long_max(
    mut v: *const gsl_vector_long,
) -> libc::c_long {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_long = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_uint_max(
    mut v: *const gsl_vector_uint,
) -> libc::c_uint {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uint = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_char_max(
    mut v: *const gsl_vector_char,
) -> libc::c_char {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_char = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_uchar_max(
    mut v: *const gsl_vector_uchar,
) -> libc::c_uchar {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uchar = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_int_max(
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_int = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_ulong_max(
    mut v: *const gsl_vector_ulong,
) -> libc::c_ulong {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ulong = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_long_double_max(
    mut v: *const gsl_vector_long_double,
) -> f128::f128 {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: f128::f128 = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        if x.is_nan() as i32 != 0 {
            return x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_short_max(
    mut v: *const gsl_vector_short,
) -> libc::c_short {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_short = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_max(mut v: *const gsl_vector) -> libc::c_double {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_double = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
        }
        if x.is_nan() as i32 != 0 {
            return x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_vector_short_min(
    mut v: *const gsl_vector_short,
) -> libc::c_short {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_short = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_float_min(
    mut v: *const gsl_vector_float,
) -> libc::c_float {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_float = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x.is_nan() as i32 != 0 {
            return x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_ulong_min(
    mut v: *const gsl_vector_ulong,
) -> libc::c_ulong {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ulong = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_ushort_min(
    mut v: *const gsl_vector_ushort,
) -> libc::c_ushort {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ushort = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_char_min(
    mut v: *const gsl_vector_char,
) -> libc::c_char {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_char = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_long_min(
    mut v: *const gsl_vector_long,
) -> libc::c_long {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_long = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_long_double_min(
    mut v: *const gsl_vector_long_double,
) -> f128::f128 {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: f128::f128 = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x.is_nan() as i32 != 0 {
            return x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_uchar_min(
    mut v: *const gsl_vector_uchar,
) -> libc::c_uchar {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uchar = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_int_min(
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_int = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_uint_min(
    mut v: *const gsl_vector_uint,
) -> libc::c_uint {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uint = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_min(mut v: *const gsl_vector) -> libc::c_double {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_double = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x.is_nan() as i32 != 0 {
            return x;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_vector_long_double_minmax(
    mut v: *const gsl_vector_long_double,
    mut min_out: *mut f128::f128,
    mut max_out: *mut f128::f128,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: f128::f128 = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        if x.is_nan() as i32 != 0 {
            min = x;
            max = x;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_uint_minmax(
    mut v: *const gsl_vector_uint,
    mut min_out: *mut libc::c_uint,
    mut max_out: *mut libc::c_uint,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uint = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_short_minmax(
    mut v: *const gsl_vector_short,
    mut min_out: *mut libc::c_short,
    mut max_out: *mut libc::c_short,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_short = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_int_minmax(
    mut v: *const gsl_vector_int,
    mut min_out: *mut libc::c_int,
    mut max_out: *mut libc::c_int,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_int = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_float_minmax(
    mut v: *const gsl_vector_float,
    mut min_out: *mut libc::c_float,
    mut max_out: *mut libc::c_float,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_float = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        if x.is_nan() as i32 != 0 {
            min = x;
            max = x;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_long_minmax(
    mut v: *const gsl_vector_long,
    mut min_out: *mut libc::c_long,
    mut max_out: *mut libc::c_long,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_long = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_ulong_minmax(
    mut v: *const gsl_vector_ulong,
    mut min_out: *mut libc::c_ulong,
    mut max_out: *mut libc::c_ulong,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ulong = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_uchar_minmax(
    mut v: *const gsl_vector_uchar,
    mut min_out: *mut libc::c_uchar,
    mut max_out: *mut libc::c_uchar,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uchar = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_minmax(
    mut v: *const gsl_vector,
    mut min_out: *mut libc::c_double,
    mut max_out: *mut libc::c_double,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_double = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        if x.is_nan() as i32 != 0 {
            min = x;
            max = x;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_char_minmax(
    mut v: *const gsl_vector_char,
    mut min_out: *mut libc::c_char,
    mut max_out: *mut libc::c_char,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_char = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_ushort_minmax(
    mut v: *const gsl_vector_ushort,
    mut min_out: *mut libc::c_ushort,
    mut max_out: *mut libc::c_ushort,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ushort = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_vector_long_double_max_index(
    mut v: *const gsl_vector_long_double,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: f128::f128 = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        if x.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_float_max_index(
    mut v: *const gsl_vector_float,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_float = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        if x.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_long_max_index(
    mut v: *const gsl_vector_long,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_long = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_uint_max_index(
    mut v: *const gsl_vector_uint,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uint = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_char_max_index(
    mut v: *const gsl_vector_char,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_char = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_int_max_index(
    mut v: *const gsl_vector_int,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_int = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_max_index(mut v: *const gsl_vector) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_double = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        if x.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_ushort_max_index(
    mut v: *const gsl_vector_ushort,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ushort = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_ulong_max_index(
    mut v: *const gsl_vector_ulong,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ulong = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_short_max_index(
    mut v: *const gsl_vector_short,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_short = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_uchar_max_index(
    mut v: *const gsl_vector_uchar,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut max: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uchar = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_vector_ushort_min_index(
    mut v: *const gsl_vector_ushort,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ushort = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_long_double_min_index(
    mut v: *const gsl_vector_long_double,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: f128::f128 = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_ulong_min_index(
    mut v: *const gsl_vector_ulong,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ulong = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_uchar_min_index(
    mut v: *const gsl_vector_uchar,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uchar = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_float_min_index(
    mut v: *const gsl_vector_float,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_float = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_char_min_index(
    mut v: *const gsl_vector_char,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_char = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_long_min_index(
    mut v: *const gsl_vector_long,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_long = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_short_min_index(
    mut v: *const gsl_vector_short,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_short = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_uint_min_index(
    mut v: *const gsl_vector_uint,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uint = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_int_min_index(
    mut v: *const gsl_vector_int,
) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_int = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_min_index(mut v: *const gsl_vector) -> size_t {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut min: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_double = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
pub unsafe extern "C" fn gsl_vector_uint_minmax_index(
    mut v: *const gsl_vector_uint,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_uint = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uint = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_minmax_index(
    mut v: *const gsl_vector,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_double = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_double = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        if x.is_nan() as i32 != 0 {
            imin = i;
            imax = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_int_minmax_index(
    mut v: *const gsl_vector_int,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_int = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_int = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_char_minmax_index(
    mut v: *const gsl_vector_char,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_char = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_char = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_long_double_minmax_index(
    mut v: *const gsl_vector_long_double,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: f128::f128 = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: f128::f128 = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        if x.is_nan() as i32 != 0 {
            imin = i;
            imax = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_ulong_minmax_index(
    mut v: *const gsl_vector_ulong,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_ulong = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ulong = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_ushort_minmax_index(
    mut v: *const gsl_vector_ushort,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_ushort = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_ushort = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_long_minmax_index(
    mut v: *const gsl_vector_long,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_long = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_long = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_short_minmax_index(
    mut v: *const gsl_vector_short,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_short = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_short = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_float_minmax_index(
    mut v: *const gsl_vector_float,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_float = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_float = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if x < min {
            min = x;
            imin = i;
        }
        if x > max {
            max = x;
            imax = i;
        }
        if x.is_nan() as i32 != 0 {
            imin = i;
            imax = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *imin_out = imin;
    *imax_out = imax;
}
pub unsafe extern "C" fn gsl_vector_uchar_minmax_index(
    mut v: *const gsl_vector_uchar,
    mut imin_out: *mut size_t,
    mut imax_out: *mut size_t,
) {
    let N: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut min: libc::c_uchar = *((*v).data)
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut x: libc::c_uchar = *((*v).data).offset(i.wrapping_mul(stride) as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
            imin = i;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *imax_out = imax;
}

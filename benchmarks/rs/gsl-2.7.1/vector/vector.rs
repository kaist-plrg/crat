use ::libc;
use ::f128;
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
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_long_double {
    pub dat: [f128::f128; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_float {
    pub dat: [libc::c_float; 2],
}
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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
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
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
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
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
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
pub unsafe extern "C" fn gsl_vector_long_double_get(
    mut v: *const gsl_vector_long_double,
    i: size_t,
) -> f128::f128 {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long_double.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return f128::f128::new(0 as libc::c_int);
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_long_double_set(
    mut v: *mut gsl_vector_long_double,
    i: size_t,
    mut x: f128::f128,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long_double.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_long_double_ptr(
    mut v: *mut gsl_vector_long_double,
    i: size_t,
) -> *mut f128::f128 {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long_double.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut f128::f128;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_long_double_const_ptr(
    mut v: *const gsl_vector_long_double,
    i: size_t,
) -> *const f128::f128 {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long_double.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const f128::f128;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize) as *const f128::f128;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_get(
    mut v: *const gsl_vector_complex_long_double,
    i: size_t,
) -> gsl_complex_long_double {
    if gsl_check_range != 0 && i >= (*v).size {
        let mut zero: gsl_complex_long_double = {
            let mut init = gsl_complex_long_double {
                dat: [
                    f128::f128::new(0 as libc::c_int),
                    f128::f128::new(0 as libc::c_int),
                ],
            };
            init
        };
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_long_double.h\0" as *const u8
                as *const libc::c_char,
            199 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    }
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut f128::f128 as *mut gsl_complex_long_double);
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_set(
    mut v: *mut gsl_vector_complex_long_double,
    i: size_t,
    mut z: gsl_complex_long_double,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_long_double.h\0" as *const u8
                as *const libc::c_char,
            213 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut f128::f128 as *mut gsl_complex_long_double) = z;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_ptr(
    mut v: *mut gsl_vector_complex_long_double,
    i: size_t,
) -> *mut gsl_complex_long_double {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_long_double.h\0" as *const u8
                as *const libc::c_char,
            227 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex_long_double;
    }
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut f128::f128 as *mut gsl_complex_long_double;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_const_ptr(
    mut v: *const gsl_vector_complex_long_double,
    i: size_t,
) -> *const gsl_complex_long_double {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_long_double.h\0" as *const u8
                as *const libc::c_char,
            241 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const gsl_complex_long_double;
    }
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut f128::f128 as *mut gsl_complex_long_double;
}
pub unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_double.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_double;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_double.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_double.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_double;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_const_ptr(
    mut v: *const gsl_vector,
    i: size_t,
) -> *const libc::c_double {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_double.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_double;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_double;
}
pub unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    if gsl_check_range != 0 && i >= (*v).size {
        let mut zero: gsl_complex = {
            let mut init = gsl_complex {
                dat: [
                    0 as libc::c_int as libc::c_double,
                    0 as libc::c_int as libc::c_double,
                ],
            };
            init
        };
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_double.h\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    }
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
pub unsafe extern "C" fn gsl_vector_complex_set(
    mut v: *mut gsl_vector_complex,
    i: size_t,
    mut z: gsl_complex,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_double.h\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex) = z;
}
pub unsafe extern "C" fn gsl_vector_complex_ptr(
    mut v: *mut gsl_vector_complex,
    i: size_t,
) -> *mut gsl_complex {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_double.h\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex;
    }
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex;
}
pub unsafe extern "C" fn gsl_vector_complex_const_ptr(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> *const gsl_complex {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_double.h\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const gsl_complex;
    }
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex;
}
pub unsafe extern "C" fn gsl_vector_float_get(
    mut v: *const gsl_vector_float,
    i: size_t,
) -> libc::c_float {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_float.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_float;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_float_set(
    mut v: *mut gsl_vector_float,
    i: size_t,
    mut x: libc::c_float,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_float.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_float_ptr(
    mut v: *mut gsl_vector_float,
    i: size_t,
) -> *mut libc::c_float {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_float.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_float;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_float_const_ptr(
    mut v: *const gsl_vector_float,
    i: size_t,
) -> *const libc::c_float {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_float.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_float;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_float;
}
pub unsafe extern "C" fn gsl_vector_char_const_ptr(
    mut v: *const gsl_vector_char,
    i: size_t,
) -> *const libc::c_char {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_char.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_char;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_char;
}
pub unsafe extern "C" fn gsl_vector_char_ptr(
    mut v: *mut gsl_vector_char,
    i: size_t,
) -> *mut libc::c_char {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_char.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_char;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_char_set(
    mut v: *mut gsl_vector_char,
    i: size_t,
    mut x: libc::c_char,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_char.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_char_get(
    mut v: *const gsl_vector_char,
    i: size_t,
) -> libc::c_char {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_char.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_char;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_ulong_get(
    mut v: *const gsl_vector_ulong,
    i: size_t,
) -> libc::c_ulong {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ulong.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ulong;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_ulong_set(
    mut v: *mut gsl_vector_ulong,
    i: size_t,
    mut x: libc::c_ulong,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ulong.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_ulong_ptr(
    mut v: *mut gsl_vector_ulong,
    i: size_t,
) -> *mut libc::c_ulong {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ulong.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_ulong;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_ulong_const_ptr(
    mut v: *const gsl_vector_ulong,
    i: size_t,
) -> *const libc::c_ulong {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ulong.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_ulong;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_ulong;
}
pub unsafe extern "C" fn gsl_vector_long_get(
    mut v: *const gsl_vector_long,
    i: size_t,
) -> libc::c_long {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_long;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_long_set(
    mut v: *mut gsl_vector_long,
    i: size_t,
    mut x: libc::c_long,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_long_ptr(
    mut v: *mut gsl_vector_long,
    i: size_t,
) -> *mut libc::c_long {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_long;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_long_const_ptr(
    mut v: *const gsl_vector_long,
    i: size_t,
) -> *const libc::c_long {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_long.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_long;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_long;
}
pub unsafe extern "C" fn gsl_vector_uint_get(
    mut v: *const gsl_vector_uint,
    i: size_t,
) -> libc::c_uint {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uint.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uint;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_uint_set(
    mut v: *mut gsl_vector_uint,
    i: size_t,
    mut x: libc::c_uint,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uint.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_uint_ptr(
    mut v: *mut gsl_vector_uint,
    i: size_t,
) -> *mut libc::c_uint {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uint.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_uint;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_uint_const_ptr(
    mut v: *const gsl_vector_uint,
    i: size_t,
) -> *const libc::c_uint {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uint.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_uint;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_uint;
}
pub unsafe extern "C" fn gsl_vector_int_get(
    mut v: *const gsl_vector_int,
    i: size_t,
) -> libc::c_int {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_int.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_int_set(
    mut v: *mut gsl_vector_int,
    i: size_t,
    mut x: libc::c_int,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_int.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_int_ptr(
    mut v: *mut gsl_vector_int,
    i: size_t,
) -> *mut libc::c_int {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_int.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_int;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_int_const_ptr(
    mut v: *const gsl_vector_int,
    i: size_t,
) -> *const libc::c_int {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_int.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_int;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_get(
    mut v: *const gsl_vector_ushort,
    i: size_t,
) -> libc::c_ushort {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ushort.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ushort;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_ushort_set(
    mut v: *mut gsl_vector_ushort,
    i: size_t,
    mut x: libc::c_ushort,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ushort.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_ushort_ptr(
    mut v: *mut gsl_vector_ushort,
    i: size_t,
) -> *mut libc::c_ushort {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ushort.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_ushort;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_ushort_const_ptr(
    mut v: *const gsl_vector_ushort,
    i: size_t,
) -> *const libc::c_ushort {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_ushort.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_ushort;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_ushort;
}
pub unsafe extern "C" fn gsl_vector_short_get(
    mut v: *const gsl_vector_short,
    i: size_t,
) -> libc::c_short {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_short.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_short;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_short_set(
    mut v: *mut gsl_vector_short,
    i: size_t,
    mut x: libc::c_short,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_short.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_short_ptr(
    mut v: *mut gsl_vector_short,
    i: size_t,
) -> *mut libc::c_short {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_short.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_short;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_short_const_ptr(
    mut v: *const gsl_vector_short,
    i: size_t,
) -> *const libc::c_short {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_short.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_short;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_short;
}
pub unsafe extern "C" fn gsl_vector_uchar_get(
    mut v: *const gsl_vector_uchar,
    i: size_t,
) -> libc::c_uchar {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uchar.h\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uchar;
    }
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_uchar_set(
    mut v: *mut gsl_vector_uchar,
    i: size_t,
    mut x: libc::c_uchar,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uchar.h\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_vector_uchar_ptr(
    mut v: *mut gsl_vector_uchar,
    i: size_t,
) -> *mut libc::c_uchar {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uchar.h\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_uchar;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_vector_uchar_const_ptr(
    mut v: *const gsl_vector_uchar,
    i: size_t,
) -> *const libc::c_uchar {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_uchar.h\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const libc::c_uchar;
    }
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize)
        as *const libc::c_uchar;
}
pub unsafe extern "C" fn gsl_vector_complex_float_get(
    mut v: *const gsl_vector_complex_float,
    i: size_t,
) -> gsl_complex_float {
    if gsl_check_range != 0 && i >= (*v).size {
        let mut zero: gsl_complex_float = {
            let mut init = gsl_complex_float {
                dat: [
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                ],
            };
            init
        };
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_float.h\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    }
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_float as *mut gsl_complex_float);
}
pub unsafe extern "C" fn gsl_vector_complex_float_set(
    mut v: *mut gsl_vector_complex_float,
    i: size_t,
    mut z: gsl_complex_float,
) {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_float.h\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_float as *mut gsl_complex_float) = z;
}
pub unsafe extern "C" fn gsl_vector_complex_float_ptr(
    mut v: *mut gsl_vector_complex_float,
    i: size_t,
) -> *mut gsl_complex_float {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_float.h\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex_float;
    }
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_float as *mut gsl_complex_float;
}
pub unsafe extern "C" fn gsl_vector_complex_float_const_ptr(
    mut v: *const gsl_vector_complex_float,
    i: size_t,
) -> *const gsl_complex_float {
    if gsl_check_range != 0 && i >= (*v).size {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"../gsl/gsl_vector_complex_float.h\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *const gsl_complex_float;
    }
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_float as *mut gsl_complex_float;
}
pub static mut gsl_check_range: libc::c_int = 1 as libc::c_int;

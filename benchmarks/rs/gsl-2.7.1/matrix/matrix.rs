use ::libc;
use ::f128;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    static mut gsl_check_range: libc::c_int;
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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ulong {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uint {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_int {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ushort {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_short {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uchar {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_get(
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> gsl_complex_long_double {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        let mut zero: gsl_complex_long_double = {
            let mut init = gsl_complex_long_double {
                dat: [
                    f128::f128::new(0 as libc::c_int),
                    f128::f128::new(0 as libc::c_int),
                ],
            };
            init
        };
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                280 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                284 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        }
    }
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_long_double);
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_set(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
    x: gsl_complex_long_double,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                301 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                305 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_long_double) = x;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_ptr(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex_long_double {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex_long_double;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex_long_double;
        }
    }
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_long_double;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_ptr(
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> *const gsl_complex_long_double {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                343 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const gsl_complex_long_double;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_long_double.h\0" as *const u8
                    as *const libc::c_char,
                347 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const gsl_complex_long_double;
        }
    }
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *const gsl_complex_long_double;
}
pub unsafe extern "C" fn gsl_matrix_complex_get(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> gsl_complex {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        let mut zero: gsl_complex = {
            let mut init = gsl_complex {
                dat: [
                    0 as libc::c_int as libc::c_double,
                    0 as libc::c_int as libc::c_double,
                ],
            };
            init
        };
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                280 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                284 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        }
    }
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex);
}
pub unsafe extern "C" fn gsl_matrix_complex_set(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
    x: gsl_complex,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                301 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                305 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex) = x;
}
pub unsafe extern "C" fn gsl_matrix_complex_ptr(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex;
        }
    }
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_ptr(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> *const gsl_complex {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                343 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const gsl_complex;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_double.h\0" as *const u8
                    as *const libc::c_char,
                347 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const gsl_complex;
        }
    }
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *const gsl_complex;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_get(
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> gsl_complex_float {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        let mut zero: gsl_complex_float = {
            let mut init = gsl_complex_float {
                dat: [
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                ],
            };
            init
        };
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                280 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                284 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        }
    }
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_float);
}
pub unsafe extern "C" fn gsl_matrix_complex_float_set(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
    x: gsl_complex_float,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                301 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                305 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_float) = x;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_ptr(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex_float {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex_float;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex_float;
        }
    }
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_float;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_ptr(
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> *const gsl_complex_float {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                343 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const gsl_complex_float;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_complex_float.h\0" as *const u8
                    as *const libc::c_char,
                347 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const gsl_complex_float;
        }
    }
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *const gsl_complex_float;
}
pub unsafe extern "C" fn gsl_matrix_long_double_get(
    mut m: *const gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> f128::f128 {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return f128::f128::new(0 as libc::c_int);
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return f128::f128::new(0 as libc::c_int);
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_long_double_set(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
    x: f128::f128,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_long_double_ptr(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> *mut f128::f128 {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut f128::f128;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut f128::f128;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_ptr(
    mut m: *const gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> *const f128::f128 {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const f128::f128;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long_double.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const f128::f128;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const f128::f128;
}
pub unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_double;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_double;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_double;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_double;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_const_ptr(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> *const libc::c_double {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_double;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_double.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_double;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_double;
}
pub unsafe extern "C" fn gsl_matrix_float_get(
    mut m: *const gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> libc::c_float {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_float;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_float;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_float_set(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
    x: libc::c_float,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_float_ptr(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> *mut libc::c_float {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_float;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_float;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_float_const_ptr(
    mut m: *const gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> *const libc::c_float {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_float;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_float.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_float;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_float;
}
pub unsafe extern "C" fn gsl_matrix_char_const_ptr(
    mut m: *const gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> *const libc::c_char {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_char;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_char;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_char;
}
pub unsafe extern "C" fn gsl_matrix_char_ptr(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> *mut libc::c_char {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_char;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_char;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_char_set(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
    x: libc::c_char,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_char_get(
    mut m: *const gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> libc::c_char {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_char;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_char.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_char;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_long_get(
    mut m: *const gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> libc::c_long {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_long;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_long;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_long_set(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
    x: libc::c_long,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_long_ptr(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> *mut libc::c_long {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_long;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_long;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_long_const_ptr(
    mut m: *const gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> *const libc::c_long {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_long;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_long.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_long;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_long;
}
pub unsafe extern "C" fn gsl_matrix_uint_get(
    mut m: *const gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> libc::c_uint {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_uint;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_uint_set(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
    x: libc::c_uint,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_uint_ptr(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> *mut libc::c_uint {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_uint;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_uint;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_uint_const_ptr(
    mut m: *const gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> *const libc::c_uint {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_uint;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uint.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_uint;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_uint;
}
pub unsafe extern "C" fn gsl_matrix_int_get(
    mut m: *const gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_int_set(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
    x: libc::c_int,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_int_ptr(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> *mut libc::c_int {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_int;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_int;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_int_const_ptr(
    mut m: *const gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> *const libc::c_int {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_int;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_int.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_int;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_get(
    mut m: *const gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_ushort {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_ushort;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_ushort;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_ushort_set(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
    x: libc::c_ushort,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_ushort_ptr(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> *mut libc::c_ushort {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_ushort;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_ushort;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_ptr(
    mut m: *const gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> *const libc::c_ushort {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_ushort;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ushort.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_ushort;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_ushort;
}
pub unsafe extern "C" fn gsl_matrix_short_get(
    mut m: *const gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> libc::c_short {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_short;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_short;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_short_set(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
    x: libc::c_short,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_short_ptr(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> *mut libc::c_short {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_short;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_short;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_short_const_ptr(
    mut m: *const gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> *const libc::c_short {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_short;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_short.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_short;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_short;
}
pub unsafe extern "C" fn gsl_matrix_uchar_get(
    mut m: *const gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_uchar {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_uchar;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_uchar;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_uchar_set(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
    x: libc::c_uchar,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_uchar_ptr(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> *mut libc::c_uchar {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_uchar;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_uchar;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_ptr(
    mut m: *const gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> *const libc::c_uchar {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_uchar;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_uchar.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_uchar;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_uchar;
}
pub unsafe extern "C" fn gsl_matrix_ulong_get(
    mut m: *const gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_ulong {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_ulong;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_ulong;
        }
    }
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_ulong_set(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
    x: libc::c_ulong,
) {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return;
        }
    }
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_matrix_ulong_ptr(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> *mut libc::c_ulong {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_ulong;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_ulong;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_ptr(
    mut m: *const gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> *const libc::c_ulong {
    if gsl_check_range != 0 && 1 as libc::c_int != 0 {
        if i >= (*m).size1 {
            gsl_error(
                b"first index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_ulong;
        } else if j >= (*m).size2 {
            gsl_error(
                b"second index out of range\0" as *const u8 as *const libc::c_char,
                b"../gsl/gsl_matrix_ulong.h\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *const libc::c_ulong;
        }
    }
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_ulong;
}

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
pub unsafe extern "C" fn gsl_sort_uchar_smallest(
    mut dest: *mut libc::c_uchar,
    k: size_t,
    mut src: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_uchar = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_uchar = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int >= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi as libc::c_int
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_ushort_smallest(
    mut dest: *mut libc::c_ushort,
    k: size_t,
    mut src: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_ushort = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_ushort = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int >= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi as libc::c_int
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_int_smallest(
    mut dest: *mut libc::c_int,
    k: size_t,
    mut src: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_int = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_int = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_long_double_smallest(
    mut dest: *mut f128::f128,
    k: size_t,
    mut src: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: f128::f128 = f128::f128::ZERO;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: f128::f128 = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_smallest(
    mut dest: *mut libc::c_double,
    k: size_t,
    mut src: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_double = 0.;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_double = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_char_smallest(
    mut dest: *mut libc::c_char,
    k: size_t,
    mut src: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_char = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_char = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int >= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi as libc::c_int
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_float_smallest(
    mut dest: *mut libc::c_float,
    k: size_t,
    mut src: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_float = 0.;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_float = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_long_smallest(
    mut dest: *mut libc::c_long,
    k: size_t,
    mut src: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_long = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_long = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_uint_smallest(
    mut dest: *mut libc::c_uint,
    k: size_t,
    mut src: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_uint = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_uint = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_short_smallest(
    mut dest: *mut libc::c_short,
    k: size_t,
    mut src: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_short = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_short = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int >= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi as libc::c_int
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_ulong_smallest(
    mut dest: *mut libc::c_ulong,
    k: size_t,
    mut src: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_ulong = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_ulong = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi >= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        > *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_short_smallest(
    mut dest: *mut libc::c_short,
    k: size_t,
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    return gsl_sort_short_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_float_smallest(
    mut dest: *mut libc::c_float,
    k: size_t,
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    return gsl_sort_float_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_ulong_smallest(
    mut dest: *mut libc::c_ulong,
    k: size_t,
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    return gsl_sort_ulong_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_char_smallest(
    mut dest: *mut libc::c_char,
    k: size_t,
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    return gsl_sort_char_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_ushort_smallest(
    mut dest: *mut libc::c_ushort,
    k: size_t,
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    return gsl_sort_ushort_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_long_double_smallest(
    mut dest: *mut f128::f128,
    k: size_t,
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    return gsl_sort_long_double_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_long_smallest(
    mut dest: *mut libc::c_long,
    k: size_t,
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    return gsl_sort_long_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_smallest(
    mut dest: *mut libc::c_double,
    k: size_t,
    mut v: *const gsl_vector,
) -> libc::c_int {
    return gsl_sort_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_int_smallest(
    mut dest: *mut libc::c_int,
    k: size_t,
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    return gsl_sort_int_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_uchar_smallest(
    mut dest: *mut libc::c_uchar,
    k: size_t,
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    return gsl_sort_uchar_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_uint_smallest(
    mut dest: *mut libc::c_uint,
    k: size_t,
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    return gsl_sort_uint_smallest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_int_largest(
    mut dest: *mut libc::c_int,
    k: size_t,
    mut src: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_int = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_int = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_float_largest(
    mut dest: *mut libc::c_float,
    k: size_t,
    mut src: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_float = 0.;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_float = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_uint_largest(
    mut dest: *mut libc::c_uint,
    k: size_t,
    mut src: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_uint = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_uint = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_long_double_largest(
    mut dest: *mut f128::f128,
    k: size_t,
    mut src: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: f128::f128 = f128::f128::ZERO;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: f128::f128 = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_long_largest(
    mut dest: *mut libc::c_long,
    k: size_t,
    mut src: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_long = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_long = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_char_largest(
    mut dest: *mut libc::c_char,
    k: size_t,
    mut src: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_char = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_char = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int <= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if (xi as libc::c_int)
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_largest(
    mut dest: *mut libc::c_double,
    k: size_t,
    mut src: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_double = 0.;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_double = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_uchar_largest(
    mut dest: *mut libc::c_uchar,
    k: size_t,
    mut src: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_uchar = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_uchar = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int <= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if (xi as libc::c_int)
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_ulong_largest(
    mut dest: *mut libc::c_ulong,
    k: size_t,
    mut src: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_ulong = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_ulong = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi <= xbound {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if xi
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_ushort_largest(
    mut dest: *mut libc::c_ushort,
    k: size_t,
    mut src: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_ushort = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_ushort = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int <= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if (xi as libc::c_int)
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_short_largest(
    mut dest: *mut libc::c_short,
    k: size_t,
    mut src: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xbound: libc::c_short = 0;
    if k > n {
        gsl_error(
            b"subset length k exceeds vector length n\0" as *const u8
                as *const libc::c_char,
            b"./subset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if k == 0 as libc::c_int as libc::c_ulong || n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 1 as libc::c_int as size_t;
    xbound = *src
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    *dest.offset(0 as libc::c_int as isize) = xbound;
    let mut current_block_19: u64;
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut i1: size_t = 0;
        let mut xi: libc::c_short = *src.offset(i.wrapping_mul(stride) as isize);
        if j < k {
            j = j.wrapping_add(1);
            j;
            current_block_19 = 17860125682698302841;
        } else if xi as libc::c_int <= xbound as libc::c_int {
            current_block_19 = 13183875560443969876;
        } else {
            current_block_19 = 17860125682698302841;
        }
        match current_block_19 {
            17860125682698302841 => {
                i1 = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i1 > 0 as libc::c_int as libc::c_ulong {
                    if (xi as libc::c_int)
                        < *dest
                            .offset(
                                i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int
                    {
                        break;
                    }
                    *dest
                        .offset(
                            i1 as isize,
                        ) = *dest
                        .offset(
                            i1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    i1 = i1.wrapping_sub(1);
                    i1;
                }
                *dest.offset(i1 as isize) = xi;
                xbound = *dest
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sort_vector_int_largest(
    mut dest: *mut libc::c_int,
    k: size_t,
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    return gsl_sort_int_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_long_largest(
    mut dest: *mut libc::c_long,
    k: size_t,
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    return gsl_sort_long_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_float_largest(
    mut dest: *mut libc::c_float,
    k: size_t,
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    return gsl_sort_float_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_short_largest(
    mut dest: *mut libc::c_short,
    k: size_t,
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    return gsl_sort_short_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_ushort_largest(
    mut dest: *mut libc::c_ushort,
    k: size_t,
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    return gsl_sort_ushort_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_ulong_largest(
    mut dest: *mut libc::c_ulong,
    k: size_t,
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    return gsl_sort_ulong_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_uint_largest(
    mut dest: *mut libc::c_uint,
    k: size_t,
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    return gsl_sort_uint_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_long_double_largest(
    mut dest: *mut f128::f128,
    k: size_t,
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    return gsl_sort_long_double_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_largest(
    mut dest: *mut libc::c_double,
    k: size_t,
    mut v: *const gsl_vector,
) -> libc::c_int {
    return gsl_sort_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_char_largest(
    mut dest: *mut libc::c_char,
    k: size_t,
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    return gsl_sort_char_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}
pub unsafe extern "C" fn gsl_sort_vector_uchar_largest(
    mut dest: *mut libc::c_uchar,
    k: size_t,
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    return gsl_sort_uchar_largest(dest, k, (*v).data, (*v).stride, (*v).size);
}

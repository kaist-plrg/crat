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
pub unsafe extern "C" fn gsl_vector_char_swap(
    mut v: *mut gsl_vector_char,
    mut w: *mut gsl_vector_char,
) -> libc::c_int {
    let mut d1: *mut libc::c_char = (*v).data;
    let mut d2: *mut libc::c_char = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_char = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_swap(
    mut v: *mut gsl_vector_ushort,
    mut w: *mut gsl_vector_ushort,
) -> libc::c_int {
    let mut d1: *mut libc::c_ushort = (*v).data;
    let mut d2: *mut libc::c_ushort = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ushort = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_swap(
    mut v: *mut gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut d1: *mut libc::c_double = (*v).data;
    let mut d2: *mut libc::c_double = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_swap(
    mut v: *mut gsl_vector_short,
    mut w: *mut gsl_vector_short,
) -> libc::c_int {
    let mut d1: *mut libc::c_short = (*v).data;
    let mut d2: *mut libc::c_short = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_short = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_swap(
    mut v: *mut gsl_vector_complex,
    mut w: *mut gsl_vector_complex,
) -> libc::c_int {
    let mut d1: *mut libc::c_double = (*v).data;
    let mut d2: *mut libc::c_double = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_swap(
    mut v: *mut gsl_vector_int,
    mut w: *mut gsl_vector_int,
) -> libc::c_int {
    let mut d1: *mut libc::c_int = (*v).data;
    let mut d2: *mut libc::c_int = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_int = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_swap(
    mut v: *mut gsl_vector_complex_long_double,
    mut w: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    let mut d1: *mut f128::f128 = (*v).data;
    let mut d2: *mut f128::f128 = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_swap(
    mut v: *mut gsl_vector_float,
    mut w: *mut gsl_vector_float,
) -> libc::c_int {
    let mut d1: *mut libc::c_float = (*v).data;
    let mut d2: *mut libc::c_float = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_swap(
    mut v: *mut gsl_vector_uint,
    mut w: *mut gsl_vector_uint,
) -> libc::c_int {
    let mut d1: *mut libc::c_uint = (*v).data;
    let mut d2: *mut libc::c_uint = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uint = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_swap(
    mut v: *mut gsl_vector_uchar,
    mut w: *mut gsl_vector_uchar,
) -> libc::c_int {
    let mut d1: *mut libc::c_uchar = (*v).data;
    let mut d2: *mut libc::c_uchar = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uchar = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_swap(
    mut v: *mut gsl_vector_long_double,
    mut w: *mut gsl_vector_long_double,
) -> libc::c_int {
    let mut d1: *mut f128::f128 = (*v).data;
    let mut d2: *mut f128::f128 = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_swap(
    mut v: *mut gsl_vector_complex_float,
    mut w: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let mut d1: *mut libc::c_float = (*v).data;
    let mut d2: *mut libc::c_float = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_swap(
    mut v: *mut gsl_vector_long,
    mut w: *mut gsl_vector_long,
) -> libc::c_int {
    let mut d1: *mut libc::c_long = (*v).data;
    let mut d2: *mut libc::c_long = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_long = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_swap(
    mut v: *mut gsl_vector_ulong,
    mut w: *mut gsl_vector_ulong,
) -> libc::c_int {
    let mut d1: *mut libc::c_ulong = (*v).data;
    let mut d2: *mut libc::c_ulong = (*w).data;
    let size: size_t = (*v).size;
    let s1: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    let s2: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul((*w).stride);
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if (*v).size != (*w).size {
        gsl_error(
            b"vector lengths must be equal\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ulong = *d1
                .offset(i.wrapping_mul(s1).wrapping_add(k) as isize);
            *d1
                .offset(
                    i.wrapping_mul(s1).wrapping_add(k) as isize,
                ) = *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize);
            *d2.offset(i.wrapping_mul(s2).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_swap_elements(
    mut v: *mut gsl_vector_long,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_long = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_long = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_swap_elements(
    mut v: *mut gsl_vector_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_int = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_int = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_swap_elements(
    mut v: *mut gsl_vector_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_ulong = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ulong = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_swap_elements(
    mut v: *mut gsl_vector_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_uchar = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uchar = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_swap_elements(
    mut v: *mut gsl_vector_complex_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_float = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_swap_elements(
    mut v: *mut gsl_vector_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut f128::f128 = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_swap_elements(
    mut v: *mut gsl_vector_char,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_char = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_char = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_swap_elements(
    mut v: *mut gsl_vector_short,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_short = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_short = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_swap_elements(
    mut v: *mut gsl_vector_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_float = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_swap_elements(
    mut v: *mut gsl_vector_complex_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut f128::f128 = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_swap_elements(
    mut v: *mut gsl_vector_uint,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_uint = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uint = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_swap_elements(
    mut v: *mut gsl_vector_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_ushort = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ushort = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_swap_elements(
    mut v: *mut gsl_vector_complex,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_double = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_swap_elements(
    mut v: *mut gsl_vector,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let mut data: *mut libc::c_double = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_reverse(
    mut v: *mut gsl_vector_int,
) -> libc::c_int {
    let mut data: *mut libc::c_int = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_int = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_reverse(
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    let mut data: *mut libc::c_double = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_reverse(
    mut v: *mut gsl_vector_ulong,
) -> libc::c_int {
    let mut data: *mut libc::c_ulong = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ulong = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_reverse(
    mut v: *mut gsl_vector_ushort,
) -> libc::c_int {
    let mut data: *mut libc::c_ushort = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ushort = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_reverse(
    mut v: *mut gsl_vector_long,
) -> libc::c_int {
    let mut data: *mut libc::c_long = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_long = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_reverse(mut v: *mut gsl_vector) -> libc::c_int {
    let mut data: *mut libc::c_double = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_reverse(
    mut v: *mut gsl_vector_float,
) -> libc::c_int {
    let mut data: *mut libc::c_float = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_reverse(
    mut v: *mut gsl_vector_short,
) -> libc::c_int {
    let mut data: *mut libc::c_short = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_short = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_reverse(
    mut v: *mut gsl_vector_uint,
) -> libc::c_int {
    let mut data: *mut libc::c_uint = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uint = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_reverse(
    mut v: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    let mut data: *mut f128::f128 = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_reverse(
    mut v: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let mut data: *mut libc::c_float = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_reverse(
    mut v: *mut gsl_vector_uchar,
) -> libc::c_int {
    let mut data: *mut libc::c_uchar = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uchar = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_reverse(
    mut v: *mut gsl_vector_char,
) -> libc::c_int {
    let mut data: *mut libc::c_char = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_char = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_reverse(
    mut v: *mut gsl_vector_long_double,
) -> libc::c_int {
    let mut data: *mut f128::f128 = (*v).data;
    let size: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let s: size_t = (1 as libc::c_int as libc::c_ulong).wrapping_mul(stride);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *data
                .offset(j.wrapping_mul(s).wrapping_add(k) as isize);
            *data
                .offset(
                    j.wrapping_mul(s).wrapping_add(k) as isize,
                ) = *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize);
            *data.offset(i.wrapping_mul(s).wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}

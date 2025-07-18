use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_complex_long_double_row(
        m: *mut gsl_matrix_complex_long_double,
        i: size_t,
    ) -> _gsl_vector_complex_long_double_view;
    fn gsl_matrix_complex_row(
        m: *mut gsl_matrix_complex,
        i: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_float_row(
        m: *mut gsl_matrix_complex_float,
        i: size_t,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_long_double_row(
        m: *mut gsl_matrix_long_double,
        i: size_t,
    ) -> _gsl_vector_long_double_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_float_row(
        m: *mut gsl_matrix_float,
        i: size_t,
    ) -> _gsl_vector_float_view;
    fn gsl_matrix_ulong_row(
        m: *mut gsl_matrix_ulong,
        i: size_t,
    ) -> _gsl_vector_ulong_view;
    fn gsl_matrix_long_row(m: *mut gsl_matrix_long, i: size_t) -> _gsl_vector_long_view;
    fn gsl_matrix_uint_row(m: *mut gsl_matrix_uint, i: size_t) -> _gsl_vector_uint_view;
    fn gsl_matrix_int_row(m: *mut gsl_matrix_int, i: size_t) -> _gsl_vector_int_view;
    fn gsl_matrix_ushort_row(
        m: *mut gsl_matrix_ushort,
        i: size_t,
    ) -> _gsl_vector_ushort_view;
    fn gsl_matrix_short_row(
        m: *mut gsl_matrix_short,
        i: size_t,
    ) -> _gsl_vector_short_view;
    fn gsl_matrix_uchar_row(
        m: *mut gsl_matrix_uchar,
        i: size_t,
    ) -> _gsl_vector_uchar_view;
    fn gsl_matrix_char_row(m: *mut gsl_matrix_char, i: size_t) -> _gsl_vector_char_view;
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
pub struct _gsl_vector_long_double_view {
    pub vector: gsl_vector_long_double,
}
pub type gsl_vector_long_double_view = _gsl_vector_long_double_view;
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
pub struct _gsl_vector_complex_long_double_view {
    pub vector: gsl_vector_complex_long_double,
}
pub type gsl_vector_complex_long_double_view = _gsl_vector_complex_long_double_view;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
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
pub struct _gsl_vector_complex_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_view = _gsl_vector_complex_view;
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
pub struct _gsl_vector_float_view {
    pub vector: gsl_vector_float,
}
pub type gsl_vector_float_view = _gsl_vector_float_view;
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
pub struct _gsl_vector_complex_float_view {
    pub vector: gsl_vector_complex_float,
}
pub type gsl_vector_complex_float_view = _gsl_vector_complex_float_view;
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
pub struct _gsl_vector_ulong_view {
    pub vector: gsl_vector_ulong,
}
pub type gsl_vector_ulong_view = _gsl_vector_ulong_view;
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
pub struct _gsl_vector_long_view {
    pub vector: gsl_vector_long,
}
pub type gsl_vector_long_view = _gsl_vector_long_view;
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
pub struct _gsl_vector_uint_view {
    pub vector: gsl_vector_uint,
}
pub type gsl_vector_uint_view = _gsl_vector_uint_view;
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
pub struct _gsl_vector_int_view {
    pub vector: gsl_vector_int,
}
pub type gsl_vector_int_view = _gsl_vector_int_view;
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
pub struct _gsl_vector_ushort_view {
    pub vector: gsl_vector_ushort,
}
pub type gsl_vector_ushort_view = _gsl_vector_ushort_view;
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
pub struct _gsl_vector_short_view {
    pub vector: gsl_vector_short,
}
pub type gsl_vector_short_view = _gsl_vector_short_view;
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
pub struct _gsl_vector_uchar_view {
    pub vector: gsl_vector_uchar,
}
pub type gsl_vector_uchar_view = _gsl_vector_uchar_view;
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
pub struct _gsl_vector_char_view {
    pub vector: gsl_vector_char,
}
pub type gsl_vector_char_view = _gsl_vector_char_view;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
pub unsafe extern "C" fn gsl_permute_ulong(
    mut p: *const size_t,
    mut data: *mut libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_ulong; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_ulong = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_int(
    mut p: *const size_t,
    mut data: *mut libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_int; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_int = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_char(
    mut p: *const size_t,
    mut data: *mut libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_char; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_char = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_complex(
    mut p: *const size_t,
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_double; 2] = [0.; 2];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 2 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_double = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_uchar(
    mut p: *const size_t,
    mut data: *mut libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_uchar; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_uchar = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_complex_float(
    mut p: *const size_t,
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_float; 2] = [0.; 2];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 2 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_float = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_short(
    mut p: *const size_t,
    mut data: *mut libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_short; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_short = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_long_double(
    mut p: *const size_t,
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [f128::f128; 1] = [f128::f128::ZERO; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: f128::f128 = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_complex_long_double(
    mut p: *const size_t,
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [f128::f128; 2] = [f128::f128::ZERO; 2];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 2 as libc::c_int as libc::c_uint {
                        let mut r1: f128::f128 = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_ushort(
    mut p: *const size_t,
    mut data: *mut libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_ushort; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_ushort = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute(
    mut p: *const size_t,
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_double; 1] = [0.; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_double = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_uint(
    mut p: *const size_t,
    mut data: *mut libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_uint; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_uint = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_float(
    mut p: *const size_t,
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_float; 1] = [0.; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_float = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_long(
    mut p: *const size_t,
    mut data: *mut libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_long; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            i
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_long = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                k
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_ulong_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_ulong; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_ulong = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_float_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_float; 1] = [0.; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_float = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_long_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_long; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_long = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_double; 1] = [0.; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_double = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_uint_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_uint; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_uint = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_char_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_char; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_char = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_int_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_int; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_int = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_long_double_inverse(
    mut p: *const size_t,
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [f128::f128; 1] = [f128::f128::ZERO; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: f128::f128 = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_ushort_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_ushort; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_ushort = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_complex_float_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_float; 2] = [0.; 2];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 2 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_float = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_short_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_short; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_short = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_complex_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_double; 2] = [0.; 2];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 2 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_double = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_uchar_inverse(
    mut p: *const size_t,
    mut data: *mut libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [libc::c_uchar; 1] = [0; 1];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 1 as libc::c_int as libc::c_uint {
                        let mut r1: libc::c_uchar = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 1 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_complex_long_double_inverse(
    mut p: *const size_t,
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut pk: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *p.offset(i as isize);
        while k > i {
            k = *p.offset(k as isize);
        }
        if !(k < i) {
            pk = *p.offset(k as isize);
            if !(pk == i) {
                let mut a: libc::c_uint = 0;
                let mut t: [f128::f128; 2] = [f128::f128::ZERO; 2];
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    t[a
                        as usize] = *data
                        .offset(
                            k
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        );
                    a = a.wrapping_add(1);
                    a;
                }
                while pk != i {
                    a = 0 as libc::c_int as libc::c_uint;
                    while a < 2 as libc::c_int as libc::c_uint {
                        let mut r1: f128::f128 = *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            );
                        *data
                            .offset(
                                pk
                                    .wrapping_mul(stride)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(a as libc::c_ulong) as isize,
                            ) = t[a as usize];
                        t[a as usize] = r1;
                        a = a.wrapping_add(1);
                        a;
                    }
                    k = pk;
                    pk = *p.offset(k as isize);
                }
                a = 0 as libc::c_int as libc::c_uint;
                while a < 2 as libc::c_int as libc::c_uint {
                    *data
                        .offset(
                            pk
                                .wrapping_mul(stride)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(a as libc::c_ulong) as isize,
                        ) = t[a as usize];
                    a = a.wrapping_add(1);
                    a;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_uchar(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_uchar,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_uchar((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_complex_long_double(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_complex_long_double((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_char(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_char,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_char((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_ulong(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_ulong,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_ulong((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_short(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_short,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_short((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_float(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_float,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_float((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_ushort(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_ushort,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_ushort((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_long(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_long,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_long((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_long_double(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_long_double,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_long_double((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_int(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_int,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_int((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_complex_float(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_complex_float,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_complex_float((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_uint(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_uint,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_uint((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_complex(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_complex((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_int_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_int,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_int_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_uchar_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_uchar,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_uchar_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_char_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_char,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_char_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_long_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_long,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_long_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_complex_long_double_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_complex_long_double_inverse(
        (*p).data,
        (*v).data,
        (*v).stride,
        (*v).size,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_uint_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_uint,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_uint_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_complex_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_complex_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_complex_float_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_complex_float,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_complex_float_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_long_double_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_long_double,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_long_double_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_ushort_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_ushort,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_ushort_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_short_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_short,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_short_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_float_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_float,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_float_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_vector_ulong_inverse(
    mut p: *const gsl_permutation,
    mut v: *mut gsl_vector_ulong,
) -> libc::c_int {
    if (*v).size != (*p).size {
        gsl_error(
            b"vector and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_permute_ulong_inverse((*p).data, (*v).data, (*v).stride, (*v).size);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permute_matrix_uint(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_uint,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_uint_view = gsl_matrix_uint_row(A, i);
            gsl_permute_vector_uint(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_int(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_int,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_int_view = gsl_matrix_int_row(A, i);
            gsl_permute_vector_int(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_complex_long_double(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_complex_long_double,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_complex_long_double_view = gsl_matrix_complex_long_double_row(
                A,
                i,
            );
            gsl_permute_vector_complex_long_double(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_complex(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_complex_view = gsl_matrix_complex_row(A, i);
            gsl_permute_vector_complex(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_complex_float(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_complex_float_view = gsl_matrix_complex_float_row(
                A,
                i,
            );
            gsl_permute_vector_complex_float(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_long_double(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_long_double,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_long_double_view = gsl_matrix_long_double_row(A, i);
            gsl_permute_vector_long_double(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_view = gsl_matrix_row(A, i);
            gsl_permute_vector(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_float(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_float,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_float_view = gsl_matrix_float_row(A, i);
            gsl_permute_vector_float(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_ulong(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_ulong,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_ulong_view = gsl_matrix_ulong_row(A, i);
            gsl_permute_vector_ulong(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_long(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_long,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_long_view = gsl_matrix_long_row(A, i);
            gsl_permute_vector_long(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_char(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_char,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_char_view = gsl_matrix_char_row(A, i);
            gsl_permute_vector_char(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_uchar(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_uchar,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_uchar_view = gsl_matrix_uchar_row(A, i);
            gsl_permute_vector_uchar(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_short(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_short,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_short_view = gsl_matrix_short_row(A, i);
            gsl_permute_vector_short(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_permute_matrix_ushort(
    mut p: *const gsl_permutation,
    mut A: *mut gsl_matrix_ushort,
) -> libc::c_int {
    if (*A).size2 != (*p).size {
        gsl_error(
            b"matrix columns and permutation must be the same length\0" as *const u8
                as *const libc::c_char,
            b"./permute_source.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut r: gsl_vector_ushort_view = gsl_matrix_ushort_row(A, i);
            gsl_permute_vector_ushort(p, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}

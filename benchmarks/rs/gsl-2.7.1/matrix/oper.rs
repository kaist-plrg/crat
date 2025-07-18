use ::libc;
use ::num_traits;
use ::f128;
use num_traits::ToPrimitive;
extern "C" {
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_vector_float_scale(a: *mut gsl_vector_float, x: libc::c_float) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_long_double_scale(
        a: *mut gsl_vector_long_double,
        x: f128::f128,
    ) -> libc::c_int;
    fn gsl_vector_complex_long_double_scale(
        a: *mut gsl_vector_complex_long_double,
        x: gsl_complex_long_double,
    ) -> libc::c_int;
    fn gsl_matrix_complex_long_double_row(
        m: *mut gsl_matrix_complex_long_double,
        i: size_t,
    ) -> _gsl_vector_complex_long_double_view;
    fn gsl_matrix_complex_long_double_column(
        m: *mut gsl_matrix_complex_long_double,
        j: size_t,
    ) -> _gsl_vector_complex_long_double_view;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_vector_complex_scale(
        a: *mut gsl_vector_complex,
        x: gsl_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_row(
        m: *mut gsl_matrix_complex,
        i: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_column(
        m: *mut gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_vector_complex_float_scale(
        a: *mut gsl_vector_complex_float,
        x: gsl_complex_float,
    ) -> libc::c_int;
    fn gsl_matrix_complex_float_row(
        m: *mut gsl_matrix_complex_float,
        i: size_t,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_complex_float_column(
        m: *mut gsl_matrix_complex_float,
        j: size_t,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_long_double_row(
        m: *mut gsl_matrix_long_double,
        i: size_t,
    ) -> _gsl_vector_long_double_view;
    fn gsl_matrix_long_double_column(
        m: *mut gsl_matrix_long_double,
        j: size_t,
    ) -> _gsl_vector_long_double_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_float_row(
        m: *mut gsl_matrix_float,
        i: size_t,
    ) -> _gsl_vector_float_view;
    fn gsl_matrix_float_column(
        m: *mut gsl_matrix_float,
        j: size_t,
    ) -> _gsl_vector_float_view;
    fn gsl_vector_ulong_scale(a: *mut gsl_vector_ulong, x: libc::c_ulong) -> libc::c_int;
    fn gsl_vector_long_scale(a: *mut gsl_vector_long, x: libc::c_long) -> libc::c_int;
    fn gsl_matrix_long_row(m: *mut gsl_matrix_long, i: size_t) -> _gsl_vector_long_view;
    fn gsl_matrix_long_column(
        m: *mut gsl_matrix_long,
        j: size_t,
    ) -> _gsl_vector_long_view;
    fn gsl_vector_uint_scale(a: *mut gsl_vector_uint, x: libc::c_uint) -> libc::c_int;
    fn gsl_matrix_uint_row(m: *mut gsl_matrix_uint, i: size_t) -> _gsl_vector_uint_view;
    fn gsl_matrix_uint_column(
        m: *mut gsl_matrix_uint,
        j: size_t,
    ) -> _gsl_vector_uint_view;
    fn gsl_vector_int_scale(a: *mut gsl_vector_int, x: libc::c_int) -> libc::c_int;
    fn gsl_matrix_int_row(m: *mut gsl_matrix_int, i: size_t) -> _gsl_vector_int_view;
    fn gsl_matrix_int_column(m: *mut gsl_matrix_int, j: size_t) -> _gsl_vector_int_view;
    fn gsl_vector_ushort_scale(
        a: *mut gsl_vector_ushort,
        x: libc::c_ushort,
    ) -> libc::c_int;
    fn gsl_matrix_ushort_row(
        m: *mut gsl_matrix_ushort,
        i: size_t,
    ) -> _gsl_vector_ushort_view;
    fn gsl_matrix_ushort_column(
        m: *mut gsl_matrix_ushort,
        j: size_t,
    ) -> _gsl_vector_ushort_view;
    fn gsl_vector_short_scale(a: *mut gsl_vector_short, x: libc::c_short) -> libc::c_int;
    fn gsl_matrix_short_row(
        m: *mut gsl_matrix_short,
        i: size_t,
    ) -> _gsl_vector_short_view;
    fn gsl_matrix_short_column(
        m: *mut gsl_matrix_short,
        j: size_t,
    ) -> _gsl_vector_short_view;
    fn gsl_vector_uchar_scale(a: *mut gsl_vector_uchar, x: libc::c_uchar) -> libc::c_int;
    fn gsl_matrix_uchar_row(
        m: *mut gsl_matrix_uchar,
        i: size_t,
    ) -> _gsl_vector_uchar_view;
    fn gsl_matrix_uchar_column(
        m: *mut gsl_matrix_uchar,
        j: size_t,
    ) -> _gsl_vector_uchar_view;
    fn gsl_matrix_char_column(
        m: *mut gsl_matrix_char,
        j: size_t,
    ) -> _gsl_vector_char_view;
    fn gsl_matrix_char_row(m: *mut gsl_matrix_char, i: size_t) -> _gsl_vector_char_view;
    fn gsl_vector_char_scale(a: *mut gsl_vector_char, x: libc::c_char) -> libc::c_int;
    fn gsl_matrix_ulong_row(
        m: *mut gsl_matrix_ulong,
        i: size_t,
    ) -> _gsl_vector_ulong_view;
    fn gsl_matrix_ulong_column(
        m: *mut gsl_matrix_ulong,
        j: size_t,
    ) -> _gsl_vector_ulong_view;
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
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
#[inline]
unsafe extern "C" fn gsl_vector_long_double_get(
    mut v: *const gsl_vector_long_double,
    i: size_t,
) -> f128::f128 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_long_double_get(
    mut v: *const gsl_vector_complex_long_double,
    i: size_t,
) -> gsl_complex_long_double {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut f128::f128 as *mut gsl_complex_long_double);
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_vector_float_get(
    mut v: *const gsl_vector_float,
    i: size_t,
) -> libc::c_float {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_float_get(
    mut v: *const gsl_vector_complex_float,
    i: size_t,
) -> gsl_complex_float {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_float as *mut gsl_complex_float);
}
#[inline]
unsafe extern "C" fn gsl_vector_ulong_get(
    mut v: *const gsl_vector_ulong,
    i: size_t,
) -> libc::c_ulong {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_long_get(
    mut v: *const gsl_vector_long,
    i: size_t,
) -> libc::c_long {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_uint_get(
    mut v: *const gsl_vector_uint,
    i: size_t,
) -> libc::c_uint {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_int_get(
    mut v: *const gsl_vector_int,
    i: size_t,
) -> libc::c_int {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_ushort_get(
    mut v: *const gsl_vector_ushort,
    i: size_t,
) -> libc::c_ushort {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_short_get(
    mut v: *const gsl_vector_short,
    i: size_t,
) -> libc::c_short {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_uchar_get(
    mut v: *const gsl_vector_uchar,
    i: size_t,
) -> libc::c_uchar {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_char_get(
    mut v: *const gsl_vector_char,
    i: size_t,
) -> libc::c_char {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_add(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                *((*a).data).offset(aij as isize) += *((*b).data).offset(bij as isize);
                *((*a).data)
                    .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    += *((*b).data)
                        .offset(
                            bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_float_add(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                *((*a).data).offset(aij as isize) += *((*b).data).offset(bij as isize);
                *((*a).data)
                    .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    += *((*b).data)
                        .offset(
                            bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_add(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                *((*a).data).offset(aij as isize) += *((*b).data).offset(bij as isize);
                *((*a).data)
                    .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    += *((*b).data)
                        .offset(
                            bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_sub(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                *((*a).data).offset(aij as isize) -= *((*b).data).offset(bij as isize);
                *((*a).data)
                    .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    -= *((*b).data)
                        .offset(
                            bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_sub(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                *((*a).data).offset(aij as isize) -= *((*b).data).offset(bij as isize);
                *((*a).data)
                    .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    -= *((*b).data)
                        .offset(
                            bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_float_sub(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                *((*a).data).offset(aij as isize) -= *((*b).data).offset(bij as isize);
                *((*a).data)
                    .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    -= *((*b).data)
                        .offset(
                            bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_mul_elements(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                let mut ar: f128::f128 = *((*a).data).offset(aij as isize);
                let mut ai: f128::f128 = *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut br: f128::f128 = *((*b).data).offset(bij as isize);
                let mut bi: f128::f128 = *((*b).data)
                    .offset(
                        bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *((*a).data).offset(aij as isize) = ar * br - ai * bi;
                *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ar * bi + ai * br;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_mul_elements(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                let mut ar: libc::c_double = *((*a).data).offset(aij as isize);
                let mut ai: libc::c_double = *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut br: libc::c_double = *((*b).data).offset(bij as isize);
                let mut bi: libc::c_double = *((*b).data)
                    .offset(
                        bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *((*a).data).offset(aij as isize) = ar * br - ai * bi;
                *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ar * bi + ai * br;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_float_mul_elements(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                let mut ar: libc::c_float = *((*a).data).offset(aij as isize);
                let mut ai: libc::c_float = *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut br: libc::c_float = *((*b).data).offset(bij as isize);
                let mut bi: libc::c_float = *((*b).data)
                    .offset(
                        bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *((*a).data).offset(aij as isize) = ar * br - ai * bi;
                *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ar * bi + ai * br;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_float_div_elements(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                let mut ar: libc::c_float = *((*a).data).offset(aij as isize);
                let mut ai: libc::c_float = *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut br: libc::c_float = *((*b).data).offset(bij as isize);
                let mut bi: libc::c_float = *((*b).data)
                    .offset(
                        bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut s: libc::c_float = (1.0f64
                    / hypot(br as libc::c_double, bi as libc::c_double))
                    as libc::c_float;
                let mut sbr: libc::c_float = s * br;
                let mut sbi: libc::c_float = s * bi;
                *((*a).data).offset(aij as isize) = (ar * sbr + ai * sbi) * s;
                *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = (ai * sbr - ar * sbi) * s;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_div_elements(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                let mut ar: libc::c_double = *((*a).data).offset(aij as isize);
                let mut ai: libc::c_double = *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut br: libc::c_double = *((*b).data).offset(bij as isize);
                let mut bi: libc::c_double = *((*b).data)
                    .offset(
                        bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut s: libc::c_double = 1.0f64 / hypot(br, bi);
                let mut sbr: libc::c_double = s * br;
                let mut sbi: libc::c_double = s * bi;
                *((*a).data).offset(aij as isize) = (ar * sbr + ai * sbi) * s;
                *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = (ai * sbr - ar * sbi) * s;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_div_elements(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_a).wrapping_add(j));
                let bij: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda_b).wrapping_add(j));
                let mut ar: f128::f128 = *((*a).data).offset(aij as isize);
                let mut ai: f128::f128 = *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut br: f128::f128 = *((*b).data).offset(bij as isize);
                let mut bi: f128::f128 = *((*b).data)
                    .offset(
                        bij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut s: f128::f128 = f128::f128::new(
                    1.0f64 / hypot(br.to_f64().unwrap(), bi.to_f64().unwrap()),
                );
                let mut sbr: f128::f128 = s * br;
                let mut sbi: f128::f128 = s * bi;
                *((*a).data).offset(aij as isize) = (ar * sbr + ai * sbi) * s;
                *((*a).data)
                    .offset(
                        aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = (ai * sbr - ar * sbi) * s;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_scale(
    mut a: *mut gsl_matrix_complex,
    x: gsl_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xr: libc::c_double = x.dat[0 as libc::c_int as usize];
    let mut xi: libc::c_double = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j));
            let mut ar: libc::c_double = *((*a).data).offset(aij as isize);
            let mut ai: libc::c_double = *((*a).data)
                .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *((*a).data).offset(aij as isize) = ar * xr - ai * xi;
            *((*a).data)
                .offset(
                    aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ar * xi + ai * xr;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_scale(
    mut a: *mut gsl_matrix_complex_long_double,
    x: gsl_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xr: f128::f128 = x.dat[0 as libc::c_int as usize];
    let mut xi: f128::f128 = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j));
            let mut ar: f128::f128 = *((*a).data).offset(aij as isize);
            let mut ai: f128::f128 = *((*a).data)
                .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *((*a).data).offset(aij as isize) = ar * xr - ai * xi;
            *((*a).data)
                .offset(
                    aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ar * xi + ai * xr;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_scale(
    mut a: *mut gsl_matrix_complex_float,
    x: gsl_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut xr: libc::c_float = x.dat[0 as libc::c_int as usize];
    let mut xi: libc::c_float = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let aij: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j));
            let mut ar: libc::c_float = *((*a).data).offset(aij as isize);
            let mut ai: libc::c_float = *((*a).data)
                .offset(aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *((*a).data).offset(aij as isize) = ar * xr - ai * xi;
            *((*a).data)
                .offset(
                    aij.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ar * xi + ai * xr;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_scale_rows(
    mut a: *mut gsl_matrix_complex_long_double,
    mut x: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: gsl_complex_long_double = gsl_vector_complex_long_double_get(x, i);
            let mut v: gsl_vector_complex_long_double_view = gsl_matrix_complex_long_double_row(
                a,
                i,
            );
            gsl_vector_complex_long_double_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_scale_rows(
    mut a: *mut gsl_matrix_complex,
    mut x: *const gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: gsl_complex = gsl_vector_complex_get(x, i);
            let mut v: gsl_vector_complex_view = gsl_matrix_complex_row(a, i);
            gsl_vector_complex_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_float_scale_rows(
    mut a: *mut gsl_matrix_complex_float,
    mut x: *const gsl_vector_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: gsl_complex_float = gsl_vector_complex_float_get(x, i);
            let mut v: gsl_vector_complex_float_view = gsl_matrix_complex_float_row(
                a,
                i,
            );
            gsl_vector_complex_float_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_scale_columns(
    mut a: *mut gsl_matrix_complex_long_double,
    mut x: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: gsl_complex_long_double = gsl_vector_complex_long_double_get(x, i);
            let mut v: gsl_vector_complex_long_double_view = gsl_matrix_complex_long_double_column(
                a,
                i,
            );
            gsl_vector_complex_long_double_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_scale_columns(
    mut a: *mut gsl_matrix_complex,
    mut x: *const gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: gsl_complex = gsl_vector_complex_get(x, i);
            let mut v: gsl_vector_complex_view = gsl_matrix_complex_column(a, i);
            gsl_vector_complex_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_float_scale_columns(
    mut a: *mut gsl_matrix_complex_float,
    mut x: *const gsl_vector_complex_float,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: gsl_complex_float = gsl_vector_complex_float_get(x, i);
            let mut v: gsl_vector_complex_float_view = gsl_matrix_complex_float_column(
                a,
                i,
            );
            gsl_vector_complex_float_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_add_constant(
    mut a: *mut gsl_matrix_complex_long_double,
    x: gsl_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) += x.dat[0 as libc::c_int as usize];
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) += x.dat[1 as libc::c_int as usize];
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_add_constant(
    mut a: *mut gsl_matrix_complex,
    x: gsl_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) += x.dat[0 as libc::c_int as usize];
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) += x.dat[1 as libc::c_int as usize];
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_add_constant(
    mut a: *mut gsl_matrix_complex_float,
    x: gsl_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) += x.dat[0 as libc::c_int as usize];
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) += x.dat[1 as libc::c_int as usize];
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_add_diagonal(
    mut a: *mut gsl_matrix_complex_long_double,
    x: gsl_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda).wrapping_add(i)) as isize,
            ) += x.dat[0 as libc::c_int as usize];
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda).wrapping_add(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) += x.dat[1 as libc::c_int as usize];
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_add_diagonal(
    mut a: *mut gsl_matrix_complex,
    x: gsl_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda).wrapping_add(i)) as isize,
            ) += x.dat[0 as libc::c_int as usize];
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda).wrapping_add(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) += x.dat[1 as libc::c_int as usize];
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_add_diagonal(
    mut a: *mut gsl_matrix_complex_float,
    x: gsl_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda).wrapping_add(i)) as isize,
            ) += x.dat[0 as libc::c_int as usize];
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i.wrapping_mul(tda).wrapping_add(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) += x.dat[1 as libc::c_int as usize];
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_add(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    += *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_add(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    += *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_char_add(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_matrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh0 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh0 = (*fresh0 as libc::c_int
                    + *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_char;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_add(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_matrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    += *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_int_add(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_matrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    += *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_double_add(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_matrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    += *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_add(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_matrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh1 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh1 = (*fresh1 as libc::c_int
                    + *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ushort_add(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_matrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh2 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh2 = (*fresh2 as libc::c_int
                    + *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_ushort;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ulong_add(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_matrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh3 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh3 = (*fresh3)
                    .wrapping_add(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_add(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_matrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh4 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh4 = (*fresh4 as libc::c_int
                    + *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_short;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uint_add(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_matrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh5 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh5 = (*fresh5)
                    .wrapping_add(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_sub(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_matrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh6 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh6 = (*fresh6 as libc::c_int
                    - *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_short;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ulong_sub(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_matrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh7 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh7 = (*fresh7)
                    .wrapping_sub(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uint_sub(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_matrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh8 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh8 = (*fresh8)
                    .wrapping_sub(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_char_sub(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_matrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh9 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh9 = (*fresh9 as libc::c_int
                    - *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_char;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_sub(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    -= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_double_sub(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_matrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    -= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_int_sub(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_matrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    -= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_sub(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_matrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    -= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_sub(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_matrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh10 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh10 = (*fresh10 as libc::c_int
                    - *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_float_sub(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    -= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ushort_sub(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_matrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh11 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh11 = (*fresh11 as libc::c_int
                    - *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_ushort;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uint_mul_elements(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_matrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh12 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh12 = (*fresh12)
                    .wrapping_mul(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ulong_mul_elements(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_matrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh13 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh13 = (*fresh13)
                    .wrapping_mul(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_mul_elements(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_matrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    *= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ushort_mul_elements(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_matrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh14 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh14 = (*fresh14 as libc::c_int
                    * *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_ushort;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_mul_elements(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    *= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_mul_elements(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_matrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh15 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh15 = (*fresh15 as libc::c_int
                    * *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_mul_elements(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_matrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh16 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh16 = (*fresh16 as libc::c_int
                    * *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_short;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_int_mul_elements(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_matrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    *= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_double_mul_elements(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_matrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    *= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_char_mul_elements(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_matrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh17 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh17 = (*fresh17 as libc::c_int
                    * *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_char;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_float_mul_elements(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    *= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_div_elements(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_matrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    /= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_int_div_elements(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_matrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    /= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ulong_div_elements(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_matrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh18 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh18 = (*fresh18)
                    .wrapping_div(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_div_elements(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    /= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_char_div_elements(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_matrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh19 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh19 = (*fresh19 as libc::c_int
                    / *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_char;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_div_elements(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_matrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh20 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh20 = (*fresh20 as libc::c_int
                    / *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_float_div_elements(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    /= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_div_elements(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_matrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh21 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh21 = (*fresh21 as libc::c_int
                    / *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_short;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_double_div_elements(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_matrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                *((*a).data).offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize)
                    /= *((*b).data)
                        .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ushort_div_elements(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_matrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh22 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh22 = (*fresh22 as libc::c_int
                    / *((*b).data).offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize)
                        as libc::c_int) as libc::c_ushort;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uint_div_elements(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_matrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < N {
                let ref mut fresh23 = *((*a).data)
                    .offset(i.wrapping_mul(tda_a).wrapping_add(j) as isize);
                *fresh23 = (*fresh23)
                    .wrapping_div(
                        *((*b).data)
                            .offset(i.wrapping_mul(tda_b).wrapping_add(j) as isize),
                    );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_scale(
    mut a: *mut gsl_matrix_short,
    x: libc::c_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh24 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh24 = (*fresh24 as libc::c_int * x as libc::c_int) as libc::c_short;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_scale(
    mut a: *mut gsl_matrix_uint,
    x: libc::c_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh25 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh25 = (*fresh25).wrapping_mul(x);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_scale(
    mut a: *mut gsl_matrix_ulong,
    x: libc::c_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh26 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh26 = (*fresh26).wrapping_mul(x);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_scale(
    mut a: *mut gsl_matrix_int,
    x: libc::c_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) *= x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_scale(
    mut a: *mut gsl_matrix_long_double,
    x: f128::f128,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) *= x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_scale(
    mut a: *mut gsl_matrix_float,
    x: libc::c_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) *= x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_scale(
    mut a: *mut gsl_matrix_ushort,
    x: libc::c_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh27 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh27 = (*fresh27 as libc::c_int * x as libc::c_int) as libc::c_ushort;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_scale(
    mut a: *mut gsl_matrix_char,
    x: libc::c_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh28 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh28 = (*fresh28 as libc::c_int * x as libc::c_int) as libc::c_char;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_scale(
    mut a: *mut gsl_matrix,
    x: libc::c_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) *= x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_scale(
    mut a: *mut gsl_matrix_uchar,
    x: libc::c_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh29 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh29 = (*fresh29 as libc::c_int * x as libc::c_int) as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_scale(
    mut a: *mut gsl_matrix_long,
    x: libc::c_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) *= x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_scale_rows(
    mut a: *mut gsl_matrix_long_double,
    mut x: *const gsl_vector_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: f128::f128 = gsl_vector_long_double_get(x, i);
            let mut v: gsl_vector_long_double_view = gsl_matrix_long_double_row(a, i);
            gsl_vector_long_double_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_scale_rows(
    mut a: *mut gsl_matrix_short,
    mut x: *const gsl_vector_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_short = gsl_vector_short_get(x, i);
            let mut v: gsl_vector_short_view = gsl_matrix_short_row(a, i);
            gsl_vector_short_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_int_scale_rows(
    mut a: *mut gsl_matrix_int,
    mut x: *const gsl_vector_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_int = gsl_vector_int_get(x, i);
            let mut v: gsl_vector_int_view = gsl_matrix_int_row(a, i);
            gsl_vector_int_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_scale_rows(
    mut a: *mut gsl_matrix_uchar,
    mut x: *const gsl_vector_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_uchar = gsl_vector_uchar_get(x, i);
            let mut v: gsl_vector_uchar_view = gsl_matrix_uchar_row(a, i);
            gsl_vector_uchar_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uint_scale_rows(
    mut a: *mut gsl_matrix_uint,
    mut x: *const gsl_vector_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_uint = gsl_vector_uint_get(x, i);
            let mut v: gsl_vector_uint_view = gsl_matrix_uint_row(a, i);
            gsl_vector_uint_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_scale_rows(
    mut a: *mut gsl_matrix,
    mut x: *const gsl_vector,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_double = gsl_vector_get(x, i);
            let mut v: gsl_vector_view = gsl_matrix_row(a, i);
            gsl_vector_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_scale_rows(
    mut a: *mut gsl_matrix_long,
    mut x: *const gsl_vector_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_long = gsl_vector_long_get(x, i);
            let mut v: gsl_vector_long_view = gsl_matrix_long_row(a, i);
            gsl_vector_long_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ushort_scale_rows(
    mut a: *mut gsl_matrix_ushort,
    mut x: *const gsl_vector_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_ushort = gsl_vector_ushort_get(x, i);
            let mut v: gsl_vector_ushort_view = gsl_matrix_ushort_row(a, i);
            gsl_vector_ushort_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_float_scale_rows(
    mut a: *mut gsl_matrix_float,
    mut x: *const gsl_vector_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_float = gsl_vector_float_get(x, i);
            let mut v: gsl_vector_float_view = gsl_matrix_float_row(a, i);
            gsl_vector_float_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ulong_scale_rows(
    mut a: *mut gsl_matrix_ulong,
    mut x: *const gsl_vector_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_ulong = gsl_vector_ulong_get(x, i);
            let mut v: gsl_vector_ulong_view = gsl_matrix_ulong_row(a, i);
            gsl_vector_ulong_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_char_scale_rows(
    mut a: *mut gsl_matrix_char,
    mut x: *const gsl_vector_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    if (*x).size != M {
        gsl_error(
            b"x must match number of rows of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let xi: libc::c_char = gsl_vector_char_get(x, i);
            let mut v: gsl_vector_char_view = gsl_matrix_char_row(a, i);
            gsl_vector_char_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_double_scale_columns(
    mut a: *mut gsl_matrix_long_double,
    mut x: *const gsl_vector_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: f128::f128 = gsl_vector_long_double_get(x, i);
            let mut v: gsl_vector_long_double_view = gsl_matrix_long_double_column(a, i);
            gsl_vector_long_double_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_short_scale_columns(
    mut a: *mut gsl_matrix_short,
    mut x: *const gsl_vector_short,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_short = gsl_vector_short_get(x, i);
            let mut v: gsl_vector_short_view = gsl_matrix_short_column(a, i);
            gsl_vector_short_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ulong_scale_columns(
    mut a: *mut gsl_matrix_ulong,
    mut x: *const gsl_vector_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_ulong = gsl_vector_ulong_get(x, i);
            let mut v: gsl_vector_ulong_view = gsl_matrix_ulong_column(a, i);
            gsl_vector_ulong_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_ushort_scale_columns(
    mut a: *mut gsl_matrix_ushort,
    mut x: *const gsl_vector_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_ushort = gsl_vector_ushort_get(x, i);
            let mut v: gsl_vector_ushort_view = gsl_matrix_ushort_column(a, i);
            gsl_vector_ushort_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_long_scale_columns(
    mut a: *mut gsl_matrix_long,
    mut x: *const gsl_vector_long,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_long = gsl_vector_long_get(x, i);
            let mut v: gsl_vector_long_view = gsl_matrix_long_column(a, i);
            gsl_vector_long_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_float_scale_columns(
    mut a: *mut gsl_matrix_float,
    mut x: *const gsl_vector_float,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_float = gsl_vector_float_get(x, i);
            let mut v: gsl_vector_float_view = gsl_matrix_float_column(a, i);
            gsl_vector_float_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_char_scale_columns(
    mut a: *mut gsl_matrix_char,
    mut x: *const gsl_vector_char,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_char = gsl_vector_char_get(x, i);
            let mut v: gsl_vector_char_view = gsl_matrix_char_column(a, i);
            gsl_vector_char_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_scale_columns(
    mut a: *mut gsl_matrix,
    mut x: *const gsl_vector,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_double = gsl_vector_get(x, i);
            let mut v: gsl_vector_view = gsl_matrix_column(a, i);
            gsl_vector_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uint_scale_columns(
    mut a: *mut gsl_matrix_uint,
    mut x: *const gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_uint = gsl_vector_uint_get(x, i);
            let mut v: gsl_vector_uint_view = gsl_matrix_uint_column(a, i);
            gsl_vector_uint_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_int_scale_columns(
    mut a: *mut gsl_matrix_int,
    mut x: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_int = gsl_vector_int_get(x, i);
            let mut v: gsl_vector_int_view = gsl_matrix_int_column(a, i);
            gsl_vector_int_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_scale_columns(
    mut a: *mut gsl_matrix_uchar,
    mut x: *const gsl_vector_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size2;
    if (*x).size != N {
        gsl_error(
            b"x must match number of columns of A\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let xi: libc::c_uchar = gsl_vector_uchar_get(x, i);
            let mut v: gsl_vector_uchar_view = gsl_matrix_uchar_column(a, i);
            gsl_vector_uchar_scale(&mut v.vector, xi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_matrix_uchar_add_constant(
    mut a: *mut gsl_matrix_uchar,
    x: libc::c_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh30 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh30 = (*fresh30 as libc::c_int + x as libc::c_int) as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_add_constant(
    mut a: *mut gsl_matrix_short,
    x: libc::c_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh31 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh31 = (*fresh31 as libc::c_int + x as libc::c_int) as libc::c_short;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_add_constant(
    mut a: *mut gsl_matrix,
    x: libc::c_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) += x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_add_constant(
    mut a: *mut gsl_matrix_long_double,
    x: f128::f128,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) += x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_add_constant(
    mut a: *mut gsl_matrix_int,
    x: libc::c_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) += x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_add_constant(
    mut a: *mut gsl_matrix_ulong,
    x: libc::c_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh32 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh32 = (*fresh32).wrapping_add(x);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_add_constant(
    mut a: *mut gsl_matrix_uint,
    x: libc::c_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh33 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh33 = (*fresh33).wrapping_add(x);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_add_constant(
    mut a: *mut gsl_matrix_char,
    x: libc::c_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh34 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh34 = (*fresh34 as libc::c_int + x as libc::c_int) as libc::c_char;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_add_constant(
    mut a: *mut gsl_matrix_float,
    x: libc::c_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) += x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_add_constant(
    mut a: *mut gsl_matrix_long,
    x: libc::c_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(j) as isize) += x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_add_constant(
    mut a: *mut gsl_matrix_ushort,
    x: libc::c_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let ref mut fresh35 = *((*a).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            *fresh35 = (*fresh35 as libc::c_int + x as libc::c_int) as libc::c_ushort;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_add_diagonal(
    mut a: *mut gsl_matrix_ushort,
    x: libc::c_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        let ref mut fresh36 = *((*a).data)
            .offset(i.wrapping_mul(tda).wrapping_add(i) as isize);
        *fresh36 = (*fresh36 as libc::c_int + x as libc::c_int) as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_add_diagonal(
    mut a: *mut gsl_matrix_char,
    x: libc::c_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        let ref mut fresh37 = *((*a).data)
            .offset(i.wrapping_mul(tda).wrapping_add(i) as isize);
        *fresh37 = (*fresh37 as libc::c_int + x as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_add_diagonal(
    mut a: *mut gsl_matrix_ulong,
    x: libc::c_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        let ref mut fresh38 = *((*a).data)
            .offset(i.wrapping_mul(tda).wrapping_add(i) as isize);
        *fresh38 = (*fresh38).wrapping_add(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_add_diagonal(
    mut a: *mut gsl_matrix_long,
    x: libc::c_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(i) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_add_diagonal(
    mut a: *mut gsl_matrix_uchar,
    x: libc::c_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        let ref mut fresh39 = *((*a).data)
            .offset(i.wrapping_mul(tda).wrapping_add(i) as isize);
        *fresh39 = (*fresh39 as libc::c_int + x as libc::c_int) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_add_diagonal(
    mut a: *mut gsl_matrix,
    x: libc::c_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(i) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_add_diagonal(
    mut a: *mut gsl_matrix_short,
    x: libc::c_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        let ref mut fresh40 = *((*a).data)
            .offset(i.wrapping_mul(tda).wrapping_add(i) as isize);
        *fresh40 = (*fresh40 as libc::c_int + x as libc::c_int) as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_add_diagonal(
    mut a: *mut gsl_matrix_int,
    x: libc::c_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(i) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_add_diagonal(
    mut a: *mut gsl_matrix_uint,
    x: libc::c_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        let ref mut fresh41 = *((*a).data)
            .offset(i.wrapping_mul(tda).wrapping_add(i) as isize);
        *fresh41 = (*fresh41).wrapping_add(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_add_diagonal(
    mut a: *mut gsl_matrix_long_double,
    x: f128::f128,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(i) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_add_diagonal(
    mut a: *mut gsl_matrix_float,
    x: libc::c_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let tda: size_t = (*a).tda;
    let loop_lim: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < loop_lim {
        *((*a).data).offset(i.wrapping_mul(tda).wrapping_add(i) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}

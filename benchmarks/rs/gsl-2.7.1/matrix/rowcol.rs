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
pub struct _gsl_vector_long_double_view {
    pub vector: gsl_vector_long_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_long_double_const_view {
    pub vector: gsl_vector_long_double,
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
pub struct _gsl_vector_complex_long_double_view {
    pub vector: gsl_vector_complex_long_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_long_double_const_view {
    pub vector: gsl_vector_complex_long_double,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
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
pub struct _gsl_vector_complex_view {
    pub vector: gsl_vector_complex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_const_view {
    pub vector: gsl_vector_complex,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_float_const_view {
    pub vector: gsl_vector_float,
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
pub struct _gsl_vector_complex_float_view {
    pub vector: gsl_vector_complex_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_float_const_view {
    pub vector: gsl_vector_complex_float,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_ulong_const_view {
    pub vector: gsl_vector_ulong,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_long_const_view {
    pub vector: gsl_vector_long,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_uint_const_view {
    pub vector: gsl_vector_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_int_const_view {
    pub vector: gsl_vector_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_ushort_const_view {
    pub vector: gsl_vector_ushort,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_short_const_view {
    pub vector: gsl_vector_short,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_uchar_const_view {
    pub vector: gsl_vector_uchar,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_char_const_view {
    pub vector: gsl_vector_char,
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
pub unsafe extern "C" fn gsl_matrix_long_row(
    mut m: *mut gsl_matrix_long,
    i: size_t,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_row(
    mut m: *const gsl_matrix_int,
    i: size_t,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_row(
    mut m: *const gsl_matrix,
    i: size_t,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_row(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_row(
    mut m: *mut gsl_matrix_float,
    i: size_t,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_row(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_row(
    mut m: *const gsl_matrix_float,
    i: size_t,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_row(
    mut m: *const gsl_matrix_char,
    i: size_t,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_row(
    mut m: *mut gsl_matrix_char,
    i: size_t,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_row(
    mut m: *const gsl_matrix_short,
    i: size_t,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_row(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_row(
    mut m: *const gsl_matrix_ulong,
    i: size_t,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_row(
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_row(
    mut m: *mut gsl_matrix,
    i: size_t,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_row(
    mut m: *const gsl_matrix_long,
    i: size_t,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_row(
    mut m: *const gsl_matrix_long_double,
    i: size_t,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_row(
    mut m: *mut gsl_matrix_short,
    i: size_t,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_row(
    mut m: *const gsl_matrix_complex,
    i: size_t,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_row(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_row(
    mut m: *const gsl_matrix_uchar,
    i: size_t,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_row(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_row(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_row(
    mut m: *const gsl_matrix_ushort,
    i: size_t,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_row(
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_row(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_row(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_row(
    mut m: *const gsl_matrix_uint,
    i: size_t,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_row(
    mut m: *mut gsl_matrix_int,
    i: size_t,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            i.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v.size = (*m).size2;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_column(
    mut m: *const gsl_matrix_ulong,
    j: size_t,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_column(
    mut m: *const gsl_matrix_complex,
    j: size_t,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_column(
    mut m: *const gsl_matrix_ushort,
    j: size_t,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_column(
    mut m: *const gsl_matrix,
    j: size_t,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_column(
    mut m: *mut gsl_matrix_float,
    j: size_t,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_column(
    mut m: *const gsl_matrix_char,
    j: size_t,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_column(
    mut m: *const gsl_matrix_float,
    j: size_t,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_column(
    mut m: *mut gsl_matrix_ushort,
    j: size_t,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_column(
    mut m: *mut gsl_matrix_complex_float,
    j: size_t,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_column(
    mut m: *const gsl_matrix_short,
    j: size_t,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_column(
    mut m: *mut gsl_matrix_complex_long_double,
    j: size_t,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_column(
    mut m: *mut gsl_matrix_char,
    j: size_t,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_column(
    mut m: *const gsl_matrix_int,
    j: size_t,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_column(
    mut m: *mut gsl_matrix_complex,
    j: size_t,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_column(
    mut m: *mut gsl_matrix_ulong,
    j: size_t,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_column(
    mut m: *mut gsl_matrix,
    j: size_t,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_column(
    mut m: *mut gsl_matrix_int,
    j: size_t,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_column(
    mut m: *mut gsl_matrix_long,
    j: size_t,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_column(
    mut m: *const gsl_matrix_complex_float,
    j: size_t,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_column(
    mut m: *const gsl_matrix_long_double,
    j: size_t,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_column(
    mut m: *mut gsl_matrix_uint,
    j: size_t,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_column(
    mut m: *mut gsl_matrix_uchar,
    j: size_t,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_column(
    mut m: *const gsl_matrix_uchar,
    j: size_t,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_column(
    mut m: *const gsl_matrix_uint,
    j: size_t,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_column(
    mut m: *mut gsl_matrix_short,
    j: size_t,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_column(
    mut m: *const gsl_matrix_complex_long_double,
    j: size_t,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_column(
    mut m: *mut gsl_matrix_long_double,
    j: size_t,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_column(
    mut m: *const gsl_matrix_long,
    j: size_t,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(j.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v.size = (*m).size1;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_diagonal(
    mut m: *const gsl_matrix_uchar,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_diagonal(
    mut m: *mut gsl_matrix_complex_float,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_diagonal(
    mut m: *mut gsl_matrix,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_diagonal(
    mut m: *const gsl_matrix_uint,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_diagonal(
    mut m: *const gsl_matrix_complex_long_double,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_diagonal(
    mut m: *const gsl_matrix,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_diagonal(
    mut m: *mut gsl_matrix_uint,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_diagonal(
    mut m: *const gsl_matrix_char,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_diagonal(
    mut m: *mut gsl_matrix_long_double,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_diagonal(
    mut m: *mut gsl_matrix_float,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_diagonal(
    mut m: *const gsl_matrix_short,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_diagonal(
    mut m: *const gsl_matrix_long,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_diagonal(
    mut m: *const gsl_matrix_ushort,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_diagonal(
    mut m: *const gsl_matrix_complex,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_diagonal(
    mut m: *const gsl_matrix_long_double,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_diagonal(
    mut m: *mut gsl_matrix_ushort,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_diagonal(
    mut m: *mut gsl_matrix_long,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_diagonal(
    mut m: *const gsl_matrix_complex_float,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_diagonal(
    mut m: *const gsl_matrix_float,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_diagonal(
    mut m: *const gsl_matrix_int,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_diagonal(
    mut m: *mut gsl_matrix_char,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_diagonal(
    mut m: *mut gsl_matrix_short,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_diagonal(
    mut m: *mut gsl_matrix_complex,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_diagonal(
    mut m: *mut gsl_matrix_complex_long_double,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_diagonal(
    mut m: *const gsl_matrix_ulong,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_diagonal(
    mut m: *mut gsl_matrix_uchar,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_diagonal(
    mut m: *mut gsl_matrix_int,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_diagonal(
    mut m: *mut gsl_matrix_ulong,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v.data = (*m).data;
    v.size = if (*m).size1 < (*m).size2 { (*m).size1 } else { (*m).size2 };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_subdiagonal(
    mut m: *mut gsl_matrix_long,
    k: size_t,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_subdiagonal(
    mut m: *const gsl_matrix_int,
    k: size_t,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_subdiagonal(
    mut m: *mut gsl_matrix_complex_long_double,
    k: size_t,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_subdiagonal(
    mut m: *const gsl_matrix_ulong,
    k: size_t,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_subdiagonal(
    mut m: *mut gsl_matrix_short,
    k: size_t,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_subdiagonal(
    mut m: *mut gsl_matrix_int,
    k: size_t,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_subdiagonal(
    mut m: *mut gsl_matrix_complex_float,
    k: size_t,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_subdiagonal(
    mut m: *const gsl_matrix_long_double,
    k: size_t,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_subdiagonal(
    mut m: *mut gsl_matrix_char,
    k: size_t,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_subdiagonal(
    mut m: *mut gsl_matrix_uchar,
    k: size_t,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_subdiagonal(
    mut m: *const gsl_matrix_complex_float,
    k: size_t,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_subdiagonal(
    mut m: *const gsl_matrix_float,
    k: size_t,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_subdiagonal(
    mut m: *const gsl_matrix_complex_long_double,
    k: size_t,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_subdiagonal(
    mut m: *mut gsl_matrix_ulong,
    k: size_t,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_subdiagonal(
    mut m: *mut gsl_matrix_complex,
    k: size_t,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_subdiagonal(
    mut m: *const gsl_matrix_long,
    k: size_t,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_subdiagonal(
    mut m: *mut gsl_matrix_float,
    k: size_t,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_subdiagonal(
    mut m: *mut gsl_matrix_long_double,
    k: size_t,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_subdiagonal(
    mut m: *mut gsl_matrix_ushort,
    k: size_t,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_subdiagonal(
    mut m: *const gsl_matrix_ushort,
    k: size_t,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_subdiagonal(
    mut m: *const gsl_matrix_short,
    k: size_t,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_subdiagonal(
    mut m: *const gsl_matrix,
    k: size_t,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_subdiagonal(
    mut m: *mut gsl_matrix_uint,
    k: size_t,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_subdiagonal(
    mut m: *const gsl_matrix_complex,
    k: size_t,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_subdiagonal(
    mut m: *const gsl_matrix_char,
    k: size_t,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_subdiagonal(
    mut m: *const gsl_matrix_uchar,
    k: size_t,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_subdiagonal(
    mut m: *mut gsl_matrix,
    k: size_t,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_subdiagonal(
    mut m: *const gsl_matrix_uint,
    k: size_t,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size1 {
        gsl_error(
            b"subdiagonal index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            k.wrapping_mul(1 as libc::c_int as libc::c_ulong).wrapping_mul((*m).tda)
                as isize,
        );
    v
        .size = if ((*m).size1).wrapping_sub(k) < (*m).size2 {
        ((*m).size1).wrapping_sub(k)
    } else {
        (*m).size2
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_superdiagonal(
    mut m: *mut gsl_matrix_ushort,
    k: size_t,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_superdiagonal(
    mut m: *mut gsl_matrix_long_double,
    k: size_t,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_superdiagonal(
    mut m: *const gsl_matrix_char,
    k: size_t,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_superdiagonal(
    mut m: *mut gsl_matrix_char,
    k: size_t,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_superdiagonal(
    mut m: *const gsl_matrix,
    k: size_t,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_superdiagonal(
    mut m: *const gsl_matrix_uchar,
    k: size_t,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_superdiagonal(
    mut m: *mut gsl_matrix_complex_long_double,
    k: size_t,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_superdiagonal(
    mut m: *mut gsl_matrix_uchar,
    k: size_t,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_superdiagonal(
    mut m: *mut gsl_matrix_float,
    k: size_t,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_superdiagonal(
    mut m: *const gsl_matrix_float,
    k: size_t,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_superdiagonal(
    mut m: *const gsl_matrix_complex_long_double,
    k: size_t,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_superdiagonal(
    mut m: *const gsl_matrix_short,
    k: size_t,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_superdiagonal(
    mut m: *mut gsl_matrix,
    k: size_t,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_superdiagonal(
    mut m: *mut gsl_matrix_ulong,
    k: size_t,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_superdiagonal(
    mut m: *mut gsl_matrix_short,
    k: size_t,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_superdiagonal(
    mut m: *mut gsl_matrix_complex,
    k: size_t,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_superdiagonal(
    mut m: *const gsl_matrix_ushort,
    k: size_t,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_superdiagonal(
    mut m: *const gsl_matrix_ulong,
    k: size_t,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_superdiagonal(
    mut m: *const gsl_matrix_complex,
    k: size_t,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_superdiagonal(
    mut m: *const gsl_matrix_int,
    k: size_t,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_superdiagonal(
    mut m: *const gsl_matrix_long_double,
    k: size_t,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_superdiagonal(
    mut m: *mut gsl_matrix_complex_float,
    k: size_t,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_superdiagonal(
    mut m: *const gsl_matrix_long,
    k: size_t,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_superdiagonal(
    mut m: *mut gsl_matrix_long,
    k: size_t,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_superdiagonal(
    mut m: *mut gsl_matrix_int,
    k: size_t,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_superdiagonal(
    mut m: *const gsl_matrix_uint,
    k: size_t,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_superdiagonal(
    mut m: *mut gsl_matrix_uint,
    k: size_t,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_superdiagonal(
    mut m: *const gsl_matrix_complex_float,
    k: size_t,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if k >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(k.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
    v
        .size = if (*m).size1 < ((*m).size2).wrapping_sub(k) {
        (*m).size1
    } else {
        ((*m).size2).wrapping_sub(k)
    };
    v.stride = ((*m).tda).wrapping_add(1 as libc::c_int as libc::c_ulong);
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_subrow(
    mut m: *const gsl_matrix_ulong,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_subrow(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_subrow(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_subrow(
    mut m: *const gsl_matrix,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_subrow(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_subrow(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_subrow(
    mut m: *const gsl_matrix_long,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_subrow(
    mut m: *const gsl_matrix_float,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_subrow(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_subrow(
    mut m: *const gsl_matrix_uchar,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_subrow(
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_subrow(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_subrow(
    mut m: *const gsl_matrix_uint,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_subrow(
    mut m: *const gsl_matrix_ushort,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_subrow(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_subrow(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_subrow(
    mut m: *const gsl_matrix_short,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_subrow(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_subrow(
    mut m: *mut gsl_matrix,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_subrow(
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_subrow(
    mut m: *const gsl_matrix_int,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_subrow(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_subrow(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_subrow(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_subrow(
    mut m: *const gsl_matrix_long_double,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_subrow(
    mut m: *const gsl_matrix_char,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_subrow(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_subrow(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size2 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(offset)) as isize,
        );
    v.size = n;
    v.stride = 1 as libc::c_int as size_t;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_subcolumn(
    mut m: *const gsl_matrix_uint,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uint_const_view {
    let mut view: _gsl_vector_uint_const_view = {
        let mut init = _gsl_vector_uint_const_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_subcolumn(
    mut m: *mut gsl_matrix_long,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_view {
    let mut view: _gsl_vector_long_view = {
        let mut init = _gsl_vector_long_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_subcolumn(
    mut m: *mut gsl_matrix_complex,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_view {
    let mut view: _gsl_vector_complex_view = {
        let mut init = _gsl_vector_complex_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_subcolumn(
    mut m: *const gsl_matrix_long_double,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_double_const_view {
    let mut view: _gsl_vector_long_double_const_view = {
        let mut init = _gsl_vector_long_double_const_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_subcolumn(
    mut m: *mut gsl_matrix_ushort,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ushort_view {
    let mut view: _gsl_vector_ushort_view = {
        let mut init = _gsl_vector_ushort_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_subcolumn(
    mut m: *const gsl_matrix_ushort,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ushort_const_view {
    let mut view: _gsl_vector_ushort_const_view = {
        let mut init = _gsl_vector_ushort_const_view {
            vector: {
                let mut init = gsl_vector_ushort {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ushort,
                    block: 0 as *mut gsl_block_ushort,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_subcolumn(
    mut m: *const gsl_matrix_char,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_char_const_view {
    let mut view: _gsl_vector_char_const_view = {
        let mut init = _gsl_vector_char_const_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_subcolumn(
    mut m: *const gsl_matrix_complex,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_const_view {
    let mut view: _gsl_vector_complex_const_view = {
        let mut init = _gsl_vector_complex_const_view {
            vector: {
                let mut init = gsl_vector_complex {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_subcolumn(
    mut m: *mut gsl_matrix_short,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_short_view {
    let mut view: _gsl_vector_short_view = {
        let mut init = _gsl_vector_short_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_subcolumn(
    mut m: *mut gsl_matrix_ulong,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ulong_view {
    let mut view: _gsl_vector_ulong_view = {
        let mut init = _gsl_vector_ulong_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_subcolumn(
    mut m: *const gsl_matrix_complex_long_double,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_long_double_const_view {
    let mut view: _gsl_vector_complex_long_double_const_view = {
        let mut init = _gsl_vector_complex_long_double_const_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_subcolumn(
    mut m: *const gsl_matrix_int,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_int_const_view {
    let mut view: _gsl_vector_int_const_view = {
        let mut init = _gsl_vector_int_const_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_subcolumn(
    mut m: *const gsl_matrix_float,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_float_const_view {
    let mut view: _gsl_vector_float_const_view = {
        let mut init = _gsl_vector_float_const_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_subcolumn(
    mut m: *mut gsl_matrix_int,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_int_view {
    let mut view: _gsl_vector_int_view = {
        let mut init = _gsl_vector_int_view {
            vector: {
                let mut init = gsl_vector_int {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_int,
                    block: 0 as *mut gsl_block_int,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_subcolumn(
    mut m: *const gsl_matrix_short,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_short_const_view {
    let mut view: _gsl_vector_short_const_view = {
        let mut init = _gsl_vector_short_const_view {
            vector: {
                let mut init = gsl_vector_short {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_short,
                    block: 0 as *mut gsl_block_short,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_subcolumn(
    mut m: *mut gsl_matrix,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_view {
    let mut view: _gsl_vector_view = {
        let mut init = _gsl_vector_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_subcolumn(
    mut m: *const gsl_matrix,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_const_view {
    let mut view: _gsl_vector_const_view = {
        let mut init = _gsl_vector_const_view {
            vector: {
                let mut init = gsl_vector {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_subcolumn(
    mut m: *mut gsl_matrix_uchar,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uchar_view {
    let mut view: _gsl_vector_uchar_view = {
        let mut init = _gsl_vector_uchar_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_subcolumn(
    mut m: *mut gsl_matrix_complex_long_double,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_long_double_view {
    let mut view: _gsl_vector_complex_long_double_view = {
        let mut init = _gsl_vector_complex_long_double_view {
            vector: {
                let mut init = gsl_vector_complex_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_complex_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_subcolumn(
    mut m: *mut gsl_matrix_complex_float,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_float_view {
    let mut view: _gsl_vector_complex_float_view = {
        let mut init = _gsl_vector_complex_float_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_subcolumn(
    mut m: *const gsl_matrix_ulong,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_ulong_const_view {
    let mut view: _gsl_vector_ulong_const_view = {
        let mut init = _gsl_vector_ulong_const_view {
            vector: {
                let mut init = gsl_vector_ulong {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_ulong,
                    block: 0 as *mut gsl_block_ulong,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_subcolumn(
    mut m: *mut gsl_matrix_float,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_float_view {
    let mut view: _gsl_vector_float_view = {
        let mut init = _gsl_vector_float_view {
            vector: {
                let mut init = gsl_vector_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_subcolumn(
    mut m: *mut gsl_matrix_uint,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uint_view {
    let mut view: _gsl_vector_uint_view = {
        let mut init = _gsl_vector_uint_view {
            vector: {
                let mut init = gsl_vector_uint {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uint,
                    block: 0 as *mut gsl_block_uint,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_subcolumn(
    mut m: *const gsl_matrix_uchar,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_uchar_const_view {
    let mut view: _gsl_vector_uchar_const_view = {
        let mut init = _gsl_vector_uchar_const_view {
            vector: {
                let mut init = gsl_vector_uchar {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_uchar,
                    block: 0 as *mut gsl_block_uchar,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_subcolumn(
    mut m: *const gsl_matrix_complex_float,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_complex_float_const_view {
    let mut view: _gsl_vector_complex_float_const_view = {
        let mut init = _gsl_vector_complex_float_const_view {
            vector: {
                let mut init = gsl_vector_complex_float {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_float,
                    block: 0 as *mut gsl_block_complex_float,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_subcolumn(
    mut m: *const gsl_matrix_long,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_const_view {
    let mut view: _gsl_vector_long_const_view = {
        let mut init = _gsl_vector_long_const_view {
            vector: {
                let mut init = gsl_vector_long {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_long,
                    block: 0 as *mut gsl_block_long,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_subcolumn(
    mut m: *mut gsl_matrix_char,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_char_view {
    let mut view: _gsl_vector_char_view = {
        let mut init = _gsl_vector_char_view {
            vector: {
                let mut init = gsl_vector_char {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut libc::c_char,
                    block: 0 as *mut gsl_block_char,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_subcolumn(
    mut m: *mut gsl_matrix_long_double,
    j: size_t,
    offset: size_t,
    n: size_t,
) -> _gsl_vector_long_double_view {
    let mut view: _gsl_vector_long_double_view = {
        let mut init = _gsl_vector_long_double_view {
            vector: {
                let mut init = gsl_vector_long_double {
                    size: 0 as libc::c_int as size_t,
                    stride: 0 as libc::c_int as size_t,
                    data: 0 as *mut f128::f128,
                    block: 0 as *mut gsl_block_long_double,
                    owner: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"vector length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if offset.wrapping_add(n) > (*m).size1 {
        gsl_error(
            b"dimension n overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./rowcol_source.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut v: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    v
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(offset.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    v.size = n;
    v.stride = (*m).tda;
    v.block = (*m).block;
    v.owner = 0 as libc::c_int;
    view.vector = v;
    return view;
}

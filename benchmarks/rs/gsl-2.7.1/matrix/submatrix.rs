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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
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
pub struct _gsl_matrix_complex_long_double_view {
    pub matrix: gsl_matrix_complex_long_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_complex_long_double_const_view {
    pub matrix: gsl_matrix_complex_long_double,
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
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
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
pub struct _gsl_matrix_complex_view {
    pub matrix: gsl_matrix_complex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_complex_const_view {
    pub matrix: gsl_matrix_complex,
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
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
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
pub struct _gsl_matrix_complex_float_view {
    pub matrix: gsl_matrix_complex_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_complex_float_const_view {
    pub matrix: gsl_matrix_complex_float,
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
pub struct _gsl_matrix_long_double_view {
    pub matrix: gsl_matrix_long_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_long_double_const_view {
    pub matrix: gsl_matrix_long_double,
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
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_const_view {
    pub matrix: gsl_matrix,
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
pub struct _gsl_matrix_float_view {
    pub matrix: gsl_matrix_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_float_const_view {
    pub matrix: gsl_matrix_float,
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
pub struct _gsl_matrix_ulong_view {
    pub matrix: gsl_matrix_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_ulong_const_view {
    pub matrix: gsl_matrix_ulong,
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
pub struct _gsl_matrix_long_view {
    pub matrix: gsl_matrix_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_long_const_view {
    pub matrix: gsl_matrix_long,
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
pub struct _gsl_matrix_uint_view {
    pub matrix: gsl_matrix_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_uint_const_view {
    pub matrix: gsl_matrix_uint,
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
pub struct _gsl_matrix_int_view {
    pub matrix: gsl_matrix_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_int_const_view {
    pub matrix: gsl_matrix_int,
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
pub struct _gsl_matrix_ushort_view {
    pub matrix: gsl_matrix_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_ushort_const_view {
    pub matrix: gsl_matrix_ushort,
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
pub struct _gsl_matrix_short_view {
    pub matrix: gsl_matrix_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_short_const_view {
    pub matrix: gsl_matrix_short,
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
pub struct _gsl_matrix_uchar_view {
    pub matrix: gsl_matrix_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_uchar_const_view {
    pub matrix: gsl_matrix_uchar,
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
pub struct _gsl_matrix_char_view {
    pub matrix: gsl_matrix_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_char_const_view {
    pub matrix: gsl_matrix_char,
}
pub unsafe extern "C" fn gsl_matrix_ulong_submatrix(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_ulong_view {
    let mut view: _gsl_matrix_ulong_view = {
        let mut init = _gsl_matrix_ulong_view {
            matrix: {
                let mut init = gsl_matrix_ulong {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_ulong = {
        let mut init = gsl_matrix_ulong {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_submatrix(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_float_view {
    let mut view: _gsl_matrix_float_view = {
        let mut init = _gsl_matrix_float_view {
            matrix: {
                let mut init = gsl_matrix_float {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_float = {
        let mut init = gsl_matrix_float {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_float_const_submatrix(
    mut m: *const gsl_matrix_float,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_float_const_view {
    let mut view: _gsl_matrix_float_const_view = {
        let mut init = _gsl_matrix_float_const_view {
            matrix: {
                let mut init = gsl_matrix_float {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_float = {
        let mut init = gsl_matrix_float {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_const_submatrix(
    mut m: *const gsl_matrix_char,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_char_const_view {
    let mut view: _gsl_matrix_char_const_view = {
        let mut init = _gsl_matrix_char_const_view {
            matrix: {
                let mut init = gsl_matrix_char {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_char = {
        let mut init = gsl_matrix_char {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ulong_const_submatrix(
    mut m: *const gsl_matrix_ulong,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_ulong_const_view {
    let mut view: _gsl_matrix_ulong_const_view = {
        let mut init = _gsl_matrix_ulong_const_view {
            matrix: {
                let mut init = gsl_matrix_ulong {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_ulong = {
        let mut init = gsl_matrix_ulong {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_submatrix(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_long_view {
    let mut view: _gsl_matrix_long_view = {
        let mut init = _gsl_matrix_long_view {
            matrix: {
                let mut init = gsl_matrix_long {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_long = {
        let mut init = gsl_matrix_long {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_const_submatrix(
    mut m: *const gsl_matrix_long,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_long_const_view {
    let mut view: _gsl_matrix_long_const_view = {
        let mut init = _gsl_matrix_long_const_view {
            matrix: {
                let mut init = gsl_matrix_long {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_long = {
        let mut init = gsl_matrix_long {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_const_submatrix(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_const_view {
    let mut view: _gsl_matrix_const_view = {
        let mut init = _gsl_matrix_const_view {
            matrix: {
                let mut init = gsl_matrix {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix = {
        let mut init = gsl_matrix {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_submatrix(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_uint_view {
    let mut view: _gsl_matrix_uint_view = {
        let mut init = _gsl_matrix_uint_view {
            matrix: {
                let mut init = gsl_matrix_uint {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_uint = {
        let mut init = gsl_matrix_uint {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_submatrix(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_view {
    let mut view: _gsl_matrix_view = {
        let mut init = _gsl_matrix_view {
            matrix: {
                let mut init = gsl_matrix {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix = {
        let mut init = gsl_matrix {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uint_const_submatrix(
    mut m: *const gsl_matrix_uint,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_uint_const_view {
    let mut view: _gsl_matrix_uint_const_view = {
        let mut init = _gsl_matrix_uint_const_view {
            matrix: {
                let mut init = gsl_matrix_uint {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_uint = {
        let mut init = gsl_matrix_uint {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_const_submatrix(
    mut m: *const gsl_matrix_long_double,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_long_double_const_view {
    let mut view: _gsl_matrix_long_double_const_view = {
        let mut init = _gsl_matrix_long_double_const_view {
            matrix: {
                let mut init = gsl_matrix_long_double {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_long_double = {
        let mut init = gsl_matrix_long_double {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_submatrix(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_int_view {
    let mut view: _gsl_matrix_int_view = {
        let mut init = _gsl_matrix_int_view {
            matrix: {
                let mut init = gsl_matrix_int {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_int = {
        let mut init = gsl_matrix_int {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_long_double_submatrix(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_long_double_view {
    let mut view: _gsl_matrix_long_double_view = {
        let mut init = _gsl_matrix_long_double_view {
            matrix: {
                let mut init = gsl_matrix_long_double {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_long_double = {
        let mut init = gsl_matrix_long_double {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_int_const_submatrix(
    mut m: *const gsl_matrix_int,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_int_const_view {
    let mut view: _gsl_matrix_int_const_view = {
        let mut init = _gsl_matrix_int_const_view {
            matrix: {
                let mut init = gsl_matrix_int {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_int = {
        let mut init = gsl_matrix_int {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_const_submatrix(
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_complex_float_const_view {
    let mut view: _gsl_matrix_complex_float_const_view = {
        let mut init = _gsl_matrix_complex_float_const_view {
            matrix: {
                let mut init = gsl_matrix_complex_float {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_complex_float = {
        let mut init = gsl_matrix_complex_float {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_submatrix(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_ushort_view {
    let mut view: _gsl_matrix_ushort_view = {
        let mut init = _gsl_matrix_ushort_view {
            matrix: {
                let mut init = gsl_matrix_ushort {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_ushort = {
        let mut init = gsl_matrix_ushort {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_submatrix(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_complex_float_view {
    let mut view: _gsl_matrix_complex_float_view = {
        let mut init = _gsl_matrix_complex_float_view {
            matrix: {
                let mut init = gsl_matrix_complex_float {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_complex_float = {
        let mut init = gsl_matrix_complex_float {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_ushort_const_submatrix(
    mut m: *const gsl_matrix_ushort,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_ushort_const_view {
    let mut view: _gsl_matrix_ushort_const_view = {
        let mut init = _gsl_matrix_ushort_const_view {
            matrix: {
                let mut init = gsl_matrix_ushort {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_ushort = {
        let mut init = gsl_matrix_ushort {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_const_submatrix(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_complex_const_view {
    let mut view: _gsl_matrix_complex_const_view = {
        let mut init = _gsl_matrix_complex_const_view {
            matrix: {
                let mut init = gsl_matrix_complex {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_complex = {
        let mut init = gsl_matrix_complex {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_submatrix(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_short_view {
    let mut view: _gsl_matrix_short_view = {
        let mut init = _gsl_matrix_short_view {
            matrix: {
                let mut init = gsl_matrix_short {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_short = {
        let mut init = gsl_matrix_short {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_submatrix(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_complex_view {
    let mut view: _gsl_matrix_complex_view = {
        let mut init = _gsl_matrix_complex_view {
            matrix: {
                let mut init = gsl_matrix_complex {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_complex = {
        let mut init = gsl_matrix_complex {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_short_const_submatrix(
    mut m: *const gsl_matrix_short,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_short_const_view {
    let mut view: _gsl_matrix_short_const_view = {
        let mut init = _gsl_matrix_short_const_view {
            matrix: {
                let mut init = gsl_matrix_short {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_short = {
        let mut init = gsl_matrix_short {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_const_submatrix(
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_complex_long_double_const_view {
    let mut view: _gsl_matrix_complex_long_double_const_view = {
        let mut init = _gsl_matrix_complex_long_double_const_view {
            matrix: {
                let mut init = gsl_matrix_complex_long_double {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_complex_long_double = {
        let mut init = gsl_matrix_complex_long_double {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_submatrix(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_uchar_view {
    let mut view: _gsl_matrix_uchar_view = {
        let mut init = _gsl_matrix_uchar_view {
            matrix: {
                let mut init = gsl_matrix_uchar {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_uchar = {
        let mut init = gsl_matrix_uchar {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_uchar_const_submatrix(
    mut m: *const gsl_matrix_uchar,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_uchar_const_view {
    let mut view: _gsl_matrix_uchar_const_view = {
        let mut init = _gsl_matrix_uchar_const_view {
            matrix: {
                let mut init = gsl_matrix_uchar {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_uchar = {
        let mut init = gsl_matrix_uchar {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_char_submatrix(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_char_view {
    let mut view: _gsl_matrix_char_view = {
        let mut init = _gsl_matrix_char_view {
            matrix: {
                let mut init = gsl_matrix_char {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_char = {
        let mut init = gsl_matrix_char {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_submatrix(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
    n1: size_t,
    n2: size_t,
) -> _gsl_matrix_complex_long_double_view {
    let mut view: _gsl_matrix_complex_long_double_view = {
        let mut init = _gsl_matrix_complex_long_double_view {
            matrix: {
                let mut init = gsl_matrix_complex_long_double {
                    size1: 0 as libc::c_int as size_t,
                    size2: 0 as libc::c_int as size_t,
                    tda: 0 as libc::c_int as size_t,
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
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j >= (*m).size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if i.wrapping_add(n1) > (*m).size1 {
        gsl_error(
            b"first dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    } else if j.wrapping_add(n2) > (*m).size2 {
        gsl_error(
            b"second dimension overflows matrix\0" as *const u8 as *const libc::c_char,
            b"./submatrix_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_matrix_complex_long_double = {
        let mut init = gsl_matrix_complex_long_double {
            size1: 0 as libc::c_int as size_t,
            size2: 0 as libc::c_int as size_t,
            tda: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        );
    s.size1 = n1;
    s.size2 = n2;
    s.tda = (*m).tda;
    s.block = (*m).block;
    s.owner = 0 as libc::c_int;
    view.matrix = s;
    return view;
}

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
pub unsafe extern "C" fn gsl_vector_complex_float_subvector(
    mut v: *mut gsl_vector_complex_float,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_short_const_subvector(
    mut v: *const gsl_vector_short,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_double_subvector(
    mut v: *mut gsl_vector_long_double,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uchar_const_subvector(
    mut v: *const gsl_vector_uchar,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ulong_subvector(
    mut v: *mut gsl_vector_ulong,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_char_const_subvector(
    mut v: *const gsl_vector_char,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_const_subvector(
    mut v: *const gsl_vector_long,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_double_const_subvector(
    mut v: *const gsl_vector_long_double,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uint_subvector(
    mut v: *mut gsl_vector_uint,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uchar_subvector(
    mut v: *mut gsl_vector_uchar,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uint_const_subvector(
    mut v: *const gsl_vector_uint,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_subvector(
    mut v: *mut gsl_vector_complex_long_double,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_int_subvector(
    mut v: *mut gsl_vector_int,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_const_subvector(
    mut v: *const gsl_vector_complex_long_double,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_int_const_subvector(
    mut v: *const gsl_vector_int,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ulong_const_subvector(
    mut v: *const gsl_vector_ulong,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_subvector(
    mut v: *mut gsl_vector,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ushort_subvector(
    mut v: *mut gsl_vector_ushort,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_float_const_subvector(
    mut v: *const gsl_vector_complex_float,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_char_subvector(
    mut v: *mut gsl_vector_char,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_const_subvector(
    mut v: *const gsl_vector,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_subvector(
    mut v: *mut gsl_vector_long,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_short_subvector(
    mut v: *mut gsl_vector_short,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_subvector(
    mut v: *mut gsl_vector_complex,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_float_const_subvector(
    mut v: *const gsl_vector_float,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_const_subvector(
    mut v: *const gsl_vector_complex,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ushort_const_subvector(
    mut v: *const gsl_vector_ushort,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_float_subvector(
    mut v: *mut gsl_vector_float,
    mut offset: size_t,
    mut n: size_t,
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
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = (*v).stride;
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_const_subvector_with_stride(
    mut v: *const gsl_vector_complex_long_double,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_float_subvector_with_stride(
    mut v: *mut gsl_vector_complex_float,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_float_subvector_with_stride(
    mut v: *mut gsl_vector_float,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_subvector_with_stride(
    mut v: *mut gsl_vector_complex,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ushort_const_subvector_with_stride(
    mut v: *const gsl_vector_ushort,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ushort_subvector_with_stride(
    mut v: *mut gsl_vector_ushort,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ushort = {
        let mut init = gsl_vector_ushort {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ushort,
            block: 0 as *mut gsl_block_ushort,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_const_subvector_with_stride(
    mut v: *const gsl_vector,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_float_const_subvector_with_stride(
    mut v: *const gsl_vector_float,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_float = {
        let mut init = gsl_vector_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_char_const_subvector_with_stride(
    mut v: *const gsl_vector_char,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_subvector_with_stride(
    mut v: *mut gsl_vector,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector = {
        let mut init = gsl_vector {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_int_const_subvector_with_stride(
    mut v: *const gsl_vector_int,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_float_const_subvector_with_stride(
    mut v: *const gsl_vector_complex_float,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_float = {
        let mut init = gsl_vector_complex_float {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_float,
            block: 0 as *mut gsl_block_complex_float,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uchar_subvector_with_stride(
    mut v: *mut gsl_vector_uchar,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_const_subvector_with_stride(
    mut v: *const gsl_vector_complex,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex = {
        let mut init = gsl_vector_complex {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ulong_const_subvector_with_stride(
    mut v: *const gsl_vector_ulong,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_char_subvector_with_stride(
    mut v: *mut gsl_vector_char,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_char = {
        let mut init = gsl_vector_char {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_char,
            block: 0 as *mut gsl_block_char,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_int_subvector_with_stride(
    mut v: *mut gsl_vector_int,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_int = {
        let mut init = gsl_vector_int {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_int,
            block: 0 as *mut gsl_block_int,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_subvector_with_stride(
    mut v: *mut gsl_vector_complex_long_double,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_complex_long_double = {
        let mut init = gsl_vector_complex_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_complex_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_short_const_subvector_with_stride(
    mut v: *const gsl_vector_short,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uint_const_subvector_with_stride(
    mut v: *const gsl_vector_uint,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_double_const_subvector_with_stride(
    mut v: *const gsl_vector_long_double,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uint_subvector_with_stride(
    mut v: *mut gsl_vector_uint,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uint = {
        let mut init = gsl_vector_uint {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uint,
            block: 0 as *mut gsl_block_uint,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_const_subvector_with_stride(
    mut v: *const gsl_vector_long,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_double_subvector_with_stride(
    mut v: *mut gsl_vector_long_double,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long_double = {
        let mut init = gsl_vector_long_double {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut f128::f128,
            block: 0 as *mut gsl_block_long_double,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_long_subvector_with_stride(
    mut v: *mut gsl_vector_long,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_long = {
        let mut init = gsl_vector_long {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_long,
            block: 0 as *mut gsl_block_long,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_short_subvector_with_stride(
    mut v: *mut gsl_vector_short,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_short = {
        let mut init = gsl_vector_short {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_short,
            block: 0 as *mut gsl_block_short,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_uchar_const_subvector_with_stride(
    mut v: *const gsl_vector_uchar,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_uchar = {
        let mut init = gsl_vector_uchar {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_uchar,
            block: 0 as *mut gsl_block_uchar,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_ulong_subvector_with_stride(
    mut v: *mut gsl_vector_ulong,
    mut offset: size_t,
    mut stride: size_t,
    mut n: size_t,
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
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*v).size
    {
        gsl_error(
            b"view would extend past end of vector\0" as *const u8
                as *const libc::c_char,
            b"./subvector_source.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return view;
    }
    let mut s: gsl_vector_ulong = {
        let mut init = gsl_vector_ulong {
            size: 0 as libc::c_int as size_t,
            stride: 0 as libc::c_int as size_t,
            data: 0 as *mut libc::c_ulong,
            block: 0 as *mut gsl_block_ulong,
            owner: 0 as libc::c_int,
        };
        init
    };
    s
        .data = ((*v).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*v).stride)
                .wrapping_mul(offset) as isize,
        );
    s.size = n;
    s.stride = ((*v).stride).wrapping_mul(stride);
    s.block = (*v).block;
    s.owner = 0 as libc::c_int;
    view.vector = s;
    return view;
}

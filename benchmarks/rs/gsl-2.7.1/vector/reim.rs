use ::libc;
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
pub unsafe extern "C" fn gsl_vector_complex_float_real(
    mut v: *mut gsl_vector_complex_float,
) -> _gsl_vector_float_view {
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
    s.data = (*v).data;
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_float;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_const_real(
    mut v: *const gsl_vector_complex,
) -> _gsl_vector_const_view {
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
    s.data = (*v).data;
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_real(
    mut v: *mut gsl_vector_complex,
) -> _gsl_vector_view {
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
    s.data = (*v).data;
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_const_real(
    mut v: *const gsl_vector_complex_long_double,
) -> _gsl_vector_long_double_const_view {
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
    s.data = (*v).data;
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_long_double;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_float_const_real(
    mut v: *const gsl_vector_complex_float,
) -> _gsl_vector_float_const_view {
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
    s.data = (*v).data;
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_float;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_real(
    mut v: *mut gsl_vector_complex_long_double,
) -> _gsl_vector_long_double_view {
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
    s.data = (*v).data;
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_long_double;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_float_imag(
    mut v: *mut gsl_vector_complex_float,
) -> _gsl_vector_float_view {
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
    s.data = ((*v).data).offset(1 as libc::c_int as isize);
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_float;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_const_imag(
    mut v: *const gsl_vector_complex,
) -> _gsl_vector_const_view {
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
    s.data = ((*v).data).offset(1 as libc::c_int as isize);
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_imag(
    mut v: *mut gsl_vector_complex,
) -> _gsl_vector_view {
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
    s.data = ((*v).data).offset(1 as libc::c_int as isize);
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_const_imag(
    mut v: *const gsl_vector_complex_long_double,
) -> _gsl_vector_long_double_const_view {
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
    s.data = ((*v).data).offset(1 as libc::c_int as isize);
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_long_double;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_float_const_imag(
    mut v: *const gsl_vector_complex_float,
) -> _gsl_vector_float_const_view {
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
    s.data = ((*v).data).offset(1 as libc::c_int as isize);
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_float;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_imag(
    mut v: *mut gsl_vector_complex_long_double,
) -> _gsl_vector_long_double_view {
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
    s.data = ((*v).data).offset(1 as libc::c_int as isize);
    s.size = (*v).size;
    s.stride = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*v).stride);
    s.block = 0 as *mut gsl_block_long_double;
    s.owner = 0 as libc::c_int;
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
    view.vector = s;
    return view;
}

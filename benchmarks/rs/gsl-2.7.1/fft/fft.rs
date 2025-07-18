use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
pub type gsl_complex_packed_array = *mut libc::c_double;
pub type gsl_complex_packed_array_float = *mut libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_float {
    pub dat: [libc::c_float; 2],
}
pub type gsl_fft_direction = libc::c_int;
pub const gsl_fft_backward: gsl_fft_direction = 1;
pub const gsl_fft_forward: gsl_fft_direction = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_complex_wavetable {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex; 64],
    pub trig: *mut gsl_complex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_complex_workspace {
    pub n: size_t,
    pub scratch: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_complex_wavetable_float {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex_float; 64],
    pub trig: *mut gsl_complex_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_complex_workspace_float {
    pub n: size_t,
    pub scratch: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_real_wavetable {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex; 64],
    pub trig: *mut gsl_complex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_real_workspace {
    pub n: size_t,
    pub scratch: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_halfcomplex_wavetable {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex; 64],
    pub trig: *mut gsl_complex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_real_wavetable_float {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex_float; 64],
    pub trig: *mut gsl_complex_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_real_workspace_float {
    pub n: size_t,
    pub scratch: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_fft_halfcomplex_wavetable_float {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex_float; 64],
    pub trig: *mut gsl_complex_float,
}
unsafe extern "C" fn fft_complex_bitreverse_order(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    mut logn: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    logn = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut k: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if i < j {
            let tmp_real: libc::c_double = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                );
            let tmp_imag: libc::c_double = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                ) = tmp_real;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = tmp_imag;
        }
        while k <= j {
            j = j.wrapping_sub(k);
            k = k.wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        j = (j as libc::c_ulong).wrapping_add(k) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_bitreverse_order(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    mut logn: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    logn = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut k: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if i < j {
            let tmp_real: libc::c_float = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                );
            let tmp_imag: libc::c_float = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                ) = tmp_real;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = tmp_imag;
        }
        while k <= j {
            j = j.wrapping_sub(k);
            k = k.wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        j = (j as libc::c_ulong).wrapping_add(k) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_real_bitreverse_order(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    mut logn: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    logn = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut k: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if i < j {
            let tmp: libc::c_double = *data.offset(stride.wrapping_mul(i) as isize);
            *data
                .offset(
                    stride.wrapping_mul(i) as isize,
                ) = *data.offset(stride.wrapping_mul(j) as isize);
            *data.offset(stride.wrapping_mul(j) as isize) = tmp;
        }
        while k <= j {
            j = j.wrapping_sub(k);
            k = k.wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        j = (j as libc::c_ulong).wrapping_add(k) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_real_float_bitreverse_order(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    mut logn: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    logn = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut k: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if i < j {
            let tmp: libc::c_float = *data.offset(stride.wrapping_mul(i) as isize);
            *data
                .offset(
                    stride.wrapping_mul(i) as isize,
                ) = *data.offset(stride.wrapping_mul(j) as isize);
            *data.offset(stride.wrapping_mul(j) as isize) = tmp;
        }
        while k <= j {
            j = j.wrapping_sub(k);
            k = k.wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        j = (j as libc::c_ulong).wrapping_add(k) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_factorize(
    n: size_t,
    mut nf: *mut size_t,
    mut factors: *mut size_t,
) -> libc::c_int {
    let complex_subtransforms: [size_t; 7] = [
        7 as libc::c_int as size_t,
        6 as libc::c_int as size_t,
        5 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    ];
    let mut status: libc::c_int = fft_factorize(
        n,
        complex_subtransforms.as_ptr(),
        nf,
        factors,
    );
    return status;
}
unsafe extern "C" fn fft_halfcomplex_factorize(
    n: size_t,
    mut nf: *mut size_t,
    mut factors: *mut size_t,
) -> libc::c_int {
    let halfcomplex_subtransforms: [size_t; 5] = [
        5 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    ];
    let mut status: libc::c_int = fft_factorize(
        n,
        halfcomplex_subtransforms.as_ptr(),
        nf,
        factors,
    );
    return status;
}
unsafe extern "C" fn fft_real_factorize(
    n: size_t,
    mut nf: *mut size_t,
    mut factors: *mut size_t,
) -> libc::c_int {
    let real_subtransforms: [size_t; 5] = [
        5 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    ];
    let mut status: libc::c_int = fft_factorize(
        n,
        real_subtransforms.as_ptr(),
        nf,
        factors,
    );
    return status;
}
unsafe extern "C" fn fft_factorize(
    n: size_t,
    mut implemented_subtransforms: *const size_t,
    mut n_factors: *mut size_t,
    mut factors: *mut size_t,
) -> libc::c_int {
    let mut nf: size_t = 0 as libc::c_int as size_t;
    let mut ntest: size_t = n;
    let mut factor: size_t = 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./factorize.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        *factors.offset(0 as libc::c_int as isize) = 1 as libc::c_int as size_t;
        *n_factors = 1 as libc::c_int as size_t;
        return 0 as libc::c_int;
    }
    while *implemented_subtransforms.offset(i as isize) != 0
        && ntest != 1 as libc::c_int as libc::c_ulong
    {
        factor = *implemented_subtransforms.offset(i as isize);
        while ntest.wrapping_rem(factor) == 0 as libc::c_int as libc::c_ulong {
            ntest = ntest.wrapping_div(factor);
            *factors.offset(nf as isize) = factor;
            nf = nf.wrapping_add(1);
            nf;
        }
        i = i.wrapping_add(1);
        i;
    }
    factor = 2 as libc::c_int as size_t;
    while ntest.wrapping_rem(factor) == 0 as libc::c_int as libc::c_ulong
        && ntest != 1 as libc::c_int as libc::c_ulong
    {
        ntest = ntest.wrapping_div(factor);
        *factors.offset(nf as isize) = factor;
        nf = nf.wrapping_add(1);
        nf;
    }
    factor = 3 as libc::c_int as size_t;
    while ntest != 1 as libc::c_int as libc::c_ulong {
        while ntest.wrapping_rem(factor) != 0 as libc::c_int as libc::c_ulong {
            factor = (factor as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        ntest = ntest.wrapping_div(factor);
        *factors.offset(nf as isize) = factor;
        nf = nf.wrapping_add(1);
        nf;
    }
    let mut product: size_t = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < nf {
        product = (product as libc::c_ulong).wrapping_mul(*factors.offset(i as isize))
            as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    if product != n {
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./factorize.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    *n_factors = nf;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_binary_logn(n: size_t) -> libc::c_int {
    let mut ntest: size_t = 0;
    let mut binary_logn: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 1 as libc::c_int as size_t;
    while k < n {
        k = (k as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        binary_logn = binary_logn.wrapping_add(1);
        binary_logn;
    }
    ntest = ((1 as libc::c_int) << binary_logn) as size_t;
    if n != ntest {
        return -(1 as libc::c_int);
    }
    return binary_logn as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_wavetable_float_alloc(
    mut n: size_t,
) -> *mut gsl_fft_complex_wavetable_float {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_factors: size_t = 0;
    let mut t: size_t = 0;
    let mut product: size_t = 0;
    let mut product_1: size_t = 0;
    let mut q: size_t = 0;
    let mut d_theta: libc::c_double = 0.;
    let mut wavetable: *mut gsl_fft_complex_wavetable_float = 0
        as *mut gsl_fft_complex_wavetable_float;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable_float;
    }
    wavetable = malloc(
        ::std::mem::size_of::<gsl_fft_complex_wavetable_float>() as libc::c_ulong,
    ) as *mut gsl_fft_complex_wavetable_float;
    if wavetable.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable_float;
    }
    (*wavetable)
        .trig = malloc(
        n.wrapping_mul(::std::mem::size_of::<gsl_complex_float>() as libc::c_ulong),
    ) as *mut gsl_complex_float;
    if ((*wavetable).trig).is_null() {
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"failed to allocate trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable_float;
    }
    (*wavetable).n = n;
    status = fft_complex_factorize(
        n,
        &mut n_factors,
        ((*wavetable).factor).as_mut_ptr(),
    );
    if status != 0 {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EFACTOR as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable_float;
    }
    (*wavetable).nf = n_factors;
    d_theta = -2.0f64 * 3.14159265358979323846f64 / n as libc::c_double;
    t = 0 as libc::c_int as size_t;
    product = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n_factors {
        let mut j: size_t = 0;
        let factor: size_t = (*wavetable).factor[i as usize];
        (*wavetable).twiddle[i as usize] = ((*wavetable).trig).offset(t as isize);
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        j = 1 as libc::c_int as size_t;
        while j < factor {
            let mut k: size_t = 0;
            let mut m: size_t = 0 as libc::c_int as size_t;
            k = 1 as libc::c_int as size_t;
            while k <= q {
                let mut theta: libc::c_double = 0.;
                m = m.wrapping_add(j.wrapping_mul(product_1));
                m = m.wrapping_rem(n);
                theta = d_theta * m as libc::c_double;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[0 as libc::c_int as usize] = cos(theta) as libc::c_float;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[1 as libc::c_int as usize] = sin(theta) as libc::c_float;
                t = t.wrapping_add(1);
                t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if t > n {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"overflowed trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable_float;
    }
    return wavetable;
}
pub unsafe extern "C" fn gsl_fft_complex_wavetable_alloc(
    mut n: size_t,
) -> *mut gsl_fft_complex_wavetable {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_factors: size_t = 0;
    let mut t: size_t = 0;
    let mut product: size_t = 0;
    let mut product_1: size_t = 0;
    let mut q: size_t = 0;
    let mut d_theta: libc::c_double = 0.;
    let mut wavetable: *mut gsl_fft_complex_wavetable = 0
        as *mut gsl_fft_complex_wavetable;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable;
    }
    wavetable = malloc(
        ::std::mem::size_of::<gsl_fft_complex_wavetable>() as libc::c_ulong,
    ) as *mut gsl_fft_complex_wavetable;
    if wavetable.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable;
    }
    (*wavetable)
        .trig = malloc(
        n.wrapping_mul(::std::mem::size_of::<gsl_complex>() as libc::c_ulong),
    ) as *mut gsl_complex;
    if ((*wavetable).trig).is_null() {
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"failed to allocate trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable;
    }
    (*wavetable).n = n;
    status = fft_complex_factorize(
        n,
        &mut n_factors,
        ((*wavetable).factor).as_mut_ptr(),
    );
    if status != 0 {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EFACTOR as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable;
    }
    (*wavetable).nf = n_factors;
    d_theta = -2.0f64 * 3.14159265358979323846f64 / n as libc::c_double;
    t = 0 as libc::c_int as size_t;
    product = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n_factors {
        let mut j: size_t = 0;
        let factor: size_t = (*wavetable).factor[i as usize];
        (*wavetable).twiddle[i as usize] = ((*wavetable).trig).offset(t as isize);
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        j = 1 as libc::c_int as size_t;
        while j < factor {
            let mut k: size_t = 0;
            let mut m: size_t = 0 as libc::c_int as size_t;
            k = 1 as libc::c_int as size_t;
            while k <= q {
                let mut theta: libc::c_double = 0.;
                m = m.wrapping_add(j.wrapping_mul(product_1));
                m = m.wrapping_rem(n);
                theta = d_theta * m as libc::c_double;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[0 as libc::c_int as usize] = cos(theta);
                (*((*wavetable).trig).offset(t as isize))
                    .dat[1 as libc::c_int as usize] = sin(theta);
                t = t.wrapping_add(1);
                t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if t > n {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"overflowed trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_wavetable;
    }
    return wavetable;
}
pub unsafe extern "C" fn gsl_fft_complex_workspace_float_alloc(
    mut n: size_t,
) -> *mut gsl_fft_complex_workspace_float {
    let mut workspace: *mut gsl_fft_complex_workspace_float = 0
        as *mut gsl_fft_complex_workspace_float;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_workspace_float;
    }
    workspace = malloc(
        ::std::mem::size_of::<gsl_fft_complex_workspace_float>() as libc::c_ulong,
    ) as *mut gsl_fft_complex_workspace_float;
    if workspace.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_workspace_float;
    }
    (*workspace).n = n;
    (*workspace)
        .scratch = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ((*workspace).scratch).is_null() {
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate scratch space\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_workspace_float;
    }
    return workspace;
}
pub unsafe extern "C" fn gsl_fft_complex_workspace_alloc(
    mut n: size_t,
) -> *mut gsl_fft_complex_workspace {
    let mut workspace: *mut gsl_fft_complex_workspace = 0
        as *mut gsl_fft_complex_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_workspace;
    }
    workspace = malloc(
        ::std::mem::size_of::<gsl_fft_complex_workspace>() as libc::c_ulong,
    ) as *mut gsl_fft_complex_workspace;
    if workspace.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_workspace;
    }
    (*workspace).n = n;
    (*workspace)
        .scratch = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*workspace).scratch).is_null() {
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate scratch space\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_complex_workspace;
    }
    return workspace;
}
pub unsafe extern "C" fn gsl_fft_complex_wavetable_float_free(
    mut wavetable: *mut gsl_fft_complex_wavetable_float,
) {
    if wavetable.is_null() {
        return;
    }
    free((*wavetable).trig as *mut libc::c_void);
    (*wavetable).trig = 0 as *mut gsl_complex_float;
    free(wavetable as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_complex_wavetable_free(
    mut wavetable: *mut gsl_fft_complex_wavetable,
) {
    if wavetable.is_null() {
        return;
    }
    free((*wavetable).trig as *mut libc::c_void);
    (*wavetable).trig = 0 as *mut gsl_complex;
    free(wavetable as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_complex_workspace_free(
    mut workspace: *mut gsl_fft_complex_workspace,
) {
    if workspace.is_null() {
        return;
    }
    free((*workspace).scratch as *mut libc::c_void);
    (*workspace).scratch = 0 as *mut libc::c_double;
    free(workspace as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_complex_workspace_float_free(
    mut workspace: *mut gsl_fft_complex_workspace_float,
) {
    if workspace.is_null() {
        return;
    }
    free((*workspace).scratch as *mut libc::c_void);
    (*workspace).scratch = 0 as *mut libc::c_float;
    free(workspace as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_complex_float_memcpy(
    mut dest: *mut gsl_fft_complex_wavetable_float,
    mut src: *mut gsl_fft_complex_wavetable_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nf: libc::c_int = 0;
    if (*dest).n != (*src).n {
        gsl_error(
            b"length of src and dest do not match\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    n = (*dest).n as libc::c_int;
    nf = (*dest).nf as libc::c_int;
    memcpy(
        (*dest).trig as *mut libc::c_void,
        (*src).trig as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < nf {
        (*dest)
            .twiddle[i
            as usize] = ((*dest).trig)
            .offset(
                ((*src).twiddle[i as usize]).offset_from((*src).trig) as libc::c_long
                    as isize,
            );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_memcpy(
    mut dest: *mut gsl_fft_complex_wavetable,
    mut src: *mut gsl_fft_complex_wavetable,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nf: libc::c_int = 0;
    if (*dest).n != (*src).n {
        gsl_error(
            b"length of src and dest do not match\0" as *const u8 as *const libc::c_char,
            b"./c_init.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    n = (*dest).n as libc::c_int;
    nf = (*dest).nf as libc::c_int;
    memcpy(
        (*dest).trig as *mut libc::c_void,
        (*src).trig as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < nf {
        (*dest)
            .twiddle[i
            as usize] = ((*dest).trig)
            .offset(
                ((*src).twiddle[i as usize]).offset_from((*src).trig) as libc::c_long
                    as isize,
            );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_transform(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable,
    mut work: *mut gsl_fft_complex_workspace,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let nf: size_t = (*wavetable).nf;
    let mut i: size_t = 0;
    let mut q: size_t = 0;
    let mut product: size_t = 1 as libc::c_int as size_t;
    let mut twiddle1: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle2: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle3: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle4: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle5: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle6: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut state: size_t = 0 as libc::c_int as size_t;
    let scratch: *mut libc::c_double = (*work).scratch;
    let mut in_0: *mut libc::c_double = data;
    let mut istride: size_t = stride;
    let mut out: *mut libc::c_double = scratch;
    let mut ostride: size_t = 1 as libc::c_int as size_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./c_main.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n != (*wavetable).n {
        gsl_error(
            b"wavetable does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./c_main.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n != (*work).n {
        gsl_error(
            b"workspace does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./c_main.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < nf {
        let factor: size_t = (*wavetable).factor[i as usize];
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        if state == 0 as libc::c_int as libc::c_ulong {
            in_0 = data;
            istride = stride;
            out = scratch;
            ostride = 1 as libc::c_int as size_t;
            state = 1 as libc::c_int as size_t;
        } else {
            in_0 = scratch;
            istride = 1 as libc::c_int as size_t;
            out = data;
            ostride = stride;
            state = 0 as libc::c_int as size_t;
        }
        if factor == 2 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_complex_pass_2(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex,
            );
        } else if factor == 3 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            fft_complex_pass_3(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
            );
        } else if factor == 4 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            fft_complex_pass_4(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
            );
        } else if factor == 5 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            twiddle4 = twiddle3.offset(q as isize);
            fft_complex_pass_5(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
                twiddle4 as *const gsl_complex,
            );
        } else if factor == 6 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            twiddle4 = twiddle3.offset(q as isize);
            twiddle5 = twiddle4.offset(q as isize);
            fft_complex_pass_6(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
                twiddle4 as *const gsl_complex,
                twiddle5 as *const gsl_complex,
            );
        } else if factor == 7 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            twiddle4 = twiddle3.offset(q as isize);
            twiddle5 = twiddle4.offset(q as isize);
            twiddle6 = twiddle5.offset(q as isize);
            fft_complex_pass_7(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
                twiddle4 as *const gsl_complex,
                twiddle5 as *const gsl_complex,
                twiddle6 as *const gsl_complex,
            );
        } else {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_complex_pass_n(
                in_0,
                istride,
                out,
                ostride,
                sign,
                factor,
                product,
                n,
                twiddle1 as *const gsl_complex,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if state == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) = *scratch
                .offset(
                    ((2 as libc::c_int * 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(i) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *scratch
                .offset(
                    ((2 as libc::c_int * 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_float_forward(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable_float,
    mut work: *mut gsl_fft_complex_workspace_float,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_fft_complex_float_transform(
        data,
        stride,
        n,
        wavetable,
        work,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_inverse(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable,
    mut work: *mut gsl_fft_complex_workspace,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_transform(
        data,
        stride,
        n,
        wavetable,
        work,
        sign,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_backward(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable,
    mut work: *mut gsl_fft_complex_workspace,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_transform(
        data,
        stride,
        n,
        wavetable,
        work,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_forward(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable,
    mut work: *mut gsl_fft_complex_workspace,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_fft_complex_transform(
        data,
        stride,
        n,
        wavetable,
        work,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_backward(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable_float,
    mut work: *mut gsl_fft_complex_workspace_float,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_float_transform(
        data,
        stride,
        n,
        wavetable,
        work,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_transform(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable_float,
    mut work: *mut gsl_fft_complex_workspace_float,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let nf: size_t = (*wavetable).nf;
    let mut i: size_t = 0;
    let mut q: size_t = 0;
    let mut product: size_t = 1 as libc::c_int as size_t;
    let mut twiddle1: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle2: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle3: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle4: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle5: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle6: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut state: size_t = 0 as libc::c_int as size_t;
    let scratch: *mut libc::c_float = (*work).scratch;
    let mut in_0: *mut libc::c_float = data;
    let mut istride: size_t = stride;
    let mut out: *mut libc::c_float = scratch;
    let mut ostride: size_t = 1 as libc::c_int as size_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./c_main.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n != (*wavetable).n {
        gsl_error(
            b"wavetable does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./c_main.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n != (*work).n {
        gsl_error(
            b"workspace does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./c_main.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < nf {
        let factor: size_t = (*wavetable).factor[i as usize];
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        if state == 0 as libc::c_int as libc::c_ulong {
            in_0 = data;
            istride = stride;
            out = scratch;
            ostride = 1 as libc::c_int as size_t;
            state = 1 as libc::c_int as size_t;
        } else {
            in_0 = scratch;
            istride = 1 as libc::c_int as size_t;
            out = data;
            ostride = stride;
            state = 0 as libc::c_int as size_t;
        }
        if factor == 2 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_complex_float_pass_2(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
            );
        } else if factor == 3 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            fft_complex_float_pass_3(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
            );
        } else if factor == 4 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            fft_complex_float_pass_4(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
            );
        } else if factor == 5 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            twiddle4 = twiddle3.offset(q as isize);
            fft_complex_float_pass_5(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
                twiddle4 as *const gsl_complex_float,
            );
        } else if factor == 6 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            twiddle4 = twiddle3.offset(q as isize);
            twiddle5 = twiddle4.offset(q as isize);
            fft_complex_float_pass_6(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
                twiddle4 as *const gsl_complex_float,
                twiddle5 as *const gsl_complex_float,
            );
        } else if factor == 7 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(q as isize);
            twiddle3 = twiddle2.offset(q as isize);
            twiddle4 = twiddle3.offset(q as isize);
            twiddle5 = twiddle4.offset(q as isize);
            twiddle6 = twiddle5.offset(q as isize);
            fft_complex_float_pass_7(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                sign,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
                twiddle4 as *const gsl_complex_float,
                twiddle5 as *const gsl_complex_float,
                twiddle6 as *const gsl_complex_float,
            );
        } else {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_complex_float_pass_n(
                in_0,
                istride,
                out,
                ostride,
                sign,
                factor,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if state == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) = *scratch
                .offset(
                    ((2 as libc::c_int * 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(i) as isize,
                );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *scratch
                .offset(
                    ((2 as libc::c_int * 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_float_inverse(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_complex_wavetable_float,
    mut work: *mut gsl_fft_complex_workspace_float,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_float_transform(
        data,
        stride,
        n,
        wavetable,
        work,
        sign,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_float = 1.0f32 / n as libc::c_float;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
unsafe extern "C" fn fft_complex_pass_2(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 2 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(product_1);
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w_real: libc::c_double = 0.;
        let mut w_imag: libc::c_double = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w_real = 1.0f64;
            w_imag = 0.0f64;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w_real = (*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w_imag = (*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w_real = (*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w_imag = -(*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let z0_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let x0_real: libc::c_double = z0_real + z1_real;
            let x0_imag: libc::c_double = z0_imag + z1_imag;
            let x1_real: libc::c_double = z0_real - z1_real;
            let x1_imag: libc::c_double = z0_imag - z1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1)) as isize,
                ) = w_real * x1_real - w_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w_real * x1_imag + w_imag * x1_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_2(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 2 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(product_1);
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w_real: libc::c_float = 0.;
        let mut w_imag: libc::c_float = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w_real = 1.0f64 as libc::c_float;
            w_imag = 0.0f64 as libc::c_float;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w_real = (*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w_imag = (*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w_real = (*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w_imag = -(*twiddle
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let z0_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let x0_real: libc::c_float = z0_real + z1_real;
            let x0_imag: libc::c_float = z0_imag + z1_imag;
            let x1_real: libc::c_float = z0_real - z1_real;
            let x1_imag: libc::c_float = z0_imag - z1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1)) as isize,
                ) = w_real * x1_real - w_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w_real * x1_imag + w_imag * x1_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_pass_3(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 3 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(product_1);
    let tau: libc::c_double = sqrt(3.0f64) / 2.0f64;
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_double = 0.;
        let mut w1_imag: libc::c_double = 0.;
        let mut w2_real: libc::c_double = 0.;
        let mut w2_imag: libc::c_double = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64;
            w1_imag = 0.0f64;
            w2_real = 1.0f64;
            w2_imag = 0.0f64;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let z0_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t1_real: libc::c_double = z1_real + z2_real;
            let t1_imag: libc::c_double = z1_imag + z2_imag;
            let t2_real: libc::c_double = z0_real - t1_real / 2.0f64;
            let t2_imag: libc::c_double = z0_imag - t1_imag / 2.0f64;
            let t3_real: libc::c_double = sign as libc::c_int as libc::c_double * tau
                * (z1_real - z2_real);
            let t3_imag: libc::c_double = sign as libc::c_int as libc::c_double * tau
                * (z1_imag - z2_imag);
            let x0_real: libc::c_double = z0_real + t1_real;
            let x0_imag: libc::c_double = z0_imag + t1_imag;
            let x1_real: libc::c_double = t2_real - t3_imag;
            let x1_imag: libc::c_double = t2_imag + t3_real;
            let x2_real: libc::c_double = t2_real + t3_imag;
            let x2_imag: libc::c_double = t2_imag - t3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_3(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 3 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(product_1);
    let tau: libc::c_float = (sqrt(3.0f64) / 2.0f64) as libc::c_float;
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_float = 0.;
        let mut w1_imag: libc::c_float = 0.;
        let mut w2_real: libc::c_float = 0.;
        let mut w2_imag: libc::c_float = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64 as libc::c_float;
            w1_imag = 0.0f64 as libc::c_float;
            w2_real = 1.0f64 as libc::c_float;
            w2_imag = 0.0f64 as libc::c_float;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let z0_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t1_real: libc::c_float = z1_real + z2_real;
            let t1_imag: libc::c_float = z1_imag + z2_imag;
            let t2_real: libc::c_float = (z0_real as libc::c_double
                - t1_real as libc::c_double / 2.0f64) as libc::c_float;
            let t2_imag: libc::c_float = (z0_imag as libc::c_double
                - t1_imag as libc::c_double / 2.0f64) as libc::c_float;
            let t3_real: libc::c_float = sign as libc::c_int as libc::c_float * tau
                * (z1_real - z2_real);
            let t3_imag: libc::c_float = sign as libc::c_int as libc::c_float * tau
                * (z1_imag - z2_imag);
            let x0_real: libc::c_float = z0_real + t1_real;
            let x0_imag: libc::c_float = z0_imag + t1_imag;
            let x1_real: libc::c_float = t2_real - t3_imag;
            let x1_imag: libc::c_float = t2_imag + t3_real;
            let x2_real: libc::c_float = t2_real + t3_imag;
            let x2_imag: libc::c_float = t2_imag - t3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(product_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_pass_4(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 4 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_double = 0.;
        let mut w1_imag: libc::c_double = 0.;
        let mut w2_real: libc::c_double = 0.;
        let mut w2_imag: libc::c_double = 0.;
        let mut w3_real: libc::c_double = 0.;
        let mut w3_imag: libc::c_double = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64;
            w1_imag = 0.0f64;
            w2_real = 1.0f64;
            w2_imag = 0.0f64;
            w3_real = 1.0f64;
            w3_imag = 0.0f64;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let z0_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t1_real: libc::c_double = z0_real + z2_real;
            let t1_imag: libc::c_double = z0_imag + z2_imag;
            let t2_real: libc::c_double = z1_real + z3_real;
            let t2_imag: libc::c_double = z1_imag + z3_imag;
            let t3_real: libc::c_double = z0_real - z2_real;
            let t3_imag: libc::c_double = z0_imag - z2_imag;
            let t4_real: libc::c_double = sign as libc::c_int as libc::c_double
                * (z1_real - z3_real);
            let t4_imag: libc::c_double = sign as libc::c_int as libc::c_double
                * (z1_imag - z3_imag);
            let x0_real: libc::c_double = t1_real + t2_real;
            let x0_imag: libc::c_double = t1_imag + t2_imag;
            let x1_real: libc::c_double = t3_real - t4_imag;
            let x1_imag: libc::c_double = t3_imag + t4_real;
            let x2_real: libc::c_double = t1_real - t2_real;
            let x2_imag: libc::c_double = t1_imag - t2_imag;
            let x3_real: libc::c_double = t3_real + t4_imag;
            let x3_imag: libc::c_double = t3_imag - t4_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_4(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 4 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_float = 0.;
        let mut w1_imag: libc::c_float = 0.;
        let mut w2_real: libc::c_float = 0.;
        let mut w2_imag: libc::c_float = 0.;
        let mut w3_real: libc::c_float = 0.;
        let mut w3_imag: libc::c_float = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64 as libc::c_float;
            w1_imag = 0.0f64 as libc::c_float;
            w2_real = 1.0f64 as libc::c_float;
            w2_imag = 0.0f64 as libc::c_float;
            w3_real = 1.0f64 as libc::c_float;
            w3_imag = 0.0f64 as libc::c_float;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let z0_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t1_real: libc::c_float = z0_real + z2_real;
            let t1_imag: libc::c_float = z0_imag + z2_imag;
            let t2_real: libc::c_float = z1_real + z3_real;
            let t2_imag: libc::c_float = z1_imag + z3_imag;
            let t3_real: libc::c_float = z0_real - z2_real;
            let t3_imag: libc::c_float = z0_imag - z2_imag;
            let t4_real: libc::c_float = sign as libc::c_int as libc::c_float
                * (z1_real - z3_real);
            let t4_imag: libc::c_float = sign as libc::c_int as libc::c_float
                * (z1_imag - z3_imag);
            let x0_real: libc::c_float = t1_real + t2_real;
            let x0_imag: libc::c_float = t1_imag + t2_imag;
            let x1_real: libc::c_float = t3_real - t4_imag;
            let x1_imag: libc::c_float = t3_imag + t4_real;
            let x2_real: libc::c_float = t1_real - t2_real;
            let x2_imag: libc::c_float = t1_imag - t2_imag;
            let x3_real: libc::c_float = t3_real + t4_imag;
            let x3_imag: libc::c_float = t3_imag - t4_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_pass_5(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
    mut twiddle4: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 5 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let sin_2pi_by_5: libc::c_double = sin(2.0f64 * 3.14159265358979323846f64 / 5.0f64);
    let sin_2pi_by_10: libc::c_double = sin(
        2.0f64 * 3.14159265358979323846f64 / 10.0f64,
    );
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_double = 0.;
        let mut w1_imag: libc::c_double = 0.;
        let mut w2_real: libc::c_double = 0.;
        let mut w2_imag: libc::c_double = 0.;
        let mut w3_real: libc::c_double = 0.;
        let mut w3_imag: libc::c_double = 0.;
        let mut w4_real: libc::c_double = 0.;
        let mut w4_imag: libc::c_double = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64;
            w1_imag = 0.0f64;
            w2_real = 1.0f64;
            w2_imag = 0.0f64;
            w3_real = 1.0f64;
            w3_imag = 0.0f64;
            w4_real = 1.0f64;
            w4_imag = 0.0f64;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = -(*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let mut x0_real: libc::c_double = 0.;
            let mut x0_imag: libc::c_double = 0.;
            let mut x1_real: libc::c_double = 0.;
            let mut x1_imag: libc::c_double = 0.;
            let mut x2_real: libc::c_double = 0.;
            let mut x2_imag: libc::c_double = 0.;
            let mut x3_real: libc::c_double = 0.;
            let mut x3_imag: libc::c_double = 0.;
            let mut x4_real: libc::c_double = 0.;
            let mut x4_imag: libc::c_double = 0.;
            let z0_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z4_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z4_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t1_real: libc::c_double = z1_real + z4_real;
            let t1_imag: libc::c_double = z1_imag + z4_imag;
            let t2_real: libc::c_double = z2_real + z3_real;
            let t2_imag: libc::c_double = z2_imag + z3_imag;
            let t3_real: libc::c_double = z1_real - z4_real;
            let t3_imag: libc::c_double = z1_imag - z4_imag;
            let t4_real: libc::c_double = z2_real - z3_real;
            let t4_imag: libc::c_double = z2_imag - z3_imag;
            let t5_real: libc::c_double = t1_real + t2_real;
            let t5_imag: libc::c_double = t1_imag + t2_imag;
            let t6_real: libc::c_double = sqrt(5.0f64) / 4.0f64 * (t1_real - t2_real);
            let t6_imag: libc::c_double = sqrt(5.0f64) / 4.0f64 * (t1_imag - t2_imag);
            let t7_real: libc::c_double = z0_real - t5_real / 4.0f64;
            let t7_imag: libc::c_double = z0_imag - t5_imag / 4.0f64;
            let t8_real: libc::c_double = t7_real + t6_real;
            let t8_imag: libc::c_double = t7_imag + t6_imag;
            let t9_real: libc::c_double = t7_real - t6_real;
            let t9_imag: libc::c_double = t7_imag - t6_imag;
            let t10_real: libc::c_double = sign as libc::c_int as libc::c_double
                * (sin_2pi_by_5 * t3_real + sin_2pi_by_10 * t4_real);
            let t10_imag: libc::c_double = sign as libc::c_int as libc::c_double
                * (sin_2pi_by_5 * t3_imag + sin_2pi_by_10 * t4_imag);
            let t11_real: libc::c_double = sign as libc::c_int as libc::c_double
                * (sin_2pi_by_10 * t3_real - sin_2pi_by_5 * t4_real);
            let t11_imag: libc::c_double = sign as libc::c_int as libc::c_double
                * (sin_2pi_by_10 * t3_imag - sin_2pi_by_5 * t4_imag);
            x0_real = z0_real + t5_real;
            x0_imag = z0_imag + t5_imag;
            x1_real = t8_real - t10_imag;
            x1_imag = t8_imag + t10_real;
            x2_real = t9_real - t11_imag;
            x2_imag = t9_imag + t11_real;
            x3_real = t9_real + t11_imag;
            x3_imag = t9_imag - t11_real;
            x4_real = t8_real + t10_imag;
            x4_imag = t8_imag - t10_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w4_real * x4_real - w4_imag * x4_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_5(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
    mut twiddle4: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 5 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let sin_2pi_by_5: libc::c_float = sin(2.0f64 * 3.14159265358979323846f64 / 5.0f64)
        as libc::c_float;
    let sin_2pi_by_10: libc::c_float = sin(2.0f64 * 3.14159265358979323846f64 / 10.0f64)
        as libc::c_float;
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_float = 0.;
        let mut w1_imag: libc::c_float = 0.;
        let mut w2_real: libc::c_float = 0.;
        let mut w2_imag: libc::c_float = 0.;
        let mut w3_real: libc::c_float = 0.;
        let mut w3_imag: libc::c_float = 0.;
        let mut w4_real: libc::c_float = 0.;
        let mut w4_imag: libc::c_float = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64 as libc::c_float;
            w1_imag = 0.0f64 as libc::c_float;
            w2_real = 1.0f64 as libc::c_float;
            w2_imag = 0.0f64 as libc::c_float;
            w3_real = 1.0f64 as libc::c_float;
            w3_imag = 0.0f64 as libc::c_float;
            w4_real = 1.0f64 as libc::c_float;
            w4_imag = 0.0f64 as libc::c_float;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = -(*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let mut x0_real: libc::c_float = 0.;
            let mut x0_imag: libc::c_float = 0.;
            let mut x1_real: libc::c_float = 0.;
            let mut x1_imag: libc::c_float = 0.;
            let mut x2_real: libc::c_float = 0.;
            let mut x2_imag: libc::c_float = 0.;
            let mut x3_real: libc::c_float = 0.;
            let mut x3_imag: libc::c_float = 0.;
            let mut x4_real: libc::c_float = 0.;
            let mut x4_imag: libc::c_float = 0.;
            let z0_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z4_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z4_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t1_real: libc::c_float = z1_real + z4_real;
            let t1_imag: libc::c_float = z1_imag + z4_imag;
            let t2_real: libc::c_float = z2_real + z3_real;
            let t2_imag: libc::c_float = z2_imag + z3_imag;
            let t3_real: libc::c_float = z1_real - z4_real;
            let t3_imag: libc::c_float = z1_imag - z4_imag;
            let t4_real: libc::c_float = z2_real - z3_real;
            let t4_imag: libc::c_float = z2_imag - z3_imag;
            let t5_real: libc::c_float = t1_real + t2_real;
            let t5_imag: libc::c_float = t1_imag + t2_imag;
            let t6_real: libc::c_float = (sqrt(5.0f64) / 4.0f64
                * (t1_real - t2_real) as libc::c_double) as libc::c_float;
            let t6_imag: libc::c_float = (sqrt(5.0f64) / 4.0f64
                * (t1_imag - t2_imag) as libc::c_double) as libc::c_float;
            let t7_real: libc::c_float = (z0_real as libc::c_double
                - t5_real as libc::c_double / 4.0f64) as libc::c_float;
            let t7_imag: libc::c_float = (z0_imag as libc::c_double
                - t5_imag as libc::c_double / 4.0f64) as libc::c_float;
            let t8_real: libc::c_float = t7_real + t6_real;
            let t8_imag: libc::c_float = t7_imag + t6_imag;
            let t9_real: libc::c_float = t7_real - t6_real;
            let t9_imag: libc::c_float = t7_imag - t6_imag;
            let t10_real: libc::c_float = sign as libc::c_int as libc::c_float
                * (sin_2pi_by_5 * t3_real + sin_2pi_by_10 * t4_real);
            let t10_imag: libc::c_float = sign as libc::c_int as libc::c_float
                * (sin_2pi_by_5 * t3_imag + sin_2pi_by_10 * t4_imag);
            let t11_real: libc::c_float = sign as libc::c_int as libc::c_float
                * (sin_2pi_by_10 * t3_real - sin_2pi_by_5 * t4_real);
            let t11_imag: libc::c_float = sign as libc::c_int as libc::c_float
                * (sin_2pi_by_10 * t3_imag - sin_2pi_by_5 * t4_imag);
            x0_real = z0_real + t5_real;
            x0_imag = z0_imag + t5_imag;
            x1_real = t8_real - t10_imag;
            x1_imag = t8_imag + t10_real;
            x2_real = t9_real - t11_imag;
            x2_imag = t9_imag + t11_real;
            x3_real = t9_real + t11_imag;
            x3_imag = t9_imag - t11_real;
            x4_real = t8_real + t10_imag;
            x4_imag = t8_imag - t10_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w4_real * x4_real - w4_imag * x4_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_pass_6(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
    mut twiddle4: *const gsl_complex,
    mut twiddle5: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 6 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let tau: libc::c_double = sqrt(3.0f64) / 2.0f64;
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_double = 0.;
        let mut w1_imag: libc::c_double = 0.;
        let mut w2_real: libc::c_double = 0.;
        let mut w2_imag: libc::c_double = 0.;
        let mut w3_real: libc::c_double = 0.;
        let mut w3_imag: libc::c_double = 0.;
        let mut w4_real: libc::c_double = 0.;
        let mut w4_imag: libc::c_double = 0.;
        let mut w5_real: libc::c_double = 0.;
        let mut w5_imag: libc::c_double = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64;
            w1_imag = 0.0f64;
            w2_real = 1.0f64;
            w2_imag = 0.0f64;
            w3_real = 1.0f64;
            w3_imag = 0.0f64;
            w4_real = 1.0f64;
            w4_imag = 0.0f64;
            w5_real = 1.0f64;
            w5_imag = 0.0f64;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = -(*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = -(*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let z0_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z4_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z4_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z5_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z5_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let ta1_real: libc::c_double = z2_real + z4_real;
            let ta1_imag: libc::c_double = z2_imag + z4_imag;
            let ta2_real: libc::c_double = z0_real
                - ta1_real / 2 as libc::c_int as libc::c_double;
            let ta2_imag: libc::c_double = z0_imag
                - ta1_imag / 2 as libc::c_int as libc::c_double;
            let ta3_real: libc::c_double = sign as libc::c_int as libc::c_double * tau
                * (z2_real - z4_real);
            let ta3_imag: libc::c_double = sign as libc::c_int as libc::c_double * tau
                * (z2_imag - z4_imag);
            let a0_real: libc::c_double = z0_real + ta1_real;
            let a0_imag: libc::c_double = z0_imag + ta1_imag;
            let a1_real: libc::c_double = ta2_real - ta3_imag;
            let a1_imag: libc::c_double = ta2_imag + ta3_real;
            let a2_real: libc::c_double = ta2_real + ta3_imag;
            let a2_imag: libc::c_double = ta2_imag - ta3_real;
            let tb1_real: libc::c_double = z5_real + z1_real;
            let tb1_imag: libc::c_double = z5_imag + z1_imag;
            let tb2_real: libc::c_double = z3_real
                - tb1_real / 2 as libc::c_int as libc::c_double;
            let tb2_imag: libc::c_double = z3_imag
                - tb1_imag / 2 as libc::c_int as libc::c_double;
            let tb3_real: libc::c_double = sign as libc::c_int as libc::c_double * tau
                * (z5_real - z1_real);
            let tb3_imag: libc::c_double = sign as libc::c_int as libc::c_double * tau
                * (z5_imag - z1_imag);
            let b0_real: libc::c_double = z3_real + tb1_real;
            let b0_imag: libc::c_double = z3_imag + tb1_imag;
            let b1_real: libc::c_double = tb2_real - tb3_imag;
            let b1_imag: libc::c_double = tb2_imag + tb3_real;
            let b2_real: libc::c_double = tb2_real + tb3_imag;
            let b2_imag: libc::c_double = tb2_imag - tb3_real;
            let x0_real: libc::c_double = a0_real + b0_real;
            let x0_imag: libc::c_double = a0_imag + b0_imag;
            let x4_real: libc::c_double = a1_real + b1_real;
            let x4_imag: libc::c_double = a1_imag + b1_imag;
            let x2_real: libc::c_double = a2_real + b2_real;
            let x2_imag: libc::c_double = a2_imag + b2_imag;
            let x3_real: libc::c_double = a0_real - b0_real;
            let x3_imag: libc::c_double = a0_imag - b0_imag;
            let x1_real: libc::c_double = a1_real - b1_real;
            let x1_imag: libc::c_double = a1_imag - b1_imag;
            let x5_real: libc::c_double = a2_real - b2_real;
            let x5_imag: libc::c_double = a2_imag - b2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w4_real * x4_real - w4_imag * x4_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w5_real * x5_real - w5_imag * x5_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w5_real * x5_imag + w5_imag * x5_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_6(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
    mut twiddle4: *const gsl_complex_float,
    mut twiddle5: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 6 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let tau: libc::c_float = (sqrt(3.0f64) / 2.0f64) as libc::c_float;
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_float = 0.;
        let mut w1_imag: libc::c_float = 0.;
        let mut w2_real: libc::c_float = 0.;
        let mut w2_imag: libc::c_float = 0.;
        let mut w3_real: libc::c_float = 0.;
        let mut w3_imag: libc::c_float = 0.;
        let mut w4_real: libc::c_float = 0.;
        let mut w4_imag: libc::c_float = 0.;
        let mut w5_real: libc::c_float = 0.;
        let mut w5_imag: libc::c_float = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64 as libc::c_float;
            w1_imag = 0.0f64 as libc::c_float;
            w2_real = 1.0f64 as libc::c_float;
            w2_imag = 0.0f64 as libc::c_float;
            w3_real = 1.0f64 as libc::c_float;
            w3_imag = 0.0f64 as libc::c_float;
            w4_real = 1.0f64 as libc::c_float;
            w4_imag = 0.0f64 as libc::c_float;
            w5_real = 1.0f64 as libc::c_float;
            w5_imag = 0.0f64 as libc::c_float;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = -(*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = -(*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let z0_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z4_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z4_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z5_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z5_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let ta1_real: libc::c_float = z2_real + z4_real;
            let ta1_imag: libc::c_float = z2_imag + z4_imag;
            let ta2_real: libc::c_float = z0_real
                - ta1_real / 2 as libc::c_int as libc::c_float;
            let ta2_imag: libc::c_float = z0_imag
                - ta1_imag / 2 as libc::c_int as libc::c_float;
            let ta3_real: libc::c_float = sign as libc::c_int as libc::c_float * tau
                * (z2_real - z4_real);
            let ta3_imag: libc::c_float = sign as libc::c_int as libc::c_float * tau
                * (z2_imag - z4_imag);
            let a0_real: libc::c_float = z0_real + ta1_real;
            let a0_imag: libc::c_float = z0_imag + ta1_imag;
            let a1_real: libc::c_float = ta2_real - ta3_imag;
            let a1_imag: libc::c_float = ta2_imag + ta3_real;
            let a2_real: libc::c_float = ta2_real + ta3_imag;
            let a2_imag: libc::c_float = ta2_imag - ta3_real;
            let tb1_real: libc::c_float = z5_real + z1_real;
            let tb1_imag: libc::c_float = z5_imag + z1_imag;
            let tb2_real: libc::c_float = z3_real
                - tb1_real / 2 as libc::c_int as libc::c_float;
            let tb2_imag: libc::c_float = z3_imag
                - tb1_imag / 2 as libc::c_int as libc::c_float;
            let tb3_real: libc::c_float = sign as libc::c_int as libc::c_float * tau
                * (z5_real - z1_real);
            let tb3_imag: libc::c_float = sign as libc::c_int as libc::c_float * tau
                * (z5_imag - z1_imag);
            let b0_real: libc::c_float = z3_real + tb1_real;
            let b0_imag: libc::c_float = z3_imag + tb1_imag;
            let b1_real: libc::c_float = tb2_real - tb3_imag;
            let b1_imag: libc::c_float = tb2_imag + tb3_real;
            let b2_real: libc::c_float = tb2_real + tb3_imag;
            let b2_imag: libc::c_float = tb2_imag - tb3_real;
            let x0_real: libc::c_float = a0_real + b0_real;
            let x0_imag: libc::c_float = a0_imag + b0_imag;
            let x4_real: libc::c_float = a1_real + b1_real;
            let x4_imag: libc::c_float = a1_imag + b1_imag;
            let x2_real: libc::c_float = a2_real + b2_real;
            let x2_imag: libc::c_float = a2_imag + b2_imag;
            let x3_real: libc::c_float = a0_real - b0_real;
            let x3_imag: libc::c_float = a0_imag - b0_imag;
            let x1_real: libc::c_float = a1_real - b1_real;
            let x1_imag: libc::c_float = a1_imag - b1_imag;
            let x5_real: libc::c_float = a2_real - b2_real;
            let x5_imag: libc::c_float = a2_imag - b2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w4_real * x4_real - w4_imag * x4_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w5_real * x5_real - w5_imag * x5_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w5_real * x5_imag + w5_imag * x5_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_7(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
    mut twiddle4: *const gsl_complex_float,
    mut twiddle5: *const gsl_complex_float,
    mut twiddle6: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 7 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let c1: libc::c_float = cos(1.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64)
        as libc::c_float;
    let c2: libc::c_float = cos(2.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64)
        as libc::c_float;
    let c3: libc::c_float = cos(3.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64)
        as libc::c_float;
    let s1: libc::c_float = sin(1.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64)
        as libc::c_float;
    let s2: libc::c_float = sin(2.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64)
        as libc::c_float;
    let s3: libc::c_float = sin(3.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64)
        as libc::c_float;
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_float = 0.;
        let mut w1_imag: libc::c_float = 0.;
        let mut w2_real: libc::c_float = 0.;
        let mut w2_imag: libc::c_float = 0.;
        let mut w3_real: libc::c_float = 0.;
        let mut w3_imag: libc::c_float = 0.;
        let mut w4_real: libc::c_float = 0.;
        let mut w4_imag: libc::c_float = 0.;
        let mut w5_real: libc::c_float = 0.;
        let mut w5_imag: libc::c_float = 0.;
        let mut w6_real: libc::c_float = 0.;
        let mut w6_imag: libc::c_float = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64 as libc::c_float;
            w1_imag = 0.0f64 as libc::c_float;
            w2_real = 1.0f64 as libc::c_float;
            w2_imag = 0.0f64 as libc::c_float;
            w3_real = 1.0f64 as libc::c_float;
            w3_imag = 0.0f64 as libc::c_float;
            w4_real = 1.0f64 as libc::c_float;
            w4_imag = 0.0f64 as libc::c_float;
            w5_real = 1.0f64 as libc::c_float;
            w5_imag = 0.0f64 as libc::c_float;
            w6_real = 1.0f64 as libc::c_float;
            w6_imag = 0.0f64 as libc::c_float;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w6_real = (*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w6_imag = (*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = -(*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = -(*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w6_real = (*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w6_imag = -(*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let z0_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z4_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z4_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z5_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z5_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z6_real: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z6_imag: libc::c_float = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t0_real: libc::c_float = z1_real + z6_real;
            let t0_imag: libc::c_float = z1_imag + z6_imag;
            let t1_real: libc::c_float = z1_real - z6_real;
            let t1_imag: libc::c_float = z1_imag - z6_imag;
            let t2_real: libc::c_float = z2_real + z5_real;
            let t2_imag: libc::c_float = z2_imag + z5_imag;
            let t3_real: libc::c_float = z2_real - z5_real;
            let t3_imag: libc::c_float = z2_imag - z5_imag;
            let t4_real: libc::c_float = z4_real + z3_real;
            let t4_imag: libc::c_float = z4_imag + z3_imag;
            let t5_real: libc::c_float = z4_real - z3_real;
            let t5_imag: libc::c_float = z4_imag - z3_imag;
            let t6_real: libc::c_float = t2_real + t0_real;
            let t6_imag: libc::c_float = t2_imag + t0_imag;
            let t7_real: libc::c_float = t5_real + t3_real;
            let t7_imag: libc::c_float = t5_imag + t3_imag;
            let b0_real: libc::c_float = z0_real + t6_real + t4_real;
            let b0_imag: libc::c_float = z0_imag + t6_imag + t4_imag;
            let b1_real: libc::c_float = (((c1 + c2 + c3) as libc::c_double / 3.0f64
                - 1.0f64) * (t6_real + t4_real) as libc::c_double) as libc::c_float;
            let b1_imag: libc::c_float = (((c1 + c2 + c3) as libc::c_double / 3.0f64
                - 1.0f64) * (t6_imag + t4_imag) as libc::c_double) as libc::c_float;
            let b2_real: libc::c_float = ((2.0f64 * c1 as libc::c_double
                - c2 as libc::c_double - c3 as libc::c_double) / 3.0f64
                * (t0_real - t4_real) as libc::c_double) as libc::c_float;
            let b2_imag: libc::c_float = ((2.0f64 * c1 as libc::c_double
                - c2 as libc::c_double - c3 as libc::c_double) / 3.0f64
                * (t0_imag - t4_imag) as libc::c_double) as libc::c_float;
            let b3_real: libc::c_float = ((c1 as libc::c_double
                - 2.0f64 * c2 as libc::c_double + c3 as libc::c_double) / 3.0f64
                * (t4_real - t2_real) as libc::c_double) as libc::c_float;
            let b3_imag: libc::c_float = ((c1 as libc::c_double
                - 2.0f64 * c2 as libc::c_double + c3 as libc::c_double) / 3.0f64
                * (t4_imag - t2_imag) as libc::c_double) as libc::c_float;
            let b4_real: libc::c_float = (((c1 + c2) as libc::c_double
                - 2.0f64 * c3 as libc::c_double) / 3.0f64
                * (t2_real - t0_real) as libc::c_double) as libc::c_float;
            let b4_imag: libc::c_float = (((c1 + c2) as libc::c_double
                - 2.0f64 * c3 as libc::c_double) / 3.0f64
                * (t2_imag - t0_imag) as libc::c_double) as libc::c_float;
            let b5_real: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * ((s1 + s2 - s3) as libc::c_double / 3.0f64)
                * (t7_real + t1_real) as libc::c_double) as libc::c_float;
            let b5_imag: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * ((s1 + s2 - s3) as libc::c_double / 3.0f64)
                * (t7_imag + t1_imag) as libc::c_double) as libc::c_float;
            let b6_real: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * ((2.0f64 * s1 as libc::c_double - s2 as libc::c_double
                    + s3 as libc::c_double) / 3.0f64)
                * (t1_real - t5_real) as libc::c_double) as libc::c_float;
            let b6_imag: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * ((2.0f64 * s1 as libc::c_double - s2 as libc::c_double
                    + s3 as libc::c_double) / 3.0f64)
                * (t1_imag - t5_imag) as libc::c_double) as libc::c_float;
            let b7_real: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * ((s1 as libc::c_double - 2.0f64 * s2 as libc::c_double
                    - s3 as libc::c_double) / 3.0f64)
                * (t5_real - t3_real) as libc::c_double) as libc::c_float;
            let b7_imag: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * ((s1 as libc::c_double - 2.0f64 * s2 as libc::c_double
                    - s3 as libc::c_double) / 3.0f64)
                * (t5_imag - t3_imag) as libc::c_double) as libc::c_float;
            let b8_real: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * (((s1 + s2) as libc::c_double + 2.0f64 * s3 as libc::c_double)
                    / 3.0f64) * (t3_real - t1_real) as libc::c_double) as libc::c_float;
            let b8_imag: libc::c_float = (-(sign as libc::c_int) as libc::c_double
                * (((s1 + s2) as libc::c_double + 2.0f64 * s3 as libc::c_double)
                    / 3.0f64) * (t3_imag - t1_imag) as libc::c_double) as libc::c_float;
            let T0_real: libc::c_float = b0_real + b1_real;
            let T0_imag: libc::c_float = b0_imag + b1_imag;
            let T1_real: libc::c_float = b2_real + b3_real;
            let T1_imag: libc::c_float = b2_imag + b3_imag;
            let T2_real: libc::c_float = b4_real - b3_real;
            let T2_imag: libc::c_float = b4_imag - b3_imag;
            let T3_real: libc::c_float = -b2_real - b4_real;
            let T3_imag: libc::c_float = -b2_imag - b4_imag;
            let T4_real: libc::c_float = b6_real + b7_real;
            let T4_imag: libc::c_float = b6_imag + b7_imag;
            let T5_real: libc::c_float = b8_real - b7_real;
            let T5_imag: libc::c_float = b8_imag - b7_imag;
            let T6_real: libc::c_float = -b8_real - b6_real;
            let T6_imag: libc::c_float = -b8_imag - b6_imag;
            let T7_real: libc::c_float = T0_real + T1_real;
            let T7_imag: libc::c_float = T0_imag + T1_imag;
            let T8_real: libc::c_float = T0_real + T2_real;
            let T8_imag: libc::c_float = T0_imag + T2_imag;
            let T9_real: libc::c_float = T0_real + T3_real;
            let T9_imag: libc::c_float = T0_imag + T3_imag;
            let T10_real: libc::c_float = T4_real + b5_real;
            let T10_imag: libc::c_float = T4_imag + b5_imag;
            let T11_real: libc::c_float = T5_real + b5_real;
            let T11_imag: libc::c_float = T5_imag + b5_imag;
            let T12_real: libc::c_float = T6_real + b5_real;
            let T12_imag: libc::c_float = T6_imag + b5_imag;
            let x0_real: libc::c_float = b0_real;
            let x0_imag: libc::c_float = b0_imag;
            let x1_real: libc::c_float = T7_real + T10_imag;
            let x1_imag: libc::c_float = T7_imag - T10_real;
            let x2_real: libc::c_float = T9_real + T12_imag;
            let x2_imag: libc::c_float = T9_imag - T12_real;
            let x3_real: libc::c_float = T8_real - T11_imag;
            let x3_imag: libc::c_float = T8_imag + T11_real;
            let x4_real: libc::c_float = T8_real + T11_imag;
            let x4_imag: libc::c_float = T8_imag - T11_real;
            let x5_real: libc::c_float = T9_real - T12_imag;
            let x5_imag: libc::c_float = T9_imag + T12_real;
            let x6_real: libc::c_float = T7_real - T10_imag;
            let x6_imag: libc::c_float = T7_imag + T10_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w4_real * x4_real - w4_imag * x4_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w5_real * x5_real - w5_imag * x5_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w5_real * x5_imag + w5_imag * x5_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w6_real * x6_real - w6_imag * x6_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w6_real * x6_imag + w6_imag * x6_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_pass_7(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
    mut twiddle4: *const gsl_complex,
    mut twiddle5: *const gsl_complex,
    mut twiddle6: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 7 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let c1: libc::c_double = cos(1.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64);
    let c2: libc::c_double = cos(2.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64);
    let c3: libc::c_double = cos(3.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64);
    let s1: libc::c_double = sin(1.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64);
    let s2: libc::c_double = sin(2.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64);
    let s3: libc::c_double = sin(3.0f64 * 2.0f64 * 3.14159265358979323846f64 / 7.0f64);
    k = 0 as libc::c_int as size_t;
    while k < q {
        let mut w1_real: libc::c_double = 0.;
        let mut w1_imag: libc::c_double = 0.;
        let mut w2_real: libc::c_double = 0.;
        let mut w2_imag: libc::c_double = 0.;
        let mut w3_real: libc::c_double = 0.;
        let mut w3_imag: libc::c_double = 0.;
        let mut w4_real: libc::c_double = 0.;
        let mut w4_imag: libc::c_double = 0.;
        let mut w5_real: libc::c_double = 0.;
        let mut w5_imag: libc::c_double = 0.;
        let mut w6_real: libc::c_double = 0.;
        let mut w6_imag: libc::c_double = 0.;
        if k == 0 as libc::c_int as libc::c_ulong {
            w1_real = 1.0f64;
            w1_imag = 0.0f64;
            w2_real = 1.0f64;
            w2_imag = 0.0f64;
            w3_real = 1.0f64;
            w3_imag = 0.0f64;
            w4_real = 1.0f64;
            w4_imag = 0.0f64;
            w5_real = 1.0f64;
            w5_imag = 0.0f64;
            w6_real = 1.0f64;
            w6_imag = 0.0f64;
        } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w6_real = (*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w6_imag = (*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        } else {
            w1_real = (*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w1_imag = -(*twiddle1
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w2_real = (*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w2_imag = -(*twiddle2
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w3_real = (*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w3_imag = -(*twiddle3
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w4_real = (*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w4_imag = -(*twiddle4
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w5_real = (*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w5_imag = -(*twiddle5
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
            w6_real = (*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[0 as libc::c_int as usize];
            w6_imag = -(*twiddle6
                .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .dat[1 as libc::c_int as usize];
        }
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            let z0_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            let z0_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m)) as isize,
                );
            let z1_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(m))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z2_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z2_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z3_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z3_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z4_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z4_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z5_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z5_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let z6_real: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        ) as isize,
                );
            let z6_imag: libc::c_double = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(
                            i
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(m),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let t0_real: libc::c_double = z1_real + z6_real;
            let t0_imag: libc::c_double = z1_imag + z6_imag;
            let t1_real: libc::c_double = z1_real - z6_real;
            let t1_imag: libc::c_double = z1_imag - z6_imag;
            let t2_real: libc::c_double = z2_real + z5_real;
            let t2_imag: libc::c_double = z2_imag + z5_imag;
            let t3_real: libc::c_double = z2_real - z5_real;
            let t3_imag: libc::c_double = z2_imag - z5_imag;
            let t4_real: libc::c_double = z4_real + z3_real;
            let t4_imag: libc::c_double = z4_imag + z3_imag;
            let t5_real: libc::c_double = z4_real - z3_real;
            let t5_imag: libc::c_double = z4_imag - z3_imag;
            let t6_real: libc::c_double = t2_real + t0_real;
            let t6_imag: libc::c_double = t2_imag + t0_imag;
            let t7_real: libc::c_double = t5_real + t3_real;
            let t7_imag: libc::c_double = t5_imag + t3_imag;
            let b0_real: libc::c_double = z0_real + t6_real + t4_real;
            let b0_imag: libc::c_double = z0_imag + t6_imag + t4_imag;
            let b1_real: libc::c_double = ((c1 + c2 + c3) / 3.0f64 - 1.0f64)
                * (t6_real + t4_real);
            let b1_imag: libc::c_double = ((c1 + c2 + c3) / 3.0f64 - 1.0f64)
                * (t6_imag + t4_imag);
            let b2_real: libc::c_double = (2.0f64 * c1 - c2 - c3) / 3.0f64
                * (t0_real - t4_real);
            let b2_imag: libc::c_double = (2.0f64 * c1 - c2 - c3) / 3.0f64
                * (t0_imag - t4_imag);
            let b3_real: libc::c_double = (c1 - 2.0f64 * c2 + c3) / 3.0f64
                * (t4_real - t2_real);
            let b3_imag: libc::c_double = (c1 - 2.0f64 * c2 + c3) / 3.0f64
                * (t4_imag - t2_imag);
            let b4_real: libc::c_double = (c1 + c2 - 2.0f64 * c3) / 3.0f64
                * (t2_real - t0_real);
            let b4_imag: libc::c_double = (c1 + c2 - 2.0f64 * c3) / 3.0f64
                * (t2_imag - t0_imag);
            let b5_real: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((s1 + s2 - s3) / 3.0f64) * (t7_real + t1_real);
            let b5_imag: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((s1 + s2 - s3) / 3.0f64) * (t7_imag + t1_imag);
            let b6_real: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((2.0f64 * s1 - s2 + s3) / 3.0f64) * (t1_real - t5_real);
            let b6_imag: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((2.0f64 * s1 - s2 + s3) / 3.0f64) * (t1_imag - t5_imag);
            let b7_real: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((s1 - 2.0f64 * s2 - s3) / 3.0f64) * (t5_real - t3_real);
            let b7_imag: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((s1 - 2.0f64 * s2 - s3) / 3.0f64) * (t5_imag - t3_imag);
            let b8_real: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((s1 + s2 + 2.0f64 * s3) / 3.0f64) * (t3_real - t1_real);
            let b8_imag: libc::c_double = -(sign as libc::c_int) as libc::c_double
                * ((s1 + s2 + 2.0f64 * s3) / 3.0f64) * (t3_imag - t1_imag);
            let T0_real: libc::c_double = b0_real + b1_real;
            let T0_imag: libc::c_double = b0_imag + b1_imag;
            let T1_real: libc::c_double = b2_real + b3_real;
            let T1_imag: libc::c_double = b2_imag + b3_imag;
            let T2_real: libc::c_double = b4_real - b3_real;
            let T2_imag: libc::c_double = b4_imag - b3_imag;
            let T3_real: libc::c_double = -b2_real - b4_real;
            let T3_imag: libc::c_double = -b2_imag - b4_imag;
            let T4_real: libc::c_double = b6_real + b7_real;
            let T4_imag: libc::c_double = b6_imag + b7_imag;
            let T5_real: libc::c_double = b8_real - b7_real;
            let T5_imag: libc::c_double = b8_imag - b7_imag;
            let T6_real: libc::c_double = -b8_real - b6_real;
            let T6_imag: libc::c_double = -b8_imag - b6_imag;
            let T7_real: libc::c_double = T0_real + T1_real;
            let T7_imag: libc::c_double = T0_imag + T1_imag;
            let T8_real: libc::c_double = T0_real + T2_real;
            let T8_imag: libc::c_double = T0_imag + T2_imag;
            let T9_real: libc::c_double = T0_real + T3_real;
            let T9_imag: libc::c_double = T0_imag + T3_imag;
            let T10_real: libc::c_double = T4_real + b5_real;
            let T10_imag: libc::c_double = T4_imag + b5_imag;
            let T11_real: libc::c_double = T5_real + b5_real;
            let T11_imag: libc::c_double = T5_imag + b5_imag;
            let T12_real: libc::c_double = T6_real + b5_real;
            let T12_imag: libc::c_double = T6_imag + b5_imag;
            let x0_real: libc::c_double = b0_real;
            let x0_imag: libc::c_double = b0_imag;
            let x1_real: libc::c_double = T7_real + T10_imag;
            let x1_imag: libc::c_double = T7_imag - T10_real;
            let x2_real: libc::c_double = T9_real + T12_imag;
            let x2_imag: libc::c_double = T9_imag - T12_real;
            let x3_real: libc::c_double = T8_real - T11_imag;
            let x3_imag: libc::c_double = T8_imag + T11_real;
            let x4_real: libc::c_double = T8_real + T11_imag;
            let x4_imag: libc::c_double = T8_imag - T11_real;
            let x5_real: libc::c_double = T9_real - T12_imag;
            let x5_imag: libc::c_double = T9_imag + T12_real;
            let x6_real: libc::c_double = T7_real - T10_imag;
            let x6_imag: libc::c_double = T7_imag + T10_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = x0_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = x0_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1)) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j.wrapping_add(p_1))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w4_real * x4_real - w4_imag * x4_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w5_real * x5_real - w5_imag * x5_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (5 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w5_real * x5_imag + w5_imag * x5_real;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        ) as isize,
                ) = w6_real * x6_real - w6_imag * x6_imag;
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(
                            j
                                .wrapping_add(
                                    (6 as libc::c_int as libc::c_ulong).wrapping_mul(p_1),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = w6_real * x6_imag + w6_imag * x6_real;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_float_pass_n(
    mut in_0: *mut libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    sign: gsl_fft_direction,
    factor: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex_float,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let mut e: size_t = 0;
    let mut e1: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < m {
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(ostride).wrapping_mul(i)
                    as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(istride).wrapping_mul(i)
                    as isize,
            );
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    e = 1 as libc::c_int as size_t;
    while e
        < factor
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        i = 0 as libc::c_int as size_t;
        while i < m {
            let idx: size_t = i.wrapping_add(e.wrapping_mul(m));
            let idxc: size_t = i.wrapping_add(factor.wrapping_sub(e).wrapping_mul(m));
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idx) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx) as isize,
                )
                + *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc) as isize,
                    );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idx)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                + *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idxc) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx) as isize,
                )
                - *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc) as isize,
                    );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idxc)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                - *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        e = e.wrapping_add(1);
        e;
    }
    i = 0 as libc::c_int as size_t;
    while i < m {
        *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(istride).wrapping_mul(i)
                    as isize,
            ) = *out
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(ostride).wrapping_mul(i)
                    as isize,
            );
        *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    e1 = 1 as libc::c_int as size_t;
    while e1
        < factor
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        i = 0 as libc::c_int as size_t;
        while i < m {
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                )
                += *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m))) as isize,
                    );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                += *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        e1 = e1.wrapping_add(1);
        e1;
    }
    e = 1 as libc::c_int as size_t;
    while e
        < factor
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut idx_0: size_t = e.wrapping_mul(q);
        let idx_step: size_t = e.wrapping_mul(q);
        let mut w_real: libc::c_float = 0.;
        let mut w_imag: libc::c_float = 0.;
        let em: size_t = e.wrapping_mul(m);
        let ecm: size_t = factor.wrapping_sub(e).wrapping_mul(m);
        i = 0 as libc::c_int as size_t;
        while i < m {
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(em)) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i) as isize,
                );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(em))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(ecm)) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i) as isize,
                );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(ecm))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
        e1 = 1 as libc::c_int as size_t;
        while e1
            < factor
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            if idx_0 == 0 as libc::c_int as libc::c_ulong {
                w_real = 1 as libc::c_int as libc::c_float;
                w_imag = 0 as libc::c_int as libc::c_float;
            } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
                w_real = (*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[0 as libc::c_int as usize];
                w_imag = (*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[1 as libc::c_int as usize];
            } else {
                w_real = (*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[0 as libc::c_int as usize];
                w_imag = -(*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[1 as libc::c_int as usize];
            }
            i = 0 as libc::c_int as size_t;
            while i < m {
                let xp_real: libc::c_float = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m))) as isize,
                    );
                let xp_imag: libc::c_float = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let xm_real: libc::c_float = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(
                                i.wrapping_add(factor.wrapping_sub(e1).wrapping_mul(m)),
                            ) as isize,
                    );
                let xm_imag: libc::c_float = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(
                                i.wrapping_add(factor.wrapping_sub(e1).wrapping_mul(m)),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let ap: libc::c_float = w_real * xp_real;
                let am: libc::c_float = w_imag * xm_imag;
                let mut sum_real: libc::c_float = ap - am;
                let mut sumc_real: libc::c_float = ap + am;
                let bp: libc::c_float = w_real * xp_imag;
                let bm: libc::c_float = w_imag * xm_real;
                let mut sum_imag: libc::c_float = bp + bm;
                let mut sumc_imag: libc::c_float = bp - bm;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(em)) as isize,
                    ) += sum_real;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(em))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) += sum_imag;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(ecm)) as isize,
                    ) += sumc_real;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(ecm))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) += sumc_imag;
                i = i.wrapping_add(1);
                i;
            }
            idx_0 = (idx_0 as libc::c_ulong).wrapping_add(idx_step) as size_t as size_t;
            idx_0 = (idx_0 as libc::c_ulong).wrapping_rem(factor.wrapping_mul(q))
                as size_t as size_t;
            e1 = e1.wrapping_add(1);
            e1;
        }
        e = e.wrapping_add(1);
        e;
    }
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    k1 = 0 as libc::c_int as size_t;
    while k1 < p_1 {
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(k1) as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(k1) as isize,
            );
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(k1)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(k1)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        k1 = k1.wrapping_add(1);
        k1;
    }
    e1 = 1 as libc::c_int as size_t;
    while e1 < factor {
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(p_1))) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(m))) as isize,
                );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(p_1)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(m)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            k1 = k1.wrapping_add(1);
            k1;
        }
        e1 = e1.wrapping_add(1);
        e1;
    }
    i = p_1;
    j = product;
    k = 1 as libc::c_int as size_t;
    while k < q {
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    i = p_1;
    j = product;
    k = 1 as libc::c_int as size_t;
    while k < q {
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            e1 = 1 as libc::c_int as size_t;
            while e1 < factor {
                let mut x_real: libc::c_float = *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m))) as isize,
                    );
                let mut x_imag: libc::c_float = *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut w_real_0: libc::c_float = 0.;
                let mut w_imag_0: libc::c_float = 0.;
                if sign as libc::c_int == gsl_fft_forward as libc::c_int {
                    w_real_0 = (*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[0 as libc::c_int as usize];
                    w_imag_0 = (*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[1 as libc::c_int as usize];
                } else {
                    w_real_0 = (*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[0 as libc::c_int as usize];
                    w_imag_0 = -(*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[1 as libc::c_int as usize];
                }
                *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(j.wrapping_add(e1.wrapping_mul(p_1))) as isize,
                    ) = w_real_0 * x_real - w_imag_0 * x_imag;
                *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(j.wrapping_add(e1.wrapping_mul(p_1)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = w_real_0 * x_imag + w_imag_0 * x_real;
                e1 = e1.wrapping_add(1);
                e1;
            }
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_complex_pass_n(
    mut in_0: *mut libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    sign: gsl_fft_direction,
    factor: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let p_1: size_t = product.wrapping_div(factor);
    let jump: size_t = factor
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(p_1);
    let mut e: size_t = 0;
    let mut e1: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < m {
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(ostride).wrapping_mul(i)
                    as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(istride).wrapping_mul(i)
                    as isize,
            );
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    e = 1 as libc::c_int as size_t;
    while e
        < factor
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        i = 0 as libc::c_int as size_t;
        while i < m {
            let idx: size_t = i.wrapping_add(e.wrapping_mul(m));
            let idxc: size_t = i.wrapping_add(factor.wrapping_sub(e).wrapping_mul(m));
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idx) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx) as isize,
                )
                + *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc) as isize,
                    );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idx)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                + *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idxc) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx) as isize,
                )
                - *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc) as isize,
                    );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(idxc)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(idx)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                - *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(idxc)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        e = e.wrapping_add(1);
        e;
    }
    i = 0 as libc::c_int as size_t;
    while i < m {
        *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(istride).wrapping_mul(i)
                    as isize,
            ) = *out
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(ostride).wrapping_mul(i)
                    as isize,
            );
        *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    e1 = 1 as libc::c_int as size_t;
    while e1
        < factor
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        i = 0 as libc::c_int as size_t;
        while i < m {
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                )
                += *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m))) as isize,
                    );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                += *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        e1 = e1.wrapping_add(1);
        e1;
    }
    e = 1 as libc::c_int as size_t;
    while e
        < factor
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut idx_0: size_t = e.wrapping_mul(q);
        let idx_step: size_t = e.wrapping_mul(q);
        let mut w_real: libc::c_double = 0.;
        let mut w_imag: libc::c_double = 0.;
        let em: size_t = e.wrapping_mul(m);
        let ecm: size_t = factor.wrapping_sub(e).wrapping_mul(m);
        i = 0 as libc::c_int as size_t;
        while i < m {
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(em)) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i) as isize,
                );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(em))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(ecm)) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i) as isize,
                );
            *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i.wrapping_add(ecm))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
        e1 = 1 as libc::c_int as size_t;
        while e1
            < factor
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            if idx_0 == 0 as libc::c_int as libc::c_ulong {
                w_real = 1 as libc::c_int as libc::c_double;
                w_imag = 0 as libc::c_int as libc::c_double;
            } else if sign as libc::c_int == gsl_fft_forward as libc::c_int {
                w_real = (*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[0 as libc::c_int as usize];
                w_imag = (*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[1 as libc::c_int as usize];
            } else {
                w_real = (*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[0 as libc::c_int as usize];
                w_imag = -(*twiddle
                    .offset(
                        idx_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .dat[1 as libc::c_int as usize];
            }
            i = 0 as libc::c_int as size_t;
            while i < m {
                let xp_real: libc::c_double = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m))) as isize,
                    );
                let xp_imag: libc::c_double = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let xm_real: libc::c_double = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(
                                i.wrapping_add(factor.wrapping_sub(e1).wrapping_mul(m)),
                            ) as isize,
                    );
                let xm_imag: libc::c_double = *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(
                                i.wrapping_add(factor.wrapping_sub(e1).wrapping_mul(m)),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let ap: libc::c_double = w_real * xp_real;
                let am: libc::c_double = w_imag * xm_imag;
                let mut sum_real: libc::c_double = ap - am;
                let mut sumc_real: libc::c_double = ap + am;
                let bp: libc::c_double = w_real * xp_imag;
                let bm: libc::c_double = w_imag * xm_real;
                let mut sum_imag: libc::c_double = bp + bm;
                let mut sumc_imag: libc::c_double = bp - bm;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(em)) as isize,
                    ) += sum_real;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(em))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) += sum_imag;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(ecm)) as isize,
                    ) += sumc_real;
                *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(ecm))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) += sumc_imag;
                i = i.wrapping_add(1);
                i;
            }
            idx_0 = (idx_0 as libc::c_ulong).wrapping_add(idx_step) as size_t as size_t;
            idx_0 = (idx_0 as libc::c_ulong).wrapping_rem(factor.wrapping_mul(q))
                as size_t as size_t;
            e1 = e1.wrapping_add(1);
            e1;
        }
        e = e.wrapping_add(1);
        e;
    }
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    k1 = 0 as libc::c_int as size_t;
    while k1 < p_1 {
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(k1) as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(k1) as isize,
            );
        *out
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(ostride)
                    .wrapping_mul(k1)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *in_0
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(istride)
                    .wrapping_mul(k1)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        k1 = k1.wrapping_add(1);
        k1;
    }
    e1 = 1 as libc::c_int as size_t;
    while e1 < factor {
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(p_1))) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(m))) as isize,
                );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(p_1)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(k1.wrapping_add(e1.wrapping_mul(m)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            k1 = k1.wrapping_add(1);
            k1;
        }
        e1 = e1.wrapping_add(1);
        e1;
    }
    i = p_1;
    j = product;
    k = 1 as libc::c_int as size_t;
    while k < q {
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i) as isize,
                );
            *out
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(ostride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *in_0
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(istride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    i = p_1;
    j = product;
    k = 1 as libc::c_int as size_t;
    while k < q {
        k1 = 0 as libc::c_int as size_t;
        while k1 < p_1 {
            e1 = 1 as libc::c_int as size_t;
            while e1 < factor {
                let mut x_real: libc::c_double = *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m))) as isize,
                    );
                let mut x_imag: libc::c_double = *in_0
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(istride)
                            .wrapping_mul(i.wrapping_add(e1.wrapping_mul(m)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let mut w_real_0: libc::c_double = 0.;
                let mut w_imag_0: libc::c_double = 0.;
                if sign as libc::c_int == gsl_fft_forward as libc::c_int {
                    w_real_0 = (*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[0 as libc::c_int as usize];
                    w_imag_0 = (*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[1 as libc::c_int as usize];
                } else {
                    w_real_0 = (*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[0 as libc::c_int as usize];
                    w_imag_0 = -(*twiddle
                        .offset(
                            e1
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(q)
                                .wrapping_add(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .dat[1 as libc::c_int as usize];
                }
                *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(j.wrapping_add(e1.wrapping_mul(p_1))) as isize,
                    ) = w_real_0 * x_real - w_imag_0 * x_imag;
                *out
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(ostride)
                            .wrapping_mul(j.wrapping_add(e1.wrapping_mul(p_1)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = w_real_0 * x_imag + w_imag_0 * x_real;
                e1 = e1.wrapping_add(1);
                e1;
            }
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
            k1 = k1.wrapping_add(1);
            k1;
        }
        j = (j as libc::c_ulong).wrapping_add(jump) as size_t as size_t;
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_forward(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_fft_complex_float_radix2_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_forward(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_fft_complex_radix2_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_backward(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_radix2_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_backward(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_float_radix2_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_inverse(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_radix2_transform(
        data,
        stride,
        n,
        sign,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_inverse(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_float_radix2_transform(
        data,
        stride,
        n,
        sign,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_float = (1.0f64 / n as libc::c_double) as libc::c_float;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_transform(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut dual: size_t = 0;
    let mut bit: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./c_radix2.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    status = fft_complex_bitreverse_order(data, stride, n, logn);
    dual = 1 as libc::c_int as size_t;
    bit = 0 as libc::c_int as size_t;
    while bit < logn {
        let mut w_real: libc::c_double = 1.0f64;
        let mut w_imag: libc::c_double = 0.0f64;
        let theta: libc::c_double = 2.0f64 * sign as libc::c_int as libc::c_double
            * 3.14159265358979323846f64 / (2.0f64 * dual as libc::c_double);
        let s: libc::c_double = sin(theta);
        let t: libc::c_double = sin(theta / 2.0f64);
        let s2: libc::c_double = 2.0f64 * t * t;
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        b = 0 as libc::c_int as size_t;
        while b < n {
            let i: size_t = b;
            let j: size_t = b.wrapping_add(dual);
            let z1_real: libc::c_double = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                );
            let z1_imag: libc::c_double = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let wd_real: libc::c_double = z1_real;
            let wd_imag: libc::c_double = z1_imag;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) - wd_real;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) - wd_imag;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) += wd_real;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) += wd_imag;
            b = (b as libc::c_ulong)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(dual))
                as size_t as size_t;
        }
        a = 1 as libc::c_int as size_t;
        while a < dual {
            let tmp_real: libc::c_double = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_double = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = 0 as libc::c_int as size_t;
            while b < n {
                let i_0: size_t = b.wrapping_add(a);
                let j_0: size_t = b.wrapping_add(a).wrapping_add(dual);
                let z1_real_0: libc::c_double = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0) as isize,
                    );
                let z1_imag_0: libc::c_double = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let wd_real_0: libc::c_double = w_real * z1_real_0 - w_imag * z1_imag_0;
                let wd_imag_0: libc::c_double = w_real * z1_imag_0 + w_imag * z1_real_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0) as isize,
                    ) = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0) as isize,
                    ) - wd_real_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) - wd_imag_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0) as isize,
                    ) += wd_real_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) += wd_imag_0;
                b = (b as libc::c_ulong)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(dual))
                    as size_t as size_t;
            }
            a = a.wrapping_add(1);
            a;
        }
        dual = (dual as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        bit = bit.wrapping_add(1);
        bit;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_transform(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut dual: size_t = 0;
    let mut bit: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./c_radix2.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    status = fft_complex_float_bitreverse_order(data, stride, n, logn);
    dual = 1 as libc::c_int as size_t;
    bit = 0 as libc::c_int as size_t;
    while bit < logn {
        let mut w_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut w_imag: libc::c_float = 0.0f64 as libc::c_float;
        let theta: libc::c_double = 2.0f64 * sign as libc::c_int as libc::c_double
            * 3.14159265358979323846f64 / (2.0f64 * dual as libc::c_double);
        let s: libc::c_float = sin(theta) as libc::c_float;
        let t: libc::c_float = sin(theta / 2.0f64) as libc::c_float;
        let s2: libc::c_float = (2.0f64 * t as libc::c_double * t as libc::c_double)
            as libc::c_float;
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        b = 0 as libc::c_int as size_t;
        while b < n {
            let i: size_t = b;
            let j: size_t = b.wrapping_add(dual);
            let z1_real: libc::c_float = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                );
            let z1_imag: libc::c_float = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let wd_real: libc::c_float = z1_real;
            let wd_imag: libc::c_float = z1_imag;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) - wd_real;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) - wd_imag;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i) as isize,
                ) += wd_real;
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) += wd_imag;
            b = (b as libc::c_ulong)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(dual))
                as size_t as size_t;
        }
        a = 1 as libc::c_int as size_t;
        while a < dual {
            let tmp_real: libc::c_float = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_float = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = 0 as libc::c_int as size_t;
            while b < n {
                let i_0: size_t = b.wrapping_add(a);
                let j_0: size_t = b.wrapping_add(a).wrapping_add(dual);
                let z1_real_0: libc::c_float = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0) as isize,
                    );
                let z1_imag_0: libc::c_float = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                let wd_real_0: libc::c_float = w_real * z1_real_0 - w_imag * z1_imag_0;
                let wd_imag_0: libc::c_float = w_real * z1_imag_0 + w_imag * z1_real_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0) as isize,
                    ) = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0) as isize,
                    ) - wd_real_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) - wd_imag_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0) as isize,
                    ) += wd_real_0;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) += wd_imag_0;
                b = (b as libc::c_ulong)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(dual))
                    as size_t as size_t;
            }
            a = a.wrapping_add(1);
            a;
        }
        dual = (dual as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        bit = bit.wrapping_add(1);
        bit;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_dif_forward(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_fft_complex_radix2_dif_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_dif_forward(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_fft_complex_float_radix2_dif_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_dif_backward(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_radix2_dif_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_dif_backward(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_float_radix2_dif_transform(
        data,
        stride,
        n,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_dif_inverse(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_radix2_dif_transform(
        data,
        stride,
        n,
        sign,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_dif_inverse(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_fft_complex_float_radix2_dif_transform(
        data,
        stride,
        n,
        sign,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_float = (1.0f64 / n as libc::c_double) as libc::c_float;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *data
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_complex_radix2_dif_transform(
    mut data: gsl_complex_packed_array,
    stride: size_t,
    n: size_t,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut dual: size_t = 0;
    let mut bit: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./c_radix2.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    dual = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    bit = 0 as libc::c_int as size_t;
    while bit < logn {
        let mut w_real: libc::c_double = 1.0f64;
        let mut w_imag: libc::c_double = 0.0f64;
        let theta: libc::c_double = 2.0f64 * sign as libc::c_int as libc::c_double
            * 3.14159265358979323846f64
            / (2 as libc::c_int as libc::c_ulong).wrapping_mul(dual) as libc::c_double;
        let s: libc::c_double = sin(theta);
        let t: libc::c_double = sin(theta / 2.0f64);
        let s2: libc::c_double = 2.0f64 * t * t;
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        b = 0 as libc::c_int as size_t;
        while b < dual {
            a = 0 as libc::c_int as size_t;
            while a < n {
                let i: size_t = b.wrapping_add(a);
                let j: size_t = b.wrapping_add(a).wrapping_add(dual);
                let t1_real: libc::c_double = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i) as isize,
                    )
                    + *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j) as isize,
                        );
                let t1_imag: libc::c_double = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                    + *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                let t2_real: libc::c_double = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i) as isize,
                    )
                    - *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j) as isize,
                        );
                let t2_imag: libc::c_double = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                    - *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i) as isize,
                    ) = t1_real;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = t1_imag;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j) as isize,
                    ) = w_real * t2_real - w_imag * t2_imag;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = w_real * t2_imag + w_imag * t2_real;
                a = (a as libc::c_ulong)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(dual))
                    as size_t as size_t;
            }
            let tmp_real: libc::c_double = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_double = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = b.wrapping_add(1);
            b;
        }
        dual = (dual as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        bit = bit.wrapping_add(1);
        bit;
    }
    status = fft_complex_bitreverse_order(data, stride, n, logn);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_complex_float_radix2_dif_transform(
    mut data: gsl_complex_packed_array_float,
    stride: size_t,
    n: size_t,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut dual: size_t = 0;
    let mut bit: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./c_radix2.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    dual = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    bit = 0 as libc::c_int as size_t;
    while bit < logn {
        let mut w_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut w_imag: libc::c_float = 0.0f64 as libc::c_float;
        let theta: libc::c_double = 2.0f64 * sign as libc::c_int as libc::c_double
            * 3.14159265358979323846f64
            / (2 as libc::c_int as libc::c_ulong).wrapping_mul(dual) as libc::c_double;
        let s: libc::c_float = sin(theta) as libc::c_float;
        let t: libc::c_float = sin(theta / 2.0f64) as libc::c_float;
        let s2: libc::c_float = (2.0f64 * t as libc::c_double * t as libc::c_double)
            as libc::c_float;
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        b = 0 as libc::c_int as size_t;
        while b < dual {
            a = 0 as libc::c_int as size_t;
            while a < n {
                let i: size_t = b.wrapping_add(a);
                let j: size_t = b.wrapping_add(a).wrapping_add(dual);
                let t1_real: libc::c_float = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i) as isize,
                    )
                    + *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j) as isize,
                        );
                let t1_imag: libc::c_float = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                    + *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                let t2_real: libc::c_float = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i) as isize,
                    )
                    - *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j) as isize,
                        );
                let t2_imag: libc::c_float = *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                    - *data
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(stride)
                                .wrapping_mul(j)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i) as isize,
                    ) = t1_real;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = t1_imag;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j) as isize,
                    ) = w_real * t2_real - w_imag * t2_imag;
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride)
                            .wrapping_mul(j)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = w_real * t2_imag + w_imag * t2_real;
                a = (a as libc::c_ulong)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(dual))
                    as size_t as size_t;
            }
            let tmp_real: libc::c_float = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_float = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = b.wrapping_add(1);
            b;
        }
        dual = (dual as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        bit = bit.wrapping_add(1);
        bit;
    }
    status = fft_complex_float_bitreverse_order(data, stride, n, logn);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_wavetable_float_alloc(
    mut n: size_t,
) -> *mut gsl_fft_halfcomplex_wavetable_float {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_factors: size_t = 0;
    let mut t: size_t = 0;
    let mut product: size_t = 0;
    let mut product_1: size_t = 0;
    let mut q: size_t = 0;
    let mut d_theta: libc::c_double = 0.;
    let mut wavetable: *mut gsl_fft_halfcomplex_wavetable_float = 0
        as *mut gsl_fft_halfcomplex_wavetable_float;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable_float;
    }
    wavetable = malloc(
        ::std::mem::size_of::<gsl_fft_halfcomplex_wavetable_float>() as libc::c_ulong,
    ) as *mut gsl_fft_halfcomplex_wavetable_float;
    if wavetable.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable_float;
    }
    (*wavetable)
        .trig = malloc(
        n.wrapping_mul(::std::mem::size_of::<gsl_complex_float>() as libc::c_ulong),
    ) as *mut gsl_complex_float;
    if ((*wavetable).trig).is_null() {
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"failed to allocate trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable_float;
    }
    (*wavetable).n = n;
    status = fft_halfcomplex_factorize(
        n,
        &mut n_factors,
        ((*wavetable).factor).as_mut_ptr(),
    );
    if status != 0 {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            GSL_EFACTOR as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable_float;
    }
    (*wavetable).nf = n_factors;
    d_theta = 2.0f64 * 3.14159265358979323846f64 / n as libc::c_double;
    t = 0 as libc::c_int as size_t;
    product = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n_factors {
        let mut j: size_t = 0;
        let factor: size_t = (*wavetable).factor[i as usize];
        (*wavetable).twiddle[i as usize] = ((*wavetable).trig).offset(t as isize);
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        j = 1 as libc::c_int as size_t;
        while j < factor {
            let mut k: size_t = 0;
            let mut m: size_t = 0 as libc::c_int as size_t;
            k = 1 as libc::c_int as size_t;
            while k
                < q
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                let mut theta: libc::c_double = 0.;
                m = m.wrapping_add(j.wrapping_mul(product_1));
                m = m.wrapping_rem(n);
                theta = d_theta * m as libc::c_double;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[0 as libc::c_int as usize] = cos(theta) as libc::c_float;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[1 as libc::c_int as usize] = sin(theta) as libc::c_float;
                t = t.wrapping_add(1);
                t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if t > n.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"overflowed trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable_float;
    }
    return wavetable;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_wavetable_alloc(
    mut n: size_t,
) -> *mut gsl_fft_halfcomplex_wavetable {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_factors: size_t = 0;
    let mut t: size_t = 0;
    let mut product: size_t = 0;
    let mut product_1: size_t = 0;
    let mut q: size_t = 0;
    let mut d_theta: libc::c_double = 0.;
    let mut wavetable: *mut gsl_fft_halfcomplex_wavetable = 0
        as *mut gsl_fft_halfcomplex_wavetable;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable;
    }
    wavetable = malloc(
        ::std::mem::size_of::<gsl_fft_halfcomplex_wavetable>() as libc::c_ulong,
    ) as *mut gsl_fft_halfcomplex_wavetable;
    if wavetable.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable;
    }
    (*wavetable)
        .trig = malloc(
        n.wrapping_mul(::std::mem::size_of::<gsl_complex>() as libc::c_ulong),
    ) as *mut gsl_complex;
    if ((*wavetable).trig).is_null() {
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"failed to allocate trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable;
    }
    (*wavetable).n = n;
    status = fft_halfcomplex_factorize(
        n,
        &mut n_factors,
        ((*wavetable).factor).as_mut_ptr(),
    );
    if status != 0 {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            GSL_EFACTOR as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable;
    }
    (*wavetable).nf = n_factors;
    d_theta = 2.0f64 * 3.14159265358979323846f64 / n as libc::c_double;
    t = 0 as libc::c_int as size_t;
    product = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n_factors {
        let mut j: size_t = 0;
        let factor: size_t = (*wavetable).factor[i as usize];
        (*wavetable).twiddle[i as usize] = ((*wavetable).trig).offset(t as isize);
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        j = 1 as libc::c_int as size_t;
        while j < factor {
            let mut k: size_t = 0;
            let mut m: size_t = 0 as libc::c_int as size_t;
            k = 1 as libc::c_int as size_t;
            while k
                < q
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                let mut theta: libc::c_double = 0.;
                m = m.wrapping_add(j.wrapping_mul(product_1));
                m = m.wrapping_rem(n);
                theta = d_theta * m as libc::c_double;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[0 as libc::c_int as usize] = cos(theta);
                (*((*wavetable).trig).offset(t as isize))
                    .dat[1 as libc::c_int as usize] = sin(theta);
                t = t.wrapping_add(1);
                t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if t > n.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"overflowed trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./hc_init.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_fft_halfcomplex_wavetable;
    }
    return wavetable;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_wavetable_float_free(
    mut wavetable: *mut gsl_fft_halfcomplex_wavetable_float,
) {
    if wavetable.is_null() {
        return;
    }
    free((*wavetable).trig as *mut libc::c_void);
    (*wavetable).trig = 0 as *mut gsl_complex_float;
    free(wavetable as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_wavetable_free(
    mut wavetable: *mut gsl_fft_halfcomplex_wavetable,
) {
    if wavetable.is_null() {
        return;
    }
    free((*wavetable).trig as *mut libc::c_void);
    (*wavetable).trig = 0 as *mut gsl_complex;
    free(wavetable as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_inverse(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_halfcomplex_wavetable,
    mut work: *mut gsl_fft_real_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_transform(
        data,
        stride,
        n,
        wavetable,
        work,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data.offset(stride.wrapping_mul(i) as isize) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_backward(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_halfcomplex_wavetable_float,
    mut work: *mut gsl_fft_real_workspace_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_float_transform(
        data,
        stride,
        n,
        wavetable,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_backward(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_halfcomplex_wavetable,
    mut work: *mut gsl_fft_real_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_transform(
        data,
        stride,
        n,
        wavetable,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_inverse(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_halfcomplex_wavetable_float,
    mut work: *mut gsl_fft_real_workspace_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_float_transform(
        data,
        stride,
        n,
        wavetable,
        work,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let ref mut fresh0 = *data.offset(stride.wrapping_mul(i) as isize);
        *fresh0 = (*fresh0 as libc::c_double * norm) as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_transform(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_halfcomplex_wavetable,
    mut work: *mut gsl_fft_real_workspace,
) -> libc::c_int {
    let scratch: *mut libc::c_double = (*work).scratch;
    let mut in_0: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut out: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut istride: size_t = 0;
    let mut ostride: size_t = 0;
    let mut factor: size_t = 0;
    let mut product: size_t = 0;
    let mut q: size_t = 0;
    let mut i: size_t = 0;
    let mut nf: size_t = 0;
    let mut state: libc::c_int = 0;
    let mut product_1: libc::c_int = 0;
    let mut tskip: libc::c_int = 0;
    let mut twiddle1: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle2: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle3: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle4: *mut gsl_complex = 0 as *mut gsl_complex;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_main.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n != (*wavetable).n {
        gsl_error(
            b"wavetable does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./hc_main.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n != (*work).n {
        gsl_error(
            b"workspace does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./hc_main.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    nf = (*wavetable).nf;
    product = 1 as libc::c_int as size_t;
    state = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < nf {
        factor = (*wavetable).factor[i as usize];
        product_1 = product as libc::c_int;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        tskip = q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if state == 0 as libc::c_int {
            in_0 = data;
            istride = stride;
            out = scratch;
            ostride = 1 as libc::c_int as size_t;
            state = 1 as libc::c_int;
        } else {
            in_0 = scratch;
            istride = 1 as libc::c_int as size_t;
            out = data;
            ostride = stride;
            state = 0 as libc::c_int;
        }
        if factor == 2 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_halfcomplex_pass_2(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
            );
        } else if factor == 3 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            fft_halfcomplex_pass_3(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
            );
        } else if factor == 4 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            fft_halfcomplex_pass_4(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
            );
        } else if factor == 5 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            twiddle4 = twiddle3.offset(tskip as isize);
            fft_halfcomplex_pass_5(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
                twiddle4 as *const gsl_complex,
            );
        } else {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_halfcomplex_pass_n(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                factor,
                product,
                n,
                twiddle1 as *const gsl_complex,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if state == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *data.offset(stride.wrapping_mul(i) as isize) = *scratch.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_transform(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_halfcomplex_wavetable_float,
    mut work: *mut gsl_fft_real_workspace_float,
) -> libc::c_int {
    let scratch: *mut libc::c_float = (*work).scratch;
    let mut in_0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut out: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut istride: size_t = 0;
    let mut ostride: size_t = 0;
    let mut factor: size_t = 0;
    let mut product: size_t = 0;
    let mut q: size_t = 0;
    let mut i: size_t = 0;
    let mut nf: size_t = 0;
    let mut state: libc::c_int = 0;
    let mut product_1: libc::c_int = 0;
    let mut tskip: libc::c_int = 0;
    let mut twiddle1: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle2: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle3: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle4: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_main.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n != (*wavetable).n {
        gsl_error(
            b"wavetable does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./hc_main.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n != (*work).n {
        gsl_error(
            b"workspace does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./hc_main.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    nf = (*wavetable).nf;
    product = 1 as libc::c_int as size_t;
    state = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < nf {
        factor = (*wavetable).factor[i as usize];
        product_1 = product as libc::c_int;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        tskip = q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if state == 0 as libc::c_int {
            in_0 = data;
            istride = stride;
            out = scratch;
            ostride = 1 as libc::c_int as size_t;
            state = 1 as libc::c_int;
        } else {
            in_0 = scratch;
            istride = 1 as libc::c_int as size_t;
            out = data;
            ostride = stride;
            state = 0 as libc::c_int;
        }
        if factor == 2 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_halfcomplex_float_pass_2(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
            );
        } else if factor == 3 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            fft_halfcomplex_float_pass_3(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
            );
        } else if factor == 4 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            fft_halfcomplex_float_pass_4(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
            );
        } else if factor == 5 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            twiddle4 = twiddle3.offset(tskip as isize);
            fft_halfcomplex_float_pass_5(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
                twiddle4 as *const gsl_complex_float,
            );
        } else {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_halfcomplex_float_pass_n(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                factor,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if state == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *data.offset(stride.wrapping_mul(i) as isize) = *scratch.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_halfcomplex_float_pass_2(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 2 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let r0: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(k1)
                            .wrapping_mul(q),
                    ) as isize,
            );
        let r1: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(k1)
                            .wrapping_mul(q)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(q),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let s0: libc::c_float = r0 + r1;
        let s1: libc::c_float = r0 - r1;
        *out.offset(ostride.wrapping_mul(q.wrapping_mul(k1)) as isize) = s0;
        *out
            .offset(
                ostride.wrapping_mul(q.wrapping_mul(k1).wrapping_add(m)) as isize,
            ) = s1;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w_real: libc::c_float = (*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w_imag: libc::c_float = (*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let z0_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0) as isize);
            let z0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1) as isize);
            let z1_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let x0_real: libc::c_float = z0_real + z1_real;
            let x0_imag: libc::c_float = z0_imag - z1_imag;
            let x1_real: libc::c_float = z0_real - z1_real;
            let x1_imag: libc::c_float = z0_imag + z1_imag;
            let to0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1: size_t = to0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1) as isize,
                ) = w_real * x1_real - w_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w_imag * x1_real + w_real * x1_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_0: size_t = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to0_0: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_0: size_t = to0_0.wrapping_add(m);
        *out
            .offset(
                ostride.wrapping_mul(to0_0) as isize,
            ) = 2 as libc::c_int as libc::c_float
            * *in_0.offset(istride.wrapping_mul(from0_0) as isize);
        *out
            .offset(
                ostride.wrapping_mul(to1_0) as isize,
            ) = -(2 as libc::c_int) as libc::c_float
            * *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_pass_2(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 2 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let r0: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(k1)
                            .wrapping_mul(q),
                    ) as isize,
            );
        let r1: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(k1)
                            .wrapping_mul(q)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(q),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let s0: libc::c_double = r0 + r1;
        let s1: libc::c_double = r0 - r1;
        *out.offset(ostride.wrapping_mul(q.wrapping_mul(k1)) as isize) = s0;
        *out
            .offset(
                ostride.wrapping_mul(q.wrapping_mul(k1).wrapping_add(m)) as isize,
            ) = s1;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w_real: libc::c_double = (*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w_imag: libc::c_double = (*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let z0_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0) as isize);
            let z0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1) as isize);
            let z1_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let x0_real: libc::c_double = z0_real + z1_real;
            let x0_imag: libc::c_double = z0_imag - z1_imag;
            let x1_real: libc::c_double = z0_real - z1_real;
            let x1_imag: libc::c_double = z0_imag + z1_imag;
            let to0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1: size_t = to0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1) as isize,
                ) = w_real * x1_real - w_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w_imag * x1_real + w_real * x1_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_0: size_t = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to0_0: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_0: size_t = to0_0.wrapping_add(m);
        *out
            .offset(
                ostride.wrapping_mul(to0_0) as isize,
            ) = 2 as libc::c_int as libc::c_double
            * *in_0.offset(istride.wrapping_mul(from0_0) as isize);
        *out
            .offset(
                ostride.wrapping_mul(to1_0) as isize,
            ) = -(2 as libc::c_int) as libc::c_double
            * *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_float_pass_3(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    let mut tau: libc::c_float = (sqrt(3.0f64) / 2.0f64) as libc::c_float;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 3 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0: size_t = (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q);
        let from1: size_t = from0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let z0_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z1_imag: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(from1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let t1_real: libc::c_float = 2 as libc::c_int as libc::c_float * z1_real;
        let t2_real: libc::c_float = z0_real - z1_real;
        let t3_imag: libc::c_float = 2 as libc::c_int as libc::c_float * tau * z1_imag;
        let to0: size_t = q.wrapping_mul(k1);
        let to1: size_t = to0.wrapping_add(m);
        let to2: size_t = to1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0) as isize) = z0_real + t1_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = t2_real - t3_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = t2_real + t3_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0_0: size_t = (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from2: size_t = (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let z0_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let z0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let z1_imag_0: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z2_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from2) as isize);
            let z2_imag: libc::c_float = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let t1_real_0: libc::c_float = z1_real_0 + z2_real;
            let t1_imag: libc::c_float = z1_imag_0 + z2_imag;
            let t2_real_0: libc::c_float = (z0_real_0 as libc::c_double
                - t1_real_0 as libc::c_double / 2.0f64) as libc::c_float;
            let t2_imag: libc::c_float = (z0_imag as libc::c_double
                - t1_imag as libc::c_double / 2.0f64) as libc::c_float;
            let t3_real: libc::c_float = tau * (z1_real_0 - z2_real);
            let t3_imag_0: libc::c_float = tau * (z1_imag_0 - z2_imag);
            let x0_real: libc::c_float = z0_real_0 + t1_real_0;
            let x0_imag: libc::c_float = z0_imag + t1_imag;
            let x1_real: libc::c_float = t2_real_0 - t3_imag_0;
            let x1_imag: libc::c_float = t2_imag + t3_real;
            let x2_real: libc::c_float = t2_real_0 + t3_imag_0;
            let x2_imag: libc::c_float = t2_imag - t3_real;
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0.wrapping_add(m);
            let to2_0: size_t = to1_0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1_0) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w1_imag * x1_real + w1_real * x1_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to2_0) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w2_imag * x2_real + w2_real * x2_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_1: size_t = (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z0_imag_0: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        from0_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let z1_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let t1_real_1: libc::c_float = z0_real_1 - z1_real_1;
        let t2_real_1: libc::c_float = 2 as libc::c_int as libc::c_float * tau
            * z0_imag_0;
        let x0_real_0: libc::c_float = 2 as libc::c_int as libc::c_float * z0_real_1
            + z1_real_1;
        let x1_real_0: libc::c_float = t1_real_1 - t2_real_1;
        let x2_real_0: libc::c_float = -t1_real_1 - t2_real_1;
        let to0_1: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1.wrapping_add(m);
        let to2_1: size_t = to1_1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_0;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_0;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = x2_real_0;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_pass_3(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    let mut tau: libc::c_double = sqrt(3.0f64) / 2.0f64;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 3 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0: size_t = (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q);
        let from1: size_t = from0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let z0_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z1_imag: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(from1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let t1_real: libc::c_double = 2 as libc::c_int as libc::c_double * z1_real;
        let t2_real: libc::c_double = z0_real - z1_real;
        let t3_imag: libc::c_double = 2 as libc::c_int as libc::c_double * tau * z1_imag;
        let to0: size_t = q.wrapping_mul(k1);
        let to1: size_t = to0.wrapping_add(m);
        let to2: size_t = to1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0) as isize) = z0_real + t1_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = t2_real - t3_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = t2_real + t3_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0_0: size_t = (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from2: size_t = (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let z0_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let z0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let z1_imag_0: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z2_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from2) as isize);
            let z2_imag: libc::c_double = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let t1_real_0: libc::c_double = z1_real_0 + z2_real;
            let t1_imag: libc::c_double = z1_imag_0 + z2_imag;
            let t2_real_0: libc::c_double = z0_real_0 - t1_real_0 / 2.0f64;
            let t2_imag: libc::c_double = z0_imag - t1_imag / 2.0f64;
            let t3_real: libc::c_double = tau * (z1_real_0 - z2_real);
            let t3_imag_0: libc::c_double = tau * (z1_imag_0 - z2_imag);
            let x0_real: libc::c_double = z0_real_0 + t1_real_0;
            let x0_imag: libc::c_double = z0_imag + t1_imag;
            let x1_real: libc::c_double = t2_real_0 - t3_imag_0;
            let x1_imag: libc::c_double = t2_imag + t3_real;
            let x2_real: libc::c_double = t2_real_0 + t3_imag_0;
            let x2_imag: libc::c_double = t2_imag - t3_real;
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0.wrapping_add(m);
            let to2_0: size_t = to1_0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1_0) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w1_imag * x1_real + w1_real * x1_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to2_0) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w2_imag * x2_real + w2_real * x2_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_1: size_t = (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z0_imag_0: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        from0_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let z1_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let t1_real_1: libc::c_double = z0_real_1 - z1_real_1;
        let t2_real_1: libc::c_double = 2 as libc::c_int as libc::c_double * tau
            * z0_imag_0;
        let x0_real_0: libc::c_double = 2 as libc::c_int as libc::c_double * z0_real_1
            + z1_real_1;
        let x1_real_0: libc::c_double = t1_real_1 - t2_real_1;
        let x2_real_0: libc::c_double = -t1_real_1 - t2_real_1;
        let to0_1: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1.wrapping_add(m);
        let to2_1: size_t = to1_1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_0;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_0;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = x2_real_0;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_float_pass_4(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 4 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0: size_t = (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q);
        let from1: size_t = from0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from2: size_t = from1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z1_imag: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(from1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let z2_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let t1_real: libc::c_float = z0_real + z2_real;
        let t2_real: libc::c_float = 2 as libc::c_int as libc::c_float * z1_real;
        let t3_real: libc::c_float = z0_real - z2_real;
        let t4_imag: libc::c_float = 2 as libc::c_int as libc::c_float * z1_imag;
        let to0: size_t = q.wrapping_mul(k1);
        let to1: size_t = to0.wrapping_add(m);
        let to2: size_t = to1.wrapping_add(m);
        let to3: size_t = to2.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0) as isize) = t1_real + t2_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = t3_real - t4_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = t1_real - t2_real;
        *out.offset(ostride.wrapping_mul(to3) as isize) = t3_real + t4_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w3_real: libc::c_float = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w3_imag: libc::c_float = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0_0: size_t = (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from2_0: size_t = (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from3: size_t = from2_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let z0_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let z0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let z1_imag_0: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z2_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from3) as isize);
            let z2_imag: libc::c_float = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z3_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let z3_imag: libc::c_float = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let t1_real_0: libc::c_float = z0_real_0 + z2_real_0;
            let t1_imag: libc::c_float = z0_imag + z2_imag;
            let t2_real_0: libc::c_float = z1_real_0 + z3_real;
            let t2_imag: libc::c_float = z1_imag_0 + z3_imag;
            let t3_real_0: libc::c_float = z0_real_0 - z2_real_0;
            let t3_imag: libc::c_float = z0_imag - z2_imag;
            let t4_real: libc::c_float = z1_real_0 - z3_real;
            let t4_imag_0: libc::c_float = z1_imag_0 - z3_imag;
            let x0_real: libc::c_float = t1_real_0 + t2_real_0;
            let x0_imag: libc::c_float = t1_imag + t2_imag;
            let x1_real: libc::c_float = t3_real_0 - t4_imag_0;
            let x1_imag: libc::c_float = t3_imag + t4_real;
            let x2_real: libc::c_float = t1_real_0 - t2_real_0;
            let x2_imag: libc::c_float = t1_imag - t2_imag;
            let x3_real: libc::c_float = t3_real_0 + t4_imag_0;
            let x3_imag: libc::c_float = t3_imag - t4_real;
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0.wrapping_add(m);
            let to2_0: size_t = to1_0.wrapping_add(m);
            let to3_0: size_t = to2_0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1_0) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w1_imag * x1_real + w1_real * x1_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to2_0) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w2_imag * x2_real + w2_real * x2_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to3_0) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_1: size_t = (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z0_imag_0: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        from0_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let z1_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let z1_imag_1: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        from1_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let t1_real_1: libc::c_float = (sqrt(2.0f64)
            * (z0_imag_0 + z1_imag_1) as libc::c_double) as libc::c_float;
        let t2_real_1: libc::c_float = (sqrt(2.0f64)
            * (z0_real_1 - z1_real_1) as libc::c_double) as libc::c_float;
        let x0_real_0: libc::c_float = 2 as libc::c_int as libc::c_float
            * (z0_real_1 + z1_real_1);
        let x1_real_0: libc::c_float = t2_real_1 - t1_real_1;
        let x2_real_0: libc::c_float = 2 as libc::c_int as libc::c_float
            * (z1_imag_1 - z0_imag_0);
        let x3_real_0: libc::c_float = -(t2_real_1 + t1_real_1);
        let to0_1: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1.wrapping_add(m);
        let to2_1: size_t = to1_1.wrapping_add(m);
        let to3_1: size_t = to2_1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_0;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_0;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = x2_real_0;
        *out.offset(ostride.wrapping_mul(to3_1) as isize) = x3_real_0;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_pass_4(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 4 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0: size_t = (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q);
        let from1: size_t = from0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from2: size_t = from1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z1_imag: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(from1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let z2_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let t1_real: libc::c_double = z0_real + z2_real;
        let t2_real: libc::c_double = 2 as libc::c_int as libc::c_double * z1_real;
        let t3_real: libc::c_double = z0_real - z2_real;
        let t4_imag: libc::c_double = 2 as libc::c_int as libc::c_double * z1_imag;
        let to0: size_t = q.wrapping_mul(k1);
        let to1: size_t = to0.wrapping_add(m);
        let to2: size_t = to1.wrapping_add(m);
        let to3: size_t = to2.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0) as isize) = t1_real + t2_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = t3_real - t4_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = t1_real - t2_real;
        *out.offset(ostride.wrapping_mul(to3) as isize) = t3_real + t4_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w3_real: libc::c_double = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w3_imag: libc::c_double = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0_0: size_t = (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from2_0: size_t = (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from3: size_t = from2_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let z0_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let z0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let z1_imag_0: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z2_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from3) as isize);
            let z2_imag: libc::c_double = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z3_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let z3_imag: libc::c_double = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let t1_real_0: libc::c_double = z0_real_0 + z2_real_0;
            let t1_imag: libc::c_double = z0_imag + z2_imag;
            let t2_real_0: libc::c_double = z1_real_0 + z3_real;
            let t2_imag: libc::c_double = z1_imag_0 + z3_imag;
            let t3_real_0: libc::c_double = z0_real_0 - z2_real_0;
            let t3_imag: libc::c_double = z0_imag - z2_imag;
            let t4_real: libc::c_double = z1_real_0 - z3_real;
            let t4_imag_0: libc::c_double = z1_imag_0 - z3_imag;
            let x0_real: libc::c_double = t1_real_0 + t2_real_0;
            let x0_imag: libc::c_double = t1_imag + t2_imag;
            let x1_real: libc::c_double = t3_real_0 - t4_imag_0;
            let x1_imag: libc::c_double = t3_imag + t4_real;
            let x2_real: libc::c_double = t1_real_0 - t2_real_0;
            let x2_imag: libc::c_double = t1_imag - t2_imag;
            let x3_real: libc::c_double = t3_real_0 + t4_imag_0;
            let x3_imag: libc::c_double = t3_imag - t4_real;
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0.wrapping_add(m);
            let to2_0: size_t = to1_0.wrapping_add(m);
            let to3_0: size_t = to2_0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1_0) as isize,
                ) = w1_real * x1_real - w1_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w1_imag * x1_real + w1_real * x1_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to2_0) as isize,
                ) = w2_real * x2_real - w2_imag * x2_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w2_imag * x2_real + w2_real * x2_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to3_0) as isize,
                ) = w3_real * x3_real - w3_imag * x3_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_1: size_t = (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z0_imag_0: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        from0_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let z1_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let z1_imag_1: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(
                        from1_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        let t1_real_1: libc::c_double = sqrt(2.0f64) * (z0_imag_0 + z1_imag_1);
        let t2_real_1: libc::c_double = sqrt(2.0f64) * (z0_real_1 - z1_real_1);
        let x0_real_0: libc::c_double = 2 as libc::c_int as libc::c_double
            * (z0_real_1 + z1_real_1);
        let x1_real_0: libc::c_double = t2_real_1 - t1_real_1;
        let x2_real_0: libc::c_double = 2 as libc::c_int as libc::c_double
            * (z1_imag_1 - z0_imag_0);
        let x3_real_0: libc::c_double = -(t2_real_1 + t1_real_1);
        let to0_1: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1.wrapping_add(m);
        let to2_1: size_t = to1_1.wrapping_add(m);
        let to3_1: size_t = to2_1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_0;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_0;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = x2_real_0;
        *out.offset(ostride.wrapping_mul(to3_1) as isize) = x3_real_0;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_pass_5(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
    mut twiddle4: *const gsl_complex,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    let sina: libc::c_double = sin(2.0f64 * 3.14159265358979323846f64 / 5.0f64);
    let sinb: libc::c_double = sin(2.0f64 * 3.14159265358979323846f64 / 10.0f64);
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 5 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0: size_t = (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q);
        let from1: size_t = from0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from2: size_t = from1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z1_imag: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(from1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let z2_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let z2_imag: libc::c_double = *in_0
            .offset(
                istride
                    .wrapping_mul(from2.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let t1_real: libc::c_double = 2 as libc::c_int as libc::c_double
            * (z1_real + z2_real);
        let t2_real: libc::c_double = 2 as libc::c_int as libc::c_double
            * (sqrt(5.0f64) / 4.0f64) * (z1_real - z2_real);
        let t3_real: libc::c_double = z0_real - t1_real / 4.0f64;
        let t4_real: libc::c_double = t2_real + t3_real;
        let t5_real: libc::c_double = -t2_real + t3_real;
        let t6_imag: libc::c_double = 2 as libc::c_int as libc::c_double
            * (sina * z1_imag + sinb * z2_imag);
        let t7_imag: libc::c_double = 2 as libc::c_int as libc::c_double
            * (sinb * z1_imag - sina * z2_imag);
        let x0_real: libc::c_double = z0_real + t1_real;
        let x1_real: libc::c_double = t4_real - t6_imag;
        let x2_real: libc::c_double = t5_real - t7_imag;
        let x3_real: libc::c_double = t5_real + t7_imag;
        let x4_real: libc::c_double = t4_real + t6_imag;
        let to0: size_t = q.wrapping_mul(k1);
        let to1: size_t = to0.wrapping_add(m);
        let to2: size_t = to1.wrapping_add(m);
        let to3: size_t = to2.wrapping_add(m);
        let to4: size_t = to3.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
        *out.offset(ostride.wrapping_mul(to3) as isize) = x3_real;
        *out.offset(ostride.wrapping_mul(to4) as isize) = x4_real;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w3_real: libc::c_double = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w3_imag: libc::c_double = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w4_real: libc::c_double = (*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w4_imag: libc::c_double = (*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0_0: size_t = (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from2_0: size_t = from1_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from3: size_t = (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from4: size_t = from3
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let z0_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let z0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let z1_imag_0: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z2_real_0: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let z2_imag_0: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z3_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from4) as isize);
            let z3_imag: libc::c_double = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from4.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z4_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from3) as isize);
            let z4_imag: libc::c_double = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let t1_real_0: libc::c_double = z1_real_0 + z4_real;
            let t1_imag: libc::c_double = z1_imag_0 + z4_imag;
            let t2_real_0: libc::c_double = z2_real_0 + z3_real;
            let t2_imag: libc::c_double = z2_imag_0 + z3_imag;
            let t3_real_0: libc::c_double = z1_real_0 - z4_real;
            let t3_imag: libc::c_double = z1_imag_0 - z4_imag;
            let t4_real_0: libc::c_double = z2_real_0 - z3_real;
            let t4_imag: libc::c_double = z2_imag_0 - z3_imag;
            let t5_real_0: libc::c_double = t1_real_0 + t2_real_0;
            let t5_imag: libc::c_double = t1_imag + t2_imag;
            let t6_real: libc::c_double = sqrt(5.0f64) / 4.0f64
                * (t1_real_0 - t2_real_0);
            let t6_imag_0: libc::c_double = sqrt(5.0f64) / 4.0f64 * (t1_imag - t2_imag);
            let t7_real: libc::c_double = z0_real_0 - t5_real_0 / 4.0f64;
            let t7_imag_0: libc::c_double = z0_imag - t5_imag / 4.0f64;
            let t8_real: libc::c_double = t7_real + t6_real;
            let t8_imag: libc::c_double = t7_imag_0 + t6_imag_0;
            let t9_real: libc::c_double = t7_real - t6_real;
            let t9_imag: libc::c_double = t7_imag_0 - t6_imag_0;
            let t10_real: libc::c_double = sina * t3_real_0 + sinb * t4_real_0;
            let t10_imag: libc::c_double = sina * t3_imag + sinb * t4_imag;
            let t11_real: libc::c_double = sinb * t3_real_0 - sina * t4_real_0;
            let t11_imag: libc::c_double = sinb * t3_imag - sina * t4_imag;
            let x0_real_0: libc::c_double = z0_real_0 + t5_real_0;
            let x0_imag: libc::c_double = z0_imag + t5_imag;
            let x1_real_0: libc::c_double = t8_real - t10_imag;
            let x1_imag: libc::c_double = t8_imag + t10_real;
            let x2_real_0: libc::c_double = t9_real - t11_imag;
            let x2_imag: libc::c_double = t9_imag + t11_real;
            let x3_real_0: libc::c_double = t9_real + t11_imag;
            let x3_imag: libc::c_double = t9_imag - t11_real;
            let x4_real_0: libc::c_double = t8_real + t10_imag;
            let x4_imag: libc::c_double = t8_imag - t10_real;
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0.wrapping_add(m);
            let to2_0: size_t = to1_0.wrapping_add(m);
            let to3_0: size_t = to2_0.wrapping_add(m);
            let to4_0: size_t = to3_0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1_0) as isize,
                ) = w1_real * x1_real_0 - w1_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real_0;
            *out
                .offset(
                    ostride.wrapping_mul(to2_0) as isize,
                ) = w2_real * x2_real_0 - w2_imag * x2_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real_0;
            *out
                .offset(
                    ostride.wrapping_mul(to3_0) as isize,
                ) = w3_real * x3_real_0 - w3_imag * x3_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real_0;
            *out
                .offset(
                    ostride.wrapping_mul(to4_0) as isize,
                ) = w4_real * x4_real_0 - w4_imag * x4_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to4_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real_0;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_1: size_t = (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let from2_1: size_t = from1_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real_1: libc::c_double = 2 as libc::c_int as libc::c_double
            * *in_0.offset(istride.wrapping_mul(from0_1) as isize);
        let z0_imag_0: libc::c_double = 2 as libc::c_int as libc::c_double
            * *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
        let z1_real_1: libc::c_double = 2 as libc::c_int as libc::c_double
            * *in_0.offset(istride.wrapping_mul(from1_1) as isize);
        let z1_imag_1: libc::c_double = 2 as libc::c_int as libc::c_double
            * *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
        let z2_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from2_1) as isize);
        let t1_real_1: libc::c_double = z0_real_1 + z1_real_1;
        let t2_real_1: libc::c_double = t1_real_1 / 4.0f64 - z2_real_1;
        let t3_real_1: libc::c_double = sqrt(5.0f64) / 4.0f64 * (z0_real_1 - z1_real_1);
        let t4_real_1: libc::c_double = sinb * z0_imag_0 + sina * z1_imag_1;
        let t5_real_1: libc::c_double = sina * z0_imag_0 - sinb * z1_imag_1;
        let t6_real_0: libc::c_double = t3_real_1 + t2_real_1;
        let t7_real_0: libc::c_double = t3_real_1 - t2_real_1;
        let x0_real_1: libc::c_double = t1_real_1 + z2_real_1;
        let x1_real_1: libc::c_double = t6_real_0 - t4_real_1;
        let x2_real_1: libc::c_double = t7_real_0 - t5_real_1;
        let x3_real_1: libc::c_double = -t7_real_0 - t5_real_1;
        let x4_real_1: libc::c_double = -t6_real_0 - t4_real_1;
        let to0_1: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1.wrapping_add(m);
        let to2_1: size_t = to1_1.wrapping_add(m);
        let to3_1: size_t = to2_1.wrapping_add(m);
        let to4_1: size_t = to3_1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_1;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_1;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = x2_real_1;
        *out.offset(ostride.wrapping_mul(to3_1) as isize) = x3_real_1;
        *out.offset(ostride.wrapping_mul(to4_1) as isize) = x4_real_1;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_float_pass_5(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
    mut twiddle4: *const gsl_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let mut jump: size_t = 0;
    let mut factor: size_t = 0;
    let mut q: size_t = 0;
    let mut m: size_t = 0;
    let mut product_1: size_t = 0;
    let sina: libc::c_float = sin(2.0f64 * 3.14159265358979323846f64 / 5.0f64)
        as libc::c_float;
    let sinb: libc::c_float = sin(2.0f64 * 3.14159265358979323846f64 / 10.0f64)
        as libc::c_float;
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    factor = 5 as libc::c_int as size_t;
    m = n.wrapping_div(factor);
    q = n.wrapping_div(product);
    product_1 = product.wrapping_div(factor);
    jump = factor.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(q);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0: size_t = (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q);
        let from1: size_t = from0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from2: size_t = from1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z1_imag: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(from1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let z2_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let z2_imag: libc::c_float = *in_0
            .offset(
                istride
                    .wrapping_mul(from2.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        let t1_real: libc::c_float = 2 as libc::c_int as libc::c_float
            * (z1_real + z2_real);
        let t2_real: libc::c_float = (2 as libc::c_int as libc::c_double
            * (sqrt(5.0f64) / 4.0f64) * (z1_real - z2_real) as libc::c_double)
            as libc::c_float;
        let t3_real: libc::c_float = (z0_real as libc::c_double
            - t1_real as libc::c_double / 4.0f64) as libc::c_float;
        let t4_real: libc::c_float = t2_real + t3_real;
        let t5_real: libc::c_float = -t2_real + t3_real;
        let t6_imag: libc::c_float = 2 as libc::c_int as libc::c_float
            * (sina * z1_imag + sinb * z2_imag);
        let t7_imag: libc::c_float = 2 as libc::c_int as libc::c_float
            * (sinb * z1_imag - sina * z2_imag);
        let x0_real: libc::c_float = z0_real + t1_real;
        let x1_real: libc::c_float = t4_real - t6_imag;
        let x2_real: libc::c_float = t5_real - t7_imag;
        let x3_real: libc::c_float = t5_real + t7_imag;
        let x4_real: libc::c_float = t4_real + t6_imag;
        let to0: size_t = q.wrapping_mul(k1);
        let to1: size_t = to0.wrapping_add(m);
        let to2: size_t = to1.wrapping_add(m);
        let to3: size_t = to2.wrapping_add(m);
        let to4: size_t = to3.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
        *out.offset(ostride.wrapping_mul(to3) as isize) = x3_real;
        *out.offset(ostride.wrapping_mul(to4) as isize) = x4_real;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w3_real: libc::c_float = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w3_imag: libc::c_float = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w4_real: libc::c_float = (*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w4_imag: libc::c_float = (*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let from0_0: size_t = (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from2_0: size_t = from1_0
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let from3: size_t = (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(k1)
                .wrapping_mul(q)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from4: size_t = from3
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
            let z0_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let z0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z1_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let z1_imag_0: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z2_real_0: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let z2_imag_0: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z3_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from4) as isize);
            let z3_imag: libc::c_float = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from4.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z4_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from3) as isize);
            let z4_imag: libc::c_float = -*in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let t1_real_0: libc::c_float = z1_real_0 + z4_real;
            let t1_imag: libc::c_float = z1_imag_0 + z4_imag;
            let t2_real_0: libc::c_float = z2_real_0 + z3_real;
            let t2_imag: libc::c_float = z2_imag_0 + z3_imag;
            let t3_real_0: libc::c_float = z1_real_0 - z4_real;
            let t3_imag: libc::c_float = z1_imag_0 - z4_imag;
            let t4_real_0: libc::c_float = z2_real_0 - z3_real;
            let t4_imag: libc::c_float = z2_imag_0 - z3_imag;
            let t5_real_0: libc::c_float = t1_real_0 + t2_real_0;
            let t5_imag: libc::c_float = t1_imag + t2_imag;
            let t6_real: libc::c_float = (sqrt(5.0f64) / 4.0f64
                * (t1_real_0 - t2_real_0) as libc::c_double) as libc::c_float;
            let t6_imag_0: libc::c_float = (sqrt(5.0f64) / 4.0f64
                * (t1_imag - t2_imag) as libc::c_double) as libc::c_float;
            let t7_real: libc::c_float = (z0_real_0 as libc::c_double
                - t5_real_0 as libc::c_double / 4.0f64) as libc::c_float;
            let t7_imag_0: libc::c_float = (z0_imag as libc::c_double
                - t5_imag as libc::c_double / 4.0f64) as libc::c_float;
            let t8_real: libc::c_float = t7_real + t6_real;
            let t8_imag: libc::c_float = t7_imag_0 + t6_imag_0;
            let t9_real: libc::c_float = t7_real - t6_real;
            let t9_imag: libc::c_float = t7_imag_0 - t6_imag_0;
            let t10_real: libc::c_float = sina * t3_real_0 + sinb * t4_real_0;
            let t10_imag: libc::c_float = sina * t3_imag + sinb * t4_imag;
            let t11_real: libc::c_float = sinb * t3_real_0 - sina * t4_real_0;
            let t11_imag: libc::c_float = sinb * t3_imag - sina * t4_imag;
            let x0_real_0: libc::c_float = z0_real_0 + t5_real_0;
            let x0_imag: libc::c_float = z0_imag + t5_imag;
            let x1_real_0: libc::c_float = t8_real - t10_imag;
            let x1_imag: libc::c_float = t8_imag + t10_real;
            let x2_real_0: libc::c_float = t9_real - t11_imag;
            let x2_imag: libc::c_float = t9_imag + t11_real;
            let x3_real_0: libc::c_float = t9_real + t11_imag;
            let x3_imag: libc::c_float = t9_imag - t11_real;
            let x4_real_0: libc::c_float = t8_real + t10_imag;
            let x4_imag: libc::c_float = t8_imag - t10_real;
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0.wrapping_add(m);
            let to2_0: size_t = to1_0.wrapping_add(m);
            let to3_0: size_t = to2_0.wrapping_add(m);
            let to4_0: size_t = to3_0.wrapping_add(m);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out
                .offset(
                    ostride.wrapping_mul(to1_0) as isize,
                ) = w1_real * x1_real_0 - w1_imag * x1_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w1_real * x1_imag + w1_imag * x1_real_0;
            *out
                .offset(
                    ostride.wrapping_mul(to2_0) as isize,
                ) = w2_real * x2_real_0 - w2_imag * x2_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w2_real * x2_imag + w2_imag * x2_real_0;
            *out
                .offset(
                    ostride.wrapping_mul(to3_0) as isize,
                ) = w3_real * x3_real_0 - w3_imag * x3_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w3_real * x3_imag + w3_imag * x3_real_0;
            *out
                .offset(
                    ostride.wrapping_mul(to4_0) as isize,
                ) = w4_real * x4_real_0 - w4_imag * x4_imag;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to4_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = w4_real * x4_imag + w4_imag * x4_real_0;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let from0_1: size_t = (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(k1)
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let from2_1: size_t = from1_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(q));
        let z0_real_1: libc::c_float = 2 as libc::c_int as libc::c_float
            * *in_0.offset(istride.wrapping_mul(from0_1) as isize);
        let z0_imag_0: libc::c_float = 2 as libc::c_int as libc::c_float
            * *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
        let z1_real_1: libc::c_float = 2 as libc::c_int as libc::c_float
            * *in_0.offset(istride.wrapping_mul(from1_1) as isize);
        let z1_imag_1: libc::c_float = 2 as libc::c_int as libc::c_float
            * *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
        let z2_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from2_1) as isize);
        let t1_real_1: libc::c_float = z0_real_1 + z1_real_1;
        let t2_real_1: libc::c_float = (t1_real_1 as libc::c_double / 4.0f64
            - z2_real_1 as libc::c_double) as libc::c_float;
        let t3_real_1: libc::c_float = (sqrt(5.0f64) / 4.0f64
            * (z0_real_1 - z1_real_1) as libc::c_double) as libc::c_float;
        let t4_real_1: libc::c_float = sinb * z0_imag_0 + sina * z1_imag_1;
        let t5_real_1: libc::c_float = sina * z0_imag_0 - sinb * z1_imag_1;
        let t6_real_0: libc::c_float = t3_real_1 + t2_real_1;
        let t7_real_0: libc::c_float = t3_real_1 - t2_real_1;
        let x0_real_1: libc::c_float = t1_real_1 + z2_real_1;
        let x1_real_1: libc::c_float = t6_real_0 - t4_real_1;
        let x2_real_1: libc::c_float = t7_real_0 - t5_real_1;
        let x3_real_1: libc::c_float = -t7_real_0 - t5_real_1;
        let x4_real_1: libc::c_float = -t6_real_0 - t4_real_1;
        let to0_1: size_t = k1
            .wrapping_mul(q)
            .wrapping_add(q)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1.wrapping_add(m);
        let to2_1: size_t = to1_1.wrapping_add(m);
        let to3_1: size_t = to2_1.wrapping_add(m);
        let to4_1: size_t = to3_1.wrapping_add(m);
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_1;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_1;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = x2_real_1;
        *out.offset(ostride.wrapping_mul(to3_1) as isize) = x3_real_1;
        *out.offset(ostride.wrapping_mul(to4_1) as isize) = x4_real_1;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_pass_n(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    factor: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let mut e1: size_t = 0;
    let mut e2: size_t = 0;
    let d_theta: libc::c_double = 2.0f64 * 3.14159265358979323846f64
        / factor as libc::c_double;
    let cos_d_theta: libc::c_double = cos(d_theta);
    let sin_d_theta: libc::c_double = sin(d_theta);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let mut dw_real: libc::c_double = 1.0f64;
        let mut dw_imag: libc::c_double = 0.0f64;
        e1 = 0 as libc::c_int as size_t;
        while e1 < factor {
            let mut sum_real: libc::c_double = 0.0f64;
            let mut w_real: libc::c_double = 1.0f64;
            let mut w_imag: libc::c_double = 0.0f64;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut tmp_real: libc::c_double = dw_real * cos_d_theta
                    - dw_imag * sin_d_theta;
                let mut tmp_imag: libc::c_double = dw_real * sin_d_theta
                    + dw_imag * cos_d_theta;
                dw_real = tmp_real;
                dw_imag = tmp_imag;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2 <= factor.wrapping_sub(e2) {
                let mut z_real: libc::c_double = 0.;
                let mut z_imag: libc::c_double = 0.;
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_0: libc::c_double = dw_real * w_real
                        - dw_imag * w_imag;
                    let mut tmp_imag_0: libc::c_double = dw_real * w_imag
                        + dw_imag * w_real;
                    w_real = tmp_real_0;
                    w_imag = tmp_imag_0;
                }
                if e2 == 0 as libc::c_int as libc::c_ulong {
                    let mut from_idx: size_t = factor.wrapping_mul(k1).wrapping_mul(q);
                    z_real = *in_0.offset(istride.wrapping_mul(from_idx) as isize);
                    z_imag = 0.0f64;
                    sum_real += w_real * z_real - w_imag * z_imag;
                } else if e2 == factor.wrapping_sub(e2) {
                    let mut from_idx_0: size_t = factor
                        .wrapping_mul(q)
                        .wrapping_mul(k1)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real = *in_0.offset(istride.wrapping_mul(from_idx_0) as isize);
                    z_imag = 0.0f64;
                    sum_real += w_real * z_real;
                } else {
                    let mut from_idx_1: size_t = factor
                        .wrapping_mul(q)
                        .wrapping_mul(k1)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real = *in_0.offset(istride.wrapping_mul(from_idx_1) as isize);
                    z_imag = *in_0
                        .offset(
                            istride
                                .wrapping_mul(
                                    from_idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
                    sum_real
                        += 2 as libc::c_int as libc::c_double
                            * (w_real * z_real - w_imag * z_imag);
                }
                e2 = e2.wrapping_add(1);
                e2;
            }
            let to_idx: size_t = q.wrapping_mul(k1).wrapping_add(e1.wrapping_mul(m));
            *out.offset(ostride.wrapping_mul(to_idx) as isize) = sum_real;
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let mut dw_real_0: libc::c_double = 1.0f64;
            let mut dw_imag_0: libc::c_double = 0.0f64;
            e1 = 0 as libc::c_int as size_t;
            while e1 < factor {
                let mut z_real_0: libc::c_double = 0.;
                let mut z_imag_0: libc::c_double = 0.;
                let mut sum_real_0: libc::c_double = 0.0f64;
                let mut sum_imag: libc::c_double = 0.0f64;
                let mut w_real_0: libc::c_double = 1.0f64;
                let mut w_imag_0: libc::c_double = 0.0f64;
                if e1 > 0 as libc::c_int as libc::c_ulong {
                    let mut t_real: libc::c_double = dw_real_0 * cos_d_theta
                        - dw_imag_0 * sin_d_theta;
                    let mut t_imag: libc::c_double = dw_real_0 * sin_d_theta
                        + dw_imag_0 * cos_d_theta;
                    dw_real_0 = t_real;
                    dw_imag_0 = t_imag;
                }
                e2 = 0 as libc::c_int as size_t;
                while e2 < factor {
                    if e2 > 0 as libc::c_int as libc::c_ulong {
                        let mut tmp_real_1: libc::c_double = dw_real_0 * w_real_0
                            - dw_imag_0 * w_imag_0;
                        let mut tmp_imag_1: libc::c_double = dw_real_0 * w_imag_0
                            + dw_imag_0 * w_real_0;
                        w_real_0 = tmp_real_1;
                        w_imag_0 = tmp_imag_1;
                    }
                    if e2 < factor.wrapping_sub(e2) {
                        let from0: size_t = factor
                            .wrapping_mul(k1)
                            .wrapping_mul(q)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                            )
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(e2)
                                    .wrapping_mul(q),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        z_real_0 = *in_0.offset(istride.wrapping_mul(from0) as isize);
                        z_imag_0 = *in_0
                            .offset(
                                istride
                                    .wrapping_mul(
                                        from0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                    } else {
                        let from0_0: size_t = factor
                            .wrapping_mul(k1)
                            .wrapping_mul(q)
                            .wrapping_sub(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                            )
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(factor.wrapping_sub(e2))
                                    .wrapping_mul(q),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        z_real_0 = *in_0.offset(istride.wrapping_mul(from0_0) as isize);
                        z_imag_0 = -*in_0
                            .offset(
                                istride
                                    .wrapping_mul(
                                        from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                    }
                    sum_real_0 += w_real_0 * z_real_0 - w_imag_0 * z_imag_0;
                    sum_imag += w_real_0 * z_imag_0 + w_imag_0 * z_real_0;
                    e2 = e2.wrapping_add(1);
                    e2;
                }
                if k == 0 as libc::c_int as libc::c_ulong
                    || e1 == 0 as libc::c_int as libc::c_ulong
                {
                    w_real_0 = 1.0f64;
                    w_imag_0 = 0.0f64;
                } else {
                    let mut tskip: size_t = q
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    w_real_0 = (*twiddle
                        .offset(
                            k
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    tskip
                                        .wrapping_mul(
                                            e1.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ),
                                ) as isize,
                        ))
                        .dat[0 as libc::c_int as usize];
                    w_imag_0 = (*twiddle
                        .offset(
                            k
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    tskip
                                        .wrapping_mul(
                                            e1.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ),
                                ) as isize,
                        ))
                        .dat[1 as libc::c_int as usize];
                }
                let to0: size_t = k1
                    .wrapping_mul(q)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                    .wrapping_add(e1.wrapping_mul(m))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                *out
                    .offset(
                        ostride.wrapping_mul(to0) as isize,
                    ) = w_real_0 * sum_real_0 - w_imag_0 * sum_imag;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = w_real_0 * sum_imag + w_imag_0 * sum_real_0;
                e1 = e1.wrapping_add(1);
                e1;
            }
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let mut tw_arg: libc::c_double = 3.14159265358979323846f64
        / factor as libc::c_double;
    let mut cos_tw_arg: libc::c_double = cos(tw_arg);
    let mut sin_tw_arg: libc::c_double = sin(tw_arg);
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let mut dw_real_1: libc::c_double = 1.0f64;
        let mut dw_imag_1: libc::c_double = 0.0f64;
        let mut tw_real: libc::c_double = 1.0f64;
        let mut tw_imag: libc::c_double = 0.0f64;
        e1 = 0 as libc::c_int as size_t;
        while e1 < factor {
            let mut w_real_1: libc::c_double = 0.;
            let mut w_imag_1: libc::c_double = 0.;
            let mut z_real_1: libc::c_double = 0.;
            let mut z_imag_1: libc::c_double = 0.;
            let mut sum_real_1: libc::c_double = 0.0f64;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut tmp_real_2: libc::c_double = tw_real * cos_tw_arg
                    - tw_imag * sin_tw_arg;
                let mut tmp_imag_2: libc::c_double = tw_real * sin_tw_arg
                    + tw_imag * cos_tw_arg;
                tw_real = tmp_real_2;
                tw_imag = tmp_imag_2;
            }
            w_real_1 = tw_real;
            w_imag_1 = tw_imag;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut t_real_0: libc::c_double = dw_real_1 * cos_d_theta
                    - dw_imag_1 * sin_d_theta;
                let mut t_imag_0: libc::c_double = dw_real_1 * sin_d_theta
                    + dw_imag_1 * cos_d_theta;
                dw_real_1 = t_real_0;
                dw_imag_1 = t_imag_0;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2
                <= factor
                    .wrapping_sub(e2)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_3: libc::c_double = dw_real_1 * w_real_1
                        - dw_imag_1 * w_imag_1;
                    let mut tmp_imag_3: libc::c_double = dw_real_1 * w_imag_1
                        + dw_imag_1 * w_real_1;
                    w_real_1 = tmp_real_3;
                    w_imag_1 = tmp_imag_3;
                }
                if e2
                    == factor
                        .wrapping_sub(e2)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    let from0_1: size_t = factor
                        .wrapping_mul(k1)
                        .wrapping_mul(q)
                        .wrapping_add(q)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real_1 = *in_0.offset(istride.wrapping_mul(from0_1) as isize);
                    z_imag_1 = 0.0f64;
                    sum_real_1 += w_real_1 * z_real_1 - w_imag_1 * z_imag_1;
                } else {
                    let from0_2: size_t = factor
                        .wrapping_mul(k1)
                        .wrapping_mul(q)
                        .wrapping_add(q)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real_1 = *in_0.offset(istride.wrapping_mul(from0_2) as isize);
                    z_imag_1 = *in_0
                        .offset(
                            istride
                                .wrapping_mul(
                                    from0_2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
                    sum_real_1
                        += 2 as libc::c_int as libc::c_double
                            * (w_real_1 * z_real_1 - w_imag_1 * z_imag_1);
                }
                e2 = e2.wrapping_add(1);
                e2;
            }
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add(q)
                .wrapping_add(e1.wrapping_mul(m))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = sum_real_1;
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_halfcomplex_float_pass_n(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    factor: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex_float,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let mut e1: size_t = 0;
    let mut e2: size_t = 0;
    let d_theta: libc::c_double = 2.0f64 * 3.14159265358979323846f64
        / factor as libc::c_double;
    let cos_d_theta: libc::c_float = cos(d_theta) as libc::c_float;
    let sin_d_theta: libc::c_float = sin(d_theta) as libc::c_float;
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let mut dw_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut dw_imag: libc::c_float = 0.0f64 as libc::c_float;
        e1 = 0 as libc::c_int as size_t;
        while e1 < factor {
            let mut sum_real: libc::c_float = 0.0f64 as libc::c_float;
            let mut w_real: libc::c_float = 1.0f64 as libc::c_float;
            let mut w_imag: libc::c_float = 0.0f64 as libc::c_float;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut tmp_real: libc::c_float = dw_real * cos_d_theta
                    - dw_imag * sin_d_theta;
                let mut tmp_imag: libc::c_float = dw_real * sin_d_theta
                    + dw_imag * cos_d_theta;
                dw_real = tmp_real;
                dw_imag = tmp_imag;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2 <= factor.wrapping_sub(e2) {
                let mut z_real: libc::c_float = 0.;
                let mut z_imag: libc::c_float = 0.;
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_0: libc::c_float = dw_real * w_real
                        - dw_imag * w_imag;
                    let mut tmp_imag_0: libc::c_float = dw_real * w_imag
                        + dw_imag * w_real;
                    w_real = tmp_real_0;
                    w_imag = tmp_imag_0;
                }
                if e2 == 0 as libc::c_int as libc::c_ulong {
                    let mut from_idx: size_t = factor.wrapping_mul(k1).wrapping_mul(q);
                    z_real = *in_0.offset(istride.wrapping_mul(from_idx) as isize);
                    z_imag = 0.0f64 as libc::c_float;
                    sum_real += w_real * z_real - w_imag * z_imag;
                } else if e2 == factor.wrapping_sub(e2) {
                    let mut from_idx_0: size_t = factor
                        .wrapping_mul(q)
                        .wrapping_mul(k1)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real = *in_0.offset(istride.wrapping_mul(from_idx_0) as isize);
                    z_imag = 0.0f64 as libc::c_float;
                    sum_real += w_real * z_real;
                } else {
                    let mut from_idx_1: size_t = factor
                        .wrapping_mul(q)
                        .wrapping_mul(k1)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real = *in_0.offset(istride.wrapping_mul(from_idx_1) as isize);
                    z_imag = *in_0
                        .offset(
                            istride
                                .wrapping_mul(
                                    from_idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
                    sum_real
                        += 2 as libc::c_int as libc::c_float
                            * (w_real * z_real - w_imag * z_imag);
                }
                e2 = e2.wrapping_add(1);
                e2;
            }
            let to_idx: size_t = q.wrapping_mul(k1).wrapping_add(e1.wrapping_mul(m));
            *out.offset(ostride.wrapping_mul(to_idx) as isize) = sum_real;
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
    if q == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < q
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        k1 = 0 as libc::c_int as size_t;
        while k1 < product_1 {
            let mut dw_real_0: libc::c_float = 1.0f64 as libc::c_float;
            let mut dw_imag_0: libc::c_float = 0.0f64 as libc::c_float;
            e1 = 0 as libc::c_int as size_t;
            while e1 < factor {
                let mut z_real_0: libc::c_float = 0.;
                let mut z_imag_0: libc::c_float = 0.;
                let mut sum_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut sum_imag: libc::c_float = 0.0f64 as libc::c_float;
                let mut w_real_0: libc::c_float = 1.0f64 as libc::c_float;
                let mut w_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                if e1 > 0 as libc::c_int as libc::c_ulong {
                    let mut t_real: libc::c_float = dw_real_0 * cos_d_theta
                        - dw_imag_0 * sin_d_theta;
                    let mut t_imag: libc::c_float = dw_real_0 * sin_d_theta
                        + dw_imag_0 * cos_d_theta;
                    dw_real_0 = t_real;
                    dw_imag_0 = t_imag;
                }
                e2 = 0 as libc::c_int as size_t;
                while e2 < factor {
                    if e2 > 0 as libc::c_int as libc::c_ulong {
                        let mut tmp_real_1: libc::c_float = dw_real_0 * w_real_0
                            - dw_imag_0 * w_imag_0;
                        let mut tmp_imag_1: libc::c_float = dw_real_0 * w_imag_0
                            + dw_imag_0 * w_real_0;
                        w_real_0 = tmp_real_1;
                        w_imag_0 = tmp_imag_1;
                    }
                    if e2 < factor.wrapping_sub(e2) {
                        let from0: size_t = factor
                            .wrapping_mul(k1)
                            .wrapping_mul(q)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                            )
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(e2)
                                    .wrapping_mul(q),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        z_real_0 = *in_0.offset(istride.wrapping_mul(from0) as isize);
                        z_imag_0 = *in_0
                            .offset(
                                istride
                                    .wrapping_mul(
                                        from0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                    } else {
                        let from0_0: size_t = factor
                            .wrapping_mul(k1)
                            .wrapping_mul(q)
                            .wrapping_sub(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                            )
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(factor.wrapping_sub(e2))
                                    .wrapping_mul(q),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        z_real_0 = *in_0.offset(istride.wrapping_mul(from0_0) as isize);
                        z_imag_0 = -*in_0
                            .offset(
                                istride
                                    .wrapping_mul(
                                        from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                    }
                    sum_real_0 += w_real_0 * z_real_0 - w_imag_0 * z_imag_0;
                    sum_imag += w_real_0 * z_imag_0 + w_imag_0 * z_real_0;
                    e2 = e2.wrapping_add(1);
                    e2;
                }
                if k == 0 as libc::c_int as libc::c_ulong
                    || e1 == 0 as libc::c_int as libc::c_ulong
                {
                    w_real_0 = 1.0f64 as libc::c_float;
                    w_imag_0 = 0.0f64 as libc::c_float;
                } else {
                    let mut tskip: size_t = q
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    w_real_0 = (*twiddle
                        .offset(
                            k
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    tskip
                                        .wrapping_mul(
                                            e1.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ),
                                ) as isize,
                        ))
                        .dat[0 as libc::c_int as usize];
                    w_imag_0 = (*twiddle
                        .offset(
                            k
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    tskip
                                        .wrapping_mul(
                                            e1.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ),
                                ) as isize,
                        ))
                        .dat[1 as libc::c_int as usize];
                }
                let to0: size_t = k1
                    .wrapping_mul(q)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                    .wrapping_add(e1.wrapping_mul(m))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                *out
                    .offset(
                        ostride.wrapping_mul(to0) as isize,
                    ) = w_real_0 * sum_real_0 - w_imag_0 * sum_imag;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = w_real_0 * sum_imag + w_imag_0 * sum_real_0;
                e1 = e1.wrapping_add(1);
                e1;
            }
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if q.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let mut tw_arg: libc::c_double = 3.14159265358979323846f64
        / factor as libc::c_double;
    let mut cos_tw_arg: libc::c_float = cos(tw_arg) as libc::c_float;
    let mut sin_tw_arg: libc::c_float = sin(tw_arg) as libc::c_float;
    k1 = 0 as libc::c_int as size_t;
    while k1 < product_1 {
        let mut dw_real_1: libc::c_float = 1.0f64 as libc::c_float;
        let mut dw_imag_1: libc::c_float = 0.0f64 as libc::c_float;
        let mut tw_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut tw_imag: libc::c_float = 0.0f64 as libc::c_float;
        e1 = 0 as libc::c_int as size_t;
        while e1 < factor {
            let mut w_real_1: libc::c_float = 0.;
            let mut w_imag_1: libc::c_float = 0.;
            let mut z_real_1: libc::c_float = 0.;
            let mut z_imag_1: libc::c_float = 0.;
            let mut sum_real_1: libc::c_float = 0.0f64 as libc::c_float;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut tmp_real_2: libc::c_float = tw_real * cos_tw_arg
                    - tw_imag * sin_tw_arg;
                let mut tmp_imag_2: libc::c_float = tw_real * sin_tw_arg
                    + tw_imag * cos_tw_arg;
                tw_real = tmp_real_2;
                tw_imag = tmp_imag_2;
            }
            w_real_1 = tw_real;
            w_imag_1 = tw_imag;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut t_real_0: libc::c_float = dw_real_1 * cos_d_theta
                    - dw_imag_1 * sin_d_theta;
                let mut t_imag_0: libc::c_float = dw_real_1 * sin_d_theta
                    + dw_imag_1 * cos_d_theta;
                dw_real_1 = t_real_0;
                dw_imag_1 = t_imag_0;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2
                <= factor
                    .wrapping_sub(e2)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_3: libc::c_float = dw_real_1 * w_real_1
                        - dw_imag_1 * w_imag_1;
                    let mut tmp_imag_3: libc::c_float = dw_real_1 * w_imag_1
                        + dw_imag_1 * w_real_1;
                    w_real_1 = tmp_real_3;
                    w_imag_1 = tmp_imag_3;
                }
                if e2
                    == factor
                        .wrapping_sub(e2)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    let from0_1: size_t = factor
                        .wrapping_mul(k1)
                        .wrapping_mul(q)
                        .wrapping_add(q)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real_1 = *in_0.offset(istride.wrapping_mul(from0_1) as isize);
                    z_imag_1 = 0.0f64 as libc::c_float;
                    sum_real_1 += w_real_1 * z_real_1 - w_imag_1 * z_imag_1;
                } else {
                    let from0_2: size_t = factor
                        .wrapping_mul(k1)
                        .wrapping_mul(q)
                        .wrapping_add(q)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e2)
                                .wrapping_mul(q),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    z_real_1 = *in_0.offset(istride.wrapping_mul(from0_2) as isize);
                    z_imag_1 = *in_0
                        .offset(
                            istride
                                .wrapping_mul(
                                    from0_2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
                    sum_real_1
                        += 2 as libc::c_int as libc::c_float
                            * (w_real_1 * z_real_1 - w_imag_1 * z_imag_1);
                }
                e2 = e2.wrapping_add(1);
                e2;
            }
            let to0_0: size_t = k1
                .wrapping_mul(q)
                .wrapping_add(q)
                .wrapping_add(e1.wrapping_mul(m))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = sum_real_1;
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_radix2_backward(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_float_radix2_transform(
        data,
        stride,
        n,
    );
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_radix2_backward(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_radix2_transform(data, stride, n);
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_radix2_inverse(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_radix2_transform(data, stride, n);
    if status != 0 {
        return status;
    }
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data.offset(stride.wrapping_mul(i) as isize) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_radix2_inverse(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_fft_halfcomplex_float_radix2_transform(
        data,
        stride,
        n,
    );
    if status != 0 {
        return status;
    }
    let norm: libc::c_float = (1.0f64 / n as libc::c_double) as libc::c_float;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data.offset(stride.wrapping_mul(i) as isize) *= norm;
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_radix2_transform(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut p: size_t = 0;
    let mut p_1: size_t = 0;
    let mut q: size_t = 0;
    let mut i: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./hc_radix2.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    p = n;
    q = 1 as libc::c_int as size_t;
    p_1 = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    i = 1 as libc::c_int as size_t;
    while i <= logn {
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        b = 0 as libc::c_int as size_t;
        while b < q {
            let z0: libc::c_double = *data
                .offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize);
            let z1: libc::c_double = *data
                .offset(
                    stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                );
            let t0_real: libc::c_double = z0 + z1;
            let t1_real: libc::c_double = z0 - z1;
            *data.offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize) = t0_real;
            *data
                .offset(
                    stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                ) = t1_real;
            b = b.wrapping_add(1);
            b;
        }
        let mut w_real: libc::c_double = 1.0f64;
        let mut w_imag: libc::c_double = 0.0f64;
        let theta: libc::c_double = 2.0f64 * 3.14159265358979323846f64
            / p as libc::c_double;
        let s: libc::c_double = sin(theta);
        let t: libc::c_double = sin(theta / 2.0f64);
        let s2: libc::c_double = 2.0f64 * t * t;
        a = 1 as libc::c_int as size_t;
        while a < p_1.wrapping_div(2 as libc::c_int as libc::c_ulong) {
            let tmp_real: libc::c_double = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_double = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = 0 as libc::c_int as size_t;
            while b < q {
                let mut z0_real: libc::c_double = *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    );
                let mut z0_imag: libc::c_double = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    );
                let mut z1_real: libc::c_double = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    );
                let mut z1_imag: libc::c_double = -*data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    );
                let mut t0_real_0: libc::c_double = z0_real + z1_real;
                let mut t0_imag: libc::c_double = z0_imag + z1_imag;
                let mut t1_real_0: libc::c_double = z0_real - z1_real;
                let mut t1_imag: libc::c_double = z0_imag - z1_imag;
                *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    ) = t0_real_0;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    ) = t0_imag;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    ) = w_real * t1_real_0 - w_imag * t1_imag;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    ) = w_real * t1_imag + w_imag * t1_real_0;
                b = b.wrapping_add(1);
                b;
            }
            a = a.wrapping_add(1);
            a;
        }
        if p_1 > 1 as libc::c_int as libc::c_ulong {
            b = 0 as libc::c_int as size_t;
            while b < q {
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b
                                    .wrapping_mul(p)
                                    .wrapping_add(
                                        p_1.wrapping_div(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as isize,
                    ) *= 2 as libc::c_int as libc::c_double;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b
                                    .wrapping_mul(p)
                                    .wrapping_add(p_1)
                                    .wrapping_add(
                                        p_1.wrapping_div(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as isize,
                    ) *= -(2 as libc::c_int) as libc::c_double;
                b = b.wrapping_add(1);
                b;
            }
        }
        p_1 = p_1.wrapping_div(2 as libc::c_int as libc::c_ulong);
        p = p.wrapping_div(2 as libc::c_int as libc::c_ulong);
        q = q.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        i = i.wrapping_add(1);
        i;
    }
    status = fft_real_bitreverse_order(data, stride, n, logn);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_radix2_transform(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut p: size_t = 0;
    let mut p_1: size_t = 0;
    let mut q: size_t = 0;
    let mut i: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./hc_radix2.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    p = n;
    q = 1 as libc::c_int as size_t;
    p_1 = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    i = 1 as libc::c_int as size_t;
    while i <= logn {
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        b = 0 as libc::c_int as size_t;
        while b < q {
            let z0: libc::c_float = *data
                .offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize);
            let z1: libc::c_float = *data
                .offset(
                    stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                );
            let t0_real: libc::c_float = z0 + z1;
            let t1_real: libc::c_float = z0 - z1;
            *data.offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize) = t0_real;
            *data
                .offset(
                    stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                ) = t1_real;
            b = b.wrapping_add(1);
            b;
        }
        let mut w_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut w_imag: libc::c_float = 0.0f64 as libc::c_float;
        let theta: libc::c_float = (2.0f64 * 3.14159265358979323846f64
            / p as libc::c_double) as libc::c_float;
        let s: libc::c_float = sin(theta as libc::c_double) as libc::c_float;
        let t: libc::c_float = sin(theta as libc::c_double / 2.0f64) as libc::c_float;
        let s2: libc::c_float = (2.0f64 * t as libc::c_double * t as libc::c_double)
            as libc::c_float;
        a = 1 as libc::c_int as size_t;
        while a < p_1.wrapping_div(2 as libc::c_int as libc::c_ulong) {
            let tmp_real: libc::c_float = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_float = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = 0 as libc::c_int as size_t;
            while b < q {
                let mut z0_real: libc::c_float = *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    );
                let mut z0_imag: libc::c_float = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    );
                let mut z1_real: libc::c_float = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    );
                let mut z1_imag: libc::c_float = -*data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    );
                let mut t0_real_0: libc::c_float = z0_real + z1_real;
                let mut t0_imag: libc::c_float = z0_imag + z1_imag;
                let mut t1_real_0: libc::c_float = z0_real - z1_real;
                let mut t1_imag: libc::c_float = z0_imag - z1_imag;
                *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    ) = t0_real_0;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    ) = t0_imag;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    ) = w_real * t1_real_0 - w_imag * t1_imag;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    ) = w_real * t1_imag + w_imag * t1_real_0;
                b = b.wrapping_add(1);
                b;
            }
            a = a.wrapping_add(1);
            a;
        }
        if p_1 > 1 as libc::c_int as libc::c_ulong {
            b = 0 as libc::c_int as size_t;
            while b < q {
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b
                                    .wrapping_mul(p)
                                    .wrapping_add(
                                        p_1.wrapping_div(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as isize,
                    ) *= 2 as libc::c_int as libc::c_float;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b
                                    .wrapping_mul(p)
                                    .wrapping_add(p_1)
                                    .wrapping_add(
                                        p_1.wrapping_div(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as isize,
                    ) *= -(2 as libc::c_int) as libc::c_float;
                b = b.wrapping_add(1);
                b;
            }
        }
        p_1 = p_1.wrapping_div(2 as libc::c_int as libc::c_ulong);
        p = p.wrapping_div(2 as libc::c_int as libc::c_ulong);
        q = q.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        i = i.wrapping_add(1);
        i;
    }
    status = fft_real_float_bitreverse_order(data, stride, n, logn);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_unpack(
    mut halfcomplex_coefficient: *const libc::c_double,
    mut complex_coefficient: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_unpack.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
        ) = *halfcomplex_coefficient.offset(0 as libc::c_int as isize);
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0.0f64;
    i = 1 as libc::c_int as size_t;
    while i < n.wrapping_sub(i) {
        let hc_real: libc::c_double = *halfcomplex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride) as isize,
            );
        let hc_imag: libc::c_double = *halfcomplex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            );
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = hc_imag;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i)) as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = -hc_imag;
        i = i.wrapping_add(1);
        i;
    }
    if i == n.wrapping_sub(i) {
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = *halfcomplex_coefficient
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0.0f64;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_unpack(
    mut halfcomplex_coefficient: *const libc::c_float,
    mut complex_coefficient: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_unpack.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
        ) = *halfcomplex_coefficient.offset(0 as libc::c_int as isize);
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0.0f64 as libc::c_float;
    i = 1 as libc::c_int as size_t;
    while i < n.wrapping_sub(i) {
        let hc_real: libc::c_float = *halfcomplex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride) as isize,
            );
        let hc_imag: libc::c_float = *halfcomplex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            );
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = hc_imag;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i)) as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = -hc_imag;
        i = i.wrapping_add(1);
        i;
    }
    if i == n.wrapping_sub(i) {
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = *halfcomplex_coefficient
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0.0f64 as libc::c_float;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_radix2_unpack(
    mut halfcomplex_coefficient: *const libc::c_double,
    mut complex_coefficient: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_unpack.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
        ) = *halfcomplex_coefficient.offset(0 as libc::c_int as isize);
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0.0f64;
    i = 1 as libc::c_int as size_t;
    while i < n.wrapping_sub(i) {
        let hc_real: libc::c_double = *halfcomplex_coefficient
            .offset(i.wrapping_mul(stride) as isize);
        let hc_imag: libc::c_double = *halfcomplex_coefficient
            .offset(n.wrapping_sub(i).wrapping_mul(stride) as isize);
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = hc_imag;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i)) as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = -hc_imag;
        i = i.wrapping_add(1);
        i;
    }
    if i == n.wrapping_sub(i) {
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = *halfcomplex_coefficient.offset(i.wrapping_mul(stride) as isize);
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0.0f64;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_halfcomplex_float_radix2_unpack(
    mut halfcomplex_coefficient: *const libc::c_float,
    mut complex_coefficient: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./hc_unpack.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
        ) = *halfcomplex_coefficient.offset(0 as libc::c_int as isize);
    *complex_coefficient
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(stride)
                .wrapping_mul(0 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0.0f64 as libc::c_float;
    i = 1 as libc::c_int as size_t;
    while i < n.wrapping_sub(i) {
        let hc_real: libc::c_float = *halfcomplex_coefficient
            .offset(i.wrapping_mul(stride) as isize);
        let hc_imag: libc::c_float = *halfcomplex_coefficient
            .offset(n.wrapping_sub(i).wrapping_mul(stride) as isize);
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = hc_imag;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i)) as isize,
            ) = hc_real;
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(n.wrapping_sub(i))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = -hc_imag;
        i = i.wrapping_add(1);
        i;
    }
    if i == n.wrapping_sub(i) {
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = *halfcomplex_coefficient.offset(i.wrapping_mul(stride) as isize);
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0.0f64 as libc::c_float;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_real_wavetable_float_alloc(
    mut n: size_t,
) -> *mut gsl_fft_real_wavetable_float {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_factors: size_t = 0;
    let mut t: size_t = 0;
    let mut product: size_t = 0;
    let mut product_1: size_t = 0;
    let mut q: size_t = 0;
    let mut d_theta: libc::c_double = 0.;
    let mut wavetable: *mut gsl_fft_real_wavetable_float = 0
        as *mut gsl_fft_real_wavetable_float;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable_float;
    }
    wavetable = malloc(
        ::std::mem::size_of::<gsl_fft_real_wavetable_float>() as libc::c_ulong,
    ) as *mut gsl_fft_real_wavetable_float;
    if wavetable.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable_float;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        (*wavetable).trig = 0 as *mut gsl_complex_float;
    } else {
        (*wavetable)
            .trig = malloc(
            n
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<gsl_complex_float>() as libc::c_ulong,
                ),
        ) as *mut gsl_complex_float;
        if ((*wavetable).trig).is_null() {
            free(wavetable as *mut libc::c_void);
            gsl_error(
                b"failed to allocate trigonometric lookup table\0" as *const u8
                    as *const libc::c_char,
                b"./real_init.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_fft_real_wavetable_float;
        }
    }
    (*wavetable).n = n;
    status = fft_real_factorize(n, &mut n_factors, ((*wavetable).factor).as_mut_ptr());
    if status != 0 {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EFACTOR as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable_float;
    }
    (*wavetable).nf = n_factors;
    d_theta = 2.0f64 * 3.14159265358979323846f64 / n as libc::c_double;
    t = 0 as libc::c_int as size_t;
    product = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < (*wavetable).nf {
        let mut j: size_t = 0;
        let factor: size_t = (*wavetable).factor[i as usize];
        (*wavetable).twiddle[i as usize] = ((*wavetable).trig).offset(t as isize);
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        j = 1 as libc::c_int as size_t;
        while j < factor {
            let mut k: size_t = 0;
            let mut m: size_t = 0 as libc::c_int as size_t;
            k = 1 as libc::c_int as size_t;
            while k
                < product_1
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                let mut theta: libc::c_double = 0.;
                m = m.wrapping_add(j.wrapping_mul(q));
                m = m.wrapping_rem(n);
                theta = d_theta * m as libc::c_double;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[0 as libc::c_int as usize] = cos(theta) as libc::c_float;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[1 as libc::c_int as usize] = sin(theta) as libc::c_float;
                t = t.wrapping_add(1);
                t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if t > n.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"overflowed trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable_float;
    }
    return wavetable;
}
pub unsafe extern "C" fn gsl_fft_real_wavetable_alloc(
    mut n: size_t,
) -> *mut gsl_fft_real_wavetable {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_factors: size_t = 0;
    let mut t: size_t = 0;
    let mut product: size_t = 0;
    let mut product_1: size_t = 0;
    let mut q: size_t = 0;
    let mut d_theta: libc::c_double = 0.;
    let mut wavetable: *mut gsl_fft_real_wavetable = 0 as *mut gsl_fft_real_wavetable;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable;
    }
    wavetable = malloc(::std::mem::size_of::<gsl_fft_real_wavetable>() as libc::c_ulong)
        as *mut gsl_fft_real_wavetable;
    if wavetable.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        (*wavetable).trig = 0 as *mut gsl_complex;
    } else {
        (*wavetable)
            .trig = malloc(
            n
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<gsl_complex>() as libc::c_ulong),
        ) as *mut gsl_complex;
        if ((*wavetable).trig).is_null() {
            free(wavetable as *mut libc::c_void);
            gsl_error(
                b"failed to allocate trigonometric lookup table\0" as *const u8
                    as *const libc::c_char,
                b"./real_init.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_fft_real_wavetable;
        }
    }
    (*wavetable).n = n;
    status = fft_real_factorize(n, &mut n_factors, ((*wavetable).factor).as_mut_ptr());
    if status != 0 {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"factorization failed\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EFACTOR as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable;
    }
    (*wavetable).nf = n_factors;
    d_theta = 2.0f64 * 3.14159265358979323846f64 / n as libc::c_double;
    t = 0 as libc::c_int as size_t;
    product = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < (*wavetable).nf {
        let mut j: size_t = 0;
        let factor: size_t = (*wavetable).factor[i as usize];
        (*wavetable).twiddle[i as usize] = ((*wavetable).trig).offset(t as isize);
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        j = 1 as libc::c_int as size_t;
        while j < factor {
            let mut k: size_t = 0;
            let mut m: size_t = 0 as libc::c_int as size_t;
            k = 1 as libc::c_int as size_t;
            while k
                < product_1
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                let mut theta: libc::c_double = 0.;
                m = m.wrapping_add(j.wrapping_mul(q));
                m = m.wrapping_rem(n);
                theta = d_theta * m as libc::c_double;
                (*((*wavetable).trig).offset(t as isize))
                    .dat[0 as libc::c_int as usize] = cos(theta);
                (*((*wavetable).trig).offset(t as isize))
                    .dat[1 as libc::c_int as usize] = sin(theta);
                t = t.wrapping_add(1);
                t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if t > n.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        free((*wavetable).trig as *mut libc::c_void);
        free(wavetable as *mut libc::c_void);
        gsl_error(
            b"overflowed trigonometric lookup table\0" as *const u8
                as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_wavetable;
    }
    return wavetable;
}
pub unsafe extern "C" fn gsl_fft_real_workspace_float_alloc(
    mut n: size_t,
) -> *mut gsl_fft_real_workspace_float {
    let mut workspace: *mut gsl_fft_real_workspace_float = 0
        as *mut gsl_fft_real_workspace_float;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_workspace_float;
    }
    workspace = malloc(
        ::std::mem::size_of::<gsl_fft_real_workspace_float>() as libc::c_ulong,
    ) as *mut gsl_fft_real_workspace_float;
    if workspace.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_workspace_float;
    }
    (*workspace).n = n;
    (*workspace)
        .scratch = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ((*workspace).scratch).is_null() {
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate scratch space\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_workspace_float;
    }
    return workspace;
}
pub unsafe extern "C" fn gsl_fft_real_workspace_alloc(
    mut n: size_t,
) -> *mut gsl_fft_real_workspace {
    let mut workspace: *mut gsl_fft_real_workspace = 0 as *mut gsl_fft_real_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_workspace;
    }
    workspace = malloc(::std::mem::size_of::<gsl_fft_real_workspace>() as libc::c_ulong)
        as *mut gsl_fft_real_workspace;
    if workspace.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_workspace;
    }
    (*workspace).n = n;
    (*workspace)
        .scratch = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*workspace).scratch).is_null() {
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate scratch space\0" as *const u8 as *const libc::c_char,
            b"./real_init.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_fft_real_workspace;
    }
    return workspace;
}
pub unsafe extern "C" fn gsl_fft_real_wavetable_free(
    mut wavetable: *mut gsl_fft_real_wavetable,
) {
    if wavetable.is_null() {
        return;
    }
    free((*wavetable).trig as *mut libc::c_void);
    (*wavetable).trig = 0 as *mut gsl_complex;
    free(wavetable as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_real_wavetable_float_free(
    mut wavetable: *mut gsl_fft_real_wavetable_float,
) {
    if wavetable.is_null() {
        return;
    }
    free((*wavetable).trig as *mut libc::c_void);
    (*wavetable).trig = 0 as *mut gsl_complex_float;
    free(wavetable as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_real_workspace_float_free(
    mut workspace: *mut gsl_fft_real_workspace_float,
) {
    if workspace.is_null() {
        return;
    }
    free((*workspace).scratch as *mut libc::c_void);
    (*workspace).scratch = 0 as *mut libc::c_float;
    free(workspace as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_real_workspace_free(
    mut workspace: *mut gsl_fft_real_workspace,
) {
    if workspace.is_null() {
        return;
    }
    free((*workspace).scratch as *mut libc::c_void);
    (*workspace).scratch = 0 as *mut libc::c_double;
    free(workspace as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_fft_real_transform(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_real_wavetable,
    mut work: *mut gsl_fft_real_workspace,
) -> libc::c_int {
    let nf: size_t = (*wavetable).nf;
    let mut i: size_t = 0;
    let mut q: size_t = 0;
    let mut product: size_t = 1 as libc::c_int as size_t;
    let mut tskip: size_t = 0;
    let mut product_1: size_t = 0;
    let scratch: *mut libc::c_double = (*work).scratch;
    let mut twiddle1: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle2: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle3: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut twiddle4: *mut gsl_complex = 0 as *mut gsl_complex;
    let mut state: size_t = 0 as libc::c_int as size_t;
    let mut in_0: *mut libc::c_double = data;
    let mut istride: size_t = stride;
    let mut out: *mut libc::c_double = scratch;
    let mut ostride: size_t = 1 as libc::c_int as size_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_main.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n != (*wavetable).n {
        gsl_error(
            b"wavetable does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./real_main.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n != (*work).n {
        gsl_error(
            b"workspace does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./real_main.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < nf {
        let factor: size_t = (*wavetable).factor[i as usize];
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        tskip = product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if state == 0 as libc::c_int as libc::c_ulong {
            in_0 = data;
            istride = stride;
            out = scratch;
            ostride = 1 as libc::c_int as size_t;
            state = 1 as libc::c_int as size_t;
        } else {
            in_0 = scratch;
            istride = 1 as libc::c_int as size_t;
            out = data;
            ostride = stride;
            state = 0 as libc::c_int as size_t;
        }
        if factor == 2 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_real_pass_2(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
            );
        } else if factor == 3 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            fft_real_pass_3(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
            );
        } else if factor == 4 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            fft_real_pass_4(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
            );
        } else if factor == 5 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            twiddle4 = twiddle3.offset(tskip as isize);
            fft_real_pass_5(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex,
                twiddle2 as *const gsl_complex,
                twiddle3 as *const gsl_complex,
                twiddle4 as *const gsl_complex,
            );
        } else {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_real_pass_n(
                in_0 as *const libc::c_double,
                istride,
                out,
                ostride,
                factor,
                product,
                n,
                twiddle1 as *const gsl_complex,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if state == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *data.offset(stride.wrapping_mul(i) as isize) = *scratch.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_real_float_transform(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    mut wavetable: *const gsl_fft_real_wavetable_float,
    mut work: *mut gsl_fft_real_workspace_float,
) -> libc::c_int {
    let nf: size_t = (*wavetable).nf;
    let mut i: size_t = 0;
    let mut q: size_t = 0;
    let mut product: size_t = 1 as libc::c_int as size_t;
    let mut tskip: size_t = 0;
    let mut product_1: size_t = 0;
    let scratch: *mut libc::c_float = (*work).scratch;
    let mut twiddle1: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle2: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle3: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut twiddle4: *mut gsl_complex_float = 0 as *mut gsl_complex_float;
    let mut state: size_t = 0 as libc::c_int as size_t;
    let mut in_0: *mut libc::c_float = data;
    let mut istride: size_t = stride;
    let mut out: *mut libc::c_float = scratch;
    let mut ostride: size_t = 1 as libc::c_int as size_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_main.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n != (*wavetable).n {
        gsl_error(
            b"wavetable does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./real_main.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n != (*work).n {
        gsl_error(
            b"workspace does not match length of data\0" as *const u8
                as *const libc::c_char,
            b"./real_main.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < nf {
        let factor: size_t = (*wavetable).factor[i as usize];
        product_1 = product;
        product = (product as libc::c_ulong).wrapping_mul(factor) as size_t as size_t;
        q = n.wrapping_div(product);
        tskip = product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if state == 0 as libc::c_int as libc::c_ulong {
            in_0 = data;
            istride = stride;
            out = scratch;
            ostride = 1 as libc::c_int as size_t;
            state = 1 as libc::c_int as size_t;
        } else {
            in_0 = scratch;
            istride = 1 as libc::c_int as size_t;
            out = data;
            ostride = stride;
            state = 0 as libc::c_int as size_t;
        }
        if factor == 2 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_real_float_pass_2(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
            );
        } else if factor == 3 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            fft_real_float_pass_3(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
            );
        } else if factor == 4 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            fft_real_float_pass_4(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
            );
        } else if factor == 5 as libc::c_int as libc::c_ulong {
            twiddle1 = (*wavetable).twiddle[i as usize];
            twiddle2 = twiddle1.offset(tskip as isize);
            twiddle3 = twiddle2.offset(tskip as isize);
            twiddle4 = twiddle3.offset(tskip as isize);
            fft_real_float_pass_5(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
                twiddle2 as *const gsl_complex_float,
                twiddle3 as *const gsl_complex_float,
                twiddle4 as *const gsl_complex_float,
            );
        } else {
            twiddle1 = (*wavetable).twiddle[i as usize];
            fft_real_float_pass_n(
                in_0 as *const libc::c_float,
                istride,
                out,
                ostride,
                factor,
                product,
                n,
                twiddle1 as *const gsl_complex_float,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if state == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *data.offset(stride.wrapping_mul(i) as isize) = *scratch.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fft_real_float_pass_2(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex_float,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 2 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let r0: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let r1: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let s0: libc::c_float = r0 + r1;
        let s1: libc::c_float = r0 - r1;
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add(product)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        *out.offset(ostride.wrapping_mul(to0) as isize) = s0;
        *out.offset(ostride.wrapping_mul(to1) as isize) = s1;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w_real: libc::c_float = (*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w_imag: libc::c_float = -(*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let f0_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real: libc::c_float = f0_real;
            let z0_imag: libc::c_float = f0_imag;
            let z1_real: libc::c_float = w_real * f1_real - w_imag * f1_imag;
            let z1_imag: libc::c_float = w_real * f1_imag + w_imag * f1_real;
            let x0_real: libc::c_float = z0_real + z1_real;
            let x0_imag: libc::c_float = z0_imag + z1_imag;
            let x1_real: libc::c_float = z0_real - z1_real;
            let x1_imag: libc::c_float = z0_imag - z1_imag;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add(product)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x1_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        *out
            .offset(
                ostride.wrapping_mul(to0_1) as isize,
            ) = *in_0.offset(istride.wrapping_mul(from0_1) as isize);
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -*in_0.offset(istride.wrapping_mul(from1_1) as isize);
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_pass_2(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 2 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let r0: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let r1: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let s0: libc::c_double = r0 + r1;
        let s1: libc::c_double = r0 - r1;
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add(product)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        *out.offset(ostride.wrapping_mul(to0) as isize) = s0;
        *out.offset(ostride.wrapping_mul(to1) as isize) = s1;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w_real: libc::c_double = (*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w_imag: libc::c_double = -(*twiddle
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let f0_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real: libc::c_double = f0_real;
            let z0_imag: libc::c_double = f0_imag;
            let z1_real: libc::c_double = w_real * f1_real - w_imag * f1_imag;
            let z1_imag: libc::c_double = w_real * f1_imag + w_imag * f1_real;
            let x0_real: libc::c_double = z0_real + z1_real;
            let x0_imag: libc::c_double = z0_imag + z1_imag;
            let x1_real: libc::c_double = z0_real - z1_real;
            let x1_imag: libc::c_double = z0_imag - z1_imag;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add(product)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x1_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        *out
            .offset(
                ostride.wrapping_mul(to0_1) as isize,
            ) = *in_0.offset(istride.wrapping_mul(from0_1) as isize);
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -*in_0.offset(istride.wrapping_mul(from1_1) as isize);
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_float_pass_3(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 3 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let tau: libc::c_float = (sqrt(3.0f64) / 2.0f64) as libc::c_float;
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let from2: size_t = from1.wrapping_add(m);
        let z0_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z2_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let t1: libc::c_float = z1_real + z2_real;
        let x0_real: libc::c_float = z0_real + t1;
        let x1_real: libc::c_float = (z0_real as libc::c_double
            - t1 as libc::c_double / 2.0f64) as libc::c_float;
        let x1_imag: libc::c_float = -tau * (z1_real - z2_real);
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out
            .offset(
                ostride.wrapping_mul(to1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x1_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_float = -(*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_float = -(*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let from2_0: size_t = from1_0.wrapping_add(m);
            let f0_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f2_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let f2_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real_0: libc::c_float = f0_real;
            let z0_imag: libc::c_float = f0_imag;
            let z1_real_0: libc::c_float = w1_real * f1_real - w1_imag * f1_imag;
            let z1_imag: libc::c_float = w1_real * f1_imag + w1_imag * f1_real;
            let z2_real_0: libc::c_float = w2_real * f2_real - w2_imag * f2_imag;
            let z2_imag: libc::c_float = w2_real * f2_imag + w2_imag * f2_real;
            let t1_real: libc::c_float = z1_real_0 + z2_real_0;
            let t1_imag: libc::c_float = z1_imag + z2_imag;
            let t2_real: libc::c_float = z0_real_0
                - t1_real / 2 as libc::c_int as libc::c_float;
            let t2_imag: libc::c_float = z0_imag
                - t1_imag / 2 as libc::c_int as libc::c_float;
            let t3_real: libc::c_float = -tau * (z1_real_0 - z2_real_0);
            let t3_imag: libc::c_float = -tau * (z1_imag - z2_imag);
            let x0_real_0: libc::c_float = z0_real_0 + t1_real;
            let x0_imag: libc::c_float = z0_imag + t1_imag;
            let x1_real_0: libc::c_float = t2_real - t3_imag;
            let x1_imag_0: libc::c_float = t2_imag + t3_real;
            let x2_real: libc::c_float = t2_real + t3_imag;
            let x2_imag: libc::c_float = t2_imag - t3_real;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to2: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(product_1)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add(k1.wrapping_mul(product))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x1_imag_0;
            *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x2_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let from2_1: size_t = from1_1.wrapping_add(m);
        let z0_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z1_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let z2_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from2_1) as isize);
        let t1_0: libc::c_float = z1_real_1 - z2_real_1;
        let x0_real_1: libc::c_float = (z0_real_1 as libc::c_double
            + t1_0 as libc::c_double / 2.0f64) as libc::c_float;
        let x0_imag_0: libc::c_float = -tau * (z1_real_1 + z2_real_1);
        let x1_real_1: libc::c_float = z0_real_1 - t1_0;
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_1;
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x0_imag_0;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_1;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_pass_3(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 3 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let tau: libc::c_double = sqrt(3.0f64) / 2.0f64;
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let from2: size_t = from1.wrapping_add(m);
        let z0_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z2_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let t1: libc::c_double = z1_real + z2_real;
        let x0_real: libc::c_double = z0_real + t1;
        let x1_real: libc::c_double = z0_real - t1 / 2.0f64;
        let x1_imag: libc::c_double = -tau * (z1_real - z2_real);
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out
            .offset(
                ostride.wrapping_mul(to1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x1_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_double = -(*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_double = -(*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let from2_0: size_t = from1_0.wrapping_add(m);
            let f0_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f2_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let f2_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real_0: libc::c_double = f0_real;
            let z0_imag: libc::c_double = f0_imag;
            let z1_real_0: libc::c_double = w1_real * f1_real - w1_imag * f1_imag;
            let z1_imag: libc::c_double = w1_real * f1_imag + w1_imag * f1_real;
            let z2_real_0: libc::c_double = w2_real * f2_real - w2_imag * f2_imag;
            let z2_imag: libc::c_double = w2_real * f2_imag + w2_imag * f2_real;
            let t1_real: libc::c_double = z1_real_0 + z2_real_0;
            let t1_imag: libc::c_double = z1_imag + z2_imag;
            let t2_real: libc::c_double = z0_real_0
                - t1_real / 2 as libc::c_int as libc::c_double;
            let t2_imag: libc::c_double = z0_imag
                - t1_imag / 2 as libc::c_int as libc::c_double;
            let t3_real: libc::c_double = -tau * (z1_real_0 - z2_real_0);
            let t3_imag: libc::c_double = -tau * (z1_imag - z2_imag);
            let x0_real_0: libc::c_double = z0_real_0 + t1_real;
            let x0_imag: libc::c_double = z0_imag + t1_imag;
            let x1_real_0: libc::c_double = t2_real - t3_imag;
            let x1_imag_0: libc::c_double = t2_imag + t3_real;
            let x2_real: libc::c_double = t2_real + t3_imag;
            let x2_imag: libc::c_double = t2_imag - t3_real;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to2: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(product_1)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add(k1.wrapping_mul(product))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x1_imag_0;
            *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x2_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let from2_1: size_t = from1_1.wrapping_add(m);
        let z0_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z1_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let z2_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from2_1) as isize);
        let t1_0: libc::c_double = z1_real_1 - z2_real_1;
        let x0_real_1: libc::c_double = z0_real_1 + t1_0 / 2.0f64;
        let x0_imag_0: libc::c_double = -tau * (z1_real_1 + z2_real_1);
        let x1_real_1: libc::c_double = z0_real_1 - t1_0;
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0_real_1;
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x0_imag_0;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x1_real_1;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_float_pass_4(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 4 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let from2: size_t = from1.wrapping_add(m);
        let from3: size_t = from2.wrapping_add(m);
        let z0_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z2_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let z3_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from3) as isize);
        let t1_real: libc::c_float = z0_real + z2_real;
        let t2_real: libc::c_float = z1_real + z3_real;
        let t3_real: libc::c_float = z0_real - z2_real;
        let t4_real: libc::c_float = -(z1_real - z3_real);
        let x0_real: libc::c_float = t1_real + t2_real;
        let x1_real: libc::c_float = t3_real;
        let x1_imag: libc::c_float = t4_real;
        let x2_real: libc::c_float = t1_real - t2_real;
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to2: size_t = to1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out
            .offset(
                ostride.wrapping_mul(to1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x1_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let mut w1_real: libc::c_float = 0.;
        let mut w1_imag: libc::c_float = 0.;
        let mut w2_real: libc::c_float = 0.;
        let mut w2_imag: libc::c_float = 0.;
        let mut w3_real: libc::c_float = 0.;
        let mut w3_imag: libc::c_float = 0.;
        w1_real = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        w1_imag = -(*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        w2_real = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        w2_imag = -(*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        w3_real = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        w3_imag = -(*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let from2_0: size_t = from1_0.wrapping_add(m);
            let from3_0: size_t = from2_0.wrapping_add(m);
            let f0_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f2_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let f2_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f3_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from3_0) as isize);
            let f3_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real_0: libc::c_float = f0_real;
            let z0_imag: libc::c_float = f0_imag;
            let z1_real_0: libc::c_float = w1_real * f1_real - w1_imag * f1_imag;
            let z1_imag: libc::c_float = w1_real * f1_imag + w1_imag * f1_real;
            let z2_real_0: libc::c_float = w2_real * f2_real - w2_imag * f2_imag;
            let z2_imag: libc::c_float = w2_real * f2_imag + w2_imag * f2_real;
            let z3_real_0: libc::c_float = w3_real * f3_real - w3_imag * f3_imag;
            let z3_imag: libc::c_float = w3_real * f3_imag + w3_imag * f3_real;
            let t1_real_0: libc::c_float = z0_real_0 + z2_real_0;
            let t1_imag: libc::c_float = z0_imag + z2_imag;
            let t2_real_0: libc::c_float = z1_real_0 + z3_real_0;
            let t2_imag: libc::c_float = z1_imag + z3_imag;
            let t3_real_0: libc::c_float = z0_real_0 - z2_real_0;
            let t3_imag: libc::c_float = z0_imag - z2_imag;
            let t4_real_0: libc::c_float = -(z1_real_0 - z3_real_0);
            let t4_imag: libc::c_float = -(z1_imag - z3_imag);
            let x0_real_0: libc::c_float = t1_real_0 + t2_real_0;
            let x0_imag: libc::c_float = t1_imag + t2_imag;
            let x1_real_0: libc::c_float = t3_real_0 - t4_imag;
            let x1_imag_0: libc::c_float = t3_imag + t4_real_0;
            let x2_real_0: libc::c_float = t1_real_0 - t2_real_0;
            let x2_imag: libc::c_float = t1_imag - t2_imag;
            let x3_real: libc::c_float = t3_real_0 + t4_imag;
            let x3_imag: libc::c_float = t3_imag - t4_real_0;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to2_0: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(product_1)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add(k1.wrapping_mul(product))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to3: size_t = to2_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x1_imag_0;
            *out.offset(ostride.wrapping_mul(to3) as isize) = x2_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x2_imag;
            *out.offset(ostride.wrapping_mul(to2_0) as isize) = x3_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x3_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let from2_1: size_t = from1_1.wrapping_add(m);
        let from3_1: size_t = from2_1.wrapping_add(m);
        let x0: libc::c_float = *in_0.offset(istride.wrapping_mul(from0_1) as isize);
        let x1: libc::c_float = *in_0.offset(istride.wrapping_mul(from1_1) as isize);
        let x2: libc::c_float = *in_0.offset(istride.wrapping_mul(from2_1) as isize);
        let x3: libc::c_float = *in_0.offset(istride.wrapping_mul(from3_1) as isize);
        let t1: libc::c_float = (1.0f64 / sqrt(2.0f64) * (x1 - x3) as libc::c_double)
            as libc::c_float;
        let t2: libc::c_float = (1.0f64 / sqrt(2.0f64) * (x1 + x3) as libc::c_double)
            as libc::c_float;
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0 + t1;
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -x2 - t2;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x0 - t1;
        *out
            .offset(
                ostride
                    .wrapping_mul(to1_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x2 - t2;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_pass_4(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 4 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let from2: size_t = from1.wrapping_add(m);
        let from3: size_t = from2.wrapping_add(m);
        let z0_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z2_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let z3_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from3) as isize);
        let t1_real: libc::c_double = z0_real + z2_real;
        let t2_real: libc::c_double = z1_real + z3_real;
        let t3_real: libc::c_double = z0_real - z2_real;
        let t4_real: libc::c_double = -(z1_real - z3_real);
        let x0_real: libc::c_double = t1_real + t2_real;
        let x1_real: libc::c_double = t3_real;
        let x1_imag: libc::c_double = t4_real;
        let x2_real: libc::c_double = t1_real - t2_real;
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to2: size_t = to1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out
            .offset(
                ostride.wrapping_mul(to1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x1_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let mut w1_real: libc::c_double = 0.;
        let mut w1_imag: libc::c_double = 0.;
        let mut w2_real: libc::c_double = 0.;
        let mut w2_imag: libc::c_double = 0.;
        let mut w3_real: libc::c_double = 0.;
        let mut w3_imag: libc::c_double = 0.;
        w1_real = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        w1_imag = -(*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        w2_real = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        w2_imag = -(*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        w3_real = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        w3_imag = -(*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let from2_0: size_t = from1_0.wrapping_add(m);
            let from3_0: size_t = from2_0.wrapping_add(m);
            let f0_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f2_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let f2_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f3_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from3_0) as isize);
            let f3_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real_0: libc::c_double = f0_real;
            let z0_imag: libc::c_double = f0_imag;
            let z1_real_0: libc::c_double = w1_real * f1_real - w1_imag * f1_imag;
            let z1_imag: libc::c_double = w1_real * f1_imag + w1_imag * f1_real;
            let z2_real_0: libc::c_double = w2_real * f2_real - w2_imag * f2_imag;
            let z2_imag: libc::c_double = w2_real * f2_imag + w2_imag * f2_real;
            let z3_real_0: libc::c_double = w3_real * f3_real - w3_imag * f3_imag;
            let z3_imag: libc::c_double = w3_real * f3_imag + w3_imag * f3_real;
            let t1_real_0: libc::c_double = z0_real_0 + z2_real_0;
            let t1_imag: libc::c_double = z0_imag + z2_imag;
            let t2_real_0: libc::c_double = z1_real_0 + z3_real_0;
            let t2_imag: libc::c_double = z1_imag + z3_imag;
            let t3_real_0: libc::c_double = z0_real_0 - z2_real_0;
            let t3_imag: libc::c_double = z0_imag - z2_imag;
            let t4_real_0: libc::c_double = -(z1_real_0 - z3_real_0);
            let t4_imag: libc::c_double = -(z1_imag - z3_imag);
            let x0_real_0: libc::c_double = t1_real_0 + t2_real_0;
            let x0_imag: libc::c_double = t1_imag + t2_imag;
            let x1_real_0: libc::c_double = t3_real_0 - t4_imag;
            let x1_imag_0: libc::c_double = t3_imag + t4_real_0;
            let x2_real_0: libc::c_double = t1_real_0 - t2_real_0;
            let x2_imag: libc::c_double = t1_imag - t2_imag;
            let x3_real: libc::c_double = t3_real_0 + t4_imag;
            let x3_imag: libc::c_double = t3_imag - t4_real_0;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to2_0: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(product_1)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add(k1.wrapping_mul(product))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to3: size_t = to2_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x1_imag_0;
            *out.offset(ostride.wrapping_mul(to3) as isize) = x2_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x2_imag;
            *out.offset(ostride.wrapping_mul(to2_0) as isize) = x3_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x3_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let from2_1: size_t = from1_1.wrapping_add(m);
        let from3_1: size_t = from2_1.wrapping_add(m);
        let x0: libc::c_double = *in_0.offset(istride.wrapping_mul(from0_1) as isize);
        let x1: libc::c_double = *in_0.offset(istride.wrapping_mul(from1_1) as isize);
        let x2: libc::c_double = *in_0.offset(istride.wrapping_mul(from2_1) as isize);
        let x3: libc::c_double = *in_0.offset(istride.wrapping_mul(from3_1) as isize);
        let t1: libc::c_double = 1.0f64 / sqrt(2.0f64) * (x1 - x3);
        let t2: libc::c_double = 1.0f64 / sqrt(2.0f64) * (x1 + x3);
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = x0 + t1;
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -x2 - t2;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = x0 - t1;
        *out
            .offset(
                ostride
                    .wrapping_mul(to1_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x2 - t2;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_float_pass_5(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex_float,
    mut twiddle2: *const gsl_complex_float,
    mut twiddle3: *const gsl_complex_float,
    mut twiddle4: *const gsl_complex_float,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 5 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let sina: libc::c_float = sin(2.0f64 * 3.14159265358979323846f64 / 5.0f64)
        as libc::c_float;
    let sinb: libc::c_float = sin(2.0f64 * 3.14159265358979323846f64 / 10.0f64)
        as libc::c_float;
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let from2: size_t = from1.wrapping_add(m);
        let from3: size_t = from2.wrapping_add(m);
        let from4: size_t = from3.wrapping_add(m);
        let z0_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z2_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let z3_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from3) as isize);
        let z4_real: libc::c_float = *in_0.offset(istride.wrapping_mul(from4) as isize);
        let t1_real: libc::c_float = z1_real + z4_real;
        let t2_real: libc::c_float = z2_real + z3_real;
        let t3_real: libc::c_float = z1_real - z4_real;
        let t4_real: libc::c_float = z2_real - z3_real;
        let t5_real: libc::c_float = t1_real + t2_real;
        let t6_real: libc::c_float = (sqrt(5.0f64) / 4.0f64
            * (t1_real - t2_real) as libc::c_double) as libc::c_float;
        let t7_real: libc::c_float = (z0_real as libc::c_double
            - t5_real as libc::c_double / 4.0f64) as libc::c_float;
        let t8_real: libc::c_float = t7_real + t6_real;
        let t9_real: libc::c_float = t7_real - t6_real;
        let t10_real: libc::c_float = -sina * t3_real - sinb * t4_real;
        let t11_real: libc::c_float = -sinb * t3_real + sina * t4_real;
        let x0_real: libc::c_float = z0_real + t5_real;
        let x1_real: libc::c_float = t8_real;
        let x1_imag: libc::c_float = t10_real;
        let x2_real: libc::c_float = t9_real;
        let x2_imag: libc::c_float = t11_real;
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to2: size_t = to1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out
            .offset(
                ostride.wrapping_mul(to1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x1_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
        *out
            .offset(
                ostride.wrapping_mul(to2.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x2_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_float = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_float = -(*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_float = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_float = -(*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w3_real: libc::c_float = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w3_imag: libc::c_float = -(*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w4_real: libc::c_float = (*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w4_imag: libc::c_float = -(*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let from2_0: size_t = from1_0.wrapping_add(m);
            let from3_0: size_t = from2_0.wrapping_add(m);
            let from4_0: size_t = from3_0.wrapping_add(m);
            let f0_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f2_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let f2_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f3_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from3_0) as isize);
            let f3_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f4_real: libc::c_float = *in_0
                .offset(istride.wrapping_mul(from4_0) as isize);
            let f4_imag: libc::c_float = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from4_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real_0: libc::c_float = f0_real;
            let z0_imag: libc::c_float = f0_imag;
            let z1_real_0: libc::c_float = w1_real * f1_real - w1_imag * f1_imag;
            let z1_imag: libc::c_float = w1_real * f1_imag + w1_imag * f1_real;
            let z2_real_0: libc::c_float = w2_real * f2_real - w2_imag * f2_imag;
            let z2_imag: libc::c_float = w2_real * f2_imag + w2_imag * f2_real;
            let z3_real_0: libc::c_float = w3_real * f3_real - w3_imag * f3_imag;
            let z3_imag: libc::c_float = w3_real * f3_imag + w3_imag * f3_real;
            let z4_real_0: libc::c_float = w4_real * f4_real - w4_imag * f4_imag;
            let z4_imag: libc::c_float = w4_real * f4_imag + w4_imag * f4_real;
            let t1_real_0: libc::c_float = z1_real_0 + z4_real_0;
            let t1_imag: libc::c_float = z1_imag + z4_imag;
            let t2_real_0: libc::c_float = z2_real_0 + z3_real_0;
            let t2_imag: libc::c_float = z2_imag + z3_imag;
            let t3_real_0: libc::c_float = z1_real_0 - z4_real_0;
            let t3_imag: libc::c_float = z1_imag - z4_imag;
            let t4_real_0: libc::c_float = z2_real_0 - z3_real_0;
            let t4_imag: libc::c_float = z2_imag - z3_imag;
            let t5_real_0: libc::c_float = t1_real_0 + t2_real_0;
            let t5_imag: libc::c_float = t1_imag + t2_imag;
            let t6_real_0: libc::c_float = (sqrt(5.0f64) / 4.0f64
                * (t1_real_0 - t2_real_0) as libc::c_double) as libc::c_float;
            let t6_imag: libc::c_float = (sqrt(5.0f64) / 4.0f64
                * (t1_imag - t2_imag) as libc::c_double) as libc::c_float;
            let t7_real_0: libc::c_float = (z0_real_0 as libc::c_double
                - t5_real_0 as libc::c_double / 4.0f64) as libc::c_float;
            let t7_imag: libc::c_float = (z0_imag as libc::c_double
                - t5_imag as libc::c_double / 4.0f64) as libc::c_float;
            let t8_real_0: libc::c_float = t7_real_0 + t6_real_0;
            let t8_imag: libc::c_float = t7_imag + t6_imag;
            let t9_real_0: libc::c_float = t7_real_0 - t6_real_0;
            let t9_imag: libc::c_float = t7_imag - t6_imag;
            let t10_real_0: libc::c_float = -sina * t3_real_0 - sinb * t4_real_0;
            let t10_imag: libc::c_float = -sina * t3_imag - sinb * t4_imag;
            let t11_real_0: libc::c_float = -sinb * t3_real_0 + sina * t4_real_0;
            let t11_imag: libc::c_float = -sinb * t3_imag + sina * t4_imag;
            let x0_real_0: libc::c_float = z0_real_0 + t5_real_0;
            let x0_imag: libc::c_float = z0_imag + t5_imag;
            let x1_real_0: libc::c_float = t8_real_0 - t10_imag;
            let x1_imag_0: libc::c_float = t8_imag + t10_real_0;
            let x2_real_0: libc::c_float = t9_real_0 - t11_imag;
            let x2_imag_0: libc::c_float = t9_imag + t11_real_0;
            let x3_real: libc::c_float = t9_real_0 + t11_imag;
            let x3_imag: libc::c_float = t9_imag - t11_real_0;
            let x4_real: libc::c_float = t8_real_0 + t10_imag;
            let x4_imag: libc::c_float = t8_imag - t10_real_0;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to2_0: size_t = to1_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to3: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(product_1)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add(k1.wrapping_mul(product))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to4: size_t = to3
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x1_imag_0;
            *out.offset(ostride.wrapping_mul(to2_0) as isize) = x2_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x2_imag_0;
            *out.offset(ostride.wrapping_mul(to3) as isize) = x4_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x4_imag;
            *out.offset(ostride.wrapping_mul(to4) as isize) = x3_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to4.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x3_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let from2_1: size_t = from1_1.wrapping_add(m);
        let from3_1: size_t = from2_1.wrapping_add(m);
        let from4_1: size_t = from3_1.wrapping_add(m);
        let z0_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z1_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let z2_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from2_1) as isize);
        let z3_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from3_1) as isize);
        let z4_real_1: libc::c_float = *in_0
            .offset(istride.wrapping_mul(from4_1) as isize);
        let t1: libc::c_float = z1_real_1 - z4_real_1;
        let t2: libc::c_float = z1_real_1 + z4_real_1;
        let t3: libc::c_float = z2_real_1 - z3_real_1;
        let t4: libc::c_float = z2_real_1 + z3_real_1;
        let t5: libc::c_float = t1 - t3;
        let t6: libc::c_float = (z0_real_1 as libc::c_double
            + t5 as libc::c_double / 4.0f64) as libc::c_float;
        let t7: libc::c_float = (sqrt(5.0f64) / 4.0f64 * (t1 + t3) as libc::c_double)
            as libc::c_float;
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        let to2_1: size_t = to1_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = t6 + t7;
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -sinb * t2 - sina * t4;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = t6 - t7;
        *out
            .offset(
                ostride
                    .wrapping_mul(to1_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -sina * t2 + sinb * t4;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = z0_real_1 - t5;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_pass_5(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    product: size_t,
    n: size_t,
    mut twiddle1: *const gsl_complex,
    mut twiddle2: *const gsl_complex,
    mut twiddle3: *const gsl_complex,
    mut twiddle4: *const gsl_complex,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let factor: size_t = 5 as libc::c_int as size_t;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let sina: libc::c_double = sin(2.0f64 * 3.14159265358979323846f64 / 5.0f64);
    let sinb: libc::c_double = sin(2.0f64 * 3.14159265358979323846f64 / 10.0f64);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0: size_t = k1.wrapping_mul(product_1);
        let from1: size_t = from0.wrapping_add(m);
        let from2: size_t = from1.wrapping_add(m);
        let from3: size_t = from2.wrapping_add(m);
        let from4: size_t = from3.wrapping_add(m);
        let z0_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from0) as isize);
        let z1_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from1) as isize);
        let z2_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from2) as isize);
        let z3_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from3) as isize);
        let z4_real: libc::c_double = *in_0.offset(istride.wrapping_mul(from4) as isize);
        let t1_real: libc::c_double = z1_real + z4_real;
        let t2_real: libc::c_double = z2_real + z3_real;
        let t3_real: libc::c_double = z1_real - z4_real;
        let t4_real: libc::c_double = z2_real - z3_real;
        let t5_real: libc::c_double = t1_real + t2_real;
        let t6_real: libc::c_double = sqrt(5.0f64) / 4.0f64 * (t1_real - t2_real);
        let t7_real: libc::c_double = z0_real - t5_real / 4.0f64;
        let t8_real: libc::c_double = t7_real + t6_real;
        let t9_real: libc::c_double = t7_real - t6_real;
        let t10_real: libc::c_double = -sina * t3_real - sinb * t4_real;
        let t11_real: libc::c_double = -sinb * t3_real + sina * t4_real;
        let x0_real: libc::c_double = z0_real + t5_real;
        let x1_real: libc::c_double = t8_real;
        let x1_imag: libc::c_double = t10_real;
        let x2_real: libc::c_double = t9_real;
        let x2_imag: libc::c_double = t11_real;
        let to0: size_t = product.wrapping_mul(k1);
        let to1: size_t = to0
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to2: size_t = to1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0) as isize) = x0_real;
        *out.offset(ostride.wrapping_mul(to1) as isize) = x1_real;
        *out
            .offset(
                ostride.wrapping_mul(to1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x1_imag;
        *out.offset(ostride.wrapping_mul(to2) as isize) = x2_real;
        *out
            .offset(
                ostride.wrapping_mul(to2.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = x2_imag;
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let w1_real: libc::c_double = (*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w1_imag: libc::c_double = -(*twiddle1
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w2_real: libc::c_double = (*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w2_imag: libc::c_double = -(*twiddle2
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w3_real: libc::c_double = (*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w3_imag: libc::c_double = -(*twiddle3
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        let w4_real: libc::c_double = (*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[0 as libc::c_int as usize];
        let w4_imag: libc::c_double = -(*twiddle4
            .offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .dat[1 as libc::c_int as usize];
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let from0_0: size_t = k1
                .wrapping_mul(product_1)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let from1_0: size_t = from0_0.wrapping_add(m);
            let from2_0: size_t = from1_0.wrapping_add(m);
            let from3_0: size_t = from2_0.wrapping_add(m);
            let from4_0: size_t = from3_0.wrapping_add(m);
            let f0_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from0_0) as isize);
            let f0_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f1_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from1_0) as isize);
            let f1_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f2_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from2_0) as isize);
            let f2_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f3_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from3_0) as isize);
            let f3_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from3_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let f4_real: libc::c_double = *in_0
                .offset(istride.wrapping_mul(from4_0) as isize);
            let f4_imag: libc::c_double = *in_0
                .offset(
                    istride
                        .wrapping_mul(
                            from4_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            let z0_real_0: libc::c_double = f0_real;
            let z0_imag: libc::c_double = f0_imag;
            let z1_real_0: libc::c_double = w1_real * f1_real - w1_imag * f1_imag;
            let z1_imag: libc::c_double = w1_real * f1_imag + w1_imag * f1_real;
            let z2_real_0: libc::c_double = w2_real * f2_real - w2_imag * f2_imag;
            let z2_imag: libc::c_double = w2_real * f2_imag + w2_imag * f2_real;
            let z3_real_0: libc::c_double = w3_real * f3_real - w3_imag * f3_imag;
            let z3_imag: libc::c_double = w3_real * f3_imag + w3_imag * f3_real;
            let z4_real_0: libc::c_double = w4_real * f4_real - w4_imag * f4_imag;
            let z4_imag: libc::c_double = w4_real * f4_imag + w4_imag * f4_real;
            let t1_real_0: libc::c_double = z1_real_0 + z4_real_0;
            let t1_imag: libc::c_double = z1_imag + z4_imag;
            let t2_real_0: libc::c_double = z2_real_0 + z3_real_0;
            let t2_imag: libc::c_double = z2_imag + z3_imag;
            let t3_real_0: libc::c_double = z1_real_0 - z4_real_0;
            let t3_imag: libc::c_double = z1_imag - z4_imag;
            let t4_real_0: libc::c_double = z2_real_0 - z3_real_0;
            let t4_imag: libc::c_double = z2_imag - z3_imag;
            let t5_real_0: libc::c_double = t1_real_0 + t2_real_0;
            let t5_imag: libc::c_double = t1_imag + t2_imag;
            let t6_real_0: libc::c_double = sqrt(5.0f64) / 4.0f64
                * (t1_real_0 - t2_real_0);
            let t6_imag: libc::c_double = sqrt(5.0f64) / 4.0f64 * (t1_imag - t2_imag);
            let t7_real_0: libc::c_double = z0_real_0 - t5_real_0 / 4.0f64;
            let t7_imag: libc::c_double = z0_imag - t5_imag / 4.0f64;
            let t8_real_0: libc::c_double = t7_real_0 + t6_real_0;
            let t8_imag: libc::c_double = t7_imag + t6_imag;
            let t9_real_0: libc::c_double = t7_real_0 - t6_real_0;
            let t9_imag: libc::c_double = t7_imag - t6_imag;
            let t10_real_0: libc::c_double = -sina * t3_real_0 - sinb * t4_real_0;
            let t10_imag: libc::c_double = -sina * t3_imag - sinb * t4_imag;
            let t11_real_0: libc::c_double = -sinb * t3_real_0 + sina * t4_real_0;
            let t11_imag: libc::c_double = -sinb * t3_imag + sina * t4_imag;
            let x0_real_0: libc::c_double = z0_real_0 + t5_real_0;
            let x0_imag: libc::c_double = z0_imag + t5_imag;
            let x1_real_0: libc::c_double = t8_real_0 - t10_imag;
            let x1_imag_0: libc::c_double = t8_imag + t10_real_0;
            let x2_real_0: libc::c_double = t9_real_0 - t11_imag;
            let x2_imag_0: libc::c_double = t9_imag + t11_real_0;
            let x3_real: libc::c_double = t9_real_0 + t11_imag;
            let x3_imag: libc::c_double = t9_imag - t11_real_0;
            let x4_real: libc::c_double = t8_real_0 + t10_imag;
            let x4_imag: libc::c_double = t8_imag - t10_real_0;
            let to0_0: size_t = k1
                .wrapping_mul(product)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to1_0: size_t = to0_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to2_0: size_t = to1_0
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            let to3: size_t = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(product_1)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                .wrapping_add(k1.wrapping_mul(product))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let to4: size_t = to3
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1),
                );
            *out.offset(ostride.wrapping_mul(to0_0) as isize) = x0_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x0_imag;
            *out.offset(ostride.wrapping_mul(to1_0) as isize) = x1_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to1_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x1_imag_0;
            *out.offset(ostride.wrapping_mul(to2_0) as isize) = x2_real_0;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to2_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = x2_imag_0;
            *out.offset(ostride.wrapping_mul(to3) as isize) = x4_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x4_imag;
            *out.offset(ostride.wrapping_mul(to4) as isize) = x3_real;
            *out
                .offset(
                    ostride
                        .wrapping_mul(
                            to4.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) = -x3_imag;
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let from0_1: size_t = k1
            .wrapping_mul(product_1)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let from1_1: size_t = from0_1.wrapping_add(m);
        let from2_1: size_t = from1_1.wrapping_add(m);
        let from3_1: size_t = from2_1.wrapping_add(m);
        let from4_1: size_t = from3_1.wrapping_add(m);
        let z0_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from0_1) as isize);
        let z1_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from1_1) as isize);
        let z2_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from2_1) as isize);
        let z3_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from3_1) as isize);
        let z4_real_1: libc::c_double = *in_0
            .offset(istride.wrapping_mul(from4_1) as isize);
        let t1: libc::c_double = z1_real_1 - z4_real_1;
        let t2: libc::c_double = z1_real_1 + z4_real_1;
        let t3: libc::c_double = z2_real_1 - z3_real_1;
        let t4: libc::c_double = z2_real_1 + z3_real_1;
        let t5: libc::c_double = t1 - t3;
        let t6: libc::c_double = z0_real_1 + t5 / 4.0f64;
        let t7: libc::c_double = sqrt(5.0f64) / 4.0f64 * (t1 + t3);
        let to0_1: size_t = k1
            .wrapping_mul(product)
            .wrapping_add(product_1)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let to1_1: size_t = to0_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        let to2_1: size_t = to1_1
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(product_1));
        *out.offset(ostride.wrapping_mul(to0_1) as isize) = t6 + t7;
        *out
            .offset(
                ostride
                    .wrapping_mul(to0_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -sinb * t2 - sina * t4;
        *out.offset(ostride.wrapping_mul(to1_1) as isize) = t6 - t7;
        *out
            .offset(
                ostride
                    .wrapping_mul(to1_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = -sina * t2 + sinb * t4;
        *out.offset(ostride.wrapping_mul(to2_1) as isize) = z0_real_1 - t5;
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_float_pass_n(
    mut in_0: *const libc::c_float,
    istride: size_t,
    mut out: *mut libc::c_float,
    ostride: size_t,
    factor: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex_float,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let mut e1: size_t = 0;
    let mut e2: size_t = 0;
    let d_theta: libc::c_double = 2.0f64 * 3.14159265358979323846f64
        / factor as libc::c_double;
    let cos_d_theta: libc::c_float = cos(d_theta) as libc::c_float;
    let sin_d_theta: libc::c_float = sin(d_theta) as libc::c_float;
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let mut dw_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut dw_imag: libc::c_float = 0.0f64 as libc::c_float;
        e1 = 0 as libc::c_int as size_t;
        while e1 <= factor.wrapping_sub(e1) {
            let mut sum_real: libc::c_float = 0.0f64 as libc::c_float;
            let mut sum_imag: libc::c_float = 0.0f64 as libc::c_float;
            let mut w_real: libc::c_float = 1.0f64 as libc::c_float;
            let mut w_imag: libc::c_float = 0.0f64 as libc::c_float;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut tmp_real: libc::c_float = dw_real * cos_d_theta
                    + dw_imag * sin_d_theta;
                let mut tmp_imag: libc::c_float = -dw_real * sin_d_theta
                    + dw_imag * cos_d_theta;
                dw_real = tmp_real;
                dw_imag = tmp_imag;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2 < factor {
                let mut z_real: libc::c_float = *in_0
                    .offset(
                        istride
                            .wrapping_mul(
                                k1.wrapping_mul(product_1).wrapping_add(e2.wrapping_mul(m)),
                            ) as isize,
                    );
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_0: libc::c_float = dw_real * w_real
                        - dw_imag * w_imag;
                    let mut tmp_imag_0: libc::c_float = dw_real * w_imag
                        + dw_imag * w_real;
                    w_real = tmp_real_0;
                    w_imag = tmp_imag_0;
                }
                sum_real += w_real * z_real;
                sum_imag += w_imag * z_real;
                e2 = e2.wrapping_add(1);
                e2;
            }
            if e1 == 0 as libc::c_int as libc::c_ulong {
                let to0: size_t = product.wrapping_mul(k1);
                *out.offset(ostride.wrapping_mul(to0) as isize) = sum_real;
            } else if e1 < factor.wrapping_sub(e1) {
                let to0_0: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                *out.offset(ostride.wrapping_mul(to0_0) as isize) = sum_real;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = sum_imag;
            } else if e1 == factor.wrapping_sub(e1) {
                let to0_1: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                *out.offset(ostride.wrapping_mul(to0_1) as isize) = sum_real;
            }
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let mut dw_real_0: libc::c_float = 1.0f64 as libc::c_float;
            let mut dw_imag_0: libc::c_float = 0.0f64 as libc::c_float;
            e1 = 0 as libc::c_int as size_t;
            while e1 < factor {
                let mut sum_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut sum_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut w_real_0: libc::c_float = 1.0f64 as libc::c_float;
                let mut w_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                if e1 > 0 as libc::c_int as libc::c_ulong {
                    let tmp_real_1: libc::c_float = dw_real_0 * cos_d_theta
                        + dw_imag_0 * sin_d_theta;
                    let tmp_imag_1: libc::c_float = -dw_real_0 * sin_d_theta
                        + dw_imag_0 * cos_d_theta;
                    dw_real_0 = tmp_real_1;
                    dw_imag_0 = tmp_imag_1;
                }
                e2 = 0 as libc::c_int as size_t;
                while e2 < factor {
                    let mut tskip: libc::c_int = product_1
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                    let from0: size_t = k1
                        .wrapping_mul(product_1)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                        )
                        .wrapping_add(e2.wrapping_mul(m))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    let mut tw_real: libc::c_float = 0.;
                    let mut tw_imag: libc::c_float = 0.;
                    let mut z_real_0: libc::c_float = 0.;
                    let mut z_imag: libc::c_float = 0.;
                    if e2 == 0 as libc::c_int as libc::c_ulong {
                        tw_real = 1.0f64 as libc::c_float;
                        tw_imag = 0.0f64 as libc::c_float;
                    } else {
                        let t_index: size_t = k
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                e2
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(tskip as libc::c_ulong),
                            );
                        tw_real = (*twiddle.offset(t_index as isize))
                            .dat[0 as libc::c_int as usize];
                        tw_imag = -(*twiddle.offset(t_index as isize))
                            .dat[1 as libc::c_int as usize];
                    }
                    let f0_real: libc::c_float = *in_0
                        .offset(istride.wrapping_mul(from0) as isize);
                    let f0_imag: libc::c_float = *in_0
                        .offset(
                            istride
                                .wrapping_mul(
                                    from0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
                    z_real_0 = tw_real * f0_real - tw_imag * f0_imag;
                    z_imag = tw_real * f0_imag + tw_imag * f0_real;
                    if e2 > 0 as libc::c_int as libc::c_ulong {
                        let tmp_real_2: libc::c_float = dw_real_0 * w_real_0
                            - dw_imag_0 * w_imag_0;
                        let tmp_imag_2: libc::c_float = dw_real_0 * w_imag_0
                            + dw_imag_0 * w_real_0;
                        w_real_0 = tmp_real_2;
                        w_imag_0 = tmp_imag_2;
                    }
                    sum_real_0 += w_real_0 * z_real_0 - w_imag_0 * z_imag;
                    sum_imag_0 += w_real_0 * z_imag + w_imag_0 * z_real_0;
                    e2 = e2.wrapping_add(1);
                    e2;
                }
                if e1 < factor.wrapping_sub(e1) {
                    let to0_2: size_t = k1
                        .wrapping_mul(product)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e1)
                                .wrapping_mul(product_1),
                        )
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                        );
                    *out.offset(ostride.wrapping_mul(to0_2) as isize) = sum_real_0;
                    *out
                        .offset(
                            ostride
                                .wrapping_mul(
                                    to0_2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        ) = sum_imag_0;
                } else {
                    let to0_3: size_t = k1
                        .wrapping_mul(product)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(factor.wrapping_sub(e1))
                                .wrapping_mul(product_1),
                        )
                        .wrapping_sub(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                        );
                    *out.offset(ostride.wrapping_mul(to0_3) as isize) = sum_real_0;
                    *out
                        .offset(
                            ostride
                                .wrapping_mul(
                                    to0_3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        ) = -sum_imag_0;
                }
                e1 = e1.wrapping_add(1);
                e1;
            }
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let mut tw_arg: libc::c_double = 3.14159265358979323846f64
        / factor as libc::c_double;
    let mut cos_tw_arg: libc::c_float = cos(tw_arg) as libc::c_float;
    let mut sin_tw_arg: libc::c_float = -sin(tw_arg) as libc::c_float;
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let mut dw_real_1: libc::c_float = 1.0f64 as libc::c_float;
        let mut dw_imag_1: libc::c_float = 0.0f64 as libc::c_float;
        e1 = 0 as libc::c_int as size_t;
        while e1 < factor {
            let mut z_real_1: libc::c_float = 0.;
            let mut z_imag_0: libc::c_float = 0.;
            let mut sum_real_1: libc::c_float = 0.0f64 as libc::c_float;
            let mut sum_imag_1: libc::c_float = 0.0f64 as libc::c_float;
            let mut w_real_1: libc::c_float = 1.0f64 as libc::c_float;
            let mut w_imag_1: libc::c_float = 0.0f64 as libc::c_float;
            let mut tw_real_0: libc::c_float = 1.0f64 as libc::c_float;
            let mut tw_imag_0: libc::c_float = 0.0f64 as libc::c_float;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut t_real: libc::c_float = dw_real_1 * cos_d_theta
                    + dw_imag_1 * sin_d_theta;
                let mut t_imag: libc::c_float = -dw_real_1 * sin_d_theta
                    + dw_imag_1 * cos_d_theta;
                dw_real_1 = t_real;
                dw_imag_1 = t_imag;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2 < factor {
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_3: libc::c_float = tw_real_0 * cos_tw_arg
                        - tw_imag_0 * sin_tw_arg;
                    let mut tmp_imag_3: libc::c_float = tw_real_0 * sin_tw_arg
                        + tw_imag_0 * cos_tw_arg;
                    tw_real_0 = tmp_real_3;
                    tw_imag_0 = tmp_imag_3;
                }
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_4: libc::c_float = dw_real_1 * w_real_1
                        - dw_imag_1 * w_imag_1;
                    let mut tmp_imag_4: libc::c_float = dw_real_1 * w_imag_1
                        + dw_imag_1 * w_real_1;
                    w_real_1 = tmp_real_4;
                    w_imag_1 = tmp_imag_4;
                }
                let from0_0: size_t = k1
                    .wrapping_mul(product_1)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                    .wrapping_add(e2.wrapping_mul(m))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let f0_real_0: libc::c_float = *in_0
                    .offset(istride.wrapping_mul(from0_0) as isize);
                z_real_1 = tw_real_0 * f0_real_0;
                z_imag_0 = tw_imag_0 * f0_real_0;
                sum_real_1 += w_real_1 * z_real_1 - w_imag_1 * z_imag_0;
                sum_imag_1 += w_real_1 * z_imag_0 + w_imag_1 * z_real_1;
                e2 = e2.wrapping_add(1);
                e2;
            }
            if e1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < factor.wrapping_sub(e1)
            {
                let to0_4: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k));
                *out.offset(ostride.wrapping_mul(to0_4) as isize) = sum_real_1;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0_4.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = sum_imag_1;
            } else if e1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                == factor.wrapping_sub(e1)
            {
                let to0_5: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k));
                *out.offset(ostride.wrapping_mul(to0_5) as isize) = sum_real_1;
            } else {
                let to0_6: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(factor.wrapping_sub(e1))
                            .wrapping_mul(product_1),
                    )
                    .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k));
                *out.offset(ostride.wrapping_mul(to0_6) as isize) = sum_real_1;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0_6.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = -sum_imag_1;
            }
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
}
unsafe extern "C" fn fft_real_pass_n(
    mut in_0: *const libc::c_double,
    istride: size_t,
    mut out: *mut libc::c_double,
    ostride: size_t,
    factor: size_t,
    product: size_t,
    n: size_t,
    mut twiddle: *const gsl_complex,
) {
    let mut k: size_t = 0;
    let mut k1: size_t = 0;
    let m: size_t = n.wrapping_div(factor);
    let q: size_t = n.wrapping_div(product);
    let product_1: size_t = product.wrapping_div(factor);
    let mut e1: size_t = 0;
    let mut e2: size_t = 0;
    let d_theta: libc::c_double = 2.0f64 * 3.14159265358979323846f64
        / factor as libc::c_double;
    let cos_d_theta: libc::c_double = cos(d_theta);
    let sin_d_theta: libc::c_double = sin(d_theta);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let mut dw_real: libc::c_double = 1.0f64;
        let mut dw_imag: libc::c_double = 0.0f64;
        e1 = 0 as libc::c_int as size_t;
        while e1 <= factor.wrapping_sub(e1) {
            let mut sum_real: libc::c_double = 0.0f64;
            let mut sum_imag: libc::c_double = 0.0f64;
            let mut w_real: libc::c_double = 1.0f64;
            let mut w_imag: libc::c_double = 0.0f64;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut tmp_real: libc::c_double = dw_real * cos_d_theta
                    + dw_imag * sin_d_theta;
                let mut tmp_imag: libc::c_double = -dw_real * sin_d_theta
                    + dw_imag * cos_d_theta;
                dw_real = tmp_real;
                dw_imag = tmp_imag;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2 < factor {
                let mut z_real: libc::c_double = *in_0
                    .offset(
                        istride
                            .wrapping_mul(
                                k1.wrapping_mul(product_1).wrapping_add(e2.wrapping_mul(m)),
                            ) as isize,
                    );
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_0: libc::c_double = dw_real * w_real
                        - dw_imag * w_imag;
                    let mut tmp_imag_0: libc::c_double = dw_real * w_imag
                        + dw_imag * w_real;
                    w_real = tmp_real_0;
                    w_imag = tmp_imag_0;
                }
                sum_real += w_real * z_real;
                sum_imag += w_imag * z_real;
                e2 = e2.wrapping_add(1);
                e2;
            }
            if e1 == 0 as libc::c_int as libc::c_ulong {
                let to0: size_t = product.wrapping_mul(k1);
                *out.offset(ostride.wrapping_mul(to0) as isize) = sum_real;
            } else if e1 < factor.wrapping_sub(e1) {
                let to0_0: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                *out.offset(ostride.wrapping_mul(to0_0) as isize) = sum_real;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = sum_imag;
            } else if e1 == factor.wrapping_sub(e1) {
                let to0_1: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                *out.offset(ostride.wrapping_mul(to0_1) as isize) = sum_real;
            }
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
    if product_1 == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    k = 1 as libc::c_int as size_t;
    while k
        < product_1
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        k1 = 0 as libc::c_int as size_t;
        while k1 < q {
            let mut dw_real_0: libc::c_double = 1.0f64;
            let mut dw_imag_0: libc::c_double = 0.0f64;
            e1 = 0 as libc::c_int as size_t;
            while e1 < factor {
                let mut sum_real_0: libc::c_double = 0.0f64;
                let mut sum_imag_0: libc::c_double = 0.0f64;
                let mut w_real_0: libc::c_double = 1.0f64;
                let mut w_imag_0: libc::c_double = 0.0f64;
                if e1 > 0 as libc::c_int as libc::c_ulong {
                    let tmp_real_1: libc::c_double = dw_real_0 * cos_d_theta
                        + dw_imag_0 * sin_d_theta;
                    let tmp_imag_1: libc::c_double = -dw_real_0 * sin_d_theta
                        + dw_imag_0 * cos_d_theta;
                    dw_real_0 = tmp_real_1;
                    dw_imag_0 = tmp_imag_1;
                }
                e2 = 0 as libc::c_int as size_t;
                while e2 < factor {
                    let mut tskip: libc::c_int = product_1
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                    let from0: size_t = k1
                        .wrapping_mul(product_1)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                        )
                        .wrapping_add(e2.wrapping_mul(m))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    let mut tw_real: libc::c_double = 0.;
                    let mut tw_imag: libc::c_double = 0.;
                    let mut z_real_0: libc::c_double = 0.;
                    let mut z_imag: libc::c_double = 0.;
                    if e2 == 0 as libc::c_int as libc::c_ulong {
                        tw_real = 1.0f64;
                        tw_imag = 0.0f64;
                    } else {
                        let t_index: size_t = k
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                e2
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(tskip as libc::c_ulong),
                            );
                        tw_real = (*twiddle.offset(t_index as isize))
                            .dat[0 as libc::c_int as usize];
                        tw_imag = -(*twiddle.offset(t_index as isize))
                            .dat[1 as libc::c_int as usize];
                    }
                    let f0_real: libc::c_double = *in_0
                        .offset(istride.wrapping_mul(from0) as isize);
                    let f0_imag: libc::c_double = *in_0
                        .offset(
                            istride
                                .wrapping_mul(
                                    from0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
                    z_real_0 = tw_real * f0_real - tw_imag * f0_imag;
                    z_imag = tw_real * f0_imag + tw_imag * f0_real;
                    if e2 > 0 as libc::c_int as libc::c_ulong {
                        let tmp_real_2: libc::c_double = dw_real_0 * w_real_0
                            - dw_imag_0 * w_imag_0;
                        let tmp_imag_2: libc::c_double = dw_real_0 * w_imag_0
                            + dw_imag_0 * w_real_0;
                        w_real_0 = tmp_real_2;
                        w_imag_0 = tmp_imag_2;
                    }
                    sum_real_0 += w_real_0 * z_real_0 - w_imag_0 * z_imag;
                    sum_imag_0 += w_real_0 * z_imag + w_imag_0 * z_real_0;
                    e2 = e2.wrapping_add(1);
                    e2;
                }
                if e1 < factor.wrapping_sub(e1) {
                    let to0_2: size_t = k1
                        .wrapping_mul(product)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(e1)
                                .wrapping_mul(product_1),
                        )
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                        );
                    *out.offset(ostride.wrapping_mul(to0_2) as isize) = sum_real_0;
                    *out
                        .offset(
                            ostride
                                .wrapping_mul(
                                    to0_2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        ) = sum_imag_0;
                } else {
                    let to0_3: size_t = k1
                        .wrapping_mul(product)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(factor.wrapping_sub(e1))
                                .wrapping_mul(product_1),
                        )
                        .wrapping_sub(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(k),
                        );
                    *out.offset(ostride.wrapping_mul(to0_3) as isize) = sum_real_0;
                    *out
                        .offset(
                            ostride
                                .wrapping_mul(
                                    to0_3.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        ) = -sum_imag_0;
                }
                e1 = e1.wrapping_add(1);
                e1;
            }
            k1 = k1.wrapping_add(1);
            k1;
        }
        k = k.wrapping_add(1);
        k;
    }
    if product_1.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let mut tw_arg: libc::c_double = 3.14159265358979323846f64
        / factor as libc::c_double;
    let mut cos_tw_arg: libc::c_double = cos(tw_arg);
    let mut sin_tw_arg: libc::c_double = -sin(tw_arg);
    k1 = 0 as libc::c_int as size_t;
    while k1 < q {
        let mut dw_real_1: libc::c_double = 1.0f64;
        let mut dw_imag_1: libc::c_double = 0.0f64;
        e1 = 0 as libc::c_int as size_t;
        while e1 < factor {
            let mut z_real_1: libc::c_double = 0.;
            let mut z_imag_0: libc::c_double = 0.;
            let mut sum_real_1: libc::c_double = 0.0f64;
            let mut sum_imag_1: libc::c_double = 0.0f64;
            let mut w_real_1: libc::c_double = 1.0f64;
            let mut w_imag_1: libc::c_double = 0.0f64;
            let mut tw_real_0: libc::c_double = 1.0f64;
            let mut tw_imag_0: libc::c_double = 0.0f64;
            if e1 > 0 as libc::c_int as libc::c_ulong {
                let mut t_real: libc::c_double = dw_real_1 * cos_d_theta
                    + dw_imag_1 * sin_d_theta;
                let mut t_imag: libc::c_double = -dw_real_1 * sin_d_theta
                    + dw_imag_1 * cos_d_theta;
                dw_real_1 = t_real;
                dw_imag_1 = t_imag;
            }
            e2 = 0 as libc::c_int as size_t;
            while e2 < factor {
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_3: libc::c_double = tw_real_0 * cos_tw_arg
                        - tw_imag_0 * sin_tw_arg;
                    let mut tmp_imag_3: libc::c_double = tw_real_0 * sin_tw_arg
                        + tw_imag_0 * cos_tw_arg;
                    tw_real_0 = tmp_real_3;
                    tw_imag_0 = tmp_imag_3;
                }
                if e2 > 0 as libc::c_int as libc::c_ulong {
                    let mut tmp_real_4: libc::c_double = dw_real_1 * w_real_1
                        - dw_imag_1 * w_imag_1;
                    let mut tmp_imag_4: libc::c_double = dw_real_1 * w_imag_1
                        + dw_imag_1 * w_real_1;
                    w_real_1 = tmp_real_4;
                    w_imag_1 = tmp_imag_4;
                }
                let from0_0: size_t = k1
                    .wrapping_mul(product_1)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k))
                    .wrapping_add(e2.wrapping_mul(m))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let f0_real_0: libc::c_double = *in_0
                    .offset(istride.wrapping_mul(from0_0) as isize);
                z_real_1 = tw_real_0 * f0_real_0;
                z_imag_0 = tw_imag_0 * f0_real_0;
                sum_real_1 += w_real_1 * z_real_1 - w_imag_1 * z_imag_0;
                sum_imag_1 += w_real_1 * z_imag_0 + w_imag_1 * z_real_1;
                e2 = e2.wrapping_add(1);
                e2;
            }
            if e1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < factor.wrapping_sub(e1)
            {
                let to0_4: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k));
                *out.offset(ostride.wrapping_mul(to0_4) as isize) = sum_real_1;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0_4.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = sum_imag_1;
            } else if e1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                == factor.wrapping_sub(e1)
            {
                let to0_5: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(e1)
                            .wrapping_mul(product_1),
                    )
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(k));
                *out.offset(ostride.wrapping_mul(to0_5) as isize) = sum_real_1;
            } else {
                let to0_6: size_t = k1
                    .wrapping_mul(product)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(factor.wrapping_sub(e1))
                            .wrapping_mul(product_1),
                    )
                    .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(k));
                *out.offset(ostride.wrapping_mul(to0_6) as isize) = sum_real_1;
                *out
                    .offset(
                        ostride
                            .wrapping_mul(
                                to0_6.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) = -sum_imag_1;
            }
            e1 = e1.wrapping_add(1);
            e1;
        }
        k1 = k1.wrapping_add(1);
        k1;
    }
}
pub unsafe extern "C" fn gsl_fft_real_float_radix2_transform(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut p: size_t = 0;
    let mut p_1: size_t = 0;
    let mut q: size_t = 0;
    let mut i: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./real_radix2.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    status = fft_real_float_bitreverse_order(data, stride, n, logn);
    p = 1 as libc::c_int as size_t;
    q = n;
    i = 1 as libc::c_int as size_t;
    while i <= logn {
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        p_1 = p;
        p = (2 as libc::c_int as libc::c_ulong).wrapping_mul(p);
        q = q.wrapping_div(2 as libc::c_int as libc::c_ulong);
        b = 0 as libc::c_int as size_t;
        while b < q {
            let mut t0_real: libc::c_float = *data
                .offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize)
                + *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                    );
            let mut t1_real: libc::c_float = *data
                .offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize)
                - *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                    );
            *data.offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize) = t0_real;
            *data
                .offset(
                    stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                ) = t1_real;
            b = b.wrapping_add(1);
            b;
        }
        let mut w_real: libc::c_float = 1.0f64 as libc::c_float;
        let mut w_imag: libc::c_float = 0.0f64 as libc::c_float;
        let theta: libc::c_double = -2.0f64 * 3.14159265358979323846f64
            / p as libc::c_double;
        let s: libc::c_float = sin(theta) as libc::c_float;
        let t: libc::c_float = sin(theta / 2.0f64) as libc::c_float;
        let s2: libc::c_float = (2.0f64 * t as libc::c_double * t as libc::c_double)
            as libc::c_float;
        a = 1 as libc::c_int as size_t;
        while a < p_1.wrapping_div(2 as libc::c_int as libc::c_ulong) {
            let tmp_real: libc::c_float = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_float = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = 0 as libc::c_int as size_t;
            while b < q {
                let mut z0_real: libc::c_float = *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    );
                let mut z0_imag: libc::c_float = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    );
                let mut z1_real: libc::c_float = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    );
                let mut z1_imag: libc::c_float = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    );
                let mut t0_real_0: libc::c_float = z0_real + w_real * z1_real
                    - w_imag * z1_imag;
                let mut t0_imag: libc::c_float = z0_imag + w_real * z1_imag
                    + w_imag * z1_real;
                let mut t1_real_0: libc::c_float = z0_real - w_real * z1_real
                    + w_imag * z1_imag;
                let mut t1_imag: libc::c_float = z0_imag - w_real * z1_imag
                    - w_imag * z1_real;
                *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    ) = t0_real_0;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    ) = t0_imag;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    ) = t1_real_0;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    ) = -t1_imag;
                b = b.wrapping_add(1);
                b;
            }
            a = a.wrapping_add(1);
            a;
        }
        if p_1 > 1 as libc::c_int as libc::c_ulong {
            b = 0 as libc::c_int as size_t;
            while b < q {
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b
                                    .wrapping_mul(p)
                                    .wrapping_add(p)
                                    .wrapping_sub(
                                        p_1.wrapping_div(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as isize,
                    ) *= -(1 as libc::c_int) as libc::c_float;
                b = b.wrapping_add(1);
                b;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_real_radix2_transform(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut p: size_t = 0;
    let mut p_1: size_t = 0;
    let mut q: size_t = 0;
    let mut i: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    result = fft_binary_logn(n);
    if result == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"./real_radix2.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        logn = result as size_t;
    }
    status = fft_real_bitreverse_order(data, stride, n, logn);
    p = 1 as libc::c_int as size_t;
    q = n;
    i = 1 as libc::c_int as size_t;
    while i <= logn {
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        p_1 = p;
        p = (2 as libc::c_int as libc::c_ulong).wrapping_mul(p);
        q = q.wrapping_div(2 as libc::c_int as libc::c_ulong);
        b = 0 as libc::c_int as size_t;
        while b < q {
            let mut t0_real: libc::c_double = *data
                .offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize)
                + *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                    );
            let mut t1_real: libc::c_double = *data
                .offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize)
                - *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                    );
            *data.offset(stride.wrapping_mul(b.wrapping_mul(p)) as isize) = t0_real;
            *data
                .offset(
                    stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(p_1)) as isize,
                ) = t1_real;
            b = b.wrapping_add(1);
            b;
        }
        let mut w_real: libc::c_double = 1.0f64;
        let mut w_imag: libc::c_double = 0.0f64;
        let theta: libc::c_double = -2.0f64 * 3.14159265358979323846f64
            / p as libc::c_double;
        let s: libc::c_double = sin(theta);
        let t: libc::c_double = sin(theta / 2.0f64);
        let s2: libc::c_double = 2.0f64 * t * t;
        a = 1 as libc::c_int as size_t;
        while a < p_1.wrapping_div(2 as libc::c_int as libc::c_ulong) {
            let tmp_real: libc::c_double = w_real - s * w_imag - s2 * w_real;
            let tmp_imag: libc::c_double = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            b = 0 as libc::c_int as size_t;
            while b < q {
                let mut z0_real: libc::c_double = *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    );
                let mut z0_imag: libc::c_double = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    );
                let mut z1_real: libc::c_double = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    );
                let mut z1_imag: libc::c_double = *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    );
                let mut t0_real_0: libc::c_double = z0_real + w_real * z1_real
                    - w_imag * z1_imag;
                let mut t0_imag: libc::c_double = z0_imag + w_real * z1_imag
                    + w_imag * z1_real;
                let mut t1_real_0: libc::c_double = z0_real - w_real * z1_real
                    + w_imag * z1_imag;
                let mut t1_imag: libc::c_double = z0_imag - w_real * z1_imag
                    - w_imag * z1_real;
                *data
                    .offset(
                        stride.wrapping_mul(b.wrapping_mul(p).wrapping_add(a)) as isize,
                    ) = t0_real_0;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p).wrapping_sub(a),
                            ) as isize,
                    ) = t0_imag;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_sub(a),
                            ) as isize,
                    ) = t1_real_0;
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b.wrapping_mul(p).wrapping_add(p_1).wrapping_add(a),
                            ) as isize,
                    ) = -t1_imag;
                b = b.wrapping_add(1);
                b;
            }
            a = a.wrapping_add(1);
            a;
        }
        if p_1 > 1 as libc::c_int as libc::c_ulong {
            b = 0 as libc::c_int as size_t;
            while b < q {
                *data
                    .offset(
                        stride
                            .wrapping_mul(
                                b
                                    .wrapping_mul(p)
                                    .wrapping_add(p)
                                    .wrapping_sub(
                                        p_1.wrapping_div(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as isize,
                    ) *= -(1 as libc::c_int) as libc::c_double;
                b = b.wrapping_add(1);
                b;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_real_unpack(
    mut real_coefficient: *const libc::c_double,
    mut complex_coefficient: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_unpack.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = *real_coefficient.offset(i.wrapping_mul(stride) as isize);
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0.0f64;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_fft_real_float_unpack(
    mut real_coefficient: *const libc::c_float,
    mut complex_coefficient: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./real_unpack.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = *real_coefficient.offset(i.wrapping_mul(stride) as isize);
        *complex_coefficient
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0.0f64 as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}

use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type gsl_fft_direction = libc::c_int;
pub const gsl_fft_backward: gsl_fft_direction = 1;
pub const gsl_fft_forward: gsl_fft_direction = -1;
pub unsafe extern "C" fn gsl_dft_complex_float_forward(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_float,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_dft_complex_float_transform(
        data,
        stride,
        n,
        result,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_dft_complex_forward(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_forward;
    let mut status: libc::c_int = gsl_dft_complex_transform(
        data,
        stride,
        n,
        result,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_dft_complex_backward(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_dft_complex_transform(
        data,
        stride,
        n,
        result,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_dft_complex_float_backward(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_float,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_dft_complex_float_transform(
        data,
        stride,
        n,
        result,
        sign,
    );
    return status;
}
pub unsafe extern "C" fn gsl_dft_complex_inverse(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_dft_complex_transform(
        data,
        stride,
        n,
        result,
        sign,
    );
    let norm: libc::c_double = 1.0f64 / n as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *result
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *result
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
pub unsafe extern "C" fn gsl_dft_complex_float_inverse(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_float,
) -> libc::c_int {
    let mut sign: gsl_fft_direction = gsl_fft_backward;
    let mut status: libc::c_int = gsl_dft_complex_float_transform(
        data,
        stride,
        n,
        result,
        sign,
    );
    let norm: libc::c_float = 1.0f32 / n as libc::c_float;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *result
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) *= norm;
        *result
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
pub unsafe extern "C" fn gsl_dft_complex_float_transform(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_float,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut exponent: size_t = 0;
    let d_theta: libc::c_double = 2.0f64 * sign as libc::c_int as libc::c_double
        * 3.14159265358979323846f64 / n as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut sum_real: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut sum_imag: libc::c_float = 0 as libc::c_int as libc::c_float;
        exponent = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while j < n {
            let mut theta: libc::c_double = d_theta * exponent as libc::c_double;
            let mut w_real: libc::c_float = cos(theta) as libc::c_float;
            let mut w_imag: libc::c_float = sin(theta) as libc::c_float;
            let mut data_real: libc::c_float = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                );
            let mut data_imag: libc::c_float = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            sum_real += w_real * data_real - w_imag * data_imag;
            sum_imag += w_real * data_imag + w_imag * data_real;
            exponent = exponent.wrapping_add(i).wrapping_rem(n);
            j = j.wrapping_add(1);
            j;
        }
        *result
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = sum_real;
        *result
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = sum_imag;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_dft_complex_transform(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut result: *mut libc::c_double,
    sign: gsl_fft_direction,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut exponent: size_t = 0;
    let d_theta: libc::c_double = 2.0f64 * sign as libc::c_int as libc::c_double
        * 3.14159265358979323846f64 / n as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut sum_real: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut sum_imag: libc::c_double = 0 as libc::c_int as libc::c_double;
        exponent = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while j < n {
            let mut theta: libc::c_double = d_theta * exponent as libc::c_double;
            let mut w_real: libc::c_double = cos(theta);
            let mut w_imag: libc::c_double = sin(theta);
            let mut data_real: libc::c_double = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j) as isize,
                );
            let mut data_imag: libc::c_double = *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            sum_real += w_real * data_real - w_imag * data_imag;
            sum_imag += w_real * data_imag + w_imag * data_real;
            exponent = exponent.wrapping_add(i).wrapping_rem(n);
            j = j.wrapping_add(1);
            j;
        }
        *result
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(stride).wrapping_mul(i)
                    as isize,
            ) = sum_real;
        *result
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(stride)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = sum_imag;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}

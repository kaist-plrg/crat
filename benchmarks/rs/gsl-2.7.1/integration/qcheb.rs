use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
pub unsafe extern "C" fn gsl_integration_qcheb(
    mut f: *mut gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut cheb12: *mut libc::c_double,
    mut cheb24: *mut libc::c_double,
) {
    let mut i: size_t = 0;
    let mut fval: [libc::c_double; 25] = [0.; 25];
    let mut v: [libc::c_double; 12] = [0.; 12];
    let x: [libc::c_double; 11] = [
        0.9914448613738104f64,
        0.9659258262890683f64,
        0.9238795325112868f64,
        0.8660254037844386f64,
        0.7933533402912352f64,
        0.7071067811865475f64,
        0.6087614290087206f64,
        0.5000000000000000f64,
        0.3826834323650898f64,
        0.2588190451025208f64,
        0.1305261922200516f64,
    ];
    let center: libc::c_double = 0.5f64 * (b + a);
    let half_length: libc::c_double = 0.5f64 * (b - a);
    fval[0 as libc::c_int
        as usize] = 0.5f64 * (Some(((*f).function).unwrap())).unwrap()(b, (*f).params);
    fval[12 as libc::c_int
        as usize] = (Some(((*f).function).unwrap())).unwrap()(center, (*f).params);
    fval[24 as libc::c_int
        as usize] = 0.5f64 * (Some(((*f).function).unwrap())).unwrap()(a, (*f).params);
    i = 1 as libc::c_int as size_t;
    while i < 12 as libc::c_int as libc::c_ulong {
        let j: size_t = (24 as libc::c_int as libc::c_ulong).wrapping_sub(i);
        let u: libc::c_double = half_length
            * x[i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize];
        fval[i
            as usize] = (Some(((*f).function).unwrap()))
            .unwrap()(center + u, (*f).params);
        fval[j
            as usize] = (Some(((*f).function).unwrap()))
            .unwrap()(center - u, (*f).params);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 12 as libc::c_int as libc::c_ulong {
        let j_0: size_t = (24 as libc::c_int as libc::c_ulong).wrapping_sub(i);
        v[i as usize] = fval[i as usize] - fval[j_0 as usize];
        fval[i as usize] = fval[i as usize] + fval[j_0 as usize];
        i = i.wrapping_add(1);
        i;
    }
    let alam1: libc::c_double = v[0 as libc::c_int as usize]
        - v[8 as libc::c_int as usize];
    let alam2: libc::c_double = x[5 as libc::c_int as usize]
        * (v[2 as libc::c_int as usize] - v[6 as libc::c_int as usize]
            - v[10 as libc::c_int as usize]);
    *cheb12.offset(3 as libc::c_int as isize) = alam1 + alam2;
    *cheb12.offset(9 as libc::c_int as isize) = alam1 - alam2;
    let alam1_0: libc::c_double = v[1 as libc::c_int as usize]
        - v[7 as libc::c_int as usize] - v[9 as libc::c_int as usize];
    let alam2_0: libc::c_double = v[3 as libc::c_int as usize]
        - v[5 as libc::c_int as usize] - v[11 as libc::c_int as usize];
    let alam: libc::c_double = x[2 as libc::c_int as usize] * alam1_0
        + x[8 as libc::c_int as usize] * alam2_0;
    *cheb24
        .offset(
            3 as libc::c_int as isize,
        ) = *cheb12.offset(3 as libc::c_int as isize) + alam;
    *cheb24
        .offset(
            21 as libc::c_int as isize,
        ) = *cheb12.offset(3 as libc::c_int as isize) - alam;
    let alam_0: libc::c_double = x[8 as libc::c_int as usize] * alam1_0
        - x[2 as libc::c_int as usize] * alam2_0;
    *cheb24
        .offset(
            9 as libc::c_int as isize,
        ) = *cheb12.offset(9 as libc::c_int as isize) + alam_0;
    *cheb24
        .offset(
            15 as libc::c_int as isize,
        ) = *cheb12.offset(9 as libc::c_int as isize) - alam_0;
    let part1: libc::c_double = x[3 as libc::c_int as usize]
        * v[4 as libc::c_int as usize];
    let part2: libc::c_double = x[7 as libc::c_int as usize]
        * v[8 as libc::c_int as usize];
    let part3: libc::c_double = x[5 as libc::c_int as usize]
        * v[6 as libc::c_int as usize];
    let alam1_1: libc::c_double = v[0 as libc::c_int as usize] + part1 + part2;
    let alam2_1: libc::c_double = x[1 as libc::c_int as usize]
        * v[2 as libc::c_int as usize] + part3
        + x[9 as libc::c_int as usize] * v[10 as libc::c_int as usize];
    *cheb12.offset(1 as libc::c_int as isize) = alam1_1 + alam2_1;
    *cheb12.offset(11 as libc::c_int as isize) = alam1_1 - alam2_1;
    let alam1_2: libc::c_double = v[0 as libc::c_int as usize] - part1 + part2;
    let alam2_2: libc::c_double = x[9 as libc::c_int as usize]
        * v[2 as libc::c_int as usize] - part3
        + x[1 as libc::c_int as usize] * v[10 as libc::c_int as usize];
    *cheb12.offset(5 as libc::c_int as isize) = alam1_2 + alam2_2;
    *cheb12.offset(7 as libc::c_int as isize) = alam1_2 - alam2_2;
    let alam_1: libc::c_double = x[0 as libc::c_int as usize]
        * v[1 as libc::c_int as usize]
        + x[2 as libc::c_int as usize] * v[3 as libc::c_int as usize]
        + x[4 as libc::c_int as usize] * v[5 as libc::c_int as usize]
        + x[6 as libc::c_int as usize] * v[7 as libc::c_int as usize]
        + x[8 as libc::c_int as usize] * v[9 as libc::c_int as usize]
        + x[10 as libc::c_int as usize] * v[11 as libc::c_int as usize];
    *cheb24
        .offset(
            1 as libc::c_int as isize,
        ) = *cheb12.offset(1 as libc::c_int as isize) + alam_1;
    *cheb24
        .offset(
            23 as libc::c_int as isize,
        ) = *cheb12.offset(1 as libc::c_int as isize) - alam_1;
    let alam_2: libc::c_double = x[10 as libc::c_int as usize]
        * v[1 as libc::c_int as usize]
        - x[8 as libc::c_int as usize] * v[3 as libc::c_int as usize]
        + x[6 as libc::c_int as usize] * v[5 as libc::c_int as usize]
        - x[4 as libc::c_int as usize] * v[7 as libc::c_int as usize]
        + x[2 as libc::c_int as usize] * v[9 as libc::c_int as usize]
        - x[0 as libc::c_int as usize] * v[11 as libc::c_int as usize];
    *cheb24
        .offset(
            11 as libc::c_int as isize,
        ) = *cheb12.offset(11 as libc::c_int as isize) + alam_2;
    *cheb24
        .offset(
            13 as libc::c_int as isize,
        ) = *cheb12.offset(11 as libc::c_int as isize) - alam_2;
    let alam_3: libc::c_double = x[4 as libc::c_int as usize]
        * v[1 as libc::c_int as usize]
        - x[8 as libc::c_int as usize] * v[3 as libc::c_int as usize]
        - x[0 as libc::c_int as usize] * v[5 as libc::c_int as usize]
        - x[10 as libc::c_int as usize] * v[7 as libc::c_int as usize]
        + x[2 as libc::c_int as usize] * v[9 as libc::c_int as usize]
        + x[6 as libc::c_int as usize] * v[11 as libc::c_int as usize];
    *cheb24
        .offset(
            5 as libc::c_int as isize,
        ) = *cheb12.offset(5 as libc::c_int as isize) + alam_3;
    *cheb24
        .offset(
            19 as libc::c_int as isize,
        ) = *cheb12.offset(5 as libc::c_int as isize) - alam_3;
    let alam_4: libc::c_double = x[6 as libc::c_int as usize]
        * v[1 as libc::c_int as usize]
        - x[2 as libc::c_int as usize] * v[3 as libc::c_int as usize]
        - x[10 as libc::c_int as usize] * v[5 as libc::c_int as usize]
        + x[0 as libc::c_int as usize] * v[7 as libc::c_int as usize]
        - x[8 as libc::c_int as usize] * v[9 as libc::c_int as usize]
        - x[4 as libc::c_int as usize] * v[11 as libc::c_int as usize];
    *cheb24
        .offset(
            7 as libc::c_int as isize,
        ) = *cheb12.offset(7 as libc::c_int as isize) + alam_4;
    *cheb24
        .offset(
            17 as libc::c_int as isize,
        ) = *cheb12.offset(7 as libc::c_int as isize) - alam_4;
    i = 0 as libc::c_int as size_t;
    while i < 6 as libc::c_int as libc::c_ulong {
        let j_1: size_t = (12 as libc::c_int as libc::c_ulong).wrapping_sub(i);
        v[i as usize] = fval[i as usize] - fval[j_1 as usize];
        fval[i as usize] = fval[i as usize] + fval[j_1 as usize];
        i = i.wrapping_add(1);
        i;
    }
    let alam1_3: libc::c_double = v[0 as libc::c_int as usize]
        + x[7 as libc::c_int as usize] * v[4 as libc::c_int as usize];
    let alam2_3: libc::c_double = x[3 as libc::c_int as usize]
        * v[2 as libc::c_int as usize];
    *cheb12.offset(2 as libc::c_int as isize) = alam1_3 + alam2_3;
    *cheb12.offset(10 as libc::c_int as isize) = alam1_3 - alam2_3;
    *cheb12
        .offset(
            6 as libc::c_int as isize,
        ) = v[0 as libc::c_int as usize] - v[4 as libc::c_int as usize];
    let alam_5: libc::c_double = x[1 as libc::c_int as usize]
        * v[1 as libc::c_int as usize]
        + x[5 as libc::c_int as usize] * v[3 as libc::c_int as usize]
        + x[9 as libc::c_int as usize] * v[5 as libc::c_int as usize];
    *cheb24
        .offset(
            2 as libc::c_int as isize,
        ) = *cheb12.offset(2 as libc::c_int as isize) + alam_5;
    *cheb24
        .offset(
            22 as libc::c_int as isize,
        ) = *cheb12.offset(2 as libc::c_int as isize) - alam_5;
    let alam_6: libc::c_double = x[5 as libc::c_int as usize]
        * (v[1 as libc::c_int as usize] - v[3 as libc::c_int as usize]
            - v[5 as libc::c_int as usize]);
    *cheb24
        .offset(
            6 as libc::c_int as isize,
        ) = *cheb12.offset(6 as libc::c_int as isize) + alam_6;
    *cheb24
        .offset(
            18 as libc::c_int as isize,
        ) = *cheb12.offset(6 as libc::c_int as isize) - alam_6;
    let alam_7: libc::c_double = x[9 as libc::c_int as usize]
        * v[1 as libc::c_int as usize]
        - x[5 as libc::c_int as usize] * v[3 as libc::c_int as usize]
        + x[1 as libc::c_int as usize] * v[5 as libc::c_int as usize];
    *cheb24
        .offset(
            10 as libc::c_int as isize,
        ) = *cheb12.offset(10 as libc::c_int as isize) + alam_7;
    *cheb24
        .offset(
            14 as libc::c_int as isize,
        ) = *cheb12.offset(10 as libc::c_int as isize) - alam_7;
    i = 0 as libc::c_int as size_t;
    while i < 3 as libc::c_int as libc::c_ulong {
        let j_2: size_t = (6 as libc::c_int as libc::c_ulong).wrapping_sub(i);
        v[i as usize] = fval[i as usize] - fval[j_2 as usize];
        fval[i as usize] = fval[i as usize] + fval[j_2 as usize];
        i = i.wrapping_add(1);
        i;
    }
    *cheb12
        .offset(
            4 as libc::c_int as isize,
        ) = v[0 as libc::c_int as usize]
        + x[7 as libc::c_int as usize] * v[2 as libc::c_int as usize];
    *cheb12
        .offset(
            8 as libc::c_int as isize,
        ) = fval[0 as libc::c_int as usize]
        - x[7 as libc::c_int as usize] * fval[2 as libc::c_int as usize];
    let alam_8: libc::c_double = x[3 as libc::c_int as usize]
        * v[1 as libc::c_int as usize];
    *cheb24
        .offset(
            4 as libc::c_int as isize,
        ) = *cheb12.offset(4 as libc::c_int as isize) + alam_8;
    *cheb24
        .offset(
            20 as libc::c_int as isize,
        ) = *cheb12.offset(4 as libc::c_int as isize) - alam_8;
    let alam_9: libc::c_double = x[7 as libc::c_int as usize]
        * fval[1 as libc::c_int as usize] - fval[3 as libc::c_int as usize];
    *cheb24
        .offset(
            8 as libc::c_int as isize,
        ) = *cheb12.offset(8 as libc::c_int as isize) + alam_9;
    *cheb24
        .offset(
            16 as libc::c_int as isize,
        ) = *cheb12.offset(8 as libc::c_int as isize) - alam_9;
    *cheb12
        .offset(
            0 as libc::c_int as isize,
        ) = fval[0 as libc::c_int as usize] + fval[2 as libc::c_int as usize];
    let alam_10: libc::c_double = fval[1 as libc::c_int as usize]
        + fval[3 as libc::c_int as usize];
    *cheb24
        .offset(
            0 as libc::c_int as isize,
        ) = *cheb12.offset(0 as libc::c_int as isize) + alam_10;
    *cheb24
        .offset(
            24 as libc::c_int as isize,
        ) = *cheb12.offset(0 as libc::c_int as isize) - alam_10;
    *cheb12
        .offset(
            12 as libc::c_int as isize,
        ) = v[0 as libc::c_int as usize] - v[2 as libc::c_int as usize];
    *cheb24
        .offset(12 as libc::c_int as isize) = *cheb12.offset(12 as libc::c_int as isize);
    i = 1 as libc::c_int as size_t;
    while i < 12 as libc::c_int as libc::c_ulong {
        *cheb12.offset(i as isize) *= 1.0f64 / 6.0f64;
        i = i.wrapping_add(1);
        i;
    }
    *cheb12.offset(0 as libc::c_int as isize) *= 1.0f64 / 12.0f64;
    *cheb12.offset(12 as libc::c_int as isize) *= 1.0f64 / 12.0f64;
    i = 1 as libc::c_int as size_t;
    while i < 24 as libc::c_int as libc::c_ulong {
        *cheb24.offset(i as isize) *= 1.0f64 / 12.0f64;
        i = i.wrapping_add(1);
        i;
    }
    *cheb24.offset(0 as libc::c_int as isize) *= 1.0f64 / 24.0f64;
    *cheb24.offset(24 as libc::c_int as isize) *= 1.0f64 / 24.0f64;
}

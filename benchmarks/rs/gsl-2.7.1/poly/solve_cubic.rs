use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_poly_solve_cubic(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut x0: *mut libc::c_double,
    mut x1: *mut libc::c_double,
    mut x2: *mut libc::c_double,
) -> libc::c_int {
    let mut q: libc::c_double = a * a - 3 as libc::c_int as libc::c_double * b;
    let mut r: libc::c_double = 2 as libc::c_int as libc::c_double * a * a * a
        - 9 as libc::c_int as libc::c_double * a * b
        + 27 as libc::c_int as libc::c_double * c;
    let mut Q: libc::c_double = q / 9 as libc::c_int as libc::c_double;
    let mut R: libc::c_double = r / 54 as libc::c_int as libc::c_double;
    let mut Q3: libc::c_double = Q * Q * Q;
    let mut R2: libc::c_double = R * R;
    let mut CR2: libc::c_double = 729 as libc::c_int as libc::c_double * r * r;
    let mut CQ3: libc::c_double = 2916 as libc::c_int as libc::c_double * q * q * q;
    if R == 0 as libc::c_int as libc::c_double && Q == 0 as libc::c_int as libc::c_double
    {
        *x0 = -a / 3 as libc::c_int as libc::c_double;
        *x1 = -a / 3 as libc::c_int as libc::c_double;
        *x2 = -a / 3 as libc::c_int as libc::c_double;
        return 3 as libc::c_int;
    } else if CR2 == CQ3 {
        let mut sqrtQ: libc::c_double = sqrt(Q);
        if R > 0 as libc::c_int as libc::c_double {
            *x0 = -(2 as libc::c_int) as libc::c_double * sqrtQ
                - a / 3 as libc::c_int as libc::c_double;
            *x1 = sqrtQ - a / 3 as libc::c_int as libc::c_double;
            *x2 = sqrtQ - a / 3 as libc::c_int as libc::c_double;
        } else {
            *x0 = -sqrtQ - a / 3 as libc::c_int as libc::c_double;
            *x1 = -sqrtQ - a / 3 as libc::c_int as libc::c_double;
            *x2 = 2 as libc::c_int as libc::c_double * sqrtQ
                - a / 3 as libc::c_int as libc::c_double;
        }
        return 3 as libc::c_int;
    } else if R2 < Q3 {
        let mut sgnR: libc::c_double = (if R >= 0 as libc::c_int as libc::c_double {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double;
        let mut ratio: libc::c_double = sgnR * sqrt(R2 / Q3);
        let mut theta: libc::c_double = acos(ratio);
        let mut norm: libc::c_double = -(2 as libc::c_int) as libc::c_double * sqrt(Q);
        *x0 = norm * cos(theta / 3 as libc::c_int as libc::c_double)
            - a / 3 as libc::c_int as libc::c_double;
        *x1 = norm
            * cos(
                (theta + 2.0f64 * 3.14159265358979323846f64)
                    / 3 as libc::c_int as libc::c_double,
            ) - a / 3 as libc::c_int as libc::c_double;
        *x2 = norm
            * cos(
                (theta - 2.0f64 * 3.14159265358979323846f64)
                    / 3 as libc::c_int as libc::c_double,
            ) - a / 3 as libc::c_int as libc::c_double;
        if *x0 > *x1 {
            let mut tmp: libc::c_double = *x1;
            *x1 = *x0;
            *x0 = tmp;
        }
        if *x1 > *x2 {
            let mut tmp_0: libc::c_double = *x2;
            *x2 = *x1;
            *x1 = tmp_0;
            if *x0 > *x1 {
                let mut tmp_1: libc::c_double = *x1;
                *x1 = *x0;
                *x0 = tmp_1;
            }
        }
        return 3 as libc::c_int;
    } else {
        let mut sgnR_0: libc::c_double = (if R >= 0 as libc::c_int as libc::c_double {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double;
        let mut A: libc::c_double = -sgnR_0
            * pow(fabs(R) + sqrt(R2 - Q3), 1.0f64 / 3.0f64);
        let mut B: libc::c_double = Q / A;
        *x0 = A + B - a / 3 as libc::c_int as libc::c_double;
        return 1 as libc::c_int;
    };
}

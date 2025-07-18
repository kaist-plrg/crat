use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
pub unsafe extern "C" fn gsl_poly_complex_solve_cubic(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut z0: *mut gsl_complex,
    mut z1: *mut gsl_complex,
    mut z2: *mut gsl_complex,
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
        (*z0).dat[0 as libc::c_int as usize] = -a / 3 as libc::c_int as libc::c_double;
        (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        (*z1).dat[0 as libc::c_int as usize] = -a / 3 as libc::c_int as libc::c_double;
        (*z1).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        (*z2).dat[0 as libc::c_int as usize] = -a / 3 as libc::c_int as libc::c_double;
        (*z2).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        return 3 as libc::c_int;
    } else if CR2 == CQ3 {
        let mut sqrtQ: libc::c_double = sqrt(Q);
        if R > 0 as libc::c_int as libc::c_double {
            (*z0)
                .dat[0 as libc::c_int
                as usize] = -(2 as libc::c_int) as libc::c_double * sqrtQ
                - a / 3 as libc::c_int as libc::c_double;
            (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            (*z1)
                .dat[0 as libc::c_int
                as usize] = sqrtQ - a / 3 as libc::c_int as libc::c_double;
            (*z1).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            (*z2)
                .dat[0 as libc::c_int
                as usize] = sqrtQ - a / 3 as libc::c_int as libc::c_double;
            (*z2).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        } else {
            (*z0)
                .dat[0 as libc::c_int
                as usize] = -sqrtQ - a / 3 as libc::c_int as libc::c_double;
            (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            (*z1)
                .dat[0 as libc::c_int
                as usize] = -sqrtQ - a / 3 as libc::c_int as libc::c_double;
            (*z1).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            (*z2)
                .dat[0 as libc::c_int
                as usize] = 2 as libc::c_int as libc::c_double * sqrtQ
                - a / 3 as libc::c_int as libc::c_double;
            (*z2).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
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
        let mut r0: libc::c_double = norm
            * cos(theta / 3 as libc::c_int as libc::c_double)
            - a / 3 as libc::c_int as libc::c_double;
        let mut r1: libc::c_double = norm
            * cos(
                (theta + 2.0f64 * 3.14159265358979323846f64)
                    / 3 as libc::c_int as libc::c_double,
            ) - a / 3 as libc::c_int as libc::c_double;
        let mut r2: libc::c_double = norm
            * cos(
                (theta - 2.0f64 * 3.14159265358979323846f64)
                    / 3 as libc::c_int as libc::c_double,
            ) - a / 3 as libc::c_int as libc::c_double;
        if r0 > r1 {
            let mut tmp: libc::c_double = r1;
            r1 = r0;
            r0 = tmp;
        }
        if r1 > r2 {
            let mut tmp_0: libc::c_double = r2;
            r2 = r1;
            r1 = tmp_0;
            if r0 > r1 {
                let mut tmp_1: libc::c_double = r1;
                r1 = r0;
                r0 = tmp_1;
            }
        }
        (*z0).dat[0 as libc::c_int as usize] = r0;
        (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        (*z1).dat[0 as libc::c_int as usize] = r1;
        (*z1).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        (*z2).dat[0 as libc::c_int as usize] = r2;
        (*z2).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
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
        if A + B < 0 as libc::c_int as libc::c_double {
            (*z0)
                .dat[0 as libc::c_int
                as usize] = A + B - a / 3 as libc::c_int as libc::c_double;
            (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            (*z1)
                .dat[0 as libc::c_int
                as usize] = -0.5f64 * (A + B) - a / 3 as libc::c_int as libc::c_double;
            (*z1)
                .dat[1 as libc::c_int as usize] = -(sqrt(3.0f64) / 2.0f64) * fabs(A - B);
            (*z2)
                .dat[0 as libc::c_int
                as usize] = -0.5f64 * (A + B) - a / 3 as libc::c_int as libc::c_double;
            (*z2).dat[1 as libc::c_int as usize] = sqrt(3.0f64) / 2.0f64 * fabs(A - B);
        } else {
            (*z0)
                .dat[0 as libc::c_int
                as usize] = -0.5f64 * (A + B) - a / 3 as libc::c_int as libc::c_double;
            (*z0)
                .dat[1 as libc::c_int as usize] = -(sqrt(3.0f64) / 2.0f64) * fabs(A - B);
            (*z1)
                .dat[0 as libc::c_int
                as usize] = -0.5f64 * (A + B) - a / 3 as libc::c_int as libc::c_double;
            (*z1).dat[1 as libc::c_int as usize] = sqrt(3.0f64) / 2.0f64 * fabs(A - B);
            (*z2)
                .dat[0 as libc::c_int
                as usize] = A + B - a / 3 as libc::c_int as libc::c_double;
            (*z2).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        }
        return 3 as libc::c_int;
    };
}

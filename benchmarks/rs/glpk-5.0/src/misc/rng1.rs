use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_rng_next_rand(rand: *mut RNG) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
pub unsafe extern "C" fn _glp_rng_unif_01(mut rand: *mut RNG) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    x = _glp_rng_next_rand(rand) as libc::c_double / 2147483647.0f64;
    (0.0f64 <= x && x <= 1.0f64
        || {
            glp_assert_(
                b"0.0 <= x && x <= 1.0\0" as *const u8 as *const libc::c_char,
                b"misc/rng1.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return x;
}
pub unsafe extern "C" fn _glp_rng_uniform(
    mut rand: *mut RNG,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    (a < b
        || {
            glp_assert_(
                b"a < b\0" as *const u8 as *const libc::c_char,
                b"misc/rng1.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    x = _glp_rng_unif_01(rand);
    x = a * (1.0f64 - x) + b * x;
    (a <= x && x <= b
        || {
            glp_assert_(
                b"a <= x && x <= b\0" as *const u8 as *const libc::c_char,
                b"misc/rng1.c\0" as *const u8 as *const libc::c_char,
                67 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return x;
}

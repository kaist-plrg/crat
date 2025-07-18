use ::libc;
extern "C" {
    fn abort() -> !;
    static mut gsl_rng_r250: *const gsl_rng_type;
    static mut gsl_rng_borosh13: *const gsl_rng_type;
    static mut gsl_rng_coveyou: *const gsl_rng_type;
    static mut gsl_rng_cmrg: *const gsl_rng_type;
    static mut gsl_rng_fishman18: *const gsl_rng_type;
    static mut gsl_rng_fishman20: *const gsl_rng_type;
    static mut gsl_rng_fishman2x: *const gsl_rng_type;
    static mut gsl_rng_gfsr4: *const gsl_rng_type;
    static mut gsl_rng_knuthran: *const gsl_rng_type;
    static mut gsl_rng_knuthran2: *const gsl_rng_type;
    static mut gsl_rng_knuthran2002: *const gsl_rng_type;
    static mut gsl_rng_lecuyer21: *const gsl_rng_type;
    static mut gsl_rng_minstd: *const gsl_rng_type;
    static mut gsl_rng_mrg: *const gsl_rng_type;
    static mut gsl_rng_mt19937: *const gsl_rng_type;
    static mut gsl_rng_mt19937_1999: *const gsl_rng_type;
    static mut gsl_rng_mt19937_1998: *const gsl_rng_type;
    static mut gsl_rng_taus2: *const gsl_rng_type;
    static mut gsl_rng_ran1: *const gsl_rng_type;
    static mut gsl_rng_ran2: *const gsl_rng_type;
    static mut gsl_rng_ran3: *const gsl_rng_type;
    static mut gsl_rng_rand: *const gsl_rng_type;
    static mut gsl_rng_rand48: *const gsl_rng_type;
    static mut gsl_rng_random128_bsd: *const gsl_rng_type;
    static mut gsl_rng_random128_glibc2: *const gsl_rng_type;
    static mut gsl_rng_random128_libc5: *const gsl_rng_type;
    static mut gsl_rng_random256_bsd: *const gsl_rng_type;
    static mut gsl_rng_random256_glibc2: *const gsl_rng_type;
    static mut gsl_rng_random256_libc5: *const gsl_rng_type;
    static mut gsl_rng_random32_bsd: *const gsl_rng_type;
    static mut gsl_rng_random32_glibc2: *const gsl_rng_type;
    static mut gsl_rng_random32_libc5: *const gsl_rng_type;
    static mut gsl_rng_random64_bsd: *const gsl_rng_type;
    static mut gsl_rng_random64_glibc2: *const gsl_rng_type;
    static mut gsl_rng_random64_libc5: *const gsl_rng_type;
    static mut gsl_rng_random8_bsd: *const gsl_rng_type;
    static mut gsl_rng_random8_glibc2: *const gsl_rng_type;
    static mut gsl_rng_random8_libc5: *const gsl_rng_type;
    static mut gsl_rng_random_bsd: *const gsl_rng_type;
    static mut gsl_rng_random_glibc2: *const gsl_rng_type;
    static mut gsl_rng_random_libc5: *const gsl_rng_type;
    static mut gsl_rng_randu: *const gsl_rng_type;
    static mut gsl_rng_ranf: *const gsl_rng_type;
    static mut gsl_rng_ranlux: *const gsl_rng_type;
    static mut gsl_rng_ranlux389: *const gsl_rng_type;
    static mut gsl_rng_ranlxd1: *const gsl_rng_type;
    static mut gsl_rng_ranlxd2: *const gsl_rng_type;
    static mut gsl_rng_ranlxs0: *const gsl_rng_type;
    static mut gsl_rng_ranlxs1: *const gsl_rng_type;
    static mut gsl_rng_ranlxs2: *const gsl_rng_type;
    static mut gsl_rng_ranmar: *const gsl_rng_type;
    static mut gsl_rng_slatec: *const gsl_rng_type;
    static mut gsl_rng_taus: *const gsl_rng_type;
    static mut gsl_rng_ran0: *const gsl_rng_type;
    static mut gsl_rng_transputer: *const gsl_rng_type;
    static mut gsl_rng_tt800: *const gsl_rng_type;
    static mut gsl_rng_uni: *const gsl_rng_type;
    static mut gsl_rng_uni32: *const gsl_rng_type;
    static mut gsl_rng_vax: *const gsl_rng_type;
    static mut gsl_rng_waterman14: *const gsl_rng_type;
    static mut gsl_rng_zuf: *const gsl_rng_type;
    static mut gsl_rng_taus113: *const gsl_rng_type;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
pub static mut gsl_rng_generator_types: [*const gsl_rng_type; 100] = [0
    as *const gsl_rng_type; 100];
pub unsafe extern "C" fn gsl_rng_types_setup() -> *mut *const gsl_rng_type {
    let mut i: libc::c_int = 0 as libc::c_int;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_borosh13;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_cmrg;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_coveyou;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_fishman18;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_fishman20;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_fishman2x;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_gfsr4;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_knuthran;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_knuthran2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_knuthran2002;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_lecuyer21;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_minstd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_mrg;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_mt19937;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_mt19937_1999;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_mt19937_1998;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_r250;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ran0;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ran1;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ran2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ran3;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_rand;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_rand48;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random128_bsd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random128_glibc2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random128_libc5;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random256_bsd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random256_glibc2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random256_libc5;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random32_bsd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random32_glibc2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random32_libc5;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random64_bsd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random64_glibc2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random64_libc5;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random8_bsd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random8_glibc2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random8_libc5;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random_bsd;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random_glibc2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_random_libc5;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_randu;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranf;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlux;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlux389;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlxd1;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlxd2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlxs0;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlxs1;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranlxs2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_ranmar;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_slatec;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_taus;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_taus2;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_taus113;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_transputer;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_tt800;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_uni;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_uni32;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_vax;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_waterman14;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = gsl_rng_zuf;
    i += 1;
    i;
    if i == 100 as libc::c_int {
        abort();
    }
    gsl_rng_generator_types[i as usize] = 0 as *const gsl_rng_type;
    i += 1;
    i;
    return gsl_rng_generator_types.as_mut_ptr();
}

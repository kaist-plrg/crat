use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut gsl_rng_default: *const gsl_rng_type;
    static mut gsl_rng_default_seed: libc::c_ulong;
    fn gsl_rng_alloc(T: *const gsl_rng_type) -> *mut gsl_rng;
    fn gsl_rng_env_setup() -> *const gsl_rng_type;
    fn gsl_ran_dir_nd(r: *const gsl_rng, n: size_t, x: *mut libc::c_double);
    fn gsl_ran_dir_3d(
        r: *const gsl_rng,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        z: *mut libc::c_double,
    );
    fn gsl_ran_dir_2d(r: *const gsl_rng, x: *mut libc::c_double, y: *mut libc::c_double);
    fn gsl_ran_weibull(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_levy_skew(
        r: *const gsl_rng,
        c: libc::c_double,
        alpha: libc::c_double,
        beta: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_levy(
        r: *const gsl_rng,
        c: libc::c_double,
        alpha: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_laplace(r: *const gsl_rng, a: libc::c_double) -> libc::c_double;
    fn gsl_ran_tdist(r: *const gsl_rng, nu: libc::c_double) -> libc::c_double;
    fn gsl_ran_rayleigh_tail(
        r: *const gsl_rng,
        a: libc::c_double,
        sigma: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_rayleigh(r: *const gsl_rng, sigma: libc::c_double) -> libc::c_double;
    fn gsl_ran_poisson(r: *const gsl_rng, mu: libc::c_double) -> libc::c_uint;
    fn gsl_ran_pareto(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_pascal(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_uint,
    ) -> libc::c_uint;
    fn gsl_ran_negative_binomial(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_uint;
    fn gsl_ran_logarithmic(r: *const gsl_rng, p: libc::c_double) -> libc::c_uint;
    fn gsl_ran_lognormal(
        r: *const gsl_rng,
        zeta: libc::c_double,
        sigma: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_logistic(r: *const gsl_rng, a: libc::c_double) -> libc::c_double;
    fn gsl_ran_gumbel2(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_gumbel1(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_hypergeometric(
        r: *const gsl_rng,
        n1: libc::c_uint,
        n2: libc::c_uint,
        t: libc::c_uint,
    ) -> libc::c_uint;
    fn gsl_ran_geometric(r: *const gsl_rng, p: libc::c_double) -> libc::c_uint;
    fn gsl_ran_landau(r: *const gsl_rng) -> libc::c_double;
    fn gsl_ran_bernoulli(r: *const gsl_rng, p: libc::c_double) -> libc::c_uint;
    fn gsl_ran_beta(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_binomial(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_uint,
    ) -> libc::c_uint;
    fn gsl_ran_exponential(r: *const gsl_rng, mu: libc::c_double) -> libc::c_double;
    fn gsl_ran_exppow(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_cauchy(r: *const gsl_rng, a: libc::c_double) -> libc::c_double;
    fn gsl_ran_chisq(r: *const gsl_rng, nu: libc::c_double) -> libc::c_double;
    fn gsl_ran_erlang(
        r: *const gsl_rng,
        a: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_fdist(
        r: *const gsl_rng,
        nu1: libc::c_double,
        nu2: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_flat(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_gamma(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_gaussian(r: *const gsl_rng, sigma: libc::c_double) -> libc::c_double;
    fn gsl_ran_ugaussian(r: *const gsl_rng) -> libc::c_double;
    fn gsl_ran_gaussian_tail(
        r: *const gsl_rng,
        a: libc::c_double,
        sigma: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_ugaussian_tail(r: *const gsl_rng, a: libc::c_double) -> libc::c_double;
    fn gsl_ran_bivariate_gaussian(
        r: *const gsl_rng,
        sigma_x: libc::c_double,
        sigma_y: libc::c_double,
        rho: libc::c_double,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut mu: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut nu: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut nu1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut nu2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sigma: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut c: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut zeta: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sigmax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sigmay: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut rho: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut p: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut z: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut N: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut t: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut seed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *mut gsl_rng = 0 as *mut gsl_rng;
    if argc < 4 as libc::c_int {
        printf(
            b"Usage: gsl-randist seed n DIST param1 param2 ...\nGenerates n samples from the distribution DIST with parameters param1,\nparam2, etc. Valid distributions are,\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"  beta\n  binomial\n  bivariate-gaussian\n  cauchy\n  chisq\n  dir-2d\n  dir-3d\n  dir-nd\n  erlang\n  exponential\n  exppow\n  fdist\n  flat\n  gamma\n  gaussian-tail\n  gaussian\n  geometric\n  gumbel1\n  gumbel2\n  hypergeometric\n  laplace\n  landau\n  levy\n  levy-skew\n  logarithmic\n  logistic\n  lognormal\n  negative-binomial\n  pareto\n  pascal\n  poisson\n  rayleigh-tail\n  rayleigh\n  tdist\n  ugaussian-tail\n  ugaussian\n  weibull\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    argv = argv.offset(1);
    argv;
    seed = atol(*argv.offset(0 as libc::c_int as isize)) as libc::c_ulong;
    argc -= 1;
    argc;
    argv = argv.offset(1);
    argv;
    n = atol(*argv.offset(0 as libc::c_int as isize)) as size_t;
    argc -= 1;
    argc;
    argv = argv.offset(1);
    argv;
    name = *argv.offset(0 as libc::c_int as isize);
    argc -= 1;
    argc;
    argc -= 1;
    argc;
    gsl_rng_env_setup();
    if gsl_rng_default_seed != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"overriding GSL_RNG_SEED with command line value, seed = %ld\n\0"
                as *const u8 as *const libc::c_char,
            seed,
        );
    }
    gsl_rng_default_seed = seed;
    r = gsl_rng_alloc(gsl_rng_default);
    if strcmp(name, b"bernoulli\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"p = probability of success\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            p = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"p\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_bernoulli(r, p),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"beta\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(b"a,b = shape parameters\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_beta(r, a, b));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"binomial\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"p = probability, N = number of trials\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            p = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"p\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            N = atoi(*argv.offset(0 as libc::c_int as isize)) as libc::c_uint;
            argc -= 1;
            argc;
        } else {
            error(b"N\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_binomial(r, p, N),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"cauchy\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"a = scale parameter\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_cauchy(r, a));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"chisq\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"nu = degrees of freedom\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            nu = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"nu\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_chisq(r, nu));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"erlang\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = scale parameter, b = order\0" as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_erlang(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"exponential\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"mu = mean value\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            mu = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"mu\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_exponential(r, mu),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"exppow\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = scale parameter, b = power (1=exponential, 2=gaussian)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_exppow(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"fdist\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"nu1, nu2 = degrees of freedom parameters\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            nu1 = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"nu1\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            nu2 = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"nu2\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_fdist(r, nu1, nu2),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"flat\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = lower limit, b = upper limit\0" as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_flat(r, a, b));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"gamma\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(b"a = order, b = scale\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_gamma(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"gaussian\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"sigma = standard deviation\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigma = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigma\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_gaussian(r, sigma),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"gaussian-tail\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = lower limit, sigma = standard deviation\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigma = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigma\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_gaussian_tail(r, a, sigma),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"ugaussian\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 0 as libc::c_int {
            error(
                b"unit gaussian, no parameters required\0" as *const u8
                    as *const libc::c_char,
            );
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_ugaussian(r));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"ugaussian-tail\0" as *const u8 as *const libc::c_char) == 0
    {
        if argc != 1 as libc::c_int {
            error(b"a = lower limit\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_ugaussian_tail(r, a),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"bivariate-gaussian\0" as *const u8 as *const libc::c_char)
        == 0
    {
        if argc != 3 as libc::c_int {
            error(
                b"sigmax = x std.dev., sigmay = y std.dev., rho = correlation\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigmax = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigmax\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigmay = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigmay\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            rho = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"rho\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            gsl_ran_bivariate_gaussian(r, sigmax, sigmay, rho, &mut x, &mut y);
            printf(b"%g %g\n\0" as *const u8 as *const libc::c_char, x, y);
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"dir-2d\0" as *const u8 as *const libc::c_char) == 0 {
        i = 0 as libc::c_int as size_t;
        while i < n {
            gsl_ran_dir_2d(r, &mut x, &mut y);
            printf(b"%g %g\n\0" as *const u8 as *const libc::c_char, x, y);
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"dir-3d\0" as *const u8 as *const libc::c_char) == 0 {
        i = 0 as libc::c_int as size_t;
        while i < n {
            gsl_ran_dir_3d(r, &mut x, &mut y, &mut z);
            printf(b"%g %g %g\n\0" as *const u8 as *const libc::c_char, x, y, z);
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"dir-nd\0" as *const u8 as *const libc::c_char) == 0 {
        let mut xarr: *mut libc::c_double = 0 as *mut libc::c_double;
        if argc != 1 as libc::c_int {
            error(
                b"n1 = number of dimensions of hypersphere\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            n1 = atoi(*argv.offset(0 as libc::c_int as isize)) as libc::c_uint;
            argc -= 1;
            argc;
        } else {
            error(b"n1\0" as *const u8 as *const libc::c_char);
        }
        xarr = malloc(
            (n1 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < n {
            gsl_ran_dir_nd(r, n1 as size_t, xarr);
            j = 0 as libc::c_int as size_t;
            while j < n1 as libc::c_ulong {
                if j != 0 {
                    putchar(' ' as i32);
                }
                printf(
                    b"%g\0" as *const u8 as *const libc::c_char,
                    *xarr.offset(j as isize),
                );
                j = j.wrapping_add(1);
                j;
            }
            putchar('\n' as i32);
            i = i.wrapping_add(1);
            i;
        }
        free(xarr as *mut libc::c_void);
    } else if strcmp(name, b"geometric\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(
                b"p = bernoulli trial probability of success\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            p = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"p\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_geometric(r, p),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"gumbel1\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = order, b = scale parameter\0" as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_gumbel1(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"gumbel2\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = order, b = scale parameter\0" as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_gumbel2(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"hypergeometric\0" as *const u8 as *const libc::c_char) == 0
    {
        if argc != 3 as libc::c_int {
            error(
                b"n1 = tagged population, n2 = untagged population, t = number of trials\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            n1 = atoi(*argv.offset(0 as libc::c_int as isize)) as libc::c_uint;
            argc -= 1;
            argc;
        } else {
            error(b"n1\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            n2 = atoi(*argv.offset(0 as libc::c_int as isize)) as libc::c_uint;
            argc -= 1;
            argc;
        } else {
            error(b"n2\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            t = atoi(*argv.offset(0 as libc::c_int as isize)) as libc::c_uint;
            argc -= 1;
            argc;
        } else {
            error(b"t\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_hypergeometric(r, n1, n2, t),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"laplace\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"a = scale parameter\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_laplace(r, a));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"landau\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 0 as libc::c_int {
            error(b"no arguments required\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_landau(r));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"levy\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"c = scale, a = power (1=cauchy, 2=gaussian)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            c = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"c\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_levy(r, c, a));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"levy-skew\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 3 as libc::c_int {
            error(
                b"c = scale, a = power (1=cauchy, 2=gaussian), b = skew\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            c = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"c\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_levy_skew(r, c, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"logarithmic\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"p = probability\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            p = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"p\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_logarithmic(r, p),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"logistic\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"a = scale parameter\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_logistic(r, a),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"lognormal\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"zeta = location parameter, sigma = scale parameter\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            zeta = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"zeta\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigma = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigma\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_lognormal(r, zeta, sigma),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"negative-binomial\0" as *const u8 as *const libc::c_char)
        == 0
    {
        if argc != 2 as libc::c_int {
            error(b"p = probability, a = order\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            p = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"p\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_negative_binomial(r, p, a),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"pareto\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = power, b = scale parameter\0" as *const u8 as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_pareto(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"pascal\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"p = probability, n = order (integer)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            p = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"p\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            N = atoi(*argv.offset(0 as libc::c_int as isize)) as libc::c_uint;
            argc -= 1;
            argc;
        } else {
            error(b"N\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_pascal(r, p, N),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"poisson\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"mu = scale parameter\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            mu = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"mu\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_poisson(r, mu),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"rayleigh\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"sigma = scale parameter\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigma = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigma\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_rayleigh(r, sigma),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"rayleigh-tail\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = lower limit, sigma = scale parameter\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            sigma = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"sigma\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_rayleigh_tail(r, a, sigma),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"tdist\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 1 as libc::c_int {
            error(b"nu = degrees of freedom\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            nu = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"nu\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, gsl_ran_tdist(r, nu));
            i = i.wrapping_add(1);
            i;
        }
    } else if strcmp(name, b"weibull\0" as *const u8 as *const libc::c_char) == 0 {
        if argc != 2 as libc::c_int {
            error(
                b"a = scale parameter, b = exponent\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if argc != 0 {
            argv = argv.offset(1);
            a = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"a\0" as *const u8 as *const libc::c_char);
        }
        if argc != 0 {
            argv = argv.offset(1);
            b = atof(*argv.offset(0 as libc::c_int as isize));
            argc -= 1;
            argc;
        } else {
            error(b"b\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            printf(
                b"%g\n\0" as *const u8 as *const libc::c_char,
                gsl_ran_weibull(r, a, b),
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        fprintf(
            stderr,
            b"Error: unrecognized distribution: %s\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn error(mut s: *const libc::c_char) {
    fprintf(
        stderr,
        b"Error: arguments should be %s\n\0" as *const u8 as *const libc::c_char,
        s,
    );
    exit(1 as libc::c_int);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

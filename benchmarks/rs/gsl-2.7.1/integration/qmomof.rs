use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type gsl_integration_qawo_enum = libc::c_uint;
pub const GSL_INTEG_SINE: gsl_integration_qawo_enum = 1;
pub const GSL_INTEG_COSINE: gsl_integration_qawo_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_integration_qawo_table {
    pub n: size_t,
    pub omega: libc::c_double,
    pub L: libc::c_double,
    pub par: libc::c_double,
    pub sine: gsl_integration_qawo_enum,
    pub chebmo: *mut libc::c_double,
}
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_EDOM: C2RustUnnamed = 1;
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
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub unsafe extern "C" fn gsl_integration_qawo_table_alloc(
    mut omega: libc::c_double,
    mut L: libc::c_double,
    mut sine: gsl_integration_qawo_enum,
    mut n: size_t,
) -> *mut gsl_integration_qawo_table {
    let mut t: *mut gsl_integration_qawo_table = 0 as *mut gsl_integration_qawo_table;
    let mut chebmo: *mut libc::c_double = 0 as *mut libc::c_double;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"table length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"qmomof.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_integration_qawo_table;
    }
    t = malloc(::std::mem::size_of::<gsl_integration_qawo_table>() as libc::c_ulong)
        as *mut gsl_integration_qawo_table;
    if t.is_null() {
        gsl_error(
            b"failed to allocate space for qawo_table struct\0" as *const u8
                as *const libc::c_char,
            b"qmomof.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_qawo_table;
    }
    chebmo = malloc(
        (25 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if chebmo.is_null() {
        free(t as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for chebmo block\0" as *const u8
                as *const libc::c_char,
            b"qmomof.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_qawo_table;
    }
    (*t).n = n;
    (*t).sine = sine;
    (*t).omega = omega;
    (*t).L = L;
    (*t).par = 0.5f64 * omega * L;
    (*t).chebmo = chebmo;
    let mut i: size_t = 0;
    let mut scale: libc::c_double = 1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < (*t).n {
        compute_moments(
            (*t).par * scale,
            ((*t).chebmo)
                .offset((25 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize),
        );
        scale *= 0.5f64;
        i = i.wrapping_add(1);
        i;
    }
    return t;
}
pub unsafe extern "C" fn gsl_integration_qawo_table_set(
    mut t: *mut gsl_integration_qawo_table,
    mut omega: libc::c_double,
    mut L: libc::c_double,
    mut sine: gsl_integration_qawo_enum,
) -> libc::c_int {
    (*t).omega = omega;
    (*t).sine = sine;
    (*t).L = L;
    (*t).par = 0.5f64 * omega * L;
    let mut i: size_t = 0;
    let mut scale: libc::c_double = 1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < (*t).n {
        compute_moments(
            (*t).par * scale,
            ((*t).chebmo)
                .offset((25 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize),
        );
        scale *= 0.5f64;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_integration_qawo_table_set_length(
    mut t: *mut gsl_integration_qawo_table,
    mut L: libc::c_double,
) -> libc::c_int {
    if L == (*t).L {
        return GSL_SUCCESS as libc::c_int;
    }
    (*t).L = L;
    (*t).par = 0.5f64 * (*t).omega * L;
    let mut i: size_t = 0;
    let mut scale: libc::c_double = 1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < (*t).n {
        compute_moments(
            (*t).par * scale,
            ((*t).chebmo)
                .offset((25 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize),
        );
        scale *= 0.5f64;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_integration_qawo_table_free(
    mut t: *mut gsl_integration_qawo_table,
) {
    if t.is_null() {
        return;
    }
    free((*t).chebmo as *mut libc::c_void);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn compute_moments(
    mut par: libc::c_double,
    mut chebmo: *mut libc::c_double,
) {
    let mut v: [libc::c_double; 28] = [0.; 28];
    let mut d: [libc::c_double; 25] = [0.; 25];
    let mut d1: [libc::c_double; 25] = [0.; 25];
    let mut d2: [libc::c_double; 25] = [0.; 25];
    let noeq: size_t = 25 as libc::c_int as size_t;
    let par2: libc::c_double = par * par;
    let par4: libc::c_double = par2 * par2;
    let par22: libc::c_double = par2 + 2.0f64;
    let sinpar: libc::c_double = sin(par);
    let cospar: libc::c_double = cos(par);
    let mut i: size_t = 0;
    let mut ac: libc::c_double = 8 as libc::c_int as libc::c_double * cospar;
    let mut as_0: libc::c_double = 24 as libc::c_int as libc::c_double * par * sinpar;
    v[0 as libc::c_int as usize] = 2 as libc::c_int as libc::c_double * sinpar / par;
    v[1 as libc::c_int
        as usize] = (8 as libc::c_int as libc::c_double * cospar
        + (2 as libc::c_int as libc::c_double * par2
            - 8 as libc::c_int as libc::c_double) * sinpar / par) / par2;
    v[2 as libc::c_int
        as usize] = (32 as libc::c_int as libc::c_double
        * (par2 - 12 as libc::c_int as libc::c_double) * cospar
        + 2 as libc::c_int as libc::c_double
            * ((par2 - 80 as libc::c_int as libc::c_double) * par2
                + 192 as libc::c_int as libc::c_double) * sinpar / par) / par4;
    if fabs(par) <= 24 as libc::c_int as libc::c_double {
        let mut an2: libc::c_double = 0.;
        let mut ass: libc::c_double = 0.;
        let mut asap: libc::c_double = 0.;
        let mut an: libc::c_double = 6 as libc::c_int as libc::c_double;
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < noeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            an2 = an * an;
            d[k
                as usize] = -(2 as libc::c_int) as libc::c_double
                * (an2 - 4 as libc::c_int as libc::c_double)
                * (par22 - 2 as libc::c_int as libc::c_double * an2);
            d2[k
                as usize] = (an - 1 as libc::c_int as libc::c_double)
                * (an - 2 as libc::c_int as libc::c_double) * par2;
            d1[k.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as usize] = (an + 3 as libc::c_int as libc::c_double)
                * (an + 4 as libc::c_int as libc::c_double) * par2;
            v[k.wrapping_add(3 as libc::c_int as libc::c_ulong)
                as usize] = as_0 - (an2 - 4 as libc::c_int as libc::c_double) * ac;
            an = an + 2.0f64;
            k = k.wrapping_add(1);
            k;
        }
        an2 = an * an;
        d[noeq.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = -(2 as libc::c_int) as libc::c_double
            * (an2 - 4 as libc::c_int as libc::c_double)
            * (par22 - 2 as libc::c_int as libc::c_double * an2);
        v[noeq.wrapping_add(2 as libc::c_int as libc::c_ulong)
            as usize] = as_0 - (an2 - 4 as libc::c_int as libc::c_double) * ac;
        v[3 as libc::c_int
            as usize] = v[3 as libc::c_int as usize]
            - 56 as libc::c_int as libc::c_double * par2 * v[2 as libc::c_int as usize];
        ass = par * sinpar;
        asap = (((((210 as libc::c_int as libc::c_double * par2
            - 1 as libc::c_int as libc::c_double) * cospar
            - (105 as libc::c_int as libc::c_double * par2
                - 63 as libc::c_int as libc::c_double) * ass) / an2
            - (1 as libc::c_int as libc::c_double
                - 15 as libc::c_int as libc::c_double * par2) * cospar
            + 15 as libc::c_int as libc::c_double * ass) / an2 - cospar
            + 3 as libc::c_int as libc::c_double * ass) / an2 - cospar) / an2;
        v[noeq.wrapping_add(2 as libc::c_int as libc::c_ulong)
            as usize] = v[noeq.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize]
            - 2 as libc::c_int as libc::c_double * asap * par2
                * (an - 1 as libc::c_int as libc::c_double)
                * (an - 2 as libc::c_int as libc::c_double);
        dgtsl(
            noeq,
            d1.as_mut_ptr(),
            d.as_mut_ptr(),
            d2.as_mut_ptr(),
            v.as_mut_ptr().offset(3 as libc::c_int as isize),
        );
    } else {
        let mut k_0: size_t = 0;
        let mut an_0: libc::c_double = 4 as libc::c_int as libc::c_double;
        k_0 = 3 as libc::c_int as size_t;
        while k_0 < 13 as libc::c_int as libc::c_ulong {
            let mut an2_0: libc::c_double = an_0 * an_0;
            v[k_0
                as usize] = ((an2_0 - 4 as libc::c_int as libc::c_double)
                * (2 as libc::c_int as libc::c_double
                    * (par22 - 2 as libc::c_int as libc::c_double * an2_0)
                    * v[k_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                    - ac) + as_0
                - par2 * (an_0 + 1 as libc::c_int as libc::c_double)
                    * (an_0 + 2 as libc::c_int as libc::c_double)
                    * v[k_0.wrapping_sub(2 as libc::c_int as libc::c_ulong) as usize])
                / (par2 * (an_0 - 1 as libc::c_int as libc::c_double)
                    * (an_0 - 2 as libc::c_int as libc::c_double));
            an_0 = an_0 + 2.0f64;
            k_0 = k_0.wrapping_add(1);
            k_0;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < 13 as libc::c_int as libc::c_ulong {
        *chebmo
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
            ) = v[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    v[0 as libc::c_int
        as usize] = 2 as libc::c_int as libc::c_double * (sinpar - par * cospar) / par2;
    v[1 as libc::c_int
        as usize] = (18 as libc::c_int as libc::c_double
        - 48 as libc::c_int as libc::c_double / par2) * sinpar / par2
        + (-(2 as libc::c_int) as libc::c_double
            + 48 as libc::c_int as libc::c_double / par2) * cospar / par;
    ac = -(24 as libc::c_int) as libc::c_double * par * cospar;
    as_0 = -(8 as libc::c_int) as libc::c_double * sinpar;
    if fabs(par) <= 24 as libc::c_int as libc::c_double {
        let mut k_1: size_t = 0;
        let mut an2_1: libc::c_double = 0.;
        let mut ass_0: libc::c_double = 0.;
        let mut asap_0: libc::c_double = 0.;
        let mut an_1: libc::c_double = 5 as libc::c_int as libc::c_double;
        k_1 = 0 as libc::c_int as size_t;
        while k_1 < noeq.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            an2_1 = an_1 * an_1;
            d[k_1
                as usize] = -(2 as libc::c_int) as libc::c_double
                * (an2_1 - 4 as libc::c_int as libc::c_double)
                * (par22 - 2 as libc::c_int as libc::c_double * an2_1);
            d2[k_1
                as usize] = (an_1 - 1 as libc::c_int as libc::c_double)
                * (an_1 - 2 as libc::c_int as libc::c_double) * par2;
            d1[k_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as usize] = (an_1 + 3 as libc::c_int as libc::c_double)
                * (an_1 + 4 as libc::c_int as libc::c_double) * par2;
            v[k_1.wrapping_add(2 as libc::c_int as libc::c_ulong)
                as usize] = ac + (an2_1 - 4 as libc::c_int as libc::c_double) * as_0;
            an_1 = an_1 + 2.0f64;
            k_1 = k_1.wrapping_add(1);
            k_1;
        }
        an2_1 = an_1 * an_1;
        d[noeq.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = -(2 as libc::c_int) as libc::c_double
            * (an2_1 - 4 as libc::c_int as libc::c_double)
            * (par22 - 2 as libc::c_int as libc::c_double * an2_1);
        v[noeq.wrapping_add(1 as libc::c_int as libc::c_ulong)
            as usize] = ac + (an2_1 - 4 as libc::c_int as libc::c_double) * as_0;
        v[2 as libc::c_int
            as usize] = v[2 as libc::c_int as usize]
            - 42 as libc::c_int as libc::c_double * par2 * v[1 as libc::c_int as usize];
        ass_0 = par * cospar;
        asap_0 = (((((105 as libc::c_int as libc::c_double * par2
            - 63 as libc::c_int as libc::c_double) * ass_0
            - (210 as libc::c_int as libc::c_double * par2
                - 1 as libc::c_int as libc::c_double) * sinpar) / an2_1
            + (15 as libc::c_int as libc::c_double * par2
                - 1 as libc::c_int as libc::c_double) * sinpar
            - 15 as libc::c_int as libc::c_double * ass_0) / an2_1 - sinpar
            - 3 as libc::c_int as libc::c_double * ass_0) / an2_1 - sinpar) / an2_1;
        v[noeq.wrapping_add(1 as libc::c_int as libc::c_ulong)
            as usize] = v[noeq.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize]
            - 2 as libc::c_int as libc::c_double * asap_0 * par2
                * (an_1 - 1 as libc::c_int as libc::c_double)
                * (an_1 - 2 as libc::c_int as libc::c_double);
        dgtsl(
            noeq,
            d1.as_mut_ptr(),
            d.as_mut_ptr(),
            d2.as_mut_ptr(),
            v.as_mut_ptr().offset(2 as libc::c_int as isize),
        );
    } else {
        let mut k_2: size_t = 0;
        let mut an_2: libc::c_double = 3 as libc::c_int as libc::c_double;
        k_2 = 2 as libc::c_int as size_t;
        while k_2 < 12 as libc::c_int as libc::c_ulong {
            let mut an2_2: libc::c_double = an_2 * an_2;
            v[k_2
                as usize] = ((an2_2 - 4 as libc::c_int as libc::c_double)
                * (2 as libc::c_int as libc::c_double
                    * (par22 - 2 as libc::c_int as libc::c_double * an2_2)
                    * v[k_2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                    + as_0) + ac
                - par2 * (an_2 + 1 as libc::c_int as libc::c_double)
                    * (an_2 + 2 as libc::c_int as libc::c_double)
                    * v[k_2.wrapping_sub(2 as libc::c_int as libc::c_ulong) as usize])
                / (par2 * (an_2 - 1 as libc::c_int as libc::c_double)
                    * (an_2 - 2 as libc::c_int as libc::c_double));
            an_2 = an_2 + 2.0f64;
            k_2 = k_2.wrapping_add(1);
            k_2;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < 12 as libc::c_int as libc::c_ulong {
        *chebmo
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = v[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn dgtsl(
    mut n: size_t,
    mut c: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut e: *mut libc::c_double,
    mut b: *mut libc::c_double,
) -> libc::c_int {
    let mut k: size_t = 0;
    *c.offset(0 as libc::c_int as isize) = *d.offset(0 as libc::c_int as isize);
    if n == 0 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        *b
            .offset(
                0 as libc::c_int as isize,
            ) = *b.offset(0 as libc::c_int as isize)
            / *d.offset(0 as libc::c_int as isize);
        return GSL_SUCCESS as libc::c_int;
    }
    *d.offset(0 as libc::c_int as isize) = *e.offset(0 as libc::c_int as isize);
    *e.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *e
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_double;
    k = 0 as libc::c_int as size_t;
    while k < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut k1: size_t = k.wrapping_add(1 as libc::c_int as libc::c_ulong);
        if fabs(*c.offset(k1 as isize)) >= fabs(*c.offset(k as isize)) {
            let mut t: libc::c_double = *c.offset(k1 as isize);
            *c.offset(k1 as isize) = *c.offset(k as isize);
            *c.offset(k as isize) = t;
            let mut t_0: libc::c_double = *d.offset(k1 as isize);
            *d.offset(k1 as isize) = *d.offset(k as isize);
            *d.offset(k as isize) = t_0;
            let mut t_1: libc::c_double = *e.offset(k1 as isize);
            *e.offset(k1 as isize) = *e.offset(k as isize);
            *e.offset(k as isize) = t_1;
            let mut t_2: libc::c_double = *b.offset(k1 as isize);
            *b.offset(k1 as isize) = *b.offset(k as isize);
            *b.offset(k as isize) = t_2;
        }
        if *c.offset(k as isize) == 0 as libc::c_int as libc::c_double {
            return GSL_FAILURE as libc::c_int;
        }
        let mut t_3: libc::c_double = -*c.offset(k1 as isize) / *c.offset(k as isize);
        *c.offset(k1 as isize) = *d.offset(k1 as isize) + t_3 * *d.offset(k as isize);
        *d.offset(k1 as isize) = *e.offset(k1 as isize) + t_3 * *e.offset(k as isize);
        *e.offset(k1 as isize) = 0 as libc::c_int as libc::c_double;
        *b.offset(k1 as isize) = *b.offset(k1 as isize) + t_3 * *b.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    if *c.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        == 0 as libc::c_int as libc::c_double
    {
        return GSL_FAILURE as libc::c_int;
    }
    *b
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *b.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        / *c.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *b
        .offset(
            n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
        ) = (*b.offset(n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
        - *d.offset(n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            * *b.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        / *c.offset(n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
    k = n;
    while k > 2 as libc::c_int as libc::c_ulong {
        let mut kb: size_t = k.wrapping_sub(3 as libc::c_int as libc::c_ulong);
        *b
            .offset(
                kb as isize,
            ) = (*b.offset(kb as isize)
            - *d.offset(kb as isize)
                * *b.offset(kb.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *e.offset(kb as isize)
                * *b.offset(kb.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize))
            / *c.offset(kb as isize);
        k = k.wrapping_sub(1);
        k;
    }
    return GSL_SUCCESS as libc::c_int;
}

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_bessel_Jnu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_zero_Jnu_e(
        nu: libc::c_double,
        s: libc::c_uint,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_dht_struct {
    pub size: size_t,
    pub nu: libc::c_double,
    pub xmax: libc::c_double,
    pub kmax: libc::c_double,
    pub j: *mut libc::c_double,
    pub Jjj: *mut libc::c_double,
    pub J2: *mut libc::c_double,
}
pub type gsl_dht = gsl_dht_struct;
pub unsafe extern "C" fn gsl_dht_alloc(mut size: size_t) -> *mut gsl_dht {
    let mut t: *mut gsl_dht = 0 as *mut gsl_dht;
    if size == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"size == 0\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_dht;
    }
    t = malloc(::std::mem::size_of::<gsl_dht>() as libc::c_ulong) as *mut gsl_dht;
    if t.is_null() {
        gsl_error(
            b"out of memory\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_dht;
    }
    (*t).size = size;
    (*t).xmax = -1.0f64;
    (*t).nu = -1.0f64;
    (*t)
        .j = malloc(
        size
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*t).j).is_null() {
        free(t as *mut libc::c_void);
        gsl_error(
            b"could not allocate memory for j\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_dht;
    }
    (*t)
        .Jjj = malloc(
        size
            .wrapping_mul(size.wrapping_add(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*t).Jjj).is_null() {
        free((*t).j as *mut libc::c_void);
        free(t as *mut libc::c_void);
        gsl_error(
            b"could not allocate memory for Jjj\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_dht;
    }
    (*t)
        .J2 = malloc(
        size
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*t).J2).is_null() {
        free((*t).Jjj as *mut libc::c_void);
        free((*t).j as *mut libc::c_void);
        free(t as *mut libc::c_void);
        gsl_error(
            b"could not allocate memory for J2\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_dht;
    }
    return t;
}
unsafe extern "C" fn dht_bessel_zeros(mut t: *mut gsl_dht) -> libc::c_int {
    let mut s: libc::c_uint = 0;
    let mut z: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_z: libc::c_int = 0 as libc::c_int;
    *((*t).j).offset(0 as libc::c_int as isize) = 0.0f64;
    s = 1 as libc::c_int as libc::c_uint;
    while (s as libc::c_ulong)
        < ((*t).size).wrapping_add(2 as libc::c_int as libc::c_ulong)
    {
        stat_z += gsl_sf_bessel_zero_Jnu_e((*t).nu, s, &mut z);
        *((*t).j).offset(s as isize) = z.val;
        s = s.wrapping_add(1);
        s;
    }
    if stat_z != 0 as libc::c_int {
        gsl_error(
            b"could not compute bessel zeroes\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_dht_new(
    mut size: size_t,
    mut nu: libc::c_double,
    mut xmax: libc::c_double,
) -> *mut gsl_dht {
    let mut status: libc::c_int = 0;
    let mut dht: *mut gsl_dht = gsl_dht_alloc(size);
    if dht.is_null() {
        return 0 as *mut gsl_dht;
    }
    status = gsl_dht_init(dht, nu, xmax);
    if status != 0 {
        return 0 as *mut gsl_dht;
    }
    return dht;
}
pub unsafe extern "C" fn gsl_dht_init(
    mut t: *mut gsl_dht,
    mut nu: libc::c_double,
    mut xmax: libc::c_double,
) -> libc::c_int {
    if xmax <= 0.0f64 {
        gsl_error(
            b"xmax is not positive\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nu < 0.0f64 {
        gsl_error(
            b"nu is negative\0" as *const u8 as *const libc::c_char,
            b"dht.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut n: size_t = 0;
        let mut m: size_t = 0;
        let mut stat_bz: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut stat_J: libc::c_int = 0 as libc::c_int;
        let mut jN: libc::c_double = 0.;
        if nu != (*t).nu {
            (*t).nu = nu;
            stat_bz = dht_bessel_zeros(t);
        }
        jN = *((*t).j)
            .offset(
                ((*t).size).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        (*t).xmax = xmax;
        (*t).kmax = jN / xmax;
        *((*t).J2).offset(0 as libc::c_int as isize) = 0.0f64;
        m = 1 as libc::c_int as size_t;
        while m < ((*t).size).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            let mut J: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_J
                += gsl_sf_bessel_Jnu_e(
                    nu + 1.0f64,
                    *((*t).j).offset(m as isize),
                    &mut J,
                );
            *((*t).J2).offset(m as isize) = J.val * J.val;
            m = m.wrapping_add(1);
            m;
        }
        n = 1 as libc::c_int as size_t;
        while n < ((*t).size).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            m = 1 as libc::c_int as size_t;
            while m <= n {
                let mut arg: libc::c_double = *((*t).j).offset(n as isize)
                    * *((*t).j).offset(m as isize) / jN;
                let mut J_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
                stat_J += gsl_sf_bessel_Jnu_e(nu, arg, &mut J_0);
                *((*t).Jjj)
                    .offset(
                        n
                            .wrapping_mul(
                                n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(m)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = J_0.val;
                m = m.wrapping_add(1);
                m;
            }
            n = n.wrapping_add(1);
            n;
        }
        if stat_J != 0 as libc::c_int {
            gsl_error(
                b"error computing bessel function\0" as *const u8 as *const libc::c_char,
                b"dht.c\0" as *const u8 as *const libc::c_char,
                160 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        } else {
            return stat_bz
        }
    };
}
pub unsafe extern "C" fn gsl_dht_x_sample(
    mut t: *const gsl_dht,
    mut n: libc::c_int,
) -> libc::c_double {
    return *((*t).j).offset((n + 1 as libc::c_int) as isize)
        / *((*t).j)
            .offset(((*t).size).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        * (*t).xmax;
}
pub unsafe extern "C" fn gsl_dht_k_sample(
    mut t: *const gsl_dht,
    mut n: libc::c_int,
) -> libc::c_double {
    return *((*t).j).offset((n + 1 as libc::c_int) as isize) / (*t).xmax;
}
pub unsafe extern "C" fn gsl_dht_free(mut t: *mut gsl_dht) {
    if t.is_null() {
        return;
    }
    free((*t).J2 as *mut libc::c_void);
    free((*t).Jjj as *mut libc::c_void);
    free((*t).j as *mut libc::c_void);
    free(t as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_dht_apply(
    mut t: *const gsl_dht,
    mut f_in: *mut libc::c_double,
    mut f_out: *mut libc::c_double,
) -> libc::c_int {
    let jN: libc::c_double = *((*t).j)
        .offset(((*t).size).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    let r: libc::c_double = (*t).xmax / jN;
    let mut m: size_t = 0;
    let mut i: size_t = 0;
    m = 0 as libc::c_int as size_t;
    while m < (*t).size {
        let mut sum: libc::c_double = 0.0f64;
        let mut Y: libc::c_double = 0.;
        i = 0 as libc::c_int as size_t;
        while i < (*t).size {
            let mut m_local: size_t = 0;
            let mut n_local: size_t = 0;
            if i < m {
                m_local = i;
                n_local = m;
            } else {
                m_local = m;
                n_local = i;
            }
            Y = *((*t).Jjj)
                .offset(
                    n_local
                        .wrapping_mul(
                            n_local.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(m_local) as isize,
                )
                / *((*t).J2)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            sum += Y * *f_in.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        *f_out.offset(m as isize) = sum * 2.0f64 * r * r;
        m = m.wrapping_add(1);
        m;
    }
    return GSL_SUCCESS as libc::c_int;
}

use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp_ui(_: mpz_srcptr, _: libc::c_ulong) -> libc::c_int;
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn nettle_random_prime(
        p: *mut __mpz_struct,
        bits: libc::c_uint,
        top_bits_set: libc::c_int,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        progress_ctx: *mut libc::c_void,
        progress: Option::<nettle_progress_func>,
    );
    fn _nettle_generate_pocklington_prime(
        p: *mut __mpz_struct,
        r: *mut __mpz_struct,
        bits: libc::c_uint,
        top_bits_set: libc::c_int,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        p0: *const __mpz_struct,
        q: *const __mpz_struct,
        p0q: *const __mpz_struct,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_random_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_progress_func = unsafe extern "C" fn(
    *mut libc::c_void,
    libc::c_int,
) -> ();
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_generate_params(
    mut params: *mut dsa_params,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut progress_ctx: *mut libc::c_void,
    mut progress: Option::<nettle_progress_func>,
    mut p_bits: libc::c_uint,
    mut q_bits: libc::c_uint,
) -> libc::c_int {
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut p0_bits: libc::c_uint = 0;
    let mut a: libc::c_uint = 0;
    if q_bits < 30 as libc::c_int as libc::c_uint
        || p_bits < q_bits.wrapping_add(30 as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(r.as_mut_ptr());
    nettle_random_prime(
        ((*params).q).as_mut_ptr(),
        q_bits,
        0 as libc::c_int,
        random_ctx,
        random,
        progress_ctx,
        progress,
    );
    if q_bits
        >= p_bits
            .wrapping_add(2 as libc::c_int as libc::c_uint)
            .wrapping_div(3 as libc::c_int as libc::c_uint)
    {
        _nettle_generate_pocklington_prime(
            ((*params).p).as_mut_ptr(),
            r.as_mut_ptr(),
            p_bits,
            0 as libc::c_int,
            random_ctx,
            random,
            ((*params).q).as_mut_ptr() as *const __mpz_struct,
            0 as *const __mpz_struct,
            ((*params).q).as_mut_ptr() as *const __mpz_struct,
        );
    } else {
        let mut p0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut p0q: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(p0.as_mut_ptr());
        __gmpz_init(p0q.as_mut_ptr());
        p0_bits = p_bits
            .wrapping_add(3 as libc::c_int as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint);
        nettle_random_prime(
            p0.as_mut_ptr(),
            p0_bits,
            0 as libc::c_int,
            random_ctx,
            random,
            progress_ctx,
            progress,
        );
        if progress.is_some() {
            progress.unwrap()(progress_ctx, 'q' as i32);
        }
        __gmpz_mul(
            p0q.as_mut_ptr(),
            p0.as_mut_ptr() as mpz_srcptr,
            ((*params).q).as_mut_ptr() as mpz_srcptr,
        );
        _nettle_generate_pocklington_prime(
            ((*params).p).as_mut_ptr(),
            r.as_mut_ptr(),
            p_bits,
            0 as libc::c_int,
            random_ctx,
            random,
            p0.as_mut_ptr() as *const __mpz_struct,
            ((*params).q).as_mut_ptr() as *const __mpz_struct,
            p0q.as_mut_ptr() as *const __mpz_struct,
        );
        __gmpz_mul(
            r.as_mut_ptr(),
            r.as_mut_ptr() as mpz_srcptr,
            p0.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_clear(p0.as_mut_ptr());
        __gmpz_clear(p0q.as_mut_ptr());
    }
    if progress.is_some() {
        progress.unwrap()(progress_ctx, 'p' as i32);
    }
    a = 2 as libc::c_int as libc::c_uint;
    loop {
        __gmpz_set_ui(((*params).g).as_mut_ptr(), a as libc::c_ulong);
        __gmpz_powm(
            ((*params).g).as_mut_ptr(),
            ((*params).g).as_mut_ptr() as mpz_srcptr,
            r.as_mut_ptr() as mpz_srcptr,
            ((*params).p).as_mut_ptr() as mpz_srcptr,
        );
        if (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
            (if (*((*params).g).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*params).g).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            })
        } else {
            __gmpz_cmp_ui(
                ((*params).g).as_mut_ptr() as mpz_srcptr,
                1 as libc::c_int as libc::c_ulong,
            )
        }) != 0 as libc::c_int
        {
            break;
        }
        a = a.wrapping_add(1);
        a;
    }
    __gmpz_clear(r.as_mut_ptr());
    if progress.is_some() {
        progress.unwrap()(progress_ctx, 'g' as i32);
    }
    return 1 as libc::c_int;
}

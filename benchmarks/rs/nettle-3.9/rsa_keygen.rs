use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp_ui(_: mpz_srcptr, _: libc::c_ulong) -> libc::c_int;
    fn __gmpz_fdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_gcd(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_invert(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_setbit(_: mpz_ptr, _: mp_bitcnt_t);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn __gmpz_sub_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_tstbit(_: mpz_srcptr, _: mp_bitcnt_t) -> libc::c_int;
    fn nettle_mpz_random_size(
        x: *mut __mpz_struct,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        bits: libc::c_uint,
    );
    fn nettle_random_prime(
        p: *mut __mpz_struct,
        bits: libc::c_uint,
        top_bits_set: libc::c_int,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        progress_ctx: *mut libc::c_void,
        progress: Option::<nettle_progress_func>,
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
pub type mp_bitcnt_t = libc::c_ulong;
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
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_private_key {
    pub size: size_t,
    pub d: mpz_t,
    pub p: mpz_t,
    pub q: mpz_t,
    pub a: mpz_t,
    pub b: mpz_t,
    pub c: mpz_t,
}
pub unsafe extern "C" fn nettle_rsa_generate_keypair(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut progress_ctx: *mut libc::c_void,
    mut progress: Option::<nettle_progress_func>,
    mut n_size: libc::c_uint,
    mut e_size: libc::c_uint,
) -> libc::c_int {
    let mut p1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut q1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut phi: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tmp: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if e_size != 0 {
        if e_size < 16 as libc::c_int as libc::c_uint || e_size >= n_size {
            return 0 as libc::c_int;
        }
    } else {
        if __gmpz_tstbit(
            ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
            0 as libc::c_int as mp_bitcnt_t,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if (if 0 != 0 && 3 as libc::c_int == 0 as libc::c_int {
            (if (*((*pub_0).e).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*pub_0).e).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            })
        } else {
            __gmpz_cmp_ui(
                ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
                3 as libc::c_int as libc::c_ulong,
            )
        }) < 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if __gmpz_sizeinbase(((*pub_0).e).as_mut_ptr() as mpz_srcptr, 2 as libc::c_int)
            >= n_size as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
    }
    if n_size < (8 as libc::c_int * 12 as libc::c_int - 7 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(p1.as_mut_ptr());
    __gmpz_init(q1.as_mut_ptr());
    __gmpz_init(phi.as_mut_ptr());
    __gmpz_init(tmp.as_mut_ptr());
    loop {
        loop {
            nettle_random_prime(
                ((*key).p).as_mut_ptr(),
                n_size
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_div(2 as libc::c_int as libc::c_uint),
                1 as libc::c_int,
                random_ctx,
                random,
                progress_ctx,
                progress,
            );
            __gmpz_sub_ui(
                p1.as_mut_ptr(),
                ((*key).p).as_mut_ptr() as mpz_srcptr,
                1 as libc::c_int as libc::c_ulong,
            );
            if e_size != 0 {
                break;
            }
            __gmpz_gcd(
                tmp.as_mut_ptr(),
                ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
                p1.as_mut_ptr() as mpz_srcptr,
            );
            if (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
                (if (*tmp.as_mut_ptr())._mp_size < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    ((*tmp.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
                })
            } else {
                __gmpz_cmp_ui(
                    tmp.as_mut_ptr() as mpz_srcptr,
                    1 as libc::c_int as libc::c_ulong,
                )
            }) == 0 as libc::c_int
            {
                break;
            }
            if progress.is_some() {
                progress.unwrap()(progress_ctx, 'c' as i32);
            }
        }
        if progress.is_some() {
            progress.unwrap()(progress_ctx, '\n' as i32);
        }
        loop {
            nettle_random_prime(
                ((*key).q).as_mut_ptr(),
                n_size.wrapping_div(2 as libc::c_int as libc::c_uint),
                1 as libc::c_int,
                random_ctx,
                random,
                progress_ctx,
                progress,
            );
            __gmpz_sub_ui(
                q1.as_mut_ptr(),
                ((*key).q).as_mut_ptr() as mpz_srcptr,
                1 as libc::c_int as libc::c_ulong,
            );
            if e_size != 0 {
                break;
            }
            __gmpz_gcd(
                tmp.as_mut_ptr(),
                ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
                q1.as_mut_ptr() as mpz_srcptr,
            );
            if (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
                (if (*tmp.as_mut_ptr())._mp_size < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    ((*tmp.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
                })
            } else {
                __gmpz_cmp_ui(
                    tmp.as_mut_ptr() as mpz_srcptr,
                    1 as libc::c_int as libc::c_ulong,
                )
            }) == 0 as libc::c_int
            {
                break;
            }
            if progress.is_some() {
                progress.unwrap()(progress_ctx, 'c' as i32);
            }
        }
        __gmpz_mul(
            ((*pub_0).n).as_mut_ptr(),
            ((*key).p).as_mut_ptr() as mpz_srcptr,
            ((*key).q).as_mut_ptr() as mpz_srcptr,
        );
        if __gmpz_sizeinbase(((*pub_0).n).as_mut_ptr() as mpz_srcptr, 2 as libc::c_int)
            == n_size as libc::c_ulong
        {} else {
            __assert_fail(
                b"mpz_sizeinbase(pub->n, 2) == n_size\0" as *const u8
                    as *const libc::c_char,
                b"rsa-keygen.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 173],
                    &[libc::c_char; 173],
                >(
                    b"int nettle_rsa_generate_keypair(struct rsa_public_key *, struct rsa_private_key *, void *, nettle_random_func *, void *, nettle_progress_func *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5165: {
            if __gmpz_sizeinbase(
                ((*pub_0).n).as_mut_ptr() as mpz_srcptr,
                2 as libc::c_int,
            ) == n_size as libc::c_ulong
            {} else {
                __assert_fail(
                    b"mpz_sizeinbase(pub->n, 2) == n_size\0" as *const u8
                        as *const libc::c_char,
                    b"rsa-keygen.c\0" as *const u8 as *const libc::c_char,
                    146 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 173],
                        &[libc::c_char; 173],
                    >(
                        b"int nettle_rsa_generate_keypair(struct rsa_public_key *, struct rsa_private_key *, void *, nettle_random_func *, void *, nettle_progress_func *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if progress.is_some() {
            progress.unwrap()(progress_ctx, '\n' as i32);
        }
        if __gmpz_invert(
            ((*key).c).as_mut_ptr(),
            ((*key).q).as_mut_ptr() as mpz_srcptr,
            ((*key).p).as_mut_ptr() as mpz_srcptr,
        ) != 0
        {
            break;
        }
        if progress.is_some() {
            progress.unwrap()(progress_ctx, '?' as i32);
        }
    }
    __gmpz_mul(
        phi.as_mut_ptr(),
        p1.as_mut_ptr() as mpz_srcptr,
        q1.as_mut_ptr() as mpz_srcptr,
    );
    if e_size != 0 {
        let mut retried: libc::c_int = 0 as libc::c_int;
        loop {
            nettle_mpz_random_size(
                ((*pub_0).e).as_mut_ptr(),
                random_ctx,
                random,
                e_size,
            );
            __gmpz_setbit(((*pub_0).e).as_mut_ptr(), 0 as libc::c_int as mp_bitcnt_t);
            __gmpz_setbit(
                ((*pub_0).e).as_mut_ptr(),
                e_size.wrapping_sub(1 as libc::c_int as libc::c_uint) as mp_bitcnt_t,
            );
            if __gmpz_invert(
                ((*key).d).as_mut_ptr(),
                ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
                phi.as_mut_ptr() as mpz_srcptr,
            ) != 0
            {
                break;
            }
            if progress.is_some() {
                progress.unwrap()(progress_ctx, 'e' as i32);
            }
            retried = 1 as libc::c_int;
        }
        if retried != 0 && progress.is_some() {
            progress.unwrap()(progress_ctx, '\n' as i32);
        }
    } else {
        let mut res: libc::c_int = __gmpz_invert(
            ((*key).d).as_mut_ptr(),
            ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
            phi.as_mut_ptr() as mpz_srcptr,
        );
        if res != 0 {} else {
            __assert_fail(
                b"res\0" as *const u8 as *const libc::c_char,
                b"rsa-keygen.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 173],
                    &[libc::c_char; 173],
                >(
                    b"int nettle_rsa_generate_keypair(struct rsa_public_key *, struct rsa_private_key *, void *, nettle_random_func *, void *, nettle_progress_func *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4966: {
            if res != 0 {} else {
                __assert_fail(
                    b"res\0" as *const u8 as *const libc::c_char,
                    b"rsa-keygen.c\0" as *const u8 as *const libc::c_char,
                    191 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 173],
                        &[libc::c_char; 173],
                    >(
                        b"int nettle_rsa_generate_keypair(struct rsa_public_key *, struct rsa_private_key *, void *, nettle_random_func *, void *, nettle_progress_func *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    __gmpz_fdiv_r(
        ((*key).a).as_mut_ptr(),
        ((*key).d).as_mut_ptr() as mpz_srcptr,
        p1.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        ((*key).b).as_mut_ptr(),
        ((*key).d).as_mut_ptr() as mpz_srcptr,
        q1.as_mut_ptr() as mpz_srcptr,
    );
    (*key)
        .size = n_size
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint) as size_t;
    (*pub_0).size = (*key).size;
    if (*pub_0).size >= 12 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"pub->size >= RSA_MINIMUM_N_OCTETS\0" as *const u8 as *const libc::c_char,
            b"rsa-keygen.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 173],
                &[libc::c_char; 173],
            >(
                b"int nettle_rsa_generate_keypair(struct rsa_public_key *, struct rsa_private_key *, void *, nettle_random_func *, void *, nettle_progress_func *, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4865: {
        if (*pub_0).size >= 12 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"pub->size >= RSA_MINIMUM_N_OCTETS\0" as *const u8
                    as *const libc::c_char,
                b"rsa-keygen.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 173],
                    &[libc::c_char; 173],
                >(
                    b"int nettle_rsa_generate_keypair(struct rsa_public_key *, struct rsa_private_key *, void *, nettle_random_func *, void *, nettle_progress_func *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    __gmpz_clear(p1.as_mut_ptr());
    __gmpz_clear(q1.as_mut_ptr());
    __gmpz_clear(phi.as_mut_ptr());
    __gmpz_clear(tmp.as_mut_ptr());
    return 1 as libc::c_int;
}

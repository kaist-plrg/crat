use ::libc;
extern "C" {
    fn nettle_dsa_generate_params(
        params: *mut dsa_params,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        progress_ctx: *mut libc::c_void,
        progress: Option::<nettle_progress_func>,
        p_bits: libc::c_uint,
        q_bits: libc::c_uint,
    ) -> libc::c_int;
    fn nettle_dsa_generate_keypair(
        params: *const dsa_params,
        pub_0: *mut __mpz_struct,
        key: *mut __mpz_struct,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_public_key {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
    pub y: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_private_key {
    pub x: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_compat_generate_keypair(
    mut pub_0: *mut dsa_public_key,
    mut key: *mut dsa_private_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut progress_ctx: *mut libc::c_void,
    mut progress: Option::<nettle_progress_func>,
    mut p_bits: libc::c_uint,
    mut q_bits: libc::c_uint,
) -> libc::c_int {
    let mut params: *mut dsa_params = 0 as *mut dsa_params;
    match q_bits {
        160 => {
            if p_bits < 512 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int;
            }
        }
        224 | 256 => {
            if p_bits < 1024 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int;
            }
        }
        _ => return 0 as libc::c_int,
    }
    params = pub_0 as *mut dsa_params;
    if nettle_dsa_generate_params(
        params,
        random_ctx,
        random,
        progress_ctx,
        progress,
        p_bits,
        q_bits,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    nettle_dsa_generate_keypair(
        params,
        ((*pub_0).y).as_mut_ptr(),
        ((*key).x).as_mut_ptr(),
        random_ctx,
        random,
    );
    return 1 as libc::c_int;
}

use ::libc;
extern "C" {
    pub type nettle_buffer;
    fn nettle_sexp_format(
        buffer: *mut nettle_buffer,
        format: *const libc::c_char,
        _: ...
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn nettle_dsa_keypair_to_sexp(
    mut buffer: *mut nettle_buffer,
    mut algorithm_name: *const libc::c_char,
    mut params: *const dsa_params,
    mut pub_0: *const __mpz_struct,
    mut priv_0: *const __mpz_struct,
) -> libc::c_int {
    if algorithm_name.is_null() {
        algorithm_name = b"dsa\0" as *const u8 as *const libc::c_char;
    }
    if !priv_0.is_null() {
        return nettle_sexp_format(
            buffer,
            b"(private-key(%0s(p%b)(q%b)(g%b)(y%b)(x%b)))\0" as *const u8
                as *const libc::c_char,
            algorithm_name,
            ((*params).p).as_ptr(),
            ((*params).q).as_ptr(),
            ((*params).g).as_ptr(),
            pub_0,
            priv_0,
        ) as libc::c_int
    } else {
        return nettle_sexp_format(
            buffer,
            b"(public-key(%0s(p%b)(q%b)(g%b)(y%b)))\0" as *const u8
                as *const libc::c_char,
            algorithm_name,
            ((*params).p).as_ptr(),
            ((*params).q).as_ptr(),
            ((*params).g).as_ptr(),
            pub_0,
        ) as libc::c_int
    };
}

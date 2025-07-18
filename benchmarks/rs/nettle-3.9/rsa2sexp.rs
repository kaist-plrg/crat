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
pub unsafe extern "C" fn nettle_rsa_keypair_to_sexp(
    mut buffer: *mut nettle_buffer,
    mut algorithm_name: *const libc::c_char,
    mut pub_0: *const rsa_public_key,
    mut priv_0: *const rsa_private_key,
) -> libc::c_int {
    if algorithm_name.is_null() {
        algorithm_name = b"rsa-pkcs1\0" as *const u8 as *const libc::c_char;
    }
    if !priv_0.is_null() {
        return nettle_sexp_format(
            buffer,
            b"(private-key(%0s(n%b)(e%b)(d%b)(p%b)(q%b)(a%b)(b%b)(c%b)))\0" as *const u8
                as *const libc::c_char,
            algorithm_name,
            ((*pub_0).n).as_ptr(),
            ((*pub_0).e).as_ptr(),
            ((*priv_0).d).as_ptr(),
            ((*priv_0).p).as_ptr(),
            ((*priv_0).q).as_ptr(),
            ((*priv_0).a).as_ptr(),
            ((*priv_0).b).as_ptr(),
            ((*priv_0).c).as_ptr(),
        ) as libc::c_int
    } else {
        return nettle_sexp_format(
            buffer,
            b"(public-key(%0s(n%b)(e%b)))\0" as *const u8 as *const libc::c_char,
            algorithm_name,
            ((*pub_0).n).as_ptr(),
            ((*pub_0).e).as_ptr(),
        ) as libc::c_int
    };
}

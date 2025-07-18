use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn read_file(
        name: *const libc::c_char,
        size: size_t,
        buffer: *mut *mut uint8_t,
    ) -> size_t;
    fn nettle_rsa_keypair_from_sexp(
        pub_0: *mut rsa_public_key,
        priv_0: *mut rsa_private_key,
        limit: libc::c_uint,
        length: size_t,
        expr: *const uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
pub type mpz_t = [__mpz_struct; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mp_limb_t = libc::c_ulong;
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
pub unsafe extern "C" fn read_rsa_key(
    mut name: *const libc::c_char,
    mut pub_0: *mut rsa_public_key,
    mut priv_0: *mut rsa_private_key,
) -> libc::c_int {
    let mut length: libc::c_uint = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut res: libc::c_int = 0;
    length = read_file(name, 0 as libc::c_int as size_t, &mut buffer) as libc::c_uint;
    if length == 0 {
        return 0 as libc::c_int;
    }
    res = nettle_rsa_keypair_from_sexp(
        pub_0,
        priv_0,
        0 as libc::c_int as libc::c_uint,
        length as size_t,
        buffer,
    );
    free(buffer as *mut libc::c_void);
    return res;
}

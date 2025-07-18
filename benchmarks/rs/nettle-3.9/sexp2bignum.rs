use ::libc;
extern "C" {
    fn nettle_sexp_iterator_next(iterator: *mut sexp_iterator) -> libc::c_int;
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_mpz_set_str_256_s(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type sexp_type = libc::c_uint;
pub const SEXP_END: sexp_type = 2;
pub const SEXP_LIST: sexp_type = 1;
pub const SEXP_ATOM: sexp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_iterator {
    pub length: size_t,
    pub buffer: *const uint8_t,
    pub start: size_t,
    pub pos: size_t,
    pub level: libc::c_uint,
    pub type_0: sexp_type,
    pub display_length: size_t,
    pub display: *const uint8_t,
    pub atom_length: size_t,
    pub atom: *const uint8_t,
}
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_srcptr = *const __mpz_struct;
pub unsafe extern "C" fn nettle_mpz_set_sexp(
    mut x: *mut __mpz_struct,
    mut limit: libc::c_uint,
    mut i: *mut sexp_iterator,
) -> libc::c_int {
    if (*i).type_0 as libc::c_uint == SEXP_ATOM as libc::c_int as libc::c_uint
        && (*i).atom_length != 0 && ((*i).display).is_null()
    {
        if limit != 0
            && (8 as libc::c_int as libc::c_ulong).wrapping_mul((*i).atom_length)
                > (16 as libc::c_int as libc::c_uint).wrapping_add(limit)
                    as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        nettle_mpz_set_str_256_s(x, (*i).atom_length, (*i).atom);
        if limit != 0
            && __gmpz_sizeinbase(x as mpz_srcptr, 2 as libc::c_int)
                > limit as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        return nettle_sexp_iterator_next(i);
    } else {
        return 0 as libc::c_int
    };
}

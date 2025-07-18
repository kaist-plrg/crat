use ::libc;
extern "C" {
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_mpz_set_sexp(
        x: *mut __mpz_struct,
        limit: libc::c_uint,
        i: *mut sexp_iterator,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_first(
        iterator: *mut sexp_iterator,
        length: size_t,
        input: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_check_type(
        iterator: *mut sexp_iterator,
        type_0: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_assoc(
        iterator: *mut sexp_iterator,
        nkeys: libc::c_uint,
        keys: *const *const libc::c_char,
        values: *mut sexp_iterator,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub type sexp_type = libc::c_uint;
pub const SEXP_END: sexp_type = 2;
pub const SEXP_LIST: sexp_type = 1;
pub const SEXP_ATOM: sexp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_signature {
    pub r: mpz_t,
    pub s: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_keypair_from_sexp_alist(
    mut params: *mut dsa_params,
    mut pub_0: *mut __mpz_struct,
    mut priv_0: *mut __mpz_struct,
    mut p_max_bits: libc::c_uint,
    mut q_bits: libc::c_uint,
    mut i: *mut sexp_iterator,
) -> libc::c_int {
    static mut names: [*const libc::c_char; 5] = [
        b"p\0" as *const u8 as *const libc::c_char,
        b"q\0" as *const u8 as *const libc::c_char,
        b"g\0" as *const u8 as *const libc::c_char,
        b"y\0" as *const u8 as *const libc::c_char,
        b"x\0" as *const u8 as *const libc::c_char,
    ];
    let mut values: [sexp_iterator; 5] = [sexp_iterator {
        length: 0,
        buffer: 0 as *const uint8_t,
        start: 0,
        pos: 0,
        level: 0,
        type_0: SEXP_ATOM,
        display_length: 0,
        display: 0 as *const uint8_t,
        atom_length: 0,
        atom: 0 as *const uint8_t,
    }; 5];
    let mut nvalues: libc::c_uint = (if !priv_0.is_null() {
        5 as libc::c_int
    } else {
        4 as libc::c_int
    }) as libc::c_uint;
    let mut p_bits: libc::c_uint = 0;
    if nettle_sexp_iterator_assoc(i, nvalues, names.as_ptr(), values.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    if nettle_mpz_set_sexp(
        ((*params).p).as_mut_ptr(),
        p_max_bits,
        &mut *values.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) == 0
        || (if (*((*params).p).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*params).p).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    p_bits = __gmpz_sizeinbase(
        ((*params).p).as_mut_ptr() as mpz_srcptr,
        2 as libc::c_int,
    ) as libc::c_uint;
    if nettle_mpz_set_sexp(
        ((*params).q).as_mut_ptr(),
        (if q_bits != 0 { q_bits } else { p_bits }),
        &mut *values.as_mut_ptr().offset(1 as libc::c_int as isize),
    ) == 0
        || (if (*((*params).q).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*params).q).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if q_bits > 0 as libc::c_int as libc::c_uint
        && __gmpz_sizeinbase(((*params).q).as_mut_ptr() as mpz_srcptr, 2 as libc::c_int)
            != q_bits as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if __gmpz_cmp(
        ((*params).q).as_mut_ptr() as mpz_srcptr,
        ((*params).p).as_mut_ptr() as mpz_srcptr,
    ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if nettle_mpz_set_sexp(
        ((*params).g).as_mut_ptr(),
        p_bits,
        &mut *values.as_mut_ptr().offset(2 as libc::c_int as isize),
    ) == 0
        || (if (*((*params).g).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*params).g).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if __gmpz_cmp(
        ((*params).g).as_mut_ptr() as mpz_srcptr,
        ((*params).p).as_mut_ptr() as mpz_srcptr,
    ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if nettle_mpz_set_sexp(
        pub_0,
        p_bits,
        &mut *values.as_mut_ptr().offset(3 as libc::c_int as isize),
    ) == 0
        || (if (*pub_0)._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*pub_0)._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if __gmpz_cmp(pub_0 as mpz_srcptr, ((*params).p).as_mut_ptr() as mpz_srcptr)
        >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !priv_0.is_null() {
        if nettle_mpz_set_sexp(
            priv_0,
            __gmpz_sizeinbase(((*params).q).as_mut_ptr() as mpz_srcptr, 2 as libc::c_int)
                as libc::c_uint,
            &mut *values.as_mut_ptr().offset(4 as libc::c_int as isize),
        ) == 0
            || (if (*priv_0)._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*priv_0)._mp_size > 0 as libc::c_int) as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if __gmpz_cmp(priv_0 as mpz_srcptr, ((*params).q).as_mut_ptr() as mpz_srcptr)
            >= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_dsa_sha1_keypair_from_sexp(
    mut params: *mut dsa_params,
    mut pub_0: *mut __mpz_struct,
    mut priv_0: *mut __mpz_struct,
    mut p_max_bits: libc::c_uint,
    mut length: size_t,
    mut expr: *const uint8_t,
) -> libc::c_int {
    let mut i: sexp_iterator = sexp_iterator {
        length: 0,
        buffer: 0 as *const uint8_t,
        start: 0,
        pos: 0,
        level: 0,
        type_0: SEXP_ATOM,
        display_length: 0,
        display: 0 as *const uint8_t,
        atom_length: 0,
        atom: 0 as *const uint8_t,
    };
    return (nettle_sexp_iterator_first(&mut i, length, expr) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            (if !priv_0.is_null() {
                b"private-key\0" as *const u8 as *const libc::c_char
            } else {
                b"public-key\0" as *const u8 as *const libc::c_char
            }),
        ) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            b"dsa\0" as *const u8 as *const libc::c_char,
        ) != 0
        && nettle_dsa_keypair_from_sexp_alist(
            params,
            pub_0,
            priv_0,
            p_max_bits,
            160 as libc::c_int as libc::c_uint,
            &mut i,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_dsa_sha256_keypair_from_sexp(
    mut params: *mut dsa_params,
    mut pub_0: *mut __mpz_struct,
    mut priv_0: *mut __mpz_struct,
    mut p_max_bits: libc::c_uint,
    mut length: size_t,
    mut expr: *const uint8_t,
) -> libc::c_int {
    let mut i: sexp_iterator = sexp_iterator {
        length: 0,
        buffer: 0 as *const uint8_t,
        start: 0,
        pos: 0,
        level: 0,
        type_0: SEXP_ATOM,
        display_length: 0,
        display: 0 as *const uint8_t,
        atom_length: 0,
        atom: 0 as *const uint8_t,
    };
    return (nettle_sexp_iterator_first(&mut i, length, expr) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            (if !priv_0.is_null() {
                b"private-key\0" as *const u8 as *const libc::c_char
            } else {
                b"public-key\0" as *const u8 as *const libc::c_char
            }),
        ) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            b"dsa-sha256\0" as *const u8 as *const libc::c_char,
        ) != 0
        && nettle_dsa_keypair_from_sexp_alist(
            params,
            pub_0,
            priv_0,
            p_max_bits,
            256 as libc::c_int as libc::c_uint,
            &mut i,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_dsa_signature_from_sexp(
    mut rs: *mut dsa_signature,
    mut i: *mut sexp_iterator,
    mut q_bits: libc::c_uint,
) -> libc::c_int {
    static mut names: [*const libc::c_char; 2] = [
        b"r\0" as *const u8 as *const libc::c_char,
        b"s\0" as *const u8 as *const libc::c_char,
    ];
    let mut values: [sexp_iterator; 2] = [sexp_iterator {
        length: 0,
        buffer: 0 as *const uint8_t,
        start: 0,
        pos: 0,
        level: 0,
        type_0: SEXP_ATOM,
        display_length: 0,
        display: 0 as *const uint8_t,
        atom_length: 0,
        atom: 0 as *const uint8_t,
    }; 2];
    if nettle_sexp_iterator_assoc(
        i,
        2 as libc::c_int as libc::c_uint,
        names.as_ptr(),
        values.as_mut_ptr(),
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if nettle_mpz_set_sexp(
        ((*rs).r).as_mut_ptr(),
        q_bits,
        &mut *values.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) == 0
        || (if (*((*rs).r).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*rs).r).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if nettle_mpz_set_sexp(
        ((*rs).s).as_mut_ptr(),
        q_bits,
        &mut *values.as_mut_ptr().offset(1 as libc::c_int as isize),
    ) == 0
        || (if (*((*rs).s).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*rs).s).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

use ::libc;
extern "C" {
    fn nettle_rsa_public_key_prepare(key: *mut rsa_public_key) -> libc::c_int;
    fn nettle_rsa_private_key_prepare(key: *mut rsa_private_key) -> libc::c_int;
    fn nettle_mpz_set_sexp(
        x: *mut __mpz_struct,
        limit: libc::c_uint,
        i: *mut sexp_iterator,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_assoc(
        iterator: *mut sexp_iterator,
        nkeys: libc::c_uint,
        keys: *const *const libc::c_char,
        values: *mut sexp_iterator,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_check_type(
        iterator: *mut sexp_iterator,
        type_0: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_first(
        iterator: *mut sexp_iterator,
        length: size_t,
        input: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_check_types(
        iterator: *mut sexp_iterator,
        ntypes: libc::c_uint,
        types: *const *const libc::c_char,
    ) -> *const libc::c_char;
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
pub unsafe extern "C" fn nettle_rsa_keypair_from_sexp_alist(
    mut pub_0: *mut rsa_public_key,
    mut priv_0: *mut rsa_private_key,
    mut limit: libc::c_uint,
    mut i: *mut sexp_iterator,
) -> libc::c_int {
    static mut names: [*const libc::c_char; 8] = [
        b"n\0" as *const u8 as *const libc::c_char,
        b"e\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
        b"p\0" as *const u8 as *const libc::c_char,
        b"q\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char,
    ];
    let mut values: [sexp_iterator; 8] = [sexp_iterator {
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
    }; 8];
    let mut nvalues: libc::c_uint = (if !priv_0.is_null() {
        8 as libc::c_int
    } else {
        2 as libc::c_int
    }) as libc::c_uint;
    if nettle_sexp_iterator_assoc(i, nvalues, names.as_ptr(), values.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    if !priv_0.is_null() {
        if nettle_mpz_set_sexp(
            ((*priv_0).d).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(2 as libc::c_int as isize),
        ) == 0
            || (if (*((*priv_0).d).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).d).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_mpz_set_sexp(
            ((*priv_0).p).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(3 as libc::c_int as isize),
        ) == 0
            || (if (*((*priv_0).p).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).p).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_mpz_set_sexp(
            ((*priv_0).q).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(4 as libc::c_int as isize),
        ) == 0
            || (if (*((*priv_0).q).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).q).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_mpz_set_sexp(
            ((*priv_0).a).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(5 as libc::c_int as isize),
        ) == 0
            || (if (*((*priv_0).a).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).a).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_mpz_set_sexp(
            ((*priv_0).b).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(6 as libc::c_int as isize),
        ) == 0
            || (if (*((*priv_0).b).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).b).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_mpz_set_sexp(
            ((*priv_0).c).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(7 as libc::c_int as isize),
        ) == 0
            || (if (*((*priv_0).c).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).c).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_rsa_private_key_prepare(priv_0) == 0 {
            return 0 as libc::c_int;
        }
    }
    if !pub_0.is_null() {
        if nettle_mpz_set_sexp(
            ((*pub_0).n).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(0 as libc::c_int as isize),
        ) == 0
            || (if (*((*pub_0).n).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*pub_0).n).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_mpz_set_sexp(
            ((*pub_0).e).as_mut_ptr(),
            limit,
            &mut *values.as_mut_ptr().offset(1 as libc::c_int as isize),
        ) == 0
            || (if (*((*pub_0).e).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*pub_0).e).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if nettle_rsa_public_key_prepare(pub_0) == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_rsa_keypair_from_sexp(
    mut pub_0: *mut rsa_public_key,
    mut priv_0: *mut rsa_private_key,
    mut limit: libc::c_uint,
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
    static mut names: [*const libc::c_char; 3] = [
        b"rsa\0" as *const u8 as *const libc::c_char,
        b"rsa-pkcs1\0" as *const u8 as *const libc::c_char,
        b"rsa-pkcs1-sha1\0" as *const u8 as *const libc::c_char,
    ];
    if nettle_sexp_iterator_first(&mut i, length, expr) == 0 {
        return 0 as libc::c_int;
    }
    if nettle_sexp_iterator_check_type(
        &mut i,
        if !priv_0.is_null() {
            b"private-key\0" as *const u8 as *const libc::c_char
        } else {
            b"public-key\0" as *const u8 as *const libc::c_char
        },
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if (nettle_sexp_iterator_check_types(
        &mut i,
        3 as libc::c_int as libc::c_uint,
        names.as_ptr(),
    ))
        .is_null()
    {
        return 0 as libc::c_int;
    }
    return nettle_rsa_keypair_from_sexp_alist(pub_0, priv_0, limit, &mut i);
}

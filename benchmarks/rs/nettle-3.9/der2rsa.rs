use ::libc;
extern "C" {
    fn nettle_rsa_public_key_prepare(key: *mut rsa_public_key) -> libc::c_int;
    fn nettle_rsa_private_key_prepare(key: *mut rsa_private_key) -> libc::c_int;
    fn nettle_asn1_der_get_bignum(
        iterator: *mut asn1_der_iterator,
        x: *mut __mpz_struct,
        max_bits: libc::c_uint,
    ) -> libc::c_int;
    fn nettle_asn1_der_iterator_first(
        iterator: *mut asn1_der_iterator,
        length: size_t,
        input: *const uint8_t,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_iterator_next(
        iterator: *mut asn1_der_iterator,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_decode_constructed_last(
        i: *mut asn1_der_iterator,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_get_uint32(
        i: *mut asn1_der_iterator,
        x: *mut uint32_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub struct asn1_der_iterator {
    pub buffer_length: size_t,
    pub buffer: *const uint8_t,
    pub pos: size_t,
    pub type_0: asn1_type,
    pub length: size_t,
    pub data: *const uint8_t,
}
pub type asn1_type = libc::c_uint;
pub const ASN1_BMPSTRING: asn1_type = 30;
pub const ASN1_UNIVERSALSTRING: asn1_type = 28;
pub const ASN1_UTC: asn1_type = 23;
pub const ASN1_IA5STRING: asn1_type = 22;
pub const ASN1_TELETEXSTRING: asn1_type = 20;
pub const ASN1_PRINTABLESTRING: asn1_type = 19;
pub const ASN1_SET: asn1_type = 4113;
pub const ASN1_SEQUENCE: asn1_type = 4112;
pub const ASN1_UTF8STRING: asn1_type = 12;
pub const ASN1_ENUMERATED: asn1_type = 10;
pub const ASN1_REAL: asn1_type = 9;
pub const ASN1_IDENTIFIER: asn1_type = 6;
pub const ASN1_NULL: asn1_type = 5;
pub const ASN1_OCTETSTRING: asn1_type = 4;
pub const ASN1_BITSTRING: asn1_type = 3;
pub const ASN1_INTEGER: asn1_type = 2;
pub const ASN1_BOOLEAN: asn1_type = 1;
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
pub const ASN1_ITERATOR_END: asn1_iterator_result = 3;
pub type asn1_iterator_result = libc::c_uint;
pub const ASN1_ITERATOR_CONSTRUCTED: asn1_iterator_result = 2;
pub const ASN1_ITERATOR_PRIMITIVE: asn1_iterator_result = 1;
pub const ASN1_ITERATOR_ERROR: asn1_iterator_result = 0;
pub unsafe extern "C" fn nettle_rsa_public_key_from_der_iterator(
    mut pub_0: *mut rsa_public_key,
    mut limit: libc::c_uint,
    mut i: *mut asn1_der_iterator,
) -> libc::c_int {
    return ((*i).type_0 as libc::c_uint == ASN1_SEQUENCE as libc::c_int as libc::c_uint
        && nettle_asn1_der_decode_constructed_last(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
        && nettle_asn1_der_get_bignum(i, ((*pub_0).n).as_mut_ptr(), limit) != 0
        && (if (*((*pub_0).n).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*pub_0).n).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) > 0 as libc::c_int
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*pub_0).e).as_mut_ptr(), limit) != 0
            && (if (*((*pub_0).e).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*pub_0).e).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) > 0 as libc::c_int)
        && nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_END as libc::c_int as libc::c_uint
        && nettle_rsa_public_key_prepare(pub_0) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_rsa_private_key_from_der_iterator(
    mut pub_0: *mut rsa_public_key,
    mut priv_0: *mut rsa_private_key,
    mut limit: libc::c_uint,
    mut i: *mut asn1_der_iterator,
) -> libc::c_int {
    let mut version: uint32_t = 0;
    if (*i).type_0 as libc::c_uint != ASN1_SEQUENCE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if nettle_asn1_der_decode_constructed_last(i) as libc::c_uint
        == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
        && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
        && nettle_asn1_der_get_uint32(i, &mut version) != 0
        && version <= 1 as libc::c_int as libc::c_uint
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*pub_0).n).as_mut_ptr(), limit) != 0
            && (if (*((*pub_0).n).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*pub_0).n).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) > 0 as libc::c_int)
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*pub_0).e).as_mut_ptr(), limit) != 0
            && (if (*((*pub_0).e).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*pub_0).e).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) > 0 as libc::c_int) && nettle_rsa_public_key_prepare(pub_0) != 0
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*priv_0).d).as_mut_ptr(), limit) != 0
            && (if (*((*priv_0).d).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).d).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) > 0 as libc::c_int)
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*priv_0).p).as_mut_ptr(), limit) != 0
            && (if (*((*priv_0).p).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).p).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) > 0 as libc::c_int)
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*priv_0).q).as_mut_ptr(), limit) != 0
            && (if (*((*priv_0).q).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).q).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) > 0 as libc::c_int)
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*priv_0).a).as_mut_ptr(), limit) != 0
            && (if (*((*priv_0).a).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).a).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) > 0 as libc::c_int)
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*priv_0).b).as_mut_ptr(), limit) != 0
            && (if (*((*priv_0).b).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).b).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) > 0 as libc::c_int)
        && (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
            && (*i).type_0 as libc::c_uint == ASN1_INTEGER as libc::c_int as libc::c_uint
            && nettle_asn1_der_get_bignum(i, ((*priv_0).c).as_mut_ptr(), limit) != 0
            && (if (*((*priv_0).c).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*priv_0).c).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) > 0 as libc::c_int) && nettle_rsa_private_key_prepare(priv_0) != 0
    {
        if version == 1 as libc::c_int as libc::c_uint {
            if !(nettle_asn1_der_iterator_next(i) as libc::c_uint
                == ASN1_ITERATOR_CONSTRUCTED as libc::c_int as libc::c_uint
                && (*i).type_0 as libc::c_uint
                    == ASN1_SEQUENCE as libc::c_int as libc::c_uint)
            {
                return 0 as libc::c_int;
            }
        }
        return (nettle_asn1_der_iterator_next(i) as libc::c_uint
            == ASN1_ITERATOR_END as libc::c_int as libc::c_uint) as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn nettle_rsa_keypair_from_der(
    mut pub_0: *mut rsa_public_key,
    mut priv_0: *mut rsa_private_key,
    mut limit: libc::c_uint,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut i: asn1_der_iterator = asn1_der_iterator {
        buffer_length: 0,
        buffer: 0 as *const uint8_t,
        pos: 0,
        type_0: 0 as asn1_type,
        length: 0,
        data: 0 as *const uint8_t,
    };
    let mut res: asn1_iterator_result = ASN1_ITERATOR_ERROR;
    res = nettle_asn1_der_iterator_first(&mut i, length, data);
    if res as libc::c_uint != ASN1_ITERATOR_CONSTRUCTED as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if !priv_0.is_null() {
        return nettle_rsa_private_key_from_der_iterator(pub_0, priv_0, limit, &mut i)
    } else {
        return nettle_rsa_public_key_from_der_iterator(pub_0, limit, &mut i)
    };
}

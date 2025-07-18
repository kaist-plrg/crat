use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_mpz_set_str_256_s(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
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
pub type mpz_srcptr = *const __mpz_struct;
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
pub type C2RustUnnamed = libc::c_uint;
pub const ASN1_CLASS_SHIFT: C2RustUnnamed = 13;
pub const ASN1_CLASS_MASK: C2RustUnnamed = 24576;
pub const ASN1_CLASS_PRIVATE: C2RustUnnamed = 24576;
pub const ASN1_CLASS_CONTEXT_SPECIFIC: C2RustUnnamed = 16384;
pub const ASN1_CLASS_APPLICATION: C2RustUnnamed = 8192;
pub const ASN1_CLASS_UNIVERSAL: C2RustUnnamed = 0;
pub const ASN1_TYPE_CONSTRUCTED: C2RustUnnamed = 4096;
pub type asn1_iterator_result = libc::c_uint;
pub const ASN1_ITERATOR_END: asn1_iterator_result = 3;
pub const ASN1_ITERATOR_CONSTRUCTED: asn1_iterator_result = 2;
pub const ASN1_ITERATOR_PRIMITIVE: asn1_iterator_result = 1;
pub const ASN1_ITERATOR_ERROR: asn1_iterator_result = 0;
pub const CONSTRUCTED_MASK: C2RustUnnamed_0 = 32;
pub const CLASS_MASK: C2RustUnnamed_0 = 192;
pub const TAG_MASK: C2RustUnnamed_0 = 31;
pub type C2RustUnnamed_0 = libc::c_uint;
unsafe extern "C" fn asn1_der_iterator_init(
    mut iterator: *mut asn1_der_iterator,
    mut length: size_t,
    mut input: *const uint8_t,
) {
    (*iterator).buffer_length = length;
    (*iterator).buffer = input;
    (*iterator).pos = 0 as libc::c_int as size_t;
    (*iterator).type_0 = 0 as asn1_type;
    (*iterator).length = 0 as libc::c_int as size_t;
    (*iterator).data = 0 as *const uint8_t;
}
pub unsafe extern "C" fn nettle_asn1_der_iterator_next(
    mut i: *mut asn1_der_iterator,
) -> asn1_iterator_result {
    let mut tag: uint8_t = 0;
    if ((*i).buffer_length).wrapping_sub((*i).pos) == 0 {
        return ASN1_ITERATOR_END;
    }
    let fresh0 = (*i).pos;
    (*i).pos = ((*i).pos).wrapping_add(1);
    tag = *((*i).buffer).offset(fresh0 as isize);
    if ((*i).buffer_length).wrapping_sub((*i).pos) == 0 {
        return ASN1_ITERATOR_ERROR;
    }
    if tag as libc::c_int & TAG_MASK as libc::c_int == TAG_MASK as libc::c_int {
        return ASN1_ITERATOR_ERROR;
    }
    let fresh1 = (*i).pos;
    (*i).pos = ((*i).pos).wrapping_add(1);
    (*i).length = *((*i).buffer).offset(fresh1 as isize) as size_t;
    if (*i).length & 0x80 as libc::c_int as libc::c_ulong != 0 {
        let mut k: libc::c_uint = ((*i).length & 0x7f as libc::c_int as libc::c_ulong)
            as libc::c_uint;
        let mut j: libc::c_uint = 0;
        let mut data: *const uint8_t = ((*i).buffer).offset((*i).pos as isize);
        if k == 0 as libc::c_int as libc::c_uint {
            return ASN1_ITERATOR_ERROR;
        }
        if ((*i).buffer_length).wrapping_sub((*i).pos) < k as libc::c_ulong {
            return ASN1_ITERATOR_ERROR;
        }
        if k as libc::c_ulong > ::std::mem::size_of::<size_t>() as libc::c_ulong {
            return ASN1_ITERATOR_ERROR;
        }
        (*i)
            .pos = ((*i).pos as libc::c_ulong).wrapping_add(k as libc::c_ulong) as size_t
            as size_t;
        (*i).length = *data.offset(0 as libc::c_int as isize) as size_t;
        if (*i).length == 0 as libc::c_int as libc::c_ulong
            || k == 1 as libc::c_int as libc::c_uint
                && (*i).length < 0x80 as libc::c_int as libc::c_ulong
        {
            return ASN1_ITERATOR_ERROR;
        }
        j = 1 as libc::c_int as libc::c_uint;
        while j < k {
            (*i)
                .length = (*i).length << 8 as libc::c_int
                | *data.offset(j as isize) as libc::c_ulong;
            j = j.wrapping_add(1);
            j;
        }
    }
    if ((*i).buffer_length).wrapping_sub((*i).pos) < (*i).length {
        return ASN1_ITERATOR_ERROR;
    }
    (*i).data = ((*i).buffer).offset((*i).pos as isize);
    (*i).pos = ((*i).pos as libc::c_ulong).wrapping_add((*i).length) as size_t as size_t;
    (*i).type_0 = (tag as libc::c_int & TAG_MASK as libc::c_int) as asn1_type;
    (*i)
        .type_0 = ::std::mem::transmute::<
        libc::c_uint,
        asn1_type,
    >(
        (*i).type_0 as libc::c_uint
            | ((tag as libc::c_int & CLASS_MASK as libc::c_int)
                << ASN1_CLASS_SHIFT as libc::c_int - 6 as libc::c_int) as libc::c_uint,
    );
    if tag as libc::c_int & CONSTRUCTED_MASK as libc::c_int != 0 {
        (*i)
            .type_0 = ::std::mem::transmute::<
            libc::c_uint,
            asn1_type,
        >(
            (*i).type_0 as libc::c_uint
                | ASN1_TYPE_CONSTRUCTED as libc::c_int as libc::c_uint,
        );
        return ASN1_ITERATOR_CONSTRUCTED;
    } else {
        return ASN1_ITERATOR_PRIMITIVE
    };
}
pub unsafe extern "C" fn nettle_asn1_der_iterator_first(
    mut i: *mut asn1_der_iterator,
    mut length: size_t,
    mut input: *const uint8_t,
) -> asn1_iterator_result {
    asn1_der_iterator_init(i, length, input);
    return nettle_asn1_der_iterator_next(i);
}
pub unsafe extern "C" fn nettle_asn1_der_decode_constructed(
    mut i: *mut asn1_der_iterator,
    mut contents: *mut asn1_der_iterator,
) -> asn1_iterator_result {
    if (*i).type_0 as libc::c_uint & ASN1_TYPE_CONSTRUCTED as libc::c_int as libc::c_uint
        != 0
    {} else {
        __assert_fail(
            b"i->type & ASN1_TYPE_CONSTRUCTED\0" as *const u8 as *const libc::c_char,
            b"der-iterator.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"enum asn1_iterator_result nettle_asn1_der_decode_constructed(struct asn1_der_iterator *, struct asn1_der_iterator *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4986: {
        if (*i).type_0 as libc::c_uint
            & ASN1_TYPE_CONSTRUCTED as libc::c_int as libc::c_uint != 0
        {} else {
            __assert_fail(
                b"i->type & ASN1_TYPE_CONSTRUCTED\0" as *const u8 as *const libc::c_char,
                b"der-iterator.c\0" as *const u8 as *const libc::c_char,
                183 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 117],
                    &[libc::c_char; 117],
                >(
                    b"enum asn1_iterator_result nettle_asn1_der_decode_constructed(struct asn1_der_iterator *, struct asn1_der_iterator *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_asn1_der_iterator_first(contents, (*i).length, (*i).data);
}
pub unsafe extern "C" fn nettle_asn1_der_decode_constructed_last(
    mut i: *mut asn1_der_iterator,
) -> asn1_iterator_result {
    if ((*i).buffer_length).wrapping_sub((*i).pos) > 0 as libc::c_int as libc::c_ulong {
        return ASN1_ITERATOR_ERROR;
    }
    return nettle_asn1_der_decode_constructed(i, i);
}
pub unsafe extern "C" fn nettle_asn1_der_decode_bitstring(
    mut i: *mut asn1_der_iterator,
    mut contents: *mut asn1_der_iterator,
) -> asn1_iterator_result {
    if (*i).type_0 as libc::c_uint == ASN1_BITSTRING as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"i->type == ASN1_BITSTRING\0" as *const u8 as *const libc::c_char,
            b"der-iterator.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"enum asn1_iterator_result nettle_asn1_der_decode_bitstring(struct asn1_der_iterator *, struct asn1_der_iterator *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5113: {
        if (*i).type_0 as libc::c_uint == ASN1_BITSTRING as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"i->type == ASN1_BITSTRING\0" as *const u8 as *const libc::c_char,
                b"der-iterator.c\0" as *const u8 as *const libc::c_char,
                201 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"enum asn1_iterator_result nettle_asn1_der_decode_bitstring(struct asn1_der_iterator *, struct asn1_der_iterator *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*i).length == 0 as libc::c_int as libc::c_ulong
        || *((*i).data).offset(0 as libc::c_int as isize) as libc::c_int
            != 0 as libc::c_int
    {
        return ASN1_ITERATOR_ERROR;
    }
    return nettle_asn1_der_iterator_first(
        contents,
        ((*i).length).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ((*i).data).offset(1 as libc::c_int as isize),
    );
}
pub unsafe extern "C" fn nettle_asn1_der_decode_bitstring_last(
    mut i: *mut asn1_der_iterator,
) -> asn1_iterator_result {
    if ((*i).buffer_length).wrapping_sub((*i).pos) > 0 as libc::c_int as libc::c_ulong {
        return ASN1_ITERATOR_ERROR;
    }
    return nettle_asn1_der_decode_bitstring(i, i);
}
pub unsafe extern "C" fn nettle_asn1_der_get_uint32(
    mut i: *mut asn1_der_iterator,
    mut x: *mut uint32_t,
) -> libc::c_int {
    let mut value: uint32_t = 0 as libc::c_int as uint32_t;
    let mut length: size_t = (*i).length;
    let mut k: libc::c_uint = 0;
    if length == 0 || length > 5 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if *((*i).data)
        .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int >= 0x80 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if length > 1 as libc::c_int as libc::c_ulong
        && *((*i).data)
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == 0 as libc::c_int
        && (*((*i).data)
            .offset(length.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int) < 0x80 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if length == 5 as libc::c_int as libc::c_ulong {
        if *((*i).data).offset(4 as libc::c_int as isize) != 0 {
            return 0 as libc::c_int;
        }
        length = length.wrapping_sub(1);
        length;
    }
    k = 0 as libc::c_int as libc::c_uint;
    value = k;
    while (k as libc::c_ulong) < length {
        value = value << 8 as libc::c_int
            | *((*i).data).offset(k as isize) as libc::c_uint;
        k = k.wrapping_add(1);
        k;
    }
    *x = value;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_asn1_der_get_bignum(
    mut i: *mut asn1_der_iterator,
    mut x: *mut __mpz_struct,
    mut max_bits: libc::c_uint,
) -> libc::c_int {
    if (*i).length > 1 as libc::c_int as libc::c_ulong
        && (*((*i).data).offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
            && (*((*i).data).offset(1 as libc::c_int as isize) as libc::c_int)
                < 0x80 as libc::c_int
            || *((*i).data).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xff as libc::c_int
                && *((*i).data).offset(1 as libc::c_int as isize) as libc::c_int
                    >= 0x80 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if max_bits != 0
        && (8 as libc::c_int as libc::c_ulong).wrapping_mul((*i).length)
            > (16 as libc::c_int as libc::c_uint).wrapping_add(max_bits) as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    nettle_mpz_set_str_256_s(x, (*i).length, (*i).data);
    if max_bits != 0
        && __gmpz_sizeinbase(x as mpz_srcptr, 2 as libc::c_int)
            > max_bits as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

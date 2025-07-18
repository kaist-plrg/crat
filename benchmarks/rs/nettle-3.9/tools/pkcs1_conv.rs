use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn abort() -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_asn1_der_decode_constructed_last(
        i: *mut asn1_der_iterator,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_iterator_first(
        iterator: *mut asn1_der_iterator,
        length: size_t,
        input: *const uint8_t,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_iterator_next(
        iterator: *mut asn1_der_iterator,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_decode_constructed(
        i: *mut asn1_der_iterator,
        contents: *mut asn1_der_iterator,
    ) -> asn1_iterator_result;
    fn nettle_asn1_der_decode_bitstring_last(
        i: *mut asn1_der_iterator,
    ) -> asn1_iterator_result;
    fn nettle_base64_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base64_decode_final(ctx: *mut base64_decode_ctx) -> libc::c_int;
    fn nettle_buffer_init_realloc(
        buffer: *mut nettle_buffer,
        realloc_ctx: *mut libc::c_void,
        realloc: Option::<nettle_realloc_func>,
    );
    fn nettle_buffer_clear(buffer: *mut nettle_buffer);
    fn nettle_xrealloc(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
    fn nettle_buffer_reset(buffer: *mut nettle_buffer);
    fn nettle_buffer_grow(buffer: *mut nettle_buffer, length: size_t) -> libc::c_int;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn nettle_rsa_public_key_init(key: *mut rsa_public_key);
    fn nettle_rsa_keypair_from_der(
        pub_0: *mut rsa_public_key,
        priv_0: *mut rsa_private_key,
        limit: libc::c_uint,
        length: size_t,
        data: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_rsa_keypair_to_sexp(
        buffer: *mut nettle_buffer,
        algorithm_name: *const libc::c_char,
        pub_0: *const rsa_public_key,
        priv_0: *const rsa_private_key,
    ) -> libc::c_int;
    fn nettle_rsa_public_key_from_der_iterator(
        pub_0: *mut rsa_public_key,
        limit: libc::c_uint,
        i: *mut asn1_der_iterator,
    ) -> libc::c_int;
    fn nettle_rsa_public_key_clear(key: *mut rsa_public_key);
    fn nettle_rsa_private_key_init(key: *mut rsa_private_key);
    fn nettle_rsa_private_key_clear(key: *mut rsa_private_key);
    fn nettle_dsa_params_init(params: *mut dsa_params);
    fn nettle_dsa_params_clear(params: *mut dsa_params);
    fn nettle_dsa_keypair_to_sexp(
        buffer: *mut nettle_buffer,
        algorithm_name: *const libc::c_char,
        params: *const dsa_params,
        pub_0: *const __mpz_struct,
        priv_0: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_dsa_params_from_der_iterator(
        params: *mut dsa_params,
        max_bits: libc::c_uint,
        q_bits: libc::c_uint,
        i: *mut asn1_der_iterator,
    ) -> libc::c_int;
    fn nettle_dsa_public_key_from_der_iterator(
        params: *const dsa_params,
        pub_0: *mut __mpz_struct,
        i: *mut asn1_der_iterator,
    ) -> libc::c_int;
    fn nettle_openssl_provate_key_from_der(
        params: *mut dsa_params,
        pub_0: *mut __mpz_struct,
        priv_0: *mut __mpz_struct,
        p_max_bits: libc::c_uint,
        length: size_t,
        data: *const uint8_t,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn die(format: *const libc::c_char, _: ...) -> !;
    fn werror(format: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
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
pub type asn1_iterator_result = libc::c_uint;
pub const ASN1_ITERATOR_END: asn1_iterator_result = 3;
pub const ASN1_ITERATOR_CONSTRUCTED: asn1_iterator_result = 2;
pub const ASN1_ITERATOR_PRIMITIVE: asn1_iterator_result = 1;
pub const ASN1_ITERATOR_ERROR: asn1_iterator_result = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type object_type = libc::c_uint;
pub const GENERAL_PUBLIC_KEY: object_type = 515;
pub const DSA_PRIVATE_KEY: object_type = 514;
pub const RSA_PUBLIC_KEY: object_type = 513;
pub const RSA_PRIVATE_KEY: object_type = 512;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pem_info {
    pub marker_start: size_t,
    pub marker_length: size_t,
    pub data_start: size_t,
    pub data_length: size_t,
}
pub const OPT_HELP: C2RustUnnamed = 768;
pub const OPT_PUBLIC_KEY: C2RustUnnamed = 515;
pub const OPT_PRIVATE_DSA: C2RustUnnamed = 514;
pub const OPT_PUBLIC_RSA: C2RustUnnamed = 513;
pub const OPT_PRIVATE_RSA: C2RustUnnamed = 512;
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn write_file(
    mut buffer: *mut nettle_buffer,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut res: size_t = fwrite(
        (*buffer).contents as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (*buffer).size,
        f,
    );
    if res < (*buffer).size {
        werror(
            b"Write failed: %s.\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn read_line(
    mut buffer: *mut nettle_buffer,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = getc(f);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh0 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh0 as isize) = c as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
        if c == '\n' as i32 {
            return 1 as libc::c_int;
        }
    }
    if ferror(f) != 0 {
        werror(
            b"Read failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn read_file(
    mut buffer: *mut nettle_buffer,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = getc(f);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh1 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh1 as isize) = c as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    if ferror(f) != 0 {
        werror(
            b"Read failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
static mut pem_start_pattern: [uint8_t; 11] = unsafe {
    *::std::mem::transmute::<&[u8; 11], &[uint8_t; 11]>(b"-----BEGIN ")
};
static mut pem_end_pattern: [uint8_t; 9] = unsafe {
    *::std::mem::transmute::<&[u8; 9], &[uint8_t; 9]>(b"-----END ")
};
static mut pem_trailer_pattern: [uint8_t; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &[uint8_t; 5]>(b"-----")
};
static mut pem_ws: [libc::c_char; 33] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn match_pem_start(
    mut length: size_t,
    mut line: *const uint8_t,
    mut marker_start: *mut size_t,
    mut marker_length: *mut size_t,
) -> libc::c_int {
    while length > 0 as libc::c_int as libc::c_ulong
        && ((*line
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_ulong)
            < ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
            && pem_ws[*line
                .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as usize] as libc::c_int != 0)
    {
        length = length.wrapping_sub(1);
        length;
    }
    if length
        > (::std::mem::size_of::<[uint8_t; 11]>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong)
        && memcmp(
            line as *const libc::c_void,
            pem_start_pattern.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[uint8_t; 11]>() as libc::c_ulong,
        ) == 0 as libc::c_int
        && memcmp(
            line
                .offset(length as isize)
                .offset(
                    -(::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong as isize),
                ) as *const libc::c_void,
            pem_trailer_pattern.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        *marker_start = 11 as libc::c_int as size_t;
        *marker_length = length
            .wrapping_sub(
                (::std::mem::size_of::<[uint8_t; 11]>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong),
            );
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn match_pem_end(
    mut length: size_t,
    mut line: *const uint8_t,
    mut marker_length: size_t,
    mut marker: *const uint8_t,
) -> libc::c_int {
    while length > 0 as libc::c_int as libc::c_ulong
        && ((*line
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_ulong)
            < ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
            && pem_ws[*line
                .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as usize] as libc::c_int != 0)
    {
        length = length.wrapping_sub(1);
        length;
    }
    if length
        > (::std::mem::size_of::<[uint8_t; 9]>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong)
        && memcmp(
            line as *const libc::c_void,
            pem_end_pattern.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[uint8_t; 9]>() as libc::c_ulong,
        ) == 0 as libc::c_int
        && memcmp(
            line
                .offset(length as isize)
                .offset(
                    -(::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong as isize),
                ) as *const libc::c_void,
            pem_trailer_pattern.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        if length
            == marker_length
                .wrapping_add(
                    (::std::mem::size_of::<[uint8_t; 9]>() as libc::c_ulong)
                        .wrapping_add(
                            ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong,
                        ),
                )
            && memcmp(
                line
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 9]>() as libc::c_ulong as isize,
                    ) as *const libc::c_void,
                marker as *const libc::c_void,
                marker_length,
            ) == 0 as libc::c_int
        {
            return 1 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn read_pem(
    mut buffer: *mut nettle_buffer,
    mut f: *mut FILE,
    mut info: *mut pem_info,
) -> libc::c_int {
    loop {
        let mut res: libc::c_int = 0;
        nettle_buffer_reset(buffer);
        res = read_line(buffer, f);
        if res != 1 as libc::c_int {
            return res;
        }
        if match_pem_start(
            (*buffer).size,
            (*buffer).contents,
            &mut (*info).marker_start,
            &mut (*info).marker_length,
        ) != 0
        {
            break;
        }
    }
    *((*buffer).contents)
        .offset(
            ((*info).marker_start).wrapping_add((*info).marker_length) as isize,
        ) = 0 as libc::c_int as uint8_t;
    (*info).data_start = (*buffer).size;
    loop {
        let mut line_start: size_t = (*buffer).size;
        if read_line(buffer, f) != 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        match match_pem_end(
            ((*buffer).size).wrapping_sub(line_start),
            ((*buffer).contents).offset(line_start as isize),
            (*info).marker_length,
            ((*buffer).contents).offset((*info).marker_start as isize),
        ) {
            -1 => {
                werror(
                    b"PEM END line doesn't match BEGIN.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            1 => {
                (*info).data_length = line_start.wrapping_sub((*info).data_start);
                return 1 as libc::c_int;
            }
            0 | _ => {}
        }
    };
}
#[inline]
unsafe extern "C" fn base64_decode_in_place(
    mut ctx: *mut base64_decode_ctx,
    mut dst_length: *mut size_t,
    mut length: size_t,
    mut data: *mut uint8_t,
) -> libc::c_int {
    return nettle_base64_decode_update(
        ctx,
        dst_length,
        data,
        length,
        data as *const libc::c_char,
    );
}
unsafe extern "C" fn decode_base64(
    mut buffer: *mut nettle_buffer,
    mut start: size_t,
    mut length: *mut size_t,
) -> libc::c_int {
    let mut ctx: base64_decode_ctx = base64_decode_ctx {
        table: 0 as *const libc::c_schar,
        word: 0,
        bits: 0,
        padding: 0,
    };
    nettle_base64_decode_init(&mut ctx);
    if base64_decode_in_place(
        &mut ctx,
        length,
        *length,
        ((*buffer).contents).offset(start as isize),
    ) != 0 && nettle_base64_decode_final(&mut ctx) != 0
    {
        return 1 as libc::c_int
    } else {
        werror(b"Invalid base64 date.\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn convert_rsa_public_key(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut pub_0: rsa_public_key = rsa_public_key {
        size: 0,
        n: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        e: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut res: libc::c_int = 0;
    nettle_rsa_public_key_init(&mut pub_0);
    if nettle_rsa_keypair_from_der(
        &mut pub_0,
        0 as *mut rsa_private_key,
        0 as libc::c_int as libc::c_uint,
        length,
        data,
    ) != 0
    {
        nettle_buffer_reset(buffer);
        res = nettle_rsa_keypair_to_sexp(
            buffer,
            0 as *const libc::c_char,
            &mut pub_0,
            0 as *const rsa_private_key,
        );
    } else {
        werror(b"Invalid PKCS#1 public key.\n\0" as *const u8 as *const libc::c_char);
        res = 0 as libc::c_int;
    }
    nettle_rsa_public_key_clear(&mut pub_0);
    return res;
}
unsafe extern "C" fn convert_rsa_private_key(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut pub_0: rsa_public_key = rsa_public_key {
        size: 0,
        n: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        e: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut priv_0: rsa_private_key = rsa_private_key {
        size: 0,
        d: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        p: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        q: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        a: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        b: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        c: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut res: libc::c_int = 0;
    nettle_rsa_public_key_init(&mut pub_0);
    nettle_rsa_private_key_init(&mut priv_0);
    if nettle_rsa_keypair_from_der(
        &mut pub_0,
        &mut priv_0,
        0 as libc::c_int as libc::c_uint,
        length,
        data,
    ) != 0
    {
        nettle_buffer_reset(buffer);
        res = nettle_rsa_keypair_to_sexp(
            buffer,
            0 as *const libc::c_char,
            &mut pub_0,
            &mut priv_0,
        );
    } else {
        werror(b"Invalid PKCS#1 private key.\n\0" as *const u8 as *const libc::c_char);
        res = 0 as libc::c_int;
    }
    nettle_rsa_public_key_clear(&mut pub_0);
    nettle_rsa_private_key_clear(&mut priv_0);
    return res;
}
unsafe extern "C" fn convert_dsa_private_key(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut params: dsa_params = dsa_params {
        p: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        q: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        g: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut pub_0: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut priv_0: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    nettle_dsa_params_init(&mut params);
    __gmpz_init(pub_0.as_mut_ptr());
    __gmpz_init(priv_0.as_mut_ptr());
    if nettle_openssl_provate_key_from_der(
        &mut params,
        pub_0.as_mut_ptr(),
        priv_0.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        length,
        data,
    ) != 0
    {
        nettle_buffer_reset(buffer);
        res = nettle_dsa_keypair_to_sexp(
            buffer,
            0 as *const libc::c_char,
            &mut params,
            pub_0.as_mut_ptr() as *const __mpz_struct,
            priv_0.as_mut_ptr() as *const __mpz_struct,
        );
    } else {
        werror(b"Invalid OpenSSL private key.\n\0" as *const u8 as *const libc::c_char);
        res = 0 as libc::c_int;
    }
    nettle_dsa_params_clear(&mut params);
    __gmpz_clear(pub_0.as_mut_ptr());
    __gmpz_clear(priv_0.as_mut_ptr());
    return res;
}
unsafe extern "C" fn convert_public_key(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: asn1_der_iterator = asn1_der_iterator {
        buffer_length: 0,
        buffer: 0 as *const uint8_t,
        pos: 0,
        type_0: 0 as asn1_type,
        length: 0,
        data: 0 as *const uint8_t,
    };
    let mut j: asn1_der_iterator = asn1_der_iterator {
        buffer_length: 0,
        buffer: 0 as *const uint8_t,
        pos: 0,
        type_0: 0 as asn1_type,
        length: 0,
        data: 0 as *const uint8_t,
    };
    let mut res: libc::c_int = 0 as libc::c_int;
    if nettle_asn1_der_iterator_first(&mut i, length, data) as libc::c_uint
        == ASN1_ITERATOR_CONSTRUCTED as libc::c_int as libc::c_uint
        && i.type_0 as libc::c_uint == ASN1_SEQUENCE as libc::c_int as libc::c_uint
        && nettle_asn1_der_decode_constructed_last(&mut i) as libc::c_uint
            == ASN1_ITERATOR_CONSTRUCTED as libc::c_int as libc::c_uint
        && i.type_0 as libc::c_uint == ASN1_SEQUENCE as libc::c_int as libc::c_uint
        && nettle_asn1_der_decode_constructed(&mut i, &mut j) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
        && j.type_0 as libc::c_uint == ASN1_IDENTIFIER as libc::c_int as libc::c_uint
        && nettle_asn1_der_iterator_next(&mut i) as libc::c_uint
            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
        && i.type_0 as libc::c_uint == ASN1_BITSTRING as libc::c_int as libc::c_uint
        && nettle_asn1_der_decode_bitstring_last(&mut i) as libc::c_uint != 0
    {
        static mut id_rsaEncryption: [uint8_t; 9] = [
            0x2a as libc::c_int as uint8_t,
            0x86 as libc::c_int as uint8_t,
            0x48 as libc::c_int as uint8_t,
            0x86 as libc::c_int as uint8_t,
            0xf7 as libc::c_int as uint8_t,
            0xd as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
        ];
        static mut id_dsa: [uint8_t; 7] = [
            0x2a as libc::c_int as uint8_t,
            0x86 as libc::c_int as uint8_t,
            0x48 as libc::c_int as uint8_t,
            0xce as libc::c_int as uint8_t,
            0x38 as libc::c_int as uint8_t,
            0x4 as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
        ];
        match j.length {
            7 => {
                if memcmp(
                    j.data as *const libc::c_void,
                    id_dsa.as_ptr() as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if nettle_asn1_der_iterator_next(&mut j) as libc::c_uint
                        == ASN1_ITERATOR_CONSTRUCTED as libc::c_int as libc::c_uint
                        && nettle_asn1_der_decode_constructed_last(&mut j)
                            as libc::c_uint
                            == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
                    {
                        let mut params: dsa_params = dsa_params {
                            p: [__mpz_struct {
                                _mp_alloc: 0,
                                _mp_size: 0,
                                _mp_d: 0 as *mut mp_limb_t,
                            }; 1],
                            q: [__mpz_struct {
                                _mp_alloc: 0,
                                _mp_size: 0,
                                _mp_d: 0 as *mut mp_limb_t,
                            }; 1],
                            g: [__mpz_struct {
                                _mp_alloc: 0,
                                _mp_size: 0,
                                _mp_d: 0 as *mut mp_limb_t,
                            }; 1],
                        };
                        let mut pub_0: mpz_t = [__mpz_struct {
                            _mp_alloc: 0,
                            _mp_size: 0,
                            _mp_d: 0 as *mut mp_limb_t,
                        }; 1];
                        nettle_dsa_params_init(&mut params);
                        __gmpz_init(pub_0.as_mut_ptr());
                        if nettle_dsa_params_from_der_iterator(
                            &mut params,
                            0 as libc::c_int as libc::c_uint,
                            0 as libc::c_int as libc::c_uint,
                            &mut i,
                        ) != 0
                            && nettle_dsa_public_key_from_der_iterator(
                                &mut params,
                                pub_0.as_mut_ptr(),
                                &mut j,
                            ) != 0
                        {
                            nettle_buffer_reset(buffer);
                            res = (nettle_dsa_keypair_to_sexp(
                                buffer,
                                0 as *const libc::c_char,
                                &mut params,
                                pub_0.as_mut_ptr() as *const __mpz_struct,
                                0 as *const __mpz_struct,
                            ) > 0 as libc::c_int) as libc::c_int;
                        }
                        nettle_dsa_params_clear(&mut params);
                        __gmpz_clear(pub_0.as_mut_ptr());
                    }
                    if res == 0 {
                        werror(
                            b"SubjectPublicKeyInfo: Invalid DSA key.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    current_block = 5689316957504528238;
                } else {
                    current_block = 2981646806506858626;
                }
            }
            9 => {
                if memcmp(
                    j.data as *const libc::c_void,
                    id_rsaEncryption.as_ptr() as *const libc::c_void,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if nettle_asn1_der_iterator_next(&mut j) as libc::c_uint
                        == ASN1_ITERATOR_PRIMITIVE as libc::c_int as libc::c_uint
                        && j.type_0 as libc::c_uint
                            == ASN1_NULL as libc::c_int as libc::c_uint
                        && j.length == 0 as libc::c_int as libc::c_ulong
                        && nettle_asn1_der_iterator_next(&mut j) as libc::c_uint
                            == ASN1_ITERATOR_END as libc::c_int as libc::c_uint
                    {
                        let mut pub_1: rsa_public_key = rsa_public_key {
                            size: 0,
                            n: [__mpz_struct {
                                _mp_alloc: 0,
                                _mp_size: 0,
                                _mp_d: 0 as *mut mp_limb_t,
                            }; 1],
                            e: [__mpz_struct {
                                _mp_alloc: 0,
                                _mp_size: 0,
                                _mp_d: 0 as *mut mp_limb_t,
                            }; 1],
                        };
                        nettle_rsa_public_key_init(&mut pub_1);
                        if nettle_rsa_public_key_from_der_iterator(
                            &mut pub_1,
                            0 as libc::c_int as libc::c_uint,
                            &mut i,
                        ) != 0
                        {
                            nettle_buffer_reset(buffer);
                            res = (nettle_rsa_keypair_to_sexp(
                                buffer,
                                0 as *const libc::c_char,
                                &mut pub_1,
                                0 as *const rsa_private_key,
                            ) > 0 as libc::c_int) as libc::c_int;
                        }
                        nettle_rsa_public_key_clear(&mut pub_1);
                    }
                    if res == 0 {
                        werror(
                            b"SubjectPublicKeyInfo: Invalid RSA key.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    current_block = 5689316957504528238;
                } else {
                    current_block = 2981646806506858626;
                }
            }
            _ => {
                current_block = 2981646806506858626;
            }
        }
        match current_block {
            5689316957504528238 => {}
            _ => {
                werror(
                    b"SubjectPublicKeyInfo: Unsupported algorithm.\n\0" as *const u8
                        as *const libc::c_char,
                );
                res = -(1 as libc::c_int);
            }
        }
    } else {
        werror(
            b"SubjectPublicKeyInfo: Invalid object.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return res;
}
unsafe extern "C" fn convert_type(
    mut buffer: *mut nettle_buffer,
    mut type_0: object_type,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    match type_0 as libc::c_uint {
        515 => {
            res = convert_public_key(buffer, length, data);
        }
        513 => {
            res = convert_rsa_public_key(buffer, length, data);
        }
        512 => {
            res = convert_rsa_private_key(buffer, length, data);
        }
        514 => {
            res = convert_dsa_private_key(buffer, length, data);
        }
        _ => {
            abort();
        }
    }
    if res > 0 as libc::c_int {
        res = write_file(buffer, stdout);
    }
    return res;
}
unsafe extern "C" fn convert_file(
    mut buffer: *mut nettle_buffer,
    mut f: *mut FILE,
    mut type_0: object_type,
    mut base64: libc::c_int,
) -> libc::c_int {
    if type_0 as u64 != 0 {
        read_file(buffer, f);
        if base64 != 0
            && decode_base64(buffer, 0 as libc::c_int as size_t, &mut (*buffer).size)
                == 0
        {
            return 0 as libc::c_int;
        }
        if convert_type(buffer, type_0, (*buffer).size, (*buffer).contents)
            != 1 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    } else {
        loop {
            let mut info: pem_info = pem_info {
                marker_start: 0,
                marker_length: 0,
                data_start: 0,
                data_length: 0,
            };
            let mut marker: *const uint8_t = 0 as *const uint8_t;
            nettle_buffer_reset(buffer);
            match read_pem(buffer, f, &mut info) {
                1 => {}
                -1 => return 1 as libc::c_int,
                _ => return 0 as libc::c_int,
            }
            if decode_base64(buffer, info.data_start, &mut info.data_length) == 0 {
                return 0 as libc::c_int;
            }
            marker = ((*buffer).contents).offset(info.marker_start as isize);
            type_0 = 0 as object_type;
            match info.marker_length {
                10 => {
                    if memcmp(
                        marker as *const libc::c_void,
                        b"PUBLIC KEY\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        type_0 = GENERAL_PUBLIC_KEY;
                    }
                }
                14 => {
                    if memcmp(
                        marker as *const libc::c_void,
                        b"RSA PUBLIC KEY\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        14 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        type_0 = RSA_PUBLIC_KEY;
                    }
                }
                15 => {
                    if memcmp(
                        marker as *const libc::c_void,
                        b"RSA PRIVATE KEY\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        15 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        type_0 = RSA_PRIVATE_KEY;
                    } else if memcmp(
                        marker as *const libc::c_void,
                        b"DSA PRIVATE KEY\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        15 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        type_0 = DSA_PRIVATE_KEY;
                    }
                }
                _ => {}
            }
            if type_0 as u64 == 0 {
                werror(
                    b"Ignoring unsupported object type `%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    marker,
                );
            } else if convert_type(
                buffer,
                type_0,
                info.data_length,
                ((*buffer).contents).offset(info.data_start as isize),
            ) != 1 as libc::c_int
            {
                return 0 as libc::c_int
            }
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buffer: nettle_buffer = nettle_buffer {
        contents: 0 as *mut uint8_t,
        alloc: 0,
        realloc_ctx: 0 as *mut libc::c_void,
        realloc: None,
        size: 0,
    };
    let mut type_0: object_type = 0 as object_type;
    let mut base64: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    static mut options: [option; 8] = [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_HELP as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"private-rsa-key\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_PRIVATE_RSA as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"public-rsa-key\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_PUBLIC_RSA as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"private-dsa-key\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_PRIVATE_DSA as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"public-key-info\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_PUBLIC_KEY as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"base-64\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    loop {
        c = getopt_long(
            argc,
            argv,
            b"Vb\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                base64 = 1 as libc::c_int;
            }
            512 | 513 | 514 | 515 => {
                type_0 = c as object_type;
            }
            768 => {
                printf(
                    b"FIXME: Usage information.\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            63 => return 1 as libc::c_int,
            86 => {
                printf(
                    b"pkcs1-conv (nettle 3.9)\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            _ => {
                abort();
            }
        }
    }
    nettle_buffer_init_realloc(
        &mut buffer,
        0 as *mut libc::c_void,
        Some(nettle_xrealloc as nettle_realloc_func),
    );
    if optind == argc {
        if convert_file(&mut buffer, stdin, type_0, base64) == 0 {
            return 1 as libc::c_int;
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut mode: *const libc::c_char = if type_0 as libc::c_uint != 0 || base64 != 0
        {
            b"r\0" as *const u8 as *const libc::c_char
        } else {
            b"rb\0" as *const u8 as *const libc::c_char
        };
        i = optind;
        while i < argc {
            let mut f: *mut FILE = fopen(*argv.offset(i as isize), mode);
            if f.is_null() {
                die(
                    b"Failed to open `%s': %s.\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            if convert_file(&mut buffer, f, type_0, base64) == 0 {
                return 1 as libc::c_int;
            }
            fclose(f);
            i += 1;
            i;
        }
    }
    nettle_buffer_clear(&mut buffer);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

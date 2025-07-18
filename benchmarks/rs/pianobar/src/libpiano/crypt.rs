use ::libc;
extern "C" {
    pub type gcry_cipher_handle;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gcry_cipher_encrypt(
        h: gcry_cipher_hd_t,
        out: *mut libc::c_void,
        outsize: size_t,
        in_0: *const libc::c_void,
        inlen: size_t,
    ) -> gcry_error_t;
    fn gcry_cipher_decrypt(
        h: gcry_cipher_hd_t,
        out: *mut libc::c_void,
        outsize: size_t,
        in_0: *const libc::c_void,
        inlen: size_t,
    ) -> gcry_error_t;
}
pub type size_t = libc::c_ulong;
pub type gpg_error_t = libc::c_uint;
pub type gcry_error_t = gpg_error_t;
pub type gcry_cipher_hd_t = *mut gcry_cipher_handle;
pub unsafe extern "C" fn PianoDecryptString(
    mut h: gcry_cipher_hd_t,
    input: *const libc::c_char,
    retSize: *mut size_t,
) -> *mut libc::c_char {
    let mut inputLen: size_t = strlen(input);
    let mut gret: gcry_error_t = 0;
    let mut output: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut outputLen: size_t = inputLen.wrapping_div(2 as libc::c_int as libc::c_ulong);
    output = calloc(
        outputLen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < outputLen {
        let mut hex: [libc::c_char; 3] = [0; 3];
        memcpy(
            hex.as_mut_ptr() as *mut libc::c_void,
            &*input.offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
                as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        hex[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        *output
            .offset(
                i as isize,
            ) = strtol(hex.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16 as libc::c_int)
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    gret = gcry_cipher_decrypt(
        h,
        output as *mut libc::c_void,
        outputLen,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    if gret != 0 {
        free(output as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *retSize = outputLen;
    return output as *mut libc::c_char;
}
pub unsafe extern "C" fn PianoEncryptString(
    mut h: gcry_cipher_hd_t,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut paddedInput: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hexOutput: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut inputLen: size_t = strlen(s);
    let mut paddedInputLen: size_t = if inputLen
        .wrapping_rem(8 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        inputLen
    } else {
        inputLen
            .wrapping_add(
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        inputLen.wrapping_rem(8 as libc::c_int as libc::c_ulong),
                    ),
            )
    };
    let mut gret: gcry_error_t = 0;
    paddedInput = calloc(
        paddedInputLen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    memcpy(paddedInput as *mut libc::c_void, s as *const libc::c_void, inputLen);
    gret = gcry_cipher_encrypt(
        h,
        paddedInput as *mut libc::c_void,
        paddedInputLen,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    if gret != 0 {
        free(paddedInput as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    hexOutput = calloc(
        paddedInputLen
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < paddedInputLen {
        snprintf(
            &mut *hexOutput
                .offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
                as *mut libc::c_uchar as *mut libc::c_char,
            3 as libc::c_int as libc::c_ulong,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *paddedInput.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    free(paddedInput as *mut libc::c_void);
    return hexOutput as *mut libc::c_char;
}

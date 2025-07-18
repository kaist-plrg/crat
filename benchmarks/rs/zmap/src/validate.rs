use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn rijndaelKeySetupEnc(
        rk: *mut u32_0,
        cipherKey: *const u8_0,
        keyBits: libc::c_int,
    ) -> libc::c_int;
    fn rijndaelEncrypt(
        rk: *const u32_0,
        Nr: libc::c_int,
        pt: *const u8_0,
        ct: *mut u8_0,
    );
    fn random_bytes(dst: *mut libc::c_void, n: size_t) -> libc::c_int;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
pub type size_t = libc::c_ulong;
static mut inited: libc::c_int = 0 as libc::c_int;
static mut aes_sched: [uint32_t; 44] = [0; 44];
pub unsafe extern "C" fn validate_init() {
    let mut key: [uint8_t; 16] = [0; 16];
    if random_bytes(key.as_mut_ptr() as *mut libc::c_void, 16 as libc::c_int as size_t)
        == 0
    {
        log_fatal(
            b"validate\0" as *const u8 as *const libc::c_char,
            b"couldn't get random bytes\0" as *const u8 as *const libc::c_char,
        );
    }
    if rijndaelKeySetupEnc(
        aes_sched.as_mut_ptr(),
        key.as_mut_ptr() as *const u8_0,
        16 as libc::c_int * 8 as libc::c_int,
    ) != 10 as libc::c_int
    {
        log_fatal(
            b"validate\0" as *const u8 as *const libc::c_char,
            b"couldn't initialize AES key\0" as *const u8 as *const libc::c_char,
        );
    }
    inited = 1 as libc::c_int;
}
pub unsafe extern "C" fn validate_gen(
    src: uint32_t,
    dst: uint32_t,
    mut output: *mut uint8_t,
) {
    validate_gen_ex(
        src,
        dst,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        output,
    );
}
pub unsafe extern "C" fn validate_gen_ex(
    input0: uint32_t,
    input1: uint32_t,
    input2: uint32_t,
    input3: uint32_t,
    mut output: *mut uint8_t,
) {
    if inited != 0 {} else {
        __assert_fail(
            b"inited\0" as *const u8 as *const libc::c_char,
            b"validate.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void validate_gen_ex(const uint32_t, const uint32_t, const uint32_t, const uint32_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2229: {
        if inited != 0 {} else {
            __assert_fail(
                b"inited\0" as *const u8 as *const libc::c_char,
                b"validate.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void validate_gen_ex(const uint32_t, const uint32_t, const uint32_t, const uint32_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut aes_input: [uint32_t; 4] = [0; 4];
    aes_input[0 as libc::c_int as usize] = input0;
    aes_input[1 as libc::c_int as usize] = input1;
    aes_input[2 as libc::c_int as usize] = input2;
    aes_input[3 as libc::c_int as usize] = input3;
    rijndaelEncrypt(
        aes_sched.as_mut_ptr() as *const u32_0,
        10 as libc::c_int,
        aes_input.as_mut_ptr() as *mut uint8_t as *const u8_0,
        output,
    );
}

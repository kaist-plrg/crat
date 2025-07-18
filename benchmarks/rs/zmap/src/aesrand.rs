use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
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
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesrand {
    pub input: [uint32_t; 4],
    pub sched: [uint32_t; 44],
    pub output: [uint8_t; 16],
}
pub type aesrand_t = aesrand;
unsafe extern "C" fn _aesrand_init(mut key: *mut uint8_t) -> *mut aesrand_t {
    let mut aes: *mut aesrand_t = xmalloc(
        ::std::mem::size_of::<aesrand_t>() as libc::c_ulong,
    ) as *mut aesrand_t;
    memset(
        &mut (*aes).input as *mut [uint32_t; 4] as *mut libc::c_void,
        0 as libc::c_int,
        (4 as libc::c_int * 4 as libc::c_int) as libc::c_ulong,
    );
    if rijndaelKeySetupEnc(
        ((*aes).sched).as_mut_ptr(),
        key as *const u8_0,
        16 as libc::c_int * 8 as libc::c_int,
    ) != 10 as libc::c_int
    {
        log_fatal(
            b"aesrand\0" as *const u8 as *const libc::c_char,
            b"could not initialize AES key\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        ((*aes).output).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    return aes;
}
pub unsafe extern "C" fn aesrand_init_from_seed(mut seed: uint64_t) -> *mut aesrand_t {
    let mut key: [uint8_t; 16] = [0; 16];
    memset(
        key.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_ulong) < ::std::mem::size_of::<uint64_t>() as libc::c_ulong {
        key[i
            as usize] = (seed >> 8 as libc::c_int * i as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    return _aesrand_init(key.as_mut_ptr());
}
pub unsafe extern "C" fn aesrand_init_from_random() -> *mut aesrand_t {
    let mut key: [uint8_t; 16] = [0; 16];
    if random_bytes(key.as_mut_ptr() as *mut libc::c_void, 16 as libc::c_int as size_t)
        == 0
    {
        log_fatal(
            b"aesrand\0" as *const u8 as *const libc::c_char,
            b"Couldn't get random bytes\0" as *const u8 as *const libc::c_char,
        );
    }
    return _aesrand_init(key.as_mut_ptr());
}
pub unsafe extern "C" fn aesrand_getword(mut aes: *mut aesrand_t) -> uint64_t {
    memcpy(
        ((*aes).input).as_mut_ptr() as *mut libc::c_void,
        ((*aes).output).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
    rijndaelEncrypt(
        ((*aes).sched).as_mut_ptr() as *const u32_0,
        10 as libc::c_int,
        ((*aes).input).as_mut_ptr() as *mut uint8_t as *const u8_0,
        ((*aes).output).as_mut_ptr(),
    );
    let mut retval: uint64_t = 0;
    memcpy(
        &mut retval as *mut uint64_t as *mut libc::c_void,
        ((*aes).output).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
    return retval;
}

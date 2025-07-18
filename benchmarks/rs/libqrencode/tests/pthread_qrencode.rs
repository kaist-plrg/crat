use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn QRcode_encodeString(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> *mut QRcode;
    fn QRcode_free(qrcode: *mut QRcode);
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type QRencodeMode = libc::c_int;
pub const QR_MODE_FNC1SECOND: QRencodeMode = 7;
pub const QR_MODE_FNC1FIRST: QRencodeMode = 6;
pub const QR_MODE_ECI: QRencodeMode = 5;
pub const QR_MODE_STRUCTURE: QRencodeMode = 4;
pub const QR_MODE_KANJI: QRencodeMode = 3;
pub const QR_MODE_8: QRencodeMode = 2;
pub const QR_MODE_AN: QRencodeMode = 1;
pub const QR_MODE_NUM: QRencodeMode = 0;
pub const QR_MODE_NUL: QRencodeMode = -1;
pub type QRecLevel = libc::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRcode {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
static mut threads: [pthread_t; 10] = [0; 10];
static mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
unsafe extern "C" fn timerStart(mut str: *const libc::c_char) {
    printf(b"%s: START\n\0" as *const u8 as *const libc::c_char, str);
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
}
unsafe extern "C" fn timerStop() {
    let mut tc: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tc, 0 as *mut libc::c_void);
    printf(
        b"STOP: %ld msec\n\0" as *const u8 as *const libc::c_char,
        (tc.tv_sec - tv.tv_sec) * 1000 as libc::c_int as libc::c_long
            + (tc.tv_usec - tv.tv_usec) / 1000 as libc::c_int as libc::c_long,
    );
}
unsafe extern "C" fn encode_ver1to10(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut i: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    static mut data: *const libc::c_char = b"This is test.\0" as *const u8
        as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        version = 0 as libc::c_int;
        while version < 11 as libc::c_int {
            code = QRcode_encodeString(
                data,
                version,
                QR_ECLEVEL_L,
                QR_MODE_8,
                0 as libc::c_int,
            );
            if code.is_null() {
                perror(b"Failed to encode:\0" as *const u8 as *const libc::c_char);
            } else {
                QRcode_free(code);
            }
            version += 1;
            version;
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn prof_ver1to10() {
    let mut i: libc::c_int = 0;
    timerStart(
        b"Version 1 - 10 (500 symbols for each)\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pthread_create(
            &mut *threads.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                encode_ver1to10
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            0 as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pthread_join(threads[i as usize], 0 as *mut *mut libc::c_void);
        i += 1;
        i;
    }
    timerStop();
}
unsafe extern "C" fn encode_ver31to40(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut i: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    static mut data: *const libc::c_char = b"This is test.\0" as *const u8
        as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        version = 31 as libc::c_int;
        while version < 41 as libc::c_int {
            code = QRcode_encodeString(
                data,
                version,
                QR_ECLEVEL_L,
                QR_MODE_8,
                0 as libc::c_int,
            );
            if code.is_null() {
                perror(b"Failed to encode:\0" as *const u8 as *const libc::c_char);
            } else {
                QRcode_free(code);
            }
            version += 1;
            version;
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn prof_ver31to40() {
    let mut i: libc::c_int = 0;
    timerStart(
        b"Version 31 - 40 (50 symbols for each)\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pthread_create(
            &mut *threads.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                encode_ver31to40
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            0 as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pthread_join(threads[i as usize], 0 as *mut *mut libc::c_void);
        i += 1;
        i;
    }
    timerStop();
}
unsafe fn main_0() -> libc::c_int {
    prof_ver1to10();
    prof_ver31to40();
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

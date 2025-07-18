use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
static mut mutexes: *mut *mut pthread_mutex_t = 0 as *const *mut pthread_mutex_t
    as *mut *mut pthread_mutex_t;
unsafe extern "C" fn get_mutex(mut fd: libc::c_int) -> *mut pthread_mutex_t {
    if fd < 3 as libc::c_int
        && !(b"todo: implement generically\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"fd < 3 && \"todo: implement generically\"\0" as *const u8
                as *const libc::c_char,
            b"lockfd.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"pthread_mutex_t *get_mutex(int)\0"))
                .as_ptr(),
        );
    }
    'c_3069: {
        if fd < 3 as libc::c_int
            && !(b"todo: implement generically\0" as *const u8 as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"fd < 3 && \"todo: implement generically\"\0" as *const u8
                    as *const libc::c_char,
                b"lockfd.c\0" as *const u8 as *const libc::c_char,
                29 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"pthread_mutex_t *get_mutex(int)\0"))
                    .as_ptr(),
            );
        }
    };
    if mutexes.is_null() {
        mutexes = xmalloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut pthread_mutex_t;
        if !mutexes.is_null() {} else {
            __assert_fail(
                b"mutexes\0" as *const u8 as *const libc::c_char,
                b"lockfd.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"pthread_mutex_t *get_mutex(int)\0"))
                    .as_ptr(),
            );
        }
        'c_3024: {
            if !mutexes.is_null() {} else {
                __assert_fail(
                    b"mutexes\0" as *const u8 as *const libc::c_char,
                    b"lockfd.c\0" as *const u8 as *const libc::c_char,
                    32 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"pthread_mutex_t *get_mutex(int)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if (*mutexes.offset(fd as isize)).is_null() {
        let ref mut fresh0 = *mutexes.offset(fd as isize);
        *fresh0 = xmalloc(::std::mem::size_of::<pthread_mutex_t>() as libc::c_ulong)
            as *mut pthread_mutex_t;
        if !(*mutexes.offset(fd as isize)).is_null() {} else {
            __assert_fail(
                b"mutexes[fd]\0" as *const u8 as *const libc::c_char,
                b"lockfd.c\0" as *const u8 as *const libc::c_char,
                36 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"pthread_mutex_t *get_mutex(int)\0"))
                    .as_ptr(),
            );
        }
        'c_2962: {
            if !(*mutexes.offset(fd as isize)).is_null() {} else {
                __assert_fail(
                    b"mutexes[fd]\0" as *const u8 as *const libc::c_char,
                    b"lockfd.c\0" as *const u8 as *const libc::c_char,
                    36 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"pthread_mutex_t *get_mutex(int)\0"))
                        .as_ptr(),
                );
            }
        };
        pthread_mutex_init(
            *mutexes.offset(fd as isize),
            0 as *const pthread_mutexattr_t,
        );
        if !(*mutexes.offset(fd as isize)).is_null() {} else {
            __assert_fail(
                b"mutexes[fd]\0" as *const u8 as *const libc::c_char,
                b"lockfd.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"pthread_mutex_t *get_mutex(int)\0"))
                    .as_ptr(),
            );
        }
        'c_2903: {
            if !(*mutexes.offset(fd as isize)).is_null() {} else {
                __assert_fail(
                    b"mutexes[fd]\0" as *const u8 as *const libc::c_char,
                    b"lockfd.c\0" as *const u8 as *const libc::c_char,
                    38 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"pthread_mutex_t *get_mutex(int)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    return *mutexes.offset(fd as isize);
}
pub unsafe extern "C" fn lock_fd(mut fd: libc::c_int) -> libc::c_int {
    return pthread_mutex_lock(get_mutex(fd));
}
pub unsafe extern "C" fn unlock_fd(mut fd: libc::c_int) -> libc::c_int {
    return pthread_mutex_unlock(get_mutex(fd));
}
pub unsafe extern "C" fn lock_file(mut f: *mut FILE) -> libc::c_int {
    if !f.is_null() {} else {
        __assert_fail(
            b"f\0" as *const u8 as *const libc::c_char,
            b"lockfd.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"int lock_file(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3153: {
        if !f.is_null() {} else {
            __assert_fail(
                b"f\0" as *const u8 as *const libc::c_char,
                b"lockfd.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"int lock_file(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    return lock_fd(fileno(f));
}
pub unsafe extern "C" fn unlock_file(mut f: *mut FILE) -> libc::c_int {
    if !f.is_null() {} else {
        __assert_fail(
            b"f\0" as *const u8 as *const libc::c_char,
            b"lockfd.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"int unlock_file(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3200: {
        if !f.is_null() {} else {
            __assert_fail(
                b"f\0" as *const u8 as *const libc::c_char,
                b"lockfd.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"int unlock_file(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    return unlock_fd(fileno(f));
}

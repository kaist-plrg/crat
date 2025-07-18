use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uintmax_t = __uintmax_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cycle_check_state {
    pub dev_ino: dev_ino,
    pub chdir_counter: uintmax_t,
    pub magic: libc::c_int,
}
unsafe extern "C" fn is_zero_or_power_of_two(mut i: uintmax_t) -> bool {
    return i & i.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn cycle_check_init(mut state: *mut cycle_check_state) {
    (*state).chdir_counter = 0 as libc::c_int as uintmax_t;
    (*state).magic = 9827862 as libc::c_int;
}
pub unsafe extern "C" fn cycle_check(
    mut state: *mut cycle_check_state,
    mut sb: *const stat,
) -> bool {
    if (*state).magic == 9827862 as libc::c_int {} else {
        __assert_fail(
            b"state->magic == 9827862\0" as *const u8 as *const libc::c_char,
            b"cycle-check.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool cycle_check(struct cycle_check_state *, const struct stat *)\0"))
                .as_ptr(),
        );
    }
    'c_1454: {
        if (*state).magic == 9827862 as libc::c_int {} else {
            __assert_fail(
                b"state->magic == 9827862\0" as *const u8 as *const libc::c_char,
                b"cycle-check.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool cycle_check(struct cycle_check_state *, const struct stat *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*state).chdir_counter != 0
        && ((*sb).st_ino == (*state).dev_ino.st_ino
            && (*sb).st_dev == (*state).dev_ino.st_dev)
    {
        return 1 as libc::c_int != 0;
    }
    (*state).chdir_counter = ((*state).chdir_counter).wrapping_add(1);
    if is_zero_or_power_of_two((*state).chdir_counter) {
        if (*state).chdir_counter == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int != 0;
        }
        (*state).dev_ino.st_dev = (*sb).st_dev;
        (*state).dev_ino.st_ino = (*sb).st_ino;
    }
    return 0 as libc::c_int != 0;
}

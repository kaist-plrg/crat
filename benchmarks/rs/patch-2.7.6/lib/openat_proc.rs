use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub const PROC_SELF_FD_DIR_SIZE_BOUND: C2RustUnnamed = 27;
pub type C2RustUnnamed = libc::c_uint;
pub unsafe extern "C" fn openat_proc_name(
    mut buf: *mut libc::c_char,
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = buf;
    let mut dirlen: libc::c_int = 0;
    if *file == 0 {
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return buf;
    }
    static mut proc_status: libc::c_int = 0 as libc::c_int;
    if proc_status == 0 {
        let mut proc_self_fd: libc::c_int = open(
            b"/proc/self/fd\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int | 0o200000 as libc::c_int | 0o400 as libc::c_int
                | 0o4000 as libc::c_int,
        );
        if proc_self_fd < 0 as libc::c_int {
            proc_status = -(1 as libc::c_int);
        } else {
            let mut dotdot_buf: [libc::c_char; 32] = [0; 32];
            sprintf(
                dotdot_buf.as_mut_ptr(),
                b"/proc/self/fd/%d/../fd\0" as *const u8 as *const libc::c_char,
                proc_self_fd,
            );
            proc_status = if access(dotdot_buf.as_mut_ptr(), 0 as libc::c_int) != 0 {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            };
            close(proc_self_fd);
        }
    }
    if proc_status < 0 as libc::c_int {
        return 0 as *mut libc::c_char
    } else {
        let mut bufsize: size_t = (PROC_SELF_FD_DIR_SIZE_BOUND as libc::c_int
            as libc::c_ulong)
            .wrapping_add(strlen(file));
        if ((if (4096 as libc::c_int) < 4096 as libc::c_int - 64 as libc::c_int {
            4096 as libc::c_int
        } else {
            4096 as libc::c_int - 64 as libc::c_int
        }) as libc::c_ulong) < bufsize
        {
            result = malloc(bufsize) as *mut libc::c_char;
            if result.is_null() {
                return 0 as *mut libc::c_char;
            }
        }
        dirlen = sprintf(
            result,
            b"/proc/self/fd/%d/\0" as *const u8 as *const libc::c_char,
            fd,
        );
    }
    strcpy(result.offset(dirlen as isize), file);
    return result;
}

use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn destroy_privs();
}
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn safePopenOut(
    mut command: *const *const libc::c_char,
    mut output: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    let mut last: ssize_t = 0;
    if pipe(pipefd.as_mut_ptr()) != 0 {
        return -(2 as libc::c_int) as ssize_t;
    }
    pid = fork();
    match pid {
        -1 => return -(2 as libc::c_int) as ssize_t,
        0 => {
            close(pipefd[0 as libc::c_int as usize]);
            destroy_privs();
            close(1 as libc::c_int);
            close(2 as libc::c_int);
            if dup(pipefd[1 as libc::c_int as usize]) < 0 as libc::c_int {
                perror(b"Dup error\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            close(pipefd[1 as libc::c_int as usize]);
            execvp(
                *command.offset(0 as libc::c_int as isize),
                command.offset(1 as libc::c_int as isize) as *const *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        _ => {
            close(pipefd[1 as libc::c_int as usize]);
        }
    }
    last = read(pipefd[0 as libc::c_int as usize], output as *mut libc::c_void, len);
    kill(pid, 9 as libc::c_int);
    wait(&mut status);
    if last < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    return last;
}
pub unsafe extern "C" fn expand(
    mut input: *const libc::c_char,
    mut ans: *mut libc::c_char,
) -> *const libc::c_char {
    let mut last: ssize_t = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut command: [*const libc::c_char; 5] = [
        b"/bin/sh\0" as *const u8 as *const libc::c_char,
        b"sh\0" as *const u8 as *const libc::c_char,
        b"-c\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    *ans
        .offset(
            (2048 as libc::c_int - 1 as libc::c_int) as isize,
        ) = '\0' as i32 as libc::c_char;
    if input.is_null() {
        return 0 as *const libc::c_char;
    }
    if *input as libc::c_int == '\0' as i32 {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    if (strpbrk(input, b"$*(){}[]\\?`~\0" as *const u8 as *const libc::c_char)).is_null()
    {
        strncpy(ans, input, (2048 as libc::c_int - 1 as libc::c_int) as libc::c_ulong);
        return ans;
    }
    snprintf(
        buf.as_mut_ptr(),
        255 as libc::c_int as libc::c_ulong,
        b"echo %s\0" as *const u8 as *const libc::c_char,
        input,
    );
    command[3 as libc::c_int as usize] = buf.as_mut_ptr();
    last = safePopenOut(
        command.as_mut_ptr(),
        ans,
        (2048 as libc::c_int - 1 as libc::c_int) as size_t,
    );
    if last < 0 as libc::c_int as libc::c_long {
        perror(b"Pipe read error\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if last != 0 {
        *ans
            .offset(
                (last - 1 as libc::c_int as libc::c_long) as isize,
            ) = '\0' as i32 as libc::c_char;
    } else {
        strncpy(ans, input, (2048 as libc::c_int - 1 as libc::c_int) as libc::c_ulong);
    }
    return ans;
}

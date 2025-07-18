use ::libc;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
pub unsafe extern "C" fn rpl_pipe2(
    mut fd: *mut libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: [libc::c_int; 2] = [0; 2];
    tmp[0 as libc::c_int as usize] = *fd.offset(0 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize] = *fd.offset(1 as libc::c_int as isize);
    static mut have_pipe2_really: libc::c_int = 0;
    if have_pipe2_really >= 0 as libc::c_int {
        let mut result: libc::c_int = pipe2(fd, flags);
        if !(result < 0 as libc::c_int && *__errno_location() == 38 as libc::c_int) {
            have_pipe2_really = 1 as libc::c_int;
            return result;
        }
        have_pipe2_really = -(1 as libc::c_int);
    }
    if flags
        & !(0o2000000 as libc::c_int | 0o4000 as libc::c_int | 0 as libc::c_int
            | 0 as libc::c_int) != 0 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if pipe(fd) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if flags & 0o4000 as libc::c_int != 0 {
        let mut fcntl_flags: libc::c_int = 0;
        fcntl_flags = rpl_fcntl(
            *fd.offset(1 as libc::c_int as isize),
            3 as libc::c_int,
            0 as libc::c_int,
        );
        if fcntl_flags < 0 as libc::c_int
            || rpl_fcntl(
                *fd.offset(1 as libc::c_int as isize),
                4 as libc::c_int,
                fcntl_flags | 0o4000 as libc::c_int,
            ) == -(1 as libc::c_int)
            || {
                fcntl_flags = rpl_fcntl(
                    *fd.offset(0 as libc::c_int as isize),
                    3 as libc::c_int,
                    0 as libc::c_int,
                );
                fcntl_flags < 0 as libc::c_int
            }
            || rpl_fcntl(
                *fd.offset(0 as libc::c_int as isize),
                4 as libc::c_int,
                fcntl_flags | 0o4000 as libc::c_int,
            ) == -(1 as libc::c_int)
        {
            current_block = 14289115176984913876;
        } else {
            current_block = 8236137900636309791;
        }
    } else {
        current_block = 8236137900636309791;
    }
    match current_block {
        8236137900636309791 => {
            if flags & 0o2000000 as libc::c_int != 0 {
                let mut fcntl_flags_0: libc::c_int = 0;
                fcntl_flags_0 = rpl_fcntl(
                    *fd.offset(1 as libc::c_int as isize),
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
                if fcntl_flags_0 < 0 as libc::c_int
                    || rpl_fcntl(
                        *fd.offset(1 as libc::c_int as isize),
                        2 as libc::c_int,
                        fcntl_flags_0 | 1 as libc::c_int,
                    ) == -(1 as libc::c_int)
                    || {
                        fcntl_flags_0 = rpl_fcntl(
                            *fd.offset(0 as libc::c_int as isize),
                            1 as libc::c_int,
                            0 as libc::c_int,
                        );
                        fcntl_flags_0 < 0 as libc::c_int
                    }
                    || rpl_fcntl(
                        *fd.offset(0 as libc::c_int as isize),
                        2 as libc::c_int,
                        fcntl_flags_0 | 1 as libc::c_int,
                    ) == -(1 as libc::c_int)
                {
                    current_block = 14289115176984913876;
                } else {
                    current_block = 17833034027772472439;
                }
            } else {
                current_block = 17833034027772472439;
            }
            match current_block {
                14289115176984913876 => {}
                _ => return 0 as libc::c_int,
            }
        }
        _ => {}
    }
    let mut saved_errno: libc::c_int = *__errno_location();
    close(*fd.offset(0 as libc::c_int as isize));
    close(*fd.offset(1 as libc::c_int as isize));
    *fd.offset(0 as libc::c_int as isize) = tmp[0 as libc::c_int as usize];
    *fd.offset(1 as libc::c_int as isize) = tmp[1 as libc::c_int as usize];
    *__errno_location() = saved_errno;
    return -(1 as libc::c_int);
}

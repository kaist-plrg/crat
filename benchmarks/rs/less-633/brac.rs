use ::libc;
extern "C" {
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn ch_tell() -> POSITION;
    fn jump_line_loc(pos: POSITION, sline: libc::c_int);
    fn ch_back_get() -> libc::c_int;
    fn ch_forw_get() -> libc::c_int;
    fn ch_seek(pos: POSITION) -> libc::c_int;
    fn position(sindex: libc::c_int) -> POSITION;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
pub unsafe extern "C" fn match_brac(
    mut obrac: libc::c_char,
    mut cbrac: libc::c_char,
    mut forwdir: libc::c_int,
    mut n: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut nest: libc::c_int = 0;
    let mut pos: POSITION = 0;
    let mut chget: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    pos = position(if forwdir != 0 { 0 as libc::c_int } else { -(1 as libc::c_int) });
    if pos == -(1 as libc::c_int) as POSITION || ch_seek(pos) != 0 {
        if forwdir != 0 {
            error(
                b"Nothing in top line\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        } else {
            error(
                b"Nothing in bottom line\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        }
        return;
    }
    loop {
        c = ch_forw_get();
        if c == '\n' as i32 || c == -(1 as libc::c_int) {
            if forwdir != 0 {
                error(
                    b"No bracket in top line\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            } else {
                error(
                    b"No bracket in bottom line\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            }
            return;
        }
        if !(c != obrac as libc::c_int
            || {
                n -= 1;
                n > 0 as libc::c_int
            })
        {
            break;
        }
    }
    if forwdir == 0 {
        ch_back_get();
    }
    chget = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> libc::c_int>,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        if forwdir != 0 {
            Some(ch_forw_get as unsafe extern "C" fn() -> libc::c_int)
        } else {
            Some(ch_back_get as unsafe extern "C" fn() -> libc::c_int)
        },
    );
    nest = 0 as libc::c_int;
    loop {
        c = ::std::mem::transmute::<
            _,
            fn() -> libc::c_int,
        >((Some(chget.unwrap())).unwrap())();
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if c == obrac as libc::c_int {
            if nest == 2147483647 as libc::c_int {
                break;
            }
            nest += 1;
            nest;
        } else if c == cbrac as libc::c_int
            && {
                nest -= 1;
                nest < 0 as libc::c_int
            }
        {
            jump_line_loc(
                ch_tell(),
                if forwdir != 0 { -(1 as libc::c_int) } else { 1 as libc::c_int },
            );
            return;
        }
    }
    error(
        b"No matching bracket\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void as *mut PARG,
    );
}

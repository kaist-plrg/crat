use ::libc;
extern "C" {
    fn fspool_file(zfile: *const libc::c_char) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zsappend3(
        zdir1: *const libc::c_char,
        zdir2: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
}
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub static mut spool_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: spool.c,v 1.14 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn zsfind_file(
    mut zsimple: *const libc::c_char,
    mut zsystem: *const libc::c_char,
    mut bgrade: libc::c_int,
) -> *mut libc::c_char {
    if (*zsimple.offset(0 as libc::c_int as isize) as libc::c_int != 'T' as i32
        || *zsimple.offset(1 as libc::c_int as isize) as libc::c_int != 'M' as i32
        || *zsimple.offset(2 as libc::c_int as isize) as libc::c_int != 'P' as i32)
        && fspool_file(zsimple) == 0
    {
        ulog(
            LOG_ERROR,
            b"Unrecognized file name %s\0" as *const u8 as *const libc::c_char,
            zsimple,
        );
        return 0 as *mut libc::c_char;
    }
    match *zsimple as libc::c_int {
        67 | 84 => {
            return zsappend3(
                zsystem,
                b"C.\0" as *const u8 as *const libc::c_char,
                zsimple,
            );
        }
        68 => {
            if *zsimple.offset(2 as libc::c_int as isize) as libc::c_int == 'X' as i32 {
                return zsappend3(
                    zsystem,
                    b"D.X\0" as *const u8 as *const libc::c_char,
                    zsimple,
                )
            } else {
                return zsappend3(
                    zsystem,
                    b"D.\0" as *const u8 as *const libc::c_char,
                    zsimple,
                )
            }
        }
        88 => {
            return zsappend3(
                zsystem,
                b"X.\0" as *const u8 as *const libc::c_char,
                zsimple,
            );
        }
        _ => {}
    }
    return 0 as *mut libc::c_char;
}

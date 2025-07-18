use ::libc;
extern "C" {
    pub type ansi_state;
    fn iswupper(__wc: wint_t) -> libc::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn put_wchar(pp: *mut *mut libc::c_char, ch: LWCHAR);
    fn step_char(
        pp: *mut *mut libc::c_char,
        dir: libc::c_int,
        limit: *const libc::c_char,
    ) -> LWCHAR;
    fn ansi_done(pansi: *mut ansi_state);
    fn ansi_step(pansi: *mut ansi_state, ch: LWCHAR) -> libc::c_int;
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state;
    static mut utf_mode: libc::c_int;
}
pub type wint_t = libc::c_uint;
pub type LWCHAR = libc::c_ulong;
pub unsafe extern "C" fn cvt_length(
    mut len: libc::c_int,
    mut ops: libc::c_int,
) -> libc::c_int {
    if utf_mode != 0 {
        len *= 4 as libc::c_int;
    }
    return len + 1 as libc::c_int;
}
pub unsafe extern "C" fn cvt_alloc_chpos(mut len: libc::c_int) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut chpos: *mut libc::c_int = ecalloc(
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        len as libc::c_uint,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        *chpos.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    return chpos;
}
pub unsafe extern "C" fn cvt_text(
    mut odst: *mut libc::c_char,
    mut osrc: *mut libc::c_char,
    mut chpos: *mut libc::c_int,
    mut lenp: *mut libc::c_int,
    mut ops: libc::c_int,
) {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut edst: *mut libc::c_char = odst;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: LWCHAR = 0;
    if !lenp.is_null() {
        src_end = osrc.offset(*lenp as isize);
    } else {
        src_end = osrc.offset(strlen(osrc) as isize);
    }
    src = osrc;
    dst = odst;
    while src < src_end {
        let mut src_pos: libc::c_int = src.offset_from(osrc) as libc::c_long
            as libc::c_int;
        let mut dst_pos: libc::c_int = dst.offset_from(odst) as libc::c_long
            as libc::c_int;
        let mut pansi: *mut ansi_state = 0 as *mut ansi_state;
        ch = step_char(&mut src, 1 as libc::c_int, src_end);
        if ops & 0o2 as libc::c_int != 0 && ch == '\u{8}' as i32 as libc::c_ulong
            && dst > odst
        {
            loop {
                dst = dst.offset(-1);
                dst;
                if !(dst > odst && utf_mode != 0
                    && !(*dst as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int)
                    && !(*dst as libc::c_int & 0xc0 as libc::c_int == 0xc0 as libc::c_int
                        && !(*dst as libc::c_int & 0xfe as libc::c_int
                            == 0xfe as libc::c_int)))
                {
                    break;
                }
            }
        } else if ops & 0o10 as libc::c_int != 0
            && {
                pansi = ansi_start(ch);
                !pansi.is_null()
            }
        {
            while src < src_end {
                if ansi_step(pansi, ch) != 1 as libc::c_int {
                    break;
                }
                let fresh0 = src;
                src = src.offset(1);
                ch = *fresh0 as LWCHAR;
            }
            ansi_done(pansi);
        } else {
            if ops & 0o1 as libc::c_int != 0 && iswupper(ch as wint_t) != 0 {
                ch = towlower(ch as wint_t) as LWCHAR;
            }
            put_wchar(&mut dst, ch);
            if !chpos.is_null() {
                *chpos.offset(dst_pos as isize) = src_pos;
            }
        }
        if dst > edst {
            edst = dst;
        }
    }
    if ops & 0o4 as libc::c_int != 0 && edst > odst
        && *edst.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32
    {
        edst = edst.offset(-1);
        edst;
    }
    *edst = '\0' as i32 as libc::c_char;
    if !lenp.is_null() {
        *lenp = edst.offset_from(odst) as libc::c_long as libc::c_int;
    }
}

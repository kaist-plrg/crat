use ::libc;
extern "C" {
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    static mut _gl_mbsrtowcs_state: mbstate_t;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type __mbstate_t = mbstate_t;
pub unsafe extern "C" fn rpl_mbsrtowcs(
    mut dest: *mut wchar_t,
    mut srcp: *mut *const libc::c_char,
    mut len: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    if ps.is_null() {
        ps = &mut _gl_mbsrtowcs_state;
    }
    let mut current_block_34: u64;
    let mut src: *const libc::c_char = *srcp;
    if !dest.is_null() {
        let mut destptr: *mut wchar_t = dest;
        loop {
            if !(len > 0 as libc::c_int as libc::c_ulong) {
                current_block_34 = 7175849428784450219;
                break;
            }
            let mut src_avail: size_t = 0;
            let mut ret: size_t = 0;
            if *src.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                src_avail = 1 as libc::c_int as size_t;
            } else if *src.offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                src_avail = 2 as libc::c_int as size_t;
            } else if *src.offset(2 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                src_avail = 3 as libc::c_int as size_t;
            } else if 16 as libc::c_int <= 4 as libc::c_int
                || *src.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                src_avail = 4 as libc::c_int as size_t;
            } else {
                src_avail = (4 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        strnlen1(
                            src.offset(4 as libc::c_int as isize),
                            (16 as libc::c_int - 4 as libc::c_int) as size_t,
                        ),
                    );
            }
            ret = rpl_mbrtowc(destptr, src, src_avail, ps);
            if ret == -(2 as libc::c_int) as size_t {
                abort();
            }
            if ret == -(1 as libc::c_int) as size_t {
                current_block_34 = 3067462480848035747;
                break;
            }
            if ret == 0 as libc::c_int as libc::c_ulong {
                src = 0 as *const libc::c_char;
                current_block_34 = 7175849428784450219;
                break;
            } else {
                src = src.offset(ret as isize);
                destptr = destptr.offset(1);
                destptr;
                len = len.wrapping_sub(1);
                len;
            }
        }
        match current_block_34 {
            3067462480848035747 => {
                *srcp = src;
            }
            _ => {
                *srcp = src;
                return destptr.offset_from(dest) as libc::c_long as size_t;
            }
        }
    } else {
        let mut state: mbstate_t = *ps;
        let mut totalcount: size_t = 0 as libc::c_int as size_t;
        loop {
            let mut src_avail_0: size_t = 0;
            let mut ret_0: size_t = 0;
            if *src.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                src_avail_0 = 1 as libc::c_int as size_t;
            } else if *src.offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                src_avail_0 = 2 as libc::c_int as size_t;
            } else if *src.offset(2 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                src_avail_0 = 3 as libc::c_int as size_t;
            } else if 16 as libc::c_int <= 4 as libc::c_int
                || *src.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                src_avail_0 = 4 as libc::c_int as size_t;
            } else {
                src_avail_0 = (4 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        strnlen1(
                            src.offset(4 as libc::c_int as isize),
                            (16 as libc::c_int - 4 as libc::c_int) as size_t,
                        ),
                    );
            }
            ret_0 = rpl_mbrtowc(0 as *mut wchar_t, src, src_avail_0, &mut state);
            if ret_0 == -(2 as libc::c_int) as size_t {
                abort();
            }
            if ret_0 == -(1 as libc::c_int) as size_t {
                current_block_34 = 14183212596463937064;
                break;
            }
            if ret_0 == 0 as libc::c_int as libc::c_ulong {
                current_block_34 = 6417057564578538666;
                break;
            }
            src = src.offset(ret_0 as isize);
            totalcount = totalcount.wrapping_add(1);
            totalcount;
        }
        match current_block_34 {
            14183212596463937064 => {}
            _ => return totalcount,
        }
    }
    *__errno_location() = 84 as libc::c_int;
    return -(1 as libc::c_int) as size_t;
}

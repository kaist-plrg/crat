use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn cvt_length(len: libc::c_int, ops: libc::c_int) -> libc::c_int;
    fn cvt_text(
        odst: *mut libc::c_char,
        osrc: *mut libc::c_char,
        chpos: *mut libc::c_int,
        lenp: *mut libc::c_int,
        ops: libc::c_int,
    );
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    static mut caseless: libc::c_int;
    static mut is_caseless: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
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
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut libc::c_char,
    pub __translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
unsafe extern "C" fn compile_pattern2(
    mut pattern: *mut libc::c_char,
    mut search_type: libc::c_int,
    mut comp_pattern: *mut *mut regex_t,
    mut show_error: libc::c_int,
) -> libc::c_int {
    if search_type & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    let mut comp: *mut regex_t = ecalloc(
        1 as libc::c_int,
        ::std::mem::size_of::<regex_t>() as libc::c_ulong as libc::c_uint,
    ) as *mut regex_t;
    if regcomp(
        comp,
        pattern,
        1 as libc::c_int
            | (if is_caseless != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                0 as libc::c_int
            }),
    ) != 0
    {
        free(comp as *mut libc::c_void);
        if show_error != 0 {
            error(
                b"Invalid pattern\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        }
        return -(1 as libc::c_int);
    }
    if !(*comp_pattern).is_null() {
        regfree(*comp_pattern);
        free(*comp_pattern as *mut libc::c_void);
    }
    *comp_pattern = comp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn compile_pattern(
    mut pattern: *mut libc::c_char,
    mut search_type: libc::c_int,
    mut show_error: libc::c_int,
    mut comp_pattern: *mut *mut regex_t,
) -> libc::c_int {
    let mut cvt_pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    if caseless != 2 as libc::c_int
        || 1 as libc::c_int != 0
            && search_type & (1 as libc::c_int) << 12 as libc::c_int == 0
    {
        cvt_pattern = pattern;
    } else {
        cvt_pattern = ecalloc(
            1 as libc::c_int,
            cvt_length(strlen(pattern) as libc::c_int, 0o1 as libc::c_int)
                as libc::c_uint,
        ) as *mut libc::c_char;
        cvt_text(
            cvt_pattern,
            pattern,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0o1 as libc::c_int,
        );
    }
    result = compile_pattern2(cvt_pattern, search_type, comp_pattern, show_error);
    if cvt_pattern != pattern {
        free(cvt_pattern as *mut libc::c_void);
    }
    return result;
}
pub unsafe extern "C" fn uncompile_pattern(mut pattern: *mut *mut regex_t) {
    if !(*pattern).is_null() {
        regfree(*pattern);
        free(*pattern as *mut libc::c_void);
    }
    *pattern = 0 as *mut regex_t;
}
pub unsafe extern "C" fn is_null_pattern(mut pattern: *mut regex_t) -> libc::c_int {
    return (pattern == 0 as *mut libc::c_void as *mut regex_t) as libc::c_int;
}
unsafe extern "C" fn match_0(
    mut pattern: *mut libc::c_char,
    mut pattern_len: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_len: libc::c_int,
    mut sp: *mut *mut *mut libc::c_char,
    mut ep: *mut *mut *mut libc::c_char,
    mut nsubs: libc::c_int,
) -> libc::c_int {
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pattern_end: *mut libc::c_char = pattern.offset(pattern_len as isize);
    let mut buf_end: *mut libc::c_char = buf.offset(buf_len as isize);
    while buf < buf_end {
        pp = pattern;
        lp = buf;
        loop {
            let mut cp: libc::c_char = *pp;
            let mut cl: libc::c_char = *lp;
            if caseless == 2 as libc::c_int
                && (cp as libc::c_int >= 'A' as i32 && cp as libc::c_int <= 'Z' as i32)
            {
                cp = (cp as libc::c_int - 'A' as i32 + 'a' as i32) as libc::c_char;
            }
            if cp as libc::c_int != cl as libc::c_int {
                break;
            }
            if pp == pattern_end || lp == buf_end {
                break;
            }
            pp = pp.offset(1);
            pp;
            lp = lp.offset(1);
            lp;
        }
        if pp == pattern_end {
            let fresh0 = *sp;
            *sp = (*sp).offset(1);
            *fresh0 = buf;
            let fresh1 = *ep;
            *ep = (*ep).offset(1);
            *fresh1 = lp;
            return 1 as libc::c_int;
        }
        buf = buf.offset(1);
        buf;
    }
    **ep = 0 as *mut libc::c_char;
    **sp = **ep;
    return 0 as libc::c_int;
}
unsafe extern "C" fn match_pattern1(
    mut pattern: *mut regex_t,
    mut tpattern: *mut libc::c_char,
    mut line: *mut libc::c_char,
    mut line_len: libc::c_int,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
    mut nsp: libc::c_int,
    mut notbol: libc::c_int,
    mut search_type: libc::c_int,
) -> libc::c_int {
    let mut matched: libc::c_int = 0;
    if search_type & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        matched = match_0(
            tpattern,
            strlen(tpattern) as libc::c_int,
            line,
            line_len,
            &mut sp,
            &mut ep,
            nsp,
        );
    } else {
        let mut rm: [regmatch_t; 7] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 7];
        let mut flags: libc::c_int = if notbol != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        flags |= (1 as libc::c_int) << 2 as libc::c_int;
        rm[0 as libc::c_int as usize].rm_so = 0 as libc::c_int;
        rm[0 as libc::c_int as usize].rm_eo = line_len;
        matched = (regexec(
            pattern,
            line,
            (16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int + 2 as libc::c_int)
                as size_t,
            rm.as_mut_ptr(),
            flags,
        ) == 0) as libc::c_int;
        if matched != 0 {
            let mut i: libc::c_int = 0;
            let mut ecount: libc::c_int = 0;
            ecount = 16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int
                + 2 as libc::c_int;
            while ecount > 0 as libc::c_int {
                if rm[(ecount - 1 as libc::c_int) as usize].rm_so >= 0 as libc::c_int {
                    break;
                }
                ecount -= 1;
                ecount;
            }
            if ecount >= nsp {
                ecount = nsp - 1 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < ecount {
                if rm[i as usize].rm_so < 0 as libc::c_int {
                    let fresh2 = ep;
                    ep = ep.offset(1);
                    *fresh2 = line;
                    let fresh3 = sp;
                    sp = sp.offset(1);
                    *fresh3 = *fresh2;
                } else {
                    let fresh4 = sp;
                    sp = sp.offset(1);
                    *fresh4 = line.offset(rm[i as usize].rm_so as isize);
                    let fresh5 = ep;
                    ep = ep.offset(1);
                    *fresh5 = line.offset(rm[i as usize].rm_eo as isize);
                }
                i += 1;
                i;
            }
        }
    }
    *ep = 0 as *mut libc::c_char;
    *sp = *ep;
    matched = (search_type & (1 as libc::c_int) << 8 as libc::c_int == 0 && matched != 0
        || search_type & (1 as libc::c_int) << 8 as libc::c_int != 0 && matched == 0)
        as libc::c_int;
    return matched;
}
pub unsafe extern "C" fn match_pattern(
    mut pattern: *mut regex_t,
    mut tpattern: *mut libc::c_char,
    mut line: *mut libc::c_char,
    mut line_len: libc::c_int,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
    mut nsp: libc::c_int,
    mut notbol: libc::c_int,
    mut search_type: libc::c_int,
) -> libc::c_int {
    let mut matched: libc::c_int = match_pattern1(
        pattern,
        tpattern,
        line,
        line_len,
        sp,
        ep,
        nsp,
        notbol,
        search_type,
    );
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int {
        if search_type & (1 as libc::c_int) << 16 as libc::c_int + i != 0
            && *ep.offset(i as isize) == *sp.offset(i as isize)
        {
            matched = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return matched;
}
pub unsafe extern "C" fn pattern_lib_name() -> *mut libc::c_char {
    return b"POSIX\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}

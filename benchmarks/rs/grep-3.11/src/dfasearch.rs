use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type kwset;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn rpl_re_match(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn rpl_regfree(__preg: *mut regex_t);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut localeinfo: localeinfo;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut match_icase: bool;
    static mut match_words: bool;
    static mut match_lines: bool;
    static mut eolbyte: libc::c_char;
    fn pattern_file_name(_: idx_t, _: *mut idx_t) -> *const libc::c_char;
    fn dfaalloc() -> *mut dfa;
    fn dfasyntax(_: *mut dfa, _: *const localeinfo, _: reg_syntax_t, _: libc::c_int);
    fn dfaparse(_: *const libc::c_char, _: idx_t, _: *mut dfa);
    fn dfamustfree(_: *mut dfamust);
    fn dfamust(_: *const dfa) -> *mut dfamust;
    fn dfacomp(_: *const libc::c_char, _: idx_t, _: *mut dfa, _: bool);
    fn dfaexec(
        d: *mut dfa,
        begin: *const libc::c_char,
        end: *mut libc::c_char,
        allow_nl: bool,
        count: *mut idx_t,
        backref: *mut bool,
    ) -> *mut libc::c_char;
    fn dfasuperset(d: *const dfa) -> *mut dfa;
    fn dfaisfast(_: *const dfa) -> bool;
    fn dfasupported(_: *const dfa) -> bool;
    fn mb_goback(
        _: *mut *const libc::c_char,
        _: *mut idx_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> ptrdiff_t;
    fn wordchar_prev(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> idx_t;
    fn wordchar_next(_: *const libc::c_char, _: *const libc::c_char) -> idx_t;
    fn kwsincr(_: kwset_t, _: *const libc::c_char, _: idx_t);
    fn kwsprep(_: kwset_t);
    fn kwsexec(
        _: kwset_t,
        _: *const libc::c_char,
        _: idx_t,
        _: *mut kwsmatch,
        _: bool,
    ) -> ptrdiff_t;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xirealloc(p: *mut libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn kwsinit(_: bool) -> kwset_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type wint_t = libc::c_uint;
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type ptrdiff_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const EXIT_TROUBLE: C2RustUnnamed = 2;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfamust {
    pub exact: bool,
    pub begline: bool,
    pub endline: bool,
    pub must: [libc::c_char; 0],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DFA_PLUS_WARN: C2RustUnnamed_0 = 32;
pub const DFA_STAR_WARN: C2RustUnnamed_0 = 16;
pub const DFA_STRAY_BACKSLASH_WARN: C2RustUnnamed_0 = 8;
pub const DFA_CONFUSING_BRACKETS_ERROR: C2RustUnnamed_0 = 4;
pub const DFA_EOL_NUL: C2RustUnnamed_0 = 2;
pub const DFA_ANCHOR: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwsmatch {
    pub index: idx_t,
    pub offset: idx_t,
    pub size: idx_t,
}
pub type kwset_t = *mut kwset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfa_comp {
    pub kwset: kwset_t,
    pub dfa: *mut dfa,
    pub patterns: *mut re_pattern_buffer,
    pub pcount: idx_t,
    pub regs: re_registers,
    pub kwset_exact_matches: idx_t,
    pub begline: bool,
}
pub unsafe extern "C" fn dfaerror(mut mesg: *const libc::c_char) {
    if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
        error(
            EXIT_TROUBLE as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            mesg,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            EXIT_TROUBLE as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            mesg,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
pub unsafe extern "C" fn dfawarn(mut mesg: *const libc::c_char) {
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"warning: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        mesg,
    );
}
unsafe extern "C" fn kwsmusts(mut dc: *mut dfa_comp) {
    let mut dm: *mut dfamust = dfamust((*dc).dfa);
    if dm.is_null() {
        return;
    }
    (*dc).kwset = kwsinit(0 as libc::c_int != 0);
    if (*dm).exact {
        (*dc).kwset_exact_matches += 1;
        (*dc).kwset_exact_matches;
        let mut old_len: idx_t = strlen(((*dm).must).as_mut_ptr()) as idx_t;
        let mut new_len: idx_t = old_len + (*dm).begline as libc::c_long
            + (*dm).endline as libc::c_long;
        let mut must: *mut libc::c_char = ximalloc(new_len) as *mut libc::c_char;
        let mut mp: *mut libc::c_char = must;
        *mp = eolbyte;
        mp = mp.offset((*dm).begline as libc::c_int as isize);
        (*dc)
            .begline = ((*dc).begline as libc::c_int | (*dm).begline as libc::c_int) != 0;
        memcpy(
            mp as *mut libc::c_void,
            ((*dm).must).as_mut_ptr() as *const libc::c_void,
            old_len as libc::c_ulong,
        );
        if (*dm).endline {
            *mp.offset(old_len as isize) = eolbyte;
        }
        kwsincr((*dc).kwset, must, new_len);
        rpl_free(must as *mut libc::c_void);
    } else {
        kwsincr(
            (*dc).kwset,
            ((*dm).must).as_mut_ptr(),
            strlen(((*dm).must).as_mut_ptr()) as idx_t,
        );
    }
    kwsprep((*dc).kwset);
    dfamustfree(dm);
}
unsafe extern "C" fn possible_backrefs_in_pattern(
    mut keys: *const libc::c_char,
    mut len: idx_t,
    mut bs_safe: bool,
) -> bool {
    let mut second_backslash: libc::c_int = if bs_safe as libc::c_int != 0 {
        '\\' as i32
    } else {
        127 as libc::c_int + 1 as libc::c_int
    };
    len -= 1;
    len;
    if 0 as libc::c_int as libc::c_long <= len {
        let mut lim: *const libc::c_char = keys.offset(len as isize);
        let mut p: *const libc::c_char = keys;
        loop {
            p = memchr(
                p as *const libc::c_void,
                '\\' as i32,
                lim.offset_from(p) as libc::c_long as libc::c_ulong,
            ) as *const libc::c_char;
            if p.is_null() {
                break;
            }
            if '1' as i32 <= *p.offset(1 as libc::c_int as isize) as libc::c_int
                && *p.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
            {
                return 1 as libc::c_int != 0;
            }
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == second_backslash {
                p = p.offset(1);
                p;
                if p == lim {
                    break;
                }
            }
            p = p.offset(1);
            p;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn regex_compile(
    mut dc: *mut dfa_comp,
    mut p: *const libc::c_char,
    mut len: idx_t,
    mut pcount: idx_t,
    mut lineno: idx_t,
    mut syntax_bits: reg_syntax_t,
    mut syntax_only: bool,
) -> bool {
    let mut pat: re_pattern_buffer = re_pattern_buffer {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    pat.buffer = 0 as *mut re_dfa_t;
    pat.allocated = 0 as libc::c_int as __re_long_size_t;
    let mut uchar_max: idx_t = (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
        as idx_t;
    pat
        .fastmap = (if syntax_only as libc::c_int | match_icase as libc::c_int != 0 {
        0 as *mut libc::c_void
    } else {
        ximalloc(uchar_max + 1 as libc::c_int as libc::c_long)
    }) as *mut libc::c_char;
    pat.translate = 0 as *mut libc::c_uchar;
    if syntax_only {
        rpl_re_set_syntax(
            syntax_bits
                | (((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int,
        );
    } else {
        rpl_re_set_syntax(syntax_bits);
    }
    let mut err: *const libc::c_char = rpl_re_compile_pattern(
        p,
        len as size_t,
        &mut pat,
    );
    if err.is_null() {
        if syntax_only {
            rpl_regfree(&mut pat);
        } else {
            *((*dc).patterns).offset(pcount as isize) = pat;
        }
        return 1 as libc::c_int != 0;
    }
    rpl_free(pat.fastmap as *mut libc::c_void);
    let mut pat_lineno: idx_t = 0;
    let mut pat_filename: *const libc::c_char = if lineno
        < 0 as libc::c_int as libc::c_long
    {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        pattern_file_name(lineno, &mut pat_lineno)
    };
    if *pat_filename as libc::c_int == '\0' as i32 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            err,
        );
    } else {
        let mut n: ptrdiff_t = pat_lineno;
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s:%td: %s\0" as *const u8 as *const libc::c_char,
            pat_filename,
            n,
            err,
        );
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn GEAcompile(
    mut pattern: *mut libc::c_char,
    mut size: idx_t,
    mut syntax_bits: reg_syntax_t,
    mut exact: bool,
) -> *mut libc::c_void {
    let mut motif: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dc: *mut dfa_comp = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<dfa_comp>() as libc::c_ulong,
    ) as *mut dfa_comp;
    (*dc).dfa = dfaalloc();
    if match_icase {
        syntax_bits
            |= ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int;
    }
    let mut dfaopts: libc::c_int = DFA_CONFUSING_BRACKETS_ERROR as libc::c_int
        | DFA_STRAY_BACKSLASH_WARN as libc::c_int | DFA_PLUS_WARN as libc::c_int
        | (if syntax_bits
            & ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
        {
            DFA_STAR_WARN as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if eolbyte as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            DFA_EOL_NUL as libc::c_int
        });
    dfasyntax((*dc).dfa, &mut localeinfo, syntax_bits, dfaopts);
    let mut bs_safe: bool = !localeinfo.multibyte as libc::c_int
        | localeinfo.using_utf8 as libc::c_int != 0;
    let mut p: *const libc::c_char = pattern;
    let mut patlim: *const libc::c_char = pattern.offset(size as isize);
    let mut compilation_failed: bool = 0 as libc::c_int != 0;
    (*dc)
        .patterns = xmalloc(::std::mem::size_of::<re_pattern_buffer>() as libc::c_ulong)
        as *mut re_pattern_buffer;
    (*dc).patterns = ((*dc).patterns).offset(1);
    (*dc).patterns;
    (*dc).pcount = 0 as libc::c_int as idx_t;
    let mut palloc: idx_t = 1 as libc::c_int as idx_t;
    let mut prev: *const libc::c_char = pattern;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: idx_t = 0 as libc::c_int as idx_t;
    let mut bufalloc: idx_t = 0 as libc::c_int as idx_t;
    let mut lineno: idx_t = 0 as libc::c_int as idx_t;
    loop {
        let mut sep: *const libc::c_char = rawmemchr(
            p as *const libc::c_void,
            '\n' as i32,
        ) as *const libc::c_char;
        let mut len: idx_t = sep.offset_from(p) as libc::c_long;
        let mut backref: bool = possible_backrefs_in_pattern(p, len, bs_safe);
        if backref as libc::c_int != 0 && prev < p {
            let mut prevlen: idx_t = p.offset_from(prev) as libc::c_long;
            let mut bufshortage: ptrdiff_t = buflen - bufalloc + prevlen;
            if (0 as libc::c_int as libc::c_long) < bufshortage {
                buf = xpalloc(
                    buf as *mut libc::c_void,
                    &mut bufalloc,
                    bufshortage,
                    -(1 as libc::c_int) as ptrdiff_t,
                    1 as libc::c_int as idx_t,
                ) as *mut libc::c_char;
            }
            memcpy(
                buf.offset(buflen as isize) as *mut libc::c_void,
                prev as *const libc::c_void,
                prevlen as libc::c_ulong,
            );
            buflen += prevlen;
        }
        let mut shortage: ptrdiff_t = (*dc).pcount - palloc
            + 2 as libc::c_int as libc::c_long;
        if (0 as libc::c_int as libc::c_long) < shortage {
            (*dc)
                .patterns = xpalloc(
                ((*dc).patterns).offset(-(1 as libc::c_int as isize))
                    as *mut libc::c_void,
                &mut palloc,
                shortage,
                -(1 as libc::c_int) as ptrdiff_t,
                ::std::mem::size_of::<re_pattern_buffer>() as libc::c_ulong as idx_t,
            ) as *mut re_pattern_buffer;
            (*dc).patterns = ((*dc).patterns).offset(1);
            (*dc).patterns;
        }
        if !regex_compile(dc, p, len, (*dc).pcount, lineno, syntax_bits, !backref) {
            compilation_failed = 1 as libc::c_int != 0;
        }
        p = sep.offset(1 as libc::c_int as isize);
        lineno += 1;
        lineno;
        if backref {
            (*dc).pcount += 1;
            (*dc).pcount;
            prev = p;
        }
        if !(p <= patlim) {
            break;
        }
    }
    if compilation_failed {
        exit(EXIT_TROUBLE as libc::c_int);
    }
    if patlim < prev {
        buflen -= 1;
        buflen;
    } else if pattern < prev as *mut libc::c_char {
        let mut prevlen_0: idx_t = patlim.offset_from(prev) as libc::c_long;
        buf = xirealloc(buf as *mut libc::c_void, buflen + prevlen_0)
            as *mut libc::c_char;
        memcpy(
            buf.offset(buflen as isize) as *mut libc::c_void,
            prev as *const libc::c_void,
            prevlen_0 as libc::c_ulong,
        );
        buflen += prevlen_0;
    } else {
        buf = pattern;
        buflen = size;
    }
    if match_words as libc::c_int != 0 || match_lines as libc::c_int != 0 {
        static mut line_beg_no_bk: [libc::c_char; 3] = unsafe {
            *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"^(\0")
        };
        static mut line_end_no_bk: [libc::c_char; 3] = unsafe {
            *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b")$\0")
        };
        static mut word_beg_no_bk: [libc::c_char; 19] = unsafe {
            *::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"(^|[^[:alnum:]_])(\0")
        };
        static mut word_end_no_bk: [libc::c_char; 19] = unsafe {
            *::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b")([^[:alnum:]_]|$)\0")
        };
        static mut line_beg_bk: [libc::c_char; 4] = unsafe {
            *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"^\\(\0")
        };
        static mut line_end_bk: [libc::c_char; 4] = unsafe {
            *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"\\)$\0")
        };
        static mut word_beg_bk: [libc::c_char; 23] = unsafe {
            *::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"\\(^\\|[^[:alnum:]_]\\)\\(\0")
        };
        static mut word_end_bk: [libc::c_char; 23] = unsafe {
            *::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"\\)\\([^[:alnum:]_]\\|$\\)\0")
        };
        let mut bk: libc::c_int = (syntax_bits
            & (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int == 0)
            as libc::c_int;
        let mut bracket_bytes: idx_t = (::std::mem::size_of::<[libc::c_char; 23]>()
            as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            as idx_t;
        let mut n: *mut libc::c_char = ximalloc(size + bracket_bytes)
            as *mut libc::c_char;
        strcpy(
            n,
            if match_lines as libc::c_int != 0 {
                if bk != 0 { line_beg_bk.as_ptr() } else { line_beg_no_bk.as_ptr() }
            } else if bk != 0 {
                word_beg_bk.as_ptr()
            } else {
                word_beg_no_bk.as_ptr()
            },
        );
        let mut total: idx_t = strlen(n) as idx_t;
        memcpy(
            n.offset(total as isize) as *mut libc::c_void,
            pattern as *const libc::c_void,
            size as libc::c_ulong,
        );
        total += size;
        strcpy(
            n.offset(total as isize),
            if match_lines as libc::c_int != 0 {
                if bk != 0 { line_end_bk.as_ptr() } else { line_end_no_bk.as_ptr() }
            } else if bk != 0 {
                word_end_bk.as_ptr()
            } else {
                word_end_no_bk.as_ptr()
            },
        );
        total = (total as libc::c_ulong).wrapping_add(strlen(n.offset(total as isize)))
            as idx_t as idx_t;
        motif = n;
        pattern = motif;
        size = total;
    } else {
        motif = 0 as *mut libc::c_char;
    }
    dfaparse(pattern, size, (*dc).dfa);
    kwsmusts(dc);
    dfacomp(
        0 as *const libc::c_char,
        0 as libc::c_int as idx_t,
        (*dc).dfa,
        1 as libc::c_int != 0,
    );
    if !buf.is_null() {
        if exact as libc::c_int != 0 || !dfasupported((*dc).dfa) {
            (*dc).patterns = ((*dc).patterns).offset(-1);
            (*dc).patterns;
            (*dc).pcount += 1;
            (*dc).pcount;
            if !regex_compile(
                dc,
                buf,
                buflen,
                0 as libc::c_int as idx_t,
                -(1 as libc::c_int) as idx_t,
                syntax_bits,
                0 as libc::c_int != 0,
            ) {
                abort();
            }
        }
        if buf != pattern {
            rpl_free(buf as *mut libc::c_void);
        }
    }
    rpl_free(motif as *mut libc::c_void);
    return dc as *mut libc::c_void;
}
pub unsafe extern "C" fn EGexecute(
    mut vdc: *mut libc::c_void,
    mut buf: *const libc::c_char,
    mut size: idx_t,
    mut match_size: *mut idx_t,
    mut start_ptr: *const libc::c_char,
) -> ptrdiff_t {
    let mut current_block: u64;
    let mut buflim: *const libc::c_char = 0 as *const libc::c_char;
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut match_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut best_match: *const libc::c_char = 0 as *const libc::c_char;
    let mut mb_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: libc::c_char = eolbyte;
    let mut start: regoff_t = 0;
    let mut len: idx_t = 0;
    let mut best_len: idx_t = 0;
    let mut kwsm: kwsmatch = kwsmatch {
        index: 0,
        offset: 0,
        size: 0,
    };
    let mut i: idx_t = 0;
    let mut dc: *mut dfa_comp = vdc as *mut dfa_comp;
    let mut superset: *mut dfa = dfasuperset((*dc).dfa);
    let mut dfafast: bool = dfaisfast((*dc).dfa);
    mb_start = buf;
    buflim = buf.offset(size as isize);
    end = buf;
    beg = end;
    's_26: loop {
        if !(end < buflim) {
            current_block = 12070711452894729854;
            break;
        }
        end = buflim;
        if start_ptr.is_null() {
            let mut next_beg: *const libc::c_char = 0 as *const libc::c_char;
            let mut dfa_beg: *const libc::c_char = beg;
            let mut count: idx_t = 0 as libc::c_int as idx_t;
            let mut exact_kwset_match: bool = 0 as libc::c_int != 0;
            let mut backref: bool = 0 as libc::c_int != 0;
            if !((*dc).kwset).is_null() {
                let mut prev_beg: *const libc::c_char = 0 as *const libc::c_char;
                let mut offset: ptrdiff_t = kwsexec(
                    (*dc).kwset,
                    beg.offset(-((*dc).begline as libc::c_int as isize)),
                    buflim.offset_from(beg) as libc::c_long
                        + (*dc).begline as libc::c_long,
                    &mut kwsm,
                    1 as libc::c_int != 0,
                );
                if offset < 0 as libc::c_int as libc::c_long {
                    return offset;
                }
                match_0 = beg.offset(offset as isize);
                prev_beg = beg;
                beg = memrchr(
                    buf as *const libc::c_void,
                    eol as libc::c_int,
                    match_0.offset_from(buf) as libc::c_long as size_t,
                ) as *const libc::c_char;
                beg = if !beg.is_null() {
                    beg.offset(1 as libc::c_int as isize)
                } else {
                    buf
                };
                dfa_beg = beg;
                exact_kwset_match = kwsm.index < (*dc).kwset_exact_matches;
                if exact_kwset_match as libc::c_int != 0 || !dfafast
                    || (if 16 as libc::c_int as libc::c_long
                        > match_0.offset_from(beg) as libc::c_long
                    {
                        16 as libc::c_int as libc::c_long
                    } else {
                        match_0.offset_from(beg) as libc::c_long
                    })
                        < match_0.offset_from(prev_beg) as libc::c_long
                            >> 2 as libc::c_int
                {
                    end = rawmemchr(match_0 as *const libc::c_void, eol as libc::c_int)
                        as *const libc::c_char;
                    end = end.offset(1);
                    end;
                } else if (if 16 as libc::c_int as libc::c_long
                    > match_0.offset_from(beg) as libc::c_long
                {
                    16 as libc::c_int as libc::c_long
                } else {
                    match_0.offset_from(beg) as libc::c_long
                }) < buflim.offset_from(prev_beg) as libc::c_long >> 2 as libc::c_int
                {
                    end = rawmemchr(
                        prev_beg
                            .offset(
                                (4 as libc::c_int as libc::c_long
                                    * (if 16 as libc::c_int as libc::c_long
                                        > match_0.offset_from(beg) as libc::c_long
                                    {
                                        16 as libc::c_int as libc::c_long
                                    } else {
                                        match_0.offset_from(beg) as libc::c_long
                                    })) as isize,
                            ) as *const libc::c_void,
                        eol as libc::c_int,
                    ) as *const libc::c_char;
                    end = end.offset(1);
                    end;
                } else {
                    end = buflim;
                }
                if exact_kwset_match {
                    if !localeinfo.multibyte as libc::c_int
                        | localeinfo.using_utf8 as libc::c_int != 0
                    {
                        current_block = 15471791276879313738;
                        break;
                    }
                    if mb_start < beg {
                        mb_start = beg;
                    }
                    if mb_goback(&mut mb_start, 0 as *mut idx_t, match_0, buflim)
                        == 0 as libc::c_int as libc::c_long
                    {
                        current_block = 15471791276879313738;
                        break;
                    }
                    dfa_beg = mb_start;
                }
            }
            if !superset.is_null() && !exact_kwset_match {
                next_beg = dfaexec(
                    superset,
                    dfa_beg,
                    end as *mut libc::c_char,
                    0 as libc::c_int != 0,
                    &mut count,
                    0 as *mut bool,
                );
                if next_beg.is_null() || next_beg == end {
                    current_block = 15240798224410183470;
                } else {
                    if count != 0 as libc::c_int as libc::c_long {
                        beg = memrchr(
                            buf as *const libc::c_void,
                            eol as libc::c_int,
                            next_beg.offset_from(buf) as libc::c_long as size_t,
                        ) as *const libc::c_char;
                        beg = beg.offset(1);
                        beg;
                        dfa_beg = beg;
                    }
                    end = rawmemchr(next_beg as *const libc::c_void, eol as libc::c_int)
                        as *const libc::c_char;
                    end = end.offset(1);
                    end;
                    count = 0 as libc::c_int as idx_t;
                    current_block = 15597372965620363352;
                }
            } else {
                current_block = 15597372965620363352;
            }
            match current_block {
                15240798224410183470 => {}
                _ => {
                    next_beg = dfaexec(
                        (*dc).dfa,
                        dfa_beg,
                        end as *mut libc::c_char,
                        0 as libc::c_int != 0,
                        &mut count,
                        &mut backref,
                    );
                    if next_beg.is_null() || next_beg == end {
                        current_block = 15240798224410183470;
                    } else {
                        if count != 0 as libc::c_int as libc::c_long {
                            beg = memrchr(
                                buf as *const libc::c_void,
                                eol as libc::c_int,
                                next_beg.offset_from(buf) as libc::c_long as size_t,
                            ) as *const libc::c_char;
                            beg = beg.offset(1);
                            beg;
                        }
                        end = rawmemchr(
                            next_beg as *const libc::c_void,
                            eol as libc::c_int,
                        ) as *const libc::c_char;
                        end = end.offset(1);
                        end;
                        if !backref {
                            current_block = 15471791276879313738;
                            break;
                        }
                        ptr = beg;
                        current_block = 10399321362245223758;
                    }
                }
            }
        } else {
            ptr = start_ptr;
            current_block = 10399321362245223758;
        }
        match current_block {
            10399321362245223758 => {
                if (if (0 as libc::c_int as regoff_t) < -(1 as libc::c_int) as regoff_t {
                    -(1 as libc::c_int) as regoff_t
                } else {
                    (((1 as libc::c_int as regoff_t)
                        << (::std::mem::size_of::<regoff_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                })
                    < end.offset_from(beg) as libc::c_long
                        - 1 as libc::c_int as libc::c_long
                {
                    xalloc_die();
                }
                best_match = end;
                best_len = 0 as libc::c_int as idx_t;
                i = 0 as libc::c_int as idx_t;
                while i < (*dc).pcount {
                    let ref mut fresh0 = *((*dc).patterns).offset(i as isize);
                    (*fresh0).set_not_eol(0 as libc::c_int as libc::c_uint);
                    let ref mut fresh1 = *((*dc).patterns).offset(i as isize);
                    (*fresh1)
                        .set_newline_anchor(
                            (eolbyte as libc::c_int == '\n' as i32) as libc::c_int
                                as libc::c_uint,
                        );
                    start = rpl_re_search(
                        &mut *((*dc).patterns).offset(i as isize),
                        beg,
                        end.offset_from(beg) as libc::c_long
                            - 1 as libc::c_int as libc::c_long,
                        ptr.offset_from(beg) as libc::c_long,
                        end.offset_from(ptr) as libc::c_long
                            - 1 as libc::c_int as libc::c_long,
                        &mut (*dc).regs,
                    );
                    if start < -(1 as libc::c_int) as libc::c_long {
                        xalloc_die();
                    } else if 0 as libc::c_int as libc::c_long <= start {
                        len = *((*dc).regs.end).offset(0 as libc::c_int as isize)
                            - start;
                        match_0 = beg.offset(start as isize);
                        if !(match_0 > best_match) {
                            if !start_ptr.is_null() && !match_words {
                                current_block = 16500971801063615340;
                            } else if !match_lines && !match_words
                                || match_lines as libc::c_int != 0
                                    && len
                                        == end.offset_from(ptr) as libc::c_long
                                            - 1 as libc::c_int as libc::c_long
                            {
                                match_0 = ptr;
                                len = end.offset_from(ptr) as libc::c_long;
                                current_block = 16500971801063615340;
                            } else if !match_lines && match_words as libc::c_int != 0 {
                                loop {
                                    if !(match_0 <= best_match) {
                                        current_block = 11793792312832361944;
                                        break;
                                    }
                                    let mut shorter_len: regoff_t = 0 as libc::c_int
                                        as regoff_t;
                                    if wordchar_next(
                                        match_0.offset(len as isize),
                                        end.offset(-(1 as libc::c_int as isize)),
                                    ) == 0
                                        && wordchar_prev(
                                            beg,
                                            match_0,
                                            end.offset(-(1 as libc::c_int as isize)),
                                        ) == 0
                                    {
                                        current_block = 16500971801063615340;
                                        break;
                                    }
                                    if len > 0 as libc::c_int as libc::c_long {
                                        len -= 1;
                                        len;
                                        let ref mut fresh2 = *((*dc).patterns).offset(i as isize);
                                        (*fresh2).set_not_eol(1 as libc::c_int as libc::c_uint);
                                        shorter_len = rpl_re_match(
                                            &mut *((*dc).patterns).offset(i as isize),
                                            beg,
                                            match_0.offset(len as isize).offset_from(ptr)
                                                as libc::c_long,
                                            match_0.offset_from(beg) as libc::c_long,
                                            &mut (*dc).regs,
                                        );
                                        if shorter_len < -(1 as libc::c_int) as libc::c_long {
                                            xalloc_die();
                                        }
                                    }
                                    if (0 as libc::c_int as libc::c_long) < shorter_len {
                                        len = shorter_len;
                                    } else {
                                        if match_0 == end.offset(-(1 as libc::c_int as isize)) {
                                            current_block = 11793792312832361944;
                                            break;
                                        }
                                        match_0 = match_0.offset(1);
                                        match_0;
                                        let ref mut fresh3 = *((*dc).patterns).offset(i as isize);
                                        (*fresh3).set_not_eol(0 as libc::c_int as libc::c_uint);
                                        start = rpl_re_search(
                                            &mut *((*dc).patterns).offset(i as isize),
                                            beg,
                                            end.offset_from(beg) as libc::c_long
                                                - 1 as libc::c_int as libc::c_long,
                                            match_0.offset_from(beg) as libc::c_long,
                                            end.offset_from(match_0) as libc::c_long
                                                - 1 as libc::c_int as libc::c_long,
                                            &mut (*dc).regs,
                                        );
                                        if start < 0 as libc::c_int as libc::c_long {
                                            if start < -(1 as libc::c_int) as libc::c_long {
                                                xalloc_die();
                                            }
                                            current_block = 11793792312832361944;
                                            break;
                                        } else {
                                            len = *((*dc).regs.end).offset(0 as libc::c_int as isize)
                                                - start;
                                            match_0 = beg.offset(start as isize);
                                        }
                                    }
                                }
                            } else {
                                current_block = 11793792312832361944;
                            }
                            match current_block {
                                11793792312832361944 => {}
                                _ => {
                                    if start_ptr.is_null() {
                                        current_block = 15471791276879313738;
                                        break 's_26;
                                    }
                                    if match_0 < best_match
                                        || match_0 == best_match && len > best_len
                                    {
                                        best_match = match_0;
                                        best_len = len;
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                    i;
                }
                if best_match < end {
                    beg = best_match;
                    len = best_len;
                    current_block = 2616667235040759262;
                    break;
                }
            }
            _ => {}
        }
        beg = end;
    }
    match current_block {
        12070711452894729854 => return -(1 as libc::c_int) as ptrdiff_t,
        15471791276879313738 => {
            len = end.offset_from(beg) as libc::c_long;
        }
        _ => {}
    }
    *match_size = len;
    return beg.offset_from(buf) as libc::c_long;
}

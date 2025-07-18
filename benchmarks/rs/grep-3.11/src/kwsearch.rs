use ::libc;
extern "C" {
    pub type kwset;
    fn rpl_free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut localeinfo: localeinfo;
    fn fgrep_to_grep_pattern(_: *mut *mut libc::c_char, _: *mut idx_t);
    fn EGexecute(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: idx_t,
        _: *mut idx_t,
        _: *const libc::c_char,
    ) -> ptrdiff_t;
    fn GEAcompile(
        _: *mut libc::c_char,
        _: idx_t,
        _: reg_syntax_t,
        _: bool,
    ) -> *mut libc::c_void;
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
    fn wordchars_size(_: *const libc::c_char, _: *const libc::c_char) -> idx_t;
    fn kwsinit(_: bool) -> kwset_t;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    static mut match_words: bool;
    static mut match_lines: bool;
    static mut eolbyte: libc::c_char;
    fn kwsincr(_: kwset_t, _: *const libc::c_char, _: idx_t);
    fn kwswords(_: kwset_t) -> idx_t;
    fn kwsprep(_: kwset_t);
    fn kwsexec(
        _: kwset_t,
        _: *const libc::c_char,
        _: idx_t,
        _: *mut kwsmatch,
        _: bool,
    ) -> ptrdiff_t;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type wint_t = libc::c_uint;
pub type reg_syntax_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub struct kwsmatch {
    pub index: idx_t,
    pub offset: idx_t,
    pub size: idx_t,
}
pub type kwset_t = *mut kwset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwsearch {
    pub kwset: kwset_t,
    pub words: idx_t,
    pub pattern: *mut libc::c_char,
    pub size: idx_t,
    pub re: *mut libc::c_void,
}
pub unsafe extern "C" fn Fcompile(
    mut pattern: *mut libc::c_char,
    mut size: idx_t,
    mut ignored: reg_syntax_t,
    mut exact: bool,
) -> *mut libc::c_void {
    let mut kwset: kwset_t = 0 as *mut kwset;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufalloc: idx_t = 0 as libc::c_int as idx_t;
    kwset = kwsinit(1 as libc::c_int != 0);
    let mut p: *const libc::c_char = pattern;
    loop {
        let mut sep: *const libc::c_char = rawmemchr(
            p as *const libc::c_void,
            '\n' as i32,
        ) as *const libc::c_char;
        let mut len: idx_t = sep.offset_from(p) as libc::c_long;
        if match_lines {
            if eolbyte as libc::c_int == '\n' as i32 && pattern < p as *mut libc::c_char
            {
                p = p.offset(-1);
                p;
            } else {
                if bufalloc < len + 2 as libc::c_int as libc::c_long {
                    rpl_free(buf as *mut libc::c_void);
                    bufalloc = len;
                    buf = xpalloc(
                        0 as *mut libc::c_void,
                        &mut bufalloc,
                        2 as libc::c_int as idx_t,
                        -(1 as libc::c_int) as ptrdiff_t,
                        1 as libc::c_int as idx_t,
                    ) as *mut libc::c_char;
                    *buf.offset(0 as libc::c_int as isize) = eolbyte;
                }
                memcpy(
                    buf.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    p as *const libc::c_void,
                    len as libc::c_ulong,
                );
                *buf.offset((len + 1 as libc::c_int as libc::c_long) as isize) = eolbyte;
                p = buf;
            }
            len += 2 as libc::c_int as libc::c_long;
        }
        kwsincr(kwset, p, len);
        p = sep.offset(1 as libc::c_int as isize);
        if !(p <= pattern.offset(size as isize) as *const libc::c_char) {
            break;
        }
    }
    rpl_free(buf as *mut libc::c_void);
    let mut words: idx_t = kwswords(kwset);
    kwsprep(kwset);
    let mut kwsearch: *mut kwsearch = xmalloc(
        ::std::mem::size_of::<kwsearch>() as libc::c_ulong,
    ) as *mut kwsearch;
    (*kwsearch).kwset = kwset;
    (*kwsearch).words = words;
    (*kwsearch).pattern = pattern;
    (*kwsearch).size = size;
    (*kwsearch).re = 0 as *mut libc::c_void;
    return kwsearch as *mut libc::c_void;
}
pub unsafe extern "C" fn Fexecute(
    mut vcp: *mut libc::c_void,
    mut buf: *const libc::c_char,
    mut size: idx_t,
    mut match_size: *mut idx_t,
    mut start_ptr: *const libc::c_char,
) -> ptrdiff_t {
    let mut current_block: u64;
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut mb_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: idx_t = 0;
    let mut eol: libc::c_char = eolbyte;
    let mut kwsearch: *mut kwsearch = vcp as *mut kwsearch;
    let mut kwset: kwset_t = (*kwsearch).kwset;
    let mut mb_check: bool = localeinfo.multibyte as libc::c_int
        & !localeinfo.using_utf8 as libc::c_int & !match_lines as libc::c_int != 0;
    let mut longest: bool = (mb_check as libc::c_int
        | !start_ptr.is_null() as libc::c_int | match_words as libc::c_int)
        & !match_lines as libc::c_int != 0;
    beg = if !start_ptr.is_null() { start_ptr } else { buf };
    mb_start = beg;
    's_16: loop {
        if !(beg <= buf.offset(size as isize)) {
            current_block = 4090602189656566074;
            break;
        }
        let mut kwsmatch: kwsmatch = kwsmatch {
            index: 0,
            offset: 0,
            size: 0,
        };
        let mut offset: ptrdiff_t = kwsexec(
            kwset,
            beg.offset(-(match_lines as libc::c_int as isize)),
            buf.offset(size as isize).offset_from(beg) as libc::c_long
                + match_lines as libc::c_long,
            &mut kwsmatch,
            longest,
        );
        if offset < 0 as libc::c_int as libc::c_long {
            current_block = 4090602189656566074;
            break;
        }
        len = kwsmatch.size
            - (2 as libc::c_int * match_lines as libc::c_int) as libc::c_long;
        let mut mbclen: idx_t = 0 as libc::c_int as idx_t;
        if mb_check as libc::c_int != 0
            && mb_goback(
                &mut mb_start,
                &mut mbclen,
                beg.offset(offset as isize),
                buf.offset(size as isize),
            ) != 0 as libc::c_int as libc::c_long
        {
            beg = mb_start.offset(-(1 as libc::c_int as isize));
        } else {
            beg = beg.offset(offset as isize);
            if !start_ptr.is_null() as libc::c_int & !match_words as libc::c_int != 0 {
                current_block = 12381812505308290051;
                break;
            }
            if match_lines {
                len
                    += (start_ptr == 0 as *mut libc::c_void as *const libc::c_char)
                        as libc::c_int as libc::c_long;
                current_block = 12381812505308290051;
                break;
            } else {
                if !match_words {
                    current_block = 8321443843601168651;
                    break;
                }
                if mbclen == 0 as libc::c_int as libc::c_long {
                    let mut nl: *const libc::c_char = memrchr(
                        mb_start as *const libc::c_void,
                        eol as libc::c_int,
                        beg.offset_from(mb_start) as libc::c_long as size_t,
                    ) as *const libc::c_char;
                    if !nl.is_null() {
                        mb_start = nl.offset(1 as libc::c_int as isize);
                    }
                }
                if if mbclen > 0 as libc::c_int as libc::c_long {
                    (wordchar_next(
                        beg.offset(-(mbclen as isize)),
                        buf.offset(size as isize),
                    ) == 0) as libc::c_int
                } else {
                    (wordchar_prev(mb_start, beg, buf.offset(size as isize)) == 0)
                        as libc::c_int
                } != 0
                {
                    loop {
                        if wordchar_next(
                            beg.offset(len as isize),
                            buf.offset(size as isize),
                        ) == 0
                        {
                            if !start_ptr.is_null() {
                                current_block = 12381812505308290051;
                                break 's_16;
                            } else {
                                current_block = 8321443843601168651;
                                break 's_16;
                            }
                        } else if start_ptr.is_null() && !localeinfo.multibyte {
                            if ((*kwsearch).re).is_null() {
                                fgrep_to_grep_pattern(
                                    &mut (*kwsearch).pattern,
                                    &mut (*kwsearch).size,
                                );
                                (*kwsearch)
                                    .re = GEAcompile(
                                    (*kwsearch).pattern,
                                    (*kwsearch).size,
                                    (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                                        << 1 as libc::c_int
                                        | ((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int
                                        | (((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int
                                        | (((((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int
                                        | ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int
                                        | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                                        | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int
                                        | (((((((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int)
                                        & !(((((((((((((((((((((((((1 as libc::c_int
                                            as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int
                                            | (((((((1 as libc::c_int as libc::c_ulong)
                                                << 1 as libc::c_int) << 1 as libc::c_int)
                                                << 1 as libc::c_int) << 1 as libc::c_int)
                                                << 1 as libc::c_int) << 1 as libc::c_int)
                                                << 1 as libc::c_int),
                                    !start_ptr.is_null(),
                                );
                            }
                            if beg.offset(len as isize) < buf.offset(size as isize) {
                                end = rawmemchr(
                                    beg.offset(len as isize) as *const libc::c_void,
                                    eol as libc::c_int,
                                ) as *const libc::c_char;
                                end = end.offset(1);
                                end;
                            } else {
                                end = buf.offset(size as isize);
                            }
                            if 0 as libc::c_int as libc::c_long
                                <= EGexecute(
                                    (*kwsearch).re,
                                    beg,
                                    end.offset_from(beg) as libc::c_long,
                                    match_size,
                                    0 as *const libc::c_char,
                                )
                            {
                                current_block = 4395120683927338619;
                                break 's_16;
                            }
                            beg = end.offset(-(1 as libc::c_int as isize));
                            break;
                        } else {
                            if len == 0 {
                                break;
                            }
                            let mut shorter_match: kwsmatch = kwsmatch {
                                index: 0,
                                offset: 0,
                                size: 0,
                            };
                            len -= 1;
                            if kwsexec(
                                kwset,
                                beg,
                                len,
                                &mut shorter_match,
                                1 as libc::c_int != 0,
                            ) != 0 as libc::c_int as libc::c_long
                            {
                                break;
                            }
                            len = shorter_match.size;
                        }
                    }
                }
                beg = beg
                    .offset(wordchars_size(beg, buf.offset(size as isize)) as isize);
                mb_start = beg;
            }
        }
        beg = beg.offset(1);
        beg;
    }
    match current_block {
        4090602189656566074 => return -(1 as libc::c_int) as ptrdiff_t,
        8321443843601168651 => {
            if beg.offset(len as isize) < buf.offset(size as isize) {
                end = rawmemchr(
                    beg.offset(len as isize) as *const libc::c_void,
                    eol as libc::c_int,
                ) as *const libc::c_char;
                end = end.offset(1);
                end;
            } else {
                end = buf.offset(size as isize);
            }
            current_block = 4395120683927338619;
        }
        _ => {}
    }
    match current_block {
        4395120683927338619 => {
            beg = memrchr(
                buf as *const libc::c_void,
                eol as libc::c_int,
                beg.offset_from(buf) as libc::c_long as size_t,
            ) as *const libc::c_char;
            beg = if !beg.is_null() {
                beg.offset(1 as libc::c_int as isize)
            } else {
                buf
            };
            len = end.offset_from(beg) as libc::c_long;
        }
        _ => {}
    }
    *match_size = len;
    return beg.offset_from(buf) as libc::c_long;
}

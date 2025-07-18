use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
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
    fn regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn regfree(__preg: *mut regex_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn current_addr() -> libc::c_int;
    fn dec_addr(addr: libc::c_int) -> libc::c_int;
    fn delete_lines(from: libc::c_int, to: libc::c_int, isglobal: bool_0) -> bool_0;
    fn get_sbuf_line(lp: *const line_t) -> *mut libc::c_char;
    fn inc_addr(addr: libc::c_int) -> libc::c_int;
    fn isbinary() -> bool_0;
    fn put_sbuf_line(buf: *const libc::c_char, size: libc::c_int) -> *const libc::c_char;
    fn search_line_node(addr: libc::c_int) -> *mut line_t;
    fn set_current_addr(addr: libc::c_int);
    fn push_undo_atom(
        type_0: libc::c_int,
        from: libc::c_int,
        to: libc::c_int,
    ) -> *mut undo_t;
    fn clear_active_list();
    fn set_active_node(lp: *const line_t) -> bool_0;
    fn get_stdin_line(sizep: *mut libc::c_int) -> *const libc::c_char;
    fn extended_regexp() -> bool_0;
    fn set_error_msg(msg: *const libc::c_char);
    fn disable_interrupts();
    fn enable_interrupts();
    fn resize_buffer(
        buf: *mut *mut libc::c_char,
        size: *mut libc::c_int,
        min_size: libc::c_uint,
    ) -> bool_0;
}
pub type size_t = libc::c_ulong;
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
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub q_forw: *mut line,
    pub q_back: *mut line,
    pub pos: libc::c_long,
    pub len: libc::c_int,
}
pub type line_t = line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undo_t {
    pub type_0: C2RustUnnamed,
    pub head: *mut line_t,
    pub tail: *mut line_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const VMOV: C2RustUnnamed = 3;
pub const UMOV: C2RustUnnamed = 2;
pub const UDEL: C2RustUnnamed = 1;
pub const UADD: C2RustUnnamed = 0;
pub const se_max: C2RustUnnamed_0 = 30;
pub type C2RustUnnamed_0 = libc::c_uint;
static mut no_prev_subst: *const libc::c_char = b"No previous substitution\0"
    as *const u8 as *const libc::c_char;
static mut inv_i_suf: *const libc::c_char = b"Suffix 'I' not allowed on empty regexp\0"
    as *const u8 as *const libc::c_char;
static mut inv_pat_del: *const libc::c_char = b"Invalid pattern delimiter\0" as *const u8
    as *const libc::c_char;
static mut mis_pat_del: *const libc::c_char = b"Missing pattern delimiter\0" as *const u8
    as *const libc::c_char;
static mut no_match: *const libc::c_char = b"No match\0" as *const u8
    as *const libc::c_char;
static mut no_prev_pat: *const libc::c_char = b"No previous pattern\0" as *const u8
    as *const libc::c_char;
static mut last_regexp: *mut regex_t = 0 as *const regex_t as *mut regex_t;
static mut subst_regexp: *mut regex_t = 0 as *const regex_t as *mut regex_t;
static mut rbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut rbufsz: libc::c_int = 0 as libc::c_int;
static mut rlen: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn subst_regex() -> bool_0 {
    return (subst_regexp != 0 as *mut regex_t) as libc::c_int as bool_0;
}
unsafe extern "C" fn translit_text(
    mut p: *mut libc::c_char,
    mut len: libc::c_int,
    from: libc::c_char,
    to: libc::c_char,
) {
    loop {
        len -= 1;
        if !(len >= 0 as libc::c_int) {
            break;
        }
        if *p as libc::c_int == from as libc::c_int {
            *p = to;
        }
        p = p.offset(1);
        p;
    };
}
unsafe extern "C" fn newline_to_nul(s: *mut libc::c_char, len: libc::c_int) {
    translit_text(s, len, '\n' as i32 as libc::c_char, '\0' as i32 as libc::c_char);
}
unsafe extern "C" fn nul_to_newline(s: *mut libc::c_char, len: libc::c_int) {
    translit_text(s, len, '\0' as i32 as libc::c_char, '\n' as i32 as libc::c_char);
}
unsafe extern "C" fn parse_char_class(
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut c: libc::c_char = 0;
    let mut d: libc::c_char = 0;
    if *p as libc::c_int == '^' as i32 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == ']' as i32 {
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int != ']' as i32 && *p as libc::c_int != '\n' as i32 {
        if *p as libc::c_int == '[' as i32
            && {
                d = *p.offset(1 as libc::c_int as isize);
                d as libc::c_int == '.' as i32 || d as libc::c_int == ':' as i32
                    || d as libc::c_int == '=' as i32
            }
        {
            p = p.offset(1);
            p;
            p = p.offset(1);
            c = *p;
            while *p as libc::c_int != ']' as i32 || c as libc::c_int != d as libc::c_int
            {
                c = *p;
                if c as libc::c_int == '\n' as i32 {
                    return 0 as *const libc::c_char;
                }
                p = p.offset(1);
                p;
            }
        }
        p = p.offset(1);
        p;
    }
    return if *p as libc::c_int == ']' as i32 { p } else { 0 as *const libc::c_char };
}
unsafe extern "C" fn extract_pattern(
    ibufpp: *mut *const libc::c_char,
    delimiter: libc::c_char,
) -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut nd: *const libc::c_char = *ibufpp;
    let mut len: libc::c_int = 0;
    while *nd as libc::c_int != delimiter as libc::c_int
        && *nd as libc::c_int != '\n' as i32
    {
        if *nd as libc::c_int == '[' as i32 {
            nd = nd.offset(1);
            nd = parse_char_class(nd);
            if nd.is_null() {
                set_error_msg(
                    b"Unbalanced brackets ([])\0" as *const u8 as *const libc::c_char,
                );
                return 0 as *mut libc::c_char;
            }
        } else if *nd as libc::c_int == '\\' as i32
            && {
                nd = nd.offset(1);
                *nd as libc::c_int == '\n' as i32
            }
        {
            set_error_msg(
                b"Trailing backslash (\\)\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut libc::c_char;
        }
        nd = nd.offset(1);
        nd;
    }
    len = nd.offset_from(*ibufpp) as libc::c_long as libc::c_int;
    if resize_buffer(&mut buf, &mut bufsz, (len + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return 0 as *mut libc::c_char;
    }
    memcpy(
        buf as *mut libc::c_void,
        *ibufpp as *const libc::c_void,
        len as libc::c_ulong,
    );
    *buf.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    *ibufpp = nd;
    if isbinary() as u64 != 0 {
        nul_to_newline(buf, len);
    }
    return buf;
}
unsafe extern "C" fn compile_regex(
    pat: *const libc::c_char,
    ignore_case: bool_0,
) -> *mut regex_t {
    static mut store: [regex_t; 3] = [regex_t {
        __buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
        __allocated: 0,
        __used: 0,
        __syntax: 0,
        __fastmap: 0 as *const libc::c_char as *mut libc::c_char,
        __translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        re_nsub: 0,
        __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    }; 3];
    let mut exp: *mut regex_t = 0 as *mut regex_t;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < 3 as libc::c_int {
        exp = &mut *store.as_mut_ptr().offset(n as isize) as *mut regex_t;
        if exp != last_regexp && exp != subst_regexp {
            break;
        }
        n += 1;
        n;
    }
    let cflags: libc::c_int = (if extended_regexp() as libc::c_uint != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    })
        | (if ignore_case as libc::c_uint != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int
        });
    n = regcomp(exp, pat, cflags);
    if n != 0 {
        let mut buf: [libc::c_char; 80] = [0; 80];
        regerror(
            n,
            last_regexp,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
        );
        set_error_msg(buf.as_mut_ptr());
        return 0 as *mut regex_t;
    }
    if !last_regexp.is_null() && last_regexp != subst_regexp {
        regfree(last_regexp);
    }
    last_regexp = exp;
    return last_regexp;
}
unsafe extern "C" fn get_compiled_regex(
    ibufpp: *mut *const libc::c_char,
) -> *mut regex_t {
    let delimiter: libc::c_char = **ibufpp;
    if delimiter as libc::c_int == ' ' as i32 || delimiter as libc::c_int == '\n' as i32
    {
        set_error_msg(inv_pat_del);
        return 0 as *mut regex_t;
    }
    *ibufpp = (*ibufpp).offset(1);
    if **ibufpp as libc::c_int == delimiter as libc::c_int
        || **ibufpp as libc::c_int == '\n' as i32
    {
        if last_regexp.is_null() {
            set_error_msg(no_prev_pat);
            return 0 as *mut regex_t;
        }
        if **ibufpp as libc::c_int == delimiter as libc::c_int
            && {
                *ibufpp = (*ibufpp).offset(1);
                **ibufpp as libc::c_int == 'I' as i32
            }
        {
            set_error_msg(inv_i_suf);
            return 0 as *mut regex_t;
        }
        return last_regexp;
    } else {
        let pat: *const libc::c_char = extract_pattern(ibufpp, delimiter);
        if pat.is_null() {
            return 0 as *mut regex_t;
        }
        let mut ignore_case: bool_0 = false_0;
        if **ibufpp as libc::c_int == delimiter as libc::c_int
            && {
                *ibufpp = (*ibufpp).offset(1);
                **ibufpp as libc::c_int == 'I' as i32
            }
        {
            ignore_case = true_0;
            *ibufpp = (*ibufpp).offset(1);
            *ibufpp;
        }
        return compile_regex(pat, ignore_case);
    };
}
pub unsafe extern "C" fn get_pattern_for_s(
    ibufpp: *mut *const libc::c_char,
) -> *const libc::c_char {
    let delimiter: libc::c_char = **ibufpp;
    if delimiter as libc::c_int == ' ' as i32 || delimiter as libc::c_int == '\n' as i32
    {
        set_error_msg(inv_pat_del);
        return 0 as *const libc::c_char;
    }
    *ibufpp = (*ibufpp).offset(1);
    if **ibufpp as libc::c_int == delimiter as libc::c_int {
        if last_regexp.is_null() {
            set_error_msg(no_prev_pat);
            return 0 as *const libc::c_char;
        }
        return b"\0" as *const u8 as *const libc::c_char;
    }
    let pat: *const libc::c_char = extract_pattern(ibufpp, delimiter);
    if pat.is_null() {
        return 0 as *const libc::c_char;
    }
    if **ibufpp as libc::c_int != delimiter as libc::c_int {
        set_error_msg(mis_pat_del);
        return 0 as *const libc::c_char;
    }
    return pat;
}
pub unsafe extern "C" fn set_subst_regex(
    pat: *const libc::c_char,
    ignore_case: bool_0,
) -> bool_0 {
    if pat.is_null() {
        return false_0;
    }
    if *pat == 0 && ignore_case as libc::c_uint != 0 {
        set_error_msg(inv_i_suf);
        return false_0;
    }
    disable_interrupts();
    let mut exp: *mut regex_t = if *pat as libc::c_int != 0 {
        compile_regex(pat, ignore_case)
    } else {
        last_regexp
    };
    if !exp.is_null() && exp != subst_regexp {
        if !subst_regexp.is_null() {
            regfree(subst_regexp);
        }
        subst_regexp = exp;
    }
    enable_interrupts();
    return (if !exp.is_null() { true_0 as libc::c_int } else { false_0 as libc::c_int })
        as bool_0;
}
pub unsafe extern "C" fn replace_subst_re_by_search_re() -> bool_0 {
    if last_regexp.is_null() {
        set_error_msg(no_prev_pat);
        return false_0;
    }
    if last_regexp != subst_regexp {
        disable_interrupts();
        if !subst_regexp.is_null() {
            regfree(subst_regexp);
        }
        subst_regexp = last_regexp;
        enable_interrupts();
    }
    return true_0;
}
pub unsafe extern "C" fn build_active_list(
    ibufpp: *mut *const libc::c_char,
    first_addr: libc::c_int,
    second_addr: libc::c_int,
    match_0: bool_0,
) -> bool_0 {
    let mut addr: libc::c_int = 0;
    let exp: *const regex_t = get_compiled_regex(ibufpp);
    if exp.is_null() {
        return false_0;
    }
    clear_active_list();
    let mut lp: *const line_t = search_line_node(first_addr);
    addr = first_addr;
    while addr <= second_addr {
        let s: *mut libc::c_char = get_sbuf_line(lp);
        if s.is_null() {
            return false_0;
        }
        if isbinary() as u64 != 0 {
            nul_to_newline(s, (*lp).len);
        }
        if match_0 as libc::c_uint
            == (regexec(
                exp,
                s,
                0 as libc::c_int as size_t,
                0 as *mut regmatch_t,
                0 as libc::c_int,
            ) == 0) as libc::c_int as libc::c_uint && set_active_node(lp) as u64 == 0
        {
            return false_0;
        }
        addr += 1;
        addr;
        lp = (*lp).q_forw;
    }
    return true_0;
}
pub unsafe extern "C" fn next_matching_node_addr(
    ibufpp: *mut *const libc::c_char,
) -> libc::c_int {
    let forward: bool_0 = (**ibufpp as libc::c_int == '/' as i32) as libc::c_int
        as bool_0;
    let exp: *const regex_t = get_compiled_regex(ibufpp);
    let mut addr: libc::c_int = current_addr();
    if exp.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        addr = if forward as libc::c_uint != 0 {
            inc_addr(addr)
        } else {
            dec_addr(addr)
        };
        if addr != 0 {
            let lp: *const line_t = search_line_node(addr);
            let s: *mut libc::c_char = get_sbuf_line(lp);
            if s.is_null() {
                return -(1 as libc::c_int);
            }
            if isbinary() as u64 != 0 {
                nul_to_newline(s, (*lp).len);
            }
            if regexec(
                exp,
                s,
                0 as libc::c_int as size_t,
                0 as *mut regmatch_t,
                0 as libc::c_int,
            ) == 0
            {
                return addr;
            }
        }
        if !(addr != current_addr()) {
            break;
        }
    }
    set_error_msg(no_match);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn extract_replacement(
    ibufpp: *mut *const libc::c_char,
    isglobal: bool_0,
) -> bool_0 {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let delimiter: libc::c_char = **ibufpp;
    if delimiter as libc::c_int == '\n' as i32 {
        set_error_msg(mis_pat_del);
        return false_0;
    }
    *ibufpp = (*ibufpp).offset(1);
    *ibufpp;
    if **ibufpp as libc::c_int == '%' as i32
        && (*(*ibufpp).offset(1 as libc::c_int as isize) as libc::c_int
            == delimiter as libc::c_int
            || *(*ibufpp).offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
                && (isglobal as u64 == 0
                    || *(*ibufpp).offset(2 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int))
    {
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
        if rbuf.is_null() {
            set_error_msg(no_prev_subst);
            return false_0;
        }
        return true_0;
    }
    while **ibufpp as libc::c_int != delimiter as libc::c_int {
        if **ibufpp as libc::c_int == '\n' as i32
            && (isglobal as u64 == 0
                || *(*ibufpp).offset(1 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int)
        {
            break;
        }
        if resize_buffer(&mut buf, &mut bufsz, (i + 2 as libc::c_int) as libc::c_uint)
            as u64 == 0
        {
            return false_0;
        }
        let fresh0 = *ibufpp;
        *ibufpp = (*ibufpp).offset(1);
        let fresh1 = i;
        i = i + 1;
        let ref mut fresh2 = *buf.offset(fresh1 as isize);
        *fresh2 = *fresh0;
        if *fresh2 as libc::c_int == '\\' as i32
            && {
                let fresh3 = *ibufpp;
                *ibufpp = (*ibufpp).offset(1);
                let fresh4 = i;
                i = i + 1;
                let ref mut fresh5 = *buf.offset(fresh4 as isize);
                *fresh5 = *fresh3;
                *fresh5 as libc::c_int == '\n' as i32
            } && isglobal as u64 == 0
        {
            let mut size: libc::c_int = 0 as libc::c_int;
            *ibufpp = get_stdin_line(&mut size);
            if (*ibufpp).is_null() {
                return false_0;
            }
            if size <= 0 as libc::c_int {
                return false_0;
            }
        }
    }
    if resize_buffer(&mut buf, &mut bufsz, (i + 1 as libc::c_int) as libc::c_uint) as u64
        == 0
    {
        return false_0;
    }
    *buf.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    disable_interrupts();
    let mut p: *mut libc::c_char = buf;
    buf = rbuf;
    rbuf = p;
    rlen = i;
    i = bufsz;
    bufsz = rbufsz;
    rbufsz = i;
    enable_interrupts();
    return true_0;
}
unsafe extern "C" fn replace_matched_text(
    mut txtbufp: *mut *mut libc::c_char,
    txtbufszp: *mut libc::c_int,
    txt: *const libc::c_char,
    rm: *const regmatch_t,
    mut offset: libc::c_int,
    re_nsub: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < rlen {
        let mut n: libc::c_int = 0;
        if *rbuf.offset(i as isize) as libc::c_int == '&' as i32 {
            let mut j: libc::c_int = (*rm.offset(0 as libc::c_int as isize)).rm_so;
            let mut k: libc::c_int = (*rm.offset(0 as libc::c_int as isize)).rm_eo;
            if resize_buffer(txtbufp, txtbufszp, (offset - j + k) as libc::c_uint) as u64
                == 0
            {
                return -(1 as libc::c_int);
            }
            while j < k {
                let fresh6 = j;
                j = j + 1;
                let fresh7 = offset;
                offset = offset + 1;
                *(*txtbufp).offset(fresh7 as isize) = *txt.offset(fresh6 as isize);
            }
        } else if *rbuf.offset(i as isize) as libc::c_int == '\\' as i32
            && {
                i += 1;
                *rbuf.offset(i as isize) as libc::c_int >= '1' as i32
            } && *rbuf.offset(i as isize) as libc::c_int <= '9' as i32
            && {
                n = *rbuf.offset(i as isize) as libc::c_int - '0' as i32;
                n <= re_nsub
            }
        {
            let mut j_0: libc::c_int = (*rm.offset(n as isize)).rm_so;
            let mut k_0: libc::c_int = (*rm.offset(n as isize)).rm_eo;
            if resize_buffer(txtbufp, txtbufszp, (offset - j_0 + k_0) as libc::c_uint)
                as u64 == 0
            {
                return -(1 as libc::c_int);
            }
            while j_0 < k_0 {
                let fresh8 = j_0;
                j_0 = j_0 + 1;
                let fresh9 = offset;
                offset = offset + 1;
                *(*txtbufp).offset(fresh9 as isize) = *txt.offset(fresh8 as isize);
            }
        } else {
            if resize_buffer(
                txtbufp,
                txtbufszp,
                (offset + 1 as libc::c_int) as libc::c_uint,
            ) as u64 == 0
            {
                return -(1 as libc::c_int);
            }
            let fresh10 = offset;
            offset = offset + 1;
            *(*txtbufp).offset(fresh10 as isize) = *rbuf.offset(i as isize);
        }
        i += 1;
        i;
    }
    if resize_buffer(txtbufp, txtbufszp, (offset + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return -(1 as libc::c_int);
    }
    *(*txtbufp).offset(offset as isize) = 0 as libc::c_int as libc::c_char;
    return offset;
}
unsafe extern "C" fn line_replace(
    mut txtbufp: *mut *mut libc::c_char,
    txtbufszp: *mut libc::c_int,
    lp: *const line_t,
    snum: libc::c_int,
) -> libc::c_int {
    let mut rm: [regmatch_t; 30] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 30];
    let mut txt: *mut libc::c_char = get_sbuf_line(lp);
    let mut eot: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0 as libc::c_int;
    let global: bool_0 = (snum <= 0 as libc::c_int) as libc::c_int as bool_0;
    let mut changed: bool_0 = false_0;
    if txt.is_null() {
        return -(1 as libc::c_int);
    }
    if isbinary() as u64 != 0 {
        nul_to_newline(txt, (*lp).len);
    }
    eot = txt.offset((*lp).len as isize);
    if regexec(
        subst_regexp,
        txt,
        se_max as libc::c_int as size_t,
        rm.as_mut_ptr(),
        0 as libc::c_int,
    ) == 0
    {
        let mut matchno: libc::c_int = 0 as libc::c_int;
        let mut infloop: bool_0 = false_0;
        loop {
            if global as libc::c_uint != 0
                || {
                    matchno += 1;
                    snum == matchno
                }
            {
                changed = true_0;
                i = rm[0 as libc::c_int as usize].rm_so;
                if resize_buffer(txtbufp, txtbufszp, (offset + i) as libc::c_uint) as u64
                    == 0
                {
                    return -(1 as libc::c_int);
                }
                if isbinary() as u64 != 0 {
                    newline_to_nul(txt, rm[0 as libc::c_int as usize].rm_eo);
                }
                memcpy(
                    (*txtbufp).offset(offset as isize) as *mut libc::c_void,
                    txt as *const libc::c_void,
                    i as libc::c_ulong,
                );
                offset += i;
                offset = replace_matched_text(
                    txtbufp,
                    txtbufszp,
                    txt,
                    rm.as_mut_ptr(),
                    offset,
                    (*subst_regexp).re_nsub as libc::c_int,
                );
                if offset < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
            } else {
                i = rm[0 as libc::c_int as usize].rm_eo;
                if resize_buffer(txtbufp, txtbufszp, (offset + i) as libc::c_uint) as u64
                    == 0
                {
                    return -(1 as libc::c_int);
                }
                if isbinary() as u64 != 0 {
                    newline_to_nul(txt, i);
                }
                memcpy(
                    (*txtbufp).offset(offset as isize) as *mut libc::c_void,
                    txt as *const libc::c_void,
                    i as libc::c_ulong,
                );
                offset += i;
            }
            txt = txt.offset(rm[0 as libc::c_int as usize].rm_eo as isize);
            if global as libc::c_uint != 0
                && rm[0 as libc::c_int as usize].rm_eo == 0 as libc::c_int
            {
                if infloop as u64 == 0 {
                    infloop = true_0;
                } else {
                    set_error_msg(
                        b"Infinite substitution loop\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            if !(*txt as libc::c_int != 0
                && (changed as u64 == 0 || global as libc::c_uint != 0)
                && regexec(
                    subst_regexp,
                    txt,
                    se_max as libc::c_int as size_t,
                    rm.as_mut_ptr(),
                    1 as libc::c_int,
                ) == 0)
            {
                break;
            }
        }
        i = eot.offset_from(txt) as libc::c_long as libc::c_int;
        if resize_buffer(
            txtbufp,
            txtbufszp,
            (offset + i + 2 as libc::c_int) as libc::c_uint,
        ) as u64 == 0
        {
            return -(1 as libc::c_int);
        }
        if isbinary() as u64 != 0 {
            newline_to_nul(txt, i);
        }
        memcpy(
            (*txtbufp).offset(offset as isize) as *mut libc::c_void,
            txt as *const libc::c_void,
            i as libc::c_ulong,
        );
        memcpy(
            (*txtbufp).offset(offset as isize).offset(i as isize) as *mut libc::c_void,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
    }
    return if changed as libc::c_uint != 0 {
        offset + i + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn search_and_replace(
    first_addr: libc::c_int,
    second_addr: libc::c_int,
    snum: libc::c_int,
    isglobal: bool_0,
) -> bool_0 {
    static mut txtbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut txtbufsz: libc::c_int = 0 as libc::c_int;
    let mut addr: libc::c_int = first_addr;
    let mut lc: libc::c_int = 0;
    let mut match_found: bool_0 = false_0;
    lc = 0 as libc::c_int;
    while lc <= second_addr - first_addr {
        let lp: *const line_t = search_line_node(addr);
        let size: libc::c_int = line_replace(&mut txtbuf, &mut txtbufsz, lp, snum);
        if size < 0 as libc::c_int {
            return false_0;
        }
        if size != 0 {
            let mut txt: *const libc::c_char = txtbuf;
            let eot: *const libc::c_char = txtbuf.offset(size as isize);
            let mut up: *mut undo_t = 0 as *mut undo_t;
            disable_interrupts();
            if delete_lines(addr, addr, isglobal) as u64 == 0 {
                enable_interrupts();
                return false_0;
            }
            set_current_addr(addr - 1 as libc::c_int);
            loop {
                txt = put_sbuf_line(
                    txt,
                    eot.offset_from(txt) as libc::c_long as libc::c_int,
                );
                if txt.is_null() {
                    enable_interrupts();
                    return false_0;
                }
                if !up.is_null() {
                    (*up).tail = search_line_node(current_addr());
                } else {
                    up = push_undo_atom(
                        UADD as libc::c_int,
                        current_addr(),
                        current_addr(),
                    );
                    if up.is_null() {
                        enable_interrupts();
                        return false_0;
                    }
                }
                if !(txt != eot) {
                    break;
                }
            }
            enable_interrupts();
            addr = current_addr();
            match_found = true_0;
        }
        lc += 1;
        lc;
        addr += 1;
        addr;
    }
    if match_found as u64 == 0 && isglobal as u64 == 0 {
        set_error_msg(no_match);
        return false_0;
    }
    return true_0;
}

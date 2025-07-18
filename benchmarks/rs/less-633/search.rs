use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn free(__ptr: *mut libc::c_void);
    fn iswupper(__wc: wint_t) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn lower_left();
    fn goto_line(sindex: libc::c_int);
    fn clear_eol();
    fn ch_end_seek() -> libc::c_int;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> libc::c_int;
    fn step_char(
        pp: *mut *mut libc::c_char,
        dir: libc::c_int,
        limit: *const libc::c_char,
    ) -> LWCHAR;
    fn overlay_header() -> libc::c_int;
    fn forw_line_seg(
        curr_pos: POSITION,
        skipeol: libc::c_int,
        rscroll: libc::c_int,
        nochop: libc::c_int,
    ) -> POSITION;
    fn forw_line(curr_pos: POSITION) -> POSITION;
    fn repaint();
    fn jump_loc(pos: POSITION, sline: libc::c_int);
    fn line_pfx_width() -> libc::c_int;
    fn set_mlist(mlist: *mut libc::c_void, cmdflags: libc::c_int);
    fn cmd_lastpattern() -> *mut libc::c_char;
    fn cvt_length(len: libc::c_int, ops: libc::c_int) -> libc::c_int;
    fn cvt_alloc_chpos(len: libc::c_int) -> *mut libc::c_int;
    fn cvt_text(
        odst: *mut libc::c_char,
        osrc: *mut libc::c_char,
        chpos: *mut libc::c_int,
        lenp: *mut libc::c_int,
        ops: libc::c_int,
    );
    fn chop_line() -> libc::c_int;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> POSITION;
    fn put_line();
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn back_raw_line(
        curr_pos: POSITION,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> POSITION;
    fn compile_pattern(
        pattern: *mut libc::c_char,
        search_type: libc::c_int,
        show_error: libc::c_int,
        comp_pattern: *mut *mut regex_t,
    ) -> libc::c_int;
    fn uncompile_pattern(pattern: *mut *mut regex_t);
    fn is_null_pattern(pattern: *mut regex_t) -> libc::c_int;
    fn match_pattern(
        pattern: *mut regex_t,
        tpattern: *mut libc::c_char,
        line: *mut libc::c_char,
        line_len: libc::c_int,
        sp: *mut *mut libc::c_char,
        ep: *mut *mut libc::c_char,
        nsp: libc::c_int,
        notbol: libc::c_int,
        search_type: libc::c_int,
    ) -> libc::c_int;
    fn skip_columns(
        cols: libc::c_int,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> libc::c_int;
    fn add_lnum(linenum: LINENUM, pos: POSITION);
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn position(sindex: libc::c_int) -> POSITION;
    fn get_scrpos(scrpos: *mut scrpos, where_0: libc::c_int);
    fn empty_screen() -> libc::c_int;
    fn sindex_from_sline(sline: libc::c_int) -> libc::c_int;
    static mut sigs: libc::c_int;
    static mut how_search: libc::c_int;
    static mut caseless: libc::c_int;
    static mut linenums: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut jump_sline: libc::c_int;
    static mut bs_mode: libc::c_int;
    static mut proc_backspace: libc::c_int;
    static mut proc_return: libc::c_int;
    static mut ctldisp: libc::c_int;
    static mut status_col: libc::c_int;
    static mut ml_search: *mut libc::c_void;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut screen_trashed: libc::c_int;
    static mut sc_width: libc::c_int;
    static mut hshift: libc::c_int;
    static mut nosearch_headers: libc::c_int;
    static mut header_lines: libc::c_int;
    static mut header_cols: libc::c_int;
    static mut hilite_search: libc::c_int;
    static mut size_linebuf: libc::c_int;
    static mut squished: libc::c_int;
    static mut can_goto_line: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
pub type wint_t = libc::c_uint;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_tree {
    pub first: *mut hilite_storage,
    pub current: *mut hilite_storage,
    pub root: *mut hilite_node,
    pub lookaside: *mut hilite_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_node {
    pub parent: *mut hilite_node,
    pub left: *mut hilite_node,
    pub right: *mut hilite_node,
    pub prev: *mut hilite_node,
    pub next: *mut hilite_node,
    pub red: libc::c_int,
    pub r: hilite,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite {
    pub hl_startpos: POSITION,
    pub hl_endpos: POSITION,
    pub hl_attr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_storage {
    pub capacity: libc::c_int,
    pub used: libc::c_int,
    pub next: *mut hilite_storage,
    pub nodes: *mut hilite_node,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_info {
    pub compiled: *mut regex_t,
    pub text: *mut libc::c_char,
    pub search_type: libc::c_int,
    pub is_ucase_pattern: libc::c_int,
    pub next: *mut pattern_info,
}
static mut hide_hilite: libc::c_int = 0;
static mut prep_startpos: POSITION = 0;
static mut prep_endpos: POSITION = 0;
static mut hilite_anchor: hilite_tree = {
    let mut init = hilite_tree {
        first: 0 as *const hilite_storage as *mut hilite_storage,
        current: 0 as *const hilite_storage as *mut hilite_storage,
        root: 0 as *const hilite_node as *mut hilite_node,
        lookaside: 0 as *const hilite_node as *mut hilite_node,
    };
    init
};
static mut filter_anchor: hilite_tree = {
    let mut init = hilite_tree {
        first: 0 as *const hilite_storage as *mut hilite_storage,
        current: 0 as *const hilite_storage as *mut hilite_storage,
        root: 0 as *const hilite_node as *mut hilite_node,
        lookaside: 0 as *const hilite_node as *mut hilite_node,
    };
    init
};
static mut filter_infos: *mut pattern_info = 0 as *const pattern_info
    as *mut pattern_info;
static mut search_info: pattern_info = pattern_info {
    compiled: 0 as *const regex_t as *mut regex_t,
    text: 0 as *const libc::c_char as *mut libc::c_char,
    search_type: 0,
    is_ucase_pattern: 0,
    next: 0 as *const pattern_info as *mut pattern_info,
};
pub static mut is_caseless: libc::c_int = 0;
unsafe extern "C" fn is_ucase(mut str: *mut libc::c_char) -> libc::c_int {
    let mut str_end: *mut libc::c_char = str.offset(strlen(str) as isize);
    let mut ch: LWCHAR = 0;
    while str < str_end {
        ch = step_char(&mut str, 1 as libc::c_int, str_end);
        if iswupper(ch as wint_t) != 0 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn clear_pattern(mut info: *mut pattern_info) {
    if !((*info).text).is_null() {
        free((*info).text as *mut libc::c_void);
    }
    (*info).text = 0 as *mut libc::c_char;
    uncompile_pattern(&mut (*info).compiled);
}
unsafe extern "C" fn set_pattern(
    mut info: *mut pattern_info,
    mut pattern: *mut libc::c_char,
    mut search_type: libc::c_int,
    mut show_error: libc::c_int,
) -> libc::c_int {
    (*info)
        .is_ucase_pattern = if pattern.is_null() {
        0 as libc::c_int
    } else {
        is_ucase(pattern)
    };
    is_caseless = if (*info).is_ucase_pattern != 0 && caseless != 2 as libc::c_int {
        0 as libc::c_int
    } else {
        caseless
    };
    if pattern.is_null() {
        (*info).compiled = 0 as *mut regex_t;
    } else if compile_pattern(pattern, search_type, show_error, &mut (*info).compiled)
        < 0 as libc::c_int
    {
        return -(1 as libc::c_int)
    }
    if !((*info).text).is_null() {
        free((*info).text as *mut libc::c_void);
    }
    (*info).text = 0 as *mut libc::c_char;
    if !pattern.is_null() {
        (*info)
            .text = ecalloc(
            1 as libc::c_int,
            (strlen(pattern)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_char;
        strcpy((*info).text, pattern);
    }
    (*info).search_type = search_type;
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_pattern(mut info: *mut pattern_info) {
    (*info).compiled = 0 as *mut regex_t;
    (*info).text = 0 as *mut libc::c_char;
    (*info).search_type = 0 as libc::c_int;
    (*info).next = 0 as *mut pattern_info;
}
pub unsafe extern "C" fn init_search() {
    init_pattern(&mut search_info);
}
unsafe extern "C" fn get_cvt_ops(mut search_type: libc::c_int) -> libc::c_int {
    let mut ops: libc::c_int = 0 as libc::c_int;
    if is_caseless != 0
        && (1 as libc::c_int == 0
            || search_type & (1 as libc::c_int) << 12 as libc::c_int != 0)
    {
        ops |= 0o1 as libc::c_int;
    }
    if proc_backspace == 1 as libc::c_int
        || bs_mode == 0 as libc::c_int && proc_backspace == 0 as libc::c_int
    {
        ops |= 0o2 as libc::c_int;
    }
    if proc_return == 1 as libc::c_int
        || bs_mode != 2 as libc::c_int && proc_backspace == 0 as libc::c_int
    {
        ops |= 0o4 as libc::c_int;
    }
    if ctldisp == 2 as libc::c_int {
        ops |= 0o10 as libc::c_int;
    }
    return ops;
}
unsafe extern "C" fn prev_pattern(mut info: *mut pattern_info) -> libc::c_int {
    if (*info).search_type & (1 as libc::c_int) << 12 as libc::c_int == 0 as libc::c_int
    {
        return (is_null_pattern((*info).compiled) == 0) as libc::c_int;
    }
    return ((*info).text != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn repaint_hilite(mut on: libc::c_int) {
    let mut sindex: libc::c_int = 0;
    let mut pos: POSITION = 0;
    let mut save_hide_hilite: libc::c_int = 0;
    if squished != 0 {
        repaint();
    }
    save_hide_hilite = hide_hilite;
    if on == 0 {
        if hide_hilite != 0 {
            return;
        }
        hide_hilite = 1 as libc::c_int;
    }
    if can_goto_line == 0 {
        repaint();
        hide_hilite = save_hide_hilite;
        return;
    }
    sindex = 0 as libc::c_int;
    while sindex < 0 as libc::c_int + sc_height - 1 as libc::c_int {
        pos = position(sindex);
        if !(pos == -(1 as libc::c_int) as POSITION) {
            forw_line(pos);
            goto_line(sindex);
            clear_eol();
            put_line();
        }
        sindex += 1;
        sindex;
    }
    overlay_header();
    lower_left();
    hide_hilite = save_hide_hilite;
}
pub unsafe extern "C" fn clear_attn() {
    let mut sindex: libc::c_int = 0;
    let mut old_start_attnpos: POSITION = 0;
    let mut old_end_attnpos: POSITION = 0;
    let mut pos: POSITION = 0;
    let mut epos: POSITION = 0;
    let mut moved: libc::c_int = 0 as libc::c_int;
    if start_attnpos == -(1 as libc::c_int) as POSITION {
        return;
    }
    old_start_attnpos = start_attnpos;
    old_end_attnpos = end_attnpos;
    end_attnpos = -(1 as libc::c_int) as POSITION;
    start_attnpos = end_attnpos;
    if can_goto_line == 0 {
        repaint();
        return;
    }
    if squished != 0 {
        repaint();
    }
    sindex = 0 as libc::c_int;
    while sindex < 0 as libc::c_int + sc_height - 1 as libc::c_int {
        pos = position(sindex);
        if !(pos == -(1 as libc::c_int) as POSITION) {
            epos = position(sindex + 1 as libc::c_int);
            if pos <= old_end_attnpos
                && (epos == -(1 as libc::c_int) as POSITION || epos > old_start_attnpos)
            {
                forw_line(pos);
                goto_line(sindex);
                clear_eol();
                put_line();
                moved = 1 as libc::c_int;
            }
        }
        sindex += 1;
        sindex;
    }
    if overlay_header() != 0 {
        moved = 1 as libc::c_int;
    }
    if moved != 0 {
        lower_left();
    }
}
pub unsafe extern "C" fn undo_search(mut clear: libc::c_int) {
    clear_pattern(&mut search_info);
    if clear != 0 {
        clr_hilite();
    } else {
        if (hilite_anchor.first).is_null() {
            error(
                b"No previous regular expression\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
            return;
        }
        hide_hilite = (hide_hilite == 0) as libc::c_int;
    }
    repaint_hilite(1 as libc::c_int);
}
pub unsafe extern "C" fn clr_hlist(mut anchor: *mut hilite_tree) {
    let mut hls: *mut hilite_storage = 0 as *mut hilite_storage;
    let mut nexthls: *mut hilite_storage = 0 as *mut hilite_storage;
    hls = (*anchor).first;
    while !hls.is_null() {
        nexthls = (*hls).next;
        free((*hls).nodes as *mut libc::c_void);
        free(hls as *mut libc::c_void);
        hls = nexthls;
    }
    (*anchor).first = 0 as *mut hilite_storage;
    (*anchor).current = 0 as *mut hilite_storage;
    (*anchor).root = 0 as *mut hilite_node;
    (*anchor).lookaside = 0 as *mut hilite_node;
    prep_endpos = -(1 as libc::c_int) as POSITION;
    prep_startpos = prep_endpos;
}
pub unsafe extern "C" fn clr_hilite() {
    clr_hlist(&mut hilite_anchor);
}
pub unsafe extern "C" fn clr_filter() {
    clr_hlist(&mut filter_anchor);
}
unsafe extern "C" fn hlist_find(
    mut anchor: *mut hilite_tree,
    mut pos: POSITION,
) -> *mut hilite_node {
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    let mut m: *mut hilite_node = 0 as *mut hilite_node;
    if !((*anchor).lookaside).is_null() {
        let mut steps: libc::c_int = 0 as libc::c_int;
        let mut hit: libc::c_int = 0 as libc::c_int;
        n = (*anchor).lookaside;
        loop {
            if pos < (*n).r.hl_endpos {
                if ((*n).prev).is_null() || pos >= (*(*n).prev).r.hl_endpos {
                    hit = 1 as libc::c_int;
                    break;
                }
            } else if ((*n).next).is_null() {
                n = 0 as *mut hilite_node;
                hit = 1 as libc::c_int;
                break;
            }
            if steps >= 2 as libc::c_int {
                break;
            }
            steps += 1;
            steps;
            if pos < (*n).r.hl_endpos {
                n = (*n).prev;
                (*anchor).lookaside = n;
            } else {
                n = (*n).next;
                (*anchor).lookaside = n;
            }
        }
        if hit != 0 {
            return n;
        }
    }
    n = (*anchor).root;
    m = 0 as *mut hilite_node;
    while !n.is_null() {
        if pos < (*n).r.hl_startpos {
            if ((*n).left).is_null() {
                break;
            }
            m = n;
            n = (*n).left;
        } else {
            if !(pos >= (*n).r.hl_endpos) {
                break;
            }
            if !((*n).right).is_null() {
                n = (*n).right;
            } else {
                if !m.is_null() {
                    n = m;
                } else {
                    m = n;
                    n = 0 as *mut hilite_node;
                }
                break;
            }
        }
    }
    if !n.is_null() {
        (*anchor).lookaside = n;
    } else if !m.is_null() {
        (*anchor).lookaside = m;
    }
    return n;
}
unsafe extern "C" fn hilited_range_attr(
    mut pos: POSITION,
    mut epos: POSITION,
) -> libc::c_int {
    let mut n: *mut hilite_node = hlist_find(&mut hilite_anchor, pos);
    if n.is_null() {
        return 0 as libc::c_int;
    }
    if epos != -(1 as libc::c_int) as POSITION && epos <= (*n).r.hl_startpos {
        return 0 as libc::c_int;
    }
    return (*n).r.hl_attr;
}
pub unsafe extern "C" fn is_filtered(mut pos: POSITION) -> libc::c_int {
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    if ch_getflags() & 0o10 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    n = hlist_find(&mut filter_anchor, pos);
    return (!n.is_null() && pos >= (*n).r.hl_startpos) as libc::c_int;
}
pub unsafe extern "C" fn next_unfiltered(mut pos: POSITION) -> POSITION {
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    if ch_getflags() & 0o10 as libc::c_int != 0 {
        return pos;
    }
    n = hlist_find(&mut filter_anchor, pos);
    while !n.is_null() && pos >= (*n).r.hl_startpos {
        pos = (*n).r.hl_endpos;
        n = (*n).next;
    }
    return pos;
}
pub unsafe extern "C" fn prev_unfiltered(mut pos: POSITION) -> POSITION {
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    if ch_getflags() & 0o10 as libc::c_int != 0 {
        return pos;
    }
    n = hlist_find(&mut filter_anchor, pos);
    while !n.is_null() && pos >= (*n).r.hl_startpos {
        pos = (*n).r.hl_startpos;
        if pos == 0 as libc::c_int as libc::c_long {
            break;
        }
        pos -= 1;
        pos;
        n = (*n).prev;
    }
    return pos;
}
pub unsafe extern "C" fn is_hilited_attr(
    mut pos: POSITION,
    mut epos: POSITION,
    mut nohide: libc::c_int,
    mut p_matches: *mut libc::c_int,
) -> libc::c_int {
    let mut attr: libc::c_int = 0;
    if !p_matches.is_null() {
        *p_matches = 0 as libc::c_int;
    }
    if status_col == 0 && start_attnpos != -(1 as libc::c_int) as POSITION
        && pos <= end_attnpos
        && (epos == -(1 as libc::c_int) as POSITION || epos >= start_attnpos)
    {
        return (1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int;
    }
    attr = hilited_range_attr(pos, epos);
    if attr == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if p_matches.is_null() {
        return attr;
    }
    *p_matches = 1 as libc::c_int;
    if hilite_search == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if nohide == 0 && hide_hilite != 0 {
        return 0 as libc::c_int;
    }
    return attr;
}
unsafe extern "C" fn hlist_getstorage(
    mut anchor: *mut hilite_tree,
) -> *mut hilite_storage {
    let mut capacity: libc::c_int = 1 as libc::c_int;
    let mut s: *mut hilite_storage = 0 as *mut hilite_storage;
    if !((*anchor).current).is_null() {
        if (*(*anchor).current).used < (*(*anchor).current).capacity {
            return (*anchor).current;
        }
        capacity = (*(*anchor).current).capacity * 2 as libc::c_int;
    }
    s = ecalloc(
        1 as libc::c_int,
        ::std::mem::size_of::<hilite_storage>() as libc::c_ulong as libc::c_uint,
    ) as *mut hilite_storage;
    (*s)
        .nodes = ecalloc(
        capacity,
        ::std::mem::size_of::<hilite_node>() as libc::c_ulong as libc::c_uint,
    ) as *mut hilite_node;
    (*s).capacity = capacity;
    (*s).used = 0 as libc::c_int;
    (*s).next = 0 as *mut hilite_storage;
    if !((*anchor).current).is_null() {
        (*(*anchor).current).next = s;
    } else {
        (*anchor).first = s;
    }
    (*anchor).current = s;
    return s;
}
unsafe extern "C" fn hlist_getnode(mut anchor: *mut hilite_tree) -> *mut hilite_node {
    let mut s: *mut hilite_storage = hlist_getstorage(anchor);
    let fresh0 = (*s).used;
    (*s).used = (*s).used + 1;
    return &mut *((*s).nodes).offset(fresh0 as isize) as *mut hilite_node;
}
unsafe extern "C" fn hlist_rotate_left(
    mut anchor: *mut hilite_tree,
    mut n: *mut hilite_node,
) {
    let mut np: *mut hilite_node = (*n).parent;
    let mut nr: *mut hilite_node = (*n).right;
    let mut nrl: *mut hilite_node = (*(*n).right).left;
    if !np.is_null() {
        if n == (*np).left {
            (*np).left = nr;
        } else {
            (*np).right = nr;
        }
    } else {
        (*anchor).root = nr;
    }
    (*nr).left = n;
    (*n).right = nrl;
    (*nr).parent = np;
    (*n).parent = nr;
    if !nrl.is_null() {
        (*nrl).parent = n;
    }
}
unsafe extern "C" fn hlist_rotate_right(
    mut anchor: *mut hilite_tree,
    mut n: *mut hilite_node,
) {
    let mut np: *mut hilite_node = (*n).parent;
    let mut nl: *mut hilite_node = (*n).left;
    let mut nlr: *mut hilite_node = (*(*n).left).right;
    if !np.is_null() {
        if n == (*np).right {
            (*np).right = nl;
        } else {
            (*np).left = nl;
        }
    } else {
        (*anchor).root = nl;
    }
    (*nl).right = n;
    (*n).left = nlr;
    (*nl).parent = np;
    (*n).parent = nl;
    if !nlr.is_null() {
        (*nlr).parent = n;
    }
}
unsafe extern "C" fn add_hilite(mut anchor: *mut hilite_tree, mut hl: *mut hilite) {
    let mut p: *mut hilite_node = 0 as *mut hilite_node;
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    let mut u: *mut hilite_node = 0 as *mut hilite_node;
    if (*hl).hl_startpos >= (*hl).hl_endpos {
        return;
    }
    p = (*anchor).root;
    if p.is_null() {
        n = hlist_getnode(anchor);
        (*n).r = *hl;
        (*anchor).root = n;
        (*anchor).lookaside = n;
        return;
    }
    loop {
        if (*hl).hl_startpos < (*p).r.hl_startpos {
            if (*hl).hl_endpos > (*p).r.hl_startpos && (*hl).hl_attr == (*p).r.hl_attr {
                (*hl).hl_endpos = (*p).r.hl_startpos;
            }
            if ((*p).left).is_null() {
                break;
            }
            p = (*p).left;
        } else {
            if (*hl).hl_startpos < (*p).r.hl_endpos && (*hl).hl_attr == (*p).r.hl_attr {
                (*hl).hl_startpos = (*p).r.hl_endpos;
                if (*hl).hl_startpos >= (*hl).hl_endpos {
                    return;
                }
            }
            if ((*p).right).is_null() {
                break;
            }
            p = (*p).right;
        }
    }
    if (*hl).hl_startpos < (*p).r.hl_startpos {
        if (*hl).hl_attr == (*p).r.hl_attr {
            if (*hl).hl_endpos == (*p).r.hl_startpos {
                (*p).r.hl_startpos = (*hl).hl_startpos;
                return;
            }
            if !((*p).prev).is_null() && (*(*p).prev).r.hl_endpos == (*hl).hl_startpos {
                (*(*p).prev).r.hl_endpos = (*hl).hl_endpos;
                return;
            }
        }
        n = hlist_getnode(anchor);
        (*p).left = n;
        (*n).next = p;
        if !((*p).prev).is_null() {
            (*n).prev = (*p).prev;
            (*(*p).prev).next = n;
        }
        (*p).prev = n;
    } else {
        if (*hl).hl_attr == (*p).r.hl_attr {
            if (*p).r.hl_endpos == (*hl).hl_startpos {
                (*p).r.hl_endpos = (*hl).hl_endpos;
                return;
            }
            if !((*p).next).is_null() && (*hl).hl_endpos == (*(*p).next).r.hl_startpos {
                (*(*p).next).r.hl_startpos = (*hl).hl_startpos;
                return;
            }
        }
        n = hlist_getnode(anchor);
        (*p).right = n;
        (*n).prev = p;
        if !((*p).next).is_null() {
            (*n).next = (*p).next;
            (*(*p).next).prev = n;
        }
        (*p).next = n;
    }
    (*n).parent = p;
    (*n).red = 1 as libc::c_int;
    (*n).r = *hl;
    loop {
        if ((*n).parent).is_null() {
            (*n).red = 0 as libc::c_int;
            break;
        } else {
            if (*(*n).parent).red == 0 {
                break;
            }
            u = (*(*(*n).parent).parent).left;
            if (*n).parent == u {
                u = (*(*(*n).parent).parent).right;
            }
            if !u.is_null() && (*u).red != 0 {
                (*(*n).parent).red = 0 as libc::c_int;
                (*u).red = 0 as libc::c_int;
                n = (*(*n).parent).parent;
                (*n).red = 1 as libc::c_int;
            } else {
                if n == (*(*n).parent).right
                    && (*n).parent == (*(*(*n).parent).parent).left
                {
                    hlist_rotate_left(anchor, (*n).parent);
                    n = (*n).left;
                } else if n == (*(*n).parent).left
                    && (*n).parent == (*(*(*n).parent).parent).right
                {
                    hlist_rotate_right(anchor, (*n).parent);
                    n = (*n).right;
                }
                (*(*n).parent).red = 0 as libc::c_int;
                (*(*(*n).parent).parent).red = 1 as libc::c_int;
                if n == (*(*n).parent).left {
                    hlist_rotate_right(anchor, (*(*n).parent).parent);
                } else {
                    hlist_rotate_left(anchor, (*(*n).parent).parent);
                }
                break;
            }
        }
    };
}
unsafe extern "C" fn create_hilites(
    mut linepos: POSITION,
    mut line: *mut libc::c_char,
    mut sp: *mut libc::c_char,
    mut ep: *mut libc::c_char,
    mut attr: libc::c_int,
    mut chpos: *mut libc::c_int,
) {
    let mut start_index: libc::c_int = sp.offset_from(line) as libc::c_long
        as libc::c_int;
    let mut end_index: libc::c_int = ep.offset_from(line) as libc::c_long as libc::c_int;
    let mut hl: hilite = hilite {
        hl_startpos: 0,
        hl_endpos: 0,
        hl_attr: 0,
    };
    let mut i: libc::c_int = 0;
    hl.hl_startpos = linepos + *chpos.offset(start_index as isize) as libc::c_long;
    hl.hl_attr = attr;
    i = start_index + 1 as libc::c_int;
    while i <= end_index {
        if *chpos.offset(i as isize)
            != *chpos.offset((i - 1 as libc::c_int) as isize) + 1 as libc::c_int
            || i == end_index
        {
            hl
                .hl_endpos = linepos
                + *chpos.offset((i - 1 as libc::c_int) as isize) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            add_hilite(&mut hilite_anchor, &mut hl);
            if i < end_index {
                hl.hl_startpos = linepos + *chpos.offset(i as isize) as libc::c_long;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn hilite_line(
    mut linepos: POSITION,
    mut line: *mut libc::c_char,
    mut line_len: libc::c_int,
    mut chpos: *mut libc::c_int,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
    mut nsp: libc::c_int,
    mut cvt_ops: libc::c_int,
) {
    let mut searchp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_end: *mut libc::c_char = line.offset(line_len as isize);
    searchp = line;
    loop {
        let mut lep: *mut libc::c_char = *sp.offset(0 as libc::c_int as isize);
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < nsp {
            if (*sp.offset(i as isize)).is_null() || (*ep.offset(i as isize)).is_null() {
                break;
            }
            if *ep.offset(i as isize) > *sp.offset(i as isize) {
                create_hilites(
                    linepos,
                    line,
                    lep,
                    *sp.offset(i as isize),
                    (1 as libc::c_int) << 6 as libc::c_int
                        | (10 as libc::c_int) << 8 as libc::c_int,
                    chpos,
                );
                create_hilites(
                    linepos,
                    line,
                    *sp.offset(i as isize),
                    *ep.offset(i as isize),
                    (1 as libc::c_int) << 6 as libc::c_int
                        | 10 as libc::c_int + i << 8 as libc::c_int,
                    chpos,
                );
                lep = *ep.offset(i as isize);
            }
            i += 1;
            i;
        }
        create_hilites(
            linepos,
            line,
            lep,
            *ep.offset(0 as libc::c_int as isize),
            (1 as libc::c_int) << 6 as libc::c_int
                | (10 as libc::c_int) << 8 as libc::c_int,
            chpos,
        );
        if *ep.offset(0 as libc::c_int as isize) > searchp {
            searchp = *ep.offset(0 as libc::c_int as isize);
        } else {
            if !(searchp != line_end) {
                break;
            }
            searchp = searchp.offset(1);
            searchp;
        }
        if !(match_pattern(
            search_info.compiled,
            search_info.text,
            searchp,
            line_end.offset_from(searchp) as libc::c_long as libc::c_int,
            sp,
            ep,
            nsp,
            1 as libc::c_int,
            search_info.search_type,
        ) != 0)
        {
            break;
        }
    };
}
unsafe extern "C" fn hilite_screen() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    get_scrpos(&mut scrpos, 0 as libc::c_int);
    if scrpos.pos == -(1 as libc::c_int) as POSITION {
        return;
    }
    prep_hilite(scrpos.pos, position(-(2 as libc::c_int)), -(1 as libc::c_int));
    repaint_hilite(1 as libc::c_int);
}
pub unsafe extern "C" fn chg_hilite() {
    clr_hilite();
    hide_hilite = 0 as libc::c_int;
    if hilite_search == 2 as libc::c_int {
        hilite_screen();
    }
}
unsafe extern "C" fn search_pos(mut search_type: libc::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    let mut sindex: libc::c_int = 0;
    if empty_screen() != 0 {
        if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            pos = 0 as libc::c_int as POSITION;
        } else {
            pos = ch_length();
            if pos == -(1 as libc::c_int) as POSITION {
                ch_end_seek();
                pos = ch_length();
            }
        }
        sindex = 0 as libc::c_int;
    } else {
        let mut add_one: libc::c_int = 0 as libc::c_int;
        if how_search == 1 as libc::c_int {
            if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                sindex = sc_height - 1 as libc::c_int;
            } else {
                sindex = 0 as libc::c_int;
            }
        } else if how_search == 2 as libc::c_int
            && search_type & (1 as libc::c_int) << 14 as libc::c_int == 0
        {
            if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                sindex = 0 as libc::c_int;
            } else {
                sindex = sc_height - 1 as libc::c_int;
            }
        } else {
            sindex = sindex_from_sline(jump_sline);
            if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                add_one = 1 as libc::c_int;
            }
        }
        pos = position(sindex);
        if add_one != 0 {
            pos = forw_raw_line(
                pos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
        }
    }
    if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        while pos == -(1 as libc::c_int) as POSITION {
            sindex += 1;
            if sindex >= sc_height {
                break;
            }
            pos = position(sindex);
        }
    } else {
        while pos == -(1 as libc::c_int) as POSITION {
            sindex -= 1;
            if sindex < 0 as libc::c_int {
                break;
            }
            pos = position(sindex);
        }
    }
    return pos;
}
unsafe extern "C" fn matches_filters(
    mut pos: POSITION,
    mut cline: *mut libc::c_char,
    mut line_len: libc::c_int,
    mut chpos: *mut libc::c_int,
    mut linepos: POSITION,
    mut sp: *mut *mut libc::c_char,
    mut ep: *mut *mut libc::c_char,
    mut nsp: libc::c_int,
) -> libc::c_int {
    let mut filter: *mut pattern_info = 0 as *mut pattern_info;
    filter = filter_infos;
    while !filter.is_null() {
        let mut line_filter: libc::c_int = match_pattern(
            (*filter).compiled,
            (*filter).text,
            cline,
            line_len,
            sp,
            ep,
            nsp,
            0 as libc::c_int,
            (*filter).search_type,
        );
        if line_filter != 0 {
            let mut hl: hilite = hilite {
                hl_startpos: 0,
                hl_endpos: 0,
                hl_attr: 0,
            };
            hl.hl_startpos = linepos;
            hl.hl_endpos = pos;
            add_hilite(&mut filter_anchor, &mut hl);
            free(cline as *mut libc::c_void);
            free(chpos as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        filter = (*filter).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_lastlinepos(
    mut pos: POSITION,
    mut tpos: POSITION,
    mut sheight: libc::c_int,
) -> POSITION {
    let mut nlines: libc::c_int = 0;
    nlines = 0 as libc::c_int;
    loop {
        let mut npos: POSITION = forw_line(pos);
        if npos > tpos {
            if nlines < sheight {
                return -(1 as libc::c_int) as POSITION;
            }
            return pos;
        }
        pos = npos;
        nlines += 1;
        nlines;
    };
}
unsafe extern "C" fn get_seg(mut pos: POSITION, mut tpos: POSITION) -> libc::c_int {
    let mut seg: libc::c_int = 0;
    seg = 0 as libc::c_int;
    loop {
        let mut npos: POSITION = forw_line_seg(
            pos,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if npos > tpos {
            return seg;
        }
        pos = npos;
        seg += 1;
        seg;
    };
}
unsafe extern "C" fn search_range(
    mut pos: POSITION,
    mut endpos: POSITION,
    mut search_type: libc::c_int,
    mut matches: libc::c_int,
    mut maxlines: libc::c_int,
    mut plinepos: *mut POSITION,
    mut pendpos: *mut POSITION,
    mut plastlinepos: *mut POSITION,
) -> libc::c_int {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_len: libc::c_int = 0;
    let mut linenum: LINENUM = 0;
    let mut sp: [*mut libc::c_char; 7] = [0 as *mut libc::c_char; 7];
    let mut ep: [*mut libc::c_char; 7] = [0 as *mut libc::c_char; 7];
    let mut line_match: libc::c_int = 0;
    let mut cvt_ops: libc::c_int = 0;
    let mut cvt_len: libc::c_int = 0;
    let mut chpos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut linepos: POSITION = 0;
    let mut oldpos: POSITION = 0;
    let mut skip_bytes: libc::c_int = 0 as libc::c_int;
    let mut swidth: libc::c_int = sc_width - line_pfx_width();
    let mut sheight: libc::c_int = sc_height - sindex_from_sline(jump_sline);
    linenum = find_linenum(pos);
    if nosearch_headers != 0 && linenum <= header_lines as libc::c_long {
        linenum = (header_lines + 1 as libc::c_int) as LINENUM;
        pos = find_pos(linenum);
    }
    if pos == -(1 as libc::c_int) as POSITION {
        return -(1 as libc::c_int);
    }
    oldpos = pos;
    if search_type & (1 as libc::c_int) << 15 as libc::c_int != 0
        && endpos == -(1 as libc::c_int) as POSITION
    {
        endpos = pos;
    }
    loop {
        if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            return -(1 as libc::c_int);
        }
        if endpos != -(1 as libc::c_int) as POSITION
            && search_type & (1 as libc::c_int) << 15 as libc::c_int == 0
            && (search_type & (1 as libc::c_int) << 0 as libc::c_int != 0
                && pos >= endpos
                || search_type & (1 as libc::c_int) << 1 as libc::c_int != 0
                    && pos <= endpos) || maxlines == 0 as libc::c_int
        {
            if !pendpos.is_null() {
                *pendpos = pos;
            }
            return matches;
        }
        if maxlines > 0 as libc::c_int {
            maxlines -= 1;
            maxlines;
        }
        if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            linepos = pos;
            pos = forw_raw_line(pos, &mut line, &mut line_len);
            if linenum != 0 as libc::c_int as libc::c_long {
                linenum += 1;
                linenum;
            }
        } else {
            pos = back_raw_line(pos, &mut line, &mut line_len);
            linepos = pos;
            if linenum != 0 as libc::c_int as libc::c_long {
                linenum -= 1;
                linenum;
            }
        }
        if pos == -(1 as libc::c_int) as POSITION {
            if search_type & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                    pos = 0 as libc::c_int as POSITION;
                } else {
                    pos = ch_length();
                    if pos == -(1 as libc::c_int) as POSITION {
                        ch_end_seek();
                        pos = ch_length();
                    }
                }
                if pos != -(1 as libc::c_int) as POSITION {
                    search_type &= !((1 as libc::c_int) << 15 as libc::c_int);
                    linenum = find_linenum(pos);
                    continue;
                }
            }
            if !pendpos.is_null() {
                *pendpos = oldpos;
            }
            return matches;
        } else {
            if linenums != 0 && abs((pos - oldpos) as libc::c_int) > 2048 as libc::c_int
            {
                add_lnum(linenum, pos);
            }
            oldpos = pos;
            if is_filtered(linepos) != 0 {
                continue;
            }
            if nosearch_headers != 0 {
                skip_bytes = skip_columns(header_cols, &mut line, &mut line_len);
            }
            cvt_ops = get_cvt_ops(search_type);
            cvt_len = cvt_length(line_len, cvt_ops);
            cline = ecalloc(1 as libc::c_int, cvt_len as libc::c_uint)
                as *mut libc::c_char;
            chpos = cvt_alloc_chpos(cvt_len);
            cvt_text(cline, line, chpos, &mut line_len, cvt_ops);
            if !filter_infos.is_null()
                && (search_type & (1 as libc::c_int) << 4 as libc::c_int != 0
                    || prep_startpos == -(1 as libc::c_int) as POSITION
                    || linepos < prep_startpos || linepos >= prep_endpos)
            {
                if matches_filters(
                    pos,
                    cline,
                    line_len,
                    chpos,
                    linepos,
                    sp.as_mut_ptr(),
                    ep.as_mut_ptr(),
                    16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int
                        + 2 as libc::c_int,
                ) != 0
                {
                    continue;
                }
            }
            if prev_pattern(&mut search_info) != 0 {
                line_match = match_pattern(
                    search_info.compiled,
                    search_info.text,
                    cline,
                    line_len,
                    sp.as_mut_ptr(),
                    ep.as_mut_ptr(),
                    16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int
                        + 2 as libc::c_int,
                    0 as libc::c_int,
                    search_type,
                );
                if line_match != 0 {
                    if search_type & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                        hilite_line(
                            linepos + skip_bytes as libc::c_long,
                            cline,
                            line_len,
                            chpos,
                            sp.as_mut_ptr(),
                            ep.as_mut_ptr(),
                            16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int
                                + 2 as libc::c_int,
                            cvt_ops,
                        );
                    } else {
                        matches -= 1;
                        if matches <= 0 as libc::c_int {
                            if hilite_search == 1 as libc::c_int {
                                clr_hilite();
                                hilite_line(
                                    linepos + skip_bytes as libc::c_long,
                                    cline,
                                    line_len,
                                    chpos,
                                    sp.as_mut_ptr(),
                                    ep.as_mut_ptr(),
                                    16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int
                                        + 2 as libc::c_int,
                                    cvt_ops,
                                );
                            }
                            if chop_line() != 0 {
                                if !(sp[0 as libc::c_int as usize]).is_null()
                                    && !(ep[0 as libc::c_int as usize]).is_null()
                                {
                                    let mut start_off: libc::c_int = (sp[0 as libc::c_int
                                        as usize])
                                        .offset_from(cline) as libc::c_long as libc::c_int;
                                    let mut end_off: libc::c_int = (ep[0 as libc::c_int
                                        as usize])
                                        .offset_from(cline) as libc::c_long as libc::c_int;
                                    let mut save_hshift: libc::c_int = hshift;
                                    let mut sshift: libc::c_int = 0;
                                    let mut eshift: libc::c_int = 0;
                                    hshift = 0 as libc::c_int;
                                    sshift = swidth
                                        * get_seg(
                                            linepos,
                                            linepos + *chpos.offset(start_off as isize) as libc::c_long,
                                        );
                                    eshift = swidth
                                        * get_seg(
                                            linepos,
                                            linepos + *chpos.offset(end_off as isize) as libc::c_long,
                                        );
                                    if sshift >= save_hshift && eshift <= save_hshift {
                                        hshift = save_hshift;
                                    } else {
                                        hshift = sshift;
                                        screen_trashed = 1 as libc::c_int;
                                    }
                                }
                            } else if !plastlinepos.is_null() {
                                if !(ep[0 as libc::c_int as usize]).is_null() {
                                    let mut end_off_0: libc::c_int = (ep[0 as libc::c_int
                                        as usize])
                                        .offset_from(cline) as libc::c_long as libc::c_int;
                                    if end_off_0 >= swidth * sheight / 4 as libc::c_int {
                                        *plastlinepos = get_lastlinepos(
                                            linepos,
                                            linepos + *chpos.offset(end_off_0 as isize) as libc::c_long,
                                            sheight,
                                        );
                                    }
                                }
                            }
                            free(cline as *mut libc::c_void);
                            free(chpos as *mut libc::c_void);
                            if !plinepos.is_null() {
                                *plinepos = linepos;
                            }
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
            free(cline as *mut libc::c_void);
            free(chpos as *mut libc::c_void);
        }
    };
}
unsafe extern "C" fn hist_pattern(mut search_type: libc::c_int) -> libc::c_int {
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    set_mlist(ml_search, 0 as libc::c_int);
    pattern = cmd_lastpattern();
    if pattern.is_null() {
        return 0 as libc::c_int;
    }
    if set_pattern(&mut search_info, pattern, search_type, 1 as libc::c_int)
        < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if hilite_search == 2 as libc::c_int && hide_hilite == 0 {
        hilite_screen();
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn chg_caseless() {
    if search_info.is_ucase_pattern == 0 {
        is_caseless = caseless;
        if 1 as libc::c_int == 0 {
            return;
        }
    }
    clear_pattern(&mut search_info);
    hist_pattern(search_info.search_type);
}
pub unsafe extern "C" fn search(
    mut search_type: libc::c_int,
    mut pattern: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut pos: POSITION = 0;
    let mut opos: POSITION = 0;
    let mut lastlinepos: POSITION = -(1 as libc::c_int) as POSITION;
    if pattern.is_null() || *pattern as libc::c_int == '\0' as i32 {
        search_type |= (1 as libc::c_int) << 14 as libc::c_int;
        if prev_pattern(&mut search_info) == 0 {
            let mut r: libc::c_int = hist_pattern(search_type);
            if r == 0 as libc::c_int {
                error(
                    b"No previous regular expression\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            }
            if r <= 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        if search_type & (1 as libc::c_int) << 12 as libc::c_int
            != search_info.search_type & (1 as libc::c_int) << 12 as libc::c_int
        {
            error(
                b"Please re-enter search pattern\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
            return -(1 as libc::c_int);
        }
        if hilite_search == 1 as libc::c_int || status_col != 0 {
            repaint_hilite(0 as libc::c_int);
        }
        if hilite_search == 2 as libc::c_int && hide_hilite != 0 {
            hide_hilite = 0 as libc::c_int;
            hilite_screen();
        }
        hide_hilite = 0 as libc::c_int;
    } else {
        let mut show_error: libc::c_int = (search_type
            & (1 as libc::c_int) << 3 as libc::c_int == 0) as libc::c_int;
        if set_pattern(&mut search_info, pattern, search_type, show_error)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if hilite_search != 0 || status_col != 0 {
            repaint_hilite(0 as libc::c_int);
            hide_hilite = 0 as libc::c_int;
            clr_hilite();
        }
        if hilite_search == 2 as libc::c_int || status_col != 0 {
            hilite_screen();
        }
    }
    pos = search_pos(search_type);
    opos = position(sindex_from_sline(jump_sline));
    if pos == -(1 as libc::c_int) as POSITION {
        if search_type & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            return n;
        }
        if hilite_search == 1 as libc::c_int || status_col != 0 {
            repaint_hilite(1 as libc::c_int);
        }
        error(
            b"Nothing to search\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return -(1 as libc::c_int);
    }
    n = search_range(
        pos,
        -(1 as libc::c_int) as POSITION,
        search_type,
        n,
        -(1 as libc::c_int),
        &mut pos,
        0 as *mut libc::c_void as *mut POSITION,
        &mut lastlinepos,
    );
    if n != 0 as libc::c_int {
        if (hilite_search == 1 as libc::c_int || status_col != 0) && n > 0 as libc::c_int
        {
            repaint_hilite(1 as libc::c_int);
        }
        return n;
    }
    if search_type & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        if lastlinepos != -(1 as libc::c_int) as POSITION {
            jump_loc(lastlinepos, -(1 as libc::c_int));
        } else if pos != opos {
            jump_loc(pos, jump_sline);
        }
    }
    if hilite_search == 1 as libc::c_int || status_col != 0 {
        repaint_hilite(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn prep_hilite(
    mut spos: POSITION,
    mut epos: POSITION,
    mut maxlines: libc::c_int,
) {
    let mut nprep_startpos: POSITION = prep_startpos;
    let mut nprep_endpos: POSITION = prep_endpos;
    let mut new_epos: POSITION = 0;
    let mut max_epos: POSITION = 0;
    let mut result: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if prev_pattern(&mut search_info) == 0 && is_filtering() == 0 {
        return;
    }
    spos = back_raw_line(
        spos + 1 as libc::c_int as libc::c_long,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    if maxlines < 0 as libc::c_int {
        max_epos = -(1 as libc::c_int) as POSITION;
    } else {
        max_epos = spos;
        i = 0 as libc::c_int;
        while i < maxlines {
            max_epos = forw_raw_line(
                max_epos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            i += 1;
            i;
        }
    }
    if prep_startpos == -(1 as libc::c_int) as POSITION
        || epos != -(1 as libc::c_int) as POSITION && epos < prep_startpos
        || spos > prep_endpos
    {
        clr_hilite();
        clr_filter();
        if epos != -(1 as libc::c_int) as POSITION {
            epos += (3 as libc::c_int * size_linebuf) as libc::c_long;
        }
        nprep_startpos = spos;
    } else {
        if !(epos == -(1 as libc::c_int) as POSITION) {
            if epos > prep_endpos {
                epos += (3 as libc::c_int * size_linebuf) as libc::c_long;
            } else {
                epos = prep_startpos;
            }
        }
        if spos < prep_startpos {
            if spos < (3 as libc::c_int * size_linebuf) as libc::c_long {
                spos = 0 as libc::c_int as POSITION;
            } else {
                spos -= (3 as libc::c_int * size_linebuf) as libc::c_long;
            }
            nprep_startpos = spos;
        } else {
            spos = prep_endpos;
        }
    }
    if epos != -(1 as libc::c_int) as POSITION
        && max_epos != -(1 as libc::c_int) as POSITION && epos > max_epos
    {
        epos = max_epos;
    }
    if epos == -(1 as libc::c_int) as POSITION || epos > spos {
        let mut search_type: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int;
        search_type
            |= search_info.search_type
                & ((1 as libc::c_int) << 12 as libc::c_int
                    | ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int
                        | (1 as libc::c_int) << 16 as libc::c_int + 2 as libc::c_int
                        | (1 as libc::c_int) << 16 as libc::c_int + 3 as libc::c_int
                        | (1 as libc::c_int) << 16 as libc::c_int + 4 as libc::c_int
                        | (1 as libc::c_int) << 16 as libc::c_int + 5 as libc::c_int));
        loop {
            result = search_range(
                spos,
                epos,
                search_type,
                0 as libc::c_int,
                maxlines,
                0 as *mut libc::c_void as *mut POSITION,
                &mut new_epos,
                0 as *mut libc::c_void as *mut POSITION,
            );
            if result < 0 as libc::c_int {
                return;
            }
            if prep_endpos == -(1 as libc::c_int) as POSITION || new_epos > prep_endpos {
                nprep_endpos = new_epos;
            }
            if prep_endpos == -(1 as libc::c_int) as POSITION
                || nprep_endpos > prep_endpos
            {
                if new_epos >= nprep_endpos
                    && is_filtered(new_epos - 1 as libc::c_int as libc::c_long) != 0
                {
                    spos = nprep_endpos;
                    epos = forw_raw_line(
                        nprep_endpos,
                        0 as *mut libc::c_void as *mut *mut libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                    if epos == -(1 as libc::c_int) as POSITION {
                        break;
                    }
                    maxlines = 1 as libc::c_int;
                    continue;
                }
            }
            if !(prep_startpos == -(1 as libc::c_int) as POSITION
                || nprep_startpos < prep_startpos)
            {
                break;
            }
            if !(nprep_startpos > 0 as libc::c_int as libc::c_long
                && is_filtered(nprep_startpos) != 0)
            {
                break;
            }
            epos = nprep_startpos;
            spos = back_raw_line(
                nprep_startpos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if spos == -(1 as libc::c_int) as POSITION {
                break;
            }
            nprep_startpos = spos;
            maxlines = 1 as libc::c_int;
        }
    }
    prep_startpos = nprep_startpos;
    prep_endpos = nprep_endpos;
}
pub unsafe extern "C" fn set_filter_pattern(
    mut pattern: *mut libc::c_char,
    mut search_type: libc::c_int,
) {
    let mut filter: *mut pattern_info = 0 as *mut pattern_info;
    clr_filter();
    if pattern.is_null() || *pattern as libc::c_int == '\0' as i32 {
        filter = filter_infos;
        while !filter.is_null() {
            let mut next_filter: *mut pattern_info = (*filter).next;
            clear_pattern(filter);
            free(filter as *mut libc::c_void);
            filter = next_filter;
        }
        filter_infos = 0 as *mut pattern_info;
    } else {
        filter = ecalloc(
            1 as libc::c_int,
            ::std::mem::size_of::<pattern_info>() as libc::c_ulong as libc::c_uint,
        ) as *mut pattern_info;
        init_pattern(filter);
        if set_pattern(filter, pattern, search_type, 1 as libc::c_int) < 0 as libc::c_int
        {
            free(filter as *mut libc::c_void);
            return;
        }
        (*filter).next = filter_infos;
        filter_infos = filter;
    }
    screen_trashed = 1 as libc::c_int;
}
pub unsafe extern "C" fn is_filtering() -> libc::c_int {
    if ch_getflags() & 0o10 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    return (filter_infos != 0 as *mut libc::c_void as *mut pattern_info) as libc::c_int;
}

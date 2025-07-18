use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    static mut inhelp: bool;
    static mut focusing: bool;
    static mut flags: [libc::c_uint; 4];
    static mut editwinrows: libc::c_int;
    static mut editwincols: libc::c_int;
    static mut openfile: *mut openfilestruct;
    static mut tabsize: ssize_t;
    static mut perturbed: bool;
    static mut recook: bool;
    static mut refresh_needed: bool;
    fn is_word_char(c: *const libc::c_char, allow_punct: bool) -> bool;
    fn is_zerowidth(ch: *const libc::c_char) -> bool;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn white_string(string: *const libc::c_char) -> bool;
    fn xplustabs() -> size_t;
    fn adjust_viewport(manner: update_type);
    fn actual_last_column(leftedge: size_t, column: size_t) -> size_t;
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn wideness(text: *const libc::c_char, maxlen: size_t) -> size_t;
    fn leftedge_for(column: size_t, line: *mut linestruct) -> size_t;
    fn go_back_chunks(
        nrows: libc::c_int,
        line: *mut *mut linestruct,
        leftedge: *mut size_t,
    ) -> libc::c_int;
    fn go_forward_chunks(
        nrows: libc::c_int,
        line: *mut *mut linestruct,
        leftedge: *mut size_t,
    ) -> libc::c_int;
    fn begpar(line: *const linestruct, depth: libc::c_int) -> bool;
    fn inpar(line: *const linestruct) -> bool;
    fn edit_redraw(old_current: *mut linestruct, manner: update_type);
    fn update_line(line: *mut linestruct, index: size_t) -> libc::c_int;
    fn line_needs_update(old_column: size_t, new_column: size_t) -> bool;
    fn indent_length(line: *const libc::c_char) -> size_t;
    fn get_softwrap_breakpoint(
        linedata: *const libc::c_char,
        leftedge: size_t,
        kickoff: *mut bool,
        end_of_line: *mut bool,
    ) -> size_t;
    fn edit_scroll(direction: bool);
    fn chunk_for(column: size_t, line: *mut linestruct) -> size_t;
    fn extra_chunks_in(line: *mut linestruct) -> size_t;
    fn full_refresh();
    fn draw_all_subwindows();
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type format_type = libc::c_uint;
pub const MAC_FILE: format_type = 3;
pub const DOS_FILE: format_type = 2;
pub const NIX_FILE: format_type = 1;
pub const UNSPECIFIED: format_type = 0;
pub type update_type = libc::c_uint;
pub const STATIONARY: update_type = 2;
pub const FLOWING: update_type = 1;
pub const CENTERING: update_type = 0;
pub type undo_type = libc::c_uint;
pub const OTHER: undo_type = 21;
pub const COUPLE_END: undo_type = 20;
pub const COUPLE_BEGIN: undo_type = 19;
pub const INSERT: undo_type = 18;
pub const PASTE: undo_type = 17;
pub const COPY: undo_type = 16;
pub const CUT_TO_EOF: undo_type = 15;
pub const CUT: undo_type = 14;
pub const ZAP: undo_type = 13;
pub const PREFLIGHT: undo_type = 12;
pub const UNCOMMENT: undo_type = 11;
pub const COMMENT: undo_type = 10;
pub const UNINDENT: undo_type = 9;
pub const INDENT: undo_type = 8;
pub const SPLIT_END: undo_type = 7;
pub const SPLIT_BEGIN: undo_type = 6;
pub const REPLACE: undo_type = 5;
pub const JOIN: undo_type = 4;
pub const DEL: undo_type = 3;
pub const BACK: undo_type = 2;
pub const ENTER: undo_type = 1;
pub const ADD: undo_type = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const ZERO: C2RustUnnamed = 48;
pub const MINIBAR: C2RustUnnamed = 47;
pub const USE_MAGIC: C2RustUnnamed = 46;
pub const STATEFLAGS: C2RustUnnamed = 45;
pub const BOOKSTYLE: C2RustUnnamed = 44;
pub const INDICATOR: C2RustUnnamed = 43;
pub const EMPTY_LINE: C2RustUnnamed = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed = 40;
pub const LET_THEM_ZAP: C2RustUnnamed = 39;
pub const AFTER_ENDS: C2RustUnnamed = 38;
pub const AT_BLANKS: C2RustUnnamed = 37;
pub const LINE_NUMBERS: C2RustUnnamed = 36;
pub const SHOW_CURSOR: C2RustUnnamed = 35;
pub const TRIM_BLANKS: C2RustUnnamed = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed = 33;
pub const NOREAD_MODE: C2RustUnnamed = 32;
pub const LOCKING: C2RustUnnamed = 31;
pub const POSITIONLOG: C2RustUnnamed = 30;
pub const SOFTWRAP: C2RustUnnamed = 29;
pub const BOLD_TEXT: C2RustUnnamed = 28;
pub const NO_NEWLINES: C2RustUnnamed = 27;
pub const WORD_BOUNDS: C2RustUnnamed = 26;
pub const QUICK_BLANK: C2RustUnnamed = 25;
pub const TABS_TO_SPACES: C2RustUnnamed = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed = 23;
pub const SMART_HOME: C2RustUnnamed = 22;
pub const RESTRICTED: C2RustUnnamed = 21;
pub const HISTORYLOG: C2RustUnnamed = 20;
pub const PRESERVE: C2RustUnnamed = 19;
pub const NO_SYNTAX: C2RustUnnamed = 18;
pub const INSECURE_BACKUP: C2RustUnnamed = 17;
pub const MAKE_BACKUP: C2RustUnnamed = 16;
pub const NO_CONVERT: C2RustUnnamed = 15;
pub const RAW_SEQUENCES: C2RustUnnamed = 14;
pub const REBIND_DELETE: C2RustUnnamed = 13;
pub const MULTIBUFFER: C2RustUnnamed = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed = 9;
pub const USE_REGEXP: C2RustUnnamed = 8;
pub const USE_MOUSE: C2RustUnnamed = 7;
pub const VIEW_MODE: C2RustUnnamed = 6;
pub const AUTOINDENT: C2RustUnnamed = 5;
pub const NO_WRAP: C2RustUnnamed = 4;
pub const NO_HELP: C2RustUnnamed = 3;
pub const CONSTANT_SHOW: C2RustUnnamed = 2;
pub const CASE_SENSITIVE: C2RustUnnamed = 1;
pub const DONTUSE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colortype {
    pub id: libc::c_short,
    pub fg: libc::c_short,
    pub bg: libc::c_short,
    pub pairnum: libc::c_short,
    pub attributes: libc::c_int,
    pub start: *mut regex_t,
    pub end: *mut regex_t,
    pub next: *mut colortype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regexlisttype {
    pub one_rgx: *mut regex_t,
    pub next: *mut regexlisttype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct augmentstruct {
    pub filename: *mut libc::c_char,
    pub lineno: ssize_t,
    pub data: *mut libc::c_char,
    pub next: *mut augmentstruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syntaxtype {
    pub name: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub lineno: size_t,
    pub augmentations: *mut augmentstruct,
    pub extensions: *mut regexlisttype,
    pub headers: *mut regexlisttype,
    pub magics: *mut regexlisttype,
    pub linter: *mut libc::c_char,
    pub formatter: *mut libc::c_char,
    pub tab: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub color: *mut colortype,
    pub nmultis: libc::c_short,
    pub next: *mut syntaxtype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linestruct {
    pub data: *mut libc::c_char,
    pub lineno: ssize_t,
    pub next: *mut linestruct,
    pub prev: *mut linestruct,
    pub multidata: *mut libc::c_short,
    pub has_anchor: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct groupstruct {
    pub top_line: ssize_t,
    pub bottom_line: ssize_t,
    pub indentations: *mut *mut libc::c_char,
    pub next: *mut groupstruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undostruct {
    pub type_0: undo_type,
    pub xflags: libc::c_int,
    pub head_lineno: ssize_t,
    pub head_x: size_t,
    pub strdata: *mut libc::c_char,
    pub wassize: size_t,
    pub newsize: size_t,
    pub grouping: *mut groupstruct,
    pub cutbuffer: *mut linestruct,
    pub tail_lineno: ssize_t,
    pub tail_x: size_t,
    pub next: *mut undostruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct openfilestruct {
    pub filename: *mut libc::c_char,
    pub filetop: *mut linestruct,
    pub filebot: *mut linestruct,
    pub edittop: *mut linestruct,
    pub current: *mut linestruct,
    pub totsize: size_t,
    pub firstcolumn: size_t,
    pub current_x: size_t,
    pub placewewant: size_t,
    pub current_y: ssize_t,
    pub statinfo: *mut stat,
    pub spillage_line: *mut linestruct,
    pub mark: *mut linestruct,
    pub mark_x: size_t,
    pub softmark: bool,
    pub fmt: format_type,
    pub lock_filename: *mut libc::c_char,
    pub undotop: *mut undostruct,
    pub current_undo: *mut undostruct,
    pub last_saved: *mut undostruct,
    pub last_action: undo_type,
    pub modified: bool,
    pub syntax: *mut syntaxtype,
    pub errormessage: *mut libc::c_char,
    pub next: *mut openfilestruct,
    pub prev: *mut openfilestruct,
}
pub unsafe extern "C" fn to_first_line() {
    (*openfile).current = (*openfile).filetop;
    (*openfile).current_x = 0 as libc::c_int as size_t;
    (*openfile).placewewant = 0 as libc::c_int as size_t;
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn to_last_line() {
    (*openfile).current = (*openfile).filebot;
    (*openfile)
        .current_x = if inhelp as libc::c_int != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        strlen((*(*openfile).filebot).data)
    };
    (*openfile).placewewant = xplustabs();
    (*openfile).current_y = (editwinrows - 1 as libc::c_int) as ssize_t;
    refresh_needed = 1 as libc::c_int != 0;
    recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
    focusing = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn get_edge_and_target(
    mut leftedge: *mut size_t,
    mut target_column: *mut size_t,
) {
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut shim: size_t = (editwincols as libc::c_long
            * (1 as libc::c_int as libc::c_long + tabsize / editwincols as libc::c_long))
            as size_t;
        *leftedge = leftedge_for(xplustabs(), (*openfile).current);
        *target_column = ((*openfile).placewewant)
            .wrapping_add(shim)
            .wrapping_sub(*leftedge)
            .wrapping_rem(editwincols as libc::c_ulong);
    } else {
        *leftedge = 0 as libc::c_int as size_t;
        *target_column = (*openfile).placewewant;
    };
}
pub unsafe extern "C" fn proper_x(
    mut line: *mut linestruct,
    mut leftedge: *mut size_t,
    mut forward: bool,
    mut column: size_t,
    mut shifted: *mut bool,
) -> size_t {
    let mut index: size_t = actual_x((*line).data, column);
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && *((*line).data).offset(index as isize) as libc::c_int == '\t' as i32
        && (forward as libc::c_int != 0 && wideness((*line).data, index) < *leftedge
            || !forward
                && column.wrapping_div(tabsize as libc::c_ulong)
                    == (*leftedge)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(tabsize as libc::c_ulong)
                && column.wrapping_div(tabsize as libc::c_ulong)
                    < (*leftedge)
                        .wrapping_add(editwincols as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(tabsize as libc::c_ulong))
    {
        index = index.wrapping_add(1);
        index;
        if !shifted.is_null() {
            *shifted = 1 as libc::c_int != 0;
        }
    }
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        *leftedge = leftedge_for(wideness((*line).data, index), line);
    }
    return index;
}
pub unsafe extern "C" fn set_proper_index_and_pww(
    mut leftedge: *mut size_t,
    mut target: size_t,
    mut forward: bool,
) {
    let mut was_edge: size_t = *leftedge;
    let mut shifted: bool = 0 as libc::c_int != 0;
    (*openfile)
        .current_x = proper_x(
        (*openfile).current,
        leftedge,
        forward,
        actual_last_column(*leftedge, target),
        &mut shifted,
    );
    if shifted as libc::c_int != 0 || *leftedge < was_edge {
        (*openfile)
            .current_x = proper_x(
            (*openfile).current,
            leftedge,
            forward,
            actual_last_column(*leftedge, target),
            &mut shifted,
        );
    }
    (*openfile).placewewant = (*leftedge).wrapping_add(target);
}
pub unsafe extern "C" fn do_page_up() {
    let mut mustmove: libc::c_int = if editwinrows < 3 as libc::c_int {
        1 as libc::c_int
    } else {
        editwinrows - 2 as libc::c_int
    };
    let mut leftedge: size_t = 0;
    let mut target_column: size_t = 0;
    if flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*openfile).current = (*openfile).edittop;
        leftedge = (*openfile).firstcolumn;
        (*openfile).current_y = 0 as libc::c_int as ssize_t;
        target_column = 0 as libc::c_int as size_t;
    } else {
        get_edge_and_target(&mut leftedge, &mut target_column);
    }
    if go_back_chunks(mustmove, &mut (*openfile).current, &mut leftedge)
        > 0 as libc::c_int
    {
        to_first_line();
        return;
    }
    set_proper_index_and_pww(&mut leftedge, target_column, 0 as libc::c_int != 0);
    adjust_viewport(STATIONARY);
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_page_down() {
    let mut mustmove: libc::c_int = if editwinrows < 3 as libc::c_int {
        1 as libc::c_int
    } else {
        editwinrows - 2 as libc::c_int
    };
    let mut leftedge: size_t = 0;
    let mut target_column: size_t = 0;
    if flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*openfile).current = (*openfile).edittop;
        leftedge = (*openfile).firstcolumn;
        (*openfile).current_y = 0 as libc::c_int as ssize_t;
        target_column = 0 as libc::c_int as size_t;
    } else {
        get_edge_and_target(&mut leftedge, &mut target_column);
    }
    if go_forward_chunks(mustmove, &mut (*openfile).current, &mut leftedge)
        > 0 as libc::c_int
    {
        to_last_line();
        return;
    }
    set_proper_index_and_pww(&mut leftedge, target_column, 1 as libc::c_int != 0);
    adjust_viewport(STATIONARY);
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_para_begin(mut line: *mut *mut linestruct) {
    if !((**line).prev).is_null() {
        *line = (**line).prev;
    }
    while !begpar(*line, 0 as libc::c_int) {
        *line = (**line).prev;
    }
}
pub unsafe extern "C" fn do_para_end(mut line: *mut *mut linestruct) {
    while !((**line).next).is_null() && !inpar(*line) {
        *line = (**line).next;
    }
    while !((**line).next).is_null() && inpar((**line).next) as libc::c_int != 0
        && !begpar((**line).next, 0 as libc::c_int)
    {
        *line = (**line).next;
    }
}
pub unsafe extern "C" fn to_para_begin() {
    let mut was_current: *mut linestruct = (*openfile).current;
    do_para_begin(&mut (*openfile).current);
    (*openfile).current_x = 0 as libc::c_int as size_t;
    edit_redraw(was_current, CENTERING);
}
pub unsafe extern "C" fn to_para_end() {
    let mut was_current: *mut linestruct = (*openfile).current;
    do_para_end(&mut (*openfile).current);
    if !((*(*openfile).current).next).is_null() {
        (*openfile).current = (*(*openfile).current).next;
        (*openfile).current_x = 0 as libc::c_int as size_t;
    } else {
        (*openfile).current_x = strlen((*(*openfile).current).data);
    }
    edit_redraw(was_current, CENTERING);
    recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
}
pub unsafe extern "C" fn to_prev_block() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut is_text: bool = 0 as libc::c_int != 0;
    let mut seen_text: bool = 0 as libc::c_int != 0;
    while !((*(*openfile).current).prev).is_null()
        && (!seen_text || is_text as libc::c_int != 0)
    {
        (*openfile).current = (*(*openfile).current).prev;
        is_text = !white_string((*(*openfile).current).data);
        seen_text = seen_text as libc::c_int != 0 || is_text as libc::c_int != 0;
    }
    if seen_text as libc::c_int != 0 && !((*(*openfile).current).next).is_null()
        && white_string((*(*openfile).current).data) as libc::c_int != 0
    {
        (*openfile).current = (*(*openfile).current).next;
    }
    (*openfile).current_x = 0 as libc::c_int as size_t;
    edit_redraw(was_current, CENTERING);
}
pub unsafe extern "C" fn to_next_block() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut is_white: bool = white_string((*(*openfile).current).data);
    let mut seen_white: bool = is_white;
    while !((*(*openfile).current).next).is_null()
        && (!seen_white || is_white as libc::c_int != 0)
    {
        (*openfile).current = (*(*openfile).current).next;
        is_white = white_string((*(*openfile).current).data);
        seen_white = seen_white as libc::c_int != 0 || is_white as libc::c_int != 0;
    }
    (*openfile).current_x = 0 as libc::c_int as size_t;
    edit_redraw(was_current, CENTERING);
    recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
}
pub unsafe extern "C" fn do_prev_word() {
    let mut punctuation_as_letters: bool = flags[(WORD_BOUNDS as libc::c_int
        as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (WORD_BOUNDS as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    let mut seen_a_word: bool = 0 as libc::c_int != 0;
    let mut step_forward: bool = 0 as libc::c_int != 0;
    loop {
        if (*openfile).current_x == 0 as libc::c_int as libc::c_ulong {
            if ((*(*openfile).current).prev).is_null() {
                break;
            }
            (*openfile).current = (*(*openfile).current).prev;
            (*openfile).current_x = strlen((*(*openfile).current).data);
        }
        (*openfile)
            .current_x = step_left((*(*openfile).current).data, (*openfile).current_x);
        if is_word_char(
            ((*(*openfile).current).data).offset((*openfile).current_x as isize),
            punctuation_as_letters,
        ) {
            seen_a_word = 1 as libc::c_int != 0;
            if (*openfile).current_x == 0 as libc::c_int as libc::c_ulong {
                break;
            }
        } else if !is_zerowidth(
            ((*(*openfile).current).data).offset((*openfile).current_x as isize),
        ) {
            if !seen_a_word {
                continue;
            }
            step_forward = 1 as libc::c_int != 0;
            break;
        }
    }
    if step_forward {
        (*openfile)
            .current_x = step_right((*(*openfile).current).data, (*openfile).current_x);
    }
}
pub unsafe extern "C" fn do_next_word(mut after_ends: bool) -> bool {
    let mut punctuation_as_letters: bool = flags[(WORD_BOUNDS as libc::c_int
        as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (WORD_BOUNDS as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    let mut started_on_word: bool = is_word_char(
        ((*(*openfile).current).data).offset((*openfile).current_x as isize),
        punctuation_as_letters,
    );
    let mut seen_space: bool = !started_on_word;
    let mut seen_word: bool = started_on_word;
    loop {
        if *((*(*openfile).current).data).offset((*openfile).current_x as isize)
            as libc::c_int == '\0' as i32
        {
            if ((*(*openfile).current).next).is_null() {
                break;
            }
            (*openfile).current = (*(*openfile).current).next;
            (*openfile).current_x = 0 as libc::c_int as size_t;
            seen_space = 1 as libc::c_int != 0;
        } else {
            (*openfile)
                .current_x = step_right(
                (*(*openfile).current).data,
                (*openfile).current_x,
            );
        }
        if after_ends {
            if is_word_char(
                ((*(*openfile).current).data).offset((*openfile).current_x as isize),
                punctuation_as_letters,
            ) {
                seen_word = 1 as libc::c_int != 0;
            } else if !is_zerowidth(
                ((*(*openfile).current).data).offset((*openfile).current_x as isize),
            ) {
                if seen_word {
                    break;
                }
            }
        } else if !is_zerowidth(
            ((*(*openfile).current).data).offset((*openfile).current_x as isize),
        ) {
            if !is_word_char(
                ((*(*openfile).current).data).offset((*openfile).current_x as isize),
                punctuation_as_letters,
            ) {
                seen_space = 1 as libc::c_int != 0;
            } else if seen_space {
                break;
            }
        }
    }
    return started_on_word;
}
pub unsafe extern "C" fn to_prev_word() {
    let mut was_current: *mut linestruct = (*openfile).current;
    do_prev_word();
    edit_redraw(was_current, FLOWING);
}
pub unsafe extern "C" fn to_next_word() {
    let mut was_current: *mut linestruct = (*openfile).current;
    do_next_word(
        flags[(AFTER_ENDS as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (AFTER_ENDS as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint,
    );
    edit_redraw(was_current, FLOWING);
}
pub unsafe extern "C" fn do_home() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_column: size_t = xplustabs();
    let mut moved_off_chunk: bool = 1 as libc::c_int != 0;
    let mut moved: bool = 0 as libc::c_int != 0;
    let mut leftedge: size_t = 0 as libc::c_int as size_t;
    let mut left_x: size_t = 0 as libc::c_int as size_t;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        leftedge = leftedge_for(was_column, (*openfile).current);
        left_x = proper_x(
            (*openfile).current,
            &mut leftedge,
            0 as libc::c_int != 0,
            leftedge,
            0 as *mut bool,
        );
    }
    if flags[(SMART_HOME as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SMART_HOME as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut indent_x: size_t = indent_length((*(*openfile).current).data);
        if *((*(*openfile).current).data).offset(indent_x as isize) as libc::c_int
            != '\0' as i32
        {
            if (*openfile).current_x == indent_x {
                (*openfile).current_x = 0 as libc::c_int as size_t;
                moved = 1 as libc::c_int != 0;
            } else if left_x <= indent_x {
                (*openfile).current_x = indent_x;
                moved = 1 as libc::c_int != 0;
            }
        }
    }
    if !moved
        && flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (SOFTWRAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        if (*openfile).current_x == left_x {
            (*openfile).current_x = 0 as libc::c_int as size_t;
        } else {
            (*openfile).current_x = left_x;
            (*openfile).placewewant = leftedge;
            moved_off_chunk = 0 as libc::c_int != 0;
        }
    } else if !moved {
        (*openfile).current_x = 0 as libc::c_int as size_t;
    }
    if moved_off_chunk {
        (*openfile).placewewant = xplustabs();
    }
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && moved_off_chunk as libc::c_int != 0
    {
        edit_redraw(was_current, FLOWING);
    } else if line_needs_update(was_column, (*openfile).placewewant) {
        update_line((*openfile).current, (*openfile).current_x);
    }
}
pub unsafe extern "C" fn do_end() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_column: size_t = xplustabs();
    let mut line_len: size_t = strlen((*(*openfile).current).data);
    let mut moved_off_chunk: bool = 1 as libc::c_int != 0;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut kickoff: bool = 1 as libc::c_int != 0;
        let mut last_chunk: bool = 0 as libc::c_int != 0;
        let mut leftedge: size_t = leftedge_for(was_column, (*openfile).current);
        let mut rightedge: size_t = get_softwrap_breakpoint(
            (*(*openfile).current).data,
            leftedge,
            &mut kickoff,
            &mut last_chunk,
        );
        let mut right_x: size_t = 0;
        if !last_chunk {
            rightedge = rightedge.wrapping_sub(1);
            rightedge;
        }
        right_x = actual_x((*(*openfile).current).data, rightedge);
        if (*openfile).current_x == right_x {
            (*openfile).current_x = line_len;
        } else {
            (*openfile).current_x = right_x;
            (*openfile).placewewant = rightedge;
            moved_off_chunk = 0 as libc::c_int != 0;
        }
    } else {
        (*openfile).current_x = line_len;
    }
    if moved_off_chunk {
        (*openfile).placewewant = xplustabs();
    }
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && moved_off_chunk as libc::c_int != 0
    {
        edit_redraw(was_current, FLOWING);
    } else if line_needs_update(was_column, (*openfile).placewewant) {
        update_line((*openfile).current, (*openfile).current_x);
    }
}
pub unsafe extern "C" fn do_up() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut leftedge: size_t = 0;
    let mut target_column: size_t = 0;
    get_edge_and_target(&mut leftedge, &mut target_column);
    if go_back_chunks(1 as libc::c_int, &mut (*openfile).current, &mut leftedge)
        > 0 as libc::c_int
    {
        return;
    }
    set_proper_index_and_pww(&mut leftedge, target_column, 0 as libc::c_int != 0);
    if (*openfile).current_y == 0 as libc::c_int as libc::c_long
        && !(flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        edit_scroll(0 as libc::c_int != 0);
    } else {
        edit_redraw(was_current, FLOWING);
    }
    (*openfile).placewewant = leftedge.wrapping_add(target_column);
}
pub unsafe extern "C" fn do_down() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut leftedge: size_t = 0;
    let mut target_column: size_t = 0;
    get_edge_and_target(&mut leftedge, &mut target_column);
    if go_forward_chunks(1 as libc::c_int, &mut (*openfile).current, &mut leftedge)
        > 0 as libc::c_int
    {
        return;
    }
    set_proper_index_and_pww(&mut leftedge, target_column, 1 as libc::c_int != 0);
    if (*openfile).current_y == (editwinrows - 1 as libc::c_int) as libc::c_long
        && !(flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        edit_scroll(1 as libc::c_int != 0);
    } else {
        edit_redraw(was_current, FLOWING);
    }
    (*openfile).placewewant = leftedge.wrapping_add(target_column);
}
pub unsafe extern "C" fn do_scroll_up() {
    if ((*(*openfile).edittop).prev).is_null()
        && (*openfile).firstcolumn == 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    if (*openfile).current_y == (editwinrows - 1 as libc::c_int) as libc::c_long {
        do_up();
    }
    if editwinrows > 1 as libc::c_int {
        edit_scroll(0 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn do_scroll_down() {
    if (*openfile).current_y == 0 as libc::c_int as libc::c_long {
        do_down();
    }
    if editwinrows > 1 as libc::c_int
        && (!((*(*openfile).edittop).next).is_null()
            || flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (SOFTWRAP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
                && extra_chunks_in((*openfile).edittop)
                    > chunk_for((*openfile).firstcolumn, (*openfile).edittop))
    {
        edit_scroll(1 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn do_center() {
    adjust_viewport(CENTERING);
    draw_all_subwindows();
    full_refresh();
}
pub unsafe extern "C" fn do_left() {
    let mut was_current: *mut linestruct = (*openfile).current;
    if (*openfile).current_x > 0 as libc::c_int as libc::c_ulong {
        (*openfile)
            .current_x = step_left((*(*openfile).current).data, (*openfile).current_x);
        while (*openfile).current_x > 0 as libc::c_int as libc::c_ulong
            && is_zerowidth(
                ((*(*openfile).current).data).offset((*openfile).current_x as isize),
            ) as libc::c_int != 0
        {
            (*openfile)
                .current_x = step_left(
                (*(*openfile).current).data,
                (*openfile).current_x,
            );
        }
    } else if (*openfile).current != (*openfile).filetop {
        (*openfile).current = (*(*openfile).current).prev;
        (*openfile).current_x = strlen((*(*openfile).current).data);
    }
    edit_redraw(was_current, FLOWING);
}
pub unsafe extern "C" fn do_right() {
    let mut was_current: *mut linestruct = (*openfile).current;
    if *((*(*openfile).current).data).offset((*openfile).current_x as isize)
        as libc::c_int != '\0' as i32
    {
        (*openfile)
            .current_x = step_right((*(*openfile).current).data, (*openfile).current_x);
        while *((*(*openfile).current).data).offset((*openfile).current_x as isize)
            as libc::c_int != '\0' as i32
            && is_zerowidth(
                ((*(*openfile).current).data).offset((*openfile).current_x as isize),
            ) as libc::c_int != 0
        {
            (*openfile)
                .current_x = step_right(
                (*(*openfile).current).data,
                (*openfile).current_x,
            );
        }
    } else if (*openfile).current != (*openfile).filebot {
        (*openfile).current = (*(*openfile).current).next;
        (*openfile).current_x = 0 as libc::c_int as size_t;
    }
    edit_redraw(was_current, FLOWING);
}

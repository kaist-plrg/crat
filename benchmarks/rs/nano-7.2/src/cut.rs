use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut ran_a_tool: bool;
    static mut focusing: bool;
    static mut flags: [libc::c_uint; 4];
    static mut editwinrows: libc::c_int;
    static mut cutbuffer: *mut linestruct;
    static mut cutbottom: *mut linestruct;
    static mut keep_cutbuffer: bool;
    static mut openfile: *mut openfilestruct;
    static mut perturbed: bool;
    static mut recook: bool;
    static mut refresh_needed: bool;
    fn is_zerowidth(ch: *const libc::c_char) -> bool;
    fn char_length(pointer: *const libc::c_char) -> libc::c_int;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn check_the_multis(line: *mut linestruct);
    fn precalc_multicolorinfo();
    fn set_modified();
    fn update_line(line: *mut linestruct, index: size_t) -> libc::c_int;
    fn renumber_from(line: *mut linestruct);
    fn unlink_node(line: *mut linestruct);
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn add_undo(action: undo_type, message: *const libc::c_char);
    fn extra_chunks_in(line: *mut linestruct) -> size_t;
    fn update_undo(action: undo_type);
    fn xplustabs() -> size_t;
    fn wipe_statusbar();
    fn new_magicline();
    fn adjust_viewport(manner: update_type);
    fn delete_node(line: *mut linestruct);
    fn number_of_characters_in(
        begin: *const linestruct,
        end: *const linestruct,
    ) -> size_t;
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn measured_copy(string: *const libc::c_char, count: size_t) -> *mut libc::c_char;
    fn get_region(
        top: *mut *mut linestruct,
        top_x: *mut size_t,
        bot: *mut *mut linestruct,
        bot_x: *mut size_t,
    );
    fn free_lines(src: *mut linestruct);
    fn statusbar(msg: *const libc::c_char);
    fn do_left();
    fn do_next_word(after_ends: bool) -> bool;
    fn do_prev_word();
    fn mark_is_before_cursor() -> bool;
    fn copy_buffer(src: *const linestruct) -> *mut linestruct;
    fn edit_redraw(old_current: *mut linestruct, manner: update_type);
    fn less_than_a_screenful(was_lineno: size_t, was_leftedge: size_t) -> bool;
    fn do_wrap();
    fn leftedge_for(column: size_t, line: *mut linestruct) -> size_t;
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub type message_type = libc::c_uint;
pub const ALERT: message_type = 7;
pub const MILD: message_type = 6;
pub const AHEM: message_type = 5;
pub const NOTICE: message_type = 4;
pub const INFO: message_type = 3;
pub const REMARK: message_type = 2;
pub const HUSH: message_type = 1;
pub const VACUUM: message_type = 0;
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
pub unsafe extern "C" fn do_deletion(mut action: undo_type) {
    (*openfile).placewewant = xplustabs();
    if *((*(*openfile).current).data).offset((*openfile).current_x as isize)
        as libc::c_int != '\0' as i32
    {
        let mut charlen: libc::c_int = char_length(
            ((*(*openfile).current).data).offset((*openfile).current_x as isize),
        );
        let mut line_len: size_t = strlen(
            ((*(*openfile).current).data).offset((*openfile).current_x as isize),
        );
        let mut old_amount: size_t = if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
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
            extra_chunks_in((*openfile).current)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if action as libc::c_uint != (*openfile).last_action as libc::c_uint
            || (*(*openfile).current).lineno != (*(*openfile).current_undo).head_lineno
        {
            add_undo(action, 0 as *const libc::c_char);
        } else {
            update_undo(action);
        }
        memmove(
            &mut *((*(*openfile).current).data).offset((*openfile).current_x as isize)
                as *mut libc::c_char as *mut libc::c_void,
            &mut *((*(*openfile).current).data)
                .offset(
                    ((*openfile).current_x).wrapping_add(charlen as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_char as *const libc::c_void,
            line_len
                .wrapping_sub(charlen as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
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
            && extra_chunks_in((*openfile).current) != old_amount
        {
            refresh_needed = 1 as libc::c_int != 0;
        }
        if (*openfile).mark == (*openfile).current
            && (*openfile).mark_x > (*openfile).current_x
        {
            (*openfile)
                .mark_x = ((*openfile).mark_x as libc::c_ulong)
                .wrapping_sub(charlen as libc::c_ulong) as size_t as size_t;
        }
    } else if (*openfile).current != (*openfile).filebot {
        let mut joining: *mut linestruct = (*(*openfile).current).next;
        if joining == (*openfile).filebot
            && (*openfile).current_x != 0 as libc::c_int as libc::c_ulong
            && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
        {
            if action as libc::c_uint == BACK as libc::c_int as libc::c_uint {
                add_undo(BACK, 0 as *const libc::c_char);
            }
            return;
        }
        add_undo(action, 0 as *const libc::c_char);
        if (*openfile).mark == joining {
            (*openfile).mark = (*openfile).current;
            (*openfile)
                .mark_x = ((*openfile).mark_x as libc::c_ulong)
                .wrapping_add((*openfile).current_x) as size_t as size_t;
        }
        (*(*openfile).current)
            .has_anchor = ((*(*openfile).current).has_anchor as libc::c_int
            | (*joining).has_anchor as libc::c_int) != 0;
        (*(*openfile).current)
            .data = nrealloc(
            (*(*openfile).current).data as *mut libc::c_void,
            (strlen((*(*openfile).current).data))
                .wrapping_add(strlen((*joining).data))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcat((*(*openfile).current).data, (*joining).data);
        unlink_node(joining);
        renumber_from((*openfile).current);
        refresh_needed = 1 as libc::c_int != 0;
    } else {
        return
    }
    if !refresh_needed {
        check_the_multis((*openfile).current);
    }
    if !refresh_needed {
        update_line((*openfile).current, (*openfile).current_x);
    }
    (*openfile).totsize = ((*openfile).totsize).wrapping_sub(1);
    (*openfile).totsize;
    (*(*openfile).current_undo).newsize = (*openfile).totsize;
    set_modified();
}
pub unsafe extern "C" fn do_delete() {
    if !((*openfile).mark).is_null()
        && flags[(LET_THEM_ZAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (LET_THEM_ZAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        zap_text();
    } else {
        do_deletion(DEL);
        while *((*(*openfile).current).data).offset((*openfile).current_x as isize)
            as libc::c_int != '\0' as i32
            && is_zerowidth(
                ((*(*openfile).current).data).offset((*openfile).current_x as isize),
            ) as libc::c_int != 0
        {
            do_deletion(DEL);
        }
    };
}
pub unsafe extern "C" fn do_backspace() {
    if !((*openfile).mark).is_null()
        && flags[(LET_THEM_ZAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (LET_THEM_ZAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        zap_text();
    } else if (*openfile).current_x > 0 as libc::c_int as libc::c_ulong {
        (*openfile)
            .current_x = step_left((*(*openfile).current).data, (*openfile).current_x);
        do_deletion(BACK);
    } else if (*openfile).current != (*openfile).filetop {
        do_left();
        do_deletion(BACK);
    }
}
pub unsafe extern "C" fn is_cuttable(mut test_cliff: bool) -> bool {
    let mut from: size_t = if test_cliff as libc::c_int != 0 {
        (*openfile).current_x
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if ((*(*openfile).current).next).is_null()
        && *((*(*openfile).current).data).offset(from as isize) as libc::c_int
            == '\0' as i32 && ((*openfile).mark).is_null()
        || (*openfile).mark == (*openfile).current
            && (*openfile).mark_x == (*openfile).current_x
        || from > 0 as libc::c_int as libc::c_ulong
            && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
            && *((*(*openfile).current).data).offset(from as isize) as libc::c_int
                == '\0' as i32 && (*(*openfile).current).next == (*openfile).filebot
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Nothing was cut\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        (*openfile).mark = 0 as *mut linestruct;
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn chop_word(mut forward: bool) {
    let mut is_current: *mut linestruct = (*openfile).current;
    let mut is_current_x: size_t = (*openfile).current_x;
    let mut is_cutbuffer: *mut linestruct = cutbuffer;
    cutbuffer = 0 as *mut linestruct;
    if !forward {
        do_prev_word();
        if (*openfile).current != is_current {
            if is_current_x > 0 as libc::c_int as libc::c_ulong {
                (*openfile).current = is_current;
                (*openfile).current_x = 0 as libc::c_int as size_t;
            } else {
                (*openfile).current_x = strlen((*(*openfile).current).data);
            }
        }
    } else {
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
        if (*openfile).current != is_current
            && *((*is_current).data).offset(is_current_x as isize) as libc::c_int
                != '\0' as i32
        {
            (*openfile).current = is_current;
            (*openfile).current_x = strlen((*is_current).data);
        }
    }
    (*openfile).mark = (*openfile).current;
    (*openfile).mark_x = (*openfile).current_x;
    (*openfile).current = is_current;
    (*openfile).current_x = is_current_x;
    add_undo(CUT, 0 as *const libc::c_char);
    do_snip(1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
    update_undo(CUT);
    free_lines(cutbuffer);
    cutbuffer = is_cutbuffer;
}
pub unsafe extern "C" fn chop_previous_word() {
    if ((*(*openfile).current).prev).is_null()
        && (*openfile).current_x == 0 as libc::c_int as libc::c_ulong
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Nothing was cut\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        chop_word(0 as libc::c_int != 0);
    };
}
pub unsafe extern "C" fn chop_next_word() {
    (*openfile).mark = 0 as *mut linestruct;
    if is_cuttable(1 as libc::c_int != 0) {
        chop_word(1 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn extract_segment(
    mut top: *mut linestruct,
    mut top_x: size_t,
    mut bot: *mut linestruct,
    mut bot_x: size_t,
) {
    let mut taken: *mut linestruct = 0 as *mut linestruct;
    let mut last: *mut linestruct = 0 as *mut linestruct;
    let mut edittop_inside: bool = (*(*openfile).edittop).lineno >= (*top).lineno
        && (*(*openfile).edittop).lineno <= (*bot).lineno;
    let mut same_line: bool = (*openfile).mark == top;
    let mut post_marked: bool = !((*openfile).mark).is_null()
        && ((*(*openfile).mark).lineno > (*top).lineno
            || same_line as libc::c_int != 0 && (*openfile).mark_x > top_x);
    static mut inherited_anchor: bool = 0 as libc::c_int != 0;
    let mut had_anchor: bool = (*top).has_anchor;
    if top == bot && top_x == bot_x {
        return;
    }
    if top != bot {
        let mut line: *mut linestruct = (*top).next;
        while line != (*bot).next {
            had_anchor = (had_anchor as libc::c_int | (*line).has_anchor as libc::c_int) != 0;
            line = (*line).next;
        }
    }
    if top == bot {
        taken = make_new_node(0 as *mut linestruct);
        (*taken)
            .data = measured_copy(
            ((*top).data).offset(top_x as isize),
            bot_x.wrapping_sub(top_x),
        );
        memmove(
            ((*top).data).offset(top_x as isize) as *mut libc::c_void,
            ((*top).data).offset(bot_x as isize) as *const libc::c_void,
            (strlen(((*top).data).offset(bot_x as isize)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        last = taken;
    } else if top_x == 0 as libc::c_int as libc::c_ulong
        && bot_x == 0 as libc::c_int as libc::c_ulong
    {
        taken = top;
        last = make_new_node(0 as *mut linestruct);
        (*last).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
        (*last).has_anchor = (*bot).has_anchor;
        (*last).prev = (*bot).prev;
        (*(*bot).prev).next = last;
        (*last).next = 0 as *mut linestruct;
        (*bot).prev = (*top).prev;
        if !((*top).prev).is_null() {
            (*(*top).prev).next = bot;
        } else {
            (*openfile).filetop = bot;
        }
        (*openfile).current = bot;
    } else {
        taken = make_new_node(0 as *mut linestruct);
        (*taken).data = copy_of(((*top).data).offset(top_x as isize));
        (*taken).next = (*top).next;
        (*(*top).next).prev = taken;
        (*top).next = (*bot).next;
        if !((*bot).next).is_null() {
            (*(*bot).next).prev = top;
        }
        (*top)
            .data = nrealloc(
            (*top).data as *mut libc::c_void,
            top_x
                .wrapping_add(strlen(((*bot).data).offset(bot_x as isize)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(
            ((*top).data).offset(top_x as isize),
            ((*bot).data).offset(bot_x as isize),
        );
        last = bot;
        *((*last).data).offset(bot_x as isize) = '\0' as i32 as libc::c_char;
        (*last).next = 0 as *mut linestruct;
        (*openfile).current = top;
    }
    (*openfile)
        .totsize = ((*openfile).totsize as libc::c_ulong)
        .wrapping_sub(number_of_characters_in(taken, last)) as size_t as size_t;
    if cutbuffer.is_null() {
        cutbuffer = taken;
        cutbottom = last;
        inherited_anchor = (*taken).has_anchor;
    } else {
        (*cutbottom)
            .data = nrealloc(
            (*cutbottom).data as *mut libc::c_void,
            (strlen((*cutbottom).data))
                .wrapping_add(strlen((*taken).data))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcat((*cutbottom).data, (*taken).data);
        (*cutbottom)
            .has_anchor = (*taken).has_anchor as libc::c_int != 0 && !inherited_anchor;
        inherited_anchor = (inherited_anchor as libc::c_int
            | (*taken).has_anchor as libc::c_int) != 0;
        (*cutbottom).next = (*taken).next;
        delete_node(taken);
        if !((*cutbottom).next).is_null() {
            (*(*cutbottom).next).prev = cutbottom;
            cutbottom = last;
        }
    }
    (*openfile).current_x = top_x;
    (*(*openfile).current).has_anchor = had_anchor;
    if post_marked as libc::c_int != 0 || same_line as libc::c_int != 0 {
        (*openfile).mark = (*openfile).current;
    }
    if post_marked {
        (*openfile).mark_x = (*openfile).current_x;
    }
    if (*openfile).filebot == bot {
        (*openfile).filebot = (*openfile).current;
    }
    renumber_from((*openfile).current);
    if edittop_inside {
        adjust_viewport(STATIONARY);
        refresh_needed = 1 as libc::c_int != 0;
    }
    if !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
        && *((*(*openfile).filebot).data).offset(0 as libc::c_int as isize)
            as libc::c_int != '\0' as i32
    {
        new_magicline();
    }
}
pub unsafe extern "C" fn ingraft_buffer(mut topline: *mut linestruct) {
    let mut line: *mut linestruct = (*openfile).current;
    let mut length: size_t = strlen((*line).data);
    let mut extralen: size_t = strlen((*topline).data);
    let mut xpos: size_t = (*openfile).current_x;
    let mut tailtext: *mut libc::c_char = copy_of(((*line).data).offset(xpos as isize));
    let mut mark_follows: bool = (*openfile).mark == line && !mark_is_before_cursor();
    let mut botline: *mut linestruct = topline;
    while !((*botline).next).is_null() {
        botline = (*botline).next;
    }
    (*openfile)
        .totsize = ((*openfile).totsize as libc::c_ulong)
        .wrapping_add(number_of_characters_in(topline, botline)) as size_t as size_t;
    if topline != botline {
        length = xpos;
    }
    if extralen > 0 as libc::c_int as libc::c_ulong {
        (*line)
            .data = nrealloc(
            (*line).data as *mut libc::c_void,
            length.wrapping_add(extralen).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memmove(
            ((*line).data).offset(xpos as isize).offset(extralen as isize)
                as *mut libc::c_void,
            ((*line).data).offset(xpos as isize) as *const libc::c_void,
            length.wrapping_sub(xpos).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        strncpy(((*line).data).offset(xpos as isize), (*topline).data, extralen);
    }
    if topline != botline {
        if ((*line).next).is_null() {
            (*openfile).filebot = botline;
        }
        *((*line).data)
            .offset(xpos.wrapping_add(extralen) as isize) = '\0' as i32 as libc::c_char;
        (*botline).next = (*(*openfile).current).next;
        if !((*botline).next).is_null() {
            (*(*botline).next).prev = botline;
        }
        (*(*openfile).current).next = (*topline).next;
        (*(*topline).next).prev = (*openfile).current;
        length = strlen((*botline).data);
        extralen = strlen(tailtext);
        (*botline)
            .data = nrealloc(
            (*botline).data as *mut libc::c_void,
            length.wrapping_add(extralen).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(((*botline).data).offset(length as isize), tailtext);
        (*openfile).current = botline;
        (*openfile).current_x = length;
    } else {
        (*openfile)
            .current_x = ((*openfile).current_x as libc::c_ulong).wrapping_add(extralen)
            as size_t as size_t;
    }
    if mark_follows as libc::c_int != 0 && topline != botline {
        (*openfile).mark = botline;
        (*openfile)
            .mark_x = ((*openfile).mark_x as libc::c_ulong)
            .wrapping_add(length.wrapping_sub(xpos)) as size_t as size_t;
    } else if mark_follows {
        (*openfile)
            .mark_x = ((*openfile).mark_x as libc::c_ulong).wrapping_add(extralen)
            as size_t as size_t;
    }
    delete_node(topline);
    rpl_free(tailtext as *mut libc::c_void);
    renumber_from(line);
    if !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
        && *((*(*openfile).filebot).data).offset(0 as libc::c_int as isize)
            as libc::c_int != '\0' as i32
    {
        new_magicline();
    }
}
pub unsafe extern "C" fn copy_from_buffer(mut somebuffer: *mut linestruct) {
    let mut threshold: size_t = ((*(*openfile).edittop).lineno
        + editwinrows as libc::c_long - 1 as libc::c_int as libc::c_long) as size_t;
    let mut the_copy: *mut linestruct = copy_buffer(somebuffer);
    ingraft_buffer(the_copy);
    if (*(*openfile).current).lineno as libc::c_ulong > threshold
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
    {
        recook = 1 as libc::c_int != 0;
    } else {
        perturbed = 1 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn cut_marked_region() {
    let mut top: *mut linestruct = 0 as *mut linestruct;
    let mut bot: *mut linestruct = 0 as *mut linestruct;
    let mut top_x: size_t = 0;
    let mut bot_x: size_t = 0;
    get_region(&mut top, &mut top_x, &mut bot, &mut bot_x);
    extract_segment(top, top_x, bot, bot_x);
    (*openfile).placewewant = xplustabs();
}
pub unsafe extern "C" fn do_snip(
    mut marked: bool,
    mut until_eof: bool,
    mut append: bool,
) {
    let mut line: *mut linestruct = (*openfile).current;
    keep_cutbuffer = (keep_cutbuffer as libc::c_int
        & ((*openfile).last_action as libc::c_uint
            != COPY as libc::c_int as libc::c_uint) as libc::c_int) != 0;
    if (marked as libc::c_int != 0 || until_eof as libc::c_int != 0 || !keep_cutbuffer)
        && !append
    {
        free_lines(cutbuffer);
        cutbuffer = 0 as *mut linestruct;
    }
    if until_eof {
        extract_segment(
            (*openfile).current,
            (*openfile).current_x,
            (*openfile).filebot,
            strlen((*(*openfile).filebot).data),
        );
    } else if !((*openfile).mark).is_null() {
        cut_marked_region();
        (*openfile).mark = 0 as *mut linestruct;
    } else if flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        if *((*line).data).offset((*openfile).current_x as isize) as libc::c_int
            != '\0' as i32
        {
            extract_segment(line, (*openfile).current_x, line, strlen((*line).data));
        } else if (*openfile).current != (*openfile).filebot {
            extract_segment(
                line,
                (*openfile).current_x,
                (*line).next,
                0 as libc::c_int as size_t,
            );
            (*openfile).placewewant = xplustabs();
        }
    } else {
        if (*openfile).current != (*openfile).filebot {
            extract_segment(
                line,
                0 as libc::c_int as size_t,
                (*line).next,
                0 as libc::c_int as size_t,
            );
        } else {
            extract_segment(
                line,
                0 as libc::c_int as size_t,
                line,
                strlen((*line).data),
            );
        }
        (*openfile).placewewant = 0 as libc::c_int as size_t;
    }
    keep_cutbuffer = !marked && !until_eof;
    set_modified();
    refresh_needed = 1 as libc::c_int != 0;
    perturbed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn cut_text() {
    if !is_cuttable(
        flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint && ((*openfile).mark).is_null(),
    ) {
        return;
    }
    if (*openfile).last_action as libc::c_uint != CUT as libc::c_int as libc::c_uint
        || !keep_cutbuffer
    {
        keep_cutbuffer = 0 as libc::c_int != 0;
        add_undo(CUT, 0 as *const libc::c_char);
    }
    do_snip(!((*openfile).mark).is_null(), 0 as libc::c_int != 0, 0 as libc::c_int != 0);
    update_undo(CUT);
    wipe_statusbar();
}
pub unsafe extern "C" fn cut_till_eof() {
    ran_a_tool = 1 as libc::c_int != 0;
    if *((*(*openfile).current).data).offset((*openfile).current_x as isize)
        as libc::c_int == '\0' as i32
        && (((*(*openfile).current).next).is_null()
            || !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
                && (*openfile).current_x > 0 as libc::c_int as libc::c_ulong
                && (*(*openfile).current).next == (*openfile).filebot)
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Nothing was cut\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    add_undo(CUT_TO_EOF, 0 as *const libc::c_char);
    do_snip(0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    update_undo(CUT_TO_EOF);
    wipe_statusbar();
}
pub unsafe extern "C" fn zap_text() {
    let mut was_cutbuffer: *mut linestruct = cutbuffer;
    if !is_cuttable(
        flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint && ((*openfile).mark).is_null(),
    ) {
        return;
    }
    if (*openfile).last_action as libc::c_uint != ZAP as libc::c_int as libc::c_uint
        || !keep_cutbuffer
    {
        add_undo(ZAP, 0 as *const libc::c_char);
    }
    cutbuffer = (*(*openfile).current_undo).cutbuffer;
    do_snip(!((*openfile).mark).is_null(), 0 as libc::c_int != 0, 1 as libc::c_int != 0);
    update_undo(ZAP);
    wipe_statusbar();
    cutbuffer = was_cutbuffer;
}
pub unsafe extern "C" fn copy_marked_region() {
    let mut topline: *mut linestruct = 0 as *mut linestruct;
    let mut botline: *mut linestruct = 0 as *mut linestruct;
    let mut afterline: *mut linestruct = 0 as *mut linestruct;
    let mut was_datastart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_byte: libc::c_char = 0;
    let mut top_x: size_t = 0;
    let mut bot_x: size_t = 0;
    get_region(&mut topline, &mut top_x, &mut botline, &mut bot_x);
    (*openfile).last_action = OTHER;
    keep_cutbuffer = 0 as libc::c_int != 0;
    (*openfile).mark = 0 as *mut linestruct;
    refresh_needed = 1 as libc::c_int != 0;
    if topline == botline && top_x == bot_x {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Copied nothing\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    afterline = (*botline).next;
    (*botline).next = 0 as *mut linestruct;
    saved_byte = *((*botline).data).offset(bot_x as isize);
    *((*botline).data).offset(bot_x as isize) = '\0' as i32 as libc::c_char;
    was_datastart = (*topline).data;
    (*topline).data = ((*topline).data).offset(top_x as isize);
    cutbuffer = copy_buffer(topline);
    (*topline).data = was_datastart;
    *((*botline).data).offset(bot_x as isize) = saved_byte;
    (*botline).next = afterline;
}
pub unsafe extern "C" fn copy_text() {
    let mut at_eol: bool = *((*(*openfile).current).data)
        .offset((*openfile).current_x as isize) as libc::c_int == '\0' as i32;
    let mut sans_newline: bool = flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && ((*(*openfile).current).next).is_null();
    let mut from_x: size_t = if flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*openfile).current_x
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut addition: *mut linestruct = 0 as *mut linestruct;
    if !((*openfile).mark).is_null()
        || (*openfile).last_action as libc::c_uint != COPY as libc::c_int as libc::c_uint
        || !keep_cutbuffer
    {
        free_lines(cutbuffer);
        cutbuffer = 0 as *mut linestruct;
    }
    wipe_statusbar();
    if !((*openfile).mark).is_null() {
        copy_marked_region();
        return;
    }
    if ((*(*openfile).current).next).is_null() && at_eol as libc::c_int != 0
        && (flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
            || (*openfile).current_x == 0 as libc::c_int as libc::c_ulong
            || !cutbuffer.is_null())
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Copied nothing\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    addition = make_new_node(0 as *mut linestruct);
    (*addition).data = copy_of(((*(*openfile).current).data).offset(from_x as isize));
    if flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        sans_newline = !at_eol;
    }
    if cutbuffer.is_null() && sans_newline as libc::c_int != 0 {
        cutbuffer = addition;
        cutbottom = addition;
    } else if cutbuffer.is_null() {
        cutbuffer = addition;
        cutbottom = make_new_node(cutbuffer);
        (*cutbottom).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
        (*cutbuffer).next = cutbottom;
    } else if sans_newline {
        (*addition).prev = (*cutbottom).prev;
        (*(*addition).prev).next = addition;
        delete_node(cutbottom);
        cutbottom = addition;
    } else if flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*addition).prev = cutbottom;
        (*cutbottom).next = addition;
        cutbottom = addition;
    } else {
        (*addition).prev = (*cutbottom).prev;
        (*(*addition).prev).next = addition;
        (*addition).next = cutbottom;
        (*cutbottom).prev = addition;
    }
    if (!(flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint) || at_eol as libc::c_int != 0)
        && !((*(*openfile).current).next).is_null()
    {
        (*openfile).current = (*(*openfile).current).next;
        (*openfile).current_x = 0 as libc::c_int as size_t;
    } else {
        (*openfile).current_x = strlen((*(*openfile).current).data);
    }
    edit_redraw(was_current, FLOWING);
    (*openfile).last_action = COPY;
    keep_cutbuffer = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn paste_text() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut had_anchor: bool = (*was_current).has_anchor;
    let mut was_lineno: ssize_t = (*(*openfile).current).lineno;
    let mut was_leftedge: size_t = 0 as libc::c_int as size_t;
    if cutbuffer.is_null() {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Cutbuffer is empty\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    add_undo(PASTE, 0 as *const libc::c_char);
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
        was_leftedge = leftedge_for(xplustabs(), (*openfile).current);
    }
    copy_from_buffer(cutbuffer);
    let mut line: *mut linestruct = was_current;
    while line != (*(*openfile).current).next {
        (*line).has_anchor = 0 as libc::c_int != 0;
        line = (*line).next;
    }
    (*was_current).has_anchor = had_anchor;
    update_undo(PASTE);
    if (*openfile).current == was_current
        && flags[(BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        do_wrap();
    }
    if less_than_a_screenful(was_lineno as size_t, was_leftedge) {
        focusing = 0 as libc::c_int != 0;
    } else {
        precalc_multicolorinfo();
    }
    (*openfile).placewewant = xplustabs();
    set_modified();
    wipe_statusbar();
    refresh_needed = 1 as libc::c_int != 0;
}

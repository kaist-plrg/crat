use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut also_the_last: bool;
    static mut flags: [libc::c_uint; 4];
    static mut editwincols: libc::c_int;
    static mut openfile: *mut openfilestruct;
    static mut search_regexp: regex_t;
    static mut regmatches: [regmatch_t; 10];
    static mut homedir: *mut libc::c_char;
    fn is_alpha_char(c: *const libc::c_char) -> bool;
    fn mbstrlen(pointer: *const libc::c_char) -> size_t;
    fn advance_over(string: *const libc::c_char, column: *mut size_t) -> libc::c_int;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn mbstrcasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn revstrstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
        pointer: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn mbrevstrcasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
        pointer: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn delete_node(line: *mut linestruct);
    fn die(msg: *const libc::c_char, _: ...);
    fn __errno_location() -> *mut libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn memcpy(
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn geteuid() -> __uid_t;
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
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub unsafe extern "C" fn get_homedir() {
    if homedir.is_null() {
        let mut homenv: *const libc::c_char = getenv(
            b"HOME\0" as *const u8 as *const libc::c_char,
        );
        if homenv.is_null() || geteuid() == 0 as libc::c_int as libc::c_uint {
            let mut userage: *const passwd = getpwuid(geteuid());
            if !userage.is_null() {
                homenv = (*userage).pw_dir;
            }
        }
        if !homenv.is_null() && *homenv as libc::c_int != '\0' as i32 {
            homedir = copy_of(homenv);
        }
    }
}
pub unsafe extern "C" fn tail(mut path: *const libc::c_char) -> *const libc::c_char {
    let mut slash: *const libc::c_char = strrchr(path, '/' as i32);
    if slash.is_null() {
        return path
    } else {
        return slash.offset(1 as libc::c_int as isize)
    };
}
pub unsafe extern "C" fn concatenate(
    mut path: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut pathlen: size_t = strlen(path);
    let mut joined: *mut libc::c_char = nmalloc(
        pathlen
            .wrapping_add(strlen(name))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(joined, path);
    strcpy(joined.offset(pathlen as isize), name);
    return joined;
}
pub unsafe extern "C" fn digits(mut n: ssize_t) -> libc::c_int {
    if n < 100000 as libc::c_int as libc::c_long {
        if n < 1000 as libc::c_int as libc::c_long {
            if n < 100 as libc::c_int as libc::c_long {
                return 2 as libc::c_int
            } else {
                return 3 as libc::c_int
            }
        } else if n < 10000 as libc::c_int as libc::c_long {
            return 4 as libc::c_int
        } else {
            return 5 as libc::c_int
        }
    } else if n < 10000000 as libc::c_int as libc::c_long {
        if n < 1000000 as libc::c_int as libc::c_long {
            return 6 as libc::c_int
        } else {
            return 7 as libc::c_int
        }
    } else if n < 100000000 as libc::c_int as libc::c_long {
        return 8 as libc::c_int
    } else {
        return 9 as libc::c_int
    };
}
pub unsafe extern "C" fn parse_num(
    mut string: *const libc::c_char,
    mut result: *mut ssize_t,
) -> bool {
    let mut value: ssize_t = 0;
    let mut excess: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    value = strtol(string, &mut excess, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int || *string as libc::c_int == '\0' as i32
        || *excess as libc::c_int != '\0' as i32
    {
        return 0 as libc::c_int != 0;
    }
    *result = value;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_line_column(
    mut str: *const libc::c_char,
    mut line: *mut ssize_t,
    mut column: *mut ssize_t,
) -> bool {
    let mut comma: *const libc::c_char = 0 as *const libc::c_char;
    let mut firstpart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: bool = false;
    while *str as libc::c_int == ' ' as i32 {
        str = str.offset(1);
        str;
    }
    comma = strpbrk(str, b"m,. /;\0" as *const u8 as *const libc::c_char);
    if comma.is_null() {
        return parse_num(str, line);
    }
    retval = parse_num(comma.offset(1 as libc::c_int as isize), column);
    if comma == str {
        return retval;
    }
    firstpart = copy_of(str);
    *firstpart
        .offset(
            comma.offset_from(str) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    retval = parse_num(firstpart, line) as libc::c_int != 0
        && retval as libc::c_int != 0;
    rpl_free(firstpart as *mut libc::c_void);
    return retval;
}
pub unsafe extern "C" fn recode_NUL_to_LF(
    mut string: *mut libc::c_char,
    mut length: size_t,
) {
    while length > 0 as libc::c_int as libc::c_ulong {
        if *string as libc::c_int == '\0' as i32 {
            *string = '\n' as i32 as libc::c_char;
        }
        length = length.wrapping_sub(1);
        length;
        string = string.offset(1);
        string;
    }
}
pub unsafe extern "C" fn recode_LF_to_NUL(mut string: *mut libc::c_char) -> size_t {
    let mut beginning: *mut libc::c_char = string;
    while *string as libc::c_int != '\0' as i32 {
        if *string as libc::c_int == '\n' as i32 {
            *string = '\0' as i32 as libc::c_char;
        }
        string = string.offset(1);
        string;
    }
    return string.offset_from(beginning) as libc::c_long as size_t;
}
pub unsafe extern "C" fn free_chararray(
    mut array: *mut *mut libc::c_char,
    mut len: size_t,
) {
    if array.is_null() {
        return;
    }
    while len > 0 as libc::c_int as libc::c_ulong {
        len = len.wrapping_sub(1);
        rpl_free(*array.offset(len as isize) as *mut libc::c_void);
    }
    rpl_free(array as *mut libc::c_void);
}
pub unsafe extern "C" fn is_separate_word(
    mut position: size_t,
    mut length: size_t,
    mut text: *const libc::c_char,
) -> bool {
    let mut before: *const libc::c_char = text
        .offset(step_left(text, position) as isize);
    let mut after: *const libc::c_char = text
        .offset(position as isize)
        .offset(length as isize);
    return (position == 0 as libc::c_int as libc::c_ulong || !is_alpha_char(before))
        && (*after as libc::c_int == '\0' as i32 || !is_alpha_char(after));
}
pub unsafe extern "C" fn strstrwrapper(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
    mut start: *const libc::c_char,
) -> *const libc::c_char {
    if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            let mut last_find: size_t = 0;
            let mut ceiling: size_t = 0;
            let mut far_end: size_t = 0;
            let mut floor: size_t = 0 as libc::c_int as size_t;
            let mut next_rung: size_t = 0 as libc::c_int as size_t;
            if rpl_regexec(
                &mut search_regexp,
                haystack,
                1 as libc::c_int as size_t,
                regmatches.as_mut_ptr(),
                0 as libc::c_int,
            ) != 0 as libc::c_int
            {
                return 0 as *const libc::c_char;
            }
            far_end = strlen(haystack);
            ceiling = start.offset_from(haystack) as libc::c_long as size_t;
            last_find = regmatches[0 as libc::c_int as usize].rm_so as size_t;
            if last_find > ceiling {
                return 0 as *const libc::c_char;
            }
            while regmatches[0 as libc::c_int as usize].rm_so as libc::c_ulong <= ceiling
            {
                floor = next_rung;
                last_find = regmatches[0 as libc::c_int as usize].rm_so as size_t;
                if last_find == ceiling {
                    break;
                }
                next_rung = step_right(haystack, last_find);
                regmatches[0 as libc::c_int as usize].rm_so = next_rung as regoff_t;
                regmatches[0 as libc::c_int as usize].rm_eo = far_end as regoff_t;
                if rpl_regexec(
                    &mut search_regexp,
                    haystack,
                    1 as libc::c_int as size_t,
                    regmatches.as_mut_ptr(),
                    (1 as libc::c_int) << 2 as libc::c_int,
                ) != 0 as libc::c_int
                {
                    break;
                }
            }
            regmatches[0 as libc::c_int as usize].rm_so = floor as regoff_t;
            regmatches[0 as libc::c_int as usize].rm_eo = far_end as regoff_t;
            if rpl_regexec(
                &mut search_regexp,
                haystack,
                10 as libc::c_int as size_t,
                regmatches.as_mut_ptr(),
                (1 as libc::c_int) << 2 as libc::c_int,
            ) != 0 as libc::c_int
            {
                return 0 as *const libc::c_char;
            }
            return haystack.offset(regmatches[0 as libc::c_int as usize].rm_so as isize);
        }
        regmatches[0 as libc::c_int as usize]
            .rm_so = start.offset_from(haystack) as libc::c_long;
        regmatches[0 as libc::c_int as usize].rm_eo = strlen(haystack) as regoff_t;
        if rpl_regexec(
            &mut search_regexp,
            haystack,
            10 as libc::c_int as size_t,
            regmatches.as_mut_ptr(),
            (1 as libc::c_int) << 2 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return 0 as *const libc::c_char
        } else {
            return haystack.offset(regmatches[0 as libc::c_int as usize].rm_so as isize)
        }
    }
    if flags[(CASE_SENSITIVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            return revstrstr(haystack, needle, start)
        } else {
            return strstr(start, needle)
        }
    }
    if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        return mbrevstrcasestr(haystack, needle, start)
    } else {
        return mbstrcasestr(start, needle)
    };
}
pub unsafe extern "C" fn nmalloc(mut howmuch: size_t) -> *mut libc::c_void {
    let mut section: *mut libc::c_void = malloc(howmuch);
    if section.is_null() {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Nano is out of memory!\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return section;
}
pub unsafe extern "C" fn nrealloc(
    mut section: *mut libc::c_void,
    mut howmuch: size_t,
) -> *mut libc::c_void {
    section = realloc(section, howmuch);
    if section.is_null() {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Nano is out of memory!\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return section;
}
pub unsafe extern "C" fn mallocstrcpy(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> *mut libc::c_char {
    let mut count: size_t = (strlen(src))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    dest = nrealloc(dest as *mut libc::c_void, count) as *mut libc::c_char;
    strncpy(dest, src, count);
    return dest;
}
pub unsafe extern "C" fn measured_copy(
    mut string: *const libc::c_char,
    mut count: size_t,
) -> *mut libc::c_char {
    let mut thecopy: *mut libc::c_char = nmalloc(
        count.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(thecopy as *mut libc::c_void, string as *const libc::c_void, count);
    *thecopy.offset(count as isize) = '\0' as i32 as libc::c_char;
    return thecopy;
}
pub unsafe extern "C" fn copy_of(mut string: *const libc::c_char) -> *mut libc::c_char {
    return measured_copy(string, strlen(string));
}
pub unsafe extern "C" fn free_and_assign(
    mut dest: *mut libc::c_char,
    mut src: *mut libc::c_char,
) -> *mut libc::c_char {
    rpl_free(dest as *mut libc::c_void);
    return src;
}
pub unsafe extern "C" fn get_page_start(mut column: size_t) -> size_t {
    if column == 0 as libc::c_int as libc::c_ulong
        || column.wrapping_add(2 as libc::c_int as libc::c_ulong)
            < editwincols as libc::c_ulong
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
        return 0 as libc::c_int as size_t
    } else if editwincols > 8 as libc::c_int {
        return column
            .wrapping_sub(6 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                column
                    .wrapping_sub(6 as libc::c_int as libc::c_ulong)
                    .wrapping_rem((editwincols - 8 as libc::c_int) as libc::c_ulong),
            )
    } else {
        return column.wrapping_sub((editwincols - 2 as libc::c_int) as libc::c_ulong)
    };
}
pub unsafe extern "C" fn xplustabs() -> size_t {
    return wideness((*(*openfile).current).data, (*openfile).current_x);
}
pub unsafe extern "C" fn actual_x(
    mut text: *const libc::c_char,
    mut column: size_t,
) -> size_t {
    let mut start: *const libc::c_char = text;
    let mut width: size_t = 0 as libc::c_int as size_t;
    while *text as libc::c_int != '\0' as i32 {
        let mut charlen: libc::c_int = advance_over(text, &mut width);
        if width > column {
            break;
        }
        text = text.offset(charlen as isize);
    }
    return text.offset_from(start) as libc::c_long as size_t;
}
pub unsafe extern "C" fn wideness(
    mut text: *const libc::c_char,
    mut maxlen: size_t,
) -> size_t {
    let mut width: size_t = 0 as libc::c_int as size_t;
    if maxlen == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    while *text as libc::c_int != '\0' as i32 {
        let mut charlen: size_t = advance_over(text, &mut width) as size_t;
        if maxlen <= charlen {
            break;
        }
        maxlen = (maxlen as libc::c_ulong).wrapping_sub(charlen) as size_t as size_t;
        text = text.offset(charlen as isize);
    }
    return width;
}
pub unsafe extern "C" fn breadth(mut text: *const libc::c_char) -> size_t {
    let mut span: size_t = 0 as libc::c_int as size_t;
    while *text as libc::c_int != '\0' as i32 {
        text = text.offset(advance_over(text, &mut span) as isize);
    }
    return span;
}
pub unsafe extern "C" fn new_magicline() {
    (*(*openfile).filebot).next = make_new_node((*openfile).filebot);
    (*(*(*openfile).filebot).next)
        .data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    (*openfile).filebot = (*(*openfile).filebot).next;
    (*openfile).totsize = ((*openfile).totsize).wrapping_add(1);
    (*openfile).totsize;
}
pub unsafe extern "C" fn remove_magicline() {
    if *((*(*openfile).filebot).data).offset(0 as libc::c_int as isize) as libc::c_int
        == '\0' as i32 && (*openfile).filebot != (*openfile).filetop
    {
        (*openfile).filebot = (*(*openfile).filebot).prev;
        delete_node((*(*openfile).filebot).next);
        (*(*openfile).filebot).next = 0 as *mut linestruct;
        (*openfile).totsize = ((*openfile).totsize).wrapping_sub(1);
        (*openfile).totsize;
    }
}
pub unsafe extern "C" fn mark_is_before_cursor() -> bool {
    return (*(*openfile).mark).lineno < (*(*openfile).current).lineno
        || (*openfile).mark == (*openfile).current
            && (*openfile).mark_x <= (*openfile).current_x;
}
pub unsafe extern "C" fn get_region(
    mut top: *mut *mut linestruct,
    mut top_x: *mut size_t,
    mut bot: *mut *mut linestruct,
    mut bot_x: *mut size_t,
) {
    if mark_is_before_cursor() {
        *top = (*openfile).mark;
        *top_x = (*openfile).mark_x;
        *bot = (*openfile).current;
        *bot_x = (*openfile).current_x;
    } else {
        *bot = (*openfile).mark;
        *bot_x = (*openfile).mark_x;
        *top = (*openfile).current;
        *top_x = (*openfile).current_x;
    };
}
pub unsafe extern "C" fn get_range(
    mut top: *mut *mut linestruct,
    mut bot: *mut *mut linestruct,
) {
    if ((*openfile).mark).is_null() {
        *top = (*openfile).current;
        *bot = (*openfile).current;
    } else {
        let mut top_x: size_t = 0;
        let mut bot_x: size_t = 0;
        get_region(top, &mut top_x, bot, &mut bot_x);
        if bot_x == 0 as libc::c_int as libc::c_ulong && *bot != *top && !also_the_last {
            *bot = (**bot).prev;
        } else {
            also_the_last = 1 as libc::c_int != 0;
        }
    };
}
pub unsafe extern "C" fn line_from_number(mut number: ssize_t) -> *mut linestruct {
    let mut line: *mut linestruct = (*openfile).current;
    if (*line).lineno > number {
        while (*line).lineno != number {
            line = (*line).prev;
        }
    } else {
        while (*line).lineno != number {
            line = (*line).next;
        }
    }
    return line;
}
pub unsafe extern "C" fn number_of_characters_in(
    mut begin: *const linestruct,
    mut end: *const linestruct,
) -> size_t {
    let mut line: *const linestruct = 0 as *const linestruct;
    let mut count: size_t = 0 as libc::c_int as size_t;
    line = begin;
    while line != (*end).next as *const linestruct {
        count = (count as libc::c_ulong)
            .wrapping_add(
                (mbstrlen((*line).data)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        line = (*line).next;
    }
    return count.wrapping_sub(1 as libc::c_int as libc::c_ulong);
}

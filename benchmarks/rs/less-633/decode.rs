use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xbuf_char_data(xbuf: *mut xbuffer) -> *mut libc::c_char;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn special_key_str(key: libc::c_int) -> *mut libc::c_char;
    fn getcc() -> libc::c_int;
    fn ungetcc(c: LWCHAR);
    fn ungetsc(s: *mut libc::c_char);
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn filesize(f: libc::c_int) -> POSITION;
    fn homefile(filename: *mut libc::c_char) -> *mut libc::c_char;
    fn setmark(c: LWCHAR, where_0: libc::c_int);
    fn dirfile(
        dirname: *mut libc::c_char,
        filename: *mut libc::c_char,
        must_exist: libc::c_int,
    ) -> *mut libc::c_char;
    fn help_ckd_add(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn help_ckd_mul(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn parse_lesskey(
        infile: *mut libc::c_char,
        tables: *mut lesskey_tables,
    ) -> libc::c_int;
    static mut erase_char: libc::c_int;
    static mut erase2_char: libc::c_int;
    static mut kill_char: libc::c_int;
    static mut secure: libc::c_int;
    static mut mousecap: libc::c_int;
    static mut screen_trashed: libc::c_int;
    static mut sc_height: libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut libc::c_uchar,
    pub end: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tablelist {
    pub t_next: *mut tablelist,
    pub t_start: *mut libc::c_char,
    pub t_end: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_table {
    pub names: *mut lesskey_cmdname,
    pub buf: xbuffer,
    pub is_var: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_cmdname {
    pub cn_name: *mut libc::c_char,
    pub cn_action: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
static mut cmdtable: [libc::c_uchar; 504] = [
    '\r' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    '\n' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    'e' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    'j' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    ('E' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    ('N' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    'k' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    'y' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    ('Y' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    ('P' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    'J' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    'K' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    'Y' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    'd' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    ('D' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    'u' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    ('U' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    'M' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    68 as libc::c_int as libc::c_uchar,
    ' ' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    'f' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    ('F' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    ('V' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    ('B' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'v' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    'z' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    'w' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ' ' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    'F' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'F' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    'R' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    'r' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    ('R' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    ('L' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'u' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'U' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    'g' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    'p' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    '%' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ']' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '(' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ')' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '{' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '}' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    '{' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (35 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '{' as i32 as libc::c_uchar,
    '}' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    '}' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (36 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '{' as i32 as libc::c_uchar,
    '}' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    '(' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (35 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '(' as i32 as libc::c_uchar,
    ')' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    ')' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (36 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '(' as i32 as libc::c_uchar,
    ')' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (35 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    ']' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    ']' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (36 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    ']' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('F' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('B' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    'G' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'G' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    'P' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    '0' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '1' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '2' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '3' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '4' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '5' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '6' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '7' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '8' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '9' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '.' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    ('G' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'f' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    '/' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    '?' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '/' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (15 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '*' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '?' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (5 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    '*' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    'n' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'n' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    'N' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'N' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    '&' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    'm' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    'M' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'm' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    '\'' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    ('X' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('X' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    'E' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'e' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    ('X' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('V' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'n' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'p' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    't' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    'T' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'x' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'd' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    '-' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    't' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (47 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    't' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    's' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (47 as libc::c_int | 0o200 as libc::c_int) as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    '_' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    '|' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    'v' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    '!' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    '#' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    69 as libc::c_int as libc::c_uchar,
    '+' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    'H' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    'h' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    'V' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    'q' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    'Q' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'q' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
    'Q' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    'Z' as i32 as libc::c_uchar,
    'Z' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
];
static mut edittable: [libc::c_uchar; 216] = [
    '\t' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    '\u{f}' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '\t' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    ('L' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    ('V' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    ('A' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'l' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'h' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'w' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'x' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'X' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '0' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '$' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'k' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    'j' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    ('K' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    ('G' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    'M' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    ('[' as i32 & 0o37 as libc::c_int) as libc::c_uchar,
    '[' as i32 as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
];
static mut list_fcmd_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
static mut list_ecmd_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
static mut list_var_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
static mut list_sysvar_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
unsafe extern "C" fn expand_special_keys(
    mut table: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut fm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_int = 0;
    let mut repl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut klen: libc::c_int = 0;
    fm = table;
    while fm < table.offset(len as isize) {
        to = fm;
        while *fm as libc::c_int != '\0' as i32 {
            if *fm as libc::c_int != 'K' as i32 & 0o37 as libc::c_int {
                let fresh0 = fm;
                fm = fm.offset(1);
                let fresh1 = to;
                to = to.offset(1);
                *fresh1 = *fresh0;
            } else {
                repl = special_key_str(
                    *fm.offset(1 as libc::c_int as isize) as libc::c_int,
                );
                klen = *fm.offset(2 as libc::c_int as isize) as libc::c_int
                    & 0o377 as libc::c_int;
                fm = fm.offset(klen as isize);
                if repl.is_null() || strlen(repl) as libc::c_int > klen {
                    repl = b"\xFF\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                while *repl as libc::c_int != '\0' as i32 {
                    let fresh2 = repl;
                    repl = repl.offset(1);
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = *fresh2;
                }
            }
        }
        let fresh4 = to;
        to = to.offset(1);
        *fresh4 = '\0' as i32 as libc::c_char;
        while to <= fm {
            let fresh5 = to;
            to = to.offset(1);
            *fresh5 = 127 as libc::c_int as libc::c_char;
        }
        fm = fm.offset(1);
        fm;
        let fresh6 = fm;
        fm = fm.offset(1);
        a = *fresh6 as libc::c_int & 0o377 as libc::c_int;
        if a & 0o200 as libc::c_int != 0 {
            loop {
                let fresh7 = fm;
                fm = fm.offset(1);
                if !(*fresh7 as libc::c_int != '\0' as i32) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn expand_cmd_table(mut tlist: *mut tablelist) {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    t = tlist;
    while !t.is_null() {
        expand_special_keys(
            (*t).t_start,
            ((*t).t_end).offset_from((*t).t_start) as libc::c_long as libc::c_int,
        );
        t = (*t).t_next;
    }
}
pub unsafe extern "C" fn expand_cmd_tables() {
    expand_cmd_table(list_fcmd_tables);
    expand_cmd_table(list_ecmd_tables);
    expand_cmd_table(list_var_tables);
    expand_cmd_table(list_sysvar_tables);
}
pub unsafe extern "C" fn init_cmds() {
    add_fcmd_table(
        cmdtable.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_uchar; 504]>() as libc::c_ulong as libc::c_int,
    );
    add_ecmd_table(
        edittable.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_uchar; 216]>() as libc::c_ulong as libc::c_int,
    );
    add_hometable(
        Some(
            lesskey
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
        ),
        0 as *mut libc::c_char,
        b"/usr/local/bin/.sysless\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int,
    );
    if add_hometable(
        Some(
            lesskey_src
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
        ),
        b"LESSKEYIN_SYSTEM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/usr/local/etc/syslesskey\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int,
    ) != 0 as libc::c_int
    {
        add_hometable(
            Some(
                lesskey
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            b"LESSKEY_SYSTEM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"/usr/local/etc/sysless\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    if add_hometable(
        Some(
            lesskey_src
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
        ),
        b"LESSKEYIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".lesskey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        add_hometable(
            Some(
                lesskey
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            b"LESSKEY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn add_cmd_table(
    mut tlist: *mut *mut tablelist,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    if len == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<tablelist>() as libc::c_ulong,
    ) as *mut tablelist;
    if t.is_null() {
        return -(1 as libc::c_int);
    }
    (*t).t_start = buf;
    (*t).t_end = buf.offset(len as isize);
    (*t).t_next = *tlist;
    *tlist = t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn add_fcmd_table(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) {
    if add_cmd_table(&mut list_fcmd_tables, buf, len) < 0 as libc::c_int {
        error(
            b"Warning: some commands disabled\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
}
pub unsafe extern "C" fn add_ecmd_table(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) {
    if add_cmd_table(&mut list_ecmd_tables, buf, len) < 0 as libc::c_int {
        error(
            b"Warning: some edit commands disabled\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
}
unsafe extern "C" fn add_var_table(
    mut tlist: *mut *mut tablelist,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) {
    if add_cmd_table(tlist, buf, len) < 0 as libc::c_int {
        error(
            b"Warning: environment variables from lesskey file unavailable\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
}
unsafe extern "C" fn mouse_wheel_down() -> libc::c_int {
    return if mousecap == 2 as libc::c_int {
        67 as libc::c_int
    } else {
        66 as libc::c_int
    };
}
unsafe extern "C" fn mouse_wheel_up() -> libc::c_int {
    return if mousecap == 2 as libc::c_int {
        66 as libc::c_int
    } else {
        67 as libc::c_int
    };
}
unsafe extern "C" fn mouse_button_rel(
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    if y < sc_height - 1 as libc::c_int {
        setmark('#' as i32 as LWCHAR, y);
        screen_trashed = 1 as libc::c_int;
    }
    return 101 as libc::c_int;
}
unsafe extern "C" fn getcc_int(mut pterm: *mut libc::c_char) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut digits: libc::c_int = 0 as libc::c_int;
    loop {
        let mut ch: libc::c_char = getcc() as libc::c_char;
        if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
            if !pterm.is_null() {
                *pterm = ch;
            }
            if digits == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            return num;
        }
        if help_ckd_mul(
            &mut num as *mut libc::c_int as *mut libc::c_void,
            num as uintmax,
            10 as libc::c_int as uintmax,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { num })
                - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
        ) != 0
            || help_ckd_add(
                &mut num as *mut libc::c_int as *mut libc::c_void,
                num as uintmax,
                (ch as libc::c_int - '0' as i32) as uintmax,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { num })
                    - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
            ) != 0
        {
            return -(1 as libc::c_int);
        }
        digits += 1;
        digits;
    };
}
unsafe extern "C" fn x11mouse_action(mut skip: libc::c_int) -> libc::c_int {
    let mut b: libc::c_int = getcc() - 0x20 as libc::c_int;
    let mut x: libc::c_int = getcc() - 0x20 as libc::c_int - 1 as libc::c_int;
    let mut y: libc::c_int = getcc() - 0x20 as libc::c_int - 1 as libc::c_int;
    if skip != 0 {
        return 101 as libc::c_int;
    }
    match b {
        65 => return mouse_wheel_down(),
        64 => return mouse_wheel_up(),
        3 => return mouse_button_rel(x, y),
        _ => return 101 as libc::c_int,
    };
}
unsafe extern "C" fn x116mouse_action(mut skip: libc::c_int) -> libc::c_int {
    let mut ch: libc::c_char = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = getcc_int(&mut ch);
    if b < 0 as libc::c_int || ch as libc::c_int != ';' as i32 {
        return 101 as libc::c_int;
    }
    x = getcc_int(&mut ch) - 1 as libc::c_int;
    if x < 0 as libc::c_int || ch as libc::c_int != ';' as i32 {
        return 101 as libc::c_int;
    }
    y = getcc_int(&mut ch) - 1 as libc::c_int;
    if y < 0 as libc::c_int {
        return 101 as libc::c_int;
    }
    if skip != 0 {
        return 101 as libc::c_int;
    }
    match b {
        65 => return mouse_wheel_down(),
        64 => return mouse_wheel_up(),
        _ => {
            if ch as libc::c_int != 'm' as i32 {
                return 101 as libc::c_int;
            }
            return mouse_button_rel(x, y);
        }
    };
}
unsafe extern "C" fn cmd_search(
    mut cmd: *mut libc::c_char,
    mut table: *mut libc::c_char,
    mut endtable: *mut libc::c_char,
    mut sp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_int = 0;
    *sp = 0 as *mut libc::c_char;
    p = table;
    q = cmd;
    while p < endtable {
        if *p as libc::c_int == *q as libc::c_int {
            if *p as libc::c_int == '\0' as i32 {
                p = p.offset(1);
                a = *p as libc::c_int & 0o377 as libc::c_int;
                while a == 127 as libc::c_int {
                    p = p.offset(1);
                    a = *p as libc::c_int & 0o377 as libc::c_int;
                }
                if a == 103 as libc::c_int {
                    return 102 as libc::c_int;
                }
                if a & 0o200 as libc::c_int != 0 {
                    p = p.offset(1);
                    *sp = p;
                    a &= !(0o200 as libc::c_int);
                }
                if a == 64 as libc::c_int {
                    a = x11mouse_action(0 as libc::c_int);
                } else if a == 68 as libc::c_int {
                    a = x116mouse_action(0 as libc::c_int);
                }
                return a;
            }
        } else if *q as libc::c_int == '\0' as i32 {
            return 105 as libc::c_int
        } else {
            if *p as libc::c_int == '\0' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == 103 as libc::c_int
            {
                return 102 as libc::c_int;
            }
            loop {
                let fresh8 = p;
                p = p.offset(1);
                if !(*fresh8 as libc::c_int != '\0' as i32) {
                    break;
                }
            }
            while *p as libc::c_int == 127 as libc::c_int {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int & 0o200 as libc::c_int != 0 {
                loop {
                    p = p.offset(1);
                    if !(*p as libc::c_int != '\0' as i32) {
                        break;
                    }
                }
            }
            q = cmd.offset(-(1 as libc::c_int as isize));
        }
        p = p.offset(1);
        p;
        q = q.offset(1);
        q;
    }
    return 100 as libc::c_int;
}
unsafe extern "C" fn cmd_decode(
    mut tlist: *mut tablelist,
    mut cmd: *mut libc::c_char,
    mut sp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    let mut action: libc::c_int = 100 as libc::c_int;
    t = tlist;
    while !t.is_null() {
        action = cmd_search(cmd, (*t).t_start, (*t).t_end, sp);
        if action != 100 as libc::c_int {
            break;
        }
        t = (*t).t_next;
    }
    if action == 102 as libc::c_int {
        action = 100 as libc::c_int;
    }
    return action;
}
pub unsafe extern "C" fn fcmd_decode(
    mut cmd: *mut libc::c_char,
    mut sp: *mut *mut libc::c_char,
) -> libc::c_int {
    return cmd_decode(list_fcmd_tables, cmd, sp);
}
pub unsafe extern "C" fn ecmd_decode(
    mut cmd: *mut libc::c_char,
    mut sp: *mut *mut libc::c_char,
) -> libc::c_int {
    return cmd_decode(list_ecmd_tables, cmd, sp);
}
pub unsafe extern "C" fn lgetenv(mut var: *mut libc::c_char) -> *mut libc::c_char {
    let mut a: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    a = cmd_decode(list_var_tables, var, &mut s);
    if a == 0o1 as libc::c_int {
        return s;
    }
    s = getenv(var);
    if !s.is_null() && *s as libc::c_int != '\0' as i32 {
        return s;
    }
    a = cmd_decode(list_sysvar_tables, var, &mut s);
    if a == 0o1 as libc::c_int {
        return s;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn isnullenv(mut s: *mut libc::c_char) -> libc::c_int {
    return (s.is_null() || *s as libc::c_int == '\0' as i32) as libc::c_int;
}
unsafe extern "C" fn gint(mut sp: *mut *mut libc::c_char) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let fresh9 = *sp;
    *sp = (*sp).offset(1);
    n = *fresh9 as libc::c_int;
    let fresh10 = *sp;
    *sp = (*sp).offset(1);
    n += *fresh10 as libc::c_int * 64 as libc::c_int;
    return n;
}
unsafe extern "C" fn old_lesskey(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    if *buf.offset((len - 1 as libc::c_int) as isize) as libc::c_int != '\0' as i32
        && *buf.offset((len - 2 as libc::c_int) as isize) as libc::c_int != '\0' as i32
    {
        return -(1 as libc::c_int);
    }
    add_fcmd_table(buf, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn new_lesskey(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut sysvar: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if *buf.offset((len - 3 as libc::c_int) as isize) as libc::c_int != 'E' as i32
        || *buf.offset((len - 2 as libc::c_int) as isize) as libc::c_int != 'n' as i32
        || *buf.offset((len - 1 as libc::c_int) as isize) as libc::c_int != 'd' as i32
    {
        return -(1 as libc::c_int);
    }
    p = buf.offset(4 as libc::c_int as isize);
    end = buf.offset(len as isize);
    loop {
        let fresh11 = p;
        p = p.offset(1);
        c = *fresh11 as libc::c_int;
        match c {
            99 => {
                n = gint(&mut p);
                if n < 0 as libc::c_int || p.offset(n as isize) >= end {
                    return -(1 as libc::c_int);
                }
                add_fcmd_table(p, n);
                p = p.offset(n as isize);
            }
            101 => {
                n = gint(&mut p);
                if n < 0 as libc::c_int || p.offset(n as isize) >= end {
                    return -(1 as libc::c_int);
                }
                add_ecmd_table(p, n);
                p = p.offset(n as isize);
            }
            118 => {
                n = gint(&mut p);
                if n < 0 as libc::c_int || p.offset(n as isize) >= end {
                    return -(1 as libc::c_int);
                }
                add_var_table(
                    if sysvar != 0 {
                        &mut list_sysvar_tables
                    } else {
                        &mut list_var_tables
                    },
                    p,
                    n,
                );
                p = p.offset(n as isize);
            }
            120 => return 0 as libc::c_int,
            _ => return -(1 as libc::c_int),
        }
    };
}
pub unsafe extern "C" fn lesskey(
    mut filename: *mut libc::c_char,
    mut sysvar: libc::c_int,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: POSITION = 0;
    let mut n: libc::c_long = 0;
    let mut f: libc::c_int = 0;
    if secure != 0 {
        return 1 as libc::c_int;
    }
    f = open(filename, 0 as libc::c_int);
    if f < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    len = filesize(f);
    if len == -(1 as libc::c_int) as POSITION || len < 3 as libc::c_int as libc::c_long {
        close(f);
        return -(1 as libc::c_int);
    }
    buf = calloc(
        len as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    if buf.is_null() {
        close(f);
        return -(1 as libc::c_int);
    }
    if lseek(f, 0 as libc::c_int as off_t, 0 as libc::c_int)
        == -(1 as libc::c_int) as off_t
    {
        free(buf as *mut libc::c_void);
        close(f);
        return -(1 as libc::c_int);
    }
    n = read(f, buf as *mut libc::c_void, len as libc::c_uint as size_t);
    close(f);
    if n != len {
        free(buf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if len < 4 as libc::c_int as libc::c_long
        || *buf.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        || *buf.offset(1 as libc::c_int as isize) as libc::c_int != 'M' as i32
        || *buf.offset(2 as libc::c_int as isize) as libc::c_int != '+' as i32
        || *buf.offset(3 as libc::c_int as isize) as libc::c_int != 'G' as i32
    {
        return old_lesskey(buf, len as libc::c_int);
    }
    return new_lesskey(buf, len as libc::c_int, sysvar);
}
pub unsafe extern "C" fn lesskey_src(
    mut filename: *mut libc::c_char,
    mut sysvar: libc::c_int,
) -> libc::c_int {
    static mut tables: lesskey_tables = lesskey_tables {
        currtable: 0 as *const lesskey_table as *mut lesskey_table,
        cmdtable: lesskey_table {
            names: 0 as *const lesskey_cmdname as *mut lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                end: 0,
                size: 0,
            },
            is_var: 0,
        },
        edittable: lesskey_table {
            names: 0 as *const lesskey_cmdname as *mut lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                end: 0,
                size: 0,
            },
            is_var: 0,
        },
        vartable: lesskey_table {
            names: 0 as *const lesskey_cmdname as *mut lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                end: 0,
                size: 0,
            },
            is_var: 0,
        },
    };
    let mut r: libc::c_int = parse_lesskey(filename, &mut tables);
    if r != 0 as libc::c_int {
        return r;
    }
    add_fcmd_table(xbuf_char_data(&mut tables.cmdtable.buf), tables.cmdtable.buf.end);
    add_ecmd_table(xbuf_char_data(&mut tables.edittable.buf), tables.edittable.buf.end);
    add_var_table(
        if sysvar != 0 { &mut list_sysvar_tables } else { &mut list_var_tables },
        xbuf_char_data(&mut tables.vartable.buf),
        tables.vartable.buf.end,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lesskey_parse_error(mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    parg.p_string = s;
    error(b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut parg);
}
pub unsafe extern "C" fn add_hometable(
    mut call_lesskey: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    mut envname: *mut libc::c_char,
    mut def_filename: *mut libc::c_char,
    mut sysvar: libc::c_int,
) -> libc::c_int {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    if !envname.is_null()
        && {
            filename = lgetenv(envname);
            !filename.is_null()
        }
    {
        filename = save(filename);
    } else if sysvar != 0 {
        filename = save(def_filename);
    } else {
        let mut xdg: *mut libc::c_char = lgetenv(
            b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if isnullenv(xdg) == 0 {
            filename = dirfile(
                xdg,
                &mut *def_filename.offset(1 as libc::c_int as isize),
                1 as libc::c_int,
            );
        }
        if filename.is_null() {
            let mut home: *mut libc::c_char = lgetenv(
                b"HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if isnullenv(home) == 0 {
                let mut cfg_dir: *mut libc::c_char = dirfile(
                    home,
                    b".config\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as libc::c_int,
                );
                filename = dirfile(
                    cfg_dir,
                    &mut *def_filename.offset(1 as libc::c_int as isize),
                    1 as libc::c_int,
                );
                free(cfg_dir as *mut libc::c_void);
            }
        }
        if filename.is_null() {
            filename = homefile(def_filename);
        }
    }
    if filename.is_null() {
        return -(1 as libc::c_int);
    }
    r = (Some(call_lesskey.unwrap())).unwrap()(filename, sysvar);
    free(filename as *mut libc::c_void);
    return r;
}
pub unsafe extern "C" fn editchar(
    mut c: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut action: libc::c_int = 0;
    let mut nch: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut usercmd: [libc::c_char; 17] = [0; 17];
    if c == erase_char || c == erase2_char {
        return 1 as libc::c_int;
    }
    if c == kill_char {
        return 2 as libc::c_int;
    }
    nch = 0 as libc::c_int;
    loop {
        if nch > 0 as libc::c_int {
            c = getcc();
        }
        usercmd[nch as usize] = c as libc::c_char;
        usercmd[(nch + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
        nch += 1;
        nch;
        action = ecmd_decode(usercmd.as_mut_ptr(), &mut s);
        if !(action == 105 as libc::c_int && nch < 16 as libc::c_int) {
            break;
        }
    }
    if action == 21 as libc::c_int {
        return x11mouse_action(1 as libc::c_int);
    }
    if action == 22 as libc::c_int {
        return x116mouse_action(1 as libc::c_int);
    }
    if flags & 0o10 as libc::c_int != 0 {
        match action {
            3 | 4 => {
                action = 100 as libc::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o2 as libc::c_int != 0 {
        match action {
            13 | 14 => {
                action = 100 as libc::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o4 as libc::c_int != 0 {
        match action {
            17 | 18 | 15 => {
                action = 100 as libc::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o1 as libc::c_int != 0 || action == 100 as libc::c_int {
        while nch > 1 as libc::c_int {
            nch -= 1;
            ungetcc(usercmd[nch as usize] as LWCHAR);
        }
    } else if !s.is_null() {
        ungetsc(s);
    }
    return action;
}

use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type buffer;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
    fn panic(str: *const libc::c_char, _: ...);
    fn ck_fopen(
        name: *const libc::c_char,
        mode: *const libc::c_char,
        fail: libc::c_int,
    ) -> *mut FILE;
    fn ck_fclose(stream: *mut FILE);
    fn init_buffer() -> *mut buffer;
    fn get_buffer(b: *const buffer) -> *mut libc::c_char;
    fn size_buffer(b: *const buffer) -> size_t;
    fn add1_buffer(b: *mut buffer, ch: libc::c_int) -> *mut libc::c_char;
    fn free_buffer(b: *mut buffer);
    static mut posixicity: posixicity_types;
    static mut no_default_output: bool;
    static mut debug: bool;
    static mut read_mode: *const libc::c_char;
    static mut write_mode: *const libc::c_char;
    fn compile_regex(
        b: *mut buffer,
        flags: libc::c_int,
        needed_sub: libc::c_int,
    ) -> *mut regex;
    static mut sandbox: bool;
    fn is_mb_char(ch: libc::c_int, ps: *mut mbstate_t) -> libc::c_int;
    static mut mb_cur_max: libc::c_int;
    fn strverscmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    static mut program_name: *const libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type FILE = _IO_FILE;
pub type ptrdiff_t = libc::c_long;
pub type __int32_t = libc::c_int;
pub type countT = libc::c_ulong;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type exit_codes = libc::c_uint;
pub const EXIT_PANIC: exit_codes = 4;
pub const EXIT_BAD_INPUT: exit_codes = 2;
pub const EXIT_BAD_USAGE: exit_codes = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: *mut sed_cmd,
    pub v_allocated: size_t,
    pub v_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_cmd {
    pub a1: *mut addr,
    pub a2: *mut addr,
    pub range_state: addr_state,
    pub addr_bang: libc::c_char,
    pub cmd: libc::c_char,
    pub x: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub cmd_txt: text_buf,
    pub int_arg: libc::c_int,
    pub jump_index: countT,
    pub readcmd: readcmd,
    pub cmd_subst: *mut subst,
    pub outf: *mut output,
    pub inf: *mut output,
    pub translate: *mut libc::c_uchar,
    pub translatemb: *mut *mut libc::c_char,
    pub label_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output {
    pub name: *mut libc::c_char,
    pub missing_newline: bool,
    pub fp: *mut FILE,
    pub link: *mut output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct subst {
    pub regx: *mut regex,
    pub replacement: *mut replacement,
    pub numb: countT,
    pub outf: *mut output,
    #[bitfield(name = "global", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "print", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "eval", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "max_id", ty = "libc::c_uint", bits = "4..=7")]
    pub global_print_eval_max_id: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replacement {
    pub prefix: *mut libc::c_char,
    pub prefix_length: size_t,
    pub subst_id: libc::c_int,
    pub repl_type: replacement_types,
    pub next: *mut replacement,
}
pub type replacement_types = libc::c_uint;
pub const REPL_LOWERCASE_LOWERCASE: replacement_types = 10;
pub const REPL_LOWERCASE_UPPERCASE: replacement_types = 9;
pub const REPL_UPPERCASE_LOWERCASE: replacement_types = 6;
pub const REPL_UPPERCASE_UPPERCASE: replacement_types = 5;
pub const REPL_MODIFIERS: replacement_types = 12;
pub const REPL_LOWERCASE_FIRST: replacement_types = 8;
pub const REPL_UPPERCASE_FIRST: replacement_types = 4;
pub const REPL_LOWERCASE: replacement_types = 2;
pub const REPL_UPPERCASE: replacement_types = 1;
pub const REPL_ASIS: replacement_types = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: regex_t,
    pub flags: libc::c_int,
    pub sz: size_t,
    pub dfa: *mut dfa,
    pub begline: bool,
    pub endline: bool,
    pub re: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readcmd {
    pub fname: *mut libc::c_char,
    pub append: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text_buf {
    pub text: *mut libc::c_char,
    pub text_length: size_t,
}
pub type addr_state = libc::c_uint;
pub const RANGE_CLOSED: addr_state = 2;
pub const RANGE_ACTIVE: addr_state = 1;
pub const RANGE_INACTIVE: addr_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr {
    pub addr_type: addr_types,
    pub addr_number: countT,
    pub addr_step: countT,
    pub addr_regex: *mut regex,
}
pub type addr_types = libc::c_uint;
pub const ADDR_IS_LAST: addr_types = 6;
pub const ADDR_IS_STEP_MOD: addr_types = 5;
pub const ADDR_IS_STEP: addr_types = 4;
pub const ADDR_IS_NUM_MOD: addr_types = 3;
pub const ADDR_IS_NUM: addr_types = 2;
pub const ADDR_IS_REGEX: addr_types = 1;
pub const ADDR_IS_NULL: addr_types = 0;
pub type text_types = libc::c_uint;
pub const TEXT_REGEX: text_types = 2;
pub const TEXT_REPLACEMENT: text_types = 1;
pub const TEXT_BUFFER: text_types = 0;
pub type posixicity_types = libc::c_uint;
pub const POSIXLY_BASIC: posixicity_types = 2;
pub const POSIXLY_CORRECT: posixicity_types = 1;
pub const POSIXLY_EXTENDED: posixicity_types = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_info {
    pub base: *const libc::c_uchar,
    pub cur: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error_info {
    pub name: *const libc::c_char,
    pub line: countT,
    pub string_expr_count: countT,
}
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct special_files {
    pub outf: output,
    pub pfp: *mut *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_label {
    pub v_index: countT,
    pub name: *mut libc::c_char,
    pub err_info: error_info,
    pub next: *mut sed_label,
}
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut my_stdin: *mut FILE = 0 as *const FILE as *mut FILE;
static mut my_stdout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut my_stderr: *mut FILE = 0 as *const FILE as *mut FILE;
static mut special_files: [special_files; 4] = unsafe {
    [
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: b"/dev/stdin\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        missing_newline: 0 as libc::c_int != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: &my_stdin as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: b"/dev/stdout\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        missing_newline: 0 as libc::c_int != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: &my_stdout as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: b"/dev/stderr\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        missing_newline: 0 as libc::c_int != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: &my_stderr as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: 0 as *const libc::c_char as *mut libc::c_char,
                        missing_newline: 0 as libc::c_int != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: 0 as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
    ]
};
static mut prog: prog_info = prog_info {
    base: 0 as *const libc::c_uchar,
    cur: 0 as *const libc::c_uchar,
    end: 0 as *const libc::c_uchar,
    file: 0 as *const FILE as *mut FILE,
};
static mut cur_input: error_info = error_info {
    name: 0 as *const libc::c_char,
    line: 0,
    string_expr_count: 0,
};
static mut jumps: *mut sed_label = 0 as *const sed_label as *mut sed_label;
static mut labels: *mut sed_label = 0 as *const sed_label as *mut sed_label;
static mut first_script: bool = 1 as libc::c_int != 0;
static mut pending_text: *mut buffer = 0 as *const buffer as *mut buffer;
static mut old_text_buf: *mut text_buf = 0 as *const text_buf as *mut text_buf;
static mut blocks: *mut sed_label = 0 as *const sed_label as *mut sed_label;
static mut obs: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_2 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_1 { plain: None },
    freefun: C2RustUnnamed_0 { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut errors: [libc::c_char; 897] = unsafe {
    *::std::mem::transmute::<
        &[u8; 897],
        &[libc::c_char; 897],
    >(
        b"multiple `!'s\0unexpected `,'\0invalid usage of +N or ~N as first address\0unmatched `{'\0unexpected `}'\0extra characters after command\0expected \\ after `a', `c' or `i'\0`}' doesn't want any addresses\0: doesn't want any addresses\0comments don't accept any addresses\0missing command\0command only uses one address\0unterminated address regex\0unterminated `s' command\0unterminated `y' command\0unknown option to `s'\0multiple `p' options to `s' command\0multiple `g' options to `s' command\0multiple number options to `s' command\0number option to `s' command may not be zero\0strings for `y' command are different lengths\0delimiter character is not a single-byte character\0expected newer version of sed\0invalid usage of line address 0\0unknown command: `%c'\0incomplete command\0\":\" lacks a label\0recursive escaping after \\c not allowed\0e/r/w commands disabled in sandbox mode\0missing filename in r/R/w/W commands\0",
    )
};
static mut file_read: *mut output = 0 as *const output as *mut output;
static mut file_write: *mut output = 0 as *const output as *mut output;
unsafe extern "C" fn bad_command(mut ch: libc::c_char) {
    let mut msg: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        errors
            .as_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong as isize,
            )
            .offset(
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as isize,
            ),
        5 as libc::c_int,
    );
    let mut unknown_cmd: *mut libc::c_char = xmalloc(strlen(msg)) as *mut libc::c_char;
    sprintf(unknown_cmd, msg, ch as libc::c_int);
    bad_prog(unknown_cmd);
}
pub unsafe extern "C" fn bad_prog(mut why: *const libc::c_char) {
    if !(cur_input.name).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: file %s line %lu: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            cur_input.name,
            cur_input.line,
            why,
        );
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: -e expression #%lu, char %lu: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            cur_input.string_expr_count,
            (prog.cur).offset_from(prog.base) as libc::c_long as libc::c_ulong,
            why,
        );
    }
    exit(EXIT_BAD_USAGE as libc::c_int);
}
unsafe extern "C" fn inchar() -> libc::c_int {
    let mut ch: libc::c_int = -(1 as libc::c_int);
    if !(prog.cur).is_null() {
        if prog.cur < prog.end {
            let fresh1 = prog.cur;
            prog.cur = (prog.cur).offset(1);
            ch = *fresh1 as libc::c_int;
        }
    } else if !(prog.file).is_null() {
        if feof_unlocked(prog.file) == 0 {
            ch = getc_unlocked(prog.file);
        }
    }
    if ch == '\n' as i32 {
        cur_input.line = (cur_input.line).wrapping_add(1);
        cur_input.line;
    }
    return ch;
}
unsafe extern "C" fn savchar(mut ch: libc::c_int) {
    if ch == -(1 as libc::c_int) {
        return;
    }
    if ch == '\n' as i32 && cur_input.line > 0 as libc::c_int as libc::c_ulong {
        cur_input.line = (cur_input.line).wrapping_sub(1);
        cur_input.line;
    }
    if !(prog.cur).is_null() {
        if prog.cur <= prog.base
            || {
                prog.cur = (prog.cur).offset(-1);
                *prog.cur as libc::c_int != ch
            }
        {
            panic(
                b"Called savchar with unexpected pushback (%x)\0" as *const u8
                    as *const libc::c_char,
                ch as libc::c_uint,
            );
        }
    } else {
        ungetc(ch, prog.file);
    };
}
unsafe extern "C" fn in_nonblank() -> libc::c_int {
    let mut ch: libc::c_int = 0;
    loop {
        ch = inchar();
        if !(1 as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            break;
        }
    }
    return ch;
}
unsafe extern "C" fn read_end_of_cmd() {
    let ch: libc::c_int = in_nonblank();
    if ch == '}' as i32 || ch == '#' as i32 {
        savchar(ch);
    } else if ch != -(1 as libc::c_int) && ch != '\n' as i32 && ch != ';' as i32 {
        bad_prog(
            dcgettext(
                0 as *const libc::c_char,
                errors
                    .as_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    ),
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn in_integer(mut ch: libc::c_int) -> countT {
    let mut num: countT = 0 as libc::c_int as countT;
    while 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(ch as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        num = num
            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(ch as libc::c_ulong)
            .wrapping_sub('0' as i32 as libc::c_ulong);
        ch = inchar();
    }
    savchar(ch);
    return num;
}
unsafe extern "C" fn add_then_next(
    mut b: *mut buffer,
    mut ch: libc::c_int,
) -> libc::c_int {
    add1_buffer(b, ch);
    return inchar();
}
unsafe extern "C" fn convert_number(
    mut result: *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufend: *const libc::c_char,
    mut base: libc::c_int,
) -> *mut libc::c_char {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 1 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = buf.offset(1 as libc::c_int as isize);
    while p < bufend as *mut libc::c_char && max <= 255 as libc::c_int {
        let mut d: libc::c_int = -(1 as libc::c_int);
        match *p as libc::c_int {
            48 => {
                d = 0 as libc::c_int;
            }
            49 => {
                d = 0x1 as libc::c_int;
            }
            50 => {
                d = 0x2 as libc::c_int;
            }
            51 => {
                d = 0x3 as libc::c_int;
            }
            52 => {
                d = 0x4 as libc::c_int;
            }
            53 => {
                d = 0x5 as libc::c_int;
            }
            54 => {
                d = 0x6 as libc::c_int;
            }
            55 => {
                d = 0x7 as libc::c_int;
            }
            56 => {
                d = 0x8 as libc::c_int;
            }
            57 => {
                d = 0x9 as libc::c_int;
            }
            65 | 97 => {
                d = 0xa as libc::c_int;
            }
            66 | 98 => {
                d = 0xb as libc::c_int;
            }
            67 | 99 => {
                d = 0xc as libc::c_int;
            }
            68 | 100 => {
                d = 0xd as libc::c_int;
            }
            69 | 101 => {
                d = 0xe as libc::c_int;
            }
            70 | 102 => {
                d = 0xf as libc::c_int;
            }
            _ => {}
        }
        if d < 0 as libc::c_int || base <= d {
            break;
        }
        n = n * base + d;
        p = p.offset(1);
        p;
        max *= base;
    }
    if p == buf.offset(1 as libc::c_int as isize) {
        *result = *buf;
    } else {
        *result = n as libc::c_char;
    }
    return p;
}
unsafe extern "C" fn read_filename() -> *mut buffer {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: libc::c_int = 0;
    if sandbox {
        bad_prog(
            dcgettext(
                0 as *const libc::c_char,
                errors
                    .as_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                            as isize,
                    ),
                5 as libc::c_int,
            ),
        );
    }
    b = init_buffer();
    ch = in_nonblank();
    while ch != -(1 as libc::c_int) && ch != '\n' as i32 {
        ch = add_then_next(b, ch);
    }
    add1_buffer(b, '\0' as i32);
    return b;
}
unsafe extern "C" fn get_openfile(
    mut file_ptrs: *mut *mut output,
    mut mode: *const libc::c_char,
    mut fail: libc::c_int,
) -> *mut output {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut output = 0 as *mut output;
    b = read_filename();
    file_name = get_buffer(b);
    if strlen(file_name) == 0 as libc::c_int as libc::c_ulong {
        bad_prog(
            dcgettext(
                0 as *const libc::c_char,
                errors
                    .as_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                            as isize,
                    ),
                5 as libc::c_int,
            ),
        );
    }
    p = *file_ptrs;
    while !p.is_null() {
        if strcmp((*p).name, file_name) == 0 as libc::c_int {
            break;
        }
        p = (*p).link;
    }
    if posixicity as libc::c_uint == POSIXLY_EXTENDED as libc::c_int as libc::c_uint {
        let mut special: *mut special_files = special_files.as_mut_ptr();
        my_stdin = stdin;
        my_stdout = stdout;
        my_stderr = stderr;
        special = special_files.as_mut_ptr();
        while !((*special).outf.name).is_null() {
            if strcmp((*special).outf.name, file_name) == 0 as libc::c_int {
                (*special).outf.fp = *(*special).pfp;
                free_buffer(b);
                return &mut (*special).outf;
            }
            special = special.offset(1);
            special;
        }
    }
    if p.is_null() {
        p = ({
            let mut __h: *mut obstack = &mut obs as *mut obstack;
            let mut __o: *mut obstack = __h;
            let mut __len: size_t = (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<output>() as libc::c_ulong);
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            {
                let mut __o1: *mut obstack = __h;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut libc::c_char {
                    (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                }
                (*__o1)
                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                    as libc::c_ulong)
                    < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    (*__o1).object_base
                } else {
                    0 as *mut libc::c_char
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                },
                            ) as libc::c_long as libc::c_ulong)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
                    > ((*__o1).chunk_limit)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            }
        }) as *mut output;
        (*p).name = xstrdup(file_name);
        (*p).fp = ck_fopen((*p).name, mode, fail);
        (*p).missing_newline = 0 as libc::c_int != 0;
        (*p).link = *file_ptrs;
        *file_ptrs = p;
    }
    free_buffer(b);
    return p;
}
unsafe extern "C" fn next_cmd_entry(mut vectorp: *mut *mut vector) -> *mut sed_cmd {
    let mut cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut v: *mut vector = 0 as *mut vector;
    v = *vectorp;
    if (*v).v_length == (*v).v_allocated {
        (*v)
            .v_allocated = ((*v).v_allocated as libc::c_ulong)
            .wrapping_add(40 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*v)
            .v = xnrealloc(
            (*v).v as *mut libc::c_void,
            (*v).v_allocated,
            ::std::mem::size_of::<sed_cmd>() as libc::c_ulong,
        ) as *mut sed_cmd;
    }
    cmd = ((*v).v).offset((*v).v_length as isize);
    (*cmd).a1 = 0 as *mut addr;
    (*cmd).a2 = 0 as *mut addr;
    (*cmd).range_state = RANGE_INACTIVE;
    (*cmd).addr_bang = 0 as libc::c_int as libc::c_char;
    (*cmd).cmd = '\0' as i32 as libc::c_char;
    *vectorp = v;
    return cmd;
}
unsafe extern "C" fn snarf_char_class(
    mut b: *mut buffer,
    mut cur_stat: *mut mbstate_t,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut state: libc::c_int = 0 as libc::c_int;
    let mut delim: libc::c_int = 0;
    ch = inchar();
    if ch == '^' as i32 {
        ch = add_then_next(b, ch);
    }
    if ch == ']' as i32 {
        ch = add_then_next(b, ch);
    }
    let mut current_block_16: u64;
    loop {
        let mb_char: libc::c_int = if mb_cur_max == 1 as libc::c_int {
            0 as libc::c_int
        } else {
            is_mb_char(ch, cur_stat)
        };
        match ch {
            -1 | 10 => return ch,
            46 | 58 | 61 => {
                if mb_char != 0 {
                    current_block_16 = 15427931788582360902;
                } else if state == 1 as libc::c_int {
                    delim = ch;
                    state = 2 as libc::c_int;
                    current_block_16 = 15427931788582360902;
                } else if state == 2 as libc::c_int && ch == delim {
                    state = 3 as libc::c_int;
                    current_block_16 = 15427931788582360902;
                } else {
                    current_block_16 = 2719512138335094285;
                }
            }
            91 => {
                if mb_char != 0 {
                    current_block_16 = 15427931788582360902;
                } else {
                    if state == 0 as libc::c_int {
                        state = 1 as libc::c_int;
                    }
                    current_block_16 = 15427931788582360902;
                }
            }
            93 => {
                if mb_char != 0 {
                    current_block_16 = 15427931788582360902;
                } else {
                    if state == 0 as libc::c_int || state == 1 as libc::c_int {
                        return ch
                    } else if state == 3 as libc::c_int {
                        state = 0 as libc::c_int;
                    }
                    current_block_16 = 2719512138335094285;
                }
            }
            _ => {
                current_block_16 = 2719512138335094285;
            }
        }
        match current_block_16 {
            2719512138335094285 => {
                state &= !(1 as libc::c_int);
            }
            _ => {}
        }
        ch = add_then_next(b, ch);
    };
}
unsafe extern "C" fn match_slash(
    mut slash: libc::c_int,
    mut regex: libc::c_int,
) -> *mut buffer {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: libc::c_int = 0;
    let mut cur_stat: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    if if mb_cur_max == 1 as libc::c_int {
        0 as libc::c_int
    } else {
        is_mb_char(slash, &mut cur_stat)
    } != 0
    {
        bad_prog(
            errors
                .as_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong as isize,
                )
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as isize,
                ),
        );
    }
    memset(
        &mut cur_stat as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    b = init_buffer();
    loop {
        ch = inchar();
        if !(ch != -(1 as libc::c_int) && ch != '\n' as i32) {
            break;
        }
        let mb_char: libc::c_int = if mb_cur_max == 1 as libc::c_int {
            0 as libc::c_int
        } else {
            is_mb_char(ch, &mut cur_stat)
        };
        if mb_char == 0 {
            if ch == slash {
                return b
            } else if ch == '\\' as i32 {
                ch = inchar();
                if ch == -(1 as libc::c_int) {
                    break;
                }
                if ch != '\n' as i32 && (ch != slash || regex == 0 && ch == '&' as i32) {
                    add1_buffer(b, '\\' as i32);
                }
            } else if ch == '[' as i32 && regex != 0 {
                add1_buffer(b, ch);
                ch = snarf_char_class(b, &mut cur_stat);
                if ch != ']' as i32 {
                    break;
                }
            }
        }
        add1_buffer(b, ch);
    }
    if ch == '\n' as i32 {
        savchar(ch);
    }
    free_buffer(b);
    return 0 as *mut buffer;
}
unsafe extern "C" fn mark_subst_opts(mut cmd: *mut subst) -> libc::c_int {
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0;
    (*cmd).set_global(0 as libc::c_int as libc::c_uint);
    (*cmd).set_print(0 as libc::c_int as libc::c_uint);
    (*cmd).set_eval(0 as libc::c_int as libc::c_uint);
    (*cmd).numb = 0 as libc::c_int as countT;
    (*cmd).outf = 0 as *mut output;
    loop {
        let mut current_block_33: u64;
        ch = in_nonblank();
        match ch {
            105 => {
                current_block_33 = 16222770899728866709;
            }
            73 => {
                current_block_33 = 16222770899728866709;
            }
            109 | 77 => {
                if posixicity as libc::c_uint
                    == POSIXLY_BASIC as libc::c_int as libc::c_uint
                {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                flags |= (1 as libc::c_int) << 2 as libc::c_int;
                current_block_33 = 4090602189656566074;
            }
            101 => {
                if posixicity as libc::c_uint
                    == POSIXLY_BASIC as libc::c_int as libc::c_uint
                {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cmd).set_eval(1 as libc::c_int as libc::c_uint);
                current_block_33 = 4090602189656566074;
            }
            112 => {
                if (*cmd).print() != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cmd)
                    .set_print(
                        (*cmd).print()
                            | ((1 as libc::c_int) << (*cmd).eval() as libc::c_int)
                                as libc::c_uint,
                    );
                current_block_33 = 4090602189656566074;
            }
            103 => {
                if (*cmd).global() != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cmd).set_global(1 as libc::c_int as libc::c_uint);
                current_block_33 = 4090602189656566074;
            }
            119 => {
                (*cmd)
                    .outf = get_openfile(&mut file_write, write_mode, 1 as libc::c_int);
                return flags;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if (*cmd).numb != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cmd).numb = in_integer(ch);
                if (*cmd).numb == 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block_33 = 4090602189656566074;
            }
            125 | 35 => {
                savchar(ch);
                current_block_33 = 10916778590618914605;
            }
            -1 | 10 | 59 => {
                current_block_33 = 10916778590618914605;
            }
            13 => {
                if inchar() == '\n' as i32 {
                    return flags;
                }
                current_block_33 = 9989330386823999577;
            }
            _ => {
                current_block_33 = 9989330386823999577;
            }
        }
        match current_block_33 {
            16222770899728866709 => {
                if posixicity as libc::c_uint
                    == POSIXLY_BASIC as libc::c_int as libc::c_uint
                {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                flags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            10916778590618914605 => return flags,
            9989330386823999577 => {
                bad_prog(
                    dcgettext(
                        0 as *const libc::c_char,
                        errors
                            .as_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                    as isize,
                            ),
                        5 as libc::c_int,
                    ),
                );
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn read_label() -> *mut libc::c_char {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    b = init_buffer();
    ch = in_nonblank();
    while ch != -(1 as libc::c_int) && ch != '\n' as i32
        && !(1 as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0)
        && ch != ';' as i32 && ch != '}' as i32 && ch != '#' as i32
    {
        ch = add_then_next(b, ch);
    }
    savchar(ch);
    add1_buffer(b, '\0' as i32);
    ret = xstrdup(get_buffer(b));
    free_buffer(b);
    return ret;
}
unsafe extern "C" fn setup_label(
    mut list: *mut sed_label,
    mut idx: countT,
    mut name: *mut libc::c_char,
    mut err_info: *const error_info,
) -> *mut sed_label {
    let mut ret: *mut sed_label = ({
        let mut __h: *mut obstack = &mut obs as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<sed_label>() as libc::c_ulong);
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        {
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }
    }) as *mut sed_label;
    (*ret).v_index = idx;
    (*ret).name = name;
    if !err_info.is_null() {
        memcpy(
            &mut (*ret).err_info as *mut error_info as *mut libc::c_void,
            err_info as *const libc::c_void,
            ::std::mem::size_of::<error_info>() as libc::c_ulong,
        );
    }
    (*ret).next = list;
    return ret;
}
unsafe extern "C" fn release_label(mut list_head: *mut sed_label) -> *mut sed_label {
    let mut ret: *mut sed_label = 0 as *mut sed_label;
    if list_head.is_null() {
        return 0 as *mut sed_label;
    }
    ret = (*list_head).next;
    rpl_free((*list_head).name as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn new_replacement(
    mut text: *mut libc::c_char,
    mut length: size_t,
    mut type_0: replacement_types,
) -> *mut replacement {
    let mut r: *mut replacement = ({
        let mut __h: *mut obstack = &mut obs as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<replacement>() as libc::c_ulong);
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        {
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }
    }) as *mut replacement;
    (*r).prefix = text;
    (*r).prefix_length = length;
    (*r).subst_id = -(1 as libc::c_int);
    (*r).repl_type = type_0;
    return r;
}
unsafe extern "C" fn setup_replacement(
    mut sub: *mut subst,
    mut text: *const libc::c_char,
    mut length: size_t,
) {
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut repl_type: replacement_types = REPL_ASIS;
    let mut save_type: replacement_types = REPL_ASIS;
    let mut root: replacement = replacement {
        prefix: 0 as *mut libc::c_char,
        prefix_length: 0,
        subst_id: 0,
        repl_type: REPL_ASIS,
        next: 0 as *mut replacement,
    };
    let mut tail: *mut replacement = 0 as *mut replacement;
    (*sub).set_max_id(0 as libc::c_int as libc::c_uint);
    base = xmemdup(
        text as *const libc::c_void,
        length.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    length = normalize_text(base, length, TEXT_REPLACEMENT);
    text_end = base.offset(length as isize);
    tail = &mut root;
    p = base;
    while p < text_end {
        if *p as libc::c_int == '\\' as i32 {
            (*tail)
                .next = new_replacement(
                base,
                p.offset_from(base) as libc::c_long as size_t,
                repl_type,
            );
            tail = (*tail).next;
            repl_type = save_type;
            p = p.offset(1);
            p;
            if p == text_end {
                (*tail).prefix_length = ((*tail).prefix_length).wrapping_add(1);
                (*tail).prefix_length;
            } else if posixicity as libc::c_uint
                == POSIXLY_BASIC as libc::c_int as libc::c_uint
                && !(1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*p as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                *p.offset(-(1 as libc::c_int) as isize) = *p;
                (*tail).prefix_length = ((*tail).prefix_length).wrapping_add(1);
                (*tail).prefix_length;
            } else {
                match *p as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        (*tail).subst_id = *p as libc::c_int - '0' as i32;
                        if ((*sub).max_id() as libc::c_int) < (*tail).subst_id {
                            (*sub).set_max_id((*tail).subst_id as libc::c_uint);
                        }
                    }
                    76 => {
                        repl_type = REPL_LOWERCASE;
                        save_type = REPL_LOWERCASE;
                    }
                    85 => {
                        repl_type = REPL_UPPERCASE;
                        save_type = REPL_UPPERCASE;
                    }
                    69 => {
                        repl_type = REPL_ASIS;
                        save_type = REPL_ASIS;
                    }
                    108 => {
                        save_type = repl_type;
                        repl_type = ::std::mem::transmute::<
                            libc::c_uint,
                            replacement_types,
                        >(
                            repl_type as libc::c_uint
                                | REPL_LOWERCASE_FIRST as libc::c_int as libc::c_uint,
                        );
                    }
                    117 => {
                        save_type = repl_type;
                        repl_type = ::std::mem::transmute::<
                            libc::c_uint,
                            replacement_types,
                        >(
                            repl_type as libc::c_uint
                                | REPL_UPPERCASE_FIRST as libc::c_int as libc::c_uint,
                        );
                    }
                    _ => {
                        *p.offset(-(1 as libc::c_int) as isize) = *p;
                        (*tail).prefix_length = ((*tail).prefix_length).wrapping_add(1);
                        (*tail).prefix_length;
                    }
                }
            }
            base = p.offset(1 as libc::c_int as isize);
        } else if *p as libc::c_int == '&' as i32 {
            (*tail)
                .next = new_replacement(
                base,
                p.offset_from(base) as libc::c_long as size_t,
                repl_type,
            );
            tail = (*tail).next;
            repl_type = save_type;
            (*tail).subst_id = 0 as libc::c_int;
            base = p.offset(1 as libc::c_int as isize);
        }
        p = p.offset(1);
        p;
    }
    if base < text_end {
        (*tail)
            .next = new_replacement(
            base,
            text_end.offset_from(base) as libc::c_long as size_t,
            repl_type,
        );
        tail = (*tail).next;
    }
    (*tail).next = 0 as *mut replacement;
    (*sub).replacement = root.next;
}
unsafe extern "C" fn read_text(mut buf: *mut text_buf, mut leadin_ch: libc::c_int) {
    let mut ch: libc::c_int = 0;
    if !buf.is_null() {
        if !pending_text.is_null() {
            free_buffer(pending_text);
        }
        pending_text = init_buffer();
        (*buf).text = 0 as *mut libc::c_char;
        (*buf).text_length = 0 as libc::c_int as size_t;
        old_text_buf = buf;
    }
    if leadin_ch == -(1 as libc::c_int) {
        return;
    }
    if leadin_ch != '\n' as i32 {
        add1_buffer(pending_text, leadin_ch);
    }
    ch = inchar();
    while ch != -(1 as libc::c_int) && ch != '\n' as i32 {
        if ch == '\\' as i32 {
            ch = inchar();
            if ch != -(1 as libc::c_int) {
                add1_buffer(pending_text, '\\' as i32);
            }
        }
        if ch == -(1 as libc::c_int) {
            add1_buffer(pending_text, '\n' as i32);
            return;
        }
        ch = add_then_next(pending_text, ch);
    }
    add1_buffer(pending_text, '\n' as i32);
    if buf.is_null() {
        buf = old_text_buf;
    }
    (*buf)
        .text_length = normalize_text(
        get_buffer(pending_text),
        size_buffer(pending_text),
        TEXT_BUFFER,
    );
    (*buf)
        .text = xmemdup(
        get_buffer(pending_text) as *const libc::c_void,
        ((*buf).text_length)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    free_buffer(pending_text);
    pending_text = 0 as *mut buffer;
}
unsafe extern "C" fn compile_address(mut addr: *mut addr, mut ch: libc::c_int) -> bool {
    (*addr).addr_type = ADDR_IS_NULL;
    (*addr).addr_step = 0 as libc::c_int as countT;
    (*addr).addr_number = !(0 as libc::c_int as countT);
    (*addr).addr_regex = 0 as *mut regex;
    if ch == '/' as i32 || ch == '\\' as i32 {
        let mut flags: libc::c_int = 0 as libc::c_int;
        let mut b: *mut buffer = 0 as *mut buffer;
        (*addr).addr_type = ADDR_IS_REGEX;
        if ch == '\\' as i32 {
            ch = inchar();
        }
        b = match_slash(ch, 1 as libc::c_int);
        if b.is_null() {
            bad_prog(
                dcgettext(
                    0 as *const libc::c_char,
                    errors
                        .as_ptr()
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                as isize,
                        ),
                    5 as libc::c_int,
                ),
            );
        }
        loop {
            's_90: {
                let mut current_block_15: u64;
                ch = in_nonblank();
                if !(posixicity as libc::c_uint
                    == POSIXLY_BASIC as libc::c_int as libc::c_uint)
                {
                    match ch {
                        73 => {
                            current_block_15 = 2822874994958576623;
                            match current_block_15 {
                                2113773419182433576 => {
                                    flags |= (1 as libc::c_int) << 2 as libc::c_int;
                                }
                                _ => {
                                    flags |= (1 as libc::c_int) << 1 as libc::c_int;
                                }
                            }
                            break 's_90;
                        }
                        77 => {
                            current_block_15 = 2113773419182433576;
                            match current_block_15 {
                                2113773419182433576 => {
                                    flags |= (1 as libc::c_int) << 2 as libc::c_int;
                                }
                                _ => {
                                    flags |= (1 as libc::c_int) << 1 as libc::c_int;
                                }
                            }
                            break 's_90;
                        }
                        _ => {}
                    }
                }
                savchar(ch);
                (*addr).addr_regex = compile_regex(b, flags, 0 as libc::c_int);
                free_buffer(b);
                return 1 as libc::c_int != 0;
            }
        }
    } else if 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(ch as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*addr).addr_number = in_integer(ch);
        (*addr).addr_type = ADDR_IS_NUM;
        ch = in_nonblank();
        if ch != '~' as i32
            || posixicity as libc::c_uint == POSIXLY_BASIC as libc::c_int as libc::c_uint
        {
            savchar(ch);
        } else {
            let mut step: countT = in_integer(in_nonblank());
            if step > 0 as libc::c_int as libc::c_ulong {
                (*addr).addr_step = step;
                (*addr).addr_type = ADDR_IS_NUM_MOD;
            }
        }
    } else if (ch == '+' as i32 || ch == '~' as i32)
        && posixicity as libc::c_uint != POSIXLY_BASIC as libc::c_int as libc::c_uint
    {
        (*addr).addr_step = in_integer(in_nonblank());
        if !((*addr).addr_step == 0 as libc::c_int as libc::c_ulong) {
            if ch == '+' as i32 {
                (*addr).addr_type = ADDR_IS_STEP;
            } else {
                (*addr).addr_type = ADDR_IS_STEP_MOD;
            }
        }
    } else if ch == '$' as i32 {
        (*addr).addr_type = ADDR_IS_LAST;
    } else {
        return 0 as libc::c_int != 0
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn compile_program(mut vector: *mut vector) -> *mut vector {
    let mut cur_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: libc::c_int = 0;
    if vector.is_null() {
        vector = (if ::std::mem::size_of::<vector>() as libc::c_ulong
            == 1 as libc::c_int as libc::c_ulong
        {
            xzalloc(1 as libc::c_int as size_t)
        } else {
            xcalloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<vector>() as libc::c_ulong,
            )
        }) as *mut vector;
        (*vector).v = 0 as *mut sed_cmd;
        (*vector).v_allocated = 0 as libc::c_int as size_t;
        (*vector).v_length = 0 as libc::c_int as size_t;
        _obstack_begin(
            &mut obs,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(xzalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    if !pending_text.is_null() {
        read_text(0 as *mut text_buf, '\n' as i32);
    }
    let mut current_block_184: u64;
    loop {
        let mut a: addr = addr {
            addr_type: ADDR_IS_NULL,
            addr_number: 0,
            addr_step: 0,
            addr_regex: 0 as *mut regex,
        };
        loop {
            ch = inchar();
            if !(ch == ';' as i32
                || 1 as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                break;
            }
        }
        if ch == -(1 as libc::c_int) {
            break;
        }
        cur_cmd = next_cmd_entry(&mut vector);
        if compile_address(&mut a, ch) {
            if a.addr_type as libc::c_uint == ADDR_IS_STEP as libc::c_int as libc::c_uint
                || a.addr_type as libc::c_uint
                    == ADDR_IS_STEP_MOD as libc::c_int as libc::c_uint
            {
                bad_prog(
                    dcgettext(
                        0 as *const libc::c_char,
                        errors
                            .as_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            ),
                        5 as libc::c_int,
                    ),
                );
            }
            (*cur_cmd)
                .a1 = xmemdup(
                &mut a as *mut addr as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<addr>() as libc::c_ulong),
            ) as *mut addr;
            ch = in_nonblank();
            if ch == ',' as i32 {
                if !compile_address(&mut a, in_nonblank()) {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cur_cmd)
                    .a2 = xmemdup(
                    &mut a as *mut addr as *const libc::c_void,
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<addr>() as libc::c_ulong),
                ) as *mut addr;
                ch = in_nonblank();
            }
            if (*(*cur_cmd).a1).addr_type as libc::c_uint
                == ADDR_IS_NUM as libc::c_int as libc::c_uint
                && (*(*cur_cmd).a1).addr_number == 0 as libc::c_int as libc::c_ulong
                && (((*cur_cmd).a2).is_null() && ch != 'r' as i32
                    || !((*cur_cmd).a2).is_null()
                        && (*(*cur_cmd).a2).addr_type as libc::c_uint
                            != ADDR_IS_REGEX as libc::c_int as libc::c_uint
                    || posixicity as libc::c_uint
                        == POSIXLY_BASIC as libc::c_int as libc::c_uint)
            {
                bad_prog(
                    dcgettext(
                        0 as *const libc::c_char,
                        errors
                            .as_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                    as isize,
                            ),
                        5 as libc::c_int,
                    ),
                );
            }
        }
        if ch == '!' as i32 {
            (*cur_cmd).addr_bang = 1 as libc::c_int as libc::c_char;
            ch = in_nonblank();
            if ch == '!' as i32 {
                bad_prog(
                    dcgettext(
                        0 as *const libc::c_char,
                        errors.as_ptr(),
                        5 as libc::c_int,
                    ),
                );
            }
        }
        if posixicity as libc::c_uint == POSIXLY_BASIC as libc::c_int as libc::c_uint {
            let mut current_block_35: u64;
            match ch {
                101 | 70 | 118 | 122 | 76 | 81 | 84 | 82 | 87 => {
                    bad_command(ch as libc::c_char);
                    current_block_35 = 13165815877891111323;
                }
                97 | 105 | 108 | 61 | 114 => {
                    current_block_35 = 13165815877891111323;
                }
                _ => {
                    current_block_35 = 14832935472441733737;
                }
            }
            match current_block_35 {
                13165815877891111323 => {
                    if !((*cur_cmd).a2).is_null() {
                        bad_prog(
                            dcgettext(
                                0 as *const libc::c_char,
                                errors
                                    .as_ptr()
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as isize,
                                    ),
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                _ => {}
            }
        }
        (*cur_cmd).cmd = ch as libc::c_char;
        match ch {
            35 => {
                if !((*cur_cmd).a1).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                ch = inchar();
                if ch == 'n' as i32 && first_script as libc::c_int != 0
                    && cur_input.line < 2 as libc::c_int as libc::c_ulong
                {
                    if !(prog.base).is_null()
                        && prog.cur == (prog.base).offset(2 as libc::c_int as isize)
                        || !(prog.file).is_null() && (prog.base).is_null()
                            && 2 as libc::c_int as libc::c_long == ftell(prog.file)
                    {
                        no_default_output = 1 as libc::c_int != 0;
                    }
                }
                while ch != -(1 as libc::c_int) && ch != '\n' as i32 {
                    ch = inchar();
                }
                continue;
            }
            118 => {
                let mut version: *mut libc::c_char = read_label();
                let mut compared_version: *const libc::c_char = 0 as *const libc::c_char;
                compared_version = if *version as libc::c_int == '\0' as i32 {
                    b"4.0\0" as *const u8 as *const libc::c_char
                } else {
                    version as *const libc::c_char
                };
                if strverscmp(
                    compared_version,
                    b"4.9\0" as *const u8 as *const libc::c_char,
                ) > 0 as libc::c_int
                {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                rpl_free(version as *mut libc::c_void);
                posixicity = POSIXLY_EXTENDED;
                continue;
            }
            123 => {
                blocks = setup_label(
                    blocks,
                    (*vector).v_length,
                    0 as *mut libc::c_char,
                    &mut cur_input,
                );
                (*cur_cmd)
                    .addr_bang = ((*cur_cmd).addr_bang == 0) as libc::c_int
                    as libc::c_char;
                current_block_184 = 6957654774345280688;
            }
            125 => {
                if blocks.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                if !((*cur_cmd).a1).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                read_end_of_cmd();
                (*((*vector).v).offset((*blocks).v_index as isize))
                    .x
                    .jump_index = (*vector).v_length;
                blocks = release_label(blocks);
                current_block_184 = 6957654774345280688;
            }
            101 => {
                if sandbox {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                ch = in_nonblank();
                if ch == -(1 as libc::c_int) || ch == '\n' as i32 {
                    (*cur_cmd).x.cmd_txt.text_length = 0 as libc::c_int as size_t;
                    current_block_184 = 6957654774345280688;
                } else {
                    current_block_184 = 16082342845391650887;
                }
            }
            97 | 105 | 99 => {
                ch = in_nonblank();
                current_block_184 = 16082342845391650887;
            }
            58 => {
                if !((*cur_cmd).a1).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                let mut label: *mut libc::c_char = read_label();
                if *label == 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                labels = setup_label(
                    labels,
                    (*vector).v_length,
                    label,
                    0 as *const error_info,
                );
                if debug {
                    (*cur_cmd).x.label_name = strdup(label);
                }
                current_block_184 = 6957654774345280688;
            }
            84 | 98 | 116 => {
                jumps = setup_label(
                    jumps,
                    (*vector).v_length,
                    read_label(),
                    0 as *const error_info,
                );
                current_block_184 = 6957654774345280688;
            }
            81 | 113 => {
                if !((*cur_cmd).a2).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block_184 = 322669159951527645;
            }
            76 | 108 => {
                current_block_184 = 322669159951527645;
            }
            61 | 100 | 68 | 70 | 103 | 71 | 104 | 72 | 110 | 78 | 112 | 80 | 122
            | 120 => {
                read_end_of_cmd();
                current_block_184 = 6957654774345280688;
            }
            114 => {
                b = read_filename();
                if strlen(get_buffer(b)) == 0 as libc::c_int as libc::c_ulong {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cur_cmd).x.readcmd.fname = xstrdup(get_buffer(b));
                if !((*cur_cmd).a1).is_null()
                    && (*(*cur_cmd).a1).addr_type as libc::c_uint
                        == ADDR_IS_NUM as libc::c_int as libc::c_uint
                    && (*(*cur_cmd).a1).addr_number == 0 as libc::c_int as libc::c_ulong
                    && ((*cur_cmd).a2).is_null()
                {
                    (*(*cur_cmd).a1).addr_number = 1 as libc::c_int as countT;
                    (*cur_cmd).x.readcmd.append = 0 as libc::c_int != 0;
                } else {
                    (*cur_cmd).x.readcmd.append = 1 as libc::c_int != 0;
                }
                free_buffer(b);
                current_block_184 = 6957654774345280688;
            }
            82 => {
                (*cur_cmd)
                    .x
                    .inf = get_openfile(&mut file_read, read_mode, 0 as libc::c_int);
                current_block_184 = 6957654774345280688;
            }
            87 | 119 => {
                (*cur_cmd)
                    .x
                    .outf = get_openfile(&mut file_write, write_mode, 1 as libc::c_int);
                current_block_184 = 6957654774345280688;
            }
            115 => {
                let mut b2: *mut buffer = 0 as *mut buffer;
                let mut flags: libc::c_int = 0;
                let mut slash: libc::c_int = 0;
                slash = inchar();
                b = match_slash(slash, 1 as libc::c_int);
                if b.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                b2 = match_slash(slash, 0 as libc::c_int);
                if b2.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                (*cur_cmd)
                    .x
                    .cmd_subst = ({
                    let mut __h: *mut obstack = &mut obs as *mut obstack;
                    let mut __o: *mut obstack = __h;
                    let mut __len: size_t = (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<subst>() as libc::c_ulong);
                    if ({
                        let mut __o1: *const obstack = __o;
                        ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                            as libc::c_long as size_t
                    }) < __len
                    {
                        _obstack_newchunk(__o, __len);
                    }
                    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                    {
                        let mut __o1: *mut obstack = __h;
                        let mut __value: *mut libc::c_void = (*__o1).object_base
                            as *mut libc::c_void;
                        if (*__o1).next_free == __value as *mut libc::c_char {
                            (*__o1)
                                .set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                        }
                        (*__o1)
                            .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                            as libc::c_ulong)
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut libc::c_char
                        })
                            .offset(
                                ((((*__o1).next_free)
                                    .offset_from(
                                        if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            < ::std::mem::size_of::<*mut libc::c_void>()
                                                as libc::c_ulong
                                        {
                                            (*__o1).object_base
                                        } else {
                                            0 as *mut libc::c_char
                                        },
                                    ) as libc::c_long as libc::c_ulong)
                                    .wrapping_add((*__o1).alignment_mask)
                                    & !(*__o1).alignment_mask) as isize,
                            );
                        if ((*__o1).next_free)
                            .offset_from((*__o1).chunk as *mut libc::c_char)
                            as libc::c_long as size_t
                            > ((*__o1).chunk_limit)
                                .offset_from((*__o1).chunk as *mut libc::c_char)
                                as libc::c_long as size_t
                        {
                            (*__o1).next_free = (*__o1).chunk_limit;
                        }
                        (*__o1).object_base = (*__o1).next_free;
                        __value
                    }
                }) as *mut subst;
                setup_replacement(
                    (*cur_cmd).x.cmd_subst,
                    get_buffer(b2),
                    size_buffer(b2),
                );
                free_buffer(b2);
                flags = mark_subst_opts((*cur_cmd).x.cmd_subst);
                (*(*cur_cmd).x.cmd_subst)
                    .regx = compile_regex(
                    b,
                    flags,
                    (*(*cur_cmd).x.cmd_subst).max_id() as libc::c_int + 1 as libc::c_int,
                );
                free_buffer(b);
                if (*(*cur_cmd).x.cmd_subst).eval() as libc::c_int != 0
                    && sandbox as libc::c_int != 0
                {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block_184 = 6957654774345280688;
            }
            121 => {
                let mut len: size_t = 0;
                let mut dest_len: size_t = 0;
                let mut slash_0: libc::c_int = 0;
                let mut b2_0: *mut buffer = 0 as *mut buffer;
                let mut src_buf: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut dest_buf: *mut libc::c_char = 0 as *mut libc::c_char;
                slash_0 = inchar();
                b = match_slash(slash_0, 0 as libc::c_int);
                if b.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                src_buf = get_buffer(b);
                len = normalize_text(src_buf, size_buffer(b), TEXT_BUFFER);
                b2_0 = match_slash(slash_0, 0 as libc::c_int);
                if b2_0.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                dest_buf = get_buffer(b2_0);
                dest_len = normalize_text(dest_buf, size_buffer(b2_0), TEXT_BUFFER);
                if mb_cur_max > 1 as libc::c_int {
                    let mut i: size_t = 0;
                    let mut j: size_t = 0;
                    let mut idx: size_t = 0;
                    let mut src_char_num: size_t = 0;
                    let mut src_lens: *mut size_t = (if ::std::mem::size_of::<size_t>()
                        as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
                    {
                        xzalloc(len)
                    } else {
                        xcalloc(len, ::std::mem::size_of::<size_t>() as libc::c_ulong)
                    }) as *mut size_t;
                    let mut trans_pairs: *mut *mut libc::c_char = 0
                        as *mut *mut libc::c_char;
                    let mut mbclen: size_t = 0;
                    let mut cur_stat: mbstate_t = {
                        let mut init = __mbstate_t {
                            __count: 0 as libc::c_int,
                            __value: C2RustUnnamed { __wch: 0 },
                        };
                        init
                    };
                    i = 0 as libc::c_int as size_t;
                    j = 0 as libc::c_int as size_t;
                    while i < len {
                        mbclen = if mb_cur_max == 1 as libc::c_int {
                            1 as libc::c_int as libc::c_ulong
                        } else {
                            rpl_mbrtowc(
                                0 as *mut wchar_t,
                                src_buf.offset(i as isize),
                                len.wrapping_sub(i),
                                &mut cur_stat,
                            )
                        };
                        if mbclen == -(1 as libc::c_int) as size_t
                            || mbclen == -(2 as libc::c_int) as size_t
                            || mbclen == 0 as libc::c_int as libc::c_ulong
                        {
                            mbclen = 1 as libc::c_int as size_t;
                        }
                        let fresh2 = j;
                        j = j.wrapping_add(1);
                        *src_lens.offset(fresh2 as isize) = mbclen;
                        i = (i as libc::c_ulong).wrapping_add(mbclen) as size_t
                            as size_t;
                    }
                    src_char_num = j;
                    memset(
                        &mut cur_stat as *mut mbstate_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                    );
                    idx = 0 as libc::c_int as size_t;
                    trans_pairs = (if ::std::mem::size_of::<*mut libc::c_char>()
                        as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
                    {
                        xzalloc(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(src_char_num)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        )
                    } else {
                        xcalloc(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(src_char_num)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        )
                    }) as *mut *mut libc::c_char;
                    (*cur_cmd).x.translatemb = trans_pairs;
                    i = 0 as libc::c_int as size_t;
                    while i < src_char_num {
                        if idx >= dest_len {
                            bad_prog(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    errors
                                        .as_ptr()
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                                as isize,
                                        )
                                        .offset(
                                            ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                                as isize,
                                        ),
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        let ref mut fresh3 = *trans_pairs
                            .offset(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                            );
                        *fresh3 = (if ::std::mem::size_of::<libc::c_char>()
                            as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
                        {
                            xzalloc(
                                (*src_lens.offset(i as isize))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            )
                        } else {
                            xcalloc(
                                (*src_lens.offset(i as isize))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                        }) as *mut libc::c_char;
                        memcpy(
                            *trans_pairs
                                .offset(
                                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                                ) as *mut libc::c_void,
                            src_buf as *const libc::c_void,
                            *src_lens.offset(i as isize),
                        );
                        *(*trans_pairs
                            .offset(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                            ))
                            .offset(
                                *src_lens.offset(i as isize) as isize,
                            ) = '\0' as i32 as libc::c_char;
                        src_buf = src_buf.offset(*src_lens.offset(i as isize) as isize);
                        mbclen = if mb_cur_max == 1 as libc::c_int {
                            1 as libc::c_int as libc::c_ulong
                        } else {
                            rpl_mbrtowc(
                                0 as *mut wchar_t,
                                dest_buf.offset(idx as isize),
                                dest_len.wrapping_sub(idx),
                                &mut cur_stat,
                            )
                        };
                        if mbclen == -(1 as libc::c_int) as size_t
                            || mbclen == -(2 as libc::c_int) as size_t
                            || mbclen == 0 as libc::c_int as libc::c_ulong
                        {
                            mbclen = 1 as libc::c_int as size_t;
                        }
                        let ref mut fresh4 = *trans_pairs
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(i)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                        *fresh4 = (if ::std::mem::size_of::<libc::c_char>()
                            as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
                        {
                            xzalloc(
                                mbclen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            )
                        } else {
                            xcalloc(
                                mbclen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                        }) as *mut libc::c_char;
                        memcpy(
                            *trans_pairs
                                .offset(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(i)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as *mut libc::c_void,
                            dest_buf.offset(idx as isize) as *const libc::c_void,
                            mbclen,
                        );
                        *(*trans_pairs
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(i)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ))
                            .offset(mbclen as isize) = '\0' as i32 as libc::c_char;
                        idx = (idx as libc::c_ulong).wrapping_add(mbclen) as size_t
                            as size_t;
                        i = i.wrapping_add(1);
                        i;
                    }
                    let ref mut fresh5 = *trans_pairs
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                        );
                    *fresh5 = 0 as *mut libc::c_char;
                    if idx != dest_len {
                        bad_prog(
                            dcgettext(
                                0 as *const libc::c_char,
                                errors
                                    .as_ptr()
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                            as isize,
                                    ),
                                5 as libc::c_int,
                            ),
                        );
                    }
                } else {
                    let mut translate: *mut libc::c_uchar = ({
                        let mut __h: *mut obstack = &mut obs as *mut obstack;
                        let mut __o: *mut obstack = __h;
                        let mut __len: size_t = (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            );
                        if ({
                            let mut __o1: *const obstack = __o;
                            ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                                as libc::c_long as size_t
                        }) < __len
                        {
                            _obstack_newchunk(__o, __len);
                        }
                        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                        {
                            let mut __o1: *mut obstack = __h;
                            let mut __value: *mut libc::c_void = (*__o1).object_base
                                as *mut libc::c_void;
                            if (*__o1).next_free == __value as *mut libc::c_char {
                                (*__o1)
                                    .set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                            }
                            (*__o1)
                                .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                                as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            })
                                .offset(
                                    ((((*__o1).next_free)
                                        .offset_from(
                                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                < ::std::mem::size_of::<*mut libc::c_void>()
                                                    as libc::c_ulong
                                            {
                                                (*__o1).object_base
                                            } else {
                                                0 as *mut libc::c_char
                                            },
                                        ) as libc::c_long as libc::c_ulong)
                                        .wrapping_add((*__o1).alignment_mask)
                                        & !(*__o1).alignment_mask) as isize,
                                );
                            if ((*__o1).next_free)
                                .offset_from((*__o1).chunk as *mut libc::c_char)
                                as libc::c_long as size_t
                                > ((*__o1).chunk_limit)
                                    .offset_from((*__o1).chunk as *mut libc::c_char)
                                    as libc::c_long as size_t
                            {
                                (*__o1).next_free = (*__o1).chunk_limit;
                            }
                            (*__o1).object_base = (*__o1).next_free;
                            __value
                        }
                    }) as *mut libc::c_uchar;
                    let mut ustring: *mut libc::c_uchar = src_buf as *mut libc::c_uchar;
                    if len != dest_len {
                        bad_prog(
                            dcgettext(
                                0 as *const libc::c_char,
                                errors
                                    .as_ptr()
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                            as isize,
                                    ),
                                5 as libc::c_int,
                            ),
                        );
                    }
                    len = 0 as libc::c_int as size_t;
                    while len < 256 as libc::c_int as libc::c_ulong {
                        *translate.offset(len as isize) = len as libc::c_uchar;
                        len = len.wrapping_add(1);
                        len;
                    }
                    loop {
                        let fresh6 = dest_len;
                        dest_len = dest_len.wrapping_sub(1);
                        if !(fresh6 != 0) {
                            break;
                        }
                        let fresh7 = dest_buf;
                        dest_buf = dest_buf.offset(1);
                        let fresh8 = ustring;
                        ustring = ustring.offset(1);
                        *translate.offset(*fresh8 as isize) = *fresh7 as libc::c_uchar;
                    }
                    (*cur_cmd).x.translate = translate;
                }
                read_end_of_cmd();
                free_buffer(b);
                free_buffer(b2_0);
                current_block_184 = 6957654774345280688;
            }
            -1 => {
                bad_prog(
                    dcgettext(
                        0 as *const libc::c_char,
                        errors
                            .as_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as isize,
                            ),
                        5 as libc::c_int,
                    ),
                );
                current_block_184 = 6411656358898446956;
            }
            _ => {
                current_block_184 = 6411656358898446956;
            }
        }
        match current_block_184 {
            322669159951527645 => {
                ch = in_nonblank();
                if 1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(ch as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && posixicity as libc::c_uint
                        != POSIXLY_BASIC as libc::c_int as libc::c_uint
                {
                    (*cur_cmd).x.int_arg = in_integer(ch) as libc::c_int;
                } else {
                    (*cur_cmd).x.int_arg = -(1 as libc::c_int);
                    savchar(ch);
                }
                read_end_of_cmd();
            }
            16082342845391650887 => {
                if ch == -(1 as libc::c_int) {
                    bad_prog(
                        dcgettext(
                            0 as *const libc::c_char,
                            errors
                                .as_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                        as isize,
                                ),
                            5 as libc::c_int,
                        ),
                    );
                }
                if ch == '\\' as i32 {
                    ch = inchar();
                } else {
                    if posixicity as libc::c_uint
                        == POSIXLY_BASIC as libc::c_int as libc::c_uint
                    {
                        bad_prog(
                            dcgettext(
                                0 as *const libc::c_char,
                                errors
                                    .as_ptr()
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                            as isize,
                                    )
                                    .offset(
                                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                            as isize,
                                    ),
                                5 as libc::c_int,
                            ),
                        );
                    }
                    savchar(ch);
                    ch = '\n' as i32;
                }
                read_text(&mut (*cur_cmd).x.cmd_txt, ch);
            }
            6411656358898446956 => {
                bad_command(ch as libc::c_char);
            }
            _ => {}
        }
        (*vector).v_length = ((*vector).v_length).wrapping_add(1);
        (*vector).v_length;
    }
    if posixicity as libc::c_uint == POSIXLY_BASIC as libc::c_int as libc::c_uint
        && !pending_text.is_null()
    {
        bad_prog(
            dcgettext(
                0 as *const libc::c_char,
                errors
                    .as_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                            as isize,
                    ),
                5 as libc::c_int,
            ),
        );
    }
    return vector;
}
pub unsafe extern "C" fn normalize_text(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut buftype: text_types,
) -> size_t {
    let mut current_block: u64;
    let mut bufend: *const libc::c_char = buf.offset(len as isize);
    let mut p: *mut libc::c_char = buf;
    let mut q: *mut libc::c_char = buf;
    let mut ch: libc::c_char = 0;
    let mut base: libc::c_int = 0;
    let mut bracket_state: libc::c_int = 0 as libc::c_int;
    let mut mbclen: libc::c_int = 0;
    let mut cur_stat: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    while p < bufend as *mut libc::c_char {
        mbclen = (if mb_cur_max == 1 as libc::c_int {
            1 as libc::c_int as libc::c_ulong
        } else {
            rpl_mbrtowc(
                0 as *mut wchar_t,
                p,
                bufend.offset_from(p) as libc::c_long as size_t,
                &mut cur_stat,
            )
        }) as libc::c_int;
        if mbclen != 1 as libc::c_int {
            if mbclen as libc::c_ulong == -(1 as libc::c_int) as size_t
                || mbclen as libc::c_ulong == -(2 as libc::c_int) as size_t
                || mbclen == 0 as libc::c_int
            {
                mbclen = 1 as libc::c_int;
            }
            memmove(
                q as *mut libc::c_void,
                p as *const libc::c_void,
                mbclen as libc::c_ulong,
            );
            q = q.offset(mbclen as isize);
            p = p.offset(mbclen as isize);
        } else {
            if *p as libc::c_int == '\\' as i32
                && p.offset(1 as libc::c_int as isize) < bufend as *mut libc::c_char
                && bracket_state == 0 as libc::c_int
            {
                p = p.offset(1);
                match *p as libc::c_int {
                    97 => {
                        let fresh9 = q;
                        q = q.offset(1);
                        *fresh9 = '\u{7}' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    102 => {
                        let fresh10 = q;
                        q = q.offset(1);
                        *fresh10 = '\u{c}' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    10 | 110 => {
                        let fresh11 = q;
                        q = q.offset(1);
                        *fresh11 = '\n' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    114 => {
                        let fresh12 = q;
                        q = q.offset(1);
                        *fresh12 = '\r' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    116 => {
                        let fresh13 = q;
                        q = q.offset(1);
                        *fresh13 = '\t' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    118 => {
                        let fresh14 = q;
                        q = q.offset(1);
                        *fresh14 = '\u{b}' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    100 => {
                        base = 10 as libc::c_int;
                        current_block = 8875191249258853179;
                    }
                    120 => {
                        base = 16 as libc::c_int;
                        current_block = 8875191249258853179;
                    }
                    111 => {
                        base = 8 as libc::c_int;
                        current_block = 8875191249258853179;
                    }
                    99 => {
                        p = p.offset(1);
                        if p < bufend as *mut libc::c_char {
                            let fresh17 = q;
                            q = q.offset(1);
                            *fresh17 = (({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *p as libc::c_uchar
                                            as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = toupper(*p as libc::c_uchar as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(*p as libc::c_uchar as libc::c_int as isize);
                                }
                                __res
                            }) ^ 0x40 as libc::c_int) as libc::c_char;
                            if *p as libc::c_int == '\\' as i32 {
                                p = p.offset(1);
                                p;
                                if *p as libc::c_int != '\\' as i32 {
                                    bad_prog(
                                        errors
                                            .as_ptr()
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 39]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                                                    as isize,
                                            )
                                            .offset(
                                                ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                                                    as isize,
                                            ),
                                    );
                                }
                            }
                            p = p.offset(1);
                            p;
                            continue;
                        } else {
                            if buftype as libc::c_uint
                                != TEXT_BUFFER as libc::c_int as libc::c_uint
                            {
                                let fresh18 = q;
                                q = q.offset(1);
                                *fresh18 = '\\' as i32 as libc::c_char;
                            }
                            continue;
                        }
                    }
                    _ => {
                        if buftype as libc::c_uint
                            != TEXT_BUFFER as libc::c_int as libc::c_uint
                        {
                            let fresh19 = q;
                            q = q.offset(1);
                            *fresh19 = '\\' as i32 as libc::c_char;
                        }
                        current_block = 8545136480011357681;
                    }
                }
                match current_block {
                    8545136480011357681 => {}
                    _ => {
                        p = convert_number(&mut ch, p, bufend, base);
                        if buftype as libc::c_uint
                            == TEXT_REPLACEMENT as libc::c_int as libc::c_uint
                            && (ch as libc::c_int == '&' as i32
                                || ch as libc::c_int == '\\' as i32)
                        {
                            let fresh15 = q;
                            q = q.offset(1);
                            *fresh15 = '\\' as i32 as libc::c_char;
                        }
                        let fresh16 = q;
                        q = q.offset(1);
                        *fresh16 = ch;
                        continue;
                    }
                }
            } else if buftype as libc::c_uint
                == TEXT_REGEX as libc::c_int as libc::c_uint
                && posixicity as libc::c_uint
                    != POSIXLY_EXTENDED as libc::c_int as libc::c_uint
            {
                match *p as libc::c_int {
                    91 => {
                        if bracket_state == 0 {
                            bracket_state = -(1 as libc::c_int);
                        }
                    }
                    58 | 46 | 61 => {
                        if bracket_state == -(1 as libc::c_int)
                            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                == '[' as i32
                        {
                            bracket_state = *p as libc::c_int;
                        }
                    }
                    93 => {
                        if !(bracket_state == 0 as libc::c_int) {
                            if bracket_state == -(1 as libc::c_int) {
                                bracket_state = 0 as libc::c_int;
                            } else if *p.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int != bracket_state
                                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    == bracket_state
                            {
                                bracket_state = -(1 as libc::c_int);
                            }
                        }
                    }
                    _ => {}
                }
            }
            let fresh20 = p;
            p = p.offset(1);
            let fresh21 = q;
            q = q.offset(1);
            *fresh21 = *fresh20;
        }
    }
    return q.offset_from(buf) as libc::c_long as size_t;
}
pub unsafe extern "C" fn compile_string(
    mut cur_program: *mut vector,
    mut str: *mut libc::c_char,
    mut len: size_t,
) -> *mut vector {
    static mut string_expr_count: countT = 0 as libc::c_int as countT;
    let mut ret: *mut vector = 0 as *mut vector;
    prog.file = 0 as *mut FILE;
    prog.base = str as *mut libc::c_uchar;
    prog.cur = prog.base;
    prog.end = (prog.cur).offset(len as isize);
    cur_input.line = 0 as libc::c_int as countT;
    cur_input.name = 0 as *const libc::c_char;
    string_expr_count = string_expr_count.wrapping_add(1);
    cur_input.string_expr_count = string_expr_count;
    ret = compile_program(cur_program);
    prog.base = 0 as *const libc::c_uchar;
    prog.cur = 0 as *const libc::c_uchar;
    prog.end = 0 as *const libc::c_uchar;
    first_script = 0 as libc::c_int != 0;
    return ret;
}
pub unsafe extern "C" fn compile_file(
    mut cur_program: *mut vector,
    mut cmdfile: *const libc::c_char,
) -> *mut vector {
    let mut ret: *mut vector = 0 as *mut vector;
    prog.file = stdin;
    if *cmdfile.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
        || *cmdfile.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        prog
            .file = ck_fopen(
            cmdfile,
            b"rt\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    cur_input.line = 1 as libc::c_int as countT;
    cur_input.name = cmdfile;
    cur_input.string_expr_count = 0 as libc::c_int as countT;
    ret = compile_program(cur_program);
    if prog.file != stdin {
        ck_fclose(prog.file);
    }
    prog.file = 0 as *mut FILE;
    first_script = 0 as libc::c_int != 0;
    return ret;
}
unsafe extern "C" fn cleanup_program_filenames() {
    let mut p: *mut output = 0 as *mut output;
    p = file_read;
    while !p.is_null() {
        if !((*p).name).is_null() {
            rpl_free((*p).name as *mut libc::c_void);
            (*p).name = 0 as *mut libc::c_char;
        }
        p = (*p).link;
    }
    p = file_write;
    while !p.is_null() {
        if !((*p).name).is_null() {
            rpl_free((*p).name as *mut libc::c_void);
            (*p).name = 0 as *mut libc::c_char;
        }
        p = (*p).link;
    }
}
pub unsafe extern "C" fn check_final_program(mut program: *mut vector) {
    let mut go: *mut sed_label = 0 as *mut sed_label;
    let mut lbl: *mut sed_label = 0 as *mut sed_label;
    if !blocks.is_null() {
        memcpy(
            &mut cur_input as *mut error_info as *mut libc::c_void,
            &mut (*blocks).err_info as *mut error_info as *const libc::c_void,
            ::std::mem::size_of::<error_info>() as libc::c_ulong,
        );
        bad_prog(
            dcgettext(
                0 as *const libc::c_char,
                errors
                    .as_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong
                            as isize,
                    ),
                5 as libc::c_int,
            ),
        );
    }
    if !pending_text.is_null() {
        (*old_text_buf).text_length = size_buffer(pending_text);
        if (*old_text_buf).text_length != 0 {
            (*old_text_buf)
                .text = xmemdup(
                get_buffer(pending_text) as *const libc::c_void,
                ((*old_text_buf).text_length)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        free_buffer(pending_text);
        pending_text = 0 as *mut buffer;
    }
    go = jumps;
    while !go.is_null() {
        lbl = labels;
        while !lbl.is_null() {
            if strcmp((*lbl).name, (*go).name) == 0 as libc::c_int {
                break;
            }
            lbl = (*lbl).next;
        }
        if !lbl.is_null() {
            (*((*program).v).offset((*go).v_index as isize))
                .x
                .jump_index = (*lbl).v_index;
        } else {
            if *(*go).name != 0 {
                panic(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't find label for jump to `%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*go).name,
                );
            }
            (*((*program).v).offset((*go).v_index as isize))
                .x
                .jump_index = (*program).v_length;
        }
        go = release_label(go);
    }
    jumps = 0 as *mut sed_label;
    lbl = labels;
    while !lbl.is_null() {
        lbl = release_label(lbl);
    }
    labels = 0 as *mut sed_label;
}
pub unsafe extern "C" fn rewind_read_files() {
    let mut p: *mut output = 0 as *mut output;
    p = file_read;
    while !p.is_null() {
        if !((*p).fp).is_null() {
            rewind((*p).fp);
        }
        p = (*p).link;
    }
}
pub unsafe extern "C" fn finish_program(mut program: *mut vector) {
    cleanup_program_filenames();
    let mut p: *mut output = 0 as *mut output;
    let mut q: *mut output = 0 as *mut output;
    p = file_read;
    while !p.is_null() {
        if !((*p).fp).is_null() {
            ck_fclose((*p).fp);
        }
        q = (*p).link;
        p = q;
    }
    p = file_write;
    while !p.is_null() {
        if !((*p).fp).is_null() {
            ck_fclose((*p).fp);
        }
        q = (*p).link;
        p = q;
    }
    file_write = 0 as *mut output;
    file_read = file_write;
}

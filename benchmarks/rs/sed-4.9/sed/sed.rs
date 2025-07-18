use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn init_localeinfo(_: *mut localeinfo);
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn ck_fclose(stream: *mut FILE);
    fn remove_cleanup_file();
    fn compile_string(
        _: *mut vector,
        str: *mut libc::c_char,
        len: size_t,
    ) -> *mut vector;
    fn compile_file(_: *mut vector, cmdfile: *const libc::c_char) -> *mut vector;
    fn check_final_program(_: *mut vector);
    fn finish_program(_: *mut vector);
    fn debug_print_program(program: *const vector);
    fn process_files(_: *mut vector, argv: *mut *mut libc::c_char) -> libc::c_int;
    fn initialize_mbcs();
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    static mut Version: *const libc::c_char;
    fn is_selinux_enabled() -> libc::c_int;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
pub type wint_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
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
    pub x: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type posixicity_types = libc::c_uint;
pub const POSIXLY_BASIC: posixicity_types = 2;
pub const POSIXLY_CORRECT: posixicity_types = 1;
pub const POSIXLY_EXTENDED: posixicity_types = 0;
pub const DEBUG_OPTION: C2RustUnnamed_0 = 129;
pub const SANDBOX_OPTION: C2RustUnnamed_0 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
pub static mut extended_regexp_flags: libc::c_int = 0 as libc::c_int;
pub static mut buffer_delimiter: libc::c_char = '\n' as i32 as libc::c_char;
pub static mut unbuffered: bool = 0 as libc::c_int != 0;
pub static mut no_default_output: bool = 0 as libc::c_int != 0;
pub static mut separate_files: bool = 0 as libc::c_int != 0;
pub static mut follow_symlinks: bool = 0 as libc::c_int != 0;
pub static mut sandbox: bool = 0 as libc::c_int != 0;
pub static mut debug: bool = 0 as libc::c_int != 0;
pub static mut in_place_extension: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut read_mode: *const libc::c_char = b"r\0" as *const u8
    as *const libc::c_char;
pub static mut write_mode: *const libc::c_char = b"w\0" as *const u8
    as *const libc::c_char;
pub static mut posixicity: posixicity_types = POSIXLY_EXTENDED;
pub static mut lcmd_out_line_len: countT = 70 as libc::c_int as countT;
static mut the_program: *mut vector = 0 as *const vector as *mut vector;
pub static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
unsafe extern "C" fn cleanup() {
    remove_cleanup_file();
}
unsafe extern "C" fn contact(mut errmsg: libc::c_int) {
    let mut out: *mut FILE = if errmsg != 0 { stderr } else { stdout };
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"GNU sed home page: <https://www.gnu.org/software/sed/>.\nGeneral help using GNU software: <https://www.gnu.org/gethelp/>.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if errmsg == 0 {
        fprintf(
            out,
            dcgettext(
                0 as *const libc::c_char,
                b"E-mail bug reports to: <%s>.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"bug-sed@gnu.org\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn selinux_support() {
    putchar_unlocked('\n' as i32);
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"This sed program was built with SELinux support.\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if is_selinux_enabled() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"SELinux is enabled on this system.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"SELinux is disabled on this system.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn usage(mut status: libc::c_int) {
    let mut out: *mut FILE = if status != 0 { stderr } else { stdout };
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... {script-only-if-no-other-script} [input-file]...\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -n, --quiet, --silent\n                 suppress automatic printing of pattern space\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"      --debug\n                 annotate program execution\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -e script, --expression=script\n                 add the script to the commands to be executed\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -f script-file, --file=script-file\n                 add the contents of script-file to the commands to be executed\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  --follow-symlinks\n                 follow symlinks when processing in place\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -i[SUFFIX], --in-place[=SUFFIX]\n                 edit files in place (makes backup if SUFFIX supplied)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -l N, --line-length=N\n                 specify the desired line-wrap length for the `l' command\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  --posix\n                 disable all GNU extensions.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -E, -r, --regexp-extended\n                 use extended regular expressions in the script\n                 (for portability use POSIX -E).\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -s, --separate\n                 consider files as separate rather than as a single,\n                 continuous long stream.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"      --sandbox\n                 operate in sandbox mode (disable e/r/w commands).\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -u, --unbuffered\n                 load minimal amounts of data from the input files and flush\n                 the output buffers more often\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z, --null-data\n                 separate lines by NUL characters\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"      --help     display this help and exit\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"      --version  output version information and exit\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"\nIf no -e, --expression, -f, or --file option is given, then the first\nnon-option argument is taken as the sed script to interpret.  All\nremaining arguments are names of input files; if no input files are\nspecified, then the standard input is read.\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    contact(status);
    ck_fclose(0 as *mut FILE);
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut longopts: [option; 19] = [
        {
            let mut init = option {
                name: b"binary\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"regexp-extended\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: DEBUG_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"expression\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"in-place\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-length\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null-data\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"posix\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"sandbox\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: SANDBOX_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"separate\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unbuffered\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"follow-symlinks\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut opt: libc::c_int = 0;
    let mut return_code: libc::c_int = 0;
    let mut cols: *const libc::c_char = getenv(
        b"COLS\0" as *const u8 as *const libc::c_char,
    );
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    initialize_mbcs();
    init_localeinfo(&mut localeinfo);
    atexit(Some(cleanup as unsafe extern "C" fn() -> ()));
    bindtextdomain(
        b"sed\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"sed\0" as *const u8 as *const libc::c_char);
    if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null() {
        posixicity = POSIXLY_CORRECT;
    } else {
        posixicity = POSIXLY_EXTENDED;
    }
    if !cols.is_null() {
        let mut t: countT = atoi(cols) as countT;
        if t > 1 as libc::c_int as libc::c_ulong {
            lcmd_out_line_len = t.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
    }
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"bsnrzuEe:f:l:i::V:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_45: u64;
        match opt {
            110 => {
                no_default_output = 1 as libc::c_int != 0;
                current_block_45 = 10150597327160359210;
            }
            101 => {
                the_program = compile_string(the_program, optarg, strlen(optarg));
                current_block_45 = 10150597327160359210;
            }
            102 => {
                the_program = compile_file(the_program, optarg);
                current_block_45 = 10150597327160359210;
            }
            122 => {
                buffer_delimiter = 0 as libc::c_int as libc::c_char;
                current_block_45 = 10150597327160359210;
            }
            70 => {
                follow_symlinks = 1 as libc::c_int != 0;
                current_block_45 = 10150597327160359210;
            }
            105 => {
                separate_files = 1 as libc::c_int != 0;
                if optarg.is_null() {
                    in_place_extension = xstrdup(
                        b"*\0" as *const u8 as *const libc::c_char,
                    );
                } else if !(strchr(optarg, '*' as i32)).is_null() {
                    in_place_extension = xstrdup(optarg);
                } else {
                    in_place_extension = (if ::std::mem::size_of::<libc::c_char>()
                        as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
                    {
                        xzalloc(
                            (strlen(optarg))
                                .wrapping_add(2 as libc::c_int as libc::c_ulong),
                        )
                    } else {
                        xcalloc(
                            (strlen(optarg))
                                .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                    }) as *mut libc::c_char;
                    *in_place_extension
                        .offset(0 as libc::c_int as isize) = '*' as i32 as libc::c_char;
                    strcpy(in_place_extension.offset(1 as libc::c_int as isize), optarg);
                }
                current_block_45 = 10150597327160359210;
            }
            108 => {
                lcmd_out_line_len = atoi(optarg) as countT;
                current_block_45 = 10150597327160359210;
            }
            112 => {
                posixicity = POSIXLY_BASIC;
                current_block_45 = 10150597327160359210;
            }
            98 => {
                read_mode = b"rb\0" as *const u8 as *const libc::c_char;
                write_mode = b"wb\0" as *const u8 as *const libc::c_char;
                current_block_45 = 10150597327160359210;
            }
            69 | 114 => {
                extended_regexp_flags = 1 as libc::c_int;
                current_block_45 = 10150597327160359210;
            }
            115 => {
                separate_files = 1 as libc::c_int != 0;
                current_block_45 = 10150597327160359210;
            }
            128 => {
                sandbox = 1 as libc::c_int != 0;
                current_block_45 = 10150597327160359210;
            }
            129 => {
                debug = 1 as libc::c_int != 0;
                current_block_45 = 10150597327160359210;
            }
            117 => {
                unbuffered = 1 as libc::c_int != 0;
                current_block_45 = 10150597327160359210;
            }
            118 => {
                version_etc(
                    stdout,
                    program_name,
                    b"GNU sed\0" as *const u8 as *const libc::c_char,
                    Version,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Jay Fenlason\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Tom Lord\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Ken Pizzini\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Paolo Bonzini\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Assaf Gordon\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                selinux_support();
                contact(0 as libc::c_int);
                ck_fclose(0 as *mut FILE);
                exit(0 as libc::c_int);
            }
            104 => {
                usage(0 as libc::c_int);
                current_block_45 = 10407000698862883697;
            }
            _ => {
                current_block_45 = 10407000698862883697;
            }
        }
        match current_block_45 {
            10407000698862883697 => {
                usage(EXIT_BAD_USAGE as libc::c_int);
            }
            _ => {}
        }
    }
    if the_program.is_null() {
        if optind < argc {
            let fresh1 = optind;
            optind = optind + 1;
            let mut arg: *mut libc::c_char = *argv.offset(fresh1 as isize);
            the_program = compile_string(the_program, arg, strlen(arg));
        } else {
            usage(EXIT_BAD_USAGE as libc::c_int);
        }
    }
    check_final_program(the_program);
    if debug {
        debug_print_program(the_program);
    }
    return_code = process_files(the_program, argv.offset(optind as isize));
    finish_program(the_program);
    ck_fclose(0 as *mut FILE);
    return return_code;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

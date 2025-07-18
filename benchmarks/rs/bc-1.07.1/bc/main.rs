use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut interactive: libc::c_char;
    static mut compile_only: libc::c_int;
    static mut use_math: libc::c_int;
    static mut warn_not_std: libc::c_int;
    static mut std_only: libc::c_int;
    static mut quiet: libc::c_int;
    static mut file_names: *mut file_node;
    static mut file_name: *mut libc::c_char;
    static mut is_std_in: libc::c_char;
    static mut line_size: libc::c_int;
    static mut line_no: libc::c_int;
    static mut optind: libc::c_int;
    static mut yyin: *mut FILE;
    static mut libmath: [*const libc::c_char; 0];
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn init_gen();
    fn init_tree();
    fn lookup(name: *mut libc::c_char, namekind: libc::c_int) -> libc::c_int;
    fn bc_malloc(_: size_t) -> *mut libc::c_void;
    fn show_bc_version();
    fn bc_exit(_: libc::c_int);
    fn init_load();
    fn load_code(code: *const libc::c_char);
    fn init_storage();
    fn yyparse() -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_node {
    pub name: *mut libc::c_char,
    pub next: *mut file_node,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut first_file: libc::c_char = 0;
static mut last: *mut file_node = 0 as *const file_node as *mut file_node;
static mut long_options: [option; 9] = unsafe {
    [
        {
            let mut init = option {
                name: b"compile\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &compile_only as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
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
                name: b"interactive\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mathlib\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &use_math as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &quiet as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"standard\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &std_only as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
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
                name: b"warn\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &warn_not_std as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
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
    ]
};
unsafe extern "C" fn usage(mut progname: *const libc::c_char) {
    printf(
        b"usage: %s [options] [file ...]\n%s%s%s%s%s%s%s\0" as *const u8
            as *const libc::c_char,
        progname,
        b"  -h  --help         print this usage and exit\n\0" as *const u8
            as *const libc::c_char,
        b"  -i  --interactive  force interactive mode\n\0" as *const u8
            as *const libc::c_char,
        b"  -l  --mathlib      use the predefined math routines\n\0" as *const u8
            as *const libc::c_char,
        b"  -q  --quiet        don't print initial banner\n\0" as *const u8
            as *const libc::c_char,
        b"  -s  --standard     non-standard bc constructs are errors\n\0" as *const u8
            as *const libc::c_char,
        b"  -w  --warn         warn about non-standard bc constructs\n\0" as *const u8
            as *const libc::c_char,
        b"  -v  --version      print version information and exit\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn parse_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut optch: libc::c_int = 0;
    let mut long_index: libc::c_int = 0;
    let mut temp: *mut file_node = 0 as *mut file_node;
    optind = 0 as libc::c_int;
    loop {
        optch = getopt_long(
            argc,
            argv,
            b"chilqswv\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut long_index,
        );
        if optch == -(1 as libc::c_int) {
            break;
        }
        match optch {
            0 => {}
            99 => {
                compile_only = 1 as libc::c_int;
            }
            104 => {
                usage(*argv.offset(0 as libc::c_int as isize));
                bc_exit(0 as libc::c_int);
            }
            105 => {
                interactive = 1 as libc::c_int as libc::c_char;
            }
            108 => {
                use_math = 1 as libc::c_int;
            }
            113 => {
                quiet = 1 as libc::c_int;
            }
            115 => {
                std_only = 1 as libc::c_int;
            }
            118 => {
                show_bc_version();
                bc_exit(0 as libc::c_int);
            }
            119 => {
                warn_not_std = 1 as libc::c_int;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                bc_exit(1 as libc::c_int);
            }
        }
    }
    while optind < argc {
        temp = bc_malloc(::std::mem::size_of::<file_node>() as libc::c_ulong)
            as *mut file_node;
        (*temp).name = *argv.offset(optind as isize);
        (*temp).next = 0 as *mut file_node;
        if last.is_null() {
            file_names = temp;
        } else {
            (*last).next = temp;
        }
        last = temp;
        optind += 1;
        optind;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut env_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env_argv: [*mut libc::c_char; 30] = [0 as *mut libc::c_char; 30];
    let mut env_argc: libc::c_int = 0;
    if isatty(0 as libc::c_int) != 0 && isatty(1 as libc::c_int) != 0 {
        interactive = 1 as libc::c_int as libc::c_char;
    }
    setvbuf(
        stdout,
        0 as *mut libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
    );
    env_value = getenv(b"BC_ENV_ARGS\0" as *const u8 as *const libc::c_char);
    if !env_value.is_null() {
        env_argc = 1 as libc::c_int;
        env_argv[0 as libc::c_int
            as usize] = strdup(b"BC_ENV_ARGS\0" as *const u8 as *const libc::c_char);
        while *env_value as libc::c_int != 0 as libc::c_int {
            if *env_value as libc::c_int != ' ' as i32 {
                let fresh0 = env_argc;
                env_argc = env_argc + 1;
                env_argv[fresh0 as usize] = env_value;
                while *env_value as libc::c_int != ' ' as i32
                    && *env_value as libc::c_int != 0 as libc::c_int
                {
                    env_value = env_value.offset(1);
                    env_value;
                }
                if *env_value as libc::c_int != 0 as libc::c_int {
                    *env_value = 0 as libc::c_int as libc::c_char;
                    env_value = env_value.offset(1);
                    env_value;
                }
            } else {
                env_value = env_value.offset(1);
                env_value;
            }
        }
        parse_args(env_argc, env_argv.as_mut_ptr());
    }
    parse_args(argc, argv);
    if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null() {
        std_only = 1 as libc::c_int;
    }
    env_value = getenv(b"BC_LINE_LENGTH\0" as *const u8 as *const libc::c_char);
    if !env_value.is_null() {
        line_size = atoi(env_value);
        if line_size < 3 as libc::c_int && line_size != 0 as libc::c_int {
            line_size = 70 as libc::c_int;
        }
    } else {
        line_size = 70 as libc::c_int;
    }
    init_storage();
    init_load();
    if interactive != 0 {
        signal(
            2 as libc::c_int,
            Some(use_quit as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    init_tree();
    init_gen();
    is_std_in = 0 as libc::c_int as libc::c_char;
    first_file = 1 as libc::c_int as libc::c_char;
    if open_new_file() == 0 {
        bc_exit(1 as libc::c_int);
    }
    yyparse();
    if compile_only != 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    bc_exit(0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn open_new_file() -> libc::c_int {
    let mut new_file: *mut FILE = 0 as *mut FILE;
    let mut temp: *mut file_node = 0 as *mut file_node;
    line_no = 1 as libc::c_int;
    if is_std_in != 0 {
        return 0 as libc::c_int;
    }
    if use_math != 0 && first_file as libc::c_int != 0 {
        let mut mstr: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        lookup(strdup(b"e\0" as *const u8 as *const libc::c_char), 2 as libc::c_int);
        lookup(strdup(b"l\0" as *const u8 as *const libc::c_char), 2 as libc::c_int);
        lookup(strdup(b"s\0" as *const u8 as *const libc::c_char), 2 as libc::c_int);
        lookup(strdup(b"a\0" as *const u8 as *const libc::c_char), 2 as libc::c_int);
        lookup(strdup(b"c\0" as *const u8 as *const libc::c_char), 2 as libc::c_int);
        lookup(strdup(b"j\0" as *const u8 as *const libc::c_char), 2 as libc::c_int);
        mstr = libmath.as_mut_ptr();
        while !(*mstr).is_null() {
            load_code(*mstr);
            mstr = mstr.offset(1);
            mstr;
        }
    }
    if !file_names.is_null() {
        new_file = fopen((*file_names).name, b"r\0" as *const u8 as *const libc::c_char);
        if !new_file.is_null() {
            new_yy_file(new_file);
            temp = file_names;
            file_name = (*temp).name;
            file_names = (*temp).next;
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        fprintf(
            stderr,
            b"File %s is unavailable.\n\0" as *const u8 as *const libc::c_char,
            (*file_names).name,
        );
        bc_exit(1 as libc::c_int);
    }
    new_yy_file(stdin);
    is_std_in = 1 as libc::c_int as libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn new_yy_file(mut file: *mut FILE) {
    if first_file == 0 {
        fclose(yyin);
    }
    yyin = file;
    first_file = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn use_quit(mut sig: libc::c_int) {
    write(
        1 as libc::c_int,
        b"\n(interrupt) Exiting bc.\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        26 as libc::c_int as size_t,
    );
    bc_exit(0 as libc::c_int);
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

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_t {
    pub benchmark: libc::c_int,
    pub filter: *const libc::c_char,
    pub init_search: *const libc::c_char,
    pub tty_filename: *const libc::c_char,
    pub show_scores: libc::c_int,
    pub num_lines: libc::c_uint,
    pub scrolloff: libc::c_uint,
    pub prompt: *const libc::c_char,
    pub workers: libc::c_uint,
    pub input_delimiter: libc::c_char,
    pub show_info: libc::c_int,
}
static mut usage_str: *const libc::c_char = b"Usage: fzy [OPTION]...\n -l, --lines=LINES        Specify how many lines of results to show (default 10)\n -p, --prompt=PROMPT      Input prompt (default '> ')\n -q, --query=QUERY        Use QUERY as the initial search string\n -e, --show-matches=QUERY Output the sorted matches of QUERY\n -t, --tty=TTY            Specify file to use as TTY device (default /dev/tty)\n -s, --show-scores        Show the scores of each match\n -0, --read-null          Read input delimited by ASCII NUL characters\n -j, --workers NUM        Use NUM workers for searching. (default is # of CPUs)\n -i, --show-info          Show selection info line\n -h, --help     Display this help and exit\n -v, --version  Output version information and exit\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn usage(mut argv0: *const libc::c_char) {
    fprintf(stderr, usage_str, argv0);
}
static mut longopts: [option; 13] = [
    {
        let mut init = option {
            name: b"show-matches\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"query\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"tty\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-scores\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"read-null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '0' as i32,
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
            name: b"benchmark\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"workers\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'j' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-info\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
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
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn options_init(mut options: *mut options_t) {
    (*options).benchmark = 0 as libc::c_int;
    (*options).filter = 0 as *const libc::c_char;
    (*options).init_search = 0 as *const libc::c_char;
    (*options).show_scores = 0 as libc::c_int;
    (*options).scrolloff = 1 as libc::c_int as libc::c_uint;
    (*options).tty_filename = b"/dev/tty\0" as *const u8 as *const libc::c_char;
    (*options).num_lines = 10 as libc::c_int as libc::c_uint;
    (*options).prompt = b"> \0" as *const u8 as *const libc::c_char;
    (*options).workers = 0 as libc::c_int as libc::c_uint;
    (*options).input_delimiter = '\n' as i32 as libc::c_char;
    (*options).show_info = 0 as libc::c_int;
}
pub unsafe extern "C" fn options_parse(
    mut options: *mut options_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    options_init(options);
    let mut c: libc::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"vhs0e:q:l:t:p:j:i\0" as *const u8 as *const libc::c_char,
            longopts.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            118 => {
                printf(
                    b"%s 1.0 \xC2\xA9 2014-2018 John Hawthorn\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
                exit(0 as libc::c_int);
            }
            115 => {
                (*options).show_scores = 1 as libc::c_int;
            }
            48 => {
                (*options).input_delimiter = '\0' as i32 as libc::c_char;
            }
            113 => {
                (*options).init_search = optarg;
            }
            101 => {
                (*options).filter = optarg;
            }
            98 => {
                if !optarg.is_null() {
                    if sscanf(
                        optarg,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut (*options).benchmark as *mut libc::c_int,
                    ) != 1 as libc::c_int
                    {
                        usage(*argv.offset(0 as libc::c_int as isize));
                        exit(1 as libc::c_int);
                    }
                } else {
                    (*options).benchmark = 100 as libc::c_int;
                }
            }
            116 => {
                (*options).tty_filename = optarg;
            }
            112 => {
                (*options).prompt = optarg;
            }
            106 => {
                if sscanf(
                    optarg,
                    b"%u\0" as *const u8 as *const libc::c_char,
                    &mut (*options).workers as *mut libc::c_uint,
                ) != 1 as libc::c_int
                {
                    usage(*argv.offset(0 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
            }
            108 => {
                let mut l: libc::c_int = 0;
                if strcmp(optarg, b"max\0" as *const u8 as *const libc::c_char) == 0 {
                    l = 2147483647 as libc::c_int;
                } else if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut l as *mut libc::c_int,
                ) != 1 as libc::c_int || l < 3 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"Invalid format for --lines: %s\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    fprintf(
                        stderr,
                        b"Must be integer in range 3..\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    usage(*argv.offset(0 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
                (*options).num_lines = l as libc::c_uint;
            }
            105 => {
                (*options).show_info = 1 as libc::c_int;
            }
            104 | _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(0 as libc::c_int);
            }
        }
    }
    if optind != argc {
        usage(*argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
}

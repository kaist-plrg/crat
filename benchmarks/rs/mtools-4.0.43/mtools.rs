use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn _basename(filename: *const libc::c_char) -> *const libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setup_signal();
    fn read_config();
    fn mattrib(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mbadblocks(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mcat(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mcd(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mcopy(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mdel(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mdir(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mdoctorfat(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mdu(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mformat(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn minfo(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mlabel(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mmd(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mmount(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mmove(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mpartition(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn mshowfat(argc: libc::c_int, argv: *mut *mut libc::c_char, mtype: libc::c_int);
    fn mshortname(argc: libc::c_int, argv: *mut *mut libc::c_char, mtype: libc::c_int);
    fn mzip(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    fn init_privs();
    fn mtoolstest(argc: libc::c_int, argv: *mut *mut libc::c_char, type_0: libc::c_int);
    static mut mversion: *const libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub struct dispatch {
    pub cmd: *const libc::c_char,
    pub fn_0: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char, libc::c_int) -> (),
    >,
    pub type_0: libc::c_int,
}
pub static mut progname: *const libc::c_char = 0 as *const libc::c_char;
static mut dispatch: [dispatch; 26] = unsafe {
    [
        {
            let mut init = dispatch {
                cmd: b"mattrib\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mattrib
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mbadblocks\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mbadblocks
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mcat\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mcat
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mcd\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mcd
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mcopy\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mcopy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdel\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mdel
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdeltree\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mdel
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdir\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mdir
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdoctorfat\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mdoctorfat
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mdu\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mdu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mformat\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mformat
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"minfo\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    minfo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mlabel\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mlabel
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mmd\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mmd
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mmount\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mmount
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mpartition\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mpartition
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mrd\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mdel
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mread\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mcopy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mmove\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mmove
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mren\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mmove
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mshowfat\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mshowfat
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mshortname\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mshortname
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mtoolstest\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mtoolstest
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mtype\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mcopy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mwrite\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mcopy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = dispatch {
                cmd: b"mzip\0" as *const u8 as *const libc::c_char,
                fn_0: Some(
                    mzip
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut locale: *mut libc::c_char = 0 as *mut libc::c_char;
    locale = setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    if locale.is_null()
        || strcmp(locale, b"C\0" as *const u8 as *const libc::c_char) == 0
    {
        setlocale(6 as libc::c_int, b"en_US\0" as *const u8 as *const libc::c_char);
    }
    init_privs();
    name = _basename(*argv.offset(0 as libc::c_int as isize));
    progname = *argv.offset(0 as libc::c_int as isize);
    if argc >= 3 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-c\0" as *const u8 as *const libc::c_char,
        ) == 0 && strcmp(name, b"mtools\0" as *const u8 as *const libc::c_char) == 0
    {
        argc -= 2 as libc::c_int;
        argv = argv.offset(2 as libc::c_int as isize);
        name = *argv.offset(0 as libc::c_int as isize);
    }
    if argc >= 2 as libc::c_int
        && (strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-V\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"--version\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
    {
        printf(
            b"%s (GNU mtools) %s\n\0" as *const u8 as *const libc::c_char,
            name,
            mversion,
        );
        printf(
            b"configured with the following options: \0" as *const u8
                as *const libc::c_char,
        );
        printf(b"enable-xdf \0" as *const u8 as *const libc::c_char);
        printf(b"disable-vold \0" as *const u8 as *const libc::c_char);
        printf(b"disable-new-vold \0" as *const u8 as *const libc::c_char);
        printf(b"disable-debug \0" as *const u8 as *const libc::c_char);
        printf(b"enable-raw-term \0" as *const u8 as *const libc::c_char);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    read_config();
    setup_signal();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[dispatch; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<dispatch>() as libc::c_ulong)
    {
        if strcmp(name, dispatch[i as usize].cmd) == 0 {
            (dispatch[i as usize].fn_0)
                .unwrap()(argc, argv, dispatch[i as usize].type_0);
        }
        i = i.wrapping_add(1);
        i;
    }
    if strcmp(name, b"mtools\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Unknown mtools command '%s'\n\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    fprintf(stderr, b"Supported commands:\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[dispatch; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<dispatch>() as libc::c_ulong)
    {
        if i.wrapping_rem(8 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint
        {
            putc('\n' as i32, stderr);
        } else {
            fprintf(stderr, b", \0" as *const u8 as *const libc::c_char);
        }
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            dispatch[i as usize].cmd,
        );
        i = i.wrapping_add(1);
        i;
    }
    putc('\n' as i32, stderr);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn helpFlag(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return (argc > 1 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0) as libc::c_int;
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

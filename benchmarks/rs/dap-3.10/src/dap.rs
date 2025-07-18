use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub static mut editor: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut edopts: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut dappp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut dapruns: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut incdir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut libdir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn ecopy(mut e: *mut libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if !e.is_null() {
        copy = malloc((strlen(e)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if copy.is_null() {
            perror(b"dap\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        strcpy(copy, e);
        return copy;
    }
    return 0 as *mut libc::c_char;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut runstat: libc::c_int = 0;
    fputs(
        b"\nDap, Copyright (C) 2001, 2002, 2003, 2004, 2005 Free Software Foundation, Inc.\n\0"
            as *const u8 as *const libc::c_char,
        stderr,
    );
    fputs(
        b"Dap comes with ABSOLUTELY NO WARRANTY;\n\0" as *const u8
            as *const libc::c_char,
        stderr,
    );
    fputs(
        b"for details see the GNU Public License.\n\0" as *const u8
            as *const libc::c_char,
        stderr,
    );
    fputs(
        b"This is free software, and you are welcome to\n\0" as *const u8
            as *const libc::c_char,
        stderr,
    );
    fputs(
        b"redistribute it under certain conditions; see\n\0" as *const u8
            as *const libc::c_char,
        stderr,
    );
    fputs(
        b"the GNU Public License for details.\n\n\0" as *const u8 as *const libc::c_char,
        stderr,
    );
    if argc < 2 as libc::c_int {
        fputs(
            b"dap: no files to process\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if argc <= 1 as libc::c_int
        || (strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-h\0" as *const u8 as *const libc::c_char,
            ) == 0)
    {
        fputs(
            b"Usage:\ndap [-k] [-d] FILE1.c [ FILE2.c ... ] [-a ARG1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            stderr,
        );
        fputs(
            b"dap [--keep] [--debug] FILE1.c [ FILE2.c ... ] [--args] ARG1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            stderr,
        );
        fputs(
            b"dap [-k] [-d] FILE1.sbs [ FILE2.c ... ]\n\0" as *const u8
                as *const libc::c_char,
            stderr,
        );
        fputs(
            b"dap [--keep] [--debug] FILE1.sbs [ FILE2.c ... ]\n\0" as *const u8
                as *const libc::c_char,
            stderr,
        );
        fputs(
            b"\nReport bugs to <bug-dap@gnu.org>\n\0" as *const u8
                as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    } else if argc == 2 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-v\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            fputs(b"Dap 3.6\n\0" as *const u8 as *const libc::c_char, stderr);
            exit(1 as libc::c_int);
        } else {
            fprintf(
                stderr,
                b"dap: bad option: %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        }
    }
    editor = ecopy(getenv(b"DAPEDITOR\0" as *const u8 as *const libc::c_char));
    if editor.is_null() {
        editor = b"/usr/bin/emacs\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    edopts = ecopy(getenv(b"DAPEDOPTS\0" as *const u8 as *const libc::c_char));
    if edopts.is_null() {
        edopts = 0 as *mut libc::c_char;
    }
    dappp = ecopy(getenv(b"DAPPP\0" as *const u8 as *const libc::c_char));
    if dappp.is_null() {
        dappp = b"/usr/local/bin/dappp\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    dapruns = ecopy(getenv(b"DAPRUNS\0" as *const u8 as *const libc::c_char));
    if dapruns.is_null() {
        dapruns = b"/usr/local/bin/dapruns\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    incdir = ecopy(getenv(b"INCDIR\0" as *const u8 as *const libc::c_char));
    if incdir.is_null() {
        incdir = b"/usr/local/include\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    libdir = ecopy(getenv(b"LIBDIR\0" as *const u8 as *const libc::c_char));
    if libdir.is_null() {
        libdir = b"/usr/local/lib\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    edrun(argc, argv);
    return 0;
}
pub unsafe extern "C" fn srctype(mut name: *mut libc::c_char) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = strlen(name) as libc::c_int;
    if n > 2 as libc::c_int
        && strcmp(
            name.offset(n as isize).offset(-(2 as libc::c_int as isize)),
            b".c\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return 0 as libc::c_int
    } else if n > 4 as libc::c_int
        && strcmp(
            name.offset(n as isize).offset(-(4 as libc::c_int as isize)),
            b".sbs\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return 1 as libc::c_int
    } else {
        fprintf(
            stderr,
            b"dap: name must end in .c or .sbs: %s\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn edrun(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut av: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut argstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argstrlen: libc::c_int = 0;
    let mut argstart: libc::c_int = 0;
    arg = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (argc + 5 as libc::c_int
                    + parseopts(edopts, 0 as *mut *mut libc::c_char)) as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    if arg.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    a = 0 as libc::c_int;
    argstart = 1 as libc::c_int;
    while argstart < argc
        && *(*argv.offset(argstart as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        argstart += 1;
        argstart;
    }
    let fresh0 = a;
    a = a + 1;
    let ref mut fresh1 = *arg.offset(fresh0 as isize);
    *fresh1 = argcpy(*argv.offset(argstart as isize), 0 as libc::c_int);
    a += parseopts(edopts, arg.offset(a as isize));
    let ref mut fresh2 = *arg.offset(a as isize);
    *fresh2 = argcpy(*argv.offset(argstart as isize), 4 as libc::c_int);
    let fresh3 = a;
    a = a + 1;
    suffix(
        *arg.offset(fresh3 as isize),
        b".log\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh4 = *arg.offset(a as isize);
    *fresh4 = argcpy(*argv.offset(argstart as isize), 4 as libc::c_int);
    let fresh5 = a;
    a = a + 1;
    suffix(
        *arg.offset(fresh5 as isize),
        b".lst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    match srctype(*argv.offset(argstart as isize)) {
        0 => {
            let ref mut fresh6 = *arg.offset(a as isize);
            *fresh6 = argcpy(*argv.offset(argstart as isize), 2 as libc::c_int);
            let fresh7 = a;
            a = a + 1;
            suffix(
                *arg.offset(fresh7 as isize),
                b".c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        1 => {
            let ref mut fresh8 = *arg.offset(a as isize);
            *fresh8 = argcpy(*argv.offset(argstart as isize), 4 as libc::c_int);
            let fresh9 = a;
            a = a + 1;
            suffix(
                *arg.offset(fresh9 as isize),
                b".sbs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        _ => {}
    }
    argstrlen = (strlen(b"(shell-command \" &\")\0" as *const u8 as *const libc::c_char))
        .wrapping_add(strlen(dapruns))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    av = 1 as libc::c_int;
    while av < argc {
        argstrlen = (argstrlen as libc::c_ulong)
            .wrapping_add(
                strlen((*argv.offset(av as isize)).offset(1 as libc::c_int as isize)),
            ) as libc::c_int as libc::c_int;
        av += 1;
        av;
    }
    argstr = malloc(argstrlen as libc::c_ulong) as *mut libc::c_char;
    sprintf(
        argstr,
        b"(shell-command \"%s \0" as *const u8 as *const libc::c_char,
        dapruns,
    );
    av = 1 as libc::c_int;
    while av < argc {
        strcat(argstr, *argv.offset(av as isize));
        strcat(argstr, b" \0" as *const u8 as *const libc::c_char);
        av += 1;
        av;
    }
    strcat(argstr, b"&\")\0" as *const u8 as *const libc::c_char);
    let fresh10 = a;
    a = a + 1;
    let ref mut fresh11 = *arg.offset(fresh10 as isize);
    *fresh11 = b"--eval\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let fresh12 = a;
    a = a + 1;
    let ref mut fresh13 = *arg.offset(fresh12 as isize);
    *fresh13 = argstr;
    av = argstart + 1 as libc::c_int;
    while av < argc
        && strcmp(*argv.offset(av as isize), b"-a\0" as *const u8 as *const libc::c_char)
            != 0
        && strcmp(
            *argv.offset(av as isize),
            b"--args\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        let fresh14 = a;
        a = a + 1;
        let ref mut fresh15 = *arg.offset(fresh14 as isize);
        *fresh15 = *argv.offset(av as isize);
        av += 1;
        av;
    }
    let ref mut fresh16 = *arg.offset(a as isize);
    *fresh16 = 0 as *mut libc::c_char;
    execv(editor, arg as *const *mut libc::c_char);
    perror(editor);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn suffix(
    mut name: *mut libc::c_char,
    mut suff: *mut libc::c_char,
) {
    let mut n: libc::c_int = 0;
    n = strlen(name as *const libc::c_char) as libc::c_int;
    if n > 2 as libc::c_int
        && strcmp(
            name.offset(n as isize).offset(-(2 as libc::c_int as isize))
                as *const libc::c_char,
            b".c\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        *name.offset((n - 2 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        strcat(name, suff as *const libc::c_char);
    } else if n > 4 as libc::c_int
        && strcmp(
            name.offset(n as isize).offset(-(4 as libc::c_int as isize))
                as *const libc::c_char,
            b".sbs\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        *name.offset((n - 4 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        strcat(name, suff as *const libc::c_char);
    } else {
        fprintf(
            stderr,
            b"dap: name must end in .c or .sbs: %s\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn argcpy(
    mut arg: *mut libc::c_char,
    mut extra: libc::c_int,
) -> *mut libc::c_char {
    let mut cpy: *mut libc::c_char = 0 as *mut libc::c_char;
    cpy = malloc(
        (strlen(arg as *const libc::c_char))
            .wrapping_add(extra as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if cpy.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    strcpy(cpy, arg as *const libc::c_char);
    return cpy;
}
pub unsafe extern "C" fn parseopts(
    mut opts: *mut libc::c_char,
    mut arg: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut optcpy: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut optlen: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    if opts.is_null() {
        return 0 as libc::c_int;
    }
    if strlen(opts) > optlen as libc::c_ulong {
        if !optcpy.is_null() {
            free(optcpy as *mut libc::c_void);
        }
        optlen = strlen(opts) as libc::c_int;
        optcpy = malloc((optlen + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if optcpy.is_null() {
            perror(b"dap\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    strcpy(optcpy, opts);
    i = 0 as libc::c_int;
    while *optcpy.offset(i as isize) as libc::c_int == ' ' as i32 {
        i += 1;
        i;
    }
    a = 0 as libc::c_int;
    while *optcpy.offset(i as isize) != 0 {
        if !arg.is_null() {
            let ref mut fresh17 = *arg.offset(a as isize);
            *fresh17 = optcpy.offset(i as isize);
        }
        while *optcpy.offset(i as isize) as libc::c_int != 0
            && *optcpy.offset(i as isize) as libc::c_int != ' ' as i32
        {
            i += 1;
            i;
        }
        if *optcpy.offset(i as isize) != 0 {
            if !arg.is_null() {
                *optcpy.offset(i as isize) = '\0' as i32 as libc::c_char;
            }
            i += 1;
            i;
            while *optcpy.offset(i as isize) as libc::c_int == ' ' as i32 {
                i += 1;
                i;
            }
        }
        a += 1;
        a;
    }
    return a;
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

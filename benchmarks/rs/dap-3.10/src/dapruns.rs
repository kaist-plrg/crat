use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut pager: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut pageopts: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut compiler: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut compopts: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut viewer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut viewopts: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut dappp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
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
    let mut keep: libc::c_int = 0;
    let mut debug: libc::c_int = 0;
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
    keep = 0 as libc::c_int;
    debug = 0 as libc::c_int;
    while *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int == '-' as i32
    {
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
        if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-k\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"--keep\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            keep = -(1 as libc::c_int);
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"--debug\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            debug = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0
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
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(0 as libc::c_int as isize),
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
    pager = ecopy(getenv(b"DAPPAGER\0" as *const u8 as *const libc::c_char));
    if pager.is_null() {
        pager = b"/bin/cat\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    pageopts = ecopy(getenv(b"DAPPAGEOPTS\0" as *const u8 as *const libc::c_char));
    if pageopts.is_null() {
        pageopts = 0 as *mut libc::c_char;
    }
    compiler = ecopy(getenv(b"DAPCOMPILER\0" as *const u8 as *const libc::c_char));
    if compiler.is_null() {
        compiler = b"/usr/bin/gcc\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    compopts = ecopy(getenv(b"DAPCOMPOPTS\0" as *const u8 as *const libc::c_char));
    if compopts.is_null() {
        compopts = 0 as *mut libc::c_char;
    }
    viewer = ecopy(getenv(b"DAPVIEWER\0" as *const u8 as *const libc::c_char));
    if viewer.is_null() {
        viewer = b"/usr/bin/X11/gv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    viewopts = ecopy(getenv(b"DAPVIEWOPTS\0" as *const u8 as *const libc::c_char));
    if viewopts.is_null() {
        viewopts = 0 as *mut libc::c_char;
    }
    dappp = ecopy(getenv(b"DAPPP\0" as *const u8 as *const libc::c_char));
    if dappp.is_null() {
        dappp = b"/usr/local/bin/dappp\0" as *const u8 as *const libc::c_char
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
    while !(ask(
        b"Compile and run\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0)
    {
        if dappprun(argc, argv) == 0 {
            if gccrun(argc, argv, debug) == 0 {
                runstat = run(argc, argv, keep);
                view(
                    *argv.offset(1 as libc::c_int as isize),
                    b".err\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if runstat == 0 {
                    if keep == -(1 as libc::c_int) {
                        keep = 1 as libc::c_int;
                    }
                    showps(*argv.offset(1 as libc::c_int as isize));
                }
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dappprun(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut a: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    arg = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if arg.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let ref mut fresh0 = *arg.offset(0 as libc::c_int as isize);
    *fresh0 = dappp;
    a = 1 as libc::c_int;
    while a < argc
        && strcmp(*argv.offset(a as isize), b"-a\0" as *const u8 as *const libc::c_char)
            != 0
        && strcmp(
            *argv.offset(a as isize),
            b"--args\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        let ref mut fresh1 = *arg.offset(a as isize);
        *fresh1 = *argv.offset(a as isize);
        a += 1;
        a;
    }
    let ref mut fresh2 = *arg.offset(a as isize);
    *fresh2 = 0 as *mut libc::c_char;
    pid = fork();
    if pid == 0 {
        fputs(b"Preprocessing...\n\0" as *const u8 as *const libc::c_char, stderr);
        execv(*arg.offset(0 as libc::c_int as isize), arg as *const *mut libc::c_char);
        perror(*arg.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    } else if pid == -(1 as libc::c_int) {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    waitpid(pid, &mut status, 0 as libc::c_int);
    return status;
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
pub unsafe extern "C" fn gccrun(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut debug: libc::c_int,
) -> libc::c_int {
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ncompopts: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut argstart: libc::c_int = 0;
    let mut argend: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    ncompopts = parseopts(compopts, 0 as *mut *mut libc::c_char);
    arg = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 11 as libc::c_int + ncompopts) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if arg.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    g = 0 as libc::c_int;
    let fresh3 = g;
    g = g + 1;
    let ref mut fresh4 = *arg.offset(fresh3 as isize);
    *fresh4 = compiler;
    g += parseopts(compopts, arg.offset(g as isize));
    let fresh5 = g;
    g = g + 1;
    let ref mut fresh6 = *arg.offset(fresh5 as isize);
    *fresh6 = b"-o\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let ref mut fresh7 = *arg.offset(g as isize);
    *fresh7 = argcpy(*argv.offset(1 as libc::c_int as isize), 4 as libc::c_int);
    suffix(
        *arg.offset(g as isize),
        b".dap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    g += 1;
    g;
    let fresh8 = g;
    g = g + 1;
    let ref mut fresh9 = *arg.offset(fresh8 as isize);
    *fresh9 = b"-I\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let fresh10 = g;
    g = g + 1;
    let ref mut fresh11 = *arg.offset(fresh10 as isize);
    *fresh11 = incdir;
    argstart = g;
    a = 1 as libc::c_int;
    while a < argc
        && strcmp(*argv.offset(a as isize), b"-a\0" as *const u8 as *const libc::c_char)
            != 0
        && strcmp(
            *argv.offset(a as isize),
            b"--args\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        let ref mut fresh12 = *arg.offset(g as isize);
        *fresh12 = argcpy(*argv.offset(a as isize), 6 as libc::c_int);
        suffix(
            *arg.offset(g as isize),
            b".dap.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        a += 1;
        a;
        g += 1;
        g;
    }
    argend = g;
    let fresh13 = g;
    g = g + 1;
    let ref mut fresh14 = *arg.offset(fresh13 as isize);
    *fresh14 = b"-L\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let fresh15 = g;
    g = g + 1;
    let ref mut fresh16 = *arg.offset(fresh15 as isize);
    *fresh16 = libdir;
    let fresh17 = g;
    g = g + 1;
    let ref mut fresh18 = *arg.offset(fresh17 as isize);
    *fresh18 = b"-ldap\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let fresh19 = g;
    g = g + 1;
    let ref mut fresh20 = *arg.offset(fresh19 as isize);
    *fresh20 = b"-lm\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let ref mut fresh21 = *arg.offset(g as isize);
    *fresh21 = 0 as *mut libc::c_char;
    pid = fork();
    if pid == 0 {
        fputs(b"Compiling...\n\0" as *const u8 as *const libc::c_char, stderr);
        execv(*arg.offset(0 as libc::c_int as isize), arg as *const *mut libc::c_char);
        perror(*arg.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    } else if pid == -(1 as libc::c_int) {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    waitpid(pid, &mut status, 0 as libc::c_int);
    if status == 0 && debug == 0 {
        g = argstart;
        while g < argend {
            unlink(*arg.offset(g as isize));
            g += 1;
            g;
        }
    }
    return status;
}
pub unsafe extern "C" fn run(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut keep: libc::c_int,
) -> libc::c_int {
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lstname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    arg = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if arg.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let ref mut fresh22 = *arg.offset(0 as libc::c_int as isize);
    *fresh22 = argcpy(*argv.offset(1 as libc::c_int as isize), 4 as libc::c_int);
    suffix(
        *arg.offset(0 as libc::c_int as isize),
        b".dap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    lstname = argcpy(*argv.offset(1 as libc::c_int as isize), 4 as libc::c_int);
    suffix(lstname, b".lst\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if keep != 1 as libc::c_int {
        unlink(lstname);
    }
    psname = argcpy(*argv.offset(1 as libc::c_int as isize), 3 as libc::c_int);
    suffix(psname, b".ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    unlink(psname);
    a = 1 as libc::c_int;
    while a < argc
        && strcmp(*argv.offset(a as isize), b"-a\0" as *const u8 as *const libc::c_char)
            != 0
        && strcmp(
            *argv.offset(a as isize),
            b"--args\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        a += 1;
        a;
    }
    g = 1 as libc::c_int;
    a += 1;
    a;
    while a < argc {
        let ref mut fresh23 = *arg.offset(g as isize);
        *fresh23 = argcpy(*argv.offset(a as isize), 0 as libc::c_int);
        a += 1;
        a;
        g += 1;
        g;
    }
    let ref mut fresh24 = *arg.offset(g as isize);
    *fresh24 = 0 as *mut libc::c_char;
    pid = fork();
    if pid == 0 {
        fputs(b"Executing...\n\0" as *const u8 as *const libc::c_char, stderr);
        execv(*arg.offset(0 as libc::c_int as isize), arg as *const *mut libc::c_char);
        perror(*arg.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    } else if pid == -(1 as libc::c_int) {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    waitpid(pid, &mut status, 0 as libc::c_int);
    return status;
}
pub unsafe extern "C" fn ask(mut question: *mut libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        fprintf(stderr, b"%s? [y/q] \0" as *const u8 as *const libc::c_char, question);
        c = getchar();
        while getchar() != '\n' as i32 {}
        if c != 'y' as i32 && c != 'q' as i32 {
            fprintf(stderr, b"Invalid response. \0" as *const u8 as *const libc::c_char);
        }
        if !(c != 'y' as i32 && c != 'q' as i32) {
            break;
        }
    }
    return (c == 'y' as i32) as libc::c_int;
}
pub unsafe extern "C" fn view(mut name: *mut libc::c_char, mut suff: *mut libc::c_char) {
    let mut lstname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut a: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut pid: pid_t = 0;
    arg = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (3 as libc::c_int + parseopts(pageopts, 0 as *mut *mut libc::c_char))
                    as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    if arg.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    lstname = argcpy(name, strlen(suff as *const libc::c_char) as libc::c_int);
    suffix(lstname, suff);
    if stat(lstname, &mut buf) == 0 && buf.st_size != 0 {
        a = 0 as libc::c_int;
        let fresh25 = a;
        a = a + 1;
        let ref mut fresh26 = *arg.offset(fresh25 as isize);
        *fresh26 = pager;
        a += parseopts(pageopts, arg.offset(a as isize));
        let fresh27 = a;
        a = a + 1;
        let ref mut fresh28 = *arg.offset(fresh27 as isize);
        *fresh28 = lstname;
        let ref mut fresh29 = *arg.offset(a as isize);
        *fresh29 = 0 as *mut libc::c_char;
        pid = fork();
        if pid == 0 {
            execv(
                *arg.offset(0 as libc::c_int as isize),
                arg as *const *mut libc::c_char,
            );
            perror(*arg.offset(0 as libc::c_int as isize));
            exit(1 as libc::c_int);
        } else if pid == -(1 as libc::c_int) {
            perror(b"dap\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        waitpid(pid, &mut status, 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn showps(mut name: *mut libc::c_char) {
    let mut psname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut a: libc::c_int = 0;
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    static mut gv: libc::c_int = 0 as libc::c_int;
    let mut pid: pid_t = 0;
    arg = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (3 as libc::c_int + parseopts(viewopts, 0 as *mut *mut libc::c_char))
                    as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    if arg.is_null() {
        perror(b"dap\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    psname = argcpy(name, 3 as libc::c_int);
    suffix(psname, b".ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if gv == 0 && stat(psname, &mut buf) == 0 {
        gv = 1 as libc::c_int;
        a = 0 as libc::c_int;
        let fresh30 = a;
        a = a + 1;
        let ref mut fresh31 = *arg.offset(fresh30 as isize);
        *fresh31 = viewer;
        a += parseopts(viewopts, arg.offset(a as isize));
        let fresh32 = a;
        a = a + 1;
        let ref mut fresh33 = *arg.offset(fresh32 as isize);
        *fresh33 = psname;
        let fresh34 = a;
        a = a + 1;
        let ref mut fresh35 = *arg.offset(fresh34 as isize);
        *fresh35 = 0 as *mut libc::c_char;
        pid = fork();
        if pid == 0 {
            execv(
                *arg.offset(0 as libc::c_int as isize),
                arg as *const *mut libc::c_char,
            );
            perror(*arg.offset(0 as libc::c_int as isize));
            exit(1 as libc::c_int);
        } else if pid == -(1 as libc::c_int) {
            perror(b"dap\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
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
            let ref mut fresh36 = *arg.offset(a as isize);
            *fresh36 = optcpy.offset(i as isize);
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

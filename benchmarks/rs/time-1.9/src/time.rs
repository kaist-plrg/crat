use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn getpagesize() -> libc::c_int;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn resuse_start(resp: *mut RESUSE);
    fn resuse_end(pid: pid_t, resp: *mut RESUSE) -> libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RESUSE {
    pub waitstatus: libc::c_int,
    pub ru: rusage,
    pub start: timeval,
    pub elapsed: timeval,
}
pub type C2RustUnnamed_13 = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed_13 = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed_13 = 126;
pub const EXIT_CANCELED: C2RustUnnamed_13 = 125;
pub type sighandler = Option::<unsafe extern "C" fn() -> ()>;
#[inline]
unsafe extern "C" fn get_rusage_maxrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed.ru_maxrss as uintmax_t;
}
#[inline]
unsafe extern "C" fn get_rusage_ixrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed_0.ru_ixrss as uintmax_t;
}
#[inline]
unsafe extern "C" fn get_rusage_idrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed_1.ru_idrss as uintmax_t;
}
#[inline]
unsafe extern "C" fn get_rusage_isrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed_2.ru_isrss as uintmax_t;
}
static mut default_format: *const libc::c_char = b"%Uuser %Ssystem %Eelapsed %PCPU (%Xavgtext+%Davgdata %Mmaxresident)k\n%Iinputs+%Ooutputs (%Fmajor+%Rminor)pagefaults %Wswaps\0"
    as *const u8 as *const libc::c_char;
static mut posix_format: *const libc::c_char = b"real %e\nuser %U\nsys %S\0" as *const u8
    as *const libc::c_char;
static mut longstats: [*const libc::c_char; 24] = [
    b"\tCommand being timed: \"%C\"\n\0" as *const u8 as *const libc::c_char,
    b"\tUser time (seconds): %U\n\0" as *const u8 as *const libc::c_char,
    b"\tSystem time (seconds): %S\n\0" as *const u8 as *const libc::c_char,
    b"\tPercent of CPU this job got: %P\n\0" as *const u8 as *const libc::c_char,
    b"\tElapsed (wall clock) time (h:mm:ss or m:ss): %E\n\0" as *const u8
        as *const libc::c_char,
    b"\tAverage shared text size (kbytes): %X\n\0" as *const u8 as *const libc::c_char,
    b"\tAverage unshared data size (kbytes): %D\n\0" as *const u8 as *const libc::c_char,
    b"\tAverage stack size (kbytes): %p\n\0" as *const u8 as *const libc::c_char,
    b"\tAverage total size (kbytes): %K\n\0" as *const u8 as *const libc::c_char,
    b"\tMaximum resident set size (kbytes): %M\n\0" as *const u8 as *const libc::c_char,
    b"\tAverage resident set size (kbytes): %t\n\0" as *const u8 as *const libc::c_char,
    b"\tMajor (requiring I/O) page faults: %F\n\0" as *const u8 as *const libc::c_char,
    b"\tMinor (reclaiming a frame) page faults: %R\n\0" as *const u8
        as *const libc::c_char,
    b"\tVoluntary context switches: %w\n\0" as *const u8 as *const libc::c_char,
    b"\tInvoluntary context switches: %c\n\0" as *const u8 as *const libc::c_char,
    b"\tSwaps: %W\n\0" as *const u8 as *const libc::c_char,
    b"\tFile system inputs: %I\n\0" as *const u8 as *const libc::c_char,
    b"\tFile system outputs: %O\n\0" as *const u8 as *const libc::c_char,
    b"\tSocket messages sent: %s\n\0" as *const u8 as *const libc::c_char,
    b"\tSocket messages received: %r\n\0" as *const u8 as *const libc::c_char,
    b"\tSignals delivered: %k\n\0" as *const u8 as *const libc::c_char,
    b"\tPage size (bytes): %Z\n\0" as *const u8 as *const libc::c_char,
    b"\tExit status: %x\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut verbose: bool = false;
static mut outfile: *const libc::c_char = 0 as *const libc::c_char;
static mut outfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut append: bool = false;
static mut output_format: *const libc::c_char = 0 as *const libc::c_char;
static mut quiet: bool = false;
static mut longopts: [option; 9] = [
    {
        let mut init = option {
            name: b"append\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
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
            name: b"output-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"portability\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
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
unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Try '%s --help' for more information.\n\0" as *const u8
                as *const libc::c_char,
            program_name,
        );
        exit(status);
    }
    printf(
        b"Usage: %s [OPTIONS] COMMAND [ARG]...\n\0" as *const u8 as *const libc::c_char,
        program_name,
    );
    fputs(
        b"Run COMMAND, then print system resource usage.\n\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -a, --append              with -o FILE, append instead of overwriting\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -f, --format=FORMAT       use the specified FORMAT instead of the default\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -o, --output=FILE         write to FILE instead of STDERR\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -p, --portability         print POSIX standard 1003.2 conformant string:\n                              real %%e\n                              user %%U\n                              sys %%S\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -q, --quiet               do not print information about abnormal program\n                            termination (non-zero exit codes or signals)\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -v, --verbose             print all resource usage information instead of\n                            the default format\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -h,  --help               display this help and exit\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  -V,  --version            output version information and exit\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"\nCommonly usaged format sequences for -f/--format:\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"(see documentation for full list)\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(b"  %%   a literal '%'\n\0" as *const u8 as *const libc::c_char, stdout);
    fputs(
        b"  %C   command line and arguments\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %c   involuntary context switches\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %E   elapsed real time (wall clock) in [hour:]min:sec\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %e   elapsed real time (wall clock) in seconds\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(b"  %F   major page faults\n\0" as *const u8 as *const libc::c_char, stdout);
    fputs(
        b"  %M   maximum resident set size in KB\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %P   percent of CPU this job got\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(b"  %R   minor page faults\n\0" as *const u8 as *const libc::c_char, stdout);
    fputs(
        b"  %S   system (kernel) time in seconds\n\0" as *const u8
            as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %U   user time in seconds\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %w   voluntary context switches\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(
        b"  %x   exit status of command\n\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fputs(b"\nDefault output format:\n\0" as *const u8 as *const libc::c_char, stdout);
    fputs(default_format, stdout);
    fputc('\n' as i32, stdout);
    printf(
        b"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
            as *const u8 as *const libc::c_char,
        b"time\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"\n%s website: <%s>\n\0" as *const u8 as *const libc::c_char,
        b"GNU Time\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/time/\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Full documentation at: <%smanual>\n\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/time/\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"E-mail bug reports to: %s\n\0" as *const u8 as *const libc::c_char,
        b"bug-time@gnu.org\0" as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
unsafe extern "C" fn fprintargv(
    mut fp: *mut FILE,
    mut argv: *const *const libc::c_char,
    mut filler: *const libc::c_char,
) {
    let mut av: *const *const libc::c_char = 0 as *const *const libc::c_char;
    av = argv;
    fputs(*av, fp);
    loop {
        av = av.offset(1);
        if (*av).is_null() {
            break;
        }
        fputs(filler, fp);
        fputs(*av, fp);
    }
    if ferror(fp) != 0 {
        error(
            1 as libc::c_int,
            *__errno_location(),
            b"write error\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn linear_argv(
    mut argv: *const *const libc::c_char,
) -> *mut libc::c_char {
    let mut s: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: libc::c_int = 0;
    size = 1 as libc::c_int;
    s = argv;
    while !(*s).is_null() {
        size = (size as libc::c_ulong).wrapping_add(strlen(*s)) as libc::c_int
            as libc::c_int;
        s = s.offset(1);
        s;
    }
    new = malloc(size as libc::c_ulong) as *mut libc::c_char;
    if new.is_null() {
        fprintf(
            stderr,
            b"%s: virtual memory exhausted\n\0" as *const u8 as *const libc::c_char,
            program_name,
        );
        return 0 as *mut libc::c_char;
    }
    s = argv;
    sp = *s;
    dp = new;
    while !(*s).is_null() {
        sp = *s;
        loop {
            let fresh0 = sp;
            sp = sp.offset(1);
            let fresh1 = dp;
            dp = dp.offset(1);
            *fresh1 = *fresh0;
            if !(*fresh1 as libc::c_int != '\0' as i32) {
                break;
            }
        }
        dp = dp.offset(-1);
        dp;
        s = s.offset(1);
        s;
    }
    return new;
}
unsafe extern "C" fn summarize(
    mut fp: *mut FILE,
    mut fmt: *const libc::c_char,
    mut command: *mut *const libc::c_char,
    mut resp: *mut RESUSE,
) {
    let mut r: libc::c_ulong = 0;
    let mut v: libc::c_ulong = 0;
    let mut us_r: libc::c_ulong = 0;
    let mut us_v: libc::c_ulong = 0;
    if !quiet && output_format != posix_format {
        if (*resp).waitstatus & 0xff as libc::c_int == 0x7f as libc::c_int {
            fprintf(
                fp,
                b"Command stopped by signal %d\n\0" as *const u8 as *const libc::c_char,
                ((*resp).waitstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int,
            );
        } else if (((*resp).waitstatus & 0x7f as libc::c_int) + 1 as libc::c_int)
            as libc::c_schar as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            fprintf(
                fp,
                b"Command terminated by signal %d\n\0" as *const u8
                    as *const libc::c_char,
                (*resp).waitstatus & 0x7f as libc::c_int,
            );
        } else if (*resp).waitstatus & 0x7f as libc::c_int == 0 as libc::c_int
            && ((*resp).waitstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
        {
            fprintf(
                fp,
                b"Command exited with non-zero status %d\n\0" as *const u8
                    as *const libc::c_char,
                ((*resp).waitstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int,
            );
        }
    }
    r = ((*resp).elapsed.tv_sec * 1000 as libc::c_int as libc::c_long
        + (*resp).elapsed.tv_usec / 1000 as libc::c_int as libc::c_long)
        as libc::c_ulong;
    v = ((*resp).ru.ru_utime.tv_sec * 1000 as libc::c_int as libc::c_long
        + (*resp).ru.ru_utime.tv_usec / 1000 as libc::c_int as libc::c_long
        + (*resp).ru.ru_stime.tv_sec * 1000 as libc::c_int as libc::c_long
        + (*resp).ru.ru_stime.tv_usec / 1000 as libc::c_int as libc::c_long)
        as libc::c_ulong;
    us_r = (*resp).elapsed.tv_usec as libc::c_ulong;
    us_v = ((*resp).ru.ru_utime.tv_usec + (*resp).ru.ru_stime.tv_usec) as libc::c_ulong;
    while *fmt != 0 {
        match *fmt as libc::c_int {
            37 => {
                fmt = fmt.offset(1);
                match *fmt as libc::c_int {
                    37 => {
                        putc('%' as i32, fp);
                    }
                    67 => {
                        fprintargv(
                            fp,
                            command,
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    68 => {
                        fprintf(
                            fp,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            if v
                                .wrapping_div(
                                    (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                ) == 0 as libc::c_int as libc::c_ulong
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (get_rusage_idrss_kb(&mut (*resp).ru))
                                    .wrapping_div(
                                        v
                                            .wrapping_div(
                                                (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                            ),
                                    )
                                    .wrapping_add(
                                        (get_rusage_isrss_kb(&mut (*resp).ru))
                                            .wrapping_div(
                                                v
                                                    .wrapping_div(
                                                        (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                                    ),
                                            ),
                                    )
                            },
                        );
                    }
                    69 => {
                        if (*resp).elapsed.tv_sec >= 3600 as libc::c_int as libc::c_long
                        {
                            fprintf(
                                fp,
                                b"%ld:%02ld:%02ld\0" as *const u8 as *const libc::c_char,
                                (*resp).elapsed.tv_sec
                                    / 3600 as libc::c_int as libc::c_long,
                                (*resp).elapsed.tv_sec % 3600 as libc::c_int as libc::c_long
                                    / 60 as libc::c_int as libc::c_long,
                                (*resp).elapsed.tv_sec % 60 as libc::c_int as libc::c_long,
                            );
                        } else {
                            fprintf(
                                fp,
                                b"%ld:%02ld.%02ld\0" as *const u8 as *const libc::c_char,
                                (*resp).elapsed.tv_sec / 60 as libc::c_int as libc::c_long,
                                (*resp).elapsed.tv_sec % 60 as libc::c_int as libc::c_long,
                                (*resp).elapsed.tv_usec
                                    / 10000 as libc::c_int as libc::c_long,
                            );
                        }
                    }
                    70 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_4.ru_majflt,
                        );
                    }
                    73 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_6.ru_inblock,
                        );
                    }
                    75 => {
                        fprintf(
                            fp,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            if v
                                .wrapping_div(
                                    (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                ) == 0 as libc::c_int as libc::c_ulong
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (get_rusage_idrss_kb(&mut (*resp).ru))
                                    .wrapping_div(
                                        v
                                            .wrapping_div(
                                                (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                            ),
                                    )
                                    .wrapping_add(
                                        (get_rusage_isrss_kb(&mut (*resp).ru))
                                            .wrapping_div(
                                                v
                                                    .wrapping_div(
                                                        (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                                    ),
                                            ),
                                    )
                                    .wrapping_add(
                                        (get_rusage_ixrss_kb(&mut (*resp).ru))
                                            .wrapping_div(
                                                v
                                                    .wrapping_div(
                                                        (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                                    ),
                                            ),
                                    )
                            },
                        );
                    }
                    77 => {
                        fprintf(
                            fp,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            get_rusage_maxrss_kb(&mut (*resp).ru),
                        );
                    }
                    79 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_7.ru_oublock,
                        );
                    }
                    80 => {
                        if r > 0 as libc::c_int as libc::c_ulong {
                            fprintf(
                                fp,
                                b"%lu%%\0" as *const u8 as *const libc::c_char,
                                v
                                    .wrapping_mul(100 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(r),
                            );
                        } else if us_r > 0 as libc::c_int as libc::c_ulong {
                            fprintf(
                                fp,
                                b"%lu%%\0" as *const u8 as *const libc::c_char,
                                us_v
                                    .wrapping_mul(100 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(us_r),
                            );
                        } else {
                            fprintf(fp, b"?%%\0" as *const u8 as *const libc::c_char);
                        }
                    }
                    82 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_3.ru_minflt,
                        );
                    }
                    83 => {
                        fprintf(
                            fp,
                            b"%ld.%02ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.ru_stime.tv_sec,
                            (*resp).ru.ru_stime.tv_usec
                                / 1000 as libc::c_int as libc::c_long
                                / 10 as libc::c_int as libc::c_long,
                        );
                    }
                    85 => {
                        fprintf(
                            fp,
                            b"%ld.%02ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.ru_utime.tv_sec,
                            (*resp).ru.ru_utime.tv_usec
                                / 1000 as libc::c_int as libc::c_long
                                / 10 as libc::c_int as libc::c_long,
                        );
                    }
                    87 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_5.ru_nswap,
                        );
                    }
                    88 => {
                        fprintf(
                            fp,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            if v
                                .wrapping_div(
                                    (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                ) == 0 as libc::c_int as libc::c_ulong
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (get_rusage_ixrss_kb(&mut (*resp).ru))
                                    .wrapping_div(
                                        v
                                            .wrapping_div(
                                                (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                            ),
                                    )
                            },
                        );
                    }
                    90 => {
                        fprintf(
                            fp,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            getpagesize(),
                        );
                    }
                    99 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_12.ru_nivcsw,
                        );
                    }
                    101 => {
                        fprintf(
                            fp,
                            b"%ld.%02ld\0" as *const u8 as *const libc::c_char,
                            (*resp).elapsed.tv_sec,
                            (*resp).elapsed.tv_usec
                                / 10000 as libc::c_int as libc::c_long,
                        );
                    }
                    107 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_10.ru_nsignals,
                        );
                    }
                    112 => {
                        fprintf(
                            fp,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            if v
                                .wrapping_div(
                                    (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                ) == 0 as libc::c_int as libc::c_ulong
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (get_rusage_isrss_kb(&mut (*resp).ru))
                                    .wrapping_div(
                                        v
                                            .wrapping_div(
                                                (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                            ),
                                    )
                            },
                        );
                    }
                    114 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_9.ru_msgrcv,
                        );
                    }
                    115 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_8.ru_msgsnd,
                        );
                    }
                    116 => {
                        fprintf(
                            fp,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            if v
                                .wrapping_div(
                                    (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                ) == 0 as libc::c_int as libc::c_ulong
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (get_rusage_idrss_kb(&mut (*resp).ru))
                                    .wrapping_div(
                                        v
                                            .wrapping_div(
                                                (1000 as libc::c_int / 100 as libc::c_int) as libc::c_ulong,
                                            ),
                                    )
                            },
                        );
                    }
                    119 => {
                        fprintf(
                            fp,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*resp).ru.c2rust_unnamed_11.ru_nvcsw,
                        );
                    }
                    120 => {
                        fprintf(
                            fp,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            ((*resp).waitstatus & 0xff00 as libc::c_int)
                                >> 8 as libc::c_int,
                        );
                    }
                    0 => {
                        putc('?' as i32, fp);
                        return;
                    }
                    _ => {
                        putc('?' as i32, fp);
                        putc(*fmt as libc::c_int, fp);
                    }
                }
                fmt = fmt.offset(1);
                fmt;
            }
            92 => {
                fmt = fmt.offset(1);
                match *fmt as libc::c_int {
                    116 => {
                        putc('\t' as i32, fp);
                    }
                    110 => {
                        putc('\n' as i32, fp);
                    }
                    92 => {
                        putc('\\' as i32, fp);
                    }
                    _ => {
                        putc('?' as i32, fp);
                        putc('\\' as i32, fp);
                        putc(*fmt as libc::c_int, fp);
                    }
                }
                fmt = fmt.offset(1);
                fmt;
            }
            _ => {
                let fresh2 = fmt;
                fmt = fmt.offset(1);
                putc(*fresh2 as libc::c_int, fp);
            }
        }
        if ferror(fp) != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"write error\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    putc('\n' as i32, fp);
    if ferror(fp) != 0 {
        error(
            1 as libc::c_int,
            *__errno_location(),
            b"write error\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn getargs(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut *const libc::c_char {
    let mut optc: libc::c_int = 0;
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    verbose = 0 as libc::c_int != 0;
    outfile = 0 as *const libc::c_char;
    outfp = stderr;
    append = 0 as libc::c_int != 0;
    output_format = default_format;
    format = getenv(b"TIME\0" as *const u8 as *const libc::c_char);
    if !format.is_null() {
        output_format = format;
    }
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"+af:o:pqvV\0" as *const u8 as *const libc::c_char,
            longopts.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_17: u64;
        match optc {
            97 => {
                append = 1 as libc::c_int != 0;
                current_block_17 = 2838571290723028321;
            }
            102 => {
                output_format = optarg;
                current_block_17 = 2838571290723028321;
            }
            104 => {
                usage(0 as libc::c_int);
                current_block_17 = 1466994801288054717;
            }
            111 => {
                current_block_17 = 1466994801288054717;
            }
            112 => {
                output_format = posix_format;
                current_block_17 = 2838571290723028321;
            }
            113 => {
                quiet = 1 as libc::c_int != 0;
                current_block_17 = 2838571290723028321;
            }
            118 => {
                verbose = 1 as libc::c_int != 0;
                current_block_17 = 2838571290723028321;
            }
            86 => {
                version_etc(
                    stdout,
                    b"time\0" as *const u8 as *const libc::c_char,
                    b"GNU Time\0" as *const u8 as *const libc::c_char,
                    b"1.9\0" as *const u8 as *const libc::c_char,
                    b"David Keppel\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Assaf Gordon\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(EXIT_CANCELED as libc::c_int);
                current_block_17 = 2838571290723028321;
            }
        }
        match current_block_17 {
            1466994801288054717 => {
                outfile = optarg;
            }
            _ => {}
        }
    }
    if optind == argc {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"missing program to run\0" as *const u8 as *const libc::c_char,
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    if !outfile.is_null() {
        if append {
            outfp = fopen(outfile, b"a\0" as *const u8 as *const libc::c_char);
        } else {
            outfp = fopen(outfile, b"w\0" as *const u8 as *const libc::c_char);
        }
        if outfp.is_null() {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                outfile,
            );
        }
    }
    if verbose {
        output_format = linear_argv(longstats.as_ptr()) as *const libc::c_char;
        if output_format.is_null() {
            exit(EXIT_CANCELED as libc::c_int);
        }
    }
    return &mut *argv.offset(optind as isize) as *mut *mut libc::c_char
        as *mut *const libc::c_char;
}
unsafe extern "C" fn run_command(
    mut cmd: *const *mut libc::c_char,
    mut resp: *mut RESUSE,
) {
    let mut pid: pid_t = 0;
    let mut interrupt_signal: sighandler = None;
    let mut quit_signal: sighandler = None;
    let mut saved_errno: libc::c_int = 0;
    resuse_start(resp);
    pid = fork();
    if pid < 0 as libc::c_int {
        error(
            EXIT_CANCELED as libc::c_int,
            *__errno_location(),
            b"cannot fork\0" as *const u8 as *const libc::c_char,
        );
    } else if pid == 0 as libc::c_int {
        execvp(*cmd.offset(0 as libc::c_int as isize), cmd);
        saved_errno = *__errno_location();
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"cannot run %s\0" as *const u8 as *const libc::c_char,
            *cmd.offset(0 as libc::c_int as isize),
        );
        _exit(
            if saved_errno == 2 as libc::c_int {
                EXIT_ENOENT as libc::c_int
            } else {
                EXIT_CANNOT_INVOKE as libc::c_int
            },
        );
    }
    interrupt_signal = ::std::mem::transmute::<
        __sighandler_t,
        sighandler,
    >(
        signal(
            2 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        ),
    );
    quit_signal = ::std::mem::transmute::<
        __sighandler_t,
        sighandler,
    >(
        signal(
            3 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        ),
    );
    if resuse_end(pid, resp) == 0 as libc::c_int {
        error(
            1 as libc::c_int,
            *__errno_location(),
            b"error waiting for child process\0" as *const u8 as *const libc::c_char,
        );
    }
    signal(
        2 as libc::c_int,
        ::std::mem::transmute::<sighandler, __sighandler_t>(interrupt_signal),
    );
    signal(
        3 as libc::c_int,
        ::std::mem::transmute::<sighandler, __sighandler_t>(quit_signal),
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut command_line: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut res: RESUSE = RESUSE {
        waitstatus: 0,
        ru: rusage {
            ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
            c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
            c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
            c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
            c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
            c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
            c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
            c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
            c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
            c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
            c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
            c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
            c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
            c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
            c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
        },
        start: timeval { tv_sec: 0, tv_usec: 0 },
        elapsed: timeval { tv_sec: 0, tv_usec: 0 },
    };
    let mut status: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    command_line = getargs(argc, argv);
    run_command(command_line as *const *mut libc::c_char, &mut res);
    summarize(outfp, output_format, command_line, &mut res);
    fflush(outfp);
    if res.waitstatus & 0xff as libc::c_int == 0x7f as libc::c_int {
        status = ((res.waitstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int)
            + 128 as libc::c_int;
    } else if ((res.waitstatus & 0x7f as libc::c_int) + 1 as libc::c_int)
        as libc::c_schar as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        status = (res.waitstatus & 0x7f as libc::c_int) + 128 as libc::c_int;
    } else if res.waitstatus & 0x7f as libc::c_int == 0 as libc::c_int {
        status = (res.waitstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int;
    } else {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"unknown status from command (%d)\0" as *const u8 as *const libc::c_char,
            res.waitstatus,
        );
        status = 1 as libc::c_int;
    }
    return status;
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

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    static ctab: [tokens; 0];
    fn thank_you_and_goodnight(how: libc::c_int);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn __errno_location() -> *mut libc::c_int;
    fn display_version(prog: *const program, flags: libc::c_int);
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn complain(fmt: *const libc::c_char, _: ...);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    static mut exit_failure: libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type off_t = __off_t;
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
pub type tokens = libc::c_uint;
pub const STRING: tokens = 14;
pub const SEMI: tokens = 13;
pub const NUM: tokens = 12;
pub const ID: tokens = 11;
pub const COLON: tokens = 10;
pub const UNKN: tokens = 9;
pub const SPACE: tokens = 8;
pub const SBEGIN: tokens = 7;
pub const PERIOD: tokens = 6;
pub const Letter: tokens = 5;
pub const LETTER: tokens = 4;
pub const NEWLN: tokens = 3;
pub const IDCHAR: tokens = 2;
pub const DIGIT: tokens = 1;
pub const DELIM: tokens = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const libc::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const libc::c_char,
    pub date: *const libc::c_char,
    pub author: *const libc::c_char,
    pub lockedby: *const libc::c_char,
    pub state: *const libc::c_char,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const libc::c_char,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const libc::c_char,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const libc::c_char,
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub help: *const libc::c_char,
    pub tyag: libc::c_int,
}
pub type maker = libc::c_uint;
pub const effective: maker = 2;
pub const real: maker = 1;
pub const notmade: maker = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const libc::c_char,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const libc::c_char,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: libc::c_int,
    pub stick_with_euid: bool,
    pub ruid: libc::c_int,
    pub euid: libc::c_int,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: libc::c_int,
    pub pe: *const libc::c_char,
    pub zone_offset: zone_offset,
    pub username: *mut libc::c_char,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut libc::c_char,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut libc::c_char,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub valid: bool,
    pub author: *mut libc::c_char,
    pub date: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub rev: *mut libc::c_char,
    pub state: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const libc::c_char,
    pub branch: *const libc::c_char,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: libc::c_int,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const libc::c_char,
    pub fd_lock: libc::c_int,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const libc::c_char,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
static mut ident_blurb: [libc::c_char; 39] = unsafe {
    *::std::mem::transmute::<
        &[u8; 39],
        &[libc::c_char; 39],
    >(b"Identify RCS keyword strings in files.\0")
};
static mut ident_help: [libc::c_char; 176] = unsafe {
    *::std::mem::transmute::<
        &[u8; 176],
        &[libc::c_char; 176],
    >(
        b"[options] [file ...]\nOptions:\n  -q            Suppress warnings if no patterns are found.\n  -V            Obsolete; do not use.\n\nIf no FILE is specified, scan standard input.\n\0",
    )
};
pub static mut top: *mut top = 0 as *const top as *mut top;
unsafe extern "C" fn match_0(mut fp: *mut FILE) -> libc::c_int {
    let mut line: [libc::c_char; 8192] = [0; 8192];
    let mut c: libc::c_int = 0;
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subversion_style: bool = 0 as libc::c_int != 0;
    tp = line.as_mut_ptr();
    loop {
        c = getc(fp);
        if !(c != ':' as i32) {
            break;
        }
        if c == -(1 as libc::c_int) && feof(fp) | ferror(fp) != 0 {
            return c;
        }
        's_43: {
            match *ctab.as_ptr().offset(c as isize) as libc::c_uint {
                4 | 5 => {
                    let fresh0 = tp;
                    tp = tp.offset(1);
                    *fresh0 = c as libc::c_char;
                    if tp
                        < line
                            .as_mut_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong as isize,
                            )
                            .offset(-(4 as libc::c_int as isize))
                    {
                        break 's_43;
                    }
                }
                _ => {}
            }
            return if c != 0 { c } else { '\n' as i32 };
        }
    }
    if tp == line.as_mut_ptr() {
        return c;
    }
    let fresh1 = tp;
    tp = tp.offset(1);
    *fresh1 = c as libc::c_char;
    c = getc(fp);
    if ':' as i32 == c {
        subversion_style = 1 as libc::c_int != 0;
        let fresh2 = tp;
        tp = tp.offset(1);
        *fresh2 = c as libc::c_char;
        c = getc(fp);
    }
    if c != ' ' as i32 {
        return if c != 0 { c } else { '\n' as i32 };
    }
    let fresh3 = tp;
    tp = tp.offset(1);
    *fresh3 = c as libc::c_char;
    loop {
        c = getc(fp);
        if !(c != '$' as i32) {
            break;
        }
        if c == -(1 as libc::c_int) && feof(fp) | ferror(fp) != 0 {
            return c;
        }
        's_119: {
            match *ctab.as_ptr().offset(c as isize) as libc::c_uint {
                3 | 9 => {}
                _ => {
                    let fresh4 = tp;
                    tp = tp.offset(1);
                    *fresh4 = c as libc::c_char;
                    if tp
                        < line
                            .as_mut_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong as isize,
                            )
                            .offset(-(2 as libc::c_int as isize))
                    {
                        break 's_119;
                    }
                }
            }
            return if c != 0 { c } else { '\n' as i32 };
        }
    }
    if !(' ' as i32 == *tp.offset(-(1 as libc::c_int) as isize) as libc::c_int
        || subversion_style as libc::c_int != 0
            && '#' as i32 == *tp.offset(-(1 as libc::c_int) as isize) as libc::c_int)
    {
        return c;
    }
    let fresh5 = tp;
    tp = tp.offset(1);
    *fresh5 = c as libc::c_char;
    *tp = '\0' as i32 as libc::c_char;
    printf(
        b"     %c%s\n\0" as *const u8 as *const libc::c_char,
        '$' as i32,
        line.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn scanfile(
    mut file: *mut FILE,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if !name.is_null() {
        printf(b"%s:\n\0" as *const u8 as *const libc::c_char, name);
        if ferror(stdout) != 0 {
            return -(1 as libc::c_int);
        }
    } else {
        name = b"standard input\0" as *const u8 as *const libc::c_char;
    }
    c = 0 as libc::c_int;
    while c != -(1 as libc::c_int) || feof(file) | ferror(file) == 0 {
        if c == '$' as i32 {
            c = match_0(file);
            if c != 0 {
                continue;
            }
            if ferror(stdout) != 0 {
                return -(1 as libc::c_int);
            }
            (*top).behavior.quiet = 1 as libc::c_int != 0;
        }
        c = getc(file);
    }
    if ferror(file) != 0 || 0 as libc::c_int > fclose(file) {
        syserror(*__errno_location(), name);
        fflush(stdout);
        thank_you_and_goodnight((*(*top).program).tyag);
    }
    if !(*top).behavior.quiet {
        complain(
            b"%s warning: no id keywords in %s\n\0" as *const u8 as *const libc::c_char,
            (*(*top).program).name,
            name,
        );
    }
    return 0 as libc::c_int;
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: ident_blurb.as_ptr(),
            help: ident_help.as_ptr(),
            tyag: 0 as libc::c_int,
        };
        init
    }
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut a: *const libc::c_char = 0 as *const libc::c_char;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = b"ident\0" as *const u8 as *const libc::c_char;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    loop {
        argv = argv.offset(1);
        a = *argv;
        if !(!a.is_null() && *a as libc::c_int == '-' as i32) {
            break;
        }
        loop {
            a = a.offset(1);
            if !(*a != 0) {
                break;
            }
            's_77: {
                match *a as libc::c_int {
                    113 => {
                        (*top).behavior.quiet = 1 as libc::c_int != 0;
                        break 's_77;
                    }
                    86 => {
                        if *a.offset(1 as libc::c_int as isize) == 0 {
                            display_version(&mut program, 1 as libc::c_int);
                            gnurcs_goodbye();
                            return 0 as libc::c_int;
                        }
                    }
                    _ => {}
                }
                bad_option(a.offset(-(1 as libc::c_int as isize)));
                gnurcs_goodbye();
                return exit_failure;
            }
        }
    }
    if a.is_null() {
        scanfile(stdin, 0 as *const libc::c_char);
    } else {
        loop {
            fp = fopen(a, b"r\0" as *const u8 as *const libc::c_char);
            if fp.is_null() {
                syserror(*__errno_location(), a);
                status = exit_failure;
            } else if 0 as libc::c_int > scanfile(fp, a)
                || !(*argv.offset(1 as libc::c_int as isize)).is_null()
                    && putchar('\n' as i32) == -(1 as libc::c_int)
            {
                break;
            }
            argv = argv.offset(1);
            a = *argv;
            if a.is_null() {
                break;
            }
        }
    }
    if ferror(stdout) != 0 || 0 as libc::c_int > fclose(stdout) {
        syserror(
            *__errno_location(),
            b"standard output\0" as *const u8 as *const libc::c_char,
        );
        status = exit_failure;
    }
    gnurcs_goodbye();
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

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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    static mut top: *mut top;
    fn thank_you_and_goodnight(how: libc::c_int);
    fn __errno_location() -> *mut libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
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
pub unsafe extern "C" fn unbuffer_standard_error() {
    (*top)
        .behavior
        .unbuffered = setvbuf(
        stderr,
        0 as *mut libc::c_char,
        2 as libc::c_int,
        0 as libc::c_int as size_t,
    ) == 0;
}
pub unsafe extern "C" fn vcomplain(
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut mstdout: *mut FILE = (*top).manifestation.standard_output;
    if !top.is_null() {
        fflush(if !mstdout.is_null() { mstdout } else { stdout });
    }
    vfprintf(stderr, fmt, args.as_va_list());
    if !top.is_null() {
        if !(*top).behavior.unbuffered {
            fflush(stderr);
        }
    }
}
pub unsafe extern "C" fn complain(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vcomplain(fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn diagnose(mut fmt: *const libc::c_char, mut args: ...) {
    if !(*top).behavior.quiet {
        let mut args_0: ::std::ffi::VaListImpl;
        args_0 = args.clone();
        vcomplain(fmt, args_0.as_va_list());
        complain(b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn whoami(mut who: *const libc::c_char) {
    complain(b"%s: \0" as *const u8 as *const libc::c_char, (*(*top).program).name);
    if !who.is_null() {
        complain(b"%s: \0" as *const u8 as *const libc::c_char, who);
    }
}
pub unsafe extern "C" fn syserror(mut e: libc::c_int, mut who: *const libc::c_char) {
    whoami(0 as *const libc::c_char);
    (*top).flow.erroneous = 1 as libc::c_int != 0;
    *__errno_location() = e;
    perror(who);
}
pub unsafe extern "C" fn generic_warn(
    mut who: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if !(*top).behavior.quiet {
        whoami(who);
        complain(b"warning: \0" as *const u8 as *const libc::c_char);
        let mut args_0: ::std::ffi::VaListImpl;
        args_0 = args.clone();
        vcomplain(fmt, args_0.as_va_list());
        complain(b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn generic_error(
    mut who: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    (*top).flow.erroneous = 1 as libc::c_int != 0;
    whoami(who);
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vcomplain(fmt, args_0.as_va_list());
    complain(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn die() {
    complain(
        b"%s aborted\n\0" as *const u8 as *const libc::c_char,
        (*(*top).program).name,
    );
    thank_you_and_goodnight((*(*top).program).tyag);
}
pub unsafe extern "C" fn generic_fatal(
    mut who: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    (*top).flow.erroneous = 1 as libc::c_int != 0;
    whoami(who);
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vcomplain(fmt, args_0.as_va_list());
    complain(b"\n\0" as *const u8 as *const libc::c_char);
    die();
}
pub unsafe extern "C" fn fatal_syntax(
    mut lno: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    complain(
        b"%s: %s:\0" as *const u8 as *const libc::c_char,
        (*(*top).program).name,
        (*top).repository.filename,
    );
    if lno != 0 {
        complain(b"%ld:\0" as *const u8 as *const libc::c_char, lno);
    }
    complain(b" \0" as *const u8 as *const libc::c_char);
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vcomplain(fmt, args_0.as_va_list());
    complain(b"\n\0" as *const u8 as *const libc::c_char);
    die();
}
pub unsafe extern "C" fn fatal_sys(mut who: *const libc::c_char) {
    syserror(*__errno_location(), who);
    die();
}

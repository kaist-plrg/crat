use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut top: *mut top;
    static ks_revno: [libc::c_char; 0];
    fn ORCSerror();
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn gettime(_: *mut timespec);
    fn __errno_location() -> *mut libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn vfork() -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn unbuffer_standard_error();
    fn vcomplain(fmt: *const libc::c_char, args: ::std::ffi::VaList);
    fn complain(fmt: *const libc::c_char, _: ...);
    fn diagnose(fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    static mut exit_failure: libc::c_int;
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut plexus: *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn make_space(name: *const libc::c_char) -> *mut divvy;
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn forget(divvy: *mut divvy);
    fn intern(
        divvy: *mut divvy,
        s: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    static mut single: *mut divvy;
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn close_space(divvy: *mut divvy);
    fn Oerror();
    fn oflush();
    fn init_ephemstuff();
    fn tempunlink();
    fn dirtempunlink();
    fn maybe_reset_sigchld();
    fn complain_signal(msg: *const libc::c_char, signo: libc::c_int);
    fn isr_init(be_quiet: *mut bool) -> *mut isr_scratch;
    fn display_version(prog: *const program, flags: libc::c_int);
    fn set_program_name(argv0: *const libc::c_char);
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
pub type __pid_t = libc::c_int;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
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
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
pub struct divvy {
    pub name: *const libc::c_char,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
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
pub struct maketimestuff {
    pub tzset_already_called: bool,
    pub TZ: *const libc::c_char,
    pub time2tm_stash: tm,
    pub t_cache: [time_t; 2],
    pub tm_cache: [tm; 2],
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
    pub prev: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: libc::c_long) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
unsafe extern "C" fn exit_diff_trouble(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vcomplain(fmt, args_0.as_va_list());
    complain(b"\n\0" as *const u8 as *const libc::c_char);
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn thank_you_and_goodnight(how: libc::c_int) {
    if how & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        ORCSerror();
    }
    if how & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        dirtempunlink();
    }
    if how & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        tempunlink();
    }
    exit(
        if how & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            exit_failure
        },
    );
}
pub unsafe extern "C" fn gnurcs_init(mut program: *const program) {
    set_program_name((*program).invoke);
    plexus = make_space(b"plexus\0" as *const u8 as *const libc::c_char);
    single = make_space(b"single\0" as *const u8 as *const libc::c_char);
    top = zlloc(
        plexus,
        (::std::mem::size_of::<top>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut top;
    unbuffer_standard_error();
    (*top).program = program;
    (*top)
        .behavior
        .pe = if 0 as libc::c_int != 0 {
        b"\\,v\0" as *const u8 as *const libc::c_char
    } else {
        b",v/\0" as *const u8 as *const libc::c_char
    };
    (*top).behavior.isr = isr_init(&mut (*top).behavior.quiet);
    init_ephemstuff();
    (*top)
        .behavior
        .maketimestuff = zlloc(
        plexus,
        (::std::mem::size_of::<maketimestuff>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut maketimestuff;
    gettime(&mut (*top).behavior.now);
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lim: libc::c_long = 0;
    v = getenv(b"RCS_MEM_LIMIT\0" as *const u8 as *const libc::c_char);
    (*top)
        .behavior
        .mem_limit = if !if !v.is_null()
        && *v.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        v
    } else {
        0 as *mut libc::c_char
    }
        .is_null()
    {
        lim = strtol(v, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
        if 0 as libc::c_int as libc::c_long > lim {
            0 as libc::c_int as libc::c_long
        } else {
            lim
        }
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
pub unsafe extern "C" fn gnurcs_goodbye() {
    top = 0 as *mut top;
    close_space(single);
    single = 0 as *mut divvy;
    close_space(plexus);
    plexus = 0 as *mut divvy;
}
pub unsafe extern "C" fn bad_option(mut option: *const libc::c_char) {
    generic_error(
        0 as *const libc::c_char,
        b"unknown option: %s\0" as *const u8 as *const libc::c_char,
        option,
    );
}
pub unsafe extern "C" fn redefined(mut c: libc::c_int) {
    generic_warn(
        0 as *const libc::c_char,
        b"redefinition of -%c option\0" as *const u8 as *const libc::c_char,
        c,
    );
}
pub unsafe extern "C" fn chk_set_rev(
    mut rev: *mut *const libc::c_char,
    mut arg: *mut libc::c_char,
) {
    if *arg == 0 {
        return;
    }
    if !(*rev).is_null() {
        generic_warn(
            0 as *const libc::c_char,
            b"redefinition of %s\0" as *const u8 as *const libc::c_char,
            ks_revno.as_ptr(),
        );
    }
    *rev = arg;
}
pub unsafe extern "C" fn minus_p(
    mut xrev: *const libc::c_char,
    mut rev: *const libc::c_char,
) -> cbuf {
    let mut rv: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    diagnose(b"retrieving revision %s\0" as *const u8 as *const libc::c_char, xrev);
    accf(single, b"-p%s\0" as *const u8 as *const libc::c_char, rev);
    rv.string = finish_string(single, &mut rv.size);
    return rv;
}
pub unsafe extern "C" fn parse_revpairs(
    mut option: libc::c_char,
    mut arg: *mut libc::c_char,
    mut data: *mut libc::c_void,
    mut put: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            bool,
            *mut libc::c_void,
        ) -> (),
    >,
) {
    let mut c: libc::c_char = 0;
    let mut separator: libc::c_int = if !(strchr(arg, ':' as i32)).is_null() {
        ':' as i32
    } else {
        '-' as i32
    };
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    c = *arg;
    if '-' as i32 == separator && !(strchr(arg, '-' as i32)).is_null()
        && 5 as libc::c_int - 5 as libc::c_int <= (*top).behavior.version
    {
        generic_warn(
            0 as *const libc::c_char,
            b"`-' is obsolete in `-%c%s'; use `:' instead\0" as *const u8
                as *const libc::c_char,
            option as libc::c_int,
            arg,
        );
    }
    loop {
        while c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
        {
            arg = arg.offset(1);
            c = *arg;
        }
        b = arg;
        loop {
            match c as libc::c_int {
                0 | 32 | 9 | 10 | 44 | 59 => {
                    break;
                }
                58 | 45 => {
                    if c as libc::c_int == separator {
                        break;
                    }
                }
                _ => {}
            }
            arg = arg.offset(1);
            c = *arg;
        }
        *arg = '\0' as i32 as libc::c_char;
        while c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
        {
            arg = arg.offset(1);
            c = *arg;
        }
        if c as libc::c_int == separator {
            loop {
                arg = arg.offset(1);
                c = *arg;
                if !(c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
                    || c as libc::c_int == '\n' as i32)
                {
                    break;
                }
            }
            e = arg;
            loop {
                match c as libc::c_int {
                    0 | 32 | 9 | 10 | 44 | 59 => {
                        break;
                    }
                    58 | 45 => {
                        if c as libc::c_int == separator {
                            break;
                        }
                    }
                    _ => {}
                }
                arg = arg.offset(1);
                c = *arg;
            }
            *arg = '\0' as i32 as libc::c_char;
            put.unwrap()(b, e, 1 as libc::c_int != 0, data);
            while c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
                || c as libc::c_int == '\n' as i32
            {
                arg = arg.offset(1);
                c = *arg;
            }
        } else {
            put.unwrap()(b, e, 0 as libc::c_int != 0, data);
        }
        if c == 0 {
            break;
        }
        if c as libc::c_int == ',' as i32 || c as libc::c_int == ';' as i32 {
            arg = arg.offset(1);
            c = *arg;
        } else {
            generic_error(
                0 as *const libc::c_char,
                b"missing `,' near `%c%s'\0" as *const u8 as *const libc::c_char,
                c as libc::c_int,
                arg.offset(1 as libc::c_int as isize),
            );
        }
    };
}
pub unsafe extern "C" fn set_empty_log_message(mut cb: *mut cbuf) {
    (*cb).string = b"*** empty log message ***\0" as *const u8 as *const libc::c_char;
    (*cb)
        .size = (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn ffree() {
    forget(single);
}
pub unsafe extern "C" fn str_save(mut s: *const libc::c_char) -> *mut libc::c_char {
    return intern(plexus, s, strlen(s));
}
pub unsafe extern "C" fn cgetenv(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = getenv(name);
    return if !p.is_null() { str_save(p) } else { p };
}
pub unsafe extern "C" fn awrite(
    mut buf: *const libc::c_char,
    mut chars: size_t,
    mut f: *mut FILE,
) {
    while (9223372036854775807 as libc::c_long as libc::c_ulong) < chars {
        if fwrite(
            buf as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            9223372036854775807 as libc::c_long as libc::c_ulong,
            f,
        ) != 9223372036854775807 as libc::c_long as libc::c_ulong
        {
            Oerror();
        }
        buf = buf.offset(9223372036854775807 as libc::c_long as isize);
        chars = (chars as libc::c_ulong)
            .wrapping_sub(9223372036854775807 as libc::c_long as libc::c_ulong) as size_t
            as size_t;
    }
    if fwrite(
        buf as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        chars,
        f,
    ) != chars
    {
        Oerror();
    }
}
unsafe extern "C" fn movefd(mut old: libc::c_int, mut new: libc::c_int) -> libc::c_int {
    if 0 as libc::c_int > old || old == new {
        return old;
    }
    new = rpl_fcntl(old, 0 as libc::c_int, new);
    return if !(0 as libc::c_int > close(old)) { new } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn fdreopen(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut newfd: libc::c_int = 0;
    close(fd);
    newfd = open(file, flags, 0o400 as libc::c_int | 0o200 as libc::c_int);
    return movefd(newfd, fd);
}
pub unsafe extern "C" fn runv(
    mut infd: libc::c_int,
    mut outname: *const libc::c_char,
    mut args: *mut *const libc::c_char,
) -> libc::c_int {
    let mut wstatus: libc::c_int = 0;
    if !(*top).behavior.fixed_SIGCHLD {
        (*top).behavior.fixed_SIGCHLD = 1 as libc::c_int != 0;
        maybe_reset_sigchld();
    }
    oflush();
    let mut pid: pid_t = 0;
    pid = vfork();
    if pid == 0 {
        let mut notfound: *const libc::c_char = 0 as *const libc::c_char;
        if infd != -(1 as libc::c_int) && 0 as libc::c_int != infd
            && {
                close(0 as libc::c_int);
                0 as libc::c_int != rpl_fcntl(infd, 0 as libc::c_int, 0 as libc::c_int)
            }
        {
            exit_diff_trouble(
                b"%s: I/O redirection failed\0" as *const u8 as *const libc::c_char,
                *args.offset(1 as libc::c_int as isize),
            );
        }
        if !outname.is_null() {
            if 0 as libc::c_int
                > fdreopen(
                    1 as libc::c_int,
                    outname,
                    0o100 as libc::c_int | 0o1000 as libc::c_int | 0o1 as libc::c_int,
                )
            {
                exit_diff_trouble(
                    b"%s: %s: cannot create\0" as *const u8 as *const libc::c_char,
                    *args.offset(1 as libc::c_int as isize),
                    outname,
                );
            }
        }
        execv(
            *args.offset(1 as libc::c_int as isize),
            args.offset(1 as libc::c_int as isize) as *mut *mut libc::c_char
                as *const *mut libc::c_char,
        );
        notfound = *args.offset(1 as libc::c_int as isize);
        if *__errno_location() == 8 as libc::c_int {
            notfound = b"/bin/sh\0" as *const u8 as *const libc::c_char;
            let ref mut fresh0 = *args.offset(0 as libc::c_int as isize);
            *fresh0 = notfound;
            execv(
                *args.offset(0 as libc::c_int as isize),
                args as *mut *mut libc::c_char as *const *mut libc::c_char,
            );
        }
        exit_diff_trouble(
            b"%s: not found\0" as *const u8 as *const libc::c_char,
            notfound,
        );
    }
    if 0 as libc::c_int > pid {
        fatal_sys(b"fork\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int > waitpid(pid, &mut wstatus, 0 as libc::c_int) {
        fatal_sys(b"waitpid\0" as *const u8 as *const libc::c_char);
    }
    if !(wstatus & 0x7f as libc::c_int == 0 as libc::c_int) {
        if ((wstatus & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            complain_signal(
                *args.offset(1 as libc::c_int as isize),
                wstatus & 0x7f as libc::c_int,
            );
            generic_fatal(
                0 as *const libc::c_char,
                b"%s got a fatal signal\0" as *const u8 as *const libc::c_char,
                *args.offset(1 as libc::c_int as isize),
            );
        }
        generic_fatal(
            0 as *const libc::c_char,
            b"%s failed for unknown reason\0" as *const u8 as *const libc::c_char,
            *args.offset(1 as libc::c_int as isize),
        );
    }
    return (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int;
}
pub unsafe extern "C" fn run(
    mut infd: libc::c_int,
    mut outname: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut rgargs: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
    let mut i: libc::c_int = 0;
    ap = args.clone();
    i = 1 as libc::c_int;
    loop {
        let fresh1 = i;
        i = i + 1;
        rgargs[fresh1 as usize] = ap.arg::<*const libc::c_char>();
        if (rgargs[fresh1 as usize]).is_null() {
            break;
        }
        if 20 as libc::c_int <= i {
            generic_fatal(
                0 as *const libc::c_char,
                b"too many command arguments\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return runv(infd, outname, rgargs.as_mut_ptr());
}
pub unsafe extern "C" fn setRCSversion(mut str: *const libc::c_char) {
    let mut s: *const libc::c_char = str.offset(2 as libc::c_int as isize);
    if *s != 0 {
        let mut v: libc::c_int = 5 as libc::c_int;
        if (*top).behavior.version_set {
            redefined('V' as i32);
        }
        (*top).behavior.version_set = 1 as libc::c_int != 0;
        v = 0 as libc::c_int;
        while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let fresh2 = s;
            s = s.offset(1);
            v = 10 as libc::c_int * v + *fresh2 as libc::c_int - '0' as i32;
        }
        if *s != 0 {
            generic_error(
                0 as *const libc::c_char,
                b"%s isn't a number\0" as *const u8 as *const libc::c_char,
                str,
            );
        } else if v < 3 as libc::c_int || (5 as libc::c_int) < v {
            generic_error(
                0 as *const libc::c_char,
                b"%s out of range %d..%d\0" as *const u8 as *const libc::c_char,
                str,
                3 as libc::c_int,
                5 as libc::c_int,
            );
        }
        (*top).behavior.version = v - 5 as libc::c_int;
    } else {
        display_version((*top).program, 1 as libc::c_int | 2 as libc::c_int);
    };
}
pub unsafe extern "C" fn getRCSINIT(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut newargv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: size_t = 0;
    q = cgetenv(b"RCSINIT\0" as *const u8 as *const libc::c_char);
    if q.is_null() {
        *newargv = argv;
    } else {
        n = (argc + 2 as libc::c_int) as size_t;
        p = q;
        loop {
            let fresh3 = p;
            p = p.offset(1);
            match *fresh3 as libc::c_int {
                32 | 8 | 12 | 10 | 13 | 9 | 11 => {}
                0 => {
                    break;
                }
                _ => {
                    continue;
                }
            }
            n = n.wrapping_add(1);
            n;
        }
        pp = pointer_array(plexus, n) as *mut *mut libc::c_char;
        *newargv = pp;
        let fresh4 = argv;
        argv = argv.offset(1);
        let fresh5 = pp;
        pp = pp.offset(1);
        *fresh5 = *fresh4;
        p = q;
        's_79: loop {
            match *q as libc::c_int {
                0 => {
                    break;
                }
                32 | 8 | 12 | 10 | 13 | 9 | 11 => {
                    q = q.offset(1);
                    q;
                }
                _ => {
                    let fresh6 = pp;
                    pp = pp.offset(1);
                    *fresh6 = p;
                    argc += 1;
                    argc;
                    loop {
                        let fresh7 = q;
                        q = q.offset(1);
                        let fresh8 = p;
                        p = p.offset(1);
                        *fresh8 = *fresh7;
                        match *fresh8 as libc::c_int {
                            0 => {
                                break 's_79;
                            }
                            92 => {}
                            32 | 8 | 12 | 10 | 13 | 9 | 11 => {
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                        if *q == 0 {
                            break 's_79;
                        }
                        let fresh9 = q;
                        q = q.offset(1);
                        *p.offset(-(1 as libc::c_int) as isize) = *fresh9;
                    }
                    *p
                        .offset(
                            -(1 as libc::c_int) as isize,
                        ) = '\0' as i32 as libc::c_char;
                }
            }
        }
        loop {
            let fresh10 = argv;
            argv = argv.offset(1);
            let fresh11 = pp;
            pp = pp.offset(1);
            *fresh11 = *fresh10;
            if (*fresh11).is_null() {
                break;
            }
        }
    }
    return argc;
}
pub unsafe extern "C" fn unspecified_timespec() -> timespec {
    return make_timespec(
        -(1 as libc::c_int) as time_t,
        0 as libc::c_int as libc::c_long,
    );
}
pub unsafe extern "C" fn file_mtime(mut enable: bool, mut st: *const stat) -> timespec {
    return if enable as libc::c_int != 0 {
        get_stat_mtime(st)
    } else {
        unspecified_timespec()
    };
}

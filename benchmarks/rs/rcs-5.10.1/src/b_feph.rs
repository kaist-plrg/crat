use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut top: *mut top;
    fn un_link(s: *const libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    static mut plexus: *mut divvy;
    fn seteid();
    fn setrid();
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
pub struct ephemstuff {
    pub standard: *const libc::c_char,
    pub tpnames: *mut sff,
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
pub unsafe extern "C" fn init_ephemstuff() {
    (*top)
        .behavior
        .sff = zlloc(
        plexus,
        (::std::mem::size_of::<sff>() as libc::c_ulong)
            .wrapping_mul((0 as libc::c_int + 2 as libc::c_int) as libc::c_ulong),
    ) as *mut sff;
    (*top)
        .behavior
        .ephemstuff = zlloc(
        plexus,
        (::std::mem::size_of::<ephemstuff>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut ephemstuff;
    (*(*top).behavior.ephemstuff)
        .tpnames = zlloc(
        plexus,
        (::std::mem::size_of::<sff>() as libc::c_ulong)
            .wrapping_mul(5 as libc::c_int as libc::c_ulong),
    ) as *mut sff;
}
unsafe extern "C" fn jam_sff(mut sff: *mut sff, mut prefix: *const libc::c_char) {
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut fd: libc::c_int = 0;
    if prefix.is_null() {
        if ((*(*top).behavior.ephemstuff).standard).is_null() {
            let mut dir: *const libc::c_char = 0 as *const libc::c_char;
            let mut slash: [libc::c_char; 2] = [
                '/' as i32 as libc::c_char,
                '\0' as i32 as libc::c_char,
            ];
            if dir.is_null() {
                dir = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
            }
            if dir.is_null() {
                dir = getenv(b"TMP\0" as *const u8 as *const libc::c_char);
            }
            if dir.is_null() {
                dir = getenv(b"TEMP\0" as *const u8 as *const libc::c_char);
            }
            if dir.is_null() {
                dir = b"/tmp\0" as *const u8 as *const libc::c_char;
            }
            accf(
                plexus,
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                dir,
                if '/' as i32
                    != *dir
                        .offset(
                            (strlen(dir)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int
                {
                    slash.as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*(*top).program).name,
            );
            (*(*top).behavior.ephemstuff).standard = finish_string(plexus, &mut len);
        }
        prefix = (*(*top).behavior.ephemstuff).standard;
    }
    accf(plexus, b"%sXXXXXX\0" as *const u8 as *const libc::c_char, prefix);
    fn_0 = finish_string(plexus, &mut len);
    if '/' as i32 != '/' as i32 {
        let mut end: *mut libc::c_char = fn_0
            .offset(len as isize)
            .offset(-(6 as libc::c_int as isize));
        let mut lastsep: *mut libc::c_char = strrchr(fn_0, '/' as i32);
        let mut ndfc: *mut libc::c_char = if !lastsep.is_null() {
            lastsep.offset(1 as libc::c_int as isize)
        } else {
            fn_0
        };
        let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
        if ndfc.offset(2 as libc::c_int as isize) < end {
            memset(
                ndfc.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                'X' as i32,
                6 as libc::c_int as libc::c_ulong,
            );
            *dot = '\0' as i32 as libc::c_char;
        }
        dot = strchr(ndfc, '.' as i32);
        if !dot.is_null() {
            *dot = ('0' as i32 + getpid() % 10 as libc::c_int) as libc::c_char;
        }
    }
    fd = fd_safer(mkstemp(fn_0));
    if 0 as libc::c_int > fd {
        generic_fatal(
            0 as *const libc::c_char,
            b"could not make temporary file name (template \"%s\")\0" as *const u8
                as *const libc::c_char,
            fn_0,
        );
    }
    close(fd);
    (*sff).filename = fn_0;
    (*sff).disposition = real;
}
pub unsafe extern "C" fn maketemp(mut n: libc::c_int) -> *const libc::c_char {
    if ((*((*(*top).behavior.ephemstuff).tpnames).offset(n as isize)).filename).is_null()
    {
        jam_sff(
            &mut *((*(*top).behavior.ephemstuff).tpnames).offset(n as isize),
            0 as *const libc::c_char,
        );
    }
    return (*((*(*top).behavior.ephemstuff).tpnames).offset(n as isize)).filename;
}
pub unsafe extern "C" fn makedirtemp(mut isworkfile: bool) -> *const libc::c_char {
    let mut sff: *mut sff = (*top).behavior.sff;
    let mut slot: libc::c_int = 0 as libc::c_int + isworkfile as libc::c_int;
    jam_sff(
        &mut *sff.offset(slot as isize),
        if isworkfile as libc::c_int != 0 {
            (*top).manifestation.filename as *const libc::c_char
        } else {
            (*top).repository.filename
        },
    );
    return (*sff.offset(slot as isize)).filename;
}
pub unsafe extern "C" fn keepdirtemp(mut name: *const libc::c_char) {
    let mut sff: *mut sff = (*top).behavior.sff;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 0 as libc::c_int + 2 as libc::c_int {
        if name == (*sff.offset(i as isize)).filename {
            (*sff.offset(i as isize)).disposition = notmade;
            return;
        }
        i += 1;
        i;
    }
    generic_fatal(
        0 as *const libc::c_char,
        b"keepdirtemp\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn reap(
    mut count: size_t,
    mut all: *mut sff,
    mut cut: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
) {
    let mut m: maker = notmade;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        m = (*all.offset(i as isize)).disposition;
        if notmade as libc::c_int as libc::c_uint != m as libc::c_uint {
            if effective as libc::c_int as libc::c_uint == m as libc::c_uint {
                seteid();
            }
            cut.unwrap()((*all.offset(i as isize)).filename);
            let ref mut fresh0 = (*all.offset(i as isize)).filename;
            *fresh0 = 0 as *const libc::c_char;
            if effective as libc::c_int as libc::c_uint == m as libc::c_uint {
                setrid();
            }
            (*all.offset(i as isize)).disposition = notmade;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn tempunlink() {
    reap(
        5 as libc::c_int as size_t,
        (*(*top).behavior.ephemstuff).tpnames,
        Some(unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int),
    );
}
pub unsafe extern "C" fn dirtempunlink() {
    reap(
        (0 as libc::c_int + 2 as libc::c_int) as size_t,
        (*top).behavior.sff,
        Some(un_link as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int),
    );
}

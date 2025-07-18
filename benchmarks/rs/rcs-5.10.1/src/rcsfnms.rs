use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wlink;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut top: *mut top;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn cgetenv(name: *const libc::c_char) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn make_space(name: *const libc::c_char) -> *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn intern(
        divvy: *mut divvy,
        s: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn accumulate_nbytes(divvy: *mut divvy, start: *const libc::c_char, count: size_t);
    fn accumulate_byte(divvy: *mut divvy, c: libc::c_int);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn close_space(divvy: *mut divvy);
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn empty_repo(to: *mut divvy) -> *mut repo;
    fn grok_all(to: *mut divvy, f: *mut fro) -> *mut repo;
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
pub type kwsub = libc::c_uint;
pub const kwsub_b: kwsub = 5;
pub const kwsub_o: kwsub = 4;
pub const kwsub_v: kwsub = 3;
pub const kwsub_k: kwsub = 2;
pub const kwsub_kvl: kwsub = 1;
pub const kwsub_kv: kwsub = 0;
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
pub struct atat {
    pub count: size_t,
    pub lno: size_t,
    pub line_count: size_t,
    pub from: *mut fro,
    pub beg: off_t,
    pub holes: [off_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fro {
    pub fd: libc::c_int,
    pub end: off_t,
    pub rm: readmethod,
    pub ptr: *mut libc::c_char,
    pub lim: *mut libc::c_char,
    pub base: *mut libc::c_char,
    pub deallocate: Option::<unsafe extern "C" fn(*mut fro) -> ()>,
    pub stream: *mut FILE,
    pub verbatim: off_t,
}
pub type readmethod = libc::c_uint;
pub const RM_STDIO: readmethod = 2;
pub const RM_MEM: readmethod = 1;
pub const RM_MMAP: readmethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maybe {
    pub open: Option::<open_rcsfile_fn>,
    pub mustread: bool,
    pub tentative: cbuf,
    pub space: *mut divvy,
    pub bestfit: cbuf,
    pub status: *mut stat,
    pub eno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const libc::c_char,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
pub type open_rcsfile_fn = unsafe extern "C" fn(*mut maybe) -> *mut fro;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compair {
    pub suffix: *const libc::c_char,
    pub comlead: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut comtable: [compair; 39] = [
    {
        let mut init = compair {
            suffix: b"a\0" as *const u8 as *const libc::c_char,
            comlead: b"-- \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ada\0" as *const u8 as *const libc::c_char,
            comlead: b"-- \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"adb\0" as *const u8 as *const libc::c_char,
            comlead: b"-- \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ads\0" as *const u8 as *const libc::c_char,
            comlead: b"-- \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"asm\0" as *const u8 as *const libc::c_char,
            comlead: b";; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"bat\0" as *const u8 as *const libc::c_char,
            comlead: b":: \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"body\0" as *const u8 as *const libc::c_char,
            comlead: b"-- \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"c\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"c++\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cc\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cpp\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cxx\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cl\0" as *const u8 as *const libc::c_char,
            comlead: b";;; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cmd\0" as *const u8 as *const libc::c_char,
            comlead: b":: \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cmf\0" as *const u8 as *const libc::c_char,
            comlead: b"c \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cs\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"el\0" as *const u8 as *const libc::c_char,
            comlead: b"; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"f\0" as *const u8 as *const libc::c_char,
            comlead: b"c \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"for\0" as *const u8 as *const libc::c_char,
            comlead: b"c \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"h\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"hpp\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"hxx\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"l\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"lisp\0" as *const u8 as *const libc::c_char,
            comlead: b";;; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"lsp\0" as *const u8 as *const libc::c_char,
            comlead: b";; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"m\0" as *const u8 as *const libc::c_char,
            comlead: b"// \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"mac\0" as *const u8 as *const libc::c_char,
            comlead: b";; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"me\0" as *const u8 as *const libc::c_char,
            comlead: b".\\\" \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ml\0" as *const u8 as *const libc::c_char,
            comlead: b"; \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"mm\0" as *const u8 as *const libc::c_char,
            comlead: b".\\\" \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ms\0" as *const u8 as *const libc::c_char,
            comlead: b".\\\" \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"p\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"pas\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ps\0" as *const u8 as *const libc::c_char,
            comlead: b"% \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"spec\0" as *const u8 as *const libc::c_char,
            comlead: b"-- \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"sty\0" as *const u8 as *const libc::c_char,
            comlead: b"% \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"tex\0" as *const u8 as *const libc::c_char,
            comlead: b"% \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"y\0" as *const u8 as *const libc::c_char,
            comlead: b" * \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = compair {
            suffix: 0 as *const libc::c_char,
            comlead: b"# \0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn InitAdmin() {
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    (*top).repository.tip = 0 as *mut delta;
    (*top).behavior.strictly_locking = 1 as libc::c_int != 0;
    (*top).repository.r = empty_repo(single);
    ext = strrchr((*top).manifestation.filename, '.' as i32);
    ext = if !ext.is_null() {
        ext.offset(1 as libc::c_int as isize)
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    let mut ent: *const compair = comtable.as_ptr();
    loop {
        if ((*ent).suffix).is_null() || strcasecmp((*ent).suffix, ext) == 0 {
            (*top).repository.log_lead.string = (*ent).comlead;
            (*top).repository.log_lead.size = strlen((*ent).comlead);
            break;
        } else {
            ent = ent.offset(1);
            ent;
        }
    }
    (*top).behavior.kws = kwsub_kv as libc::c_int;
}
pub unsafe extern "C" fn basefilename(
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut b: *const libc::c_char = p;
    let mut q: *const libc::c_char = p;
    loop {
        let fresh0 = q;
        q = q.offset(1);
        match *fresh0 as libc::c_int {
            47 => {
                b = q;
            }
            0 => return b,
            _ => {}
        }
    };
}
unsafe extern "C" fn suffixlen(mut x: *const libc::c_char) -> size_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = x;
    loop {
        match *p as libc::c_int {
            0 | 47 => return p.offset_from(x) as libc::c_long as size_t,
            _ => {
                p = p.offset(1);
                p;
            }
        }
    };
}
pub unsafe extern "C" fn rcssuffix(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut nz: *const libc::c_char = 0 as *const libc::c_char;
    let mut nl: size_t = 0;
    let mut xl: size_t = 0;
    nl = strlen(name);
    nz = name.offset(nl as isize);
    x = (*top).behavior.pe;
    loop {
        xl = suffixlen(x);
        if xl != 0 {
            if xl <= nl
                && {
                    p = nz.offset(-(xl as isize));
                    0 as libc::c_int
                        == memcmp(p as *const libc::c_void, x as *const libc::c_void, xl)
                }
            {
                return p;
            }
        } else {
            p = name;
            while p
                < nz
                    .offset(
                        -((::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                    )
            {
                if isSLASH(
                    *p
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int,
                ) as libc::c_int != 0
                    && (p == name
                        || isSLASH(
                            *p.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                        ) as libc::c_int != 0)
                    && 0 as libc::c_int
                        == memcmp(
                            p as *const libc::c_void,
                            b"RCS\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                {
                    return nz;
                }
                p = p.offset(1);
                p;
            }
        }
        x = x.offset(xl as isize);
        let fresh1 = x;
        x = x.offset(1);
        if !(*fresh1 != 0) {
            break;
        }
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn rcsreadopen(mut m: *mut maybe) -> *mut fro {
    return fro_open(
        (*m).tentative.string,
        b"r\0" as *const u8 as *const libc::c_char,
        (*m).status,
    );
}
unsafe extern "C" fn finopen(mut m: *mut maybe) -> bool {
    let mut interesting: bool = false;
    let mut preferold: bool = false;
    preferold = *((*m).bestfit.string).offset(0 as libc::c_int as isize) as libc::c_int
        != 0
        && ((*m).mustread as libc::c_int != 0
            || 0 as libc::c_int <= (*top).repository.fd_lock);
    (*top).flow.from = ((*m).open).unwrap()(m);
    interesting = !((*top).flow.from).is_null()
        || *__errno_location() != 2 as libc::c_int;
    if interesting as libc::c_int != 0 || !preferold {
        (*m).eno = *__errno_location();
        (*m).bestfit = (*m).tentative;
    }
    return interesting;
}
unsafe extern "C" fn fin2open(
    mut d: *const libc::c_char,
    mut dlen: size_t,
    mut base: *const libc::c_char,
    mut baselen: size_t,
    mut x: *const libc::c_char,
    mut xlen: size_t,
    mut m: *mut maybe,
) -> bool {
    accumulate_nbytes((*m).space, d, dlen);
    accumulate_nbytes(
        (*m).space,
        b"RCS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    accumulate_byte((*m).space, '/' as i32);
    accumulate_nbytes((*m).space, base, baselen);
    accumulate_nbytes((*m).space, x, xlen);
    (*m).tentative.string = finish_string((*m).space, &mut (*m).tentative.size);
    if xlen != 0 {
        if finopen(m) {
            return 1 as libc::c_int != 0;
        }
        accumulate_nbytes((*m).space, d, dlen);
        accumulate_nbytes((*m).space, base, baselen);
        accumulate_nbytes((*m).space, x, xlen);
        (*m).tentative.string = finish_string((*m).space, &mut (*m).tentative.size);
    }
    return finopen(m);
}
pub unsafe extern "C" fn pairnames(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut rcsopen: Option::<open_rcsfile_fn>,
    mut mustread: bool,
    mut quiet: bool,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut RCS1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    let mut RCSbase: *const libc::c_char = 0 as *const libc::c_char;
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    let mut mani_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut paired: bool = false;
    let mut arglen: size_t = 0;
    let mut dlen: size_t = 0;
    let mut baselen: size_t = 0;
    let mut xlen: size_t = 0;
    let mut from: *mut fro = 0 as *mut fro;
    let mut maybe: maybe = {
        let mut init = maybe {
            open: rcsopen,
            mustread: mustread,
            tentative: cbuf {
                string: 0 as *const libc::c_char,
                size: 0,
            },
            space: 0 as *mut divvy,
            bestfit: cbuf {
                string: 0 as *const libc::c_char,
                size: 0,
            },
            status: &mut (*top).repository.stat,
            eno: 0,
        };
        init
    };
    (*top).repository.fd_lock = -(1 as libc::c_int);
    arg = *argv;
    if arg.is_null() {
        return 0 as libc::c_int;
    }
    if *arg as libc::c_int == '-' as i32 {
        generic_error(
            0 as *const libc::c_char,
            b"%s option is ignored after filenames\0" as *const u8
                as *const libc::c_char,
            arg,
        );
        return 0 as libc::c_int;
    }
    base = basefilename(arg);
    paired = 0 as libc::c_int != 0;
    x = rcssuffix(arg);
    if !x.is_null() {
        RCS1 = arg;
        RCSbase = base;
        baselen = x.offset_from(base) as libc::c_long as size_t;
        if (1 as libc::c_int) < argc
            && {
                p = *argv.offset(1 as libc::c_int as isize);
                mani_filename = p;
                (rcssuffix(mani_filename)).is_null()
            }
            && {
                arglen = strlen(p);
                baselen <= arglen
            }
            && {
                p = p.offset(arglen.wrapping_sub(baselen) as isize);
                p == mani_filename
                    || isSLASH(*p.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as libc::c_int != 0
            }
            && 0 as libc::c_int
                == memcmp(base as *const libc::c_void, p as *const libc::c_void, baselen)
        {
            let ref mut fresh2 = *argv.offset(1 as libc::c_int as isize);
            *fresh2 = 0 as *mut libc::c_char;
            paired = 1 as libc::c_int != 0;
        } else {
            mani_filename = intern(
                single,
                base,
                baselen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            *mani_filename.offset(baselen as isize) = '\0' as i32 as libc::c_char;
        }
    } else {
        mani_filename = arg;
        baselen = strlen(base);
        if (1 as libc::c_int) < argc
            && {
                RCS1 = *argv.offset(1 as libc::c_int as isize);
                x = rcssuffix(RCS1);
                !x.is_null()
            } && RCS1.offset(baselen as isize) <= x as *mut libc::c_char
            && {
                RCSbase = x.offset(-(baselen as isize));
                RCSbase == RCS1 as *const libc::c_char
                    || isSLASH(
                        *RCSbase.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                    ) as libc::c_int != 0
            }
            && 0 as libc::c_int
                == memcmp(
                    base as *const libc::c_void,
                    RCSbase as *const libc::c_void,
                    baselen,
                )
        {
            let ref mut fresh3 = *argv.offset(1 as libc::c_int as isize);
            *fresh3 = 0 as *mut libc::c_char;
            paired = 1 as libc::c_int != 0;
        } else {
            RCS1 = 0 as *mut libc::c_char;
            RCSbase = RCS1;
        }
    }
    (*top).manifestation.filename = mani_filename;
    maybe
        .space = make_space(
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pairnames\0"))
            .as_ptr(),
    );
    if RCSbase != RCS1 as *const libc::c_char {
        maybe.bestfit.string = RCS1;
        maybe.bestfit.size = strlen(RCS1);
        maybe.tentative = maybe.bestfit;
        (*top).flow.from = (Some(rcsopen.unwrap())).unwrap()(&mut maybe);
        maybe.eno = *__errno_location();
    } else {
        maybe.bestfit.string = b"\0" as *const u8 as *const libc::c_char;
        maybe.bestfit.size = 0 as libc::c_int as size_t;
        if !RCS1.is_null() {
            fin2open(
                arg,
                0 as libc::c_int as size_t,
                RCSbase,
                baselen,
                x,
                strlen(x),
                &mut maybe,
            );
        } else {
            dlen = base.offset_from(arg) as libc::c_long as size_t;
            x = (*top).behavior.pe;
            loop {
                xlen = suffixlen(x);
                if fin2open(arg, dlen, base, baselen, x, xlen, &mut maybe) {
                    break;
                }
                x = x.offset(xlen as isize);
                let fresh4 = x;
                x = x.offset(1);
                if *fresh4 == 0 {
                    break;
                }
            }
        }
    }
    p = intern(single, maybe.bestfit.string, maybe.bestfit.size);
    (*top).repository.filename = p;
    (*top).flow.erroneous = 0 as libc::c_int != 0;
    (*top).behavior.Oerrloop = 0 as libc::c_int != 0;
    from = (*top).flow.from;
    if !from.is_null() {
        if !((*maybe.status).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        {
            generic_error(
                0 as *const libc::c_char,
                b"%s isn't a regular file -- ignored\0" as *const u8
                    as *const libc::c_char,
                p,
            );
            return 0 as libc::c_int;
        }
        (*top).repository.r = grok_all(single, from);
        (*top).flow.to = 0 as *mut FILE;
    } else {
        if maybe.eno != 2 as libc::c_int || mustread as libc::c_int != 0
            || 0 as libc::c_int > (*top).repository.fd_lock
        {
            if maybe.eno == 17 as libc::c_int {
                generic_error(
                    0 as *const libc::c_char,
                    b"RCS file %s is in use\0" as *const u8 as *const libc::c_char,
                    p,
                );
            } else if !quiet || maybe.eno != 2 as libc::c_int {
                syserror(maybe.eno, p);
            }
            return 0 as libc::c_int;
        }
        InitAdmin();
    }
    if paired as libc::c_int != 0 && !((*top).manifestation.standard_output).is_null() {
        generic_warn(
            (*top).manifestation.filename,
            b"Working file ignored due to -p option\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*top).manifestation.prev.valid = 0 as libc::c_int != 0;
    close_space(maybe.space);
    return if !from.is_null() { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn dir_useful_len(mut d: *const libc::c_char) -> size_t {
    let mut dlen: size_t = strlen(d);
    if 0 as libc::c_int != 0 && dlen == 2 as libc::c_int as libc::c_ulong
        && isSLASH(*d.offset(0 as libc::c_int as isize) as libc::c_int) as libc::c_int
            != 0
        && isSLASH(*d.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_int
            != 0
    {
        dlen = dlen.wrapping_sub(1);
        dlen;
    } else {
        while dlen != 0
            && isSLASH(
                *d.offset(dlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int,
            ) as libc::c_int != 0
        {
            dlen = dlen.wrapping_sub(1);
            dlen;
        }
    }
    return dlen;
}
pub unsafe extern "C" fn getfullRCSname() -> *const libc::c_char {
    let mut r: *const libc::c_char = (*top).repository.filename;
    if isSLASH(*r.offset(0 as libc::c_int as isize) as libc::c_int) {
        return r
    } else {
        let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        cwd = (*top).behavior.cwd;
        if cwd.is_null() {
            let mut PWD: *mut libc::c_char = cgetenv(
                b"PWD\0" as *const u8 as *const libc::c_char,
            );
            let mut PWDstat: stat = stat {
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
            let mut dotstat: stat = stat {
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
            cwd = PWD;
            if !(!cwd.is_null()
                && isSLASH(*PWD.offset(0 as libc::c_int as isize) as libc::c_int)
                    as libc::c_int != 0 && !(0 as libc::c_int > stat(PWD, &mut PWDstat))
                && !(0 as libc::c_int
                    > stat(b".\0" as *const u8 as *const libc::c_char, &mut dotstat))
                && (PWDstat.st_ino == dotstat.st_ino
                    && PWDstat.st_dev == dotstat.st_dev))
            {
                let mut sz: size_t = 64 as libc::c_int as size_t;
                loop {
                    cwd = alloc(plexus, sz) as *mut libc::c_char;
                    if !(getcwd(cwd, sz)).is_null() {
                        break;
                    }
                    brush_off(plexus, cwd as *mut libc::c_void);
                    if *__errno_location() == 34 as libc::c_int {
                        sz <<= 1 as libc::c_int;
                    } else {
                        cwd = PWD;
                        if !cwd.is_null() {
                            break;
                        }
                        fatal_sys(b"getcwd\0" as *const u8 as *const libc::c_char);
                    }
                }
            }
            *cwd.offset(dir_useful_len(cwd) as isize) = '\0' as i32 as libc::c_char;
            (*top).behavior.cwd = cwd;
        }
        while *r.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && isSLASH(*r.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            while isSLASH(*r.offset(2 as libc::c_int as isize) as libc::c_int) {
                r = r.offset(1);
                r;
            }
            r = r.offset(2 as libc::c_int as isize);
        }
        accf(
            single,
            b"%s%c%s\0" as *const u8 as *const libc::c_char,
            cwd,
            '/' as i32,
            r,
        );
        rv = finish_string(single, &mut len);
        return rv;
    };
}
pub unsafe extern "C" fn isSLASH(mut c: libc::c_int) -> bool {
    if 0 as libc::c_int == 0 {
        return '/' as i32 == c;
    }
    match c {
        47 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}

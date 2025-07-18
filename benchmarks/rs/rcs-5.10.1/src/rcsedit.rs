use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn rewind(__stream: *mut FILE);
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    static mut top: *mut top;
    static tiny_desc: tinysym;
    fn putadmin();
    fn puttree(root: *const delta, fout: *mut FILE);
    fn isSLASH(c: libc::c_int) -> bool;
    fn basefilename(p: *const libc::c_char) -> *const libc::c_char;
    fn rcssuffix(name: *const libc::c_char) -> *const libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn utimens(_: *const libc::c_char, _: *const timespec) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_syntax(lno: size_t, fmt: *const libc::c_char, _: ...);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn intern(
        divvy: *mut divvy,
        s: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn accumulate_range(
        divvy: *mut divvy,
        beg: *const libc::c_char,
        end: *const libc::c_char,
    );
    fn accs(divvy: *mut divvy, string: *const libc::c_char);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn close_space(divvy: *mut divvy);
    fn prepend(x: *const libc::c_void, ls: *mut link, to: *mut divvy) -> *mut link;
    fn stat_mine_p(st: *mut stat) -> bool;
    fn seteid();
    fn setrid();
    fn getcaller() -> *const libc::c_char;
    fn caller_login_p(login: *const libc::c_char) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn change_mode(fd: libc::c_int, mode: mode_t) -> libc::c_int;
    fn testOerror(o: *mut FILE);
    fn fopen_safer(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
    ) -> *mut FILE;
    fn Ozclose(p: *mut *mut FILE);
    fn aflush(f: *mut FILE);
    fn afputc(c: libc::c_int, f: *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const libc::c_char, _: ...);
    fn maketemp(n: libc::c_int) -> *const libc::c_char;
    fn keepdirtemp(name: *const libc::c_char);
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    fn fro_close(f: *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut libc::c_int, f: *mut fro) -> bool;
    fn fro_must_getbyte(c: *mut libc::c_int, f: *mut fro);
    fn fro_trundling(sequential: bool, f: *mut fro);
    fn fro_spew(f: *mut fro, to: *mut FILE);
    fn atat_put(to: *mut FILE, atat: *const atat);
    fn atat_display(to: *mut FILE, atat: *const atat, ensure_end_nl: bool);
    fn isr_do(scratch: *mut isr_scratch, action: isr_actions);
    fn expandline(ctx: *mut expctx) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
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
pub type ssize_t = __ssize_t;
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
pub type mode_t = __mode_t;
pub type time_t = __time_t;
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
pub struct diffcmd {
    pub line1: libc::c_long,
    pub nlines: libc::c_long,
    pub adprev: libc::c_long,
    pub dafter: libc::c_long,
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
pub struct wlink {
    pub entry: *mut libc::c_void,
    pub next: *mut wlink,
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
pub struct rcslock {
    pub login: *const libc::c_char,
    pub delta: *mut delta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdef {
    pub meaningful: *const libc::c_char,
    pub underlying: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 32],
}
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
pub struct link {
    pub entry: *const libc::c_void,
    pub next: *mut link,
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
pub struct editstuff {
    pub fedit: *mut fro,
    pub filename: *const libc::c_char,
    pub script_lno: size_t,
    pub lcount: libc::c_long,
    pub corr: libc::c_long,
    pub line: *mut *mut libc::c_char,
    pub gap: size_t,
    pub gapsize: size_t,
    pub lim: size_t,
    pub sff: *mut sff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expctx {
    pub to: *mut FILE,
    pub rewr: *mut FILE,
    pub from: *mut fro,
    pub delta: *const delta,
    pub delimstuffed: bool,
    pub dolog: bool,
    pub lparts: *mut divvy,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct finctx {
    pub ctx: expctx,
    pub script_lno: size_t,
}
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub type isr_actions = libc::c_uint;
pub const ISR_CATCHMMAPINTS: isr_actions = 3;
pub const ISR_RESTOREINTS: isr_actions = 2;
pub const ISR_IGNOREINTS: isr_actions = 1;
pub const ISR_CATCHINTS: isr_actions = 0;
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
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub unsafe extern "C" fn make_editstuff() -> *mut editstuff {
    return zlloc(
        plexus,
        (::std::mem::size_of::<editstuff>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut editstuff;
}
pub unsafe extern "C" fn unmake_editstuff(mut es: *mut editstuff) {
    rpl_free((*es).line as *mut libc::c_void);
    memset(
        es as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<editstuff>() as libc::c_ulong,
    );
}
unsafe extern "C" fn nfs_NOENT_p() -> bool {
    return 1 as libc::c_int != 0 && 2 as libc::c_int == *__errno_location();
}
pub unsafe extern "C" fn un_link(mut s: *const libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = unlink(s);
    if 0 as libc::c_int > rv {
        if nfs_NOENT_p() {
            rv = 0 as libc::c_int;
        }
    }
    return rv;
}
unsafe extern "C" fn insertline(
    mut es: *mut editstuff,
    mut n: libc::c_ulong,
    mut l: *mut libc::c_char,
) {
    if ((*es).lim).wrapping_sub((*es).gapsize) < n {
        fatal_syntax(
            (*es).script_lno,
            b"edit script refers to line past end of file\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*es).gapsize == 0 {
        (*es)
            .line = (if (*es).lim == 0 {
            (*es).gapsize = 1024 as libc::c_int as size_t;
            (*es).lim = (*es).gapsize;
            xmalloc(
                ((*es).lim)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            )
        } else {
            (*es).gapsize = (*es).lim;
            (*es).gap = (*es).gapsize;
            (*es).lim <<= 1 as libc::c_int;
            xrealloc(
                (*es).line as *mut libc::c_void,
                ((*es).lim)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            )
        }) as *mut *mut libc::c_char;
    }
    if n < (*es).gap {
        memmove(
            ((*es).line).offset(n as isize).offset((*es).gapsize as isize)
                as *mut libc::c_void,
            ((*es).line).offset(n as isize) as *const libc::c_void,
            ((*es).gap)
                .wrapping_sub(n)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
    } else if (*es).gap < n {
        memmove(
            ((*es).line).offset((*es).gap as isize) as *mut libc::c_void,
            ((*es).line).offset((*es).gap as isize).offset((*es).gapsize as isize)
                as *const libc::c_void,
            n
                .wrapping_sub((*es).gap)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
    }
    let ref mut fresh0 = *((*es).line).offset(n as isize);
    *fresh0 = l;
    (*es).gap = n.wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*es).gapsize = ((*es).gapsize).wrapping_sub(1);
    (*es).gapsize;
}
unsafe extern "C" fn deletelines(
    mut es: *mut editstuff,
    mut n: libc::c_ulong,
    mut nlines: libc::c_ulong,
) {
    let mut l: libc::c_ulong = n.wrapping_add(nlines);
    if ((*es).lim).wrapping_sub((*es).gapsize) < l || l < n {
        fatal_syntax(
            (*es).script_lno,
            b"edit script refers to line past end of file\0" as *const u8
                as *const libc::c_char,
        );
    }
    if l < (*es).gap {
        memmove(
            ((*es).line).offset(l as isize).offset((*es).gapsize as isize)
                as *mut libc::c_void,
            ((*es).line).offset(l as isize) as *const libc::c_void,
            ((*es).gap)
                .wrapping_sub(l)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
    } else if (*es).gap < n {
        memmove(
            ((*es).line).offset((*es).gap as isize) as *mut libc::c_void,
            ((*es).line).offset((*es).gap as isize).offset((*es).gapsize as isize)
                as *const libc::c_void,
            n
                .wrapping_sub((*es).gap)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
    }
    (*es).gap = n;
    (*es)
        .gapsize = ((*es).gapsize as libc::c_ulong).wrapping_add(nlines) as size_t
        as size_t;
}
unsafe extern "C" fn snapshotline(mut f: *mut FILE, mut l: *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    loop {
        let fresh1 = l;
        l = l.offset(1);
        c = *fresh1 as libc::c_int;
        if c == '@' as i32
            && {
                let fresh2 = l;
                l = l.offset(1);
                *fresh2 as libc::c_int != '@' as i32
            }
        {
            return;
        }
        if putc(c, f) == -(1 as libc::c_int) {
            testOerror(f);
        }
        if !(c != '\n' as i32) {
            break;
        }
    };
}
unsafe extern "C" fn snapshotedit_fast(mut es: *mut editstuff, mut f: *mut FILE) {
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lim: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut l: *mut *mut libc::c_char = (*es).line;
    p = l;
    lim = l.offset((*es).gap as isize);
    while p < lim {
        let fresh3 = p;
        p = p.offset(1);
        snapshotline(f, *fresh3);
    }
    p = p.offset((*es).gapsize as isize);
    lim = l.offset((*es).lim as isize);
    while p < lim {
        let fresh4 = p;
        p = p.offset(1);
        snapshotline(f, *fresh4);
    }
}
unsafe extern "C" fn finisheditline(mut finctx: *mut finctx, mut l: *mut libc::c_char) {
    let mut ctx: *mut expctx = &mut (*finctx).ctx;
    (*(*ctx).from).ptr = l;
    if expandline(ctx) < 0 as libc::c_int {
        generic_fatal(
            0 as *const libc::c_char,
            b"%s:%zu: error expanding keywords while applying delta %s\0" as *const u8
                as *const libc::c_char,
            (*top).repository.filename,
            (*finctx).script_lno,
            (*(*ctx).delta).num,
        );
    }
}
unsafe extern "C" fn finishedit_fast(
    mut es: *mut editstuff,
    mut delta: *const delta,
    mut outfile: *mut FILE,
    mut done: bool,
) {
    if done {
        openfcopy(outfile);
        outfile = (*top).flow.res;
        if delta.is_null() {
            snapshotedit_fast(es, outfile);
        } else {
            let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut lim: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut l: *mut *mut libc::c_char = (*es).line;
            let mut fin: *mut fro = (*top).flow.from;
            let mut here: *mut libc::c_char = (*fin).ptr;
            let mut finctx: finctx = {
                let mut init = finctx {
                    ctx: {
                        let mut init = expctx {
                            to: outfile,
                            rewr: 0 as *mut FILE,
                            from: fin,
                            delta: delta,
                            delimstuffed: 1 as libc::c_int != 0,
                            dolog: 1 as libc::c_int != 0,
                            lparts: 0 as *mut divvy,
                        };
                        init
                    },
                    script_lno: (*es).script_lno,
                };
                init
            };
            p = l;
            lim = l.offset((*es).gap as isize);
            while p < lim {
                let fresh5 = p;
                p = p.offset(1);
                finisheditline(&mut finctx, *fresh5);
            }
            p = p.offset((*es).gapsize as isize);
            lim = l.offset((*es).lim as isize);
            while p < lim {
                let fresh6 = p;
                p = p.offset(1);
                finisheditline(&mut finctx, *fresh6);
            }
            (*fin).ptr = here;
            if !(finctx.ctx.lparts).is_null() {
                close_space(finctx.ctx.lparts);
            }
        }
    }
}
unsafe extern "C" fn fopen_update_truncate(mut name: *const libc::c_char) -> *mut FILE {
    if !(RM_STDIO as libc::c_int as libc::c_uint
        == (*(*top).flow.from).rm as libc::c_uint)
    {
        return fopen_safer(name, b"w\0" as *const u8 as *const libc::c_char)
    } else {
        if 0 as libc::c_int != 0 && 0 as libc::c_int > un_link(name) {
            fatal_sys(name);
        }
        return fopen_safer(name, b"w+\0" as *const u8 as *const libc::c_char);
    };
}
pub unsafe extern "C" fn openfcopy(mut f: *mut FILE) {
    (*top).flow.res = f;
    if ((*top).flow.res).is_null() {
        if ((*top).flow.result).is_null() {
            (*top).flow.result = maketemp(2 as libc::c_int);
        }
        (*top).flow.res = fopen_update_truncate((*top).flow.result);
        if ((*top).flow.res).is_null() {
            fatal_sys((*top).flow.result);
        }
    }
}
unsafe extern "C" fn swapeditfiles(mut es: *mut editstuff, mut outfile: *mut FILE) {
    let mut tmpptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut stream: *mut FILE = (*top).flow.res;
    let mut f: *mut fro = (*es).fedit;
    (*es).lcount = 0 as libc::c_int as libc::c_long;
    (*es).corr = 0 as libc::c_int as libc::c_long;
    if f.is_null() {
        (*es)
            .fedit = zlloc(single, ::std::mem::size_of::<fro>() as libc::c_ulong)
            as *mut fro;
        f = (*es).fedit;
        (*f).rm = RM_STDIO;
    }
    (*f).stream = stream;
    (*f).end = ftello(stream);
    rewind(stream);
    tmpptr = (*es).filename;
    (*es).filename = (*top).flow.result;
    (*top).flow.result = tmpptr;
    openfcopy(outfile);
}
unsafe extern "C" fn finishedit_slow(
    mut es: *mut editstuff,
    mut delta: *const delta,
    mut outfile: *mut FILE,
    mut done: bool,
) {
    let mut fe: *mut fro = 0 as *mut fro;
    let mut fc: *mut FILE = 0 as *mut FILE;
    fe = (*es).fedit;
    if !fe.is_null() {
        fc = (*top).flow.res;
        if !delta.is_null() {
            let mut ctx: expctx = {
                let mut init = expctx {
                    to: fc,
                    rewr: 0 as *mut FILE,
                    from: fe,
                    delta: delta,
                    delimstuffed: 0 as libc::c_int != 0,
                    dolog: 1 as libc::c_int != 0,
                    lparts: 0 as *mut divvy,
                };
                init
            };
            while (1 as libc::c_int) < expandline(&mut ctx) {}
            if !(ctx.lparts).is_null() {
                close_space(ctx.lparts);
            }
        } else {
            (*fe).verbatim = ftello((*fe).stream);
            fro_spew(fe, fc);
        }
        fro_close(fe);
    }
    if !done {
        swapeditfiles(es, outfile);
    }
}
unsafe extern "C" fn snapshotedit_slow(mut es: *mut editstuff, mut f: *mut FILE) {
    finishedit_slow(es, 0 as *const delta, 0 as *mut FILE, 0 as libc::c_int != 0);
    fro_spew((*es).fedit, f);
    fro_move((*es).fedit, 0 as libc::c_int as off_t);
}
pub unsafe extern "C" fn finishedit(
    mut es: *mut editstuff,
    mut delta: *const delta,
    mut outfile: *mut FILE,
    mut done: bool,
) {
    if !(RM_STDIO as libc::c_int as libc::c_uint
        == (*(*top).flow.from).rm as libc::c_uint)
    {
        Some(
            finishedit_fast
                as unsafe extern "C" fn(
                    *mut editstuff,
                    *const delta,
                    *mut FILE,
                    bool,
                ) -> (),
        )
    } else {
        Some(
            finishedit_slow
                as unsafe extern "C" fn(
                    *mut editstuff,
                    *const delta,
                    *mut FILE,
                    bool,
                ) -> (),
        )
    }
        .unwrap()(es, delta, outfile, done);
}
pub unsafe extern "C" fn snapshotedit(mut es: *mut editstuff, mut f: *mut FILE) {
    if !(RM_STDIO as libc::c_int as libc::c_uint
        == (*(*top).flow.from).rm as libc::c_uint)
    {
        Some(snapshotedit_fast as unsafe extern "C" fn(*mut editstuff, *mut FILE) -> ())
    } else {
        Some(snapshotedit_slow as unsafe extern "C" fn(*mut editstuff, *mut FILE) -> ())
    }
        .unwrap()(es, f);
}
unsafe extern "C" fn copylines(
    mut es: *mut editstuff,
    mut upto: libc::c_long,
    mut delta: *const delta,
) {
    if !(RM_STDIO as libc::c_int as libc::c_uint
        == (*(*top).flow.from).rm as libc::c_uint)
    {
        (*es).lcount = upto;
    } else {
        let mut c: libc::c_int = 0;
        let mut fc: *mut FILE = 0 as *mut FILE;
        let mut fe: *mut fro = 0 as *mut fro;
        if upto < (*es).lcount {
            finishedit_slow(
                es,
                0 as *const delta,
                0 as *mut FILE,
                0 as libc::c_int != 0,
            );
        }
        fe = (*es).fedit;
        fc = (*top).flow.res;
        if (*es).lcount < upto {
            let mut ctx: expctx = {
                let mut init = expctx {
                    to: fc,
                    rewr: 0 as *mut FILE,
                    from: fe,
                    delta: delta,
                    delimstuffed: 0 as libc::c_int != 0,
                    dolog: 1 as libc::c_int != 0,
                    lparts: 0 as *mut divvy,
                };
                init
            };
            if !delta.is_null() {
                loop {
                    if expandline(&mut ctx) <= 1 as libc::c_int {
                        fatal_syntax(
                            (*es).script_lno,
                            b"edit script refers to line past end of file\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    (*es).lcount += 1;
                    if !((*es).lcount < upto) {
                        break;
                    }
                }
            } else {
                loop {
                    loop {
                        if fro_try_getbyte(&mut c, fe) {
                            fatal_syntax(
                                (*es).script_lno,
                                b"edit script refers to line past end of file\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        if putc(c, fc) == -(1 as libc::c_int) {
                            testOerror(fc);
                        }
                        if !(c != '\n' as i32) {
                            break;
                        }
                    }
                    (*es).lcount += 1;
                    if !((*es).lcount < upto) {
                        break;
                    }
                }
            }
            if !(ctx.lparts).is_null() {
                close_space(ctx.lparts);
            }
        }
    };
}
pub unsafe extern "C" fn copystring(mut es: *mut editstuff, mut atat: *mut atat) {
    atat_display((*top).flow.res, atat, 0 as libc::c_int != 0);
    if !((*top).flow.to).is_null() {
        atat_put((*top).flow.to, atat);
    }
    (*es)
        .lcount = ((*es).lcount as libc::c_ulong).wrapping_add((*atat).line_count)
        as libc::c_long as libc::c_long;
}
pub unsafe extern "C" fn enterstring(mut es: *mut editstuff, mut atat: *mut atat) {
    if RM_STDIO as libc::c_int as libc::c_uint == (*(*top).flow.from).rm as libc::c_uint
    {
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        (*es).filename = 0 as *const libc::c_char;
        (*es).fedit = 0 as *mut fro;
        (*es).corr = 0 as libc::c_int as libc::c_long;
        (*es).lcount = (*es).corr;
        filename = maketemp(1 as libc::c_int);
        (*top).flow.result = filename;
        (*top).flow.res = fopen_update_truncate(filename);
        if ((*top).flow.res).is_null() {
            fatal_sys(filename);
        }
        copystring(es, atat);
    } else {
        let mut c: libc::c_int = 0;
        let mut frew: *mut FILE = 0 as *mut FILE;
        let mut e: libc::c_long = 0;
        let mut oe: libc::c_long = 0;
        let mut amidline: bool = false;
        let mut oamidline: bool = false;
        let mut optr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fin: *mut fro = 0 as *mut fro;
        e = 0 as libc::c_int as libc::c_long;
        (*es).gap = 0 as libc::c_int as size_t;
        (*es).gapsize = (*es).lim;
        fin = (*top).flow.from;
        fro_trundling(0 as libc::c_int != 0, fin);
        frew = (*top).flow.to;
        fro_must_getbyte(&mut c, fin);
        if !frew.is_null() {
            afputc(c, frew);
        }
        amidline = 0 as libc::c_int != 0;
        loop {
            optr = (*fin).ptr;
            fro_must_getbyte(&mut c, fin);
            if !frew.is_null() {
                afputc(c, frew);
            }
            oamidline = amidline;
            oe = e;
            let mut current_block_38: u64;
            match c {
                10 => {
                    e += 1;
                    e;
                    amidline = 0 as libc::c_int != 0;
                    current_block_38 = 9853141518545631134;
                }
                64 => {
                    fro_must_getbyte(&mut c, fin);
                    if !frew.is_null() {
                        afputc(c, frew);
                    }
                    if c != '@' as i32 {
                        (*es).lcount = e + amidline as libc::c_long;
                        (*es).corr = 0 as libc::c_int as libc::c_long;
                        return;
                    }
                    current_block_38 = 14308627023463179144;
                }
                _ => {
                    current_block_38 = 14308627023463179144;
                }
            }
            match current_block_38 {
                14308627023463179144 => {
                    amidline = 1 as libc::c_int != 0;
                }
                _ => {}
            }
            if !oamidline {
                insertline(es, oe as libc::c_ulong, optr);
            }
        }
    };
}
pub unsafe extern "C" fn editstring(
    mut es: *mut editstuff,
    mut script: *const atat,
    mut delta: *const delta,
) {
    let mut ed: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut frew: *mut FILE = 0 as *mut FILE;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut line_lim: libc::c_long = 9223372036854775807 as libc::c_long;
    let mut fe: *mut fro = 0 as *mut fro;
    let mut i: libc::c_long = 0;
    let mut fin: *mut fro = 0 as *mut fro;
    let mut j: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut dc: diffcmd = diffcmd {
        line1: 0,
        nlines: 0,
        adprev: 0,
        dafter: 0,
    };
    (*es).script_lno = (*script).lno;
    (*es).lcount += (*es).corr;
    (*es).corr = 0 as libc::c_int as libc::c_long;
    frew = (*top).flow.to;
    fin = (*top).flow.from;
    fro_must_getbyte(&mut c, fin);
    if !frew.is_null() {
        afputc(c, frew);
    }
    initdiffcmd(&mut dc);
    loop {
        ed = getdiffcmd(fin, 1 as libc::c_int != 0, frew, &mut dc);
        if !(0 as libc::c_int <= ed) {
            break;
        }
        if RM_STDIO as libc::c_int as libc::c_uint == (*fin).rm as libc::c_uint
            && line_lim <= dc.line1
        {
            fatal_syntax(
                (*es).script_lno,
                b"edit script refers to line past end of file\0" as *const u8
                    as *const libc::c_char,
            );
        } else if ed == 0 {
            copylines(es, dc.line1 - 1 as libc::c_int as libc::c_long, delta);
            i = dc.nlines;
            (*es).corr -= i;
            (*es).lcount += i;
            if !(RM_STDIO as libc::c_int as libc::c_uint == (*fin).rm as libc::c_uint) {
                deletelines(
                    es,
                    ((*es).lcount + (*es).corr) as libc::c_ulong,
                    i as libc::c_ulong,
                );
            } else {
                fe = (*es).fedit;
                loop {
                    loop {
                        if fro_try_getbyte(&mut c, fe) {
                            if i != 1 as libc::c_int as libc::c_long {
                                fatal_syntax(
                                    (*es).script_lno,
                                    b"edit script refers to line past end of file\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            line_lim = dc.dafter;
                            break;
                        } else if !(c != '\n' as i32) {
                            break;
                        }
                    }
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                }
            }
            (*es).script_lno = ((*es).script_lno).wrapping_add(1);
            (*es).script_lno;
        } else {
            copylines(es, dc.line1, delta);
            i = dc.nlines;
            if !(RM_STDIO as libc::c_int as libc::c_uint == (*fin).rm as libc::c_uint) {
                j = (*es).lcount + (*es).corr;
            }
            (*es).corr += i;
            if RM_STDIO as libc::c_int as libc::c_uint == (*fin).rm as libc::c_uint
                && {
                    f = (*top).flow.res;
                    !delta.is_null()
                }
            {
                let mut ctx: expctx = {
                    let mut init = expctx {
                        to: f,
                        rewr: frew,
                        from: fin,
                        delta: delta,
                        delimstuffed: 1 as libc::c_int != 0,
                        dolog: 1 as libc::c_int != 0,
                        lparts: 0 as *mut divvy,
                    };
                    init
                };
                loop {
                    let mut current_block_34: u64;
                    match expandline(&mut ctx) {
                        0 | 1 => {
                            if i == 1 as libc::c_int as libc::c_long {
                                return;
                            }
                            current_block_34 = 4253290527279597971;
                        }
                        -1 => {
                            current_block_34 = 4253290527279597971;
                        }
                        _ => {
                            current_block_34 = 14072441030219150333;
                        }
                    }
                    match current_block_34 {
                        4253290527279597971 => {
                            fatal_syntax(
                                (*es).script_lno,
                                b"edit script ends prematurely\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        _ => {}
                    }
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                }
                if !(ctx.lparts).is_null() {
                    close_space(ctx.lparts);
                }
            } else {
                loop {
                    if !(RM_STDIO as libc::c_int as libc::c_uint
                        == (*fin).rm as libc::c_uint)
                    {
                        let fresh7 = j;
                        j = j + 1;
                        insertline(es, fresh7 as libc::c_ulong, (*fin).ptr);
                    }
                    loop {
                        fro_must_getbyte(&mut c, fin);
                        if !frew.is_null() {
                            afputc(c, frew);
                        }
                        if c == '@' as i32 {
                            fro_must_getbyte(&mut c, fin);
                            if !frew.is_null() {
                                afputc(c, frew);
                            }
                            if c != '@' as i32 {
                                i -= 1;
                                if i != 0 {
                                    fatal_syntax(
                                        (*es).script_lno,
                                        b"edit script ends prematurely\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                return;
                            }
                        }
                        if RM_STDIO as libc::c_int as libc::c_uint
                            == (*fin).rm as libc::c_uint
                        {
                            if putc(c, f) == -(1 as libc::c_int) {
                                testOerror(f);
                            }
                        }
                        if c == '\n' as i32 {
                            break;
                        }
                    }
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                }
            }
            (*es)
                .script_lno = ((*es).script_lno as libc::c_ulong)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_long + dc.nlines) as libc::c_ulong,
                ) as size_t as size_t;
        }
    };
}
unsafe extern "C" fn naturalize(
    mut m: *mut maybe,
    mut symbolic: *mut bool,
) -> libc::c_int {
    let mut e: libc::c_int = 0;
    let mut r: ssize_t = 0;
    let mut linkcount: libc::c_int = 8 as libc::c_int;
    let mut fn_0: *mut cbuf = &mut (*m).tentative;
    let mut space: *mut divvy = (*m).space;
    let mut len: size_t = 256 as libc::c_int as size_t;
    let mut chased: *mut libc::c_char = alloc(space, len) as *mut libc::c_char;
    let mut orig: *const libc::c_char = (*fn_0).string;
    loop {
        r = readlink((*fn_0).string, chased, len);
        if !(r != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        if len == r as size_t {
            len <<= 1 as libc::c_int;
            chased = alloc(space, len) as *mut libc::c_char;
        } else {
            let fresh8 = linkcount;
            linkcount = linkcount - 1;
            if fresh8 == 0 {
                *__errno_location() = 40 as libc::c_int;
                return -(1 as libc::c_int);
            } else {
                *chased.offset(r as isize) = '\0' as i32 as libc::c_char;
                if isSLASH(*chased.offset(0 as libc::c_int as isize) as libc::c_int) {
                    (*fn_0).string = chased;
                    (*fn_0).size = r as size_t;
                } else {
                    let mut s: *const libc::c_char = (*fn_0).string;
                    accumulate_range(space, s, basefilename(s));
                    accs(space, chased);
                    (*fn_0).string = finish_string(space, &mut (*fn_0).size);
                }
            }
        }
    }
    e = *__errno_location();
    *symbolic = (*fn_0).string != orig;
    *__errno_location() = e;
    match e {
        22 => return 1 as libc::c_int,
        2 => return 0 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
pub unsafe extern "C" fn rcswriteopen(mut m: *mut maybe) -> *mut fro {
    let mut RCSbuf: *mut cbuf = &mut (*m).tentative;
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut repofn: *const libc::c_char = 0 as *const libc::c_char;
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    let mut f: *mut fro = 0 as *mut fro;
    let mut len: size_t = 0;
    let mut e: libc::c_int = 0;
    let mut exists: libc::c_int = 0;
    let mut fdesc: libc::c_int = 0;
    let mut fdescSafer: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut waslocked: bool = false;
    let mut statbuf: stat = stat {
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
    let mut symbolic: bool = 0 as libc::c_int != 0;
    let mut lfn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sff: *mut sff = (*top).behavior.sff;
    waslocked = 0 as libc::c_int <= (*top).repository.fd_lock;
    exists = naturalize(m, &mut symbolic);
    if exists < (*m).mustread as libc::c_int | waslocked as libc::c_int {
        return 0 as *mut fro;
    }
    repofn = (*RCSbuf).string;
    accs((*m).space, (*RCSbuf).string);
    lfn = finish_string((*m).space, &mut len);
    sp = basefilename(lfn);
    x = rcssuffix(repofn);
    if x.is_null() && symbolic as libc::c_int != 0 {
        generic_error(
            0 as *const libc::c_char,
            b"symbolic link to non RCS file `%s'\0" as *const u8 as *const libc::c_char,
            repofn,
        );
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut fro;
    }
    if *sp as libc::c_int == *x as libc::c_int {
        generic_error(
            0 as *const libc::c_char,
            b"RCS filename `%s' incompatible with suffix `%s'\0" as *const u8
                as *const libc::c_char,
            sp,
            x,
        );
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut fro;
    }
    if *x != 0 {
        tp = lfn.offset(len as isize);
        let fresh9 = tp;
        tp = tp.offset(-1);
        *fresh9 = '\0' as i32 as libc::c_char;
        while sp < tp as *const libc::c_char {
            *tp = *tp.offset(-(1 as libc::c_int) as isize);
            tp = tp.offset(-1);
            tp;
        }
        *tp = *x;
    } else {
        tp = lfn.offset(len as isize).offset(-(1 as libc::c_int as isize));
        if '_' as i32 == *tp as libc::c_int {
            generic_error(
                0 as *const libc::c_char,
                b"RCS filename `%s' ends with `%c'\0" as *const u8
                    as *const libc::c_char,
                repofn,
                *tp as libc::c_int,
            );
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut fro;
        }
        *tp = '_' as i32 as libc::c_char;
    }
    let ref mut fresh10 = (*sff.offset(waslocked as isize)).filename;
    *fresh10 = intern(single, lfn, len);
    f = 0 as *mut fro;
    isr_do((*top).behavior.isr, ISR_CATCHINTS);
    isr_do((*top).behavior.isr, ISR_IGNOREINTS);
    seteid();
    fdesc = open(
        lfn,
        0 as libc::c_int | 0 as libc::c_int | 0o1 as libc::c_int | 0o100 as libc::c_int
            | 0o200 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
    );
    fdescSafer = fd_safer(fdesc);
    e = *__errno_location();
    setrid();
    if 0 as libc::c_int <= fdesc {
        (*sff.offset(0 as libc::c_int as isize)).disposition = effective;
    }
    if 0 as libc::c_int > fdescSafer {
        if e == 13 as libc::c_int && !(0 as libc::c_int > stat(lfn, &mut statbuf)) {
            e = 17 as libc::c_int;
        }
    } else {
        e = 2 as libc::c_int;
        if exists != 0 {
            f = fro_open(
                repofn,
                b"r\0" as *const u8 as *const libc::c_char,
                (*m).status,
            );
            e = *__errno_location();
            if !f.is_null() && waslocked as libc::c_int != 0 {
                ORCSclose();
                seteid();
                r = un_link(
                    (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
                );
                e = *__errno_location();
                setrid();
                *__errno_location() = e;
                if 0 as libc::c_int > r {
                    fatal_sys(
                        (*((*top).behavior.sff).offset(0 as libc::c_int as isize))
                            .filename,
                    );
                }
                let ref mut fresh11 = (*sff.offset(0 as libc::c_int as isize)).filename;
                *fresh11 = lfn;
            }
        }
        (*top).repository.fd_lock = fdescSafer;
    }
    isr_do((*top).behavior.isr, ISR_RESTOREINTS);
    *__errno_location() = e;
    return f;
}
pub unsafe extern "C" fn chnamemod(
    mut fromp: *mut *mut FILE,
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut set_mode: libc::c_int,
    mut mode: mode_t,
    mtime: timespec,
) -> libc::c_int {
    let mut mode_while_renaming: mode_t = mode;
    let mut fchmod_set_mode: libc::c_int = 0 as libc::c_int;
    let mut st: stat = stat {
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
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 && set_mode <= 0 as libc::c_int
        {
            if 0 as libc::c_int > fstat(fileno(*fromp), &mut st) {
                return -(1 as libc::c_int);
            }
            if 0 as libc::c_int != 0 && set_mode <= 0 as libc::c_int {
                mode = st.st_mode;
            }
        }
    }
    if (0 as libc::c_int) < set_mode
        && !(0 as libc::c_int > change_mode(fileno(*fromp), mode_while_renaming))
    {
        fchmod_set_mode = set_mode;
    }
    Ozclose(fromp);
    if fchmod_set_mode < set_mode && 0 as libc::c_int > chmod(from, mode_while_renaming)
    {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int > setmtime(from, mtime) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != 0 && 0 as libc::c_int > un_link(to)
        && *__errno_location() != 2 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int > rename(from, to) && !nfs_NOENT_p() {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != 0 && (0 as libc::c_int) < set_mode
        && 0 as libc::c_int > chmod(to, mode)
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn setmtime(
    mut file: *const libc::c_char,
    mtime: timespec,
) -> libc::c_int {
    let mut amtime: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    if mtime.tv_sec == -(1 as libc::c_int) as time_t {
        return 0 as libc::c_int;
    }
    amtime[0 as libc::c_int as usize] = (*top).behavior.now;
    amtime[1 as libc::c_int as usize] = mtime;
    return utimens(file, amtime.as_mut_ptr() as *const timespec);
}
pub unsafe extern "C" fn findlock(
    mut delete: bool,
    mut target: *mut *mut delta,
) -> libc::c_int {
    let mut rl: *const rcslock = 0 as *const rcslock;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut found: *mut link = 0 as *mut link;
    let mut me: *const libc::c_char = getcaller();
    box_0.next = (*(*top).repository.r).locks;
    if (box_0.next).is_null()
        || {
            found = lock_memq(
                &mut box_0,
                1 as libc::c_int != 0,
                me as *const libc::c_void,
            );
            found.is_null()
        }
    {
        return 0 as libc::c_int;
    }
    if !(lock_memq((*found).next, 1 as libc::c_int != 0, me as *const libc::c_void))
        .is_null()
    {
        generic_error(
            (*top).repository.filename,
            b"multiple revisions locked by %s; please specify one\0" as *const u8
                as *const libc::c_char,
            me,
        );
        return 2 as libc::c_int;
    }
    rl = (*(*found).next).entry as *const rcslock;
    *target = (*rl).delta;
    if delete {
        lock_drop(&mut box_0, found);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn addsymbol(
    mut num: *const libc::c_char,
    mut name: *const libc::c_char,
    mut rebind: bool,
) -> libc::c_int {
    let mut current_block: u64;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut d: *mut symdef = 0 as *mut symdef;
    box_0.next = (*(*top).repository.r).symbols;
    tp = &mut box_0;
    loop {
        if ((*tp).next).is_null() {
            current_block = 1856101646708284338;
            break;
        }
        let mut dk: *const symdef = (*(*tp).next).entry as *const symdef;
        if strcmp(name, (*dk).meaningful) == 0 {
            if strcmp((*dk).underlying, num) == 0 {
                return 0 as libc::c_int
            } else if rebind {
                d = alloc(single, ::std::mem::size_of::<symdef>() as libc::c_ulong)
                    as *mut symdef;
                (*d).meaningful = name;
                (*d).underlying = num;
                (*tp)
                    .next = prepend(
                    d as *const libc::c_void,
                    (*(*tp).next).next,
                    single,
                );
                current_block = 15833951440152127805;
                break;
            } else {
                generic_error(
                    (*top).repository.filename,
                    b"symbolic name %s already bound to %s\0" as *const u8
                        as *const libc::c_char,
                    name,
                    (*dk).underlying,
                );
                return -(1 as libc::c_int);
            }
        } else {
            tp = (*tp).next;
        }
    }
    match current_block {
        1856101646708284338 => {
            d = alloc(single, ::std::mem::size_of::<symdef>() as libc::c_ulong)
                as *mut symdef;
            (*d).meaningful = name;
            (*d).underlying = num;
            box_0.next = prepend(d as *const libc::c_void, box_0.next, single);
        }
        _ => {}
    }
    (*(*top).repository.r).symbols = box_0.next;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn checkaccesslist() -> bool {
    let mut ls: *mut link = (*(*top).repository.r).access;
    if ls.is_null() || stat_mine_p(&mut (*top).repository.stat) as libc::c_int != 0
        || caller_login_p(b"root\0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0
    {
        return 1 as libc::c_int != 0;
    }
    while !ls.is_null() {
        if caller_login_p((*ls).entry as *const libc::c_char) {
            return 1 as libc::c_int != 0;
        }
        ls = (*ls).next;
    }
    generic_error(
        (*top).repository.filename,
        b"user %s not on the access list\0" as *const u8 as *const libc::c_char,
        getcaller(),
    );
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn dorewrite(
    mut lockflag: bool,
    mut changed: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut e: libc::c_int = 0;
    if lockflag {
        if changed != 0 {
            let mut frew: *mut FILE = 0 as *mut FILE;
            if changed < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            putadmin();
            frew = (*top).flow.rewr;
            puttree((*top).repository.tip, frew);
            aprintf(
                frew,
                b"\n\n%s\n\0" as *const u8 as *const libc::c_char,
                (tiny_desc.bytes).as_ptr() as *const libc::c_char,
            );
            (*top).flow.to = frew;
        } else {
            let mut nr: libc::c_int = !((*top).flow.rewr).is_null() as libc::c_int;
            let mut ne: libc::c_int = 0 as libc::c_int;
            ORCSclose();
            seteid();
            isr_do((*top).behavior.isr, ISR_IGNOREINTS);
            if 0 as libc::c_int != 0 && nr != 0 {
                nr = un_link(
                    (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
                );
                ne = *__errno_location();
                keepdirtemp(
                    (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
                );
            }
            r = un_link(
                (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
            );
            e = *__errno_location();
            keepdirtemp(
                (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
            );
            isr_do((*top).behavior.isr, ISR_RESTOREINTS);
            setrid();
            if 0 as libc::c_int > r {
                syserror(
                    e,
                    (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
                );
            }
            if 0 as libc::c_int != 0 && nr != 0 as libc::c_int {
                syserror(
                    ne,
                    (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
                );
                r = -(1 as libc::c_int);
            }
        }
    }
    return r;
}
pub unsafe extern "C" fn donerewrite(
    mut changed: libc::c_int,
    mtime: timespec,
) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut lr: libc::c_int = 0;
    let mut le: libc::c_int = 0;
    if changed != 0 && !(*top).flow.erroneous {
        let mut repo_stat: *mut stat = &mut (*top).repository.stat;
        let mut repo_filename: *const libc::c_char = (*top).repository.filename;
        let mut from: *mut fro = (*top).flow.from;
        let mut frew: *mut FILE = (*top).flow.rewr;
        if !from.is_null() {
            fro_spew(from, frew);
            fro_zclose(&mut (*top).flow.from);
        }
        if (1 as libc::c_int as libc::c_ulong) < (*repo_stat).st_nlink {
            generic_warn(
                (*top).repository.filename,
                b"breaking hard link\0" as *const u8 as *const libc::c_char,
            );
        }
        aflush(frew);
        seteid();
        isr_do((*top).behavior.isr, ISR_IGNOREINTS);
        r = chnamemod(
            &mut (*top).flow.rewr,
            (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
            repo_filename,
            changed,
            (*repo_stat).st_mode
                & !(0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as mode_t,
            mtime,
        );
        frew = (*top).flow.rewr;
        e = *__errno_location();
        keepdirtemp((*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename);
        isr_do((*top).behavior.isr, ISR_RESTOREINTS);
        setrid();
        if 0 as libc::c_int > r {
            syserror(e, repo_filename);
            generic_error(
                0 as *const libc::c_char,
                b"saved in %s\0" as *const u8 as *const libc::c_char,
                (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
            );
        }
        if 0 as libc::c_int != 0 && 0 as libc::c_int > lr {
            syserror(
                le,
                (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
            );
            r = -(1 as libc::c_int);
        }
    }
    return r;
}
pub unsafe extern "C" fn ORCSclose() {
    let mut repo_fd_lock: libc::c_int = (*top).repository.fd_lock;
    if 0 as libc::c_int <= repo_fd_lock {
        if 0 as libc::c_int > close(repo_fd_lock) {
            fatal_sys(
                (*((*top).behavior.sff).offset(0 as libc::c_int as isize)).filename,
            );
        }
        (*top).repository.fd_lock = -(1 as libc::c_int);
    }
    Ozclose(&mut (*top).flow.rewr);
}
pub unsafe extern "C" fn ORCSerror() {
    let mut repo_fd_lock: libc::c_int = (*top).repository.fd_lock;
    if 0 as libc::c_int <= repo_fd_lock {
        close(repo_fd_lock);
    }
    if !((*top).flow.rewr).is_null() {
        close(fileno((*top).flow.rewr));
    }
}
pub unsafe extern "C" fn unexpected_EOF() {
    generic_fatal(
        (*top).repository.filename,
        b"unexpected EOF in diff output\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn initdiffcmd(mut dc: *mut diffcmd) {
    (*dc).adprev = 0 as libc::c_int as libc::c_long;
    (*dc).dafter = 0 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn badDiffOutput(mut buf: *const libc::c_char) {
    generic_fatal(
        (*top).repository.filename,
        b"bad diff output line: %s\0" as *const u8 as *const libc::c_char,
        buf,
    );
}
unsafe extern "C" fn diffLineNumberTooLarge(mut buf: *const libc::c_char) {
    generic_fatal(
        (*top).repository.filename,
        b"diff line number too large: %s\0" as *const u8 as *const libc::c_char,
        buf,
    );
}
pub unsafe extern "C" fn getdiffcmd(
    mut finfile: *mut fro,
    mut delimiter: bool,
    mut foutfile: *mut FILE,
    mut dc: *mut diffcmd,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut fout: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fin: *mut fro = 0 as *mut fro;
    let mut line1: libc::c_long = 0;
    let mut nlines: libc::c_long = 0;
    let mut t: libc::c_long = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    fin = finfile;
    fout = foutfile;
    if fro_try_getbyte(&mut c, fin) {
        if delimiter {
            unexpected_EOF();
        }
        return -(1 as libc::c_int);
    }
    if delimiter {
        if c == '@' as i32 {
            fro_must_getbyte(&mut c, fin);
            if c == '@' as i32 {
                buf[0 as libc::c_int as usize] = c as libc::c_char;
                buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                badDiffOutput(buf.as_mut_ptr());
            }
            if !fout.is_null() {
                aprintf(
                    fout,
                    b"%c%c\0" as *const u8 as *const libc::c_char,
                    '@' as i32,
                    c,
                );
            }
            return -(1 as libc::c_int);
        }
    }
    p = buf.as_mut_ptr();
    loop {
        if buf
            .as_mut_ptr()
            .offset(8192 as libc::c_int as isize)
            .offset(-(2 as libc::c_int as isize)) <= p
        {
            generic_fatal(
                (*top).repository.filename,
                b"diff output command line too long\0" as *const u8
                    as *const libc::c_char,
            );
        }
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = c as libc::c_char;
        if fro_try_getbyte(&mut c, fin) {
            unexpected_EOF();
        }
        if !(c != '\n' as i32) {
            break;
        }
    }
    *p = '\0' as i32 as libc::c_char;
    p = buf.as_mut_ptr().offset(1 as libc::c_int as isize);
    loop {
        let fresh13 = p;
        p = p.offset(1);
        c = *fresh13 as libc::c_int;
        if !(c == ' ' as i32) {
            break;
        }
    }
    line1 = 0 as libc::c_int as libc::c_long;
    while *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if (9223372036854775807 as libc::c_long / 10 as libc::c_int as libc::c_long)
            < line1
            || {
                t = line1 * 10 as libc::c_int as libc::c_long;
                line1 = t + (c - '0' as i32) as libc::c_long;
                line1 < t
            }
        {
            diffLineNumberTooLarge(buf.as_mut_ptr());
        }
        let fresh14 = p;
        p = p.offset(1);
        c = *fresh14 as libc::c_int;
    }
    while c == ' ' as i32 {
        let fresh15 = p;
        p = p.offset(1);
        c = *fresh15 as libc::c_int;
    }
    nlines = 0 as libc::c_int as libc::c_long;
    while *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if (9223372036854775807 as libc::c_long / 10 as libc::c_int as libc::c_long)
            < nlines
            || {
                t = nlines * 10 as libc::c_int as libc::c_long;
                nlines = t + (c - '0' as i32) as libc::c_long;
                nlines < t
            }
        {
            diffLineNumberTooLarge(buf.as_mut_ptr());
        }
        let fresh16 = p;
        p = p.offset(1);
        c = *fresh16 as libc::c_int;
    }
    if c == '\r' as i32 {
        let fresh17 = p;
        p = p.offset(1);
        c = *fresh17 as libc::c_int;
    }
    if c != 0 || nlines == 0 {
        badDiffOutput(buf.as_mut_ptr());
    }
    if line1 + nlines < line1 {
        diffLineNumberTooLarge(buf.as_mut_ptr());
    }
    match buf[0 as libc::c_int as usize] as libc::c_int {
        97 => {
            if line1 < (*dc).adprev {
                generic_fatal(
                    (*top).repository.filename,
                    b"backward insertion in diff output: %s\0" as *const u8
                        as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
            }
            (*dc).adprev = line1 + 1 as libc::c_int as libc::c_long;
        }
        100 => {
            if line1 < (*dc).adprev || line1 < (*dc).dafter {
                generic_fatal(
                    (*top).repository.filename,
                    b"backward deletion in diff output: %s\0" as *const u8
                        as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
            }
            (*dc).adprev = line1;
            (*dc).dafter = line1 + nlines;
        }
        _ => {
            badDiffOutput(buf.as_mut_ptr());
        }
    }
    if !fout.is_null() {
        aprintf(fout, b"%s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    }
    (*dc).line1 = line1;
    (*dc).nlines = nlines;
    return (buf[0 as libc::c_int as usize] as libc::c_int == 'a' as i32) as libc::c_int;
}

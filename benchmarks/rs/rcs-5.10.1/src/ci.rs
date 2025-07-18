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
    fn rewind(__stream: *mut FILE);
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    static prog_diff: [libc::c_char; 0];
    static diff_flags: [libc::c_char; 0];
    static mut top: *mut top;
    static ks_revno: [libc::c_char; 0];
    static tiny_ciklog: tinysym;
    fn un_link(s: *const libc::c_char) -> libc::c_int;
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn chnamemod(
        fromp: *mut *mut FILE,
        from: *const libc::c_char,
        to: *const libc::c_char,
        set_mode: libc::c_int,
        mode: mode_t,
        mtime: timespec,
    ) -> libc::c_int;
    fn setmtime(file: *const libc::c_char, mtime: timespec) -> libc::c_int;
    fn findlock(delete: bool, target: *mut *mut delta) -> libc::c_int;
    fn addsymbol(
        num: *const libc::c_char,
        name: *const libc::c_char,
        rebind: bool,
    ) -> libc::c_int;
    fn checkaccesslist() -> bool;
    fn dorewrite(lockflag: bool, changed: libc::c_int) -> libc::c_int;
    fn donerewrite(changed: libc::c_int, mtime: timespec) -> libc::c_int;
    fn ORCSclose();
    fn rcsfcmp(
        xfp: *mut fro,
        xstatp: *const stat,
        uname: *const libc::c_char,
        delta: *const delta,
    ) -> libc::c_int;
    fn pairnames(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        rcsopen: Option::<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> libc::c_int;
    fn buildrevision(
        deltas: *const wlink,
        target: *mut delta,
        outfile: *mut FILE,
        expandflag: bool,
    ) -> *const libc::c_char;
    fn cleanlogmsg(m: *const libc::c_char, s: size_t) -> cbuf;
    fn yesorno(default_answer: bool, question: *const libc::c_char, _: ...) -> bool;
    fn putdesc(cb: *mut cbuf, textflag: bool, textfile: *mut libc::c_char);
    fn getsstdin(
        option: *const libc::c_char,
        name: *const libc::c_char,
        note: *const libc::c_char,
    ) -> cbuf;
    fn putadmin();
    fn puttree(root: *const delta, fout: *mut FILE);
    fn putdtext(
        delta: *const delta,
        srcname: *const libc::c_char,
        fout: *mut FILE,
        diffmt: bool,
    ) -> bool;
    fn putdftext(
        delta: *const delta,
        finfile: *mut fro,
        foutfile: *mut FILE,
        diffmt: bool,
    );
    fn getoldkeys(fp: *mut fro) -> bool;
    fn checksid(id: *const libc::c_char);
    fn checkssym(sym: *const libc::c_char);
    fn countnumflds(s: *const libc::c_char) -> libc::c_int;
    fn take(count: size_t, ref_0: *const libc::c_char) -> cbuf;
    fn cmpnum(num1: *const libc::c_char, num2: *const libc::c_char) -> libc::c_int;
    fn cmpnumfld(
        num1: *const libc::c_char,
        num2: *const libc::c_char,
        fld: libc::c_int,
    ) -> libc::c_int;
    fn cmpdate(d1: *const libc::c_char, d2: *const libc::c_char) -> libc::c_int;
    fn compartial(
        num1: *const libc::c_char,
        num2: *const libc::c_char,
        length: libc::c_int,
    ) -> libc::c_int;
    fn gr_revno(revno: *const libc::c_char, store: *mut *mut wlink) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const libc::c_char, fp: *mut fro) -> bool;
    fn namedrev(name: *const libc::c_char, delta: *mut delta) -> *const libc::c_char;
    fn time2date(unixtime: time_t, date: *mut libc::c_char);
    fn str2date(source: *const libc::c_char, target: *mut libc::c_char);
    fn date2time(source: *const libc::c_char) -> time_t;
    fn zone_set(s: *const libc::c_char);
    fn date2str(
        date: *const libc::c_char,
        datebuf: *mut libc::c_char,
    ) -> *const libc::c_char;
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn redefined(c: libc::c_int);
    fn chk_set_rev(rev: *mut *const libc::c_char, arg: *mut libc::c_char);
    fn set_empty_log_message(cb: *mut cbuf);
    fn ffree();
    fn runv(
        infd: libc::c_int,
        outname: *const libc::c_char,
        args: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn setRCSversion(str: *const libc::c_char);
    fn getRCSINIT(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        newargv: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn unspecified_timespec() -> timespec;
    fn file_mtime(enable: bool, st: *const stat) -> timespec;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn abort() -> !;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn diagnose(fmt: *const libc::c_char, _: ...);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut exit_failure: libc::c_int;
    static mut plexus: *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn accumulate_nbytes(divvy: *mut divvy, start: *const libc::c_char, count: size_t);
    fn accumulate_range(
        divvy: *mut divvy,
        beg: *const libc::c_char,
        end: *const libc::c_char,
    );
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn close_space(divvy: *mut divvy);
    fn prepend(x: *const libc::c_void, ls: *mut link, to: *mut divvy) -> *mut link;
    fn stat_mine_p(st: *mut stat) -> bool;
    fn currently_setuid_p() -> bool;
    fn setrid();
    fn getcaller() -> *const libc::c_char;
    fn caller_login_p(login: *const libc::c_char) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn addlock_maybe(delta: *mut delta, selfsame: bool, verbose: bool) -> libc::c_int;
    fn change_mode(fd: libc::c_int, mode: mode_t) -> libc::c_int;
    fn Ierror();
    fn fopen_safer(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
    ) -> *mut FILE;
    fn Ozclose(p: *mut *mut FILE);
    fn aflush(f: *mut FILE);
    fn newline(f: *mut FILE);
    fn maketemp(n: libc::c_int) -> *const libc::c_char;
    fn makedirtemp(isworkfile: bool) -> *const libc::c_char;
    fn keepdirtemp(name: *const libc::c_char);
    fn tempunlink();
    fn dirtempunlink();
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_spew(f: *mut fro, to: *mut FILE);
    fn grok_resynch(repo: *mut repo);
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
pub struct u_symdef {
    pub u: symdef,
    pub override_0: bool,
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub type submain_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
    *mut *mut libc::c_char,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yacmd {
    pub func: Option::<submain_t>,
    pub aka: *const uint8_t,
    pub pr: *mut program,
}
pub type isr_actions = libc::c_uint;
pub const ISR_CATCHMMAPINTS: isr_actions = 3;
pub const ISR_RESTOREINTS: isr_actions = 2;
pub const ISR_IGNOREINTS: isr_actions = 1;
pub const ISR_CATCHINTS: isr_actions = 0;
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
pub struct reason {
    pub upfront: cbuf,
    pub delayed: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work {
    pub st: stat,
    pub fro: *mut fro,
    pub ex: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bud {
    pub num: cbuf,
    pub d: delta,
    pub br: wlink,
    pub keep: bool,
    pub target: *mut delta,
    pub getcurdate_buffer: [libc::c_char; 22],
    pub work_mtime: timespec,
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: libc::c_long) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
static mut ci_blurb: [libc::c_char; 52] = unsafe {
    *::std::mem::transmute::<
        &[u8; 52],
        &[libc::c_char; 52],
    >(b"Check in revisions of RCS files from working files.\0")
};
static mut ci_help: [libc::c_char; 2285] = unsafe {
    *::std::mem::transmute::<
        &[u8; 2285],
        &[libc::c_char; 2285],
    >(
        b"[options] file...\nOptions:\n  -f[REV]       Force new entry, even if no content changed.\n  -I[REV]       Interactive.\n  -i[REV]       Initial checkin; error if RCS file already exists.\n  -j[REV]       Just checkin, don't init; error if RCS file does not exist.\n  -k[REV]       Compute revision from working file keywords.\n  -q[REV]       Quiet mode.\n  -r[REV]       Do normal checkin, if REV is specified;\n                otherwise, release lock and delete working file.\n  -l[REV]       Like -r, but immediately checkout locked (co -l) afterwards.\n  -u[REV]       Like -l, but checkout unlocked (co -u).\n  -M[REV]       Reset working file mtime (relevant for -l, -u).\n  -d[DATE]      Use DATE (or working file mtime).\n  -mMSG         Use MSG as the log message.\n  -nNAME        Assign symbolic NAME to the entry; NAME must be new.\n  -NNAME        Like -n, but overwrite any previous assignment.\n  -sSTATE       Set state to STATE.\n  -t-TEXT       Set description to TEXT.\n  -tFILENAME    Set description from text read from FILENAME.\n  -T            Set the RCS file's modification time to the new\n                revision's time if the former precedes the latter and there\n                is a new revision; preserve the RCS file's modification\n                time otherwise.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -wWHO         Use WHO as the author.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution\n                and also the default timezone for -dDATE.\n\nMultiple flags in {fiIjklMqru} may be used, except for -r, -l, -u, which are\nmutually exclusive.  If specified, REV can be symbolic, numeric, or mixed:\n  symbolic      Must have been defined previously (see -n, -N).\n  $             Determine from keyword values in the working file.\n  .N            Prepend default branch => DEFBR.N\n  BR.N          Use this, but N must be greater than any existing\n                on BR, or BR must be new.\n  BR            Latest rev on branch BR + 1 => BR.(L+1), or BR.1 if new branch.\nIf REV is omitted, compute it from the last lock (co -l), perhaps\nstarting a new branch.  If there is no lock, use DEFBR.(L+1).\n\0",
    )
};
unsafe extern "C" fn cleanup(mut exitstatus: *mut libc::c_int, mut work: *mut work) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    fro_zclose(&mut (*work).fro);
    Ozclose(&mut (*work).ex);
    Ozclose(&mut (*top).flow.res);
    ORCSclose();
    dirtempunlink();
}
unsafe extern "C" fn incnum(mut onum: *const libc::c_char, mut nnum: *mut cbuf) {
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut np: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: size_t = 0;
    accf(plexus, b"%s%c\0" as *const u8 as *const libc::c_char, onum, '\0' as i32);
    np = finish_string(plexus, &mut (*nnum).size);
    (*nnum).string = np;
    l = ((*nnum).size).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    tp = np.offset(l as isize);
    while np != tp {
        tp = tp.offset(-1);
        if *(*__ctype_b_loc()).offset(*tp as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            if *tp as libc::c_int != '9' as i32 {
                *tp += 1;
                *tp;
                (*nnum).size = ((*nnum).size).wrapping_sub(1);
                (*nnum).size;
                return;
            }
            *tp = '0' as i32 as libc::c_char;
        } else {
            tp = tp.offset(1);
            tp;
            break;
        }
    }
    *tp = '1' as i32 as libc::c_char;
    tp = np.offset(l as isize);
    let fresh0 = tp;
    tp = tp.offset(1);
    *fresh0 = '0' as i32 as libc::c_char;
    *tp = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn removelock(mut delta: *mut delta) -> libc::c_int {
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut rl: *const rcslock = 0 as *const rcslock;
    let mut num: *const libc::c_char = 0 as *const libc::c_char;
    num = (*delta).num;
    box_0.next = (*(*top).repository.r).locks;
    tp = lock_memq(&mut box_0, 0 as libc::c_int != 0, delta as *const libc::c_void);
    if tp.is_null() {
        if !(*top).behavior.strictly_locking
            && stat_mine_p(&mut (*top).repository.stat) as libc::c_int != 0
        {
            return 0 as libc::c_int;
        }
        generic_error(
            (*top).repository.filename,
            b"no lock set by %s for revision %s\0" as *const u8 as *const libc::c_char,
            getcaller(),
            num,
        );
        return -(1 as libc::c_int);
    }
    rl = (*(*tp).next).entry as *const rcslock;
    if !caller_login_p((*rl).login) {
        generic_error(
            (*top).repository.filename,
            b"revision %s locked by %s\0" as *const u8 as *const libc::c_char,
            num,
            (*rl).login,
        );
        return -(1 as libc::c_int);
    }
    lock_drop(&mut box_0, tp);
    return 1 as libc::c_int;
}
unsafe extern "C" fn addbranch(
    mut branchpoint: *mut delta,
    mut bud: *mut bud,
    mut removedlock: libc::c_int,
    mut tp_deltas: *mut *mut wlink,
) -> libc::c_int {
    let mut num: *mut cbuf = &mut (*bud).num;
    let mut btrail: *mut *mut wlink = 0 as *mut *mut wlink;
    let mut d: *mut delta = 0 as *mut delta;
    let mut result: libc::c_int = 0;
    let mut field: libc::c_int = 0;
    let mut numlength: libc::c_int = 0;
    numlength = countnumflds((*num).string);
    if ((*branchpoint).branches).is_null() {
        (*branchpoint).branches = &mut (*bud).br;
        if numlength == 0 as libc::c_int {
            accf(
                plexus,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*branchpoint).num,
            );
            (*num).string = finish_string(plexus, &mut (*num).size);
            accumulate_nbytes(plexus, (*num).string, (*num).size);
            accf(
                plexus,
                b"%s\0" as *const u8 as *const libc::c_char,
                b".1.1\0" as *const u8 as *const libc::c_char,
            );
            (*num).string = finish_string(plexus, &mut (*num).size);
        } else if numlength & 1 as libc::c_int != 0 {
            accumulate_nbytes(plexus, (*num).string, (*num).size);
            accf(
                plexus,
                b"%s\0" as *const u8 as *const libc::c_char,
                b".1\0" as *const u8 as *const libc::c_char,
            );
            (*num).string = finish_string(plexus, &mut (*num).size);
        }
        (*bud).br.next = 0 as *mut wlink;
    } else if numlength == 0 as libc::c_int {
        let mut bhead: *mut wlink = (*branchpoint).branches;
        while !((*bhead).next).is_null() {
            bhead = (*bhead).next;
        }
        (*bhead).next = &mut (*bud).br;
        d = (*bhead).entry as *mut delta;
        incnum((take(0 as libc::c_int as size_t, (*d).num)).string, num);
        accumulate_nbytes(plexus, (*num).string, (*num).size);
        accf(
            plexus,
            b"%s\0" as *const u8 as *const libc::c_char,
            b".1\0" as *const u8 as *const libc::c_char,
        );
        (*num).string = finish_string(plexus, &mut (*num).size);
        (*bud).br.next = 0 as *mut wlink;
    } else {
        field = numlength - (numlength & 1 as libc::c_int == 0) as libc::c_int;
        btrail = &mut (*branchpoint).branches;
        loop {
            d = (**btrail).entry as *mut delta;
            result = cmpnumfld((*num).string, (*d).num, field);
            if !((0 as libc::c_int) < result) {
                break;
            }
            btrail = &mut (**btrail).next;
            if !(*btrail).is_null() {
                continue;
            }
            result = -(1 as libc::c_int);
            break;
        }
        if result < 0 as libc::c_int {
            (*bud).br.next = *btrail;
            *btrail = &mut (*bud).br;
            if numlength & 1 as libc::c_int != 0 {
                accumulate_nbytes(plexus, (*num).string, (*num).size);
                accf(
                    plexus,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b".1\0" as *const u8 as *const libc::c_char,
                );
                (*num).string = finish_string(plexus, &mut (*num).size);
            }
        } else {
            (*bud)
                .target = gr_revno(
                (take(0 as libc::c_int as size_t, (*num).string)).string,
                tp_deltas,
            );
            if ((*bud).target).is_null() {
                return -(1 as libc::c_int);
            }
            if !((0 as libc::c_int) < cmpnum((*num).string, (*(*bud).target).num)) {
                generic_error(
                    (*top).repository.filename,
                    b"revision %s too low; must be higher than %s\0" as *const u8
                        as *const libc::c_char,
                    (*num).string,
                    (*(*bud).target).num,
                );
                return -(1 as libc::c_int);
            }
            if removedlock == 0
                && {
                    removedlock = removelock((*bud).target);
                    0 as libc::c_int <= removedlock
                }
            {
                if numlength & 1 as libc::c_int != 0 {
                    incnum((*(*bud).target).num, num);
                }
                (*(*bud).target).ilk = &mut (*bud).d;
                (*bud).d.ilk = 0 as *mut delta;
            }
            return removedlock;
        }
    }
    (*bud).br.entry = &mut (*bud).d as *mut delta as *mut libc::c_void;
    (*bud).d.ilk = 0 as *mut delta;
    if !((*branchpoint).lockedby).is_null() {
        if caller_login_p((*branchpoint).lockedby) {
            return removelock(branchpoint);
        }
    }
    return removedlock;
}
unsafe extern "C" fn prune(mut wrong: *mut delta, mut bp: *mut delta) {
    let mut box_0: wlink = wlink {
        entry: 0 as *mut libc::c_void,
        next: 0 as *mut wlink,
    };
    let mut tp: *mut wlink = 0 as *mut wlink;
    let mut d: *mut delta = 0 as *mut delta;
    let mut same: libc::c_int = countnumflds((*wrong).num) - 2 as libc::c_int;
    (*wrong).selector = 0 as libc::c_int != 0;
    if 0 as libc::c_int >= same {
        return;
    }
    if wrong == (*bp).ilk {
        (*bp).ilk = 0 as *mut delta;
        return;
    }
    box_0.next = (*bp).branches;
    tp = &mut box_0;
    while !((*tp).next).is_null() {
        d = (*(*tp).next).entry as *mut delta;
        if wrong == d {
            (*tp).next = (*(*tp).next).next;
            (*bp).branches = box_0.next;
            return;
        }
        tp = (*tp).next;
    }
    tp = (*bp).branches;
    while !tp.is_null() {
        d = (*tp).entry as *mut delta;
        if 0 as libc::c_int == compartial((*wrong).num, (*d).num, same) {
            while (*d).ilk != wrong {
                d = (*d).ilk;
            }
            (*d).ilk = 0 as *mut delta;
            return;
        }
        tp = (*tp).next;
    }
    abort();
}
unsafe extern "C" fn addelta(
    mut tp_deltas: *mut *mut wlink,
    mut bud: *mut bud,
    mut rcsinitflag: bool,
) -> libc::c_int {
    let mut tp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut removedlock: libc::c_int = 0;
    let mut newdnumlength: libc::c_int = 0;
    let mut tip: *mut delta = (*top).repository.tip;
    let mut defbr: *const libc::c_char = (*(*top).repository.r).branch;
    newdnumlength = countnumflds((*bud).num.string);
    if rcsinitflag {
        if newdnumlength == 0 as libc::c_int && !defbr.is_null() {
            accf(plexus, b"%s\0" as *const u8 as *const libc::c_char, defbr);
            (*bud).num.string = finish_string(plexus, &mut (*bud).num.size);
            newdnumlength = countnumflds(defbr);
        }
        if newdnumlength == 0 as libc::c_int {
            accf(
                plexus,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"1.1\0" as *const u8 as *const libc::c_char,
            );
            (*bud).num.string = finish_string(plexus, &mut (*bud).num.size);
        } else if newdnumlength == 1 as libc::c_int {
            accumulate_nbytes(plexus, (*bud).num.string, (*bud).num.size);
            accf(
                plexus,
                b"%s\0" as *const u8 as *const libc::c_char,
                b".1\0" as *const u8 as *const libc::c_char,
            );
            (*bud).num.string = finish_string(plexus, &mut (*bud).num.size);
        } else if newdnumlength > 2 as libc::c_int {
            generic_error(
                (*top).repository.filename,
                b"Branch point doesn't exist for revision %s.\0" as *const u8
                    as *const libc::c_char,
                (*bud).num.string,
            );
            return -(1 as libc::c_int);
        }
        (*top).repository.tip = &mut (*bud).d;
        tip = (*top).repository.tip;
        (*bud).d.ilk = 0 as *mut delta;
        return 0 as libc::c_int;
    }
    if newdnumlength == 0 as libc::c_int {
        match findlock(1 as libc::c_int != 0, &mut (*bud).target) {
            1 => {
                if (gr_revno((*(*bud).target).num, tp_deltas)).is_null() {
                    return -(1 as libc::c_int);
                }
                if (*bud).target == tip {
                    (*bud).d.ilk = tip;
                    (*top).repository.tip = &mut (*bud).d;
                    tip = (*top).repository.tip;
                } else if ((*(*bud).target).ilk).is_null()
                    && countnumflds((*(*bud).target).num) > 2 as libc::c_int
                {
                    (*(*bud).target).ilk = &mut (*bud).d;
                    (*bud).d.ilk = 0 as *mut delta;
                } else {
                    accf(
                        plexus,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    (*bud).num.string = finish_string(plexus, &mut (*bud).num.size);
                    return addbranch((*bud).target, bud, 1 as libc::c_int, tp_deltas);
                }
                incnum((*(*bud).target).num, &mut (*bud).num);
                return 1 as libc::c_int;
            }
            0 => {
                if (*top).behavior.strictly_locking as libc::c_int != 0
                    || !stat_mine_p(&mut (*top).repository.stat)
                {
                    generic_error(
                        (*top).repository.filename,
                        b"no lock set by %s\0" as *const u8 as *const libc::c_char,
                        getcaller(),
                    );
                    return -(1 as libc::c_int);
                }
                if !defbr.is_null() {
                    accf(plexus, b"%s\0" as *const u8 as *const libc::c_char, defbr);
                    (*bud).num.string = finish_string(plexus, &mut (*bud).num.size);
                } else {
                    incnum((*tip).num, &mut (*bud).num);
                }
                newdnumlength = countnumflds((*bud).num.string);
            }
            _ => return -(1 as libc::c_int),
        }
    }
    if newdnumlength <= 2 as libc::c_int {
        if newdnumlength == 1 as libc::c_int {
            if 0 as libc::c_int
                == cmpnumfld((*bud).num.string, (*tip).num, 1 as libc::c_int)
            {
                incnum((*tip).num, &mut (*bud).num);
            } else {
                accumulate_nbytes(plexus, (*bud).num.string, (*bud).num.size);
                accf(
                    plexus,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b".1\0" as *const u8 as *const libc::c_char,
                );
                (*bud).num.string = finish_string(plexus, &mut (*bud).num.size);
            }
        }
        if !((0 as libc::c_int) < cmpnum((*bud).num.string, (*tip).num)) {
            generic_error(
                (*top).repository.filename,
                b"revision %s too low; must be higher than %s\0" as *const u8
                    as *const libc::c_char,
                (*bud).num.string,
                (*tip).num,
            );
            return -(1 as libc::c_int);
        }
        (*bud).target = tip;
        removedlock = removelock(tip);
        if 0 as libc::c_int <= removedlock {
            if (gr_revno((*tip).num, tp_deltas)).is_null() {
                return -(1 as libc::c_int);
            }
            (*bud).d.ilk = tip;
            (*top).repository.tip = &mut (*bud).d;
            tip = (*top).repository.tip;
        }
        return removedlock;
    } else {
        let mut old: cbuf = (*bud).num;
        tp = old.string;
        i = newdnumlength - (newdnumlength & 1 as libc::c_int == 0) as libc::c_int;
        loop {
            i -= 1;
            if !(i != 0) {
                break;
            }
            loop {
                let fresh1 = tp;
                tp = tp.offset(1);
                if !(*fresh1 as libc::c_int != '.' as i32) {
                    break;
                }
            }
        }
        accumulate_range(plexus, old.string, tp.offset(-(1 as libc::c_int as isize)));
        old.string = finish_string(plexus, &mut old.size);
        (*bud).target = gr_revno(old.string, tp_deltas);
        if ((*bud).target).is_null() {
            return -(1 as libc::c_int);
        }
        if !(0 as libc::c_int == cmpnum((*(*bud).target).num, old.string)) {
            generic_error(
                (*top).repository.filename,
                b"can't find branch point %s\0" as *const u8 as *const libc::c_char,
                old.string,
            );
            return -(1 as libc::c_int);
        }
        return addbranch((*bud).target, bud, 0 as libc::c_int, tp_deltas);
    };
}
unsafe extern "C" fn addsyms(mut num: *const libc::c_char, mut ls: *mut link) -> bool {
    let mut ud: *const u_symdef = 0 as *const u_symdef;
    while !ls.is_null() {
        ud = (*ls).entry as *const u_symdef;
        if addsymbol(num, (*ud).u.meaningful, (*ud).override_0) < 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        ls = (*ls).next;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn getcurdate(mut bud: *mut bud) -> *const libc::c_char {
    if (*bud).getcurdate_buffer[0 as libc::c_int as usize] == 0 {
        time2date((*top).behavior.now.tv_sec, ((*bud).getcurdate_buffer).as_mut_ptr());
    }
    return ((*bud).getcurdate_buffer).as_mut_ptr();
}
unsafe extern "C" fn fixwork(
    mut newworkmode: mode_t,
    mtime: timespec,
    mut work: *mut work,
) -> libc::c_int {
    let mut mani_filename: *const libc::c_char = (*top).manifestation.filename;
    let mut st: *mut stat = &mut (*work).st;
    return if (1 as libc::c_int as libc::c_ulong) < (*st).st_nlink
        || newworkmode & 0o200 as libc::c_int as libc::c_uint != 0 && !stat_mine_p(st)
        || 0 as libc::c_int > setmtime(mani_filename, mtime)
    {
        -(1 as libc::c_int)
    } else if (*st).st_mode == newworkmode {
        0 as libc::c_int
    } else if !(0 as libc::c_int > change_mode((*(*work).fro).fd, newworkmode)) {
        0 as libc::c_int
    } else {
        chmod(mani_filename, newworkmode)
    };
}
unsafe extern "C" fn xpandfile(
    mut work: *mut work,
    mut delta: *const delta,
    mut exname: *mut *const libc::c_char,
    mut dolog: bool,
) -> libc::c_int {
    let mut targetname: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    targetname = makedirtemp(1 as libc::c_int != 0);
    (*work).ex = fopen_safer(targetname, b"w\0" as *const u8 as *const libc::c_char);
    if ((*work).ex).is_null() {
        syserror(*__errno_location(), targetname);
        generic_error(
            (*top).manifestation.filename,
            b"can't build working file\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    r = 0 as libc::c_int;
    if kwsub_o as libc::c_int <= (*top).behavior.kws {
        fro_spew((*work).fro, (*work).ex);
    } else {
        let mut ctx: expctx = {
            let mut init = expctx {
                to: (*work).ex,
                rewr: 0 as *mut FILE,
                from: (*work).fro,
                delta: delta,
                delimstuffed: 0 as libc::c_int != 0,
                dolog: dolog,
                lparts: 0 as *mut divvy,
            };
            init
        };
        loop {
            e = expandline(&mut ctx);
            if e < 0 as libc::c_int {
                break;
            }
            r |= e;
            if e <= 1 as libc::c_int {
                break;
            }
        }
        if !(ctx.lparts).is_null() {
            close_space(ctx.lparts);
        }
    }
    *exname = targetname;
    return r & 1 as libc::c_int;
}
unsafe extern "C" fn getlogmsg(mut reason: *mut reason, mut bud: *mut bud) -> cbuf {
    let mut num: *const libc::c_char = 0 as *const libc::c_char;
    if (*reason).upfront.size != 0 {
        return (*reason).upfront;
    }
    if (*bud).keep {
        let mut datebuf: [libc::c_char; 31] = [0; 31];
        date2str(getcurdate(bud), datebuf.as_mut_ptr());
        accf(
            plexus,
            b"%s%s at %s\0" as *const u8 as *const libc::c_char,
            (tiny_ciklog.bytes).as_ptr() as *const libc::c_char,
            getcaller(),
            datebuf.as_mut_ptr(),
        );
        (*reason).delayed.string = finish_string(plexus, &mut (*reason).delayed.size);
        return (*reason).delayed;
    }
    if ((*bud).target).is_null() && (*bud).num.size != 0
        && {
            num = (*bud).num.string;
            !num.is_null()
        }
        && (0 as libc::c_int == cmpnum(num, b"1.1\0" as *const u8 as *const libc::c_char)
            || 0 as libc::c_int
                == cmpnum(num, b"1.0\0" as *const u8 as *const libc::c_char))
    {
        let initiallog: cbuf = {
            let mut init = cbuf {
                string: b"Initial revision\0" as *const u8 as *const libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        };
        return initiallog;
    }
    if (*reason).delayed.size != 0 {
        if yesorno(
            1 as libc::c_int != 0,
            b"reuse log message of previous file\0" as *const u8 as *const libc::c_char,
        ) {
            return (*reason).delayed;
        }
    }
    (*reason)
        .delayed = getsstdin(
        b"m\0" as *const u8 as *const libc::c_char,
        b"log message\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if (*reason).delayed.size == 0 {
        set_empty_log_message(&mut (*reason).delayed);
    }
    return (*reason).delayed;
}
unsafe extern "C" fn first_meaningful_symbolic_name(
    mut ls: *mut link,
) -> *const libc::c_char {
    let mut ud: *const u_symdef = 0 as *const u_symdef;
    while !ls.is_null() && !((*ls).next).is_null() {
        ls = (*ls).next;
    }
    ud = (*ls).entry as *const u_symdef;
    return (*ud).u.meaningful;
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: ci_blurb.as_ptr(),
            help: ci_help.as_ptr(),
            tyag: (1 as libc::c_int) << 3 as libc::c_int
                | ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int),
        };
        init
    }
};
unsafe extern "C" fn ci_main(
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut exitstatus: libc::c_int = 0 as libc::c_int;
    let mut reason: reason = reason {
        upfront: cbuf {
            string: 0 as *const libc::c_char,
            size: 0,
        },
        delayed: cbuf {
            string: 0 as *const libc::c_char,
            size: 0,
        },
    };
    let mut altdate: [libc::c_char; 22] = [0; 22];
    let mut olddate: [libc::c_char; 22] = [0; 22];
    let mut newdatebuf: [libc::c_char; 31] = [0; 31];
    let mut targetdatebuf: [libc::c_char; 31] = [0; 31];
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut textfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut author: *const libc::c_char = 0 as *const libc::c_char;
    let mut krev: *const libc::c_char = 0 as *const libc::c_char;
    let mut rev: *const libc::c_char = 0 as *const libc::c_char;
    let mut state: *const libc::c_char = 0 as *const libc::c_char;
    let mut diffname: *const libc::c_char = 0 as *const libc::c_char;
    let mut expname: *const libc::c_char = 0 as *const libc::c_char;
    let mut newworkname: *const libc::c_char = 0 as *const libc::c_char;
    let mut work: work = {
        let mut init = work {
            st: stat {
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
            },
            fro: 0 as *mut fro,
            ex: 0 as *mut FILE,
        };
        init
    };
    let mut forceciflag: bool = 0 as libc::c_int != 0;
    let mut keepworkingfile: bool = 0 as libc::c_int != 0;
    let mut rcsinitflag: bool = 0 as libc::c_int != 0;
    let mut initflag: bool = false;
    let mut mustread: bool = false;
    let mut lockflag: bool = false;
    let mut lockthis: bool = false;
    let mut mtimeflag: bool = false;
    let mut removedlock: libc::c_int = 0;
    let mut Ttimeflag: bool = false;
    let mut r: libc::c_int = 0;
    let mut changedRCS: libc::c_int = 0;
    let mut changework: libc::c_int = 0;
    let mut dolog: bool = false;
    let mut newhead: bool = false;
    let mut usestatdate: bool = false;
    let mut newworkmode: mode_t = 0;
    let mut mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut wtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut bud: bud = bud {
        num: cbuf {
            string: 0 as *const libc::c_char,
            size: 0,
        },
        d: delta {
            num: 0 as *const libc::c_char,
            date: 0 as *const libc::c_char,
            author: 0 as *const libc::c_char,
            lockedby: 0 as *const libc::c_char,
            state: 0 as *const libc::c_char,
            log: 0 as *mut atat,
            text: 0 as *mut atat,
            name: 0 as *const libc::c_char,
            pretty_log: cbuf {
                string: 0 as *const libc::c_char,
                size: 0,
            },
            branches: 0 as *mut wlink,
            commitid: 0 as *const libc::c_char,
            ilk: 0 as *mut delta,
            selector: false,
            neck: 0,
        },
        br: wlink {
            entry: 0 as *mut libc::c_void,
            next: 0 as *mut wlink,
        },
        keep: false,
        target: 0 as *mut delta,
        getcurdate_buffer: [0; 22],
        work_mtime: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut workdelta: *mut delta = 0 as *mut delta;
    let mut symbolic_names: *mut link = 0 as *mut link;
    let mut deltas: *mut wlink = 0 as *mut wlink;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut bud as *mut bud as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bud>() as libc::c_ulong,
    );
    memset(
        &mut reason as *mut reason as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<reason>() as libc::c_ulong,
    );
    setrid();
    textfile = 0 as *mut libc::c_char;
    state = textfile;
    rev = state;
    author = rev;
    mustread = 0 as libc::c_int != 0;
    lockflag = mustread;
    initflag = lockflag;
    mtimeflag = 0 as libc::c_int != 0;
    Ttimeflag = 0 as libc::c_int != 0;
    altdate[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    usestatdate = 0 as libc::c_int != 0;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as libc::c_int) < argc
            && {
                let fresh2 = a;
                a = a.offset(1);
                *fresh2 as libc::c_int == '-' as i32
            })
        {
            break;
        }
        let fresh3 = a;
        a = a.offset(1);
        match *fresh3 as libc::c_int {
            114 => {
                if *a != 0 {
                    current_block = 5265603397418392414;
                } else {
                    lockflag = 0 as libc::c_int != 0;
                    keepworkingfile = lockflag;
                    continue;
                }
            }
            108 => {
                lockflag = 1 as libc::c_int != 0;
                keepworkingfile = lockflag;
                current_block = 5265603397418392414;
            }
            117 => {
                keepworkingfile = 1 as libc::c_int != 0;
                lockflag = 0 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            105 => {
                initflag = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            106 => {
                mustread = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            73 => {
                (*top).behavior.interactive = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            113 => {
                (*top).behavior.quiet = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            102 => {
                forceciflag = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            107 => {
                bud.keep = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            109 => {
                if reason.upfront.size != 0 {
                    redefined('m' as i32);
                }
                reason.upfront = cleanlogmsg(a, strlen(a));
                if reason.upfront.size == 0 {
                    set_empty_log_message(&mut reason.upfront);
                }
                continue;
            }
            110 | 78 => {
                let mut option: libc::c_char = *a.offset(-(1 as libc::c_int) as isize);
                let mut ud: *mut u_symdef = 0 as *mut u_symdef;
                if *a == 0 {
                    generic_error(
                        0 as *const libc::c_char,
                        b"missing symbolic name after -%c\0" as *const u8
                            as *const libc::c_char,
                        option as libc::c_int,
                    );
                    continue;
                } else {
                    checkssym(a);
                    ud = zlloc(
                        plexus,
                        (::std::mem::size_of::<u_symdef>() as libc::c_ulong)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
                    ) as *mut u_symdef;
                    (*ud).override_0 = 'N' as i32 == option as libc::c_int;
                    (*ud).u.meaningful = a;
                    symbolic_names = prepend(
                        ud as *const libc::c_void,
                        symbolic_names,
                        plexus,
                    );
                    continue;
                }
            }
            115 => {
                if *a != 0 {
                    if !state.is_null() {
                        redefined('s' as i32);
                    }
                    checksid(a);
                    state = a;
                } else {
                    generic_error(
                        0 as *const libc::c_char,
                        b"missing state for -s option\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                continue;
            }
            116 => {
                if *a != 0 {
                    if !textfile.is_null() {
                        redefined('t' as i32);
                    }
                    textfile = a;
                }
                continue;
            }
            100 => {
                if altdate[0 as libc::c_int as usize] as libc::c_int != 0
                    || usestatdate as libc::c_int != 0
                {
                    redefined('d' as i32);
                }
                altdate[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                usestatdate = *a == 0;
                if !usestatdate {
                    str2date(a, altdate.as_mut_ptr());
                }
                continue;
            }
            77 => {
                mtimeflag = 1 as libc::c_int != 0;
                current_block = 5265603397418392414;
            }
            119 => {
                if *a != 0 {
                    if !author.is_null() {
                        redefined('w' as i32);
                    }
                    checksid(a);
                    author = a;
                } else {
                    generic_error(
                        0 as *const libc::c_char,
                        b"missing author for -w option\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                continue;
            }
            120 => {
                (*top).behavior.pe = a;
                continue;
            }
            86 => {
                setRCSversion(*argv);
                continue;
            }
            122 => {
                zone_set(a);
                continue;
            }
            84 => {
                if *a == 0 {
                    Ttimeflag = 1 as libc::c_int != 0;
                    continue;
                } else {
                    current_block = 9229921293278419701;
                }
            }
            _ => {
                current_block = 9229921293278419701;
            }
        }
        match current_block {
            5265603397418392414 => {
                chk_set_rev(&mut rev, a);
            }
            _ => {
                bad_option(*argv);
            }
        }
    }
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut work);
    } else if argc < 1 as libc::c_int {
        generic_fatal(
            0 as *const libc::c_char,
            b"no input file\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut current_block_194: u64;
        while (0 as libc::c_int) < argc {
            let mut default_state: *const libc::c_char = b"Exp\0" as *const u8
                as *const libc::c_char;
            let mut mani_filename: *const libc::c_char = 0 as *const libc::c_char;
            let mut pv: *const libc::c_char = 0 as *const libc::c_char;
            let mut from: *mut fro = 0 as *mut fro;
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut fs_mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut frew: *mut FILE = 0 as *mut FILE;
            let mut tip: *mut delta = 0 as *mut delta;
            let mut kws: libc::c_int = 0;
            let mut newdesc: cbuf = {
                let mut init = cbuf {
                    string: 0 as *const libc::c_char,
                    size: 0 as libc::c_int as size_t,
                };
                init
            };
            bud.target = 0 as *mut delta;
            ffree();
            match pairnames(
                argc,
                argv,
                Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                mustread,
                0 as libc::c_int != 0,
            ) {
                -1 => {
                    if currently_setuid_p() {
                        generic_error(
                            (*top).manifestation.filename,
                            b"setuid initial checkin prohibited; use `rcs -i -a' first\0"
                                as *const u8 as *const libc::c_char,
                        );
                        current_block_194 = 10393716428851982524;
                    } else {
                        rcsinitflag = 1 as libc::c_int != 0;
                        current_block_194 = 796174441944384681;
                    }
                }
                0 => {
                    current_block_194 = 10393716428851982524;
                }
                1 => {
                    if initflag {
                        generic_error(
                            (*top).repository.filename,
                            b"already exists\0" as *const u8 as *const libc::c_char,
                        );
                        current_block_194 = 10393716428851982524;
                    } else {
                        tip = (*top).repository.tip;
                        rcsinitflag = tip.is_null();
                        current_block_194 = 796174441944384681;
                    }
                }
                _ => {
                    current_block_194 = 796174441944384681;
                }
            }
            match current_block_194 {
                796174441944384681 => {
                    mani_filename = (*top).manifestation.filename;
                    from = (*top).flow.from;
                    repo_stat = &mut (*top).repository.stat;
                    kws = (*top).behavior.kws;
                    diagnose(
                        b"%s  <--  %s\0" as *const u8 as *const libc::c_char,
                        (*top).repository.filename,
                        mani_filename,
                    );
                    work
                        .fro = fro_open(
                        mani_filename,
                        b"r\0" as *const u8 as *const libc::c_char,
                        &mut work.st,
                    );
                    if (work.fro).is_null() {
                        syserror(*__errno_location(), mani_filename);
                    } else {
                        if !from.is_null() {
                            if (*top).repository.stat.st_ino == work.st.st_ino
                                && (*top).repository.stat.st_dev == work.st.st_dev
                            {
                                generic_error(
                                    (*top).repository.filename,
                                    b"RCS file is the same as working file %s.\0" as *const u8
                                        as *const libc::c_char,
                                    mani_filename,
                                );
                                current_block_194 = 10393716428851982524;
                            } else if !checkaccesslist() {
                                current_block_194 = 10393716428851982524;
                            } else {
                                current_block_194 = 16590946904645350046;
                            }
                        } else {
                            current_block_194 = 16590946904645350046;
                        }
                        match current_block_194 {
                            10393716428851982524 => {}
                            _ => {
                                krev = rev;
                                if bud.keep {
                                    if !getoldkeys(work.fro) {
                                        current_block_194 = 10393716428851982524;
                                    } else if rev.is_null()
                                        && {
                                            krev = (*top).manifestation.prev.rev;
                                            krev.is_null()
                                        }
                                    {
                                        generic_error(
                                            (*top).manifestation.filename,
                                            b"can't find a %s\0" as *const u8 as *const libc::c_char,
                                            ks_revno.as_ptr(),
                                        );
                                        current_block_194 = 10393716428851982524;
                                    } else {
                                        if ((*top).manifestation.prev.date).is_null()
                                            && *altdate.as_mut_ptr() as libc::c_int == '\0' as i32
                                            && usestatdate as libc::c_int == 0 as libc::c_int
                                        {
                                            generic_warn(
                                                (*top).manifestation.filename,
                                                b"can't find a date\0" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        if ((*top).manifestation.prev.author).is_null()
                                            && author.is_null()
                                        {
                                            generic_warn(
                                                (*top).manifestation.filename,
                                                b"can't find an author\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                        }
                                        if ((*top).manifestation.prev.state).is_null()
                                            && state.is_null()
                                        {
                                            generic_warn(
                                                (*top).manifestation.filename,
                                                b"can't find a state\0" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        current_block_194 = 14652688882591975137;
                                    }
                                } else {
                                    current_block_194 = 14652688882591975137;
                                }
                                match current_block_194 {
                                    10393716428851982524 => {}
                                    _ => {
                                        if fully_numeric(&mut bud.num, krev, work.fro) {
                                            removedlock = addelta(&mut deltas, &mut bud, rcsinitflag);
                                            if !(0 as libc::c_int > removedlock) {
                                                tip = (*top).repository.tip;
                                                bud.d.num = bud.num.string;
                                                bud.d.branches = 0 as *mut wlink;
                                                bud.d.lockedby = 0 as *const libc::c_char;
                                                bud.d.selector = 1 as libc::c_int != 0;
                                                bud.d.name = 0 as *const libc::c_char;
                                                bud
                                                    .d
                                                    .author = if !author.is_null() {
                                                    author
                                                } else if bud.keep as libc::c_int != 0
                                                    && {
                                                        pv = (*top).manifestation.prev.author;
                                                        !pv.is_null()
                                                    }
                                                {
                                                    pv
                                                } else {
                                                    getcaller()
                                                };
                                                bud
                                                    .d
                                                    .state = if !state.is_null() {
                                                    state
                                                } else if bud.keep as libc::c_int != 0
                                                    && {
                                                        pv = (*top).manifestation.prev.state;
                                                        !pv.is_null()
                                                    }
                                                {
                                                    pv
                                                } else {
                                                    default_state
                                                };
                                                bud.work_mtime = get_stat_mtime(&mut work.st);
                                                if usestatdate {
                                                    time2date(work.st.st_mtim.tv_sec, altdate.as_mut_ptr());
                                                }
                                                if *altdate.as_mut_ptr() as libc::c_int != '\0' as i32 {
                                                    bud.d.date = altdate.as_mut_ptr();
                                                } else if bud.keep as libc::c_int != 0
                                                    && {
                                                        pv = (*top).manifestation.prev.date;
                                                        !pv.is_null()
                                                    }
                                                {
                                                    str2date(pv, olddate.as_mut_ptr());
                                                    bud.d.date = olddate.as_mut_ptr();
                                                } else {
                                                    bud.d.date = getcurdate(&mut bud);
                                                }
                                                if !(bud.target).is_null()
                                                    && 0 as libc::c_int
                                                        > cmpdate(bud.d.date, (*bud.target).date)
                                                {
                                                    generic_error(
                                                        (*top).repository.filename,
                                                        b"Date %s precedes %s in revision %s.\0" as *const u8
                                                            as *const libc::c_char,
                                                        date2str(bud.d.date, newdatebuf.as_mut_ptr()),
                                                        date2str((*bud.target).date, targetdatebuf.as_mut_ptr()),
                                                        (*bud.target).num,
                                                    );
                                                } else if !(lockflag as libc::c_int != 0
                                                    && addlock_maybe(
                                                        &mut bud.d,
                                                        0 as libc::c_int != 0,
                                                        1 as libc::c_int != 0,
                                                    ) < 0 as libc::c_int)
                                                {
                                                    if bud.keep as libc::c_int != 0
                                                        && {
                                                            pv = (*top).manifestation.prev.name;
                                                            !pv.is_null()
                                                        }
                                                    {
                                                        if addsymbol(bud.d.num, pv, 0 as libc::c_int != 0)
                                                            < 0 as libc::c_int
                                                        {
                                                            current_block_194 = 10393716428851982524;
                                                        } else {
                                                            current_block_194 = 2473505634946569239;
                                                        }
                                                    } else {
                                                        current_block_194 = 2473505634946569239;
                                                    }
                                                    match current_block_194 {
                                                        10393716428851982524 => {}
                                                        _ => {
                                                            if addsyms(bud.d.num, symbolic_names) {
                                                                putadmin();
                                                                frew = (*top).flow.rewr;
                                                                puttree(tip, frew);
                                                                putdesc(&mut newdesc, 0 as libc::c_int != 0, textfile);
                                                                changework = (kws
                                                                    < (if 0 as libc::c_int != 0 {
                                                                        kwsub_b as libc::c_int
                                                                    } else {
                                                                        kwsub_o as libc::c_int
                                                                    })) as libc::c_int;
                                                                dolog = 1 as libc::c_int != 0;
                                                                lockthis = lockflag;
                                                                workdelta = &mut bud.d;
                                                                if rcsinitflag {
                                                                    diagnose(
                                                                        b"initial revision: %s\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        bud.d.num,
                                                                    );
                                                                    bud.d.pretty_log = getlogmsg(&mut reason, &mut bud);
                                                                    putdftext(
                                                                        &mut bud.d,
                                                                        work.fro,
                                                                        frew,
                                                                        0 as libc::c_int != 0,
                                                                    );
                                                                    (*repo_stat).st_mode = work.st.st_mode;
                                                                    (*repo_stat).st_nlink = 0 as libc::c_int as __nlink_t;
                                                                    changedRCS = 1 as libc::c_int;
                                                                    if !from.is_null() {
                                                                        (*from).verbatim = (*from).end;
                                                                    }
                                                                    current_block_194 = 11577926782275222206;
                                                                } else {
                                                                    diffname = maketemp(0 as libc::c_int);
                                                                    newhead = tip == &mut bud.d as *mut delta;
                                                                    if !newhead {
                                                                        (*top).flow.to = frew;
                                                                    }
                                                                    expname = buildrevision(
                                                                        deltas,
                                                                        bud.target,
                                                                        0 as *mut FILE,
                                                                        0 as libc::c_int != 0,
                                                                    );
                                                                    if !forceciflag
                                                                        && strcmp(bud.d.state, (*bud.target).state) == 0
                                                                        && {
                                                                            changework = rcsfcmp(
                                                                                work.fro,
                                                                                &mut work.st,
                                                                                expname,
                                                                                bud.target,
                                                                            );
                                                                            changework <= 0 as libc::c_int
                                                                        }
                                                                    {
                                                                        diagnose(
                                                                            b"file is unchanged; reverting to previous revision %s\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            (*bud.target).num,
                                                                        );
                                                                        if removedlock < lockflag as libc::c_int {
                                                                            diagnose(
                                                                                b"previous revision was not locked; ignoring -l option\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                            );
                                                                            lockthis = 0 as libc::c_int != 0;
                                                                        }
                                                                        dolog = 0 as libc::c_int != 0;
                                                                        changedRCS = ((lockflag as libc::c_int) < removedlock
                                                                            || !symbolic_names.is_null()) as libc::c_int;
                                                                        if changedRCS == 0 {
                                                                            workdelta = bud.target;
                                                                            (*from)
                                                                                .verbatim = *((*(*bud.target).text).holes)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    ((*(*bud.target).text).count)
                                                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                                                                ) + 2 as libc::c_int as libc::c_long;
                                                                            current_block_194 = 11577926782275222206;
                                                                        } else {
                                                                            let mut hwm: off_t = ftello(frew);
                                                                            let mut bad_truncate: bool = false;
                                                                            rewind(frew);
                                                                            bad_truncate = 0 as libc::c_int
                                                                                > ftruncate(fileno(frew), 0 as libc::c_int as off_t);
                                                                            grok_resynch((*top).repository.r);
                                                                            prune(&mut bud.d, bud.target);
                                                                            workdelta = gr_revno((*bud.target).num, &mut deltas);
                                                                            if workdelta.is_null() {
                                                                                current_block_194 = 10393716428851982524;
                                                                            } else {
                                                                                (*workdelta).pretty_log = (*bud.target).pretty_log;
                                                                                if bud.d.state != default_state {
                                                                                    (*workdelta).state = bud.d.state;
                                                                                }
                                                                                if (lockthis as libc::c_int) < removedlock
                                                                                    && removelock(workdelta) < 0 as libc::c_int
                                                                                {
                                                                                    current_block_194 = 10393716428851982524;
                                                                                } else if !addsyms((*workdelta).num, symbolic_names) {
                                                                                    current_block_194 = 10393716428851982524;
                                                                                } else if 0 as libc::c_int
                                                                                    > dorewrite(1 as libc::c_int != 0, 1 as libc::c_int)
                                                                                {
                                                                                    current_block_194 = 10393716428851982524;
                                                                                } else {
                                                                                    (*from).verbatim = (*(*top).repository.r).neck;
                                                                                    fro_spew(from, frew);
                                                                                    if bad_truncate {
                                                                                        while ftello(frew) < hwm {
                                                                                            newline(frew);
                                                                                        }
                                                                                    }
                                                                                    current_block_194 = 11577926782275222206;
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let mut wfd: libc::c_int = (*work.fro).fd;
                                                                        let mut checkworkstat: stat = stat {
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
                                                                        let mut diffv: [*const libc::c_char; 6] = [0
                                                                            as *const libc::c_char; 6];
                                                                        let mut diffp: *mut *const libc::c_char = 0
                                                                            as *mut *const libc::c_char;
                                                                        diagnose(
                                                                            b"new revision: %s; previous revision: %s\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            bud.d.num,
                                                                            (*bud.target).num,
                                                                        );
                                                                        (*from)
                                                                            .verbatim = *((*(*bud.target).text).holes)
                                                                            .as_mut_ptr()
                                                                            .offset(
                                                                                ((*(*bud.target).text).count)
                                                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                                                            ) + 2 as libc::c_int as libc::c_long;
                                                                        bud.d.pretty_log = getlogmsg(&mut reason, &mut bud);
                                                                        if 0 as libc::c_int as libc::c_long
                                                                            > lseek(wfd, 0 as libc::c_int as __off_t, 0 as libc::c_int)
                                                                        {
                                                                            Ierror();
                                                                        }
                                                                        diffp = diffv.as_mut_ptr();
                                                                        diffp = diffp.offset(1);
                                                                        *diffp = prog_diff.as_ptr();
                                                                        diffp = diffp.offset(1);
                                                                        *diffp = diff_flags.as_ptr();
                                                                        if 0 as libc::c_int != 0 && kws == kwsub_b as libc::c_int {
                                                                            diffp = diffp.offset(1);
                                                                            *diffp = b"--binary\0" as *const u8 as *const libc::c_char;
                                                                        }
                                                                        diffp = diffp.offset(1);
                                                                        *diffp = if newhead as libc::c_int != 0 {
                                                                            b"-\0" as *const u8 as *const libc::c_char
                                                                        } else {
                                                                            expname
                                                                        };
                                                                        diffp = diffp.offset(1);
                                                                        *diffp = if newhead as libc::c_int != 0 {
                                                                            expname
                                                                        } else {
                                                                            b"-\0" as *const u8 as *const libc::c_char
                                                                        };
                                                                        diffp = diffp.offset(1);
                                                                        *diffp = 0 as *const libc::c_char;
                                                                        if 2 as libc::c_int
                                                                            == runv(wfd, diffname, diffv.as_mut_ptr())
                                                                        {
                                                                            generic_fatal(
                                                                                (*top).repository.filename,
                                                                                b"diff failed\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                        }
                                                                        fro_move(work.fro, 0 as libc::c_int as off_t);
                                                                        if newhead {
                                                                            fro_move(work.fro, 0 as libc::c_int as off_t);
                                                                            putdftext(
                                                                                &mut bud.d,
                                                                                work.fro,
                                                                                frew,
                                                                                0 as libc::c_int != 0,
                                                                            );
                                                                            if !putdtext(
                                                                                bud.target,
                                                                                diffname,
                                                                                frew,
                                                                                1 as libc::c_int != 0,
                                                                            ) {
                                                                                current_block_194 = 10393716428851982524;
                                                                            } else {
                                                                                current_block_194 = 3705522161509601321;
                                                                            }
                                                                        } else if !putdtext(
                                                                            &mut bud.d,
                                                                            diffname,
                                                                            frew,
                                                                            1 as libc::c_int != 0,
                                                                        ) {
                                                                            current_block_194 = 10393716428851982524;
                                                                        } else {
                                                                            current_block_194 = 3705522161509601321;
                                                                        }
                                                                        match current_block_194 {
                                                                            10393716428851982524 => {}
                                                                            _ => {
                                                                                if 0 as libc::c_int > fstat(wfd, &mut checkworkstat)
                                                                                    || 0 as libc::c_int
                                                                                        != timespec_cmp(
                                                                                            get_stat_mtime(&mut checkworkstat),
                                                                                            bud.work_mtime,
                                                                                        ) || work.st.st_size != checkworkstat.st_size
                                                                                {
                                                                                    generic_error(
                                                                                        (*top).manifestation.filename,
                                                                                        b"file changed during checkin\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    );
                                                                                    current_block_194 = 10393716428851982524;
                                                                                } else {
                                                                                    changedRCS = 1 as libc::c_int;
                                                                                    current_block_194 = 11577926782275222206;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                match current_block_194 {
                                                                    10393716428851982524 => {}
                                                                    _ => {
                                                                        wtime = if mtimeflag as libc::c_int
                                                                            | Ttimeflag as libc::c_int != 0
                                                                        {
                                                                            if usestatdate as libc::c_int != 0 {
                                                                                bud.work_mtime
                                                                            } else {
                                                                                make_timespec(
                                                                                    date2time((*workdelta).date),
                                                                                    0 as libc::c_int as libc::c_long,
                                                                                )
                                                                            }
                                                                        } else {
                                                                            unspecified_timespec()
                                                                        };
                                                                        if Ttimeflag {
                                                                            fs_mtime = file_mtime(!from.is_null(), repo_stat);
                                                                        }
                                                                        if !(0 as libc::c_int
                                                                            > donerewrite(
                                                                                changedRCS,
                                                                                if !Ttimeflag {
                                                                                    unspecified_timespec()
                                                                                } else {
                                                                                    if 0 as libc::c_int > timespec_cmp(wtime, fs_mtime) {
                                                                                        fs_mtime
                                                                                    } else {
                                                                                        wtime
                                                                                    }
                                                                                },
                                                                            ))
                                                                        {
                                                                            if !keepworkingfile {
                                                                                fro_zclose(&mut work.fro);
                                                                                r = un_link(mani_filename);
                                                                                current_block_194 = 5577234879133443219;
                                                                            } else {
                                                                                newworkmode = (*repo_stat).st_mode
                                                                                    & !(0o200 as libc::c_int
                                                                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                                                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                                                                            >> 3 as libc::c_int) as mode_t
                                                                                    | (if !(kws == kwsub_v as libc::c_int
                                                                                        || (lockthis as libc::c_int)
                                                                                            < (*top).behavior.strictly_locking as libc::c_int)
                                                                                    {
                                                                                        0o200 as libc::c_int
                                                                                    } else {
                                                                                        0 as libc::c_int
                                                                                    }) as libc::c_uint;
                                                                                mtime = if mtimeflag as libc::c_int != 0 {
                                                                                    wtime
                                                                                } else {
                                                                                    unspecified_timespec()
                                                                                };
                                                                                if changework != 0
                                                                                    || {
                                                                                        r = fixwork(newworkmode, mtime, &mut work);
                                                                                        0 as libc::c_int > r
                                                                                    }
                                                                                {
                                                                                    fro_move(work.fro, 0 as libc::c_int as off_t);
                                                                                    (*top).behavior.inclusive_of_Locker_in_Id_val = lockthis;
                                                                                    (*workdelta)
                                                                                        .name = namedrev(
                                                                                        if !symbolic_names.is_null() {
                                                                                            first_meaningful_symbolic_name(symbolic_names)
                                                                                        } else if bud.keep as libc::c_int != 0
                                                                                            && {
                                                                                                pv = (*top).manifestation.prev.name;
                                                                                                !pv.is_null()
                                                                                            }
                                                                                        {
                                                                                            pv
                                                                                        } else {
                                                                                            rev
                                                                                        },
                                                                                        workdelta,
                                                                                    );
                                                                                    match xpandfile(
                                                                                        &mut work,
                                                                                        workdelta,
                                                                                        &mut newworkname,
                                                                                        dolog,
                                                                                    ) {
                                                                                        0 => {
                                                                                            current_block_194 = 13624143383088606119;
                                                                                            match current_block_194 {
                                                                                                13624143383088606119 => {
                                                                                                    if changework != 0 {
                                                                                                        r = fixwork(newworkmode, mtime, &mut work);
                                                                                                        if r == 0 as libc::c_int {
                                                                                                            current_block_194 = 5577234879133443219;
                                                                                                        } else {
                                                                                                            current_block_194 = 17011164281425506503;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block_194 = 17011164281425506503;
                                                                                                    }
                                                                                                }
                                                                                                _ => {}
                                                                                            }
                                                                                            match current_block_194 {
                                                                                                5577234879133443219 => {}
                                                                                                _ => {
                                                                                                    fro_zclose(&mut work.fro);
                                                                                                    aflush(work.ex);
                                                                                                    isr_do((*top).behavior.isr, ISR_IGNOREINTS);
                                                                                                    r = chnamemod(
                                                                                                        &mut work.ex,
                                                                                                        newworkname,
                                                                                                        mani_filename,
                                                                                                        1 as libc::c_int,
                                                                                                        newworkmode,
                                                                                                        mtime,
                                                                                                    );
                                                                                                    keepdirtemp(newworkname);
                                                                                                    isr_do((*top).behavior.isr, ISR_RESTOREINTS);
                                                                                                    current_block_194 = 5577234879133443219;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        1 => {
                                                                                            current_block_194 = 17011164281425506503;
                                                                                            match current_block_194 {
                                                                                                13624143383088606119 => {
                                                                                                    if changework != 0 {
                                                                                                        r = fixwork(newworkmode, mtime, &mut work);
                                                                                                        if r == 0 as libc::c_int {
                                                                                                            current_block_194 = 5577234879133443219;
                                                                                                        } else {
                                                                                                            current_block_194 = 17011164281425506503;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block_194 = 17011164281425506503;
                                                                                                    }
                                                                                                }
                                                                                                _ => {}
                                                                                            }
                                                                                            match current_block_194 {
                                                                                                5577234879133443219 => {}
                                                                                                _ => {
                                                                                                    fro_zclose(&mut work.fro);
                                                                                                    aflush(work.ex);
                                                                                                    isr_do((*top).behavior.isr, ISR_IGNOREINTS);
                                                                                                    r = chnamemod(
                                                                                                        &mut work.ex,
                                                                                                        newworkname,
                                                                                                        mani_filename,
                                                                                                        1 as libc::c_int,
                                                                                                        newworkmode,
                                                                                                        mtime,
                                                                                                    );
                                                                                                    keepdirtemp(newworkname);
                                                                                                    isr_do((*top).behavior.isr, ISR_RESTOREINTS);
                                                                                                    current_block_194 = 5577234879133443219;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        _ => {
                                                                                            current_block_194 = 10393716428851982524;
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    current_block_194 = 5577234879133443219;
                                                                                }
                                                                            }
                                                                            match current_block_194 {
                                                                                10393716428851982524 => {}
                                                                                _ => {
                                                                                    if 0 as libc::c_int > r {
                                                                                        syserror(*__errno_location(), mani_filename);
                                                                                    } else {
                                                                                        diagnose(b"done\0" as *const u8 as *const libc::c_char);
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            cleanup(&mut exitstatus, &mut work);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    gnurcs_goodbye();
    return exitstatus;
}
static mut ci_aka: [uint8_t; 19] = [
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    'c' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    7 as libc::c_int as uint8_t,
    'c' as i32 as uint8_t,
    'h' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    6 as libc::c_int as uint8_t,
    'c' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    't' as i32 as uint8_t,
];
pub static mut ya_ci: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                ci_main
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            aka: ci_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};

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
    static mut top: *mut top;
    static mut stdout: *mut FILE;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn str2expmode(s: *const libc::c_char) -> libc::c_int;
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn chnamemod(
        fromp: *mut *mut FILE,
        from: *const libc::c_char,
        to: *const libc::c_char,
        set_mode: libc::c_int,
        mode: mode_t,
        mtime: timespec,
    ) -> libc::c_int;
    fn findlock(delete: bool, target: *mut *mut delta) -> libc::c_int;
    fn checkaccesslist() -> bool;
    fn dorewrite(lockflag: bool, changed: libc::c_int) -> libc::c_int;
    fn donerewrite(changed: libc::c_int, mtime: timespec) -> libc::c_int;
    fn ORCSclose();
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
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
    fn ttystdin() -> bool;
    fn yesorno(default_answer: bool, question: *const libc::c_char, _: ...) -> bool;
    fn write_desc_maybe(to: *mut FILE);
    fn countnumflds(s: *const libc::c_char) -> libc::c_int;
    fn take(count: size_t, ref_0: *const libc::c_char) -> cbuf;
    fn cmpnum(num1: *const libc::c_char, num2: *const libc::c_char) -> libc::c_int;
    fn cmpnumfld(
        num1: *const libc::c_char,
        num2: *const libc::c_char,
        fld: libc::c_int,
    ) -> libc::c_int;
    fn genrevs(
        revno: *const libc::c_char,
        date: *const libc::c_char,
        author: *const libc::c_char,
        state: *const libc::c_char,
        store: *mut *mut wlink,
    ) -> *mut delta;
    fn delta_from_ref(ref_0: *const libc::c_char) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const libc::c_char, fp: *mut fro) -> bool;
    fn namedrev(name: *const libc::c_char, delta: *mut delta) -> *const libc::c_char;
    fn str2date(source: *const libc::c_char, target: *mut libc::c_char);
    fn date2time(source: *const libc::c_char) -> time_t;
    fn zone_set(s: *const libc::c_char);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn redefined(c: libc::c_int);
    fn chk_set_rev(rev: *mut *const libc::c_char, arg: *mut libc::c_char);
    fn ffree();
    fn str_save(s: *const libc::c_char) -> *mut libc::c_char;
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
    fn file_mtime(enable: bool, st: *const stat) -> timespec;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    static mut exit_failure: libc::c_int;
    fn diagnose(fmt: *const libc::c_char, _: ...);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn make_space(name: *const libc::c_char) -> *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn close_space(divvy: *mut divvy);
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn extend(tp: *mut link, x: *const libc::c_void, to: *mut divvy) -> *mut link;
    fn stat_mine_p(st: *mut stat) -> bool;
    fn setrid();
    fn getcaller() -> *const libc::c_char;
    fn caller_login_p(login: *const libc::c_char) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn addlock_maybe(delta: *mut delta, selfsame: bool, verbose: bool) -> libc::c_int;
    fn fopen_safer(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
    ) -> *mut FILE;
    fn Ozclose(p: *mut *mut FILE);
    fn aflush(f: *mut FILE);
    fn maketemp(n: libc::c_int) -> *const libc::c_char;
    fn makedirtemp(isworkfile: bool) -> *const libc::c_char;
    fn keepdirtemp(name: *const libc::c_char);
    fn tempunlink();
    fn dirtempunlink();
    fn fro_zclose(p: *mut *mut fro);
    fn fro_trundling(sequential: bool, f: *mut fro);
    fn isr_do(scratch: *mut isr_scratch, action: isr_actions);
    static mut peer_super: symdef;
    fn find_peer_prog(prog: *mut symdef) -> *const libc::c_char;
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
pub struct work {
    pub st: stat,
    pub force: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jstuff {
    pub jstuff: *mut divvy,
    pub head: link,
    pub tp: *mut link,
    pub merge: *mut symdef,
    pub expand: *const libc::c_char,
    pub suffix: *const libc::c_char,
    pub version: *const libc::c_char,
    pub zone: *const libc::c_char,
    pub d: *mut delta,
    pub ls: *mut *const libc::c_char,
    pub lastidx: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: libc::c_long) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
static mut co_blurb: [libc::c_char; 53] = unsafe {
    *::std::mem::transmute::<
        &[u8; 53],
        &[libc::c_char; 53],
    >(b"Check out working files from revisions of RCS files.\0")
};
static mut co_help: [libc::c_char; 1679] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1679],
        &[libc::c_char; 1679],
    >(
        b"[options] file ...\nOptions:\n  -f[REV]       Force overwrite of working file.\n  -I[REV]       Interactive.\n  -p[REV]       Write to stdout instead of the working file.\n  -q[REV]       Quiet mode.\n  -r[REV]       Normal checkout.\n  -l[REV]       Like -r, but also lock.\n  -u[REV]       Like -l, but unlock.\n  -M[REV]       Reset working file mtime (relevant for -l, -u).\n  -kSUBST       Use SUBST substitution, one of: kv, kvl, k, o, b, v.\n  -dDATE        Select latest before or on DATE.\n  -jJOINS       Merge using JOINS, a list of REV:REV pairs;\n                this option is obsolete -- see rcsmerge(1).\n  -sSTATE       Select matching state STATE.\n  -S            Enable \"self-same\" mode.\n  -T            Preserve the modification time on the RCS file\n                even if it changes because a lock is added or removed.\n  -wWHO         Select matching login WHO.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution\n                and also the default timezone for -dDATE.\n\nMultiple flags in {fIlMpqru} may be used, except for -r, -l, -u, which are\nmutually exclusive.  If specified, REV can be symbolic, numeric, or mixed:\n  symbolic -- must have been defined previously (see ci(1))\n  $        -- determine the revision number from keyword values\n              in the working file\n  .N       -- prepend default branch => DEFBR.N\n  BR.N     -- use this\n  BR       -- latest revision on branch BR\nIf REV is omitted, take it to be the latest on the default branch.\n\0",
    )
};
static mut ks_hws: [libc::c_char; 3] = unsafe {
    *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b" \t\0")
};
static mut quietarg: [libc::c_char; 3] = unsafe {
    *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"-q\0")
};
unsafe extern "C" fn cleanup(
    mut exitstatus: *mut libc::c_int,
    mut neworkptr: *mut *mut FILE,
) {
    let mut mstdout: *mut FILE = (*top).manifestation.standard_output;
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    ORCSclose();
    if !((*top).flow.from).is_null()
        && RM_STDIO as libc::c_int as libc::c_uint
            == (*(*top).flow.from).rm as libc::c_uint && !((*top).flow.res).is_null()
        && (*top).flow.res != mstdout
    {
        Ozclose(&mut (*top).flow.res);
    }
    if *neworkptr != mstdout {
        Ozclose(neworkptr);
    }
    dirtempunlink();
}
unsafe extern "C" fn rmworkfile(mut work: *mut work) -> bool {
    if (*work).st.st_mode
        & (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
            as libc::c_uint != 0 && !(*work).force
    {
        let mut mani_filename: *const libc::c_char = (*top).manifestation.filename;
        if !yesorno(
            0 as libc::c_int != 0,
            b"writable %s exists%s; remove it\0" as *const u8 as *const libc::c_char,
            mani_filename,
            if stat_mine_p(&mut (*work).st) as libc::c_int != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b", and you do not own it\0" as *const u8 as *const libc::c_char
            },
        ) {
            if !(*top).behavior.quiet && ttystdin() as libc::c_int != 0 {
                generic_error(
                    0 as *const libc::c_char,
                    b"checkout aborted\0" as *const u8 as *const libc::c_char,
                );
            } else {
                generic_error(
                    0 as *const libc::c_char,
                    b"writable %s exists; checkout aborted\0" as *const u8
                        as *const libc::c_char,
                    mani_filename,
                );
            }
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn rmlock(mut delta: *const delta) -> libc::c_int {
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut rl: *const rcslock = 0 as *const rcslock;
    box_0.next = (*(*top).repository.r).locks;
    tp = lock_memq(&mut box_0, 0 as libc::c_int != 0, delta as *const libc::c_void);
    if tp.is_null() {
        return 0 as libc::c_int;
    }
    rl = (*(*tp).next).entry as *const rcslock;
    if !caller_login_p((*rl).login) {
        generic_error(
            (*top).repository.filename,
            b"revision %s locked by %s; use co -r or rcs -u\0" as *const u8
                as *const libc::c_char,
            (*delta).num,
            (*rl).login,
        );
        return -(1 as libc::c_int);
    }
    lock_drop(&mut box_0, tp);
    return 1 as libc::c_int;
}
unsafe extern "C" fn jpush(mut rev: *const libc::c_char, mut js: *mut jstuff) {
    (*js).tp = extend((*js).tp, rev as *const libc::c_void, (*js).jstuff);
    (*js).lastidx += 1;
    (*js).lastidx;
}
unsafe extern "C" fn addjoin(
    mut spec: *mut libc::c_char,
    mut js: *mut jstuff,
) -> *mut libc::c_char {
    let delims: [libc::c_char; 7] = *::std::mem::transmute::<
        &[u8; 7],
        &[libc::c_char; 7],
    >(b" \t\n:,;\0");
    let mut eot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    let mut cool: *mut delta = 0 as *mut delta;
    let mut numrev: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    spec = spec.offset(strspn(spec, ks_hws.as_ptr()) as isize);
    eot = spec.offset(strcspn(spec, delims.as_ptr()) as isize);
    save = *eot;
    *eot = '\0' as i32 as libc::c_char;
    cool = if fully_numeric(&mut numrev, spec, 0 as *mut fro) as libc::c_int != 0 {
        delta_from_ref(numrev.string)
    } else {
        0 as *mut delta
    };
    *eot = save;
    if cool.is_null() {
        return 0 as *mut libc::c_char;
    }
    jpush((*cool).num, js);
    eot = eot.offset(strspn(eot, ks_hws.as_ptr()) as isize);
    return eot;
}
unsafe extern "C" fn getancestor(
    mut r1: *const libc::c_char,
    mut r2: *const libc::c_char,
) -> *const libc::c_char {
    let mut t1: *const libc::c_char = 0 as *const libc::c_char;
    let mut t2: *const libc::c_char = 0 as *const libc::c_char;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut l3: libc::c_int = 0;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    l1 = countnumflds(r1);
    l2 = countnumflds(r2);
    if ((2 as libc::c_int) < l1 || (2 as libc::c_int) < l2)
        && !(0 as libc::c_int == cmpnum(r1, r2))
    {
        l3 = 0 as libc::c_int;
        while 0 as libc::c_int == cmpnumfld(r1, r2, 1 as libc::c_int + l3)
            && 0 as libc::c_int == cmpnumfld(r1, r2, 2 as libc::c_int + l3)
        {
            l3 += 2 as libc::c_int;
        }
        if l3 == 0 as libc::c_int {
            t1 = (take(
                (if l1 > 2 as libc::c_int { 2 as libc::c_int } else { l1 }) as size_t,
                r1,
            ))
                .string;
            t2 = (take(
                (if l2 > 2 as libc::c_int { 2 as libc::c_int } else { l2 }) as size_t,
                r2,
            ))
                .string;
            r = if 0 as libc::c_int > cmpnum(t1, t2) { t1 } else { t2 };
            if !(0 as libc::c_int == cmpnum(r, r1))
                && !(0 as libc::c_int == cmpnum(r, r2))
            {
                return str_save(r);
            }
        } else if !(0 as libc::c_int == cmpnumfld(r1, r2, 1 as libc::c_int + l3)) {
            return str_save((take(l3 as size_t, r1)).string)
        }
    }
    generic_error(
        (*top).repository.filename,
        b"common ancestor of %s and %s undefined\0" as *const u8 as *const libc::c_char,
        r1,
        r2,
    );
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn preparejoin(
    mut argv: *mut libc::c_char,
    mut js: *mut jstuff,
) -> bool {
    let ks_comma: [libc::c_char; 2] = *::std::mem::transmute::<
        &[u8; 2],
        &[libc::c_char; 2],
    >(b",\0");
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: bool = 1 as libc::c_int != 0;
    (*js).jstuff = make_space(b"jstuff\0" as *const u8 as *const libc::c_char);
    (*js).head.next = 0 as *mut link;
    (*js).tp = &mut (*js).head;
    if ((*js).merge).is_null() {
        (*js)
            .merge = zlloc(
            plexus,
            (::std::mem::size_of::<symdef>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong),
        ) as *mut symdef;
        (*(*js).merge).meaningful = b"merge\0" as *const u8 as *const libc::c_char;
    }
    (*js).lastidx = -(1 as libc::c_int);
    s = argv;
    loop {
        j = strtok_r(s, ks_comma.as_ptr(), &mut save);
        if j.is_null() {
            break;
        }
        j = addjoin(j, js);
        if j.is_null() {
            return 0 as libc::c_int != 0;
        }
        let mut current_block_18: u64;
        let fresh0 = j;
        j = j.offset(1);
        if *fresh0 as libc::c_int == ':' as i32 {
            j = j.offset(strspn(j, ks_hws.as_ptr()) as isize);
            if *j as libc::c_int == '\0' as i32 {
                current_block_18 = 1261244174291975988;
            } else {
                j = addjoin(j, js);
                if j.is_null() {
                    return 0 as libc::c_int != 0;
                }
                current_block_18 = 11584701595673473500;
            }
        } else if (*js).lastidx == 0 as libc::c_int {
            let mut two: *const libc::c_char = (*(*js).tp).entry as *const libc::c_char;
            jpush(two, js);
            (*(*js).tp).entry = getancestor((*(*js).d).num, two) as *const libc::c_void;
            if ((*(*js).tp).entry).is_null() {
                rv = 0 as libc::c_int != 0;
            }
            current_block_18 = 11584701595673473500;
        } else {
            current_block_18 = 1261244174291975988;
        }
        match current_block_18 {
            1261244174291975988 => {
                generic_fatal(
                    (*top).repository.filename,
                    b"join pair incomplete\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
        s = 0 as *mut libc::c_char;
    }
    if (*js).lastidx < 1 as libc::c_int {
        generic_fatal(
            (*top).repository.filename,
            b"empty join\0" as *const u8 as *const libc::c_char,
        );
    }
    (*js)
        .ls = pointer_array(plexus, (1 as libc::c_int + (*js).lastidx) as size_t)
        as *mut *const libc::c_char;
    (*js).tp = (*js).head.next;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= (*js).lastidx {
        let ref mut fresh1 = *((*js).ls).offset(i as isize);
        *fresh1 = (*(*js).tp).entry as *const libc::c_char;
        i += 1;
        i;
        (*js).tp = (*(*js).tp).next;
    }
    close_space((*js).jstuff);
    (*js).jstuff = 0 as *mut divvy;
    return rv;
}
unsafe extern "C" fn buildjoin(
    mut initialfile: *const libc::c_char,
    mut js: *mut jstuff,
) -> bool {
    let mut current_block: u64;
    let mut rev2: *const libc::c_char = 0 as *const libc::c_char;
    let mut rev3: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut cov: [*const libc::c_char; 11] = [0 as *const libc::c_char; 11];
    let mut mergev: [*const libc::c_char; 11] = [0 as *const libc::c_char; 11];
    let mut p: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut len: size_t = 0;
    let mut subs: *const libc::c_char = 0 as *const libc::c_char;
    rev2 = maketemp(0 as libc::c_int);
    rev3 = maketemp(3 as libc::c_int);
    cov[1 as libc::c_int as usize] = find_peer_prog(&mut peer_super);
    cov[2 as libc::c_int as usize] = b"co\0" as *const u8 as *const libc::c_char;
    p = &mut *cov.as_mut_ptr().offset((1 as libc::c_int + 3 as libc::c_int) as isize)
        as *mut *const libc::c_char;
    if !((*js).expand).is_null() {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = (*js).expand;
    }
    if !((*js).suffix).is_null() {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = (*js).suffix;
    }
    if !((*js).version).is_null() {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = (*js).version;
    }
    if !((*js).zone).is_null() {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = (*js).zone;
    }
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = quietarg.as_ptr();
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = (*top).repository.filename;
    *p = 0 as *const libc::c_char;
    mergev[1 as libc::c_int as usize] = find_peer_prog((*js).merge);
    mergev[4 as libc::c_int as usize] = b"-L\0" as *const u8 as *const libc::c_char;
    mergev[2 as libc::c_int as usize] = mergev[4 as libc::c_int as usize];
    i = 0 as libc::c_int;
    loop {
        if !(i < (*js).lastidx) {
            current_block = 17184638872671510253;
            break;
        }
        if i == 0 as libc::c_int {
            subs = (*(*js).d).num;
        } else {
            accf(
                single,
                b"%s,%s:%s\0" as *const u8 as *const libc::c_char,
                subs,
                *((*js).ls).offset((i - 2 as libc::c_int) as isize),
                *((*js).ls).offset((i - 1 as libc::c_int) as isize),
            );
            subs = finish_string(single, &mut len);
        }
        diagnose(
            b"revision %s\0" as *const u8 as *const libc::c_char,
            *((*js).ls).offset(i as isize),
        );
        accf(
            single,
            b"-p%s\0" as *const u8 as *const libc::c_char,
            *((*js).ls).offset(i as isize),
        );
        cov[3 as libc::c_int as usize] = finish_string(single, &mut len);
        if runv(-(1 as libc::c_int), rev2, cov.as_mut_ptr()) != 0 {
            current_block = 7097255640795091767;
            break;
        }
        diagnose(
            b"revision %s\0" as *const u8 as *const libc::c_char,
            *((*js).ls).offset((i + 1 as libc::c_int) as isize),
        );
        accf(
            single,
            b"-p%s\0" as *const u8 as *const libc::c_char,
            *((*js).ls).offset((i + 1 as libc::c_int) as isize),
        );
        cov[3 as libc::c_int as usize] = finish_string(single, &mut len);
        if runv(-(1 as libc::c_int), rev3, cov.as_mut_ptr()) != 0 {
            current_block = 7097255640795091767;
            break;
        }
        diagnose(b"merging...\0" as *const u8 as *const libc::c_char);
        mergev[3 as libc::c_int as usize] = subs;
        mergev[5 as libc::c_int
            as usize] = *((*js).ls).offset((i + 1 as libc::c_int) as isize);
        p = &mut *mergev.as_mut_ptr().offset(6 as libc::c_int as isize)
            as *mut *const libc::c_char;
        if (*top).behavior.quiet {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = quietarg.as_ptr();
        }
        if (*js).lastidx <= i + 2 as libc::c_int
            && !((*top).manifestation.standard_output).is_null()
        {
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = b"-p\0" as *const u8 as *const libc::c_char;
        }
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = initialfile;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = rev2;
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = rev3;
        *p = 0 as *const libc::c_char;
        if 2 as libc::c_int
            == runv(-(1 as libc::c_int), 0 as *const libc::c_char, mergev.as_mut_ptr())
        {
            current_block = 7097255640795091767;
            break;
        }
        i = i + 2 as libc::c_int;
    }
    match current_block {
        17184638872671510253 => return 1 as libc::c_int != 0,
        _ => {
            (*top).flow.erroneous = 1 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
    };
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: co_blurb.as_ptr(),
            help: co_help.as_ptr(),
            tyag: (1 as libc::c_int) << 3 as libc::c_int
                | ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int),
        };
        init
    }
};
unsafe extern "C" fn co_main(
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut exitstatus: libc::c_int = 0 as libc::c_int;
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
            force: 0 as libc::c_int != 0,
        };
        init
    };
    let mut jstuff: jstuff = jstuff {
        jstuff: 0 as *mut divvy,
        head: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp: 0 as *mut link,
        merge: 0 as *mut symdef,
        expand: 0 as *const libc::c_char,
        suffix: 0 as *const libc::c_char,
        version: 0 as *const libc::c_char,
        zone: 0 as *const libc::c_char,
        d: 0 as *mut delta,
        ls: 0 as *mut *const libc::c_char,
        lastidx: 0,
    };
    let mut neworkptr: *mut FILE = 0 as *mut FILE;
    let mut lockflag: libc::c_int = 0 as libc::c_int;
    let mut mtimeflag: bool = 0 as libc::c_int != 0;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut joinflag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut author: *const libc::c_char = 0 as *const libc::c_char;
    let mut date: *const libc::c_char = 0 as *const libc::c_char;
    let mut rev: *const libc::c_char = 0 as *const libc::c_char;
    let mut state: *const libc::c_char = 0 as *const libc::c_char;
    let mut joinname: *const libc::c_char = 0 as *const libc::c_char;
    let mut newdate: *const libc::c_char = 0 as *const libc::c_char;
    let mut neworkname: *const libc::c_char = 0 as *const libc::c_char;
    let mut changelock: libc::c_int = 0;
    let mut expmode: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut workstatstat: libc::c_int = 0;
    let mut tostdout: bool = false;
    let mut Ttimeflag: bool = false;
    let mut selfsame: bool = 0 as libc::c_int != 0;
    let mut finaldate: [libc::c_char; 22] = [0; 22];
    let mut deltas: *mut wlink = 0 as *mut wlink;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut jstuff as *mut jstuff as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<jstuff>() as libc::c_ulong,
    );
    setrid();
    state = 0 as *const libc::c_char;
    rev = state;
    date = rev;
    author = date;
    joinflag = 0 as *mut libc::c_char;
    expmode = -(1 as libc::c_int);
    tostdout = 0 as libc::c_int != 0;
    Ttimeflag = 0 as libc::c_int != 0;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as libc::c_int) < argc
            && {
                let fresh13 = a;
                a = a.offset(1);
                *fresh13 as libc::c_int == '-' as i32
            })
        {
            break;
        }
        let fresh14 = a;
        a = a.offset(1);
        match *fresh14 as libc::c_int {
            114 => {
                current_block = 13993563052237549288;
            }
            102 => {
                work.force = 1 as libc::c_int != 0;
                current_block = 13993563052237549288;
            }
            108 => {
                if lockflag < 0 as libc::c_int {
                    generic_warn(
                        0 as *const libc::c_char,
                        b"-u overridden by -l.\0" as *const u8 as *const libc::c_char,
                    );
                }
                lockflag = 1 as libc::c_int;
                current_block = 13993563052237549288;
            }
            117 => {
                if (0 as libc::c_int) < lockflag {
                    generic_warn(
                        0 as *const libc::c_char,
                        b"-l overridden by -u.\0" as *const u8 as *const libc::c_char,
                    );
                }
                lockflag = -(1 as libc::c_int);
                current_block = 13993563052237549288;
            }
            112 => {
                tostdout = 1 as libc::c_int != 0;
                current_block = 13993563052237549288;
            }
            73 => {
                (*top).behavior.interactive = 1 as libc::c_int != 0;
                current_block = 13993563052237549288;
            }
            113 => {
                (*top).behavior.quiet = 1 as libc::c_int != 0;
                current_block = 13993563052237549288;
            }
            100 => {
                if !date.is_null() {
                    redefined('d' as i32);
                }
                str2date(a, finaldate.as_mut_ptr());
                date = finaldate.as_mut_ptr();
                continue;
            }
            106 => {
                if *a != 0 {
                    if !joinflag.is_null() {
                        redefined('j' as i32);
                    }
                    joinflag = a;
                }
                continue;
            }
            77 => {
                mtimeflag = 1 as libc::c_int != 0;
                current_block = 13993563052237549288;
            }
            115 => {
                if *a != 0 {
                    if !state.is_null() {
                        redefined('s' as i32);
                    }
                    state = a;
                }
                continue;
            }
            83 => {
                selfsame = 1 as libc::c_int != 0;
                continue;
            }
            84 => {
                if *a != 0 {
                    current_block = 7078942599093635947;
                } else {
                    Ttimeflag = 1 as libc::c_int != 0;
                    continue;
                }
            }
            119 => {
                if !author.is_null() {
                    redefined('w' as i32);
                }
                author = if *a as libc::c_int != 0 {
                    a as *const libc::c_char
                } else {
                    getcaller()
                };
                continue;
            }
            120 => {
                jstuff.suffix = *argv;
                (*top).behavior.pe = a;
                continue;
            }
            86 => {
                jstuff.version = *argv;
                setRCSversion(jstuff.version);
                continue;
            }
            122 => {
                jstuff.zone = *argv;
                zone_set(a);
                continue;
            }
            107 => {
                jstuff.expand = *argv;
                if 0 as libc::c_int <= expmode {
                    redefined('k' as i32);
                }
                expmode = str2expmode(a);
                if 0 as libc::c_int <= expmode {
                    continue;
                }
                current_block = 7078942599093635947;
            }
            _ => {
                current_block = 7078942599093635947;
            }
        }
        match current_block {
            7078942599093635947 => {
                bad_option(*argv);
            }
            _ => {
                chk_set_rev(&mut rev, a);
            }
        }
    }
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut neworkptr);
    } else if argc < 1 as libc::c_int {
        generic_fatal(
            0 as *const libc::c_char,
            b"no input file\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut current_block_118: u64;
        while (0 as libc::c_int) < argc {
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut mani_filename: *const libc::c_char = 0 as *const libc::c_char;
            let mut kws: libc::c_int = 0;
            ffree();
            if !(pairnames(
                argc,
                argv,
                if lockflag != 0 {
                    Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro)
                } else {
                    Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro)
                },
                1 as libc::c_int != 0,
                0 as libc::c_int != 0,
            ) <= 0 as libc::c_int)
            {
                repo_stat = &mut (*top).repository.stat;
                mani_filename = (*top).manifestation.filename;
                kws = (*top).behavior.kws;
                diagnose(
                    b"%s  -->  %s\0" as *const u8 as *const libc::c_char,
                    (*top).repository.filename,
                    if tostdout as libc::c_int != 0 {
                        b"standard output\0" as *const u8 as *const libc::c_char
                    } else {
                        mani_filename
                    },
                );
                workstatstat = -(1 as libc::c_int);
                if tostdout {
                    neworkname = 0 as *const libc::c_char;
                    (*top).manifestation.standard_output = stdout;
                    neworkptr = (*top).manifestation.standard_output;
                    current_block_118 = 9199578309995299736;
                } else {
                    workstatstat = stat(mani_filename, &mut work.st);
                    if !(0 as libc::c_int > workstatstat)
                        && ((*top).repository.stat.st_ino == work.st.st_ino
                            && (*top).repository.stat.st_dev == work.st.st_dev)
                    {
                        generic_error(
                            (*top).repository.filename,
                            b"RCS file is the same as working file %s.\0" as *const u8
                                as *const libc::c_char,
                            mani_filename,
                        );
                        current_block_118 = 18038362259723567392;
                    } else {
                        neworkname = makedirtemp(1 as libc::c_int != 0);
                        neworkptr = fopen_safer(
                            neworkname,
                            b"w\0" as *const u8 as *const libc::c_char,
                        );
                        if neworkptr.is_null() {
                            if *__errno_location() == 13 as libc::c_int {
                                generic_error(
                                    (*top).manifestation.filename,
                                    b"permission denied on parent directory\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                syserror(*__errno_location(), neworkname);
                            }
                            current_block_118 = 18038362259723567392;
                        } else {
                            current_block_118 = 9199578309995299736;
                        }
                    }
                }
                match current_block_118 {
                    18038362259723567392 => {}
                    _ => {
                        if ((*top).repository.tip).is_null() {
                            diagnose(
                                b"no revisions present; generating empty revision 0.0\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            if lockflag != 0 {
                                generic_warn(
                                    0 as *const libc::c_char,
                                    b"no revisions, so nothing can be %slocked\0" as *const u8
                                        as *const libc::c_char,
                                    if lockflag < 0 as libc::c_int {
                                        b"un\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            Ozclose(&mut (*top).flow.res);
                            if !(0 as libc::c_int > workstatstat) {
                                if !rmworkfile(&mut work) {
                                    current_block_118 = 18038362259723567392;
                                } else {
                                    current_block_118 = 10369920510435091891;
                                }
                            } else {
                                current_block_118 = 10369920510435091891;
                            }
                            match current_block_118 {
                                18038362259723567392 => {}
                                _ => {
                                    changelock = 0 as libc::c_int;
                                    newdate = 0 as *const libc::c_char;
                                    current_block_118 = 10945915984064580713;
                                }
                            }
                        } else {
                            let mut numericrev: cbuf = cbuf {
                                string: 0 as *const libc::c_char,
                                size: 0,
                            };
                            let mut locks: libc::c_int = if lockflag != 0 {
                                findlock(0 as libc::c_int != 0, &mut jstuff.d)
                            } else {
                                0 as libc::c_int
                            };
                            let mut from: *mut fro = (*top).flow.from;
                            if !rev.is_null() {
                                if !fully_numeric(&mut numericrev, rev, 0 as *mut fro) {
                                    current_block_118 = 18038362259723567392;
                                } else {
                                    current_block_118 = 2220405792722996547;
                                }
                            } else {
                                match locks {
                                    0 => {
                                        current_block_118 = 8748695652387263882;
                                        match current_block_118 {
                                            4066539042126921219 => {
                                                numericrev.string = str_save((*jstuff.d).num);
                                            }
                                            _ => {
                                                numericrev
                                                    .string = if !((*(*top).repository.r).branch).is_null() {
                                                    (*(*top).repository.r).branch
                                                } else {
                                                    b"\0" as *const u8 as *const libc::c_char
                                                };
                                            }
                                        }
                                        current_block_118 = 2220405792722996547;
                                    }
                                    1 => {
                                        current_block_118 = 4066539042126921219;
                                        match current_block_118 {
                                            4066539042126921219 => {
                                                numericrev.string = str_save((*jstuff.d).num);
                                            }
                                            _ => {
                                                numericrev
                                                    .string = if !((*(*top).repository.r).branch).is_null() {
                                                    (*(*top).repository.r).branch
                                                } else {
                                                    b"\0" as *const u8 as *const libc::c_char
                                                };
                                            }
                                        }
                                        current_block_118 = 2220405792722996547;
                                    }
                                    _ => {
                                        current_block_118 = 18038362259723567392;
                                    }
                                }
                            }
                            match current_block_118 {
                                18038362259723567392 => {}
                                _ => {
                                    jstuff
                                        .d = genrevs(
                                        numericrev.string,
                                        date,
                                        author,
                                        state,
                                        &mut deltas,
                                    );
                                    if (jstuff.d).is_null() {
                                        current_block_118 = 18038362259723567392;
                                    } else {
                                        changelock = if lockflag < 0 as libc::c_int {
                                            rmlock(jstuff.d)
                                        } else if lockflag == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            addlock_maybe(jstuff.d, selfsame, 1 as libc::c_int != 0)
                                        };
                                        if changelock < 0 as libc::c_int
                                            || changelock != 0 && !checkaccesslist()
                                            || 0 as libc::c_int > dorewrite(lockflag != 0, changelock)
                                        {
                                            current_block_118 = 18038362259723567392;
                                        } else {
                                            if 0 as libc::c_int <= expmode {
                                                (*top).behavior.kws = expmode;
                                                kws = (*top).behavior.kws;
                                            }
                                            if (0 as libc::c_int) < lockflag
                                                && kws == kwsub_v as libc::c_int
                                            {
                                                generic_error(
                                                    (*top).repository.filename,
                                                    b"cannot combine -kv and -l\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                current_block_118 = 18038362259723567392;
                                            } else if !joinflag.is_null()
                                                && !preparejoin(joinflag, &mut jstuff)
                                            {
                                                current_block_118 = 18038362259723567392;
                                            } else {
                                                diagnose(
                                                    b"revision %s%s\0" as *const u8 as *const libc::c_char,
                                                    (*jstuff.d).num,
                                                    if (0 as libc::c_int) < lockflag {
                                                        b" (locked)\0" as *const u8 as *const libc::c_char
                                                    } else if lockflag < 0 as libc::c_int {
                                                        b" (unlocked)\0" as *const u8 as *const libc::c_char
                                                    } else {
                                                        b"\0" as *const u8 as *const libc::c_char
                                                    },
                                                );
                                                (*from)
                                                    .verbatim = *((*(*jstuff.d).text).holes)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        ((*(*jstuff.d).text).count)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                                    ) + 2 as libc::c_int as libc::c_long;
                                                if !(0 as libc::c_int > workstatstat) {
                                                    if !rmworkfile(&mut work) {
                                                        current_block_118 = 18038362259723567392;
                                                    } else {
                                                        current_block_118 = 1677945370889843322;
                                                    }
                                                } else {
                                                    current_block_118 = 1677945370889843322;
                                                }
                                                match current_block_118 {
                                                    18038362259723567392 => {}
                                                    _ => {
                                                        write_desc_maybe((*top).flow.to);
                                                        (*top)
                                                            .behavior
                                                            .inclusive_of_Locker_in_Id_val = (0 as libc::c_int)
                                                            < lockflag;
                                                        (*jstuff.d).name = namedrev(rev, jstuff.d);
                                                        joinname = buildrevision(
                                                            deltas,
                                                            jstuff.d,
                                                            if !joinflag.is_null() && tostdout as libc::c_int != 0 {
                                                                0 as *mut FILE
                                                            } else {
                                                                neworkptr
                                                            },
                                                            kws < kwsub_o as libc::c_int,
                                                        );
                                                        if (*top).flow.res == neworkptr {
                                                            (*top).flow.res = 0 as *mut FILE;
                                                        }
                                                        if changelock != 0
                                                            && (*deltas).entry != jstuff.d as *mut libc::c_void
                                                        {
                                                            fro_trundling(1 as libc::c_int != 0, from);
                                                        }
                                                        if 0 as libc::c_int
                                                            > donerewrite(changelock, file_mtime(Ttimeflag, repo_stat))
                                                        {
                                                            current_block_118 = 18038362259723567392;
                                                        } else {
                                                            if changelock != 0 {
                                                                locks += lockflag;
                                                                if (1 as libc::c_int) < locks {
                                                                    generic_warn(
                                                                        (*top).repository.filename,
                                                                        b"You now have %d locks.\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        locks,
                                                                    );
                                                                }
                                                            }
                                                            newdate = (*jstuff.d).date;
                                                            if !joinflag.is_null() {
                                                                newdate = 0 as *const libc::c_char;
                                                                if joinname.is_null() {
                                                                    aflush(neworkptr);
                                                                    joinname = neworkname;
                                                                }
                                                                if kws == kwsub_b as libc::c_int {
                                                                    generic_error(
                                                                        (*top).manifestation.filename,
                                                                        b"merging binary files\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                if !buildjoin(joinname, &mut jstuff) {
                                                                    current_block_118 = 18038362259723567392;
                                                                } else {
                                                                    current_block_118 = 10945915984064580713;
                                                                }
                                                            } else {
                                                                current_block_118 = 10945915984064580713;
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
                        match current_block_118 {
                            18038362259723567392 => {}
                            _ => {
                                if !tostdout {
                                    let mut m: mode_t = (*repo_stat).st_mode
                                        & !(0o200 as libc::c_int
                                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                                >> 3 as libc::c_int) as mode_t
                                        | (if !(kws == kwsub_v as libc::c_int
                                            || lockflag <= 0 as libc::c_int
                                                && (*top).behavior.strictly_locking as libc::c_int != 0)
                                        {
                                            0o200 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as libc::c_uint;
                                    let mut t: time_t = if mtimeflag as libc::c_int != 0
                                        && !newdate.is_null()
                                    {
                                        date2time(newdate)
                                    } else {
                                        -(1 as libc::c_int) as time_t
                                    };
                                    aflush(neworkptr);
                                    isr_do((*top).behavior.isr, ISR_IGNOREINTS);
                                    r = chnamemod(
                                        &mut neworkptr,
                                        neworkname,
                                        mani_filename,
                                        1 as libc::c_int,
                                        m,
                                        make_timespec(t, 0 as libc::c_int as libc::c_long),
                                    );
                                    keepdirtemp(neworkname);
                                    isr_do((*top).behavior.isr, ISR_RESTOREINTS);
                                    if 0 as libc::c_int > r {
                                        syserror(*__errno_location(), mani_filename);
                                        generic_error(
                                            0 as *const libc::c_char,
                                            b"see %s\0" as *const u8 as *const libc::c_char,
                                            neworkname,
                                        );
                                    } else {
                                        diagnose(b"done\0" as *const u8 as *const libc::c_char);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            cleanup(&mut exitstatus, &mut neworkptr);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    Ozclose(&mut (*top).manifestation.standard_output);
    gnurcs_goodbye();
    return exitstatus;
}
static mut co_aka: [uint8_t; 13] = [
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    'c' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    8 as libc::c_int as uint8_t,
    'c' as i32 as uint8_t,
    'h' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'u' as i32 as uint8_t,
    't' as i32 as uint8_t,
];
pub static mut ya_co: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                co_main
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            aka: co_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};

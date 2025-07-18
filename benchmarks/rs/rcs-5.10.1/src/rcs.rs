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
    pub type editstuff;
    static prog_diff: [libc::c_char; 0];
    static diff_flags: [libc::c_char; 0];
    static mut top: *mut top;
    static ks_revno: [libc::c_char; 0];
    fn str2expmode(s: *const libc::c_char) -> libc::c_int;
    fn make_editstuff() -> *mut editstuff;
    fn unmake_editstuff(es: *mut editstuff);
    fn finishedit(
        es: *mut editstuff,
        delta: *const delta,
        outfile: *mut FILE,
        done: bool,
    );
    fn snapshotedit(es: *mut editstuff, f: *mut FILE);
    fn enterstring(es: *mut editstuff, atat: *mut atat);
    fn editstring(es: *mut editstuff, script: *const atat, delta: *const delta);
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn findlock(delete: bool, target: *mut *mut delta) -> libc::c_int;
    fn addsymbol(
        num: *const libc::c_char,
        name: *const libc::c_char,
        rebind: bool,
    ) -> libc::c_int;
    fn checkaccesslist() -> bool;
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
    fn cleanlogmsg(m: *const libc::c_char, s: size_t) -> cbuf;
    fn yesorno(default_answer: bool, question: *const libc::c_char, _: ...) -> bool;
    fn putdesc(cb: *mut cbuf, textflag: bool, textfile: *mut libc::c_char);
    fn putadmin();
    fn puttree(root: *const delta, fout: *mut FILE);
    fn putdtext(
        delta: *const delta,
        srcname: *const libc::c_char,
        fout: *mut FILE,
        diffmt: bool,
    ) -> bool;
    fn putstring(out: *mut FILE, s: cbuf, log: bool);
    fn checkid(id: *const libc::c_char, delimiter: libc::c_int) -> *const libc::c_char;
    fn checksym(sym: *const libc::c_char, delimiter: libc::c_int) -> *const libc::c_char;
    fn countnumflds(s: *const libc::c_char) -> libc::c_int;
    fn take(count: size_t, ref_0: *const libc::c_char) -> cbuf;
    fn cmpnum(num1: *const libc::c_char, num2: *const libc::c_char) -> libc::c_int;
    fn cmpnumfld(
        num1: *const libc::c_char,
        num2: *const libc::c_char,
        fld: libc::c_int,
    ) -> libc::c_int;
    fn compartial(
        num1: *const libc::c_char,
        num2: *const libc::c_char,
        length: libc::c_int,
    ) -> libc::c_int;
    fn gr_revno(revno: *const libc::c_char, store: *mut *mut wlink) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const libc::c_char, fp: *mut fro) -> bool;
    fn tiprev() -> *const libc::c_char;
    fn tmpfile() -> *mut FILE;
    fn rewind(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn zone_set(s: *const libc::c_char);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn redefined(c: libc::c_int);
    fn parse_revpairs(
        option: libc::c_char,
        arg: *mut libc::c_char,
        data: *mut libc::c_void,
        put: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_char,
                bool,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn set_empty_log_message(cb: *mut cbuf);
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn complain(fmt: *const libc::c_char, _: ...);
    fn diagnose(fmt: *const libc::c_char, _: ...);
    static mut exit_failure: libc::c_int;
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accumulate_range(
        divvy: *mut divvy,
        beg: *const libc::c_char,
        end: *const libc::c_char,
    );
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn extend(tp: *mut link, x: *const libc::c_void, to: *mut divvy) -> *mut link;
    fn nosetid();
    fn caller_login_p(login: *const libc::c_char) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_on(delta: *const delta) -> *const rcslock;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn addlock_maybe(delta: *mut delta, selfsame: bool, verbose: bool) -> libc::c_int;
    fn Ozclose(p: *mut *mut FILE);
    fn aflush(f: *mut FILE);
    fn newline(f: *mut FILE);
    fn maketemp(n: libc::c_int) -> *const libc::c_char;
    fn tempunlink();
    fn dirtempunlink();
    fn fro_zclose(p: *mut *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_trundling(sequential: bool, f: *mut fro);
    fn fro_spew_partial(to: *mut FILE, f: *mut fro, r: *mut range);
    fn string_from_atat(space: *mut divvy, atat: *const atat) -> cbuf;
    fn atat_put(to: *mut FILE, atat: *const atat);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range {
    pub beg: off_t,
    pub end: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_log {
    pub revno: *const libc::c_char,
    pub message: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_state {
    pub revno: *const libc::c_char,
    pub status: *const libc::c_char,
}
pub type changeaccess = libc::c_uint;
pub const erase: changeaccess = 1;
pub const append: changeaccess = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chaccess {
    pub login: *const libc::c_char,
    pub command: changeaccess,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delrevpair {
    pub strt: *const libc::c_char,
    pub end: *const libc::c_char,
    pub code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adminstuff {
    pub rv: libc::c_int,
    pub deltas: *mut wlink,
    pub suppress_mail: bool,
    pub lockhead: bool,
    pub unlockcaller: bool,
    pub newlocks: *mut link,
    pub byelocks: *mut link,
    pub headstate: *const libc::c_char,
    pub headstate_changed: bool,
    pub states: link,
    pub tp_state: *mut link,
    pub accesses: link,
    pub tp_access: *mut link,
    pub assocs: link,
    pub tp_assoc: *mut link,
    pub logs: link,
    pub tp_log: *mut link,
    pub cuthead: *mut delta,
    pub cuttail: *mut delta,
    pub delstrt: *mut delta,
    pub delrev: delrevpair,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut rcs_blurb: [libc::c_char; 28] = unsafe {
    *::std::mem::transmute::<
        &[u8; 28],
        &[libc::c_char; 28],
    >(b"Change RCS file attributes.\0")
};
static mut rcs_help: [libc::c_char; 2044] = unsafe {
    *::std::mem::transmute::<
        &[u8; 2044],
        &[libc::c_char; 2044],
    >(
        b"[options] file ...\nOptions:\n  -i              Create and initialize a new RCS file.\n  -L              Set strict locking.\n  -U              Set non-strict locking.\n  -M              Don't send mail when breaking someone else's lock.\n  -T              Preserve the modification time on the\n                  RCS file unless a revision is removed.\n  -I              Interactive.\n  -q              Quiet mode.\n  -aLOGINS        Append LOGINS (comma-separated) to access-list.\n  -e[LOGINS]      Erase LOGINS (all if unspecified) from access-list.\n  -AFILENAME      Append access-list of FILENAME to current access-list.\n  -b[REV]         Set default branch to that of REV or\n                  highest branch on trunk if REV is omitted.\n  -l[REV]         Lock revision REV.\n  -u[REV]         Unlock revision REV.\n  -cSTRING        Set comment leader to STRING; don't use: obsolete.\n  -kSUBST         Set default keyword substitution to SUBST (see co(1)).\n  -mREV:MSG       Replace REV's log message with MSG.\n  -nNAME[:[REV]]  If :REV is omitted, delete symbolic NAME.\n                  Otherwise, associate NAME with REV; NAME must be new.\n  -NNAME[:[REV]]  Like -n, but overwrite any previous assignment.\n  -oRANGE         Outdate revisions in RANGE:\n                    REV       -- single revision\n                    BR        -- latest revision on branch BR\n                    REV1:REV2 -- REV1 to REV2 on same branch\n                    :REV      -- beginning of branch to REV\n                    REV:      -- REV to end of branch\n  -sSTATE[:REV]   Set state of REV to STATE.\n  -t-TEXT         Set description in RCS file to TEXT.\n  -tFILENAME      Set description in RCS file to contents of FILENAME.\n  -V              Obsolete; do not use.\n  -VN             Emulate RCS version N.\n  -xSUFF          Specify SUFF as a slash-separated list of suffixes\n                  used to identify RCS file names.\n  -zZONE          No effect; included for compatibility with other commands.\n\nREV defaults to the latest revision on the default branch.\n\0",
    )
};
static mut ks_ws_comma: [libc::c_char; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b" \n\t,\0")
};
unsafe extern "C" fn cleanup(mut exitstatus: *mut libc::c_int) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    Ozclose(&mut (*top).flow.res);
    ORCSclose();
    dirtempunlink();
}
unsafe extern "C" fn getassoclst(mut dc: *mut adminstuff, mut sp: *mut libc::c_char) {
    let fresh0 = sp;
    sp = sp.offset(1);
    let mut option: libc::c_char = *fresh0;
    let mut ud: *mut u_symdef = 0 as *mut u_symdef;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut c: libc::c_int = *sp as libc::c_int;
    let mut tp: *mut *mut link = &mut (*dc).tp_assoc;
    if (*tp).is_null() {
        *tp = &mut (*dc).assocs;
    }
    while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
        sp = sp.offset(1);
        c = *sp as libc::c_int;
    }
    accumulate_range(plexus, sp, checksym(sp, ':' as i32));
    name = finish_string(plexus, &mut len);
    sp = sp.offset(len as isize);
    c = *sp as libc::c_int;
    while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
        sp = sp.offset(1);
        c = *sp as libc::c_int;
    }
    if c != ':' as i32 && c != '\0' as i32 {
        generic_error(
            0 as *const libc::c_char,
            b"invalid string `%s' after option `-%c'\0" as *const u8
                as *const libc::c_char,
            sp,
            option as libc::c_int,
        );
        return;
    }
    ud = zlloc(
        plexus,
        (::std::mem::size_of::<u_symdef>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut u_symdef;
    (*ud).u.meaningful = name;
    (*ud).override_0 = 'N' as i32 == option as libc::c_int;
    if c == '\0' as i32 {
        (*ud).u.underlying = 0 as *const libc::c_char;
    } else {
        sp = sp.offset(1);
        c = *sp as libc::c_int;
        while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
            sp = sp.offset(1);
            c = *sp as libc::c_int;
        }
        (*ud).u.underlying = sp;
    }
    *tp = extend(*tp, ud as *const libc::c_void, plexus);
}
unsafe extern "C" fn getchaccess(
    mut dc: *mut adminstuff,
    mut login: *const libc::c_char,
    mut command: changeaccess,
) {
    let mut ch: *mut chaccess = 0 as *mut chaccess;
    let mut tp: *mut *mut link = &mut (*dc).tp_access;
    if (*tp).is_null() {
        *tp = &mut (*dc).accesses;
    }
    ch = zlloc(
        plexus,
        (::std::mem::size_of::<chaccess>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut chaccess;
    (*ch).login = login;
    (*ch).command = command;
    *tp = extend(*tp, ch as *const libc::c_void, plexus);
}
unsafe extern "C" fn getaccessor(
    mut dc: *mut adminstuff,
    mut argv: *mut libc::c_char,
    mut command: changeaccess,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut who: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut any: bool = false;
    any = 0 as libc::c_int != 0;
    s = argv.offset(2 as libc::c_int as isize);
    loop {
        who = strtok_r(s, ks_ws_comma.as_ptr(), &mut save);
        if who.is_null() {
            break;
        }
        checkid(who, 0 as libc::c_int);
        getchaccess(dc, who, command);
        any = 1 as libc::c_int != 0;
        s = 0 as *mut libc::c_char;
    }
    if !any {
        let mut current_block_6: u64;
        match command as libc::c_uint {
            0 => {
                generic_error(
                    0 as *const libc::c_char,
                    b"missing login name after option -a\0" as *const u8
                        as *const libc::c_char,
                );
                current_block_6 = 696700231052356982;
            }
            1 => {
                current_block_6 = 696700231052356982;
            }
            _ => {
                current_block_6 = 17216689946888361452;
            }
        }
        match current_block_6 {
            696700231052356982 => {
                getchaccess(dc, 0 as *const libc::c_char, command);
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn getmessage(mut dc: *mut adminstuff, mut option: *mut libc::c_char) {
    let mut um: *mut u_log = 0 as *mut u_log;
    let mut cb: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tp: *mut *mut link = &mut (*dc).tp_log;
    if (*tp).is_null() {
        *tp = &mut (*dc).logs;
    }
    m = strchr(option, ':' as i32);
    if m.is_null() {
        generic_error(
            0 as *const libc::c_char,
            b"-m option lacks %s\0" as *const u8 as *const libc::c_char,
            ks_revno.as_ptr(),
        );
        return;
    }
    let fresh1 = m;
    m = m.offset(1);
    *fresh1 = '\0' as i32 as libc::c_char;
    cb = cleanlogmsg(m, strlen(m));
    if cb.size == 0 {
        set_empty_log_message(&mut cb);
    }
    um = zlloc(
        plexus,
        (::std::mem::size_of::<u_log>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut u_log;
    (*um).revno = option;
    (*um).message = cb;
    *tp = extend(*tp, um as *const libc::c_void, plexus);
}
unsafe extern "C" fn getstates(mut dc: *mut adminstuff, mut sp: *mut libc::c_char) {
    let mut temp: *const libc::c_char = 0 as *const libc::c_char;
    let mut us: *mut u_state = 0 as *mut u_state;
    let mut c: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut tp: *mut *mut link = &mut (*dc).tp_state;
    if (*tp).is_null() {
        *tp = &mut (*dc).states;
    }
    loop {
        sp = sp.offset(1);
        c = *sp as libc::c_int;
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32) {
            break;
        }
    }
    temp = checkid(sp, ':' as i32);
    accumulate_range(plexus, sp, temp);
    temp = finish_string(plexus, &mut len);
    sp = sp.offset(len as isize);
    c = *sp as libc::c_int;
    while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
        sp = sp.offset(1);
        c = *sp as libc::c_int;
    }
    if c == '\0' as i32 {
        (*dc).headstate_changed = 1 as libc::c_int != 0;
        (*dc).headstate = temp;
        return;
    } else if c != ':' as i32 {
        generic_error(
            0 as *const libc::c_char,
            b"missing ':' after state in option -s\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    loop {
        sp = sp.offset(1);
        c = *sp as libc::c_int;
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32) {
            break;
        }
    }
    us = zlloc(
        plexus,
        (::std::mem::size_of::<u_state>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut u_state;
    (*us).status = temp;
    (*us).revno = sp;
    *tp = extend(*tp, us as *const libc::c_void, plexus);
}
unsafe extern "C" fn putdelrev(
    mut b: *const libc::c_char,
    mut e: *const libc::c_char,
    mut sawsep: bool,
    mut data: *mut libc::c_void,
) {
    let mut dc: *mut adminstuff = data as *mut adminstuff;
    if !((*dc).delrev.strt).is_null() || !((*dc).delrev.end).is_null() {
        generic_warn(
            0 as *const libc::c_char,
            b"ignoring spurious `-o' range `%s:%s'\0" as *const u8
                as *const libc::c_char,
            if !b.is_null() {
                b
            } else {
                b"(unspecified)\0" as *const u8 as *const libc::c_char
            },
            if !e.is_null() {
                e
            } else {
                b"(unspecified)\0" as *const u8 as *const libc::c_char
            },
        );
        return;
    }
    if !sawsep {
        (*dc).delrev.strt = b;
        (*dc).delrev.code = 0 as libc::c_int;
    } else if b.is_null() || *b.offset(0 as libc::c_int as isize) == 0 {
        (*dc).delrev.strt = e;
        (*dc).delrev.code = 1 as libc::c_int;
    } else if *e.offset(0 as libc::c_int as isize) == 0 {
        (*dc).delrev.strt = b;
        (*dc).delrev.code = 2 as libc::c_int;
    } else {
        (*dc).delrev.strt = b;
        (*dc).delrev.end = e;
        (*dc).delrev.code = 3 as libc::c_int;
    };
}
unsafe extern "C" fn scanlogtext(
    mut dc: *mut adminstuff,
    mut es: *mut editstuff,
    mut ls: *mut *mut wlink,
    mut delta: *mut delta,
    mut edit: bool,
) {
    let mut nextdelta: *const delta = 0 as *const delta;
    let mut from: *mut fro = (*top).flow.from;
    let mut to: *mut FILE = 0 as *mut FILE;
    let mut log: *mut atat = 0 as *mut atat;
    let mut text: *mut atat = 0 as *mut atat;
    let mut range: range = range { beg: 0, end: 0 };
    loop {
        if (*ls).is_null() {
            return;
        }
        (*top).flow.to = 0 as *mut FILE;
        to = (*top).flow.to;
        nextdelta = (**ls).entry as *const delta;
        log = (*nextdelta).log;
        text = (*nextdelta).text;
        range.beg = (*nextdelta).neck;
        if (*nextdelta).selector {
            (*top).flow.to = (*top).flow.rewr;
            to = (*top).flow.to;
            range.end = (*log).beg;
            fro_spew_partial(to, from, &mut range);
        }
        if nextdelta == (*dc).cuttail as *const delta {
            if ((*delta).pretty_log.string).is_null() {
                (*delta).pretty_log = string_from_atat(single, log);
                (*delta)
                    .pretty_log = cleanlogmsg(
                    (*delta).pretty_log.string,
                    (*delta).pretty_log.size,
                );
            }
        } else if !((*nextdelta).pretty_log.string).is_null()
            && (*nextdelta).selector as libc::c_int != 0
        {
            putstring(to, (*nextdelta).pretty_log, 1 as libc::c_int != 0);
            newline(to);
        } else if !to.is_null() {
            atat_put(to, log);
        }
        range
            .beg = *((*log).holes)
            .as_mut_ptr()
            .offset(
                ((*log).count).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) + 2 as libc::c_int as libc::c_long;
        range.end = (*text).beg;
        if !to.is_null() {
            fro_spew_partial(to, from, &mut range);
        }
        if delta == nextdelta as *mut delta {
            break;
        }
        if !to.is_null() {
            atat_put(to, text);
        }
        *ls = (**ls).next;
    }
    fro_move(from, range.end);
    if edit {
        editstring(es, text, 0 as *const delta);
    } else {
        enterstring(es, text);
    };
}
unsafe extern "C" fn rmnewlocklst(
    mut dc: *mut adminstuff,
    mut which: *const libc::c_char,
) -> *mut link {
    let mut pt: *mut link = 0 as *mut link;
    let mut pre: *mut *mut link = 0 as *mut *mut link;
    pre = &mut (*dc).newlocks;
    loop {
        pt = *pre;
        if pt.is_null() {
            break;
        }
        if strcmp((*pt).entry as *const libc::c_char, which) != 0 {
            pre = &mut (*pt).next;
        } else {
            *pre = (*pt).next;
        }
    }
    return *pre;
}
unsafe extern "C" fn doaccess(mut dc: *mut adminstuff) -> bool {
    let mut changed: bool = 0 as libc::c_int != 0;
    let mut ls: *mut link = 0 as *mut link;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    ls = (*dc).accesses.next;
    while !ls.is_null() {
        let mut ch: *const chaccess = (*ls).entry as *const chaccess;
        let mut login: *const libc::c_char = (*ch).login;
        match (*ch).command as libc::c_uint {
            1 => {
                if login.is_null() {
                    if !((*(*top).repository.r).access).is_null() {
                        (*(*top).repository.r).access = 0 as *mut link;
                        changed = 1 as libc::c_int != 0;
                    }
                } else {
                    box_0.next = (*(*top).repository.r).access;
                    tp = &mut box_0;
                    while !((*tp).next).is_null() {
                        if strcmp(login, (*(*tp).next).entry as *const libc::c_char) == 0
                        {
                            (*tp).next = (*(*tp).next).next;
                            changed = 1 as libc::c_int != 0;
                            (*(*top).repository.r).access = box_0.next;
                            break;
                        } else {
                            tp = (*tp).next;
                        }
                    }
                }
            }
            0 => {
                box_0.next = (*(*top).repository.r).access;
                tp = &mut box_0;
                while !((*tp).next).is_null() {
                    if strcmp(login, (*(*tp).next).entry as *const libc::c_char) == 0 {
                        break;
                    }
                    tp = (*tp).next;
                }
                if ((*tp).next).is_null() {
                    extend(tp, login as *const libc::c_void, single);
                    changed = 1 as libc::c_int != 0;
                    (*(*top).repository.r).access = box_0.next;
                }
            }
            _ => {}
        }
        ls = (*ls).next;
    }
    return changed;
}
unsafe extern "C" fn sendmail(
    mut Delta: *const libc::c_char,
    mut who: *const libc::c_char,
    mut suppress_mail: bool,
) -> bool {
    complain(
        b"Revision %s is already locked by %s.\n\0" as *const u8 as *const libc::c_char,
        Delta,
        who,
    );
    if suppress_mail {
        return 1 as libc::c_int != 0;
    }
    if !yesorno(
        0 as libc::c_int != 0,
        b"Do you want to break the lock\0" as *const u8 as *const libc::c_char,
    ) {
        return 0 as libc::c_int != 0;
    }
    generic_warn(
        0 as *const libc::c_char,
        b"Mail notification of broken locks is not available.\0" as *const u8
            as *const libc::c_char,
    );
    generic_warn(
        0 as *const libc::c_char,
        b"Please tell `%s' why you broke the lock.\0" as *const u8
            as *const libc::c_char,
        who,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn breaklock(
    mut delta: *const delta,
    mut suppress_mail: bool,
) -> bool {
    let mut rl: *const rcslock = 0 as *const rcslock;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut num: *const libc::c_char = 0 as *const libc::c_char;
    let mut before: *const libc::c_char = 0 as *const libc::c_char;
    num = (*delta).num;
    box_0.next = (*(*top).repository.r).locks;
    tp = lock_memq(&mut box_0, 0 as libc::c_int != 0, delta as *const libc::c_void);
    if tp.is_null() {
        generic_error(
            (*top).repository.filename,
            b"no lock set on revision %s\0" as *const u8 as *const libc::c_char,
            num,
        );
        return 0 as libc::c_int != 0;
    }
    rl = (*(*tp).next).entry as *const rcslock;
    before = (*rl).login;
    if !caller_login_p(before) && !sendmail(num, before, suppress_mail) {
        generic_error(
            (*top).repository.filename,
            b"revision %s still locked by %s\0" as *const u8 as *const libc::c_char,
            num,
            before,
        );
        return 0 as libc::c_int != 0;
    }
    diagnose(b"%s unlocked\0" as *const u8 as *const libc::c_char, num);
    lock_drop(&mut box_0, tp);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn searchcutpt(
    mut dc: *mut adminstuff,
    mut object: *const libc::c_char,
    mut length: libc::c_int,
    mut store: *mut wlink,
) -> *mut delta {
    let mut delta: *mut delta = 0 as *mut delta;
    (*dc).cuthead = 0 as *mut delta;
    loop {
        delta = (*store).entry as *mut delta;
        if !(compartial((*delta).num, object, length) != 0) {
            break;
        }
        (*dc).cuthead = delta;
        store = (*store).next;
    }
    return delta;
}
unsafe extern "C" fn branchpoint(mut strt: *mut delta, mut tail: *mut delta) -> bool {
    let mut pt: *mut delta = 0 as *mut delta;
    pt = strt;
    while pt != tail {
        if !((*pt).branches).is_null() {
            generic_error(
                (*top).repository.filename,
                b"can't remove branch point %s\0" as *const u8 as *const libc::c_char,
                (*pt).num,
            );
            return 1 as libc::c_int != 0;
        }
        if !(lock_on(pt)).is_null() {
            generic_error(
                (*top).repository.filename,
                b"can't remove locked revision %s\0" as *const u8 as *const libc::c_char,
                (*pt).num,
            );
            return 1 as libc::c_int != 0;
        }
        (*pt).selector = 0 as libc::c_int != 0;
        diagnose(
            b"deleting revision %s\0" as *const u8 as *const libc::c_char,
            (*pt).num,
        );
        pt = (*pt).ilk;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn removerevs(mut dc: *mut adminstuff) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    let mut target2: *mut delta = 0 as *mut delta;
    let mut temp: *mut delta = 0 as *mut delta;
    let mut ls: *mut wlink = 0 as *mut wlink;
    let mut length: libc::c_int = 0;
    let mut different: bool = false;
    if !fully_numeric(&mut numrev, (*dc).delrev.strt, 0 as *mut fro) {
        return 0 as libc::c_int != 0;
    }
    target = gr_revno(numrev.string, &mut ls);
    if target.is_null() {
        return 0 as libc::c_int != 0;
    }
    different = !(0 as libc::c_int == cmpnum((*target).num, numrev.string));
    length = countnumflds(numrev.string);
    if (*dc).delrev.code == 0 as libc::c_int {
        if length & 1 as libc::c_int != 0 {
            temp = searchcutpt(dc, (*target).num, length + 1 as libc::c_int, ls);
        } else if different {
            generic_error(
                (*top).repository.filename,
                b"Revision %s doesn't exist.\0" as *const u8 as *const libc::c_char,
                numrev.string,
            );
            return 0 as libc::c_int != 0;
        } else {
            temp = searchcutpt(dc, numrev.string, length, ls);
        }
        (*dc).cuttail = (*target).ilk;
        if branchpoint(temp, (*dc).cuttail) {
            (*dc).cuttail = 0 as *mut delta;
            return 0 as libc::c_int != 0;
        }
        (*dc).delstrt = temp;
        return 1 as libc::c_int != 0;
    }
    if length & 1 as libc::c_int != 0 {
        generic_error(
            (*top).repository.filename,
            b"invalid branch range %s after -o\0" as *const u8 as *const libc::c_char,
            numrev.string,
        );
        return 0 as libc::c_int != 0;
    }
    if (*dc).delrev.code == 1 as libc::c_int {
        if length > 2 as libc::c_int {
            temp = searchcutpt(dc, (*target).num, length - 1 as libc::c_int, ls);
            (*dc).cuttail = (*target).ilk;
        } else {
            temp = searchcutpt(dc, (*target).num, length, ls);
            (*dc).cuttail = target;
            while !((*dc).cuttail).is_null()
                && 0 as libc::c_int
                    == cmpnumfld((*target).num, (*(*dc).cuttail).num, 1 as libc::c_int)
            {
                (*dc).cuttail = (*(*dc).cuttail).ilk;
            }
        }
        if branchpoint(temp, (*dc).cuttail) {
            (*dc).cuttail = 0 as *mut delta;
            return 0 as libc::c_int != 0;
        }
        (*dc).delstrt = temp;
        return 1 as libc::c_int != 0;
    }
    if (*dc).delrev.code == 2 as libc::c_int {
        if length == 2 as libc::c_int {
            temp = searchcutpt(dc, (*target).num, 1 as libc::c_int, ls);
            (*dc)
                .cuttail = if different as libc::c_int != 0 {
                target
            } else {
                (*target).ilk
            };
        } else {
            if different {
                (*dc).cuthead = target;
                temp = (*target).ilk;
                if temp.is_null() {
                    return 0 as libc::c_int != 0;
                }
            } else {
                temp = searchcutpt(dc, (*target).num, length, ls);
            }
            gr_revno((take(0 as libc::c_int as size_t, (*temp).num)).string, &mut ls);
        }
        if branchpoint(temp, (*dc).cuttail) {
            (*dc).cuttail = 0 as *mut delta;
            return 0 as libc::c_int != 0;
        }
        (*dc).delstrt = temp;
        return 1 as libc::c_int != 0;
    }
    if !fully_numeric(&mut numrev, (*dc).delrev.end, 0 as *mut fro) {
        return 0 as libc::c_int != 0;
    }
    if length != countnumflds(numrev.string)
        || length > 2 as libc::c_int
            && compartial(numrev.string, (*target).num, length - 1 as libc::c_int) != 0
    {
        generic_error(
            (*top).repository.filename,
            b"invalid revision range %s-%s\0" as *const u8 as *const libc::c_char,
            (*target).num,
            numrev.string,
        );
        return 0 as libc::c_int != 0;
    }
    target2 = gr_revno(numrev.string, &mut ls);
    if target2.is_null() {
        return 0 as libc::c_int != 0;
    }
    if length > 2 as libc::c_int {
        if (0 as libc::c_int) < cmpnum((*target).num, (*target2).num) {
            different = !(0 as libc::c_int == cmpnum((*target2).num, numrev.string));
            temp = target;
            target = target2;
            target2 = temp;
        }
        if different {
            if 0 as libc::c_int == cmpnum((*target).num, (*target2).num) {
                generic_error(
                    (*top).repository.filename,
                    b"Revisions %s-%s don't exist.\0" as *const u8
                        as *const libc::c_char,
                    (*dc).delrev.strt,
                    (*dc).delrev.end,
                );
                return 0 as libc::c_int != 0;
            }
            (*dc).cuthead = target;
            temp = (*target).ilk;
        } else {
            temp = searchcutpt(dc, (*target).num, length, ls);
        }
        (*dc).cuttail = (*target2).ilk;
    } else {
        if 0 as libc::c_int > cmpnum((*target).num, (*target2).num) {
            temp = target;
            target = target2;
            target2 = temp;
        } else {
            different = !(0 as libc::c_int == cmpnum((*target2).num, numrev.string));
        }
        if different {
            if 0 as libc::c_int == cmpnum((*target).num, (*target2).num) {
                generic_error(
                    (*top).repository.filename,
                    b"Revisions %s-%s don't exist.\0" as *const u8
                        as *const libc::c_char,
                    (*dc).delrev.strt,
                    (*dc).delrev.end,
                );
                return 0 as libc::c_int != 0;
            }
            (*dc).cuttail = target2;
        } else {
            (*dc).cuttail = (*target2).ilk;
        }
        temp = searchcutpt(dc, (*target).num, length, ls);
    }
    if branchpoint(temp, (*dc).cuttail) {
        (*dc).cuttail = 0 as *mut delta;
        return 0 as libc::c_int != 0;
    }
    (*dc).delstrt = temp;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn doassoc(mut dc: *mut adminstuff) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut changed: bool = 0 as libc::c_int != 0;
    let mut cur: *mut link = (*dc).assocs.next;
    while !cur.is_null() {
        let mut u: *const u_symdef = (*cur).entry as *const u_symdef;
        let mut ssymbol: *const libc::c_char = (*u).u.meaningful;
        let mut under: *const libc::c_char = (*u).u.underlying;
        if under.is_null() {
            let mut box_0: link = link {
                entry: 0 as *const libc::c_void,
                next: 0 as *mut link,
            };
            let mut tp: *mut link = 0 as *mut link;
            let mut d: *const symdef = 0 as *const symdef;
            box_0.next = (*(*top).repository.r).symbols;
            tp = &mut box_0;
            while !((*tp).next).is_null() {
                d = (*(*tp).next).entry as *const symdef;
                if strcmp(ssymbol, (*d).meaningful) == 0 {
                    (*tp).next = (*(*tp).next).next;
                    changed = 1 as libc::c_int != 0;
                    break;
                } else {
                    tp = (*tp).next;
                }
            }
            (*(*top).repository.r).symbols = box_0.next;
            if d.is_null() {
                generic_warn(
                    (*top).repository.filename,
                    b"can't delete nonexisting symbol %s\0" as *const u8
                        as *const libc::c_char,
                    ssymbol,
                );
            }
        } else {
            if *under.offset(0 as libc::c_int as isize) != 0 {
                p = if fully_numeric(&mut numrev, under, 0 as *mut fro) as libc::c_int
                    != 0
                {
                    numrev.string
                } else {
                    0 as *const libc::c_char
                };
            } else {
                p = tiprev();
                if p.is_null() {
                    generic_error(
                        (*top).repository.filename,
                        b"no latest revision to associate with symbol %s\0" as *const u8
                            as *const libc::c_char,
                        ssymbol,
                    );
                }
            }
            if !p.is_null() {
                changed = (changed as libc::c_int
                    | addsymbol(p, ssymbol, (*u).override_0)) != 0;
            }
        }
        cur = (*cur).next;
    }
    return changed;
}
unsafe extern "C" fn setlock(
    mut dc: *mut adminstuff,
    mut rev: *const libc::c_char,
) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    let mut r: libc::c_int = 0;
    if fully_numeric(&mut numrev, rev, 0 as *mut fro) {
        target = gr_revno(numrev.string, &mut (*dc).deltas);
        if !target.is_null() {
            if countnumflds(numrev.string) & 1 as libc::c_int == 0
                && !(0 as libc::c_int == cmpnum((*target).num, numrev.string))
            {
                generic_error(
                    (*top).repository.filename,
                    b"can't lock nonexisting revision %s\0" as *const u8
                        as *const libc::c_char,
                    numrev.string,
                );
            } else {
                r = addlock_maybe(target, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
                if r < 0 as libc::c_int
                    && breaklock(target, (*dc).suppress_mail) as libc::c_int != 0
                {
                    r = addlock_maybe(
                        target,
                        0 as libc::c_int != 0,
                        1 as libc::c_int != 0,
                    );
                }
                if 0 as libc::c_int <= r {
                    if r != 0 {
                        diagnose(
                            b"%s locked\0" as *const u8 as *const libc::c_char,
                            (*target).num,
                        );
                    }
                    return r != 0;
                }
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn dolocks(mut dc: *mut adminstuff) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut lockpt: *const link = 0 as *const link;
    let mut target: *mut delta = 0 as *mut delta;
    let mut tip: *mut delta = (*top).repository.tip;
    let mut changed: bool = 0 as libc::c_int != 0;
    let mut bye: *const libc::c_char = 0 as *const libc::c_char;
    if (*dc).unlockcaller {
        if !tip.is_null() {
            let mut locks: *mut link = (*(*top).repository.r).locks;
            if !locks.is_null() {
                match findlock(1 as libc::c_int != 0, &mut target) {
                    0 => {
                        let mut rl: *const rcslock = (*locks).entry as *const rcslock;
                        changed = (changed as libc::c_int
                            | breaklock((*rl).delta, (*dc).suppress_mail) as libc::c_int) != 0;
                    }
                    1 => {
                        diagnose(
                            b"%s unlocked\0" as *const u8 as *const libc::c_char,
                            (*target).num,
                        );
                        changed = 1 as libc::c_int != 0;
                    }
                    _ => {}
                }
            } else {
                generic_warn(
                    (*top).repository.filename,
                    b"No locks are set.\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            generic_warn(
                (*top).repository.filename,
                b"can't unlock an empty tree\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    lockpt = (*dc).byelocks;
    while !lockpt.is_null() {
        bye = (*lockpt).entry as *const libc::c_char;
        if fully_numeric(&mut numrev, bye, 0 as *mut fro) {
            target = gr_revno(numrev.string, &mut (*dc).deltas);
            if !target.is_null() {
                if countnumflds(numrev.string) & 1 as libc::c_int == 0
                    && !(0 as libc::c_int == cmpnum((*target).num, numrev.string))
                {
                    generic_error(
                        (*top).repository.filename,
                        b"can't unlock nonexisting revision %s\0" as *const u8
                            as *const libc::c_char,
                        bye,
                    );
                } else {
                    changed = (changed as libc::c_int
                        | breaklock(target, (*dc).suppress_mail) as libc::c_int) != 0;
                }
            }
        }
        lockpt = (*lockpt).next;
    }
    lockpt = (*dc).newlocks;
    while !lockpt.is_null() {
        changed = (changed as libc::c_int
            | setlock(dc, (*lockpt).entry as *const libc::c_char) as libc::c_int) != 0;
        lockpt = (*lockpt).next;
    }
    if (*dc).lockhead {
        let mut defbr: *const libc::c_char = (*(*top).repository.r).branch;
        if !defbr.is_null() {
            changed = (changed as libc::c_int | setlock(dc, defbr) as libc::c_int) != 0;
        } else if !tip.is_null() {
            changed = (changed as libc::c_int | setlock(dc, (*tip).num) as libc::c_int) != 0;
        } else {
            generic_warn(
                (*top).repository.filename,
                b"can't lock an empty tree\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return changed;
}
unsafe extern "C" fn domessages(mut dc: *mut adminstuff) -> bool {
    let mut target: *mut delta = 0 as *mut delta;
    let mut changed: bool = 0 as libc::c_int != 0;
    let mut ls: *mut link = (*dc).logs.next;
    while !ls.is_null() {
        let mut um: *const u_log = (*ls).entry as *const u_log;
        let mut numrev: cbuf = cbuf {
            string: 0 as *const libc::c_char,
            size: 0,
        };
        if fully_numeric(&mut numrev, (*um).revno, 0 as *mut fro) as libc::c_int != 0
            && {
                target = gr_revno(numrev.string, &mut (*dc).deltas);
                !target.is_null()
            }
        {
            (*target).pretty_log = (*um).message;
            changed = 1 as libc::c_int != 0;
        }
        ls = (*ls).next;
    }
    return changed;
}
unsafe extern "C" fn rcs_setstate(
    mut dc: *mut adminstuff,
    mut rev: *const libc::c_char,
    mut status: *const libc::c_char,
) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    if fully_numeric(&mut numrev, rev, 0 as *mut fro) {
        target = gr_revno(numrev.string, &mut (*dc).deltas);
        if !target.is_null() {
            if countnumflds(numrev.string) & 1 as libc::c_int == 0
                && !(0 as libc::c_int == cmpnum((*target).num, numrev.string))
            {
                generic_error(
                    (*top).repository.filename,
                    b"can't set state of nonexisting revision %s\0" as *const u8
                        as *const libc::c_char,
                    numrev.string,
                );
            } else if strcmp((*target).state, status) != 0 {
                (*target).state = status;
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn buildeltatext(
    mut dc: *mut adminstuff,
    mut es: *mut editstuff,
    mut ls: *mut *mut wlink,
    mut deltas: *const wlink,
) -> bool {
    let mut fcut: *mut FILE = 0 as *mut FILE;
    let mut frew: *mut FILE = (*top).flow.rewr;
    fcut = 0 as *mut FILE;
    (*(*dc).cuttail).selector = 0 as libc::c_int != 0;
    scanlogtext(dc, es, ls, (*deltas).entry as *mut delta, 0 as libc::c_int != 0);
    if !((*dc).cuthead).is_null() {
        fcut = tmpfile();
        if fcut.is_null() {
            fatal_sys(b"tmpfile\0" as *const u8 as *const libc::c_char);
        }
        while (*deltas).entry != (*dc).cuthead as *mut libc::c_void {
            *ls = (**ls).next;
            deltas = (*deltas).next;
            scanlogtext(
                dc,
                es,
                ls,
                (*deltas).entry as *mut delta,
                1 as libc::c_int != 0,
            );
        }
        snapshotedit(es, fcut);
        rewind(fcut);
        aflush(fcut);
    }
    while (*deltas).entry != (*dc).cuttail as *mut libc::c_void {
        *ls = (**ls).next;
        deltas = (*deltas).next;
        scanlogtext(dc, es, ls, (*deltas).entry as *mut delta, 1 as libc::c_int != 0);
    }
    finishedit(es, 0 as *const delta, 0 as *mut FILE, 1 as libc::c_int != 0);
    Ozclose(&mut (*top).flow.res);
    if !fcut.is_null() {
        let mut diffname: *const libc::c_char = maketemp(0 as libc::c_int);
        let mut diffv: [*const libc::c_char; 6] = [0 as *const libc::c_char; 6];
        let mut diffp: *mut *const libc::c_char = diffv.as_mut_ptr();
        diffp = diffp.offset(1);
        *diffp = prog_diff.as_ptr();
        diffp = diffp.offset(1);
        *diffp = diff_flags.as_ptr();
        if 0 as libc::c_int != 0 && (*top).behavior.kws == kwsub_b as libc::c_int {
            diffp = diffp.offset(1);
            *diffp = b"--binary\0" as *const u8 as *const libc::c_char;
        }
        diffp = diffp.offset(1);
        *diffp = b"-\0" as *const u8 as *const libc::c_char;
        diffp = diffp.offset(1);
        *diffp = (*top).flow.result;
        diffp = diffp.offset(1);
        *diffp = 0 as *const libc::c_char;
        if 2 as libc::c_int == runv(fileno(fcut), diffname, diffv.as_mut_ptr()) {
            generic_fatal(
                (*top).repository.filename,
                b"diff failed\0" as *const u8 as *const libc::c_char,
            );
        }
        Ozclose(&mut fcut);
        return putdtext((*dc).cuttail, diffname, frew, 1 as libc::c_int != 0);
    } else {
        return putdtext((*dc).cuttail, (*top).flow.result, frew, 0 as libc::c_int != 0)
    };
}
unsafe extern "C" fn buildtree(mut dc: *mut adminstuff) {
    let mut Delta: *mut delta = 0 as *mut delta;
    if !((*dc).cuthead).is_null() {
        if (*(*dc).cuthead).ilk == (*dc).delstrt {
            (*(*dc).cuthead).ilk = (*dc).cuttail;
        } else {
            let mut pt: *mut wlink = (*(*dc).cuthead).branches;
            let mut pre: *mut wlink = pt;
            while !pt.is_null() && (*pt).entry != (*dc).delstrt as *mut libc::c_void {
                pre = pt;
                pt = (*pt).next;
            }
            if !((*dc).cuttail).is_null() {
                (*pt).entry = (*dc).cuttail as *mut libc::c_void;
            } else if pt == pre {
                (*(*dc).cuthead).branches = (*pt).next;
            } else {
                (*pre).next = (*pt).next;
            }
        }
    } else {
        if ((*dc).cuttail).is_null() && !(*top).behavior.quiet {
            if !yesorno(
                0 as libc::c_int != 0,
                b"Do you really want to delete all revisions\0" as *const u8
                    as *const libc::c_char,
            ) {
                generic_error(
                    (*top).repository.filename,
                    b"No revision deleted\0" as *const u8 as *const libc::c_char,
                );
                Delta = (*dc).delstrt;
                while !Delta.is_null() {
                    (*Delta).selector = 1 as libc::c_int != 0;
                    Delta = (*Delta).ilk;
                }
                return;
            }
        }
        (*top).repository.tip = (*dc).cuttail;
    };
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: rcs_blurb.as_ptr(),
            help: rcs_help.as_ptr(),
            tyag: (1 as libc::c_int) << 3 as libc::c_int
                | ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int),
        };
        init
    }
};
unsafe extern "C" fn rcs_main(
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dc: adminstuff = adminstuff {
        rv: 0,
        deltas: 0 as *mut wlink,
        suppress_mail: false,
        lockhead: false,
        unlockcaller: false,
        newlocks: 0 as *mut link,
        byelocks: 0 as *mut link,
        headstate: 0 as *const libc::c_char,
        headstate_changed: false,
        states: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_state: 0 as *mut link,
        accesses: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_access: 0 as *mut link,
        assocs: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_assoc: 0 as *mut link,
        logs: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_log: 0 as *mut link,
        cuthead: 0 as *mut delta,
        cuttail: 0 as *mut delta,
        delstrt: 0 as *mut delta,
        delrev: delrevpair {
            strt: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            code: 0,
        },
    };
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut textfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut branchsym: *const libc::c_char = 0 as *const libc::c_char;
    let mut commsyml: *const libc::c_char = 0 as *const libc::c_char;
    let mut branchflag: bool = false;
    let mut initflag: bool = false;
    let mut textflag: bool = false;
    let mut changed: libc::c_int = 0;
    let mut expmode: libc::c_int = 0;
    let mut strictlock: bool = false;
    let mut strict_selected: bool = false;
    let mut Ttimeflag: bool = false;
    let mut keepRCStime: bool = false;
    let mut commsymlen: size_t = 0;
    let mut branchnum: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut boxlock: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tplock: *mut link = 0 as *mut link;
    let mut boxrm: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tprm: *mut link = 0 as *mut link;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut dc as *mut adminstuff as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<adminstuff>() as libc::c_ulong,
    );
    dc.rv = 0 as libc::c_int;
    nosetid();
    textfile = 0 as *mut libc::c_char;
    commsyml = textfile;
    branchsym = commsyml;
    strictlock = 0 as libc::c_int != 0;
    branchflag = strictlock;
    commsymlen = 0 as libc::c_int as size_t;
    boxlock.next = dc.newlocks;
    tplock = &mut boxlock;
    boxrm.next = dc.byelocks;
    tprm = &mut boxrm;
    expmode = -(1 as libc::c_int);
    textflag = 0 as libc::c_int != 0;
    initflag = textflag;
    strict_selected = 0 as libc::c_int != 0;
    Ttimeflag = 0 as libc::c_int != 0;
    if (1 as libc::c_int) < argc
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
    {
        generic_warn(
            0 as *const libc::c_char,
            b"No options were given; this usage is obsolescent.\0" as *const u8
                as *const libc::c_char,
        );
    }
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
        let mut current_block_83: u64;
        let fresh3 = a;
        a = a.offset(1);
        match *fresh3 as libc::c_int {
            105 => {
                initflag = 1 as libc::c_int != 0;
                current_block_83 = 16313536926714486912;
            }
            98 => {
                if branchflag {
                    redefined('b' as i32);
                }
                branchflag = 1 as libc::c_int != 0;
                branchsym = a;
                current_block_83 = 16313536926714486912;
            }
            99 => {
                if !commsyml.is_null() {
                    redefined('c' as i32);
                }
                commsyml = a;
                commsymlen = strlen(a);
                current_block_83 = 16313536926714486912;
            }
            97 => {
                getaccessor(&mut dc, *argv, append);
                current_block_83 = 16313536926714486912;
            }
            65 => {
                if *a == 0 {
                    generic_error(
                        0 as *const libc::c_char,
                        b"missing filename after -A\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    *argv = a;
                    if (0 as libc::c_int)
                        < pairnames(
                            1 as libc::c_int,
                            argv,
                            Some(
                                rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro,
                            ),
                            1 as libc::c_int != 0,
                            0 as libc::c_int != 0,
                        )
                    {
                        let mut ls: *mut link = (*(*top).repository.r).access;
                        while !ls.is_null() {
                            getchaccess(
                                &mut dc,
                                str_save((*ls).entry as *const libc::c_char),
                                append,
                            );
                            ls = (*ls).next;
                        }
                        fro_zclose(&mut (*top).flow.from);
                    }
                }
                current_block_83 = 16313536926714486912;
            }
            101 => {
                getaccessor(&mut dc, *argv, erase);
                current_block_83 = 16313536926714486912;
            }
            108 => {
                if *a == 0 {
                    dc.lockhead = 1 as libc::c_int != 0;
                } else {
                    tplock = extend(tplock, a as *const libc::c_void, plexus);
                }
                current_block_83 = 16313536926714486912;
            }
            117 => {
                if *a == 0 {
                    dc.unlockcaller = 1 as libc::c_int != 0;
                } else {
                    tprm = extend(tprm, a as *const libc::c_void, plexus);
                    dc.newlocks = boxlock.next;
                    tplock = rmnewlocklst(&mut dc, a);
                }
                current_block_83 = 16313536926714486912;
            }
            76 => {
                if strict_selected {
                    if !strictlock {
                        generic_warn(
                            0 as *const libc::c_char,
                            b"-U overridden by -L\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                strictlock = 1 as libc::c_int != 0;
                strict_selected = 1 as libc::c_int != 0;
                current_block_83 = 16313536926714486912;
            }
            85 => {
                if strict_selected {
                    if strictlock {
                        generic_warn(
                            0 as *const libc::c_char,
                            b"-L overridden by -U\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                strict_selected = 1 as libc::c_int != 0;
                current_block_83 = 16313536926714486912;
            }
            110 => {
                current_block_83 = 17179227041652854834;
            }
            78 => {
                current_block_83 = 17179227041652854834;
            }
            109 => {
                getmessage(&mut dc, a);
                current_block_83 = 16313536926714486912;
            }
            77 => {
                dc.suppress_mail = 1 as libc::c_int != 0;
                current_block_83 = 16313536926714486912;
            }
            111 => {
                if !(dc.delrev.strt).is_null() {
                    redefined('o' as i32);
                }
                if *a == 0 {
                    generic_error(
                        0 as *const libc::c_char,
                        b"missing revision range after -o\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    parse_revpairs(
                        'o' as i32 as libc::c_char,
                        (*argv).offset(2 as libc::c_int as isize),
                        &mut dc as *mut adminstuff as *mut libc::c_void,
                        Some(
                            putdelrev
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *const libc::c_char,
                                    bool,
                                    *mut libc::c_void,
                                ) -> (),
                        ),
                    );
                }
                current_block_83 = 16313536926714486912;
            }
            115 => {
                if *a == 0 {
                    generic_error(
                        0 as *const libc::c_char,
                        b"state missing after -s\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    getstates(&mut dc, (*argv).offset(1 as libc::c_int as isize));
                }
                current_block_83 = 16313536926714486912;
            }
            116 => {
                textflag = 1 as libc::c_int != 0;
                if *a != 0 {
                    if !textfile.is_null() {
                        redefined('t' as i32);
                    }
                    textfile = a;
                }
                current_block_83 = 16313536926714486912;
            }
            84 => {
                if *a != 0 {
                    current_block_83 = 16570555384594190488;
                } else {
                    Ttimeflag = 1 as libc::c_int != 0;
                    current_block_83 = 16313536926714486912;
                }
            }
            73 => {
                (*top).behavior.interactive = 1 as libc::c_int != 0;
                current_block_83 = 16313536926714486912;
            }
            113 => {
                (*top).behavior.quiet = 1 as libc::c_int != 0;
                current_block_83 = 16313536926714486912;
            }
            120 => {
                (*top).behavior.pe = a;
                current_block_83 = 16313536926714486912;
            }
            86 => {
                setRCSversion(*argv);
                current_block_83 = 16313536926714486912;
            }
            122 => {
                zone_set(a);
                current_block_83 = 16313536926714486912;
            }
            107 => {
                if 0 as libc::c_int <= expmode {
                    redefined('k' as i32);
                }
                expmode = str2expmode(a);
                if 0 as libc::c_int <= expmode {
                    current_block_83 = 16313536926714486912;
                } else {
                    current_block_83 = 16570555384594190488;
                }
            }
            _ => {
                current_block_83 = 16570555384594190488;
            }
        }
        match current_block_83 {
            17179227041652854834 => {
                if *a == 0 {
                    generic_error(
                        0 as *const libc::c_char,
                        b"missing symbolic name after -%c\0" as *const u8
                            as *const libc::c_char,
                        *(*argv).offset(1 as libc::c_int as isize) as libc::c_int,
                    );
                } else {
                    getassoclst(&mut dc, (*argv).offset(1 as libc::c_int as isize));
                }
            }
            16570555384594190488 => {
                bad_option(*argv);
            }
            _ => {}
        }
    }
    dc.newlocks = boxlock.next;
    dc.byelocks = boxrm.next;
    if (*top).flow.erroneous {
        cleanup(&mut dc.rv);
    } else if argc < 1 as libc::c_int {
        generic_fatal(
            0 as *const libc::c_char,
            b"no input file\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut current_block_182: u64;
        while (0 as libc::c_int) < argc {
            let mut tip: *mut delta = 0 as *mut delta;
            let mut defbr: *const libc::c_char = 0 as *const libc::c_char;
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut newdesc: cbuf = {
                let mut init = cbuf {
                    string: 0 as *const libc::c_char,
                    size: 0 as libc::c_int as size_t,
                };
                init
            };
            ffree();
            if initflag {
                match pairnames(
                    argc,
                    argv,
                    Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                ) {
                    0 => {
                        current_block_182 = 796174441944384681;
                    }
                    1 => {
                        generic_error(
                            (*top).repository.filename,
                            b"already exists\0" as *const u8 as *const libc::c_char,
                        );
                        current_block_182 = 796174441944384681;
                    }
                    -1 | _ => {
                        current_block_182 = 5404178929002277151;
                    }
                }
            } else {
                match pairnames(
                    argc,
                    argv,
                    Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                    1 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                ) {
                    -1 | 0 => {
                        current_block_182 = 796174441944384681;
                    }
                    1 | _ => {
                        current_block_182 = 5404178929002277151;
                    }
                }
            }
            match current_block_182 {
                5404178929002277151 => {
                    repo_stat = &mut (*top).repository.stat;
                    tip = (*top).repository.tip;
                    defbr = (*(*top).repository.r).branch;
                    diagnose(
                        b"RCS file: %s\0" as *const u8 as *const libc::c_char,
                        (*top).repository.filename,
                    );
                    changed = initflag as libc::c_int | textflag as libc::c_int;
                    keepRCStime = Ttimeflag;
                    if !initflag {
                        if !checkaccesslist() {
                            current_block_182 = 796174441944384681;
                        } else {
                            current_block_182 = 8225018548522317130;
                        }
                    } else {
                        current_block_182 = 8225018548522317130;
                    }
                    match current_block_182 {
                        796174441944384681 => {}
                        _ => {
                            if strict_selected {
                                changed
                                    |= (*top).behavior.strictly_locking as libc::c_int
                                        ^ strictlock as libc::c_int;
                                (*top).behavior.strictly_locking = strictlock;
                            }
                            if !commsyml.is_null()
                                && (commsymlen != (*top).repository.log_lead.size
                                    || 0 as libc::c_int
                                        != memcmp(
                                            commsyml as *const libc::c_void,
                                            (*top).repository.log_lead.string as *const libc::c_void,
                                            commsymlen,
                                        ))
                            {
                                (*top).repository.log_lead.string = commsyml;
                                (*top).repository.log_lead.size = commsymlen;
                                changed = 1 as libc::c_int;
                            }
                            if 0 as libc::c_int <= expmode
                                && (*top).behavior.kws != expmode
                            {
                                (*top).behavior.kws = expmode;
                                changed = 1 as libc::c_int;
                            }
                            if branchflag as libc::c_int != 0
                                && fully_numeric(&mut branchnum, branchsym, 0 as *mut fro)
                                    as libc::c_int != 0
                            {
                                if countnumflds(branchnum.string) != 0 {
                                    if !(0 as libc::c_int == cmpnum(defbr, branchnum.string)) {
                                        (*(*top).repository.r).branch = branchnum.string;
                                        defbr = (*(*top).repository.r).branch;
                                        changed = 1 as libc::c_int;
                                    }
                                } else if !defbr.is_null() {
                                    (*(*top).repository.r).branch = 0 as *const libc::c_char;
                                    defbr = (*(*top).repository.r).branch;
                                    changed = 1 as libc::c_int;
                                }
                            }
                            changed |= doaccess(&mut dc) as libc::c_int;
                            changed |= doassoc(&mut dc) as libc::c_int;
                            changed |= dolocks(&mut dc) as libc::c_int;
                            changed |= domessages(&mut dc) as libc::c_int;
                            if dc.headstate_changed {
                                if defbr.is_null() {
                                    if tip.is_null() {
                                        generic_warn(
                                            (*top).repository.filename,
                                            b"can't change states in an empty tree\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    } else if strcmp((*tip).state, dc.headstate) != 0 {
                                        (*tip).state = dc.headstate;
                                        changed = 1 as libc::c_int;
                                    }
                                } else {
                                    changed
                                        |= rcs_setstate(&mut dc, defbr, dc.headstate)
                                            as libc::c_int;
                                }
                            }
                            let mut ls_0: *mut link = dc.states.next;
                            while !ls_0.is_null() {
                                let mut us: *const u_state = (*ls_0).entry
                                    as *const u_state;
                                changed
                                    |= rcs_setstate(&mut dc, (*us).revno, (*us).status)
                                        as libc::c_int;
                                ls_0 = (*ls_0).next;
                            }
                            dc.cuttail = 0 as *mut delta;
                            if !(dc.delrev.strt).is_null()
                                && removerevs(&mut dc) as libc::c_int != 0
                            {
                                if !(dc.cuttail).is_null() {
                                    gr_revno((*dc.cuttail).num, &mut dc.deltas);
                                }
                                buildtree(&mut dc);
                                tip = (*top).repository.tip;
                                changed = 1 as libc::c_int;
                                keepRCStime = 0 as libc::c_int != 0;
                            }
                            if !(*top).flow.erroneous {
                                putadmin();
                                if !tip.is_null() {
                                    puttree(tip, (*top).flow.rewr);
                                }
                                putdesc(&mut newdesc, textflag, textfile);
                                if !(dc.delrev.strt).is_null() || !(dc.logs.next).is_null()
                                {
                                    let mut from: *mut fro = (*top).flow.from;
                                    let mut es: *mut editstuff = make_editstuff();
                                    let mut ls_1: *mut wlink = (*(*top).repository.r).deltas;
                                    if (dc.cuttail).is_null()
                                        || buildeltatext(&mut dc, es, &mut ls_1, dc.deltas)
                                            as libc::c_int != 0
                                    {
                                        fro_trundling(1 as libc::c_int != 0, from);
                                        if !(dc.cuttail).is_null() {
                                            ls_1 = (*ls_1).next;
                                        }
                                        scanlogtext(
                                            &mut dc,
                                            es,
                                            &mut ls_1,
                                            0 as *mut delta,
                                            0 as libc::c_int != 0,
                                        );
                                        changed = 1 as libc::c_int;
                                    }
                                    unmake_editstuff(es);
                                    (*from).verbatim = (*from).end;
                                } else if !((*(*top).repository.r).desc).is_null() {
                                    (*(*top).flow.from)
                                        .verbatim = *((*(*(*top).repository.r).desc).holes)
                                        .as_mut_ptr()
                                        .offset(
                                            ((*(*(*top).repository.r).desc).count)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) + 2 as libc::c_int as libc::c_long;
                                }
                                if initflag {
                                    if 0 as libc::c_int
                                        > stat((*top).manifestation.filename, repo_stat)
                                    {
                                        changed = -(1 as libc::c_int);
                                    }
                                    (*repo_stat).st_nlink = 0 as libc::c_int as __nlink_t;
                                    keepRCStime = 0 as libc::c_int != 0;
                                }
                                if 0 as libc::c_int
                                    > donerewrite(changed, file_mtime(keepRCStime, repo_stat))
                                {
                                    break;
                                }
                                diagnose(b"done\0" as *const u8 as *const libc::c_char);
                            }
                        }
                    }
                }
                _ => {}
            }
            cleanup(&mut dc.rv);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    gnurcs_goodbye();
    return dc.rv;
}
static mut rcs_aka: [uint8_t; 16] = [
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    'f' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'b' as i32 as uint8_t,
    3 as libc::c_int as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    5 as libc::c_int as uint8_t,
    'a' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'n' as i32 as uint8_t,
];
pub static mut ya_rcs: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcs_main
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            aka: rcs_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};

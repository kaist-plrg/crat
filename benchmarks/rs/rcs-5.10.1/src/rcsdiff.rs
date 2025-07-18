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
    static prog_diff: [libc::c_char; 0];
    static equal_line: [libc::c_char; 0];
    static mut top: *mut top;
    static ks_revno: [libc::c_char; 0];
    fn str2expmode(s: *const libc::c_char) -> libc::c_int;
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
    fn pairnames(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        rcsopen: Option::<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> libc::c_int;
    fn delta_from_ref(ref_0: *const libc::c_char) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const libc::c_char, fp: *mut fro) -> bool;
    fn time2date(unixtime: time_t, date: *mut libc::c_char);
    fn zone_set(s: *const libc::c_char);
    fn date2str(
        date: *const libc::c_char,
        datebuf: *mut libc::c_char,
    ) -> *const libc::c_char;
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn minus_p(xrev: *const libc::c_char, rev: *const libc::c_char) -> cbuf;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn diagnose(fmt: *const libc::c_char, _: ...);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut plexus: *mut divvy;
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn maketemp(n: libc::c_int) -> *const libc::c_char;
    fn tempunlink();
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
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
pub struct unique {
    pub eqval_ok: bool,
    pub minlen: size_t,
    pub full: [libc::c_char; 32],
}
pub type s_unique = unique;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work {
    pub st: stat,
    pub fro: *mut fro,
}
static mut rcsdiff_blurb: [libc::c_char; 23] = unsafe {
    *::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"Compare RCS revisions.\0")
};
static mut rcsdiff_help: [libc::c_char; 1073] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1073],
        &[libc::c_char; 1073],
    >(
        b"[options] file ...\nOptions:\n  -rREV         (zero, one, or two times) Name a revision.\n  -kSUBST       Substitute using mode SUBST (see co(1)).\n  -q            Quiet mode.\n  -T            No effect; included for compatibility with other commands.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n\nIf given two revisions (-rREV1 -rREV2), compare those revisions.\nIf given only one revision (-rREV), compare the working file with it.\nIf given no revisions, compare the working file with the latest\nrevision on the default branch.\n\nAdditionally, the following options (and their argument, if any) are\npassed to the underlying diff(1) command:\n  -0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -B, -C, -D, -F, -H, -I,\n  -L, -U, -W, -a, -b, -c, -d, -e, -f, -h, -i, -n, -p, -t, -u, -w, -y,\n  [long options (that start with \"--\")].\n(Not all of these options are meaningful.)\n\0",
    )
};
static mut minus_y: s_unique = unsafe {
    {
        let mut init = unique {
            eqval_ok: 0 as libc::c_int != 0,
            minlen: 4 as libc::c_int as size_t,
            full: *::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"--side-by-side\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        };
        init
    }
};
static mut minus_D: s_unique = unsafe {
    {
        let mut init = unique {
            eqval_ok: 1 as libc::c_int != 0,
            minlen: 4 as libc::c_int as size_t,
            full: *::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"--ifdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn longopt_maybe_p(
    mut arg: *const libc::c_char,
    mut u: *const s_unique,
) -> bool {
    let mut equal: *const libc::c_char = if (*u).eqval_ok as libc::c_int != 0 {
        strchr(arg, '=' as i32)
    } else {
        0 as *mut libc::c_char
    };
    let mut len: size_t = if !equal.is_null() {
        equal.offset_from(arg) as libc::c_long as size_t
    } else {
        strlen(arg)
    };
    return !((*u).minlen > len)
        && 0 as libc::c_int == strncmp(arg, ((*u).full).as_ptr(), len);
}
unsafe extern "C" fn cleanup(mut exitstatus: *mut libc::c_int, mut work: *mut work) {
    if (*top).flow.erroneous {
        *exitstatus = 2 as libc::c_int;
    }
    fro_zclose(&mut (*top).flow.from);
    fro_zclose(&mut (*work).fro);
}
unsafe extern "C" fn setup_label(
    mut num: *const libc::c_char,
    mut date: *const libc::c_char,
) -> *const libc::c_char {
    let mut len: size_t = 0;
    let mut datestr: [libc::c_char; 31] = [0; 31];
    date2str(date, datestr.as_mut_ptr());
    accf(
        plexus,
        b"--label=%s\t%s\0" as *const u8 as *const libc::c_char,
        (*top).manifestation.filename,
        datestr.as_mut_ptr(),
    );
    if !num.is_null() {
        accf(plexus, b"\t%s\0" as *const u8 as *const libc::c_char, num);
    }
    return finish_string(plexus, &mut len);
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: rcsdiff_blurb.as_ptr(),
            help: rcsdiff_help.as_ptr(),
            tyag: (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn rcsdiff_main(
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut exitstatus: libc::c_int = 0 as libc::c_int;
    let mut work: work = work {
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
    };
    let mut revnums: libc::c_int = 0;
    let mut rev1: *const libc::c_char = 0 as *const libc::c_char;
    let mut rev2: *const libc::c_char = 0 as *const libc::c_char;
    let mut xrev1: *const libc::c_char = 0 as *const libc::c_char;
    let mut xrev2: *const libc::c_char = 0 as *const libc::c_char;
    let mut expandarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut lexpandarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffixarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut versionarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut zonearg: *const libc::c_char = 0 as *const libc::c_char;
    let mut file_labels: libc::c_int = 0;
    let mut diff_label1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut diff_label2: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut date2: [libc::c_char; 22] = [0; 22];
    let mut cov: [*const libc::c_char; 11] = [0 as *const libc::c_char; 11];
    let mut diffv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut diffp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut diffpend: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut pp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut diffvstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut commarg: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dcp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut no_diff_means_no_output: bool = false;
    let mut c: libc::c_int = 0;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut work as *mut work as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<work>() as libc::c_ulong,
    );
    exitstatus = 0 as libc::c_int;
    revnums = 0 as libc::c_int;
    xrev2 = 0 as *const libc::c_char;
    rev2 = xrev2;
    rev1 = rev2;
    file_labels = 0 as libc::c_int;
    zonearg = 0 as *const libc::c_char;
    versionarg = zonearg;
    suffixarg = versionarg;
    expandarg = suffixarg;
    no_diff_means_no_output = 1 as libc::c_int != 0;
    diffv = pointer_array(
        plexus,
        (1 as libc::c_int + argc + (0 as libc::c_int != 0) as libc::c_int
            + 2 as libc::c_int * 1 as libc::c_int + 2 as libc::c_int) as size_t,
    ) as *mut *const libc::c_char;
    diffp = diffv.offset(1 as libc::c_int as isize);
    let fresh0 = diffp;
    diffp = diffp.offset(1);
    *fresh0 = prog_diff.as_ptr();
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as libc::c_int) < argc
            && {
                let fresh1 = a;
                a = a.offset(1);
                *fresh1 as libc::c_int == '-' as i32
            })
        {
            break;
        }
        dcp = a;
        loop {
            let fresh2 = a;
            a = a.offset(1);
            c = *fresh2 as libc::c_int;
            if !(c != 0) {
                break;
            }
            match c {
                114 => {
                    revnums += 1;
                    match revnums {
                        1 => {
                            rev1 = a;
                        }
                        2 => {
                            rev2 = a;
                        }
                        _ => {
                            generic_error(
                                0 as *const libc::c_char,
                                b"too many %ss\0" as *const u8 as *const libc::c_char,
                                ks_revno.as_ptr(),
                            );
                        }
                    }
                    break;
                }
                45 | 68 => {
                    if 'D' as i32 == c
                        || longopt_maybe_p(*argv, &minus_D) as libc::c_int != 0
                        || longopt_maybe_p(*argv, &minus_y) as libc::c_int != 0
                    {
                        no_diff_means_no_output = 0 as libc::c_int != 0;
                    }
                    current_block = 9857854851893061578;
                }
                67 | 70 | 73 | 76 | 85 | 87 => {
                    current_block = 9857854851893061578;
                }
                121 => {
                    no_diff_means_no_output = 0 as libc::c_int != 0;
                    current_block = 3239950539056491389;
                }
                66 | 72 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99
                | 100 | 101 | 102 | 104 | 105 | 110 | 112 | 116 | 117 | 119 => {
                    current_block = 3239950539056491389;
                }
                113 => {
                    (*top).behavior.quiet = 1 as libc::c_int != 0;
                    continue;
                }
                120 => {
                    suffixarg = *argv;
                    (*top).behavior.pe = (*argv).offset(2 as libc::c_int as isize);
                    break;
                }
                122 => {
                    zonearg = *argv;
                    zone_set((*argv).offset(2 as libc::c_int as isize));
                    break;
                }
                84 => {
                    if !(*a != 0) {
                        continue;
                    }
                    current_block = 8925846151124234692;
                }
                86 => {
                    versionarg = *argv;
                    setRCSversion(versionarg);
                    break;
                }
                107 => {
                    expandarg = *argv;
                    if 0 as libc::c_int
                        <= str2expmode(expandarg.offset(2 as libc::c_int as isize))
                    {
                        break;
                    }
                    current_block = 8925846151124234692;
                }
                _ => {
                    current_block = 8925846151124234692;
                }
            }
            match current_block {
                8925846151124234692 => {
                    bad_option(*argv);
                }
                3239950539056491389 => {
                    let fresh8 = dcp;
                    dcp = dcp.offset(1);
                    *fresh8 = c as libc::c_char;
                }
                _ => {
                    if 1 as libc::c_int != 0 && c == 'L' as i32
                        && {
                            file_labels += 1;
                            file_labels == 2 as libc::c_int
                        }
                    {
                        generic_fatal(
                            0 as *const libc::c_char,
                            b"too many -L options\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    let fresh3 = dcp;
                    dcp = dcp.offset(1);
                    *fresh3 = c as libc::c_char;
                    if *a != 0 {
                        loop {
                            let fresh4 = a;
                            a = a.offset(1);
                            let fresh5 = dcp;
                            dcp = dcp.offset(1);
                            *fresh5 = *fresh4;
                            if !(*a != 0) {
                                break;
                            }
                        }
                    } else {
                        argc -= 1;
                        if argc == 0 {
                            generic_fatal(
                                0 as *const libc::c_char,
                                b"-%c needs following argument\0" as *const u8
                                    as *const libc::c_char,
                                c,
                            );
                        }
                        let fresh6 = argv;
                        argv = argv.offset(1);
                        let fresh7 = diffp;
                        diffp = diffp.offset(1);
                        *fresh7 = *fresh6;
                    }
                }
            }
        }
        if dcp != (*argv).offset(1 as libc::c_int as isize) {
            *dcp = '\0' as i32 as libc::c_char;
            let fresh9 = diffp;
            diffp = diffp.offset(1);
            *fresh9 = *argv;
        }
    }
    if !(*top).behavior.quiet {
        let mut len: size_t = 0;
        pp = diffv.offset(2 as libc::c_int as isize);
        while pp < diffp {
            let fresh10 = pp;
            pp = pp.offset(1);
            accf(plexus, b" %s\0" as *const u8 as *const libc::c_char, *fresh10);
        }
        diffvstr = finish_string(plexus, &mut len);
    }
    diff_label2 = 0 as *mut *const libc::c_char;
    diff_label1 = diff_label2;
    if file_labels < 2 as libc::c_int {
        if file_labels == 0 {
            let fresh11 = diffp;
            diffp = diffp.offset(1);
            diff_label1 = fresh11;
        }
        let fresh12 = diffp;
        diffp = diffp.offset(1);
        diff_label2 = fresh12;
    }
    diffpend = diffp;
    cov[1 as libc::c_int as usize] = find_peer_prog(&mut peer_super);
    cov[2 as libc::c_int as usize] = b"co\0" as *const u8 as *const libc::c_char;
    cov[3 as libc::c_int as usize] = b"-q\0" as *const u8 as *const libc::c_char;
    if 1 as libc::c_int == 0 {
        cov[(4 as libc::c_int + (1 as libc::c_int == 0) as libc::c_int
            - 1 as libc::c_int) as usize] = b"-M\0" as *const u8 as *const libc::c_char;
    }
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut work);
    } else if argc < 1 as libc::c_int {
        generic_fatal(
            0 as *const libc::c_char,
            b"no input file\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut current_block_134: u64;
        while (0 as libc::c_int) < argc {
            let mut numericrev: cbuf = cbuf {
                string: 0 as *const libc::c_char,
                size: 0,
            };
            let mut tip: *mut delta = 0 as *mut delta;
            let mut mani_filename: *const libc::c_char = 0 as *const libc::c_char;
            let mut defbr: *const libc::c_char = 0 as *const libc::c_char;
            let mut kws: libc::c_int = 0;
            ffree();
            if !(pairnames(
                argc,
                argv,
                Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                1 as libc::c_int != 0,
                0 as libc::c_int != 0,
            ) <= 0 as libc::c_int)
            {
                tip = (*top).repository.tip;
                mani_filename = (*top).manifestation.filename;
                kws = (*top).behavior.kws;
                defbr = (*(*top).repository.r).branch;
                diagnose(
                    b"%sRCS file: %s\0" as *const u8 as *const libc::c_char,
                    equal_line.as_ptr().offset(10 as libc::c_int as isize),
                    (*top).repository.filename,
                );
                if rev2.is_null() {
                    work
                        .fro = fro_open(
                        mani_filename,
                        b"r\0" as *const u8 as *const libc::c_char,
                        &mut work.st,
                    );
                    if (work.fro).is_null() {
                        syserror(*__errno_location(), mani_filename);
                        current_block_134 = 9505035279996566320;
                    } else {
                        current_block_134 = 1765866445182206997;
                    }
                } else {
                    current_block_134 = 1765866445182206997;
                }
                match current_block_134 {
                    9505035279996566320 => {}
                    _ => {
                        if tip.is_null() {
                            generic_error(
                                (*top).repository.filename,
                                b"no revisions present\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            if revnums == 0 as libc::c_int || *rev1 == 0 {
                                rev1 = if !defbr.is_null() { defbr } else { (*tip).num };
                            }
                            if fully_numeric(&mut numericrev, rev1, work.fro) {
                                target = delta_from_ref(numericrev.string);
                                if !target.is_null() {
                                    xrev1 = (*target).num;
                                    if 1 as libc::c_int != 0 && !diff_label1.is_null() {
                                        *diff_label1 = setup_label((*target).num, (*target).date);
                                    }
                                    lexpandarg = expandarg;
                                    if revnums == 2 as libc::c_int {
                                        if !fully_numeric(
                                            &mut numericrev,
                                            if *rev2 as libc::c_int != 0 {
                                                rev2
                                            } else if !defbr.is_null() {
                                                defbr
                                            } else {
                                                (*tip).num
                                            },
                                            work.fro,
                                        ) {
                                            current_block_134 = 9505035279996566320;
                                        } else {
                                            target = delta_from_ref(numericrev.string);
                                            if target.is_null() {
                                                current_block_134 = 9505035279996566320;
                                            } else {
                                                xrev2 = (*target).num;
                                                if no_diff_means_no_output as libc::c_int != 0
                                                    && xrev1 == xrev2
                                                {
                                                    current_block_134 = 9505035279996566320;
                                                } else {
                                                    current_block_134 = 8602574157404971894;
                                                }
                                            }
                                        }
                                    } else {
                                        if !((*target).lockedby).is_null() && lexpandarg.is_null()
                                            && kws == kwsub_kv as libc::c_int
                                            && (*top).repository.stat.st_mode
                                                & !(0o200 as libc::c_int
                                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                                        >> 3 as libc::c_int) as mode_t
                                                | (if 1 as libc::c_int != 0 {
                                                    0o200 as libc::c_int
                                                } else {
                                                    0 as libc::c_int
                                                }) as libc::c_uint == work.st.st_mode
                                        {
                                            lexpandarg = b"-kkvl\0" as *const u8 as *const libc::c_char;
                                        }
                                        current_block_134 = 8602574157404971894;
                                    }
                                    match current_block_134 {
                                        9505035279996566320 => {}
                                        _ => {
                                            fro_zclose(&mut work.fro);
                                            if 1 as libc::c_int != 0 && !diff_label2.is_null() {
                                                if revnums == 2 as libc::c_int {
                                                    *diff_label2 = setup_label((*target).num, (*target).date);
                                                } else {
                                                    time2date(work.st.st_mtim.tv_sec, date2.as_mut_ptr());
                                                    *diff_label2 = setup_label(
                                                        0 as *const libc::c_char,
                                                        date2.as_mut_ptr() as *const libc::c_char,
                                                    );
                                                }
                                            }
                                            commarg = minus_p(xrev1, rev1);
                                            pp = &mut *cov
                                                .as_mut_ptr()
                                                .offset(
                                                    (4 as libc::c_int + (1 as libc::c_int == 0) as libc::c_int)
                                                        as isize,
                                                ) as *mut *const libc::c_char;
                                            let fresh13 = pp;
                                            pp = pp.offset(1);
                                            *fresh13 = commarg.string;
                                            if !lexpandarg.is_null() {
                                                let fresh14 = pp;
                                                pp = pp.offset(1);
                                                *fresh14 = lexpandarg;
                                            }
                                            if !suffixarg.is_null() {
                                                let fresh15 = pp;
                                                pp = pp.offset(1);
                                                *fresh15 = suffixarg;
                                            }
                                            if !versionarg.is_null() {
                                                let fresh16 = pp;
                                                pp = pp.offset(1);
                                                *fresh16 = versionarg;
                                            }
                                            if !zonearg.is_null() {
                                                let fresh17 = pp;
                                                pp = pp.offset(1);
                                                *fresh17 = zonearg;
                                            }
                                            let fresh18 = pp;
                                            pp = pp.offset(1);
                                            *fresh18 = (*top).repository.filename;
                                            *pp = 0 as *const libc::c_char;
                                            diffp = diffpend;
                                            if 0 as libc::c_int != 0 && kws == kwsub_b as libc::c_int {
                                                let fresh19 = diffp;
                                                diffp = diffp.offset(1);
                                                *fresh19 = b"--binary\0" as *const u8
                                                    as *const libc::c_char;
                                            }
                                            let ref mut fresh20 = *diffp
                                                .offset(0 as libc::c_int as isize);
                                            *fresh20 = maketemp(0 as libc::c_int);
                                            if runv(
                                                -(1 as libc::c_int),
                                                *diffp.offset(0 as libc::c_int as isize),
                                                cov.as_mut_ptr(),
                                            ) != 0
                                            {
                                                generic_error(
                                                    (*top).repository.filename,
                                                    b"co failed\0" as *const u8 as *const libc::c_char,
                                                );
                                            } else {
                                                if rev2.is_null() {
                                                    let ref mut fresh21 = *diffp
                                                        .offset(1 as libc::c_int as isize);
                                                    *fresh21 = mani_filename;
                                                    if *mani_filename as libc::c_int == '-' as i32 {
                                                        accf(
                                                            plexus,
                                                            b".%c\0" as *const u8 as *const libc::c_char,
                                                            '/' as i32,
                                                        );
                                                        let ref mut fresh22 = *diffp
                                                            .offset(1 as libc::c_int as isize);
                                                        *fresh22 = str_save(mani_filename);
                                                    }
                                                    current_block_134 = 1830138855519935310;
                                                } else {
                                                    commarg = minus_p(xrev2, rev2);
                                                    cov[(4 as libc::c_int
                                                        + (1 as libc::c_int == 0) as libc::c_int)
                                                        as usize] = commarg.string;
                                                    let ref mut fresh23 = *diffp
                                                        .offset(1 as libc::c_int as isize);
                                                    *fresh23 = maketemp(1 as libc::c_int);
                                                    if runv(
                                                        -(1 as libc::c_int),
                                                        *diffp.offset(1 as libc::c_int as isize),
                                                        cov.as_mut_ptr(),
                                                    ) != 0
                                                    {
                                                        generic_error(
                                                            (*top).repository.filename,
                                                            b"co failed\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        current_block_134 = 9505035279996566320;
                                                    } else {
                                                        current_block_134 = 1830138855519935310;
                                                    }
                                                }
                                                match current_block_134 {
                                                    9505035279996566320 => {}
                                                    _ => {
                                                        if rev2.is_null() {
                                                            diagnose(
                                                                b"diff%s -r%s %s\0" as *const u8 as *const libc::c_char,
                                                                diffvstr,
                                                                xrev1,
                                                                mani_filename,
                                                            );
                                                        } else {
                                                            diagnose(
                                                                b"diff%s -r%s -r%s\0" as *const u8 as *const libc::c_char,
                                                                diffvstr,
                                                                xrev1,
                                                                xrev2,
                                                            );
                                                        }
                                                        let ref mut fresh24 = *diffp
                                                            .offset(2 as libc::c_int as isize);
                                                        *fresh24 = 0 as *const libc::c_char;
                                                        let mut s: libc::c_int = runv(
                                                            -(1 as libc::c_int),
                                                            0 as *const libc::c_char,
                                                            diffv,
                                                        );
                                                        if 2 as libc::c_int == s {
                                                            generic_error(
                                                                (*top).manifestation.filename,
                                                                b"diff failed\0" as *const u8 as *const libc::c_char,
                                                            );
                                                        }
                                                        if 1 as libc::c_int == s && 0 as libc::c_int == exitstatus {
                                                            exitstatus = s;
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
static mut rcsdiff_aka: [uint8_t; 14] = [
    2 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    'd' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    7 as libc::c_int as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'f' as i32 as uint8_t,
];
pub static mut ya_rcsdiff: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcsdiff_main
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            aka: rcsdiff_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};

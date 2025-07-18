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
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn getRCSINIT(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        newargv: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn setRCSversion(str: *const libc::c_char);
    fn run(infd: libc::c_int, outname: *const libc::c_char, _: ...) -> libc::c_int;
    static mut top: *mut top;
    static ks_revno: [libc::c_char; 0];
    fn str2expmode(s: *const libc::c_char) -> libc::c_int;
    fn merge(
        tostdout: bool,
        edarg: *const libc::c_char,
        three_manifestations: *mut symdef,
    ) -> libc::c_int;
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
    fn zone_set(s: *const libc::c_char);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn minus_p(xrev: *const libc::c_char, rev: *const libc::c_char) -> cbuf;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn diagnose(fmt: *const libc::c_char, _: ...);
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    fn maketemp(n: libc::c_int) -> *const libc::c_char;
    fn tempunlink();
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    fn fro_spew(f: *mut fro, to: *mut FILE);
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
static mut rcsmerge_blurb: [libc::c_char; 21] = unsafe {
    *::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"Merge RCS revisions.\0")
};
static mut rcsmerge_help: [libc::c_char; 894] = unsafe {
    *::std::mem::transmute::<
        &[u8; 894],
        &[libc::c_char; 894],
    >(
        b"[options] file\nOptions:\n  -A            Passed to diff3(1).\n  -E            Passed to diff3(1); default if unspecified.\n  -e            Passed to diff3(1); do not warn on conflicts.\n  -p[REV]       Write to stdout instead of overwriting the working file.\n  -q[REV]       Quiet mode.\n  -rREV         (one or two times) specify a revision.\n  -kSUBST       Substitute using mode SUBST (see co(1)).\n  -T            No effect; included for compatibility with other commands.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n\nOne or two revisions must be specified (using -p, -q, or -r).\nIf only one is specified, use the latest revision on the default\nbranch to be the second revision.\n\0",
    )
};
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: rcsmerge_blurb.as_ptr(),
            help: rcsmerge_help.as_ptr(),
            tyag: (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn rcsmerge_main(
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut three_manifestations: [symdef; 3] = [symdef {
        meaningful: 0 as *const libc::c_char,
        underlying: 0 as *const libc::c_char,
    }; 3];
    let mut rev: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut edarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut expandarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffixarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut versionarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut zonearg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tostdout: bool = false;
    let mut status: libc::c_int = 0;
    let mut exitstatus: libc::c_int = 0;
    let mut workptr: *mut fro = 0 as *mut fro;
    let mut target: *mut delta = 0 as *mut delta;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    rev[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    rev[1 as libc::c_int as usize] = rev[2 as libc::c_int as usize];
    edarg = rev[1 as libc::c_int as usize];
    status = 0 as libc::c_int;
    tostdout = 0 as libc::c_int != 0;
    zonearg = b"-q\0" as *const u8 as *const libc::c_char;
    versionarg = zonearg;
    suffixarg = versionarg;
    expandarg = suffixarg;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as libc::c_int) < argc
            && {
                let fresh0 = a;
                a = a.offset(1);
                *fresh0 as libc::c_int == '-' as i32
            })
        {
            break;
        }
        let fresh1 = a;
        a = a.offset(1);
        match *fresh1 as libc::c_int {
            112 => {
                tostdout = 1 as libc::c_int != 0;
                current_block = 6872901883738955241;
            }
            113 => {
                (*top).behavior.quiet = 1 as libc::c_int != 0;
                current_block = 6872901883738955241;
            }
            114 => {
                current_block = 8367529978370297332;
            }
            65 | 69 | 101 => {
                if *a != 0 {
                    current_block = 8591603307576814817;
                } else {
                    edarg = *argv;
                    continue;
                }
            }
            120 => {
                suffixarg = *argv;
                (*top).behavior.pe = a;
                continue;
            }
            122 => {
                zonearg = *argv;
                zone_set(a);
                continue;
            }
            84 => {
                if !(*a != 0) {
                    continue;
                }
                current_block = 8591603307576814817;
            }
            86 => {
                versionarg = *argv;
                setRCSversion(versionarg);
                continue;
            }
            107 => {
                expandarg = *argv;
                if 0 as libc::c_int
                    <= str2expmode(expandarg.offset(2 as libc::c_int as isize))
                {
                    continue;
                }
                current_block = 8591603307576814817;
            }
            _ => {
                current_block = 8591603307576814817;
            }
        }
        match current_block {
            8591603307576814817 => {
                bad_option(*argv);
                continue;
            }
            6872901883738955241 => {
                if *a == 0 {
                    continue;
                }
            }
            _ => {}
        }
        if (rev[1 as libc::c_int as usize]).is_null() {
            rev[1 as libc::c_int as usize] = a;
        } else if (rev[2 as libc::c_int as usize]).is_null() {
            rev[2 as libc::c_int as usize] = a;
        } else {
            generic_error(
                0 as *const libc::c_char,
                b"too many %ss\0" as *const u8 as *const libc::c_char,
                ks_revno.as_ptr(),
            );
        }
    }
    if (rev[1 as libc::c_int as usize]).is_null() {
        generic_fatal(
            0 as *const libc::c_char,
            b"no base %s given\0" as *const u8 as *const libc::c_char,
            ks_revno.as_ptr(),
        );
    }
    if !(*top).flow.erroneous {
        if argc < 1 as libc::c_int {
            generic_fatal(
                0 as *const libc::c_char,
                b"no input file\0" as *const u8 as *const libc::c_char,
            );
        }
        if (0 as libc::c_int)
            < pairnames(
                argc,
                argv,
                Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                1 as libc::c_int != 0,
                0 as libc::c_int != 0,
            )
        {
            let mut numericrev: cbuf = cbuf {
                string: 0 as *const libc::c_char,
                size: 0,
            };
            let mut repo_filename: *const libc::c_char = (*top).repository.filename;
            let mut mani_filename: *const libc::c_char = (*top).manifestation.filename;
            let mut defbr: *const libc::c_char = (*(*top).repository.r).branch;
            let mut tip: *mut delta = (*top).repository.tip;
            if argc > 2 as libc::c_int
                || argc == 2 as libc::c_int
                    && !(*argv.offset(1 as libc::c_int as isize)).is_null()
            {
                generic_warn(
                    0 as *const libc::c_char,
                    b"excess arguments ignored\0" as *const u8 as *const libc::c_char,
                );
            }
            if (*top).behavior.kws == kwsub_b as libc::c_int {
                generic_error(
                    (*top).manifestation.filename,
                    b"merging binary files\0" as *const u8 as *const libc::c_char,
                );
            }
            diagnose(
                b"RCS file: %s\0" as *const u8 as *const libc::c_char,
                repo_filename,
            );
            workptr = fro_open(
                mani_filename,
                b"r\0" as *const u8 as *const libc::c_char,
                0 as *mut stat,
            );
            if workptr.is_null() {
                fatal_sys(mani_filename);
            }
            if tip.is_null() {
                generic_fatal(
                    (*top).repository.filename,
                    b"no revisions present\0" as *const u8 as *const libc::c_char,
                );
            }
            if *rev[1 as libc::c_int as usize] == 0 {
                rev[1 as libc::c_int
                    as usize] = if !defbr.is_null() { defbr } else { (*tip).num };
            }
            if fully_numeric(&mut numericrev, rev[1 as libc::c_int as usize], workptr)
                as libc::c_int != 0
                && {
                    target = delta_from_ref(numericrev.string);
                    !target.is_null()
                }
            {
                three_manifestations[1 as libc::c_int as usize]
                    .meaningful = (*target).num;
                if (rev[2 as libc::c_int as usize]).is_null()
                    || *rev[2 as libc::c_int as usize] == 0
                {
                    rev[2 as libc::c_int
                        as usize] = if !defbr.is_null() { defbr } else { (*tip).num };
                }
                if fully_numeric(
                    &mut numericrev,
                    rev[2 as libc::c_int as usize],
                    workptr,
                ) as libc::c_int != 0
                    && {
                        target = delta_from_ref(numericrev.string);
                        !target.is_null()
                    }
                {
                    three_manifestations[2 as libc::c_int as usize]
                        .meaningful = (*target).num;
                    if strcmp(
                        three_manifestations[1 as libc::c_int as usize].meaningful,
                        three_manifestations[2 as libc::c_int as usize].meaningful,
                    ) == 0
                    {
                        if tostdout {
                            fro_spew(workptr, stdout);
                            fclose(stdout);
                        }
                    } else {
                        fro_zclose(&mut workptr);
                        i = 1 as libc::c_int;
                        while i <= 2 as libc::c_int {
                            let mut commarg: cbuf = minus_p(
                                three_manifestations[i as usize].meaningful,
                                rev[i as usize],
                            );
                            three_manifestations[i as usize]
                                .underlying = maketemp(i + 2 as libc::c_int);
                            if run(
                                -(1 as libc::c_int),
                                three_manifestations[i as usize].underlying,
                                find_peer_prog(&mut peer_super),
                                b"co\0" as *const u8 as *const libc::c_char,
                                b"-q\0" as *const u8 as *const libc::c_char,
                                commarg.string,
                                expandarg,
                                suffixarg,
                                versionarg,
                                zonearg,
                                repo_filename,
                                0 as *mut libc::c_void,
                            ) != 0
                            {
                                generic_fatal(
                                    (*top).repository.filename,
                                    b"co failed\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            i += 1;
                            i;
                        }
                        diagnose(
                            b"Merging differences between %s and %s into %s%s\0"
                                as *const u8 as *const libc::c_char,
                            three_manifestations[1 as libc::c_int as usize].meaningful,
                            three_manifestations[2 as libc::c_int as usize].meaningful,
                            mani_filename,
                            if tostdout as libc::c_int != 0 {
                                b"; result to stdout\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                        three_manifestations[0 as libc::c_int as usize]
                            .meaningful = mani_filename;
                        three_manifestations[0 as libc::c_int as usize]
                            .underlying = three_manifestations[0 as libc::c_int as usize]
                            .meaningful;
                        status = merge(
                            tostdout,
                            edarg,
                            three_manifestations.as_mut_ptr(),
                        );
                    }
                }
            }
            fro_zclose(&mut workptr);
        }
    }
    tempunlink();
    exitstatus = if (*top).flow.erroneous as libc::c_int != 0 {
        2 as libc::c_int
    } else {
        status
    };
    gnurcs_goodbye();
    return exitstatus;
}
static mut rcsmerge_aka: [uint8_t; 16] = [
    2 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    'm' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    8 as libc::c_int as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    'e' as i32 as uint8_t,
];
pub static mut ya_rcsmerge: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcsmerge_main
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            aka: rcsmerge_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};

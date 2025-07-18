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
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn file_mtime(enable: bool, st: *const stat) -> timespec;
    fn getRCSINIT(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        newargv: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn setRCSversion(str: *const libc::c_char);
    fn str_save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ffree();
    fn chk_set_rev(rev: *mut *const libc::c_char, arg: *mut libc::c_char);
    fn redefined(c: libc::c_int);
    fn bad_option(option: *const libc::c_char);
    fn gnurcs_goodbye();
    fn gnurcs_init(program_0: *const program);
    fn zone_set(s: *const libc::c_char);
    static mut top: *mut top;
    fn fully_numeric(ans: *mut cbuf, source: *const libc::c_char, fp: *mut fro) -> bool;
    fn str2expmode(s: *const libc::c_char) -> libc::c_int;
    fn un_link(s: *const libc::c_char) -> libc::c_int;
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn findlock(delete: bool, target: *mut *mut delta) -> libc::c_int;
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
    fn rcssuffix(name: *const libc::c_char) -> *const libc::c_char;
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
    fn write_desc_maybe(to: *mut FILE);
    fn gr_revno(revno: *const libc::c_char, store: *mut *mut wlink) -> *mut delta;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn scandir(
        __dir: *const libc::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: Option::<unsafe extern "C" fn(*const dirent) -> libc::c_int>,
        __cmp: Option::<
            unsafe extern "C" fn(*mut *const dirent, *mut *const dirent) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut exit_failure: libc::c_int;
    fn fatal_sys(who: *const libc::c_char);
    static mut plexus: *mut divvy;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn setrid();
    fn caller_login_p(login: *const libc::c_char) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn Ozclose(p: *mut *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const libc::c_char, _: ...);
    fn tempunlink();
    fn dirtempunlink();
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    fn fro_trundling(sequential: bool, f: *mut fro);
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
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
static mut rcsclean_blurb: [libc::c_char; 24] = unsafe {
    *::std::mem::transmute::<
        &[u8; 24],
        &[libc::c_char; 24],
    >(b"Clean up working files.\0")
};
static mut rcsclean_help: [libc::c_char; 705] = unsafe {
    *::std::mem::transmute::<
        &[u8; 705],
        &[libc::c_char; 705],
    >(
        b"[options] file ...\nOptions:\n  -r[REV]       Specify revision.\n  -u[REV]       Unlock if is locked and no differences found.\n  -n[REV]       Dry run (no act, don't operate).\n  -q[REV]       Quiet mode.\n  -kSUBST       Substitute using mode SUBST (see co(1)).\n  -T            Preserve the modification time on the RCS file\n                even if it changes because a lock is removed.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n\nREV defaults to the latest revision on the default branch.\n\0",
    )
};
unsafe extern "C" fn cleanup(
    mut exitstatus: *mut libc::c_int,
    mut workptr: *mut *mut fro,
) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    fro_zclose(workptr);
    Ozclose(&mut (*top).flow.res);
    ORCSclose();
    dirtempunlink();
}
unsafe extern "C" fn unlock(mut delta: *mut delta) -> bool {
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    if !delta.is_null() && !((*delta).lockedby).is_null()
        && caller_login_p((*delta).lockedby) as libc::c_int != 0
        && {
            box_0.next = (*(*top).repository.r).locks;
            !(box_0.next).is_null()
        }
        && {
            tp = lock_memq(
                &mut box_0,
                0 as libc::c_int != 0,
                delta as *const libc::c_void,
            );
            !tp.is_null()
        }
    {
        lock_drop(&mut box_0, tp);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn valid_filename_p(mut ent: *const dirent) -> libc::c_int {
    let mut en: *const libc::c_char = ((*ent).d_name).as_ptr();
    if *en.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && (*en.offset(1 as libc::c_int as isize) == 0
            || *en.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *en.offset(2 as libc::c_int as isize) == 0)
    {
        return 0 as libc::c_int;
    }
    if !(rcssuffix(en)).is_null() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn get_cwd_filenames(
    mut aargv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dot: [libc::c_char; 2] = *::std::mem::transmute::<
        &[u8; 2],
        &mut [libc::c_char; 2],
    >(b".\0");
    let mut names: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    count = scandir(
        dot.as_mut_ptr(),
        &mut names,
        Some(valid_filename_p as unsafe extern "C" fn(*const dirent) -> libc::c_int),
        None,
    );
    if 0 as libc::c_int > count {
        fatal_sys(dot.as_mut_ptr());
    }
    *aargv = pointer_array(plexus, count as size_t) as *mut *mut libc::c_char;
    i = count;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let ref mut fresh1 = *(*aargv).offset(i as isize);
        *fresh1 = str_save(((**names.offset(i as isize)).d_name).as_mut_ptr());
        rpl_free(*names.offset(i as isize) as *mut libc::c_void);
    }
    rpl_free(names as *mut libc::c_void);
    return count;
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: rcsclean_blurb.as_ptr(),
            help: rcsclean_help.as_ptr(),
            tyag: (1 as libc::c_int) << 3 as libc::c_int
                | ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int),
        };
        init
    }
};
unsafe extern "C" fn rcsclean_main(
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut exitstatus: libc::c_int = 0 as libc::c_int;
    let mut workptr: *mut fro = 0 as *mut fro;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut rev: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut dounlock: bool = false;
    let mut perform: bool = false;
    let mut unlocked: bool = false;
    let mut unlockflag: bool = false;
    let mut waslocked: bool = false;
    let mut Ttimeflag: bool = false;
    let mut expmode: libc::c_int = 0;
    let mut deltas: *mut wlink = 0 as *mut wlink;
    let mut delta: *mut delta = 0 as *mut delta;
    let mut workstat: stat = stat {
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
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    setrid();
    expmode = -(1 as libc::c_int);
    rev = 0 as *const libc::c_char;
    perform = 1 as libc::c_int != 0;
    unlockflag = 0 as libc::c_int != 0;
    Ttimeflag = 0 as libc::c_int != 0;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argc -= 1;
        if argc < 1 as libc::c_int {
            argc = get_cwd_filenames(&mut newargv);
            argv = newargv;
            break;
        } else {
            argv = argv.offset(1);
            a = *argv;
            if *a == 0
                || {
                    let fresh2 = a;
                    a = a.offset(1);
                    *fresh2 as libc::c_int != '-' as i32
                }
            {
                break;
            }
            let fresh3 = a;
            a = a.offset(1);
            match *fresh3 as libc::c_int {
                107 => {
                    if 0 as libc::c_int <= expmode {
                        redefined('k' as i32);
                    }
                    expmode = str2expmode(a);
                    if !(0 as libc::c_int > expmode) {
                        continue;
                    }
                    current_block = 5610426204187414840;
                }
                110 => {
                    perform = 0 as libc::c_int != 0;
                    current_block = 11495208372108337748;
                }
                113 => {
                    (*top).behavior.quiet = 1 as libc::c_int != 0;
                    current_block = 11495208372108337748;
                }
                114 => {
                    current_block = 11495208372108337748;
                }
                84 => {
                    if *a != 0 {
                        current_block = 5610426204187414840;
                    } else {
                        Ttimeflag = 1 as libc::c_int != 0;
                        continue;
                    }
                }
                117 => {
                    unlockflag = 1 as libc::c_int != 0;
                    current_block = 11495208372108337748;
                }
                86 => {
                    setRCSversion(*argv);
                    continue;
                }
                120 => {
                    (*top).behavior.pe = a;
                    continue;
                }
                122 => {
                    zone_set(a);
                    continue;
                }
                _ => {
                    current_block = 5610426204187414840;
                }
            }
            match current_block {
                5610426204187414840 => {
                    bad_option(*argv);
                }
                _ => {
                    chk_set_rev(&mut rev, a);
                }
            }
        }
    }
    dounlock = perform as libc::c_int & unlockflag as libc::c_int != 0;
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut workptr);
    } else {
        let mut current_block_56: u64;
        while (0 as libc::c_int) < argc {
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut mani_filename: *const libc::c_char = 0 as *const libc::c_char;
            ffree();
            if (0 as libc::c_int)
                < pairnames(
                    argc,
                    argv,
                    if dounlock as libc::c_int != 0 {
                        Some(
                            rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro,
                        )
                    } else {
                        Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro)
                    },
                    1 as libc::c_int != 0,
                    1 as libc::c_int != 0,
                )
                && {
                    mani_filename = (*top).manifestation.filename;
                    !mani_filename.is_null()
                }
                && {
                    workptr = fro_open(
                        mani_filename,
                        b"r\0" as *const u8 as *const libc::c_char,
                        &mut workstat,
                    );
                    !workptr.is_null()
                }
            {
                repo_stat = &mut (*top).repository.stat;
                if (*top).repository.stat.st_ino == workstat.st_ino
                    && (*top).repository.stat.st_dev == workstat.st_dev
                {
                    generic_error(
                        (*top).repository.filename,
                        b"RCS file is the same as working file %s.\0" as *const u8
                            as *const libc::c_char,
                        mani_filename,
                    );
                } else {
                    p = 0 as *const libc::c_char;
                    if !rev.is_null() {
                        let mut numeric: cbuf = cbuf {
                            string: 0 as *const libc::c_char,
                            size: 0,
                        };
                        if !fully_numeric(&mut numeric, rev, workptr) {
                            current_block_56 = 11743904203796629665;
                        } else {
                            p = numeric.string;
                            current_block_56 = 1854459640724737493;
                        }
                    } else if !((*top).repository.tip).is_null() {
                        match if unlockflag as libc::c_int != 0 {
                            findlock(0 as libc::c_int != 0, &mut delta)
                        } else {
                            0 as libc::c_int
                        } {
                            0 => {
                                current_block_56 = 1592669187010848564;
                                match current_block_56 {
                                    1341191498691135805 => {
                                        p = (*delta).num;
                                    }
                                    _ => {
                                        p = if !((*(*top).repository.r).branch).is_null() {
                                            (*(*top).repository.r).branch
                                        } else {
                                            b"\0" as *const u8 as *const libc::c_char
                                        };
                                    }
                                }
                                current_block_56 = 1854459640724737493;
                            }
                            1 => {
                                current_block_56 = 1341191498691135805;
                                match current_block_56 {
                                    1341191498691135805 => {
                                        p = (*delta).num;
                                    }
                                    _ => {
                                        p = if !((*(*top).repository.r).branch).is_null() {
                                            (*(*top).repository.r).branch
                                        } else {
                                            b"\0" as *const u8 as *const libc::c_char
                                        };
                                    }
                                }
                                current_block_56 = 1854459640724737493;
                            }
                            _ => {
                                current_block_56 = 11743904203796629665;
                            }
                        }
                    } else {
                        current_block_56 = 1854459640724737493;
                    }
                    match current_block_56 {
                        11743904203796629665 => {}
                        _ => {
                            delta = 0 as *mut delta;
                            deltas = 0 as *mut wlink;
                            if !(!p.is_null()
                                && {
                                    delta = gr_revno(p, &mut deltas);
                                    delta.is_null()
                                })
                            {
                                waslocked = !delta.is_null()
                                    && !((*delta).lockedby).is_null();
                                (*top)
                                    .behavior
                                    .inclusive_of_Locker_in_Id_val = unlock(delta);
                                unlocked = (*top).behavior.inclusive_of_Locker_in_Id_val
                                    as libc::c_int & unlockflag as libc::c_int != 0;
                                if !((unlocked as libc::c_int) < waslocked as libc::c_int
                                    && workstat.st_mode
                                        & (0o200 as libc::c_int
                                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                                >> 3 as libc::c_int) as libc::c_uint != 0)
                                {
                                    if !(unlocked as libc::c_int != 0 && !checkaccesslist()) {
                                        if !(0 as libc::c_int
                                            > dorewrite(dounlock, unlocked as libc::c_int))
                                        {
                                            if 0 as libc::c_int <= expmode {
                                                (*top).behavior.kws = expmode;
                                            } else if waslocked as libc::c_int != 0
                                                && (*top).behavior.kws == kwsub_kv as libc::c_int
                                                && (*repo_stat).st_mode
                                                    & !(0o200 as libc::c_int
                                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                                            >> 3 as libc::c_int) as mode_t
                                                    | (if 1 as libc::c_int != 0 {
                                                        0o200 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    }) as libc::c_uint == workstat.st_mode
                                            {
                                                (*top).behavior.kws = kwsub_kvl as libc::c_int;
                                            }
                                            write_desc_maybe((*top).flow.to);
                                            if !(if delta.is_null() {
                                                (workstat.st_size != 0 as libc::c_int as libc::c_long)
                                                    as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < rcsfcmp(
                                                        workptr,
                                                        &mut workstat,
                                                        buildrevision(
                                                            deltas,
                                                            delta,
                                                            0 as *mut FILE,
                                                            0 as libc::c_int != 0,
                                                        ),
                                                        delta,
                                                    )) as libc::c_int
                                            } != 0)
                                            {
                                                if ((*top).behavior.quiet as libc::c_int)
                                                    < unlocked as libc::c_int
                                                {
                                                    aprintf(
                                                        stdout,
                                                        b"rcs -u%s %s\n\0" as *const u8 as *const libc::c_char,
                                                        (*delta).num,
                                                        (*top).repository.filename,
                                                    );
                                                }
                                                if perform as libc::c_int & unlocked as libc::c_int != 0 {
                                                    let mut from: *mut fro = (*top).flow.from;
                                                    (*from)
                                                        .verbatim = *((*(*delta).text).holes)
                                                        .as_mut_ptr()
                                                        .offset(
                                                            ((*(*delta).text).count)
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                                        ) + 2 as libc::c_int as libc::c_long;
                                                    if (*deltas).entry != delta as *mut libc::c_void {
                                                        fro_trundling(1 as libc::c_int != 0, from);
                                                    }
                                                    if 0 as libc::c_int
                                                        > donerewrite(
                                                            1 as libc::c_int,
                                                            file_mtime(Ttimeflag, repo_stat),
                                                        )
                                                    {
                                                        current_block_56 = 11743904203796629665;
                                                    } else {
                                                        current_block_56 = 6367734732029634840;
                                                    }
                                                } else {
                                                    current_block_56 = 6367734732029634840;
                                                }
                                                match current_block_56 {
                                                    11743904203796629665 => {}
                                                    _ => {
                                                        if !(*top).behavior.quiet {
                                                            aprintf(
                                                                stdout,
                                                                b"rm -f %s\n\0" as *const u8 as *const libc::c_char,
                                                                mani_filename,
                                                            );
                                                        }
                                                        fro_zclose(&mut workptr);
                                                        if perform as libc::c_int != 0
                                                            && 0 as libc::c_int > un_link(mani_filename)
                                                        {
                                                            syserror(*__errno_location(), mani_filename);
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
            cleanup(&mut exitstatus, &mut workptr);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    if !(*top).behavior.quiet {
        fclose(stdout);
    }
    gnurcs_goodbye();
    return exitstatus;
}
static mut rcsclean_aka: [uint8_t; 16] = [
    2 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    'c' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    8 as libc::c_int as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'n' as i32 as uint8_t,
];
pub static mut ya_rcsclean: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcsclean_main
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            aka: rcsclean_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};

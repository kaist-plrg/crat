use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    static mut top: *mut top;
    fn checksid(id: *const libc::c_char);
    fn str_save(s: *const libc::c_char) -> *mut libc::c_char;
    fn cgetenv(name: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn getlogin_r(__name: *mut libc::c_char, __name_len: size_t) -> libc::c_int;
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn prepend(x: *const libc::c_void, ls: *mut link, to: *mut divvy) -> *mut link;
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
pub type uid_t = __uid_t;
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
pub struct wlink {
    pub entry: *mut libc::c_void,
    pub next: *mut wlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rcslock {
    pub login: *const libc::c_char,
    pub delta: *mut delta,
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
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
unsafe extern "C" fn ruid() -> uid_t {
    if !(*top).behavior.ruid_cached {
        (*top).behavior.ruid = getuid() as libc::c_int;
        (*top).behavior.ruid_cached = 1 as libc::c_int != 0;
    }
    return (*top).behavior.ruid as uid_t;
}
pub unsafe extern "C" fn stat_mine_p(mut st: *mut stat) -> bool {
    return ruid() == (*st).st_uid;
}
unsafe extern "C" fn euid() -> uid_t {
    if !(*top).behavior.euid_cached {
        (*top).behavior.euid = geteuid() as libc::c_int;
        (*top).behavior.euid_cached = 1 as libc::c_int != 0;
    }
    return (*top).behavior.euid as uid_t;
}
pub unsafe extern "C" fn currently_setuid_p() -> bool {
    return euid() != ruid();
}
unsafe extern "C" fn set_uid_to(mut u: uid_t) {
    if !currently_setuid_p() {
        return;
    }
    if 0 as libc::c_int > seteuid(u) {
        fatal_sys(b"setuid\0" as *const u8 as *const libc::c_char);
    }
    if geteuid() != u {
        if (*top).behavior.already_setuid {
            return;
        }
        (*top).behavior.already_setuid = 1 as libc::c_int != 0;
        if u != 0 {
            generic_fatal(
                0 as *const libc::c_char,
                b"setuid not supported\0" as *const u8 as *const libc::c_char,
            );
        } else {
            generic_fatal(
                0 as *const libc::c_char,
                b"root setuid not supported\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
pub unsafe extern "C" fn nosetid() {
    (*top).behavior.stick_with_euid = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn seteid() {
    if !(*top).behavior.stick_with_euid {
        set_uid_to(euid());
    }
}
pub unsafe extern "C" fn setrid() {
    if !(*top).behavior.stick_with_euid {
        set_uid_to(ruid());
    }
}
pub unsafe extern "C" fn getusername(mut suspicious: bool) -> *const libc::c_char {
    if ((*top).behavior.username).is_null() {
        let mut buf: [libc::c_char; 8192] = [0; 8192];
        if suspicious as libc::c_int != 0
            || {
                (*top)
                    .behavior
                    .username = cgetenv(
                    b"LOGNAME\0" as *const u8 as *const libc::c_char,
                );
                ((*top).behavior.username).is_null()
                    && {
                        (*top)
                            .behavior
                            .username = cgetenv(
                            b"USER\0" as *const u8 as *const libc::c_char,
                        );
                        ((*top).behavior.username).is_null()
                    }
                    && !(0 as libc::c_int
                        == getlogin_r(buf.as_mut_ptr(), 8192 as libc::c_int as size_t)
                        && {
                            (*top).behavior.username = str_save(buf.as_mut_ptr());
                            !((*top).behavior.username).is_null()
                        })
            }
        {
            let mut pwbuf: passwd = passwd {
                pw_name: 0 as *mut libc::c_char,
                pw_passwd: 0 as *mut libc::c_char,
                pw_uid: 0,
                pw_gid: 0,
                pw_gecos: 0 as *mut libc::c_char,
                pw_dir: 0 as *mut libc::c_char,
                pw_shell: 0 as *mut libc::c_char,
            };
            let mut pw: *mut passwd = 0 as *mut passwd;
            if getpwuid_r(
                ruid(),
                &mut pwbuf,
                buf.as_mut_ptr(),
                8192 as libc::c_int as size_t,
                &mut pw,
            ) != 0 || &mut pwbuf as *mut passwd != pw || ((*pw).pw_name).is_null()
            {
                generic_fatal(
                    0 as *const libc::c_char,
                    b"no password entry for userid %d\0" as *const u8
                        as *const libc::c_char,
                    ruid(),
                );
            }
            (*top).behavior.username = str_save((*pw).pw_name);
        }
        checksid((*top).behavior.username);
    }
    return (*top).behavior.username;
}
pub unsafe extern "C" fn getcaller() -> *const libc::c_char {
    return getusername(currently_setuid_p());
}
pub unsafe extern "C" fn caller_login_p(mut login: *const libc::c_char) -> bool {
    return strcmp(getcaller(), login) == 0;
}
pub unsafe extern "C" fn lock_memq(
    mut ls: *mut link,
    mut login: bool,
    mut x: *const libc::c_void,
) -> *mut link {
    let mut rl: *const rcslock = 0 as *const rcslock;
    while !((*ls).next).is_null() {
        rl = (*(*ls).next).entry as *const rcslock;
        if if login as libc::c_int != 0 {
            (strcmp(x as *const libc::c_char, (*rl).login) == 0) as libc::c_int
        } else {
            (x == (*rl).delta as *const libc::c_void) as libc::c_int
        } != 0
        {
            return ls;
        }
        ls = (*ls).next;
    }
    return 0 as *mut link;
}
pub unsafe extern "C" fn lock_on(mut delta: *const delta) -> *const rcslock {
    let mut ls: *mut link = (*(*top).repository.r).locks;
    while !ls.is_null() {
        let mut rl: *const rcslock = (*ls).entry as *const rcslock;
        if delta == (*rl).delta as *const delta {
            return rl;
        }
        ls = (*ls).next;
    }
    return 0 as *const rcslock;
}
pub unsafe extern "C" fn lock_drop(mut box_0: *mut link, mut tp: *mut link) {
    let mut rl: *const rcslock = (*(*tp).next).entry as *const rcslock;
    (*(*rl).delta).lockedby = 0 as *const libc::c_char;
    (*tp).next = (*(*tp).next).next;
    (*(*top).repository.r).locks = (*box_0).next;
}
pub unsafe extern "C" fn addlock_maybe(
    mut delta: *mut delta,
    mut selfsame: bool,
    mut verbose: bool,
) -> libc::c_int {
    let mut rl: *mut rcslock = 0 as *mut rcslock;
    let mut was: *const rcslock = lock_on(delta);
    if !was.is_null() {
        if !selfsame && caller_login_p((*was).login) as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
        if verbose {
            generic_error(
                (*top).repository.filename,
                b"Revision %s is already locked by %s.\0" as *const u8
                    as *const libc::c_char,
                (*delta).num,
                (*was).login,
            );
        }
        return -(1 as libc::c_int);
    }
    rl = alloc(single, ::std::mem::size_of::<rcslock>() as libc::c_ulong)
        as *mut rcslock;
    (*delta).lockedby = getcaller();
    (*rl).login = (*delta).lockedby;
    (*rl).delta = delta;
    (*(*top).repository.r)
        .locks = prepend(
        rl as *const libc::c_void,
        (*(*top).repository.r).locks,
        single,
    );
    return 1 as libc::c_int;
}

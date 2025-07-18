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
    pub type ephemstuff;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn perror(__s: *const libc::c_char);
    static mut top: *mut top;
    fn thank_you_and_goodnight(how: libc::c_int);
    fn __errno_location() -> *mut libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn psiginfo(__pinfo: *const siginfo_t, __s: *const libc::c_char);
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaltstack(__ss: *const stack_t, __oss: *mut stack_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn fatal_sys(who: *const libc::c_char);
    static mut plexus: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn setrid();
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
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
pub struct isr_scratch {
    pub held: sig_atomic_t,
    pub level: sig_atomic_t,
    pub bufinfo: siginfo_t,
    pub held_info: *mut siginfo_t,
    pub access_name: *const libc::c_char,
    pub catching: C2RustUnnamed_2,
    pub be_quiet: *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub regular: bool,
    pub memory_map: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_12,
    pub _timer: C2RustUnnamed_11,
    pub _rt: C2RustUnnamed_10,
    pub _sigchld: C2RustUnnamed_9,
    pub _sigfault: C2RustUnnamed_6,
    pub _sigpoll: C2RustUnnamed_5,
    pub _sigsys: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _addr_bnd: C2RustUnnamed_8,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type sig_atomic_t = __sig_atomic_t;
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
    pub prev: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_14,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub type isr_actions = libc::c_uint;
pub const ISR_CATCHMMAPINTS: isr_actions = 3;
pub const ISR_RESTOREINTS: isr_actions = 2;
pub const ISR_IGNOREINTS: isr_actions = 1;
pub const ISR_CATCHINTS: isr_actions = 0;
pub unsafe extern "C" fn maybe_reset_sigchld() {
    signal(17 as libc::c_int, None);
}
unsafe extern "C" fn werr(mut s: *const libc::c_char) {
    let mut len: ssize_t = 0;
    len = strlen(s) as ssize_t;
    if len == 0 {
        return;
    }
    if len != write(2 as libc::c_int, s as *const libc::c_void, len as size_t) {
        thank_you_and_goodnight((*(*top).program).tyag);
    }
}
pub unsafe extern "C" fn complain_signal(
    mut msg: *const libc::c_char,
    mut signo: libc::c_int,
) {
    werr(msg);
    werr(b": \0" as *const u8 as *const libc::c_char);
    werr(strsignal(signo));
    werr(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn access_page(
    mut scratch: *mut isr_scratch,
    mut filename: *const libc::c_char,
    mut p: *const libc::c_char,
) -> libc::c_char {
    let mut t: libc::c_char = 0;
    (*scratch).access_name = filename;
    ::std::ptr::write_volatile(&mut t as *mut libc::c_char, *p);
    (*scratch).access_name = 0 as *const libc::c_char;
    return t;
}
unsafe extern "C" fn ignore(mut scratch: *mut isr_scratch) {
    ::std::ptr::write_volatile(
        &mut (*scratch).level as *mut sig_atomic_t,
        ::std::ptr::read_volatile::<
            sig_atomic_t,
        >(&(*scratch).level as *const sig_atomic_t) + 1,
    );
    ::std::ptr::read_volatile::<sig_atomic_t>(&(*scratch).level as *const sig_atomic_t);
}
unsafe extern "C" fn catchsigaction(
    mut signo: libc::c_int,
    mut info: *mut siginfo_t,
    mut uc: *mut libc::c_void,
) {
    let mut scratch: *mut isr_scratch = (*top).behavior.isr;
    let mut from_mmap: bool = 7 as libc::c_int != 0 && 7 as libc::c_int == signo;
    if (*scratch).level != 0 {
        ::std::ptr::write_volatile(&mut (*scratch).held as *mut sig_atomic_t, signo);
        if !info.is_null() {
            (*scratch).bufinfo = *info;
            ::std::ptr::write_volatile(
                &mut (*scratch).held_info as *mut *mut siginfo_t,
                &mut (*scratch).bufinfo,
            );
        }
        return;
    }
    ignore(scratch);
    setrid();
    if !*(*scratch).be_quiet {
        if !(from_mmap as libc::c_int != 0 && !((*scratch).access_name).is_null()) {
            let mut nRCS: *mut libc::c_char = b"\nRCS\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            if from_mmap as libc::c_int != 0 && !info.is_null() && (*info).si_errno != 0
            {
                *__errno_location() = (*info).si_errno;
                let fresh0 = nRCS;
                nRCS = nRCS.offset(1);
                perror(fresh0);
            }
            if !info.is_null() {
                psiginfo(info, nRCS);
            } else {
                complain_signal(nRCS, signo);
            }
        }
        werr(b"RCS: \0" as *const u8 as *const libc::c_char);
        if from_mmap {
            if !((*scratch).access_name).is_null() {
                werr((*scratch).access_name);
                werr(b": Permission denied.  \0" as *const u8 as *const libc::c_char);
            } else {
                werr(
                    b"Was a file changed by some other process?  \0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        werr(b"Cleaning up.\n\0" as *const u8 as *const libc::c_char);
    }
    thank_you_and_goodnight((*(*top).program).tyag);
}
unsafe extern "C" fn setup_catchsig(mut count: size_t, mut set: *const libc::c_int) {
    let mut i_0: size_t = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_14 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sig: libc::c_int = 0;
    let mut current_block: u64;
    let mut blocked: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut blocked);
    let mut i: size_t = 0 as libc::c_int as size_t;
    loop {
        if !(i < count) {
            current_block = 4644295000439058019;
            break;
        }
        if 0 as libc::c_int > sigaddset(&mut blocked, *set.offset(i as isize)) {
            current_block = 13132549488237172318;
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        4644295000439058019 => {
            i_0 = 0 as libc::c_int as size_t;
            current_block = 17778012151635330486;
        }
        _ => {}
    }
    loop {
        match current_block {
            13132549488237172318 => {
                fatal_sys(b"signal handling\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                if !(i_0 < count) {
                    break;
                }
                act = sigaction {
                    __sigaction_handler: C2RustUnnamed_14 {
                        sa_handler: None,
                    },
                    sa_mask: sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                sig = *set.offset(i_0 as isize);
                if 0 as libc::c_int > sigaction(sig, 0 as *const sigaction, &mut act) {
                    current_block = 13132549488237172318;
                    continue;
                }
                if ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
                    != act.__sigaction_handler.sa_handler
                {
                    act
                        .__sigaction_handler
                        .sa_sigaction = Some(
                        catchsigaction
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *mut siginfo_t,
                                *mut libc::c_void,
                            ) -> (),
                    );
                    act.sa_flags |= 4 as libc::c_int | 0x8000000 as libc::c_int;
                    act.sa_mask = blocked;
                    if 0 as libc::c_int > sigaction(sig, &mut act, 0 as *mut sigaction) {
                        current_block = 13132549488237172318;
                        continue;
                    }
                }
            }
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
        current_block = 17778012151635330486;
    };
}
pub unsafe extern "C" fn isr_init(mut be_quiet: *mut bool) -> *mut isr_scratch {
    let mut scratch: *mut isr_scratch = zlloc(
        plexus,
        (::std::mem::size_of::<isr_scratch>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    ) as *mut isr_scratch;
    let mut stack_size: size_t = (10 as libc::c_int * 8192 as libc::c_int) as size_t;
    let mut ss: stack_t = {
        let mut init = stack_t {
            ss_sp: alloc(plexus, stack_size),
            ss_flags: 0 as libc::c_int,
            ss_size: stack_size,
        };
        init
    };
    if 0 as libc::c_int > sigaltstack(&mut ss, 0 as *mut stack_t) {
        fatal_sys(b"sigaltstack\0" as *const u8 as *const libc::c_char);
    }
    (*scratch).be_quiet = be_quiet;
    return scratch;
}
pub unsafe extern "C" fn isr_do(mut scratch: *mut isr_scratch, mut action: isr_actions) {
    match action as libc::c_uint {
        0 => {
            let regular: [libc::c_int; 7] = [
                1 as libc::c_int,
                2 as libc::c_int,
                3 as libc::c_int,
                13 as libc::c_int,
                15 as libc::c_int,
                24 as libc::c_int,
                25 as libc::c_int,
            ];
            if !(*scratch).catching.regular {
                (*scratch).catching.regular = 1 as libc::c_int != 0;
                setup_catchsig(
                    (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) as libc::c_int as size_t,
                    regular.as_ptr(),
                );
            }
        }
        1 => {
            ignore(scratch);
        }
        2 => {
            ::std::ptr::write_volatile(
                &mut (*scratch).level as *mut sig_atomic_t,
                ::std::ptr::read_volatile::<
                    sig_atomic_t,
                >(&(*scratch).level as *const sig_atomic_t) - 1,
            );
            if ::std::ptr::read_volatile::<
                sig_atomic_t,
            >(&(*scratch).level as *const sig_atomic_t) == 0 && (*scratch).held != 0
            {
                catchsigaction(
                    (*scratch).held,
                    (*scratch).held_info,
                    0 as *mut libc::c_void,
                );
            }
        }
        3 => {
            let mmapsigs: [libc::c_int; 1] = [7 as libc::c_int];
            if 7 as libc::c_int != 0 && !(*scratch).catching.memory_map {
                (*scratch).catching.memory_map = 1 as libc::c_int != 0;
                setup_catchsig(
                    (::std::mem::size_of::<[libc::c_int; 1]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) as libc::c_int as size_t,
                    mmapsigs.as_ptr(),
                );
            }
        }
        _ => {}
    };
}

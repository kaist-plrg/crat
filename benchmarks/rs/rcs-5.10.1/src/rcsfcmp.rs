use ::libc;
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
    static tiny_ciklog: tinysym;
    fn looking_at(sym: *const tinysym, start: *const libc::c_char) -> bool;
    fn recognize_keyword(string: *const libc::c_char, found: *mut pool_found) -> bool;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn fatal_sys(who: *const libc::c_char);
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_close(f: *mut fro);
    fn fro_try_getbyte(c: *mut libc::c_int, f: *mut fro) -> bool;
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
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_found {
    pub i: libc::c_int,
    pub sym: *const tinysym,
}
pub type markers = libc::c_uint;
pub const State: markers = 10;
pub const Source: markers = 9;
pub const Revision: markers = 8;
pub const RCSfile: markers = 7;
pub const Name: markers = 6;
pub const Log: markers = 5;
pub const Locker: markers = 4;
pub const Id: markers = 3;
pub const Header: markers = 2;
pub const Date: markers = 1;
pub const Author: markers = 0;
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
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
unsafe extern "C" fn discardkeyval(mut c: libc::c_int, mut f: *mut fro) -> libc::c_int {
    loop {
        match c {
            36 | 10 => return c,
            _ => {
                if fro_try_getbyte(&mut c, f) {
                    return -(1 as libc::c_int);
                }
            }
        }
    };
}
pub unsafe extern "C" fn rcsfcmp(
    mut xfp: *mut fro,
    mut xstatp: *const stat,
    mut uname: *const libc::c_char,
    mut delta: *const delta,
) -> libc::c_int {
    let mut current_block: u64;
    let mut xc: libc::c_int = 0;
    let mut uc: libc::c_int = 0;
    let mut xkeyword: [libc::c_char; 10] = [0; 10];
    let mut eqkeyvals: bool = false;
    let mut ufp: *mut fro = 0 as *mut fro;
    let mut xeof: bool = false;
    let mut ueof: bool = false;
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut leaderlen: size_t = 0;
    let mut result: libc::c_int = 0;
    let mut match1: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    let mut ustat: stat = stat {
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
    ufp = fro_open(uname, b"r\0" as *const u8 as *const libc::c_char, &mut ustat);
    if ufp.is_null() {
        fatal_sys(uname);
    }
    ueof = 0 as libc::c_int != 0;
    xeof = ueof;
    if kwsub_o as libc::c_int <= (*top).behavior.kws {
        result = ((*xstatp).st_size != ustat.st_size) as libc::c_int;
        if result == 0 {
            if RM_STDIO as libc::c_int as libc::c_uint != (*xfp).rm as libc::c_uint
                && RM_STDIO as libc::c_int as libc::c_uint != (*ufp).rm as libc::c_uint
            {
                result = (0 as libc::c_int
                    != memcmp(
                        (*xfp).base as *const libc::c_void,
                        (*ufp).base as *const libc::c_void,
                        (*xstatp).st_size as libc::c_ulong,
                    )) as libc::c_int;
                current_block = 3740490469874191334;
            } else {
                loop {
                    if fro_try_getbyte(&mut xc, xfp) {
                        xeof = 1 as libc::c_int != 0;
                    }
                    if fro_try_getbyte(&mut uc, ufp) {
                        ueof = 1 as libc::c_int != 0;
                    }
                    if xeof as libc::c_int | ueof as libc::c_int != 0 {
                        current_block = 3740490469874191334;
                        break;
                    }
                    if xc != uc {
                        current_block = 6517894696834634596;
                        break;
                    }
                }
            }
        } else {
            current_block = 3740490469874191334;
        }
    } else {
        xc = 0 as libc::c_int;
        uc = 0 as libc::c_int;
        leaderlen = 0 as libc::c_int as size_t;
        result = 0 as libc::c_int;
        's_133: loop {
            if xc != '$' as i32 {
                if fro_try_getbyte(&mut xc, xfp) {
                    xeof = 1 as libc::c_int != 0;
                }
                if fro_try_getbyte(&mut uc, ufp) {
                    ueof = 1 as libc::c_int != 0;
                }
                if xeof as libc::c_int | ueof as libc::c_int != 0 {
                    current_block = 3740490469874191334;
                    break;
                }
            } else {
                tp = xkeyword.as_mut_ptr();
                loop {
                    if fro_try_getbyte(&mut xc, xfp) {
                        xeof = 1 as libc::c_int != 0;
                    }
                    if fro_try_getbyte(&mut uc, ufp) {
                        ueof = 1 as libc::c_int != 0;
                    }
                    if xeof as libc::c_int | ueof as libc::c_int != 0 {
                        current_block = 3740490469874191334;
                        break 's_133;
                    }
                    if xc != uc {
                        break;
                    }
                    match xc {
                        10 | 36 | 58 => {
                            break;
                        }
                        _ => {}
                    }
                    if xkeyword.as_mut_ptr().offset(8 as libc::c_int as isize) <= tp {
                        break;
                    }
                    let fresh0 = tp;
                    tp = tp.offset(1);
                    *fresh0 = xc as libc::c_char;
                }
                if (xc == '$' as i32 || xc == ':' as i32)
                    && (uc == '$' as i32 || uc == ':' as i32)
                    && {
                        *tp = xc as libc::c_char;
                        recognize_keyword(xkeyword.as_mut_ptr(), &mut match1)
                            as libc::c_int != 0
                    }
                {
                    result = -(1 as libc::c_int);
                    loop {
                        if xc != uc {
                            xc = discardkeyval(xc, xfp);
                            uc = discardkeyval(uc, ufp);
                            xeof = xc == -(1 as libc::c_int);
                            ueof = uc == -(1 as libc::c_int);
                            if xeof as libc::c_int | ueof as libc::c_int != 0 {
                                current_block = 3740490469874191334;
                                break 's_133;
                            }
                            eqkeyvals = 0 as libc::c_int != 0;
                            break;
                        } else {
                            match xc {
                                10 | 36 => {
                                    eqkeyvals = 1 as libc::c_int != 0;
                                    break;
                                }
                                _ => {
                                    if fro_try_getbyte(&mut xc, xfp) {
                                        xeof = 1 as libc::c_int != 0;
                                    }
                                    if fro_try_getbyte(&mut uc, ufp) {
                                        ueof = 1 as libc::c_int != 0;
                                    }
                                    if xeof as libc::c_int | ueof as libc::c_int != 0 {
                                        current_block = 3740490469874191334;
                                        break 's_133;
                                    }
                                }
                            }
                        }
                    }
                    if xc != uc {
                        current_block = 6517894696834634596;
                        break;
                    }
                    if xc == '$' as i32 {
                        if fro_try_getbyte(&mut xc, xfp) {
                            xeof = 1 as libc::c_int != 0;
                        }
                        if fro_try_getbyte(&mut uc, ufp) {
                            ueof = 1 as libc::c_int != 0;
                        }
                        if xeof as libc::c_int | ueof as libc::c_int != 0 {
                            current_block = 3740490469874191334;
                            break;
                        }
                        if match1.i == Log as libc::c_int {
                            let mut lncnt: libc::c_int = 0;
                            let mut ls: size_t = 0;
                            let mut ccnt: size_t = 0;
                            sp = (*delta).pretty_log.string;
                            ls = (*delta).pretty_log.size;
                            if !looking_at(&tiny_ciklog, (*delta).pretty_log.string) {
                                let mut c1: libc::c_int = 1 as libc::c_int;
                                ccnt = (*top).repository.log_lead.size;
                                loop {
                                    let fresh1 = ccnt;
                                    ccnt = ccnt.wrapping_sub(1);
                                    if !(fresh1 != 0) {
                                        break;
                                    }
                                    c1
                                        += (*((*top).repository.log_lead.string)
                                            .offset(ccnt as isize) as libc::c_int == '\n' as i32)
                                            as libc::c_int;
                                }
                                lncnt = 2 as libc::c_int * c1 + 1 as libc::c_int;
                                loop {
                                    let fresh2 = ls;
                                    ls = ls.wrapping_sub(1);
                                    if !(fresh2 != 0) {
                                        break;
                                    }
                                    let fresh3 = sp;
                                    sp = sp.offset(1);
                                    if *fresh3 as libc::c_int == '\n' as i32 {
                                        lncnt += c1;
                                    }
                                }
                                loop {
                                    if xc == '\n' as i32 {
                                        lncnt -= 1;
                                        if lncnt == 0 as libc::c_int {
                                            break;
                                        }
                                    }
                                    if fro_try_getbyte(&mut xc, xfp) {
                                        current_block = 14072982829676520661;
                                        break 's_133;
                                    }
                                }
                                ccnt = if (*top).behavior.version
                                    < 5 as libc::c_int - 5 as libc::c_int
                                {
                                    (*top).repository.log_lead.size
                                } else {
                                    leaderlen
                                };
                                loop {
                                    if fro_try_getbyte(&mut xc, xfp) {
                                        current_block = 14072982829676520661;
                                        break 's_133;
                                    }
                                    let fresh4 = ccnt;
                                    ccnt = ccnt.wrapping_sub(1);
                                    if !(fresh4 != 0
                                        && (xc != '\n' as i32
                                            || {
                                                c1 -= 1;
                                                c1 != 0
                                            }))
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                    } else if !eqkeyvals {
                        current_block = 6517894696834634596;
                        break;
                    }
                }
            }
            if xc != uc {
                current_block = 6517894696834634596;
                break;
            }
            if xc == '\n' as i32 {
                leaderlen = 0 as libc::c_int as size_t;
            } else {
                leaderlen = leaderlen.wrapping_add(1);
                leaderlen;
            }
        }
    }
    match current_block {
        3740490469874191334 => {
            if xeof as libc::c_int == ueof as libc::c_int {
                current_block = 14072982829676520661;
            } else {
                current_block = 6517894696834634596;
            }
        }
        _ => {}
    }
    match current_block {
        6517894696834634596 => {
            result = 1 as libc::c_int;
        }
        _ => {}
    }
    fro_close(ufp);
    return result;
}

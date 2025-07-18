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
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    static mut top: *mut top;
    static tiny_ciklog: tinysym;
    fn looking_at(sym: *const tinysym, start: *const libc::c_char) -> bool;
    fn recognize_keyword(string: *const libc::c_char, found: *mut pool_found) -> bool;
    fn basefilename(p: *const libc::c_char) -> *const libc::c_char;
    fn getfullRCSname() -> *const libc::c_char;
    static ctab: [tokens; 0];
    fn date2str(
        date: *const libc::c_char,
        datebuf: *mut libc::c_char,
    ) -> *const libc::c_char;
    fn awrite(buf: *const libc::c_char, chars: size_t, f: *mut FILE);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn forget(divvy: *mut divvy);
    fn accumulate_byte(divvy: *mut divvy, c: libc::c_int);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    static mut single: *mut divvy;
    fn make_space(name: *const libc::c_char) -> *mut divvy;
    fn testOerror(o: *mut FILE);
    fn afputc(c: libc::c_int, f: *mut FILE);
    fn newline(f: *mut FILE);
    fn aputs(s: *const libc::c_char, iop: *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const libc::c_char, _: ...);
    fn fro_tello(f: *mut fro) -> off_t;
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut libc::c_int, f: *mut fro) -> bool;
    fn fro_must_getbyte(c: *mut libc::c_int, f: *mut fro);
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
pub type tokens = libc::c_uint;
pub const STRING: tokens = 14;
pub const SEMI: tokens = 13;
pub const NUM: tokens = 12;
pub const ID: tokens = 11;
pub const COLON: tokens = 10;
pub const UNKN: tokens = 9;
pub const SPACE: tokens = 8;
pub const SBEGIN: tokens = 7;
pub const PERIOD: tokens = 6;
pub const Letter: tokens = 5;
pub const LETTER: tokens = 4;
pub const NEWLN: tokens = 3;
pub const IDCHAR: tokens = 2;
pub const DIGIT: tokens = 1;
pub const DELIM: tokens = 0;
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
pub struct expctx {
    pub to: *mut FILE,
    pub rewr: *mut FILE,
    pub from: *mut fro,
    pub delta: *const delta,
    pub delimstuffed: bool,
    pub dolog: bool,
    pub lparts: *mut divvy,
}
unsafe extern "C" fn afilename(mut base: bool, mut out: *mut FILE) {
    let mut filename: *const libc::c_char = if base as libc::c_int != 0 {
        basefilename((*top).repository.filename)
    } else {
        getfullRCSname()
    };
    let mut c: libc::c_char = 0;
    loop {
        let fresh0 = filename;
        filename = filename.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        let mut current_block_7: u64;
        match c as libc::c_int {
            9 => {
                aputs(b"\\t\0" as *const u8 as *const libc::c_char, out);
                current_block_7 = 10599921512955367680;
            }
            10 => {
                aputs(b"\\n\0" as *const u8 as *const libc::c_char, out);
                current_block_7 = 10599921512955367680;
            }
            32 => {
                aputs(b"\\040\0" as *const u8 as *const libc::c_char, out);
                current_block_7 = 10599921512955367680;
            }
            36 => {
                aputs(b"\\044\0" as *const u8 as *const libc::c_char, out);
                current_block_7 = 10599921512955367680;
            }
            92 => {
                if 5 as libc::c_int - 5 as libc::c_int <= (*top).behavior.version {
                    aputs(b"\\\\\0" as *const u8 as *const libc::c_char, out);
                    current_block_7 = 10599921512955367680;
                } else {
                    current_block_7 = 8098838428831486434;
                }
            }
            _ => {
                current_block_7 = 8098838428831486434;
            }
        }
        match current_block_7 {
            8098838428831486434 => {
                if putc(c as libc::c_int, out) == -(1 as libc::c_int) {
                    testOerror(out);
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn keyreplace(mut marker: *mut pool_found, mut ctx: *mut expctx) {
    let mut infile: *mut fro = (*ctx).from;
    let mut out: *mut FILE = (*ctx).to;
    let mut delta: *const delta = (*ctx).delta;
    let mut dolog: bool = (*ctx).dolog;
    let mut delimstuffed: bool = (*ctx).delimstuffed;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut date: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let mut cs: size_t = 0;
    let mut cw: size_t = 0;
    let mut ls: size_t = 0;
    let mut sp1: *const libc::c_char = 0 as *const libc::c_char;
    let mut datebuf: [libc::c_char; 31] = [0; 31];
    let mut RCSv: libc::c_int = 0;
    let mut exp: libc::c_int = 0;
    let mut include_locker: bool = (*top).behavior.inclusive_of_Locker_in_Id_val;
    exp = (*top).behavior.kws;
    date = (*delta).date;
    RCSv = (*top).behavior.version;
    if exp != kwsub_v as libc::c_int {
        aprintf(
            out,
            b"%c%s\0" as *const u8 as *const libc::c_char,
            '$' as i32,
            ((*(*marker).sym).bytes).as_ptr(),
        );
    }
    if exp != kwsub_k as libc::c_int {
        if exp != kwsub_v as libc::c_int {
            aprintf(
                out,
                b"%c%c\0" as *const u8 as *const libc::c_char,
                ':' as i32,
                if (*marker).i == Log as libc::c_int
                    && RCSv < 5 as libc::c_int - 5 as libc::c_int
                {
                    '\t' as i32
                } else {
                    ' ' as i32
                },
            );
        }
        match (*marker).i {
            0 => {
                aputs((*delta).author, out);
            }
            1 => {
                aputs(date2str(date, datebuf.as_mut_ptr()), out);
            }
            3 | 2 => {
                afilename(
                    (*marker).i == Id as libc::c_int
                        || RCSv < 4 as libc::c_int - 5 as libc::c_int,
                    out,
                );
                aprintf(
                    out,
                    b" %s %s %s %s\0" as *const u8 as *const libc::c_char,
                    (*delta).num,
                    date2str(date, datebuf.as_mut_ptr()),
                    (*delta).author,
                    if RCSv == 3 as libc::c_int - 5 as libc::c_int
                        && !((*delta).lockedby).is_null()
                    {
                        b"Locked\0" as *const u8 as *const libc::c_char
                    } else {
                        (*delta).state
                    },
                );
                if !((*delta).lockedby).is_null() {
                    if 5 as libc::c_int - 5 as libc::c_int <= RCSv {
                        if include_locker as libc::c_int != 0
                            || exp == kwsub_kvl as libc::c_int
                        {
                            aprintf(
                                out,
                                b" %s\0" as *const u8 as *const libc::c_char,
                                (*delta).lockedby,
                            );
                        }
                    } else if RCSv == 4 as libc::c_int - 5 as libc::c_int {
                        aprintf(
                            out,
                            b" Locker: %s\0" as *const u8 as *const libc::c_char,
                            (*delta).lockedby,
                        );
                    }
                }
            }
            4 => {
                if !((*delta).lockedby).is_null() {
                    if include_locker as libc::c_int != 0
                        || exp == kwsub_kvl as libc::c_int
                        || RCSv <= 4 as libc::c_int - 5 as libc::c_int
                    {
                        aputs((*delta).lockedby, out);
                    }
                }
            }
            5 | 7 => {
                afilename(1 as libc::c_int != 0, out);
            }
            6 => {
                if !((*delta).name).is_null() {
                    aputs((*delta).name, out);
                }
            }
            8 => {
                aputs((*delta).num, out);
            }
            9 => {
                afilename(0 as libc::c_int != 0, out);
            }
            10 => {
                aputs((*delta).state, out);
            }
            _ => {}
        }
        if exp != kwsub_v as libc::c_int {
            afputc(' ' as i32, out);
        }
    }
    if exp != kwsub_v as libc::c_int {
        afputc('$' as i32, out);
    }
    if (*marker).i == Log as libc::c_int && dolog as libc::c_int != 0 {
        let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
        sp = (*delta).pretty_log.string;
        ls = (*delta).pretty_log.size;
        if looking_at(&tiny_ciklog, (*delta).pretty_log.string) {
            return;
        }
        if RCSv < 5 as libc::c_int - 5 as libc::c_int {
            cp = (*top).repository.log_lead.string;
            cs = (*top).repository.log_lead.size;
        } else {
            let mut current_block_74: u64;
            let mut kdelim_found: bool = 0 as libc::c_int != 0;
            let mut chars_read: off_t = fro_tello(infile);
            c = 0 as libc::c_int;
            cs = 0 as libc::c_int as size_t;
            loop {
                chars_read -= 1;
                if chars_read == 0 {
                    current_block_74 = 8869332144787829186;
                    break;
                }
                fro_move(infile, -(2 as libc::c_int) as off_t);
                fro_must_getbyte(&mut c, infile);
                if c == '\n' as i32 {
                    current_block_74 = 1134115459065347084;
                    break;
                }
                if c == '@' as i32 && delimstuffed as libc::c_int != 0 {
                    chars_read -= 1;
                    if chars_read == 0 {
                        current_block_74 = 1134115459065347084;
                        break;
                    }
                    fro_move(infile, -(2 as libc::c_int) as off_t);
                    fro_must_getbyte(&mut c, infile);
                    if c != '@' as i32 {
                        fro_must_getbyte(&mut c, infile);
                        current_block_74 = 1134115459065347084;
                        break;
                    }
                }
                cs = (cs as libc::c_ulong).wrapping_add(kdelim_found as libc::c_ulong)
                    as size_t as size_t;
                kdelim_found = (kdelim_found as libc::c_int
                    | (c == '$' as i32) as libc::c_int) != 0;
            }
            match current_block_74 {
                1134115459065347084 => {
                    fro_must_getbyte(&mut c, infile);
                }
                _ => {}
            }
            leader = alloc(single, (1 as libc::c_int as libc::c_ulong).wrapping_add(cs))
                as *mut libc::c_char;
            cp = leader;
            cw = 0 as libc::c_int as size_t;
            while cw < cs {
                *leader.offset(cw as isize) = c as libc::c_char;
                if c == '@' as i32 && delimstuffed as libc::c_int != 0 {
                    fro_must_getbyte(&mut c, infile);
                }
                fro_must_getbyte(&mut c, infile);
                cw = cw.wrapping_add(1);
                cw;
            }
            cw = 0 as libc::c_int as size_t;
            while cw < cs {
                if *ctab
                    .as_ptr()
                    .offset(*cp.offset(cw as isize) as libc::c_uchar as isize)
                    as libc::c_uint != SPACE as libc::c_int as libc::c_uint
                {
                    break;
                }
                cw = cw.wrapping_add(1);
                cw;
            }
            if cw.wrapping_add(1 as libc::c_int as libc::c_ulong) < cs
                && *cp
                    .offset(cw.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '*' as i32
                && (*cp.offset(cw as isize) as libc::c_int == '/' as i32
                    || *cp.offset(cw as isize) as libc::c_int == '(' as i32)
            {
                let mut i: size_t = cw.wrapping_add(1 as libc::c_int as libc::c_ulong);
                loop {
                    i = i.wrapping_add(1);
                    if i == cs {
                        generic_warn(
                            0 as *const libc::c_char,
                            b"`%c* $Log' is obsolescent; use ` * $Log'.\0" as *const u8
                                as *const libc::c_char,
                            *cp.offset(cw as isize) as libc::c_int,
                        );
                        *leader.offset(cw as isize) = ' ' as i32 as libc::c_char;
                        break;
                    } else if *ctab
                        .as_ptr()
                        .offset(*cp.offset(i as isize) as libc::c_uchar as isize)
                        as libc::c_uint != SPACE as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                }
            }
            loop {
                fro_must_getbyte(&mut c, infile);
                if !(c != '$' as i32) {
                    break;
                }
            }
        }
        newline(out);
        awrite(cp, cs, out);
        sp1 = date2str(date, datebuf.as_mut_ptr());
        if 5 as libc::c_int - 5 as libc::c_int <= RCSv {
            aprintf(
                out,
                b"Revision %s  %s  %s\0" as *const u8 as *const libc::c_char,
                (*delta).num,
                sp1,
                (*delta).author,
            );
        } else {
            sp1 = strchr(sp1, ' ' as i32);
            aprintf(
                out,
                b"Revision %s  %.*s %s  %s\0" as *const u8 as *const libc::c_char,
                (*delta).num,
                sp1.offset_from(datebuf.as_mut_ptr()) as libc::c_long as libc::c_int,
                datebuf.as_mut_ptr(),
                sp1,
                (*delta).author,
            );
        }
        cw = cs;
        if 5 as libc::c_int - 5 as libc::c_int <= RCSv {
            while cw != 0
                && (*cp
                    .offset(cw.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == ' ' as i32
                    || *cp
                        .offset(
                            cw.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\t' as i32)
            {
                cw = cw.wrapping_sub(1);
                cw;
            }
        }
        loop {
            newline(out);
            awrite(cp, cw, out);
            if ls == 0 {
                break;
            }
            ls = ls.wrapping_sub(1);
            ls;
            let fresh1 = sp;
            sp = sp.offset(1);
            c = *fresh1 as libc::c_int;
            if c != '\n' as i32 {
                awrite(cp.offset(cw as isize), cs.wrapping_sub(cw), out);
                loop {
                    afputc(c, out);
                    if ls == 0 {
                        break;
                    }
                    ls = ls.wrapping_sub(1);
                    ls;
                    let fresh2 = sp;
                    sp = sp.offset(1);
                    c = *fresh2 as libc::c_int;
                    if !(c != '\n' as i32) {
                        break;
                    }
                }
            }
        }
        if !leader.is_null() {
            brush_off(single, leader as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn expandline(mut ctx: *mut expctx) -> libc::c_int {
    let mut current_block: u64;
    let mut lparts: *mut divvy = (*ctx).lparts;
    let mut fin: *mut fro = (*ctx).from;
    let mut delimstuffed: bool = (*ctx).delimstuffed;
    let mut c: libc::c_int = 0;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut frew: *mut FILE = 0 as *mut FILE;
    let mut r: libc::c_int = 0;
    let mut e: bool = false;
    let mut matchresult: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    let mut cooked: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if lparts.is_null() {
        (*ctx).lparts = make_space(b"lparts\0" as *const u8 as *const libc::c_char);
        lparts = (*ctx).lparts;
    }
    out = (*ctx).to;
    frew = (*ctx).rewr;
    forget(lparts);
    e = 0 as libc::c_int != 0;
    r = -(1 as libc::c_int);
    's_49: loop {
        if delimstuffed {
            fro_must_getbyte(&mut c, fin);
            if !frew.is_null() {
                afputc(c, frew);
            }
        } else if fro_try_getbyte(&mut c, fin) {
            current_block = 14126849059734845053;
            break;
        }
        loop {
            match c {
                64 => {
                    if delimstuffed {
                        fro_must_getbyte(&mut c, fin);
                        if !frew.is_null() {
                            afputc(c, frew);
                        }
                        if c != '@' as i32 {
                            current_block = 14126849059734845053;
                            break 's_49;
                        }
                    }
                }
                10 => {
                    if putc(c, out) == -(1 as libc::c_int) {
                        testOerror(out);
                    }
                    r = 2 as libc::c_int;
                    current_block = 14126849059734845053;
                    break 's_49;
                }
                36 => {
                    r = 0 as libc::c_int;
                    accumulate_byte(lparts, '$' as i32);
                    len = 0 as libc::c_int as size_t;
                    loop {
                        if delimstuffed {
                            fro_must_getbyte(&mut c, fin);
                            if !frew.is_null() {
                                afputc(c, frew);
                            }
                        } else if fro_try_getbyte(&mut c, fin) {
                            current_block = 3986075400051247039;
                            break 's_49;
                        }
                        if !(len
                            <= (8 as libc::c_int + 3 as libc::c_int) as libc::c_ulong)
                        {
                            break;
                        }
                        match *ctab.as_ptr().offset(c as isize) as libc::c_uint {
                            4 | 5 => {}
                            _ => {
                                break;
                            }
                        }
                        accumulate_byte(lparts, c);
                        len = len.wrapping_add(1);
                        len;
                    }
                    accumulate_byte(lparts, c);
                    cooked = finish_string(lparts, &mut len);
                    if !recognize_keyword(
                        cooked.offset(1 as libc::c_int as isize),
                        &mut matchresult,
                    ) {
                        *cooked
                            .offset(
                                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) = '\0' as i32 as libc::c_char;
                        aputs(cooked, out);
                        continue;
                    } else {
                        if c == ':' as i32 {
                            loop {
                                if delimstuffed {
                                    fro_must_getbyte(&mut c, fin);
                                    if !frew.is_null() {
                                        afputc(c, frew);
                                    }
                                } else if fro_try_getbyte(&mut c, fin) {
                                    current_block = 3986075400051247039;
                                    break 's_49;
                                }
                                if c == '\n' as i32 || c == '$' as i32 {
                                    break;
                                }
                                accumulate_byte(lparts, c);
                                if !(c == '@' as i32 && delimstuffed as libc::c_int != 0) {
                                    continue;
                                }
                                fro_must_getbyte(&mut c, fin);
                                if !frew.is_null() {
                                    afputc(c, frew);
                                }
                                if c != '@' as i32 {
                                    current_block = 3986075400051247039;
                                    break 's_49;
                                }
                            }
                            if c != '$' as i32 {
                                cooked = finish_string(lparts, &mut len);
                                aputs(cooked, out);
                                continue;
                            } else {
                                cooked = finish_string(lparts, &mut len);
                            }
                        }
                        keyreplace(&mut matchresult, ctx);
                        e = 1 as libc::c_int != 0;
                        break;
                    }
                }
                _ => {}
            }
            if putc(c, out) == -(1 as libc::c_int) {
                testOerror(out);
            }
            r = 0 as libc::c_int;
            break;
        }
    }
    match current_block {
        3986075400051247039 => {
            cooked = finish_string(lparts, &mut len);
            aputs(cooked, out);
        }
        _ => {}
    }
    return r + e as libc::c_int;
}

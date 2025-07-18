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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut top: *mut top;
    static ks_revno: [libc::c_char; 0];
    fn recognize_keyword(string: *const libc::c_char, found: *mut pool_found) -> bool;
    fn checksid(id: *const libc::c_char);
    fn checkssym(sym: *const libc::c_char);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut single: *mut divvy;
    fn accumulate_byte(divvy: *mut divvy, c: libc::c_int);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn intern(
        divvy: *mut divvy,
        s: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_close(f: *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut libc::c_int, f: *mut fro) -> bool;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
unsafe extern "C" fn sorry(
    mut save: bool,
    mut msg: *const libc::c_char,
) -> *mut libc::c_char {
    if save {
        let mut partial: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        partial = finish_string(single, &mut len);
        brush_off(single, partial as *mut libc::c_void);
    }
    if !msg.is_null() {
        generic_error(
            (*top).manifestation.filename,
            b"%s\0" as *const u8 as *const libc::c_char,
            msg,
        );
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn badly_terminated(mut save: bool) -> *mut libc::c_char {
    return sorry(
        save,
        b"badly terminated keyword value\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn get0val(
    mut c: libc::c_int,
    mut fp: *mut fro,
    mut save: bool,
    mut optional: bool,
) -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut got1: bool = false;
    got1 = 0 as libc::c_int != 0;
    loop {
        's_102: {
            match c {
                32 | 9 => {
                    if save {
                        val = finish_string(single, &mut len);
                        if !got1 {
                            brush_off(single, val as *mut libc::c_void);
                            val = 0 as *mut libc::c_char;
                        }
                    }
                    if got1 as libc::c_int != 0 && val.is_null() {
                        val = b"non-NULL\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    return val;
                }
                36 => {
                    if !got1 && optional as libc::c_int != 0 {
                        if !val.is_null() {
                            brush_off(single, val as *mut libc::c_void);
                        }
                        return 0 as *mut libc::c_char;
                    }
                }
                10 | 0 => {}
                _ => {
                    got1 = 1 as libc::c_int != 0;
                    if save {
                        accumulate_byte(single, c);
                    }
                    break 's_102;
                }
            }
            return badly_terminated(save);
        }
        if fro_try_getbyte(&mut c, fp) {
            return badly_terminated(save);
        }
    };
}
unsafe extern "C" fn keepid(mut c: libc::c_int, mut fp: *mut fro) -> *mut libc::c_char {
    let mut maybe: *mut libc::c_char = 0 as *mut libc::c_char;
    if c == 0 {
        if fro_try_getbyte(&mut c, fp) {
            return sorry(1 as libc::c_int != 0, 0 as *const libc::c_char);
        }
    }
    maybe = get0val(c, fp, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    if maybe.is_null() {
        return 0 as *mut libc::c_char;
    }
    checksid(maybe);
    if (*top).flow.erroneous {
        brush_off(single, maybe as *mut libc::c_void);
        maybe = 0 as *mut libc::c_char;
    }
    return maybe;
}
unsafe extern "C" fn getval(
    mut fp: *mut fro,
    mut save: bool,
    mut optional: bool,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    if fro_try_getbyte(&mut c, fp) {
        return badly_terminated(save);
    }
    return get0val(c, fp, save, optional);
}
unsafe extern "C" fn keepdate(mut fp: *mut fro) -> libc::c_int {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    c = 0 as libc::c_int;
    d = getval(fp, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    if !d.is_null() {
        t = getval(fp, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
        if t.is_null() {
            brush_off(single, d as *mut libc::c_void);
        } else {
            if fro_try_getbyte(&mut c, fp) {
                c = 0 as libc::c_int;
            }
            if c == 0 {
                brush_off(single, t as *mut libc::c_void);
            } else {
                let mut buf: [libc::c_char; 64] = [0; 64];
                let mut len: size_t = 0;
                len = snprintf(
                    buf.as_mut_ptr(),
                    64 as libc::c_int as libc::c_ulong,
                    b"%s%s %s%s\0" as *const u8 as *const libc::c_char,
                    if *(*__ctype_b_loc())
                        .offset(
                            *d.offset(0 as libc::c_int as isize) as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *d.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *d.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                    {
                        b"19\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    d,
                    t,
                    if (strchr(t, '-' as i32)).is_null()
                        && (strchr(t, '+' as i32)).is_null()
                    {
                        b"+0000\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                ) as size_t;
                brush_off(single, t as *mut libc::c_void);
                brush_off(single, d as *mut libc::c_void);
                (*top).manifestation.prev.date = intern(single, buf.as_mut_ptr(), len);
            }
        }
    }
    return c;
}
unsafe extern "C" fn keeprev(mut fp: *mut fro) -> *const libc::c_char {
    let mut current_block: u64;
    let mut s: *mut libc::c_char = getval(
        fp,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    if !s.is_null() {
        let mut sp: *const libc::c_char = 0 as *const libc::c_char;
        let mut dotcount: libc::c_int = 0 as libc::c_int;
        sp = s;
        loop {
            match *sp as libc::c_int {
                0 => {
                    if dotcount & 1 as libc::c_int != 0 {
                        current_block = 11760095776775611553;
                        break;
                    } else {
                        current_block = 1917311967535052937;
                        break;
                    }
                }
                46 => {
                    dotcount += 1;
                    dotcount;
                }
                _ => {
                    if !(*(*__ctype_b_loc()).offset(*sp as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                    {
                        current_block = 1917311967535052937;
                        break;
                    }
                }
            }
            sp = sp.offset(1);
            sp;
        }
        match current_block {
            11760095776775611553 => {}
            _ => {
                generic_error(
                    (*top).manifestation.filename,
                    b"%s is not a %s\0" as *const u8 as *const libc::c_char,
                    s,
                    ks_revno.as_ptr(),
                );
                brush_off(single, s as *mut libc::c_void);
                s = 0 as *mut libc::c_char;
            }
        }
    }
    (*top).manifestation.prev.rev = s;
    return (*top).manifestation.prev.rev;
}
pub unsafe extern "C" fn getoldkeys(mut fp: *mut fro) -> bool {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut keyword: [libc::c_char; 9] = [0; 9];
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut needs_closing: bool = false;
    let mut match_0: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    let mut mani_filename: *const libc::c_char = (*top).manifestation.filename;
    if (*top).manifestation.prev.valid {
        return 1 as libc::c_int != 0;
    }
    needs_closing = 0 as libc::c_int != 0;
    if fp.is_null() {
        fp = fro_open(
            mani_filename,
            b"r\0" as *const u8 as *const libc::c_char,
            0 as *mut stat,
        );
        if fp.is_null() {
            syserror(*__errno_location(), mani_filename);
            return 0 as libc::c_int != 0;
        }
        needs_closing = 1 as libc::c_int != 0;
    }
    c = '\0' as i32;
    's_56: loop {
        if c == '$' as i32 {
            loop {
                tp = keyword.as_mut_ptr();
                loop {
                    if fro_try_getbyte(&mut c, fp) {
                        current_block = 13710068785004051242;
                        break 's_56;
                    }
                    match c {
                        10 | 36 | 58 => {
                            break;
                        }
                        _ => {}
                    }
                    if keyword.as_mut_ptr().offset(8 as libc::c_int as isize) <= tp {
                        break;
                    }
                    let fresh0 = tp;
                    tp = tp.offset(1);
                    *fresh0 = c as libc::c_char;
                }
                if !(c == '$' as i32) {
                    break;
                }
            }
            if c != ':' as i32 {
                continue;
            }
            *tp = c as libc::c_char;
            if fro_try_getbyte(&mut c, fp) {
                current_block = 13710068785004051242;
                break;
            }
            match c {
                32 | 9 => {}
                _ => {
                    continue;
                }
            }
            recognize_keyword(keyword.as_mut_ptr(), &mut match_0);
            match match_0.i {
                0 => {
                    (*top).manifestation.prev.author = keepid('\0' as i32, fp);
                    if ((*top).manifestation.prev.author).is_null() {
                        current_block = 10592144176299490509;
                        break;
                    }
                    c = 0 as libc::c_int;
                }
                1 => {
                    c = keepdate(fp);
                    if c == 0 {
                        current_block = 10592144176299490509;
                        break;
                    }
                }
                2 | 3 => {
                    if !(!(getval(fp, 0 as libc::c_int != 0, 0 as libc::c_int != 0))
                        .is_null() && !(keeprev(fp)).is_null()
                        && {
                            c = keepdate(fp);
                            c != 0
                        }
                        && {
                            (*top).manifestation.prev.author = keepid(c, fp);
                            !((*top).manifestation.prev.author).is_null()
                        }
                        && {
                            (*top).manifestation.prev.state = keepid('\0' as i32, fp);
                            !((*top).manifestation.prev.state).is_null()
                        })
                    {
                        current_block = 10592144176299490509;
                        break;
                    }
                    if !(getval(fp, 0 as libc::c_int != 0, 1 as libc::c_int != 0))
                        .is_null()
                        && !(getval(fp, 0 as libc::c_int != 0, 1 as libc::c_int != 0))
                            .is_null()
                    {
                        c = 0 as libc::c_int;
                    } else {
                        if (*top).flow.erroneous {
                            current_block = 10592144176299490509;
                            break;
                        }
                        c = '$' as i32;
                    }
                }
                4 => {
                    getval(fp, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
                    c = 0 as libc::c_int;
                }
                5 | 7 | 9 => {
                    if (getval(fp, 0 as libc::c_int != 0, 0 as libc::c_int != 0))
                        .is_null()
                    {
                        current_block = 10592144176299490509;
                        break;
                    }
                    c = 0 as libc::c_int;
                }
                6 => {
                    (*top)
                        .manifestation
                        .prev
                        .name = getval(fp, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
                    if !((*top).manifestation.prev.name).is_null()
                        && *(*top).manifestation.prev.name as libc::c_int != 0
                    {
                        checkssym((*top).manifestation.prev.name);
                    }
                    c = 0 as libc::c_int;
                }
                8 => {
                    if (keeprev(fp)).is_null() {
                        current_block = 10592144176299490509;
                        break;
                    }
                    c = 0 as libc::c_int;
                }
                10 => {
                    (*top).manifestation.prev.state = keepid('\0' as i32, fp);
                    if ((*top).manifestation.prev.state).is_null() {
                        current_block = 10592144176299490509;
                        break;
                    }
                    c = 0 as libc::c_int;
                }
                _ => {
                    continue;
                }
            }
            if c == 0 {
                if fro_try_getbyte(&mut c, fp) {
                    c = 0 as libc::c_int;
                }
            }
            if c != '$' as i32 {
                generic_error(
                    (*top).manifestation.filename,
                    b"closing %c missing on keyword\0" as *const u8
                        as *const libc::c_char,
                    '$' as i32,
                );
                current_block = 10592144176299490509;
                break;
            } else if !((*top).manifestation.prev.name).is_null()
                && !((*top).manifestation.prev.author).is_null()
                && !((*top).manifestation.prev.date).is_null()
                && !((*top).manifestation.prev.rev).is_null()
                && !((*top).manifestation.prev.state).is_null()
            {
                current_block = 13710068785004051242;
                break;
            }
        }
        if fro_try_getbyte(&mut c, fp) {
            current_block = 13710068785004051242;
            break;
        }
    }
    match current_block {
        10592144176299490509 => return 0 as libc::c_int != 0,
        _ => {
            if needs_closing {
                fro_close(fp);
            } else {
                fro_move(fp, 0 as libc::c_int as off_t);
            }
            if !((*top).manifestation.prev.name).is_null()
                && *(*top).manifestation.prev.name == 0
            {
                (*top).manifestation.prev.name = 0 as *mut libc::c_char;
            }
            if !((*top).manifestation.prev.author).is_null()
                && *(*top).manifestation.prev.author == 0
            {
                (*top).manifestation.prev.author = 0 as *mut libc::c_char;
            }
            if !((*top).manifestation.prev.date).is_null()
                && *(*top).manifestation.prev.date == 0
            {
                (*top).manifestation.prev.date = 0 as *mut libc::c_char;
            }
            if !((*top).manifestation.prev.rev).is_null()
                && *(*top).manifestation.prev.rev == 0
            {
                (*top).manifestation.prev.rev = 0 as *mut libc::c_char;
            }
            if !((*top).manifestation.prev.state).is_null()
                && *(*top).manifestation.prev.state == 0
            {
                (*top).manifestation.prev.state = 0 as *mut libc::c_char;
            }
            (*top).manifestation.prev.valid = 1 as libc::c_int != 0;
            return 1 as libc::c_int != 0;
        }
    };
}

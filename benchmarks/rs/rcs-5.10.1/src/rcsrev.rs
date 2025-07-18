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
    static ks_revno: [libc::c_char; 0];
    fn getoldkeys(fp: *mut fro) -> bool;
    static ctab: [tokens; 0];
    fn date2str(
        date: *const libc::c_char,
        datebuf: *mut libc::c_char,
    ) -> *const libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn accf(divvy: *mut divvy, fmt: *const libc::c_char, _: ...);
    fn accumulate_byte(divvy: *mut divvy, c: libc::c_int);
    fn accumulate_range(
        divvy: *mut divvy,
        beg: *const libc::c_char,
        end: *const libc::c_char,
    );
    fn accs(divvy: *mut divvy, string: *const libc::c_char);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
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
pub struct wlink {
    pub entry: *mut libc::c_void,
    pub next: *mut wlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdef {
    pub meaningful: *const libc::c_char,
    pub underlying: *const libc::c_char,
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
unsafe extern "C" fn split(
    mut s: *const libc::c_char,
    mut lastdot: *mut *const libc::c_char,
) -> libc::c_int {
    let mut count: size_t = 0;
    *lastdot = 0 as *const libc::c_char;
    if s.is_null() || *s == 0 {
        return 0 as libc::c_int;
    }
    count = 1 as libc::c_int as size_t;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        if *fresh0 as libc::c_int == '.' as i32 {
            *lastdot = s.offset(-(1 as libc::c_int as isize));
            count = count.wrapping_add(1);
            count;
        }
        if !(*s != 0) {
            break;
        }
    }
    return count as libc::c_int;
}
pub unsafe extern "C" fn countnumflds(mut s: *const libc::c_char) -> libc::c_int {
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: libc::c_int = 0;
    sp = s;
    if sp.is_null() || *sp == 0 {
        return 0 as libc::c_int;
    }
    count = 1 as libc::c_int;
    loop {
        let fresh1 = sp;
        sp = sp.offset(1);
        if *fresh1 as libc::c_int == '.' as i32 {
            count += 1;
            count;
        }
        if !(*sp != 0) {
            break;
        }
    }
    return count;
}
unsafe extern "C" fn accumulate_branchno(
    mut space: *mut divvy,
    mut revno: *const libc::c_char,
) {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut nfields: libc::c_int = split(revno, &mut end);
    if nfields & 1 as libc::c_int != 0 {
        accs(space, revno);
    } else {
        accumulate_range(space, revno, end);
    };
}
pub unsafe extern "C" fn take(
    mut count: size_t,
    mut ref_0: *const libc::c_char,
) -> cbuf {
    let mut rv: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut end: *const libc::c_char = ref_0;
    if count == 0 {
        count = (-(2 as libc::c_int) as libc::c_uint)
            .wrapping_add(
                1 as libc::c_uint
                    | (1 as libc::c_int + countnumflds(ref_0)) as libc::c_uint,
            ) as size_t;
    }
    loop {
        let fresh2 = count;
        count = count.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        while *end as libc::c_int != 0
            && {
                let fresh3 = end;
                end = end.offset(1);
                '.' as i32 != *fresh3 as libc::c_int
            }
        {}
    }
    accumulate_range(
        single,
        ref_0,
        if *end as libc::c_int != 0 {
            end.offset(-(1 as libc::c_int as isize))
        } else {
            end
        },
    );
    rv.string = finish_string(single, &mut rv.size);
    return rv;
}
pub unsafe extern "C" fn cmpnum(
    mut num1: *const libc::c_char,
    mut num2: *const libc::c_char,
) -> libc::c_int {
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    let mut d1: size_t = 0;
    let mut d2: size_t = 0;
    let mut r: libc::c_int = 0;
    s1 = if !num1.is_null() { num1 } else { b"\0" as *const u8 as *const libc::c_char };
    s2 = if !num2.is_null() { num2 } else { b"\0" as *const u8 as *const libc::c_char };
    loop {
        if *s1 == 0 {
            return *s2 as libc::c_uchar as libc::c_int;
        }
        if *s2 == 0 {
            return -(1 as libc::c_int);
        }
        while *s1 as libc::c_int == '0' as i32 {
            s1 = s1.offset(1);
            s1;
        }
        while *s2 as libc::c_int == '0' as i32 {
            s2 = s2.offset(1);
            s2;
        }
        d1 = 0 as libc::c_int as size_t;
        while *(*__ctype_b_loc()).offset(*s1.offset(d1 as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            d1 = d1.wrapping_add(1);
            d1;
        }
        d2 = 0 as libc::c_int as size_t;
        while *(*__ctype_b_loc()).offset(*s2.offset(d2 as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            d2 = d2.wrapping_add(1);
            d2;
        }
        if d1 != d2 {
            return if d1 < d2 { -(1 as libc::c_int) } else { 1 as libc::c_int };
        }
        r = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, d1);
        if r != 0 {
            return r;
        }
        s1 = s1.offset(d1 as isize);
        s2 = s2.offset(d1 as isize);
        if *s1 != 0 {
            s1 = s1.offset(1);
            s1;
        }
        if *s2 != 0 {
            s2 = s2.offset(1);
            s2;
        }
    };
}
pub unsafe extern "C" fn cmpnumfld(
    mut num1: *const libc::c_char,
    mut num2: *const libc::c_char,
    mut fld: libc::c_int,
) -> libc::c_int {
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    let mut d1: size_t = 0;
    let mut d2: size_t = 0;
    s1 = num1;
    s2 = num2;
    loop {
        fld -= 1;
        if !(fld != 0) {
            break;
        }
        loop {
            let fresh4 = s1;
            s1 = s1.offset(1);
            if !(*fresh4 as libc::c_int != '.' as i32) {
                break;
            }
        }
        loop {
            let fresh5 = s2;
            s2 = s2.offset(1);
            if !(*fresh5 as libc::c_int != '.' as i32) {
                break;
            }
        }
    }
    while *s1 as libc::c_int == '0' as i32 {
        s1 = s1.offset(1);
        s1;
    }
    d1 = 0 as libc::c_int as size_t;
    while *(*__ctype_b_loc()).offset(*s1.offset(d1 as isize) as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        d1 = d1.wrapping_add(1);
        d1;
    }
    while *s2 as libc::c_int == '0' as i32 {
        s2 = s2.offset(1);
        s2;
    }
    d2 = 0 as libc::c_int as size_t;
    while *(*__ctype_b_loc()).offset(*s2.offset(d2 as isize) as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        d2 = d2.wrapping_add(1);
        d2;
    }
    return if d1 < d2 {
        -(1 as libc::c_int)
    } else if d1 == d2 {
        memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, d1)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn normalizeyear(
    mut date: *const libc::c_char,
    mut year: *mut libc::c_char,
) -> *const libc::c_char {
    if *(*__ctype_b_loc())
        .offset(*date.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*date.offset(1 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        && *(*__ctype_b_loc())
            .offset(*date.offset(2 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        *year.offset(0 as libc::c_int as isize) = '1' as i32 as libc::c_char;
        *year.offset(1 as libc::c_int as isize) = '9' as i32 as libc::c_char;
        *year
            .offset(2 as libc::c_int as isize) = *date.offset(0 as libc::c_int as isize);
        *year
            .offset(3 as libc::c_int as isize) = *date.offset(1 as libc::c_int as isize);
        *year.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return year as *const libc::c_char;
    } else {
        return date
    };
}
pub unsafe extern "C" fn cmpdate(
    mut d1: *const libc::c_char,
    mut d2: *const libc::c_char,
) -> libc::c_int {
    let mut year1: [libc::c_char; 5] = [0; 5];
    let mut year2: [libc::c_char; 5] = [0; 5];
    let mut r: libc::c_int = cmpnumfld(
        normalizeyear(d1, year1.as_mut_ptr()),
        normalizeyear(d2, year2.as_mut_ptr()),
        1 as libc::c_int,
    );
    if r != 0 {
        return r
    } else {
        while *(*__ctype_b_loc()).offset(*d1 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            d1 = d1.offset(1);
            d1;
        }
        d1 = d1.offset((*d1 as libc::c_int == '.' as i32) as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*d2 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            d2 = d2.offset(1);
            d2;
        }
        d2 = d2.offset((*d2 as libc::c_int == '.' as i32) as libc::c_int as isize);
        return cmpnum(d1, d2);
    };
}
unsafe extern "C" fn cantfindbranch(
    mut revno: *const libc::c_char,
    mut date: *const libc::c_char,
    mut author: *const libc::c_char,
    mut state: *const libc::c_char,
) {
    let mut datebuf: [libc::c_char; 31] = [0; 31];
    generic_error(
        (*top).repository.filename,
        b"No revision on branch %s has%s%s%s%s%s%s.\0" as *const u8
            as *const libc::c_char,
        revno,
        if !date.is_null() {
            b" a date before \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !date.is_null() {
            date2str(date, datebuf.as_mut_ptr())
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !author.is_null() {
            (b" and author \0" as *const u8 as *const libc::c_char)
                .offset(
                    (if !date.is_null() { 0 as libc::c_int } else { 4 as libc::c_int })
                        as isize,
                )
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !author.is_null() {
            author
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !state.is_null() {
            (b" and state \0" as *const u8 as *const libc::c_char)
                .offset(
                    (if !date.is_null() || !author.is_null() {
                        0 as libc::c_int
                    } else {
                        4 as libc::c_int
                    }) as isize,
                )
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !state.is_null() { state } else { b"\0" as *const u8 as *const libc::c_char },
    );
}
unsafe extern "C" fn absent(mut revno: *const libc::c_char, mut field: libc::c_int) {
    generic_error(
        (*top).repository.filename,
        b"%s %s absent\0" as *const u8 as *const libc::c_char,
        if field & 1 as libc::c_int != 0 {
            b"revision\0" as *const u8 as *const libc::c_char
        } else {
            b"branch\0" as *const u8 as *const libc::c_char
        },
        (take(field as size_t, revno)).string,
    );
}
pub unsafe extern "C" fn compartial(
    mut num1: *const libc::c_char,
    mut num2: *const libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    let mut d1: size_t = 0;
    let mut d2: size_t = 0;
    let mut r: libc::c_int = 0;
    s1 = num1;
    s2 = num2;
    if s1.is_null() {
        return 1 as libc::c_int;
    }
    if s2.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        if *s1 == 0 {
            return 1 as libc::c_int;
        }
        if *s2 == 0 {
            return -(1 as libc::c_int);
        }
        while *s1 as libc::c_int == '0' as i32 {
            s1 = s1.offset(1);
            s1;
        }
        d1 = 0 as libc::c_int as size_t;
        while *(*__ctype_b_loc()).offset(*s1.offset(d1 as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            d1 = d1.wrapping_add(1);
            d1;
        }
        while *s2 as libc::c_int == '0' as i32 {
            s2 = s2.offset(1);
            s2;
        }
        d2 = 0 as libc::c_int as size_t;
        while *(*__ctype_b_loc()).offset(*s2.offset(d2 as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            d2 = d2.wrapping_add(1);
            d2;
        }
        if d1 != d2 {
            return if d1 < d2 { -(1 as libc::c_int) } else { 1 as libc::c_int };
        }
        r = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, d1);
        if r != 0 {
            return r;
        }
        length -= 1;
        if length == 0 {
            return 0 as libc::c_int;
        }
        s1 = s1.offset(d1 as isize);
        s2 = s2.offset(d1 as isize);
        if *s1 as libc::c_int == '.' as i32 {
            s1 = s1.offset(1);
            s1;
        }
        if *s2 as libc::c_int == '.' as i32 {
            s2 = s2.offset(1);
            s2;
        }
    };
}
unsafe extern "C" fn store1(mut store: *mut *mut *mut wlink, mut next: *mut delta) {
    let mut p: *mut wlink = 0 as *mut wlink;
    p = alloc(single, ::std::mem::size_of::<wlink>() as libc::c_ulong) as *mut wlink;
    (*p).entry = next as *mut libc::c_void;
    **store = p;
    *store = &mut (*p).next;
}
unsafe extern "C" fn genbranch(
    mut bpoint: *const delta,
    mut revno: *const libc::c_char,
    mut length: libc::c_int,
    mut date: *const libc::c_char,
    mut author: *const libc::c_char,
    mut state: *const libc::c_char,
    mut store: *mut *mut wlink,
) -> *mut delta {
    let mut field: libc::c_int = 0;
    let mut d: *mut delta = 0 as *mut delta;
    let mut trail: *mut delta = 0 as *mut delta;
    let mut bhead: *const wlink = 0 as *const wlink;
    let mut result: libc::c_int = 0;
    let mut datebuf: [libc::c_char; 31] = [0; 31];
    field = 3 as libc::c_int;
    bhead = (*bpoint).branches;
    loop {
        if bhead.is_null() {
            generic_error(
                (*top).repository.filename,
                b"no side branches present for %s\0" as *const u8 as *const libc::c_char,
                (take((field - 1 as libc::c_int) as size_t, revno)).string,
            );
            return 0 as *mut delta;
        }
        loop {
            d = (*bhead).entry as *mut delta;
            result = cmpnumfld(revno, (*d).num, field);
            if !((0 as libc::c_int) < result) {
                break;
            }
            bhead = (*bhead).next;
            if bhead.is_null() {
                generic_error(
                    (*top).repository.filename,
                    b"branch number %s too high\0" as *const u8 as *const libc::c_char,
                    (take(field as size_t, revno)).string,
                );
                return 0 as *mut delta;
            }
        }
        if result < 0 as libc::c_int {
            absent(revno, field);
            return 0 as *mut delta;
        }
        d = (*bhead).entry as *mut delta;
        if length == field {
            trail = 0 as *mut delta;
            loop {
                if (date.is_null() || !(0 as libc::c_int > cmpdate(date, (*d).date)))
                    && (author.is_null() || strcmp(author, (*d).author) == 0)
                    && (state.is_null() || strcmp(state, (*d).state) == 0)
                {
                    trail = d;
                }
                d = (*d).ilk;
                if d.is_null() {
                    break;
                }
            }
            if trail.is_null() {
                cantfindbranch(revno, date, author, state);
                return 0 as *mut delta;
            } else {
                d = (*bhead).entry as *mut delta;
                while d != trail {
                    if !store.is_null() {
                        store1(&mut store, d);
                    }
                    d = (*d).ilk;
                }
                if !store.is_null() {
                    store1(&mut store, d);
                }
            }
            if !store.is_null() {
                *store = 0 as *mut wlink;
            }
            return d;
        }
        if 0 as libc::c_int > cmpnumfld(revno, (*d).num, 1 as libc::c_int + field) {
            generic_error(
                (*top).repository.filename,
                b"%s %s too low\0" as *const u8 as *const libc::c_char,
                ks_revno.as_ptr(),
                (take((field + 1 as libc::c_int) as size_t, revno)).string,
            );
            return 0 as *mut delta;
        }
        loop {
            if !store.is_null() {
                store1(&mut store, d);
            }
            trail = d;
            d = (*d).ilk;
            if !(!d.is_null()
                && !(0 as libc::c_int
                    > cmpnumfld(revno, (*d).num, 1 as libc::c_int + field)))
            {
                break;
            }
        }
        if length > field + 1 as libc::c_int
            && !(0 as libc::c_int
                == cmpnumfld(revno, (*trail).num, 1 as libc::c_int + field))
        {
            absent(revno, field + 1 as libc::c_int);
            return 0 as *mut delta;
        }
        if length == field + 1 as libc::c_int {
            if !date.is_null() && 0 as libc::c_int > cmpdate(date, (*trail).date) {
                generic_error(
                    (*top).repository.filename,
                    b"Revision %s has date %s.\0" as *const u8 as *const libc::c_char,
                    (*trail).num,
                    date2str((*trail).date, datebuf.as_mut_ptr()),
                );
                return 0 as *mut delta;
            }
            if !author.is_null() && strcmp(author, (*trail).author) != 0 {
                generic_error(
                    (*top).repository.filename,
                    b"Revision %s has author %s.\0" as *const u8 as *const libc::c_char,
                    (*trail).num,
                    (*trail).author,
                );
                return 0 as *mut delta;
            }
            if !state.is_null() && strcmp(state, (*trail).state) != 0 {
                generic_error(
                    (*top).repository.filename,
                    b"Revision %s has state %s.\0" as *const u8 as *const libc::c_char,
                    (*trail).num,
                    if !((*trail).state).is_null() {
                        (*trail).state
                    } else {
                        b"<empty>\0" as *const u8 as *const libc::c_char
                    },
                );
                return 0 as *mut delta;
            }
        }
        bhead = (*trail).branches;
        field += 2 as libc::c_int;
        if !(field <= length) {
            break;
        }
    }
    if !store.is_null() {
        *store = 0 as *mut wlink;
    }
    return trail;
}
pub unsafe extern "C" fn genrevs(
    mut revno: *const libc::c_char,
    mut date: *const libc::c_char,
    mut author: *const libc::c_char,
    mut state: *const libc::c_char,
    mut store: *mut *mut wlink,
) -> *mut delta {
    let mut current_block: u64;
    let mut length: libc::c_int = 0;
    let mut d: *mut delta = 0 as *mut delta;
    let mut result: libc::c_int = 0;
    let mut branchnum: *const libc::c_char = 0 as *const libc::c_char;
    let mut datebuf: [libc::c_char; 31] = [0; 31];
    d = (*top).repository.tip;
    if d.is_null() {
        generic_error(
            (*top).repository.filename,
            b"RCS file empty\0" as *const u8 as *const libc::c_char,
        );
    } else {
        length = countnumflds(revno);
        if length >= 1 as libc::c_int {
            loop {
                result = cmpnumfld(revno, (*d).num, 1 as libc::c_int);
                if !(result < 0 as libc::c_int) {
                    current_block = 11650488183268122163;
                    break;
                }
                if !store.is_null() {
                    store1(&mut store, d);
                }
                d = (*d).ilk;
                if !d.is_null() {
                    continue;
                }
                generic_error(
                    (*top).repository.filename,
                    b"branch number %s too low\0" as *const u8 as *const libc::c_char,
                    (take(1 as libc::c_int as size_t, revno)).string,
                );
                current_block = 8989168935509356198;
                break;
            }
            match current_block {
                8989168935509356198 => {}
                _ => {
                    if result > 0 as libc::c_int {
                        absent(revno, 1 as libc::c_int);
                        current_block = 8989168935509356198;
                    } else {
                        current_block = 10599921512955367680;
                    }
                }
            }
        } else {
            current_block = 10599921512955367680;
        }
        match current_block {
            8989168935509356198 => {}
            _ => {
                if length <= 1 as libc::c_int {
                    branchnum = (*d).num;
                    while !d.is_null()
                        && 0 as libc::c_int
                            == cmpnumfld(branchnum, (*d).num, 1 as libc::c_int)
                        && (!date.is_null()
                            && 0 as libc::c_int > cmpdate(date, (*d).date)
                            || !author.is_null() && strcmp(author, (*d).author) != 0
                            || !state.is_null() && strcmp(state, (*d).state) != 0)
                    {
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        d = (*d).ilk;
                    }
                    if d.is_null()
                        || !(0 as libc::c_int
                            == cmpnumfld(branchnum, (*d).num, 1 as libc::c_int))
                    {
                        cantfindbranch(
                            if length != 0 {
                                revno
                            } else {
                                (take(1 as libc::c_int as size_t, branchnum)).string
                            },
                            date,
                            author,
                            state,
                        );
                    } else {
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        if !store.is_null() {
                            *store = 0 as *mut wlink;
                        }
                        return d;
                    }
                } else {
                    loop {
                        result = cmpnumfld(revno, (*d).num, 2 as libc::c_int);
                        if !(result < 0 as libc::c_int
                            && 0 as libc::c_int
                                == cmpnumfld(revno, (*d).num, 1 as libc::c_int))
                        {
                            break;
                        }
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        d = (*d).ilk;
                        if d.is_null() {
                            break;
                        }
                    }
                    if d.is_null()
                        || !(0 as libc::c_int
                            == cmpnumfld(revno, (*d).num, 1 as libc::c_int))
                    {
                        generic_error(
                            (*top).repository.filename,
                            b"%s %s too low\0" as *const u8 as *const libc::c_char,
                            ks_revno.as_ptr(),
                            (take(2 as libc::c_int as size_t, revno)).string,
                        );
                    } else if length > 2 as libc::c_int && result != 0 as libc::c_int {
                        absent(revno, 2 as libc::c_int);
                    } else {
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        if length > 2 as libc::c_int {
                            return genbranch(
                                d,
                                revno,
                                length,
                                date,
                                author,
                                state,
                                store,
                            )
                        } else {
                            if !date.is_null()
                                && 0 as libc::c_int > cmpdate(date, (*d).date)
                            {
                                generic_error(
                                    (*top).repository.filename,
                                    b"Revision %s has date %s.\0" as *const u8
                                        as *const libc::c_char,
                                    (*d).num,
                                    date2str((*d).date, datebuf.as_mut_ptr()),
                                );
                                return 0 as *mut delta;
                            }
                            if !author.is_null() && strcmp(author, (*d).author) != 0 {
                                generic_error(
                                    (*top).repository.filename,
                                    b"Revision %s has author %s.\0" as *const u8
                                        as *const libc::c_char,
                                    (*d).num,
                                    (*d).author,
                                );
                                return 0 as *mut delta;
                            }
                            if !state.is_null() && strcmp(state, (*d).state) != 0 {
                                generic_error(
                                    (*top).repository.filename,
                                    b"Revision %s has state %s.\0" as *const u8
                                        as *const libc::c_char,
                                    (*d).num,
                                    if !((*d).state).is_null() {
                                        (*d).state
                                    } else {
                                        b"<empty>\0" as *const u8 as *const libc::c_char
                                    },
                                );
                                return 0 as *mut delta;
                            }
                            if !store.is_null() {
                                *store = 0 as *mut wlink;
                            }
                            return d;
                        }
                    }
                }
            }
        }
    }
    return 0 as *mut delta;
}
pub unsafe extern "C" fn gr_revno(
    mut revno: *const libc::c_char,
    mut store: *mut *mut wlink,
) -> *mut delta {
    return genrevs(
        revno,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        store,
    );
}
pub unsafe extern "C" fn delta_from_ref(mut ref_0: *const libc::c_char) -> *mut delta {
    return genrevs(
        ref_0,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *mut *mut wlink,
    );
}
unsafe extern "C" fn rev_from_symbol(mut id: *const cbuf) -> *const libc::c_char {
    let mut ls: *mut link = (*(*top).repository.r).symbols;
    while !ls.is_null() {
        let mut d: *const symdef = (*ls).entry as *const symdef;
        if '\0' as i32 == *((*d).meaningful).offset((*id).size as isize) as libc::c_int
            && strncmp((*d).meaningful, (*id).string, (*id).size) == 0
        {
            return (*d).underlying;
        }
        ls = (*ls).next;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn lookupsym(mut id: *const libc::c_char) -> *const libc::c_char {
    let mut identifier: cbuf = {
        let mut init = cbuf {
            string: id,
            size: strlen(id),
        };
        init
    };
    return rev_from_symbol(&mut identifier);
}
unsafe extern "C" fn branchtip(mut branch: *const libc::c_char) -> *const libc::c_char {
    let mut h: *mut delta = 0 as *mut delta;
    h = delta_from_ref(branch);
    return if !h.is_null() { (*h).num } else { 0 as *const libc::c_char };
}
pub unsafe extern "C" fn fully_numeric(
    mut ans: *mut cbuf,
    mut source: *const libc::c_char,
    mut fp: *mut fro,
) -> bool {
    let mut current_block: u64;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut bp: *const libc::c_char = 0 as *const libc::c_char;
    let mut dots: libc::c_int = 0;
    let mut ugh: *mut libc::c_char = 0 as *mut libc::c_char;
    sp = source;
    if !(sp.is_null() || *sp == 0) {
        if *sp.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
            && *sp.offset(1 as libc::c_int as isize) == 0
        {
            if !getoldkeys(fp) {
                current_block = 10628954987011324951;
            } else if ((*top).manifestation.prev.rev).is_null() {
                generic_error(
                    (*top).manifestation.filename,
                    b"working file lacks %s\0" as *const u8 as *const libc::c_char,
                    ks_revno.as_ptr(),
                );
                current_block = 10628954987011324951;
            } else {
                accs(single, (*top).manifestation.prev.rev);
                current_block = 14966503543959925615;
            }
        } else {
            dots = 0 as libc::c_int;
            loop {
                let mut was: *const libc::c_char = sp;
                let mut id: bool = 0 as libc::c_int != 0;
                loop {
                    match *ctab.as_ptr().offset(*sp as libc::c_uchar as isize)
                        as libc::c_uint
                    {
                        2 | 4 | 5 => {
                            id = 1 as libc::c_int != 0;
                        }
                        1 => {}
                        _ => {
                            break;
                        }
                    }
                    sp = sp.offset(1);
                    sp;
                }
                if id {
                    let mut orig: cbuf = {
                        let mut init = cbuf {
                            string: was,
                            size: sp.offset_from(was) as libc::c_long as size_t,
                        };
                        init
                    };
                    let mut expanded: *const libc::c_char = rev_from_symbol(&mut orig);
                    if expanded.is_null() {
                        generic_error(
                            (*top).repository.filename,
                            b"Symbolic name `%s' is undefined.\0" as *const u8
                                as *const libc::c_char,
                            was,
                        );
                        current_block = 10628954987011324951;
                        break;
                    } else {
                        accs(single, expanded);
                    }
                } else {
                    if was != sp {
                        accumulate_range(single, was, sp);
                        bp = was;
                    }
                    while '0' as i32
                        == *sp.offset(0 as libc::c_int as isize) as libc::c_int
                        && *(*__ctype_b_loc())
                            .offset(
                                *sp.offset(1 as libc::c_int as isize) as libc::c_int
                                    as isize,
                            ) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        sp = sp.offset(1);
                        sp;
                    }
                    if bp.is_null() {
                        let mut s: libc::c_int = 0 as libc::c_int;
                        if s != 0 || *sp as libc::c_int != '.' as i32 {
                            current_block = 10399321362245223758;
                            break;
                        }
                        let mut b: *const libc::c_char = 0 as *const libc::c_char;
                        let mut tip: *mut delta = 0 as *mut delta;
                        if !((*(*top).repository.r).branch).is_null() {
                            b = (*(*top).repository.r).branch;
                        } else {
                            tip = (*top).repository.tip;
                            if tip.is_null() {
                                current_block = 10399321362245223758;
                                break;
                            }
                            b = (*tip).num;
                        }
                        ugh = finish_string(single, &mut (*ans).size);
                        (*ans).string = ugh;
                        if !ugh.is_null() {
                            brush_off(single, ugh as *mut libc::c_void);
                        }
                        accumulate_branchno(single, b);
                    }
                }
                let fresh6 = sp;
                sp = sp.offset(1);
                match *fresh6 as libc::c_int {
                    0 => {
                        current_block = 14966503543959925615;
                        break;
                    }
                    46 => {}
                    _ => {
                        current_block = 10399321362245223758;
                        break;
                    }
                }
                if *sp == 0 {
                    if dots & 1 as libc::c_int != 0 {
                        current_block = 10399321362245223758;
                        break;
                    }
                    ugh = finish_string(single, &mut (*ans).size);
                    (*ans).string = ugh;
                    bp = branchtip((*ans).string);
                    if bp.is_null() {
                        current_block = 10628954987011324951;
                        break;
                    }
                    accf(
                        single,
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        (*ans).string,
                        bp.offset((*ans).size as isize),
                    );
                    current_block = 14966503543959925615;
                    break;
                } else {
                    dots += 1;
                    dots;
                    accumulate_byte(single, '.' as i32);
                }
            }
            match current_block {
                10628954987011324951 => {}
                14966503543959925615 => {}
                _ => {
                    generic_error(
                        (*top).repository.filename,
                        b"improper %s: %s\0" as *const u8 as *const libc::c_char,
                        ks_revno.as_ptr(),
                        source,
                    );
                    current_block = 10628954987011324951;
                }
            }
        }
        match current_block {
            14966503543959925615 => {}
            _ => {
                ugh = finish_string(single, &mut (*ans).size);
                (*ans).string = ugh;
                if !ugh.is_null() {
                    brush_off(single, ugh as *mut libc::c_void);
                }
                return 0 as libc::c_int != 0;
            }
        }
    }
    ugh = finish_string(single, &mut (*ans).size);
    (*ans).string = ugh;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn namedrev(
    mut name: *const libc::c_char,
    mut delta: *mut delta,
) -> *const libc::c_char {
    if !name.is_null() {
        let mut id: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut val: *const libc::c_char = 0 as *const libc::c_char;
        p = name;
        loop {
            let mut current_block_4: u64;
            match *ctab.as_ptr().offset(*p as libc::c_uchar as isize) as libc::c_uint {
                2 | 4 | 5 => {
                    id = name;
                    current_block_4 = 8515828400728868193;
                }
                1 => {
                    current_block_4 = 8515828400728868193;
                }
                9 => {
                    if *p == 0 && !id.is_null()
                        && {
                            val = lookupsym(id);
                            !val.is_null()
                        } && strcmp(val, (*delta).num) == 0
                    {
                        return id;
                    }
                    current_block_4 = 17438867486747911763;
                }
                _ => {
                    current_block_4 = 17438867486747911763;
                }
            }
            match current_block_4 {
                17438867486747911763 => return 0 as *const libc::c_char,
                _ => {}
            }
            p = p.offset(1);
            p;
        }
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn tiprev() -> *const libc::c_char {
    let mut tip: *mut delta = 0 as *mut delta;
    let mut defbr: *const libc::c_char = (*(*top).repository.r).branch;
    return if !defbr.is_null() {
        branchtip(defbr)
    } else {
        tip = (*top).repository.tip;
        if !tip.is_null() { (*tip).num } else { 0 as *const libc::c_char }
    };
}

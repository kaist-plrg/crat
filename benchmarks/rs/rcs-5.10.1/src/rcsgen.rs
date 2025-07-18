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
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn clearerr(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn enterstring(es: *mut editstuff, atat: *mut atat);
    static mut top: *mut top;
    static tiny_access: tinysym;
    static tiny_author: tinysym;
    static tiny_branch: tinysym;
    static tiny_branches: tinysym;
    static tiny_comment: tinysym;
    static tiny_commitid: tinysym;
    static tiny_date: tinysym;
    static tiny_desc: tinysym;
    static tiny_expand: tinysym;
    static tiny_head: tinysym;
    static tiny_integrity: tinysym;
    static tiny_locks: tinysym;
    static tiny_log: tinysym;
    static tiny_next: tinysym;
    static tiny_state: tinysym;
    static tiny_strict: tinysym;
    static tiny_symbols: tinysym;
    static tiny_text: tinysym;
    fn kwsub_string(i: kwsub) -> *const libc::c_char;
    fn make_editstuff() -> *mut editstuff;
    fn unmake_editstuff(es: *mut editstuff);
    fn openfcopy(f: *mut FILE);
    fn finishedit(
        es: *mut editstuff,
        delta: *const delta,
        outfile: *mut FILE,
        done: bool,
    );
    fn copystring(es: *mut editstuff, atat: *mut atat);
    fn editstring(es: *mut editstuff, script: *const atat, delta: *const delta);
    fn ORCSclose();
    fn unexpected_EOF();
    fn initdiffcmd(dc: *mut diffcmd);
    fn getdiffcmd(
        finfile: *mut fro,
        delimiter: bool,
        foutfile: *mut FILE,
        dc: *mut diffcmd,
    ) -> libc::c_int;
    fn awrite(buf: *const libc::c_char, chars: size_t, f: *mut FILE);
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn vcomplain(fmt: *const libc::c_char, args: ::std::ffi::VaList);
    fn complain(fmt: *const libc::c_char, _: ...);
    fn syserror(e: libc::c_int, who: *const libc::c_char);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn close_space(divvy: *mut divvy);
    fn accumulate_byte(divvy: *mut divvy, c: libc::c_int);
    fn Ierror();
    fn testIerror(f: *mut FILE);
    fn testOerror(o: *mut FILE);
    fn fopen_safer(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
    ) -> *mut FILE;
    fn Ozclose(p: *mut *mut FILE);
    fn oflush();
    fn afputc(c: libc::c_int, f: *mut FILE);
    fn newline(f: *mut FILE);
    fn aputs(s: *const libc::c_char, iop: *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const libc::c_char, _: ...);
    fn makedirtemp(isworkfile: bool) -> *const libc::c_char;
    fn fro_open(
        filename: *const libc::c_char,
        type_0: *const libc::c_char,
        status: *mut stat,
    ) -> *mut fro;
    fn fro_close(f: *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut libc::c_int, f: *mut fro) -> bool;
    fn fro_must_getbyte(c: *mut libc::c_int, f: *mut fro);
    fn fro_spew_partial(to: *mut FILE, f: *mut fro, r: *mut range);
    fn string_from_atat(space: *mut divvy, atat: *const atat) -> cbuf;
    fn atat_put(to: *mut FILE, atat: *const atat);
    fn expandline(ctx: *mut expctx) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
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
pub struct diffcmd {
    pub line1: libc::c_long,
    pub nlines: libc::c_long,
    pub adprev: libc::c_long,
    pub dafter: libc::c_long,
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
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 32],
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
pub type stringwork = libc::c_uint;
pub const edit_expand: stringwork = 4;
pub const expand: stringwork = 3;
pub const edit: stringwork = 2;
pub const copy: stringwork = 1;
pub const enter: stringwork = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range {
    pub beg: off_t,
    pub end: off_t,
}
unsafe extern "C" fn scandeltatext(
    mut es: *mut editstuff,
    mut ls: *mut *mut wlink,
    mut delta: *mut delta,
    mut func: stringwork,
    mut needlog: bool,
) {
    let mut nextdelta: *const delta = 0 as *const delta;
    let mut from: *mut fro = (*top).flow.from;
    let mut to: *mut FILE = (*top).flow.to;
    let mut log: *mut atat = 0 as *mut atat;
    let mut text: *mut atat = 0 as *mut atat;
    let mut range: range = range { beg: 0, end: 0 };
    loop {
        nextdelta = (**ls).entry as *const delta;
        log = (*nextdelta).log;
        text = (*nextdelta).text;
        range.beg = (*nextdelta).neck;
        range.end = (*text).beg;
        if needlog as libc::c_int != 0 && delta == nextdelta as *mut delta {
            (*delta).pretty_log = string_from_atat(single, log);
            (*delta)
                .pretty_log = cleanlogmsg(
                (*delta).pretty_log.string,
                (*delta).pretty_log.size,
            );
        }
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
    match func as libc::c_uint {
        0 => {
            enterstring(es, text);
        }
        1 => {
            copystring(es, text);
        }
        3 => {
            let mut c: libc::c_int = 0;
            let mut ctx: expctx = {
                let mut init = expctx {
                    to: (*top).flow.res,
                    rewr: to,
                    from: from,
                    delta: delta,
                    delimstuffed: 1 as libc::c_int != 0,
                    dolog: 1 as libc::c_int != 0,
                    lparts: 0 as *mut divvy,
                };
                init
            };
            fro_must_getbyte(&mut c, from);
            if !to.is_null() {
                afputc(c, to);
            }
            while (1 as libc::c_int) < expandline(&mut ctx) {}
            if !(ctx.lparts).is_null() {
                close_space(ctx.lparts);
            }
        }
        2 => {
            editstring(es, text, 0 as *const delta);
        }
        4 => {
            editstring(es, text, delta);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn buildrevision(
    mut deltas: *const wlink,
    mut target: *mut delta,
    mut outfile: *mut FILE,
    mut expandflag: bool,
) -> *const libc::c_char {
    let mut es: *mut editstuff = make_editstuff();
    let mut ls: *mut wlink = (*(*top).repository.r).deltas;
    if (*deltas).entry == target as *mut libc::c_void {
        openfcopy(outfile);
        scandeltatext(
            es,
            &mut ls,
            target,
            (if expandflag as libc::c_int != 0 {
                expand as libc::c_int
            } else {
                copy as libc::c_int
            }) as stringwork,
            1 as libc::c_int != 0,
        );
    } else {
        scandeltatext(
            es,
            &mut ls,
            (*deltas).entry as *mut delta,
            enter,
            0 as libc::c_int != 0,
        );
        loop {
            ls = (*ls).next;
            deltas = (*deltas).next;
            if ((*deltas).next).is_null() {
                break;
            }
            scandeltatext(
                es,
                &mut ls,
                (*deltas).entry as *mut delta,
                edit,
                0 as libc::c_int != 0,
            );
        }
        if expandflag as libc::c_int != 0 || !outfile.is_null() {
            finishedit(es, 0 as *const delta, outfile, 0 as libc::c_int != 0);
        }
        scandeltatext(
            es,
            &mut ls,
            target,
            (if expandflag as libc::c_int != 0 {
                edit_expand as libc::c_int
            } else {
                edit as libc::c_int
            }) as stringwork,
            1 as libc::c_int != 0,
        );
        finishedit(
            es,
            if expandflag as libc::c_int != 0 { target } else { 0 as *mut delta },
            outfile,
            1 as libc::c_int != 0,
        );
    }
    unmake_editstuff(es);
    if !outfile.is_null() {
        return 0 as *const libc::c_char;
    }
    Ozclose(&mut (*top).flow.res);
    return (*top).flow.result;
}
pub unsafe extern "C" fn cleanlogmsg(mut m: *const libc::c_char, mut s: size_t) -> cbuf {
    let mut r: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    while s != 0
        && (' ' as i32 == *m as libc::c_int || '\t' as i32 == *m as libc::c_int
            || '\n' as i32 == *m as libc::c_int)
    {
        s = s.wrapping_sub(1);
        s;
        m = m.offset(1);
        m;
    }
    while s != 0
        && (' ' as i32
            == *m.offset(s.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
            || '\t' as i32
                == *m.offset(s.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
            || '\n' as i32
                == *m.offset(s.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int)
    {
        s = s.wrapping_sub(1);
        s;
    }
    r.string = m;
    r.size = s;
    return r;
}
pub unsafe extern "C" fn ttystdin() -> bool {
    if !(*top).behavior.interactive_valid {
        if !(*top).behavior.interactive {
            (*top).behavior.interactive = isatty(0 as libc::c_int) != 0;
        }
        (*top).behavior.interactive_valid = 1 as libc::c_int != 0;
    }
    return (*top).behavior.interactive;
}
pub unsafe extern "C" fn getcstdin() -> libc::c_int {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    in_0 = stdin;
    if feof(in_0) != 0 && ttystdin() as libc::c_int != 0 {
        clearerr(in_0);
    }
    c = getc(in_0);
    if c == -(1 as libc::c_int) {
        testIerror(in_0);
        if feof(in_0) != 0 && ttystdin() as libc::c_int != 0 {
            complain(b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return c;
}
pub unsafe extern "C" fn yesorno(
    mut default_answer: bool,
    mut question: *const libc::c_char,
    mut args: ...
) -> bool {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut c: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if !(*top).behavior.quiet && ttystdin() as libc::c_int != 0 {
        let mut ans: *mut libc::c_char = (if default_answer as libc::c_int != 0 {
            b"yn\0" as *const u8 as *const libc::c_char
        } else {
            b"ny\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        oflush();
        args_0 = args.clone();
        vcomplain(question, args_0.as_va_list());
        complain(
            b"? [%s](%c): \0" as *const u8 as *const libc::c_char,
            ans,
            *ans.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        c = getcstdin();
        r = c;
        while c != '\n' as i32 && feof(stdin) == 0 {
            c = getcstdin();
        }
        if r == 'y' as i32 || r == 'Y' as i32 {
            return 1 as libc::c_int != 0;
        }
        if r == 'n' as i32 || r == 'N' as i32 {
            return 0 as libc::c_int != 0;
        }
    }
    return default_answer;
}
pub unsafe extern "C" fn write_desc_maybe(mut to: *mut FILE) {
    let mut desc: *mut atat = (*(*top).repository.r).desc;
    if !to.is_null() {
        atat_put(to, desc);
    }
}
pub unsafe extern "C" fn putdesc(
    mut cb: *mut cbuf,
    mut textflag: bool,
    mut textfile: *mut libc::c_char,
) {
    let mut txt: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut frew: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: size_t = 0;
    let mut from: *mut fro = (*top).flow.from;
    frew = (*top).flow.rewr;
    if !from.is_null() && !textflag {
        aprintf(
            frew,
            b"\n\n%s\n\0" as *const u8 as *const libc::c_char,
            (tiny_desc.bytes).as_ptr() as *const libc::c_char,
        );
        write_desc_maybe(frew);
    } else {
        (*top).flow.to = 0 as *mut FILE;
        aprintf(
            frew,
            b"\n\n%s\n\0" as *const u8 as *const libc::c_char,
            (tiny_desc.bytes).as_ptr() as *const libc::c_char,
        );
        if textfile.is_null() {
            *cb = getsstdin(
                b"t-\0" as *const u8 as *const libc::c_char,
                b"description\0" as *const u8 as *const libc::c_char,
                b"NOTE: This is NOT the log message!\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if ((*cb).string).is_null() {
            if *textfile as libc::c_int == '-' as i32 {
                p = textfile.offset(1 as libc::c_int as isize);
                s = strlen(p);
            } else {
                txt = fopen_safer(textfile, b"r\0" as *const u8 as *const libc::c_char);
                if txt.is_null() {
                    fatal_sys(textfile);
                }
                loop {
                    c = getc(txt);
                    if c == -(1 as libc::c_int) {
                        testIerror(txt);
                        if feof(txt) != 0 {
                            break;
                        }
                    }
                    accumulate_byte(plexus, c);
                }
                if 0 as libc::c_int > fclose(txt) {
                    Ierror();
                }
                p = finish_string(plexus, &mut s);
            }
            *cb = cleanlogmsg(p, s);
        }
        putstring(frew, *cb, 1 as libc::c_int != 0);
        newline(frew);
    };
}
pub unsafe extern "C" fn getsstdin(
    mut option: *const libc::c_char,
    mut name: *const libc::c_char,
    mut note: *const libc::c_char,
) -> cbuf {
    let mut c: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tty: bool = ttystdin();
    let mut len: size_t = 0;
    let mut column: size_t = 0 as libc::c_int as size_t;
    let mut dot_in_first_column: bool = 0 as libc::c_int != 0;
    let mut discard: bool = 0 as libc::c_int != 0;
    if tty {
        complain(
            b"enter %s, terminated with single '.' or end of file:\n%s>> \0" as *const u8
                as *const libc::c_char,
            name,
            note,
        );
    } else if feof(stdin) != 0 {
        generic_fatal(
            (*top).repository.filename,
            b"can't reread redirected stdin for %s; use -%s<%s>\0" as *const u8
                as *const libc::c_char,
            name,
            option,
            name,
        );
    }
    loop {
        c = getcstdin();
        if !(feof(stdin) == 0) {
            break;
        }
        if column == 0 {
            dot_in_first_column = '.' as i32 == c;
        }
        if c == '\n' as i32 {
            if 1 as libc::c_int as libc::c_ulong == column
                && dot_in_first_column as libc::c_int != 0
            {
                discard = 1 as libc::c_int != 0;
                break;
            } else {
                if tty {
                    complain(b">> \0" as *const u8 as *const libc::c_char);
                }
                column = 0 as libc::c_int as size_t;
            }
        } else {
            column = column.wrapping_add(1);
            column;
        }
        accumulate_byte(plexus, c);
    }
    p = finish_string(plexus, &mut len);
    return cleanlogmsg(
        p,
        len
            .wrapping_sub(
                (if discard as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong,
            ),
    );
}
pub unsafe extern "C" fn format_assocs(
    mut out: *mut FILE,
    mut fmt: *const libc::c_char,
) {
    let mut ls: *mut link = (*(*top).repository.r).symbols;
    while !ls.is_null() {
        let mut d: *const symdef = (*ls).entry as *const symdef;
        aprintf(out, fmt, (*d).meaningful, (*d).underlying);
        ls = (*ls).next;
    }
}
pub unsafe extern "C" fn format_locks(mut out: *mut FILE, mut fmt: *const libc::c_char) {
    let mut ls: *mut link = (*(*top).repository.r).locks;
    while !ls.is_null() {
        let mut rl: *const rcslock = (*ls).entry as *const rcslock;
        aprintf(out, fmt, (*rl).login, (*(*rl).delta).num);
        ls = (*ls).next;
    }
}
static mut semi_lf: *const libc::c_char = b";\n\0" as *const u8 as *const libc::c_char;
pub unsafe extern "C" fn putadmin() {
    let mut fout: *mut FILE = 0 as *mut FILE;
    let mut r: *mut repo = (*top).repository.r;
    let mut tip: *mut delta = (*top).repository.tip;
    let mut defbr: *const libc::c_char = if !r.is_null() {
        (*(*top).repository.r).branch
    } else {
        0 as *const libc::c_char
    };
    let mut kws: libc::c_int = (*top).behavior.kws;
    fout = (*top).flow.rewr;
    if fout.is_null() {
        let mut fo: libc::c_int = (*top).repository.fd_lock;
        (*top).repository.fd_lock = -(1 as libc::c_int);
        fout = fdopen(fo, b"w\0" as *const u8 as *const libc::c_char);
        (*top).flow.rewr = fout;
        if ((*top).flow.rewr).is_null() {
            fatal_sys((*top).repository.filename);
        }
    }
    aprintf(
        fout,
        b"%s\t%s%s\0" as *const u8 as *const libc::c_char,
        (tiny_head.bytes).as_ptr() as *const libc::c_char,
        if !tip.is_null() {
            (*tip).num
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        semi_lf,
    );
    if !defbr.is_null() && 4 as libc::c_int - 5 as libc::c_int <= (*top).behavior.version
    {
        aprintf(
            fout,
            b"%s\t%s%s\0" as *const u8 as *const libc::c_char,
            (tiny_branch.bytes).as_ptr() as *const libc::c_char,
            defbr,
            semi_lf,
        );
    }
    aputs((tiny_access.bytes).as_ptr() as *const libc::c_char, fout);
    let mut ls: *mut link = if !r.is_null() {
        (*(*top).repository.r).access
    } else {
        0 as *mut link
    };
    while !ls.is_null() {
        aprintf(
            fout,
            b"\n\t%s\0" as *const u8 as *const libc::c_char,
            (*ls).entry as *const libc::c_char,
        );
        ls = (*ls).next;
    }
    aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    aprintf(
        fout,
        b"%s\0" as *const u8 as *const libc::c_char,
        (tiny_symbols.bytes).as_ptr() as *const libc::c_char,
    );
    format_assocs(fout, b"\n\t%s:%s\0" as *const u8 as *const libc::c_char);
    aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    aprintf(
        fout,
        b"%s\0" as *const u8 as *const libc::c_char,
        (tiny_locks.bytes).as_ptr() as *const libc::c_char,
    );
    if !r.is_null() {
        format_locks(fout, b"\n\t%s:%s\0" as *const u8 as *const libc::c_char);
    }
    if (*top).behavior.strictly_locking {
        aprintf(
            fout,
            b"; %s\0" as *const u8 as *const libc::c_char,
            (tiny_strict.bytes).as_ptr() as *const libc::c_char,
        );
    }
    aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    if !((*(*top).repository.r).integrity).is_null() {
        aprintf(
            fout,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (tiny_integrity.bytes).as_ptr() as *const libc::c_char,
        );
        atat_put(fout, (*(*top).repository.r).integrity);
        aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    }
    if (*top).repository.log_lead.size != 0 {
        aprintf(
            fout,
            b"%s\t\0" as *const u8 as *const libc::c_char,
            (tiny_comment.bytes).as_ptr() as *const libc::c_char,
        );
        putstring(fout, (*top).repository.log_lead, 0 as libc::c_int != 0);
        aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    }
    if kws != kwsub_kv as libc::c_int {
        aprintf(
            fout,
            b"%s\t%c%s%c%s\0" as *const u8 as *const libc::c_char,
            (tiny_expand.bytes).as_ptr() as *const libc::c_char,
            '@' as i32,
            kwsub_string(kws as kwsub),
            '@' as i32,
            semi_lf,
        );
    }
    aprintf(fout, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn putdelta(mut node: *const delta, mut fout: *mut FILE) {
    if node.is_null() {
        return;
    }
    aprintf(
        fout,
        b"\n%s\n%s\t%s;\t%s %s;\t%s %s%s%s\0" as *const u8 as *const libc::c_char,
        (*node).num,
        (tiny_date.bytes).as_ptr() as *const libc::c_char,
        (*node).date,
        (tiny_author.bytes).as_ptr() as *const libc::c_char,
        (*node).author,
        (tiny_state.bytes).as_ptr() as *const libc::c_char,
        if !((*node).state).is_null() {
            (*node).state
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        semi_lf,
        (tiny_branches.bytes).as_ptr() as *const libc::c_char,
    );
    let mut ls: *mut wlink = (*node).branches;
    while !ls.is_null() {
        let mut delta: *mut delta = (*ls).entry as *mut delta;
        aprintf(fout, b"\n\t%s\0" as *const u8 as *const libc::c_char, (*delta).num);
        ls = (*ls).next;
    }
    aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    aprintf(
        fout,
        b"%s\t%s\0" as *const u8 as *const libc::c_char,
        (tiny_next.bytes).as_ptr() as *const libc::c_char,
        if !((*node).ilk).is_null() {
            (*(*node).ilk).num
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    aprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, semi_lf);
    if !((*node).commitid).is_null() {
        aprintf(
            fout,
            b"%s\t%s%s\0" as *const u8 as *const libc::c_char,
            (tiny_commitid.bytes).as_ptr() as *const libc::c_char,
            (*node).commitid,
            semi_lf,
        );
    }
}
pub unsafe extern "C" fn puttree(mut root: *const delta, mut fout: *mut FILE) {
    while !root.is_null() {
        if (*root).selector {
            putdelta(root, fout);
        }
        let mut ls: *mut wlink = (*root).branches;
        if ls.is_null() {
            root = (*root).ilk;
        } else {
            puttree((*root).ilk, fout);
            while !((*ls).next).is_null() {
                puttree((*ls).entry as *const delta, fout);
                ls = (*ls).next;
            }
            root = (*ls).entry as *const delta;
        }
    }
}
pub unsafe extern "C" fn putdtext(
    mut delta: *const delta,
    mut srcname: *const libc::c_char,
    mut fout: *mut FILE,
    mut diffmt: bool,
) -> bool {
    let mut fin: *mut fro = 0 as *mut fro;
    fin = fro_open(srcname, b"r\0" as *const u8 as *const libc::c_char, 0 as *mut stat);
    if fin.is_null() {
        syserror(*__errno_location(), srcname);
        return 0 as libc::c_int != 0;
    }
    putdftext(delta, fin, fout, diffmt);
    fro_close(fin);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn put_SDELIM(mut out: *mut FILE) {
    if putc('@' as i32, out) == -(1 as libc::c_int) {
        testOerror(out);
    }
}
pub unsafe extern "C" fn putstring(mut out: *mut FILE, mut s: cbuf, mut log: bool) {
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut delim: *const libc::c_char = 0 as *const libc::c_char;
    let mut ss: size_t = 0;
    let mut span: size_t = 0;
    put_SDELIM(out);
    sp = s.string;
    ss = s.size;
    while ss != 0 {
        delim = memchr(sp as *const libc::c_void, '@' as i32, ss) as *const libc::c_char;
        span = if !delim.is_null() {
            (delim.offset_from(sp) as libc::c_long as size_t)
                .wrapping_add(1 as libc::c_uint as libc::c_ulong)
        } else {
            ss
        };
        awrite(sp, span, out);
        if !delim.is_null() {
            put_SDELIM(out);
        }
        sp = sp.offset(span as isize);
        ss = (ss as libc::c_ulong).wrapping_sub(span) as size_t as size_t;
    }
    if s.size != 0 && log as libc::c_int != 0 {
        newline(out);
    }
    put_SDELIM(out);
}
pub unsafe extern "C" fn putdftext(
    mut delta: *const delta,
    mut finfile: *mut fro,
    mut foutfile: *mut FILE,
    mut diffmt: bool,
) {
    let mut fout: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut fin: *mut fro = 0 as *mut fro;
    let mut ed: libc::c_int = 0;
    let mut dc: diffcmd = diffcmd {
        line1: 0,
        nlines: 0,
        adprev: 0,
        dafter: 0,
    };
    fout = foutfile;
    aprintf(
        fout,
        b"\n\n%s\n%s\n\0" as *const u8 as *const libc::c_char,
        (*delta).num,
        (tiny_log.bytes).as_ptr() as *const libc::c_char,
    );
    putstring(fout, (*delta).pretty_log, 1 as libc::c_int != 0);
    newline(fout);
    aprintf(
        fout,
        b"%s\n%c\0" as *const u8 as *const libc::c_char,
        (tiny_text.bytes).as_ptr() as *const libc::c_char,
        '@' as i32,
    );
    fin = finfile;
    if !diffmt {
        while !fro_try_getbyte(&mut c, fin) {
            if c == '@' as i32 {
                put_SDELIM(fout);
            }
            if putc(c, fout) == -(1 as libc::c_int) {
                testOerror(fout);
            }
        }
    } else {
        initdiffcmd(&mut dc);
        's_89: loop {
            ed = getdiffcmd(fin, 0 as libc::c_int != 0, fout, &mut dc);
            if !(0 as libc::c_int <= ed) {
                break;
            }
            if !(ed != 0) {
                continue;
            }
            loop {
                let fresh0 = dc.nlines;
                dc.nlines = dc.nlines - 1;
                if !(fresh0 != 0) {
                    break;
                }
                loop {
                    if fro_try_getbyte(&mut c, fin) {
                        if dc.nlines == 0 {
                            break 's_89;
                        }
                        unexpected_EOF();
                    }
                    if c == '@' as i32 {
                        put_SDELIM(fout);
                    }
                    if putc(c, fout) == -(1 as libc::c_int) {
                        testOerror(fout);
                    }
                    if !(c != '\n' as i32) {
                        break;
                    }
                }
            }
        }
    }
    aprintf(fout, b"%c\n\0" as *const u8 as *const libc::c_char, '@' as i32);
}

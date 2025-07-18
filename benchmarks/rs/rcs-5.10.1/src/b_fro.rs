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
    static mut top: *mut top;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn awrite(buf: *const libc::c_char, chars: size_t, f: *mut FILE);
    fn __errno_location() -> *mut libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn fatal_syntax(lno: size_t, fmt: *const libc::c_char, _: ...);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn fatal_sys(who: *const libc::c_char);
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accumulate_nbytes(divvy: *mut divvy, start: *const libc::c_char, count: size_t);
    fn accumulate_byte(divvy: *mut divvy, c: libc::c_int);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut libc::c_char;
    fn Ierror();
    fn testIerror(f: *mut FILE);
    fn newline(f: *mut FILE);
    fn access_page(
        scratch: *mut isr_scratch,
        filename: *const libc::c_char,
        p: *const libc::c_char,
    ) -> libc::c_char;
    fn isr_do(scratch: *mut isr_scratch, action: isr_actions);
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
pub type __ssize_t = libc::c_long;
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
pub struct range {
    pub beg: off_t,
    pub end: off_t,
}
pub type isr_actions = libc::c_uint;
pub const ISR_CATCHMMAPINTS: isr_actions = 3;
pub const ISR_RESTOREINTS: isr_actions = 2;
pub const ISR_IGNOREINTS: isr_actions = 1;
pub const ISR_CATCHINTS: isr_actions = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn mmap_deallocate(mut f: *mut fro) {
    if 0 as libc::c_int
        > munmap(
            (*f).base as *mut libc::c_void,
            ((*f).lim).offset_from((*f).base) as libc::c_long as size_t,
        )
    {
        fatal_sys(b"munmap\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn fro_open(
    mut name: *const libc::c_char,
    mut type_0: *const libc::c_char,
    mut status: *mut stat,
) -> *mut fro {
    let mut current_block: u64;
    let mut f: *mut fro = 0 as *mut fro;
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut st: stat = stat {
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
    let mut s: off_t = 0;
    let mut fd: libc::c_int = fd_safer(
        open(
            name,
            0 as libc::c_int
                | (if 0 as libc::c_int != 0 && !(strchr(type_0, 'b' as i32)).is_null() {
                    0 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
        ),
    );
    let mut unlimited: bool = -(1 as libc::c_int) as libc::c_long
        == (*top).behavior.mem_limit;
    if 0 as libc::c_int > fd {
        return 0 as *mut fro;
    }
    if status.is_null() {
        status = &mut st;
    }
    if 0 as libc::c_int > fstat(fd, status) {
        fatal_sys(name);
    }
    if !((*status).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        generic_error(
            0 as *const libc::c_char,
            b"`%s' is not a regular file\0" as *const u8 as *const libc::c_char,
            name,
        );
        close(fd);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut fro;
    }
    s = (*status).st_size;
    f = zlloc(single, ::std::mem::size_of::<fro>() as libc::c_ulong) as *mut fro;
    (*f).end = s;
    (*f)
        .rm = (if unlimited as libc::c_int != 0
        || ((*status).st_size >> 10 as libc::c_int) < (*top).behavior.mem_limit
    {
        if 7 as libc::c_int != 0 && (*status).st_size != 0 {
            RM_MMAP as libc::c_int
        } else {
            RM_MEM as libc::c_int
        }
    } else {
        RM_STDIO as libc::c_int
    }) as readmethod;
    '_retry: loop {
        match (*f).rm as libc::c_uint {
            0 => {
                if s != (*status).st_size {
                    generic_fatal(
                        0 as *const libc::c_char,
                        b"%s: too large\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
                (*f).stream = 0 as *mut FILE;
                (*f).deallocate = None;
                isr_do((*top).behavior.isr, ISR_CATCHMMAPINTS);
                (*f)
                    .base = mmap(
                    0 as *mut libc::c_void,
                    s as size_t,
                    0x1 as libc::c_int,
                    0x1 as libc::c_int,
                    fd,
                    0 as libc::c_int as __off_t,
                ) as *mut libc::c_char;
                if !((*f).base
                    == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char)
                {
                    current_block = 11307063007268554308;
                    break;
                }
                if unlimited {
                    (*f).rm = RM_MEM;
                } else {
                    fatal_sys(name);
                    current_block = 11307063007268554308;
                    break;
                }
            }
            1 => {
                if s == 0 {
                    (*f).base = 0 as *mut libc::c_char;
                    current_block = 11777552016271000781;
                    break;
                } else {
                    let mut r: ssize_t = 0;
                    let mut bufptr: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut bufsiz: size_t = s as size_t;
                    (*f).base = alloc(single, s as size_t) as *mut libc::c_char;
                    bufptr = (*f).base;
                    loop {
                        r = read(fd, bufptr as *mut libc::c_void, bufsiz);
                        if 0 as libc::c_int as libc::c_long > r {
                            if unlimited {
                                (*f).rm = RM_STDIO;
                                continue '_retry;
                            } else {
                                fatal_sys(name);
                            }
                        }
                        if r == 0 {
                            s = (s as libc::c_ulong).wrapping_sub(bufsiz) as off_t
                                as off_t;
                            (*status).st_size = s;
                            bufsiz = 0 as libc::c_int as size_t;
                        } else {
                            bufptr = bufptr.offset(r as isize);
                            bufsiz = (bufsiz as libc::c_ulong)
                                .wrapping_sub(r as libc::c_ulong) as size_t as size_t;
                        }
                        if !(bufsiz != 0) {
                            break;
                        }
                    }
                    if !(0 as libc::c_int as libc::c_long
                        > lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int))
                    {
                        current_block = 11777552016271000781;
                        break;
                    }
                    if unlimited {
                        (*f).rm = RM_STDIO;
                    } else {
                        fatal_sys(name);
                        current_block = 11777552016271000781;
                        break;
                    }
                }
            }
            2 => {
                stream = fdopen(fd, type_0);
                if stream.is_null() {
                    fatal_sys(name);
                }
                (*f).stream = stream;
                current_block = 6072622540298447352;
                break;
            }
            _ => {
                current_block = 6072622540298447352;
                break;
            }
        }
    }
    match current_block {
        11307063007268554308 => {
            access_page((*top).behavior.isr, name, (*f).base);
            (*f)
                .deallocate = Some(
                mmap_deallocate as unsafe extern "C" fn(*mut fro) -> (),
            );
            (*f).ptr = (*f).base;
            (*f).lim = ((*f).base).offset(s as isize);
            fro_trundling(1 as libc::c_int != 0, f);
        }
        11777552016271000781 => {
            (*f).ptr = (*f).base;
            (*f).lim = ((*f).base).offset(s as isize);
        }
        _ => {}
    }
    (*f).fd = fd;
    return f;
}
pub unsafe extern "C" fn fro_close(mut f: *mut fro) {
    let mut res: libc::c_int = -(1 as libc::c_int);
    if f.is_null() {
        return;
    }
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            if ((*f).deallocate).is_some() {
                (Some(((*f).deallocate).unwrap())).unwrap()(f);
            }
            (*f).base = 0 as *mut libc::c_char;
            res = close((*f).fd);
        }
        2 => {
            res = fclose((*f).stream);
        }
        _ => {}
    }
    if res != 0 {
        Ierror();
    }
    (*f).fd = -(1 as libc::c_int);
}
pub unsafe extern "C" fn fro_zclose(mut p: *mut *mut fro) {
    fro_close(*p);
    *p = 0 as *mut fro;
}
pub unsafe extern "C" fn fro_tello(mut f: *mut fro) -> off_t {
    let mut rv: off_t = 0 as libc::c_int as off_t;
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            rv = ((*f).ptr).offset_from((*f).base) as libc::c_long;
        }
        2 => {
            rv = ftello((*f).stream);
        }
        _ => {}
    }
    return rv;
}
pub unsafe extern "C" fn fro_move(mut f: *mut fro, mut change: off_t) {
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            (*f)
                .ptr = (if 0 as libc::c_int as libc::c_long > change {
                (*f).ptr
            } else {
                (*f).base
            })
                .offset(change as isize);
        }
        2 => {
            if 0 as libc::c_int
                > fseeko(
                    (*f).stream,
                    change,
                    if 0 as libc::c_int as libc::c_long > change {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                )
            {
                Ierror();
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn fro_try_getbyte(
    mut c: *mut libc::c_int,
    mut f: *mut fro,
) -> bool {
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            if (*f).ptr == (*f).lim {
                return 1 as libc::c_int != 0;
            }
            let fresh0 = (*f).ptr;
            (*f).ptr = ((*f).ptr).offset(1);
            *c = *fresh0 as libc::c_int;
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut maybe: libc::c_int = getc(stream);
            if -(1 as libc::c_int) == maybe {
                testIerror(stream);
                return 1 as libc::c_int != 0;
            }
            *c = maybe;
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn fro_must_getbyte(mut c: *mut libc::c_int, mut f: *mut fro) {
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            if (*f).ptr == (*f).lim {
                fatal_syntax(
                    0 as libc::c_int as size_t,
                    b"unexpected end of file\0" as *const u8 as *const libc::c_char,
                );
            }
            let fresh1 = (*f).ptr;
            (*f).ptr = ((*f).ptr).offset(1);
            *c = *fresh1 as libc::c_int;
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut maybe: libc::c_int = getc(stream);
            if -(1 as libc::c_int) == maybe {
                testIerror(stream);
                fatal_syntax(
                    0 as libc::c_int as size_t,
                    b"unexpected end of file\0" as *const u8 as *const libc::c_char,
                );
            }
            *c = maybe;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn fro_trundling(mut sequential: bool, mut f: *mut fro) {
    match (*f).rm as libc::c_uint {
        0 => {
            madvise(
                (*f).base as *mut libc::c_void,
                ((*f).lim).offset_from((*f).base) as libc::c_long as size_t,
                if sequential as libc::c_int != 0 {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                },
            );
        }
        1 | 2 | _ => {}
    };
}
pub unsafe extern "C" fn fro_spew_partial(
    mut to: *mut FILE,
    mut f: *mut fro,
    mut r: *mut range,
) {
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            awrite(
                ((*f).base).offset((*r).beg as isize),
                ((*r).end - (*r).beg) as size_t,
                to,
            );
            if (*f).end == (*r).end {
                (*f).ptr = (*f).lim;
            }
        }
        2 => {
            let mut buf: [libc::c_char; 65536] = [0; 65536];
            let mut count: size_t = 0;
            let mut pos: off_t = (*r).beg;
            fseeko((*f).stream, pos, 0 as libc::c_int);
            while pos < (*r).end {
                count = fread(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    (if pos
                        < (*r).end
                            - (8 as libc::c_int * 8192 as libc::c_int) as libc::c_long
                    {
                        (8 as libc::c_int * 8192 as libc::c_int) as libc::c_long
                    } else {
                        (*r).end - pos
                    }) as libc::c_ulong,
                    (*f).stream,
                );
                if count == 0 {
                    testIerror((*f).stream);
                    return;
                }
                awrite(buf.as_mut_ptr(), count, to);
                pos = (pos as libc::c_ulong).wrapping_add(count) as off_t as off_t;
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn fro_spew(mut f: *mut fro, mut to: *mut FILE) {
    let mut finish: range = {
        let mut init = range {
            beg: (*f).verbatim,
            end: (*f).end,
        };
        init
    };
    fro_spew_partial(to, f, &mut finish);
    (*f).verbatim = (*f).end;
}
pub unsafe extern "C" fn string_from_atat(
    mut space: *mut divvy,
    mut atat: *const atat,
) -> cbuf {
    let mut f: *mut fro = (*atat).from;
    let mut count: size_t = (*atat).count;
    let mut cb: cbuf = cbuf {
        string: 0 as *const libc::c_char,
        size: 0,
    };
    let mut i: size_t = 0;
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            i = 0 as libc::c_int as size_t;
            while i < count {
                let mut rbeg: off_t = 1 as libc::c_int as libc::c_long
                    + (if i != 0 {
                        *((*atat).holes)
                            .as_ptr()
                            .offset(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    } else {
                        (*atat).beg
                    });
                let mut beg: *const libc::c_char = ((*f).base).offset(rbeg as isize);
                let mut len: off_t = *((*atat).holes).as_ptr().offset(i as isize) - rbeg;
                while (9223372036854775807 as libc::c_long) < len {
                    accumulate_nbytes(
                        space,
                        beg,
                        9223372036854775807 as libc::c_long as size_t,
                    );
                    len -= 9223372036854775807 as libc::c_long;
                    beg = beg.offset(9223372036854775807 as libc::c_long as isize);
                }
                accumulate_nbytes(space, beg, len as size_t);
                i = i.wrapping_add(1);
                i;
            }
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut was: off_t = ftello(stream);
            i = 0 as libc::c_int as size_t;
            while i < count {
                let mut pos: off_t = 1 as libc::c_int as libc::c_long
                    + (if i != 0 {
                        *((*atat).holes)
                            .as_ptr()
                            .offset(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    } else {
                        (*atat).beg
                    });
                fseeko(stream, pos, 0 as libc::c_int);
                loop {
                    let fresh2 = pos;
                    pos = pos + 1;
                    if !(fresh2 < *((*atat).holes).as_ptr().offset(i as isize)) {
                        break;
                    }
                    accumulate_byte(space, getc((*f).stream));
                }
                i = i.wrapping_add(1);
                i;
            }
            fseeko(stream, was, 0 as libc::c_int);
        }
        _ => {}
    }
    cb.string = finish_string(space, &mut cb.size);
    return cb;
}
pub unsafe extern "C" fn atat_put(mut to: *mut FILE, mut atat: *const atat) {
    let mut range: range = {
        let mut init = range {
            beg: (*atat).beg,
            end: *((*atat).holes)
                .as_ptr()
                .offset(
                    ((*atat).count).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) + 2 as libc::c_int as libc::c_long,
        };
        init
    };
    fro_spew_partial(to, (*atat).from, &mut range);
}
pub unsafe extern "C" fn atat_display(
    mut to: *mut FILE,
    mut atat: *const atat,
    mut ensure_end_nl: bool,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*atat).count {
        let mut range: range = {
            let mut init = range {
                beg: 1 as libc::c_int as libc::c_long
                    + (if i != 0 {
                        *((*atat).holes)
                            .as_ptr()
                            .offset(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            )
                    } else {
                        (*atat).beg
                    }),
                end: *((*atat).holes).as_ptr().offset(i as isize),
            };
            init
        };
        fro_spew_partial(to, (*atat).from, &mut range);
        i = i.wrapping_add(1);
        i;
    }
    if !ensure_end_nl
        || 1 as libc::c_int as libc::c_ulong == (*atat).count
            && (*atat).beg + 1 as libc::c_int as libc::c_long
                == *((*atat).holes).as_ptr().offset(0 as libc::c_int as isize)
    {
        return;
    }
    let mut f: *mut fro = (*atat).from;
    let mut pos: off_t = *((*atat).holes)
        .as_ptr()
        .offset(((*atat).count).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        - 1 as libc::c_int as libc::c_long;
    let mut lc: libc::c_char = '\0' as i32 as libc::c_char;
    match (*f).rm as libc::c_uint {
        0 | 1 => {
            lc = *((*f).base).offset(pos as isize);
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut was: off_t = ftello(stream);
            fseeko(stream, pos, 0 as libc::c_int);
            lc = fgetc(stream) as libc::c_char;
            fseeko(stream, was, 0 as libc::c_int);
        }
        _ => {}
    }
    if '\n' as i32 != lc as libc::c_int {
        newline(to);
    }
}
